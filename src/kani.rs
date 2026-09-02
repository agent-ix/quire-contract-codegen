//! Deterministic, version-adapted Kani proof lowering.

use std::{collections::BTreeSet, fmt::Write as _, sync::OnceLock};

use quire_contract_ir::{
    ClauseId, DependencyIdentity, DependencyKind, RequirementRef, StateObservation, TypedExpression,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::{
    generate_boolean_oracle,
    oracle::{
        attestation_context_is_valid, dependency_parameters, generated_output_attestation,
        length_delimited_identity, oracle_symbol, GeneratedAttestationSpec,
    },
    Artifact, AttestationContext, GenerationErrorCode, GenerationTerminalState, OracleRequest,
    MAX_GENERATED_SOURCE_BYTES,
};

/// Exact first supported Kani backend version.
pub const KANI_BACKEND_VERSION: &str = "0.67.0";

/// Stable identity for the isolated first function-contract adapter.
pub const KANI_ADAPTER_PROFILE: &str = "kani-0.67.0-function-contracts-v1";

const PROOF_GRAPH_SCHEMA: &[u8] = include_bytes!("../schemas/kani-proof-graph-v1.schema.json");
const RUST_KANI_SCHEMA: &[u8] = include_bytes!("../schemas/generated-rust-kani-v1.schema.json");
const KANI_SPEC: &[u8] = include_bytes!("../spec/functional/FR-003-kani-lowering.md");
const KANI_SOURCE: &[u8] = include_bytes!("kani.rs");
const ORACLE_SOURCE: &[u8] = include_bytes!("oracle.rs");
const BUILD_SOURCE: &[u8] = include_bytes!("../build.rs");
const LOCKFILE: &[u8] = include_bytes!("../Cargo.lock");

/// Kind of proof dependency declared by one generated harness.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofDependencyKind {
    /// A separately executed proof that must pass.
    Required,
    /// A Boolean dependency predicate introduced with `kani::assume`.
    Assumed,
    /// A function replacement introduced with `kani::stub`.
    Stubbed,
}

/// State of one declared proof dependency at generation time.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofDependencyState {
    /// A required dependency proof passed under its retained identity.
    Passed,
    /// A required dependency proof has no retained result.
    Missing,
    /// A required dependency proof failed.
    Failed,
    /// The dependency is explicitly assumed rather than proved.
    Assumed,
    /// The dependency implementation is explicitly replaced by a stub.
    Stubbed,
}

/// Generation-time readiness derived from the complete dependency census.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofReadiness {
    /// Every required proof passed and no assumptions or stubs are present.
    Ready,
    /// Required proofs passed, but an assumption or stub makes the proof conditional.
    Conditional,
    /// A required proof is missing or failed.
    Incomplete,
}

/// Supported solver choice for the first pinned adapter.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniSolver {
    /// Kani's CaDiCaL SAT solver.
    Cadical,
}

impl KaniSolver {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Cadical => "cadical",
        }
    }
}

/// One caller-declared proof dependency.
#[derive(Clone, Copy, Debug)]
pub struct ProofDependencyRequest<'a> {
    /// Stable dependency proof identity.
    pub proof_id: &'a str,
    /// Relationship to the generated root proof.
    pub kind: ProofDependencyKind,
    /// Current retained dependency state.
    pub state: ProofDependencyState,
    /// Assumption predicate or original stubbed function path, when required by `kind`.
    pub original_path: Option<&'a str>,
    /// Stub replacement function path, present only for `Stubbed`.
    pub replacement_path: Option<&'a str>,
}

/// Explicit inputs for one bounded Boolean Kani proof bundle.
pub struct KaniRequest<'a> {
    /// Requirement identity and revision retained by every artifact.
    pub requirement: &'a RequirementRef,
    /// Boolean precondition clause.
    pub precondition_clause: &'a ClauseId,
    /// Boolean postcondition clause.
    pub postcondition_clause: &'a ClauseId,
    /// Validated Boolean precondition expression.
    pub precondition: &'a TypedExpression,
    /// Validated Boolean postcondition expression.
    pub postcondition: &'a TypedExpression,
    /// Stable proof identity within the requirement revision.
    pub proof_id: &'a str,
    /// Rust path to a customer function with signature `fn(bool, bool) -> bool`.
    pub subject_path: &'a str,
    /// Exact requested `cargo-kani` version.
    pub backend_version: &'a str,
    /// Lowercase SHA-256 identity supplied for the backend executable distribution.
    pub backend_executable_sha256: &'a str,
    /// Explicit bounded loop unwind value.
    pub unwind: u32,
    /// Explicit solver choice.
    pub solver: KaniSolver,
    /// Complete caller-owned dependency census.
    pub dependencies: &'a [ProofDependencyRequest<'a>],
    /// Caller-owned binding shared by the two generated-artifact attestations.
    pub attestation: AttestationContext<'a>,
}

/// Stable reason a Kani bundle could not be generated.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniErrorCode {
    /// The requested Kani version is not supported by the pinned adapter.
    UnsupportedBackendVersion,
    /// A proof, subject, assumption, or stub identity is invalid or duplicated.
    InvalidIdentity,
    /// A dependency kind/state/path combination is invalid.
    InvalidDependency,
    /// The first slice cannot bind the supplied clause dependencies to `fn(bool, bool) -> bool`.
    UnsupportedBinding,
    /// A Boolean clause could not be lowered without approximation.
    ClauseGenerationFailed,
    /// The explicit unwind value is zero or exceeds the first-slice bound.
    InvalidUnwind,
    /// Caller attestation context is invalid.
    InvalidAttestationContext,
    /// Generated Rust did not parse.
    InvalidGeneratedSyntax,
    /// A deterministic graph or attestation could not be serialized.
    SerializationFailed,
    /// The generated source exceeds the bounded artifact size.
    ResourceLimitExceeded,
}

impl KaniErrorCode {
    /// Maps a Kani diagnostic category to interface-001 terminal state.
    #[must_use]
    pub const fn terminal_state(self) -> GenerationTerminalState {
        match self {
            Self::UnsupportedBackendVersion => GenerationTerminalState::BackendUnavailable,
            Self::InvalidIdentity
            | Self::InvalidDependency
            | Self::InvalidUnwind
            | Self::InvalidAttestationContext => GenerationTerminalState::InvalidInput,
            Self::UnsupportedBinding | Self::ResourceLimitExceeded => {
                GenerationTerminalState::Unsupported
            }
            Self::ClauseGenerationFailed
            | Self::InvalidGeneratedSyntax
            | Self::SerializationFailed => GenerationTerminalState::Inconclusive,
        }
    }
}

/// Structured Kani-generation failure returned without a partial bundle.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct KaniDiagnostic {
    /// Stable Kani diagnostic category.
    pub code: KaniErrorCode,
    /// Interface-001 terminal state.
    pub terminal_state: GenerationTerminalState,
    /// Preserved Boolean-lowering code, when the failure originated in an oracle clause.
    pub generation_code: Option<GenerationErrorCode>,
    /// Stable path to the rejected request element.
    pub path: String,
    /// Human-readable detail not used as machine identity.
    pub message: String,
}

/// One normalized proof dependency edge in the generated graph.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProofDependencyEdge {
    /// Stable dependency proof identity.
    pub proof_id: String,
    /// Dependency kind.
    pub kind: ProofDependencyKind,
    /// Dependency state.
    pub state: ProofDependencyState,
    /// Generated assumption/stub source-site identity, or `None` for required proof edges.
    pub source_site: Option<String>,
}

/// Deterministic Kani proof-dependency graph.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProofDependencyGraph {
    /// Stable graph schema identity.
    pub schema_version: String,
    /// Root proof identity.
    pub proof_id: String,
    /// Requirement identity.
    pub requirement_id: String,
    /// Requirement revision.
    pub requirement_revision: u64,
    /// Exact adapter profile.
    pub adapter_profile: String,
    /// Exact Kani version.
    pub backend_version: String,
    /// Caller-supplied identity of the pinned Kani executable distribution.
    pub backend_executable_sha256: String,
    /// Complete Kani option vector.
    pub options: Vec<String>,
    /// Derived dependency readiness; this is not a proof execution result.
    pub readiness: ProofReadiness,
    /// Explicit proof execution state; generation never executes or classifies a proof.
    pub proof_execution_state: String,
    /// Generated Rust artifact path.
    pub source_artifact_path: String,
    /// Generated Rust artifact digest.
    pub source_artifact_sha256: String,
    /// Sorted complete dependency census.
    pub dependencies: Vec<ProofDependencyEdge>,
}

/// All-or-nothing Kani source, proof graph, and shared attestations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniArtifactBundle {
    /// Generated Rust contract and proof source.
    pub rust: Artifact,
    /// Deterministic proof-dependency graph.
    pub proof_graph: Artifact,
    /// Quoin proof-attestation body for the Rust artifact.
    pub rust_attestation: Artifact,
    /// Quoin proof-attestation body for the proof graph.
    pub proof_graph_attestation: Artifact,
}

struct KaniBinding {
    input_name: String,
    state_name: String,
}

/// Generates one bounded Kani contract/proof bundle or structured diagnostics with no partial output.
// Implements: FR-003
pub fn generate_kani_bundle(
    request: &KaniRequest<'_>,
) -> Result<KaniArtifactBundle, Vec<KaniDiagnostic>> {
    validate_request(request)?;
    let precondition_request = OracleRequest {
        requirement: request.requirement,
        clause: request.precondition_clause,
        expression: request.precondition,
        attestation: request.attestation,
    };
    let postcondition_request = OracleRequest {
        requirement: request.requirement,
        clause: request.postcondition_clause,
        expression: request.postcondition,
        attestation: request.attestation,
    };
    let precondition = generate_boolean_oracle(&precondition_request)
        .map_err(|values| map_clause_diagnostics("precondition", values))?;
    let postcondition = generate_boolean_oracle(&postcondition_request)
        .map_err(|values| map_clause_diagnostics("postcondition", values))?;
    let precondition_parameters = dependency_parameters(&precondition_request)
        .map_err(|values| map_clause_diagnostics("precondition", values))?;
    let postcondition_parameters = dependency_parameters(&postcondition_request)
        .map_err(|values| map_clause_diagnostics("postcondition", values))?;
    let binding = derive_binding(&precondition_parameters, &postcondition_parameters)?;
    let requirement = request.requirement.requirement().as_str();
    let revision = request.requirement.revision().get();
    let symbol = kani_symbol(requirement, revision, request.proof_id);
    let contract_symbol = format!("{symbol}_contract");
    let harness_symbol = format!("{symbol}_proof");
    let module_symbol = format!("{symbol}_module");
    let precondition_symbol =
        oracle_symbol(requirement, revision, request.precondition_clause.as_str());
    let postcondition_symbol =
        oracle_symbol(requirement, revision, request.postcondition_clause.as_str());
    let precondition_arguments = predicate_arguments(&precondition_parameters, false)?;
    let postcondition_arguments = predicate_arguments(&postcondition_parameters, true)?;
    let exact_harness = format!("{module_symbol}::{harness_symbol}");
    let options = adapter_options(
        &exact_harness,
        request.unwind,
        request.solver,
        request
            .dependencies
            .iter()
            .any(|dependency| dependency.kind == ProofDependencyKind::Stubbed),
    );
    let normalized_dependencies = normalize_dependencies(request.dependencies);
    let mut source_dependencies = request.dependencies.iter().collect::<Vec<_>>();
    source_dependencies.sort_by(|left, right| left.proof_id.cmp(right.proof_id));
    let stub_attributes = source_dependencies
        .iter()
        .filter(|dependency| dependency.kind == ProofDependencyKind::Stubbed)
        .filter_map(|dependency| {
            dependency
                .original_path
                .zip(dependency.replacement_path)
                .map(|(original, replacement)| {
                    let site = dependency_site("stub", dependency.proof_id);
                    format!(
                        "    // proof-dependency-site: {site}\n    #[kani::stub({original}, {replacement})]\n"
                    )
                })
        })
        .collect::<String>();
    let assumption_statements = source_dependencies
        .iter()
        .filter(|dependency| dependency.kind == ProofDependencyKind::Assumed)
        .filter_map(|dependency| {
            dependency.original_path.map(|path| {
                let site = dependency_site("assumption", dependency.proof_id);
                format!(
                    "        // proof-dependency-site: {site}\n        kani::assume({path}());\n"
                )
            })
        })
        .collect::<String>();
    let conditional_attribute = "cfg";
    let kani_configuration = "kani";
    let source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Generated by quire-contract-codegen {}; DO NOT EDIT.\n\
// Requirement: {requirement}@{revision}; Proof: {}\n\
// Kani adapter: {KANI_ADAPTER_PROFILE}; backend: {KANI_BACKEND_VERSION}\n\
\n\
{}\n\
{}\n\
#[{conditional_attribute}({kani_configuration})]\n\
mod {module_symbol} {{\n\
    use super::*;\n\
\n\
    // BEGIN framing\n\
    // proof-id: {}\n\
    // input-binding: {}\n\
    // state-binding: {}\n\
    // END framing\n\
\n\
    // BEGIN binding\n\
    fn call_subject(input: bool, pre_state: bool) -> bool {{\n\
        {}(input, pre_state)\n\
    }}\n\
    // END binding\n\
\n\
    // BEGIN contract\n\
    #[kani::requires({precondition_symbol}({precondition_arguments}))]\n\
    #[kani::ensures(|post_state: &bool| {postcondition_symbol}({postcondition_arguments}))]\n\
    fn {contract_symbol}(input: bool, pre_state: bool) -> bool {{\n\
        call_subject(input, pre_state)\n\
    }}\n\
    // END contract\n\
\n\
    // BEGIN proof harness\n\
{stub_attributes}    #[kani::proof_for_contract({contract_symbol})]\n\
    fn {harness_symbol}() {{\n\
{assumption_statements}        let input: bool = kani::any();\n\
        let pre_state: bool = kani::any();\n\
        let _post_state = {contract_symbol}(input, pre_state);\n\
    }}\n\
    // END proof harness\n\
}}\n",
        env!("CARGO_PKG_VERSION"),
        request.proof_id,
        precondition.rust.contents,
        postcondition.rust.contents,
        request.proof_id,
        binding.input_name,
        binding.state_name,
        request.subject_path,
    );
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(single_diagnostic(
            KaniErrorCode::ResourceLimitExceeded,
            "generated.rust",
            "generated Kani source exceeds the bounded artifact size",
        ));
    }
    syn::parse_file(&source).map_err(|error| {
        single_diagnostic(
            KaniErrorCode::InvalidGeneratedSyntax,
            "generated.rust",
            &error.to_string(),
        )
    })?;
    let rust = artifact(format!("src/generated/{symbol}.rs"), source);
    let graph_value = ProofDependencyGraph {
        schema_version: "quire.kani-proof-graph/v1".to_owned(),
        proof_id: request.proof_id.to_owned(),
        requirement_id: requirement.to_owned(),
        requirement_revision: revision,
        adapter_profile: KANI_ADAPTER_PROFILE.to_owned(),
        backend_version: KANI_BACKEND_VERSION.to_owned(),
        backend_executable_sha256: request.backend_executable_sha256.to_owned(),
        options: options.clone(),
        readiness: dependency_readiness(&normalized_dependencies),
        proof_execution_state: "not_run".to_owned(),
        source_artifact_path: rust.path.clone(),
        source_artifact_sha256: rust.sha256.clone(),
        dependencies: normalized_dependencies,
    };
    let graph_contents = deterministic_json(&graph_value).map_err(|message| {
        single_diagnostic(
            KaniErrorCode::SerializationFailed,
            "generated.proof_graph",
            &message,
        )
    })?;
    let proof_graph = artifact(format!("proof-graphs/{symbol}.json"), graph_contents);
    let option_identity = options.join("\n");
    let request_identity = length_delimited_identity(&[
        request.proof_id,
        request.subject_path,
        request.precondition_clause.as_str(),
        request.postcondition_clause.as_str(),
        request.backend_version,
        request.backend_executable_sha256,
        &request.unwind.to_string(),
        request.solver.as_str(),
        &option_identity,
        &precondition.rust.sha256,
        &postcondition.rust.sha256,
        &proof_graph.sha256,
    ]);
    let readiness = match graph_value.readiness {
        ProofReadiness::Ready => "ready",
        ProofReadiness::Conditional => "conditional",
        ProofReadiness::Incomplete => "incomplete",
    };
    let mut attestation_arguments = vec![
        "--proof-id".to_owned(),
        request.proof_id.to_owned(),
        "--subject".to_owned(),
        request.subject_path.to_owned(),
        "--adapter-profile".to_owned(),
        KANI_ADAPTER_PROFILE.to_owned(),
        "--backend-version".to_owned(),
        KANI_BACKEND_VERSION.to_owned(),
        "--backend-executable-sha256".to_owned(),
        request.backend_executable_sha256.to_owned(),
        "--unwind".to_owned(),
        request.unwind.to_string(),
        "--solver".to_owned(),
        request.solver.as_str().to_owned(),
        "--dependency-readiness".to_owned(),
        readiness.to_owned(),
        "--proof-execution-state".to_owned(),
        "not_run".to_owned(),
    ];
    for option in &options {
        attestation_arguments.extend(["--kani-option".to_owned(), option.clone()]);
    }
    let configuration_digest = kani_implementation_digest();
    let rust_attestation = generated_output_attestation(
        &request.attestation,
        request.requirement,
        &GeneratedAttestationSpec {
            operation: "generate_kani_bundle",
            stable_identity: request.proof_id,
            input_bytes: request_identity.as_bytes(),
            output_role: "generated-rust-kani-proof",
            media_type: "text/x-rust",
            output_schema: "quire.codegen.rust-kani/v1",
            schema_digest: Some(rust_kani_schema_digest()),
            canonical_profile: KANI_ADAPTER_PROFILE,
            backend: "cargo-kani",
            configuration_digest,
            extra_argv: &attestation_arguments,
        },
        &rust,
    )
    .map_err(|code| attestation_diagnostic(code, "generated.rust_attestation"))?;
    let proof_graph_attestation = generated_output_attestation(
        &request.attestation,
        request.requirement,
        &GeneratedAttestationSpec {
            operation: "generate_kani_bundle",
            stable_identity: request.proof_id,
            input_bytes: request_identity.as_bytes(),
            output_role: "kani-proof-dependency-graph",
            media_type: "application/json",
            output_schema: "quire.codegen.kani-proof-graph/v1",
            schema_digest: Some(proof_graph_schema_digest()),
            canonical_profile: KANI_ADAPTER_PROFILE,
            backend: "cargo-kani",
            configuration_digest,
            extra_argv: &attestation_arguments,
        },
        &proof_graph,
    )
    .map_err(|code| attestation_diagnostic(code, "generated.proof_graph_attestation"))?;
    Ok(KaniArtifactBundle {
        rust,
        proof_graph,
        rust_attestation,
        proof_graph_attestation,
    })
}

fn validate_request(request: &KaniRequest<'_>) -> Result<(), Vec<KaniDiagnostic>> {
    if request.backend_version != KANI_BACKEND_VERSION {
        return Err(single_diagnostic(
            KaniErrorCode::UnsupportedBackendVersion,
            "backend_version",
            "the requested Kani version has no supported adapter",
        ));
    }
    if request.precondition_clause == request.postcondition_clause {
        return Err(single_diagnostic(
            KaniErrorCode::InvalidIdentity,
            "clauses",
            "precondition and postcondition clause identities must be distinct",
        ));
    }
    validate_plain_identity(request.proof_id, "proof_id")?;
    validate_path(request.subject_path, "subject_path")?;
    if !is_sha256(request.backend_executable_sha256) {
        return Err(single_diagnostic(
            KaniErrorCode::InvalidIdentity,
            "backend_executable_sha256",
            "backend executable identity must be lowercase SHA-256",
        ));
    }
    if request.unwind == 0 || request.unwind > 1024 {
        return Err(single_diagnostic(
            KaniErrorCode::InvalidUnwind,
            "unwind",
            "unwind must be between 1 and 1024",
        ));
    }
    if !attestation_context_is_valid(&request.attestation) {
        return Err(single_diagnostic(
            KaniErrorCode::InvalidAttestationContext,
            "attestation.context",
            "the Kani attestation binding is invalid",
        ));
    }
    let mut identities = BTreeSet::new();
    for (index, dependency) in request.dependencies.iter().enumerate() {
        let base_path = format!("dependencies[{index}]");
        validate_plain_identity(dependency.proof_id, &format!("{base_path}.proof_id"))?;
        if dependency.proof_id == request.proof_id || !identities.insert(dependency.proof_id) {
            return Err(single_diagnostic(
                KaniErrorCode::InvalidDependency,
                &format!("{base_path}.proof_id"),
                "dependency identities must be unique and distinct from the root proof",
            ));
        }
        match (dependency.kind, dependency.state) {
            (
                ProofDependencyKind::Required,
                ProofDependencyState::Passed
                | ProofDependencyState::Missing
                | ProofDependencyState::Failed,
            ) if dependency.original_path.is_none() && dependency.replacement_path.is_none() => {}
            (ProofDependencyKind::Assumed, ProofDependencyState::Assumed) => {
                let Some(original_path) = dependency.original_path else {
                    return Err(single_diagnostic(
                        KaniErrorCode::InvalidDependency,
                        &base_path,
                        "an assumed dependency requires exactly one predicate path",
                    ));
                };
                if dependency.replacement_path.is_some() {
                    return Err(single_diagnostic(
                        KaniErrorCode::InvalidDependency,
                        &base_path,
                        "an assumed dependency cannot declare a replacement path",
                    ));
                }
                validate_path(original_path, &format!("{base_path}.original_path"))?;
            }
            (ProofDependencyKind::Stubbed, ProofDependencyState::Stubbed) => {
                let (Some(original_path), Some(replacement_path)) =
                    (dependency.original_path, dependency.replacement_path)
                else {
                    return Err(single_diagnostic(
                        KaniErrorCode::InvalidDependency,
                        &base_path,
                        "a stubbed dependency requires original and replacement paths",
                    ));
                };
                validate_path(original_path, &format!("{base_path}.original_path"))?;
                validate_path(replacement_path, &format!("{base_path}.replacement_path"))?;
            }
            _ => {
                return Err(single_diagnostic(
                    KaniErrorCode::InvalidDependency,
                    &base_path,
                    "dependency kind, state, and source paths are inconsistent",
                ));
            }
        }
    }
    Ok(())
}

fn validate_plain_identity(value: &str, path: &str) -> Result<(), Vec<KaniDiagnostic>> {
    if value.is_empty() || value.chars().any(char::is_control) {
        Err(single_diagnostic(
            KaniErrorCode::InvalidIdentity,
            path,
            "identity must be non-empty and contain no control characters",
        ))
    } else {
        Ok(())
    }
}

fn validate_path(value: &str, path: &str) -> Result<(), Vec<KaniDiagnostic>> {
    if syn::parse_str::<syn::Path>(value).is_err() {
        Err(single_diagnostic(
            KaniErrorCode::InvalidIdentity,
            path,
            "value must be a valid Rust path",
        ))
    } else {
        Ok(())
    }
}

fn derive_binding(
    precondition: &[(DependencyIdentity, String)],
    postcondition: &[(DependencyIdentity, String)],
) -> Result<KaniBinding, Vec<KaniDiagnostic>> {
    let mut inputs = BTreeSet::new();
    let mut states = BTreeSet::new();
    for (role, dependencies) in [
        ("precondition", precondition),
        ("postcondition", postcondition),
    ] {
        for (dependency, _) in dependencies {
            let name = dependency.path()[0].as_str();
            match (role, dependency.kind(), dependency.observation()) {
                (_, DependencyKind::Input, None | Some(StateObservation::Current)) => {
                    inputs.insert(name.to_owned());
                }
                ("precondition", DependencyKind::State, Some(StateObservation::Pre))
                | (
                    "postcondition",
                    DependencyKind::State,
                    Some(StateObservation::Pre | StateObservation::Post),
                ) => {
                    states.insert(name.to_owned());
                }
                _ => {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "clauses.dependencies",
                        "the first Kani slice supports current input plus pre/post state only",
                    ));
                }
            }
        }
    }
    if inputs.len() != 1 || states.len() != 1 {
        return Err(single_diagnostic(
            KaniErrorCode::UnsupportedBinding,
            "clauses.dependencies",
            "the first Kani slice requires exactly one Boolean input and one Boolean state",
        ));
    }
    let input_name = inputs.into_iter().next().ok_or_else(|| {
        single_diagnostic(
            KaniErrorCode::UnsupportedBinding,
            "clauses.dependencies",
            "the input binding is unavailable",
        )
    })?;
    let state_name = states.into_iter().next().ok_or_else(|| {
        single_diagnostic(
            KaniErrorCode::UnsupportedBinding,
            "clauses.dependencies",
            "the state binding is unavailable",
        )
    })?;
    Ok(KaniBinding {
        input_name,
        state_name,
    })
}

fn predicate_arguments(
    parameters: &[(DependencyIdentity, String)],
    postcondition: bool,
) -> Result<String, Vec<KaniDiagnostic>> {
    parameters
        .iter()
        .map(
            |(dependency, _)| match (dependency.kind(), dependency.observation()) {
                (DependencyKind::Input, None | Some(StateObservation::Current)) => {
                    Ok("input".to_owned())
                }
                (DependencyKind::State, Some(StateObservation::Pre)) => Ok("pre_state".to_owned()),
                (DependencyKind::State, Some(StateObservation::Post)) if postcondition => {
                    Ok("*post_state".to_owned())
                }
                _ => Err(single_diagnostic(
                    KaniErrorCode::UnsupportedBinding,
                    "clauses.dependencies",
                    "a clause dependency cannot be represented in the first Kani adapter",
                )),
            },
        )
        .collect::<Result<Vec<_>, _>>()
        .map(|values| values.join(", "))
}

fn normalize_dependencies(dependencies: &[ProofDependencyRequest<'_>]) -> Vec<ProofDependencyEdge> {
    let mut result = dependencies
        .iter()
        .map(|dependency| ProofDependencyEdge {
            proof_id: dependency.proof_id.to_owned(),
            kind: dependency.kind,
            state: dependency.state,
            source_site: match dependency.kind {
                ProofDependencyKind::Required => None,
                ProofDependencyKind::Assumed => {
                    Some(dependency_site("assumption", dependency.proof_id))
                }
                ProofDependencyKind::Stubbed => Some(dependency_site("stub", dependency.proof_id)),
            },
        })
        .collect::<Vec<_>>();
    result.sort_by(|left, right| left.proof_id.cmp(&right.proof_id));
    result
}

fn dependency_readiness(dependencies: &[ProofDependencyEdge]) -> ProofReadiness {
    if dependencies.iter().any(|dependency| {
        dependency.kind == ProofDependencyKind::Required
            && matches!(
                dependency.state,
                ProofDependencyState::Missing | ProofDependencyState::Failed
            )
    }) {
        ProofReadiness::Incomplete
    } else if dependencies.iter().any(|dependency| {
        matches!(
            dependency.kind,
            ProofDependencyKind::Assumed | ProofDependencyKind::Stubbed
        )
    }) {
        ProofReadiness::Conditional
    } else {
        ProofReadiness::Ready
    }
}

fn adapter_options(
    harness: &str,
    unwind: u32,
    solver: KaniSolver,
    uses_stubbing: bool,
) -> Vec<String> {
    let mut options = vec!["-Z".to_owned(), "function-contracts".to_owned()];
    if uses_stubbing {
        options.extend(["-Z".to_owned(), "stubbing".to_owned()]);
    }
    options.extend([
        "--harness".to_owned(),
        harness.to_owned(),
        "--exact".to_owned(),
        "--unwind".to_owned(),
        unwind.to_string(),
        "--solver".to_owned(),
        solver.as_str().to_owned(),
    ]);
    options
}

fn map_clause_diagnostics(
    role: &str,
    diagnostics: Vec<crate::GenerationDiagnostic>,
) -> Vec<KaniDiagnostic> {
    diagnostics
        .into_iter()
        .map(|diagnostic| KaniDiagnostic {
            code: KaniErrorCode::ClauseGenerationFailed,
            terminal_state: diagnostic.terminal_state,
            generation_code: Some(diagnostic.code),
            path: format!("{role}.{}", diagnostic.path),
            message: diagnostic.message,
        })
        .collect()
}

fn attestation_diagnostic(code: GenerationErrorCode, path: &str) -> Vec<KaniDiagnostic> {
    let kani_code = match code {
        GenerationErrorCode::InvalidAttestationContext => KaniErrorCode::InvalidAttestationContext,
        _ => KaniErrorCode::SerializationFailed,
    };
    single_diagnostic(
        kani_code,
        path,
        "the shared proof-attestation body could not be emitted",
    )
}

fn single_diagnostic(code: KaniErrorCode, path: &str, message: &str) -> Vec<KaniDiagnostic> {
    vec![KaniDiagnostic {
        code,
        terminal_state: code.terminal_state(),
        generation_code: None,
        path: path.to_owned(),
        message: message.to_owned(),
    }]
}

fn kani_symbol(requirement: &str, revision: u64, proof_id: &str) -> String {
    let readable_requirement = readable_component(requirement);
    let readable_proof = readable_component(proof_id);
    let revision_text = revision.to_string();
    let identity = length_delimited_identity(&[requirement, &revision_text, proof_id]);
    // Kani synthesizes contract symbols from these names. Keep the Rust symbol bounded so those
    // derived object-file names remain below common filesystem component limits; the complete
    // identity and full artifact digests remain in framing, graph, and attestation records.
    let digest_prefix = sha256(identity.as_bytes())
        .chars()
        .take(32)
        .collect::<String>();
    format!(
        "kani_{readable_requirement}_{revision}_{readable_proof}_id_{}",
        digest_prefix
    )
}

fn readable_component(value: &str) -> String {
    let mut result = String::with_capacity(value.len().min(12));
    for byte in value.bytes().take(12) {
        if byte.is_ascii_alphanumeric() {
            result.push(char::from(byte.to_ascii_lowercase()));
        } else {
            result.push('_');
        }
    }
    if result.is_empty() || result.as_bytes()[0].is_ascii_digit() {
        result.insert(0, '_');
    }
    result
}

fn dependency_site(kind: &str, proof_id: &str) -> String {
    format!("{kind}:{}", sha256(proof_id.as_bytes()))
}

fn deterministic_json(value: &impl Serialize) -> Result<String, String> {
    let mut bytes = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    String::from_utf8(bytes).map_err(|error| error.to_string())
}

fn artifact(path: String, contents: String) -> Artifact {
    Artifact {
        sha256: sha256(contents.as_bytes()),
        path,
        contents,
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn sha256(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        let _ = write!(result, "{byte:02x}");
    }
    result
}

fn kani_implementation_digest() -> &'static str {
    static VALUE: OnceLock<String> = OnceLock::new();
    VALUE.get_or_init(|| {
        let mut hasher = Sha256::new();
        for value in [
            KANI_SOURCE,
            ORACLE_SOURCE,
            BUILD_SOURCE,
            LOCKFILE,
            PROOF_GRAPH_SCHEMA,
            RUST_KANI_SCHEMA,
            KANI_SPEC,
        ] {
            hasher.update(value.len().to_le_bytes());
            hasher.update(value);
        }
        let mut result = String::with_capacity(64);
        for byte in hasher.finalize() {
            let _ = write!(result, "{byte:02x}");
        }
        result
    })
}

fn proof_graph_schema_digest() -> &'static str {
    static VALUE: OnceLock<String> = OnceLock::new();
    VALUE.get_or_init(|| sha256(PROOF_GRAPH_SCHEMA))
}

fn rust_kani_schema_digest() -> &'static str {
    static VALUE: OnceLock<String> = OnceLock::new();
    VALUE.get_or_init(|| sha256(RUST_KANI_SCHEMA))
}

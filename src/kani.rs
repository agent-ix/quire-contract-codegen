//! Deterministic Kani proof lowering.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
};

use quire_contract_ir::{
    ClauseId, DependencyIdentity, DependencyKind, IntegerDomain, OverflowPolicy, RequirementRef,
    SourceSpan, StateObservation, TypedExpression,
};
use serde::{Deserialize, Serialize};

use crate::{
    oracle::{
        bounded_readable_component, generate_named_boolean_oracle, oracle_symbol,
        typed_dependency_parameters, unique_pair, DependencyParameter, RustValueType,
    },
    Artifact, GenerationErrorCode, GenerationTerminalState, OracleRequest,
    MAX_GENERATED_SOURCE_BYTES, MAX_OBLIGATION_UNWIND,
};

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

/// Position of one primitive dependency in the generated subject ABI.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniBindingRole {
    /// A copied current/pre value passed to the customer subject.
    Argument,
    /// A copied post-state value returned by the customer subject.
    Result,
}

/// Rust primitive used for one generated Kani subject binding.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniPrimitiveType {
    /// Rust `bool`.
    Boolean,
    /// Rust `i64`.
    I64,
}

impl KaniPrimitiveType {
    pub(crate) const fn source_name(self) -> &'static str {
        match self {
            Self::Boolean => "bool",
            Self::I64 => "i64",
        }
    }
}

/// Exact checked IR domain retained for one bounded-integer binding.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct KaniIntegerBounds {
    /// Signed or unsigned checked IR domain.
    pub domain: IntegerDomain,
    /// Inclusive checked minimum.
    pub minimum: i64,
    /// Inclusive checked maximum.
    pub maximum: i64,
    /// Checked overflow policy; generation never replaces it.
    pub overflow: OverflowPolicy,
}

/// One normalized primitive argument or result in the generated subject ABI.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct KaniSubjectBinding {
    /// Complete checked dependency identity used for uniqueness and ordering.
    pub dependency: DependencyIdentity,
    /// Deterministic generated Rust identifier.
    pub identifier: String,
    /// Argument or result position.
    pub role: KaniBindingRole,
    /// Rust primitive type.
    pub primitive_type: KaniPrimitiveType,
    /// Exact integer bounds, or `None` for Boolean bindings.
    pub integer_bounds: Option<KaniIntegerBounds>,
    /// Every authored occurrence contributing this normalized binding.
    pub source_spans: Vec<SourceSpan>,
}

/// Supported solver choice for the Kani adapter.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniSolver {
    /// Kani's CaDiCaL SAT solver.
    Cadical,
}

impl KaniSolver {
    pub(crate) const fn as_str(self) -> &'static str {
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
    /// Explicit bounded loop unwind value.
    pub unwind: u32,
    /// Explicit solver choice.
    pub solver: KaniSolver,
    /// Complete caller-owned dependency census.
    pub dependencies: &'a [ProofDependencyRequest<'a>],
}

/// Stable reason a Kani bundle could not be generated.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniErrorCode {
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
    /// Generated Rust did not parse.
    InvalidGeneratedSyntax,
    /// A deterministic graph could not be serialized.
    SerializationFailed,
    /// The generated source exceeds the bounded artifact size.
    ResourceLimitExceeded,
}

impl KaniErrorCode {
    /// Maps a Kani diagnostic category to interface-001 terminal state.
    #[must_use]
    pub const fn terminal_state(self) -> GenerationTerminalState {
        match self {
            Self::InvalidIdentity | Self::InvalidDependency | Self::InvalidUnwind => {
                GenerationTerminalState::InvalidInput
            }
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
    /// Exact IR-owned locus when clause lowering identified one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span: Option<SourceSpan>,
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
    /// Complete Kani option vector.
    pub options: Vec<String>,
    /// Derived dependency readiness; this is not a proof execution result.
    pub readiness: ProofReadiness,
    /// Explicit proof execution state; generation never executes or classifies a proof.
    pub proof_execution_state: String,
    /// Ordered copied current/pre values supplied to the customer subject.
    pub subject_arguments: Vec<KaniSubjectBinding>,
    /// Ordered post-state values returned by the customer subject.
    pub subject_results: Vec<KaniSubjectBinding>,
    /// Generated Rust artifact path.
    pub source_artifact_path: String,
    /// Sorted complete dependency census.
    pub dependencies: Vec<ProofDependencyEdge>,
}

/// All-or-nothing Kani source and proof graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniArtifactBundle {
    /// Generated Rust contract and proof source.
    pub rust: Artifact,
    /// Deterministic proof-dependency graph.
    pub proof_graph: Artifact,
}

struct SubjectAbi {
    arguments: Vec<KaniSubjectBinding>,
    results: Vec<KaniSubjectBinding>,
}

struct KaniSource<'a> {
    requirement: &'a str,
    revision: u64,
    proof_id: &'a str,
    subject_path: &'a str,
    module_symbol: &'a str,
    contract_symbol: &'a str,
    harness_symbol: &'a str,
    precondition_symbol: &'a str,
    postcondition_symbol: &'a str,
    precondition_arguments: &'a str,
    postcondition_arguments: &'a str,
    precondition_source: &'a str,
    postcondition_source: &'a str,
    stub_attributes: &'a str,
    dependency_assumptions: &'a str,
    abi: &'a SubjectAbi,
}

/// Generates one bounded Kani contract/proof bundle or structured diagnostics with no partial output.
// Implements: FR-015
pub fn generate_kani_bundle(
    request: &KaniRequest<'_>,
) -> Result<KaniArtifactBundle, Vec<KaniDiagnostic>> {
    validate_request(request)?;
    let precondition_request = OracleRequest {
        requirement: request.requirement,
        clause: request.precondition_clause,
        expression: request.precondition,
    };
    let postcondition_request = OracleRequest {
        requirement: request.requirement,
        clause: request.postcondition_clause,
        expression: request.postcondition,
    };
    let requirement = request.requirement.requirement().as_str();
    let revision = request.requirement.revision().get();
    let (precondition_clause, postcondition_clause) = (
        request.precondition_clause.as_str(),
        request.postcondition_clause.as_str(),
    );
    let (precondition_symbol, postcondition_symbol) = unique_pair(
        (
            oracle_symbol(requirement, revision, precondition_clause),
            precondition_clause,
        ),
        (
            oracle_symbol(requirement, revision, postcondition_clause),
            postcondition_clause,
        ),
    );
    let precondition = generate_named_boolean_oracle(&precondition_request, &precondition_symbol)
        .map_err(|values| map_clause_diagnostics("precondition", values))?;
    let postcondition =
        generate_named_boolean_oracle(&postcondition_request, &postcondition_symbol)
            .map_err(|values| map_clause_diagnostics("postcondition", values))?;
    let precondition_parameters = typed_dependency_parameters(&precondition_request)
        .map_err(|values| map_clause_diagnostics("precondition", values))?;
    let postcondition_parameters = typed_dependency_parameters(&postcondition_request)
        .map_err(|values| map_clause_diagnostics("postcondition", values))?;
    let abi = derive_subject_abi(&precondition_parameters, &postcondition_parameters)?;
    let symbol = kani_symbol(requirement, revision, request.proof_id);
    let contract_symbol = format!("{symbol}_contract");
    let harness_symbol = format!("{symbol}_proof");
    let module_symbol = format!("{symbol}_module");
    let precondition_arguments = predicate_arguments(&precondition_parameters, &abi, false)?;
    let postcondition_arguments = predicate_arguments(&postcondition_parameters, &abi, true)?;
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
    let dependency_assumptions = source_dependencies
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
    let source = render_kani_source(&KaniSource {
        requirement,
        revision,
        proof_id: request.proof_id,
        subject_path: request.subject_path,
        module_symbol: &module_symbol,
        contract_symbol: &contract_symbol,
        harness_symbol: &harness_symbol,
        precondition_symbol: &precondition_symbol,
        postcondition_symbol: &postcondition_symbol,
        precondition_arguments: &precondition_arguments,
        postcondition_arguments: &postcondition_arguments,
        precondition_source: &precondition.rust.contents,
        postcondition_source: &postcondition.rust.contents,
        stub_attributes: &stub_attributes,
        dependency_assumptions: &dependency_assumptions,
        abi: &abi,
    });
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
        schema_version: "quire.kani-proof-graph/v2".to_owned(),
        proof_id: request.proof_id.to_owned(),
        requirement_id: requirement.to_owned(),
        requirement_revision: revision,
        options,
        readiness: dependency_readiness(&normalized_dependencies),
        proof_execution_state: "not_run".to_owned(),
        subject_arguments: abi.arguments.clone(),
        subject_results: abi.results.clone(),
        source_artifact_path: rust.path.clone(),
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
    Ok(KaniArtifactBundle { rust, proof_graph })
}

fn validate_request(request: &KaniRequest<'_>) -> Result<(), Vec<KaniDiagnostic>> {
    if request.precondition_clause == request.postcondition_clause {
        return Err(single_diagnostic(
            KaniErrorCode::InvalidIdentity,
            "clauses",
            "precondition and postcondition clause identities must be distinct",
        ));
    }
    validate_plain_identity(request.proof_id, "proof_id")?;
    validate_path(request.subject_path, "subject_path")?;
    if request.unwind == 0 || request.unwind > MAX_OBLIGATION_UNWIND {
        return Err(single_diagnostic(
            KaniErrorCode::InvalidUnwind,
            "unwind",
            &format!("unwind must be between 1 and {MAX_OBLIGATION_UNWIND}"),
        ));
    }
    validate_dependencies(request.dependencies, request.proof_id)?;
    Ok(())
}

/// Validates one caller-declared proof-dependency census: every declared identity is non-empty,
/// unique within the census, and distinct from `proof_id` (the root proof this census is declared
/// against), and its kind/state/path combination is one of the three closed shapes (`Required`,
/// `Assumed`, `Stubbed`).
///
/// Shared by [`generate_kani_bundle`]'s request validation and the bounded-Kani corpus's
/// declared-census validation, so there is exactly one definition of what a valid
/// proof-dependency census looks like rather than two that can drift apart.
pub(crate) fn validate_dependencies(
    dependencies: &[ProofDependencyRequest<'_>],
    proof_id: &str,
) -> Result<(), Vec<KaniDiagnostic>> {
    let mut identities = BTreeSet::new();
    for (index, dependency) in dependencies.iter().enumerate() {
        let base_path = format!("dependencies[{index}]");
        validate_plain_identity(dependency.proof_id, &format!("{base_path}.proof_id"))?;
        if dependency.proof_id == proof_id || !identities.insert(dependency.proof_id) {
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

fn derive_subject_abi(
    precondition: &[DependencyParameter],
    postcondition: &[DependencyParameter],
) -> Result<SubjectAbi, Vec<KaniDiagnostic>> {
    let mut arguments: BTreeMap<DependencyIdentity, KaniSubjectBinding> = BTreeMap::new();
    let mut results: BTreeMap<DependencyIdentity, KaniSubjectBinding> = BTreeMap::new();
    let mut logical_types: BTreeMap<(DependencyKind, Vec<String>), RustValueType> = BTreeMap::new();
    let mut generated_names: BTreeMap<String, DependencyIdentity> = BTreeMap::new();
    for (is_postcondition, parameters) in [(false, precondition), (true, postcondition)] {
        for parameter in parameters {
            let dependency = &parameter.dependency;
            let role = match (dependency.kind(), dependency.observation()) {
                (DependencyKind::Input, None | Some(StateObservation::Current))
                | (
                    DependencyKind::State,
                    Some(StateObservation::Current | StateObservation::Pre),
                ) => KaniBindingRole::Argument,
                (DependencyKind::State, Some(StateObservation::Post)) if is_postcondition => {
                    KaniBindingRole::Result
                }
                (DependencyKind::State, Some(StateObservation::Post)) => {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "precondition.dependencies",
                        "post-state data cannot be bound in a Kani precondition",
                    ));
                }
                _ => {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "clauses.dependencies",
                        "the Kani adapter supports only direct current input/current state/pre-state arguments and post-state results",
                    ));
                }
            };
            let logical_key = (
                dependency.kind(),
                dependency
                    .path()
                    .iter()
                    .map(|part| part.as_str().to_owned())
                    .collect::<Vec<_>>(),
            );
            if let Some(existing) = logical_types.get(&logical_key) {
                if existing != &parameter.value_type {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "clauses.dependencies",
                        "one logical dependency has conflicting primitive types or integer domains",
                    ));
                }
            } else {
                logical_types.insert(logical_key, parameter.value_type.clone());
            }
            if let Some(existing) = generated_names.get(&parameter.identifier) {
                if existing != dependency {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "clauses.dependencies",
                        "distinct dependency identities collide in the generated subject ABI",
                    ));
                }
            } else {
                generated_names.insert(parameter.identifier.clone(), dependency.clone());
            }
            let bindings = match role {
                KaniBindingRole::Argument => &mut arguments,
                KaniBindingRole::Result => &mut results,
            };
            if let Some(existing) = bindings.get_mut(dependency) {
                if existing.identifier != parameter.identifier
                    || existing.role != role
                    || !binding_matches_value_type(existing, &parameter.value_type)
                {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "clauses.dependencies",
                        "one dependency identity has incompatible generated bindings",
                    ));
                }
                existing.source_spans.push(parameter.source.clone());
                existing.source_spans.sort();
                existing.source_spans.dedup();
            } else {
                bindings.insert(dependency.clone(), subject_binding(parameter, role));
            }
        }
    }
    Ok(SubjectAbi {
        arguments: arguments.into_values().collect(),
        results: results.into_values().collect(),
    })
}

fn subject_binding(parameter: &DependencyParameter, role: KaniBindingRole) -> KaniSubjectBinding {
    let (primitive_type, integer_bounds) = match &parameter.value_type {
        RustValueType::Boolean => (KaniPrimitiveType::Boolean, None),
        RustValueType::Integer(value) => (
            KaniPrimitiveType::I64,
            Some(KaniIntegerBounds {
                domain: value.domain(),
                minimum: value.minimum(),
                maximum: value.maximum(),
                overflow: value.overflow(),
            }),
        ),
    };
    KaniSubjectBinding {
        dependency: parameter.dependency.clone(),
        identifier: parameter.identifier.clone(),
        role,
        primitive_type,
        integer_bounds,
        source_spans: vec![parameter.source.clone()],
    }
}

fn binding_matches_value_type(binding: &KaniSubjectBinding, value_type: &RustValueType) -> bool {
    match (value_type, binding.primitive_type, &binding.integer_bounds) {
        (RustValueType::Boolean, KaniPrimitiveType::Boolean, None) => true,
        (RustValueType::Integer(value), KaniPrimitiveType::I64, Some(bounds)) => {
            bounds.domain == value.domain()
                && bounds.minimum == value.minimum()
                && bounds.maximum == value.maximum()
                && bounds.overflow == value.overflow()
        }
        _ => false,
    }
}

fn predicate_arguments(
    parameters: &[DependencyParameter],
    abi: &SubjectAbi,
    postcondition: bool,
) -> Result<String, Vec<KaniDiagnostic>> {
    parameters
        .iter()
        .map(|parameter| {
            if let Some(binding) = abi
                .arguments
                .iter()
                .find(|binding| binding.dependency == parameter.dependency)
            {
                return Ok(binding.identifier.clone());
            }
            if postcondition {
                if let Some(index) = abi
                    .results
                    .iter()
                    .position(|binding| binding.dependency == parameter.dependency)
                {
                    return Ok(result_access(abi.results.len(), index));
                }
            }
            Err(single_diagnostic(
                KaniErrorCode::UnsupportedBinding,
                "clauses.dependencies",
                "a clause dependency has no position in the normalized subject ABI",
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|values| values.join(", "))
}

fn result_access(result_count: usize, index: usize) -> String {
    if result_count == 1 {
        "*post_state".to_owned()
    } else {
        format!("post_state.{index}")
    }
}

fn render_kani_source(value: &KaniSource<'_>) -> String {
    let argument_declarations = value
        .abi
        .arguments
        .iter()
        .map(|binding| {
            format!(
                "{}: {}",
                binding.identifier,
                binding.primitive_type.source_name()
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let argument_names = value
        .abi
        .arguments
        .iter()
        .map(|binding| binding.identifier.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let result_type = result_type(&value.abi.results);
    let framing = render_framing(value.abi);
    let symbolic_arguments = render_symbolic_arguments(&value.abi.arguments);
    let result_bounds = render_result_bounds(&value.abi.results);
    let postcondition_call = format!(
        "{}({})",
        value.postcondition_symbol, value.postcondition_arguments
    );
    let ensures = if result_bounds.is_empty() {
        postcondition_call
    } else {
        format!("({result_bounds}) && ({postcondition_call})")
    };
    let post_state_name = if value.abi.results.is_empty() {
        "_post_state"
    } else {
        "post_state"
    };
    format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Requirement: {}@{}; Proof: {}\n\
\n\
{}\n\
{}\n\
#[cfg(kani)]\n\
mod {} {{\n\
    use super::*;\n\
\n\
    // BEGIN framing\n\
    // proof-id: {}\n\
{}    // END framing\n\
\n\
    // BEGIN binding\n\
    fn call_subject({argument_declarations}) -> {result_type} {{\n\
        {}({argument_names})\n\
    }}\n\
    // END binding\n\
\n\
    // BEGIN contract\n\
    #[kani::requires({}({}))]\n\
    #[kani::ensures(|{post_state_name}: &{result_type}| {ensures})]\n\
    fn {}({argument_declarations}) -> {result_type} {{\n\
        call_subject({argument_names})\n\
    }}\n\
    // END contract\n\
\n\
    // BEGIN proof harness\n\
{}    #[kani::proof_for_contract({})]\n\
    fn {}() {{\n\
{}{}        let _post_state = {}({argument_names});\n\
    }}\n\
    // END proof harness\n\
}}\n",
        value.requirement,
        value.revision,
        value.proof_id,
        value.precondition_source,
        value.postcondition_source,
        value.module_symbol,
        value.proof_id,
        framing,
        value.subject_path,
        value.precondition_symbol,
        value.precondition_arguments,
        value.contract_symbol,
        value.stub_attributes,
        value.contract_symbol,
        value.harness_symbol,
        value.dependency_assumptions,
        symbolic_arguments,
        value.contract_symbol,
    )
}

fn render_framing(abi: &SubjectAbi) -> String {
    abi.arguments
        .iter()
        .chain(&abi.results)
        .map(|binding| {
            let role = match binding.role {
                KaniBindingRole::Argument => "argument",
                KaniBindingRole::Result => "result",
            };
            format!(
                "    // {role}-binding: {}:{}\n",
                binding.identifier,
                binding.primitive_type.source_name()
            )
        })
        .collect()
}

fn result_type(results: &[KaniSubjectBinding]) -> String {
    match results {
        [] => "()".to_owned(),
        [binding] => binding.primitive_type.source_name().to_owned(),
        values => format!(
            "({})",
            values
                .iter()
                .map(|binding| binding.primitive_type.source_name())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn render_symbolic_arguments(arguments: &[KaniSubjectBinding]) -> String {
    let mut source = String::new();
    for binding in arguments {
        let _ = writeln!(
            source,
            "        let {}: {} = kani::any();",
            binding.identifier,
            binding.primitive_type.source_name()
        );
        if let Some(bounds) = &binding.integer_bounds {
            let _ = writeln!(
                source,
                "        kani::assume({} >= {} && {} <= {});",
                binding.identifier,
                i64_literal(bounds.minimum),
                binding.identifier,
                i64_literal(bounds.maximum)
            );
        }
    }
    source
}

fn render_result_bounds(results: &[KaniSubjectBinding]) -> String {
    results
        .iter()
        .enumerate()
        .filter_map(|(index, binding)| {
            binding.integer_bounds.as_ref().map(|bounds| {
                let access = result_access(results.len(), index);
                format!(
                    "{access} >= {} && {access} <= {}",
                    i64_literal(bounds.minimum),
                    i64_literal(bounds.maximum)
                )
            })
        })
        .collect::<Vec<_>>()
        .join(" && ")
}

pub(crate) fn i64_literal(value: i64) -> String {
    match value {
        i64::MIN => "i64::MIN".to_owned(),
        i64::MAX => "i64::MAX".to_owned(),
        _ => format!("{value}_i64"),
    }
}

pub(crate) fn normalize_dependencies(
    dependencies: &[ProofDependencyRequest<'_>],
) -> Vec<ProofDependencyEdge> {
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

pub(crate) fn dependency_readiness(dependencies: &[ProofDependencyEdge]) -> ProofReadiness {
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

pub(crate) fn adapter_options(
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
        "-Z".to_owned(),
        "concrete-playback".to_owned(),
        "--harness".to_owned(),
        harness.to_owned(),
        "--exact".to_owned(),
        "--unwind".to_owned(),
        unwind.to_string(),
        "--solver".to_owned(),
        solver.as_str().to_owned(),
        "--output-format".to_owned(),
        "regular".to_owned(),
        "--concrete-playback".to_owned(),
        "print".to_owned(),
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
            source_span: diagnostic.source_span,
            message: diagnostic.message,
        })
        .collect()
}

fn single_diagnostic(code: KaniErrorCode, path: &str, message: &str) -> Vec<KaniDiagnostic> {
    vec![KaniDiagnostic {
        code,
        terminal_state: code.terminal_state(),
        generation_code: None,
        path: path.to_owned(),
        source_span: None,
        message: message.to_owned(),
    }]
}

/// The generated module, contract and proof name stem, read from the requirement, revision and
/// proof id. Kani synthesizes contract symbols and object-file names from these names, so each
/// readable component is bounded; the complete identity remains in framing and the graph.
fn kani_symbol(requirement: &str, revision: u64, proof_id: &str) -> String {
    format!(
        "kani_{}_{revision}_{}",
        bounded_readable_component(requirement),
        bounded_readable_component(proof_id)
    )
}

/// `value` as a readable snake-case name component of at most 12 characters.
pub(crate) fn readable_component(value: &str) -> String {
    crate::oracle::readable_name_component(value, 12)
}

fn dependency_site(kind: &str, proof_id: &str) -> String {
    format!("{kind}:{proof_id}")
}

// `?Sized` so an unsized `[T]` slice (e.g. `&[ProofDependencyEdge]`) can be passed directly, with
// no intermediate owned `Vec` allocation at the call site, alongside every already-`Sized` caller
// (ir#80 review finding F10).
pub(crate) fn deterministic_json(value: &(impl Serialize + ?Sized)) -> Result<String, String> {
    let mut bytes = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    String::from_utf8(bytes).map_err(|error| error.to_string())
}

fn artifact(path: String, contents: String) -> Artifact {
    Artifact::new(path, contents)
}

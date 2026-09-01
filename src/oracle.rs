//! Deterministic, fail-closed lowering for the first Boolean-oracle slice.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    sync::OnceLock,
};

use quire_contract_ir::{
    BooleanOperator, CanonicalProfile, ClauseId, DependencyIdentity, DependencyKind, Expression,
    ExpressionKind, RequirementRef, StateObservation, TypedExpression, ValueType,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

/// Exact accepted IR PR #19 merge consumed by this implementation.
pub const IR_CANDIDATE_REVISION: &str = "5c49ebfd1c87415f74420ad047392bd03b1bd202";

/// Exact merged runtime revision required by generated source.
pub const RUNTIME_REVISION: &str = "e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3";

/// Exact codegen Git revision captured by the build.
pub const GENERATOR_SOURCE_REVISION: &str = env!("QUIRE_CODEGEN_SOURCE_REVISION");

/// Reports whether generator build inputs differed from `GENERATOR_SOURCE_REVISION`.
#[must_use]
pub fn generator_source_is_dirty() -> bool {
    env!("QUIRE_CODEGEN_SOURCE_DIRTY") == "true"
}

/// Maximum generated Rust bytes for one clause.
pub const MAX_GENERATED_SOURCE_BYTES: usize = 1_048_576;

const PGM_SCHEMA: &[u8] =
    include_bytes!("../schemas/pgm01-derivation-evidence-envelope-v1.schema.json");
const SOURCE_MAP_SCHEMA: &[u8] = include_bytes!("../schemas/oracle-source-map-v1.schema.json");
const RUST_ORACLE_SCHEMA: &[u8] = include_bytes!("../schemas/generated-rust-oracle-v1.schema.json");
const ORACLE_SPEC: &[u8] = include_bytes!("../spec/functional/FR-001-deterministic-oracles.md");
const GENERATOR_SOURCE: &[u8] = include_bytes!("oracle.rs");
const BUILD_SOURCE: &[u8] = include_bytes!("../build.rs");
const LOCKFILE: &[u8] = include_bytes!("../Cargo.lock");

/// One validated clause supplied to the Boolean lowering core.
///
/// This explicit boundary is required because accepted IR PR #19 does not bind typed expressions
/// directly into `ContractPackage` clauses.
pub struct OracleRequest<'a> {
    /// Requirement identity and revision.
    pub requirement: &'a RequirementRef,
    /// Stable clause identity within the requirement revision.
    pub clause: &'a ClauseId,
    /// Validated typed expression for the clause root.
    pub expression: &'a TypedExpression,
    /// Caller-owned provenance and bounded result claim for the emitted manifest.
    pub manifest: ManifestContext<'a>,
}

/// Caller-owned PGM-01 provenance and result fields for one generated bundle.
#[derive(Clone, Copy, Debug)]
pub struct ManifestContext<'a> {
    /// Candidate revision against which the generated artifact is evaluated.
    pub candidate_revision: &'a str,
    /// PGM-01 contribution method (`human`, `agent-assisted`, `generated`, or `mixed`).
    pub contribution_method: &'a str,
    /// Accountable reviewers; these are evidence identities, not implicit approvals.
    pub reviewers: &'a [&'a str],
    /// PGM-01 result status chosen by the consuming assurance process.
    pub result_status: &'a str,
    /// Bounded result summary chosen by the consuming assurance process.
    pub result_summary: &'a str,
    /// Requirements supported by this result, supplied by the consuming package.
    pub requirement_refs: &'a [&'a str],
}

/// Interface-001 terminal state for a generation result.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GenerationTerminalState {
    /// A complete supported artifact was generated.
    Generated,
    /// The input uses semantics outside the bounded generator slice.
    Unsupported,
    /// The input is invalid for the requested generation operation.
    InvalidInput,
    /// The configured backend is unavailable.
    BackendUnavailable,
    /// Atomic publication failed.
    IoFailed,
    /// An internal generation control could not reach a conclusion.
    Inconclusive,
}

/// Stable machine-readable generation failure category.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationErrorCode {
    /// The clause root is not Boolean.
    NonBooleanRoot,
    /// The first slice cannot lower an expression without approximation.
    UnsupportedExpression,
    /// A dependency cannot be represented in the generated signature.
    UnsupportedDependency,
    /// The typed expression carries definedness obligations this slice cannot preserve.
    UnsupportedObligations,
    /// Two input identities would claim the same generated name.
    NameCollision,
    /// Caller-supplied evidence provenance or result context is invalid.
    InvalidManifestContext,
    /// The bounded output resource would be exceeded.
    ResourceLimitExceeded,
    /// Generated tokens did not parse as a Rust source file.
    InvalidGeneratedSyntax,
    /// A deterministic manifest or source-map value could not be encoded.
    SerializationFailed,
}

impl GenerationErrorCode {
    /// Maps the diagnostic category to its interface-001 terminal state.
    #[must_use]
    pub const fn terminal_state(self) -> GenerationTerminalState {
        match self {
            Self::NonBooleanRoot | Self::NameCollision | Self::InvalidManifestContext => {
                GenerationTerminalState::InvalidInput
            }
            Self::UnsupportedExpression
            | Self::UnsupportedDependency
            | Self::UnsupportedObligations
            | Self::ResourceLimitExceeded => GenerationTerminalState::Unsupported,
            Self::InvalidGeneratedSyntax | Self::SerializationFailed => {
                GenerationTerminalState::Inconclusive
            }
        }
    }
}

/// Structured diagnostic returned without a partial artifact bundle.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GenerationDiagnostic {
    /// Stable diagnostic category.
    pub code: GenerationErrorCode,
    /// Interface-001 terminal state implied by `code`.
    pub terminal_state: GenerationTerminalState,
    /// Requirement identity associated with the failure.
    pub requirement_id: String,
    /// Exact requirement revision associated with the failure.
    pub requirement_revision: u64,
    /// Clause identity associated with the failure.
    pub clause_id: String,
    /// Stable path to the rejected input element.
    pub path: String,
    /// Human-readable detail that is not used as machine identity.
    pub message: String,
}

/// One generated file with its content digest.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Artifact {
    /// Deterministic bundle-relative path.
    pub path: String,
    /// UTF-8 artifact contents.
    pub contents: String,
    /// Lowercase SHA-256 of `contents`.
    pub sha256: String,
}

/// Trace from a generated source range back to one requirement clause.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SourceRegion {
    /// Generated source path.
    pub artifact_path: String,
    /// Semantic role, such as `clause` or `implication_consequent`.
    pub role: String,
    /// One-based inclusive starting line.
    pub start_line: u32,
    /// One-based inclusive ending line.
    pub end_line: u32,
    /// Requirement identity.
    pub requirement_id: String,
    /// Exact requirement revision.
    pub requirement_revision: u64,
    /// Clause identity.
    pub clause_id: String,
}

/// SHA-256 identity used by the PGM-01 envelope.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DigestIdentity {
    /// Digest algorithm.
    pub algorithm: String,
    /// Lowercase hexadecimal digest value.
    pub value: String,
}

/// Versioned schema identity used by a manifest artifact.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaIdentity {
    /// Stable schema name.
    pub id: String,
    /// Schema version.
    pub version: String,
    /// Schema content identity.
    pub digest: DigestIdentity,
}

/// PGM-01 producer identity for the in-process generator.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProducerIdentity {
    /// Producer name.
    pub name: String,
    /// Crate version.
    pub version: String,
    /// Exact Git revision used to build the generator.
    pub source_revision: String,
    /// Digest over the lowering implementation and dependency lock.
    pub executable_digest: DigestIdentity,
    /// Stable in-process invocation description.
    pub invocation: Vec<String>,
}

/// PGM-01 input or output artifact identity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ManifestArtifact {
    /// Semantic artifact role.
    pub role: String,
    /// Artifact URI or bundle-relative path.
    pub uri: String,
    /// Artifact media type.
    pub media_type: String,
    /// Artifact schema identity.
    pub schema: SchemaIdentity,
    /// Artifact content identity.
    pub content_digest: DigestIdentity,
}

/// Explicit identity for in-process lowering without an external backend.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NoBackend {
    /// PGM-01 no-backend discriminator.
    pub kind: String,
    /// Reason no external backend participates.
    pub reason: String,
}

/// Deterministic build environment identity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GenerationEnvironment {
    /// Compilation target triple.
    pub target_triple: String,
    /// Compilation target operating system.
    pub operating_system: String,
    /// Rust compiler identity.
    pub toolchain: String,
    /// Exact Cargo lockfile digest.
    pub dependencies_digest: DigestIdentity,
}

/// Source provenance for the generator invocation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GenerationProvenance {
    /// Generator repository.
    pub repository: String,
    /// Exact producer revision.
    pub source_revision: String,
    /// Candidate revision reviewed by this draft.
    pub candidate_revision: String,
    /// Contribution method.
    pub contribution_method: String,
    /// Reviewer of record; this is not an approval claim.
    pub reviewers: Vec<String>,
}

/// PGM-01 result for one successful derivation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GenerationResult {
    /// Evidence result state.
    pub status: String,
    /// Bounded claim made by this manifest.
    pub summary: String,
    /// Requirements supported by the result.
    pub requirement_refs: Vec<String>,
}

/// Codegen-specific extension carried within the PGM-01 envelope.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CodegenExtension {
    /// Interface-001 generation state.
    pub terminal_state: GenerationTerminalState,
    /// Whether build inputs differed from the recorded Git revision.
    pub generator_source_dirty: bool,
    /// Whether an exact generator revision was available from Git or archive metadata.
    pub generator_source_revision_available: bool,
    /// Canonical IR expression profile.
    pub canonical_profile: String,
    /// Canonical semantic expression digest.
    pub expression_canonical_digest: String,
    /// Exact IR revision.
    pub ir_revision: String,
    /// Exact runtime revision.
    pub runtime_revision: String,
    /// Stable clause identity.
    pub clause_id: String,
    /// Reviewer field semantics.
    pub reviewer_role: String,
    /// Bounded source-size policy.
    pub maximum_source_bytes: usize,
}

/// Complete PGM-01 derivation identity for emitted source and source map.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DerivationManifest {
    /// PGM-01 schema identity.
    pub schema_version: String,
    /// Stable record identity.
    pub record_id: String,
    /// Deterministic source-commit timestamp.
    pub recorded_at: String,
    /// Generator implementation identity.
    pub producer: ProducerIdentity,
    /// Canonical semantic inputs.
    pub inputs: Vec<ManifestArtifact>,
    /// Explicit in-process backend identity.
    pub backend: NoBackend,
    /// Generated output identities.
    pub outputs: Vec<ManifestArtifact>,
    /// Generation-configuration identity.
    pub parameters_digest: DigestIdentity,
    /// Build environment identity.
    pub environment: GenerationEnvironment,
    /// Generator source provenance.
    pub provenance: GenerationProvenance,
    /// Bounded derivation result.
    pub result: GenerationResult,
    /// Namespaced codegen details.
    pub extensions: BTreeMap<String, CodegenExtension>,
}

/// Complete all-or-nothing result for one supported oracle clause.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleArtifactBundle {
    /// Generated Rust source.
    pub rust: Artifact,
    /// Machine-readable source-region map.
    pub source_map: Artifact,
    /// Machine-readable derivation manifest.
    pub manifest: Artifact,
}

struct RenderedExpression {
    source: String,
    implication_regions: Vec<(u32, u32)>,
}

enum SerializationError {
    Json(serde_json::Error),
    Utf8(std::string::FromUtf8Error),
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::Utf8(error) => write!(formatter, "{error}"),
        }
    }
}

struct SourceBuilder {
    source: String,
    next_line: u32,
    implication_regions: Vec<(u32, u32)>,
}

impl SourceBuilder {
    fn new() -> Self {
        Self {
            source: String::new(),
            next_line: 1,
            implication_regions: Vec::new(),
        }
    }

    fn line(&mut self, value: &str) -> Result<(), GenerationErrorCode> {
        if self
            .source
            .len()
            .saturating_add(value.len())
            .saturating_add(1)
            > MAX_GENERATED_SOURCE_BYTES
        {
            return Err(GenerationErrorCode::ResourceLimitExceeded);
        }
        self.source.push_str(value);
        self.source.push('\n');
        self.next_line = self.next_line.saturating_add(1);
        Ok(())
    }
}

/// Generates one deterministic Boolean oracle or diagnostics with no partial bundle.
///
/// Trace: TC-001, TC-003
// Implements: FR-001
pub fn generate_boolean_oracle(
    request: &OracleRequest<'_>,
) -> Result<OracleArtifactBundle, Vec<GenerationDiagnostic>> {
    if request.expression.nodes().len() < 128 {
        return generate_boolean_oracle_inner(request);
    }
    std::thread::scope(|scope| {
        let handle = std::thread::Builder::new()
            .name("contract-oracle-generation".to_owned())
            .stack_size(16 * 1024 * 1024)
            .spawn_scoped(scope, || generate_boolean_oracle_inner(request))
            .map_err(|error| {
                single_diagnostic(
                    request,
                    GenerationErrorCode::ResourceLimitExceeded,
                    "expression",
                    format!("cannot allocate bounded generation stack: {error}"),
                )
            })?;
        handle.join().map_err(|_| {
            single_diagnostic(
                request,
                GenerationErrorCode::ResourceLimitExceeded,
                "expression",
                "generation exceeded the bounded stack resource",
            )
        })?
    })
}

fn generate_boolean_oracle_inner(
    request: &OracleRequest<'_>,
) -> Result<OracleArtifactBundle, Vec<GenerationDiagnostic>> {
    validate_manifest_context(request)?;
    if request.expression.value_type() != &ValueType::Boolean {
        return Err(single_diagnostic(
            request,
            GenerationErrorCode::NonBooleanRoot,
            "expression.value_type",
            "oracle roots must have Boolean type",
        ));
    }
    if !request.expression.obligations().is_empty() {
        return Err(single_diagnostic(
            request,
            GenerationErrorCode::UnsupportedObligations,
            "expression.obligations",
            "the Boolean slice cannot preserve discharged definedness obligations",
        ));
    }

    let parameters = dependency_parameters(request)?;
    let parameter_lookup = parameters
        .iter()
        .map(|(dependency, identifier)| (dependency_key(dependency), identifier.clone()))
        .collect::<BTreeMap<_, _>>();
    let rendered = render_expression(request, request.expression.expression(), &parameter_lookup)?;

    let requirement = request.requirement.requirement().as_str();
    let revision = request.requirement.revision().get();
    let clause = request.clause.as_str();
    let symbol_text = oracle_symbol(requirement, revision, clause);
    let identity_symbol = format!("{}_IDENTITY", symbol_text.to_ascii_uppercase());
    let clause_symbol = format!("{}_CLAUSE", symbol_text.to_ascii_uppercase());
    let requirement_literal = format!("{requirement:?}");
    let revision_literal = format!("{:?}", revision.to_string());
    let clause_literal = format!("{clause:?}");
    let parameter_text = parameters
        .iter()
        .map(|(_, identifier)| format!("{identifier}: bool"))
        .collect::<Vec<_>>()
        .join(", ");

    let mut source = SourceBuilder::new();
    for line in [
        "// SPDX-License-Identifier: MIT OR Apache-2.0".to_owned(),
        format!(
            "// Generated by quire-contract-codegen {}; DO NOT EDIT.",
            env!("CARGO_PKG_VERSION")
        ),
        format!("// Requirement: {requirement}@{revision}; Clause: {clause}"),
        String::new(),
        format!("/// Generated contract identity for `{requirement}@{revision}`."),
        format!("pub const {identity_symbol}: quire_contract_runtime::ContractIdentity<'static> ="),
        "quire_contract_runtime::ContractIdentity::new(".to_owned(),
        format!("quire_contract_runtime::RequirementId::new({requirement_literal}),"),
        format!("quire_contract_runtime::RevisionId::new({revision_literal}),"),
        ");".to_owned(),
        format!("/// Generated clause identity for `{clause}`."),
        format!("pub const {clause_symbol}: quire_contract_runtime::ClauseId<'static> ="),
        format!("quire_contract_runtime::ClauseId::new({clause_literal});"),
        format!("/// Evaluates generated oracle `{requirement}@{revision}/{clause}`."),
        "#[must_use]".to_owned(),
        format!("pub fn {symbol_text}({parameter_text}) -> bool {{"),
    ] {
        source.line(&line).map_err(|_| resource_error(request))?;
    }
    let expression_start = source.next_line;
    let offset = expression_start.saturating_sub(1);
    for line in rendered.source.lines() {
        source.line(line).map_err(|_| resource_error(request))?;
    }
    source.line("}").map_err(|_| resource_error(request))?;
    source.implication_regions = rendered
        .implication_regions
        .into_iter()
        .map(|(start, end)| (start.saturating_add(offset), end.saturating_add(offset)))
        .collect();

    syn::parse_file(&source.source).map_err(|error| {
        single_diagnostic(
            request,
            GenerationErrorCode::InvalidGeneratedSyntax,
            "generated.rust",
            error.to_string(),
        )
    })?;

    let source_path = format!("src/generated/{symbol_text}.rs");
    let source_line_count = line_count(&source.source);
    let mut regions = vec![source_region(
        request,
        &source_path,
        "clause",
        1,
        source_line_count,
    )];
    regions.extend(source.implication_regions.into_iter().map(|(start, end)| {
        source_region(request, &source_path, "implication_consequent", start, end)
    }));

    let rust = artifact(source_path, source.source);
    let source_map_contents = deterministic_json(&regions).map_err(|error| {
        single_diagnostic(
            request,
            GenerationErrorCode::SerializationFailed,
            "generated.source_map",
            error.to_string(),
        )
    })?;
    let source_map = artifact(
        format!("source-maps/{symbol_text}.json"),
        source_map_contents,
    );
    let canonical_expression = request
        .expression
        .canonical_expression(CanonicalProfile::V1)
        .map_err(|error| {
            single_diagnostic(
                request,
                GenerationErrorCode::SerializationFailed,
                "expression",
                error.to_string(),
            )
        })?;
    let manifest_value = manifest(
        request,
        &symbol_text,
        &rust,
        &source_map,
        canonical_expression.bytes().as_slice(),
        &canonical_expression.digest().to_string(),
    );
    let manifest_contents = deterministic_json(&manifest_value).map_err(|error| {
        single_diagnostic(
            request,
            GenerationErrorCode::SerializationFailed,
            "generated.manifest",
            error.to_string(),
        )
    })?;
    let manifest = artifact(format!("manifests/{symbol_text}.json"), manifest_contents);
    Ok(OracleArtifactBundle {
        rust,
        source_map,
        manifest,
    })
}

fn validate_manifest_context(request: &OracleRequest<'_>) -> Result<(), Vec<GenerationDiagnostic>> {
    let context = &request.manifest;
    let valid_revision = (40..=64).contains(&context.candidate_revision.len())
        && context
            .candidate_revision
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    let valid_contribution = matches!(
        context.contribution_method,
        "human" | "agent-assisted" | "generated" | "mixed"
    );
    let valid_result = matches!(
        context.result_status,
        "conclusive"
            | "inconclusive"
            | "unsupported"
            | "rejected"
            | "timed-out"
            | "pending"
            | "error"
    );
    let valid_reviewers = !context.reviewers.is_empty()
        && context.reviewers.iter().all(|reviewer| {
            reviewer.strip_prefix('@').is_some_and(|login| {
                !login.is_empty()
                    && login
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            })
        });
    if !valid_revision
        || !valid_contribution
        || !valid_result
        || !valid_reviewers
        || context.result_summary.is_empty()
        || context.requirement_refs.is_empty()
        || context
            .requirement_refs
            .iter()
            .any(|value| value.is_empty())
    {
        return Err(single_diagnostic(
            request,
            GenerationErrorCode::InvalidManifestContext,
            "manifest.context",
            "candidate revision, contribution method, reviewers, result, and requirement refs must satisfy PGM-01",
        ));
    }
    Ok(())
}

pub(crate) fn dependency_parameters(
    request: &OracleRequest<'_>,
) -> Result<Vec<(DependencyIdentity, String)>, Vec<GenerationDiagnostic>> {
    let mut parameters = Vec::with_capacity(request.expression.dependencies().len());
    let mut generated_names = BTreeMap::new();
    for dependency in request.expression.dependencies() {
        if !matches!(
            dependency.kind(),
            DependencyKind::Input | DependencyKind::State
        ) || dependency.path().len() != 1
        {
            return Err(single_diagnostic(
                request,
                GenerationErrorCode::UnsupportedDependency,
                "expression.dependencies",
                "the Boolean slice supports only direct input or state dependencies",
            ));
        }
        let name = dependency.path()[0].as_str();
        let identifier = reference_identifier(name, dependency.observation());
        if let Some(existing) = generated_names.insert(identifier.clone(), dependency) {
            return Err(single_diagnostic(
                request,
                GenerationErrorCode::NameCollision,
                "expression.dependencies",
                format!(
                    "dependency identities {:?} and {:?} claim the same Rust parameter {:?}",
                    existing.path()[0].as_str(),
                    name,
                    identifier
                ),
            ));
        }
        parameters.push((dependency.clone(), identifier));
    }
    Ok(parameters)
}

fn render_expression(
    request: &OracleRequest<'_>,
    expression: &Expression,
    parameters: &BTreeMap<String, String>,
) -> Result<RenderedExpression, Vec<GenerationDiagnostic>> {
    let mut builder = SourceBuilder::new();
    render_node(request, expression, parameters, &mut builder)?;
    Ok(RenderedExpression {
        source: builder.source,
        implication_regions: builder.implication_regions,
    })
}

fn render_node(
    request: &OracleRequest<'_>,
    expression: &Expression,
    parameters: &BTreeMap<String, String>,
    output: &mut SourceBuilder,
) -> Result<(), Vec<GenerationDiagnostic>> {
    let result = match expression.kind() {
        ExpressionKind::BooleanLiteral { value } => {
            output.line(if *value { "true" } else { "false" })
        }
        ExpressionKind::ValueReference { name, observation } => {
            let key = reference_key(name.as_str(), Some(*observation));
            let Some(identifier) = parameters.get(&key) else {
                return Err(single_diagnostic(
                    request,
                    GenerationErrorCode::UnsupportedDependency,
                    "expression.value_reference",
                    "typed dependency census does not contain the referenced Boolean value",
                ));
            };
            output.line(identifier)
        }
        ExpressionKind::BooleanNot { operand } => {
            output.line("!(").map_err(|_| resource_error(request))?;
            render_node(request, operand, parameters, output)?;
            output.line(")")
        }
        ExpressionKind::Boolean {
            operator,
            left,
            right,
        } => {
            let (function, left_closure) = match operator {
                BooleanOperator::ShortCircuitAnd => ("and_short_circuit", false),
                BooleanOperator::ShortCircuitOr => ("or_short_circuit", false),
                BooleanOperator::TotalAnd => ("and_total", true),
                BooleanOperator::TotalOr => ("or_total", true),
                BooleanOperator::Implication => ("implies_short_circuit", false),
            };
            output
                .line(&format!("quire_contract_runtime::operators::{function}("))
                .map_err(|_| resource_error(request))?;
            if left_closure {
                output.line("|| {").map_err(|_| resource_error(request))?;
            }
            render_node(request, left, parameters, output)?;
            if left_closure {
                output.line("},").map_err(|_| resource_error(request))?;
            } else {
                output.line(",").map_err(|_| resource_error(request))?;
            }
            output.line("|| {").map_err(|_| resource_error(request))?;
            let region_index = if *operator == BooleanOperator::Implication {
                let index = output.implication_regions.len();
                output.implication_regions.push((0, 0));
                Some(index)
            } else {
                None
            };
            let start_line = output.next_line;
            render_node(request, right, parameters, output)?;
            let end_line = output.next_line.saturating_sub(1);
            if let Some(index) = region_index {
                output.implication_regions[index] = (start_line, end_line);
            }
            output.line("},").map_err(|_| resource_error(request))?;
            output.line(")")
        }
        other => {
            return Err(single_diagnostic(
                request,
                GenerationErrorCode::UnsupportedExpression,
                "expression.node",
                format!(
                    "unsupported expression in Boolean slice: {}",
                    node_name(other)
                ),
            ));
        }
    };
    result.map_err(|_| resource_error(request))
}

fn manifest(
    request: &OracleRequest<'_>,
    symbol: &str,
    rust: &Artifact,
    source_map: &Artifact,
    canonical_expression: &[u8],
    canonical_digest: &str,
) -> DerivationManifest {
    let requirement = request.requirement.requirement().as_str();
    let revision = GENERATOR_SOURCE_REVISION;
    let implementation_digest = generator_implementation_digest();
    let input_digest = sha256(canonical_expression);
    let parameters = format!(
        "ir={IR_CANDIDATE_REVISION}\nruntime={RUNTIME_REVISION}\nmaximumSourceBytes={MAX_GENERATED_SOURCE_BYTES}\n"
    );
    let symbol_digest = sha256(symbol.as_bytes());
    let record_digest = sha256(format!("{symbol_digest}:{input_digest}").as_bytes());
    let requirement_refs = request
        .manifest
        .requirement_refs
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<BTreeSet<_>>();
    let mut extensions = BTreeMap::new();
    extensions.insert(
        "dev.agent-ix.codegen".to_owned(),
        CodegenExtension {
            terminal_state: GenerationTerminalState::Generated,
            generator_source_dirty: generator_source_is_dirty(),
            generator_source_revision_available: env!("QUIRE_CODEGEN_SOURCE_REVISION_AVAILABLE")
                == "true",
            canonical_profile: CanonicalProfile::V1.as_str().to_owned(),
            expression_canonical_digest: canonical_digest.to_owned(),
            ir_revision: IR_CANDIDATE_REVISION.to_owned(),
            runtime_revision: RUNTIME_REVISION.to_owned(),
            clause_id: request.clause.as_str().to_owned(),
            reviewer_role: "reviewer-of-record; not a GitHub approval".to_owned(),
            maximum_source_bytes: MAX_GENERATED_SOURCE_BYTES,
        },
    );
    DerivationManifest {
        schema_version: "quire.derivation-evidence/v1".to_owned(),
        record_id: format!("oracle:{record_digest}"),
        recorded_at: env!("QUIRE_CODEGEN_RECORDED_AT").to_owned(),
        producer: ProducerIdentity {
            name: "quire-contract-codegen".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            source_revision: revision.to_owned(),
            executable_digest: digest(implementation_digest),
            invocation: vec![
                "generate_boolean_oracle".to_owned(),
                format!(
                    "{requirement}@{}/{}",
                    request.requirement.revision().get(),
                    request.clause.as_str()
                ),
            ],
        },
        inputs: vec![ManifestArtifact {
            role: "canonical-typed-expression".to_owned(),
            uri: format!("urn:sha256:{input_digest}"),
            media_type: "application/json".to_owned(),
            schema: SchemaIdentity {
                id: "quire.contract.canonical-json".to_owned(),
                version: "v1".to_owned(),
                digest: digest(&sha256(CanonicalProfile::V1.as_str().as_bytes())),
            },
            content_digest: digest(&input_digest),
        }],
        backend: NoBackend {
            kind: "none".to_owned(),
            reason: "deterministic in-process Rust lowering; no external backend".to_owned(),
        },
        outputs: vec![
            ManifestArtifact {
                role: "generated-rust-oracle".to_owned(),
                uri: rust.path.clone(),
                media_type: "text/x-rust".to_owned(),
                schema: SchemaIdentity {
                    id: "quire.codegen.rust-oracle".to_owned(),
                    version: "v1".to_owned(),
                    digest: digest(rust_oracle_schema_digest()),
                },
                content_digest: digest(&rust.sha256),
            },
            ManifestArtifact {
                role: "oracle-source-map".to_owned(),
                uri: source_map.path.clone(),
                media_type: "application/json".to_owned(),
                schema: SchemaIdentity {
                    id: "quire.codegen.oracle-source-map".to_owned(),
                    version: "v1".to_owned(),
                    digest: digest(source_map_schema_digest()),
                },
                content_digest: digest(&source_map.sha256),
            },
        ],
        parameters_digest: digest(&sha256(parameters.as_bytes())),
        environment: GenerationEnvironment {
            target_triple: env!("QUIRE_CODEGEN_TARGET").to_owned(),
            operating_system: env!("QUIRE_CODEGEN_TARGET_OS").to_owned(),
            toolchain: env!("QUIRE_CODEGEN_TOOLCHAIN").to_owned(),
            dependencies_digest: digest(lockfile_digest()),
        },
        provenance: GenerationProvenance {
            repository: "https://github.com/agent-ix/quire-contract-codegen".to_owned(),
            source_revision: revision.to_owned(),
            candidate_revision: request.manifest.candidate_revision.to_owned(),
            contribution_method: request.manifest.contribution_method.to_owned(),
            reviewers: request
                .manifest
                .reviewers
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
        },
        result: GenerationResult {
            status: request.manifest.result_status.to_owned(),
            summary: request.manifest.result_summary.to_owned(),
            requirement_refs: requirement_refs.into_iter().collect(),
        },
        extensions,
    }
}

fn node_name(kind: &ExpressionKind) -> &'static str {
    match kind {
        ExpressionKind::BooleanLiteral { .. } => "boolean_literal",
        ExpressionKind::IntegerLiteral { .. } => "integer_literal",
        ExpressionKind::RationalLiteral { .. } => "rational_literal",
        ExpressionKind::TextLiteral { .. } => "text_literal",
        ExpressionKind::EnumLiteral { .. } => "enum_literal",
        ExpressionKind::OptionNone { .. } => "option_none",
        ExpressionKind::OptionSome { .. } => "option_some",
        ExpressionKind::RecordLiteral { .. } => "record_literal",
        ExpressionKind::CollectionLiteral { .. } => "collection_literal",
        ExpressionKind::ValueReference { .. } => "value_reference",
        ExpressionKind::LocalReference { .. } => "local_reference",
        ExpressionKind::FieldAccess { .. } => "field_access",
        ExpressionKind::IsPresent { .. } => "is_present",
        ExpressionKind::Unwrap { .. } => "unwrap",
        ExpressionKind::Length { .. } => "length",
        ExpressionKind::Index { .. } => "index",
        ExpressionKind::Call { .. } => "call",
        ExpressionKind::Numeric { .. } => "numeric",
        ExpressionKind::NumericNegate { .. } => "numeric_negate",
        ExpressionKind::Compare { .. } => "compare",
        ExpressionKind::BooleanNot { .. } => "boolean_not",
        ExpressionKind::Boolean { .. } => "boolean",
        ExpressionKind::Quantifier { .. } => "quantifier",
    }
}

fn dependency_key(dependency: &DependencyIdentity) -> String {
    reference_key(dependency.path()[0].as_str(), dependency.observation())
}

fn reference_key(name: &str, observation: Option<StateObservation>) -> String {
    format!("{}:{name}:{}", name.len(), observation_name(observation))
}

pub(crate) fn reference_identifier(name: &str, observation: Option<StateObservation>) -> String {
    format!("{}_{}", rust_component(name), observation_name(observation))
}

fn observation_name(observation: Option<StateObservation>) -> &'static str {
    match observation {
        Some(StateObservation::Pre) => "pre",
        Some(StateObservation::Post) => "post",
        Some(StateObservation::Current) | None => "current",
    }
}

fn rust_component(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
            result.push(char::from(byte));
        } else {
            let _ = write!(result, "_{byte:02x}");
        }
    }
    if result.is_empty() {
        result.push_str("empty");
    }
    result
}

fn readable_component(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for byte in value.bytes() {
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

pub(crate) fn oracle_symbol(requirement: &str, revision: u64, clause: &str) -> String {
    let readable_requirement = readable_component(requirement)
        .chars()
        .take(24)
        .collect::<String>();
    let readable_clause = readable_component(clause)
        .chars()
        .take(24)
        .collect::<String>();
    let identity = format!(
        "{}:{requirement}:{revision}:{}:{clause}",
        requirement.len(),
        clause.len()
    );
    format!(
        "oracle_{readable_requirement}_{revision}_{readable_clause}_id_{}",
        sha256(identity.as_bytes())
    )
}

fn artifact(path: String, contents: String) -> Artifact {
    Artifact {
        sha256: sha256(contents.as_bytes()),
        path,
        contents,
    }
}

fn deterministic_json(value: &impl Serialize) -> Result<String, SerializationError> {
    let mut bytes = serde_json::to_vec(value).map_err(SerializationError::Json)?;
    bytes.push(b'\n');
    String::from_utf8(bytes).map_err(SerializationError::Utf8)
}

fn sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut value = String::with_capacity(64);
    for byte in digest {
        let _ = write!(value, "{byte:02x}");
    }
    value
}

fn generator_implementation_digest() -> &'static str {
    static VALUE: OnceLock<String> = OnceLock::new();
    VALUE.get_or_init(|| {
        let mut hasher = Sha256::new();
        for value in [
            GENERATOR_SOURCE,
            BUILD_SOURCE,
            LOCKFILE,
            PGM_SCHEMA,
            SOURCE_MAP_SCHEMA,
            RUST_ORACLE_SCHEMA,
            ORACLE_SPEC,
        ] {
            hasher.update(value.len().to_le_bytes());
            hasher.update(value);
        }
        let bytes = hasher.finalize();
        let mut value = String::with_capacity(64);
        for byte in bytes {
            let _ = write!(value, "{byte:02x}");
        }
        value
    })
}

fn cached_digest(value: &'static [u8], cache: &'static OnceLock<String>) -> &'static str {
    cache.get_or_init(|| sha256(value))
}

fn lockfile_digest() -> &'static str {
    static VALUE: OnceLock<String> = OnceLock::new();
    cached_digest(LOCKFILE, &VALUE)
}

fn source_map_schema_digest() -> &'static str {
    static VALUE: OnceLock<String> = OnceLock::new();
    cached_digest(SOURCE_MAP_SCHEMA, &VALUE)
}

fn rust_oracle_schema_digest() -> &'static str {
    static VALUE: OnceLock<String> = OnceLock::new();
    cached_digest(RUST_ORACLE_SCHEMA, &VALUE)
}

fn digest(value: &str) -> DigestIdentity {
    DigestIdentity {
        algorithm: "sha256".to_owned(),
        value: value.to_owned(),
    }
}

fn line_count(value: &str) -> u32 {
    value
        .as_bytes()
        .iter()
        .filter(|byte| **byte == b'\n')
        .count()
        .saturating_add(usize::from(!value.ends_with('\n')))
        .try_into()
        .unwrap_or(u32::MAX)
}

fn source_region(
    request: &OracleRequest<'_>,
    artifact_path: &str,
    role: &str,
    start_line: u32,
    end_line: u32,
) -> SourceRegion {
    SourceRegion {
        artifact_path: artifact_path.to_owned(),
        role: role.to_owned(),
        start_line,
        end_line,
        requirement_id: request.requirement.requirement().as_str().to_owned(),
        requirement_revision: request.requirement.revision().get(),
        clause_id: request.clause.as_str().to_owned(),
    }
}

fn resource_error(request: &OracleRequest<'_>) -> Vec<GenerationDiagnostic> {
    single_diagnostic(
        request,
        GenerationErrorCode::ResourceLimitExceeded,
        "generated.rust",
        format!("generated Rust exceeds {MAX_GENERATED_SOURCE_BYTES} bytes"),
    )
}

fn single_diagnostic(
    request: &OracleRequest<'_>,
    code: GenerationErrorCode,
    path: impl Into<String>,
    message: impl Into<String>,
) -> Vec<GenerationDiagnostic> {
    vec![GenerationDiagnostic {
        code,
        terminal_state: code.terminal_state(),
        requirement_id: request.requirement.requirement().as_str().to_owned(),
        requirement_revision: request.requirement.revision().get(),
        clause_id: request.clause.as_str().to_owned(),
        path: path.into(),
        message: message.into(),
    }]
}

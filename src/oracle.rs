//! Deterministic, fail-closed lowering for the first Boolean-oracle slice.

use std::{collections::BTreeMap, fmt::Write as _, sync::OnceLock};

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

const SOURCE_MAP_SCHEMA: &[u8] = include_bytes!("../schemas/oracle-source-map-v1.schema.json");
const RUST_ORACLE_SCHEMA: &[u8] = include_bytes!("../schemas/generated-rust-oracle-v1.schema.json");
const ORACLE_SPEC: &[u8] = include_bytes!("../spec/functional/FR-001-deterministic-oracles.md");
const GENERATOR_SOURCE: &[u8] = include_bytes!("oracle.rs");
const HARNESS_SOURCE: &[u8] = include_bytes!("harness.rs");
const STRATEGY_SOURCE: &[u8] = include_bytes!("strategy.rs");
const HARNESS_SPEC: &[u8] = include_bytes!("../spec/functional/FR-002-tristate-proptest.md");
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
    /// Caller-owned binding for the attestations emitted with this bundle.
    pub attestation: AttestationContext<'a>,
}

/// Caller-owned binding shared by every attestation emitted with one bundle.
///
/// Both fields are the consuming assurance process's to state, because neither is
/// knowable to a generator: the record is sealed by the process that reviews the
/// change, and the candidate revision is the revision that process is reviewing.
/// The other nine fields a proof attestation declares — the command, the tool, the
/// environment, the time and the result — are stated here and are never accepted
/// from a caller.
///
/// "Stated", not "observed", and the distinction matters for one of them.
/// `observed_at` is the generator's own source-commit timestamp, frozen at build so
/// that regeneration stays byte-identical; it is not an observation of when
/// generation ran, and a consumer generating months later emits an attestation
/// whose time predates the generation. Verification receipts derive staleness from
/// `candidate_revision` rather than from this field, so nothing downstream is
/// misled — but calling it an observation would be.
#[derive(Clone, Copy, Debug)]
pub struct AttestationContext<'a> {
    /// Digest of the sealed change-assurance record these attestations bind to.
    pub record_digest: &'a str,
    /// Candidate revision the generated artifact is offered as evidence about.
    pub candidate_revision: &'a str,
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
    /// The caller-supplied attestation binding is invalid.
    InvalidAttestationContext,
    /// The bounded output resource would be exceeded.
    ResourceLimitExceeded,
    /// Generated tokens did not parse as a Rust source file.
    InvalidGeneratedSyntax,
    /// A deterministic attestation or source-map value could not be encoded.
    SerializationFailed,
}

impl GenerationErrorCode {
    /// Maps the diagnostic category to its interface-001 terminal state.
    #[must_use]
    pub const fn terminal_state(self) -> GenerationTerminalState {
        match self {
            Self::NonBooleanRoot | Self::NameCollision | Self::InvalidAttestationContext => {
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

/// The command a proof attestation declares (`ProofAttestationV1.command`).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttestationCommand {
    /// Argument vector, rendering the in-process invocation and every parameter it used.
    pub argv: Vec<String>,
    /// Directory the argv is stated relative to.
    pub working_directory: String,
}

/// The tool a proof attestation declares (`ProofAttestationV1.tool`).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttestationTool {
    /// Producing repository.
    pub identity: String,
    /// Exact generator revision, in the 40-hexadecimal form the shared schema admits.
    pub version: String,
    /// Digest over the lowering implementation, build script, lockfile, output schemas, and specs.
    pub configuration_digest: String,
}

/// The build environment a generation was observed in (`ProofAttestationV1.environment`).
///
/// The shared schema accepts any object of scalars here. This crate emits a fixed
/// set of them, so that a field going missing is a compile error rather than a
/// quietly smaller map.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttestationEnvironment {
    /// Compilation target triple.
    pub target_triple: String,
    /// Compilation target operating system.
    pub operating_system: String,
    /// Rust compiler identity.
    pub toolchain: String,
    /// Exact Cargo lockfile digest.
    pub dependencies_digest: String,
    /// Whether an exact generator revision was available from Git or archive metadata.
    pub source_revision_available: bool,
    /// Whether build inputs differed from the recorded Git revision.
    pub source_dirty: bool,
}

/// The four results a proof attestation may state (`ProofAttestationV1.result`).
///
/// This is the shared vocabulary and not the generator's. A bundle exists only
/// when generation succeeded, so this crate emits `Passed` and nothing else; the
/// six Interface-001 terminal states stay in [`GenerationDiagnostic`], which no
/// attestation ever accompanies.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttestationResult {
    /// The proof obligation held.
    Passed,
    /// The proof obligation did not hold.
    Failed,
    /// The proof could not be attempted.
    Unavailable,
    /// The proof was attempted and reached no conclusion.
    NotComputed,
}

/// One `ProofAttestationV1` body, emitted beside the artifact it describes.
///
/// This is Quoin's packaged `proof-attestation-v1.schema.json` shape without
/// `digest` and without `retained_output`. Those two fields are not omitted by
/// choice: `quoin change-assurance seal-attestation` derives both and **refuses a
/// body that supplies either**, because they are statements about the retained
/// bytes and about the sealed form, and a producer is not the thing that seals.
/// So this crate emits the body and Quoin seals it — which is the same division
/// of labour `scripts/assurance_chain.py` uses for this repository's four
/// existing proof obligations.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProofAttestationBody {
    /// Shared schema version; always `1`.
    pub schema_version: u8,
    /// Shared record discriminator; always `proof_attestation`.
    pub record_type: String,
    /// Stable identity of this attestation.
    pub attestation_id: String,
    /// Digest of the sealed change-assurance record this attestation binds to.
    pub record_digest: String,
    /// Candidate revision the attestation is about.
    pub candidate_revision: String,
    /// Proof obligation this attestation discharges.
    pub proof_id: String,
    /// The generation invocation, with every parameter it used.
    pub command: AttestationCommand,
    /// The generator's own identity.
    pub tool: AttestationTool,
    /// The build environment the generation was observed in.
    pub environment: AttestationEnvironment,
    /// Deterministic source-commit timestamp, in RFC 3339.
    pub observed_at: String,
    /// The shared four-value result.
    pub result: AttestationResult,
}

/// Complete all-or-nothing result for one supported oracle clause.
///
/// Two artifacts, and therefore two attestations: a proof attestation binds one
/// retained output, so the pair the deprecated manifest packed into a single
/// record is one attestation each.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleArtifactBundle {
    /// Generated Rust source.
    pub rust: Artifact,
    /// Machine-readable source-region map.
    pub source_map: Artifact,
    /// Proof-attestation body for `rust`.
    pub rust_attestation: Artifact,
    /// Proof-attestation body for `source_map`.
    pub source_map_attestation: Artifact,
}

/// Complete all-or-nothing result for one generated Rust artifact.
///
/// Harness and strategy slices do not currently emit clause-level source maps, so
/// they emit one generated artifact and the one attestation that binds it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedArtifactBundle {
    /// Generated Rust source.
    pub rust: Artifact,
    /// Proof-attestation body for `rust`.
    pub attestation: Artifact,
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
    validate_attestation_context(request)?;
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
    let input_digest = sha256(canonical_expression.bytes().as_slice());
    let subject = vec![
        "--requirement".to_owned(),
        format!("{requirement}@{revision}"),
        "--clause".to_owned(),
        clause.to_owned(),
    ];
    let identity = sha256(format!("{}:{input_digest}", sha256(symbol_text.as_bytes())).as_bytes());
    let rust_attestation = oracle_attestation(
        request,
        &identity,
        &AttestationOutput {
            role: ORACLE_RUST_ROLE,
            path: &rust.path,
            media_type: "text/x-rust",
            schema: "quire.codegen.rust-oracle/v1",
            schema_digest: Some(rust_oracle_schema_digest()),
        },
        &subject,
        &input_digest,
        &canonical_expression.digest().to_string(),
    );
    let source_map_attestation = oracle_attestation(
        request,
        &identity,
        &AttestationOutput {
            role: ORACLE_SOURCE_MAP_ROLE,
            path: &source_map.path,
            media_type: "application/json",
            schema: "quire.codegen.oracle-source-map/v1",
            schema_digest: Some(source_map_schema_digest()),
        },
        &subject,
        &input_digest,
        &canonical_expression.digest().to_string(),
    );
    let rust_attestation = attestation_artifact(&symbol_text, ORACLE_RUST_ROLE, &rust_attestation)
        .map_err(|error| {
            single_diagnostic(
                request,
                GenerationErrorCode::SerializationFailed,
                "generated.attestation",
                error.to_string(),
            )
        })?;
    let source_map_attestation = attestation_artifact(
        &symbol_text,
        ORACLE_SOURCE_MAP_ROLE,
        &source_map_attestation,
    )
    .map_err(|error| {
        single_diagnostic(
            request,
            GenerationErrorCode::SerializationFailed,
            "generated.attestation",
            error.to_string(),
        )
    })?;
    Ok(OracleArtifactBundle {
        rust,
        source_map,
        rust_attestation,
        source_map_attestation,
    })
}

fn validate_attestation_context(
    request: &OracleRequest<'_>,
) -> Result<(), Vec<GenerationDiagnostic>> {
    if attestation_context_is_valid(&request.attestation) {
        return Ok(());
    }
    Err(single_diagnostic(
        request,
        GenerationErrorCode::InvalidAttestationContext,
        "attestation.context",
        "record_digest must be 64 lowercase hexadecimal characters and candidate_revision \
         must be a 40-to-64 character lowercase hexadecimal revision",
    ))
}

/// Reports whether a caller's attestation binding can be stated as it stands.
///
/// `record_digest` is held to the shared schema's own `digest` pattern. The
/// `candidate_revision` rule is deliberately stricter than the shared schema,
/// which asks only for a non-empty string: a generated artifact that names an
/// unresolvable revision cannot be checked against anything later, and the
/// deprecated manifest rejected exactly this input, so the rule is carried over
/// rather than relaxed to the schema's floor.
pub(crate) fn attestation_context_is_valid(context: &AttestationContext<'_>) -> bool {
    is_lowercase_hexadecimal(context.record_digest, 64, 64)
        && is_lowercase_hexadecimal(context.candidate_revision, 40, 64)
}

fn is_lowercase_hexadecimal(value: &str, minimum: usize, maximum: usize) -> bool {
    (minimum..=maximum).contains(&value.len())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
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

/// `tool.identity` for every attestation this crate emits.
///
/// The owning repository rather than the bare crate name, because the deprecated
/// envelope's `provenance.repository` has no field of its own in the shared shape
/// and the tool identity is where a reader resolves `tool.version` from.
const TOOL_IDENTITY: &str = "agent-ix/quire-contract-codegen";

/// `command.argv[0]` for every attestation this crate emits.
///
/// The crate name, and it names no program that can be run today: lowering happens
/// in process, `Cargo.toml` declares a library and no binary, and the `cli_generate`
/// operation interface-001 specifies is unimplemented. The deprecated envelope had
/// the same property in `producer.invocation` and this does not pretend otherwise.
/// A runnable-looking `argv[0]` naming something that does not exist would be worse
/// than an honest description of an in-process call, and the limitation is declared
/// as `UNKNOWN-attested-command-is-not-runnable` in the change declaration.
const TOOL_ARGV0: &str = "quire-contract-codegen";

/// `command.working_directory` for every attestation this crate emits.
const WORKING_DIRECTORY: &str = ".";

/// The output role of a generated Rust oracle.
const ORACLE_RUST_ROLE: &str = "generated-rust-oracle";

/// The output role of a generated oracle source map.
const ORACLE_SOURCE_MAP_ROLE: &str = "oracle-source-map";

/// One generated artifact an attestation is about.
struct AttestationOutput<'a> {
    /// Semantic role, which also names the proof obligation.
    role: &'a str,
    /// Bundle-relative path of the artifact.
    path: &'a str,
    /// Media type of the artifact, stated by the generator that knows it.
    media_type: &'a str,
    /// Identifier of the versioned schema the artifact validates against.
    schema: &'a str,
    /// Digest of that schema's own bytes, when the schema is a file in this repository.
    ///
    /// `None` for the harness and strategy slices, which name a schema identifier
    /// for which no schema document exists. The deprecated envelope filled that
    /// slot with `sha256` of the identifier string — the digest of a name rather
    /// than of a schema — and stating nothing is the honest replacement.
    schema_digest: Option<&'a str>,
}

/// The proof obligation an output role discharges.
fn proof_id_for(role: &str) -> String {
    format!("PROOF-codegen-{role}")
}

/// Renders one in-process generation as the argv its attestation declares.
///
/// Everything the deprecated envelope carried as a `parameters_digest`, a
/// `backend` discriminator, an `inputs[]` entry or a namespaced extension is
/// written out here in full. A command line that names its parameters is
/// readable and checkable where a digest over three of them was neither.
fn generation_command(
    operation: &str,
    subject: &[String],
    canonical_profile: &str,
    input_digest: &str,
    output: &AttestationOutput<'_>,
) -> AttestationCommand {
    let mut argv = vec![TOOL_ARGV0.to_owned(), operation.to_owned()];
    argv.extend(subject.iter().cloned());
    argv.extend([
        "--canonical-profile".to_owned(),
        canonical_profile.to_owned(),
        "--input-digest".to_owned(),
        input_digest.to_owned(),
        "--ir-revision".to_owned(),
        IR_CANDIDATE_REVISION.to_owned(),
        "--runtime-revision".to_owned(),
        RUNTIME_REVISION.to_owned(),
        "--backend".to_owned(),
        "none".to_owned(),
        "--maximum-source-bytes".to_owned(),
        MAX_GENERATED_SOURCE_BYTES.to_string(),
        "--output-schema".to_owned(),
        output.schema.to_owned(),
        "--output-media-type".to_owned(),
        output.media_type.to_owned(),
    ]);
    if let Some(schema_digest) = output.schema_digest {
        argv.extend([
            "--output-schema-digest".to_owned(),
            schema_digest.to_owned(),
        ]);
    }
    argv.extend(["--output".to_owned(), output.path.to_owned()]);
    AttestationCommand {
        argv,
        working_directory: WORKING_DIRECTORY.to_owned(),
    }
}

/// Assembles one proof-attestation body over an already-generated artifact.
///
/// `result` is derived rather than accepted: this function is only reached when
/// an artifact exists, so the only honest answer is `passed`. The deprecated
/// envelope took its result status from the caller, which permitted an artifact
/// that generated cleanly to carry `rejected` or `timed-out`.
fn attestation_body(
    context: &AttestationContext<'_>,
    identity: &str,
    output: &AttestationOutput<'_>,
    command: AttestationCommand,
) -> ProofAttestationBody {
    let proof_id = proof_id_for(output.role);
    ProofAttestationBody {
        schema_version: 1,
        record_type: "proof_attestation".to_owned(),
        attestation_id: format!("{proof_id}:{identity}"),
        record_digest: context.record_digest.to_owned(),
        candidate_revision: context.candidate_revision.to_owned(),
        proof_id,
        command,
        tool: AttestationTool {
            identity: TOOL_IDENTITY.to_owned(),
            version: GENERATOR_SOURCE_REVISION.to_owned(),
            configuration_digest: generator_implementation_digest().to_owned(),
        },
        environment: AttestationEnvironment {
            target_triple: env!("QUIRE_CODEGEN_TARGET").to_owned(),
            operating_system: env!("QUIRE_CODEGEN_TARGET_OS").to_owned(),
            toolchain: env!("QUIRE_CODEGEN_TOOLCHAIN").to_owned(),
            dependencies_digest: lockfile_digest().to_owned(),
            source_revision_available: env!("QUIRE_CODEGEN_SOURCE_REVISION_AVAILABLE") == "true",
            source_dirty: generator_source_is_dirty(),
        },
        observed_at: env!("QUIRE_CODEGEN_RECORDED_AT").to_owned(),
        result: AttestationResult::Passed,
    }
}

/// The attestation body for one oracle-slice output.
fn oracle_attestation(
    request: &OracleRequest<'_>,
    identity: &str,
    output: &AttestationOutput<'_>,
    subject: &[String],
    input_digest: &str,
    canonical_digest: &str,
) -> ProofAttestationBody {
    let mut command = generation_command(
        "generate_boolean_oracle",
        subject,
        CanonicalProfile::V1.as_str(),
        input_digest,
        output,
    );
    // The canonical semantic digest of the expression, which is a different fact
    // from the digest of its canonical bytes and was a separate extension field.
    command.argv.extend([
        "--expression-canonical-digest".to_owned(),
        canonical_digest.to_owned(),
    ]);
    attestation_body(&request.attestation, identity, output, command)
}

/// Serializes one attestation body into the artifact emitted beside its output.
fn attestation_artifact(
    symbol: &str,
    role: &str,
    body: &ProofAttestationBody,
) -> Result<Artifact, SerializationError> {
    let contents = deterministic_json(body)?;
    Ok(artifact(
        format!("attestations/{symbol}.{role}.json"),
        contents,
    ))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn generated_artifact_bundle(
    context: &AttestationContext<'_>,
    requirement: &RequirementRef,
    operation: &str,
    stable_identity: &str,
    input_bytes: &[u8],
    output_role: &str,
    output_schema: &str,
    rust: Artifact,
) -> Result<GeneratedArtifactBundle, GenerationErrorCode> {
    if !attestation_context_is_valid(context) {
        return Err(GenerationErrorCode::InvalidAttestationContext);
    }
    if rust.contents.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(GenerationErrorCode::ResourceLimitExceeded);
    }
    let input_digest = sha256(input_bytes);
    let identity = sha256(
        length_delimited_identity(&[
            operation,
            requirement.requirement().as_str(),
            &requirement.revision().get().to_string(),
            stable_identity,
            &input_digest,
        ])
        .as_bytes(),
    );
    let output = AttestationOutput {
        role: output_role,
        path: &rust.path,
        media_type: "text/x-rust",
        schema: output_schema,
        schema_digest: None,
    };
    let subject = vec![
        "--requirement".to_owned(),
        format!(
            "{}@{}",
            requirement.requirement().as_str(),
            requirement.revision().get()
        ),
        "--identity".to_owned(),
        stable_identity.to_owned(),
    ];
    let command = generation_command(
        operation,
        &subject,
        "quire.codegen.request/v1",
        &input_digest,
        &output,
    );
    let body = attestation_body(context, &identity, &output, command);
    let attestation = attestation_artifact(
        &format!("{}_{}", bounded_readable_component(operation), identity),
        output_role,
        &body,
    )
    .map_err(|_| GenerationErrorCode::SerializationFailed)?;
    Ok(GeneratedArtifactBundle { rust, attestation })
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

pub(crate) fn bounded_readable_component(value: &str) -> String {
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
    result.chars().take(24).collect()
}

pub(crate) fn length_delimited_identity(values: &[&str]) -> String {
    values
        .iter()
        .map(|value| format!("{}:{value}", value.len()))
        .collect::<Vec<_>>()
        .join(":")
}

pub(crate) fn oracle_symbol(requirement: &str, revision: u64, clause: &str) -> String {
    let readable_requirement = bounded_readable_component(requirement);
    let readable_clause = bounded_readable_component(clause);
    let revision_text = revision.to_string();
    let identity = length_delimited_identity(&[requirement, &revision_text, clause]);
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
            HARNESS_SOURCE,
            STRATEGY_SOURCE,
            BUILD_SOURCE,
            LOCKFILE,
            SOURCE_MAP_SCHEMA,
            RUST_ORACLE_SCHEMA,
            ORACLE_SPEC,
            HARNESS_SPEC,
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

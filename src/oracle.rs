//! Deterministic, fail-closed lowering for Boolean clauses over Boolean and integer values.

use std::{collections::BTreeMap, fmt::Write as _};

use quire_contract_ir::{
    BooleanOperator, ClauseId, ComparisonOperator, DefinednessObligationKind, DependencyIdentity,
    DependencyKind, Expression, ExpressionKind, IntegerType, NumericOperator, RequirementRef,
    SourceSpan, StateObservation, TypedExpression, ValueType,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

/// Exact reviewed public executable-binding IR revision consumed by this implementation.
pub const IR_CANDIDATE_REVISION: &str = "48ab5dc29213c3975a5fe8f04ecbb3d1c2b345bb";

/// Exact merged runtime revision required by generated source.
pub const RUNTIME_REVISION: &str = "ed0a04b482216b79d3559a6ac59e6e260c5591cf";

/// The `[package.metadata.kani]` table every generated oracle crate's manifest carries. CBMC
/// tracks heap objects field by field only up to 64 bytes by default; RT's `Value` and `ValueType`
/// are larger, and a non-field-sensitive read of them cannot be constant-folded, so the crate raises
/// the limit the way RT's own `Cargo.toml` does.
pub(crate) const ORACLE_KANI_METADATA: &str = "[package.metadata.kani]\nunstable = { unstable-options = true }\nflags = { cbmc-args = [\"--max-field-sensitivity-array-size\", \"1024\"] }\n\n";

/// Maximum generated Rust bytes for one clause.
pub const MAX_GENERATED_SOURCE_BYTES: usize = 1_048_576;

/// One validated clause supplied to the oracle lowering core.
///
/// This low-level boundary does not establish complete package binding. Normal package consumers
/// use [`crate::generate_bound_oracles`] with the IR-owned validated projection.
pub struct OracleRequest<'a> {
    /// Requirement identity and revision.
    pub requirement: &'a RequirementRef,
    /// Stable clause identity within the requirement revision.
    pub clause: &'a ClauseId,
    /// Validated typed expression for the clause root.
    pub expression: &'a TypedExpression,
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

impl GenerationTerminalState {
    /// Every terminal state, in declaration order. This is the census `tests/interface_001.rs`
    /// compares against interface-001's declared `diagnostics.terminal_states`, kept beside the
    /// enum rather than hand-copied into the test, so the two live in the same file a developer
    /// edits when adding a variant.
    ///
    /// This array is not itself compiler-checked against the enum's variant set — Rust has no
    /// stable way to derive that without a proc-macro crate this workspace does not depend on.
    /// What the compiler does enforce is [`Self::label`] below: its `match` is exhaustive, so an
    /// added variant fails the build until it is named there. Nothing forces the same edit to
    /// reach this array at compile time; that is left to the developer fixing the build, standing
    /// right next to it. `tests/it/interface_001.rs`'s `census_enum_variants` closes the gap at
    /// test time instead, by counting this enum's own declared variants and asserting the count
    /// equals `ALL.len()`.
    pub const ALL: [Self; 6] = [
        Self::Generated,
        Self::Unsupported,
        Self::InvalidInput,
        Self::BackendUnavailable,
        Self::IoFailed,
        Self::Inconclusive,
    ];

    /// interface-001's declared label for this terminal state. Exhaustive: a variant not named
    /// here fails the build.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Generated => "generated",
            Self::Unsupported => "unsupported",
            Self::InvalidInput => "invalid-input",
            Self::BackendUnavailable => "backend-unavailable",
            Self::IoFailed => "io-failed",
            Self::Inconclusive => "inconclusive",
        }
    }
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
    /// The bounded output resource would be exceeded.
    ResourceLimitExceeded,
    /// Generated tokens did not parse as a Rust source file.
    InvalidGeneratedSyntax,
    /// A deterministic source-map value could not be encoded.
    SerializationFailed,
}

impl GenerationErrorCode {
    /// Maps the diagnostic category to its interface-001 terminal state.
    #[must_use]
    pub const fn terminal_state(self) -> GenerationTerminalState {
        match self {
            Self::NonBooleanRoot | Self::NameCollision => GenerationTerminalState::InvalidInput,
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
    /// Exact IR-owned locus for expression-related failures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span: Option<SourceSpan>,
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
    /// Package identity completing the clause reference.
    pub package_id: String,
    /// Requirement identity.
    pub requirement_id: String,
    /// Exact requirement revision.
    pub requirement_revision: u64,
    /// Clause identity.
    pub clause_id: String,
    /// Entry-token probe for executable semantic roles; never the whole clause envelope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe: Option<SourceProbe>,
    /// Implication census independently traversed from typed IR, on the clause envelope only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_consequents: Option<u32>,
}

/// Single-line source token to be contained by one measured LLVM active span.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SourceProbe {
    /// One-based source line.
    pub line: u32,
    /// One-based UTF-8 byte column, inclusive.
    pub start_column: u32,
    /// One-based UTF-8 byte column, exclusive.
    pub end_column: u32,
}

/// Complete all-or-nothing result for one supported oracle clause.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleArtifactBundle {
    /// Generated Rust source.
    pub rust: Artifact,
    /// Machine-readable source-region map.
    pub source_map: Artifact,
}

/// Complete all-or-nothing result for one generated Rust artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedArtifactBundle {
    /// Generated Rust source.
    pub rust: Artifact,
}

struct RenderedExpression {
    source: String,
    implication_regions: Vec<(u32, u32)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RustValueType {
    Boolean,
    Integer(IntegerType),
}

impl RustValueType {
    const fn source_name(&self) -> &'static str {
        match self {
            Self::Boolean => "bool",
            Self::Integer(_) => "i64",
        }
    }
}

#[derive(Clone)]
struct ReferenceInfo {
    value_type: RustValueType,
    source: SourceSpan,
}

#[derive(Default)]
struct ExpressionAnalysis {
    references: BTreeMap<String, ReferenceInfo>,
    reference_order: Vec<String>,
}

pub(crate) struct DependencyParameter {
    pub(crate) dependency: DependencyIdentity,
    pub(crate) identifier: String,
    pub(crate) value_type: RustValueType,
    pub(crate) source: SourceSpan,
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
    if request.expression.value_type() != &ValueType::Boolean {
        return Err(expression_diagnostic(
            request,
            GenerationErrorCode::NonBooleanRoot,
            "expression.value_type",
            "oracle roots must have Boolean type",
            request.expression.expression().source(),
        ));
    }
    if let Some(obligation) = request.expression.obligations().iter().find(|obligation| {
        !matches!(
            obligation.kind(),
            DefinednessObligationKind::NonZeroDivisor | DefinednessObligationKind::CheckedRange
        )
    }) {
        return Err(expression_diagnostic(
            request,
            GenerationErrorCode::UnsupportedObligations,
            "expression.obligations",
            "the oracle slice cannot preserve discharged definedness obligations",
            obligation.source(),
        ));
    }

    let parameters = typed_dependency_parameters(request)?;
    let parameter_lookup = parameters
        .iter()
        .map(|parameter| {
            (
                dependency_key(&parameter.dependency),
                parameter.identifier.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let rendered = render_expression(request, request.expression.expression(), &parameter_lookup)?;

    let requirement = request.requirement.requirement().as_str();
    let revision = request.requirement.revision().get();
    let clause = request.clause.as_str();
    let symbol_text = oracle_symbol(
        request.requirement.package().as_str(),
        requirement,
        revision,
        clause,
    );
    let identity_symbol = format!("{}_IDENTITY", symbol_text.to_ascii_uppercase());
    let clause_symbol = format!("{}_CLAUSE", symbol_text.to_ascii_uppercase());
    let requirement_literal = format!("{requirement:?}");
    let revision_literal = format!("{:?}", revision.to_string());
    let clause_literal = format!("{clause:?}");
    let parameter_text = parameters
        .iter()
        .map(|parameter| {
            format!(
                "{}: {}",
                parameter.identifier,
                parameter.value_type.source_name()
            )
        })
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
    regions[0].expected_consequents = Some(implication_count(request.expression.expression()));
    // Index once: rescanning lines for every consequent would make probe extraction quadratic.
    let source_lines = source.source.lines().collect::<Vec<_>>();
    let mut evaluation = source_region(request, &source_path, "oracle_evaluation", offset, offset);
    evaluation.probe = Some(entry_probe(source_lines[offset as usize - 1], offset));
    regions.push(evaluation);
    regions.extend(source.implication_regions.into_iter().map(|(start, end)| {
        let mut region = source_region(request, &source_path, "implication_consequent", start, end);
        region.probe = Some(entry_probe(source_lines[start as usize - 1], start));
        region
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
    Ok(OracleArtifactBundle { rust, source_map })
}

pub(crate) fn dependency_parameters(
    request: &OracleRequest<'_>,
) -> Result<Vec<(DependencyIdentity, String)>, Vec<GenerationDiagnostic>> {
    let parameters = typed_dependency_parameters(request)?;
    if let Some(parameter) = parameters
        .iter()
        .find(|parameter| parameter.value_type != RustValueType::Boolean)
    {
        return Err(expression_diagnostic(
            request,
            GenerationErrorCode::UnsupportedDependency,
            "expression.dependencies",
            "the current harness and Kani slices do not support integer dependencies",
            &parameter.source,
        ));
    }
    Ok(parameters
        .into_iter()
        .map(|parameter| (parameter.dependency, parameter.identifier))
        .collect())
}

pub(crate) fn typed_dependency_parameters(
    request: &OracleRequest<'_>,
) -> Result<Vec<DependencyParameter>, Vec<GenerationDiagnostic>> {
    let analysis = analyze_supported_expression(request)?;
    for key in &analysis.reference_order {
        let represented = request.expression.dependencies().iter().any(|dependency| {
            matches!(
                dependency.kind(),
                DependencyKind::Input | DependencyKind::State
            ) && dependency.path().len() == 1
                && dependency_key(dependency) == *key
        });
        if !represented {
            let reference = &analysis.references[key];
            return Err(expression_diagnostic(
                request,
                GenerationErrorCode::UnsupportedDependency,
                "expression.value_reference",
                "typed dependency census does not contain the referenced value",
                &reference.source,
            ));
        }
    }

    let mut parameters = Vec::with_capacity(request.expression.dependencies().len());
    let mut generated_names = BTreeMap::new();
    for dependency in request.expression.dependencies() {
        if !matches!(
            dependency.kind(),
            DependencyKind::Input | DependencyKind::State
        ) || dependency.path().len() != 1
        {
            return Err(expression_diagnostic(
                request,
                GenerationErrorCode::UnsupportedDependency,
                "expression.dependencies",
                "the oracle slice supports only direct input or state dependencies",
                first_reference_span(request, &analysis),
            ));
        }
        let key = dependency_key(dependency);
        let Some(reference) = analysis.references.get(&key) else {
            return Err(expression_diagnostic(
                request,
                GenerationErrorCode::UnsupportedDependency,
                "expression.dependencies",
                "typed dependency census contains no matching value reference",
                first_reference_span(request, &analysis),
            ));
        };
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
        parameters.push(DependencyParameter {
            dependency: dependency.clone(),
            identifier,
            value_type: reference.value_type.clone(),
            source: reference.source.clone(),
        });
    }
    Ok(parameters)
}

fn first_reference_span<'a>(
    request: &'a OracleRequest<'_>,
    analysis: &'a ExpressionAnalysis,
) -> &'a SourceSpan {
    analysis
        .reference_order
        .first()
        .and_then(|key| analysis.references.get(key))
        .map_or_else(
            || request.expression.expression().source(),
            |item| &item.source,
        )
}

fn analyze_supported_expression(
    request: &OracleRequest<'_>,
) -> Result<ExpressionAnalysis, Vec<GenerationDiagnostic>> {
    let mut analysis = ExpressionAnalysis::default();
    let mut next_index = 0_u32;
    analyze_node(
        request,
        request.expression.expression(),
        &mut next_index,
        &mut analysis,
    )?;
    if next_index as usize != request.expression.nodes().len() {
        let span = request
            .expression
            .nodes()
            .get(next_index as usize)
            .map_or_else(
                || request.expression.expression().source(),
                |node| node.source(),
            );
        return Err(expression_diagnostic(
            request,
            GenerationErrorCode::UnsupportedExpression,
            "expression.nodes",
            "typed node census does not match the authored expression tree",
            span,
        ));
    }
    Ok(analysis)
}

fn analyze_node(
    request: &OracleRequest<'_>,
    expression: &Expression,
    next_index: &mut u32,
    analysis: &mut ExpressionAnalysis,
) -> Result<RustValueType, Vec<GenerationDiagnostic>> {
    let index = *next_index;
    *next_index = next_index.saturating_add(1);
    let Some(typed_node) = request.expression.nodes().get(index as usize) else {
        return Err(expression_diagnostic(
            request,
            GenerationErrorCode::UnsupportedExpression,
            "expression.nodes",
            "typed node census ended before the authored expression tree",
            expression.source(),
        ));
    };
    if typed_node.index() != index || typed_node.source() != expression.source() {
        return Err(expression_diagnostic(
            request,
            GenerationErrorCode::UnsupportedExpression,
            "expression.nodes",
            "typed node identity does not match authored preorder",
            expression.source(),
        ));
    }
    let typed_value = rust_value_type(typed_node.value_type());
    let analyzed = match expression.kind() {
        ExpressionKind::BooleanLiteral { .. } => RustValueType::Boolean,
        ExpressionKind::IntegerLiteral { value_type, .. } => {
            if typed_node.value_type() != &ValueType::integer(value_type.clone()) {
                return Err(unsupported_node(
                    request,
                    expression,
                    "integer literal type differs from its typed node",
                ));
            }
            RustValueType::Integer(value_type.clone())
        }
        ExpressionKind::ValueReference { name, observation } => {
            let Some(value_type) = typed_value.as_ref() else {
                return Err(expression_diagnostic(
                    request,
                    GenerationErrorCode::UnsupportedDependency,
                    "expression.value_reference",
                    "only Boolean and bounded-integer direct values can be oracle parameters",
                    expression.source(),
                ));
            };
            let key = reference_key(name.as_str(), Some(*observation));
            match analysis.references.get(&key) {
                Some(existing) if &existing.value_type != value_type => {
                    return Err(expression_diagnostic(
                        request,
                        GenerationErrorCode::UnsupportedDependency,
                        "expression.value_reference",
                        "one dependency identity has conflicting typed value references",
                        expression.source(),
                    ));
                }
                Some(_) => {}
                None => {
                    analysis.reference_order.push(key.clone());
                    analysis.references.insert(
                        key,
                        ReferenceInfo {
                            value_type: value_type.clone(),
                            source: expression.source().clone(),
                        },
                    );
                }
            }
            value_type.clone()
        }
        ExpressionKind::BooleanNot { operand } => {
            require_boolean_child(request, operand, next_index, analysis, expression)?;
            RustValueType::Boolean
        }
        ExpressionKind::Boolean { left, right, .. } => {
            require_boolean_child(request, left, next_index, analysis, expression)?;
            require_boolean_child(request, right, next_index, analysis, expression)?;
            RustValueType::Boolean
        }
        ExpressionKind::Compare { left, right, .. } => {
            let left_type = analyze_node(request, left, next_index, analysis)?;
            let right_type = analyze_node(request, right, next_index, analysis)?;
            if !matches!(left_type, RustValueType::Integer(_))
                || !matches!(right_type, RustValueType::Integer(_))
            {
                return Err(unsupported_node(
                    request,
                    expression,
                    "the oracle slice supports comparisons only between bounded integers",
                ));
            }
            RustValueType::Boolean
        }
        ExpressionKind::Numeric { left, right, .. } => {
            let left_type = analyze_node(request, left, next_index, analysis)?;
            let right_type = analyze_node(request, right, next_index, analysis)?;
            if !matches!(left_type, RustValueType::Integer(_)) || left_type != right_type {
                return Err(unsupported_node(
                    request,
                    expression,
                    "checked arithmetic requires matching bounded integer operands",
                ));
            }
            left_type
        }
        ExpressionKind::NumericNegate { .. } => {
            return Err(unsupported_node(
                request,
                expression,
                "numeric negation requires an invalid-result API and is refused",
            ));
        }
        _ => {
            return Err(unsupported_node(
                request,
                expression,
                format!(
                    "unsupported expression in oracle slice: {}",
                    node_name(expression.kind())
                ),
            ));
        }
    };
    if typed_value.as_ref() != Some(&analyzed) {
        return Err(unsupported_node(
            request,
            expression,
            "typed node value type does not match the supported expression grammar",
        ));
    }
    Ok(analyzed)
}

fn rust_value_type(value_type: &ValueType) -> Option<RustValueType> {
    match value_type {
        ValueType::Boolean => Some(RustValueType::Boolean),
        ValueType::Integer { value } => Some(RustValueType::Integer(value.clone())),
        _ => None,
    }
}

fn require_boolean_child(
    request: &OracleRequest<'_>,
    child: &Expression,
    next_index: &mut u32,
    analysis: &mut ExpressionAnalysis,
    parent: &Expression,
) -> Result<(), Vec<GenerationDiagnostic>> {
    if analyze_node(request, child, next_index, analysis)? != RustValueType::Boolean {
        return Err(unsupported_node(
            request,
            parent,
            "Boolean operators require Boolean operands",
        ));
    }
    Ok(())
}

fn unsupported_node(
    request: &OracleRequest<'_>,
    expression: &Expression,
    message: impl Into<String>,
) -> Vec<GenerationDiagnostic> {
    expression_diagnostic(
        request,
        GenerationErrorCode::UnsupportedExpression,
        "expression.node",
        message,
        expression.source(),
    )
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
        ExpressionKind::IntegerLiteral { value, .. } => {
            let literal = if *value == i64::MIN {
                "i64::MIN".to_owned()
            } else {
                format!("{value}_i64")
            };
            output.line(&literal)
        }
        ExpressionKind::ValueReference { name, observation } => {
            let key = reference_key(name.as_str(), Some(*observation));
            let Some(identifier) = parameters.get(&key) else {
                return Err(expression_diagnostic(
                    request,
                    GenerationErrorCode::UnsupportedDependency,
                    "expression.value_reference",
                    "typed dependency census does not contain the referenced value",
                    expression.source(),
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
        ExpressionKind::Compare {
            operator,
            left,
            right,
        } => {
            let operator = match operator {
                ComparisonOperator::Equal => "==",
                ComparisonOperator::NotEqual => "!=",
                ComparisonOperator::Less => "<",
                ComparisonOperator::LessEqual => "<=",
                ComparisonOperator::Greater => ">",
                ComparisonOperator::GreaterEqual => ">=",
            };
            output.line("(").map_err(|_| resource_error(request))?;
            render_node(request, left, parameters, output)?;
            output.line(")").map_err(|_| resource_error(request))?;
            output.line(operator).map_err(|_| resource_error(request))?;
            output.line("(").map_err(|_| resource_error(request))?;
            render_node(request, right, parameters, output)?;
            output.line(")")
        }
        ExpressionKind::Numeric {
            operator,
            left,
            right,
        } => {
            let operator = match operator {
                NumericOperator::Add => "+",
                NumericOperator::Subtract => "-",
                NumericOperator::Multiply => "*",
                NumericOperator::Divide => "/",
                NumericOperator::Remainder => "%",
            };
            output.line("(").map_err(|_| resource_error(request))?;
            render_node(request, left, parameters, output)?;
            output.line(")").map_err(|_| resource_error(request))?;
            output.line(operator).map_err(|_| resource_error(request))?;
            output.line("(").map_err(|_| resource_error(request))?;
            render_node(request, right, parameters, output)?;
            output.line(")")
        }
        other => {
            return Err(expression_diagnostic(
                request,
                GenerationErrorCode::UnsupportedExpression,
                "expression.node",
                format!(
                    "unsupported expression in oracle slice: {}",
                    node_name(other)
                ),
                expression.source(),
            ));
        }
    };
    result.map_err(|_| resource_error(request))
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

pub(crate) fn oracle_symbol(
    package: &str,
    requirement: &str,
    revision: u64,
    clause: &str,
) -> String {
    let readable_requirement = bounded_readable_component(requirement);
    let readable_clause = bounded_readable_component(clause);
    let revision_text = revision.to_string();
    let identity = length_delimited_identity(&[package, requirement, &revision_text, clause]);
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
        package_id: request.requirement.package().as_str().to_owned(),
        requirement_id: request.requirement.requirement().as_str().to_owned(),
        requirement_revision: request.requirement.revision().get(),
        clause_id: request.clause.as_str().to_owned(),
        probe: None,
        expected_consequents: None,
    }
}

fn entry_probe(text: &str, line: u32) -> SourceProbe {
    // Generated expressions and declarations begin with an ASCII identifier or punctuation.
    let start = text.len() - text.trim_start().len();
    let token_length = text[start..]
        .bytes()
        .take_while(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
        .count()
        .max(1);
    SourceProbe {
        line,
        start_column: start as u32 + 1,
        end_column: (start + token_length) as u32 + 1,
    }
}

fn implication_count(expression: &Expression) -> u32 {
    let mut pending = vec![expression];
    let mut count = 0;
    while let Some(expression) = pending.pop() {
        match expression.kind() {
            ExpressionKind::Boolean {
                operator,
                left,
                right,
            } => {
                count += u32::from(*operator == BooleanOperator::Implication);
                pending.extend([left.as_ref(), right.as_ref()]);
            }
            ExpressionKind::BooleanNot { operand } => pending.push(operand),
            _ => {}
        }
    }
    count
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
    diagnostic(request, code, path, message, None)
}

fn expression_diagnostic(
    request: &OracleRequest<'_>,
    code: GenerationErrorCode,
    path: impl Into<String>,
    message: impl Into<String>,
    source_span: &SourceSpan,
) -> Vec<GenerationDiagnostic> {
    debug_assert!(matches!(
        code,
        GenerationErrorCode::NonBooleanRoot
            | GenerationErrorCode::UnsupportedExpression
            | GenerationErrorCode::UnsupportedDependency
            | GenerationErrorCode::UnsupportedObligations
    ));
    diagnostic(request, code, path, message, Some(source_span.clone()))
}

fn diagnostic(
    request: &OracleRequest<'_>,
    code: GenerationErrorCode,
    path: impl Into<String>,
    message: impl Into<String>,
    source_span: Option<SourceSpan>,
) -> Vec<GenerationDiagnostic> {
    vec![GenerationDiagnostic {
        code,
        terminal_state: code.terminal_state(),
        requirement_id: request.requirement.requirement().as_str().to_owned(),
        requirement_revision: request.requirement.revision().get(),
        clause_id: request.clause.as_str().to_owned(),
        path: path.into(),
        source_span,
        message: message.into(),
    }]
}

//! Deterministic, fail-closed lowering for Boolean clauses over Boolean and integer values.

use std::collections::BTreeMap;

use quire_contract_model::{
    BooleanOperator, ClauseId, ComparisonOperator, DefinednessObligationKind, DependencyIdentity,
    DependencyKind, Expression, ExpressionKind, IntegerType, NumericOperator, OverflowPolicy,
    RequirementRef, SourceSpan, StateObservation, TypedExpression, ValueType,
};
use serde::Serialize;

use crate::{
    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},
    core::diagnostic::{GenerationDiagnostic, GenerationErrorCode},
    core::naming::{observation_name, oracle_symbol, reference_identifier},
    core::source_map::{SourceProbe, SourceRegion},
};

/// One validated clause supplied to the oracle lowering core.
///
/// This low-level boundary does not establish complete package binding. Normal package consumers
/// use [`generate_bound_oracles`](crate::oracle::bound_v1::generate_bound_oracles) with the
/// IR-owned validated projection.
pub struct OracleRequest<'a> {
    /// Requirement identity and revision.
    pub requirement: &'a RequirementRef,
    /// Stable clause identity within the requirement revision.
    pub clause: &'a ClauseId,
    /// Validated typed expression for the clause root.
    pub expression: &'a TypedExpression,
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

/// The consumer a Boolean oracle is rendered for. FR-031 gives integer arithmetic a different
/// shape for each, because they need different things from the same rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OracleShape {
    /// A native oracle, evaluated by a Rust program. Integer add, subtract and multiply are
    /// Contract Runtime `exact` calls, and an oracle holding any returns `Outcome<bool>` and
    /// takes a trailing `&mut Meter`. An arithmetic-free oracle returns `bool`.
    Native,
    /// The oracle `generate_kani_bundle` embeds in a `proof_for_contract` harness. It is
    /// evaluated only by `cargo kani`, so add, subtract and multiply stay the infix `i64`
    /// operators whose overflow is Kani's own falsifiable check, and it returns `bool`.
    KaniBundle,
    /// A consumer that needs a plain `bool` and cannot carry an outcome. It refuses a clause
    /// holding integer arithmetic rather than read an `Outcome<bool>` as a `bool`.
    PlainBool,
}

struct RenderedExpression {
    source: String,
    implication_regions: Vec<(u32, u32)>,
    /// Whether the clause holds an integer arithmetic node.
    has_arithmetic: bool,
    /// The generated helper functions the rendered body calls.
    helpers: Helpers,
}

/// The helper functions a native arithmetic oracle's body calls, emitted once per file.
#[derive(Clone, Copy, Default)]
struct Helpers {
    bound: bool,
    equality: bool,
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

struct ExpressionAnalysis {
    shape: OracleShape,
    references: BTreeMap<String, ReferenceInfo>,
    reference_order: Vec<String>,
}

impl ExpressionAnalysis {
    fn new(shape: OracleShape) -> Self {
        Self {
            shape,
            references: BTreeMap::new(),
            reference_order: Vec::new(),
        }
    }
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
pub fn generate_boolean_oracle(
    request: &OracleRequest<'_>,
) -> Result<OracleArtifactBundle, Vec<GenerationDiagnostic>> {
    generate_boolean_oracle_shaped(request, OracleShape::Native)
}

/// [`generate_boolean_oracle`] rendered for `shape`, naming the function from the request.
pub(crate) fn generate_boolean_oracle_shaped(
    request: &OracleRequest<'_>,
    shape: OracleShape,
) -> Result<OracleArtifactBundle, Vec<GenerationDiagnostic>> {
    let symbol = oracle_symbol(
        request.requirement.requirement().as_str(),
        request.requirement.revision().get(),
        request.clause.as_str(),
    );
    generate_named_boolean_oracle(request, &symbol, shape)
}

/// [`generate_boolean_oracle`] with the oracle function named `symbol`, for generators that name
/// several oracles together through [`unique_names`](crate::core::naming::unique_names), rendered
/// for `shape`.
pub(crate) fn generate_named_boolean_oracle(
    request: &OracleRequest<'_>,
    symbol: &str,
    shape: OracleShape,
) -> Result<OracleArtifactBundle, Vec<GenerationDiagnostic>> {
    if request.expression.nodes().len() < 128 {
        return generate_boolean_oracle_inner(request, symbol, shape);
    }
    std::thread::scope(|scope| {
        let handle = std::thread::Builder::new()
            .name("contract-oracle-generation".to_owned())
            .stack_size(16 * 1024 * 1024)
            .spawn_scoped(scope, || {
                generate_boolean_oracle_inner(request, symbol, shape)
            })
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
    symbol_text: &str,
    shape: OracleShape,
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

    let parameters = typed_dependency_parameters(request, shape)?;
    let parameter_lookup = parameters
        .iter()
        .map(|parameter| {
            (
                dependency_key(&parameter.dependency),
                parameter.identifier.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let rendered = render_expression(
        request,
        request.expression.expression(),
        &parameter_lookup,
        shape,
    )?;
    // A native oracle holding arithmetic returns an outcome and takes the caller's meter; every
    // other oracle returns a plain bool.
    let returns_outcome = shape == OracleShape::Native && rendered.has_arithmetic;

    let requirement = request.requirement.requirement().as_str();
    let revision = request.requirement.revision().get();
    let clause = request.clause.as_str();
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

    let signature = if returns_outcome {
        let meter = "meter: &mut rt::Meter";
        let parameter_text = if parameter_text.is_empty() {
            meter.to_owned()
        } else {
            format!("{parameter_text}, {meter}")
        };
        format!("pub fn {symbol_text}({parameter_text}) -> rt::Outcome<bool> {{")
    } else {
        format!("pub fn {symbol_text}({parameter_text}) -> bool {{")
    };
    let mut header = vec![
        "// SPDX-License-Identifier: MIT OR Apache-2.0".to_owned(),
        format!("// Requirement: {requirement}@{revision}; Clause: {clause}"),
    ];
    if shape == OracleShape::KaniBundle && rendered.has_arithmetic {
        header.push(KANI_ARITHMETIC_NOTE.to_owned());
    }
    header.push(String::new());
    if returns_outcome {
        header.push("use quire_contract_runtime::exact as rt;".to_owned());
        header.extend(native_helper_lines(rendered.helpers));
    }

    let mut source = SourceBuilder::new();
    for line in header.into_iter().chain([
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
        signature,
    ]) {
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
    let clause_region = SourceRegion {
        expected_consequents: Some(implication_count(request.expression.expression())),
        ..source_region(request, &source_path, "clause", 1, source_line_count)
    };
    // Index once: rescanning lines for every consequent would make probe extraction quadratic.
    let source_lines = source.source.lines().collect::<Vec<_>>();
    // The renderer's own output names these lines; a line it does not have is a source map that
    // does not match the source, refused rather than indexed.
    let probe_at = |line: u32| match line
        .checked_sub(1)
        .and_then(|at| source_lines.get(at as usize))
    {
        Some(text) => Ok(entry_probe(text, line)),
        None => Err(single_diagnostic(
            request,
            GenerationErrorCode::InvalidGeneratedSyntax,
            "generated.source_map",
            format!("source map names line {line}, which the generated source does not have"),
        )),
    };
    let mut evaluation = source_region(request, &source_path, "oracle_evaluation", offset, offset);
    evaluation.probe = Some(probe_at(offset)?);
    let consequents = source
        .implication_regions
        .into_iter()
        .map(|(start, end)| {
            let mut region =
                source_region(request, &source_path, "implication_consequent", start, end);
            region.probe = Some(probe_at(start)?);
            Ok(region)
        })
        .collect::<Result<Vec<_>, Vec<GenerationDiagnostic>>>()?;
    let mut regions = vec![clause_region, evaluation];
    regions.extend(consequents);

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
    let parameters = typed_dependency_parameters(request, OracleShape::PlainBool)?;
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
    shape: OracleShape,
) -> Result<Vec<DependencyParameter>, Vec<GenerationDiagnostic>> {
    let analysis = analyze_supported_expression(request, shape)?;
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
    shape: OracleShape,
) -> Result<ExpressionAnalysis, Vec<GenerationDiagnostic>> {
    let mut analysis = ExpressionAnalysis::new(shape);
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
    if let Some(RustValueType::Integer(value)) = &typed_value {
        if i64::try_from(value.minimum()).is_err() || i64::try_from(value.maximum()).is_err() {
            return Err(unsupported_node(
                request,
                expression,
                "the integer type's bounds are not representable as i64",
            ));
        }
    }
    let analyzed = match expression.kind() {
        ExpressionKind::BooleanLiteral { .. } => RustValueType::Boolean,
        ExpressionKind::IntegerLiteral { value, value_type } => {
            if i64::try_from(*value).is_err() {
                return Err(unsupported_node(
                    request,
                    expression,
                    "the integer literal is not representable as i64",
                ));
            }
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
        ExpressionKind::Numeric {
            operator,
            left,
            right,
        } => {
            // The node is refused before its operands are read, so the refusal locus is the
            // first unsupported node in authored preorder.
            if let Some(refusal) = arithmetic_refusal(
                request,
                expression,
                *operator,
                typed_node.value_type(),
                analysis.shape,
            ) {
                return Err(refusal);
            }
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

/// Why an arithmetic node cannot be rendered for `shape`, or `None` when it can (FR-031).
///
/// Divide and remainder are refused under both overflow policies and for every consumer until
/// the IR-601 ruling. Add, subtract and multiply over a `saturate` integer type are refused for
/// every consumer, because the runtime has no saturating operation and clamping inline would be a
/// second implementation of the rule. A consumer that needs a plain `bool` refuses the rest.
fn arithmetic_refusal(
    request: &OracleRequest<'_>,
    expression: &Expression,
    operator: NumericOperator,
    value_type: &ValueType,
    shape: OracleShape,
) -> Option<Vec<GenerationDiagnostic>> {
    let (operation, is_division) = match operator {
        NumericOperator::Add => ("add", false),
        NumericOperator::Subtract => ("subtract", false),
        NumericOperator::Multiply => ("multiply", false),
        NumericOperator::Divide => ("divide", true),
        NumericOperator::Remainder => ("remainder", true),
    };
    let ValueType::Integer { value } = value_type else {
        // A node that is not integer-typed is refused by the grammar checks.
        return None;
    };
    if is_division {
        return Some(expression_diagnostic(
            request,
            GenerationErrorCode::UnsupportedIntegerDivision,
            "expression.node",
            format!(
                "integer {operation} is refused until IR-601 rules which division semantics a V1 \
                 oracle takes and which runtime operation provides them; no raw `/` or `%` is emitted"
            ),
            expression.source(),
        ));
    }
    match value.overflow() {
        OverflowPolicy::Saturate => Some(expression_diagnostic(
            request,
            GenerationErrorCode::UnsupportedSaturatingArithmetic,
            "expression.node",
            format!(
                "integer {operation} over a `saturate` integer type needs a saturating integer \
                 operation in Contract Runtime, which the runtime does not define; the generator \
                 does not restate saturation inline"
            ),
            expression.source(),
        )),
        OverflowPolicy::Reject if shape == OracleShape::PlainBool => Some(unsupported_node(
            request,
            expression,
            format!(
                "integer {operation} needs an outcome-carrying oracle, and this consumer needs a \
                 plain bool"
            ),
        )),
        OverflowPolicy::Reject => None,
    }
}

/// Comment a Kani bundle oracle's file carries when it holds arithmetic (FR-031-AC-19). It holds
/// no `+`, because the exemplar's mutation control counts the additions of the oracle text.
const KANI_ARITHMETIC_NOTE: &str = "// Integer arithmetic in this oracle is checked by Kani, which fails the proof on overflow; this file is not a native evaluator.";

/// The `use`-free helper functions of a native arithmetic oracle, as source lines. `carry` is
/// always present; the others are emitted only when the body calls them.
fn native_helper_lines(helpers: Helpers) -> Vec<String> {
    let mut source = String::from(NATIVE_CARRY_HELPER);
    if helpers.bound {
        source.push_str(NATIVE_BOUND_HELPER);
    }
    if helpers.equality {
        source.push_str(NATIVE_EQUALITY_HELPER);
    }
    source.lines().map(str::to_owned).collect()
}

const NATIVE_CARRY_HELPER: &str = "
/// Splits an outcome into its completed value or the first stop, carried at another type.
fn carry<T, U>(outcome: rt::Outcome<T>) -> Result<T, rt::Outcome<U>> {
    match outcome {
        rt::Outcome::Completed(value) => Ok(value),
        rt::Outcome::Undefined(reason) => Err(rt::Outcome::Undefined(reason)),
        rt::Outcome::Refused(reason) => Err(rt::Outcome::Refused(reason)),
        rt::Outcome::Incomplete(record) => Err(rt::Outcome::Incomplete(record)),
        _ => Err(rt::Outcome::Refused(rt::Refusal::CheckedInvariant)),
    }
}
";

const NATIVE_BOUND_HELPER: &str = "
/// The inclusive result interval of a `reject` integer type.
fn bound<U>(minimum: i64, maximum: i64) -> Result<rt::IntegerInterval, rt::Outcome<U>> {
    rt::IntegerInterval::new(rt::Integer::from(minimum), rt::Integer::from(maximum))
        .map_err(|_| rt::Outcome::Refused(rt::Refusal::CheckedInvariant))
}
";

const NATIVE_EQUALITY_HELPER: &str = "
/// Type-checks then evaluates integer equality through the runtime's equality evaluation.
fn equality(
    operator: rt::EqualityOperator,
    left: rt::Integer,
    right: rt::Integer,
    meter: &mut rt::Meter,
) -> rt::Outcome<bool> {
    let environment = match rt::TypeEnvironment::new(
        core::iter::empty::<rt::CompositeDeclaration>(),
        core::iter::empty::<rt::ObjectTypeDeclaration>(),
    ) {
        Ok(environment) => environment,
        Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),
    };
    let checked = match environment.check_equality(
        operator,
        rt::EqualityOperand::typed(rt::ValueType::Integer),
        rt::EqualityOperand::typed(rt::ValueType::Integer),
    ) {
        Ok(checked) => checked,
        Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),
    };
    checked.evaluate(&rt::Value::Integer(left), &rt::Value::Integer(right), meter)
}
";

/// Whether each node, in authored preorder, holds an integer arithmetic node in its subtree
/// (itself included). Only grammar nodes the analysis admitted are descended into.
fn mark_arithmetic(expression: &Expression, marks: &mut Vec<bool>) -> bool {
    let slot = marks.len();
    marks.push(false);
    let own = matches!(expression.kind(), ExpressionKind::Numeric { .. });
    let below = match expression.kind() {
        ExpressionKind::Boolean { left, right, .. }
        | ExpressionKind::Compare { left, right, .. }
        | ExpressionKind::Numeric { left, right, .. } => {
            let left = mark_arithmetic(left, marks);
            let right = mark_arithmetic(right, marks);
            left || right
        }
        ExpressionKind::BooleanNot { operand } | ExpressionKind::NumericNegate { operand } => {
            mark_arithmetic(operand, marks)
        }
        _ => false,
    };
    if let Some(mark) = marks.get_mut(slot) {
        *mark = own || below;
    }
    own || below
}

fn render_expression(
    request: &OracleRequest<'_>,
    expression: &Expression,
    parameters: &BTreeMap<String, String>,
    shape: OracleShape,
) -> Result<RenderedExpression, Vec<GenerationDiagnostic>> {
    let mut arithmetic = Vec::new();
    let has_arithmetic = mark_arithmetic(expression, &mut arithmetic);
    let mut renderer = Renderer {
        request,
        shape,
        parameters,
        arithmetic,
        next_index: 0,
        output: SourceBuilder::new(),
        helpers: Helpers::default(),
    };
    if shape == OracleShape::Native && has_arithmetic {
        renderer.outcome_bool(expression)?;
    } else {
        renderer.plain_node(expression)?;
    }
    Ok(RenderedExpression {
        source: renderer.output.source,
        implication_regions: renderer.output.implication_regions,
        has_arithmetic,
        helpers: renderer.helpers,
    })
}

/// Renders one clause's expression tree. Every `*_node` and `outcome_*` method consumes one node
/// of authored preorder per call, so `next_index` names the typed node under render.
///
/// The refusals of [`arithmetic_refusal`] are decided once, by the analysis that runs before any
/// rendering. The renderer relies on them and only keeps the shape honest: an infix operator is
/// printed for the Kani bundle shape alone.
struct Renderer<'a> {
    request: &'a OracleRequest<'a>,
    shape: OracleShape,
    parameters: &'a BTreeMap<String, String>,
    arithmetic: Vec<bool>,
    next_index: usize,
    output: SourceBuilder,
    helpers: Helpers,
}

type Rendered = Result<(), Vec<GenerationDiagnostic>>;

impl<'a> Renderer<'a> {
    fn emit(&mut self, text: &str) -> Rendered {
        self.output
            .line(text)
            .map_err(|_| resource_error(self.request))
    }

    /// Whether the node about to be rendered holds an arithmetic node.
    fn holds_arithmetic(&self) -> bool {
        self.arithmetic
            .get(self.next_index)
            .copied()
            .unwrap_or(false)
    }

    /// Takes the typed node under render, advancing to the next in preorder.
    fn take_index(&mut self) -> usize {
        let index = self.next_index;
        self.next_index = self.next_index.saturating_add(1);
        index
    }

    /// The expression as the plain Rust `bool` or `i64` it has always been: native operators, and
    /// for the Kani bundle oracle the infix `+`, `-` and `*` on `i64`.
    fn plain_node(&mut self, expression: &Expression) -> Rendered {
        self.take_index();
        match expression.kind() {
            ExpressionKind::BooleanLiteral { value } => {
                self.emit(if *value { "true" } else { "false" })
            }
            ExpressionKind::IntegerLiteral { value, .. } => {
                self.emit(&integer_literal_source(*value))
            }
            ExpressionKind::ValueReference { name, observation } => {
                let key = reference_key(name.as_str(), Some(*observation));
                let Some(identifier) = self.parameters.get(&key) else {
                    return Err(expression_diagnostic(
                        self.request,
                        GenerationErrorCode::UnsupportedDependency,
                        "expression.value_reference",
                        "typed dependency census does not contain the referenced value",
                        expression.source(),
                    ));
                };
                let identifier = identifier.clone();
                self.emit(&identifier)
            }
            ExpressionKind::BooleanNot { operand } => {
                self.emit("!(")?;
                self.plain_node(operand)?;
                self.emit(")")
            }
            ExpressionKind::Boolean {
                operator,
                left,
                right,
            } => {
                let (function, left_closure) = connective_function(*operator);
                self.emit(&format!("quire_contract_runtime::operators::{function}("))?;
                if left_closure {
                    self.emit("|| {")?;
                }
                self.plain_node(left)?;
                self.emit(if left_closure { "}," } else { "," })?;
                self.emit("|| {")?;
                let region_index = self.open_implication_region(*operator);
                let start_line = self.output.next_line;
                self.plain_node(right)?;
                self.close_implication_region(region_index, start_line);
                self.emit("},")?;
                self.emit(")")
            }
            ExpressionKind::Compare {
                operator,
                left,
                right,
            } => {
                self.emit("(")?;
                self.plain_node(left)?;
                self.emit(")")?;
                self.emit(comparison_source(*operator))?;
                self.emit("(")?;
                self.plain_node(right)?;
                self.emit(")")
            }
            ExpressionKind::Numeric {
                operator,
                left,
                right,
            } => {
                let infix = self.infix_operator(expression, *operator)?;
                self.emit("(")?;
                self.plain_node(left)?;
                self.emit(")")?;
                self.emit(infix)?;
                self.emit("(")?;
                self.plain_node(right)?;
                self.emit(")")
            }
            other => Err(unsupported_kind(self.request, expression, other)),
        }
    }

    /// The infix operator of an add, subtract or multiply, for the Kani bundle oracle only: a
    /// native oracle renders arithmetic as runtime calls and a plain-bool consumer refuses it, so
    /// any other shape reaching a numeric node here is refused rather than printed. Divide and
    /// remainder have no operator, so no raw `/` or `%` is ever rendered.
    fn infix_operator(
        &self,
        expression: &Expression,
        operator: NumericOperator,
    ) -> Result<&'static str, Vec<GenerationDiagnostic>> {
        if self.shape != OracleShape::KaniBundle {
            return Err(unsupported_node(
                self.request,
                expression,
                "integer arithmetic is rendered as an infix operator in the Kani bundle oracle only",
            ));
        }
        match operator {
            NumericOperator::Add => Ok("+"),
            NumericOperator::Subtract => Ok("-"),
            NumericOperator::Multiply => Ok("*"),
            NumericOperator::Divide | NumericOperator::Remainder => Err(unsupported_node(
                self.request,
                expression,
                "integer divide and remainder are refused",
            )),
        }
    }

    /// The checked type of the typed node at `index`.
    fn typed_value_type(
        &self,
        index: usize,
        expression: &Expression,
    ) -> Result<&'a ValueType, Vec<GenerationDiagnostic>> {
        let typed: &'a TypedExpression = self.request.expression;
        typed
            .nodes()
            .get(index)
            .map(|node| node.value_type())
            .ok_or_else(|| {
                expression_diagnostic(
                    self.request,
                    GenerationErrorCode::UnsupportedExpression,
                    "expression.nodes",
                    "typed node census ended before the authored expression tree",
                    expression.source(),
                )
            })
    }

    fn open_implication_region(&mut self, operator: BooleanOperator) -> Option<usize> {
        (operator == BooleanOperator::Implication).then(|| {
            let index = self.output.implication_regions.len();
            self.output.implication_regions.push((0, 0));
            index
        })
    }

    fn close_implication_region(&mut self, region_index: Option<usize>, start_line: u32) {
        let end_line = self.output.next_line.saturating_sub(1);
        if let Some(region) =
            region_index.and_then(|index| self.output.implication_regions.get_mut(index))
        {
            *region = (start_line, end_line);
        }
    }

    /// A Boolean node as an `rt::Outcome<bool>` expression, for the native oracle. A node with no
    /// arithmetic below it is the plain `bool` it has always been, completed.
    fn outcome_bool(&mut self, expression: &Expression) -> Rendered {
        if !self.holds_arithmetic() {
            self.emit("rt::Outcome::Completed(")?;
            self.plain_node(expression)?;
            return self.emit(")");
        }
        self.take_index();
        match expression.kind() {
            ExpressionKind::BooleanNot { operand } => {
                self.emit("match carry(")?;
                self.outcome_bool(operand)?;
                self.emit(") {")?;
                self.emit("Ok(value) => rt::Outcome::Completed(!value),")?;
                self.emit("Err(stop) => stop,")?;
                self.emit("}")
            }
            ExpressionKind::Boolean {
                operator,
                left,
                right,
            } => self.outcome_connective(*operator, left, right),
            ExpressionKind::Compare {
                operator,
                left,
                right,
            } => {
                self.emit("match carry(")?;
                self.outcome_integer(left)?;
                self.emit(") {")?;
                self.emit("Err(stop) => stop,")?;
                self.emit("Ok(left_value) => match carry(")?;
                self.outcome_integer(right)?;
                self.emit(") {")?;
                self.emit("Err(stop) => stop,")?;
                let call = self.comparison_call(*operator);
                self.emit(&format!("Ok(right_value) => {call},"))?;
                self.emit("},")?;
                self.emit("}")
            }
            other => Err(unsupported_kind(self.request, expression, other)),
        }
    }

    /// A connective with arithmetic below it. A short-circuit connective calls the runtime's
    /// operator, so its right operand is a closure the left operand's decision never reaches. A
    /// total connective evaluates both operands left to right and takes the first stop.
    fn outcome_connective(
        &mut self,
        operator: BooleanOperator,
        left: &Expression,
        right: &Expression,
    ) -> Rendered {
        let (function, total) = connective_function(operator);
        if total {
            self.emit("match (carry(")?;
            self.outcome_bool(left)?;
            self.emit("), carry(")?;
            self.outcome_bool(right)?;
            self.emit(")) {")?;
            self.emit(&format!(
                "(Ok(left_value), Ok(right_value)) => rt::Outcome::Completed(quire_contract_runtime::operators::{function}(|| left_value, || right_value)),"
            ))?;
            self.emit("(Err(stop), _) | (_, Err(stop)) => stop,")?;
            return self.emit("}");
        }
        let left_stops = self.holds_arithmetic();
        if left_stops {
            self.emit("match carry(")?;
            self.outcome_bool(left)?;
            self.emit(") {")?;
            self.emit(&format!(
                "Ok(left_value) => quire_contract_runtime::operators::{function}(left_value, || {{"
            ))?;
        } else {
            self.emit(&format!("quire_contract_runtime::operators::{function}("))?;
            self.plain_node(left)?;
            self.emit(",")?;
            self.emit("|| {")?;
        }
        let region_index = self.open_implication_region(operator);
        let start_line = self.output.next_line;
        self.outcome_bool(right)?;
        self.close_implication_region(region_index, start_line);
        if left_stops {
            self.emit("}),")?;
            self.emit("Err(stop) => stop,")?;
            self.emit("}")
        } else {
            self.emit("},")?;
            self.emit(")")
        }
    }

    /// The runtime call that orders or equates the two completed integers `left_value` and
    /// `right_value`.
    fn comparison_call(&mut self, operator: ComparisonOperator) -> String {
        let ordering = match operator {
            ComparisonOperator::Equal | ComparisonOperator::NotEqual => {
                self.helpers.equality = true;
                let equality = if operator == ComparisonOperator::Equal {
                    "Equal"
                } else {
                    "NotEqual"
                };
                return format!(
                    "equality(rt::EqualityOperator::{equality}, left_value, right_value, meter)"
                );
            }
            ComparisonOperator::Less => "Less",
            ComparisonOperator::LessEqual => "LessOrEqual",
            ComparisonOperator::Greater => "Greater",
            ComparisonOperator::GreaterEqual => "GreaterOrEqual",
        };
        format!(
            "rt::order_numbers(rt::OrderingOperator::{ordering}, rt::OrderedOperands::Integers(&left_value, &right_value), meter)"
        )
    }

    /// An integer node as an `rt::Outcome<rt::Integer>` expression, for the native oracle. An
    /// operand with no arithmetic below it is converted from its `i64` parameter or literal; an
    /// arithmetic operand is the runtime's exact operation over the completed integers of its two
    /// operands, bounded by the node's interval.
    fn outcome_integer(&mut self, expression: &Expression) -> Rendered {
        if !self.holds_arithmetic() {
            self.emit("rt::Outcome::Completed(rt::Integer::from(")?;
            self.plain_node(expression)?;
            return self.emit("))");
        }
        let index = self.take_index();
        let ExpressionKind::Numeric {
            operator,
            left,
            right,
        } = expression.kind()
        else {
            return Err(unsupported_kind(
                self.request,
                expression,
                expression.kind(),
            ));
        };
        let value_type = self.typed_value_type(index, expression)?;
        let ValueType::Integer { value: interval } = value_type else {
            return Err(unsupported_node(
                self.request,
                expression,
                "arithmetic requires a bounded integer type",
            ));
        };
        let bound = format!(
            "bound({}, {})",
            integer_literal_source(interval.minimum()),
            integer_literal_source(interval.maximum())
        );
        let variant = match operator {
            NumericOperator::Add => "Add",
            NumericOperator::Subtract => "Subtract",
            NumericOperator::Multiply => "Multiply",
            NumericOperator::Divide | NumericOperator::Remainder => {
                return Err(unsupported_node(
                    self.request,
                    expression,
                    "integer divide and remainder are refused",
                ));
            }
        };
        self.helpers.bound = true;
        self.emit("match carry(")?;
        self.outcome_integer(left)?;
        self.emit(") {")?;
        self.emit("Err(stop) => stop,")?;
        self.emit("Ok(left_value) => match carry(")?;
        self.outcome_integer(right)?;
        self.emit(") {")?;
        self.emit("Err(stop) => stop,")?;
        self.emit(&format!("Ok(right_value) => match {bound} {{"))?;
        self.emit("Err(stop) => stop,")?;
        self.emit("Ok(interval) => rt::evaluate_integer_arithmetic(")?;
        self.emit(&format!(
            "rt::IntegerArithmetic::{variant}(&left_value, &right_value),"
        ))?;
        self.emit("Some(&interval),")?;
        self.emit("meter,")?;
        self.emit("),")?;
        self.emit("},")?;
        self.emit("},")?;
        self.emit("}")
    }
}

fn integer_literal_source(value: i128) -> String {
    if value == i128::from(i64::MIN) {
        "i64::MIN".to_owned()
    } else {
        format!("{value}_i64")
    }
}

fn comparison_source(operator: ComparisonOperator) -> &'static str {
    match operator {
        ComparisonOperator::Equal => "==",
        ComparisonOperator::NotEqual => "!=",
        ComparisonOperator::Less => "<",
        ComparisonOperator::LessEqual => "<=",
        ComparisonOperator::Greater => ">",
        ComparisonOperator::GreaterEqual => ">=",
    }
}

/// The runtime operator that decides a connective, and whether its left operand is a closure.
fn connective_function(operator: BooleanOperator) -> (&'static str, bool) {
    match operator {
        BooleanOperator::ShortCircuitAnd => ("and_short_circuit", false),
        BooleanOperator::ShortCircuitOr => ("or_short_circuit", false),
        BooleanOperator::TotalAnd => ("and_total", true),
        BooleanOperator::TotalOr => ("or_total", true),
        BooleanOperator::Implication => ("implies_short_circuit", false),
    }
}

fn unsupported_kind(
    request: &OracleRequest<'_>,
    expression: &Expression,
    kind: &ExpressionKind,
) -> Vec<GenerationDiagnostic> {
    expression_diagnostic(
        request,
        GenerationErrorCode::UnsupportedExpression,
        "expression.node",
        format!(
            "unsupported expression in oracle slice: {}",
            node_name(kind)
        ),
        expression.source(),
    )
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

fn artifact(path: String, contents: String) -> Artifact {
    Artifact::new(path, contents)
}

fn deterministic_json(value: &impl Serialize) -> Result<String, SerializationError> {
    let mut bytes = serde_json::to_vec(value).map_err(SerializationError::Json)?;
    bytes.push(b'\n');
    String::from_utf8(bytes).map_err(SerializationError::Utf8)
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

// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Lower and check scalar claims against admitted checked nodes.

use super::derive::{
    conversion_target, derive_operation_from_view, quantity_conversion_target_is_unit, Derivation,
};
use super::{
    BooleanOperator, BoundForm, DecimalOperator, EqualityOperandKind, ExactScalarOperation,
    ExactScalarRefusal, GeneratedScalarClaim, IeeeArithmeticOperator, OperandRangeLimitKind,
    OperationClaim, OperationProvenance, OrderingOperandKind, QuantityOperator, RationalOperator,
    ScalarForm, QUANTITY_CONVERT_IDENTITY,
};
use crate::core::naming::bounded_readable_component;
use crate::oracle::checked_graph::{term_name, CheckedGraph as Graph, GraphBody};
use crate::oracle::claim::{ClaimDisposition, OracleGenerationError, UpstreamBlocker};
use crate::oracle::operator_catalog::{IntegerOperator, Operator as CatalogOperator};
use crate::oracle::{classify_lowering_failure, direct_unsupported_form, LoweringFailure};
use num_bigint::BigInt;
use quire_contract_model::{
    ApplicationOperator, BoundedDomainForm, CheckedNodeId, CheckedNodeTag, CheckedOccurrenceRole,
    CheckedSemanticNodeV2, CheckedTerm, CompleteContractNodeV2, CompleteLoweringRecordV2,
    ContractOccurrenceKeyV1, ContractPackageV1, ContractResultContextV1, ContractResultViewV1,
};
use quire_exact::{
    ComparisonOperator, IeeeComparison, IeeeWidth, IntegerDomain, IntegerInterval,
    OrderingOperator, RoundingMode,
};
use quire_semantic_value::declaration::EqualityOperator;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

use super::{
    ExactScalarClaim, ExactScalarItem, ExactScalarPlan, PendingOraclePlan,
    SCALAR_LOWERING_SUPPORTED_TAGS, SCALAR_LOWERING_WORK_LIMIT,
};
use crate::core::contract_package::{
    ContractDriverResultSelection, ContractRequestPosition, PackageItem, PackageItems,
    PackageRequestError,
};
use crate::core::ir::checked_term::{aggregate_members, literal_integer};
use crate::core::ir::scalar_bounds::{
    read_decimal_range, read_float_rounding, read_integer_range, read_rational_range,
    read_text_bounds,
};
use quire_contract_model::{CheckedPackageV2, CompleteLoweringProfileV2, ContractRootViewError};

/// The lowered node and the bounds its descriptor was checked against.
type CheckedItem<'r> = (&'r CompleteContractNodeV2, Vec<CheckedNodeId>);

/// Either [`ExactScalarRefusal`] (a per-item business refusal, folded into
/// the claim map) or [`OracleGenerationError`] (an RT `#[non_exhaustive]`
/// enum yielding a variant this generator's own match (in `check_parameters`
/// or one of the other scalar sites) does not know -- see [`OracleGenerationError::UnknownRuntimeVariant`]'s own doc for
/// why that aborts the whole generation instead of refusing one item).
pub(super) enum ItemCheckError {
    Refusal(ExactScalarRefusal),
    Generation(OracleGenerationError),
}

impl From<ExactScalarRefusal> for ItemCheckError {
    fn from(refusal: ExactScalarRefusal) -> Self {
        Self::Refusal(refusal)
    }
}

pub(super) fn check_item<'r, Body: GraphBody>(
    graph: &Graph<'_, Body>,
    requested_node_id: &CheckedNodeId,
    record: &'r CompleteLoweringRecordV2,
    operation: &ExactScalarOperation,
) -> Result<CheckedItem<'r>, ItemCheckError> {
    if let Some(node) = graph.node(requested_node_id) {
        if let Some(node_tag) = direct_unsupported_form(node, graph.term(requested_node_id)) {
            return Err(ExactScalarRefusal::Unsupported {
                unsupported_node_id: requested_node_id.clone(),
                node_tag,
            }
            .into());
        }
    }
    if let CompleteLoweringRecordV2::Unsupported {
        unsupported_node_id,
        ..
    } = record
    {
        if let Some(node) = graph.node(unsupported_node_id) {
            if let Some(node_tag) = direct_unsupported_form(node, graph.term(unsupported_node_id)) {
                return Err(ExactScalarRefusal::Unsupported {
                    unsupported_node_id: unsupported_node_id.clone(),
                    node_tag,
                }
                .into());
            }
        }
    }
    let node = lowered(record)?;
    if node.node_tag != CheckedNodeTag::Expression {
        return Err(ExactScalarRefusal::NotExpression {
            node_tag: node.node_tag.as_wire(),
        }
        .into());
    }
    if matches!(operation, ExactScalarOperation::QuantityConversion) {
        let derivation = Derivation {
            graph,
            node: &node.node,
            bounds: BoundView::complete(node),
            identity: QUANTITY_CONVERT_IDENTITY,
        };
        derivation.require_mode(RoundingMode::Exact)?;
        if !quantity_conversion_target_is_unit(graph, &node.node) {
            return Err(derivation.operand_forms_not_derivable().into());
        }
    }
    if let ExactScalarOperation::Boolean { right_may_stop, .. } = operation {
        let derivation = Derivation {
            graph,
            node: &node.node,
            bounds: BoundView::complete(node),
            identity: "",
        };
        let actual_may_stop = derivation.right_may_stop();
        if *right_may_stop != actual_may_stop {
            return Err(ExactScalarRefusal::BooleanRightOperandMismatch {
                declared_may_stop: *right_may_stop,
                actual_may_stop,
            }
            .into());
        }
    }
    let shape = Shape::of(operation).map_err(ItemCheckError::Generation)?;
    if &*node.node.semantic_form != shape.form {
        return Err(ExactScalarRefusal::FormMismatch {
            expected: shape.form,
            found: node.node.semantic_form.to_string(),
        }
        .into());
    }
    let arguments = application_arguments(graph.term(&node.node.node_id), shape.body_operator)
        .filter(|arguments| arguments.len() == shape.operands.len())
        .ok_or(ExactScalarRefusal::BodyMismatch {
            expected_operator: shape.body_operator,
            expected_arguments: shape.operands.len(),
        })?;
    let result = scalar_type_form(graph, &node.semantic_type);
    if result.as_deref() != Some(shape.result.wire()) {
        return Err(ExactScalarRefusal::ResultTypeMismatch {
            expected: shape.result,
            found: result,
        }
        .into());
    }
    for (position, (argument, expected)) in arguments.iter().zip(shape.operands).enumerate() {
        check_operand(graph, position, argument, *expected)?;
    }
    let checked_bounds = check_parameters(graph, node, operation)?;
    Ok((node, checked_bounds))
}

pub(super) fn lowered(
    record: &CompleteLoweringRecordV2,
) -> Result<&CompleteContractNodeV2, ExactScalarRefusal> {
    match record {
        CompleteLoweringRecordV2::Lowered { node } => Ok(node),
        CompleteLoweringRecordV2::Unsupported {
            unsupported_node_id,
            node_tag,
            ..
        } => Err(unsupported_family(unsupported_node_id, *node_tag)),
        CompleteLoweringRecordV2::RequiresBound { unbounded_type, .. } => {
            Err(ExactScalarRefusal::RequiresBound {
                unbounded_type: unbounded_type.clone(),
            })
        }
        CompleteLoweringRecordV2::InvalidInput { .. } => Err(ExactScalarRefusal::InvalidInput),
        CompleteLoweringRecordV2::InvalidBody { body_node_id, .. } => {
            Err(ExactScalarRefusal::InvalidBody {
                body_node_id: body_node_id.clone(),
            })
        }
        CompleteLoweringRecordV2::BodyIncomplete { body_node_id, .. } => {
            Err(ExactScalarRefusal::BodyIncomplete {
                body_node_id: body_node_id.clone(),
            })
        }
        // FR-014-AC-42 keeps this arm from binding the record's fields, so the typed outcome is
        // read back through the classifier, which returns `None` for a record that is not
        // `Failed`. This arm only runs for a `Failed` record, so the `InvalidInput` fallback is
        // unreachable; it exists because the arm cannot hand the classifier a narrower type.
        CompleteLoweringRecordV2::Failed { .. } => Err(classify_lowering_failure(record)
            .map_or(ExactScalarRefusal::InvalidInput, ExactScalarRefusal::from)),
    }
}

impl From<LoweringFailure> for ExactScalarRefusal {
    fn from(failure: LoweringFailure) -> Self {
        match failure {
            LoweringFailure::WorkExhausted { limit, consumed } => {
                Self::LoweringWorkExhausted { limit, consumed }
            }
            LoweringFailure::ByteLimitExceeded { limit, consumed } => {
                Self::LoweringByteLimitExceeded { limit, consumed }
            }
            LoweringFailure::LimitUnrecognised {
                limit_kind,
                limit,
                consumed,
            } => Self::LoweringLimitUnrecognised {
                limit_kind,
                limit,
                consumed,
            },
        }
    }
}

pub(super) fn unsupported_family(
    node_id: &CheckedNodeId,
    tag: CheckedNodeTag,
) -> ExactScalarRefusal {
    let blocked = |issue| ExactScalarRefusal::BlockedOnUpstream {
        unsupported_node_id: node_id.clone(),
        node_tag: tag.as_wire(),
        issue,
    };
    match tag {
        CheckedNodeTag::Function => blocked(UpstreamBlocker::QuireContractRuntime34),
        CheckedNodeTag::Model | CheckedNodeTag::Relation => {
            blocked(UpstreamBlocker::QuireSpecLanguage120)
        }
        CheckedNodeTag::State
        | CheckedNodeTag::Temporal
        | CheckedNodeTag::Protocol
        | CheckedNodeTag::TimedModel
        | CheckedNodeTag::ScalarType
        | CheckedNodeTag::CompositeType
        | CheckedNodeTag::BoundedDomain
        | CheckedNodeTag::Value
        | CheckedNodeTag::Expression
        | CheckedNodeTag::Claim
        | CheckedNodeTag::Correspondence
        | CheckedNodeTag::Extension
        | CheckedNodeTag::Hybrid => ExactScalarRefusal::Unsupported {
            unsupported_node_id: node_id.clone(),
            node_tag: tag.as_wire(),
        },
    }
}

pub(super) fn application_arguments<'a>(
    body: impl Into<Option<&'a CheckedTerm>>,
    expected: &str,
) -> Option<&'a [CheckedTerm]> {
    match body.into()? {
        CheckedTerm::Application {
            operator,
            arguments,
            ..
        } if operator.as_wire() == expected => Some(arguments),
        _ => None,
    }
}

fn type_form<Body: GraphBody>(graph: &Graph<'_, Body>, type_id: &CheckedNodeId) -> Option<String> {
    let node = graph.node(type_id)?;
    (&*node.node_tag == CheckedNodeTag::ScalarType.as_wire())
        .then(|| node.semantic_form.to_string())
}

/// An operand is a literal classified by its value kind, or a reference
/// classified by its target's scalar type. A literal carries no unit, so it is
/// never a quantity; any other term is refused as unsupported.
pub(super) fn check_operand<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    position: usize,
    term: &CheckedTerm,
    expected: ScalarForm,
) -> Result<(), ExactScalarRefusal> {
    let found = match term {
        CheckedTerm::Literal { .. } if expected == ScalarForm::Unit => {
            return Err(ExactScalarRefusal::UnitlessLiteralOperand { position });
        }
        CheckedTerm::Literal { .. } => literal_form(term),
        CheckedTerm::Reference { .. } => reference_form(graph, term),
        other => {
            return Err(ExactScalarRefusal::OperandUnsupported {
                position,
                term: term_name(other).to_owned(),
            });
        }
    };
    if found.as_deref() == Some(expected.wire()) {
        Ok(())
    } else {
        Err(ExactScalarRefusal::OperandTypeMismatch {
            position,
            expected,
            found,
        })
    }
}

fn literal_form(term: &CheckedTerm) -> Option<String> {
    let CheckedTerm::Literal { value_kind, .. } = term else {
        return None;
    };
    ScalarForm::from_literal_kind(*value_kind).map(|form| form.wire().to_owned())
}

/// Classify only the owner's literal and reference variants.
pub(super) fn operand_form<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    term: &CheckedTerm,
) -> Option<String> {
    match term {
        CheckedTerm::Literal { .. } => literal_form(term),
        CheckedTerm::Reference { .. } => reference_form(graph, term),
        _ => None,
    }
}

fn reference_form<Body: GraphBody>(graph: &Graph<'_, Body>, term: &CheckedTerm) -> Option<String> {
    let target = reference_target(term)?;
    scalar_type_form(graph, &graph.node(&target)?.semantic_type)
}

/// The scalar form of `type_id`, following one `bounded_domain` step. A parameter typed
/// `Int[0, 9]` is typed by a `bounded_domain` whose own `semantic_type` is its base scalar type:
/// QSL emits a bounded domain directly over its scalar base, so one step reaches the type the form
/// is read from. A node's result typed by a bounded domain reads the same way.
pub(super) fn scalar_type_form<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    type_id: &CheckedNodeId,
) -> Option<String> {
    let node = graph.node(type_id)?;
    if is_bounded_domain(node) {
        type_form(graph, &node.semantic_type)
    } else {
        type_form(graph, type_id)
    }
}

fn is_bounded_domain<Body>(node: &CheckedSemanticNodeV2<Body>) -> bool {
    &*node.node_tag == CheckedNodeTag::BoundedDomain.as_wire()
}

/// The `bounded_domain` node that types a `reference` operand's target: the operand's own bound.
/// A literal, a reference typed by a scalar type, or any other term has none.
fn operand_own_bound<'g, Body: GraphBody>(
    graph: &Graph<'g, Body>,
    term: &CheckedTerm,
) -> Option<&'g CheckedSemanticNodeV2<Body>> {
    let target = reference_target(term)?;
    let typing = graph.node(&graph.node(&target)?.semantic_type)?;
    is_bounded_domain(typing).then_some(typing)
}

/// The type of a `reference` operand that no `bounded_domain` types; `None` for a literal, a
/// reference to a literal value node, or an operand with an own bound.
///
/// QSL emits a literal operand as a `reference` to its own `value` node, whose body is the
/// literal and whose type is the plain scalar type. That is a constant, like an inline literal,
/// and is decided by the referenced node's body, never by its type: a `value` node with any other
/// body (a parameter) and a plain type is an unbounded operand.
fn plain_reference_type<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    term: &CheckedTerm,
) -> Option<CheckedNodeId> {
    if operand_own_bound(graph, term).is_some() {
        return None;
    }
    let node = graph.node(&reference_target(term)?)?;
    (!is_literal_value(graph, node)).then(|| node.semantic_type.clone())
}

/// A `value` node whose body is a literal: the node QSL emits for a literal operand.
fn is_literal_value<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node: &CheckedSemanticNodeV2<Body>,
) -> bool {
    &*node.node_tag == CheckedNodeTag::Value.as_wire()
        && matches!(graph.term(&node.node_id), Some(CheckedTerm::Literal { .. }))
}

/// The catalogued identity of the narrowing conversion QSL wraps a bounded-context expression in.
pub(super) const NARROW_IDENTITY: &str = "quire.op.numeric.narrow";

/// How the nodes that consume a scalar-typed node bound its result.
pub(super) enum Narrowing<'g, Body> {
    /// No narrowing conversion consumes the node.
    Unnarrowed,
    /// Only narrowing conversions consume the node: the `bounded_domain` nodes they are typed by,
    /// ascending and deduplicated (empty where none is typed by a `bounded_domain`).
    To(Vec<&'g CheckedSemanticNodeV2<Body>>),
    /// A narrowing conversion consumes the node and so does a node that is not one: the narrow's
    /// bound does not bound every use of the node's result.
    Mixed,
}

impl<Body> Narrowing<'_, Body> {
    /// Whether a consuming conversion narrows the node's result, so the package bounds it.
    fn narrowed(&self) -> bool {
        match self {
            Self::Unnarrowed => false,
            Self::To(bounds) => !bounds.is_empty(),
            Self::Mixed => true,
        }
    }
}

/// Whether `consumer` is a narrowing conversion of the node `id`: an `expression` of form
/// `conversion` whose body is a `convert` application with the catalogued identity
/// `quire.op.numeric.narrow` and exactly one argument, a `reference` to `id`.
pub(super) fn narrows<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    consumer: &CheckedSemanticNodeV2<Body>,
    id: &CheckedNodeId,
) -> bool {
    &*consumer.node_tag == CheckedNodeTag::Expression.as_wire()
        && &*consumer.semantic_form == "conversion"
        && graph
            .operation(&consumer.node_id)
            .and_then(|op| op.get("identity"))
            .and_then(Value::as_str)
            == Some(NARROW_IDENTITY)
        && application_arguments(graph.term(&consumer.node_id), "convert").is_some_and(
            |arguments| matches!(arguments, [only] if reference_target(only).as_ref() == Some(id)),
        )
}

/// Walk only the reader-admitted term structure; operation payloads are not term bodies.
pub(super) fn references(term: Option<&CheckedTerm>, id: &CheckedNodeId) -> bool {
    match term {
        Some(CheckedTerm::Reference { target }) => target == id,
        Some(CheckedTerm::Application { arguments, .. }) => {
            arguments.iter().any(|term| references(Some(term), id))
        }
        Some(CheckedTerm::Aggregate { members }) => {
            members.iter().any(|term| references(Some(term), id))
        }
        Some(CheckedTerm::Binding { value, .. }) => references(Some(value), id),
        Some(CheckedTerm::ExtensionForm { clauses, .. }) => {
            clauses.iter().any(|term| references(Some(term), id))
        }
        Some(
            CheckedTerm::Literal { .. }
            | CheckedTerm::DependencyReference { .. }
            | CheckedTerm::Frame { .. }
            | CheckedTerm::AbstractionRelation { .. },
        )
        | None => false,
    }
}

/// How the consumers of the node `id` narrow it. A consumer is any other node whose body holds a
/// `reference` to `id`; a narrowing conversion is one [`narrows`] accepts.
pub(super) fn narrowing<'g, Body: GraphBody>(
    graph: &Graph<'g, Body>,
    id: &CheckedNodeId,
) -> Narrowing<'g, Body> {
    let mut bounds = BTreeMap::new();
    let (mut narrowed, mut other) = (false, false);
    for consumer in graph.nodes() {
        if &consumer.node_id == id || !references(graph.term(&consumer.node_id), id) {
            continue;
        }
        if !narrows(graph, consumer, id) {
            other = true;
            continue;
        }
        narrowed = true;
        if let Some(bound) = graph
            .node(&consumer.semantic_type)
            .filter(|typing| is_bounded_domain(typing))
        {
            bounds.insert(&bound.node_id, bound);
        }
    }
    match (narrowed, other) {
        (false, _) => Narrowing::Unnarrowed,
        (true, true) => Narrowing::Mixed,
        (true, false) => Narrowing::To(bounds.into_values().collect()),
    }
}

/// The node a `reference` term names.
fn reference_target(term: &CheckedTerm) -> Option<CheckedNodeId> {
    match term {
        CheckedTerm::Reference { target } => Some(target.clone()),
        _ => None,
    }
}

fn own_bounds<'a, Body: GraphBody>(
    graph: &Graph<'a, Body>,
    body: Option<&CheckedTerm>,
) -> Vec<Option<&'a CheckedSemanticNodeV2<Body>>> {
    let arguments = match body {
        Some(CheckedTerm::Application { arguments, .. }) => arguments.as_slice(),
        _ => &[],
    };
    arguments
        .iter()
        .map(|argument| operand_own_bound(graph, argument))
        .collect()
}

fn operand_literal<'a, Body: GraphBody>(
    graph: &Graph<'a, Body>,
    term: &'a CheckedTerm,
) -> Option<&'a CheckedTerm> {
    if matches!(term, CheckedTerm::Literal { .. }) {
        return Some(term);
    }
    let node = graph.node(&reference_target(term)?)?;
    is_literal_value(graph, node)
        .then(|| graph.term(&node.node_id))
        .flatten()
}

/// How a Kani harness ranges one operand of a scalar node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum OperandRange {
    /// Over the operand's own `bounded_domain`, the node named.
    Bound(CheckedNodeId),
    /// Exactly the operand's own literal value, its spelling as the package carries it.
    Literal(String),
    /// Over the result bound: an operand that is neither (the shape IR-299 tracks).
    Result,
    /// Exact range recursively inferred from this referenced application tree.
    Inferred {
        source: CheckedNodeId,
        lower: String,
        upper: String,
    },
}

/// How a Kani harness ranges each of the node's arguments, by position: over the
/// `bounded_domain` typing the argument, exactly at the value of a literal argument (inline or a
/// reference to a literal `value` node), or over the result bound otherwise. Reads the same graph
/// [`super::generate_exact_scalar_oracles`] checked, so a Kani harness ranges each operand as the
/// oracle's parameter check did. A claim records the own-bound ids in `checked_bounds` but not
/// their positions, so the harness takes the positions from here and requires every id to appear
/// in the claim's `checked_bounds` (`lower_scalar_claim`): the two sources cannot silently
/// disagree.
pub(crate) fn operand_ranges(graph: &Graph<'_>, node_id: &CheckedNodeId) -> Vec<OperandRange> {
    let Some(node) = graph.node(node_id) else {
        return Vec::new();
    };
    graph
        .arguments(&node.node_id)
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(position, argument)| {
            if let Some(bound) = operand_own_bound(graph, argument) {
                OperandRange::Bound(bound.node_id.clone())
            } else if let Some(literal) = operand_literal(graph, argument) {
                OperandRange::Literal(
                    match literal {
                        CheckedTerm::Literal { value, .. } => value.as_str().unwrap_or_default(),
                        _ => "",
                    }
                    .to_owned(),
                )
            } else if let Some(source) = reference_target(argument) {
                infer_integer_range(graph, &source, position).ok().map_or(
                    OperandRange::Result,
                    |(lower, upper)| OperandRange::Inferred {
                        source,
                        lower,
                        upper,
                    },
                )
            } else {
                OperandRange::Result
            }
        })
        .collect()
}

/// Infer an exact integer interval from canonical literals, bounded integer parameters, and the
/// four admitted integer operations. The recursion is deliberately limited to plain-Integer
/// application results: an operand's own bounded type is handled by `operand_own_bound`.
const OPERAND_RANGE_MAX_DEPTH: u64 = 64;
const OPERAND_RANGE_MAX_WORK: u64 = 4096;
const OPERAND_RANGE_MAX_ENDPOINT_BITS: u64 = 4096;

pub(super) fn infer_integer_range<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node_id: &CheckedNodeId,
    position: usize,
) -> Result<(String, String), ExactScalarRefusal> {
    fn visit<Body: GraphBody>(
        graph: &Graph<'_, Body>,
        node_id: &CheckedNodeId,
        position: usize,
        active: &mut BTreeSet<CheckedNodeId>,
        memo: &mut BTreeMap<CheckedNodeId, (BigInt, BigInt)>,
        application_depth: u64,
        work: &mut u64,
    ) -> Result<(BigInt, BigInt), ExactScalarRefusal> {
        let node = graph
            .node(node_id)
            .ok_or_else(|| ExactScalarRefusal::InvalidBody {
                body_node_id: node_id.clone(),
            })?;
        let is_application = &*node.node_tag == CheckedNodeTag::Expression.as_wire();
        let application_depth = if is_application {
            application_depth + 1
        } else {
            application_depth
        };
        if is_application && application_depth > OPERAND_RANGE_MAX_DEPTH {
            return Err(ExactScalarRefusal::OperandRangeLimitExceeded {
                node_id: node_id.clone(),
                resource_kind: OperandRangeLimitKind::Depth,
                limit: OPERAND_RANGE_MAX_DEPTH,
                consumed: application_depth,
            });
        }
        if let Some(range) = memo.get(node_id) {
            return Ok(range.clone());
        }
        *work += 1;
        if *work > OPERAND_RANGE_MAX_WORK {
            return Err(ExactScalarRefusal::OperandRangeLimitExceeded {
                node_id: node_id.clone(),
                resource_kind: OperandRangeLimitKind::Nodes,
                limit: OPERAND_RANGE_MAX_WORK,
                consumed: *work,
            });
        }
        if !active.insert(node_id.clone()) {
            return Err(ExactScalarRefusal::InvalidBody {
                body_node_id: node_id.clone(),
            });
        }
        let result = if is_literal_value(graph, node) {
            let literal = graph
                .term(&node.node_id)
                .and_then(literal_integer)
                .ok_or_else(|| ExactScalarRefusal::OperandTypeMismatch {
                    position,
                    expected: ScalarForm::Integer,
                    found: graph.term(&node.node_id).and_then(literal_form),
                })?;
            let value = literal.to_string().parse::<BigInt>().map_err(|_| {
                ExactScalarRefusal::InvalidBody {
                    body_node_id: node_id.clone(),
                }
            })?;
            (value.clone(), value)
        } else if let Some(typing) = graph.node(&node.semantic_type) {
            if is_bounded_domain(typing) {
                let range =
                    read_integer_range(aggregate_members(graph.term(&typing.node_id)).ok_or_else(
                        || ExactScalarRefusal::InvalidBody {
                            body_node_id: typing.node_id.clone(),
                        },
                    )?)
                    .ok_or_else(|| ExactScalarRefusal::InvalidBody {
                        body_node_id: typing.node_id.clone(),
                    })?;
                (
                    range.lower().to_string().parse().map_err(|_| {
                        ExactScalarRefusal::InvalidBody {
                            body_node_id: typing.node_id.clone(),
                        }
                    })?,
                    range.upper().to_string().parse().map_err(|_| {
                        ExactScalarRefusal::InvalidBody {
                            body_node_id: typing.node_id.clone(),
                        }
                    })?,
                )
            } else if &*node.node_tag == CheckedNodeTag::Value.as_wire()
                && scalar_type_form(graph, &node.semantic_type).as_deref() == Some("integer")
            {
                return Err(ExactScalarRefusal::RequiresBound {
                    unbounded_type: node.semantic_type.clone(),
                });
            } else if &*node.node_tag == CheckedNodeTag::Expression.as_wire()
                && scalar_type_form(graph, &node.semantic_type).as_deref() == Some("integer")
            {
                let operation = graph
                    .operation(&node.node_id)
                    .and_then(|v| v.get("identity"))
                    .and_then(Value::as_str)
                    .ok_or_else(|| ExactScalarRefusal::InvalidBody {
                        body_node_id: node_id.clone(),
                    })?;
                let integer_operator = CatalogOperator::from_identity(operation)
                    .and_then(|catalogued| catalogued.entry().scalar_operator)
                    .ok_or_else(|| ExactScalarRefusal::OperandUnsupported {
                        position,
                        term: operation.to_owned(),
                    })?;
                let args = graph.arguments(&node.node_id).ok_or_else(|| {
                    ExactScalarRefusal::InvalidBody {
                        body_node_id: node_id.clone(),
                    }
                })?;
                let (expected_operator, expected_arity) = match integer_operator {
                    IntegerOperator::Add
                    | IntegerOperator::Subtract
                    | IntegerOperator::Multiply => ("binary", 2),
                    IntegerOperator::Negate => ("unary", 1),
                };
                if graph
                    .operator(&node.node_id)
                    .map(ApplicationOperator::as_wire)
                    != Some(expected_operator)
                    || args.len() != expected_arity
                {
                    return Err(ExactScalarRefusal::InvalidBody {
                        body_node_id: node_id.clone(),
                    });
                }
                let mut ranges = Vec::with_capacity(args.len());
                for arg in args {
                    if matches!(arg, CheckedTerm::Literal { .. }) {
                        let literal = literal_integer(arg).ok_or_else(|| {
                            ExactScalarRefusal::OperandTypeMismatch {
                                position,
                                expected: ScalarForm::Integer,
                                found: literal_form(arg),
                            }
                        })?;
                        let value = literal.to_string().parse::<BigInt>().map_err(|_| {
                            ExactScalarRefusal::InvalidBody {
                                body_node_id: node_id.clone(),
                            }
                        })?;
                        check_operand_endpoint_bits(node_id, &value, &value)?;
                        ranges.push((value.clone(), value));
                        continue;
                    }
                    let Some(target) = reference_target(arg) else {
                        return Err(ExactScalarRefusal::OperandUnsupported {
                            position,
                            term: term_name(arg).to_owned(),
                        });
                    };
                    ranges.push(visit(
                        graph,
                        &target,
                        position,
                        active,
                        memo,
                        application_depth,
                        work,
                    )?);
                }
                match (integer_operator, ranges.as_slice()) {
                    (IntegerOperator::Add, [(al, au), (bl, bu)]) => (al + bl, au + bu),
                    (IntegerOperator::Subtract, [(al, au), (bl, bu)]) => (al - bu, au - bl),
                    (IntegerOperator::Multiply, [(al, au), (bl, bu)]) => {
                        let products = [al * bl, al * bu, au * bl, au * bu];
                        let mut products = products.into_iter();
                        let first =
                            products
                                .next()
                                .ok_or_else(|| ExactScalarRefusal::InvalidBody {
                                    body_node_id: node_id.clone(),
                                })?;
                        products.fold((first.clone(), first), |(lower, upper), product| {
                            (lower.min(product.clone()), upper.max(product))
                        })
                    }
                    (IntegerOperator::Negate, [(lower, upper)]) => (-upper, -lower),
                    _ => {
                        return Err(ExactScalarRefusal::InvalidBody {
                            body_node_id: node_id.clone(),
                        });
                    }
                }
            } else {
                return Err(ExactScalarRefusal::OperandTypeMismatch {
                    position,
                    expected: ScalarForm::Integer,
                    found: scalar_type_form(graph, &node.semantic_type),
                });
            }
        } else if &*node.node_tag == CheckedNodeTag::Expression.as_wire() {
            return Err(ExactScalarRefusal::InvalidBody {
                body_node_id: node_id.clone(),
            });
        } else {
            return Err(ExactScalarRefusal::OperandUnsupported {
                position,
                term: node.node_tag.to_string(),
            });
        };
        check_operand_endpoint_bits(node_id, &result.0, &result.1)?;
        active.remove(node_id);
        memo.insert(node_id.clone(), result.clone());
        Ok(result)
    }
    let (lower, upper) = visit(
        graph,
        node_id,
        position,
        &mut BTreeSet::new(),
        &mut BTreeMap::new(),
        0,
        &mut 0,
    )?;
    Ok((lower.to_string(), upper.to_string()))
}

pub(super) fn check_operand_endpoint_bits(
    node_id: &CheckedNodeId,
    lower: &BigInt,
    upper: &BigInt,
) -> Result<(), ExactScalarRefusal> {
    for endpoint in [lower, upper] {
        let consumed = endpoint.bits();
        if consumed > OPERAND_RANGE_MAX_ENDPOINT_BITS {
            return Err(ExactScalarRefusal::OperandRangeLimitExceeded {
                node_id: node_id.clone(),
                resource_kind: OperandRangeLimitKind::EndpointBits,
                limit: OPERAND_RANGE_MAX_ENDPOINT_BITS,
                consumed,
            });
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Parameters against IR bounds
// ---------------------------------------------------------------------------

/// Check every descriptor parameter the IR carries against the node's result bound of its form,
/// returning the bounds checked: the result bound first, then, for integer arithmetic, division
/// and modulo, each operand's own bound. Only the result bound is compared with the descriptor.
fn check_parameters<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node: &CompleteContractNodeV2,
    operation: &ExactScalarOperation,
) -> Result<Vec<CheckedNodeId>, ItemCheckError> {
    check_parameters_from_view(graph, BoundView::complete(node), operation)
}

pub(super) fn check_parameters_from_view<Body: GraphBody, NodeBody>(
    graph: &Graph<'_, Body>,
    node: BoundView<'_, NodeBody>,
    operation: &ExactScalarOperation,
) -> Result<Vec<CheckedNodeId>, ItemCheckError> {
    let bounds = Bounds { graph, node };
    let checked = match operation {
        ExactScalarOperation::IntegerArithmetic { domain, .. }
        | ExactScalarOperation::IntegerDivision { domain, .. }
        | ExactScalarOperation::IntegerModulo { domain } => {
            let declared = match domain {
                IntegerDomain::Bounded(interval) => Some(interval),
                IntegerDomain::Mathematical => None,
            };
            let result = bounds.equal(BoundForm::IntegerRange, read_integer_range, declared)?;
            let mut checked = vec![result];
            for bound in bounds.integer_operand_bounds()? {
                if !checked.contains(&bound) {
                    checked.push(bound);
                }
            }
            return Ok(checked);
        }
        ExactScalarOperation::RationalArithmetic { domain, .. } => bounds.equal(
            BoundForm::RationalRange,
            read_rational_range,
            domain.as_ref(),
        )?,
        ExactScalarOperation::DecimalArithmetic { target, .. } => {
            bounds.equal(BoundForm::DecimalRange, read_decimal_range, Some(target))?
        }
        ExactScalarOperation::IeeeArithmetic { rounding, .. }
        | ExactScalarOperation::IeeeWidthConversion { rounding, .. } => bounds.equal(
            BoundForm::FloatRounding,
            read_float_rounding,
            Some(rounding),
        )?,
        ExactScalarOperation::TextAdmission { text_type } => {
            bounds.equal(BoundForm::TextBounds, read_text_bounds, Some(text_type))?
        }
        ExactScalarOperation::Ordering { .. }
        | ExactScalarOperation::Boolean { .. }
        | ExactScalarOperation::Equality { .. }
        | ExactScalarOperation::IeeeComparison { .. }
        | ExactScalarOperation::TextComparison { .. }
        | ExactScalarOperation::EnumComparison { .. }
        | ExactScalarOperation::QuantityArithmetic { .. }
        | ExactScalarOperation::QuantityComparison { .. }
        | ExactScalarOperation::QuantityConversion => return Ok(Vec::new()),
    };
    Ok(vec![checked])
}

/// The bound inputs shared by old lowering records and admitted FR-330 closures.
/// No per-node IR identity is reconstructed for the package path.
pub(super) struct BoundView<'n, Body = CheckedTerm> {
    root: &'n CheckedSemanticNodeV2<Body>,
    semantic_type: CheckedNodeId,
    bounds: Vec<CheckedNodeId>,
    pub(super) selected: Option<SelectedResult<'n>>,
}

impl<Body> Clone for BoundView<'_, Body> {
    fn clone(&self) -> Self {
        Self {
            root: self.root,
            semantic_type: self.semantic_type.clone(),
            bounds: self.bounds.clone(),
            selected: self.selected.clone(),
        }
    }
}

#[derive(Clone)]
pub(super) struct SelectedResult<'a> {
    pub(super) package: &'a ContractPackageV1,
    pub(super) application: &'a ContractOccurrenceKeyV1,
    pub(super) conversion: Option<&'a ContractOccurrenceKeyV1>,
}

impl<'n> BoundView<'n, CheckedTerm> {
    pub(super) fn complete(node: &'n CompleteContractNodeV2) -> Self {
        Self {
            root: &node.node,
            semantic_type: node.semantic_type.clone(),
            bounds: node.bounds.clone(),
            selected: None,
        }
    }
}

impl<'n> BoundView<'n, Value> {
    pub(super) fn admitted(
        closure: &'n quire_contract_model::ContractReachedClosureV1<'_>,
        selected: SelectedResult<'n>,
    ) -> Self {
        Self {
            root: closure.root,
            semantic_type: closure.root.semantic_type.clone(),
            bounds: closure.bounds.clone(),
            selected: Some(selected),
        }
    }
}

pub(super) struct Bounds<'g, 'n, Body: GraphBody = Value, NodeBody = CheckedTerm> {
    pub(super) graph: &'g Graph<'g, Body>,
    pub(super) node: BoundView<'n, NodeBody>,
}

impl<Body: GraphBody, NodeBody> Bounds<'_, '_, Body, NodeBody> {
    /// Read the result bound of `form`: the one selector both [`Self::equal`] and
    /// [`super::derive_exact_scalar_items`] use. Returns the bound's node and the value it reads as.
    ///
    /// The result's role is named by the node itself. A result typed by a `bounded_domain` has
    /// that domain as its bound. A result typed by a scalar type has the bound of the narrowing
    /// conversion that consumes it (QSL's shape for a bounded context); failing one, the one
    /// reachable bound of `form` over that type and, where several are reachable, the one that is
    /// not an operand's own bound; a node whose bounds do not resolve that way is ambiguous.
    pub(super) fn read<T>(
        &self,
        form: BoundForm,
        read: fn(&[CheckedTerm]) -> Option<T>,
    ) -> Result<(CheckedNodeId, T), ExactScalarRefusal> {
        if let Some(selected) = &self.node.selected {
            let result = selected
                .package
                .result_context(
                    &self.node.root.node_id,
                    selected.application,
                    selected.conversion,
                    form.ir(),
                )
                .map_err(|_| ExactScalarRefusal::IncompleteContext)?;
            let bound = match result {
                ContractResultViewV1::Lowered(result) => match result.context {
                    ContractResultContextV1::DirectBounded { bound }
                    | ContractResultContextV1::Narrowed { bound, .. } => bound.node,
                    ContractResultContextV1::MissingBound { type_key } => {
                        return Err(ExactScalarRefusal::MissingBound {
                            bounded_type: type_key,
                            expected_form: form,
                        });
                    }
                    ContractResultContextV1::Unnarrowed => {
                        return Err(ExactScalarRefusal::MissingBound {
                            bounded_type: self.node.semantic_type.clone(),
                            expected_form: form,
                        });
                    }
                    ContractResultContextV1::IncompleteContext => {
                        return Err(ExactScalarRefusal::IncompleteContext);
                    }
                },
                ContractResultViewV1::NonLowered(_) | ContractResultViewV1::Absent => {
                    return Err(ExactScalarRefusal::IncompleteContext);
                }
            };
            let value = aggregate_members(self.graph.term(&bound.node_id))
                .and_then(read)
                .ok_or_else(|| ExactScalarRefusal::UnreadableBound {
                    bound: bound.node_id.clone(),
                })?;
            return Ok((bound.node_id.clone(), value));
        }
        let bounded_type = &self.node.semantic_type;
        let typing = self.graph.node(bounded_type);
        let bound = match typing.filter(|typing| is_bounded_domain(typing)) {
            Some(bound) if &*bound.semantic_form == form.wire() => bound,
            Some(bound) => {
                return Err(ExactScalarRefusal::MissingBound {
                    bounded_type: bound.semantic_type.clone(),
                    expected_form: form,
                });
            }
            None => self.scalar_result_bound(form)?,
        };
        let value = aggregate_members(self.graph.term(&bound.node_id))
            .and_then(read)
            .ok_or_else(|| ExactScalarRefusal::UnreadableBound {
                bound: bound.node_id.clone(),
            })?;
        Ok((bound.node_id.clone(), value))
    }

    /// How the nodes consuming this node narrow it, for a node whose result is typed by a scalar
    /// type. QSL wraps an expression in a bounded context in a narrowing `conversion` typed by the
    /// bound, and IR's forward-only `bounds` closure never reaches it from the expression, so the
    /// enclosing conversion is where the result bound is named.
    pub(super) fn narrowing(&self) -> Narrowing<'_, Body> {
        narrowing(self.graph, &self.node.root.node_id)
    }

    /// The result bound of a node whose result is typed by a scalar type.
    ///
    /// Where narrowing conversions consume the node, the result bound is the one bound they
    /// narrow it to: of the descriptor's form and over the node's own scalar type, or it is
    /// `MissingBound`; two or more distinct bounds are `AmbiguousBound`, and so is a node that a
    /// narrowing conversion and any other node both consume, since the narrow's bound does not
    /// bound the other use. Otherwise it is the one
    /// reachable bound of the form over that type, or, where several are reachable, the one that
    /// is no operand's own.
    fn scalar_result_bound(
        &self,
        form: BoundForm,
    ) -> Result<&CheckedSemanticNodeV2<Body>, ExactScalarRefusal> {
        let bounded_type = &self.node.semantic_type;
        let narrowing = self.narrowing();
        let narrowed = match &narrowing {
            Narrowing::Unnarrowed => &[][..],
            Narrowing::To(bounds) => bounds.as_slice(),
            Narrowing::Mixed => {
                return Err(ExactScalarRefusal::AmbiguousBound {
                    bounded_type: bounded_type.clone(),
                    expected_form: form,
                });
            }
        };
        match narrowed {
            [] => {}
            [bound] if &*bound.semantic_form != form.wire() => {
                return Err(ExactScalarRefusal::MissingBound {
                    bounded_type: bound.semantic_type.clone(),
                    expected_form: form,
                });
            }
            [bound] if &bound.semantic_type != bounded_type => {
                return Err(ExactScalarRefusal::MissingBound {
                    bounded_type: bounded_type.clone(),
                    expected_form: form,
                });
            }
            [bound] => return Ok(bound),
            [_, _, ..] => {
                return Err(ExactScalarRefusal::AmbiguousBound {
                    bounded_type: bounded_type.clone(),
                    expected_form: form,
                });
            }
        }
        let candidates = self
            .node
            .bounds
            .iter()
            .filter_map(|id| self.graph.node(id))
            .filter(|bound| {
                &bound.semantic_type == bounded_type && &*bound.semantic_form == form.wire()
            })
            .collect::<Vec<_>>();
        match candidates.as_slice() {
            [bound] => Ok(bound),
            [] => Err(ExactScalarRefusal::MissingBound {
                bounded_type: bounded_type.clone(),
                expected_form: form,
            }),
            [_, _, ..] => {
                let own = own_bounds(self.graph, self.graph.term(&self.node.root.node_id));
                let mut free = candidates.iter().filter(|candidate| {
                    !own.iter()
                        .flatten()
                        .any(|operand| operand.node_id == candidate.node_id)
                });
                match (free.next(), free.next()) {
                    (Some(bound), None) => Ok(bound),
                    _ => Err(ExactScalarRefusal::AmbiguousBound {
                        bounded_type: bounded_type.clone(),
                        expected_form: form,
                    }),
                }
            }
        }
    }

    /// The own `integer_range` bound of each operand, in argument order and deduplicated. An
    /// operand typed by a bound of another form has no integer bound and is `MissingBound`.
    ///
    /// A `reference` operand typed by a plain scalar type owns no bound, so nothing bounds it but
    /// the result's: where any operand owns a bound, or the result is typed by one or narrowed to
    /// one by a consuming conversion, the package bounds each operand by its own typing and such an
    /// operand is `RequiresBound`. A literal, inline or a reference to a `value` node whose body is
    /// a literal, is a constant: it owns no bound and a Kani harness ranges it at its own value
    /// ([`operand_ranges`]); one that is not a canonical `integer` literal is `OperandTypeMismatch`
    /// naming its kind, whatever the referenced node's type. Where no
    /// operand owns a bound and the result is scalar-typed and not narrowed, the operands are
    /// bounded by the one bound the node reaches (the shape IR-299 tracks), as before.
    pub(super) fn integer_operand_bounds(&self) -> Result<Vec<CheckedNodeId>, ExactScalarRefusal> {
        let form = BoundForm::IntegerRange;
        let arguments = self
            .graph
            .arguments(&self.node.root.node_id)
            .unwrap_or_default();
        for (position, argument) in arguments.iter().enumerate() {
            if let Some(literal) = operand_literal(self.graph, argument) {
                if literal_integer(literal).is_none() {
                    return Err(ExactScalarRefusal::OperandTypeMismatch {
                        position,
                        expected: ScalarForm::Integer,
                        found: literal_form(literal),
                    });
                }
            }
        }
        let owned = own_bounds(self.graph, self.graph.term(&self.node.root.node_id));
        let result_typed = if let Some(selected) = &self.node.selected {
            match selected
                .package
                .result_context(
                    &self.node.root.node_id,
                    selected.application,
                    selected.conversion,
                    BoundedDomainForm::IntegerRange,
                )
                .map_err(|_| ExactScalarRefusal::IncompleteContext)?
            {
                ContractResultViewV1::Lowered(result) => match result.context {
                    ContractResultContextV1::DirectBounded { .. }
                    | ContractResultContextV1::Narrowed { .. } => true,
                    ContractResultContextV1::Unnarrowed => false,
                    ContractResultContextV1::MissingBound { .. } => selected.conversion.is_some(),
                    ContractResultContextV1::IncompleteContext => {
                        return Err(ExactScalarRefusal::IncompleteContext);
                    }
                },
                ContractResultViewV1::NonLowered(_) | ContractResultViewV1::Absent => {
                    return Err(ExactScalarRefusal::IncompleteContext);
                }
            }
        } else {
            self.graph
                .node(&self.node.semantic_type)
                .is_some_and(is_bounded_domain)
                || self.narrowing().narrowed()
        };
        if result_typed || owned.iter().any(Option::is_some) {
            for (position, (argument, own)) in arguments.iter().zip(&owned).enumerate() {
                let Some(target) = reference_target(argument) else {
                    continue;
                };
                let eligible = own.is_none()
                    && self.graph.node(&target).is_some_and(|node| {
                        &*node.node_tag == CheckedNodeTag::Expression.as_wire()
                            && scalar_type_form(self.graph, &node.semantic_type).as_deref()
                                == Some("integer")
                    });
                if eligible {
                    infer_integer_range(self.graph, &target, position)?;
                }
            }
            let plain = arguments
                .iter()
                .zip(&owned)
                .find_map(|(argument, own)| match own {
                    None if reference_target(argument).is_some_and(|target| {
                        self.graph.node(&target).is_some_and(|node| {
                            &*node.node_tag == CheckedNodeTag::Expression.as_wire()
                                && scalar_type_form(self.graph, &node.semantic_type).as_deref()
                                    == Some("integer")
                        })
                    }) =>
                    {
                        None
                    }
                    None => plain_reference_type(self.graph, argument),
                    Some(_) => None,
                });
            if let Some(unbounded_type) = plain {
                return Err(ExactScalarRefusal::RequiresBound { unbounded_type });
            }
        }
        let mut checked = Vec::new();
        for bound in owned.into_iter().flatten() {
            if &*bound.semantic_form != form.wire() {
                return Err(ExactScalarRefusal::MissingBound {
                    bounded_type: bound.semantic_type.clone(),
                    expected_form: form,
                });
            }
            if aggregate_members(self.graph.term(&bound.node_id))
                .and_then(read_integer_range)
                .is_none()
            {
                return Err(ExactScalarRefusal::UnreadableBound {
                    bound: bound.node_id.clone(),
                });
            }
            if !checked.contains(&bound.node_id) {
                checked.push(bound.node_id.clone());
            }
        }
        Ok(checked)
    }

    /// Read the single bound of `form` on the result type and require the
    /// declared parameter to equal it. An absent declaration (an unbounded
    /// descriptor) never equals an IR bound.
    pub(super) fn equal<T: PartialEq>(
        &self,
        form: BoundForm,
        read: fn(&[CheckedTerm]) -> Option<T>,
        declared: Option<&T>,
    ) -> Result<CheckedNodeId, ExactScalarRefusal> {
        let (bound, value) = self.read(form, read)?;
        if declared == Some(&value) {
            Ok(bound)
        } else {
            Err(ExactScalarRefusal::BoundMismatch { bound, form })
        }
    }
}

impl ScalarForm {
    pub(super) fn wire(self) -> &'static str {
        match self {
            Self::Boolean => "boolean",
            Self::Integer => "integer",
            Self::Rational => "rational",
            Self::Decimal => "decimal",
            Self::Float32 => "float32",
            Self::Float64 => "float64",
            Self::Text => "text",
            Self::Enum => "enum",
            Self::Unit => "unit",
        }
    }
}

/// The node shape a descriptor requires.
pub(super) struct Shape {
    pub(super) form: &'static str,
    pub(super) body_operator: &'static str,
    pub(super) operands: &'static [ScalarForm],
    pub(super) result: ScalarForm,
}

impl Shape {
    fn binary(operands: &'static [ScalarForm; 2], result: ScalarForm) -> Self {
        Self {
            form: "binary",
            body_operator: "binary",
            operands,
            result,
        }
    }

    fn unary(operand: &'static [ScalarForm; 1], result: ScalarForm) -> Self {
        Self {
            form: "unary",
            body_operator: "unary",
            operands: operand,
            result,
        }
    }

    fn conversion(operand: &'static [ScalarForm; 1], result: ScalarForm) -> Self {
        Self {
            form: "conversion",
            body_operator: "convert",
            operands: operand,
            result,
        }
    }

    /// The `body_operator: "call"` half is grounded in the catalog: the
    /// operation-catalog's three IEEE-comparison identities
    /// (`quire.op.ieee.numeric_equal`, `.total_order`, `.bit_identical`) are
    /// all `call`-operator entries, and Contract IR's
    /// `validate_operations` refuses any package whose `body.operator`
    /// disagrees with its catalogued `operation.identity`'s own `operator`,
    /// so a real, admitted IEEE comparison node's `body.operator` is always
    /// `"call"`, never `"binary"`.
    ///
    /// The `form: "binary"` half is grounded only in this repo's own
    /// fixtures, not in IR or the catalog: IR does not couple
    /// `semantic_form` to `body.operator` -- `Expression::forms()` admits
    /// `"call"` and `"binary"` as independent, unrelated values -- so a real
    /// producer is free to emit `semantic_form: "call"` for an IEEE
    /// comparison node, and this shape would then refuse it `FormMismatch`.
    fn binary_call(operands: &'static [ScalarForm; 2], result: ScalarForm) -> Self {
        Self {
            form: "binary",
            body_operator: "call",
            operands,
            result,
        }
    }

    pub(super) fn of(operation: &ExactScalarOperation) -> Result<Self, OracleGenerationError> {
        use ScalarForm as F;
        Ok(match operation {
            ExactScalarOperation::Boolean { operator, .. } => match operator {
                BooleanOperator::Not => Self::unary(&[F::Boolean], F::Boolean),
                BooleanOperator::And | BooleanOperator::Or | BooleanOperator::Implies => {
                    Self::binary(&[F::Boolean, F::Boolean], F::Boolean)
                }
            },
            ExactScalarOperation::Equality { operands, .. } => match operands {
                EqualityOperandKind::Boolean => Self::binary(&[F::Boolean, F::Boolean], F::Boolean),
                EqualityOperandKind::Integer => Self::binary(&[F::Integer, F::Integer], F::Boolean),
            },
            ExactScalarOperation::IntegerArithmetic { operator, .. } => match operator {
                IntegerOperator::Negate => Self::unary(&[F::Integer], F::Integer),
                IntegerOperator::Add | IntegerOperator::Subtract | IntegerOperator::Multiply => {
                    Self::binary(&[F::Integer, F::Integer], F::Integer)
                }
            },
            ExactScalarOperation::IntegerDivision { .. }
            | ExactScalarOperation::IntegerModulo { .. } => {
                Self::binary(&[F::Integer, F::Integer], F::Integer)
            }
            ExactScalarOperation::RationalArithmetic { operator, .. } => match operator {
                RationalOperator::Negate => Self::unary(&[F::Rational], F::Rational),
                RationalOperator::IntegerDivide => {
                    Self::binary(&[F::Integer, F::Integer], F::Rational)
                }
                RationalOperator::Add
                | RationalOperator::Subtract
                | RationalOperator::Multiply
                | RationalOperator::Divide => {
                    Self::binary(&[F::Rational, F::Rational], F::Rational)
                }
            },
            ExactScalarOperation::Ordering { operands, .. } => match operands {
                OrderingOperandKind::Integer => Self::binary(&[F::Integer, F::Integer], F::Boolean),
                OrderingOperandKind::Rational => {
                    Self::binary(&[F::Rational, F::Rational], F::Boolean)
                }
                OrderingOperandKind::Decimal => Self::binary(&[F::Decimal, F::Decimal], F::Boolean),
            },
            ExactScalarOperation::DecimalArithmetic { operator, .. } => match operator {
                DecimalOperator::Negate => Self::unary(&[F::Decimal], F::Decimal),
                DecimalOperator::Round => Self::conversion(&[F::Decimal], F::Decimal),
                DecimalOperator::Add
                | DecimalOperator::Subtract
                | DecimalOperator::Multiply
                | DecimalOperator::Divide => Self::binary(&[F::Decimal, F::Decimal], F::Decimal),
            },
            ExactScalarOperation::IeeeArithmetic { width, .. } => match width {
                IeeeWidth::Binary32 => Self::binary(&[F::Float32, F::Float32], F::Float32),
                IeeeWidth::Binary64 => Self::binary(&[F::Float64, F::Float64], F::Float64),
            },
            ExactScalarOperation::IeeeComparison { width, .. } => match width {
                IeeeWidth::Binary32 => Self::binary_call(&[F::Float32, F::Float32], F::Boolean),
                IeeeWidth::Binary64 => Self::binary_call(&[F::Float64, F::Float64], F::Boolean),
            },
            ExactScalarOperation::IeeeWidthConversion { source, target, .. } => {
                let operand: &'static [ScalarForm; 1] = match source {
                    IeeeWidth::Binary32 => &[F::Float32],
                    IeeeWidth::Binary64 => &[F::Float64],
                };
                Self::conversion(operand, F::of_width(*target)?)
            }
            ExactScalarOperation::TextAdmission { .. } => Self::conversion(&[F::Text], F::Text),
            ExactScalarOperation::TextComparison { .. } => {
                Self::binary(&[F::Text, F::Text], F::Boolean)
            }
            ExactScalarOperation::EnumComparison { .. } => {
                Self::binary(&[F::Enum, F::Enum], F::Boolean)
            }
            ExactScalarOperation::QuantityArithmetic { operator } => match operator {
                QuantityOperator::Power => Self::binary(&[F::Unit, F::Integer], F::Unit),
                QuantityOperator::Add
                | QuantityOperator::Subtract
                | QuantityOperator::Multiply
                | QuantityOperator::Divide => Self::binary(&[F::Unit, F::Unit], F::Unit),
            },
            ExactScalarOperation::QuantityConversion => Self::conversion(&[F::Unit], F::Unit),
            ExactScalarOperation::QuantityComparison { .. } => {
                Self::binary(&[F::Unit, F::Unit], F::Boolean)
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Operation identity
// ---------------------------------------------------------------------------

/// A refused item's claim and disposition together: the request item's own
/// descriptor-derived identity, marked
/// [`OperationProvenance::CallerDeclared`], paired with the
/// [`ClaimDisposition::Refused`] that names why. Returning the pair
/// keeps them from drifting apart -- a call site that built only the claim
/// half, or built a different disposition beside it, would not compile.
///
/// One of the three call sites, the `Err` arm guarding
/// [`catalogued_operation_identity`], used to be unreachable:
/// [`operation_confirmed`] was asked first and had already required that same
/// member, so `MissingOperationIdentity` was unconstructible through it and a
/// node missing the member was reported here instead, as though codegen had
/// declined to consume an identity that was not there. IR-224 reordered the
/// two, so that arm is now reachable: an identity-absent node reaches it, and
/// this function names that claim caller-declared while the disposition
/// beside it is `Refused`. Before the reorder such a node took the inline
/// `CallerDeclared` construction instead and generated an oracle. Nothing is
/// claimed here about the other sites: they refuse items `check_item`
/// rejected and duplicate requests, neither of which turns on the operation
/// member at all.
///
/// A lowered node whose catalogued operation disagrees with the descriptor is
/// `CallerDeclared` too, but does not come through here: it generates, and
/// builds its provenance inline at the disagreement site.
pub(super) fn refused_claim(
    identity: String,
    refusal: ExactScalarRefusal,
) -> (
    OperationClaim,
    ClaimDisposition<GeneratedScalarClaim, ExactScalarRefusal>,
) {
    (
        OperationClaim {
            identity,
            provenance: OperationProvenance::CallerDeclared {
                blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
            },
        },
        ClaimDisposition::Refused { refusal },
    )
}

/// The node's own catalogued operation identity, or a typed refusal when the
/// member IR admission is supposed to guarantee is absent. `check_item`
/// having returned this node `Ok` already establishes that its `body` is an
/// `application` term (`application_arguments` requires `term ==
/// "application"`), and Contract IR's `validate_operations`
/// requires `identity` on every such node's `operation` member before the
/// package that contains it is ever admitted -- so this generator should
/// never receive a node for which this member is absent. It is another
/// crate's invariant, not this one's, so a violation is reported rather than
/// panicked on.
pub(super) fn catalogued_operation_identity<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node: &CompleteContractNodeV2,
) -> Result<String, ExactScalarRefusal> {
    graph
        .operation(&node.node.node_id)
        .and_then(|operation| operation.get("identity"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| ExactScalarRefusal::MissingOperationIdentity {
            node_id: node.node.node_id.clone(),
        })
}

/// The catalogued identity a descriptor implies, and -- where the catalog's
/// own entry for it carries a `mode` member -- the `(kind, value)` that
/// member must equal for the descriptor's own rounding parameter to be
/// trusted. `None` when the descriptor names a combination the catalog has
/// no entry for at all (same-width [`ExactScalarOperation::IeeeWidthConversion`]).
///
/// This mirrors `tests/exact_scalar_support/package.rs`'s own
/// `corpus_operation`, which builds every fixture node from the identical
/// mapping (see its own doc for the catalog cross-reference); moving it here
/// lets [`operation_confirmed`] recognize an identity and mode IR already
/// confirmed -- not translate between codegen's and IR's vocabularies to
/// reconcile a disagreement between them, the same kind of closed match
/// `Shape::of` already makes over `ExactScalarOperation`.
pub(super) struct CataloguedOperation {
    pub(super) identity: &'static str,
    mode: Option<(&'static str, &'static str)>,
}

fn plain(identity: &'static str) -> Option<CataloguedOperation> {
    Some(CataloguedOperation {
        identity,
        mode: None,
    })
}

fn with_rounding(identity: &'static str, rounding: RoundingMode) -> Option<CataloguedOperation> {
    Some(CataloguedOperation {
        identity,
        mode: Some(("rounding", rounding.as_str())),
    })
}

pub(super) fn catalogued_operation(
    operation: &ExactScalarOperation,
) -> Result<Option<CataloguedOperation>, OracleGenerationError> {
    use ExactScalarOperation as Op;
    Ok(match operation {
        Op::Boolean { operator, .. } => plain(match operator {
            BooleanOperator::And => "quire.op.boolean.and",
            BooleanOperator::Or => "quire.op.boolean.or",
            BooleanOperator::Not => "quire.op.boolean.not",
            BooleanOperator::Implies => "quire.op.boolean.implies",
        }),
        Op::Equality { operator, operands } => plain(match (operands, operator) {
            (EqualityOperandKind::Boolean, EqualityOperator::Equal) => "quire.op.boolean.eq",
            (EqualityOperandKind::Boolean, EqualityOperator::NotEqual) => "quire.op.boolean.ne",
            (EqualityOperandKind::Integer, EqualityOperator::Equal) => "quire.op.integer.eq",
            (EqualityOperandKind::Integer, EqualityOperator::NotEqual) => "quire.op.integer.ne",
        }),
        Op::IntegerArithmetic { operator, .. } => plain(match operator {
            IntegerOperator::Add => CatalogOperator::Add.entry().identity,
            IntegerOperator::Subtract => CatalogOperator::Subtract.entry().identity,
            IntegerOperator::Multiply => CatalogOperator::Multiply.entry().identity,
            IntegerOperator::Negate => CatalogOperator::Negate.entry().identity,
        }),
        Op::IntegerDivision { .. } => plain("quire.op.integer.div"),
        Op::IntegerModulo { .. } => plain("quire.op.integer.mod"),
        Op::RationalArithmetic { operator, .. } => plain(match operator {
            RationalOperator::Add => "quire.op.rational.add",
            RationalOperator::Subtract => "quire.op.rational.sub",
            RationalOperator::Multiply => "quire.op.rational.mul",
            // `rational.div`'s operands are `rational_promotable` (integer or
            // rational): there is no separate catalogued "integer division to
            // a rational result" identity.
            RationalOperator::Divide | RationalOperator::IntegerDivide => "quire.op.rational.div",
            RationalOperator::Negate => "quire.op.rational.negate",
        }),
        Op::Ordering { operator, operands } => plain(ordering_identity(operands, operator)?),
        Op::DecimalArithmetic { operator, target } => match operator {
            DecimalOperator::Add => with_rounding("quire.op.decimal.add", target.rounding()),
            DecimalOperator::Subtract => with_rounding("quire.op.decimal.sub", target.rounding()),
            DecimalOperator::Multiply => with_rounding("quire.op.decimal.mul", target.rounding()),
            DecimalOperator::Divide => with_rounding("quire.op.decimal.div", target.rounding()),
            DecimalOperator::Negate => plain("quire.op.decimal.negate"),
            DecimalOperator::Round => {
                with_rounding("quire.op.numeric.convert_rounding", target.rounding())
            }
        },
        Op::IeeeArithmetic {
            operator,
            width,
            rounding,
        } => {
            let identity = match (width, operator) {
                (IeeeWidth::Binary32, IeeeArithmeticOperator::Add) => "quire.op.ieee.float32.add",
                (IeeeWidth::Binary32, IeeeArithmeticOperator::Subtract) => {
                    "quire.op.ieee.float32.sub"
                }
                (IeeeWidth::Binary32, IeeeArithmeticOperator::Multiply) => {
                    "quire.op.ieee.float32.mul"
                }
                (IeeeWidth::Binary32, IeeeArithmeticOperator::Divide) => {
                    "quire.op.ieee.float32.div"
                }
                (IeeeWidth::Binary64, IeeeArithmeticOperator::Add) => "quire.op.ieee.float64.add",
                (IeeeWidth::Binary64, IeeeArithmeticOperator::Subtract) => {
                    "quire.op.ieee.float64.sub"
                }
                (IeeeWidth::Binary64, IeeeArithmeticOperator::Multiply) => {
                    "quire.op.ieee.float64.mul"
                }
                (IeeeWidth::Binary64, IeeeArithmeticOperator::Divide) => {
                    "quire.op.ieee.float64.div"
                }
            };
            with_rounding(identity, *rounding)
        }
        Op::IeeeComparison { comparison, .. } => plain(match comparison {
            IeeeComparison::NumericEqual => "quire.op.ieee.numeric_equal",
            IeeeComparison::TotalOrder => "quire.op.ieee.total_order",
            IeeeComparison::BitIdentical => "quire.op.ieee.bit_identical",
        }),
        Op::IeeeWidthConversion {
            source,
            target,
            rounding,
        } => match (source, target) {
            (IeeeWidth::Binary64, IeeeWidth::Binary32) => {
                with_rounding("quire.op.ieee.to_float32", *rounding)
            }
            (IeeeWidth::Binary32, IeeeWidth::Binary64) => plain("quire.op.ieee.to_float64"),
            // Same-width "conversion" has no catalogued identity at all; a
            // descriptor naming one can never be confirmed.
            (IeeeWidth::Binary32, IeeeWidth::Binary32)
            | (IeeeWidth::Binary64, IeeeWidth::Binary64) => None,
        },
        Op::TextAdmission { .. } => plain("quire.op.numeric.convert"),
        // `text.*`/`enum.*` comparisons carry a catalogued `text_profile` mode
        // (`"nfc"`), but the descriptor itself has no profile-bearing field to
        // compare it against, so only the identity is checked.
        Op::TextComparison { operator } => {
            plain(comparison_identity(ComparisonFamily::Text, operator)?)
        }
        Op::EnumComparison { operator } => {
            plain(comparison_identity(ComparisonFamily::Enum, operator)?)
        }
        Op::QuantityConversion => with_rounding(QUANTITY_CONVERT_IDENTITY, RoundingMode::Exact),
        Op::QuantityArithmetic { operator } => plain(match operator {
            QuantityOperator::Add => "quire.op.quantity.add",
            QuantityOperator::Subtract => "quire.op.quantity.sub",
            QuantityOperator::Multiply => "quire.op.quantity.mul",
            QuantityOperator::Divide => "quire.op.quantity.div",
            QuantityOperator::Power => "quire.op.quantity.pow",
        }),
        Op::QuantityComparison { operator } => {
            plain(comparison_identity(ComparisonFamily::Quantity, operator)?)
        }
    })
}

/// `quire.op.<family>.<lt|le|gt|ge>` for an integer, rational or decimal
/// `Ordering` descriptor, one catalogued identity per (operands, operator).
fn ordering_identity(
    operands: &OrderingOperandKind,
    operator: &OrderingOperator,
) -> Result<&'static str, OracleGenerationError> {
    use OrderingOperandKind as Kind;
    use OrderingOperator as Op;
    if matches!(operands, Kind::Integer) {
        return CatalogOperator::from_ordering_operator(*operator)
            .map(|operation| operation.entry().identity)
            .ok_or_else(|| OracleGenerationError::unknown_variant("OrderingOperator"));
    }
    Ok(match (operands, operator) {
        (Kind::Rational, Op::Less) => "quire.op.rational.lt",
        (Kind::Rational, Op::LessOrEqual) => "quire.op.rational.le",
        (Kind::Rational, Op::Greater) => "quire.op.rational.gt",
        (Kind::Rational, Op::GreaterOrEqual) => "quire.op.rational.ge",
        (Kind::Decimal, Op::Less) => "quire.op.decimal.lt",
        (Kind::Decimal, Op::LessOrEqual) => "quire.op.decimal.le",
        (Kind::Decimal, Op::Greater) => "quire.op.decimal.gt",
        (Kind::Decimal, Op::GreaterOrEqual) => "quire.op.decimal.ge",
        (Kind::Integer, _) => {
            return Err(OracleGenerationError::unknown_variant(
                "OrderingOperandKind",
            ));
        }
    })
}

/// The catalog's comparison families that carry all six of
/// `eq`/`ne`/`lt`/`le`/`gt`/`ge`.
#[derive(Clone, Copy)]
enum ComparisonFamily {
    Text,
    Enum,
    Quantity,
}

/// `quire.op.<family>.<suffix>` for one of the catalog's six comparison
/// identities per family.
fn comparison_identity(
    family: ComparisonFamily,
    operator: &ComparisonOperator,
) -> Result<&'static str, OracleGenerationError> {
    use ComparisonFamily as F;
    use ComparisonOperator as Op;
    Ok(match (family, operator) {
        (F::Text, Op::Equal) => "quire.op.text.eq",
        (F::Text, Op::NotEqual) => "quire.op.text.ne",
        (F::Text, Op::Less) => "quire.op.text.lt",
        (F::Text, Op::LessOrEqual) => "quire.op.text.le",
        (F::Text, Op::Greater) => "quire.op.text.gt",
        (F::Text, Op::GreaterOrEqual) => "quire.op.text.ge",
        (F::Enum, Op::Equal) => "quire.op.enum.eq",
        (F::Enum, Op::NotEqual) => "quire.op.enum.ne",
        (F::Enum, Op::Less) => "quire.op.enum.lt",
        (F::Enum, Op::LessOrEqual) => "quire.op.enum.le",
        (F::Enum, Op::Greater) => "quire.op.enum.gt",
        (F::Enum, Op::GreaterOrEqual) => "quire.op.enum.ge",
        (F::Quantity, Op::Equal) => "quire.op.quantity.eq",
        (F::Quantity, Op::NotEqual) => "quire.op.quantity.ne",
        (F::Quantity, Op::Less) => "quire.op.quantity.lt",
        (F::Quantity, Op::LessOrEqual) => "quire.op.quantity.le",
        (F::Quantity, Op::Greater) => "quire.op.quantity.gt",
        (F::Quantity, Op::GreaterOrEqual) => "quire.op.quantity.ge",
    })
}

/// Whether the checked node's own catalogued operation genuinely confirms the
/// descriptor beyond `check_item`'s shape checks: its `operation.identity`
/// names the same catalogued operation the descriptor implies
/// ([`catalogued_operation`]), its `operation.mode` (where that catalog entry
/// has one) carries the same value, and -- for `IntegerDivision`, the one
/// family with more than one catalogued law definition -- its
/// `operation.laws` names the descriptor's own division profile. IR's
/// `integer_division` law role and this crate's own
/// `DivisionProfile::definition_identity()` name the same three law
/// definitions (`quire.value.integer-division.{truncating,floor,euclidean}/v1`
/// -- see `operation_identity`'s own use of `profile.definition_identity()`),
/// so comparing them is not a bridge between codegen's and IR's vocabularies:
/// it is one vocabulary, read from two places. Every other family either has
/// no more than one catalogued law definition (`ieee_profile`,
/// `text_profile`) or none at all, so its shape and identity/mode checks are
/// already the whole story for it.
pub(super) fn operation_confirmed<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node: &CompleteContractNodeV2,
    operation: &ExactScalarOperation,
) -> Result<bool, OracleGenerationError> {
    let Some(catalogued) = catalogued_operation(operation)? else {
        return Ok(false);
    };
    let Some(payload) = graph.operation(&node.node.node_id) else {
        return Ok(false);
    };
    if payload["identity"].as_str() != Some(catalogued.identity) {
        return Ok(false);
    }
    if let Some((kind, value)) = catalogued.mode {
        let mode = &payload["mode"];
        if mode["kind"].as_str() != Some(kind) || mode["value"].as_str() != Some(value) {
            return Ok(false);
        }
    }
    if let ExactScalarOperation::IntegerDivision { profile, .. } = operation {
        let law_confirmed = payload["laws"].as_array().into_iter().flatten().any(|law| {
            law["definition"]["identity"].as_str() == Some(profile.definition_identity())
        });
        if !law_confirmed {
            return Ok(false);
        }
    }
    Ok(true)
}

/// One oracle's function, self-contained for direct embedding in a generated
/// Kani harness file (`kani::generate::scalar::render_scalar`): unlike
/// [`super::render::SourceBuilder::finish`]'s crate-wide accumulation (one set of shared
/// helpers for every oracle in the crate), this always includes whatever
/// preamble helpers its own body needs, even when that duplicates them
/// across separate harness files that each embed a different oracle.
pub(super) fn operation_name(
    operation: &ExactScalarOperation,
) -> Result<String, OracleGenerationError> {
    let identity = operation_identity(operation)?;
    Ok(bounded_readable_component(
        identity.split(' ').next().unwrap_or_default(),
    ))
}

pub(super) fn operation_identity(
    operation: &ExactScalarOperation,
) -> Result<String, OracleGenerationError> {
    Ok(match operation {
        ExactScalarOperation::Boolean { operator, .. } => {
            let name = match operator {
                BooleanOperator::And => "and",
                BooleanOperator::Or => "or",
                BooleanOperator::Not => "not",
                BooleanOperator::Implies => "implies",
            };
            format!("boolean.{name}")
        }
        ExactScalarOperation::Equality { operator, operands } => {
            let kind = match operands {
                EqualityOperandKind::Boolean => "boolean",
                EqualityOperandKind::Integer => "integer",
            };
            let name = match operator {
                EqualityOperator::Equal => "eq",
                EqualityOperator::NotEqual => "ne",
            };
            format!("{kind}.{name}")
        }
        ExactScalarOperation::IntegerArithmetic { operator, domain } => {
            let name = match operator {
                IntegerOperator::Add => "add",
                IntegerOperator::Subtract => "subtract",
                IntegerOperator::Multiply => "multiply",
                IntegerOperator::Negate => "negate",
            };
            format!("integer.{name} domain={}", integer_domain_identity(domain)?)
        }
        ExactScalarOperation::IntegerDivision { profile, domain } => format!(
            "{} domain={}",
            profile.definition_identity(),
            integer_domain_identity(domain)?
        ),
        ExactScalarOperation::IntegerModulo { domain } => {
            format!("integer.modulo domain={}", integer_domain_identity(domain)?)
        }
        ExactScalarOperation::RationalArithmetic { operator, domain } => {
            let name = match operator {
                RationalOperator::Add => "add",
                RationalOperator::Subtract => "subtract",
                RationalOperator::Multiply => "multiply",
                RationalOperator::Divide => "divide",
                RationalOperator::Negate => "negate",
                RationalOperator::IntegerDivide => "integer_divide",
            };
            let domain = domain.as_ref().map_or_else(
                || "unbounded".to_owned(),
                |domain| {
                    format!(
                        "{}/{}",
                        interval_identity(domain.numerator()),
                        interval_identity(domain.denominator())
                    )
                },
            );
            format!("rational.{name} domain={domain}")
        }
        ExactScalarOperation::Ordering { operator, operands } => {
            let name = match operator {
                OrderingOperator::Less => "less",
                OrderingOperator::LessOrEqual => "less_or_equal",
                OrderingOperator::Greater => "greater",
                OrderingOperator::GreaterOrEqual => "greater_or_equal",
            };
            let kind = match operands {
                OrderingOperandKind::Integer => "integer",
                OrderingOperandKind::Rational => "rational",
                OrderingOperandKind::Decimal => "decimal",
            };
            format!("ordering.{name} operands={kind}")
        }
        ExactScalarOperation::DecimalArithmetic { operator, target } => {
            let name = match operator {
                DecimalOperator::Add => "add",
                DecimalOperator::Subtract => "subtract",
                DecimalOperator::Multiply => "multiply",
                DecimalOperator::Negate => "negate",
                DecimalOperator::Divide => "divide",
                DecimalOperator::Round => "round",
            };
            format!(
                "decimal.{name} target=[{},{}] scale=[{},{}] rounding={}",
                target.lower(),
                target.upper(),
                target.min_scale(),
                target.max_scale(),
                target.rounding().as_str()
            )
        }
        ExactScalarOperation::IeeeArithmetic {
            operator,
            width,
            rounding,
        } => {
            let name = match operator {
                IeeeArithmeticOperator::Add => "add",
                IeeeArithmeticOperator::Subtract => "subtract",
                IeeeArithmeticOperator::Multiply => "multiply",
                IeeeArithmeticOperator::Divide => "divide",
            };
            format!(
                "ieee.{name} width={} rounding={}",
                width.as_str(),
                rounding.as_str()
            )
        }
        ExactScalarOperation::IeeeComparison { comparison, width } => {
            let name = match comparison {
                IeeeComparison::NumericEqual => "numeric_equal",
                IeeeComparison::TotalOrder => "total_order",
                IeeeComparison::BitIdentical => "bit_identical",
            };
            format!("ieee.{name} width={}", width.as_str())
        }
        ExactScalarOperation::IeeeWidthConversion {
            source,
            target,
            rounding,
        } => format!(
            "ieee.convert_width source={} target={} rounding={}",
            source.as_str(),
            target.as_str(),
            rounding.as_str()
        ),
        ExactScalarOperation::TextAdmission { text_type } => format!(
            "text.admit length=[{},{}] profile={}",
            text_type.min(),
            text_type.max(),
            text_type.profile().as_str()
        ),
        ExactScalarOperation::TextComparison { operator } => {
            format!("text.compare.{}", comparison_name(*operator)?)
        }
        ExactScalarOperation::EnumComparison { operator } => {
            format!("enum.compare.{}", comparison_name(*operator)?)
        }
        ExactScalarOperation::QuantityConversion => "quantity.convert.exact".to_owned(),
        ExactScalarOperation::QuantityArithmetic { operator } => {
            let name = match operator {
                QuantityOperator::Add => "add",
                QuantityOperator::Subtract => "subtract",
                QuantityOperator::Multiply => "multiply",
                QuantityOperator::Divide => "divide",
                QuantityOperator::Power => "power",
            };
            format!("quantity.{name}")
        }
        ExactScalarOperation::QuantityComparison { operator } => {
            format!("quantity.compare.{}", comparison_name(*operator)?)
        }
    })
}

fn comparison_name(operator: ComparisonOperator) -> Result<&'static str, OracleGenerationError> {
    match operator {
        ComparisonOperator::Equal => Ok("equal"),
        ComparisonOperator::NotEqual => Ok("not_equal"),
        ComparisonOperator::Less => Ok("less"),
        ComparisonOperator::LessOrEqual => Ok("less_or_equal"),
        ComparisonOperator::Greater => Ok("greater"),
        ComparisonOperator::GreaterOrEqual => Ok("greater_or_equal"),
    }
}

fn integer_domain_identity(domain: &IntegerDomain) -> Result<String, OracleGenerationError> {
    match domain {
        IntegerDomain::Mathematical => Ok("mathematical".to_owned()),
        IntegerDomain::Bounded(interval) => Ok(interval_identity(interval)),
    }
}

fn interval_identity(interval: &IntegerInterval) -> String {
    format!("[{},{}]", interval.lower(), interval.upper())
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum SelectedScalarKey {
    NotApplicable(CheckedNodeId),
    OwnResult {
        node_id: CheckedNodeId,
        application_ordinal: u64,
        conversion: Option<(CheckedNodeId, u64)>,
    },
}

pub(super) fn selected_scalar_key(
    package: &ContractPackageV1,
    items: &PackageItems<'_>,
    request_index: usize,
    node_id: &CheckedNodeId,
    selection: &ContractDriverResultSelection,
) -> Option<SelectedScalarKey> {
    let ContractDriverResultSelection::OwnResult {
        application,
        conversion,
    } = selection
    else {
        return Some(SelectedScalarKey::NotApplicable(node_id.clone()));
    };
    #[cfg(test)]
    source_selection_work::source(true);
    package.source_occurrences(node_id)?.iter().find(|source| {
        #[cfg(test)]
        source_selection_work::occurrence(false);
        application.node_id == *node_id
            && application.role == CheckedOccurrenceRole::Expression
            && source.role == application.role
            && source.ordinal == application.ordinal
    })?;
    if let Some(conversion) = conversion {
        if conversion.role != CheckedOccurrenceRole::Expression
            || !items.has_context_root(request_index, &conversion.node_id)
        {
            return None;
        }
        #[cfg(test)]
        source_selection_work::source(true);
        package
            .source_occurrences(&conversion.node_id)?
            .iter()
            .find(|entry| {
                #[cfg(test)]
                source_selection_work::occurrence(true);
                entry.role == conversion.role && entry.ordinal == conversion.ordinal
            })?;
    }
    let ContractResultViewV1::Lowered(view) = package
        .result_context(
            node_id,
            application,
            conversion.as_ref(),
            BoundedDomainForm::IntegerRange,
        )
        .ok()?
    else {
        return None;
    };
    if matches!(view.context, ContractResultContextV1::IncompleteContext) {
        return None;
    }
    Some(SelectedScalarKey::OwnResult {
        node_id: node_id.clone(),
        application_ordinal: application.ordinal,
        conversion: conversion
            .as_ref()
            .map(|key| (key.node_id.clone(), key.ordinal)),
    })
}

pub(super) fn contract_checked_operation<'a>(
    graph: &Graph<'_>,
    closure: &'a quire_contract_model::ContractReachedClosureV1<'_>,
    tag: Option<CheckedNodeTag>,
    node_id: &CheckedNodeId,
    package: &'a ContractPackageV1,
    selection: &'a ContractDriverResultSelection,
) -> Result<
    Result<(ExactScalarOperation, Vec<CheckedNodeId>), ExactScalarRefusal>,
    OracleGenerationError,
> {
    if tag != Some(CheckedNodeTag::Expression) {
        return Ok(Err(tag.map_or_else(
            || ExactScalarRefusal::Unsupported {
                unsupported_node_id: node_id.clone(),
                node_tag: "unknown",
            },
            |tag| unsupported_family(node_id, tag),
        )));
    }
    let ContractDriverResultSelection::OwnResult {
        application,
        conversion,
    } = selection
    else {
        return Ok(Err(ExactScalarRefusal::IncompleteContext));
    };
    if &application.node_id != node_id {
        return Ok(Err(ExactScalarRefusal::IncompleteContext));
    }
    let bounds = BoundView::admitted(
        closure,
        SelectedResult {
            package,
            application,
            conversion: conversion.as_ref(),
        },
    );
    let operation = match derive_operation_from_view(graph, closure.root, bounds.clone()) {
        Ok(operation) => operation,
        Err(refusal) => return Ok(Err(refusal)),
    };
    if let Err(refusal) = contract_scalar_shape(graph, closure.root, &operation)? {
        return Ok(Err(refusal));
    }
    match check_parameters_from_view(graph, bounds, &operation) {
        Ok(checked) => Ok(Ok((operation, checked))),
        Err(ItemCheckError::Refusal(refusal)) => Ok(Err(refusal)),
        Err(ItemCheckError::Generation(error)) => Err(error),
    }
}

fn contract_scalar_shape(
    graph: &Graph<'_>,
    node: &CheckedSemanticNodeV2<Value>,
    operation: &ExactScalarOperation,
) -> Result<Result<(), ExactScalarRefusal>, OracleGenerationError> {
    let shape = Shape::of(operation)?;
    Ok((|| {
        if &*node.semantic_form != shape.form {
            return Err(ExactScalarRefusal::FormMismatch {
                expected: shape.form,
                found: node.semantic_form.to_string(),
            });
        }
        let arguments = application_arguments(graph.term(&node.node_id), shape.body_operator)
            .filter(|arguments| arguments.len() == shape.operands.len())
            .ok_or(ExactScalarRefusal::BodyMismatch {
                expected_operator: shape.body_operator,
                expected_arguments: shape.operands.len(),
            })?;
        let result = scalar_type_form(graph, &node.semantic_type);
        if result.as_deref() != Some(shape.result.wire()) {
            return Err(ExactScalarRefusal::ResultTypeMismatch {
                expected: shape.result,
                found: result,
            });
        }
        for (position, (argument, expected)) in arguments.iter().zip(shape.operands).enumerate() {
            check_operand(graph, position, argument, *expected)?;
        }
        Ok(())
    })())
}

pub(super) fn node_implication_count<Body: GraphBody, NodeBody>(
    graph: &Graph<'_, Body>,
    node: &CheckedSemanticNodeV2<NodeBody>,
) -> u32 {
    u32::from(
        graph
            .operation(&node.node_id)
            .and_then(|op| op.get("identity"))
            .and_then(Value::as_str)
            == Some("quire.op.boolean.implies"),
    )
}

/// Checked selected claims before oracle source is rendered.
pub(super) struct ContractScalarPlan {
    pub(super) claims: Vec<ExactScalarClaim>,
    pub(super) pending: Vec<(usize, ExactScalarOperation, u32)>,
    pub(super) selected_keys: Vec<Option<SelectedScalarKey>>,
}

pub(super) fn plan_contract_scalar_oracles(
    package: &ContractPackageV1,
    requested: &[ContractRequestPosition],
    positions: &[usize],
) -> Result<ContractScalarPlan, super::ContractScalarError> {
    let items =
        PackageItems::new(package, requested).map_err(super::ContractScalarError::Request)?;
    let graph = Graph::new(package);
    let mut claims = Vec::with_capacity(positions.len());
    let mut pending = Vec::new();
    let mut counts = BTreeMap::new();
    let mut selected_keys = Vec::with_capacity(positions.len());
    for &request_index in positions {
        let node_id = items
            .bound_driver_node(request_index)
            .map_err(super::ContractScalarError::Request)?;
        let selection =
            items
                .result_selection(request_index)
                .ok_or(super::ContractScalarError::Request(
                    PackageRequestError::Position { request_index },
                ))?;
        let key = selected_scalar_key(package, &items, request_index, node_id, selection);
        if let Some(key) = &key {
            *counts.entry(key.clone()).or_insert(0usize) += 1;
        }
        selected_keys.push(key);
    }
    for (&request_index, selected_key) in positions.iter().zip(&selected_keys) {
        let node_id = items
            .bound_driver_node(request_index)
            .map_err(super::ContractScalarError::Request)?;
        let item = items
            .at(request_index, node_id)
            .map_err(super::ContractScalarError::Request)?;
        let (identity, result) = match item {
            PackageItem::Refused {
                record,
                occurrences,
            } => (
                String::new(),
                ClaimDisposition::Refused {
                    refusal: ExactScalarRefusal::PackageAccounting {
                        record: Box::new(record.clone()),
                        occurrences: occurrences.to_vec(),
                    },
                },
            ),
            PackageItem::Lowered { closure, .. } => {
                let root_sources = closure
                    .nodes
                    .iter()
                    .find(|entry| entry.node.node_id == *node_id)
                    .map(|entry| entry.sources)
                    .ok_or_else(|| {
                        super::ContractScalarError::Request(PackageRequestError::ReachedClosure {
                            request_index,
                            node_id: node_id.clone(),
                            cause: Box::new(ContractRootViewError::Inconsistent),
                        })
                    })?;
                let identity = graph
                    .operation(&closure.root.node_id)
                    .and_then(|operation| operation.get("identity"))
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned();
                let tag = CheckedNodeTag::from_wire(&closure.root.node_tag);
                if selected_key.is_none() {
                    (
                        identity,
                        ClaimDisposition::Refused {
                            refusal: ExactScalarRefusal::IncompleteContext,
                        },
                    )
                } else if selected_key
                    .as_ref()
                    .is_some_and(|key| counts.get(key).copied().unwrap_or(0) > 1)
                {
                    (
                        identity,
                        ClaimDisposition::Refused {
                            refusal: ExactScalarRefusal::DuplicateRequest,
                        },
                    )
                } else if !closure.selected_references.is_empty() {
                    (
                        identity,
                        ClaimDisposition::Refused {
                            refusal: ExactScalarRefusal::SelectedDependencyUnsupported {
                                references: closure
                                    .selected_references
                                    .into_iter()
                                    .map(|reference| (reference.package, reference.node))
                                    .collect(),
                            },
                        },
                    )
                } else {
                    let selection = items.result_selection(request_index).ok_or(
                        super::ContractScalarError::Request(PackageRequestError::Position {
                            request_index,
                        }),
                    )?;
                    match contract_checked_operation(
                        &graph, &closure, tag, node_id, package, selection,
                    )
                    .map_err(super::ContractScalarError::Oracle)?
                    {
                        Ok((operation, checked_bounds)) => {
                            let dependencies = closure
                                .nodes
                                .iter()
                                .filter(|entry| entry.node.node_id != *node_id)
                                .map(|entry| entry.node.node_id.clone())
                                .collect();
                            pending.push((
                                claims.len(),
                                operation.clone(),
                                node_implication_count(&graph, closure.root),
                            ));
                            (
                                identity,
                                ClaimDisposition::Generated(Box::new(GeneratedScalarClaim {
                                    symbol: String::new(),
                                    contract_package_identity: package.identity().clone(),
                                    node_id: node_id.clone(),
                                    package_id: package.wire().package.source.package_id.clone(),
                                    semantic_form: closure.root.semantic_form.to_string(),
                                    semantic_type: closure.root.semantic_type.clone(),
                                    quantity_conversion_target: conversion_target(
                                        &graph,
                                        closure.root,
                                    ),
                                    source_map: root_sources.to_vec(),
                                    claims: closure.claims,
                                    bounds: closure.bounds,
                                    checked_bounds,
                                    dependencies,
                                    oracle_source: String::new(),
                                    boolean_right_may_stop: matches!(
                                        operation,
                                        ExactScalarOperation::Boolean {
                                            right_may_stop: true,
                                            ..
                                        }
                                    ),
                                })),
                            )
                        }
                        Err(refusal) => (identity, ClaimDisposition::Refused { refusal }),
                    }
                }
            }
        };
        let provenance = if matches!(&result, ClaimDisposition::Generated(_)) {
            OperationProvenance::IrConfirmed
        } else {
            OperationProvenance::Underived
        };
        claims.push(ExactScalarClaim {
            node_id: node_id.clone(),
            operation: OperationClaim {
                identity,
                provenance,
            },
            result,
        });
    }
    Ok(ContractScalarPlan {
        claims,
        pending,
        selected_keys,
    })
}

/// Build the complete checked scalar plan without publishing generated artifacts.
pub(super) fn plan_exact_scalar_oracles(
    package: &CheckedPackageV2,
    items: &[ExactScalarItem],
) -> Result<ExactScalarPlan, OracleGenerationError> {
    let mut requests: BTreeMap<&CheckedNodeId, Vec<&ExactScalarOperation>> = BTreeMap::new();
    for item in items {
        requests
            .entry(&item.node_id)
            .or_default()
            .push(&item.operation);
    }
    let requested = requests.keys().map(|id| (*id).clone()).collect::<Vec<_>>();
    let lowering = package.lower(&requested, &scalar_profile());
    let graph = Graph::from_checked(package);

    let mut claims = Vec::with_capacity(requests.len());
    let mut pending: Vec<PendingOraclePlan> = Vec::new();
    for ((node_id, operations), record) in requests.into_iter().zip(&lowering.records) {
        let operation_claim = match operations.as_slice() {
            [operation] => match check_item(&graph, node_id, record, operation) {
                Ok((node, checked_bounds)) => {
                    // This node was lowered from an admitted package, so IR-216's
                    // `validate_operations` already confirmed `body.operation.identity`
                    // against the closed catalog before this generator ever saw it. That
                    // confirms the node's own operation, not that it is the one the
                    // descriptor named: `check_item`'s shape checks alone cannot
                    // distinguish, say, `add` from `mul` (both are a binary
                    // `&rt::Integer` shape) or truncating from floor division, so
                    // `operation_confirmed` additionally compares the node's own
                    // `operation.identity`/`operation.mode` against the descriptor's
                    // implied catalog entry, plus the one law family with more than
                    // one catalogued definition. Confirmed, this generator reads the
                    // node's own identity rather than re-deriving codegen's own
                    // descriptor string; not confirmed, it reports the descriptor's
                    // own string exactly as before this generator ever read
                    // `operation.identity` -- never a silent preference between the
                    // two when they disagree.
                    // The node's own identity is read before confirmation, not
                    // behind it. `operation_confirmed` returns false for a node
                    // missing `operation.identity` just as it does for one whose
                    // identity simply disagrees, so asking it first collapsed a
                    // violated upstream invariant into the ordinary
                    // caller-declared answer -- reporting that codegen had not
                    // consumed an identity the node does not carry. Reading it
                    // first keeps the two apart: absent is refused, present and
                    // disagreeing is caller-declared.
                    let confirmed_identity = match catalogued_operation_identity(&graph, node) {
                        Ok(catalogued) => {
                            Ok(operation_confirmed(&graph, node, operation)?.then_some(catalogued))
                        }
                        Err(refusal) => Err(refusal),
                    };
                    match confirmed_identity {
                        Err(refusal) => refused_claim(operation_identity(operation)?, refusal),
                        Ok(maybe_identity) => {
                            let confirmed = maybe_identity.is_some();
                            let (identity, provenance) = match maybe_identity {
                                Some(identity) => (identity, OperationProvenance::IrConfirmed),
                                None => (
                                    operation_identity(operation)?,
                                    OperationProvenance::CallerDeclared {
                                        blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
                                    },
                                ),
                            };
                            pending.push(PendingOraclePlan {
                                claim: claims.len(),
                                identity: identity.clone(),
                                confirmed,
                                operation: (*operation).clone(),
                                expected_consequents: node_implication_count(&graph, &node.node),
                            });
                            (
                                OperationClaim {
                                    identity,
                                    provenance,
                                },
                                ClaimDisposition::Generated(Box::new(GeneratedScalarClaim {
                                    symbol: String::new(),
                                    contract_package_identity: lowering
                                        .package
                                        .wire()
                                        .ok_or(OracleGenerationError::ContractPackageUnavailable)?
                                        .identity()
                                        .clone(),
                                    node_id: node_id.clone(),
                                    package_id: lowering.package.source_package_id().clone(),
                                    semantic_form: node.node.semantic_form.to_string(),
                                    semantic_type: node.semantic_type.clone(),
                                    quantity_conversion_target: conversion_target(
                                        &graph, &node.node,
                                    ),
                                    source_map: node.source_map.clone(),
                                    claims: node.claims.clone(),
                                    bounds: node.bounds.clone(),
                                    checked_bounds,
                                    dependencies: node.dependencies.clone(),
                                    oracle_source: String::new(),
                                    boolean_right_may_stop: matches!(
                                        operation,
                                        ExactScalarOperation::Boolean {
                                            right_may_stop: true,
                                            ..
                                        }
                                    ),
                                })),
                            )
                        }
                    }
                }
                Err(ItemCheckError::Refusal(refusal)) => {
                    refused_claim(operation_identity(operation)?, refusal)
                }
                // An RT `#[non_exhaustive]` enum yielded a variant
                // `check_parameters`'s own exhaustive match does not know:
                // this is not this one item's business refusal, so it aborts
                // the whole generation instead (see
                // `OracleGenerationError::UnknownRuntimeVariant`'s own doc).
                Err(ItemCheckError::Generation(error)) => return Err(error),
            },
            // Every copy is refused. The entry is named by the least identity
            // so that it does not depend on which copy arrived first.
            copies => {
                let identities = copies
                    .iter()
                    .map(|op| operation_identity(op))
                    .collect::<Result<Vec<_>, _>>()?;
                let Some(identity) = identities.into_iter().min() else {
                    continue;
                };
                refused_claim(identity, ExactScalarRefusal::DuplicateRequest)
            }
        };
        let (claim_operation, result) = operation_claim;
        claims.push(ExactScalarClaim {
            node_id: node_id.clone(),
            operation: claim_operation,
            result,
        });
    }
    Ok(ExactScalarPlan {
        package_id: lowering.package.source_package_id().clone(),
        claims,
        pending,
    })
}

pub(super) fn scalar_profile() -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
        reference: quire_contract_model::LoweringProfileRef {
            authority: "agent-ix".into(),
            identity: "quire.codegen.exact-scalar/v1".into(),
        },
        supported_tags: BTreeSet::from(SCALAR_LOWERING_SUPPORTED_TAGS),
        require_bounds: true,
        work_limit: SCALAR_LOWERING_WORK_LIMIT,
    }
}

#[cfg(test)]
mod source_selection_tests;

#[cfg(test)]
mod source_selection_work {
    use std::cell::Cell;

    #[derive(Clone, Copy, Debug, Default)]
    pub(super) struct Work {
        pub(super) whole_map_visits: usize,
        pub(super) indexed_lookups: usize,
        pub(super) application_occurrences: usize,
        pub(super) conversion_occurrences: usize,
    }

    thread_local! {
        static WORK: Cell<Work> = const { Cell::new(Work {
            whole_map_visits: 0,
            indexed_lookups: 0,
            application_occurrences: 0,
            conversion_occurrences: 0,
        }) };
    }

    // The historical qualification attaches false to each executed whole-map
    // predicate; the retained path attaches true to each indexed API call.
    pub(super) fn source(indexed: bool) {
        WORK.with(|work| {
            let mut current = work.get();
            if indexed {
                current.indexed_lookups += 1;
            } else {
                current.whole_map_visits += 1;
            }
            work.set(current);
        });
    }

    pub(super) fn occurrence(conversion: bool) {
        WORK.with(|work| {
            let mut current = work.get();
            if conversion {
                current.conversion_occurrences += 1;
            } else {
                current.application_occurrences += 1;
            }
            work.set(current);
        });
    }

    pub(super) fn take() -> Work {
        WORK.with(|work| work.replace(Work::default()))
    }
}

#[cfg(test)]
pub(in crate::oracle::scalar) mod tests;

#[cfg(test)]
mod opaque_metadata_tests;

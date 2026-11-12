//! Reading a routed item's path guards from the admitted package (FR-022, IR-304).
//!
//! The guard's node is a context root of the item, so the package holds its accounting and, when
//! lowered, its expression. A guard is lowerable as a direct Boolean parameter reference or an
//! IR-confirmed integer comparison between bounded-integer parameters and `i64` literals.

use quire_contract_model::{
    CheckedNodeId, CheckedNodeTag, CheckedTerm, ContractFeatureRecordV1, ContractSourceKeyV1,
};

use super::{CheckedGraph, ContractLookup};
use crate::{
    core::ir::{
        checked_term::{aggregate_members, literal_integer},
        scalar::identity,
        scalar_bounds::read_integer_range,
    },
    core::requirement::RequirementGuard,
    kani::generate::outcome::PathGuardRefusal,
    kani::generate::scalar::guard::{
        Comparison, DeclaredDomain, GuardForm, GuardParameter, GuardPlan, GuardRefusal, GuardSide,
        ResolvedGuard,
    },
    oracle::scalar::lower::scalar_type_form,
};

/// One comparison operand, or why it cannot be one.
enum Side {
    Parameter {
        node: CheckedNodeId,
        lower: String,
        upper: String,
    },
    Literal(i64),
}

/// Why an operand is not a lowerable side.
enum Fault {
    /// Neither a parameter reference nor an integer literal.
    Form,
    /// An integer parameter with no bounded domain.
    Unbounded(CheckedNodeId),
    /// An integer literal outside `i64`.
    LiteralNotI64(String),
}

/// Lower the item's path condition, or name the first guard that is not lowered. `operand_targets`
/// holds, per operand position in call order, the V2 parameter the position directly references.
pub(super) fn resolve_guards(
    lookup: &ContractLookup<'_>,
    path_condition: &[RequirementGuard],
    operand_targets: &[Option<CheckedNodeId>],
) -> Result<GuardPlan, GuardRefusal> {
    let mut plan = GuardPlan {
        path_condition: path_condition.to_vec(),
        ..GuardPlan::default()
    };
    for guard in path_condition {
        let refuse = |reason| Err(Box::new((guard.occurrence.clone(), reason)));
        let node = lookup
            .admitted_node(&guard.occurrence.node_id)
            .filter(|_| root_lowered(lookup, &guard.occurrence.node_id))
            .and_then(|admitted| lookup.node(admitted));
        let Some(node) = node else {
            return refuse(PathGuardRefusal::GuardRootNotLowered);
        };
        let form = if is_boolean_parameter(lookup, node) {
            GuardForm::Boolean(parameter_index(
                &mut plan,
                &guard.occurrence.node_id,
                DeclaredDomain::Boolean,
                operand_targets,
            ))
        } else {
            let Some((operator, left, right)) = comparison(lookup, &node.node_id) else {
                return refuse(PathGuardRefusal::UnsupportedGuardForm);
            };
            let sides = [left, right];
            if sides.iter().any(|side| matches!(side, Err(Fault::Form)))
                || matches!(sides, [Ok(Side::Literal(_)), Ok(Side::Literal(_))])
            {
                return refuse(PathGuardRefusal::UnsupportedGuardForm);
            }
            if let Some(parameter) = sides.iter().find_map(|side| match side {
                Err(Fault::Unbounded(node)) => Some(node.clone()),
                _ => None,
            }) {
                return refuse(PathGuardRefusal::UnboundedGuardOperand { parameter });
            }
            if let Some(value) = sides.iter().find_map(|side| match side {
                Err(Fault::LiteralNotI64(value)) => Some(value.clone()),
                _ => None,
            }) {
                return refuse(PathGuardRefusal::GuardLiteralNotI64 { value });
            }
            let [left, right] = sides.map(|side| match side {
                Ok(Side::Parameter { node, lower, upper }) => {
                    GuardSide::Parameter(parameter_index(
                        &mut plan,
                        &node,
                        DeclaredDomain::Integer { lower, upper },
                        operand_targets,
                    ))
                }
                Ok(Side::Literal(value)) => GuardSide::Literal(value),
                Err(_) => GuardSide::Literal(0),
            });
            GuardForm::Compare {
                operator,
                left,
                right,
            }
        };
        plan.guards.push(ResolvedGuard {
            occurrence: guard.occurrence.clone(),
            holds: guard.holds,
            form,
        });
    }
    Ok(plan)
}

/// The index of `node` among the plan's parameters, adding it on first read.
fn parameter_index(
    plan: &mut GuardPlan,
    node: &CheckedNodeId,
    declared: DeclaredDomain,
    operand_targets: &[Option<CheckedNodeId>],
) -> usize {
    if let Some(index) = plan
        .parameters
        .iter()
        .position(|parameter| &parameter.node == node)
    {
        return index;
    }
    plan.parameters.push(GuardParameter {
        node: node.clone(),
        declared,
        operand_positions: operand_targets
            .iter()
            .enumerate()
            .filter_map(|(position, target)| (target.as_ref() == Some(node)).then_some(position))
            .collect(),
    });
    plan.parameters.len() - 1
}

/// Whether the guard's own accounting record in the admitted package is `lowered`.
fn root_lowered(lookup: &ContractLookup<'_>, guard: &CheckedNodeId) -> bool {
    let Some(requested) = ContractSourceKeyV1::from_v2_request(lookup.source, guard) else {
        return false;
    };
    let wire = lookup.package.wire();
    wire.feature_accounting.iter().any(|record| {
        matches!(record, ContractFeatureRecordV1::Lowered { .. })
            && wire
                .source_keys
                .iter()
                .position(|key| key == record.node_id())
                .and_then(|index| lookup.package.source_key_at(index))
                .is_some_and(|key| key == requested)
    })
}

/// Whether the node is a parameter typed Boolean.
fn is_boolean_parameter(
    lookup: &ContractLookup<'_>,
    node: &quire_contract_model::CheckedSemanticNodeV2<quire_contract_model::CheckedJson>,
) -> bool {
    &*node.semantic_form == "parameter"
        && scalar_type_form(&lookup.graph, &node.semantic_type).as_deref() == Some("boolean")
}

type Comparison3 = (Comparison, Result<Side, Fault>, Result<Side, Fault>);

/// The operator and both operands of an IR-confirmed integer comparison.
fn comparison(lookup: &ContractLookup<'_>, node: &CheckedNodeId) -> Option<Comparison3> {
    let operator = Comparison::from_identity(identity(lookup.graph.term(node))?)?;
    let [left, right] = lookup.graph.arguments(node)? else {
        return None;
    };
    Some((operator, side(lookup, left), side(lookup, right)))
}

/// The side a conversion of `inner` is: only a literal passes through it, with a literal outside
/// `i64` still reported as such; a conversion over a parameter is not that parameter.
fn through_conversion(inner: Result<Side, Fault>) -> Result<Side, Fault> {
    match inner {
        literal @ (Ok(Side::Literal(_)) | Err(Fault::LiteralNotI64(_))) => literal,
        _ => Err(Fault::Form),
    }
}

fn side(lookup: &ContractLookup<'_>, term: &CheckedTerm) -> Result<Side, Fault> {
    let graph: &CheckedGraph<'_> = &lookup.graph;
    let literal = |term: &CheckedTerm| {
        let value = literal_integer(term).ok_or(Fault::Form)?.to_string();
        value
            .parse::<i64>()
            .map(Side::Literal)
            .map_err(|_| Fault::LiteralNotI64(value))
    };
    let target = match term {
        CheckedTerm::Literal { .. } => return literal(term),
        CheckedTerm::Reference { target } => target,
        _ => return Err(Fault::Form),
    };
    let node = graph.node(target).ok_or(Fault::Form)?;
    if &*node.node_tag == CheckedNodeTag::Value.as_wire() {
        if let Some(body @ CheckedTerm::Literal { .. }) = graph.term(&node.node_id) {
            return literal(body);
        }
    }
    // QSL widens a literal compared with a differently ranged operand through a conversion.
    // QSL proves the literal lies in the conversion's target range before it admits the package
    // (an out-of-range literal is an `Unproved(Range)` check refusal and no such package exists),
    // so the conversion keeps the literal's value and needs no second range check here.
    if identity(graph.term(&node.node_id)) == Some("quire.op.numeric.narrow") {
        if let Some([argument]) = graph.arguments(&node.node_id) {
            return through_conversion(side(lookup, argument));
        }
    }
    let original = lookup.original_node(&node.node_id).ok_or(Fault::Form)?;
    if &*node.semantic_form != "parameter" {
        return Err(Fault::Form);
    }
    let typing = graph.node(&node.semantic_type).ok_or(Fault::Form)?;
    if &*typing.node_tag == CheckedNodeTag::BoundedDomain.as_wire() {
        let range = aggregate_members(graph.term(&typing.node_id))
            .and_then(read_integer_range)
            .ok_or(Fault::Form)?;
        return Ok(Side::Parameter {
            node: original.clone(),
            lower: range.lower().to_string(),
            upper: range.upper().to_string(),
        });
    }
    if scalar_type_form(graph, &node.semantic_type).as_deref() == Some("integer") {
        Err(Fault::Unbounded(original.clone()))
    } else {
        Err(Fault::Form)
    }
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;
    use serde_json::json;

    use super::*;

    fn node() -> CheckedNodeId {
        serde_json::from_value(json!({"package": "guard-test", "path": ["x"], "ordinal": 0}))
            .expect("node id")
    }

    /// A conversion reads through to a literal, keeps a non-`i64` literal's reason, and is never
    /// the parameter or unbounded parameter it converts.
    #[trace("FR-022-AC-32", "FR-022-AC-33", "TC-033")]
    #[test]
    fn a_conversion_passes_only_a_literal_through() {
        assert!(matches!(
            through_conversion(Ok(Side::Literal(5))),
            Ok(Side::Literal(5))
        ));
        assert!(matches!(
            through_conversion(Err(Fault::LiteralNotI64("100000000000000000000000".to_owned()))),
            Err(Fault::LiteralNotI64(value)) if value == "100000000000000000000000"
        ));
        let parameter = Side::Parameter {
            node: node(),
            lower: "0".to_owned(),
            upper: "9".to_owned(),
        };
        assert!(matches!(
            through_conversion(Ok(parameter)),
            Err(Fault::Form)
        ));
        assert!(matches!(
            through_conversion(Err(Fault::Unbounded(node()))),
            Err(Fault::Form)
        ));
        assert!(matches!(
            through_conversion(Err(Fault::Form)),
            Err(Fault::Form)
        ));
    }
}

// SPDX-License-Identifier: AGPL-3.0-or-later
//! Whole-object collection clause lowering from the canonical admitted graph.

use super::{
    aggregate_members, application_arguments, binding, parameter_name, resolve_collection_field,
    state_clause_parts, term_target, CollectionClaimRole, CollectionDomainRefusal,
    CollectionElementDomain, CollectionFieldDomain, CollectionGraph,
};
use crate::kani::{
    generate::{
        outcome::CollectionUnsupportedReason,
        spec::{CollectionOperationSpec, ExprText},
    },
    identity::{
        CollectionElementIdentity, CollectionKindIdentity, CollectionObjectFieldIdentity,
        CollectionOperationConditionIdentity, CollectionOperationIdentity,
        CollectionOperationValueIdentity, SymbolicBounds,
    },
};
use quire_contract_model::{
    CheckedCollectionKind, CheckedMemberType, CheckedNodeId, CheckedPackageV2, CheckedTerm,
    LiteralKind, StateClauseKind,
};
use std::collections::BTreeSet;

struct ConditionBindings<'a> {
    fields: &'a [CollectionObjectFieldIdentity],
    parameters: &'a [CollectionOperationValueIdentity],
    result: Option<&'a CollectionOperationValueIdentity>,
}

pub(super) struct LoweredOperation {
    pub(super) spec: CollectionOperationSpec,
    pub(super) package_instance: *const CheckedPackageV2,
}

fn refused(reason: CollectionUnsupportedReason) -> CollectionDomainRefusal {
    CollectionDomainRefusal::OperationContext(reason)
}

fn primitive(
    domain: CollectionElementDomain,
) -> Result<CollectionElementIdentity, CollectionDomainRefusal> {
    match domain {
        CollectionElementDomain::Boolean => Ok(CollectionElementIdentity::Boolean),
        CollectionElementDomain::Integer { minimum, maximum } => {
            Ok(CollectionElementIdentity::Integer {
                minimum: i64::try_from(minimum).map_err(|_| {
                    refused(CollectionUnsupportedReason::DomainNotRepresentableInI64)
                })?,
                maximum: i64::try_from(maximum).map_err(|_| {
                    refused(CollectionUnsupportedReason::DomainNotRepresentableInI64)
                })?,
            })
        }
    }
}

fn field_domain(
    member: &CheckedMemberType,
) -> Result<(SymbolicBounds, Option<CollectionKindIdentity>), CollectionDomainRefusal> {
    let scalar = |member: &CheckedMemberType| match member {
        CheckedMemberType::Boolean => Ok(SymbolicBounds::Boolean),
        CheckedMemberType::IntRange { lower, upper } if lower <= upper => {
            Ok(SymbolicBounds::Integer {
                minimum: i64::try_from(*lower).map_err(|_| {
                    refused(CollectionUnsupportedReason::DomainNotRepresentableInI64)
                })?,
                maximum: i64::try_from(*upper).map_err(|_| {
                    refused(CollectionUnsupportedReason::DomainNotRepresentableInI64)
                })?,
            })
        }
        _ => Err(refused(CollectionUnsupportedReason::UnsupportedObjectState)),
    };
    match member {
        CheckedMemberType::Collection {
            kind,
            element,
            bounds: Some((minimum, maximum)),
        } if minimum <= maximum => {
            let kind = match kind {
                CheckedCollectionKind::Sequence => CollectionKindIdentity::Sequence,
                CheckedCollectionKind::OrderedSet => CollectionKindIdentity::OrderedSet,
                _ => return Err(refused(CollectionUnsupportedReason::UnsupportedObjectState)),
            };
            Ok((
                SymbolicBounds::Sequence {
                    minimum_length: i64::try_from(*minimum).map_err(|_| {
                        refused(CollectionUnsupportedReason::DomainNotRepresentableInHarness)
                    })?,
                    maximum_length: i64::try_from(*maximum).map_err(|_| {
                        refused(CollectionUnsupportedReason::DomainNotRepresentableInHarness)
                    })?,
                    element: Box::new(scalar(element)?),
                },
                Some(kind),
            ))
        }
        _ => Ok((scalar(member)?, None)),
    }
}

fn parameter(
    graph: &CollectionGraph<'_>,
    id: &CheckedNodeId,
    index: usize,
) -> Result<CollectionOperationValueIdentity, CollectionDomainRefusal> {
    let failure = || refused(CollectionUnsupportedReason::UnsupportedOperationParameter);
    let node = graph.index.node(id).ok_or_else(failure)?;
    let ty = graph.index.node(&node.semantic_type).ok_or_else(failure)?;
    let domain = match ty.semantic_form.as_ref() {
        "boolean" => CollectionElementDomain::Boolean,
        "integer_range" => {
            let members = graph
                .index
                .term(ty)
                .and_then(aggregate_members)
                .ok_or_else(failure)?;
            let number = |name| match binding(members, name)? {
                CheckedTerm::Literal { value, .. } => value.as_str()?.parse::<i128>().ok(),
                _ => None,
            };
            let (minimum, maximum) = (
                number("min").ok_or_else(failure)?,
                number("max").ok_or_else(failure)?,
            );
            if minimum > maximum {
                return Err(failure());
            }
            CollectionElementDomain::Integer { minimum, maximum }
        }
        _ => return Err(failure()),
    };
    Ok(CollectionOperationValueIdentity {
        node_id: id.clone(),
        type_id: node.semantic_type.clone(),
        name: parameter_name(&graph.index, node)
            .ok_or_else(failure)?
            .to_owned(),
        binding: format!("parameter_{index}"),
        domain: primitive(domain)?,
    })
}

fn anchor(graph: &CollectionGraph<'_>, clause: &CheckedNodeId) -> Option<CheckedNodeId> {
    let node = graph.index.node(clause)?;
    let arguments = graph.index.term(node).and_then(application_arguments)?;
    term_target(arguments.get(1)?)
}

pub(super) fn lower(
    graph: &CollectionGraph<'_>,
    clause: &CheckedNodeId,
    selected: &CollectionFieldDomain,
) -> Result<LoweredOperation, CollectionDomainRefusal> {
    let failure = || refused(CollectionUnsupportedReason::UnsupportedClaimShape);
    let node = graph.index.node(clause).ok_or_else(failure)?;
    let (kind, declared, body) = state_clause_parts(&graph.index, node).ok_or_else(failure)?;
    let anchor_id = anchor(graph, clause).ok_or_else(failure)?;
    let fields = graph
        .package
        .model_object_fields(&selected.declaration)
        .map_err(|_| refused(CollectionUnsupportedReason::FieldNotAdmitted))?;
    let fields = fields
        .fields()
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let (bounds, collection_kind) =
                field_domain(field.member_type().ok_or_else(|| {
                    refused(CollectionUnsupportedReason::UnsupportedObjectState)
                })?)?;
            Ok(CollectionObjectFieldIdentity {
                declaring_node_id: field.declaring_node_id().clone(),
                field_name: field.name().to_owned(),
                slot: format!("field_{index}"),
                bounds,
                collection_kind,
            })
        })
        .collect::<Result<Vec<_>, CollectionDomainRefusal>>()?;
    let selected_field = fields
        .iter()
        .position(|field| field.field_name == selected.field_name)
        .ok_or_else(|| refused(CollectionUnsupportedReason::FieldNotAdmitted))?;
    let mut parameters = Vec::new();
    let mut result = None;
    for term in declared.iter().skip(1) {
        let id = term_target(term).ok_or_else(failure)?;
        let is_result = kind == StateClauseKind::Postcondition
            && graph
                .index
                .node(&id)
                .and_then(|node| parameter_name(&graph.index, node))
                == Some("result");
        let mut value = parameter(graph, &id, parameters.len()).map_err(|reason| {
            if is_result {
                refused(CollectionUnsupportedReason::UnsupportedOperationResult)
            } else {
                reason
            }
        })?;
        if is_result {
            value.binding = "subject_result".to_owned();
            if result.replace(value).is_some() {
                return Err(refused(
                    CollectionUnsupportedReason::UnsupportedOperationResult,
                ));
            }
        } else {
            parameters.push(value);
        }
    }
    let primary = condition(
        graph,
        clause,
        &body,
        &ConditionBindings {
            fields: &fields,
            parameters: &parameters,
            result: result.as_ref(),
        },
        "state",
        &mut 256usize,
    )
    .ok_or_else(failure)?;
    let identity = CollectionOperationIdentity {
        quantifier_id: selected.quantifier.clone(),
        projection_id: selected.projection.clone(),
        predicate_id: selected.predicate_id.clone(),
        predicate: selected.predicate.clone(),
        anchor_id: anchor_id.clone(),
        fields,
        selected_field,
        parameters,
        result,
        preconditions: Vec::new(),
        condition: primary,
        precondition_bodies: Vec::new(),
    };
    let mut spec = CollectionOperationSpec {
        identity,
        postcondition: kind == StateClauseKind::Postcondition,
        subject_path: String::new(),
    };
    if spec.postcondition {
        for sibling in graph.preconditions.get(&anchor_id).into_iter().flatten() {
            let Some((StateClauseKind::Precondition, _, body)) =
                state_clause_parts(&graph.index, sibling)
            else {
                continue;
            };
            let condition = condition(
                graph,
                &sibling.node_id,
                &body,
                &ConditionBindings {
                    fields: &spec.identity.fields,
                    parameters: &spec.identity.parameters,
                    result: None,
                },
                "pre",
                &mut 256usize,
            )
            .ok_or_else(|| {
                refused(
                    CollectionUnsupportedReason::UnsupportedSiblingPrecondition {
                        precondition: sibling.node_id.clone(),
                    },
                )
            })?;
            spec.identity.preconditions.push(sibling.node_id.clone());
            spec.identity.precondition_bodies.push(condition);
        }
    }
    Ok(LoweredOperation {
        spec,
        package_instance: std::ptr::from_ref(graph.package),
    })
}

pub(super) fn first_quantifier<'a>(
    graph: &CollectionGraph<'a>,
    body: &CheckedNodeId,
) -> Option<&'a quire_contract_model::CheckedSemanticNodeV2> {
    let mut pending = vec![body.clone()];
    let mut budget = 256usize;
    while let Some(id) = pending.pop() {
        budget = budget.checked_sub(1)?;
        let node = graph.index.node(&id)?;
        let CheckedTerm::Application {
            operation,
            arguments,
            ..
        } = graph.index.term(node)?
        else {
            continue;
        };
        match operation.identity()? {
            "quire.op.collection.forall" | "quire.op.collection.exists" => return Some(node),
            "quire.op.boolean.and" => {
                for argument in arguments.iter().rev() {
                    if let Some(target) = term_target(argument) {
                        pending.push(target);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn collection_read(
    graph: &CollectionGraph<'_>,
    clause: &CheckedNodeId,
    term: &CheckedTerm,
    context: &ConditionBindings<'_>,
    current: &str,
) -> Option<String> {
    let node = graph.index.node(&term_target(term)?)?;
    let CheckedTerm::Application {
        operation,
        arguments,
        ..
    } = graph.index.term(node)?
    else {
        return None;
    };
    if operation.identity()? == "quire.op.state.pre" {
        return collection_read(graph, clause, arguments.first()?, context, "pre");
    }
    if operation.identity()? != "quire.op.record.project" {
        return None;
    }
    let (declaration, name) = operation.projection_field().ok()??;
    super::verify_claim_receiver(&graph.index, graph.index.node(clause)?, node, declaration)
        .ok()?;
    let fields = graph.package.model_object_fields(declaration).ok()?;
    let field = fields.field(name)?;
    let index = context.fields.iter().position(|candidate| {
        candidate.field_name == name && candidate.declaring_node_id == *field.declaring_node_id()
    })?;
    matches!(
        context.fields[index].bounds,
        SymbolicBounds::Sequence { .. }
    )
    .then(|| format!("&{current}.field_{index}"))
}

fn condition(
    graph: &CollectionGraph<'_>,
    clause: &CheckedNodeId,
    body: &CheckedNodeId,
    context: &ConditionBindings<'_>,
    current: &str,
    budget: &mut usize,
) -> Option<CollectionOperationConditionIdentity> {
    *budget = budget.checked_sub(1)?;
    let node = graph.index.node(body)?;
    if let Some(CheckedTerm::Application {
        operation,
        arguments,
        ..
    }) = graph.index.term(node)
    {
        if operation.identity()? == "quire.op.boolean.and" {
            let [left, right] = arguments.as_slice() else {
                return None;
            };
            return Some(CollectionOperationConditionIdentity::And(
                Box::new(condition_operand(
                    graph, clause, left, context, current, budget,
                )?),
                Box::new(condition_operand(
                    graph, clause, right, context, current, budget,
                )?),
            ));
        }
        if operation.identity()? == "quire.op.structural.eq" {
            let [left, right] = arguments.as_slice() else {
                return None;
            };
            let (left, right) = (
                collection_read(graph, clause, left, context, current)?,
                collection_read(graph, clause, right, context, current)?,
            );
            return Some(CollectionOperationConditionIdentity::Scalar {
                expression: ExprText::new(format!("structural_equal({left}, {right})"))
                    .ok()?
                    .as_str()
                    .to_owned(),
            });
        }
    }
    let role = if state_clause_parts(&graph.index, graph.index.node(clause)?)?.0
        == StateClauseKind::Postcondition
    {
        CollectionClaimRole::Postcondition
    } else {
        CollectionClaimRole::Precondition
    };
    if let Ok(collection) =
        resolve_collection_field(graph.package, &graph.index, clause, node, role)
    {
        let field = context
            .fields
            .iter()
            .position(|field| field.field_name == collection.field_name)?;
        return Some(CollectionOperationConditionIdentity::Quantifier {
            field,
            operation: collection.quantifier_operation,
            predicate: collection.predicate,
        });
    }
    let mut visiting = BTreeSet::new();
    let expression = scalar_condition(
        graph,
        clause,
        &CheckedTerm::Reference {
            target: body.clone(),
        },
        context,
        &mut visiting,
        budget,
        current,
    )?;
    Some(CollectionOperationConditionIdentity::Scalar {
        expression: ExprText::new(expression).ok()?.as_str().to_owned(),
    })
}

fn condition_operand(
    graph: &CollectionGraph<'_>,
    clause: &CheckedNodeId,
    term: &CheckedTerm,
    context: &ConditionBindings<'_>,
    current: &str,
    budget: &mut usize,
) -> Option<CollectionOperationConditionIdentity> {
    if let CheckedTerm::Reference { target } = term {
        return condition(graph, clause, target, context, current, budget);
    }
    let expression = scalar_condition(
        graph,
        clause,
        term,
        context,
        &mut BTreeSet::new(),
        budget,
        current,
    )?;
    Some(CollectionOperationConditionIdentity::Scalar {
        expression: ExprText::new(expression).ok()?.as_str().to_owned(),
    })
}

fn scalar_condition(
    graph: &CollectionGraph<'_>,
    clause: &CheckedNodeId,
    term: &CheckedTerm,
    context: &ConditionBindings<'_>,
    visiting: &mut BTreeSet<CheckedNodeId>,
    budget: &mut usize,
    current: &str,
) -> Option<String> {
    *budget = budget.checked_sub(1)?;
    match term {
        CheckedTerm::Reference { target } => {
            if !visiting.insert(target.clone()) {
                return None;
            }
            let node = graph.index.node(target)?;
            let expression = if node.semantic_form.as_ref() == "parameter" {
                let name = parameter_name(&graph.index, node)?;
                let value = context
                    .parameters
                    .iter()
                    .chain(context.result)
                    .find(|value| value.name == name && value.type_id == node.semantic_type)?;
                match value.domain {
                    CollectionElementIdentity::Boolean => value.binding.clone(),
                    CollectionElementIdentity::Integer { .. } => {
                        format!("quire_exact::Integer::from({})", value.binding)
                    }
                }
            } else {
                scalar_condition(
                    graph,
                    clause,
                    graph.index.term(node)?,
                    context,
                    visiting,
                    budget,
                    current,
                )?
            };
            visiting.remove(target);
            Some(expression)
        }
        CheckedTerm::Literal {
            value_kind: LiteralKind::Boolean,
            value,
            ..
        } => match value.as_str()? {
            "true" => Some("true".into()),
            "false" => Some("false".into()),
            _ => None,
        },
        CheckedTerm::Literal {
            value_kind: LiteralKind::Integer,
            value,
            ..
        } => {
            let value = value.as_str()?.parse::<i64>().ok()?;
            Some(format!("quire_exact::Integer::from({value}_i64)"))
        }
        CheckedTerm::Application {
            operation,
            arguments,
            ..
        } => {
            let identity = operation.identity()?;
            if identity == "quire.op.record.project" {
                let (declaration, name) = operation.projection_field().ok()??;
                let deref = graph.index.node(&term_target(arguments.first()?)?)?;
                let CheckedTerm::Application {
                    operation: deref_operation,
                    arguments: deref_arguments,
                    result_type,
                    ..
                } = graph.index.term(deref)?
                else {
                    return None;
                };
                if deref_operation.identity()? != "quire.op.model.deref"
                    || result_type != declaration
                {
                    return None;
                }
                let receiver = term_target(deref_arguments.first()?)?;
                let claim = graph.index.node(clause)?;
                let (_, parameters, _) = state_clause_parts(&graph.index, claim)?;
                if term_target(parameters.first()?)? != receiver {
                    return None;
                }
                let fields = graph.package.model_object_fields(declaration).ok()?;
                let field = fields.field(name)?;
                let index = context.fields.iter().position(|candidate| {
                    candidate.field_name == name
                        && candidate.declaring_node_id == *field.declaring_node_id()
                })?;
                return match context.fields[index].bounds {
                    SymbolicBounds::Boolean => Some(format!("boolean(&{current}.field_{index})")),
                    SymbolicBounds::Integer { .. } => {
                        Some(format!("integer(&{current}.field_{index}).clone()"))
                    }
                    SymbolicBounds::Sequence { .. } => None,
                };
            }
            let args = arguments
                .iter()
                .map(|arg| scalar_condition(graph, clause, arg, context, visiting, budget, current))
                .collect::<Option<Vec<_>>>()?;
            match (identity, args.as_slice()) {
                ("quire.op.boolean.not", [a]) => Some(format!("!({a})")),
                ("quire.op.boolean.and", [a, b]) => Some(format!("({a}) && ({b})")),
                ("quire.op.boolean.or", [a, b]) => Some(format!("({a}) || ({b})")),
                ("quire.op.boolean.implies", [a, b]) => Some(format!("!({a}) || ({b})")),
                ("quire.op.boolean.eq" | "quire.op.integer.eq" | "quire.op.numeric.eq", [a, b]) => {
                    Some(format!("({a}) == ({b})"))
                }
                ("quire.op.boolean.ne" | "quire.op.integer.ne" | "quire.op.numeric.ne", [a, b]) => {
                    Some(format!("({a}) != ({b})"))
                }
                ("quire.op.integer.ge" | "quire.op.numeric.ge", [a, b]) => {
                    Some(format!("({a}) >= ({b})"))
                }
                ("quire.op.integer.le" | "quire.op.numeric.le", [a, b]) => {
                    Some(format!("({a}) <= ({b})"))
                }
                ("quire.op.integer.gt" | "quire.op.numeric.gt", [a, b]) => {
                    Some(format!("({a}) > ({b})"))
                }
                ("quire.op.integer.lt" | "quire.op.numeric.lt", [a, b]) => {
                    Some(format!("({a}) < ({b})"))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub(super) fn validate(
    request: &crate::kani::generate::outcome::KaniObligationRequest<'_>,
    operation: &LoweredOperation,
    node_id: &CheckedNodeId,
) -> Result<(), crate::kani::generate::outcome::UnsupportedObligation> {
    use crate::kani::generate::{outcome::UnsupportedObligation, spec::CollectionElement};
    let failure = |reason| UnsupportedObligation::CollectionClaimNotRendered {
        node_id: node_id.clone(),
        reason,
    };
    let value_bytes = u64::try_from(std::mem::size_of::<quire_exact::Value>())
        .map_err(|_| failure(CollectionUnsupportedReason::DomainNotRepresentableInHarness))?;
    let mut bytes = 0u64;
    let mut capacity_failure = None;
    for field in &operation.spec.identity.fields {
        let storage = match &field.bounds {
            SymbolicBounds::Sequence {
                maximum_length,
                element,
                ..
            } => {
                let maximum = u64::try_from(*maximum_length).map_err(|_| {
                    failure(CollectionUnsupportedReason::DomainNotRepresentableInHarness)
                })?;
                let required = maximum.checked_add(1).ok_or_else(|| {
                    failure(CollectionUnsupportedReason::DomainNotRepresentableInHarness)
                })?;
                if required > u64::from(request.unwind) {
                    return Err(failure(
                        CollectionUnsupportedReason::UnwindBelowCollectionBound {
                            required,
                            supplied: request.unwind,
                        },
                    ));
                }
                let (primitive, input_bytes) = match element.as_ref() {
                    SymbolicBounds::Boolean => (CollectionElement::Boolean, 1),
                    SymbolicBounds::Integer { minimum, maximum } => (
                        CollectionElement::Integer {
                            minimum: *minimum,
                            maximum: *maximum,
                        },
                        8,
                    ),
                    _ => return Err(failure(CollectionUnsupportedReason::UnsupportedObjectState)),
                };
                if field.collection_kind == Some(CollectionKindIdentity::OrderedSet)
                    && capacity_failure.is_none()
                {
                    capacity_failure =
                        super::unique_capacity_exceeded(primitive, maximum).map(|capacity| {
                            CollectionUnsupportedReason::UniqueElementCapacityExceeded {
                                capacity,
                                declared_maximum: maximum,
                            }
                        });
                }
                // Both retained exact states, their elements, the symbolic array and length.
                value_bytes
                    .checked_mul(2)
                    .and_then(|state| state.checked_add(input_bytes))
                    .and_then(|element| maximum.checked_mul(element))
                    .and_then(|elements| elements.checked_add(2 * value_bytes + 8))
            }
            SymbolicBounds::Boolean => Some(2 * value_bytes + 1),
            SymbolicBounds::Integer { .. } => Some(2 * value_bytes + 8),
        }
        .ok_or_else(|| failure(CollectionUnsupportedReason::DomainNotRepresentableInHarness))?;
        bytes = bytes
            .checked_add(storage)
            .ok_or_else(|| failure(CollectionUnsupportedReason::DomainNotRepresentableInHarness))?;
    }
    for parameter in &operation.spec.identity.parameters {
        let storage = match parameter.domain {
            CollectionElementIdentity::Boolean => 1,
            CollectionElementIdentity::Integer { .. } => 8,
        };
        bytes = bytes
            .checked_add(storage)
            .ok_or_else(|| failure(CollectionUnsupportedReason::DomainNotRepresentableInHarness))?;
    }
    if bytes > request.ceilings.memory_bytes.get() {
        return Err(failure(
            CollectionUnsupportedReason::SymbolicDomainExceedsMemoryCeiling {
                minimum_required_bytes: bytes,
                ceiling_bytes: request.ceilings.memory_bytes.get(),
            },
        ));
    }
    if let Some(reason) = capacity_failure {
        return Err(failure(reason));
    }
    Ok(())
}

//! Whole-object collection clause lowering from the canonical admitted graph.

use super::{
    aggregate_members, application_arguments, binding, parameter_name, resolve_collection_field,
    state_clause_parts, term_target, CollectionClaimRole, CollectionDomainRefusal,
    CollectionElementDomain, CollectionFieldDomain, CollectionGraph,
};
use crate::kani::{
    generate::{
        outcome::CollectionUnsupportedReason,
        spec::{CollectionCondition, CollectionOperationSpec, ExprText},
    },
    identity::{
        CollectionElementIdentity, CollectionKindIdentity, CollectionObjectFieldIdentity,
        CollectionOperationIdentity, CollectionOperationValueIdentity, SymbolicBounds,
    },
};
use quire_contract_model::{
    CheckedCollectionKind, CheckedMemberType, CheckedNodeId, CheckedPackageV2, CheckedTerm,
    LiteralKind, StateClauseKind,
};
use std::collections::BTreeSet;

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
                number("minimum").ok_or_else(failure)?,
                number("maximum").ok_or_else(failure)?,
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
    let (kind, declared, _) = state_clause_parts(&graph.index, node).ok_or_else(failure)?;
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
        let mut value = parameter(graph, &id, parameters.len())?;
        if kind == StateClauseKind::Postcondition && value.name == "result" {
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
    let identity = CollectionOperationIdentity {
        anchor_id: anchor_id.clone(),
        fields,
        selected_field,
        parameters,
        result,
        preconditions: Vec::new(),
    };
    let mut spec = CollectionOperationSpec {
        identity,
        postcondition: kind == StateClauseKind::Postcondition,
        subject_path: String::new(),
        preconditions: Vec::new(),
    };
    if spec.postcondition {
        for sibling in &graph.package.graph().nodes {
            if sibling.node_id == *clause
                || anchor(graph, &sibling.node_id).as_ref() != Some(&anchor_id)
            {
                continue;
            }
            let Some((StateClauseKind::Precondition, _, body)) =
                state_clause_parts(&graph.index, sibling)
            else {
                continue;
            };
            let condition = sibling_condition(graph, &sibling.node_id, &body, &spec.identity)
                .ok_or_else(|| {
                    refused(CollectionUnsupportedReason::UnsupportedSiblingPrecondition)
                })?;
            spec.identity.preconditions.push(sibling.node_id.clone());
            spec.preconditions.push(condition);
        }
    }
    Ok(LoweredOperation {
        spec,
        package_instance: std::ptr::from_ref(graph.package),
    })
}

fn sibling_condition(
    graph: &CollectionGraph<'_>,
    clause: &CheckedNodeId,
    body: &CheckedNodeId,
    context: &CollectionOperationIdentity,
) -> Option<CollectionCondition> {
    let node = graph.index.node(body)?;
    if let Ok(collection) = resolve_collection_field(
        graph.package,
        &graph.index,
        clause,
        node,
        CollectionClaimRole::Precondition,
    ) {
        let field = context
            .fields
            .iter()
            .position(|field| field.field_name == collection.field_name)?;
        return Some(CollectionCondition::Quantifier {
            field,
            operation: collection.quantifier_operation,
            predicate: collection.predicate,
        });
    }
    let mut visiting = BTreeSet::new();
    let mut budget = 256usize;
    let expression = scalar_condition(
        graph,
        clause,
        &CheckedTerm::Reference {
            target: body.clone(),
        },
        context,
        &mut visiting,
        &mut budget,
    )?;
    Some(CollectionCondition::Scalar(ExprText::new(expression).ok()?))
}

fn scalar_condition(
    graph: &CollectionGraph<'_>,
    clause: &CheckedNodeId,
    term: &CheckedTerm,
    context: &CollectionOperationIdentity,
    visiting: &mut BTreeSet<CheckedNodeId>,
    budget: &mut usize,
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
                    SymbolicBounds::Boolean => Some(format!("boolean(&pre.field_{index})")),
                    SymbolicBounds::Integer { .. } => {
                        Some(format!("integer(&pre.field_{index}).clone()"))
                    }
                    SymbolicBounds::Sequence { .. } => None,
                };
            }
            let args = arguments
                .iter()
                .map(|arg| scalar_condition(graph, clause, arg, context, visiting, budget))
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

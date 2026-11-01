// SPDX-License-Identifier: AGPL-3.0-or-later
//! A constrained search method retains its boundary separately from its safety obligation.

use super::outcome::constrained::*;
use super::spec::{ConstrainedConditionSpec, ConstrainedConditionsSpec, HarnessSpec};
use crate::kani::identity::constrained::*;
use crate::{
    kani::{
        abi::{adapter_options, KaniSolver},
        identity::{ProofCeilings, SymbolicBounds},
    },
    oracle::clause::{
        generate_v2_clause_native_oracle_with_view, generate_v2_clause_oracle_with_view,
    },
};
use quire_contract_model::{
    CheckedMemberType, CheckedNodeId, CheckedPackageV2, CheckedTerm, ContractSourceV1,
    StateClauseKind,
};

fn owner(package: &CheckedPackageV2) -> ContractSourceV1 {
    ContractSourceV1 {
        name: package.name().into(),
        lock: package.lock().clone(),
    }
}

fn selected_clause(
    selection: ConstrainedClauseSelection<'_>,
    expected: StateClauseKind,
    context: &CheckedNodeId,
) -> Result<(), ConstrainedMethodRefusal> {
    use ConstrainedMethodRefusal as R;
    let node = selection
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == selection.clause)
        .ok_or(R::MissingConstraint)?;
    if node.occurrences.len() != 1 {
        return Err(R::AmbiguousConstraint);
    }
    let Some(CheckedTerm::Application {
        operation,
        arguments,
        ..
    }) = selection.package.term(selection.clause)
    else {
        return Err(R::NotConstraint);
    };
    if operation.state_clause_kind().ok().flatten() != Some(expected) {
        return Err(R::NotConstraint);
    }
    // A current model clause has no named initialization/handler/operation point.
    if operation.execution_point().is_some() {
        return Err(R::WrongContext);
    }

    let [_, CheckedTerm::Reference { target }, _] = arguments.as_slice() else {
        return Err(R::NotConstraint);
    };
    if target != context {
        return Err(R::WrongContext);
    }
    Ok(())
}

fn domain(member: &CheckedMemberType) -> Result<SymbolicBounds, ConstrainedMethodRefusal> {
    use ConstrainedMethodRefusal as R;
    match member {
        CheckedMemberType::Boolean => Ok(SymbolicBounds::Boolean),
        CheckedMemberType::IntRange { lower, upper } if lower <= upper => {
            Ok(SymbolicBounds::Integer {
                minimum: i64::try_from(*lower).map_err(|_| R::UnsupportedDomain)?,
                maximum: i64::try_from(*upper).map_err(|_| R::UnsupportedDomain)?,
            })
        }
        CheckedMemberType::Collection {
            kind:
                quire_contract_model::CheckedCollectionKind::Sequence
                | quire_contract_model::CheckedCollectionKind::OrderedSet,
            element,
            bounds: Some((lower, upper)),
        } if lower <= upper
            && !matches!(element.as_ref(), CheckedMemberType::Collection { .. }) =>
        {
            Ok(SymbolicBounds::Sequence {
                minimum_length: i64::try_from(*lower).map_err(|_| R::UnsupportedDomain)?,
                maximum_length: i64::try_from(*upper).map_err(|_| R::UnsupportedDomain)?,
                element: Box::new(domain(element)?),
            })
        }
        _ => Err(R::UnsupportedDomain),
    }
}

fn binding_domain(
    member: &CheckedMemberType,
) -> Result<ConstrainedValueDomain, ConstrainedMethodRefusal> {
    use crate::kani::identity::CollectionKindIdentity as K;
    let collection_kind = match member {
        CheckedMemberType::Collection {
            kind: quire_contract_model::CheckedCollectionKind::Sequence,
            ..
        } => Some(K::Sequence),
        CheckedMemberType::Collection {
            kind: quire_contract_model::CheckedCollectionKind::OrderedSet,
            ..
        } => Some(K::OrderedSet),
        _ => None,
    };
    Ok(ConstrainedValueDomain {
        bounds: domain(member)?,
        collection_kind,
    })
}

fn prepare_condition(
    _package: &CheckedPackageV2,
    view: &crate::core::ir::clause::ClauseGraphView<'_>,
    id: &CheckedNodeId,
    native_only: bool,
) -> Result<ConstrainedConditionSpec, ConstrainedMethodRefusal> {
    let inputs = crate::core::ir::clause::read_clause(id, view)
        .ok_or(ConstrainedMethodRefusal::Condition)?
        .inputs;
    let oracle = generate_v2_clause_native_oracle_with_view(id, view)
        .or_else(|error| {
            if native_only {
                Err(error)
            } else {
                generate_v2_clause_oracle_with_view(id, view)
            }
        })
        .map_err(|_| ConstrainedMethodRefusal::Condition)?;
    Ok(ConstrainedConditionSpec { oracle, inputs })
}

/// Generate a finite constrained method from exact admitted clauses and the complete census.
///
/// # Errors
/// Returns a typed selection, domain, binding or provision refusal before exposing a harness.
pub(super) fn lower(
    request: &ConstrainedMethodRequest<'_>,
    ceilings: ProofCeilings,
    unwind: u32,
    native_only: bool,
    view: &crate::core::ir::clause::ClauseGraphView<'_>,
) -> Result<LoweredConstrainedMethod, ConstrainedMethodRefusal> {
    use ConstrainedMethodRefusal as R;
    let package = request.safety.package;
    if let Some(constraint) = request.constraint {
        if !std::ptr::eq(package, constraint.package) {
            return Err(R::WrongPackage);
        }
        selected_clause(constraint, StateClauseKind::Constraint, request.context)?;
    }
    selected_clause(request.safety, StateClauseKind::Invariant, request.context).map_err(
        |reason| {
            if reason == R::WrongContext {
                reason
            } else {
                R::UnsupportedSafety
            }
        },
    )?;
    if unwind <= request.maximum_steps
        || unwind > crate::kani::generate::outcome::MAX_OBLIGATION_UNWIND
    {
        return Err(R::Provision);
    }
    let checked_fields = package
        .model_object_fields(request.context)
        .map_err(|_| R::WrongContext)?;
    if checked_fields.fields().len() != request.initial.len() {
        return Err(R::InitialState);
    }
    let mut fields = Vec::new();
    for field in checked_fields.fields() {
        let member = field.member_type().ok_or(R::UnsupportedDomain)?;
        let initial = request
            .initial
            .get(field.name())
            .ok_or(R::InitialState)?
            .clone();
        let entry = ConstrainedFieldIdentity {
            declaring_node: field.declaring_node_id().clone(),
            name: field.name().into(),
            domain: domain(member)?,
            unique: matches!(
                member,
                CheckedMemberType::Collection {
                    kind: quire_contract_model::CheckedCollectionKind::OrderedSet,
                    ..
                }
            ),
            initial,
        };
        super::render::constrained::initial(&entry)?;
        fields.push(entry);
    }
    let census = package
        .model_operations(request.context, request.census_work)
        .map_err(|source| R::Census { source })?;
    if i64::try_from(census.operations().len()).is_err() {
        return Err(R::Provision);
    }
    if census.operations().len() != request.operations.len() {
        return Err(R::OperationBindings);
    }
    let mut operations = Vec::new();
    for operation in census.operations() {
        let matches = request
            .operations
            .iter()
            .filter(|binding| {
                binding.name == operation.name()
                    && binding.declaring_node == operation.declaring_node_id()
            })
            .collect::<Vec<_>>();
        let [binding] = matches.as_slice() else {
            return Err(R::OperationBindings);
        };
        if syn::parse_str::<syn::Path>(binding.subject_path).is_err() {
            return Err(R::Provision);
        }
        let parameters = operation
            .parameters()
            .iter()
            .map(|parameter| {
                Ok((
                    parameter.name().to_owned(),
                    binding_domain(parameter.member_type().ok_or(R::UnsupportedDomain)?)?,
                ))
            })
            .collect::<Result<Vec<_>, R>>()?;
        let result = operation
            .result()
            .map(|member| binding_domain(member.ok_or(R::UnsupportedDomain)?))
            .transpose()?;
        operations.push(ConstrainedOperationIdentity {
            declaring_node: operation.declaring_node_id().clone(),
            name: operation.name().into(),
            subject_path: binding.subject_path.into(),
            parameters,
            result,
            preconditions: Vec::new(),
        });
    }
    let mut preconditions = vec![Vec::new(); operations.len()];
    for node in &package.graph().nodes {
        let Some(CheckedTerm::Application {
            operation,
            arguments,
            ..
        }) = package.term(&node.node_id)
        else {
            continue;
        };
        if operation.state_clause_kind().ok().flatten() != Some(StateClauseKind::Precondition) {
            continue;
        }
        let [_, CheckedTerm::Reference { target: anchor }, _] = arguments.as_slice() else {
            return Err(R::Condition);
        };
        let Some(CheckedTerm::OperationAnchorBody(body)) = package.term(anchor) else {
            return Err(R::Condition);
        };
        let Some([context, name, _]) = crate::core::ir::checked_term::bound_members(
            &body.members,
            ["context", "operation", "frame"],
        ) else {
            return Err(R::Condition);
        };
        let (Some(context), Some(name)) = (
            crate::core::ir::checked_term::reference(context),
            crate::core::ir::checked_term::literal(name, quire_contract_model::LiteralKind::Text),
        ) else {
            return Err(R::Condition);
        };
        for (index, entry) in operations.iter_mut().enumerate() {
            if entry.name != name
                || (context != request.context && context != &entry.declaring_node)
            {
                continue;
            }
            if body.execution_point.is_some() {
                return Err(R::Condition);
            }
            let oracle = prepare_condition(package, view, &node.node_id, native_only)?;
            entry.preconditions.push(node.node_id.clone());
            preconditions[index].push(oracle);
        }
    }
    let prepared = ConstrainedConditionsSpec {
        safety: prepare_condition(package, view, request.safety.clause, native_only)?,
        constraint: request
            .constraint
            .map(|selection| prepare_condition(package, view, selection.clause, native_only))
            .transpose()?,
        preconditions,
    };
    let l = request.evaluator_limits;
    let mut identity = ConstrainedMethodIdentity {
        obligation: ConstrainedObligationIdentity {
            package: owner(package),
            clause: request.safety.clause.clone(),
        },
        constraint: request.constraint.map(|selection| selection.clause.clone()),
        context: request.context.clone(),
        fields,
        operations,
        maximum_steps: request.maximum_steps,
        ceilings,
        unwind,
        evaluator_limits: [
            l.integer_bits,
            l.decimal_digits,
            l.scale_expansion,
            l.text_input_bytes,
            l.text_scalars,
            l.normalized_scalars,
            l.unit_edges,
            l.value_occurrences,
            l.work_units,
            l.result_units,
        ],
        oracles: std::iter::once(&prepared.safety)
            .chain(prepared.constraint.iter())
            .chain(prepared.preconditions.iter().flatten())
            .map(|condition| {
                (
                    condition.oracle.descriptor.node_id.clone(),
                    condition.oracle.native_integer_selection.clone(),
                )
            })
            .collect(),
        oracle_contents: std::iter::once(&prepared.safety)
            .chain(prepared.constraint.iter())
            .chain(prepared.preconditions.iter().flatten())
            .map(|condition| {
                let preimage = crate::core::canonical::preimage_value(&condition.oracle.source)
                    .map_err(|_| R::Render)?;
                let digest =
                    crate::core::canonical::content_digest(&preimage).map_err(|_| R::Render)?;
                Ok((condition.oracle.descriptor.node_id.clone(), digest))
            })
            .collect::<Result<Vec<_>, R>>()?,
        module_symbol: String::new(),
        options: Vec::new(),
        safety_assertion_line: 0,
        ordinary_cover_line: 0,
        boundary_cover_line: None,
    };
    let bytes = crate::core::canonical::preimage_value(&identity).map_err(|_| R::Render)?;
    let digest = crate::core::canonical::content_digest(&bytes).map_err(|_| R::Render)?;
    identity.module_symbol = format!(
        "constrained_{}",
        digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    );
    identity.options = adapter_options(
        &format!("{}::search", identity.module_symbol),
        identity.unwind,
        KaniSolver::Cadical,
        false,
    );
    Ok(LoweredConstrainedMethod {
        identity,
        conditions: prepared,
    })
}

pub(super) struct LoweredConstrainedMethod {
    pub(super) identity: ConstrainedMethodIdentity,
    pub(super) conditions: ConstrainedConditionsSpec,
}

pub(super) fn render(
    lowered: &LoweredConstrainedMethod,
) -> Result<KaniConstrainedMethodHarness, super::outcome::UnsupportedObligation> {
    let mut identity = lowered.identity.clone();
    let spec = HarnessSpec::new_constrained(identity.clone(), lowered.conditions.clone()).map_err(
        |_| super::outcome::UnsupportedObligation::ConstrainedMethod {
            reason: ConstrainedMethodRefusal::Render,
        },
    )?;
    let rust = super::render::render(&spec)?;
    let (safety, ordinary, boundary) =
        super::render::constrained::observation_lines(&rust, identity.constraint.is_some())
            .map_err(
                |reason| super::outcome::UnsupportedObligation::ConstrainedMethod { reason },
            )?;
    identity.safety_assertion_line = safety;
    identity.ordinary_cover_line = ordinary;
    identity.boundary_cover_line = boundary;
    let record = super::record::record(&identity.module_symbol, &identity, &rust)?;
    Ok(KaniConstrainedMethodHarness {
        identity,
        rust,
        record,
    })
}

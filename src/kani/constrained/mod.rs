// SPDX-License-Identifier: AGPL-3.0-or-later
//! A constrained search method retains its boundary separately from its safety obligation.

mod execution;
mod render;
pub use execution::*;

use crate::{
    core::artifact::Artifact,
    kani::{
        abi::{adapter_options, KaniSolver},
        identity::{ProofCeilings, SymbolicBounds},
    },
    oracle::clause::{generate_v2_clause_native_oracle, generate_v2_clause_oracle, V2ClauseOracle},
};
use qsl_replay::SnapshotValue;
use quire_contract_model::{
    CheckedMemberType, CheckedNodeId, CheckedPackageV2, CheckedTerm, ContractSourceV1,
    StateClauseKind,
};
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

/// An exact clause selection in its admitted package, without a display-name lookup.
#[derive(Clone, Copy, Debug)]
pub struct ConstrainedClauseSelection<'a> {
    /// The admitted owner of the selected clause.
    pub package: &'a CheckedPackageV2,
    /// The selected checked clause.
    pub clause: &'a CheckedNodeId,
}

/// The implementation binding of one effective operation declaration.
#[derive(Clone, Copy, Debug)]
pub struct ConstrainedOperationBinding<'a> {
    /// Actual declaring node from the canonical operation census.
    pub declaring_node: &'a CheckedNodeId,
    /// Effective operation name in the requested context.
    pub name: &'a str,
    /// Typed Rust function receiving all state fields, then the declared binders.
    pub subject_path: &'a str,
}

/// One finite, source-backed method request; a constraint is not an obligation.
#[derive(Debug)]
pub struct ConstrainedMethodRequest<'a> {
    /// Safety clause and admitted package.
    pub safety: ConstrainedClauseSelection<'a>,
    /// Constraint selected separately in the same admitted package.
    pub constraint: Option<ConstrainedClauseSelection<'a>>,
    /// Actual admitted state context of both clauses.
    pub context: &'a CheckedNodeId,
    /// Complete initial object fields in QSL's own snapshot value representation.
    pub initial: &'a BTreeMap<String, SnapshotValue>,
    /// Exactly one implementation binding for every effective operation.
    pub operations: &'a [ConstrainedOperationBinding<'a>],
    /// Inclusive transition horizon; zero still checks the initial state.
    pub maximum_steps: u32,
    /// Existing backend resource ceilings.
    pub ceilings: ProofCeilings,
    /// Loop unwind, strictly greater than the transition horizon.
    pub unwind: u32,
    /// Caller-owned evaluator provisions, separate from the state constraint.
    pub evaluator_limits: quire_exact::ScalarLimits,
    /// Work available for the complete operation census.
    pub census_work: u64,
}

/// A refused method selection or unsupported admitted domain, before rendering.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ConstrainedMethodRefusal {
    /// Clause owners disagree even if their structural node names collide.
    #[error("constraint belongs to another admitted package")]
    WrongPackage,
    /// A selected constraint node is absent.
    #[error("selected constraint is absent")]
    MissingConstraint,
    /// Multiple occurrences would require an unprovided occurrence selection.
    #[error("selected constraint is ambiguous")]
    AmbiguousConstraint,
    /// The selection is not a constraint clause.
    #[error("selected node is not a constraint")]
    NotConstraint,
    /// Safety or constraint names another state context.
    #[error("state contexts disagree")]
    WrongContext,
    /// The safety selection is not one admitted invariant occurrence.
    #[error("unsupported safety obligation")]
    UnsupportedSafety,
    /// A complete finite field or binder domain cannot be retained.
    #[error("unsupported complete state or binder domain")]
    UnsupportedDomain,
    /// Census lookup failed, without exposing a partial set.
    #[error("complete operation census refused")]
    Census,
    /// Missing, repeated or extra operation implementation binding.
    #[error("operation bindings disagree with the complete census")]
    OperationBindings,
    /// An initial field is missing, extra or outside its admitted domain.
    #[error("initial state is not the complete admitted object")]
    InitialState,
    /// The owner oracle cannot render the checked condition.
    #[error("checked owner condition is unsupported")]
    Condition,
    /// A Rust path, unwind or horizon is invalid.
    #[error("invalid method execution provision")]
    Provision,
    /// The complete generated artifact exceeds its source ceiling.
    #[error("method rendering failed")]
    Render,
}

/// The semantic obligation identity, deliberately excluding the selected method.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstrainedObligationIdentity {
    /// Admitted package name and complete dependency/source lock.
    pub package: ContractSourceV1,
    /// Exact safety clause.
    pub clause: CheckedNodeId,
}

/// A complete state slot in canonical declaration order.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstrainedFieldIdentity {
    /// Authentic declaring object.
    pub declaring_node: CheckedNodeId,
    /// Declared field name.
    pub name: String,
    /// Complete finite domain, without a constraint-derived narrowing.
    #[serde(serialize_with = "serialize_domain")]
    pub domain: SymbolicBounds,
    /// Whether a bounded ordered collection requires uniqueness.
    pub unique: bool,
    /// Initial value retained using the owning QSL writer.
    #[serde(serialize_with = "serialize_snapshot")]
    pub initial: SnapshotValue,
}

fn serialize_snapshot<S: Serializer>(
    value: &SnapshotValue,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    value
        .to_json()
        .map_err(serde::ser::Error::custom)?
        .serialize(serializer)
}

fn serialize_limits<S: Serializer>(limits: &[u64; 10], serializer: S) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeSeq;
    let mut sequence = serializer.serialize_seq(Some(limits.len()))?;
    for limit in limits {
        sequence.serialize_element(&limit.to_string())?;
    }
    sequence.end()
}

struct DomainWire<'a>(&'a SymbolicBounds);
impl Serialize for DomainWire<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let length = match self.0 {
            SymbolicBounds::Boolean => 1,
            SymbolicBounds::Integer { .. } => 3,
            SymbolicBounds::Sequence { .. } => 4,
        };
        let mut sequence = serializer.serialize_seq(Some(length))?;
        match self.0 {
            SymbolicBounds::Boolean => sequence.serialize_element("boolean")?,
            SymbolicBounds::Integer { minimum, maximum } => {
                sequence.serialize_element("integer")?;
                sequence.serialize_element(&minimum.to_string())?;
                sequence.serialize_element(&maximum.to_string())?;
            }
            SymbolicBounds::Sequence {
                minimum_length,
                maximum_length,
                element,
            } => {
                sequence.serialize_element("sequence")?;
                sequence.serialize_element(&minimum_length.to_string())?;
                sequence.serialize_element(&maximum_length.to_string())?;
                sequence.serialize_element(&DomainWire(element))?;
            }
        }
        sequence.end()
    }
}
fn serialize_domain<S: Serializer>(
    domain: &SymbolicBounds,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    DomainWire(domain).serialize(serializer)
}

/// One complete generated binder domain with its admitted collection kind.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstrainedValueDomain {
    /// Finite bounds, retaining exact decimal endpoints in the identity wire form.
    #[serde(serialize_with = "serialize_domain")]
    pub bounds: SymbolicBounds,
    /// Distinguishes sequences from ordered sets instead of discarding uniqueness.
    pub collection_kind: Option<crate::kani::identity::CollectionKindIdentity>,
}

/// An effective operation ABI, independent of callable-body and dispatch reads.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstrainedOperationIdentity {
    /// Actual declaring node, never reconstructed from an implementation body.
    pub declaring_node: CheckedNodeId,
    /// Effective operation name.
    pub name: String,
    /// Selected customer implementation.
    pub subject_path: String,
    /// Parameter names and complete domains in producer order.
    pub parameters: Vec<(String, ConstrainedValueDomain)>,
    /// Complete result domain, absent only for a declared void result.
    pub result: Option<ConstrainedValueDomain>,
    /// Every admitted precondition attached to this effective operation.
    pub preconditions: Vec<CheckedNodeId>,
}

/// Exact method execution identity; the safety obligation stays separate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConstrainedMethodIdentity {
    /// Semantic obligation, unchanged when another constraint is selected.
    pub obligation: ConstrainedObligationIdentity,
    /// Separately selected constraint.
    pub constraint: Option<CheckedNodeId>,
    /// Common admitted state context.
    pub context: CheckedNodeId,
    /// Whole finite object, in canonical field declaration order.
    pub fields: Vec<ConstrainedFieldIdentity>,
    /// Complete operation census and implementation binding.
    pub operations: Vec<ConstrainedOperationIdentity>,
    /// Finite transition horizon.
    pub maximum_steps: u32,
    /// Backend ceilings.
    pub ceilings: ProofCeilings,
    /// Exact unwind.
    pub unwind: u32,
    /// Owner evaluator limits in their documented declaration order.
    #[serde(serialize_with = "serialize_limits")]
    pub evaluator_limits: [u64; 10],
    /// Owning oracle decisions, including complete native capacity or general integer policy.
    pub oracles: Vec<(
        CheckedNodeId,
        Option<crate::core::native_integer::NativeIntegerSelection>,
    )>,
    /// Generated module symbol.
    pub module_symbol: String,
    /// Exact backend options.
    pub options: Vec<String>,
    /// Ordinary non-vacuity cover line, identified separately from the boundary cover.
    pub ordinary_cover_line: u32,
    /// Boundary reachability cover line.
    pub boundary_cover_line: Option<u32>,
}

/// A sealed generated method. Ordinary proof execution cannot consume it.
#[derive(Clone, Debug)]
pub struct KaniConstrainedMethodHarness {
    pub(crate) identity: ConstrainedMethodIdentity,
    pub(crate) rust: Artifact,
    pub(crate) record: Artifact,
}
impl KaniConstrainedMethodHarness {
    /// The selected method and complete domains.
    pub fn identity(&self) -> &ConstrainedMethodIdentity {
        &self.identity
    }
    /// Generated method source, retained separately from the identity record.
    pub fn rust(&self) -> &Artifact {
        &self.rust
    }
    /// Serialized identity and source path.
    pub fn record(&self) -> &Artifact {
        &self.record
    }
}

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

pub(super) struct Prepared {
    safety: PreparedCondition,
    constraint: Option<PreparedCondition>,
    preconditions: Vec<Vec<PreparedCondition>>,
}

#[derive(Clone)]
pub(super) struct PreparedCondition {
    oracle: V2ClauseOracle,
    inputs: Vec<crate::core::ir::clause::ClauseInput>,
}
fn prepare_condition(
    package: &CheckedPackageV2,
    view: &crate::core::ir::clause::ClauseGraphView<'_>,
    id: &CheckedNodeId,
) -> Result<PreparedCondition, ConstrainedMethodRefusal> {
    let inputs = crate::core::ir::clause::read_clause(id, view)
        .ok_or(ConstrainedMethodRefusal::Condition)?
        .inputs;
    let oracle = generate_v2_clause_native_oracle(package, id)
        .or_else(|_| generate_v2_clause_oracle(package, id))
        .map_err(|_| ConstrainedMethodRefusal::Condition)?;
    Ok(PreparedCondition { oracle, inputs })
}

/// Generate a finite constrained method from exact admitted clauses and the complete census.
///
/// # Errors
/// Returns a typed selection, domain, binding or provision refusal before exposing a harness.
pub fn generate_constrained_method(
    request: &ConstrainedMethodRequest<'_>,
) -> Result<KaniConstrainedMethodHarness, ConstrainedMethodRefusal> {
    use ConstrainedMethodRefusal as R;
    let package = request.safety.package;
    if let Some(constraint) = request.constraint {
        if owner(package) != owner(constraint.package) {
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
    if request.unwind <= request.maximum_steps
        || request.unwind > crate::kani::generate::outcome::MAX_OBLIGATION_UNWIND
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
        render::initial(&entry)?;
        fields.push(entry);
    }
    let census = package
        .model_operations(request.context, request.census_work)
        .map_err(|_| R::Census)?;
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
    let view = crate::core::ir::clause::ClauseGraphView::new(package);
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
            let oracle = prepare_condition(package, &view, &node.node_id)?;
            entry.preconditions.push(node.node_id.clone());
            preconditions[index].push(oracle);
        }
    }
    let prepared = Prepared {
        safety: prepare_condition(package, &view, request.safety.clause)?,
        constraint: request
            .constraint
            .map(|selection| prepare_condition(package, &view, selection.clause))
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
        ceilings: request.ceilings,
        unwind: request.unwind,
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
        module_symbol: String::new(),
        options: Vec::new(),
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
    render::render(identity, &prepared)
}

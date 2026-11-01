// SPDX-License-Identifier: AGPL-3.0-or-later
//! The constrained method request and refusal vocabulary imports no family implementation.
use qsl_replay::SnapshotValue;
use quire_contract_model::{CheckedNodeId, CheckedPackageV2};
use serde::Serialize;
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
#[derive(Clone, Copy, Debug)]
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
    /// Caller-owned evaluator provisions, separate from the state constraint.
    pub evaluator_limits: quire_exact::ScalarLimits,
    /// Work available for the complete operation census.
    pub census_work: u64,
}

/// A refused method selection or unsupported admitted domain, before rendering.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, thiserror::Error)]
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
    #[error("complete operation census refused: {source}")]
    Census {
        /// Original canonical-reader refusal, including its exact context and work provision.
        #[serde(serialize_with = "serialize_census")]
        source: quire_contract_model::CheckedOperationReadError,
    },
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

fn serialize_census<S: serde::Serializer>(
    error: &quire_contract_model::CheckedOperationReadError,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeStruct;
    let mut record = serializer.serialize_struct("CheckedOperationReadError", 4)?;
    record.serialize_field("code", &format!("{:?}", error.code))?;
    record.serialize_field("context", &error.context)?;
    record.serialize_field("work", &error.work.to_string())?;
    record.serialize_field("limit", &error.limit.to_string())?;
    record.end()
}

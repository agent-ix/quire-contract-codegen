// SPDX-License-Identifier: AGPL-3.0-or-later
//! Borrow the canonical transition carrier and recognize its Boolean proposition.

use quire_contract_model::{
    ApplicationOperator, CheckedMemberType, CheckedNodeId, CheckedNodeKind,
    ContractSourceV1, StateForm,
};
use serde::Serialize;

use crate::{
    core::{
        canonical::{content_digest, preimage_value},
        ir::{
            checked_term::{aggregate_members, application, bound_members, reference},
            clause::{read_clause, ClauseGraphView, ClauseInputSide, ClauseTerm},
            frame::projected_field,
        },
    },
    kani::{
        abi::{adapter_options, KaniSolver},
        generate::record::harness_path,
        identity::{state_transition::StateTransitionIdentity, ProofCeilings},
    },
};

/// Why the selected admitted transition has no Boolean field-transition proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StateTransitionRefusal {
    /// The selection is not a canonical state-transition application.
    Transition,
    /// The selected transition does not have exactly one Boolean parameter.
    Input,
    /// The selected subject is not the required Boolean field projection.
    Subject,
    /// The precondition is not the selected input OR the selected pre-state field.
    Precondition,
    /// The postcondition is not the selected post-state field.
    Postcondition,
    /// The selected claim has no canonical occurrence or source correspondence.
    SourceCorrespondence,
    /// Complete admitted identity material cannot be encoded within the identity ceiling.
    Identity,
}

pub(super) fn lower(
    view: &ClauseGraphView<'_>,
    transition: &CheckedNodeId,
    subject_path: &str,
    ceilings: ProofCeilings,
    unwind: u32,
) -> Result<StateTransitionIdentity, StateTransitionRefusal> {
    use StateTransitionRefusal as R;
    let package = view.package();
    let node = view.node(transition).ok_or(R::Transition)?;
    if view.kind(transition) != Some(CheckedNodeKind::State(StateForm::Transition)) {
        return Err(R::Transition);
    }
    let members = application(
        package.term(transition).ok_or(R::Transition)?,
        ApplicationOperator::StateTransition,
        "quire.op.state.transition",
    )
    .ok_or(R::Transition)?;
    let [anchor, subject, inputs, pre, post] =
        bound_members(members, ["anchor", "subject", "inputs", "pre", "post"])
            .ok_or(R::Transition)?;
    let anchor = reference(anchor).ok_or(R::Transition)?;
    let subject = reference(subject).ok_or(R::Subject)?;
    let [input] = aggregate_members(inputs).ok_or(R::Input)? else {
        return Err(R::Input);
    };
    let input = reference(input).ok_or(R::Input)?;
    let parameter = package.parameter_declaration(input).map_err(|_| R::Input)?;
    let (declaring_node, field) = projected_field(package.term(subject).ok_or(R::Subject)?)
        .ok_or(R::Subject)?;
    let fields = package
        .model_object_fields(&declaring_node)
        .map_err(|_| R::Subject)?;
    let selected = fields.field(field).ok_or(R::Subject)?;
    if selected.member_type() != Some(&CheckedMemberType::Boolean) {
        return Err(R::Subject);
    }
    let [pre] = aggregate_members(pre).ok_or(R::Precondition)? else {
        return Err(R::Precondition);
    };
    let pre = reference(pre).ok_or(R::Precondition)?;
    let post = reference(post).ok_or(R::Postcondition)?;
    let precondition = read_clause(pre, view).ok_or(R::Precondition)?;
    let postcondition = read_clause(post, view).ok_or(R::Postcondition)?;
    let field_read = |term: &ClauseTerm| matches!(term,
        ClauseTerm::Input(value) if value.boolean && value.name == field
            && value.side == ClauseInputSide::Current);
    let parameter_read = |term: &ClauseTerm| matches!(term,
        ClauseTerm::Input(value) if value.boolean && value.name == parameter.name.as_ref()
            && value.side == ClauseInputSide::Parameter);
    let ClauseTerm::Operation { identity, arguments, .. } = &precondition.term else {
        return Err(R::Precondition);
    };
    let [left, right] = arguments.as_slice() else {
        return Err(R::Precondition);
    };
    if identity != "quire.op.boolean.or"
        || !((parameter_read(left) && field_read(right))
            || (field_read(left) && parameter_read(right)))
    {
        return Err(R::Precondition);
    }
    if !field_read(&postcondition.term) {
        return Err(R::Postcondition);
    }
    let source_map = package
        .source_map()
        .iter()
        .filter(|entry| &entry.node_id == transition)
        .cloned()
        .collect::<Vec<_>>();
    if node.occurrences.is_empty() || source_map.is_empty() {
        return Err(R::SourceCorrespondence);
    }
    let package_identity = ContractSourceV1 {
        name: package.name().into(),
        lock: package.lock().clone(),
    };
    let digest = |value| {
        content_digest(&value)
            .map(|bytes| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>())
            .map_err(|_| R::Identity)
    };
    let material = preimage_value(&(
        &package_identity,
        package.graph(),
        package.source_map(),
    ))
    .map_err(|_| R::Identity)?;
    let semantic_digest = digest(material)?;
    let material = preimage_value(&(
        "boolean-field-transition-or/v1",
        &semantic_digest,
        transition,
        subject_path,
        ceilings,
        unwind,
    ))
    .map_err(|_| R::Identity)?;
    let path = harness_path(&format!("kob_state_transition_{}", digest(material)?), "prove")
        .map_err(|_| R::Identity)?;
    let options = adapter_options(&path.to_string(), unwind, KaniSolver::Cadical, false);
    Ok(StateTransitionIdentity {
        ceilings,
        package: package_identity,
        transition: transition.clone(),
        occurrences: node.occurrences.clone(),
        source_map,
        semantic_digest,
        anchor: anchor.clone(),
        subject: subject.clone(),
        input: input.clone(),
        precondition: pre.clone(),
        postcondition: post.clone(),
        declaring_node,
        field: field.to_owned(),
        parameter: parameter.name.to_string(),
        subject_path: subject_path.to_owned(),
        module_symbol: path.module,
        harness_symbol: path.harness,
        solver: KaniSolver::Cadical,
        unwind,
        options,
    })
}

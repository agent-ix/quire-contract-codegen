//! QSL-1336 negative controls for selected frame claims. These are kept outside the
//! `kani_obligations` filter because they document an unresolved producer defect, not a Kani proof.

use ix_trace_rs::trace;
use quire_contract_codegen::{FrameReplayError, StateFrameProperty};

use super::kani_obligations_state_frame::{
    frame_harness,
    native_twin::{Twin, CLAUSES},
    replay_of, run_of,
};

/// A held frame node is insufficient if its harness changes which state field it grants.
/// Both field lists still cover the same state fields, so the selected-claim body must be checked.
#[trace("TC-035")]
#[test]
#[ignore = "QSL-1336: OperationSite lacks selected frame-grant body (FR-024-AC-49/51)"]
fn tc_035_a_held_frame_node_rejects_changed_grants() {
    let twin = Twin::new();
    let baseline = frame_harness(&twin);
    let mut changed = baseline.clone();
    changed.property = StateFrameProperty::Frame {
        granted: vec!["audit".to_owned()],
        checked: vec!["balance".to_owned()],
    };
    assert_ne!(changed.property, baseline.property);
    assert_eq!(changed.scope.frame, baseline.scope.frame);
    let error = match replay_of(&twin, "deposit", &run_of(&changed, [5, 0])) {
        Ok(_) => panic!("a changed grant under the held frame node must refuse"),
        Err(error) => error,
    };
    assert!(
        matches!(error, FrameReplayError::FieldSetMismatch { .. }),
        "a changed grant under the held frame node must refuse: {error}"
    );
}

/// A grant change must rekey the producer frame claim in both replay carriers.
#[trace("TC-035")]
#[test]
#[ignore = "QSL-1336: producer frame node does not rekey for a changed grant"]
fn tc_035_a_changed_grant_changes_the_identity_in_the_request_and_the_envelope() {
    let twin = Twin::new();
    let base = replay_of(&twin, "deposit", &run_of(&frame_harness(&twin), [5, 0]))
        .expect("baseline frame replay builds");

    let granting_nothing = Twin::build(&[], &CLAUSES, 0);
    let changed = replay_of(
        &granting_nothing,
        "deposit",
        &run_of(&frame_harness(&granting_nothing), [5, 0]),
    )
    .expect("changed frame replay builds");

    assert_ne!(
        changed.wire.obligation_identity,
        base.wire.obligation_identity
    );
    assert_ne!(
        changed.packet.obligation_identity,
        base.packet.obligation_identity
    );
    let site = granting_nothing
        .operation_site("deposit")
        .expect("the changed operation is located");
    assert_eq!(
        changed.wire.obligation_identity,
        *site.obligation_identity()
    );
    assert_eq!(
        changed.packet.obligation_identity,
        Some(changed.wire.obligation_identity.clone())
    );
}

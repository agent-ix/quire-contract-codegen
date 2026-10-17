//! Crate-internal parity checks for state-frame role negotiation.

// The shared model declares fields beyond those this seam test reads directly.
#[allow(dead_code)]
#[path = "../../../tests/state_frame_support/model.rs"]
mod model;
// The QSL twin supplies an admitted CheckedPackageV2; this test uses only its package builder.
#[allow(dead_code)]
#[path = "../../../tests/state_frame_support/native_twin.rs"]
mod native_twin;

use crate::kani::generate::frame::{
    generate_state_frame_obligations, generate_state_frame_role, StateFrameRequest,
};
use crate::kani::generate::negotiate::negotiate_kani_obligations;
use crate::kani::generate::outcome::{
    KaniObligationOutcome, KaniObligationRequest, ObligationDisposition, ObligationItem,
    StateFrameLoweringRefusal, StateFrameRefusal, StateFrameRole, UnsupportedObligation,
};
use crate::kani::identity::StateFrameHarness;
use ix_trace_rs::trace;

const FIELDS: [&str; 2] = [model::FIELDS[0].0, model::FIELDS[1].0];
const STATE_PATH: &str = "crate::subject::Account";
const SUBJECT_PATH: &str = "crate::subject::deposit";

fn compare_roles(twin: &native_twin::Twin, refused_role: Option<StateFrameRole>) {
    let (package, clause) = twin.emitted_package("BalanceNeverDrops");
    let ceilings = crate::kani::test_support::proof_ceilings::proof_ceilings();
    let direct = StateFrameRequest {
        ceilings,
        package: &package,
        clause: &clause,
        state_path: STATE_PATH,
        state_fields: &FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    };
    let roles = [StateFrameRole::Frame, StateFrameRole::Contract];
    let items = roles.map(|role| ObligationItem::StateFrame {
        package: &package,
        clause: &clause,
        role,
        state_path: STATE_PATH,
        state_fields: &FIELDS,
        subject_path: SUBJECT_PATH,
    });
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings,
        items: &items,
        subject_path: "crate::request::unrelated",
        unwind: 4,
    })
    .expect("well-formed request");
    let KaniObligationOutcome::Emitted {
        records,
        state_frame_harnesses,
        ..
    } = outcome
    else {
        panic!("one refused role must not reject the whole request");
    };
    assert_eq!(records.len(), roles.len());
    let mut emitted = state_frame_harnesses.iter();
    let mut direct_refusal = None;
    for (role, record) in roles.into_iter().zip(&records) {
        assert_eq!(
            record.request_index,
            if role == StateFrameRole::Frame { 0 } else { 1 }
        );
        assert_eq!(record.kind, Some(role.obligation_kind()));
        match generate_state_frame_role(&direct, role) {
            Ok(expected) => {
                let actual = emitted.next().expect("supported role has a harness");
                assert_eq!(
                    actual, &expected,
                    "exact identity, record and harness bytes"
                );
                assert_eq!(
                    actual.rust.contents.as_bytes(),
                    expected.rust.contents.as_bytes()
                );
                assert_eq!(
                    record.disposition,
                    ObligationDisposition::Supported {
                        harness_symbol: expected.identity.harness_symbol.clone(),
                    }
                );
            }
            Err(refusal) => {
                assert_eq!(Some(role), refused_role, "only the expected role refuses");
                assert_eq!(
                    record.disposition,
                    ObligationDisposition::Unsupported {
                        reason: UnsupportedObligation::StateFrameRefused {
                            refusal: refusal.clone(),
                        },
                    }
                );
                direct_refusal = Some(refusal);
            }
        }
    }
    assert!(emitted.next().is_none(), "no unaccounted harness");
    assert_eq!(
        state_frame_harnesses.len(),
        2 - usize::from(refused_role.is_some())
    );
    assert_eq!(direct_refusal.is_some(), refused_role.is_some());
    if let Some(refusal) = direct_refusal {
        assert_eq!(generate_state_frame_obligations(&direct), Err(refusal));
    } else {
        let pair = generate_state_frame_obligations(&direct).expect("both roles generate");
        let expected: [&StateFrameHarness; 2] = [&pair.frame, &pair.postcondition];
        assert_eq!(state_frame_harnesses.iter().collect::<Vec<_>>(), expected);
    }
}

#[trace("FR-015-AC-60", "TC-025")]
#[test]
fn state_frame_both_roles_match_direct_generation() {
    compare_roles(&native_twin::Twin::new(), None);
}

#[trace("FR-015-AC-68", "TC-025")]
#[test]
fn state_frame_one_role_refused_matches_direct_generation() {
    compare_roles(
        &native_twin::Twin::build(&["balance", "audit"], &[native_twin::CLAUSES[0]], 0),
        Some(StateFrameRole::Frame),
    );
    compare_roles(
        &native_twin::Twin::with_negated_balance_clause(),
        Some(StateFrameRole::Contract),
    );

    let (package, clause) =
        native_twin::Twin::with_unbounded_audit_clause().emitted_package("AuditNeverDrops");
    let ceilings = crate::kani::test_support::proof_ceilings::proof_ceilings();
    let direct = StateFrameRequest {
        ceilings,
        package: &package,
        clause: &clause,
        state_path: STATE_PATH,
        state_fields: &FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    };
    let roles = [StateFrameRole::Frame, StateFrameRole::Contract];
    let items = roles.map(|role| ObligationItem::StateFrame {
        package: &package,
        clause: &clause,
        role,
        state_path: STATE_PATH,
        state_fields: &FIELDS,
        subject_path: SUBJECT_PATH,
    });
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings,
        items: &items,
        subject_path: "crate::request::unrelated",
        unwind: 4,
    })
    .expect("well-formed request");
    let KaniObligationOutcome::Emitted {
        records,
        state_frame_harnesses,
        ..
    } = outcome
    else {
        panic!("refused lowering must retain both item records");
    };
    assert_eq!(records.len(), roles.len());
    assert!(state_frame_harnesses.is_empty());
    let refusals = roles.map(|role| generate_state_frame_role(&direct, role).unwrap_err());
    assert_eq!(refusals[0], refusals[1]);
    let StateFrameRefusal::NotLowered { refusal } = &refusals[0] else {
        panic!("direct roles must refuse at lowering");
    };
    let StateFrameLoweringRefusal::RequiresBound { unbounded_type, .. } = &**refusal else {
        panic!("unbounded audit must require a bound");
    };
    for (index, record) in records.iter().enumerate() {
        assert_eq!(record.request_index, index);
        assert_eq!(record.kind, Some(roles[index].obligation_kind()));
        assert_eq!(
            record.disposition,
            ObligationDisposition::RequiresBound {
                unbounded_type: unbounded_type.clone(),
            }
        );
    }
    assert_eq!(records[0].disposition, records[1].disposition);
    assert_eq!(
        generate_state_frame_obligations(&direct),
        Err(refusals[0].clone())
    );
}

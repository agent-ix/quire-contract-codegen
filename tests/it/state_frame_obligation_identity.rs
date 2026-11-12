//! FR-015-AC-137 to FR-015-AC-142 and FR-024-AC-71 to FR-024-AC-75 (IR-1520): the state-frame
//! lane carries one QSL `ObligationIdentity` per role in the harness identity and the persisted
//! record, and replay cross-checks it.
//!
//! Every identity here is a hand-written literal from `common::state_frame_identities`, never
//! derived from the code under test; the replay controls use the identities QSL's selected
//! sites mint, taken from the twin.

use ix_trace_rs::trace;
use qsl_replay::{ModelIdentity, ObligationIdentity};
use quire_contract_codegen::{
    generate_state_frame_obligations, negotiate_kani_obligations, parse_state_clause_falsification,
    FrameReplayError, KaniObligationOutcome, KaniObligationRequest, KaniRunOutcome, ObligationItem,
    ReplaySettlement, StateClauseHarnessMember, StateClauseReplayError, StateFrameHarness,
    StateFrameIdentity, StateFrameObligations, StateFrameRecordError, StateFrameRefusal,
    StateFrameRequest, StateFrameRole, StateFrameRoleIdentities, StateFrameRoleIdentitiesError,
};
use serde_json::{json, Value};

use super::kani_obligations_state_frame::{
    emitted_fixture, frame_harness,
    native_twin::{playback_text, Twin},
    replay_of, run_of, Fixture,
};
use crate::common::state_frame_identities as ids;

const BALANCE: &str = "BalanceNeverDrops";
const STATE_FIELDS: [&str; 2] = ["balance", "audit"];
const STATE_PATH: &str = "crate::subject::Account";
const SUBJECT_PATH: &str = "crate::subject::deposit_debiting_within_range";

fn request(fixture: &Fixture) -> StateFrameRequest<'_> {
    StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        package: &fixture.package,
        clause: &fixture.clause,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    }
}

fn generate(
    fixture: &Fixture,
    contract: ObligationIdentity,
    frame: ObligationIdentity,
) -> StateFrameObligations {
    generate_state_frame_obligations(
        &request(fixture),
        StateFrameRoleIdentities::new(contract, frame).expect("distinct identities"),
    )
    .expect("both roles generate")
}

fn record_json(harness: &StateFrameHarness) -> Value {
    serde_json::from_str(&harness.record.contents).expect("the record is JSON")
}

fn with_record(harness: &StateFrameHarness, edit: impl FnOnce(&mut Value)) -> String {
    let mut record = record_json(harness);
    edit(&mut record);
    record.to_string()
}

/// FR-015-AC-137: each role's harness carries its role's identity, member for member.
#[trace("TC-025", "FR-015-AC-137", "FR-032-AC-14", "TC-047")]
#[test]
fn tc_025_each_role_harness_carries_its_own_identity() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, BALANCE);
    let pair = generate(&fixture, ids::contract(), ids::frame());
    let (contract, frame) = (&pair.postcondition.identity, &pair.frame.identity);
    assert_eq!(contract.obligation, ids::contract());
    assert_eq!(frame.obligation, ids::frame());
    // Member for member, not only by equality of the whole.
    assert_eq!(contract.obligation.ordinal(), 3);
    assert_eq!(contract.obligation.model(), None);
    assert_eq!(contract.obligation.node().package(), "ids");
    assert_eq!(contract.obligation.node().ordinal(), 7);
    assert_eq!(frame.obligation.ordinal(), 5);
    assert_eq!(
        frame.obligation.model(),
        Some(&ModelIdentity::new("model-b"))
    );
    assert_eq!(frame.obligation.node().ordinal(), 9);
    // The swapped request swaps the harnesses' identities, so the roles are not interchangeable.
    let swapped = generate(&fixture, ids::frame(), ids::contract());
    assert_eq!(swapped.postcondition.identity.obligation, ids::frame());
    assert_eq!(swapped.frame.identity.obligation, ids::contract());
}

/// FR-015-AC-138: the record writes the documented object and rebuilds it equal; a `Node`
/// segment nests a whole key and is never flattened to text.
#[trace("TC-025", "FR-015-AC-138")]
#[test]
fn tc_025_the_record_wire_form_round_trips_including_nested_node_segments() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, BALANCE);
    let pair = generate(&fixture, ids::contract(), ids::frame());
    assert_eq!(
        record_json(&pair.postcondition)["identity"]["obligation"],
        json!({
            "node": {"package": "ids", "path": [{"text": "contract"}], "ordinal": 7},
            "ordinal": 3,
        }),
        "no model member when the identity has none"
    );
    assert_eq!(
        record_json(&pair.frame)["identity"]["obligation"],
        json!({
            "node": {"package": "ids", "path": [{"text": "frame"}], "ordinal": 9},
            "model": {"identity": "model-b"},
            "ordinal": 5,
        })
    );
    for harness in [&pair.postcondition, &pair.frame] {
        let read = StateFrameIdentity::from_record(&harness.record.contents)
            .expect("the record is a state-frame identity");
        assert_eq!(read.obligation, harness.identity.obligation);
        assert_eq!(read, harness.identity);
    }

    let nested = generate(&fixture, ids::nested(), ids::frame());
    let written = &record_json(&nested.postcondition)["identity"]["obligation"];
    assert_eq!(
        written,
        &json!({
            "node": {
                "package": "ids",
                "path": [
                    {"node": {
                        "package": "ids",
                        "path": [
                            {"text": "middle"},
                            {"node": {
                                "package": "ids",
                                "path": [{"text": "leaf"}, {"text": ""}],
                                "ordinal": 1,
                            }},
                        ],
                        "ordinal": 2,
                    }},
                    {"text": "tail"},
                ],
                "ordinal": 3,
            },
            "model": {"identity": "model-n"},
            "ordinal": 11,
        }),
        "a Node segment is written as {{\"node\": ...}} and nests recursively"
    );
    let read = StateFrameIdentity::from_record(&nested.postcondition.record.contents)
        .expect("the nested record reads");
    assert_eq!(read.obligation, ids::nested());
}

/// FR-015-AC-138: a key nested deeper than the wire form reads back is refused when written,
/// not written and then unreadable.
#[trace("TC-025", "FR-015-AC-138")]
#[test]
fn tc_025_a_key_nested_past_the_readable_depth_is_refused_when_written() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, BALANCE);
    let nest = |levels: usize| {
        let mut key = quire_exact::NodeKey::new(
            "deep",
            vec![quire_exact::PathSegment::Text("leaf".to_owned())],
            0,
        )
        .expect("leaf key");
        for _ in 0..levels {
            key = quire_exact::NodeKey::new("deep", vec![quire_exact::PathSegment::Node(key)], 0)
                .expect("wrapper key");
        }
        ObligationIdentity::new(key, None, 0)
    };
    // Control: a key of exactly 32 node levels (the bound) is written and read back equal.
    for levels in [8, 31] {
        let at_bound = generate(&fixture, nest(levels), ids::frame());
        let read = StateFrameIdentity::from_record(&at_bound.postcondition.record.contents)
            .expect("a supported depth reads");
        assert_eq!(read.obligation, nest(levels), "{} node levels", levels + 1);
    }
    // 33 node levels is one past the bound.
    assert_eq!(
        generate_state_frame_obligations(
            &request(&fixture),
            StateFrameRoleIdentities::new(nest(32), ids::frame()).expect("distinct"),
        ),
        Err(StateFrameRefusal::RecordSerialization)
    );
    let refused = generate_state_frame_obligations(
        &request(&fixture),
        StateFrameRoleIdentities::new(nest(200), ids::frame()).expect("distinct"),
    );
    assert_eq!(refused, Err(StateFrameRefusal::RecordSerialization));
}

/// FR-015-AC-138: the reader refuses a key nested past the bound as well as the writer.
#[trace("TC-025", "FR-015-AC-138")]
#[test]
fn tc_025_a_record_nested_past_the_readable_depth_is_not_read() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, BALANCE);
    let pair = generate(&fixture, ids::contract(), ids::frame());
    let wire_of = |levels: usize| {
        let mut node = json!({"package": "deep", "path": [{"text": "leaf"}], "ordinal": 0});
        for _ in 1..levels {
            node = json!({"package": "deep", "path": [{"node": node}], "ordinal": 0});
        }
        node
    };
    let with_levels = |levels: usize| {
        with_record(&pair.postcondition, |record| {
            record["identity"]["obligation"]["node"] = wire_of(levels);
        })
    };
    StateFrameIdentity::from_record(&with_levels(32)).expect("32 node levels read");
    let error =
        StateFrameIdentity::from_record(&with_levels(33)).expect_err("33 node levels are refused");
    assert!(matches!(error, StateFrameRecordError::Json(_)), "{error}");
}

/// FR-015-AC-139: a record without the member, its `node` or its `ordinal` is not read.
#[trace("TC-025", "FR-015-AC-139")]
#[test]
fn tc_025_a_record_lacking_the_obligation_or_its_members_is_refused() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, BALANCE);
    let pair = generate(&fixture, ids::contract(), ids::frame());
    for harness in [&pair.postcondition, &pair.frame] {
        // Control: the unedited record reads.
        StateFrameIdentity::from_record(&harness.record.contents).expect("control reads");
        let cases: [(&str, RecordEdit); 5] = [
            (
                "obligation",
                Box::new(|r| {
                    r["identity"]
                        .as_object_mut()
                        .expect("object")
                        .remove("obligation");
                }),
            ),
            (
                "obligation.node",
                Box::new(|r| {
                    r["identity"]["obligation"]
                        .as_object_mut()
                        .expect("object")
                        .remove("node");
                }),
            ),
            (
                "obligation.ordinal",
                Box::new(|r| {
                    r["identity"]["obligation"]
                        .as_object_mut()
                        .expect("object")
                        .remove("ordinal");
                }),
            ),
            (
                "obligation.node.ordinal",
                Box::new(|r| {
                    r["identity"]["obligation"]["node"]
                        .as_object_mut()
                        .expect("object")
                        .remove("ordinal");
                }),
            ),
            (
                "obligation.node.package",
                Box::new(|r| {
                    r["identity"]["obligation"]["node"]
                        .as_object_mut()
                        .expect("object")
                        .remove("package");
                }),
            ),
        ];
        for (removed, edit) in cases {
            let edited = with_record(harness, edit);
            let error = StateFrameIdentity::from_record(&edited)
                .expect_err(&format!("a record without {removed} must be refused"));
            assert!(
                matches!(error, StateFrameRecordError::Json(_)),
                "{removed}: {error}"
            );
        }
    }
}

/// FR-015-AC-140: the identity is no input to the harness source, symbols or ceilings.
#[trace("TC-025", "FR-015-AC-140")]
#[test]
fn tc_025_the_identity_is_not_an_input_to_the_harness_source_or_symbols() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, BALANCE);
    let base = generate(&fixture, ids::contract(), ids::frame());
    let changed = generate(&fixture, ids::contract_ordinal_only(), ids::frame());
    assert_ne!(
        base.postcondition.identity.obligation,
        changed.postcondition.identity.obligation
    );
    for (a, b) in [
        (&base.postcondition, &changed.postcondition),
        (&base.frame, &changed.frame),
    ] {
        assert_eq!(a.rust.contents.as_bytes(), b.rust.contents.as_bytes());
        assert_eq!(a.rust.path, b.rust.path);
        assert_eq!(a.identity.harness_symbol, b.identity.harness_symbol);
        assert_eq!(a.identity.module_symbol, b.identity.module_symbol);
    }
    // The contract records differ, and only in `obligation`.
    assert_ne!(base.postcondition.record, changed.postcondition.record);
    let without = |harness: &StateFrameHarness| {
        with_record(harness, |record| {
            record["identity"]
                .as_object_mut()
                .expect("object")
                .remove("obligation");
        })
    };
    assert_eq!(
        without(&base.postcondition),
        without(&changed.postcondition)
    );
    // The frame role's record is untouched by the contract identity change.
    assert_eq!(base.frame.record, changed.frame.record);
}

fn negotiate(
    fixture: &Fixture,
    roles: [(StateFrameRole, &ObligationIdentity); 2],
) -> (
    Vec<quire_contract_codegen::ObligationRecord>,
    Vec<StateFrameHarness>,
) {
    let items = roles.map(|(role, obligation)| ObligationItem::StateFrame {
        package: &fixture.package,
        clause: &fixture.clause,
        role,
        obligation,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
    });
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items: &items,
        subject_path: "crate::request::unrelated",
        unwind: 4,
    })
    .expect("a well-formed request");
    let KaniObligationOutcome::Emitted {
        records,
        state_frame_harnesses,
        ..
    } = outcome
    else {
        panic!("the request emits");
    };
    (records, state_frame_harnesses)
}

/// FR-015-AC-141: the negotiated harness of each item carries its own identity; negotiation
/// compares nothing across items; classification does not read the identity.
#[trace("TC-025", "FR-015-AC-141")]
#[test]
fn tc_025_negotiation_carries_each_items_identity_and_compares_none() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, BALANCE);
    let (contract, frame) = (ids::contract(), ids::frame());
    let (records, harnesses) = negotiate(
        &fixture,
        [
            (StateFrameRole::Contract, &contract),
            (StateFrameRole::Frame, &frame),
        ],
    );
    assert_eq!(records.len(), 2);
    let [postcondition, frame_harness] = harnesses.as_slice() else {
        panic!("both items emit a harness");
    };
    assert_eq!(postcondition.identity.obligation, contract);
    assert_eq!(frame_harness.identity.obligation, frame);
    // The negotiated harness is the direct entry's, identity included.
    let direct = generate(&fixture, ids::contract(), ids::frame());
    assert_eq!(postcondition, &direct.postcondition);
    assert_eq!(frame_harness, &direct.frame);

    // Two items of one clause carrying equal identities are each carried and neither refused.
    let shared = ids::contract();
    let (same_records, same) = negotiate(
        &fixture,
        [
            (StateFrameRole::Contract, &shared),
            (StateFrameRole::Frame, &shared),
        ],
    );
    assert_eq!(
        same_records, records,
        "equal identities change no disposition"
    );
    assert_eq!(same.len(), 2);
    assert!(same
        .iter()
        .all(|harness| harness.identity.obligation == shared));

    // Classification reads no identity: the records are the same for any identity.
    let other = ids::frame();
    let (other_records, _) = negotiate(
        &fixture,
        [
            (StateFrameRole::Contract, &other),
            (StateFrameRole::Frame, &contract),
        ],
    );
    assert_eq!(other_records, records);
}

/// FR-015-AC-142: the pair constructor refuses an equal pair and builds any other.
#[trace("TC-025", "FR-015-AC-142")]
#[test]
fn tc_025_the_pair_constructor_refuses_only_an_equal_pair() {
    assert_eq!(
        StateFrameRoleIdentities::new(ids::contract(), ids::contract()),
        Err(StateFrameRoleIdentitiesError::SharedIdentity),
        "an equal pair, built independently, is refused"
    );
    for other in [
        ids::frame(),
        ids::contract_ordinal_only(),
        ids::contract_node_only(),
        ids::contract_model_only(),
    ] {
        let built = StateFrameRoleIdentities::new(ids::contract(), other.clone())
            .expect("a pair differing in one member builds");
        assert_eq!(built.contract(), &ids::contract());
        assert_eq!(built.frame(), &other);
    }
}

/// One edit of a parsed record.
type RecordEdit = Box<dyn Fn(&mut Value)>;

fn kani_check() -> String {
    let check = "operation `deposit`: postcondition `post.balance >= pre.balance` failed";
    format!("\"{{}}\",\n{check:?}")
}

fn decode(
    inputs: &mut quire_contract_codegen::StateClauseReplayInputs<'_>,
    harness: &StateFrameIdentity,
) -> Result<(), StateClauseReplayError> {
    let transcript = playback_text(harness, &kani_check(), &[5, 7]);
    let limits = quire_contract_codegen::WitnessDecodeLimits {
        bytes: transcript.len(),
        values: 2,
    };
    inputs.run.decode_limits = limits;
    let outcome = KaniRunOutcome::Falsified {
        counterexample: transcript,
    };
    let playback = parse_state_clause_falsification(
        &outcome,
        harness.harness_symbol.as_str(),
        harness.module_symbol.as_str(),
        limits,
        limits,
    )
    .expect("the playback parses");
    inputs.decode_falsification(harness, &playback, limits)
}

/// FR-024-AC-71 and FR-024-AC-72: the postcondition cross-check accepts the caller's own
/// identity, carries it unchanged, and refuses every other identity.
#[trace("TC-035", "FR-024-AC-71", "FR-024-AC-72")]
#[test]
fn tc_035_postcondition_decode_cross_checks_the_identity() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, BALANCE);
    let pair = generate(&fixture, ids::contract(), ids::frame());
    let harness = &pair.postcondition.identity;
    // FR-024-AC-72: the accepted identity's node and ordinal differ from every other identity
    // this test constructs, so a carry of any other identity cannot pass.
    let distinct = generate(&fixture, ids::accepted(), ids::frame());
    let inputs = |identity: ObligationIdentity| {
        let mut inputs =
            twin.state_clause_inputs(&fixture.package, &fixture.clause, BALANCE, (0, 0), (4, 7));
        inputs.playback.clear();
        inputs.obligation_identity = identity;
        inputs
    };

    // Control: the harness's own identity is accepted and carried into packet and request.
    let mut accepted = inputs(ids::accepted());
    decode(&mut accepted, &distinct.postcondition.identity)
        .expect("the matching identity is accepted");
    assert_eq!(accepted.playback.len(), 2);
    let replay = quire_contract_codegen::StateClauseReplay::new(accepted)
        .expect("the accepted inputs build a replay");
    assert_eq!(replay.packet.obligation_identity, Some(ids::accepted()));
    assert_eq!(replay.wire.obligation_identity, ids::accepted());
    // The refusals below also accept the plain contract identity against its own harness.
    let mut plain = inputs(ids::contract());
    decode(&mut plain, harness).expect("the matching identity is accepted");

    for (name, wrong) in [
        ("ordinal only", ids::contract_ordinal_only()),
        ("node only", ids::contract_node_only()),
        ("model only", ids::contract_model_only()),
        ("the frame role's identity", ids::frame()),
    ] {
        let mut refused = inputs(wrong);
        let error = decode(&mut refused, harness).expect_err(name);
        assert!(
            matches!(error, StateClauseReplayError::ObligationIdentityMismatch),
            "{name}: {error:?}"
        );
        assert!(
            refused.playback.is_empty(),
            "{name}: playback stays unbound"
        );
        assert!(matches!(
            ReplaySettlement::from(&error),
            ReplaySettlement::CgDefect
        ));
    }

    // The identity check precedes the HarnessMismatch checks: wrong identity and a wrong state
    // field list return the identity refusal, while the right identity reaches the field list.
    let mut reordered = harness.clone();
    reordered.state_fields.reverse();
    let mut both = inputs(ids::contract_ordinal_only());
    let error = decode(&mut both, &reordered).expect_err("both differ");
    assert!(matches!(
        error,
        StateClauseReplayError::ObligationIdentityMismatch
    ));
    let mut fields_only = inputs(ids::contract());
    let error = decode(&mut fields_only, &reordered).expect_err("fields differ");
    assert!(matches!(
        error,
        StateClauseReplayError::HarnessMismatch {
            member: StateClauseHarnessMember::StateFields
        }
    ));

    // A frame harness is not a postcondition harness whatever the identity.
    for identity in [ids::frame(), ids::contract(), ids::contract_ordinal_only()] {
        let mut frame_inputs = inputs(identity);
        let error = decode(&mut frame_inputs, &pair.frame.identity).expect_err("a frame harness");
        assert!(matches!(
            error,
            StateClauseReplayError::NotPostconditionHarness
        ));
    }
}

/// FR-024-AC-73 and FR-024-AC-74: the frame replay compares the harness's identity with the one
/// QSL's selected site mints.
#[trace("TC-035", "FR-024-AC-73", "FR-024-AC-74", "FR-032-AC-14", "TC-047")]
#[test]
fn tc_035_frame_replay_cross_checks_the_minted_identity() {
    let twin = Twin::new();
    let minted = twin
        .operation_site("deposit")
        .expect("the operation is located")
        .obligation_identity()
        .clone();
    let harness = frame_harness(&twin);
    assert_eq!(
        harness.obligation, minted,
        "the control carries the minted identity"
    );

    // Control (FR-024-AC-74): accepted; request and packet carry the minted identity.
    let replay = replay_of(&twin, "deposit", &run_of(&harness, [5, 0])).expect("accepted");
    assert_eq!(replay.wire.obligation_identity, minted);
    assert_eq!(replay.packet.obligation_identity, Some(minted.clone()));

    let wrong = [
        ("the contract role's hand-written identity", ids::contract()),
        (
            "the contract harness's minted identity",
            twin.clause_identity(BALANCE),
        ),
        (
            "ordinal only",
            ObligationIdentity::new(
                minted.node().clone(),
                minted.model().cloned(),
                minted.ordinal() + 1,
            ),
        ),
        (
            "node only",
            ObligationIdentity::new(
                twin.clause_identity(BALANCE).node().clone(),
                minted.model().cloned(),
                minted.ordinal(),
            ),
        ),
        (
            "model only",
            ObligationIdentity::new(
                minted.node().clone(),
                Some(ModelIdentity::new("another-model")),
                minted.ordinal(),
            ),
        ),
    ];
    for (name, identity) in wrong {
        assert_ne!(identity, minted, "{name} differs");
        let mut changed = harness.clone();
        changed.obligation = identity;
        let error = match replay_of(&twin, "deposit", &run_of(&changed, [5, 0])) {
            Ok(_) => panic!("{name} must be refused"),
            Err(error) => error,
        };
        assert!(
            matches!(error, FrameReplayError::ObligationIdentityMismatch),
            "{name}: {error}"
        );
        // FR-024-AC-75: a CG defect.
        assert!(matches!(
            ReplaySettlement::from(&error),
            ReplaySettlement::CgDefect
        ));
    }

    // The refusals raised earlier keep their order: a wrong identity beside a wrong operation or
    // a wrong field set still reports those.
    let mut scoped = harness.clone();
    scoped.obligation = ids::contract();
    scoped.scope.operation = "withdraw".to_owned();
    let error = match replay_of(&twin, "deposit", &run_of(&scoped, [5, 0])) {
        Ok(_) => panic!("a wrong operation must be refused"),
        Err(error) => error,
    };
    assert!(
        matches!(error, FrameReplayError::ScopeMismatch { .. }),
        "{error}"
    );
    let mut field_set = harness.clone();
    field_set.obligation = ids::contract();
    field_set.property = quire_contract_codegen::StateFrameProperty::Frame {
        granted: vec!["audit".to_owned()],
        checked: vec![],
    };
    let error = match replay_of(&twin, "deposit", &run_of(&field_set, [5, 0])) {
        Ok(_) => panic!("a wrong field set must be refused"),
        Err(error) => error,
    };
    assert!(
        matches!(error, FrameReplayError::FieldSetMismatch { .. }),
        "{error}"
    );
}

/// FR-024-AC-75: the new variant settles as a CG defect, as its postcondition sibling does.
#[trace("TC-035", "FR-024-AC-75")]
#[test]
fn tc_035_the_frame_identity_mismatch_settles_as_a_cg_defect() {
    let twin = Twin::new();
    let mut harness = frame_harness(&twin);
    harness.obligation = ids::frame();
    let error = match replay_of(&twin, "deposit", &run_of(&harness, [5, 0])) {
        Ok(_) => panic!("a foreign identity must be refused"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        FrameReplayError::ObligationIdentityMismatch
    ));
    assert!(matches!(
        ReplaySettlement::from(&error),
        ReplaySettlement::CgDefect
    ));
    // A non-defect mapping would be a refusal or a reproduction; neither is this.
    assert!(!matches!(
        ReplaySettlement::from(&error),
        ReplaySettlement::Reproduced | ReplaySettlement::Refused(_)
    ));
}

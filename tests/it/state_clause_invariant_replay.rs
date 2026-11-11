//! FR-024-AC-57 to FR-024-AC-64 (IR-1505): the playback of a Falsified V2 invariant harness is put
//! before QSL's `replay_state_clause` as a `Current` observation built by
//! `StateClauseReplay::new_invariant`.
//!
//! Every playback is hand-authored text in Kani's concrete-playback format for the harness CG
//! generates for the fixture (no installed Kani run), parsed by `parse_assertion_playback` and
//! decoded by the shared decoder against that harness's identity and state fields. The package is
//! QSL's own emission for the twin's unit, so the node identities are QSL's.

// `StateClauseReplay` and the decoded inputs are not `Debug`, so `expect_err` is unavailable.
#![allow(clippy::err_expect)]

use std::cell::Cell;

use qsl_replay::{
    replay_state_clause, AnchorKind, ClauseName, ClauseSelectionInput, DisagreementCause,
    Identifier, ModelIdentity, ObligationIdentity, ReplayRefusal, ReplayResult,
    StateClauseReplayResult, WitnessSettlement,
};
use quire_contract_codegen::{
    negotiate_kani_native_obligations, negotiate_kani_obligations, parse_assertion_playback,
    KaniObligationOutcome, KaniObligationRequest, KaniV2ClauseObligationHarness, ObligationItem,
    ReplaySettlement, StateClauseHarness, StateClauseHarnessMember, StateClauseReplay,
    StateClauseReplayError, StateFrameObligations, StateFrameRequest,
};
use quire_contract_model::{CheckedNodeId, CheckedPackageV2};
use quire_semantic_value::catalog::Category;
use serde_json::{json, Value};

use super::kani_collection_operation as collection_operation;
use super::kani_obligations_state_frame::native_twin::{playback_text_for_path, Twin};
use ix_trace_rs::trace;

/// The message of the generated invariant assertion, as Kani prints it between two quote layers.
const ASSERTION: &str = "\"the V2 invariant holds for every admitted state\"";
const SUBJECT: &str = "crate::request::unrelated";

/// A generated V2 clause harness over a QSL-emitted package.
struct Fixture {
    package: CheckedPackageV2,
    clause: CheckedNodeId,
    harness: KaniV2ClauseObligationHarness,
}

fn generate(twin: &Twin, clause: &str) -> Fixture {
    generate_with(twin, clause, false)
}

/// The harness generated for `clause`; a native postcondition goes through the native route.
fn generate_with(twin: &Twin, clause: &str, native: bool) -> Fixture {
    generate_claimed(twin, clause, native, None)
}

/// As [`generate_with`], the claim's identity replaced by `identity` when given (the identity is
/// built by hand here, so the harness carries a node, model and ordinal of this test's choosing).
fn generate_claimed(
    twin: &Twin,
    clause: &str,
    native: bool,
    identity: Option<&ObligationIdentity>,
) -> Fixture {
    let (package, node) = twin.emitted_package(clause);
    let negotiate = if native {
        negotiate_kani_native_obligations
    } else {
        negotiate_kani_obligations
    };
    let mut claims = crate::common::v2_claim::claims_for(&package, std::slice::from_ref(&node));
    if let Some(identity) = identity {
        claims[0].obligation = identity.clone();
    }
    let outcome = negotiate(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items: &[ObligationItem::V2Clause {
            package: &package,
            clause: &node,
            claim: &claims[0],
            dependencies: None,
        }],
        subject_path: SUBJECT,
        unwind: 4,
    })
    .expect("a well-formed request");
    let KaniObligationOutcome::Emitted {
        mut v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the clause is admitted");
    };
    assert_eq!(v2_clause_harnesses.len(), 1, "one harness");
    Fixture {
        package,
        clause: node,
        harness: v2_clause_harnesses.remove(0),
    }
}

impl Fixture {
    /// The object declaration the harness draws its fields from.
    fn object(&self) -> CheckedNodeId {
        self.harness.identity.state_fields[0]
            .declaring_node_id
            .clone()
    }

    /// Hand-authored Kani playback text for this harness's falsified invariant assertion.
    fn playback(&self, values: &[i64]) -> String {
        self.playback_of(ASSERTION, values)
    }

    fn playback_of(&self, check: &str, values: &[i64]) -> String {
        let identity = &self.harness.identity;
        playback_text_for_path(
            &format!("{}::{}", identity.module_symbol, identity.harness_symbol),
            check,
            values,
        )
    }

    /// Inputs decoded from `text` as the playback of `harness`.
    fn decoded<'a>(
        &'a self,
        twin: &Twin,
        object: &'a CheckedNodeId,
        clause: &str,
        text: &str,
        harness: StateClauseHarness<'_>,
    ) -> Result<quire_contract_codegen::InvariantReplayInputs<'a>, StateClauseReplayError> {
        let mut inputs =
            twin.invariant_inputs(&self.package, &self.clause, object, clause, &["balance"]);
        // The inputs hold QSL's own call-site identity, which equals the harness's now that a
        // claim's NodeKey is the clause's resolved key (FR-024-AC-65).
        assert_eq!(inputs.obligation_identity, self.harness.obligation);
        let identity = &self.harness.identity;
        let parsed = parse_assertion_playback(
            text,
            identity.harness_symbol.as_str(),
            identity.module_symbol.as_str(),
            usize::MAX,
            usize::MAX,
        )
        .expect("the hand-authored playback parses");
        inputs.decode_falsification(harness, &parsed, crate::unbounded_decode_limits())?;
        Ok(inputs)
    }

    fn built(&self, twin: &Twin, clause: &str, values: &[i64]) -> StateClauseReplay {
        let object = self.object();
        let inputs = self
            .decoded(
                twin,
                &object,
                clause,
                &self.playback(values),
                StateClauseHarness::V2Clause(&self.harness),
            )
            .expect("the playback decodes");
        StateClauseReplay::new_invariant(inputs).expect("the replay is built")
    }
}

/// What a replay settled: the arm's settlement, its category, its disagreement cause and the
/// clause node it names.
fn settled(
    result: &StateClauseReplayResult,
) -> (WitnessSettlement, Category, Option<DisagreementCause>) {
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("the invariant envelope is on the witness arm");
    };
    (
        arm.settlement(),
        arm.category(),
        arm.disagreement().cloned(),
    )
}

fn assert_violation(result: &StateClauseReplayResult) {
    assert_eq!(
        settled(result),
        (
            WitnessSettlement::ReproducedWithEvaluatedWitness,
            Category::Violation,
            None
        )
    );
}

fn assert_verdicts(result: &StateClauseReplayResult) {
    let (settlement, category, cause) = settled(result);
    assert_eq!(settlement, WitnessSettlement::Inconclusive);
    assert_eq!(category, Category::Success);
    assert!(
        matches!(cause, Some(DisagreementCause::Verdicts { .. })),
        "{cause:?}"
    );
}

fn snapshot_document(replay: &StateClauseReplay) -> Value {
    let snapshots = replay
        .wire
        .documents
        .iter()
        .filter_map(|bytes| serde_json::from_slice::<Value>(bytes).ok())
        .filter(|document| document["format"] == "quire.state.snapshot/v1")
        .collect::<Vec<_>>();
    assert_eq!(snapshots.len(), 1, "exactly one state document");
    snapshots.into_iter().next().expect("one snapshot")
}

fn strengthened() -> Twin {
    Twin::with_bank_invariants(&[("NonNegative", "self.balance >= 1")], (0, 100))
}

fn widened() -> Twin {
    Twin::with_bank_invariants(&[("NonNegative", "self.balance >= 0")], (-1, 100))
}

fn unmutated() -> Twin {
    Twin::with_bank_invariants(&[("NonNegative", "self.balance >= 0")], (0, 100))
}

const CLAUSE: &str = "NonNegative";
const VAULT: &str = "ix://test/bank/Vault";

/// Step 39 (FR-024-AC-57): the envelope's observation is `Current` carrying the snapshot, the
/// caller's anchor and the self object; one state document, no invocation, no post snapshot; the
/// snapshot's anchor is the selection's.
#[trace("TC-035", "FR-024-AC-57")]
#[test]
fn tc_035_invariant_playback_builds_the_current_envelope() {
    let twin = strengthened();
    let fixture = generate(&twin, CLAUSE);
    let replay = fixture.built(&twin, CLAUSE, &[0]);
    let payload = replay.packet.family_payload.as_ref().expect("a payload");
    let ClauseSelectionInput::Current {
        snapshot,
        anchor,
        self_object,
    } = &payload.observation
    else {
        panic!("the observation is `Current`, not `Invocation`");
    };
    assert_eq!(anchor.kind, AnchorKind::Handler);
    assert_eq!(anchor.name, "deposit");
    let object = self_object.as_ref().expect("a self object");
    assert_eq!(
        (object.population.as_str(), object.key.as_str()),
        ("ix://test/bank/accounts", "account")
    );

    // One state document: the current snapshot. No invocation and no post snapshot.
    let formats = replay
        .wire
        .documents
        .iter()
        .filter_map(|bytes| serde_json::from_slice::<Value>(bytes).ok())
        .filter_map(|document| document["format"].as_str().map(str::to_owned))
        .collect::<Vec<_>>();
    assert_eq!(formats, ["quire.state.snapshot/v1"]);
    let document = snapshot_document(&replay);
    assert_eq!(document["observation"], "current");
    assert_eq!(
        document["identity"],
        json!({"authority": snapshot.authority, "identity": snapshot.identity})
    );
    assert_eq!(
        document["anchor"],
        json!({"kind": "handler", "name": "deposit"}),
        "the snapshot names the anchor the selection names"
    );
    assert_eq!(
        document["populations"][0]["objects"][0]["fields"],
        json!({"balance": {"integer": "0"}})
    );

    // The playback value reaches the document.
    let other = fixture.built(&twin, CLAUSE, &[7]);
    assert_eq!(
        snapshot_document(&other)["populations"][0]["objects"][0]["fields"],
        json!({"balance": {"integer": "7"}})
    );

    // The identities are the call site's.
    let site = twin.clause_site(CLAUSE).expect("the clause is declared");
    assert_eq!(replay.packet.clause_node, Some(site.node));
    assert_eq!(
        replay.packet.occurrence_key.as_ref(),
        Some(&site.occurrence)
    );
}

/// Step 40 (FR-024-AC-58): an object declaring `balance` and an unread `audit`. The harness draws
/// `balance`; CG does not return `MissingField` for `audit` and invents no value, so QSL refuses
/// the snapshot for the missing `audit`.
#[trace("TC-035", "FR-024-AC-58")]
#[test]
fn tc_035_unread_declared_field_ends_refused_with_missing_member() {
    let twin = Twin::with_balance_invariant();
    let fixture = generate(&twin, "BalanceNonnegative");
    assert_eq!(
        fixture
            .harness
            .identity
            .state_fields
            .iter()
            .map(|field| field.field_name.as_str())
            .collect::<Vec<_>>(),
        ["balance"],
        "the harness draws only the field the invariant reads"
    );
    let replay = fixture.built(&twin, "BalanceNonnegative", &[0]);
    assert_eq!(
        snapshot_document(&replay)["populations"][0]["objects"][0]["fields"],
        json!({"balance": {"integer": "0"}}),
        "no value is invented for `audit`"
    );
    let error = replay.replay().err().expect("QSL refuses the snapshot");
    let StateClauseReplayError::Refused(refusal) = error else {
        panic!("the replay ends Refused, not MissingField: {error}");
    };
    assert_eq!(refusal.code().as_str(), "invalid_runtime_input");
    assert_eq!(refusal.cause(), Some("missing-member"));
    assert!(
        format!("{refusal:?}").contains("audit"),
        "the refusal names the unread field: {refusal:?}"
    );
}

/// Step 41 (FR-024-AC-59): the invariant is selected by its declared name, not by position.
#[trace("TC-035", "FR-024-AC-59")]
#[test]
fn tc_035_invariant_is_selected_by_its_declared_name() {
    let invariants = [
        ("Positive", "self.balance >= 1"),
        ("Capped", "self.balance <= 100"),
    ];
    let mut nodes = Vec::new();
    for (clause, expected_violation) in [("Positive", true), ("Capped", false)] {
        // Declared in the order Positive, Capped, then in reverse.
        for order in [
            invariants.to_vec(),
            invariants.iter().rev().copied().collect(),
        ] {
            let twin = Twin::with_bank_invariants(&order, (0, 100));
            let fixture = generate(&twin, clause);
            let replay = fixture.built(&twin, clause, &[0]);
            let site = twin.clause_site(clause).expect("the clause is declared");
            assert_eq!(
                replay
                    .packet
                    .replay_target
                    .state_clause
                    .as_ref()
                    .map(ToString::to_string),
                Some(clause.to_owned())
            );
            assert_eq!(replay.packet.clause_node, Some(site.node.clone()));
            assert_eq!(
                replay.packet.occurrence_key.as_ref(),
                Some(&site.occurrence)
            );
            let result = replay.replay().expect("replays");
            assert_eq!(result.clause_node(), &site.node);
            assert_eq!(result.occurrence_key(), &site.occurrence);
            if expected_violation {
                assert_violation(&result);
            } else {
                assert_verdicts(&result);
            }
            nodes.push((clause, site.node));
        }
    }
    assert_ne!(nodes[0].1, nodes[2].1, "each invariant has its own node");
}

/// Step 42 (FR-024-AC-60): the result is the executor's, unchanged; a violating playback
/// reproduces, a satisfying one is `Ok` and `inconclusive`/`Verdicts`.
#[trace("TC-035", "FR-024-AC-60")]
#[test]
fn tc_035_invariant_replay_returns_the_executor_result_unchanged() {
    let twin = strengthened();
    let fixture = generate(&twin, CLAUSE);
    for (value, violating) in [(0, true), (5, false)] {
        let replay = fixture.built(&twin, CLAUSE, &[value]);
        let limits = replay.replay_limits();
        let direct = Cell::new(None);
        let through = fixture
            .built(&twin, CLAUSE, &[value])
            .replay_through(|wire, envelope| {
                let result = replay_state_clause(wire, envelope, limits);
                direct.set(Some(()));
                result
            })
            .expect("the replay returns Ok");
        assert!(direct.get().is_some());
        let result = replay.replay().expect("a satisfying playback is Ok too");
        assert_eq!(
            result, through,
            "the executor's result is returned unchanged"
        );
        if violating {
            assert_violation(&result);
        } else {
            assert_verdicts(&result);
        }
    }

    // The executor seam: a sentinel changes the result.
    let sentinel_refusal = ReplayRefusal::WrongModelGraphEntry;
    let sentinel = fixture
        .built(&twin, CLAUSE, &[0])
        .replay_through(|_, _| Err(sentinel_refusal))
        .err()
        .expect("the executor's refusal is returned");
    assert!(matches!(
        sentinel,
        StateClauseReplayError::Refused(refusal) if matches!(*refusal, ReplayRefusal::WrongModelGraphEntry)
    ));
}

/// Step 43 (FR-024-AC-61): each playback refusal is its own variant and the executor is not
/// called.
#[trace("TC-035", "FR-024-AC-61")]
#[test]
fn tc_035_each_playback_refusal_is_a_distinct_variant_before_any_document() {
    let twin = widened();
    let fixture = generate(&twin, CLAUSE);
    let object = fixture.object();
    let good = fixture.playback(&[1]);
    let decode = |text: &str| {
        fixture
            .decoded(
                &twin,
                &object,
                CLAUSE,
                text,
                StateClauseHarness::V2Clause(&fixture.harness),
            )
            .expect("decodes")
    };
    let refuse = |inputs| {
        let called = Cell::new(false);
        let error = match StateClauseReplay::new_invariant(inputs) {
            Ok(replay) => replay
                .replay_through(|_, _| {
                    called.set(true);
                    Err(ReplayRefusal::WrongModelGraphEntry)
                })
                .err()
                .expect("refused"),
            Err(error) => error,
        };
        assert!(!called.get(), "the executor is not called");
        error
    };

    let mut missing = decode(&good);
    missing.playback.clear();
    assert!(
        matches!(refuse(missing), StateClauseReplayError::MissingField { field } if field == "balance")
    );

    let mut undeclared = decode(&good);
    undeclared.playback.push(("ghost".to_owned(), 1));
    assert!(
        matches!(refuse(undeclared), StateClauseReplayError::UndeclaredField { field } if field == "ghost")
    );

    let mut duplicate = decode(&good);
    duplicate.playback.push(("balance".to_owned(), 2));
    assert!(
        matches!(refuse(duplicate), StateClauseReplayError::DuplicateField { field } if field == "balance")
    );

    // The widened field admits -1 and refuses -2 and 101.
    assert!(StateClauseReplay::new_invariant(decode(&fixture.playback(&[-1]))).is_ok());
    for value in [-2, 101] {
        let mut out = decode(&good);
        out.playback = vec![("balance".to_owned(), value)];
        assert!(
            matches!(refuse(out), StateClauseReplayError::OutOfDomain { field } if field == "balance"),
            "{value}"
        );
    }

    // Another harness's playback, and the wrong count or width, are decode refusals.
    let other_path = fixture.harness.identity.module_symbol.as_str().to_owned() + "::other";
    let wrong_harness = playback_text_for_path(&other_path, ASSERTION, &[1]);
    for text in [
        wrong_harness,
        fixture.playback(&[1, 2]),
        fixture.playback(&[]),
        good.replace("vec![1, 0, 0, 0, 0, 0, 0, 0]", "vec![1, 0]"),
    ] {
        let error = fixture
            .decoded(
                &twin,
                &object,
                CLAUSE,
                &text,
                StateClauseHarness::V2Clause(&fixture.harness),
            )
            .err()
            .expect("refused");
        assert!(
            matches!(error, StateClauseReplayError::Decode(_)),
            "{error}"
        );
    }
}

/// Step 44 (FR-024-AC-62): the closed refusal set; the wrong-anchor refusal is QSL's.
#[trace("TC-035", "FR-024-AC-62")]
#[test]
fn tc_035_the_refusal_set_is_closed_and_typed() {
    let twin = strengthened();
    let fixture = generate(&twin, CLAUSE);
    let object = fixture.object();
    let good = fixture.playback(&[0]);
    let v2 = StateClauseHarness::V2Clause(&fixture.harness);

    // A postcondition, a precondition and a frame harness are not invariant harnesses.
    for (other, name, native) in [
        (
            Twin::with_bounded_amount_implication(),
            "BalanceNeverDrops",
            true,
        ),
        (
            Twin::with_bounded_other_precondition(),
            "OtherBelowLimit",
            false,
        ),
    ] {
        let other_fixture = generate_with(&other, name, native);
        let text = other_fixture.playback(&[0]);
        let error = fixture
            .decoded(
                &twin,
                &object,
                CLAUSE,
                &text,
                StateClauseHarness::V2Clause(&other_fixture.harness),
            )
            .err()
            .expect("refused");
        assert!(
            matches!(error, StateClauseReplayError::NotInvariantHarness),
            "{name}: {error}"
        );
    }
    let frame_twin = Twin::new();
    let (frame_package, frame_clause) = frame_twin.emitted_package("BalanceNeverDrops");
    let frames: StateFrameObligations =
        quire_contract_codegen::generate_state_frame_obligations(&StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            package: &frame_package,
            clause: &frame_clause,
            state_path: "crate::subject::Account",
            state_fields: &["balance", "audit"],
            subject_path: "crate::subject::deposit_debiting_within_range",
            unwind: 4,
        })
        .expect("state-frame harnesses generate");
    for harness in [&frames.postcondition.identity, &frames.frame.identity] {
        let error = fixture
            .decoded(
                &twin,
                &object,
                CLAUSE,
                &good,
                StateClauseHarness::StateFrame(harness),
            )
            .err()
            .expect("refused");
        assert!(
            matches!(error, StateClauseReplayError::NotInvariantHarness),
            "{error}"
        );
    }

    // The harness differs from the replay inputs: obligation, object and field order.
    let other_clause = Twin::with_bank_invariants(
        &[
            ("NonNegative", "self.balance >= 1"),
            ("Other", "self.balance >= 2"),
        ],
        (0, 100),
    );
    let two = generate(&other_clause, "Other");
    let mut inputs =
        other_clause.invariant_inputs(&two.package, &two.clause, &object, "Other", &["balance"]);
    assert_eq!(inputs.obligation_identity, two.harness.obligation);
    let two_text = two.playback(&[0]);
    let parsed_two = parse_assertion_playback(
        &two_text,
        two.harness.identity.harness_symbol.as_str(),
        two.harness.identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("parses");
    // Replay inputs naming another clause node than the harness's.
    let first = generate(&other_clause, "NonNegative");
    inputs.clause_node = &first.clause;
    let error = inputs
        .decode_falsification(
            StateClauseHarness::V2Clause(&two.harness),
            &parsed_two,
            crate::unbounded_decode_limits(),
        )
        .err()
        .expect("refused");
    assert!(
        matches!(
            error,
            StateClauseReplayError::HarnessMismatch {
                member: StateClauseHarnessMember::Clause
            }
        ),
        "{error}"
    );
    let parsed = parse_assertion_playback(
        &good,
        fixture.harness.identity.harness_symbol.as_str(),
        fixture.harness.identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("parses");
    let mut inputs = twin.invariant_inputs(
        &fixture.package,
        &fixture.clause,
        &object,
        CLAUSE,
        &["balance", "audit"],
    );
    assert_eq!(inputs.obligation_identity, fixture.harness.obligation);
    let error = inputs
        .decode_falsification(v2, &parsed, crate::unbounded_decode_limits())
        .err()
        .expect("refused");
    assert!(
        matches!(
            error,
            StateClauseReplayError::HarnessMismatch {
                member: StateClauseHarnessMember::StateFields
            }
        ),
        "{error}"
    );
    let wrong_object = fixture.clause.clone();
    let mut inputs = twin.invariant_inputs(
        &fixture.package,
        &fixture.clause,
        &wrong_object,
        CLAUSE,
        &["balance"],
    );
    assert_eq!(inputs.obligation_identity, fixture.harness.obligation);
    let error = inputs
        .decode_falsification(v2, &parsed, crate::unbounded_decode_limits())
        .err()
        .expect("refused");
    assert!(
        matches!(
            error,
            StateClauseReplayError::HarnessMismatch {
                member: StateClauseHarnessMember::Object
            }
        ),
        "{error}"
    );

    // The failed check is not the generated invariant assertion.
    for check in [
        "\"another assertion\"".to_owned(),
        format!("{ASSERTION} extra"),
        "\"{}\",\n\"operation `deposit`: postcondition `post.balance >= pre.balance` failed\""
            .to_owned(),
    ] {
        let error = fixture
            .decoded(
                &twin,
                &object,
                CLAUSE,
                &fixture.playback_of(&check, &[0]),
                v2,
            )
            .err()
            .expect("refused");
        assert!(
            matches!(error, StateClauseReplayError::AssertionMismatch),
            "{check}: {error}"
        );
    }

    // An undeclared clause name is the call site's refusal.
    let mut undeclared = fixture
        .decoded(&twin, &object, CLAUSE, &good, v2)
        .expect("decodes");
    undeclared.clause = ClauseName(Identifier::new("NoSuchClause").expect("identifier"));
    assert!(matches!(
        StateClauseReplay::new_invariant(undeclared)
            .err()
            .expect("refused"),
        StateClauseReplayError::CallSite(_)
    ));

    // A clause node other than the one QSL names for the clause.
    let multi = Twin::with_bank_invariants(
        &[
            ("NonNegative", "self.balance >= 1"),
            ("Other", "self.balance >= 2"),
        ],
        (0, 100),
    );
    let (multi_package, other_node) = multi.emitted_package("Other");
    let mut selected = multi.invariant_inputs(
        &multi_package,
        &other_node,
        &object,
        "NonNegative",
        &["balance"],
    );
    selected.playback = vec![("balance".to_owned(), 0)];
    assert!(matches!(
        StateClauseReplay::new_invariant(selected)
            .err()
            .expect("refused"),
        StateClauseReplayError::ClauseSelectionMismatch
    ));

    // A snapshot anchored elsewhere than the selection is QSL's refusal, not CG's: replace the
    // snapshot in the built request and call QSL.
    let replay = fixture.built(&twin, CLAUSE, &[0]);
    let limits = replay.replay_limits();
    let error = replay
        .replay_through(|mut wire, envelope| {
            for bytes in &mut wire.documents {
                let Ok(mut document) = serde_json::from_slice::<Value>(bytes) else {
                    continue;
                };
                if document["format"] == "quire.state.snapshot/v1" {
                    document["anchor"] = json!({"kind": "handler", "name": "withdraw"});
                    *bytes = document.to_string().into_bytes();
                }
            }
            replay_state_clause(wire, envelope, limits)
        })
        .err()
        .expect("QSL refuses the wrong anchor");
    let StateClauseReplayError::Refused(refusal) = error else {
        panic!("QSL's refusal comes back as Refused: {error}");
    };
    assert_eq!(refusal.code().as_str(), "wrong_snapshot");
    assert_eq!(refusal.cause(), Some("wrong-anchor"));
}

/// Step 45 (FR-024-AC-63, FR-024-AC-64): the transcript names the anchor, the payload carries no
/// witness record; the Bank mutants settle a violation and the unmutated package does not.
#[trace("TC-035", "FR-024-AC-63", "FR-024-AC-64")]
#[test]
fn tc_035_bank_invariant_mutants_settle_a_violation() {
    // The transcript's operation name is the anchor's name and the witness member is none.
    let twin = strengthened();
    let fixture = generate(&twin, CLAUSE);
    let replay = fixture.built(&twin, CLAUSE, &[0]);
    let payload = replay.packet.family_payload.as_ref().expect("a payload");
    assert!(payload.witness.is_none());
    let qsl_replay::ReplaySource::Witness(witness) = &replay.wire.source else {
        panic!("the invariant envelope rides the witness arm");
    };
    assert_eq!(
        witness.transcript(),
        "<<<assertion|deposit|NonNegative|balance=0>>>",
        "the operation name is the anchor's name"
    );

    // Hand-authored playback decoded against each generated mutant harness.
    for (twin, value) in [(strengthened(), 0), (widened(), -1)] {
        let fixture = generate(&twin, CLAUSE);
        let result = fixture
            .built(&twin, CLAUSE, &[value])
            .replay()
            .expect("the mutant replays");
        assert_violation(&result);
        let site = twin.clause_site(CLAUSE).expect("site");
        assert_eq!(result.clause_node(), &site.node);
    }

    // The unmutated package with `balance = 0` is Ok and does not settle a violation.
    let twin = unmutated();
    let fixture = generate(&twin, CLAUSE);
    let result = fixture
        .built(&twin, CLAUSE, &[0])
        .replay()
        .expect("the unmutated invariant replays");
    assert_verdicts(&result);

    // The widened playback is outside the unmutated range: the range is the package's.
    let unmutated_object = fixture.object();
    let inputs = fixture
        .decoded(
            &twin,
            &unmutated_object,
            CLAUSE,
            &fixture.playback(&[-1]),
            StateClauseHarness::V2Clause(&fixture.harness),
        )
        .expect("decodes");
    assert!(matches!(
        StateClauseReplay::new_invariant(inputs)
            .err()
            .expect("refused"),
        StateClauseReplayError::OutOfDomain { .. }
    ));
}

/// FR-024-AC-62: the remaining CG-side refusals of the decoder and the constructor.
#[trace("TC-035", "FR-024-AC-62")]
#[test]
fn tc_035_the_remaining_invariant_refusals_are_typed() {
    let twin = strengthened();
    let fixture = generate(&twin, CLAUSE);
    let object = fixture.object();
    let good = fixture.playback(&[0]);
    let identity = &fixture.harness.identity;
    let parsed = parse_assertion_playback(
        &good,
        identity.harness_symbol.as_str(),
        identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("parses");
    let decode = |harness: &KaniV2ClauseObligationHarness| {
        let mut inputs = twin.invariant_inputs(
            &fixture.package,
            &fixture.clause,
            &object,
            CLAUSE,
            &["balance"],
        );
        assert_eq!(inputs.obligation_identity, fixture.harness.obligation);
        inputs.decode_falsification(
            StateClauseHarness::V2Clause(harness),
            &parsed,
            crate::unbounded_decode_limits(),
        )
    };

    // A collection harness is no scalar invariant harness, whatever its other members say.
    let package = collection_operation::fixture(
        &collection_operation::document(),
        "pre Ready using v on M::Node::step { forall(item in self.z_items: item >= 1) }",
    );
    let ids = collection_operation::clauses(&package);
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = collection_operation::negotiate(&package, &ids)
    else {
        panic!("valid request")
    };
    let collection = v2_clause_harnesses
        .iter()
        .find(|harness| {
            harness.identity.collection_precondition.is_some()
                && harness.identity.collection_operation.is_some()
        })
        .expect("a collection precondition harness");
    let mut with_precondition = fixture.harness.clone();
    with_precondition.identity.collection_precondition =
        collection.identity.collection_precondition.clone();
    let mut with_operation = fixture.harness.clone();
    with_operation.identity.collection_operation = collection.identity.collection_operation.clone();
    for harness in [&with_precondition, &with_operation] {
        assert!(matches!(
            decode(harness),
            Err(StateClauseReplayError::NotInvariantHarness)
        ));
    }

    // An edited public field no longer matches what the generator sealed.
    let mut edited = fixture.harness.clone();
    edited.identity.subject_path = "crate::edited".to_owned();
    assert!(matches!(
        decode(&edited),
        Err(StateClauseReplayError::Decode(cause))
            if cause.code == "cg_witness_harness_provenance_mismatch"
    ));
    assert!(decode(&fixture.harness).is_ok());

    // An invariant that reads no field draws none: the object cannot be derived.
    let constant = Twin::with_bank_invariants(&[("Constant", "1 >= 0")], (0, 100));
    let none = generate(&constant, "Constant");
    assert!(none.harness.identity.state_fields.is_empty());
    let none_object = none.clause.clone();
    let mut inputs =
        constant.invariant_inputs(&none.package, &none.clause, &none_object, "Constant", &[]);
    assert_eq!(inputs.obligation_identity, none.harness.obligation);
    let none_identity = &none.harness.identity;
    let none_text = none.playback(&[]);
    let none_parsed = parse_assertion_playback(
        &none_text,
        none_identity.harness_symbol.as_str(),
        none_identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("parses");
    assert!(matches!(
        inputs.decode_falsification(
            StateClauseHarness::V2Clause(&none.harness),
            &none_parsed,
            crate::unbounded_decode_limits()
        ),
        Err(StateClauseReplayError::HarnessMismatch {
            member: StateClauseHarnessMember::StateFields
        })
    ));

    // The constructor itself refuses an object other than the one the invariant is declared on,
    // and a drawn field that object does not declare, without calling the executor.
    let vault_twin = Twin::with_bank_invariants(&[(CLAUSE, "self.balance >= 0")], (0, 100))
        .with_vault_declaring_balance();
    let vault_fixture = generate(&vault_twin, CLAUSE);
    let account = vault_fixture.object();
    let vault = vault_fixture
        .package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| {
            matches!(&node.owner, Some(quire_contract_model::CheckedNodeOwner::Model { node, .. })
                if node.as_ref() == VAULT)
        })
        .expect("the Vault object type")
        .node_id
        .clone();
    assert_ne!(vault, account);
    let vault_good = vault_fixture.playback(&[0]);
    let object_mismatch = |object_node: &CheckedNodeId, object_type: &str| {
        let mut inputs = vault_fixture
            .decoded(
                &vault_twin,
                &account,
                CLAUSE,
                &vault_good,
                StateClauseHarness::V2Clause(&vault_fixture.harness),
            )
            .expect("decodes");
        inputs.object_node = object_node;
        inputs.object.object_type = object_type.to_owned();
        StateClauseReplay::new_invariant(inputs)
            .err()
            .expect("refused")
    };
    // (a) Vault declares `balance` too, with another range, and is a model object whose own
    // address the snapshot names: only the invariant's declaring object separates them.
    assert!(matches!(
        object_mismatch(&vault, VAULT),
        StateClauseReplayError::HarnessMismatch {
            member: StateClauseHarnessMember::Object
        }
    ));
    // (b) The declaring object with another object type's address.
    assert!(matches!(
        object_mismatch(&account, VAULT),
        StateClauseReplayError::HarnessMismatch {
            member: StateClauseHarnessMember::Object
        }
    ));
    // The matching pair is accepted.
    let ok = vault_fixture
        .decoded(
            &vault_twin,
            &account,
            CLAUSE,
            &vault_good,
            StateClauseHarness::V2Clause(&vault_fixture.harness),
        )
        .expect("decodes");
    assert!(StateClauseReplay::new_invariant(ok).is_ok());
    let mut absent = fixture
        .decoded(
            &twin,
            &object,
            CLAUSE,
            &good,
            StateClauseHarness::V2Clause(&fixture.harness),
        )
        .expect("decodes");
    absent.state_fields = vec!["ghost".to_owned()];
    assert!(matches!(
        StateClauseReplay::new_invariant(absent).err().expect("refused"),
        StateClauseReplayError::ModelFields { cause: quire_contract_codegen::StateClauseModelFieldsCause::Absent { field }, .. }
            if field == "ghost"
    ));
}

/// The harness identity of step 46: a node, a model and an ordinal that none of the decoys share.
const IDENTITY_ORDINAL: u32 = 7;

/// A fixture whose harness carries an identity built by hand, and the same package's other
/// clause node, whose key is the decoy node.
fn identified(twin: &Twin) -> (Fixture, ObligationIdentity, quire_exact::NodeKey) {
    let (package, node) = twin.emitted_package(CLAUSE);
    let own = crate::common::v2_claim::claim_for(&package, &node).obligation;
    let (_, other) = twin.emitted_package("Other");
    let other_node = crate::common::v2_claim::claim_for(&package, &other)
        .obligation
        .node()
        .clone();
    assert_ne!(&other_node, own.node(), "the decoy node is another node");
    let identity = ObligationIdentity::new(
        own.node().clone(),
        Some(ModelIdentity::new("test/bank-model")),
        IDENTITY_ORDINAL,
    );
    (
        generate_claimed(twin, CLAUSE, false, Some(&identity)),
        identity,
        other_node,
    )
}

fn two_clause_twin() -> Twin {
    Twin::with_bank_invariants(
        &[
            ("NonNegative", "self.balance >= 1"),
            ("Other", "self.balance >= 2"),
        ],
        (0, 100),
    )
}

/// Step 46 (FR-024-AC-66): an identity equal to the harness's, member for member, is accepted
/// and carried unchanged into the packet and the request; its ordinal and node differ from every
/// other identity this test constructs.
#[trace("TC-035", "FR-024-AC-66")]
#[test]
fn tc_035_an_equal_obligation_identity_is_accepted_and_carried_unchanged() {
    let twin = two_clause_twin();
    let (fixture, identity, other_node) = identified(&twin);
    assert_eq!(
        fixture.harness.obligation, identity,
        "the harness carries it"
    );
    let object = fixture.object();
    let mut inputs = twin.invariant_inputs(
        &fixture.package,
        &fixture.clause,
        &object,
        CLAUSE,
        &["balance"],
    );
    // The caller's own copy, equal member for member, not the harness's value.
    inputs.obligation_identity = ObligationIdentity::new(
        identity.node().clone(),
        identity.model().cloned(),
        identity.ordinal(),
    );
    let harness_identity = &fixture.harness.identity;
    let text = fixture.playback(&[0]);
    let parsed = parse_assertion_playback(
        &text,
        harness_identity.harness_symbol.as_str(),
        harness_identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("parses");
    inputs
        .decode_falsification(
            StateClauseHarness::V2Clause(&fixture.harness),
            &parsed,
            crate::unbounded_decode_limits(),
        )
        .expect("an equal identity is accepted");
    assert_eq!(inputs.playback, vec![("balance".to_owned(), 0)]);

    let replay = StateClauseReplay::new_invariant(inputs).expect("the replay is built");
    let packet = replay
        .packet
        .obligation_identity
        .as_ref()
        .expect("the packet carries an identity");
    for carried in [packet, &replay.wire.obligation_identity] {
        assert_eq!(carried.node(), identity.node());
        assert_eq!(carried.model(), identity.model());
        assert_eq!(carried.ordinal(), identity.ordinal());
        assert_eq!(carried.ordinal(), IDENTITY_ORDINAL);
        assert_ne!(carried.node(), &other_node);
        assert_ne!(carried.ordinal(), 0);
        assert!(carried.model().is_some());
    }
}

/// Step 46 (FR-024-AC-65): a caller identity differing from the harness's in any one member is
/// `ObligationIdentityMismatch`, never `HarnessMismatch`, with the playback unbound; the check
/// follows `NotInvariantHarness` and precedes the field-order check; the variant settles as a CG
/// defect.
#[trace("TC-035", "FR-024-AC-65")]
#[test]
fn tc_035_a_different_obligation_identity_is_refused_before_the_harness_members() {
    let twin = two_clause_twin();
    let (fixture, identity, other_node) = identified(&twin);
    let object = fixture.object();
    let text = fixture.playback(&[0]);
    let parsed = {
        let harness_identity = &fixture.harness.identity;
        parse_assertion_playback(
            &text,
            harness_identity.harness_symbol.as_str(),
            harness_identity.module_symbol.as_str(),
            usize::MAX,
            usize::MAX,
        )
        .expect("parses")
    };
    let v2 = StateClauseHarness::V2Clause(&fixture.harness);
    let refuse = |caller: ObligationIdentity, fields: &[&str], harness: StateClauseHarness<'_>| {
        let mut inputs =
            twin.invariant_inputs(&fixture.package, &fixture.clause, &object, CLAUSE, fields);
        inputs.obligation_identity = caller;
        let error = inputs
            .decode_falsification(harness, &parsed, crate::unbounded_decode_limits())
            .err()
            .expect("refused");
        assert!(inputs.playback.is_empty(), "the playback is left unbound");
        error
    };

    // Each member alone, the ordinal alone first.
    let ordinal_only = ObligationIdentity::new(
        identity.node().clone(),
        identity.model().cloned(),
        IDENTITY_ORDINAL + 1,
    );
    let model_only = ObligationIdentity::new(identity.node().clone(), None, IDENTITY_ORDINAL);
    let node_only = ObligationIdentity::new(
        other_node.clone(),
        identity.model().cloned(),
        IDENTITY_ORDINAL,
    );
    let all_three = ObligationIdentity::new(other_node, None, 0);
    for (name, caller) in [
        ("ordinal only", ordinal_only.clone()),
        ("model only", model_only),
        ("node only", node_only),
        ("every member", all_three),
    ] {
        let error = refuse(caller, &["balance"], v2);
        assert!(
            matches!(error, StateClauseReplayError::ObligationIdentityMismatch),
            "{name}: {error}"
        );
        assert!(matches!(
            ReplaySettlement::from(&error),
            ReplaySettlement::CgDefect
        ));
    }

    // Identity first: a harness differing in identity and in field order is the identity refusal.
    let error = refuse(ordinal_only.clone(), &["balance", "audit"], v2);
    assert!(
        matches!(error, StateClauseReplayError::ObligationIdentityMismatch),
        "{error}"
    );
    // The same field-order difference with the matching identity is the field-order refusal.
    let error = refuse(identity.clone(), &["balance", "audit"], v2);
    assert!(
        matches!(
            error,
            StateClauseReplayError::HarnessMismatch {
                member: StateClauseHarnessMember::StateFields
            }
        ),
        "{error}"
    );

    // The harness-kind check comes first: a frame harness with a differing identity is
    // `NotInvariantHarness`.
    let frame_twin = Twin::new();
    let (frame_package, frame_clause) = frame_twin.emitted_package("BalanceNeverDrops");
    let frames = quire_contract_codegen::generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        package: &frame_package,
        clause: &frame_clause,
        state_path: "crate::subject::Account",
        state_fields: &["balance", "audit"],
        subject_path: "crate::subject::deposit_debiting_within_range",
        unwind: 4,
    })
    .expect("state-frame harnesses generate");
    let error = refuse(
        ordinal_only.clone(),
        &["balance"],
        StateClauseHarness::StateFrame(&frames.frame.identity),
    );
    assert!(
        matches!(error, StateClauseReplayError::NotInvariantHarness),
        "{error}"
    );
    // A V2 postcondition and a V2 precondition harness pass the `V2Clause` destructuring, so only
    // the V2 kind check separates them from the identity check: the identity check must follow it.
    for (other, name, native) in [
        (
            Twin::with_bounded_amount_implication(),
            "BalanceNeverDrops",
            true,
        ),
        (
            Twin::with_bounded_other_precondition(),
            "OtherBelowLimit",
            false,
        ),
    ] {
        let other_fixture = generate_with(&other, name, native);
        let error = refuse(
            ordinal_only.clone(),
            &["balance"],
            StateClauseHarness::V2Clause(&other_fixture.harness),
        );
        assert!(
            matches!(error, StateClauseReplayError::NotInvariantHarness),
            "{name}: {error}"
        );
    }
}

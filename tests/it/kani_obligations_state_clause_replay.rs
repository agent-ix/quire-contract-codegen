//! FR-024 (IR-460): a postcondition state-clause counterexample is put before QSL's
//! `replay_state_clause` as a request and a witness-arm envelope built by `StateClauseReplay`.
//!
//! The admitted package is emitted by QSL from the native twin of the state-frame fixture,
//! with two postcondition clauses on `deposit`. The invocation and snapshot documents
//! are built by the crate under test and read back from the request's byte provision, so each
//! assertion is about the bytes QSL receives.
//!
//! The default lane runs without Kani. The `kani` lane (`#[ignore]` here, run serially by
//! `make kani`, whose `kani_obligations` filter this module's name matches) replays the real
//! playback of the falsified operation-contract harness.

use std::cell::RefCell;

use qsl_replay::{
    replay_state_clause, CallSiteRefusal, ClauseName, ClauseSelectionInput, DisagreementCause,
    DomainKey, DomainKind, EvaluatedValue, FiniteBound, Identifier, Integer, ReplayRequestWire,
    ReplayResult, ReplaySource, ReplayTargetMembers, StateClauseReplayResult, TerminalValue,
    Verdict, WireNodeId, WitnessEnvelope, WitnessPacket, WitnessRefusal, WitnessSettlement,
};
use quire_contract_codegen::{
    parse_state_clause_falsification, run_terminal_value, DocumentLabel, KaniRunOutcome,
    OperationDeclaration, StateClauseHarnessMember, StateClauseModelFieldsCause,
    StateClausePlaybackError, StateClauseReplay, StateClauseReplayError, StateClauseReplayInputs,
    StateComparison, StateFrameIdentity, StateFrameProperty, StateObjectAddress,
};
use quire_semantic_value::catalog::Category;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use super::kani_obligations_state_frame::{
    emitted_fixture, falsified, model,
    native_twin::{playback_text, Twin},
    prove, subject, Declares, Fixture,
};
use ix_trace_rs::trace;

/// The clause whose counterexample the tests replay: `balance` never drops.
const BALANCE: &str = "BalanceNeverDrops";
/// The second postcondition clause on `deposit`: `audit` never drops.
const AUDIT: &str = "AuditNeverDrops";

trait DecodeStateClauseRun {
    fn decode_falsification_from_run(
        &mut self,
        harness: &StateFrameIdentity,
        outcome: &KaniRunOutcome,
        limits: quire_contract_codegen::WitnessDecodeLimits,
    ) -> Result<(), StateClauseReplayError>;
}

impl DecodeStateClauseRun for StateClauseReplayInputs<'_> {
    fn decode_falsification_from_run(
        &mut self,
        harness: &StateFrameIdentity,
        outcome: &KaniRunOutcome,
        limits: quire_contract_codegen::WitnessDecodeLimits,
    ) -> Result<(), StateClauseReplayError> {
        let playback = parse_state_clause_falsification(
            outcome,
            harness.harness_symbol.as_str(),
            harness.module_symbol.as_str(),
            limits,
            self.run.decode_limits,
        )
        .map_err(|error| match error {
            StateClausePlaybackError::Refused(refusal) => {
                StateClauseReplayError::Decode(refusal.into())
            }
            StateClausePlaybackError::NotFalsified => {
                panic!("the test helper requires a falsified run")
            }
        })?;
        self.decode_falsification(harness, &playback, limits)
    }
}

#[trace("TC-026", "FR-016-AC-6")]
#[test]
fn tc_026_state_clause_decode_refuses_poisoned_suffixes_at_both_bounds() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, BALANCE);
    let generated = quire_contract_codegen::generate_state_frame_obligations(
        &quire_contract_codegen::StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            package: &fixture.package,
            clause: &fixture.clause,
            state_path: "crate::subject::Account",
            state_fields: &["balance", "audit"],
            subject_path: "crate::subject::deposit_debiting_within_range",
            unwind: 4,
        },
    )
    .expect("postcondition harness generates");
    let harness = &generated.postcondition.identity;
    let check = "operation `deposit`: postcondition `post.balance >= pre.balance` failed";
    let kani_check = format!("\"{{}}\",\n{check:?}");
    let transcript = playback_text(harness, &kani_check, &[5, 7]);
    let mut inputs =
        twin.state_clause_inputs(&fixture.package, &fixture.clause, BALANCE, (0, 0), (4, 7));
    inputs.playback.clear();
    let limits = quire_contract_codegen::WitnessDecodeLimits {
        bytes: transcript.len(),
        values: 2,
    };
    inputs.run.decode_limits = limits;
    inputs
        .decode_falsification_from_run(
            harness,
            &KaniRunOutcome::Falsified {
                counterexample: transcript.clone(),
            },
            limits,
        )
        .expect("at both bounds");
    assert_eq!(inputs.playback.len(), 2);

    // A caller can hand the typed decoder a playback parsed under wider bounds. It must still
    // enforce both the requested limit and the run's own limit without relying on the parser.
    let parsed = quire_contract_codegen::parse_assertion_playback(
        &transcript,
        harness.harness_symbol.as_str(),
        harness.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("the output reader accepts the complete playback");
    for (field_limit, parameter_limit, code) in [
        (
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: transcript.len() - 1,
                values: 2,
            },
            crate::unbounded_decode_limits(),
            "cg_witness_decode_bytes_limit",
        ),
        (
            crate::unbounded_decode_limits(),
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: transcript.len() - 1,
                values: 2,
            },
            "cg_witness_decode_bytes_limit",
        ),
        (
            crate::unbounded_decode_limits(),
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: usize::MAX,
                values: 1,
            },
            "cg_witness_decode_values_limit",
        ),
        (
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: usize::MAX,
                values: 1,
            },
            crate::unbounded_decode_limits(),
            "cg_witness_decode_values_limit",
        ),
    ] {
        let mut inputs =
            twin.state_clause_inputs(&fixture.package, &fixture.clause, BALANCE, (0, 0), (4, 7));
        inputs.playback.clear();
        inputs.run.decode_limits = field_limit;
        let error = inputs
            .decode_falsification(harness, &parsed, parameter_limit)
            .expect_err("the typed decoder enforces its own effective limits");
        assert!(matches!(error, StateClauseReplayError::Decode(cause) if cause.code == code));
    }

    for (poisoned, limits, code) in [
        (
            format!("{transcript}{transcript}"),
            limits,
            "cg_witness_decode_bytes_limit",
        ),
        (
            transcript.replace("    ];", "        // 0\n        vec![not_a_byte],\n    ];"),
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: usize::MAX,
                values: 2,
            },
            "cg_witness_decode_values_limit",
        ),
    ] {
        let wider = crate::unbounded_decode_limits();
        for (field_limit, parameter_limit) in [(limits, wider), (wider, limits)] {
            let mut inputs = twin.state_clause_inputs(
                &fixture.package,
                &fixture.clause,
                BALANCE,
                (0, 0),
                (4, 7),
            );
            inputs.playback.clear();
            inputs.run.decode_limits = field_limit;
            let error = inputs
                .decode_falsification_from_run(
                    harness,
                    &KaniRunOutcome::Falsified {
                        counterexample: poisoned.clone(),
                    },
                    parameter_limit,
                )
                .expect_err("over-bound playback refuses");
            assert!(matches!(error, StateClauseReplayError::Decode(cause) if cause.code == code));
        }
        let mut inputs =
            twin.state_clause_inputs(&fixture.package, &fixture.clause, BALANCE, (0, 0), (4, 7));
        inputs.playback.clear();
        inputs.run.decode_limits = wider;
        let error = inputs
            .decode_falsification_from_run(
                harness,
                &KaniRunOutcome::Falsified {
                    counterexample: poisoned,
                },
                wider,
            )
            .expect_err("the poisoned suffix has a distinct decode error when read");
        let poisoned_code = if code == "cg_witness_decode_bytes_limit" {
            "kani_witness_multiple_assertions_refused"
        } else {
            "kani_witness_byte_invalid"
        };
        assert!(
            matches!(error, StateClauseReplayError::Decode(cause) if cause.code == poisoned_code)
        );
    }
}

/// The production adapter types the persisted draw order, accepts only the matching failed
/// assertion, and leaves model-domain admission to the existing replay builder.
#[trace(
    "FR-016-AC-8",
    "FR-024-AC-13",
    "FR-024-AC-15",
    "FR-024-AC-17",
    "TC-035"
)]
#[test]
fn state_clause_kani_playback_adapter_refuses_wrong_evidence() {
    let twin = Twin::new();
    let (package, clause) = twin.emitted_package(BALANCE);
    let generated = quire_contract_codegen::generate_state_frame_obligations(
        &quire_contract_codegen::StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            package: &package,
            clause: &clause,
            state_path: "crate::subject::Account",
            state_fields: &["balance", "audit"],
            subject_path: "crate::subject::deposit_debiting_within_range",
            unwind: 4,
        },
    )
    .expect("postcondition harness generates");
    let harness = &generated.postcondition.identity;
    let check = "operation `deposit`: postcondition `post.balance >= pre.balance` failed";
    // Kani prints the generated `assert!(predicate, "{}", message)` arguments inside
    // outer quotes, across two lines of the check clause.
    let kani_check = format!("\"{{}}\",\n{check:?}");
    let good = playback_text(harness, &kani_check, &[5, 7]);
    for spoof in [
        good.replace("post.balance >= pre.balance", "post.audit >= pre.audit"),
        good.replace("\"{}\",\n", "\"{}\", 9,\n"),
        good.replace("failed\"\"\n", "failed\"\" extra\n"),
        good.replace("\"{}\",\n", "\"{0}\",\n"),
        playback_text(harness, check, &[5, 7]),
        playback_text(harness, &format!("assertion failed: {check}"), &[5, 7]),
    ] {
        assert_ne!(spoof, good, "the spoof must change the captured form");
        let mut inputs = twin.state_clause_inputs(&package, &clause, BALANCE, (0, 0), (4, 7));
        inputs.playback.clear();
        assert!(matches!(
            inputs.decode_falsification_from_run(
                harness,
                &KaniRunOutcome::Falsified {
                    counterexample: spoof,
                },
                crate::unbounded_decode_limits()
            ),
            Err(StateClauseReplayError::AssertionMismatch)
        ));
    }
    let mut inputs = twin.state_clause_inputs(&package, &clause, BALANCE, (0, 0), (4, 7));
    inputs.playback.clear();
    inputs
        .decode_falsification_from_run(
            harness,
            &KaniRunOutcome::Falsified {
                counterexample: good.clone(),
            },
            crate::unbounded_decode_limits(),
        )
        .expect("matching playback decodes");
    assert_eq!(
        inputs.playback,
        vec![("balance".to_owned(), 5), ("audit".to_owned(), 7)]
    );
    let mut selected_other_clause = inputs.clone();
    selected_other_clause.clause = ClauseName(Identifier::new(AUDIT).expect("clause name"));
    assert!(matches!(
        StateClauseReplay::new(selected_other_clause),
        Err(StateClauseReplayError::ClauseSelectionMismatch)
    ));
    StateClauseReplay::new(inputs).expect("decoded state reaches replay builder");

    let empty = || {
        let mut inputs = twin.state_clause_inputs(&package, &clause, BALANCE, (0, 0), (4, 7));
        inputs.playback.clear();
        inputs
    };
    let wrong = good.replace(&harness.harness_path().to_string(), "other::check");
    let cover = good.replace("`assertion`", "`cover`");
    let width = good.replace("vec![5, 0, 0, 0, 0, 0, 0, 0]", "vec![5, 0]");
    let comment = good.replace("// 5", "// 9");
    for (text, code) in [
        (wrong, "cg_witness_harness_identity_mismatch"),
        (cover, "kani_witness_cover_refused"),
        (
            playback_text(harness, &kani_check, &[5]),
            "kani_witness_arity_mismatch",
        ),
        (
            playback_text(harness, &kani_check, &[5, 7, 9]),
            "kani_witness_arity_mismatch",
        ),
        (width, "kani_witness_width_mismatch"),
        (comment, "kani_witness_comment_mismatch"),
    ] {
        let error = empty()
            .decode_falsification_from_run(
                harness,
                &KaniRunOutcome::Falsified {
                    counterexample: text,
                },
                crate::unbounded_decode_limits(),
            )
            .expect_err("invalid playback refuses");
        assert!(matches!(error, StateClauseReplayError::Decode(cause) if cause.code == code));
    }
    assert!(matches!(
        parse_state_clause_falsification(
            &KaniRunOutcome::Verified,
            harness.harness_symbol.as_str(),
            harness.module_symbol.as_str(),
            crate::unbounded_decode_limits(),
            crate::unbounded_decode_limits(),
        ),
        Err(StateClausePlaybackError::NotFalsified)
    ));
    assert!(matches!(
        empty().decode_falsification_from_run(
            &generated.frame.identity,
            &KaniRunOutcome::Falsified {
                counterexample: good.clone(),
            },
            crate::unbounded_decode_limits()
        ),
        Err(StateClauseReplayError::NotPostconditionHarness)
    ));
    for (field, comparison, left_is_pre, check) in [
        (
            "audit",
            StateComparison::Ge,
            false,
            "operation `deposit`: postcondition `post.audit >= pre.audit` failed",
        ),
        (
            "balance",
            StateComparison::Le,
            false,
            "operation `deposit`: postcondition `post.balance <= pre.balance` failed",
        ),
        (
            "balance",
            StateComparison::Ge,
            true,
            "operation `deposit`: postcondition `pre.balance >= post.balance` failed",
        ),
    ] {
        let mut tampered = harness.clone();
        tampered.property = StateFrameProperty::Postcondition {
            field: field.to_owned(),
            comparison,
            left_is_pre,
        };
        assert!(matches!(
            empty().decode_falsification_from_run(
                &tampered,
                &KaniRunOutcome::Falsified {
                    counterexample: playback_text(
                        &tampered,
                        &format!("\"{{}}\",\n{check:?}"),
                        &[5, 7]
                    ),
                },
                crate::unbounded_decode_limits()
            ),
            Err(StateClauseReplayError::HarnessMismatch {
                member: StateClauseHarnessMember::Property
            })
        ));
    }
    let mut wrong_scope = harness.clone();
    wrong_scope.scope.operation = "withdraw".to_owned();
    assert!(matches!(
        empty().decode_falsification_from_run(
            &wrong_scope,
            &KaniRunOutcome::Falsified {
                counterexample: good.clone(),
            },
            crate::unbounded_decode_limits()
        ),
        Err(StateClauseReplayError::HarnessMismatch {
            member: StateClauseHarnessMember::Operation
        })
    ));
    assert!(matches!(
        empty().decode_falsification_from_run(
            harness,
            &KaniRunOutcome::Falsified {
                counterexample: playback_text(harness, "unrelated assertion", &[5, 7]),
            },
            crate::unbounded_decode_limits()
        ),
        Err(StateClauseReplayError::AssertionMismatch)
    ));
    let mut repeated = harness.clone();
    repeated.state_fields = vec!["balance".to_owned(), "balance".to_owned()];
    let mut repeated_inputs = empty();
    repeated_inputs.state_fields = repeated.state_fields.clone();
    assert!(matches!(
        repeated_inputs.decode_falsification_from_run(&repeated, &KaniRunOutcome::Falsified {
            counterexample: playback_text(&repeated, &kani_check, &[5, 7]),
        }, crate::unbounded_decode_limits()),
        Err(StateClauseReplayError::Decode(cause))
            if cause.code == "cg_witness_schema_duplicate_binding"
    ));
    let mut outside = empty();
    outside
        .decode_falsification_from_run(
            harness,
            &KaniRunOutcome::Falsified {
                counterexample: playback_text(harness, &kani_check, &[-1, 7]),
            },
            crate::unbounded_decode_limits(),
        )
        .expect("i64 decodes before domain admission");
    assert!(matches!(
        StateClauseReplay::new(outside),
        Err(StateClauseReplayError::OutOfDomain { field }) if field == "balance"
    ));
}

/// QSL's emitted checked package has an empty model body, yet replay takes the caller's field
/// order and both inclusive ranges from the admitted model declaration's accessor.
#[trace("FR-024-AC-31", "FR-024-AC-33")]
#[test]
fn tc_035_emitted_model_fields_bind_replay_before_playback() {
    let twin = Twin::new();
    let (package, clause) = twin.emitted_package(BALANCE);
    let input = twin.state_clause_inputs(&package, &clause, BALANCE, (5, 7), (4, 7));
    let replay = StateClauseReplay::new(input.clone()).expect("model fields resolve");
    let domains = replay.packet.declared_domains.as_ref().expect("domains");
    assert_eq!(domains.len(), model::FIELDS.len());
    for (position, (domain, (_, (lower, upper)))) in domains.iter().zip(model::FIELDS).enumerate() {
        assert_eq!(domain.proof_bound().kind(), Some(DomainKind::Integer));
        assert!(matches!(
            domain.domain(),
            DomainKey::Node { path, .. } if path == &[u32::try_from(position).expect("small")]
        ));
        assert_eq!(
            domain.bound(),
            &FiniteBound::integer_range(Integer::from(lower), Integer::from(upper)).expect("range")
        );
    }
    let result = replay.replay().expect("the debit replays");
    assert_eq!(
        witness_arm(&result).settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    let respecting = StateClauseReplay::new(twin.state_clause_inputs(
        &package,
        &clause,
        BALANCE,
        (5, 7),
        (6, 7),
    ))
    .expect("the same emitted package builds a respecting replay");
    let result = respecting.replay().expect("the respecting run replays");
    assert_eq!(
        witness_arm(&result).settlement(),
        WitnessSettlement::Inconclusive
    );
    assert!(matches!(
        witness_arm(&result).disagreement(),
        Some(DisagreementCause::Verdicts { .. })
    ));

    let mut absent = input.clone();
    absent.state_fields.push("ghost".to_owned());
    absent.playback.clear();
    let error = StateClauseReplay::new(absent)
        .err()
        .expect("model absence precedes binding");
    assert!(matches!(
        error,
        StateClauseReplayError::ModelFields {
            cause: StateClauseModelFieldsCause::Absent { field }, ..
        } if field == "ghost"
    ));

    let mut outside = input;
    outside.playback[1].1 = 1001;
    assert!(matches!(
        StateClauseReplay::new(outside),
        Err(StateClauseReplayError::OutOfDomain { field }) if field == "audit"
    ));
}

fn healthy() -> Fixture {
    emitted_fixture(&Twin::new(), BALANCE)
}

fn built(
    twin: &Twin,
    fixture: &Fixture,
    clause: &str,
    pre: (i64, i64),
    post: (i64, i64),
) -> StateClauseReplay {
    StateClauseReplay::new(twin.state_clause_inputs(
        &fixture.package,
        &fixture.clause,
        clause,
        pre,
        post,
    ))
    .expect("the replay is built")
}

fn inputs<'a>(
    twin: &Twin,
    fixture: &'a Fixture,
    pre: (i64, i64),
    post: (i64, i64),
) -> StateClauseReplayInputs<'a> {
    twin.state_clause_inputs(&fixture.package, &fixture.clause, BALANCE, pre, post)
}

/// What `qsl_replay::replay_state_clause` returns for the request and envelope `replay` holds,
/// called directly.
fn direct(replay: StateClauseReplay) -> StateClauseReplayResult {
    let replay_limits = replay.replay_limits();
    let envelope = WitnessEnvelope::reconstruct(replay.packet, replay_limits)
        .expect("a complete packet reconstructs");
    replay_state_clause(replay.wire, &envelope, replay_limits).expect("QSL settles the replay")
}

/// The violating run: the debiting subject takes `balance` from 5 to 4.
const VIOLATING: ((i64, i64), (i64, i64)) = ((5, 0), (4, 0));
/// The respecting run over the same pre state: the crediting subject takes `balance` to 6.
const RESPECTING: ((i64, i64), (i64, i64)) = ((5, 0), (6, 0));

fn witness_arm(result: &StateClauseReplayResult) -> &qsl_replay::WitnessArmResult {
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    arm
}

/// `StateClauseReplay::replay` returns what `replay_state_clause` returns for the same request
/// and envelope, for a violating and a respecting run whose results differ, and `replay_through`
/// hands its executor the request and envelope it built and returns the executor's value as it is.
#[trace("FR-024-AC-11", "TC-035")]
#[test]
fn tc_035_replay_returns_the_direct_result_and_replay_through_returns_the_executors_value() {
    let (twin, fixture) = (Twin::new(), healthy());
    let mut results = Vec::new();
    for (pre, post) in [VIOLATING, RESPECTING] {
        let through = built(&twin, &fixture, BALANCE, pre, post)
            .replay()
            .expect("the replay settles");
        assert_eq!(
            through,
            direct(built(&twin, &fixture, BALANCE, pre, post)),
            "replay is the direct call"
        );
        results.push(through);
    }
    let [violating, respecting] = &results[..] else {
        panic!("two results");
    };
    assert_ne!(violating, respecting, "the two runs settle differently");

    // The executor sees the request and envelope the replay built, and its value comes back.
    let seen = RefCell::new(None);
    let replay = built(&twin, &fixture, BALANCE, VIOLATING.0, VIOLATING.1);
    let expected_package = replay.wire.package_id.clone();
    let sentinel = respecting.clone();
    let returned = replay
        .replay_through(|wire: ReplayRequestWire, envelope| {
            *seen.borrow_mut() = Some((
                wire.package_id.clone(),
                envelope
                    .replay_target()
                    .as_state_clause()
                    .expect("state-clause target")
                    .to_string(),
                envelope.clause_node(),
            ));
            Ok(sentinel)
        })
        .expect("the executor's value is returned");
    assert_eq!(&returned, respecting, "the sentinel replaces the verdict");
    assert_ne!(&returned, violating);
    let Some((package_id, clause, node)) = seen.take() else {
        panic!("the executor was called");
    };
    assert_eq!(package_id, expected_package);
    assert_eq!(clause, BALANCE);
    assert_eq!(
        node,
        twin.clause_site(BALANCE)
            .expect("the clause is declared")
            .node
    );
}

/// Every missing packet member is reported by QSL admission before the executor runs. A present
/// trace position whose value is none still reaches the executor.
#[trace("TC-035", "FR-024-AC-4")]
#[test]
fn tc_035_missing_packet_members_stop_before_replay() {
    let (twin, fixture) = (Twin::new(), healthy());
    macro_rules! missing_member {
        ($member:ident) => {{
            let mut replay = built(&twin, &fixture, BALANCE, VIOLATING.0, VIOLATING.1);
            replay.packet.$member = None;
            let mut calls = 0;
            let result = replay.replay_through(|_, _| {
                calls += 1;
                panic!("a missing packet member reached the executor")
            });
            assert!(matches!(
                result,
                Err(StateClauseReplayError::Envelope(WitnessRefusal::MissingMember(name)))
                    if name == stringify!($member)
            ));
            assert_eq!(calls, 0, "{} reached replay", stringify!($member));
        }};
    }
    missing_member!(obligation_identity);
    missing_member!(occurrence_key);
    missing_member!(clause_node);
    {
        let mut replay = built(&twin, &fixture, BALANCE, VIOLATING.0, VIOLATING.1);
        replay.packet.replay_target = ReplayTargetMembers::default();
        let mut calls = 0;
        let result = replay.replay_through(|_, _| {
            calls += 1;
            panic!("a missing replay target reached the executor")
        });
        assert!(matches!(
            result,
            Err(StateClauseReplayError::Envelope(
                WitnessRefusal::MissingMember("replay_target")
            ))
        ));
        assert_eq!(calls, 0);
    }
    missing_member!(package_id);
    missing_member!(source_digests);
    missing_member!(profile_selections);
    missing_member!(run_limits);
    missing_member!(declared_domains);
    missing_member!(backend);
    missing_member!(trace_position);
    missing_member!(source);
    missing_member!(family_payload);

    let replay = built(&twin, &fixture, BALANCE, VIOLATING.0, VIOLATING.1);
    let WitnessPacket {
        obligation_identity: _,
        occurrence_key: _,
        clause_node: _,
        replay_target: _,
        package_id: _,
        source_digests: _,
        profile_selections: _,
        run_limits: _,
        declared_domains: _,
        backend: _,
        trace_position: _,
        source: _,
        family_payload: _,
    } = &replay.packet;
    assert!(matches!(replay.packet.trace_position, Some(None)));
    let limits = replay.replay_limits();
    let mut calls = 0;
    replay
        .replay_through(|wire, envelope| {
            calls += 1;
            replay_state_clause(wire, envelope, limits)
        })
        .expect("present trace position with no value is admitted");
    assert_eq!(calls, 1);
}

/// Two selected target families refuse before the replay executor is invoked.
#[trace("FR-024-AC-42", "TC-035")]
#[test]
fn tc_035_conflicting_replay_targets_stop_before_replay() {
    let (twin, fixture) = (Twin::new(), healthy());
    let mut replay = built(&twin, &fixture, BALANCE, VIOLATING.0, VIOLATING.1);
    replay.packet.replay_target.function = replay.packet.replay_target.state_clause.clone();
    let mut calls = 0;
    let result = replay.replay_through(|_, _| {
        calls += 1;
        panic!("a conflicting target reached the executor")
    });
    assert!(matches!(
        result,
        Err(StateClauseReplayError::Envelope(
            WitnessRefusal::ConflictingTarget { .. }
        ))
    ));
    assert_eq!(calls, 0);
}

/// The replay target names its clause and the payload holds the `Invocation` arm; the envelope's identities are the
/// `ClauseSite`'s that `call_site` names, two clauses carry their own, and an undeclared clause is
/// refused by the call site when the request is built.
#[trace("FR-024-AC-12", "FR-024-AC-41", "TC-035")]
#[test]
fn tc_035_the_envelope_carries_the_call_sites_clause_identities() {
    let twin = Twin::new();
    let mut nodes = Vec::new();
    for clause in [BALANCE, AUDIT] {
        let fixture = emitted_fixture(&twin, clause);
        let replay = built(&twin, &fixture, clause, RESPECTING.0, RESPECTING.1);
        let site = twin.clause_site(clause).expect("the clause is declared");
        let payload = replay.packet.family_payload.as_ref().expect("a payload");
        assert_eq!(
            replay
                .packet
                .replay_target
                .state_clause
                .as_ref()
                .expect("state-clause target")
                .to_string(),
            clause
        );
        assert_eq!(site.name.as_str(), clause);
        assert!(replay.packet.replay_target.function.is_none());
        assert!(replay.packet.replay_target.frame_operation.is_none());
        assert_eq!(
            replay.wire.replay_target.members,
            replay.packet.replay_target
        );
        assert!(!replay.wire.replay_target.arguments);
        assert!(replay.wire.replay_target.occurrence_key.is_none());
        assert!(matches!(
            payload.observation,
            ClauseSelectionInput::Invocation { .. }
        ));
        assert!(payload.witness.is_none());
        assert_eq!(replay.packet.clause_node, Some(site.node));
        assert_eq!(
            replay.packet.occurrence_key.as_ref(),
            Some(&site.occurrence)
        );
        nodes.push(site.node);
    }
    assert_ne!(nodes[0], nodes[1], "each clause has its own node");

    let fixture = healthy();
    let mut missing = inputs(&twin, &fixture, RESPECTING.0, RESPECTING.1);
    missing.clause = ClauseName(Identifier::new("NoSuchClause").expect("identifier"));
    let refusal = StateClauseReplay::new(missing)
        .err()
        .expect("an undeclared clause is refused");
    assert!(
        matches!(
            &refusal,
            StateClauseReplayError::CallSite(cause)
                if matches!(&**cause, CallSiteRefusal::UnknownClause { selection, .. }
                    if selection.0.as_str() == "NoSuchClause")
        ),
        "{refusal}"
    );
}

/// The documents of a request, by what each is.
struct Documents {
    invocation: (String, Value),
    pre: (String, Value),
    post: (String, Value),
}

/// Reads the invocation and the two snapshots from `wire`'s byte provision, each with the digest
/// the request addresses it by, after checking that digest is SHA-256 of the bytes provided.
fn documents(wire: &ReplayRequestWire) -> Documents {
    let mut found = (None, None, None);
    for (domain, hex, bytes) in &wire.byte_provision {
        if domain.as_deref() != Some("sha256-jcs") {
            continue;
        }
        let Ok(document) = serde_json::from_slice::<Value>(bytes) else {
            continue;
        };
        let entry = (hex.clone(), document.clone());
        match document.get("format").and_then(Value::as_str) {
            Some("quire.state.invocation/v1") => found.0 = Some(entry),
            Some("quire.state.snapshot/v1") => match document.get("observation") {
                Some(Value::String(role)) if role == "pre" => found.1 = Some(entry),
                Some(Value::String(role)) if role == "post" => found.2 = Some(entry),
                other => panic!("an unknown snapshot role: {other:?}"),
            },
            _ => continue,
        }
        let digest = Sha256::digest(bytes);
        let actual = digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        assert_eq!(
            &actual, hex,
            "the request addresses a document by its bytes' digest"
        );
    }
    let (Some(invocation), Some(pre), Some(post)) = found else {
        panic!("the request provides the invocation and both snapshots");
    };
    Documents {
        invocation,
        pre,
        post,
    }
}

fn integers(snapshot: &Value) -> Value {
    snapshot
        .pointer("/populations/0/objects/0/fields")
        .cloned()
        .expect("the snapshot holds one object's fields")
}

/// A label as a document's identity member spells it.
fn identity_member(label: &DocumentLabel) -> Value {
    json!({
        "authority": label.authority,
        "identity": label.identity,
        "revision_namespace": label.revision_namespace,
        "revision": label.revision,
    })
}

/// Checks the three documents against the address, the labels (invocation, pre, post) and the
/// `(balance, audit)` decimal strings of the pre and post snapshots.
fn assert_documents(
    documents: &Documents,
    address: &StateObjectAddress,
    labels: &[DocumentLabel; 3],
    [(pre_balance, pre_audit), (post_balance, post_audit)]: [(&str, &str); 2],
) {
    let Documents {
        invocation,
        pre,
        post,
    } = documents;
    let [invocation_label, pre_label, post_label] = labels;
    for (document, observation, label, balance, audit) in [
        (&pre.1, "pre", pre_label, pre_balance, pre_audit),
        (&post.1, "post", post_label, post_balance, post_audit),
    ] {
        assert_eq!(document["observation"], observation);
        assert_eq!(document["identity"], identity_member(label));
        assert_eq!(document["model"]["identity"], "test/bank");
        let populations = document["populations"].as_array().expect("populations");
        assert_eq!(populations.len(), 1);
        assert_eq!(populations[0]["population"], address.population.as_str());
        assert_eq!(populations[0]["complete"], true);
        let objects = populations[0]["objects"].as_array().expect("objects");
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0]["key"], address.key.as_str());
        assert_eq!(objects[0]["type"], address.object_type.as_str());
        assert_eq!(
            integers(document),
            json!({"balance": {"integer": balance}, "audit": {"integer": audit}})
        );
    }

    assert_eq!(invocation.1["identity"], identity_member(invocation_label));
    assert_eq!(invocation.1["model"], pre.1["model"]);
    assert_eq!(invocation.1["context"], address.object_type.as_str());
    assert_eq!(invocation.1["operation"], "deposit");
    assert_eq!(
        invocation.1["self"],
        json!({"population": address.population, "key": address.key})
    );
    assert_eq!(invocation.1["parameters"], json!({}));
    assert_eq!(invocation.1["result"], Value::Null);
    assert_eq!(invocation.1["created"], json!([]));
    assert_eq!(invocation.1["deleted"], json!([]));
    for (link, snapshot, label) in [("pre", pre, pre_label), ("post", post, post_label)] {
        assert_eq!(invocation.1[link]["identity"], identity_member(label));
        assert_eq!(
            invocation.1[link]["digest"],
            format!("sha256-jcs:{}", snapshot.0)
        );
    }
}

/// The snapshots hold the playback's and the supplied values, the invocation holds the stated
/// members with both digests, and a changed value changes only its own snapshot.
#[trace("FR-024-AC-13", "TC-035")]
#[test]
fn tc_035_the_documents_hold_the_playback_the_post_state_and_their_digests() {
    let (twin, fixture) = (Twin::new(), healthy());
    let default = inputs(&twin, &fixture, (5, 7), (4, 8));
    let (address, labels) = (
        default.object.clone(),
        [
            default.invocation_label.clone(),
            default.pre_label.clone(),
            default.post_label.clone(),
        ],
    );
    let wire = StateClauseReplay::new(default)
        .expect("the replay is built")
        .wire;
    let Documents {
        invocation,
        pre,
        post,
    } = documents(&wire);
    assert_documents(
        &Documents {
            invocation: invocation.clone(),
            pre: pre.clone(),
            post: post.clone(),
        },
        &address,
        &labels,
        [("5", "7"), ("4", "8")],
    );

    // The address and every member of the three labels are carried as given, each its own value:
    // a document that wrote a literal, or swapped two label members, would not match.
    let label = |tag: &str| DocumentLabel {
        authority: format!("authority-{tag}"),
        identity: format!("identity-{tag}"),
        revision_namespace: format!("namespace-{tag}"),
        revision: format!("revision-{tag}"),
    };
    let other_address = StateObjectAddress {
        population: "ix://test/bank/other-accounts".to_owned(),
        key: "account-two".to_owned(),
        object_type: "ix://test/bank/Savings".to_owned(),
    };
    let other_labels = [label("invocation"), label("pre"), label("post")];
    let mut varied = inputs(&twin, &fixture, (1, 2), (3, 4));
    varied.object = other_address.clone();
    varied.invocation_label = other_labels[0].clone();
    varied.pre_label = other_labels[1].clone();
    varied.post_label = other_labels[2].clone();
    let varied = StateClauseReplay::new(varied)
        .expect("the replay is built")
        .wire;
    assert_documents(
        &documents(&varied),
        &other_address,
        &other_labels,
        [("1", "2"), ("3", "4")],
    );

    // One changed playback value changes the pre snapshot and nothing of the post snapshot.
    let changed_pre = documents(&built(&twin, &fixture, BALANCE, (6, 7), (4, 8)).wire);
    assert_ne!(changed_pre.pre.0, pre.0);
    assert_ne!(changed_pre.pre.1, pre.1);
    assert_eq!(changed_pre.post.0, post.0);
    // One changed post-state value changes the post snapshot and nothing of the pre snapshot.
    let changed_post = documents(&built(&twin, &fixture, BALANCE, (5, 7), (4, 9)).wire);
    assert_ne!(changed_post.post.0, post.0);
    assert_ne!(changed_post.post.1, post.1);
    assert_eq!(changed_post.pre.0, pre.0);
}

/// The debit at the declared floor leaves an exact out-of-range post-state observation. QSL
/// replays it as a violated witness and the ordinary falsified terminal is refuted.
#[trace("FR-029-AC-18", "TC-040")]
#[test]
fn tc_040_out_of_range_debit_post_state_is_exact_and_refuted() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&twin, BALANCE);
    let replay = built(&twin, &fixture, BALANCE, (0, 0), (-1, 0));
    let docs = documents(&replay.wire);
    assert_eq!(integers(&docs.pre.1)["balance"]["integer"], "0");
    assert_eq!(integers(&docs.post.1)["balance"]["integer"], "-1");
    let result = replay
        .replay()
        .expect("QSL replays the observed post state");
    let arm = witness_arm(&result);
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
    let outcome = KaniRunOutcome::Falsified {
        counterexample: String::new(),
    };
    assert_eq!(
        run_terminal_value(
            &crate::terminal_map::synthetic_evidence(outcome.clone(), 0),
            Some((&result).into())
        )
        .expect("falsified run maps"),
        TerminalValue::Refuted
    );
}

/// A state field with no playback value, or no post-state value, is refused by name and no
/// default is supplied.
#[trace("FR-024-AC-15", "TC-035")]
#[test]
fn tc_035_a_missing_field_is_refused_by_name() {
    let (twin, fixture) = (Twin::new(), healthy());
    for field in ["balance", "audit"] {
        let mut no_playback = inputs(&twin, &fixture, RESPECTING.0, RESPECTING.1);
        no_playback.playback.retain(|(name, _)| name != field);
        let mut no_post = inputs(&twin, &fixture, RESPECTING.0, RESPECTING.1);
        no_post.post_state.retain(|(name, _)| name != field);
        for missing in [no_playback, no_post] {
            let error = StateClauseReplay::new(missing)
                .err()
                .expect("a missing field is refused");
            assert!(
                matches!(&error, StateClauseReplayError::MissingField { field: named } if named == field),
                "{field}: {error}"
            );
        }
    }
}

/// A name the framed object does not declare, and a field bound twice, are refused by name on
/// the playback and on the post state alike: no binding is dropped or taken first-wins.
#[trace("FR-024-AC-15", "TC-035")]
#[test]
fn tc_035_an_undeclared_or_repeated_binding_is_refused_by_name() {
    let (twin, fixture) = (Twin::new(), healthy());
    for on_post_state in [false, true] {
        let side = |candidate: &mut StateClauseReplayInputs<'_>, name: &str, value: i64| {
            let values = if on_post_state {
                &mut candidate.post_state
            } else {
                &mut candidate.playback
            };
            values.push((name.to_owned(), value));
        };
        let mut undeclared = inputs(&twin, &fixture, RESPECTING.0, RESPECTING.1);
        side(&mut undeclared, "ghost", 1);
        let error = StateClauseReplay::new(undeclared)
            .err()
            .expect("an undeclared name is refused");
        assert!(
            matches!(&error, StateClauseReplayError::UndeclaredField { field } if field == "ghost"),
            "{error}"
        );

        let mut repeated = inputs(&twin, &fixture, RESPECTING.0, RESPECTING.1);
        side(&mut repeated, "audit", 9);
        let error = StateClauseReplay::new(repeated)
            .err()
            .expect("a repeated field is refused");
        assert!(
            matches!(&error, StateClauseReplayError::DuplicateField { field } if field == "audit"),
            "{error}"
        );
    }
}

/// The envelope's `declared_domains` are one per ranged state field, on `self`'s node at the
/// field's child index with the declared range, and the witness transcript names the operation,
/// the clause and each playback value. QSL's replay reads neither, so only these assertions
/// pin them. A field whose member declares no range is carried and is not range-checked or
/// given a domain.
#[trace("FR-024-AC-12", "TC-035")]
#[test]
fn tc_035_the_envelope_declares_each_ranged_field_and_the_transcript_names_the_playback() {
    let (twin, fixture) = (Twin::new(), healthy());
    let candidate = inputs(&twin, &fixture, (5, 7), (4, 8));
    let operation = candidate.operation.to_string();
    let replay = StateClauseReplay::new(candidate).expect("the replay is built");

    let Some(qsl_replay::EnvelopeSource::Witness(witness)) = &replay.packet.source else {
        panic!("the envelope is on the witness arm");
    };
    assert_eq!(
        witness.transcript(),
        format!("<<<assertion|{operation}|{BALANCE}|balance=5;audit=7>>>")
    );

    let parameters = fixture
        .package
        .graph()
        .nodes
        .iter()
        .filter(|node| node.semantic_form.as_ref() == "parameter")
        .collect::<Vec<_>>();
    let [parameter] = parameters.as_slice() else {
        panic!("the emitted clause has one self parameter");
    };
    let parameter = WireNodeId::from_hex(&parameter.node_id.digest).expect("a node id");
    let domains = replay.packet.declared_domains.as_ref().expect("domains");
    assert_eq!(domains.len(), model::FIELDS.len());
    for (position, (domain, (_, (minimum, maximum)))) in
        domains.iter().zip(model::FIELDS).enumerate()
    {
        assert_eq!(domain.proof_bound().kind(), Some(DomainKind::Integer));
        assert_eq!(
            domain.domain(),
            &DomainKey::Node {
                node: parameter,
                path: vec![u32::try_from(position).expect("small")],
            }
        );
        assert_eq!(
            domain.bound(),
            &FiniteBound::integer_range(Integer::from(minimum), Integer::from(maximum))
                .expect("a range")
        );
    }

    // `balance` declares no range: any playback value is carried, and only `audit` has a domain.
    let unbounded_twin = Twin::without_balance_range();
    let unbounded = emitted_fixture(&unbounded_twin, BALANCE);
    let mut candidate = inputs(&unbounded_twin, &unbounded, (5, 7), (4, 8));
    candidate.playback[0].1 = 1_000_000;
    let replay = StateClauseReplay::new(candidate).expect("an unranged field is not checked");
    let domains = replay.packet.declared_domains.as_ref().expect("domains");
    assert_eq!(domains.len(), 1);
    assert!(matches!(
        domains[0].domain(),
        DomainKey::Node { path, .. } if path == &[1]
    ));
}

/// The mutated subject's counterexample settles `reproduced-with-evaluated-witness` in category
/// `violation` with an evaluated `false`; the unmutated subject's run over the same pre state
/// settles `inconclusive` with cause `Verdicts`; both envelopes are on the `Witness` arm with a
/// payload `witness` of none.
#[trace("FR-024-AC-16", "TC-035")]
#[test]
fn tc_035_a_violating_run_reproduces_and_a_respecting_run_is_inconclusive() {
    let (twin, fixture) = (Twin::new(), healthy());
    for (pre, post) in [VIOLATING, RESPECTING] {
        let replay = built(&twin, &fixture, BALANCE, pre, post);
        assert!(matches!(
            replay.packet.source,
            Some(qsl_replay::EnvelopeSource::Witness(_))
        ));
        let payload = replay.packet.family_payload.as_ref().expect("a payload");
        assert!(payload.witness.is_none());
    }

    let violating = built(&twin, &fixture, BALANCE, VIOLATING.0, VIOLATING.1)
        .replay()
        .expect("the replay settles");
    let arm = witness_arm(&violating);
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
    assert_eq!(arm.value(), Some(EvaluatedValue::Boolean(false)));

    let respecting = built(&twin, &fixture, BALANCE, RESPECTING.0, RESPECTING.1)
        .replay()
        .expect("the replay settles");
    let arm = witness_arm(&respecting);
    assert_eq!(arm.settlement(), WitnessSettlement::Inconclusive);
    assert_eq!(
        arm.disagreement(),
        Some(&DisagreementCause::Verdicts {
            proved: Verdict::from_category(Category::Violation),
            replayed: Verdict::from_category(Category::Success),
        })
    );
}

/// A playback value outside its field's declared range is refused by name, and the range's two
/// endpoints are admitted. The out-of-range value is put in the playback alone, and separately in
/// the post state alone, so the test says which side the range is read from: the playback. The
/// post state is whatever the subject ran to; how a post state outside the range should settle
/// is a pending QSL ruling (see the replay spec's Current state), and this test pins no answer to
/// it.
#[trace("FR-024-AC-17", "TC-035")]
#[test]
fn tc_035_a_value_outside_its_declared_range_is_out_of_domain() {
    let (twin, fixture) = (Twin::new(), healthy());
    for (field, (minimum, maximum)) in model::FIELDS {
        // Only the playback holds `value` for `field`; the post state keeps the field as it was.
        let in_playback = |value: i64| {
            let mut candidate = inputs(&twin, &fixture, (5, 5), (5, 5));
            if let Some(entry) = candidate
                .playback
                .iter_mut()
                .find(|(name, _)| name == field)
            {
                entry.1 = value;
            }
            candidate
        };
        // Only the post state holds `value` for `field`; the playback is in range.
        let in_post_state = |value: i64| {
            let mut candidate = inputs(&twin, &fixture, (5, 5), (5, 5));
            if let Some(entry) = candidate
                .post_state
                .iter_mut()
                .find(|(name, _)| name == field)
            {
                entry.1 = value;
            }
            candidate
        };
        for admitted in [minimum, maximum] {
            // A replayed run changes only what the frame grants, so the post state holds the
            // same value.
            let mut candidate = in_playback(admitted);
            if let Some(entry) = candidate
                .post_state
                .iter_mut()
                .find(|(name, _)| name == field)
            {
                entry.1 = admitted;
            }
            let replay = StateClauseReplay::new(candidate)
                .unwrap_or_else(|error| panic!("{field}={admitted} is admitted: {error}"));
            replay.replay().expect("an admitted value replays");
        }
        for refused in [minimum - 1, maximum + 1] {
            let error = StateClauseReplay::new(in_playback(refused))
                .err()
                .unwrap_or_else(|| panic!("{field}={refused} is outside the range"));
            assert!(
                matches!(&error, StateClauseReplayError::OutOfDomain { field: named } if named == field),
                "{field}={refused}: {error}"
            );
            // The same value on the post-state side is not the playback's out-of-domain error.
            StateClauseReplay::new(in_post_state(refused))
                .unwrap_or_else(|error| panic!("{field}={refused} in the post state: {error}"));
        }
    }
}

/// An operation that declares a parameter, and one that declares a result, are refused by shape
/// before anything else is read; one that declares neither is not.
#[trace("FR-024-AC-19", "TC-035")]
#[test]
fn tc_035_an_operation_declaring_a_parameter_or_a_result_is_refused_by_shape() {
    let twin = Twin::new();
    let operation = twin
        .state_clause_inputs(
            &healthy().package,
            &healthy().clause,
            BALANCE,
            (5, 0),
            (6, 0),
        )
        .operation;
    for (declares, declaration) in [
        (
            Declares::Parameter,
            OperationDeclaration {
                parameters: vec!["other".to_owned()],
                result: false,
            },
        ),
        (
            Declares::Result,
            OperationDeclaration {
                parameters: Vec::new(),
                result: true,
            },
        ),
    ] {
        let twin = match declares {
            Declares::Parameter => Twin::with_deposit_parameter(),
            Declares::Result => Twin::with_deposit_result(),
        };
        let fixture = emitted_fixture(&twin, BALANCE);
        // A playback with a field missing would be a different refusal: the shape is read first.
        let mut candidate = inputs(&twin, &fixture, (5, 0), (6, 0));
        candidate.playback.clear();
        let error = StateClauseReplay::new(candidate)
            .err()
            .expect("the shape is refused");
        assert!(
            matches!(
                &error,
                StateClauseReplayError::UnsupportedOperationShape { operation: named, declaration: found }
                    if *named == operation && *found == declaration
            ),
            "{error}"
        );
    }
    let supported = healthy();
    assert!(StateClauseReplay::new(inputs(&twin, &supported, (5, 0), (6, 0))).is_ok());
}

// ---- kani lane ---------------------------------------------------------------

/// The falsified operation-contract harness of a postcondition state clause over a subject
/// mutated to debit is replayed from its real Kani playback through `replay_state_clause`, which
/// reproduces the violation. The subject is the debit that stops at the floor of `balance`'s range
/// (`deposit_debiting_within_range`); the wrapping debit's out-of-range post state is covered by
/// FR-029-AC-18 below.
#[trace("FR-024-AC-18", "FR-024-AC-34", "TC-035")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_035_real_kani_state_clause_counterexample_replays_through_qsl() {
    let twin = Twin::new();
    let (package, clause) = twin.emitted_package(BALANCE);
    let generated = quire_contract_codegen::generate_state_frame_obligations(
        &quire_contract_codegen::StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            package: &package,
            clause: &clause,
            state_path: "crate::subject::Account",
            state_fields: &["balance", "audit"],
            subject_path: "crate::subject::deposit_debiting_within_range",
            unwind: 4,
        },
    )
    .expect("emitted package generates");
    let outcome = prove(&generated.postcondition);
    let counterexample = falsified(
        outcome.clone(),
        "postcondition `post.balance >= pre.balance` failed",
    );
    let mut inputs = twin.state_clause_inputs(&package, &clause, BALANCE, (0, 0), (0, 0));
    inputs.playback.clear();
    inputs
        .decode_falsification_from_run(
            &generated.postcondition.identity,
            &outcome,
            crate::unbounded_decode_limits(),
        )
        .unwrap_or_else(|error| panic!("real playback decodes: {error:?}\n{counterexample}"));
    let [(balance_name, balance), (audit_name, audit)] = inputs.playback.as_slice() else {
        panic!("the two declared state fields decode");
    };
    assert_eq!(
        (balance_name.as_str(), audit_name.as_str()),
        ("balance", "audit")
    );
    let (balance, audit) = (*balance, *audit);
    let mut account = subject::Account { balance, audit };
    subject::deposit_debiting_within_range(&mut account);
    assert_ne!(
        account.balance, balance,
        "the native run reproduces the debit"
    );
    inputs.post_state = vec![
        ("balance".to_owned(), account.balance),
        ("audit".to_owned(), account.audit),
    ];
    let replay = StateClauseReplay::new(inputs.clone()).expect("emitted package replays");
    assert_eq!(replay.packet.obligation_identity, Some([1; 32]));
    assert!(matches!(
        replay.packet.source,
        Some(qsl_replay::EnvelopeSource::Witness(_))
    ));
    let result = replay.replay().expect("the replay settles");
    let arm = witness_arm(&result);
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);

    let mut respecting = subject::Account { balance, audit };
    subject::deposit(&mut respecting);
    inputs.post_state = vec![
        ("balance".to_owned(), respecting.balance),
        ("audit".to_owned(), respecting.audit),
    ];
    let respecting = StateClauseReplay::new(inputs).expect("respecting replay builds");
    let result = respecting.replay().expect("respecting replay settles");
    let arm = witness_arm(&result);
    assert_eq!(arm.settlement(), WitnessSettlement::Inconclusive);
    assert!(matches!(
        arm.disagreement(),
        Some(DisagreementCause::Verdicts { .. })
    ));

    let mut wrong_harness = twin.state_clause_inputs(&package, &clause, BALANCE, (0, 0), (0, 0));
    wrong_harness.playback.clear();
    let wrong = KaniRunOutcome::Falsified {
        counterexample: counterexample.replace(
            &generated.postcondition.identity.harness_path().to_string(),
            "wrong::harness",
        ),
    };
    assert!(matches!(
        wrong_harness.decode_falsification_from_run(
            &generated.postcondition.identity,
            &wrong,
            crate::unbounded_decode_limits()
        ),
        Err(StateClauseReplayError::Decode(_))
    ));
    let mut malformed = twin.state_clause_inputs(&package, &clause, BALANCE, (0, 0), (0, 0));
    malformed.playback.clear();
    let malformed_run = KaniRunOutcome::Falsified {
        counterexample: counterexample.replacen("vec![", "vec![not-a-byte, ", 2),
    };
    assert!(matches!(
        malformed.decode_falsification_from_run(
            &generated.postcondition.identity,
            &malformed_run,
            crate::unbounded_decode_limits()
        ),
        Err(StateClauseReplayError::Decode(_))
    ));
}

/// Real Kani playback of the wrapping debit starts at the range floor. The native subject's
/// -1 post state survives CG's snapshot construction and QSL settles the violation.
#[trace("FR-029-AC-18", "TC-040")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_040_real_kani_out_of_range_debit_replays_as_refuted() {
    let twin = Twin::new();
    let (package, clause) = twin.emitted_package(BALANCE);
    let generated = quire_contract_codegen::generate_state_frame_obligations(
        &quire_contract_codegen::StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            package: &package,
            clause: &clause,
            state_path: "crate::subject::Account",
            state_fields: &["balance", "audit"],
            subject_path: "crate::subject::deposit_debiting",
            unwind: 4,
        },
    )
    .expect("emitted package generates");
    let evidence = super::kani_obligations_state_frame::prove_evidence(&generated.postcondition);
    let outcome = &evidence.outcome;
    falsified(
        outcome.clone(),
        "postcondition `post.balance >= pre.balance` failed",
    );
    let mut inputs = twin.state_clause_inputs(&package, &clause, BALANCE, (0, 0), (0, 0));
    inputs.playback.clear();
    inputs
        .decode_falsification_from_run(
            &generated.postcondition.identity,
            outcome,
            crate::unbounded_decode_limits(),
        )
        .expect("real Kani playback decodes");
    assert_eq!(
        inputs.playback,
        [("balance".to_owned(), 0), ("audit".to_owned(), 0)]
    );
    let mut account = subject::Account {
        balance: 0,
        audit: 0,
    };
    subject::deposit_debiting(&mut account);
    assert_eq!((account.balance, account.audit), (-1, 0));
    inputs.post_state = vec![
        ("balance".to_owned(), account.balance),
        ("audit".to_owned(), account.audit),
    ];
    let replay = StateClauseReplay::new(inputs).expect("exact post state builds");
    let docs = documents(&replay.wire);
    assert_eq!(integers(&docs.pre.1)["balance"]["integer"], "0");
    assert_eq!(integers(&docs.post.1)["balance"]["integer"], "-1");
    let result = replay.replay().expect("QSL replays the exact post state");
    let arm = witness_arm(&result);
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
    assert_eq!(
        run_terminal_value(&evidence, Some((&result).into())).expect("falsified run maps"),
        TerminalValue::Refuted
    );
}

//! The two-phase run (FR-017-AC-5, FR-017-AC-63, FR-017-AC-21, FR-028-AC-23): launcher stand-ins
//! answer a launch whose vector holds `concrete-playback` (the witness phase) differently from
//! every other launch (the proof phase).

use ix_trace_rs::trace;

use super::{
    batch_tests::{block, entry, member, memory_member, StandIn, FAILED, T},
    *,
};
use crate::kani::{
    identity::StateFrameHarness,
    test_support::{batch_report, COVER_OK, PASSED},
};

const UNWIND: (&str, &str, &str) = ("Failure", "unwind", "10");

/// A report of `harness` whose checks are `(status, category, line)`.
fn report_at(harness: &str, status: &str, checks: &[(&str, &str, &str)]) -> Vec<u8> {
    let checks: Vec<_> = checks
        .iter()
        .enumerate()
        .map(|(index, (status, category, line))| {
            serde_json::json!({
                "id": index + 1,
                "status": status,
                "category": category,
                "location": { "file": "src/lib.rs", "line": line, "column": "5" },
            })
        })
        .collect();
    let result = serde_json::json!({ "harness_id": harness, "status": status, "checks": checks });
    serde_json::to_vec(&serde_json::json!({
        "metadata": { "version": "1.0" },
        "verification_results": { "results": [result] }
    }))
    .unwrap()
}

/// One check of a report: status, category and line.
type Check = (&'static str, &'static str, &'static str);

const FAIL_10: (&str, &str, &str) = ("Failure", "assertion", "10");
const COVER: (&str, &str, &str) = ("Satisfied", "cover", "11");

/// Shell that answers one launch with `report` (when given), `console` and exit status `exit`.
fn answer(
    stand_in: &StandIn,
    tag: &str,
    report: Option<&[u8]>,
    console: &str,
    exit: i32,
) -> String {
    let mut body = String::new();
    if let Some(report) = report {
        let exported = stand_in.directory.join(format!("{tag}-report.json"));
        fs::write(&exported, report).unwrap();
        body.push_str(&format!("cp '{}' \"$last\"\n", exported.display()));
    }
    let printed = stand_in.directory.join(format!("{tag}-console"));
    fs::write(&printed, console).unwrap();
    body.push_str(&format!("cat '{}'\nexit {exit}\n", printed.display()));
    body
}

/// A stand-in whose proof launch fails `FAIL_10` of `a::check` and whose witness launch answers
/// with `witness_report` and `console`.
fn failing_then(
    name: &str,
    proof_checks: &[(&str, &str, &str)],
    witness: (Option<&[u8]>, &str, i32),
) -> StandIn {
    let proof = report_at("a::check", "Failure", proof_checks);
    let stand_in = StandIn::scaffold(name);
    let proof = answer(&stand_in, "proof", Some(&proof), "", 1);
    let witness = answer(&stand_in, "witness", witness.0, witness.1, witness.2);
    stand_in.install(&format!(
        "if [ \"$witness\" = yes ]; then\n{witness}\nelse\n{proof}\nfi\n"
    ));
    stand_in
}

fn run(stand_in: &StandIn, harness: &StateFrameHarness) -> KaniExecutionEvidence {
    execute_kani_obligation(&stand_in.request(harness, harness.identity.ceilings.wall_clock))
        .unwrap()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn literal_proof() -> Vec<String> {
    strings(&[
        "-Z",
        "function-contracts",
        "--harness",
        "a::check",
        "--exact",
        "--unwind",
        "4",
        "--solver",
        "cadical",
        "--output-format",
        "regular",
    ])
}

fn literal_witness() -> Vec<String> {
    strings(&[
        "-Z",
        "function-contracts",
        "-Z",
        "concrete-playback",
        "--harness",
        "a::check",
        "--exact",
        "--unwind",
        "4",
        "--solver",
        "cadical",
        "--output-format",
        "regular",
        "--concrete-playback",
        "print",
    ])
}

/// The options of a recorded launch: after `kani`, before the report export.
fn options_of(call: &[String]) -> &[String] {
    &call[1..call.len() - 4]
}

fn has_playback(call: &[String]) -> bool {
    call.iter()
        .any(|argument| argument.contains("concrete-playback"))
}

fn disagreed() -> KaniRunOutcome {
    KaniRunOutcome::Inconclusive {
        reason: KaniInconclusiveReason::WitnessDisagreed,
    }
}

fn inconclusive(reason: KaniInconclusiveReason) -> KaniRunOutcome {
    KaniRunOutcome::Inconclusive { reason }
}

/// (a) A failed proof run, then a witness run failing the same check and printing a playback, is
/// falsified carrying that playback verbatim, from two launches with the literal vectors, equal
/// ceilings, and both phases' evidence bound together.
#[trace("TC-027", "FR-017-AC-63")]
#[test]
fn the_witness_phase_falsifies_with_its_playback_under_the_literal_vectors() {
    let witness = report_at("a::check", "Failure", &[FAIL_10, COVER]);
    let console = block("a::check", "assertion", "falsify_a");
    let stand_in = failing_then(
        "two-phase-falsified",
        &[FAIL_10, COVER],
        (Some(&witness), &console, 1),
    );
    let harness = member("a", "check", 4);
    let evidence = run(&stand_in, &harness);
    assert!(matches!(
        &evidence.outcome,
        KaniRunOutcome::Falsified { counterexample } if counterexample.contains("fn from_falsify_a")
    ));
    let calls = stand_in.calls();
    assert_eq!(calls.len(), 2);
    assert_eq!(options_of(&calls[0]), literal_proof());
    assert_eq!(options_of(&calls[1]), literal_witness());
    let proof = evidence.proof_phase.as_deref().expect("the proof phase");
    assert_eq!(proof.arguments, calls[0]);
    assert_eq!(evidence.arguments, calls[1]);
    assert_eq!(proof.ceilings, evidence.ceilings);
    assert_eq!(proof.ceilings, harness.identity.ceilings);
    assert_eq!(proof.harness_path, evidence.harness_path);
    assert_eq!(
        proof.outcome,
        inconclusive(KaniInconclusiveReason::FailedWithoutCounterexample),
        "the proof vector prints no playback"
    );
    let wire = serde_json::to_value(&evidence).unwrap();
    assert_eq!(wire["proofPhase"]["arguments"][1], "-Z");
}

/// (b) A witness run that does not reproduce the proof run's failed (class, location) set is
/// `WitnessDisagreed`, never `Falsified` from either phase, and a witness that reproduces it in
/// another order is the control that stays falsified.
#[trace("TC-027", "FR-017-AC-5", "FR-017-AC-63")]
#[test]
fn a_witness_that_does_not_reproduce_the_failed_checks_disagrees() {
    let fail_other_location = ("Failure", "assertion", "11");
    let fail_other_class = ("Failure", "overflow", "10");
    let fail_12 = ("Failure", "assertion", "12");
    let fail_overflow_12 = ("Failure", "overflow", "12");
    let cases: Vec<(&str, Vec<Check>, &str, Vec<Check>)> = vec![
        (
            "verifies",
            vec![FAIL_10],
            "Success",
            vec![("Success", "assertion", "10"), COVER],
        ),
        (
            "other-location",
            vec![FAIL_10],
            "Failure",
            vec![fail_other_location],
        ),
        (
            "other-class",
            vec![FAIL_10],
            "Failure",
            vec![fail_other_class],
        ),
        ("plus-one", vec![FAIL_10], "Failure", vec![FAIL_10, fail_12]),
        (
            "subset-of-two",
            vec![FAIL_10, fail_overflow_12],
            "Failure",
            vec![FAIL_10],
        ),
        (
            "differs-from-two",
            vec![FAIL_10, fail_overflow_12],
            "Failure",
            vec![FAIL_10, fail_12],
        ),
        (
            "cover-unsatisfied",
            vec![FAIL_10],
            "Success",
            vec![
                ("Success", "assertion", "10"),
                ("Unsatisfiable", "cover", "11"),
            ],
        ),
        (
            "unwinding",
            vec![FAIL_10],
            "Failure",
            vec![("Failure", "unwind", "10")],
        ),
        (
            "unwinding-beside-match",
            vec![FAIL_10],
            "Failure",
            vec![FAIL_10, ("Failure", "unwind", "9")],
        ),
    ];
    for (name, proof, status, checks) in cases {
        let witness = report_at("a::check", status, &checks);
        let console = block("a::check", "assertion", "falsify_a");
        let stand_in = failing_then(
            &format!("two-phase-{name}"),
            &proof,
            (
                Some(&witness),
                &console,
                if status == "Success" { 0 } else { 1 },
            ),
        );
        let evidence = run(&stand_in, &member("a", "check", 4));
        assert_eq!(evidence.outcome, disagreed(), "{name}");
        assert_eq!(stand_in.calls().len(), 2, "{name}");
        assert_eq!(
            serde_json::to_value(&evidence).unwrap()["outcome"]["reason"],
            "witness_disagreed",
            "{name}"
        );
    }
    let proof = [FAIL_10, fail_overflow_12];
    let reordered = report_at("a::check", "Failure", &[fail_overflow_12, FAIL_10, COVER]);
    let stand_in = failing_then(
        "two-phase-reordered",
        &proof,
        (
            Some(&reordered),
            &block("a::check", "assertion", "falsify_a"),
            1,
        ),
    );
    assert!(matches!(
        run(&stand_in, &member("a", "check", 4)).outcome,
        KaniRunOutcome::Falsified { .. }
    ));
}

/// A witness that reproduces the failed checks but prints no counterexample block is inconclusive
/// with no counterexample.
#[trace("TC-027", "FR-017-AC-5", "FR-017-AC-63")]
#[test]
fn a_matching_witness_without_a_playback_has_no_counterexample() {
    let witness = report_at("a::check", "Failure", &[FAIL_10, COVER]);
    let console = block("a::check", "cover", "cover_a");
    let stand_in = failing_then(
        "two-phase-no-playback",
        &[FAIL_10],
        (Some(&witness), &console, 1),
    );
    let evidence = run(&stand_in, &member("a", "check", 4));
    assert_eq!(
        evidence.outcome,
        inconclusive(KaniInconclusiveReason::FailedWithoutCounterexample),
        "a playback printed for a satisfied cover is never a counterexample"
    );
}

/// (c) A witness launch that overruns the wall-clock ceiling, and one that would finish with a
/// verifying report only at 1.25 times it, are stopped at the unchanged ceiling.
#[trace("TC-027", "TC-039", "FR-017-AC-63", "FR-028-AC-2")]
#[test]
fn a_witness_phase_is_stopped_at_the_unchanged_wall_clock_ceiling() {
    let verifying = report_at(
        "a::check",
        "Success",
        &[("Success", "assertion", "10"), COVER],
    );
    for (name, sleep) in [("overrun", "20"), ("one-and-a-quarter", "2.5")] {
        let mut harness = member("a", "check", 4);
        harness.identity.ceilings.wall_clock = Duration::from_secs(2);
        let proof = report_at("a::check", "Failure", &[FAIL_10]);
        let stand_in = StandIn::scaffold(&format!("two-phase-wall-{name}"));
        let proof = answer(&stand_in, "proof", Some(&proof), "", 1);
        let exported = stand_in.directory.join("verifying.json");
        fs::write(&exported, &verifying).unwrap();
        let witness = format!(
            "sleep {sleep}\ncp '{}' \"$last\"\nexit 0\n",
            exported.display()
        );
        stand_in.install(&format!(
            "if [ \"$witness\" = yes ]; then\n{witness}\nelse\n{proof}\nfi\n"
        ));
        let evidence = run(&stand_in, &harness);
        assert_eq!(
            evidence.outcome,
            inconclusive(KaniInconclusiveReason::TimedOut),
            "{name}"
        );
        assert_eq!(stand_in.calls().len(), 2, "{name}: no third launch");
        assert_eq!(evidence.ceilings, harness.identity.ceilings, "{name}");
        assert_eq!(
            evidence.proof_phase.as_deref().unwrap().ceilings,
            evidence.ceilings,
            "{name}"
        );
    }
}

/// (c) A witness launch the environment kills with SIGKILL (as a memory cap does) exports no
/// report: the run is `NoVerdict`, which the runner takes as it finds it and never relabels as a
/// memory outcome (FR-028-AC-25).
#[trace("TC-027", "TC-039", "FR-017-AC-63", "FR-028-AC-25")]
#[test]
fn a_witness_phase_killed_by_the_environment_is_no_verdict() {
    let harness = member("a", "check", 4);
    let proof = report_at("a::check", "Failure", &[FAIL_10]);
    let stand_in = StandIn::scaffold("two-phase-witness-sigkill");
    let proof = answer(&stand_in, "proof", Some(&proof), "", 1);
    stand_in.install(&format!(
        "if [ \"$witness\" = yes ]; then\nkill -9 $$\nelse\n{proof}\nfi\n"
    ));
    let evidence = run(&stand_in, &harness);
    assert_eq!(
        evidence.outcome,
        inconclusive(KaniInconclusiveReason::NoVerdict)
    );
    assert_eq!(evidence.exit_code, None);
    assert_eq!(stand_in.calls().len(), 2, "no third launch");
}

/// (c) A witness launch exiting unsuccessfully with no report is `NoVerdict`.
#[trace("TC-027", "FR-017-AC-63")]
#[test]
fn a_witness_phase_with_no_report_after_a_failed_exit_has_no_verdict() {
    let stand_in = failing_then(
        "two-phase-no-verdict",
        &[FAIL_10],
        (None, "error: bad argument", 2),
    );
    let evidence = run(&stand_in, &member("a", "check", 4));
    assert_eq!(
        evidence.outcome,
        inconclusive(KaniInconclusiveReason::NoVerdict)
    );
    assert_eq!(stand_in.calls().len(), 2);
}

/// (c) A witness launch cancelled by its source is `Cancelled`, and its proof evidence stays.
#[trace("TC-027", "FR-017-AC-63", "FR-040-AC-3")]
#[test]
fn a_witness_phase_cancelled_by_its_source_is_cancelled() {
    let proof = report_at("a::check", "Failure", &[FAIL_10]);
    let stand_in = StandIn::scaffold("two-phase-cancelled");
    let proof = answer(&stand_in, "proof", Some(&proof), "", 1);
    let fifo = stand_in.directory.join("witness-started");
    let made = std::process::Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .unwrap();
    assert!(made.success());
    let witness = format!("echo started > '{}'\nsleep 30\n", fifo.display());
    stand_in.install(&format!(
        "if [ \"$witness\" = yes ]; then\n{witness}\nelse\n{proof}\nfi\n"
    ));
    let harness = member("a", "check", 4);
    let evidence = std::thread::scope(|scope| {
        let worker = scope.spawn(|| run(&stand_in, &harness));
        // The witness launch writes here only once it has started, so this read ends then.
        fs::read_to_string(&fifo).unwrap();
        stand_in.cancel.cancel(quire_exact::CancelCause::Requested);
        worker.join().unwrap()
    });
    assert!(matches!(evidence.outcome, KaniRunOutcome::Cancelled { .. }));
    assert!(evidence.proof_phase.is_some());
    assert_eq!(stand_in.calls().len(), 2);
}

/// (c) A witness report that is refused, and a witness launch that is refused, refuse the run with
/// `WitnessPhaseRefused` retaining the proof evidence, never classifying from the proof phase.
#[trace("TC-027", "FR-017-AC-63", "FR-017-AC-42")]
#[test]
fn a_refused_witness_report_or_launch_refuses_the_run_with_the_proof_evidence() {
    let stand_in = failing_then(
        "two-phase-report-refused",
        &[FAIL_10],
        (Some(b"not json"), "", 1),
    );
    let refusal = execute_kani_obligation(&stand_in.request(&member("a", "check", 4), T))
        .expect_err("a refused witness report refuses the run");
    let KaniExecutionRefusal::WitnessPhaseRefused {
        refusal,
        proof_evidence,
    } = refusal
    else {
        panic!("expected WitnessPhaseRefused: {refusal:?}");
    };
    assert!(matches!(*refusal, KaniExecutionRefusal::Report(_)));
    assert_eq!(proof_evidence.len(), 1);
    assert_eq!(
        proof_evidence[0].outcome,
        inconclusive(KaniInconclusiveReason::FailedWithoutCounterexample)
    );

    // The proof launch removes the launcher, so the witness launch cannot start.
    let proof = report_at("a::check", "Failure", &[FAIL_10]);
    let stand_in = StandIn::scaffold("two-phase-launch-refused");
    let exported = stand_in.directory.join("proof-report.json");
    fs::write(&exported, &proof).unwrap();
    stand_in.install(&format!(
        "cp '{}' \"$last\"\nrm -f \"$0\"\nexit 1\n",
        exported.display()
    ));
    let refusal = execute_kani_obligation(&stand_in.request(&member("a", "check", 4), T))
        .expect_err("a refused witness launch refuses the run");
    let KaniExecutionRefusal::WitnessPhaseRefused {
        refusal,
        proof_evidence,
    } = refusal
    else {
        panic!("expected WitnessPhaseRefused: {refusal:?}");
    };
    assert!(
        matches!(*refusal, KaniExecutionRefusal::Tool(_)),
        "{refusal:?}"
    );
    assert_eq!(proof_evidence.len(), 1);
}

/// (d, f) Only a failed non-unwinding check launches the witness phase: every other proof report
/// makes exactly one launch whose vector holds no `concrete-playback`.
#[trace("TC-027", "FR-017-AC-5", "FR-017-AC-63")]
#[test]
fn a_proof_phase_that_does_not_fail_a_property_launches_no_witness() {
    let cases: Vec<(&str, Option<Vec<u8>>, i32, KaniRunOutcome)> = vec![
        (
            "unwind-only",
            Some(report_at("a::check", "Failure", &[UNWIND])),
            1,
            inconclusive(KaniInconclusiveReason::UnwindBoundExhausted),
        ),
        (
            "unwind-beside-assertion",
            Some(report_at(
                "a::check",
                "Failure",
                &[FAIL_10, ("Failure", "unwind", "9")],
            )),
            1,
            inconclusive(KaniInconclusiveReason::UnwindBoundExhausted),
        ),
        (
            "no-failed-check",
            Some(report_at("a::check", "Failure", &[])),
            1,
            inconclusive(KaniInconclusiveReason::FailedWithoutCounterexample),
        ),
        (
            "verified",
            Some(report_at(
                "a::check",
                "Success",
                &[("Success", "assertion", "10"), COVER],
            )),
            0,
            KaniRunOutcome::Verified,
        ),
        (
            "cover-unsatisfied",
            Some(report_at(
                "a::check",
                "Success",
                &[
                    ("Success", "assertion", "10"),
                    ("Unsatisfiable", "cover", "11"),
                ],
            )),
            0,
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1,
            },
        ),
        (
            "no-report",
            None,
            2,
            inconclusive(KaniInconclusiveReason::NoVerdict),
        ),
    ];
    for (name, report, exit, expected) in cases {
        let stand_in = StandIn::scaffold(&format!("two-phase-single-{name}"));
        let body = answer(&stand_in, "only", report.as_deref(), "", exit);
        stand_in.install(&body);
        let evidence = run(&stand_in, &member("a", "check", 4));
        assert_eq!(evidence.outcome, expected, "{name}");
        assert!(evidence.proof_phase.is_none(), "{name}");
        let calls = stand_in.calls();
        assert_eq!(calls.len(), 1, "{name}: exactly one launch");
        assert!(!has_playback(&calls[0]), "{name}");
    }
}

fn three_with_middle_failing() -> Vec<u8> {
    batch_report(&[
        entry("a::check", "Success", &[PASSED, COVER_OK]),
        entry("b::check", "Failure", &[FAILED, COVER_OK]),
        entry("c::check", "Success", &[PASSED, COVER_OK]),
    ])
}

/// (e) In a batch of three whose proof report fails only the middle member, the outer members are
/// classified from the proof report and the middle member alone gets a witness launch, a single
/// run; the member the proof entry holds is never classified as lacking a counterexample.
#[trace("TC-027", "FR-017-AC-63", "FR-017-AC-21")]
#[test]
fn a_batch_relaunches_only_the_failed_member_as_a_single_witness_run() {
    let witness = report_at("b::check", "Failure", &[FAIL_10, COVER]);
    let witness_script_stand_in = StandIn::scaffold("two-phase-batch-three");
    let proof = answer(
        &witness_script_stand_in,
        "proof",
        Some(&three_with_middle_failing()),
        "",
        1,
    );
    let console = block("b::check", "assertion", "falsify_b");
    let witness = answer(
        &witness_script_stand_in,
        "witness",
        Some(&witness),
        &console,
        1,
    );
    witness_script_stand_in.install(&format!(
        "if [ \"$witness\" = yes ]; then\n{witness}\nelse\n{proof}\nfi\n"
    ));
    let harnesses = [
        member("a", "check", 4),
        member("b", "check", 4),
        member("c", "check", 4),
    ];
    let runs = witness_script_stand_in.batch(&harnesses, T).unwrap();
    assert_eq!(runs.len(), 2);
    assert_eq!(runs[0].members, [0, 2]);
    let outer = runs[0].evidence.as_ref().unwrap();
    assert!(outer
        .iter()
        .all(|evidence| evidence.outcome == KaniRunOutcome::Verified));
    assert!(outer.iter().all(|evidence| evidence.proof_phase.is_none()));
    assert_eq!(runs[1].members, [1]);
    let middle = runs[1].evidence.as_ref().unwrap();
    assert!(matches!(
        middle[0].outcome,
        KaniRunOutcome::Falsified { .. }
    ));
    assert!(middle[0].batch.is_none(), "a group of one is a single run");
    let calls = witness_script_stand_in.calls();
    assert_eq!(calls.len(), 2);
    assert!(!has_playback(&calls[0]));
    let mut single = vec!["kani".to_owned()];
    single.extend(harnesses[1].identity.witness_options.iter().cloned());
    assert_eq!(calls[1][..single.len()], single[..]);
    let proof_phase = middle[0].proof_phase.as_deref().unwrap();
    assert_eq!(proof_phase.batch.as_ref().unwrap().members.len(), 3);
}

fn four_with_middle_two_failing() -> Vec<u8> {
    batch_report(&[
        entry("a::check", "Success", &[PASSED, COVER_OK]),
        entry("b::check", "Failure", &[FAILED, COVER_OK]),
        entry("c::check", "Failure", &[FAILED, COVER_OK]),
        entry("d::check", "Success", &[PASSED, COVER_OK]),
    ])
}

/// A witness group classifies each held member from its own entry and its own playback, and the
/// batch result gains one group run for it.
#[trace("TC-027", "FR-017-AC-21", "FR-017-AC-63")]
#[test]
fn a_witness_group_classifies_each_held_member_from_its_own_playback() {
    let witness = batch_report(&[
        entry("b::check", "Failure", &[FAILED, COVER_OK]),
        entry("c::check", "Failure", &[FAILED, COVER_OK]),
    ]);
    let console = format!(
        "{}{}",
        block("b::check", "assertion", "falsify_b"),
        block("c::check", "assertion", "falsify_c")
    );
    let stand_in = StandIn::scaffold("two-phase-witness-group");
    let proof = answer(
        &stand_in,
        "proof",
        Some(&four_with_middle_two_failing()),
        "",
        1,
    );
    let witness = answer(&stand_in, "witness", Some(&witness), &console, 1);
    stand_in.install(&format!(
        "if [ \"$witness\" = yes ]; then\n{witness}\nelse\n{proof}\nfi\n"
    ));
    let harnesses = [
        member("a", "check", 4),
        member("b", "check", 4),
        member("c", "check", 4),
        member("d", "check", 4),
    ];
    let runs = stand_in.batch(&harnesses, T).unwrap();
    assert_eq!(runs[1].members, [1, 2]);
    let held = runs[1].evidence.as_ref().unwrap();
    assert!(matches!(
        &held[0].outcome,
        KaniRunOutcome::Falsified { counterexample } if counterexample.contains("falsify_b")
    ));
    assert!(matches!(
        &held[1].outcome,
        KaniRunOutcome::Falsified { counterexample } if counterexample.contains("falsify_c")
    ));
    assert!(held.iter().all(|evidence| evidence.proof_phase.is_some()));
    assert!(held.iter().all(|evidence| evidence.batch.is_some()));
}

/// FR-017-AC-21: held members whose witness vectors are equal but whose identity ceilings differ
/// start separate witness processes, each holding only the members of its own ceilings.
#[trace("TC-043", "FR-017-AC-21", "FR-017-AC-63")]
#[test]
fn held_members_with_different_ceilings_form_separate_witness_runs() {
    let low: u64 = 64 * 1024 * 1024;
    let high: u64 = 96 * 1024 * 1024;
    let stand_in = StandIn::scaffold("two-phase-witness-ceilings");
    let mut arms = String::new();
    for (pair, (first, second)) in [("ab", ("a", "b")), ("cd", ("c", "d"))] {
        let failing = batch_report(&[
            entry(&format!("{first}::check"), "Failure", &[FAILED, COVER_OK]),
            entry(&format!("{second}::check"), "Failure", &[FAILED, COVER_OK]),
        ]);
        let console = format!(
            "{}{}",
            block(&format!("{first}::check"), "assertion", "falsify"),
            block(&format!("{second}::check"), "assertion", "falsify")
        );
        let proof = answer(&stand_in, &format!("proof-{pair}"), Some(&failing), "", 1);
        let witness = answer(
            &stand_in,
            &format!("witness-{pair}"),
            Some(&failing),
            &console,
            1,
        );
        arms.push_str(&format!(
            "*{first}::check*)\nif [ \"$witness\" = yes ]; then\n{witness}\nelse\n{proof}\nfi\n;;\n"
        ));
    }
    stand_in.install(&format!("case \"$ids\" in\n{arms}esac\n"));
    let harnesses = [
        memory_member("a", low),
        memory_member("b", low),
        memory_member("c", high),
        memory_member("d", high),
    ];
    let runs = stand_in.batch(&harnesses, T).unwrap();
    let members: Vec<_> = runs.iter().map(|run| run.members.clone()).collect();
    assert_eq!(
        members,
        [vec![0, 1], vec![2, 3]],
        "the proof groups hold every member, so only one witness group for each ceiling remains"
    );
    let calls = stand_in.calls();
    assert_eq!(calls.len(), 4, "{calls:?} {runs:?}");
    for call in &calls {
        assert_eq!(
            call.iter()
                .filter(|argument| *argument == "--harness")
                .count(),
            2
        );
    }
    assert!(!has_playback(&calls[0]) && !has_playback(&calls[1]));
    assert!(has_playback(&calls[2]) && has_playback(&calls[3]));
    for run in &runs {
        let held = run.evidence.as_ref().unwrap();
        assert!(held
            .iter()
            .all(|evidence| matches!(evidence.outcome, KaniRunOutcome::Falsified { .. })));
    }
}

/// FR-015-AC-9 and AD-004: the proof vector holds no concrete-playback option and the witness
/// vector is the proof vector with `-Z concrete-playback` before `--harness` and
/// `--concrete-playback print` last, equal to the literal single vector recorded before the
/// two-phase run.
#[trace("TC-025", "FR-015-AC-9")]
#[test]
fn the_witness_vector_is_the_proof_vector_with_the_playback_options() {
    use crate::kani::abi::{adapter_options, witness_options};
    let proof = adapter_options("a::check", 4, KaniSolver::Cadical, false);
    let witness = witness_options("a::check", 4, KaniSolver::Cadical, false);
    assert_eq!(proof, literal_proof());
    assert_eq!(witness, literal_witness());
    assert!(!has_playback(&proof));
    assert_eq!(
        crate::kani::test_support::witness_of(&proof),
        witness,
        "the test builder agrees with the production vector"
    );
    let harness = member("a", "check", 4);
    assert_eq!(harness.identity.options, proof);
    assert_eq!(harness.identity.witness_options, witness);
    let stubbed = witness_options("a::check", 4, KaniSolver::Cadical, true);
    assert_eq!(
        stubbed
            .iter()
            .filter(|option| *option == "stubbing")
            .count(),
        1
    );
}

/// FR-028-AC-23: the run's evidence names the vector that produced each part of the verdict.
#[trace("TC-039", "FR-028-AC-23")]
#[test]
fn a_two_phase_verdict_names_the_vector_of_each_phase() {
    let witness = report_at("a::check", "Failure", &[FAIL_10]);
    let stand_in = failing_then(
        "two-phase-evidence-vectors",
        &[FAIL_10],
        (Some(&witness), "", 1),
    );
    let evidence = run(&stand_in, &member("a", "check", 4));
    let proof = evidence.proof_phase.as_deref().unwrap();
    assert!(!has_playback(&proof.arguments));
    assert!(has_playback(&evidence.arguments));
    assert_eq!(proof.unwind, evidence.unwind);
    assert_eq!(proof.solver, evidence.solver);
}

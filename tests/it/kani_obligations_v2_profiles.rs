//! FR-015-AC-126 and FR-015-AC-129 (IR-948): the V2 pre/post/invariant profiles over the QSL-source
//! `test/bank` package, `Account.balance: Int[0, 100]` and `withdraw(amount: Int[0, 100])`.
//!
//! The default lane checks what generation yields for the profile package: a harness for the
//! precondition and the invariant, and the two refusals that keep the postcondition member
//! planned. The `kani` lane
//! (`#[ignore]`, run one at a time by `make kani`) runs the installed Kani through CG's runner at
//! unwind 8, the two-phase run of FR-017-AC-63, and replays each invariant counterexample through
//! QSL. Peak memory is the `memory.peak` of the cgroup scope the test runs in.

use std::{path::PathBuf, sync::Arc, time::Instant};

use ix_trace_rs::trace;
use qsl_replay::{ReplayResult, WitnessSettlement};
use quire_contract_codegen::{
    execute_kani_obligation, negotiate_kani_native_obligations, parse_assertion_playback,
    KaniCheckClass, KaniCheckStatus, KaniExecutionEvidence, KaniExecutionRequest, KaniInstallation,
    KaniObligationOutcome, KaniObligationRequest, KaniRunOutcome, KaniV2ClauseObligationHarness,
    ObligationDisposition, ObligationItem, ObligationKind, ProofCeilings, StateClauseHarness,
    StateClauseReplay, UnsupportedObligation,
};
use quire_semantic_value::catalog::Category;

use super::kani_obligations_state_frame::native_twin::Twin;
use super::kani_obligations_v2_clause_controls::{subject_module, ScopePeak, ScratchCrate};

const UNWIND: u32 = 8;
const CAP_BYTES: u64 = 24 * 1024 * 1024 * 1024;
const INVARIANT_COVER: &str = "the V2 invariant's admitted state domain is reachable";
const PRECONDITION_COVER: &str = "the V2 precondition is satisfiable within admitted input bounds";

/// The ceilings of the installed runs: the 24 GiB cgroup of the record and 30 minutes.
const CEILINGS: ProofCeilings = ProofCeilings {
    memory_bytes: std::num::NonZeroU64::new(CAP_BYTES).unwrap(),
    wall_clock: std::time::Duration::from_secs(30 * 60),
};
const PRECONDITION: &str = "amount <= self.balance";
const POSTCONDITION: &str =
    "self.balance <= pre(self.balance) - amount and self.balance >= pre(self.balance) - amount";
const INVARIANT: &str = "self.balance >= 0";

fn profile(balance: (i64, i64), precondition: &str, invariant: &str) -> Twin {
    Twin::with_withdraw_profile(balance, precondition, POSTCONDITION, invariant)
}

fn healthy() -> Twin {
    profile((0, 100), PRECONDITION, INVARIANT)
}

fn harness_of(twin: &Twin, clause: &str) -> KaniV2ClauseObligationHarness {
    let (package, node) = twin.emitted_package(clause);
    let claims = crate::common::v2_claim::claims_for(&package, std::slice::from_ref(&node));
    let outcome = negotiate_kani_native_obligations(&KaniObligationRequest {
        ceilings: CEILINGS,
        items: &[ObligationItem::V2Clause {
            package: &package,
            clause: &node,
            claim: &claims[0],
            dependencies: None,
        }],
        subject_path: "crate::subject::check",
        unwind: UNWIND,
    })
    .expect("the clause negotiates");
    let KaniObligationOutcome::Emitted {
        mut v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("{clause} emits a harness");
    };
    assert_eq!(v2_clause_harnesses.len(), 1);
    v2_clause_harnesses.remove(0)
}

/// FR-015-AC-129: the precondition and the invariant of the profile package each yield one
/// harness of their own kind, with distinct harness identities.
#[trace("FR-015-AC-129", "FR-015-AC-38", "TC-025")]
#[test]
fn the_profile_package_yields_a_harness_per_kind_with_distinct_identities() {
    let twin = healthy();
    let precondition = harness_of(&twin, "CanAfford");
    let invariant = harness_of(&twin, "NonNegative");
    assert_eq!(precondition.identity.kind, ObligationKind::Precondition);
    assert_eq!(invariant.identity.kind, ObligationKind::Invariant);
    assert_ne!(precondition.identity, invariant.identity);
    assert_ne!(
        precondition.identity.harness_symbol,
        invariant.identity.harness_symbol
    );
    assert_eq!(precondition.identity.ceilings, invariant.identity.ceilings);
}

/// The postcondition member of FR-015-AC-129 is blocked twice, and this test fails when either
/// blocker lifts so that the PARTIAL status is revisited: the CG generator refuses a
/// state-reading postcondition (IR-948 slice 2b), and QSL refuses the specified equality text
/// (QSL-1901).
#[trace("FR-015-AC-129", "FR-015-AC-43", "TC-025")]
#[test]
fn the_postcondition_member_is_blocked_by_the_generator_and_by_qsl() {
    let twin = healthy();
    let located = ["CanAfford", "Debited", "NonNegative"].map(|name| twin.emitted_package(name));
    let package = &located[0].0;
    let nodes = located.iter().map(|l| l.1.clone()).collect::<Vec<_>>();
    let claims = crate::common::v2_claim::claims_for(package, &nodes);
    let items = nodes
        .iter()
        .zip(&claims)
        .map(|(clause, claim)| ObligationItem::V2Clause {
            package,
            clause,
            claim,
            dependencies: None,
        })
        .collect::<Vec<_>>();
    let outcome = negotiate_kani_native_obligations(&KaniObligationRequest {
        ceilings: CEILINGS,
        items: &items,
        subject_path: "crate::subject::check",
        unwind: UNWIND,
    })
    .expect("the request negotiates");
    let dispositions = outcome
        .records()
        .iter()
        .map(|record| (record.kind, &record.disposition))
        .collect::<Vec<_>>();
    assert!(
        matches!(
            dispositions[..],
            [
                (
                    Some(ObligationKind::Precondition),
                    ObligationDisposition::Supported { .. }
                ),
                (
                    Some(ObligationKind::Postcondition),
                    ObligationDisposition::Unsupported {
                        reason: UnsupportedObligation::V2ClauseConditionNotRendered { .. }
                    }
                ),
                (
                    Some(ObligationKind::Invariant),
                    ObligationDisposition::Supported { .. }
                ),
            ]
        ),
        "{dispositions:?}"
    );

    let equality = Twin::with_withdraw_profile(
        (0, 100),
        PRECONDITION,
        "self.balance = pre(self.balance) - amount",
        INVARIANT,
    );
    let Err(refusal) = equality.clause_site("Debited") else {
        panic!("QSL refuses the specified equality text");
    };
    let refusal = format!("{refusal:?}");
    assert!(
        refusal.contains("UndefinedExpression") && refusal.contains("unproved-range"),
        "{refusal}"
    );
}

fn run(label: &str, harness: &KaniV2ClauseObligationHarness) -> KaniExecutionEvidence {
    let lib = format!("{}{}", harness.rust.contents, subject_module("true"));
    let scratch = ScratchCrate::new(label, &lib, None);
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover()
        .expect("cargo-kani is installed")
        .with_peak_memory_source(Arc::new(ScopePeak));
    let target = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ir948-profile-kani-target");
    let started = Instant::now();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: harness.into(),
        crate_directory: &scratch.directory.path,
        target_directory: &target,
        home_directory: None,
    })
    .unwrap_or_else(|refusal| panic!("the run must start: {refusal}"));
    println!(
        "IR948 {label}: outcome={} wall={:.1}s memory={:?} success_checks={} checks={} two_phase={}",
        match &evidence.outcome {
            KaniRunOutcome::Falsified { .. } => "Falsified".to_owned(),
            other => format!("{other:?}"),
        },
        started.elapsed().as_secs_f64(),
        evidence.memory,
        evidence.success_checks,
        evidence.checks.len(),
        evidence.proof_phase.is_some(),
    );
    evidence
}

/// The one `kani::cover!` of the harness, which Kani attributes to the line of `message` in the
/// compiled source, is `Satisfied`.
fn named_cover_satisfied(
    evidence: &KaniExecutionEvidence,
    harness: &KaniV2ClauseObligationHarness,
    message: &str,
) -> bool {
    let line = harness
        .rust
        .contents
        .lines()
        .position(|line| line.contains("kani::cover!") && line.contains(message))
        .map(|index| u32::try_from(index + 1).expect("line number"));
    assert!(line.is_some(), "the harness covers `{message}`");
    let covers = evidence
        .checks
        .iter()
        .filter(|check| check.class == KaniCheckClass::Cover)
        .collect::<Vec<_>>();
    assert_eq!(covers.len(), 1, "exactly one cover: {covers:?}");
    assert_eq!(covers[0].location.line, line, "the cover is the named one");
    covers[0].status == KaniCheckStatus::Satisfied
}

/// The run held the 24 GiB cap's budget: the identity ceiling is the cap and the environment
/// supplied a peak below it. `make kani` runs the lane without a scope; the cap itself is the
/// caller's (see the record).
fn assert_within_cap(evidence: &KaniExecutionEvidence) {
    assert_eq!(evidence.ceilings.memory_bytes.get(), CAP_BYTES);
    let peak = evidence
        .memory
        .peak_resident_bytes
        .expect("the cgroup source supplied a peak");
    assert!(peak < CAP_BYTES, "peak {peak} B is under the cap");
}

#[trace("FR-015-AC-126", "FR-015-AC-129", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_verifies_the_profile_precondition_with_its_cover_satisfied() {
    let harness = harness_of(&healthy(), "CanAfford");
    let evidence = run("ir948-pre-healthy", &harness);
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert!(evidence.proof_phase.is_none());
    assert!(named_cover_satisfied(
        &evidence,
        &harness,
        PRECONDITION_COVER
    ));
    assert_within_cap(&evidence);
}

#[trace("FR-015-AC-126", "FR-015-AC-129", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_reports_the_impossible_profile_precondition_as_cover_unsatisfied() {
    let twin = profile((0, 100), "amount > 100", INVARIANT);
    let harness = harness_of(&twin, "CanAfford");
    let evidence = run("ir948-pre-impossible", &harness);
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: 1
        }
    );
    assert!(
        evidence.proof_phase.is_none(),
        "the verdict is the proof phase's"
    );
    assert!(!named_cover_satisfied(
        &evidence,
        &harness,
        PRECONDITION_COVER
    ));
    assert_within_cap(&evidence);
}

#[trace("FR-015-AC-126", "FR-015-AC-129", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_verifies_the_profile_invariant_with_its_cover_satisfied() {
    let harness = harness_of(&healthy(), "NonNegative");
    let evidence = run("ir948-inv-healthy", &harness);
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert!(evidence.proof_phase.is_none());
    assert!(named_cover_satisfied(&evidence, &harness, INVARIANT_COVER));
    assert_within_cap(&evidence);
}

/// Runs the mutant's invariant harness, requires a two-phase `Falsified` verdict whose in-domain
/// counterexample is `expected`, and replays it through QSL as a reproduced violation.
fn falsified_and_replayed(label: &str, twin: &Twin, expected: i64) {
    let harness = harness_of(twin, "NonNegative");
    let evidence = run(label, &harness);
    assert_within_cap(&evidence);
    let KaniRunOutcome::Falsified { counterexample } = &evidence.outcome else {
        panic!("expected Falsified, got {:?}", evidence.outcome);
    };
    assert!(
        evidence.proof_phase.is_some(),
        "the verdict comes from the witness phase"
    );
    let parsed = parse_assertion_playback(
        counterexample,
        harness.identity.harness_symbol.as_str(),
        harness.identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("the real playback parses");
    let (package, node) = twin.emitted_package("NonNegative");
    let object = harness.identity.state_fields[0].declaring_node_id.clone();
    let mut inputs = twin.invariant_inputs(&package, &node, &object, "NonNegative", &["balance"]);
    inputs
        .decode_falsification(
            StateClauseHarness::V2Clause(&harness),
            &parsed,
            crate::unbounded_decode_limits(),
        )
        .expect("the counterexample decodes in the package's domain");
    let replay = StateClauseReplay::new_invariant(inputs).expect("the replay is built");
    let snapshot = replay
        .wire
        .documents
        .iter()
        .filter_map(|bytes| serde_json::from_slice::<serde_json::Value>(bytes).ok())
        .find(|document| document["format"] == "quire.state.snapshot/v1")
        .expect("a state snapshot");
    assert_eq!(
        snapshot["populations"][0]["objects"][0]["fields"]["balance"]["integer"],
        expected.to_string(),
        "the replayed state is the counterexample"
    );
    let qsl_replay::ReplaySource::Witness(witness) = &replay.wire.source else {
        panic!("the invariant envelope rides the witness arm");
    };
    assert_eq!(
        witness.transcript(),
        format!("<<<assertion|withdraw|NonNegative|balance={expected}>>>"),
        "the transcript's operation is the selected anchor's"
    );
    let result = replay.replay().expect("the counterexample replays");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("the invariant envelope is on the witness arm");
    };
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
}

#[trace(
    "FR-015-AC-126",
    "FR-015-AC-129",
    "FR-017-AC-63",
    "FR-024-AC-63",
    "FR-024-AC-64",
    "TC-025",
    "TC-035"
)]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_falsifies_the_strengthened_invariant_mutant_at_balance_0() {
    falsified_and_replayed(
        "ir948-inv-strengthened",
        &profile((0, 100), PRECONDITION, "self.balance >= 1"),
        0,
    );
}

#[trace(
    "FR-015-AC-126",
    "FR-015-AC-129",
    "FR-017-AC-63",
    "FR-024-AC-63",
    "FR-024-AC-64",
    "TC-025",
    "TC-035"
)]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_falsifies_the_widened_invariant_mutant_at_balance_minus_1() {
    falsified_and_replayed(
        "ir948-inv-widened",
        &profile((-1, 100), PRECONDITION, INVARIANT),
        -1,
    );
}

//! FR-015-AC-126 and FR-015-AC-129 (IR-948): the V2 pre/post/invariant profiles over the QSL-source
//! `test/bank` package, `Account.balance: Int[0, 100]` and `withdraw(amount: Int[0, 100])`.
//!
//! The default lane checks what generation yields for the profile package. The `kani` lane
//! (`#[ignore]`, run one at a time by `make kani`) runs the installed Kani through CG's runner at
//! unwind 8, the two-phase run of FR-017-AC-63, and replays each invariant counterexample through
//! QSL. Peak memory is the `memory.peak` of the cgroup scope the test runs in.

use std::{path::PathBuf, sync::Arc, time::Instant};

use ix_trace_rs::trace;
use qsl_replay::{ReplayResult, WitnessSettlement, WitnessValue};
use quire_contract_codegen::{
    decode_v2_clause_falsification, execute_kani_obligation, negotiate_kani_native_obligations,
    parse_assertion_playback, KaniExecutionEvidence, KaniExecutionRequest, KaniInstallation,
    KaniObligationOutcome, KaniObligationRequest, KaniRunOutcome, KaniV2ClauseObligationHarness,
    ObligationItem, ObligationKind, StateClauseHarness, StateClauseReplay,
};
use quire_semantic_value::catalog::Category;

use super::kani_obligations_state_frame::native_twin::Twin;
use super::kani_obligations_v2_clause_controls::{subject_module, ScopePeak, ScratchCrate};

const UNWIND: u32 = 8;
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
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
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

const BANK_SUBJECT_PATH: &str = "crate::subject::check";

/// The customer subject of `withdraw`: it receives the state field by `&mut` and then the
/// operation parameter (FR-025-AC-8).
fn bank_subject(body: &str) -> String {
    format!(
        "\npub mod subject {{\n    pub fn check(balance: &mut i64, amount: i64) {{ {body} }}\n}}\n"
    )
}

const HEALTHY_WITHDRAW: &str = "*balance -= amount;";
/// Debits one less than asked once anything is asked: the produced balance stays in
/// `[0, 100]` and the postcondition fails whenever `amount >= 1`.
const UNDER_DEBITING_WITHDRAW: &str = "*balance -= if amount > 0 { amount - 1 } else { 0 };";

/// The postcondition `Debited` requested with its same-anchor precondition `CanAfford`.
fn postcondition_harness(twin: &Twin) -> KaniV2ClauseObligationHarness {
    let (package, post) = twin.emitted_package("Debited");
    let (_, pre) = twin.emitted_package("CanAfford");
    let nodes = [pre, post.clone()];
    let claims = crate::common::v2_claim::claims_for(&package, &nodes);
    let items = nodes
        .iter()
        .zip(&claims)
        .map(|(clause, claim)| ObligationItem::V2Clause {
            package: &package,
            clause,
            claim,
            dependencies: None,
        })
        .collect::<Vec<_>>();
    let outcome = negotiate_kani_native_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items: &items,
        subject_path: BANK_SUBJECT_PATH,
        unwind: UNWIND,
    })
    .expect("the clauses negotiate");
    let KaniObligationOutcome::Emitted {
        mut v2_clause_harnesses,
        records,
        ..
    } = outcome
    else {
        panic!("the postcondition emits a harness");
    };
    assert!(!records.is_empty());
    let index = v2_clause_harnesses
        .iter()
        .position(|harness| harness.identity.clause == post)
        .expect("the postcondition has a harness");
    v2_clause_harnesses.remove(index)
}

/// FR-015-AC-43, FR-015-AC-39, FR-025-AC-8: the state-reading postcondition yields a harness that
/// draws the pre-state and the parameter, assumes the sibling precondition, calls the subject with
/// `&mut` to a harness-owned copy and then the parameter, asserts the produced field's domain
/// before the clause, and records the sibling as its `requires`.
#[trace("FR-015-AC-43", "FR-015-AC-39", "FR-025-AC-8", "TC-025")]
#[test]
fn the_state_reading_postcondition_yields_a_harness_with_requires_and_a_mut_subject() {
    let twin = healthy();
    let harness = postcondition_harness(&twin);
    let identity = &harness.identity;
    assert_eq!(identity.kind, ObligationKind::Postcondition);
    assert_eq!(identity.requires.len(), 1);
    assert_eq!(
        identity.state_fields.len(),
        1,
        "one drawn pre-state for the field read bare and under pre()"
    );
    assert_eq!(identity.state_fields[0].field_name, "balance");
    assert_eq!(identity.arguments.len(), 1);
    let source = harness.rust.contents.as_str();
    let position = |needle: &str| {
        source
            .find(needle)
            .unwrap_or_else(|| panic!("the harness holds `{needle}`:\n{source}"))
    };
    let draw = position("let pre_balance_current: i64 = kani::any();");
    let parameter = position("let parameter_amount_current: i64 = kani::any();");
    let requires = position(
        "kani::assume((i128::from(parameter_amount_current) <= i128::from(pre_balance_current)));",
    );
    let copy = position("let mut post_balance_current: i64 = pre_balance_current;");
    let call =
        position("crate::subject::check(&mut post_balance_current, parameter_amount_current);");
    let domain = position("the produced V2 state field lies in its admitted domain");
    let oracle = call
        + source[call..]
            .find("oracle_clause(")
            .expect("the harness calls the oracle");
    let holds = position("the V2 postcondition holds for the produced state");
    assert!(draw < parameter && parameter < requires && requires < copy);
    assert!(
        copy < call && call < domain && domain < expected && expected < oracle && oracle < holds
    );
    assert_eq!(
        source.matches("kani::assume(post_balance_current").count(),
        0,
        "the produced state is asserted, never assumed"
    );
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

fn run(label: &str, harness: &KaniV2ClauseObligationHarness) -> KaniExecutionEvidence {
    run_with_subject(label, harness, &subject_module("true"))
}

fn run_with_subject(
    label: &str,
    harness: &KaniV2ClauseObligationHarness,
    subject: &str,
) -> KaniExecutionEvidence {
    let lib = format!("{}{subject}", harness.rust.contents);
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

fn cover_satisfied(evidence: &KaniExecutionEvidence) -> bool {
    evidence
        .checks
        .iter()
        .any(|check| format!("{:?}", check.status) == "Satisfied")
}

#[trace("FR-015-AC-126", "FR-015-AC-129", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_verifies_the_profile_precondition_with_its_cover_satisfied() {
    let evidence = run("ir948-pre-healthy", &harness_of(&healthy(), "CanAfford"));
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert!(evidence.proof_phase.is_none());
    assert!(cover_satisfied(&evidence), "the final cover is satisfied");
}

#[trace("FR-015-AC-126", "FR-015-AC-129", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_reports_the_impossible_profile_precondition_as_cover_unsatisfied() {
    let twin = profile((0, 100), "amount > 100", INVARIANT);
    let evidence = run("ir948-pre-impossible", &harness_of(&twin, "CanAfford"));
    assert!(
        matches!(evidence.outcome, KaniRunOutcome::CoverUnsatisfied { .. }),
        "{:?}",
        evidence.outcome
    );
}

#[trace("FR-015-AC-126", "FR-015-AC-129", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_verifies_the_profile_invariant_with_its_cover_satisfied() {
    let evidence = run("ir948-inv-healthy", &harness_of(&healthy(), "NonNegative"));
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert!(evidence.proof_phase.is_none());
    assert!(cover_satisfied(&evidence), "the final cover is satisfied");
}

/// Runs the mutant's invariant harness, requires a two-phase `Falsified` verdict whose in-domain
/// counterexample is `expected`, and replays it through QSL as a reproduced violation.
fn falsified_and_replayed(label: &str, twin: &Twin, expected: i64) {
    let harness = harness_of(twin, "NonNegative");
    let evidence = run(label, &harness);
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

#[trace(
    "FR-015-AC-126",
    "FR-015-AC-129",
    "FR-015-AC-43",
    "FR-015-AC-39",
    "TC-025"
)]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_verifies_the_profile_postcondition_with_its_cover_satisfied() {
    let evidence = run_with_subject(
        "ir948-post-healthy",
        &postcondition_harness(&healthy()),
        &bank_subject(HEALTHY_WITHDRAW),
    );
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert!(evidence.proof_phase.is_none());
    assert!(cover_satisfied(&evidence), "the final cover is satisfied");
}

/// The under-debiting subject keeps the balance in its domain and breaks the postcondition for
/// every `amount >= 1`; the witness phase's playback is decoded in the drawn domains and the
/// postcondition is evaluated natively over it.
#[trace(
    "FR-015-AC-126",
    "FR-015-AC-129",
    "FR-017-AC-63",
    "FR-015-AC-43",
    "TC-025",
    "TC-035"
)]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_falsifies_the_under_debiting_subject_with_in_domain_native_playback() {
    let harness = postcondition_harness(&healthy());
    let evidence = run_with_subject(
        "ir948-post-mutant",
        &harness,
        &bank_subject(UNDER_DEBITING_WITHDRAW),
    );
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
    assert!(
        format!("{parsed:?}").contains("the V2 postcondition holds for the produced state"),
        "the failed check is the postcondition"
    );
    let values =
        decode_v2_clause_falsification(&harness, &parsed, crate::unbounded_decode_limits())
            .expect("the playback decodes inside the drawn domains");
    let [(_, WitnessValue::Integer(balance)), (_, WitnessValue::Integer(amount))] =
        values.as_slice()
    else {
        panic!("one pre-state balance then one amount: {values:?}");
    };
    let (balance, amount) = (*balance, *amount);
    assert!((0..=100).contains(&balance) && (0..=100).contains(&amount));
    assert!(amount <= balance, "the precondition holds on the witness");
    let produced = balance - (amount - 1).max(0);
    assert!(
        !(produced <= balance - amount && produced >= balance - amount),
        "the postcondition is false on the witness: balance {balance}, amount {amount}, produced {produced}"
    );
}

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
use qsl_replay::{ReplayResult, WitnessSettlement, WitnessValue};
use quire_contract_codegen::{
    decode_v2_clause_falsification, execute_kani_obligation, negotiate_kani_native_obligations,
    parse_assertion_playback, KaniCheckClass, KaniCheckStatus, KaniExecutionEvidence,
    KaniExecutionRequest, KaniInstallation, KaniObligationOutcome, KaniObligationRequest,
    KaniRunOutcome, KaniV2ClauseObligationHarness, ObligationDisposition, ObligationItem,
    ObligationKind, ProofCeilings, StateClauseHarness, StateClauseReplay, UnsupportedObligation,
};
use quire_contract_model::CheckedNodeId;
use quire_semantic_value::catalog::Category;

use super::kani_obligations_state_frame::native_twin::Twin;
use super::kani_obligations_v2_clause_controls::{subject_module, ScopePeak, ScratchCrate};

const UNWIND: u32 = 8;
const CAP_BYTES: u64 = 24 * 1024 * 1024 * 1024;
const INVARIANT_COVER: &str = "the V2 invariant's admitted state domain is reachable";
const POSTCONDITION_COVER: &str =
    "the V2 postcondition is evaluated on an admitted state and parameters";
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

const BANK_SUBJECT_PATH: &str = "crate::subject::check";

/// A customer subject of `withdraw` that is both compiled into the test (`$name`) and handed to
/// Kani as source (`$text`): it receives the state field by `&mut` and then the operation
/// parameter. The native evaluation of a witness runs the very subject Kani falsified.
macro_rules! bank_subject {
    ($name:ident, $text:ident, |$balance:ident, $amount:ident| $($body:tt)*) => {
        fn $name($balance: &mut i64, $amount: i64) {
            $($body)*
        }
        const $text: &str = concat!(
            "\npub mod subject {\n    pub fn check(",
            stringify!($balance),
            ": &mut i64, ",
            stringify!($amount),
            ": i64) { ",
            stringify!($($body)*),
            " }\n}\n"
        );
    };
}

bank_subject!(healthy_withdraw, HEALTHY_WITHDRAW, |balance, amount| *balance -= amount;);
// Debits one less than asked once anything is asked: the produced balance stays in `[0, 100]`
// and the postcondition fails whenever `amount >= 1`.
bank_subject!(
    under_debiting_withdraw,
    UNDER_DEBITING_WITHDRAW,
    |balance, amount| *balance -= if amount > 0 { amount - 1 } else { 0 };
);

/// The profile postcondition `Debited` read natively over the pre-state, the produced state and
/// the parameter: the conjunction of its two comparisons.
fn debited(pre: i64, post: i64, amount: i64) -> bool {
    post <= pre - amount && post >= pre - amount
}

/// The profile negotiated with `CanAfford` and `Debited` requested together; the postcondition
/// is the second item.
fn negotiate_postcondition(twin: &Twin) -> (KaniObligationOutcome, CheckedNodeId) {
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
        ceilings: CEILINGS,
        items: &items,
        subject_path: BANK_SUBJECT_PATH,
        unwind: UNWIND,
    })
    .expect("the clauses negotiate");
    (outcome, post)
}

/// The postcondition `Debited` requested with its same-anchor precondition `CanAfford`.
fn postcondition_harness(twin: &Twin) -> KaniV2ClauseObligationHarness {
    let (outcome, post) = negotiate_postcondition(twin);
    let KaniObligationOutcome::Emitted {
        mut v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the postcondition emits a harness");
    };
    let index = v2_clause_harnesses
        .iter()
        .position(|harness| harness.identity.clause == post)
        .expect("the postcondition has a harness");
    v2_clause_harnesses.remove(index)
}

/// FR-015-AC-43, FR-015-AC-39: the state-reading postcondition yields a harness that draws the
/// pre-state and the parameter, assumes the sibling precondition through that precondition's own
/// oracle, copies the pre-state into a harness-owned value, calls the subject with `&mut` to the
/// copy and then the parameter, asserts the produced field's domain before the clause, feeds the
/// oracle the pre-state copy for `pre(...)` reads and the produced value for bare reads, and
/// records the sibling as its `requires`.
#[trace("FR-015-AC-43", "FR-015-AC-39", "TC-025")]
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
    let sibling_input = position(
        "let requires_0_current_balance = nx::NativeInteger::new(i128::from(pre_balance_current));",
    );
    let sibling_amount = position("let requires_0_parameter_amount = nx::NativeInteger::new(i128::from(parameter_amount_current));");
    let sibling_call = position("let requires_0_outcome = requires_0::oracle_clause(");
    let requires = position("kani::assume(matches!(&requires_0_outcome, Ok(true)));");
    let copy = position("let mut post_balance_current: i64 = pre_balance_current;");
    let call =
        position("crate::subject::check(&mut post_balance_current, parameter_amount_current);");
    let domain = position("the produced V2 state field lies in its admitted domain");
    let expected = position("let expected = ");
    let pre_input =
        position("let pre_balance = nx::NativeInteger::new(i128::from(pre_balance_current));");
    let post_input =
        position("let current_balance = nx::NativeInteger::new(i128::from(post_balance_current));");
    let oracle = call
        + source[call..]
            .find("oracle_clause(")
            .expect("the harness calls the oracle");
    let holds = position("the V2 postcondition holds for the produced state");
    assert!(parameter < sibling_input.min(sibling_amount) && draw < parameter);
    assert!(
        sibling_input.max(sibling_amount) < sibling_call
            && sibling_call < requires
            && requires < copy
    );
    assert!(copy < call && call < domain && domain < expected && expected < oracle);
    assert!(domain < pre_input.min(post_input) && pre_input.max(post_input) < oracle);
    assert!(oracle < holds);
    // The independent expectation reads the produced value for bare reads and the copy for pre().
    let expectation = &source[expected
        ..source[expected..]
            .find(';')
            .map(|end| expected + end)
            .unwrap()];
    assert!(
        expectation.contains("from (post_balance_current)")
            && expectation.contains("from (pre_balance_current)"),
        "{expectation}"
    );
    assert!(
        source.contains("mod requires_0 {"),
        "the sibling oracle is embedded in its own module"
    );
    assert_eq!(
        source.matches("kani::assume(post_balance_current").count(),
        0,
        "the produced state is asserted, never assumed"
    );
}

/// FR-025-AC-8: the subject receives each state value by `&mut` to a harness-owned value and the
/// harness copies the pre-state before the call.
#[trace("FR-025-AC-8", "TC-036")]
#[test]
fn the_postcondition_subject_receives_a_mut_copy_and_the_pre_state_survives() {
    let harness = postcondition_harness(&healthy());
    let source = harness.rust.contents.as_str();
    let copy = source
        .find("let mut post_balance_current: i64 = pre_balance_current;")
        .expect("the copy");
    let call = source
        .find("crate::subject::check(&mut post_balance_current, parameter_amount_current);")
        .expect("the call");
    assert!(copy < call);
    assert!(
        !source.contains("check(&mut pre_balance_current"),
        "the pre-state is never handed to the subject"
    );
}

/// FR-015-AC-43: a sibling precondition the state-reading postcondition cannot assume refuses the
/// postcondition naming that precondition, with no harness for it.
fn assert_sibling_refused(twin: &Twin) {
    let (outcome, post) = negotiate_postcondition(twin);
    let record = &outcome.records()[1];
    assert_eq!(record.kind, Some(ObligationKind::Postcondition));
    assert!(
        matches!(
            &record.disposition,
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::V2ClauseSiblingRequiresNotRendered { .. }
            }
        ),
        "{:?}",
        record.disposition
    );
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the request is emitted");
    };
    assert!(v2_clause_harnesses
        .iter()
        .all(|harness| harness.identity.clause != post));
}

#[trace("FR-015-AC-43", "TC-025")]
#[test]
fn an_arithmetic_sibling_precondition_refuses_the_state_postcondition() {
    assert_sibling_refused(&profile((0, 100), "amount + 1 <= self.balance", INVARIANT));
}

/// QSL refuses a `pre(...)` read in a precondition, so no sibling reaches the renderer's
/// `pre(...)` sibling arm; this pins that refusal and fails if QSL starts to admit one.
#[trace("FR-015-AC-43", "TC-025")]
#[test]
fn qsl_refuses_a_pre_read_in_a_sibling_precondition() {
    let twin = profile((0, 100), "amount <= pre(self.balance)", INVARIANT);
    let Err(refusal) = twin.clause_site("CanAfford") else {
        panic!("QSL refuses a pre read in a precondition");
    };
    assert!(format!("{refusal:?}").contains("forbidden-pre-read"));
}

/// FR-015-AC-39, FR-016 decode: a harness draws its state fields before its parameters, whatever
/// order its oracle takes its inputs in. The precondition `amount <= self.balance` is a mixed
/// clause whose oracle takes `parameter_amount` before `current_balance`; its draws are the
/// state field first, and a playback decodes onto the matching names.
#[trace("FR-015-AC-39", "TC-025")]
#[test]
fn a_mixed_precondition_draws_its_state_field_first_and_decodes_by_name() {
    let harness = harness_of(&healthy(), "CanAfford");
    let source = harness.rust.contents.as_str();
    let oracle_signature = source
        .find("pub fn oracle_clause(")
        .map(|start| &source[start..source[start..].find('{').map(|end| start + end).unwrap()])
        .expect("the oracle");
    assert!(
        oracle_signature.find("parameter_amount").unwrap()
            < oracle_signature.find("current_balance").unwrap(),
        "the oracle takes the parameter first: {oracle_signature}"
    );
    let state = source.find("let current_balance: i64 = kani::any();");
    let amount = source.find("let parameter_amount: i64 = kani::any();");
    assert!(
        state.is_some() && amount.is_some() && state < amount,
        "the state field is drawn first:\n{source}"
    );
    let state_identifier = harness.identity.state_fields[0].identifier.clone();
    let argument_identifier = harness.identity.arguments[0].identifier.clone();
    // Kani's concrete playback lists the draws in source order: the balance 90, then the amount 7.
    let entry =
        |value: u8| format!("        // {value}\n        vec![{value}, 0, 0, 0, 0, 0, 0, 0],\n");
    let counterexample = format!(
        "/// Test generated for harness `{}::{}` \n///\n/// Check for `assertion`: \"\"x\"\"\n\n#[test]\nfn kani_concrete_playback_x_1() {{\n    let concrete_vals: Vec<Vec<u8>> = vec![\n{}{}    ];\n    kani::concrete_playback_run(concrete_vals, x);\n}}",
        harness.identity.module_symbol.as_str(),
        harness.identity.harness_symbol.as_str(),
        entry(90),
        entry(7),
    );
    let parsed = parse_assertion_playback(
        &counterexample,
        harness.identity.harness_symbol.as_str(),
        harness.identity.module_symbol.as_str(),
        usize::MAX,
        usize::MAX,
    )
    .expect("the playback parses");
    let values =
        decode_v2_clause_falsification(&harness, &parsed, crate::unbounded_decode_limits())
            .expect("the playback decodes");
    assert_eq!(
        values,
        [
            (state_identifier, WitnessValue::Integer(90)),
            (argument_identifier, WitnessValue::Integer(7)),
        ]
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

/// The postcondition member of FR-015-AC-129 is blocked once now: the CG generator renders the
/// state-reading postcondition (IR-948 slice 2b), and QSL still refuses the specified equality
/// text (QSL-1901). This test fails when QSL lifts so that the PARTIAL status is revisited.
#[trace("FR-015-AC-129", "FR-015-AC-43", "TC-025")]
#[test]
fn the_postcondition_renders_and_the_specified_equality_text_is_blocked_by_qsl() {
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
                    ObligationDisposition::Supported { .. }
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
    let harness = postcondition_harness(&healthy());
    let evidence = run_with_subject("ir948-post-healthy", &harness, HEALTHY_WITHDRAW);
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert!(evidence.proof_phase.is_none());
    assert!(named_cover_satisfied(
        &evidence,
        &harness,
        POSTCONDITION_COVER
    ));
    assert_within_cap(&evidence);
}

/// The under-debiting subject keeps the balance in its domain and breaks the postcondition for
/// every `amount >= 1`. The witness phase's playback is decoded by name inside the drawn domains,
/// and the same compiled subject then runs natively over the decoded pre-state and amount; the
/// postcondition, read natively over the pre-state, the produced state and the amount, is false.
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
    let evidence = run_with_subject("ir948-post-mutant", &harness, UNDER_DEBITING_WITHDRAW);
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
    assert!(
        format!("{parsed:?}").contains("the V2 postcondition holds for the produced state"),
        "the failed check is the postcondition"
    );
    let values =
        decode_v2_clause_falsification(&harness, &parsed, crate::unbounded_decode_limits())
            .expect("the playback decodes inside the drawn domains");
    let [(balance_name, WitnessValue::Integer(balance)), (amount_name, WitnessValue::Integer(amount))] =
        values.as_slice()
    else {
        panic!("one pre-state balance then one amount: {values:?}");
    };
    assert_eq!(balance_name, "pre_balance_current");
    assert_eq!(amount_name, "parameter_amount_current");
    let (pre, amount) = (
        i64::try_from(*balance).expect("in i64"),
        i64::try_from(*amount).expect("in i64"),
    );
    assert!((0..=100).contains(&pre) && (0..=100).contains(&amount));
    assert!(amount <= pre, "the precondition holds on the witness");
    let mut produced = pre;
    under_debiting_withdraw(&mut produced, amount);
    assert!(
        (0..=100).contains(&produced),
        "the produced state is in its domain"
    );
    assert!(
        !debited(pre, produced, amount),
        "the postcondition is false on the witness: balance {pre}, amount {amount}, produced {produced}"
    );
    assert!(
        debited(
            pre,
            {
                let mut healthy = pre;
                healthy_withdraw(&mut healthy, amount);
                healthy
            },
            amount
        ),
        "the healthy subject satisfies it on the same witness"
    );
}

// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scalar replay settlement converter and terminal pairing (TC-047 steps 10 to 14, FR-032-AC-8,
//! FR-032-AC-20 to FR-032-AC-25). Native controls only: no Kani proof or backend falsification.

use super::kani_obligations::boolean_implies_scalar;
use super::scalar_playback::{replay_fixture, transcript, ReplayFixture};
use super::scalar_replay_binding::{context, observe, plan};
use super::terminal_map::synthetic_evidence;
use ix_trace_rs::trace;
use qsl_replay::{
    Code, IdentityEncodeError, IncompleteCause, InconclusiveCause, NativeOutcome,
    OperatorParityReport, OperatorParityResult, ReplayLimits, Std001Code, TerminalValue,
};
use quire_contract_codegen::{
    run_terminal_value, scalar_ir_outcome_terminal_value, scalar_run_terminal_value,
    CompletedScalarReplay, KaniInconclusiveReason, KaniRunOutcome, LockedSource,
    OriginalScalarContext, ReplayInputs, ScalarAuthorityError, ScalarBuildError, ScalarSentRequest,
    ScalarSettlement, TerminalPairError, WitnessDecodeLimits, SCALAR_PLAYBACK_BYTES,
};
use quire_contract_ir::kani::{KaniCancellationSource, KaniOutcome, KaniOutcomeKind};
use quire_contract_model::CheckedPackageReadLimits;
use quire_exact::Integer;
use std::collections::BTreeMap;

fn falsified() -> KaniRunOutcome {
    KaniRunOutcome::Falsified {
        counterexample: "#[test] fn kani_concrete_playback() {}".to_owned(),
    }
}

fn run_pair(settlement: Option<ScalarSettlement<'_>>) -> Result<TerminalValue, TerminalPairError> {
    scalar_run_terminal_value(&synthetic_evidence(falsified(), 0), settlement)
}

fn ir_pair(settlement: Option<ScalarSettlement<'_>>) -> Result<TerminalValue, TerminalPairError> {
    scalar_ir_outcome_terminal_value(
        &KaniOutcome::counterexample("source", "context"),
        settlement,
    )
}

fn sent_of(completed: &CompletedScalarReplay) -> ScalarSentRequest {
    let (wire, claim, replay_limits) = completed.request();
    ScalarSentRequest {
        wire,
        claim,
        replay_limits,
    }
}

/// A completed `x + x` plan over `[2, 4]` and its genuine native observation.
fn completed_sum(fixture: &ReplayFixture) -> (CompletedScalarReplay, ScalarSentRequest) {
    let original = plan(fixture, &[2, 4]);
    let observation = observe(fixture, &original, false);
    assert_eq!(
        observation.outcome,
        NativeOutcome::Completed(Integer::from(6_i64))
    );
    let completed = original.complete(observation).unwrap();
    let sent = sent_of(&completed);
    (completed, sent)
}

fn sum_fixture() -> ReplayFixture {
    replay_fixture(
        "quire.op.integer.add",
        "x: Int[0, 9]",
        "Int[0, 18]",
        "x + x",
    )
}

/// One genuine report per outcome class reachable through a completed plan, with the actual
/// request it was produced from and the plan completed with that claim's native outcome.
fn reachable_reports(
    fixture: &ReplayFixture,
) -> Vec<(
    &'static str,
    CompletedScalarReplay,
    ScalarSentRequest,
    OperatorParityReport,
)> {
    let original = plan(fixture, &[2, 4]);
    let observation = observe(fixture, &original, false);
    let mut reports = Vec::new();
    for (label, outcome) in [
        ("agrees", NativeOutcome::Completed(Integer::from(6_i64))),
        ("diverged", NativeOutcome::Completed(Integer::from(7_i64))),
        ("incomplete", NativeOutcome::Incomplete),
        ("fault", NativeOutcome::ExecutionFault),
    ] {
        let mut driver = observation.clone();
        driver.outcome = outcome;
        let completed = plan(fixture, &[2, 4]).complete(driver).unwrap();
        let sent = sent_of(&completed);
        let report = qsl_replay::replay_operator_parity(
            sent.wire.clone(),
            sent.claim.clone(),
            sent.replay_limits,
        );
        reports.push((label, completed, sent, report));
    }
    reports
}

#[trace("TC-047", "FR-032-AC-7", "FR-032-AC-20", "FR-032-AC-21")]
#[test]
fn step_10_settlement_wraps_the_genuine_report_result_and_reads_the_fr032_table() {
    let fixture = sum_fixture();
    for (label, completed, sent, report) in reachable_reports(&fixture) {
        let settlement = completed.settlement(&sent, Some(&report)).unwrap();
        assert!(
            matches!(&settlement, ScalarSettlement::Parity(result) if std::ptr::eq(*result, report.result())),
            "{label}: Parity borrows the report's own result"
        );
        let value = run_pair(Some(settlement)).unwrap();
        assert_eq!(value, report.terminal_value(), "{label}");
        match (label, report.result(), &value) {
            ("agrees", OperatorParityResult::Agrees { .. }, TerminalValue::Inconclusive(cause)) => {
                assert!(matches!(cause, InconclusiveCause::ScalarAgrees(_)))
            }
            ("diverged", OperatorParityResult::Diverged { .. }, TerminalValue::Failed)
            | (
                "incomplete" | "fault",
                OperatorParityResult::GeneratedFault(_),
                TerminalValue::Failed,
            ) => {}
            other => panic!("{label}: unexpected class {other:?}"),
        }
    }
}

#[trace("TC-047", "FR-032-AC-7", "FR-032-AC-21")]
#[test]
fn step_10_refused_input_exact_incomplete_and_refusal_readings_come_from_genuine_reports() {
    let fixture = sum_fixture();
    let (completed, sent) = completed_sum(&fixture);
    let claim = sent.claim.clone();
    let wire = sent.wire.clone();
    let limits = sent.replay_limits;

    let mut out_of_range = claim.clone();
    let qsl_replay::ScalarOperation::Add { left, .. } = &mut out_of_range.operation else {
        panic!("addition")
    };
    left.value = 10;
    let refused_input = qsl_replay::replay_operator_parity(wire.clone(), out_of_range, limits);
    assert!(matches!(
        refused_input.result(),
        OperatorParityResult::RefusedInput(_)
    ));
    assert_eq!(
        run_pair(Some(ScalarSettlement::Parity(refused_input.result()))).unwrap(),
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(Code::InvalidRuntimeInput))
    );

    let mut exhausted = claim.clone();
    exhausted.limits.work_units = 0;
    let incomplete = qsl_replay::replay_operator_parity(wire.clone(), exhausted, limits);
    assert!(matches!(
        incomplete.result(),
        OperatorParityResult::Incomplete(_)
    ));
    assert_eq!(
        run_pair(Some(ScalarSettlement::Parity(incomplete.result()))).unwrap(),
        TerminalValue::Incomplete(IncompleteCause::ResourceExhausted)
    );

    let mut occurrence = claim.clone();
    occurrence.occurrence = qsl_replay::Origin::new(
        qsl_replay::Role::new("expression"),
        claim.occurrence.ordinal() + 1,
    );
    let refused = qsl_replay::replay_operator_parity(wire, occurrence, limits);
    let OperatorParityResult::Refused(refusal) = refused.result() else {
        panic!("a changed occurrence is refused: {:?}", refused.result())
    };
    let expected = TerminalValue::from_replay_refusal(refusal);
    assert!(
        matches!(
            &expected,
            TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(_))
        ),
        "a non-fault refusal keeps its catalog code: {expected:?}"
    );
    assert_eq!(
        run_pair(Some(ScalarSettlement::Parity(refused.result()))).unwrap(),
        expected
    );
    assert_eq!(expected, refused.terminal_value());
    // The converter refuses these claims against the retained original: they are reachable only
    // as genuine facade reports, never as a completed plan's settlement.
    assert!(matches!(
        completed.settlement(&sent, Some(&refused)),
        Err(ScalarBuildError::Report)
    ));
}

#[trace("TC-047", "FR-032-AC-20")]
#[test]
fn step_10_settlement_refuses_a_changed_input_bytes_budget_and_a_missing_report() {
    let fixture = sum_fixture();
    let (completed, sent) = completed_sum(&fixture);
    let report = qsl_replay::replay_operator_parity(
        sent.wire.clone(),
        sent.claim.clone(),
        sent.replay_limits,
    );
    assert!(completed.settlement(&sent, Some(&report)).is_ok());
    let smaller = sent
        .replay_limits
        .with_input_bytes(sent.replay_limits.input_bytes - 1);
    let changed = ScalarSentRequest {
        wire: sent.wire.clone(),
        claim: sent.claim.clone(),
        replay_limits: smaller,
    };
    let changed_report =
        qsl_replay::replay_operator_parity(sent.wire.clone(), sent.claim.clone(), smaller);
    assert!(matches!(
        completed.settlement(&changed, Some(&changed_report)),
        Err(ScalarBuildError::SentContext)
    ));
    assert!(matches!(
        completed.settlement(&sent, None),
        Err(ScalarBuildError::Report)
    ));
}

/// Counts, in every match arm pattern at any depth, a `_` or a bare binding (a catch-all).
fn catch_all_patterns(pattern: &syn::Pat) -> usize {
    use syn::Pat;
    match pattern {
        Pat::Wild(_) => 1,
        Pat::Ident(ident) => match &ident.subpat {
            Some((_, inner)) => catch_all_patterns(inner),
            None => 1,
        },
        Pat::Or(or) => or.cases.iter().map(catch_all_patterns).sum(),
        Pat::Paren(paren) => catch_all_patterns(&paren.pat),
        Pat::Reference(reference) => catch_all_patterns(&reference.pat),
        Pat::Tuple(tuple) => tuple.elems.iter().map(catch_all_patterns).sum(),
        _ => 0,
    }
}

/// Top-level arms only: a `_` inside a variant's fields (`Refused(_)`) is not a catch-all arm.
fn arm_is_catch_all(pattern: &syn::Pat) -> bool {
    use syn::Pat;
    match pattern {
        Pat::Wild(_) => true,
        Pat::Ident(ident) => ident.subpat.is_none(),
        Pat::Or(or) => or.cases.iter().any(arm_is_catch_all),
        Pat::Paren(paren) => arm_is_catch_all(&paren.pat),
        Pat::Reference(reference) => arm_is_catch_all(&reference.pat),
        _ => false,
    }
}

fn denies_wildcards(function: &syn::ItemFn) -> bool {
    function.attrs.iter().any(|attribute| {
        let text = quote_tokens(attribute);
        text.contains("deny") && text.contains("wildcard_enum_match_arm")
    })
}

#[trace("TC-047", "FR-032-AC-21", "FR-032-AC-24")]
#[test]
fn step_10_terminal_map_is_an_exhaustive_match_returning_the_qsl_terminal_value() {
    use syn::{visit::Visit, Expr, ExprMatch, ExprMethodCall, Item};
    #[derive(Default)]
    struct Scan {
        matches: usize,
        catch_alls: usize,
        terminal_calls: usize,
    }
    impl<'a> Visit<'a> for Scan {
        fn visit_expr_match(&mut self, node: &'a ExprMatch) {
            self.matches += 1;
            self.catch_alls += node
                .arms
                .iter()
                .filter(|arm| arm_is_catch_all(&arm.pat))
                .count();
            syn::visit::visit_expr_match(self, node);
        }
        fn visit_expr_method_call(&mut self, node: &'a ExprMethodCall) {
            if node.method == "terminal_value"
                && matches!(&*node.receiver, Expr::Path(path) if path.path.is_ident("result"))
            {
                self.terminal_calls += 1;
            }
            syn::visit::visit_expr_method_call(self, node);
        }
    }
    // The inspection must be able to fail: a catch-all binding or an or-pattern `_` counts.
    let bare: syn::Pat = syn::parse_str::<syn::Arm>("other => 1").unwrap().pat;
    let or: syn::Pat = syn::parse_str::<syn::Arm>("A | _ => 1").unwrap().pat;
    let field: syn::Pat = syn::parse_str::<syn::Arm>("A(_) => 1").unwrap().pat;
    assert!(arm_is_catch_all(&bare) && arm_is_catch_all(&or) && !arm_is_catch_all(&field));
    assert_eq!(catch_all_patterns(&field), 0);

    let root = env!("CARGO_MANIFEST_DIR");
    let source = std::fs::read_to_string(format!("{root}/src/kani/terminal.rs")).unwrap();
    let file = syn::parse_file(&source).unwrap();
    let function = file
        .items
        .iter()
        .find_map(|item| match item {
            Item::Fn(function) if function.sig.ident == "scalar_settled" => Some(function),
            _ => None,
        })
        .expect("the scalar terminal map");
    assert!(
        denies_wildcards(function),
        "deny(clippy::wildcard_enum_match_arm) is on the map"
    );
    let mut scan = Scan::default();
    scan.visit_block(&function.block);
    assert_eq!(scan.matches, 2, "the settlement match and the result match");
    assert_eq!(scan.catch_alls, 0, "no wildcard or catch-all arm");
    assert_eq!(
        scan.terminal_calls, 6,
        "one `result.terminal_value()` arm per class"
    );
    let classes: Vec<_> = [
        "Diverged",
        "Agrees",
        "GeneratedFault",
        "RefusedInput",
        "Incomplete",
        "Refused",
    ]
    .iter()
    .filter(|name| source.contains(&format!("OperatorParityResult::{name}")))
    .collect();
    assert_eq!(classes.len(), 6);

    // `setup_settlement` is the AC-24 exhaustive match: same lint, no catch-all arm at any depth.
    let builder = std::fs::read_to_string(format!("{root}/src/replay/scalar_builder.rs")).unwrap();
    let file = syn::parse_file(&builder).unwrap();
    let method = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Impl(implementation) => Some(implementation),
            _ => None,
        })
        .flat_map(|implementation| &implementation.items)
        .find_map(|item| match item {
            syn::ImplItem::Fn(method) if method.sig.ident == "setup_settlement" => Some(method),
            _ => None,
        })
        .expect("setup_settlement");
    assert!(method.attrs.iter().any(|attribute| {
        let text = quote_tokens(attribute);
        text.contains("deny") && text.contains("wildcard_enum_match_arm")
    }));
    let mut scan = Scan::default();
    scan.visit_block(&method.block);
    assert_eq!(scan.matches, 2, "the error match and the authority match");
    assert_eq!(scan.catch_alls, 0, "no catch-all arm in setup_settlement");
}

fn quote_tokens(attribute: &syn::Attribute) -> String {
    match &attribute.meta {
        syn::Meta::List(list) => format!(
            "{} {}",
            attribute
                .path()
                .get_ident()
                .map(ToString::to_string)
                .unwrap_or_default(),
            list.tokens
        ),
        syn::Meta::Path(_) | syn::Meta::NameValue(_) => String::new(),
    }
}

#[trace("TC-047", "FR-032-AC-22")]
#[test]
fn step_11_pairing_requires_a_falsified_run_and_never_refutes() {
    let fixture = sum_fixture();
    let reports = reachable_reports(&fixture);
    let mut values = Vec::new();
    for (label, completed, sent, report) in &reports {
        let settlement = completed.settlement(sent, Some(report)).unwrap();
        let by_run = run_pair(Some(settlement)).unwrap();
        let settlement = completed.settlement(sent, Some(report)).unwrap();
        let by_ir = ir_pair(Some(settlement)).unwrap();
        assert_eq!(by_run, by_ir, "{label}");
        values.push(by_run);
    }
    for value in &values {
        assert!(
            !matches!(
                value,
                TerminalValue::Refuted
                    | TerminalValue::Proved { .. }
                    | TerminalValue::Tested
                    | TerminalValue::Inconclusive(InconclusiveCause::ReplayParity(_))
            ),
            "{value:?}"
        );
    }
    let report = &reports[0].3;
    let settlements = || {
        [
            ScalarSettlement::Parity(report.result()),
            ScalarSettlement::CgDefect,
        ]
    };
    // Every run outcome except Falsified refuses a settlement and keeps the ordinary value
    // without one.
    let non_falsified = [
        KaniRunOutcome::Verified,
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: 1,
        },
        KaniRunOutcome::Cancelled {
            source: KaniCancellationSource::Requested,
        },
        KaniRunOutcome::Cancelled {
            source: KaniCancellationSource::Deadline,
        },
    ]
    .into_iter()
    .chain(
        [
            KaniInconclusiveReason::FailedWithoutCounterexample,
            KaniInconclusiveReason::NoVerdict,
            KaniInconclusiveReason::MissingCoverSummary,
            KaniInconclusiveReason::VacuousProof,
            KaniInconclusiveReason::UnwindBoundExhausted,
            KaniInconclusiveReason::TimedOut,
            KaniInconclusiveReason::MemoryExhausted,
            KaniInconclusiveReason::WitnessDisagreed,
        ]
        .map(|reason| KaniRunOutcome::Inconclusive { reason }),
    );
    for outcome in non_falsified {
        let evidence = synthetic_evidence(outcome.clone(), 3);
        for settlement in settlements() {
            assert_eq!(
                scalar_run_terminal_value(&evidence, Some(settlement)),
                Err(TerminalPairError::UnexpectedSettlement),
                "{outcome:?}"
            );
        }
        assert_eq!(
            scalar_run_terminal_value(&evidence, None),
            run_terminal_value(&evidence, None),
            "{outcome:?}"
        );
    }
    // Every IR kind except Counterexample likewise.
    let kinds = [
        (
            KaniOutcomeKind::Refused,
            Std001Code::KANI_CAPABILITY_MISSING,
        ),
        (
            KaniOutcomeKind::InvalidInput,
            Std001Code::KANI_IDENTITY_INVALID,
        ),
        (
            KaniOutcomeKind::IncompleteInput,
            Std001Code::KANI_POPULATION_INCOMPLETE,
        ),
        (
            KaniOutcomeKind::Unavailable,
            Std001Code::KANI_CHECKER_ABSENT,
        ),
        (KaniOutcomeKind::TimedOut, Std001Code::KANI_BOUND_EXHAUSTED),
        (
            KaniOutcomeKind::ResourceExhausted,
            Std001Code::KANI_BOUND_EXHAUSTED,
        ),
        (
            KaniOutcomeKind::Cancelled {
                source: KaniCancellationSource::Requested,
            },
            Std001Code::KANI_BOUND_EXHAUSTED,
        ),
        (
            KaniOutcomeKind::Cancelled {
                source: KaniCancellationSource::Deadline,
            },
            Std001Code::KANI_BOUND_EXHAUSTED,
        ),
        (
            KaniOutcomeKind::Inconclusive,
            Std001Code::KANI_VACUOUS_PROOF,
        ),
    ];
    let mut outcomes = vec![KaniOutcome::proved(1, "source", "context").unwrap()];
    for (kind, code) in kinds {
        outcomes.push(KaniOutcome::non_success(kind, code, "source", "context").unwrap());
    }
    for outcome in &outcomes {
        for settlement in settlements() {
            assert_eq!(
                scalar_ir_outcome_terminal_value(outcome, Some(settlement)),
                Err(TerminalPairError::UnexpectedSettlement),
                "{:?}",
                outcome.kind()
            );
        }
        assert_eq!(
            scalar_ir_outcome_terminal_value(outcome, None),
            quire_contract_codegen::ir_outcome_terminal_value(outcome, None),
            "{:?}",
            outcome.kind()
        );
    }
    // A Counterexample reads the settlement: CgDefect is Failed from both entry points.
    assert_eq!(
        run_pair(Some(ScalarSettlement::CgDefect)),
        Ok(TerminalValue::Failed)
    );
    assert_eq!(
        ir_pair(Some(ScalarSettlement::CgDefect)),
        Ok(TerminalValue::Failed)
    );
}

#[trace("TC-047", "FR-032-AC-23")]
#[test]
fn step_12_an_omitted_or_refused_settlement_leaves_no_terminal_value() {
    assert_eq!(run_pair(None), Err(TerminalPairError::MissingSettlement));
    assert_eq!(ir_pair(None), Err(TerminalPairError::MissingSettlement));

    let fixture = sum_fixture();
    let (completed, sent) = completed_sum(&fixture);
    let absent = completed.settlement(&sent, None).ok();
    assert!(absent.is_none());
    assert_eq!(run_pair(absent), Err(TerminalPairError::MissingSettlement));
    let absent = completed.settlement(&sent, None).ok();
    assert_eq!(ir_pair(absent), Err(TerminalPairError::MissingSettlement));

    // Another run's report: a genuine report over a claim with a different native outcome.
    let mut other = sent.clone_for_test();
    other.claim.generated = NativeOutcome::Completed(Integer::from(7_i64));
    let other_report = qsl_replay::replay_operator_parity(
        other.wire.clone(),
        other.claim.clone(),
        other.replay_limits,
    );
    assert!(matches!(
        other_report.result(),
        OperatorParityResult::Diverged { .. }
    ));
    let other_run = completed.settlement(&other, Some(&other_report)).ok();
    assert!(other_run.is_none());
    assert_eq!(
        run_pair(other_run),
        Err(TerminalPairError::MissingSettlement)
    );
    let other_run = completed.settlement(&other, Some(&other_report)).ok();
    assert_eq!(
        ir_pair(other_run),
        Err(TerminalPairError::MissingSettlement)
    );
}

trait CloneForTest {
    fn clone_for_test(&self) -> Self;
}
impl CloneForTest for ScalarSentRequest {
    fn clone_for_test(&self) -> Self {
        Self {
            wire: self.wire.clone(),
            claim: self.claim.clone(),
            replay_limits: self.replay_limits,
        }
    }
}

/// The classification the spec states, written as an exhaustive `match` of the test's own, so a
/// new `ScalarBuildError` or `ScalarAuthorityError` variant fails the build here as well.
fn expected_cg_defect(error: &ScalarBuildError) -> bool {
    match error {
        ScalarBuildError::Authority(cause) => match cause {
            ScalarAuthorityError::Operation
            | ScalarAuthorityError::Ranges
            | ScalarAuthorityError::Record => true,
            ScalarAuthorityError::Unavailable
            | ScalarAuthorityError::Occurrence
            | ScalarAuthorityError::Operands(_)
            | ScalarAuthorityError::GeneratedArtifactChanged => false,
        },
        ScalarBuildError::Playback(_)
        | ScalarBuildError::Staging
        | ScalarBuildError::Identity(_) => true,
        ScalarBuildError::BodyMismatch
        | ScalarBuildError::Context(_)
        | ScalarBuildError::Artifacts
        | ScalarBuildError::Operation
        | ScalarBuildError::Observation
        | ScalarBuildError::Report
        | ScalarBuildError::SentContext => false,
    }
}

fn assert_classified(label: &str, error: &ScalarBuildError) {
    let settlement = error.setup_settlement();
    assert_eq!(
        settlement.is_some(),
        expected_cg_defect(error),
        "{label}: {error:?}"
    );
    match settlement {
        Some(ScalarSettlement::CgDefect) => {
            assert_eq!(
                run_pair(error.setup_settlement()),
                Ok(TerminalValue::Failed),
                "{label}"
            );
            assert_eq!(
                ir_pair(error.setup_settlement()),
                Ok(TerminalValue::Failed),
                "{label}"
            );
        }
        Some(ScalarSettlement::Parity(_)) => {
            panic!("{label}: a setup defect is never a parity result")
        }
        None => {
            assert_eq!(
                run_pair(error.setup_settlement()),
                Err(TerminalPairError::MissingSettlement),
                "{label}"
            );
            assert_eq!(
                ir_pair(error.setup_settlement()),
                Err(TerminalPairError::MissingSettlement),
                "{label}"
            );
        }
    }
}

fn small_input_context(fixture: &ReplayFixture) -> OriginalScalarContext {
    let source = LockedSource {
        authority: "test".to_owned(),
        identity: "scalar-playback.native".to_owned(),
        bytes: fixture.source.as_bytes().to_vec(),
    };
    OriginalScalarContext::new(
        fixture.checked.clone(),
        fixture.raw_package.clone(),
        &source,
        ReplayInputs {
            decode_limits: WitnessDecodeLimits {
                bytes: SCALAR_PLAYBACK_BYTES,
                values: 2,
            },
            package_name: "scalar-playback".to_owned(),
            source: source.clone(),
            dependencies: vec![],
            accounting_limits: fixture.harness.original_scalar_limits().unwrap(),
            stage_limits: BTreeMap::new(),
            declared_domains: vec![],
            replay_limits: ReplayLimits::default().with_input_bytes(1),
        },
        "subject",
        CheckedPackageReadLimits::bounded(),
    )
    .expect("a small input budget is a plan-time refusal")
}

#[trace("TC-047", "FR-032-AC-24")]
#[test]
fn step_13_each_reachable_setup_error_is_classified_and_pairs_as_specified() {
    let fixture = sum_fixture();
    let body = fixture.harness.scalar_replay_body().unwrap();
    let ok_transcript = transcript(&fixture.harness.identity, &[2, 4]);

    // Playback: a four-byte operand.
    let narrow = ok_transcript.replacen("[2, 0, 0, 0, 0, 0, 0, 0]", "[2, 0, 0, 0]", 1);
    assert_ne!(narrow, ok_transcript);
    let playback = context(&fixture)
        .plan(
            &fixture.harness,
            &fixture.harness.identity,
            body,
            &fixture.artifacts,
            &narrow,
        )
        .unwrap_err();
    assert!(
        matches!(playback, ScalarBuildError::Playback(_)),
        "{playback:?}"
    );
    assert!(matches!(
        playback.setup_settlement(),
        Some(ScalarSettlement::CgDefect)
    ));
    assert_classified("playback", &playback);

    // BodyMismatch: a changed retained body member.
    let mut changed = body.clone();
    changed.operands[0].range.upper -= 1;
    let mismatch = context(&fixture)
        .plan(
            &fixture.harness,
            &fixture.harness.identity,
            &changed,
            &fixture.artifacts,
            &ok_transcript,
        )
        .unwrap_err();
    assert!(matches!(mismatch, ScalarBuildError::BodyMismatch));
    assert!(mismatch.setup_settlement().is_none());
    assert_classified("body mismatch", &mismatch);

    // Artifacts: changed generated artifacts.
    let mut artifacts = fixture.artifacts.clone();
    artifacts[0].contents.push(' ');
    let changed_artifacts = context(&fixture)
        .plan(
            &fixture.harness,
            &fixture.harness.identity,
            body,
            &artifacts,
            &ok_transcript,
        )
        .unwrap_err();
    assert!(matches!(changed_artifacts, ScalarBuildError::Artifacts));
    assert!(changed_artifacts.setup_settlement().is_none());
    assert_classified("artifacts", &changed_artifacts);

    // Observation: a mutated observation.
    let original = plan(&fixture, &[4, 2]);
    let mutant = observe(&fixture, &original, true);
    let observation = original.complete(mutant).unwrap_err();
    assert!(matches!(observation, ScalarBuildError::Observation));
    assert!(observation.setup_settlement().is_none());
    assert_classified("observation", &observation);

    // Authority, tamper: a changed `harness.rust`.
    let mut tampered = fixture.harness.clone();
    tampered.rust.contents.push(' ');
    let tamper = context(&fixture)
        .plan(
            &tampered,
            &fixture.harness.identity,
            body,
            &fixture.artifacts,
            &ok_transcript,
        )
        .unwrap_err();
    assert!(
        matches!(
            tamper,
            ScalarBuildError::Authority(ScalarAuthorityError::GeneratedArtifactChanged)
        ),
        "{tamper:?}"
    );
    assert!(tamper.setup_settlement().is_none());
    assert_classified("tamper", &tamper);

    // Authority, unavailable capability: the routed Boolean `Implies` scalar harness.
    let (implies, _, _) = boolean_implies_scalar();
    assert!(matches!(
        implies.scalar_replay_body(),
        Err(ScalarAuthorityError::Unavailable)
    ));
    let unavailable = context(&fixture)
        .plan(
            &implies,
            &implies.identity,
            body,
            &fixture.artifacts,
            &ok_transcript,
        )
        .unwrap_err();
    assert!(
        matches!(
            unavailable,
            ScalarBuildError::Authority(ScalarAuthorityError::Unavailable)
        ),
        "{unavailable:?}"
    );
    assert!(
        unavailable.setup_settlement().is_none(),
        "a family with no operator-parity route is not relabelled a CG defect"
    );
    assert_classified("implies", &unavailable);

    // Context: an unknown function name, and a plan under a too-small `input_bytes`.
    let source = LockedSource {
        authority: "test".to_owned(),
        identity: "scalar-playback.native".to_owned(),
        bytes: fixture.source.as_bytes().to_vec(),
    };
    let unknown = OriginalScalarContext::new(
        fixture.checked.clone(),
        fixture.raw_package.clone(),
        &source,
        ReplayInputs {
            decode_limits: WitnessDecodeLimits {
                bytes: SCALAR_PLAYBACK_BYTES,
                values: 2,
            },
            package_name: "scalar-playback".to_owned(),
            source: source.clone(),
            dependencies: vec![],
            accounting_limits: fixture.harness.original_scalar_limits().unwrap(),
            stage_limits: BTreeMap::new(),
            declared_domains: vec![],
            replay_limits: ReplayLimits::default(),
        },
        "no_such_function",
        CheckedPackageReadLimits::bounded(),
    )
    .unwrap_err();
    assert!(
        matches!(unknown, ScalarBuildError::Context(_)),
        "{unknown:?}"
    );
    assert_classified("unknown function", &unknown);
    let too_small = small_input_context(&fixture)
        .plan(
            &fixture.harness,
            &fixture.harness.identity,
            body,
            &fixture.artifacts,
            &ok_transcript,
        )
        .unwrap_err();
    assert!(
        matches!(too_small, ScalarBuildError::Context(_)),
        "{too_small:?}"
    );
    assert!(too_small.setup_settlement().is_none());
    assert_classified("small input_bytes", &too_small);

    // Through `settlement`: a changed budget (SentContext) and another claim's report (Report).
    let (completed, sent) = completed_sum(&fixture);
    let smaller = sent
        .replay_limits
        .with_input_bytes(sent.replay_limits.input_bytes - 1);
    let budget = ScalarSentRequest {
        wire: sent.wire.clone(),
        claim: sent.claim.clone(),
        replay_limits: smaller,
    };
    let budget_report =
        qsl_replay::replay_operator_parity(sent.wire.clone(), sent.claim.clone(), smaller);
    let sent_context = completed
        .settlement(&budget, Some(&budget_report))
        .unwrap_err();
    assert!(matches!(sent_context, ScalarBuildError::SentContext));
    assert_classified("sent context", &sent_context);
    let mut other = sent.clone_for_test();
    other.claim.generated = NativeOutcome::Completed(Integer::from(7_i64));
    let other_report = qsl_replay::replay_operator_parity(
        other.wire.clone(),
        other.claim.clone(),
        other.replay_limits,
    );
    let report = completed
        .settlement(&other, Some(&other_report))
        .unwrap_err();
    assert!(matches!(report, ScalarBuildError::Report));
    assert_classified("report", &report);
}

#[trace("TC-047", "FR-032-AC-24")]
#[test]
fn step_13_causes_no_public_input_reaches_keep_their_classification() {
    let unreachable = [
        ("staging", ScalarBuildError::Staging),
        (
            "identity",
            ScalarBuildError::Identity(IdentityEncodeError::IntegerBeyondEncoder { magnitude: 1 }),
        ),
        ("operation", ScalarBuildError::Operation),
        (
            "authority operands",
            ScalarBuildError::Authority(ScalarAuthorityError::Operands(
                quire_contract_model::CheckedScalarOperandError::UnknownNode,
            )),
        ),
        (
            "authority occurrence",
            ScalarBuildError::Authority(ScalarAuthorityError::Occurrence),
        ),
        (
            "authority operation",
            ScalarBuildError::Authority(ScalarAuthorityError::Operation),
        ),
        (
            "authority ranges",
            ScalarBuildError::Authority(ScalarAuthorityError::Ranges),
        ),
        (
            "authority record",
            ScalarBuildError::Authority(ScalarAuthorityError::Record),
        ),
        (
            "authority unavailable",
            ScalarBuildError::Authority(ScalarAuthorityError::Unavailable),
        ),
        (
            "authority tamper",
            ScalarBuildError::Authority(ScalarAuthorityError::GeneratedArtifactChanged),
        ),
    ];
    for (label, error) in &unreachable {
        assert_classified(label, error);
    }
    let defects: Vec<_> = unreachable
        .iter()
        .filter(|(_, error)| error.setup_settlement().is_some())
        .map(|(label, _)| *label)
        .collect();
    assert_eq!(
        defects,
        [
            "staging",
            "identity",
            "authority operation",
            "authority ranges",
            "authority record"
        ]
    );
}

#[trace("TC-047", "FR-032-AC-25")]
#[test]
fn step_14_the_flow_uses_only_the_scalar_identity_and_the_scan_finds_nothing() {
    // Compile-time half: generation, plan, completion, request, settlement and pairing are
    // called with `ScalarObligationIdentity` (the fixture's `harness.identity`) alone.
    let fixture = sum_fixture();
    let identity: &quire_contract_codegen::ScalarObligationIdentity = &fixture.harness.identity;
    let original = context(&fixture)
        .plan(
            &fixture.harness,
            identity,
            fixture.harness.scalar_replay_body().unwrap(),
            &fixture.artifacts,
            &transcript(identity, &[2, 4]),
        )
        .unwrap();
    let observation = observe(&fixture, &original, false);
    let completed = original.complete(observation).unwrap();
    let (wire, claim, limits) = completed.request();
    let sent = ScalarSentRequest {
        wire: wire.clone(),
        claim: claim.clone(),
        replay_limits: limits,
    };
    let report = qsl_replay::replay_operator_parity(wire, claim, limits);
    let settlement = completed.settlement(&sent, Some(&report)).unwrap();
    assert!(
        scalar_run_terminal_value(&synthetic_evidence(falsified(), 0), Some(settlement)).is_ok()
    );

    // Scan half.
    let root = env!("CARGO_MANIFEST_DIR");
    let builder = std::fs::read_to_string(format!("{root}/src/replay/scalar_builder.rs")).unwrap();
    let lib = std::fs::read_to_string(format!("{root}/src/lib.rs")).unwrap();
    let start = lib
        .find("pub use replay::scalar_builder::{")
        .expect("scalar re-exports");
    let reexports = &lib[start..start + lib[start..].find("};").unwrap()];
    for (name, text) in [
        ("scalar_builder.rs", builder.as_str()),
        ("lib.rs re-exports", reexports),
    ] {
        assert!(!text.contains("KaniObligationIdentity"), "{name}");
        for (at, _) in text.match_indices("ReplayPackage") {
            assert!(
                text[..at].ends_with("qsl_replay::"),
                "{name}: CG's ReplayPackage at byte {at}"
            );
        }
        for (at, _) in text.match_indices("settle") {
            let rest = &text[at..];
            assert!(
                rest.starts_with("settlement") || rest.starts_with("settled"),
                "{name}: a `settle` token at byte {at}"
            );
        }
    }
}

// FR-032-AC-8: the increment and bounded-addition fixtures through the real routed generation,
// plan and native observation. Real-Kani playback is unavailable to this lane, so the witness is
// the decoded transcript of a chosen vector; the relation to the result range is asserted first.

#[trace("TC-047", "FR-032-AC-8")]
#[test]
fn increment_fixture_has_no_out_of_result_range_value_and_agrees_natively() {
    let fixture = replay_fixture(
        "quire.op.integer.add",
        "x: Int[0, 9]",
        "Int[0, 10]",
        "x + 1",
    );
    let identity = &fixture.harness.identity;
    assert_eq!(identity.arguments.len(), 2, "x and the literal singleton");
    assert_eq!(
        identity
            .arguments
            .iter()
            .map(|a| (a.minimum, a.maximum))
            .collect::<Vec<_>>(),
        [(0, 9), (1, 1)]
    );
    // The reachable exact sums come from the generated identity's own argument ranges; the
    // result range comes from the generated harness's checked-domain line.
    let reach_lower: i128 = identity
        .arguments
        .iter()
        .map(|a| i128::from(a.minimum))
        .sum();
    let reach_upper: i128 = identity
        .arguments
        .iter()
        .map(|a| i128::from(a.maximum))
        .sum();
    let line = fixture
        .harness
        .rust
        .contents
        .lines()
        .find(|line| line.contains("let admitted = exact >= i128::from("))
        .expect("the harness states its checked domain");
    let bounds: Vec<i128> = line
        .split("i128::from(")
        .skip(1)
        .map(|part| part.split("_i64)").next().unwrap().parse().unwrap())
        .collect();
    let [result_lower, result_upper] = bounds.as_slice() else {
        panic!("a lower and an upper result bound: {line}")
    };
    assert_eq!((*result_lower, *result_upper), (0, 10));
    assert!(
        *result_lower <= reach_lower && reach_upper <= *result_upper,
        "every reachable exact sum [{reach_lower}, {reach_upper}] lies inside the result range: no refusal case exists"
    );
    let original = plan(&fixture, &[9, 1]);
    let operands = original.operands().to_vec();
    assert_eq!(operands, [9, 1]);
    let observation = observe(&fixture, &original, false);
    assert_eq!(
        observation.outcome,
        NativeOutcome::Completed(Integer::from(10_i64))
    );
    let completed = original.complete(observation).unwrap();
    let sent = sent_of(&completed);
    let report = qsl_replay::replay_operator_parity(
        sent.wire.clone(),
        sent.claim.clone(),
        sent.replay_limits,
    );
    let settlement = completed.settlement(&sent, Some(&report)).unwrap();
    assert!(matches!(
        run_pair(Some(settlement)),
        Ok(TerminalValue::Inconclusive(
            InconclusiveCause::ScalarAgrees(_)
        ))
    ));
    // The arithmetic mutation control: the compiled mutant's observation is refused.
    let mutant_plan = plan(&fixture, &[9, 1]);
    let mutant = observe(&fixture, &mutant_plan, true);
    assert_ne!(
        mutant.outcome,
        NativeOutcome::Completed(Integer::from(10_i64))
    );
    assert!(matches!(
        mutant_plan.complete(mutant),
        Err(ScalarBuildError::Observation)
    ));
}

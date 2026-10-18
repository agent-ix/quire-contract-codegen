//! Private orchestration: reuse bounded report stand-ins without reversing kani layer imports.
use crate::kani::{
    classify::{KaniInconclusiveReason, KaniRunOutcome},
    run::execute::batch_tests::no_verdict_diagnostics::failure_cause,
    terminal::run_terminal_value,
};
use ix_trace_rs::trace;
use qsl_replay::{Certification, IncompleteCause, ProofBasis, TerminalValue};

#[trace(
    "FR-017-AC-37",
    "FR-029-AC-32",
    "FR-029-AC-33",
    "FR-029-AC-34",
    "TC-027",
    "TC-040"
)]
#[test]
fn bounded_single_reports_project_resource_causes_and_keep_other_failures() {
    failure_cause::single_failure_reports(|status, evidence| {
        assert!(
            evidence.checks.is_empty(),
            "fixture {status} has no listed checks"
        );
        let expected = match status {
            "out_of_memory" => TerminalValue::Incomplete(IncompleteCause::ResourceExhausted),
            "timeout" => TerminalValue::Incomplete(IncompleteCause::TimedOut),
            _ => TerminalValue::Failed,
        };
        assert_eq!(
            run_terminal_value(evidence, None),
            Ok(expected.clone()),
            "fixture {status}"
        );
        if status == "out_of_memory" {
            let mut memory_stop = evidence.clone();
            memory_stop.outcome = KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::MemoryExhausted,
            };
            memory_stop.failure_cause = None;
            assert_eq!(run_terminal_value(&memory_stop, None), Ok(expected));
        }
    });
}

#[trace("FR-017-AC-38", "FR-029-AC-32", "TC-027", "TC-040")]
#[test]
fn bounded_failed_check_reports_keep_both_absent_cause_shapes_failed() {
    failure_cause::unattributed_failure_reports(|label, evidence| {
        assert!(
            !evidence.checks.is_empty(),
            "fixture {label} lists a failed assertion"
        );
        assert_eq!(
            run_terminal_value(evidence, None),
            Ok(TerminalValue::Failed),
            "fixture {label}"
        );
    });
}

#[trace("FR-017-AC-39", "FR-029-AC-33", "TC-043", "TC-040")]
#[test]
fn bounded_five_member_batch_keeps_classifications_and_agrees_on_timeouts() {
    failure_cause::batch_failure_reports(|evidence| {
        assert_eq!(
            evidence
                .iter()
                .map(|member| run_terminal_value(member, None).unwrap())
                .collect::<Vec<_>>(),
            [
                TerminalValue::Proved {
                    basis: ProofBasis::Checks {
                        success_checks: evidence[0].success_checks
                    },
                    certification: Certification::Certified
                },
                TerminalValue::Failed,
                TerminalValue::Incomplete(IncompleteCause::TimedOut),
                TerminalValue::Incomplete(IncompleteCause::TimedOut),
                TerminalValue::Failed,
            ]
        );
        assert!(evidence[0].success_checks > 0);
    });
}

#[trace("FR-017-AC-40", "FR-029-AC-32", "FR-029-AC-35", "TC-027", "TC-040")]
#[test]
fn bounded_cause_changes_only_terminal_projection_and_retains_evidence() {
    failure_cause::cause_independent_classification(|status, evidence| {
        let before = evidence.clone();
        let expected = match status {
            "out_of_memory" => TerminalValue::Incomplete(IncompleteCause::ResourceExhausted),
            "exit_code_64" | "cbmc_exploded" => TerminalValue::Failed,
            _ => panic!("unexpected fixture {status}"),
        };
        assert_eq!(
            run_terminal_value(evidence, None),
            Ok(expected),
            "fixture {status}"
        );
        assert_eq!(evidence, &before);
    });
}

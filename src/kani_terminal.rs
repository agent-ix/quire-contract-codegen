//! The map from a Kani run to QSL's FR-331 terminal value (FR-029, ADR-013 C-09).
//!
//! [`terminal_value`] is the one function that does it: one `match` over [`KaniRunOutcome`] and
//! [`KaniInconclusiveReason`] with no wildcard arm, so a variant added to either type does not
//! compile until it is placed in the O-16 proof column. The terminal value type is QSL's
//! (`qsl_replay::TerminalValue`); this crate defines none of its own.

use qsl_replay::{IncompleteCause, TerminalValue};

use crate::kani_execution::{KaniInconclusiveReason, KaniRunOutcome};

/// The terminal value an executed run settles as.
///
/// `success_checks` is the run's SUCCESS-check count ([`crate::KaniExecutionEvidence::success_checks`]).
/// It is carried on every `Proved` so QSL reads zero as a vacuous proof. No outcome maps to
/// `Tested`: a Kani run is a proof or it is not.
pub fn terminal_value(outcome: &KaniRunOutcome, success_checks: u32) -> TerminalValue {
    match outcome {
        KaniRunOutcome::Verified => TerminalValue::Proved { success_checks },
        KaniRunOutcome::CoverUnsatisfied { .. } => TerminalValue::Proved { success_checks: 0 },
        KaniRunOutcome::Falsified { .. } => TerminalValue::Refuted,
        KaniRunOutcome::Inconclusive { reason } => match reason {
            KaniInconclusiveReason::VacuousProof => TerminalValue::Proved { success_checks: 0 },
            KaniInconclusiveReason::TimedOut => {
                TerminalValue::Incomplete(IncompleteCause::TimedOut)
            }
            KaniInconclusiveReason::UnwindBoundExhausted => {
                TerminalValue::Incomplete(IncompleteCause::ResourceExhausted)
            }
            KaniInconclusiveReason::NoVerdict
            | KaniInconclusiveReason::FailedWithoutCounterexample
            | KaniInconclusiveReason::MissingCoverSummary => TerminalValue::Failed,
        },
    }
}

#[cfg(test)]
mod tests {
    use qsl_replay::ProofCategory;

    use super::*;

    fn inconclusive(reason: KaniInconclusiveReason) -> KaniRunOutcome {
        KaniRunOutcome::Inconclusive { reason }
    }

    /// Trace: FR-029-AC-1, TC-040
    #[test]
    fn tc_040_verified_keeps_its_check_count_and_falsified_is_refuted() {
        let proved = terminal_value(&KaniRunOutcome::Verified, 3);
        assert_eq!(proved, TerminalValue::Proved { success_checks: 3 });
        assert_eq!(proved.category(), ProofCategory::Success);
        let refuted = terminal_value(
            &KaniRunOutcome::Falsified {
                counterexample: "t".to_owned(),
            },
            3,
        );
        assert_eq!(refuted, TerminalValue::Refuted);
        assert_eq!(refuted.category(), ProofCategory::Violation);
    }

    /// A vacuous proof and a cover-unsatisfied run both map to a zero-check proof whatever count
    /// the evidence carried, so QSL reads them as inconclusive with the vacuity cause.
    ///
    /// Trace: FR-029-AC-2, TC-040
    #[test]
    fn tc_040_vacuous_and_cover_unsatisfied_runs_map_to_a_zero_check_proof() {
        for outcome in [
            inconclusive(KaniInconclusiveReason::VacuousProof),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1,
            },
        ] {
            let value = terminal_value(&outcome, 7);
            assert_eq!(
                value,
                TerminalValue::Proved { success_checks: 0 },
                "{outcome:?}"
            );
            assert_eq!(value.category(), ProofCategory::Inconclusive);
            assert!(value.vacuous_proof_cause().is_some());
        }
    }

    /// Trace: FR-029-AC-3, TC-040
    #[test]
    fn tc_040_timeout_and_exhausted_bound_are_incomplete_with_their_cause() {
        assert_eq!(
            terminal_value(&inconclusive(KaniInconclusiveReason::TimedOut), 0),
            TerminalValue::Incomplete(IncompleteCause::TimedOut)
        );
        assert_eq!(
            terminal_value(
                &inconclusive(KaniInconclusiveReason::UnwindBoundExhausted),
                0
            ),
            TerminalValue::Incomplete(IncompleteCause::ResourceExhausted)
        );
        assert_eq!(
            terminal_value(&inconclusive(KaniInconclusiveReason::TimedOut), 0).category(),
            ProofCategory::Incomplete
        );
    }

    /// Trace: FR-029-AC-4, FR-029-AC-5, TC-040
    #[test]
    fn tc_040_a_run_the_tool_did_not_complete_is_a_tool_failure() {
        for reason in [
            KaniInconclusiveReason::NoVerdict,
            KaniInconclusiveReason::FailedWithoutCounterexample,
            KaniInconclusiveReason::MissingCoverSummary,
        ] {
            let value = terminal_value(&inconclusive(reason), 5);
            assert_eq!(value, TerminalValue::Failed, "{reason:?}");
            assert_eq!(value.category(), ProofCategory::InternalFailure);
        }
    }

    /// Trace: FR-029-AC-6, TC-040
    #[test]
    fn tc_040_no_run_outcome_maps_to_tested() {
        let outcomes = [
            KaniRunOutcome::Verified,
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 1,
                total: 2,
            },
            KaniRunOutcome::Falsified {
                counterexample: "t".to_owned(),
            },
            inconclusive(KaniInconclusiveReason::FailedWithoutCounterexample),
            inconclusive(KaniInconclusiveReason::NoVerdict),
            inconclusive(KaniInconclusiveReason::MissingCoverSummary),
            inconclusive(KaniInconclusiveReason::VacuousProof),
            inconclusive(KaniInconclusiveReason::UnwindBoundExhausted),
            inconclusive(KaniInconclusiveReason::TimedOut),
        ];
        for outcome in &outcomes {
            assert_ne!(
                terminal_value(outcome, 1),
                TerminalValue::Tested,
                "{outcome:?}"
            );
        }
    }
}

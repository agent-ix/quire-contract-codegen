//! The maps from a Kani outcome to QSL's FR-331 terminal value (FR-029, FR-030, ADR-013 C-09).
//!
//! [`terminal_value`] maps a run this crate executed: one `match` over [`KaniRunOutcome`] and
//! [`KaniInconclusiveReason`] with no wildcard arm. [`ir_outcome_terminal_value`] maps an outcome
//! Contract IR's Kani boundary produced: one `match` over `KaniOutcomeKind` with no wildcard arm.
//! A variant added to any of those types does not compile until it is placed in the O-16 proof
//! column. The terminal value type is QSL's (`qsl_replay::TerminalValue`); this crate defines
//! none of its own.

use qsl_replay::{IncompleteCause, ProofRefusalCause, TerminalValue, UnavailabilityCause};
use quire_contract_ir::kani::{KaniOutcome, KaniOutcomeKind};

/// The cause code Contract IR gives an `Unavailable` outcome whose solver is absent.
const SOLVER_ABSENT: &str = "kani_solver_absent";
/// The cause code Contract IR gives an `Inconclusive` outcome that is a vacuous proof.
const VACUOUS_PROOF: &str = "kani_vacuous_proof";

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

/// The terminal value of an outcome Contract IR produced and handed in, for a run this crate did
/// not execute (FR-030); an outcome derived from a run this crate executed goes through
/// [`terminal_value`].
///
/// `success_checks` is the SUCCESS-check count of the transcript the `Proved` outcome was read
/// from, which the IR outcome does not carry. The outcome's `code` is read only for `Unavailable`
/// and `Inconclusive`; `source_id` and `context` are never read. The kinds QSL collapses into one
/// variant stay distinguishable through that variant's typed cause.
pub fn ir_outcome_terminal_value(outcome: &KaniOutcome, success_checks: u32) -> TerminalValue {
    match outcome.kind {
        KaniOutcomeKind::Proved => TerminalValue::Proved { success_checks },
        KaniOutcomeKind::Counterexample => TerminalValue::Refuted,
        KaniOutcomeKind::Refused => TerminalValue::Declined(ProofRefusalCause::Refused),
        KaniOutcomeKind::InvalidInput => TerminalValue::Declined(ProofRefusalCause::InvalidInput),
        KaniOutcomeKind::IncompleteInput => {
            TerminalValue::Declined(ProofRefusalCause::IncompleteInput)
        }
        KaniOutcomeKind::Unavailable => {
            TerminalValue::Unsupported(if outcome.code == SOLVER_ABSENT {
                UnavailabilityCause::SolverAbsent
            } else {
                UnavailabilityCause::BackendAbsent
            })
        }
        KaniOutcomeKind::TimedOut => TerminalValue::Incomplete(IncompleteCause::TimedOut),
        KaniOutcomeKind::ResourceExhausted => {
            TerminalValue::Incomplete(IncompleteCause::ResourceExhausted)
        }
        KaniOutcomeKind::Cancelled => TerminalValue::Incomplete(IncompleteCause::Cancelled),
        KaniOutcomeKind::Inconclusive if outcome.code == VACUOUS_PROOF => {
            TerminalValue::Proved { success_checks: 0 }
        }
        KaniOutcomeKind::Inconclusive => TerminalValue::Failed,
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

    fn ir(kind: KaniOutcomeKind, code: &str) -> KaniOutcome {
        KaniOutcome::non_success(kind, code, "item", "context")
    }

    const ALL_KINDS: [KaniOutcomeKind; 10] = [
        KaniOutcomeKind::Proved,
        KaniOutcomeKind::Counterexample,
        KaniOutcomeKind::Refused,
        KaniOutcomeKind::InvalidInput,
        KaniOutcomeKind::IncompleteInput,
        KaniOutcomeKind::Unavailable,
        KaniOutcomeKind::TimedOut,
        KaniOutcomeKind::ResourceExhausted,
        KaniOutcomeKind::Cancelled,
        KaniOutcomeKind::Inconclusive,
    ];

    /// `KaniOutcome::non_success` refuses to build the two success kinds, so those are built with
    /// their own constructors; every other kind is built directly.
    fn outcome(kind: KaniOutcomeKind, code: &str) -> KaniOutcome {
        match kind {
            KaniOutcomeKind::Proved => KaniOutcome::proved("item", "context"),
            KaniOutcomeKind::Counterexample => KaniOutcome::counterexample("item", "context"),
            other => ir(other, code),
        }
    }

    /// Trace: FR-030-AC-1, FR-030-AC-6, TC-041
    #[test]
    fn tc_041_every_ir_outcome_kind_maps_and_none_is_tested() {
        for kind in ALL_KINDS {
            let value = ir_outcome_terminal_value(&outcome(kind.clone(), "cause"), 3);
            assert_ne!(value, TerminalValue::Tested, "{kind:?}");
        }
    }

    /// Trace: FR-030-AC-2, FR-030-AC-3, TC-041
    #[test]
    fn tc_041_refusal_and_limit_kinds_keep_their_own_cause() {
        let map = |kind| ir_outcome_terminal_value(&outcome(kind, "cause"), 0);
        assert_eq!(
            [
                map(KaniOutcomeKind::Refused),
                map(KaniOutcomeKind::InvalidInput),
                map(KaniOutcomeKind::IncompleteInput),
            ],
            [
                TerminalValue::Declined(ProofRefusalCause::Refused),
                TerminalValue::Declined(ProofRefusalCause::InvalidInput),
                TerminalValue::Declined(ProofRefusalCause::IncompleteInput),
            ]
        );
        assert_eq!(
            [
                map(KaniOutcomeKind::TimedOut),
                map(KaniOutcomeKind::ResourceExhausted),
                map(KaniOutcomeKind::Cancelled),
            ],
            [
                TerminalValue::Incomplete(IncompleteCause::TimedOut),
                TerminalValue::Incomplete(IncompleteCause::ResourceExhausted),
                TerminalValue::Incomplete(IncompleteCause::Cancelled),
            ]
        );
    }

    /// Trace: FR-030-AC-4, FR-030-AC-5, TC-041
    #[test]
    fn tc_041_proof_counterexample_and_inconclusive_outcomes_map_by_count_and_cause() {
        let proved = outcome(KaniOutcomeKind::Proved, "");
        assert_eq!(
            ir_outcome_terminal_value(&proved, 3),
            TerminalValue::Proved { success_checks: 3 }
        );
        let vacuous = ir_outcome_terminal_value(&proved, 0);
        assert_eq!(vacuous, TerminalValue::Proved { success_checks: 0 });
        assert_eq!(vacuous.category(), ProofCategory::Inconclusive);
        assert_eq!(
            ir_outcome_terminal_value(&outcome(KaniOutcomeKind::Counterexample, ""), 3),
            TerminalValue::Refuted
        );
        assert_eq!(
            ir_outcome_terminal_value(&ir(KaniOutcomeKind::Inconclusive, "kani_vacuous_proof"), 9),
            TerminalValue::Proved { success_checks: 0 }
        );
        assert_eq!(
            ir_outcome_terminal_value(&ir(KaniOutcomeKind::Inconclusive, "kani_no_verdict"), 9),
            TerminalValue::Failed
        );
    }

    /// Trace: FR-030-AC-8, TC-041
    #[test]
    fn tc_041_an_unavailable_outcome_selects_its_cause_from_the_code() {
        let map = |code| ir_outcome_terminal_value(&ir(KaniOutcomeKind::Unavailable, code), 0);
        assert_eq!(
            map("kani_solver_absent"),
            TerminalValue::Unsupported(UnavailabilityCause::SolverAbsent)
        );
        assert_eq!(
            map("kani_backend_absent"),
            TerminalValue::Unsupported(UnavailabilityCause::BackendAbsent)
        );
        assert_eq!(
            map("anything_else"),
            TerminalValue::Unsupported(UnavailabilityCause::BackendAbsent)
        );
    }
}

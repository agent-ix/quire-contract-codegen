//! The terminal-value maps of a Kani run (FR-029) and of a Contract IR outcome (FR-030): one
//! total `match` each, from [`KaniExecutionEvidence`] or an IR [`KaniOutcome`], paired with the
//! settlement of its counterexample's replay, to QSL's FR-331 terminal value.
//!
//! The map is total over the pair, not over the outcome alone: a falsified run is a refutation
//! only once its replay reproduces it. This module defines the replay settlement as the typed
//! input the map reads, built from `qsl-replay` types only; `replay/` owns the conversion from
//! its own errors and from QSL's result into it, so `kani/` imports nothing from `replay/`
//! (AD-004). The value's category is [`TerminalValue::category`] and nothing else (AD-003 E-9):
//! no `proof_category` function exists here.

use qsl_replay::{
    Certification, Code, DeclineCode, DisagreementCause, IncompleteCause, InconclusiveCause,
    ProofBasis, ProofRefusalCause, ReplayRefusal, Std001Code, TerminalValue, UnavailabilityCause,
};
use quire_contract_ir::kani::{
    KaniCancellationSource, KaniOutcome, KaniOutcomeError, KaniOutcomeKind,
};
use quire_exact::CancelCause;

use crate::kani::classify::{KaniInconclusiveReason, KaniRunOutcome};
use crate::kani::output::report::KaniFailureCause;
use crate::kani::run::execute::KaniExecutionEvidence;

/// The settlement of a falsified run's replay, read as one of the six readings FR-029 states.
/// The driver builds it from the replay path's result or error; [`run_terminal_value`] reads it.
#[derive(Debug)]
pub enum ReplaySettlement<'a> {
    /// The replay settled `ReproducedWithEvaluatedWitness` in the `violation` category: the
    /// counterexample is a refutation.
    Reproduced,
    /// The replay settled `Inconclusive`, with QSL's typed cause.
    Disagreement(&'a DisagreementCause),
    /// `qsl_replay::replay`, `replay_frame` or `replay_state_clause` returned a refusal. The
    /// refusal is read by QSL's own rule ([`TerminalValue::from_replay_refusal`]): a fault is
    /// `Failed`, any other refusal is `Inconclusive` carrying the refusal's catalog code.
    Refused(&'a ReplayRefusal),
    /// A refusal of the replay setup on data, reached after the run was falsified, that is not a
    /// `ReplayRefusal`: a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`. The code
    /// is the one QSL's refusal value supplied.
    SetupRefused(Code),
    /// An internal fault of QSL's call-site facade (`CallSiteRefusal::Fault`).
    Fault,
    /// A failure this repository raised that carries no QSL catalog code: the replay could not
    /// be built or read, its result cannot be read as a reproduction or a disagreement, the
    /// playback lacks a state field or lies outside its declared range, or the operation has a
    /// shape the state-clause replay does not support (FR-029-AC-16). None is a QSL data refusal,
    /// so none is `Incomplete` or `Inconclusive(ReplayRefused)`.
    CgDefect,
}

/// A refused terminal conversion: invalid pairing or rejected IR cancellation construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalPairError {
    /// A falsified outcome with no replay settlement: nothing says whether the counterexample
    /// reproduces.
    MissingSettlement,
    /// An outcome other than falsified with a replay settlement: only a counterexample is
    /// replayed.
    UnexpectedSettlement,
    /// IR's nonzero proof count exceeds QSL's `u32` terminal count.
    ProvedCountOutOfRange {
        /// The retained count that cannot fit QSL's terminal value.
        success_checks: usize,
    },
    /// The owning IR constructor refused cancellation execution evidence.
    InvalidCancellation(KaniOutcomeError),
}

impl std::fmt::Display for TerminalPairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSettlement => {
                f.write_str("a falsified run needs the settlement of its counterexample's replay")
            }
            Self::InvalidCancellation(error) => std::fmt::Display::fmt(error, f),
            Self::UnexpectedSettlement => {
                f.write_str("only a falsified run carries a replay settlement")
            }
            Self::ProvedCountOutOfRange { success_checks } => {
                write!(
                    f,
                    "IR proof check count {success_checks} exceeds QSL's u32 limit"
                )
            }
        }
    }
}

impl std::error::Error for TerminalPairError {}

/// Maps one run's evidence and, for a falsified outcome, its replay settlement to QSL's
/// terminal value (FR-029). Outcome, SUCCESS count and CBMC cause come from the same evidence.
/// A resource cause affects only failure-without-counterexample; diagnostics are not read.
///
/// A vacuous proof and a cover-unsatisfied run have a zero-check `Checks` basis and
/// `Certified` certification. QSL reads them as category `inconclusive`; no outcome maps to
/// `Tested`.
///
/// # Errors
///
/// [`TerminalPairError`] when a falsified outcome has no settlement, or another outcome has one.
pub fn run_terminal_value(
    evidence: &KaniExecutionEvidence,
    settlement: Option<ReplaySettlement<'_>>,
) -> Result<TerminalValue, TerminalPairError> {
    use KaniInconclusiveReason as Reason;
    use KaniRunOutcome::{Cancelled, CoverUnsatisfied, Falsified, Inconclusive, Verified};
    Ok(match (&evidence.outcome, settlement) {
        (Verified, None) => TerminalValue::Proved {
            basis: ProofBasis::Checks {
                success_checks: evidence.success_checks,
            },
            certification: Certification::Certified,
        },
        (CoverUnsatisfied { .. }, None) => TerminalValue::Proved {
            basis: ProofBasis::Checks { success_checks: 0 },
            certification: Certification::Certified,
        },
        (
            Inconclusive {
                reason: Reason::VacuousProof,
            },
            None,
        ) => TerminalValue::Proved {
            basis: ProofBasis::Checks { success_checks: 0 },
            certification: Certification::Certified,
        },
        (
            Inconclusive {
                reason: Reason::TimedOut,
            },
            None,
        ) => TerminalValue::Incomplete(IncompleteCause::TimedOut),
        (
            Inconclusive {
                reason: Reason::UnwindBoundExhausted | Reason::MemoryExhausted,
            },
            None,
        ) => TerminalValue::Incomplete(IncompleteCause::ResourceExhausted),
        (
            Inconclusive {
                reason: Reason::NoVerdict | Reason::MissingCoverSummary,
            },
            None,
        ) => TerminalValue::Failed,
        (
            Inconclusive {
                reason: Reason::FailedWithoutCounterexample,
            },
            None,
        ) => match evidence.failure_cause.as_ref() {
            Some(KaniFailureCause::OutOfMemory) => {
                TerminalValue::Incomplete(IncompleteCause::ResourceExhausted)
            }
            Some(KaniFailureCause::Timeout) => TerminalValue::Incomplete(IncompleteCause::TimedOut),
            None
            | Some(KaniFailureCause::PropertiesFailed)
            | Some(KaniFailureCause::ExitCode { .. })
            | Some(KaniFailureCause::Unknown { .. }) => TerminalValue::Failed,
        },
        (Cancelled { source }, None) => cancelled_value(*source),
        (Falsified { .. }, Some(settlement)) => settled(settlement),
        (Falsified { .. }, None) => return Err(TerminalPairError::MissingSettlement),
        (Verified | CoverUnsatisfied { .. } | Inconclusive { .. } | Cancelled { .. }, Some(_)) => {
            return Err(TerminalPairError::UnexpectedSettlement)
        }
    })
}

/// Maps one Contract IR [`KaniOutcome`] that arrived from Contract IR, using its retained
/// SUCCESS-check count and, for a `Counterexample`, its replay settlement, to QSL's terminal
/// value (FR-030).
///
/// This is not applied to an outcome derived from a run this repository executed: that run's
/// value is [`run_terminal_value`]'s (FR-030 precedence). The map reads the kind, and the `code`
/// only for `Unavailable` and `Inconclusive`, plus the `Declined` kinds, whose code is carried
/// as the STD-001 code IR issued: [`DeclineCode::Std001`], never respelled as a QSL catalog
/// code and never checked for registration (QSL records no issuer and refuses no unregistered
/// code). It never reads `source_id` or `context`. A vacuous proof has a zero-check `Checks`
/// basis and `Certified` certification; no outcome maps to `Tested`.
///
/// # Errors
///
/// [`TerminalPairError`] when a `Counterexample` has no settlement, another kind has one,
/// or the retained proof count exceeds QSL's `u32` range.
pub fn ir_outcome_terminal_value(
    outcome: &KaniOutcome,
    settlement: Option<ReplaySettlement<'_>>,
) -> Result<TerminalValue, TerminalPairError> {
    use KaniOutcomeKind as Kind;
    let declined = |cause| TerminalValue::Declined {
        cause,
        code: DeclineCode::Std001(outcome.code()),
    };
    Ok(match (&outcome.kind(), settlement) {
        (Kind::Proved, None) => {
            let retained_count = outcome
                .success_checks()
                .expect("IR Proved always retains a nonzero check count");
            let success_checks = u32::try_from(retained_count).map_err(|_| {
                TerminalPairError::ProvedCountOutOfRange {
                    success_checks: retained_count,
                }
            })?;
            TerminalValue::Proved {
                basis: ProofBasis::Checks { success_checks },
                certification: Certification::Certified,
            }
        }
        (Kind::Counterexample, Some(settlement)) => settled(settlement),
        (Kind::Counterexample, None) => return Err(TerminalPairError::MissingSettlement),
        (Kind::Refused, None) => declined(ProofRefusalCause::Refused),
        (Kind::InvalidInput, None) => declined(ProofRefusalCause::InvalidInput),
        (Kind::IncompleteInput, None) => declined(ProofRefusalCause::IncompleteInput),
        (Kind::Unavailable, None) if outcome.code() == Std001Code::KANI_CHECKER_ABSENT => {
            TerminalValue::Unsupported(UnavailabilityCause::SolverAbsent)
        }
        // IR admits `kani_backend_absent` here. The fallback keeps this match exhaustive.
        (Kind::Unavailable, None) => TerminalValue::Unsupported(UnavailabilityCause::BackendAbsent),
        (Kind::TimedOut, None) => TerminalValue::Incomplete(IncompleteCause::TimedOut),
        (Kind::ResourceExhausted, None) => {
            TerminalValue::Incomplete(IncompleteCause::ResourceExhausted)
        }
        (Kind::Cancelled { source }, None) => cancelled_value(*source),
        (Kind::Inconclusive, None) if outcome.code() == Std001Code::KANI_VACUOUS_PROOF => {
            TerminalValue::Proved {
                basis: ProofBasis::Checks { success_checks: 0 },
                certification: Certification::Certified,
            }
        }
        (Kind::Inconclusive, None) => TerminalValue::Failed,
        (
            Kind::Proved
            | Kind::Refused
            | Kind::InvalidInput
            | Kind::IncompleteInput
            | Kind::Unavailable
            | Kind::TimedOut
            | Kind::ResourceExhausted
            | Kind::Cancelled { .. }
            | Kind::Inconclusive,
            Some(_),
        ) => return Err(TerminalPairError::UnexpectedSettlement),
    })
}

/// The value of a falsified run under `settlement`: `Refuted` only for a reproduced replay.
fn settled(settlement: ReplaySettlement<'_>) -> TerminalValue {
    match settlement {
        ReplaySettlement::Reproduced => TerminalValue::Refuted,
        ReplaySettlement::Disagreement(cause) => {
            TerminalValue::Inconclusive(InconclusiveCause::ReplayParity(cause.clone()))
        }
        ReplaySettlement::Refused(refusal) => TerminalValue::from_replay_refusal(refusal),
        ReplaySettlement::SetupRefused(code) => {
            TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(code))
        }
        ReplaySettlement::Fault | ReplaySettlement::CgDefect => TerminalValue::Failed,
    }
}

/// Convert only the source recorded by the owning IR cancellation carrier.
fn cancelled_value(source: KaniCancellationSource) -> TerminalValue {
    let source = match source {
        KaniCancellationSource::Requested => CancelCause::Requested,
        KaniCancellationSource::Deadline => CancelCause::Deadline,
    };
    TerminalValue::Incomplete(IncompleteCause::Cancelled { source })
}

impl KaniExecutionEvidence {
    /// Construct and map the IR cancellation of this settled execution (FR-040).
    ///
    /// Non-cancelled evidence returns `None` and retains FR-029's ordinary map. Both returned
    /// values retain the launcher's stored source; this entry never reads the caller's handle.
    ///
    /// # Errors
    ///
    /// The owning IR constructor's refusal, or the FR-030 pair refusal.
    pub fn cancellation_terminal_value(
        &self,
    ) -> Result<Option<(KaniOutcome, qsl_replay::TerminalValue)>, TerminalPairError> {
        let KaniRunOutcome::Cancelled { source } = self.outcome else {
            return Ok(None);
        };
        let outcome = KaniOutcome::non_success(
            KaniOutcomeKind::Cancelled { source },
            quire_contract_model::std001_code!("kani_cancelled"),
            &self.harness_path,
            &self.launcher_path,
        )
        .map_err(TerminalPairError::InvalidCancellation)?;
        let terminal = ir_outcome_terminal_value(&outcome, None)?;
        Ok(Some((outcome, terminal)))
    }
}

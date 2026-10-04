//! The terminal-value map of a Kani run (FR-029): one total `match` from a [`KaniRunOutcome`],
//! paired with the settlement of its counterexample's replay, to QSL's FR-331 terminal value.
//!
//! The map is total over the pair, not over the outcome alone: a falsified run is a refutation
//! only once its replay reproduces it. This module defines the replay settlement as the typed
//! input the map reads, built from `qsl-replay` types only; `replay/` owns the conversion from
//! its own errors and from QSL's result into it, so `kani/` imports nothing from `replay/`
//! (AD-004). The value's category is [`TerminalValue::category`] and nothing else (AD-003 E-9):
//! no `proof_category` function exists here.

use qsl_replay::{
    Code, DisagreementCause, IncompleteCause, InconclusiveCause, ReplayRefusal, TerminalValue,
};

use crate::kani::classify::{KaniInconclusiveReason, KaniRunOutcome};

/// The settlement of a falsified run's replay, read as one of the six readings FR-029 states.
/// The driver builds it from the replay path's result or error; [`run_terminal_value`] reads it.
#[derive(Debug)]
pub enum ReplaySettlement<'a> {
    /// The replay settled `ReproducedWithEvaluatedWitness` in the `violation` category: the
    /// counterexample is a refutation.
    Reproduced,
    /// The replay settled `Inconclusive`, with QSL's typed cause.
    Disagreement(&'a DisagreementCause),
    /// `qsl_replay::replay` returned a refusal. The refusal is read by QSL's own rule
    /// ([`TerminalValue::from_replay_refusal`]): a fault is `Failed`, any other refusal is
    /// `Inconclusive` carrying the refusal's catalog code.
    Refused(&'a ReplayRefusal),
    /// A refusal of the replay setup on data, reached after the run was falsified, that is not a
    /// `ReplayRefusal`: a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`. The code
    /// is the one QSL's refusal value supplied.
    SetupRefused(Code),
    /// An internal fault of QSL's call-site facade (`CallSiteRefusal::Fault`).
    Fault,
    /// A failure this repository raised that carries no QSL catalog code: the replay could not
    /// be built or read, or its result cannot be read as a reproduction or a disagreement.
    CgDefect,
}

/// A pair the map does not take: [`ReplaySettlement`] accompanies a falsified outcome only.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalPairError {
    /// A falsified outcome with no replay settlement: nothing says whether the counterexample
    /// reproduces.
    MissingSettlement,
    /// An outcome other than falsified with a replay settlement: only a counterexample is
    /// replayed.
    UnexpectedSettlement,
}

impl std::fmt::Display for TerminalPairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSettlement => {
                f.write_str("a falsified run needs the settlement of its counterexample's replay")
            }
            Self::UnexpectedSettlement => {
                f.write_str("only a falsified run carries a replay settlement")
            }
        }
    }
}

impl std::error::Error for TerminalPairError {}

/// Maps one run outcome, with the SUCCESS check count its transcript reports and, for a
/// falsified outcome, its replay settlement, to QSL's terminal value (FR-029).
///
/// A vacuous proof and a cover-unsatisfied run are `Proved { success_checks: 0 }`, which QSL
/// reads as category `inconclusive`; no outcome maps to `Tested`.
///
/// # Errors
///
/// [`TerminalPairError`] when a falsified outcome has no settlement, or another outcome has one.
pub fn run_terminal_value(
    outcome: &KaniRunOutcome,
    success_checks: u32,
    settlement: Option<ReplaySettlement<'_>>,
) -> Result<TerminalValue, TerminalPairError> {
    use KaniInconclusiveReason as Reason;
    use KaniRunOutcome::{CoverUnsatisfied, Falsified, Inconclusive, Verified};
    Ok(match (outcome, settlement) {
        (Verified, None) => TerminalValue::Proved { success_checks },
        (CoverUnsatisfied { .. }, None) => TerminalValue::Proved { success_checks: 0 },
        (
            Inconclusive {
                reason: Reason::VacuousProof,
            },
            None,
        ) => TerminalValue::Proved { success_checks: 0 },
        (
            Inconclusive {
                reason: Reason::TimedOut,
            },
            None,
        ) => TerminalValue::Incomplete(IncompleteCause::TimedOut),
        (
            Inconclusive {
                reason: Reason::UnwindBoundExhausted,
            },
            None,
        ) => TerminalValue::Incomplete(IncompleteCause::ResourceExhausted),
        (
            Inconclusive {
                reason:
                    Reason::NoVerdict
                    | Reason::FailedWithoutCounterexample
                    | Reason::MissingCoverSummary,
            },
            None,
        ) => TerminalValue::Failed,
        (Falsified { .. }, Some(settlement)) => settled(settlement),
        (Falsified { .. }, None) => return Err(TerminalPairError::MissingSettlement),
        (Verified | CoverUnsatisfied { .. } | Inconclusive { .. }, Some(_)) => {
            return Err(TerminalPairError::UnexpectedSettlement)
        }
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

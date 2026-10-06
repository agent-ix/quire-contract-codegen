//! The terminal-value maps of a Kani run (FR-029) and of a Contract IR outcome (FR-030): one
//! total `match` each, from a [`KaniRunOutcome`] or an IR [`KaniOutcome`], paired with the
//! settlement of its counterexample's replay, to QSL's FR-331 terminal value.
//!
//! The map is total over the pair, not over the outcome alone: a falsified run is a refutation
//! only once its replay reproduces it. This module defines the replay settlement as the typed
//! input the map reads, built from `qsl-replay` types only; `replay/` owns the conversion from
//! its own errors and from QSL's result into it, so `kani/` imports nothing from `replay/`
//! (AD-004). The value's category is [`TerminalValue::category`] and nothing else (AD-003 E-9):
//! no `proof_category` function exists here.

use qsl_replay::{
    Code, DeclineCode, DisagreementCause, IncompleteCause, InconclusiveCause, ProofRefusalCause,
    ReplayRefusal, Std001Code, TerminalValue, UnavailabilityCause,
};
use quire_contract_ir::kani::{KaniOutcome, KaniOutcomeKind};

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
                reason: Reason::UnwindBoundExhausted | Reason::MemoryExhausted,
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

/// Maps one Contract IR [`KaniOutcome`] that arrived from Contract IR, with the SUCCESS check
/// count its transcript reports and, for a `Counterexample`, its replay settlement, to QSL's
/// terminal value (FR-030).
///
/// This is not applied to an outcome derived from a run this repository executed: that run's
/// value is [`run_terminal_value`]'s (FR-030 precedence). The map reads the kind, and the `code`
/// only for `Unavailable` and `Inconclusive`, plus the `Declined` kinds, whose code is carried
/// as the STD-001 code IR issued: [`DeclineCode::Std001`], never respelled as a QSL catalog
/// code and never checked for registration (QSL records no issuer and refuses no unregistered
/// code). It never reads `source_id` or `context`. A vacuous proof is `Proved { success_checks:
/// 0 }`; no outcome maps to `Tested`.
///
/// # Errors
///
/// [`TerminalPairError`] when a `Counterexample` has no settlement, or another kind has one.
pub fn ir_outcome_terminal_value(
    outcome: &KaniOutcome,
    success_checks: u32,
    settlement: Option<ReplaySettlement<'_>>,
) -> Result<TerminalValue, TerminalPairError> {
    use KaniOutcomeKind as Kind;
    let declined = |cause| TerminalValue::Declined {
        cause,
        code: DeclineCode::Std001(outcome.code),
    };
    Ok(match (&outcome.kind, settlement) {
        (Kind::Proved, None) => TerminalValue::Proved { success_checks },
        (Kind::Counterexample, Some(settlement)) => settled(settlement),
        (Kind::Counterexample, None) => return Err(TerminalPairError::MissingSettlement),
        (Kind::Refused, None) => declined(ProofRefusalCause::Refused),
        (Kind::InvalidInput, None) => declined(ProofRefusalCause::InvalidInput),
        (Kind::IncompleteInput, None) => declined(ProofRefusalCause::IncompleteInput),
        (Kind::Unavailable, None) if outcome.code == Std001Code::KANI_SOLVER_ABSENT => {
            TerminalValue::Unsupported(UnavailabilityCause::SolverAbsent)
        }
        // `kani_backend_absent` and every other cause: FR-030's table sends both here.
        (Kind::Unavailable, None) => TerminalValue::Unsupported(UnavailabilityCause::BackendAbsent),
        (Kind::TimedOut, None) => TerminalValue::Incomplete(IncompleteCause::TimedOut),
        (Kind::ResourceExhausted, None) => {
            TerminalValue::Incomplete(IncompleteCause::ResourceExhausted)
        }
        (Kind::Cancelled, None) => TerminalValue::Incomplete(IncompleteCause::Cancelled),
        (Kind::Inconclusive, None) if outcome.code == Std001Code::KANI_VACUOUS_PROOF => {
            TerminalValue::Proved { success_checks: 0 }
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
            | Kind::Cancelled
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

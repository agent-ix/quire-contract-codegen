// SPDX-License-Identifier: AGPL-3.0-or-later
//! Dedicated constrained execution and the one typed owner-observation conversion.

use super::*;
use crate::kani::{
    classify::{KaniInconclusiveReason, KaniRunOutcome},
    output::report::{KaniCheckClass, KaniCheckResult, KaniCheckStatus},
    run::{
        execute::{
            execute_constrained_raw, KaniExecutionEvidence, KaniExecutionRefusal,
            KaniExecutionRequest,
        },
        harness::KaniExecutableHarness,
    },
    terminal::ReplaySettlement,
};

/// Reachability of a constraint boundary within the exact retained method bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum ConstraintBoundary {
    /// At least one boundary was reached, without claiming a distinct-state count.
    Reached,
    /// A complete boundary check found none within these bounds.
    NotReached,
    /// A boundary check is absent, malformed, interrupted or undecided.
    Unknown,
}

/// Backend evidence sealed to its originating constrained method and source bindings.
#[derive(Debug, Serialize)]
pub struct ConstrainedBackendEvidence {
    method: ConstrainedMethodIdentity,
    backend: KaniExecutionEvidence,
    ordinary_cover_line: u32,
    boundary_cover_line: Option<u32>,
}
impl ConstrainedBackendEvidence {
    /// The exact method that produced this evidence.
    pub fn method(&self) -> &ConstrainedMethodIdentity {
        &self.method
    }
    /// Original typed backend checks, without an unconditional proof conversion.
    pub fn checks(&self) -> &[KaniCheckResult] {
        &self.backend.checks
    }
    /// Original falsifying playback, when the backend actually retained one.
    pub fn playback(&self) -> Option<&str> {
        if let KaniRunOutcome::Falsified { counterexample } = &self.backend.outcome {
            Some(counterexample)
        } else {
            None
        }
    }
    /// The original typed non-proof reason, retained without inventing a cover result.
    pub fn nonproof_reason(&self) -> Option<KaniInconclusiveReason> {
        if let KaniRunOutcome::Inconclusive { reason } = self.backend.outcome {
            Some(reason)
        } else {
            None
        }
    }
}

/// Run only a sealed constrained harness through the existing bounded launcher.
///
/// # Errors
/// Refuses an ordinary harness, a nonunique source binding or any existing launch/report error.
pub fn execute_constrained_method(
    request: &KaniExecutionRequest<'_>,
) -> Result<ConstrainedBackendEvidence, KaniExecutionRefusal> {
    let KaniExecutableHarness::Constrained(harness) = request.harness else {
        return Err(KaniExecutionRefusal::WrongMethodExecutionRoute);
    };
    let library =
        std::fs::read_to_string(request.crate_directory.join("src/lib.rs")).map_err(|_| {
            KaniExecutionRefusal::HarnessNotInCrate {
                harness_path: harness.rust.path.clone(),
            }
        })?;
    let mut matches = library.match_indices(&harness.rust.contents);
    let (Some((start, _)), None) = (matches.next(), matches.next()) else {
        return Err(KaniExecutionRefusal::HarnessNotInCrate {
            harness_path: harness.rust.path.clone(),
        });
    };
    let offset = u32::try_from(
        library[..start]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count(),
    )
    .map_err(|_| KaniExecutionRefusal::HarnessNotInCrate {
        harness_path: harness.rust.path.clone(),
    })?;
    let line = |value: u32| {
        offset
            .checked_add(value)
            .ok_or_else(|| KaniExecutionRefusal::HarnessNotInCrate {
                harness_path: harness.rust.path.clone(),
            })
    };
    let ordinary_cover_line = line(harness.identity.ordinary_cover_line)?;
    let boundary_cover_line = harness.identity.boundary_cover_line.map(line).transpose()?;
    let backend = execute_constrained_raw(request)?;
    Ok(ConstrainedBackendEvidence {
        method: harness.identity.clone(),
        backend,
        ordinary_cover_line,
        boundary_cover_line,
    })
}

/// Original owner evaluations at the retained observation, bound to one exact method.
#[derive(Debug)]
pub struct ConstrainedOwnerObservation<'a> {
    /// Method whose admitted clauses and snapshot were evaluated.
    pub method: &'a ConstrainedMethodIdentity,
    /// Original QSL/kernel safety evaluation or original call failure.
    pub safety: Result<&'a qsl_replay::CallOutcome, &'a quire_eval::CallFailure>,
    /// Original QSL/kernel constraint evaluation or original call failure.
    pub constraint: Option<Result<&'a qsl_replay::CallOutcome, &'a quire_eval::CallFailure>>,
    /// Exact backend playback when evaluating a counterexample; absent on a nonfalsified run.
    pub playback: Option<&'a str>,
}

/// Method-qualified status; no variant is an unconditional Verified or Proved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum ConstrainedMethodStatus {
    /// A genuine safety violation with retained backend playback and agreeing owner evaluation.
    Falsified,
    /// The original owner evaluation was undefined, never a false constraint.
    EvaluationViolation,
    /// The original owner evaluation was refused.
    Refused,
    /// The original owner evaluation was incomplete.
    Incomplete,
    /// An owner call failed: runtime_invariant / established-invariant-broken.
    RuntimeInvariant,
    /// Safety held but a constraint boundary was reached.
    ConstraintReached,
    /// Complete checking found no boundary within these exact method bounds.
    BoundedSuccess,
    /// The original ordinary non-vacuity cover was not satisfied.
    CoverUnsatisfied,
    /// Cancellation, malformed/incomplete checks or an unknown boundary decides no proof.
    Inconclusive,
}

/// The original evidence and owner outcomes retained together under the selected method.
#[derive(Debug)]
pub struct ConstrainedMethodResult<'a> {
    evidence: &'a ConstrainedBackendEvidence,
    owner: &'a ConstrainedOwnerObservation<'a>,
    status: ConstrainedMethodStatus,
    boundary: Option<ConstraintBoundary>,
}
impl ConstrainedMethodResult<'_> {
    /// Method-qualified classification.
    pub fn status(&self) -> ConstrainedMethodStatus {
        self.status
    }
    /// Reachability, never a fabricated exact boundary count.
    pub fn boundary(&self) -> Option<ConstraintBoundary> {
        self.boundary
    }
    /// Sealed original installed-backend evidence.
    pub fn evidence(&self) -> &ConstrainedBackendEvidence {
        self.evidence
    }
    /// Original typed owner observations, preserving all refusal/incomplete/fault payloads.
    pub fn owner(&self) -> &ConstrainedOwnerObservation<'_> {
        self.owner
    }
}

/// A disagreement in method, original playback, observation or replay binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ConstrainedBindingRefusal {
    /// Evidence belongs to another selected constraint, package or bound request.
    #[error("constrained method identity mismatch")]
    Method,
    /// A counterexample observation does not retain the original playback.
    #[error("constrained playback identity mismatch")]
    Playback,
    /// Owner observations disagree with the backend's claimed classification.
    #[error("owner and backend observations disagree")]
    Observation,
    /// A falsified/undefined result has no replay settlement.
    #[error("missing constrained replay settlement")]
    MissingReplay,
    /// A nonfalsified result carries an unexpected replay settlement.
    #[error("unexpected constrained replay settlement")]
    UnexpectedReplay,
    /// The public QSL terminal API cannot retain this constraint method.
    #[error("the public terminal representation cannot retain the constrained method")]
    UnsupportedTerminalRepresentation,
}

fn cover(evidence: &ConstrainedBackendEvidence, line: u32) -> Option<KaniCheckStatus> {
    let mut matches = evidence.backend.checks.iter().filter(|check| {
        check.class == KaniCheckClass::Cover
            && check.location.line == Some(line)
            && std::path::Path::new(&check.location.file).ends_with("src/lib.rs")
    });
    let found = matches.next()?;
    matches.next().is_none().then_some(found.status)
}

/// Convert only original typed checks and same-method owner observations.
///
/// # Errors
/// Refuses cross-method substitution, a changed playback or conflicting owner evidence.
pub fn classify_constrained_method<'a>(
    evidence: &'a ConstrainedBackendEvidence,
    owner: &'a ConstrainedOwnerObservation<'a>,
) -> Result<ConstrainedMethodResult<'a>, ConstrainedBindingRefusal> {
    use qsl_replay::{CallOutcome as O, CallValue as V};
    use ConstrainedMethodStatus as S;
    if &evidence.method != owner.method {
        return Err(ConstrainedBindingRefusal::Method);
    }
    if evidence.playback() != owner.playback {
        return Err(ConstrainedBindingRefusal::Playback);
    }
    if evidence.method.constraint.is_none() && owner.constraint.is_some() {
        return Err(ConstrainedBindingRefusal::Observation);
    }
    let ordinary = cover(evidence, evidence.ordinary_cover_line);
    let boundary_check = evidence
        .boundary_cover_line
        .and_then(|line| cover(evidence, line));
    let complete = evidence.backend.success_checks > 0
        && evidence
            .backend
            .checks
            .iter()
            .filter(|check| check.class != KaniCheckClass::Cover)
            .all(|check| {
                matches!(
                    check.status,
                    KaniCheckStatus::Success | KaniCheckStatus::Unreachable
                )
            });
    let completed_run = matches!(
        evidence.backend.outcome,
        KaniRunOutcome::Verified | KaniRunOutcome::CoverUnsatisfied { .. }
    );
    let boundary = evidence.method.constraint.as_ref().map(|_| {
        if !completed_run || !complete {
            ConstraintBoundary::Unknown
        } else {
            match boundary_check {
                Some(KaniCheckStatus::Satisfied) => ConstraintBoundary::Reached,
                Some(KaniCheckStatus::Unsatisfiable) => ConstraintBoundary::NotReached,
                _ => ConstraintBoundary::Unknown,
            }
        }
    });
    let status = if owner.safety.is_err() || owner.constraint.is_some_and(|value| value.is_err()) {
        S::RuntimeInvariant
    } else {
        let safety = owner
            .safety
            .map_err(|_| ConstrainedBindingRefusal::Observation)?;
        let constraint = owner
            .constraint
            .map(|value| value.map_err(|_| ConstrainedBindingRefusal::Observation))
            .transpose()?;
        match &evidence.backend.outcome {
            KaniRunOutcome::Falsified { .. } => match (safety, constraint) {
                (O::Completed(V::Boolean(false)), _) => S::Falsified,
                (O::Undefined { .. }, _) => S::EvaluationViolation,
                (O::Refused(_), _) => S::Refused,
                (O::Incomplete(_), _) => S::Incomplete,
                (_, Some(O::Undefined { .. })) => S::EvaluationViolation,
                (_, Some(O::Refused(_))) => S::Refused,
                (_, Some(O::Incomplete(_))) => S::Incomplete,
                _ => return Err(ConstrainedBindingRefusal::Observation),
            },
            KaniRunOutcome::Verified | KaniRunOutcome::CoverUnsatisfied { .. } => {
                if !matches!(safety, O::Completed(V::Boolean(true))) {
                    return Err(ConstrainedBindingRefusal::Observation);
                }
                let covers = evidence
                    .backend
                    .checks
                    .iter()
                    .filter(|check| check.class == KaniCheckClass::Cover)
                    .count();
                let expected = if evidence.method.constraint.is_some() {
                    2
                } else {
                    1
                };
                if covers != expected || !complete {
                    S::Inconclusive
                } else if ordinary == Some(KaniCheckStatus::Unsatisfiable) {
                    S::CoverUnsatisfied
                } else if ordinary != Some(KaniCheckStatus::Satisfied) {
                    S::Inconclusive
                } else {
                    match (boundary, constraint) {
                        (
                            Some(ConstraintBoundary::Reached),
                            Some(O::Completed(V::Boolean(false))),
                        ) => S::ConstraintReached,
                        (
                            Some(ConstraintBoundary::NotReached),
                            Some(O::Completed(V::Boolean(true))),
                        )
                        | (None, None) => S::BoundedSuccess,
                        (Some(ConstraintBoundary::Unknown), _) => S::Inconclusive,
                        _ => return Err(ConstrainedBindingRefusal::Observation),
                    }
                }
            }
            KaniRunOutcome::Cancelled { .. } | KaniRunOutcome::Inconclusive { .. } => {
                S::Inconclusive
            }
        }
    };
    Ok(ConstrainedMethodResult {
        evidence,
        owner,
        status,
        boundary,
    })
}

/// Original replay settlement, explicitly paired with the selected method.
#[derive(Debug)]
pub struct ConstrainedReplaySettlement<'a> {
    /// Method actually replayed.
    pub method: &'a ConstrainedMethodIdentity,
    /// Original QSL replay reading.
    pub settlement: ReplaySettlement<'a>,
}

/// Preserve replay pairing, then refuse an unavailable QSL method terminal representation.
///
/// # Errors
/// Returns a typed binding/pairing refusal, or UnsupportedTerminalRepresentation. No private
/// terminal variant or unconditional proved fallback is emitted.
pub fn constrained_terminal_value(
    result: &ConstrainedMethodResult<'_>,
    replay: Option<ConstrainedReplaySettlement<'_>>,
) -> Result<qsl_replay::TerminalValue, ConstrainedBindingRefusal> {
    let needs_replay = matches!(
        result.status,
        ConstrainedMethodStatus::Falsified | ConstrainedMethodStatus::EvaluationViolation
    );
    match replay {
        Some(replay) if replay.method != result.evidence.method() => {
            return Err(ConstrainedBindingRefusal::Method)
        }
        Some(_) if !needs_replay => return Err(ConstrainedBindingRefusal::UnexpectedReplay),
        None if needs_replay => return Err(ConstrainedBindingRefusal::MissingReplay),
        _ => {}
    }
    Err(ConstrainedBindingRefusal::UnsupportedTerminalRepresentation)
}

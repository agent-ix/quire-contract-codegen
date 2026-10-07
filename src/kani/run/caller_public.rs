//! Ordinary public C execution over the actual retained production caller owner.
//!
//! Results are assembled only after the selected authenticated terminal transaction and actual
//! role/capture/creator settlement. Failed cancellation attempts use exact retained containment
//! authority and return CleanupUnconfirmed; they never upgrade a missing receipt to evidence.

use std::{num::NonZeroUsize, path::PathBuf, sync::Arc, time::Instant};

use super::{
    caller_bootstrap::{CallerBootstrapError, PreparedCleanupDetail},
    caller_driver::{CallerDriveError, CallerDriveProgress},
    caller_error,
    caller_execution::{CallerExecution, CallerExecutionError, PolicyTerminalOutcome},
    caller_prepare::PreparationError,
    execute::{
        ChargedPeakNotObservedReason, ChargedPeakObservation, KaniStartupAdmissionCause,
        KaniStartupCapability,
    },
    launch::{
        BoundedLaunch, BoundedLaunchError, BoundedProductionLaunch, GuardianFailureKind,
        LaunchOutcome,
    },
    memory::MemoryObservation,
    namespace::BackendCommand,
    publication::Publication,
    stdin::OriginalStdin,
};

pub(super) fn run(
    command: BackendCommand,
    helper: PathBuf,
    stdin: OriginalStdin,
    ceilings: crate::kani::identity::ProofCeilings,
    harnesses: NonZeroUsize,
    deadline: Option<Instant>,
    availability: MemoryObservation,
) -> Result<BoundedProductionLaunch, BoundedLaunchError> {
    let mut owner = match CallerExecution::prepare(
        command,
        helper,
        stdin,
        ceilings,
        harnesses,
        deadline,
        Arc::new(Publication::default()),
    ) {
        Ok(owner) => owner,
        Err(CallerExecutionError::Preparation(PreparationError::PreRoleExpired)) => {
            // Preparation creates neither L nor O. This is the actual allocated stage fact,
            // not a missing-measurement inference or an upgrade of C's availability probe.
            return Ok(BoundedProductionLaunch {
                launch: BoundedLaunch {
                    report: Ok(None),
                    outcome: LaunchOutcome::TimedOut,
                    memory: availability,
                },
                charged_peak: ChargedPeakObservation::NotObserved {
                    reason: ChargedPeakNotObservedReason::PreRoleTimeout,
                },
            });
        }
        Err(error) => return Err(startup_error(error)),
    };
    // This actual pre-L reservation is retained locally through every fallible actor/cleanup
    // operation. Diagnostic formatting cannot allocate a second full error String after L.
    let detail = owner
        .bootstrap
        .take_cleanup_detail()
        .map_err(|error| startup_error(CallerExecutionError::Bootstrap(error)))?;
    let result = match owner.drive_to_terminal() {
        Ok(CallerDriveProgress::Completed { .. }) => owner.finish_completed_transaction(),
        Ok(CallerDriveProgress::OwnerStopped) => owner.finish_owner_stopped(),
        Ok(CallerDriveProgress::Pending | CallerDriveProgress::Dispatched) => Err(
            CallerExecutionError::Bootstrap(CallerBootstrapError::TerminalTransition),
        ),
        Err(error @ CallerExecutionError::Progress(CallerDriveError::WorkExpired)) => {
            return cancelled_result(&mut owner, detail, error, CancelCandidate::WorkExpired);
        }
        Err(error @ CallerExecutionError::Progress(CallerDriveError::CaptureFailedFlag)) => {
            return cancelled_result(&mut owner, detail, error, CancelCandidate::CaptureFailed);
        }
        Err(CallerExecutionError::Progress(CallerDriveError::Bootstrap(
            CallerBootstrapError::OperationalFailureObserved,
        ))) => match owner.finish_operational_failure() {
            Ok(error) => return Err(error),
            Err(error) => Err(error),
        },
        Err(CallerExecutionError::Progress(CallerDriveError::Bootstrap(
            CallerBootstrapError::PolicyRefusalObserved,
        ))) => match owner.finish_policy_refused() {
            Ok(PolicyTerminalOutcome::Refused(error)) => return Err(error),
            Ok(PolicyTerminalOutcome::OwnerStopped { launch, peaks }) => {
                return Ok(BoundedProductionLaunch::from_measured(launch, peaks))
            }
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    };
    match result {
        Ok((launch, peaks)) => Ok(BoundedProductionLaunch::from_measured(launch, peaks)),
        Err(error) => cancelled_result(&mut owner, detail, error, CancelCandidate::LocalError),
    }
}

enum CancelCandidate {
    WorkExpired,
    CaptureFailed,
    LocalError,
}

fn cancelled_result(
    owner: &mut CallerExecution,
    detail: PreparedCleanupDetail,
    original: CallerExecutionError,
    candidate: CancelCandidate,
) -> Result<BoundedProductionLaunch, BoundedLaunchError> {
    if owner.is_settled() {
        return Err(local_error(owner, original));
    }
    let cancelled = (|| {
        if matches!(candidate, CancelCandidate::LocalError) && owner.finish_uncreated_error()? {
            return Ok(None);
        }
        loop {
            if owner.cancel_step()? {
                break;
            }
            let cutoff = owner
                .clock
                .settlement_deadline()
                .map_err(CallerExecutionError::Deadline)?;
            owner.pause_until(cutoff)?;
        }
        if owner.bootstrap.owner_stop_pending() {
            return owner.finish_owner_stopped().map(Some);
        }
        match candidate {
            CancelCandidate::WorkExpired => owner.finish_cancelled_timeout().map(Some),
            CancelCandidate::CaptureFailed => owner.finish_cancelled_capture().map(Some),
            CancelCandidate::LocalError => {
                let captures = owner.finish_cancelled()?;
                drop(captures);
                Ok(None)
            }
        }
    })();
    match cancelled {
        Ok(Some((launch, peaks))) => Ok(BoundedProductionLaunch::from_measured(launch, peaks)),
        Ok(None) => Err(local_error(owner, original)),
        Err(failure) => {
            if owner.is_settled() {
                return Err(local_error(owner, original));
            }
            // Forced outer containment is an attempt, not a substitute for the separately
            // authenticated M/terminal proof. Keep the override even if those retained child
            // waits/captures finish: no missing ordinary transaction is converted to success.
            let containment = owner.attempt_unconfirmed_containment();
            Err(BoundedLaunchError::Guardian {
                kind: GuardianFailureKind::CleanupUnconfirmed,
                detail: detail.seal(format_args!("original caller failure: {original}; cancellation: {failure}; containment attempt: {containment:?}")),
            })
        }
    }
}

fn startup_error(error: CallerExecutionError) -> BoundedLaunchError {
    // Named preparation reservations are the explicit memory-enforcement sites. Other C
    // bootstrap errors refuse establishment of the retained trusted ownership/control chain;
    // they are never recast as memory exhaustion from their errno or diagnostic prose.
    let memory_reservation = matches!(
        &error,
        CallerExecutionError::Preparation(PreparationError::CaptureSizeOverflow)
            | CallerExecutionError::Preparation(PreparationError::CallerBootstrap(
                CallerBootstrapError::ReservationUnrepresentable
            ))
            | CallerExecutionError::Bootstrap(CallerBootstrapError::ReservationUnrepresentable)
            | CallerExecutionError::Progress(CallerDriveError::Bootstrap(
                CallerBootstrapError::ReservationUnrepresentable
            ))
    );
    match error {
        CallerExecutionError::Assembly(error) => error,
        original => BoundedLaunchError::Unavailable {
            admission: if memory_reservation {
                KaniStartupAdmissionCause::MemoryEnforcement
            } else {
                KaniStartupAdmissionCause::CapabilityUnavailable {
                    capability: KaniStartupCapability::TrustedOwnerProtection,
                }
            },
            cause: caller_error::into_original_io(original),
        },
    }
}

fn local_error(owner: &mut CallerExecution, error: CallerExecutionError) -> BoundedLaunchError {
    let command_failed = matches!(
        &error,
        CallerExecutionError::Bootstrap(CallerBootstrapError::Spawn(
            super::spawner::SpawnFailure::Command(_)
        )) | CallerExecutionError::Progress(CallerDriveError::Bootstrap(
            CallerBootstrapError::Spawn(super::spawner::SpawnFailure::Command(_))
        ))
    );
    if command_failed {
        // The original Command::spawn error is not a creator/pidfd/clock error. Path storage
        // was reserved before L and refers to precisely the configured helper program.
        if let Some(path) = owner.bootstrap.take_helper_path() {
            return BoundedLaunchError::BoundaryIo {
                path,
                cause: caller_error::into_original_io(error),
            };
        }
        // Missing path custody cannot invent a backend executable context.
        return startup_error(error);
    }
    if owner.was_dispatched() {
        BoundedLaunchError::Io(caller_error::into_original_io(error))
    } else {
        startup_error(error)
    }
}

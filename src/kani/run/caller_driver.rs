//! Finite caller startup, Dispatch and genuine completion progress over retained owners.
//!
//! No error drops the original lease. The public owner loop takes its actual custody explicitly
//! before closing it and settling roles; progress here grants no report, evidence or result.

use std::{sync::atomic::Ordering, time::Instant};

use super::{
    caller_bootstrap::{CallerBootstrap, CallerBootstrapError},
    control::ControlError,
    protocol::BackendExit,
    role_deadline::{DeadlineError, ExecutionClock, StopStamp},
    stages::{BackendCompletion, CallerLeaseClient, CallerLeaseProgress, InitReady, StageError},
};

/// Private provisional progress; actual process/capture settlement remains with the caller owner.
pub(super) enum CallerDriveProgress {
    Pending,
    Dispatched,
    OwnerStopped,
    Completed {
        outcome: BackendExit,
        stop: StopStamp,
    },
}

/// Typed driver failures retain actual custody and grant no public classification.
#[derive(Debug)]
pub(super) enum CallerDriveError {
    Bootstrap(CallerBootstrapError),
    Stage(StageError),
    Deadline(DeadlineError),
    CaptureFailedFlag,
    WorkExpired,
}

impl From<CallerBootstrapError> for CallerDriveError {
    fn from(error: CallerBootstrapError) -> Self {
        match error {
            CallerBootstrapError::Deadline(error) => Self::Deadline(error),
            other => Self::Bootstrap(other),
        }
    }
}

impl std::fmt::Display for CallerDriveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "caller progress refused: {self:?}")
    }
}

impl std::error::Error for CallerDriveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Bootstrap(error) => Some(error),
            Self::Stage(error) => Some(error),
            Self::Deadline(error) => Some(error),
            Self::CaptureFailedFlag | Self::WorkExpired => None,
        }
    }
}

#[derive(Default)]
enum CallerDriveState {
    #[default]
    Startup,
    // The actual Ready lease is stored before even a missing Dispatch can refuse progress.
    PendingReady(InitReady),
    Dispatch,
    Running,
    Completed(BackendCompletion),
    OwnerStopped {
        ready: Option<InitReady>,
    },
}

/// Original lease custody survives every fallible step; this type performs no Drop cleanup.
#[derive(Default)]
pub(super) struct CallerDriver {
    client: Option<CallerLeaseClient>,
    state: CallerDriveState,
    poisoned: bool,
}

impl CallerDriver {
    pub(super) fn new() -> Self {
        Self::default()
    }

    /// CallerBootstrap already charges CallerLeaseClient metadata; add only this actual extra
    /// fixed storage. Dispatch frames and receive buffers retain their original reservations.
    pub(super) fn metadata_reservation() -> Result<u64, CallerDriveError> {
        let extra = std::mem::size_of::<Self>()
            .checked_sub(std::mem::size_of::<CallerLeaseClient>())
            .ok_or(CallerDriveError::Stage(StageError::Control(
                ControlError::EncodedBytesExceeded,
            )))?;
        u64::try_from(extra).map_err(|_| {
            CallerDriveError::Stage(StageError::Control(ControlError::EncodedBytesExceeded))
        })
    }

    /// Drive one finite operation with the caller's existing cutoff, never a new allowance.
    pub(super) fn step(
        &mut self,
        owner: &mut CallerBootstrap,
        clock: &mut ExecutionClock,
        cutoff: Option<Instant>,
    ) -> Result<CallerDriveProgress, CallerDriveError> {
        if self.poisoned {
            return Err(CallerDriveError::Stage(StageError::UnexpectedControl));
        }
        self.poisoned = true;
        let result = self.advance(owner, clock, cutoff);
        if result.is_ok() {
            self.poisoned = false;
        }
        result
    }

    fn advance(
        &mut self,
        owner: &mut CallerBootstrap,
        clock: &mut ExecutionClock,
        cutoff: Option<Instant>,
    ) -> Result<CallerDriveProgress, CallerDriveError> {
        if owner.streams.flags.failed.load(Ordering::Acquire) {
            owner.driver_capture_stop(clock)?;
            return Err(CallerDriveError::CaptureFailedFlag);
        }
        if let CallerDriveState::Completed(completion) = &self.state {
            return Ok(CallerDriveProgress::Completed {
                outcome: completion.outcome,
                stop: completion.stop,
            });
        }
        if matches!(self.state, CallerDriveState::OwnerStopped { .. }) {
            return Ok(CallerDriveProgress::OwnerStopped);
        }
        if clock.work_expired(Instant::now()) {
            owner.driver_capture_stop(clock)?;
            return Err(CallerDriveError::WorkExpired);
        }
        if matches!(self.state, CallerDriveState::Startup) {
            if let Some(ready) = owner.startup_step(
                clock,
                cutoff.map_or(owner.startup_cutoff(), |cutoff| {
                    cutoff.min(owner.startup_cutoff())
                }),
            )? {
                self.state = CallerDriveState::PendingReady(ready);
            }
            return Ok(CallerDriveProgress::Pending);
        }
        if owner.driver_control_step(clock, cutoff)? {
            let state = std::mem::replace(&mut self.state, CallerDriveState::Startup);
            let ready = match state {
                CallerDriveState::PendingReady(ready) => Some(ready),
                CallerDriveState::Startup
                | CallerDriveState::Dispatch
                | CallerDriveState::Running => None,
                CallerDriveState::Completed(completion) => {
                    self.state = CallerDriveState::Completed(completion);
                    return Err(CallerDriveError::Stage(StageError::UnexpectedControl));
                }
                CallerDriveState::OwnerStopped { ready } => ready,
            };
            self.state = CallerDriveState::OwnerStopped { ready };
            return Ok(CallerDriveProgress::OwnerStopped);
        }
        if matches!(self.state, CallerDriveState::PendingReady(_)) {
            let dispatch = owner.dispatch.take().ok_or(CallerDriveError::Bootstrap(
                CallerBootstrapError::UnexpectedPhase,
            ))?;
            // No fallible operation occurs after moving Ready and before storing the client.
            let state = std::mem::replace(&mut self.state, CallerDriveState::Startup);
            let CallerDriveState::PendingReady(ready) = state else {
                self.state = state;
                owner.dispatch = Some(dispatch);
                return Err(CallerDriveError::Stage(StageError::UnexpectedControl));
            };
            self.client = Some(CallerLeaseClient::new(ready, dispatch));
            self.state = CallerDriveState::Dispatch;
            return Ok(CallerDriveProgress::Pending);
        }
        let client = self
            .client
            .as_mut()
            .ok_or(CallerDriveError::Stage(StageError::UnexpectedControl))?;
        if client.sending() {
            client
                .send_step(&owner.stdin, cutoff)
                .map_err(CallerDriveError::Stage)?;
            return Ok(CallerDriveProgress::Pending);
        }
        match owner.receive_inner_event_step(client, cutoff)? {
            CallerLeaseProgress::Pending => Ok(CallerDriveProgress::Pending),
            CallerLeaseProgress::Dispatched => {
                self.state = CallerDriveState::Running;
                Ok(CallerDriveProgress::Dispatched)
            }
            CallerLeaseProgress::Completed => {
                let actual = client
                    .completion()
                    .ok_or(CallerDriveError::Stage(StageError::UnexpectedControl))?;
                // The lease client retains genuine completion even if clock adoption fails.
                owner.driver_adopt_completion(clock, actual.stop)?;
                let completion = BackendCompletion {
                    outcome: actual.outcome,
                    stop: actual.stop,
                };
                let progress = CallerDriveProgress::Completed {
                    outcome: completion.outcome,
                    stop: completion.stop,
                };
                self.state = CallerDriveState::Completed(completion);
                Ok(progress)
            }
        }
    }

    /// Transfer only actual client custody to the public owner loop's explicit close operation.
    pub(super) fn take_client(&mut self) -> Option<CallerLeaseClient> {
        let client = self.client.take();
        if client.is_some() {
            self.poisoned = true;
        }
        client
    }

    /// Transfer a retained Ready lease after failure before the client could be constructed.
    pub(super) fn take_pending_ready(&mut self) -> Option<InitReady> {
        let state = std::mem::replace(&mut self.state, CallerDriveState::Startup);
        match state {
            CallerDriveState::PendingReady(ready) => {
                self.poisoned = true;
                Some(ready)
            }
            CallerDriveState::OwnerStopped { ready } => {
                self.state = CallerDriveState::OwnerStopped { ready: None };
                if ready.is_some() {
                    self.poisoned = true;
                }
                ready
            }
            other => {
                self.state = other;
                None
            }
        }
    }
}

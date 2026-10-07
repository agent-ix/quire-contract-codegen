//! Original caller execution over one retained bootstrap, lease driver and absolute clock.
//!
//! Progress and report bytes are provisional. Every fallible operation borrows this owner;
//! neither returning an error nor dropping it confirms role, capture or creator settlement.
//! Cancellation/error selection stays distinct from genuine completion/report acceptance.

use std::{
    io,
    num::NonZeroUsize,
    path::PathBuf,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use super::{
    caller_bootstrap::{CallerBootstrap, CallerBootstrapError, CallerRoleSettlement},
    caller_driver::{CallerDriveError, CallerDriveProgress, CallerDriver},
    caller_prepare::{self, PreparationError},
    caller_result,
    launch::{BoundedLaunch, BoundedLaunchError, CAPTURE_LIMIT},
    namespace::BackendCommand,
    protocol::BackendExit,
    publication::{Publication, Stage},
    resource_ledger::MeasuredPeaks,
    role_deadline::{DeadlineError, ExecutionClock},
    role_protocol::LauncherSettlementMode,
    stdin::OriginalStdin,
};
use crate::kani::identity::ProofCeilings;

const CALLER_TICK: Duration = Duration::from_millis(20);

/// Local typed errors retain their original values while the execution owner remains borrowed.
/// The public caller selects its existing refusal only after actual settlement or its truthful
/// CleanupUnconfirmed override. No remote cause is reconstructed by this type.
#[derive(Debug)]
pub(super) enum CallerExecutionError {
    Preparation(PreparationError),
    Progress(CallerDriveError),
    Bootstrap(CallerBootstrapError),
    Deadline(DeadlineError),
    Io(io::Error),
    Assembly(BoundedLaunchError),
}

impl std::fmt::Display for CallerExecutionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "caller execution refused: {self:?}")
    }
}

impl std::error::Error for CallerExecutionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Preparation(error) => Some(error),
            Self::Progress(error) => Some(error),
            Self::Bootstrap(error) => Some(error),
            Self::Deadline(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::Assembly(BoundedLaunchError::Io(error)) => Some(error),
            Self::Assembly(BoundedLaunchError::Unavailable { cause, .. }) => Some(cause),
            Self::Assembly(BoundedLaunchError::Guardian { .. }) => None,
        }
    }
}

/// The actual caller ownership composition. Its fields and retained buffers are charged before
/// L; it has no cleanup Drop implementation and no error-owned/background custody transfer.
pub(super) struct CallerExecution {
    pub(super) bootstrap: CallerBootstrap,
    pub(super) driver: CallerDriver,
    pub(super) clock: ExecutionClock,
    completed: Option<BackendExit>,
    report: Option<Vec<u8>>,
    roles: Option<CallerRoleSettlement>,
    lease_closed: bool,
    limit: usize,
    harnesses: NonZeroUsize,
}

impl CallerExecution {
    pub(super) fn additional_metadata() -> Result<u64, CallerBootstrapError> {
        // The bootstrap already charges all three retained component values. Only real fixed
        // result/lease/output state and alignment in this composition are additional.
        let bytes = std::mem::size_of::<Self>()
            .checked_sub(std::mem::size_of::<CallerBootstrap>())
            .and_then(|bytes| bytes.checked_sub(std::mem::size_of::<CallerDriver>()))
            .and_then(|bytes| bytes.checked_sub(std::mem::size_of::<ExecutionClock>()))
            .ok_or(CallerBootstrapError::ReservationUnrepresentable)?;
        u64::try_from(bytes).map_err(|_| CallerBootstrapError::ReservationUnrepresentable)
    }

    pub(super) fn prepare(
        recipe: BackendCommand,
        helper: PathBuf,
        stdin: OriginalStdin,
        ceilings: ProofCeilings,
        harnesses: NonZeroUsize,
        deadline: Option<Instant>,
        publication: Arc<Publication>,
    ) -> Result<Self, CallerExecutionError> {
        let clock = ExecutionClock::prepare(deadline, ceilings.wall_clock)
            .map_err(CallerExecutionError::Deadline)?;
        let bootstrap = caller_prepare::prepare(
            recipe,
            helper,
            stdin,
            ceilings,
            harnesses,
            &clock,
            publication,
        )
        .map_err(CallerExecutionError::Preparation)?;
        let limit =
            CAPTURE_LIMIT
                .checked_mul(harnesses.get())
                .ok_or(CallerExecutionError::Preparation(
                    PreparationError::CaptureSizeOverflow,
                ))?;
        Ok(Self {
            bootstrap,
            driver: CallerDriver::new(),
            clock,
            completed: None,
            report: None,
            roles: None,
            lease_closed: false,
            limit,
            harnesses,
        })
    }

    pub(super) fn begin(&mut self) -> Result<(), CallerExecutionError> {
        if self.clock.work_expired(Instant::now()) {
            return Err(CallerExecutionError::Preparation(
                PreparationError::PreRoleExpired,
            ));
        }
        self.bootstrap
            .begin_launch()
            .map_err(CallerExecutionError::Bootstrap)
    }

    /// One actual nonblocking actor step; None retains the original no-work-cutoff admission.
    pub(super) fn advance(&mut self) -> Result<CallerDriveProgress, CallerExecutionError> {
        let cutoff = self.clock.progress_cutoff();
        let progress = self
            .driver
            .step(&mut self.bootstrap, &mut self.clock, cutoff)
            .map_err(CallerExecutionError::Progress)?;
        match progress {
            CallerDriveProgress::Dispatched => {
                self.bootstrap.publication.publish(Stage::Dispatched)
            }
            CallerDriveProgress::Completed { outcome, .. } => self.completed = Some(outcome),
            CallerDriveProgress::Pending | CallerDriveProgress::OwnerStopped => {}
        }
        Ok(progress)
    }

    /// Close actual transferred lease custody before publishing close completion. The same
    /// owner still retains every process/capture/creator authority. Cancellation must send its
    /// original authenticated close-control first; this method alone sends no cancellation.
    pub(super) fn close_original_lease(&mut self) -> Result<(), CallerExecutionError> {
        if self.lease_closed {
            return Err(CallerExecutionError::Bootstrap(
                CallerBootstrapError::TerminalTransition,
            ));
        }
        if let Some(client) = self.driver.take_client() {
            drop(client.into_lease());
        } else if let Some(ready) = self.driver.take_pending_ready() {
            drop(ready.into_lease());
        } else {
            self.bootstrap
                .close_bootstrap_lease()
                .map_err(CallerExecutionError::Bootstrap)?;
            self.lease_closed = true;
            return Ok(());
        }
        self.bootstrap.publication.lease_closed();
        self.bootstrap.publication.publish(Stage::LeaseClosing);
        self.lease_closed = true;
        Ok(())
    }

    /// Genuine I-completion report path only. A late authenticated stop remains a different
    /// candidate; provisional report bytes stay in this owner until its settlement route.
    pub(super) fn receive_completed_report(&mut self) -> Result<(), CallerExecutionError> {
        if self.completed.is_none() || !self.lease_closed {
            return Err(CallerExecutionError::Bootstrap(
                CallerBootstrapError::TerminalTransition,
            ));
        }
        self.report = self
            .bootstrap
            .read_terminal_report(&mut self.clock)
            .map_err(CallerExecutionError::Bootstrap)?;
        Ok(())
    }

    /// Complete commit reception alone does not produce evidence; actual normal O wait and
    /// all remaining role/capture/creator settlement are still mandatory below.
    pub(super) fn advance_commit(&mut self) -> Result<bool, CallerExecutionError> {
        self.bootstrap
            .receive_terminal_commit(&mut self.clock)
            .map_err(CallerExecutionError::Bootstrap)
    }

    pub(super) fn finish_completed(
        &mut self,
    ) -> Result<(BoundedLaunch, MeasuredPeaks), CallerExecutionError> {
        if self.bootstrap.owner_stop_pending() {
            return Err(CallerExecutionError::Bootstrap(
                CallerBootstrapError::OwnerStopObserved,
            ));
        }
        let outcome = self.completed.ok_or(CallerExecutionError::Bootstrap(
            CallerBootstrapError::TerminalTransition,
        ))?;
        if self.roles.is_none() {
            self.roles = Some(
                self.bootstrap
                    .settle_launcher_chain(&self.clock, LauncherSettlementMode::AfterCommit)
                    .map_err(CallerExecutionError::Bootstrap)?,
            );
        }
        let roles = self.roles.as_ref().ok_or(CallerExecutionError::Bootstrap(
            CallerBootstrapError::TerminalTransition,
        ))?;
        let peaks = loop {
            if let Some(peaks) = self
                .bootstrap
                .finish_terminal_after_roles(roles)
                .map_err(CallerExecutionError::Bootstrap)?
            {
                break peaks;
            }
            self.pause_until(roles.cutoff())?;
        };
        let captures = self
            .bootstrap
            .streams
            .settle(roles)
            .map_err(CallerExecutionError::Io)?;
        let text =
            self.bootstrap
                .streams
                .combined_text
                .take()
                .ok_or(CallerExecutionError::Bootstrap(
                    CallerBootstrapError::TerminalTransition,
                ))?;
        let launch = caller_result::assemble_completed(
            outcome,
            captures,
            text,
            Ok(self.report.take()),
            peaks,
            self.limit,
            self.harnesses,
        )
        .map_err(CallerExecutionError::Assembly)?;
        Ok((launch, peaks))
    }

    /// Wait only between actor steps under the actual existing cutoff. This bounds a wait,
    /// grants no progress/authorization and does not promise scheduler or kernel completion.
    pub(super) fn pause_until(&self, cutoff: Instant) -> Result<(), CallerExecutionError> {
        let remaining = cutoff
            .checked_duration_since(Instant::now())
            .filter(|remaining| !remaining.is_zero())
            .ok_or(CallerExecutionError::Deadline(DeadlineError::Expired))?;
        thread::park_timeout(remaining.min(CALLER_TICK));
        Ok(())
    }
}

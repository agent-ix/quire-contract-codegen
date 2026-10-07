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
    caller_streams::SettledCaptures,
    launch::{BoundedLaunch, BoundedLaunchError, LaunchOutcome, CAPTURE_LIMIT},
    memory::{MemoryMechanism, MemoryObservation},
    namespace::BackendCommand,
    protocol::BackendExit,
    publication::{Publication, Stage},
    resource_ledger::MeasuredPeaks,
    role_deadline::{DeadlineError, ExecutionClock},
    role_protocol::{LauncherSettlementMode, OwnerStopCause},
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
    PolicyProjection(super::startup_cause::RepresentationError),
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
            Self::PolicyProjection(error) => Some(error),
            Self::Assembly(BoundedLaunchError::Io(error)) => Some(error),
            Self::Assembly(BoundedLaunchError::BoundaryIo { cause, .. }) => Some(cause),
            Self::Assembly(BoundedLaunchError::Unavailable { cause, .. }) => Some(cause),
            Self::Assembly(BoundedLaunchError::Guardian { .. }) => None,
        }
    }
}

/// Actual settled policy refusal versus a genuine independent resource/deadline candidate.
/// No report/proof is manufactured by this private selection.
pub(super) enum PolicyTerminalOutcome {
    Refused(BoundedLaunchError),
    OwnerStopped {
        launch: BoundedLaunch,
        peaks: MeasuredPeaks,
    },
}

/// The actual caller ownership composition. Its fields and retained buffers are charged before
/// L; it has no cleanup Drop implementation and no error-owned/background custody transfer.
pub(super) struct CallerExecution {
    pub(super) bootstrap: CallerBootstrap,
    pub(super) driver: CallerDriver,
    pub(super) clock: ExecutionClock,
    completed: Option<BackendExit>,
    dispatched: bool,
    report: Option<Vec<u8>>,
    roles: Option<CallerRoleSettlement>,
    lease_closed: bool,
    work_expired: bool,
    capture_failed: bool,
    settled: bool,
    producer_clock_refusal: Option<DeadlineError>,
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
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<PolicyTerminalOutcome>()))
            .and_then(|bytes| {
                bytes.checked_add(std::mem::size_of::<super::launch::BoundedProductionLaunch>())
            })
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
            dispatched: false,
            report: None,
            roles: None,
            lease_closed: false,
            work_expired: false,
            capture_failed: false,
            settled: false,
            producer_clock_refusal: None,
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
        let progress = match self
            .driver
            .step(&mut self.bootstrap, &mut self.clock, cutoff)
        {
            Ok(progress) => progress,
            Err(error) => {
                if matches!(error, CallerDriveError::WorkExpired) {
                    // The actual driver captured C's first trigger before returning this error.
                    // Generic local failure/cancellation cannot select a timeout outcome.
                    self.work_expired = true;
                }
                if matches!(error, CallerDriveError::CaptureFailedFlag) {
                    self.capture_failed = true;
                }
                return Err(CallerExecutionError::Progress(error));
            }
        };
        match progress {
            CallerDriveProgress::Dispatched => {
                self.dispatched = true;
                self.bootstrap.publication.publish(Stage::Dispatched)
            }
            CallerDriveProgress::Completed { outcome, .. } => self.completed = Some(outcome),
            CallerDriveProgress::Pending | CallerDriveProgress::OwnerStopped => {}
        }
        Ok(progress)
    }

    /// Drive the one actual caller actor until genuine completion or an authenticated owner
    /// stop. Every error keeps this owner borrowed for explicit cancellation/settlement. Startup
    /// retains its original finite setup bound; post-Dispatch None mints no phase/work expiry.
    pub(super) fn drive_to_terminal(
        &mut self,
    ) -> Result<CallerDriveProgress, CallerExecutionError> {
        loop {
            match self.advance()? {
                progress @ (CallerDriveProgress::Completed { .. }
                | CallerDriveProgress::OwnerStopped) => return Ok(progress),
                CallerDriveProgress::Pending | CallerDriveProgress::Dispatched => {}
            }
            let cutoff = self.clock.progress_cutoff();
            let cutoff = if self.dispatched {
                cutoff
            } else {
                Some(cutoff.map_or(self.bootstrap.startup_cutoff(), |cutoff| {
                    cutoff.min(self.bootstrap.startup_cutoff())
                }))
            };
            let wait = cutoff.map_or(CALLER_TICK, |cutoff| {
                cutoff
                    .saturating_duration_since(Instant::now())
                    .min(CALLER_TICK)
            });
            if !wait.is_zero() {
                thread::park_timeout(wait);
            }
            // An elapsed wait is not a synthetic error/trigger. The next actual actor step
            // records C's real first stop and preserves capture/resource failure precedence.
        }
    }

    pub(super) fn was_dispatched(&self) -> bool {
        self.dispatched
    }

    pub(super) fn is_settled(&self) -> bool {
        self.settled
    }

    /// After protocol/transport failure, attempt containment using only this owner's actual
    /// original lease and retained L/O child custody. This does not fabricate a separate M reap,
    /// normal terminal receipt or successful cleanup. Its caller must still return the truthful
    /// CleanupUnconfirmed override if the ordinary authenticated transaction could not settle.
    pub(super) fn attempt_unconfirmed_containment(&mut self) -> Result<(), CallerExecutionError> {
        // An explicit producer-clock refusal cannot authorize a later receipt-time trigger.
        // Preserve unconfirmed custody instead of manufacturing a replacement allowance.
        if let Some(error) = self.producer_clock_refusal {
            return Err(CallerExecutionError::Deadline(error));
        }
        self.bootstrap
            .driver_capture_stop(&mut self.clock)
            .map_err(CallerExecutionError::Bootstrap)?;
        if !self.lease_closed {
            self.close_original_lease()?;
        }
        if self.roles.is_none() {
            self.roles = Some(
                self.bootstrap
                    .settle_launcher_chain(&self.clock, LauncherSettlementMode::CancelOuter)
                    .map_err(CallerExecutionError::Bootstrap)?,
            );
        }
        let roles = self.roles.as_ref().ok_or(CallerExecutionError::Bootstrap(
            CallerBootstrapError::TerminalTransition,
        ))?;
        let captures = self
            .bootstrap
            .streams
            .settle(roles)
            .map_err(CallerExecutionError::Io)?;
        drop(captures);
        Ok(())
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

    /// Original local startup errors need no remote receipt when actual completed creation
    /// proves L was never created. This never mints O/M/INIT or measurement evidence.
    pub(super) fn finish_uncreated_error(&mut self) -> Result<bool, CallerExecutionError> {
        let roles = match self.bootstrap.settle_never_created(&mut self.clock) {
            Ok(Some(roles)) => roles,
            Ok(None) => return Ok(false),
            Err(CallerBootstrapError::Deadline(error)) => {
                self.producer_clock_refusal = Some(error);
                return Err(CallerExecutionError::Bootstrap(
                    CallerBootstrapError::Deadline(error),
                ));
            }
            Err(error) => return Err(CallerExecutionError::Bootstrap(error)),
        };
        if !self.lease_closed {
            self.close_original_lease()?;
        }
        self.roles = Some(roles);
        self.bootstrap
            .streams
            .discard_after_roles(self.roles.as_ref().ok_or(CallerExecutionError::Bootstrap(
                CallerBootstrapError::SettlementReplyMismatch,
            ))?)
            .map_err(CallerExecutionError::Io)?;
        self.settled = true;
        Ok(true)
    }

    /// C retains its original local candidate outside this finite progress operation. Publish
    /// original cancellation authority completely before actually closing its I lease.
    pub(super) fn cancel_step(&mut self) -> Result<bool, CallerExecutionError> {
        if !self
            .bootstrap
            .cancel_close_step(&mut self.clock)
            .map_err(CallerExecutionError::Bootstrap)?
        {
            return Ok(false);
        }
        if !self.lease_closed {
            self.close_original_lease()?;
        }
        self.bootstrap
            .cancellation_reply_step(&mut self.clock)
            .map_err(CallerExecutionError::Bootstrap)
    }

    /// A cancellation receipt is not cleanup. Retain the actual normal O/L/creator proof even
    /// if the later stream/capture operation fails; return only real settled captures.
    pub(super) fn finish_cancelled(&mut self) -> Result<SettledCaptures, CallerExecutionError> {
        if self.roles.is_none() {
            self.roles = Some(
                self.bootstrap
                    .settle_launcher_chain(&self.clock, LauncherSettlementMode::ObserveOuterExit)
                    .map_err(CallerExecutionError::Bootstrap)?,
            );
        }
        let roles = self.roles.as_ref().ok_or(CallerExecutionError::Bootstrap(
            CallerBootstrapError::TerminalTransition,
        ))?;
        loop {
            if self
                .bootstrap
                .finish_cancellation_after_roles(roles)
                .map_err(CallerExecutionError::Bootstrap)?
            {
                break;
            }
            self.pause_until(roles.cutoff())?;
        }
        let captures = self
            .bootstrap
            .streams
            .settle(roles)
            .map_err(CallerExecutionError::Io)?;
        self.settled = true;
        Ok(captures)
    }

    /// Classify only C's retained genuine work-expiry candidate after authenticated cancellation,
    /// final actual O measurements and every normal role/EOF/capture/creator settlement. A real
    /// independently established owner stop retains its original resource/timeout precedence.
    pub(super) fn finish_cancelled_timeout(
        &mut self,
    ) -> Result<(BoundedLaunch, MeasuredPeaks), CallerExecutionError> {
        if !self.work_expired {
            return Err(CallerExecutionError::Bootstrap(
                CallerBootstrapError::TerminalTransition,
            ));
        }
        if self.bootstrap.owner_stop_pending() {
            return self.finish_owner_stopped();
        }
        let captures = self.finish_cancelled()?;
        drop(captures);
        let roles = self.roles.as_ref().ok_or(CallerExecutionError::Bootstrap(
            CallerBootstrapError::TerminalTransition,
        ))?;
        let peaks = self
            .bootstrap
            .settled_cancellation_peaks(roles)
            .map_err(CallerExecutionError::Bootstrap)?;
        drop(self.report.take());
        Ok((
            BoundedLaunch {
                report: Ok(None),
                outcome: LaunchOutcome::TimedOut,
                memory: MemoryObservation {
                    mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                    peak_resident_bytes: Some(peaks.tree_rss_bytes),
                },
            },
            peaks,
        ))
    }

    /// Preserve the existing stdout-first capture refusal from the actual joined readers. The
    /// shared failed flag triggers cancellation but is not itself an unread/overflow result.
    /// Both successful captures beside that flag refuse the inconsistent transition instead.
    pub(super) fn finish_cancelled_capture(
        &mut self,
    ) -> Result<(BoundedLaunch, MeasuredPeaks), CallerExecutionError> {
        if !self.capture_failed {
            return Err(CallerExecutionError::Bootstrap(
                CallerBootstrapError::TerminalTransition,
            ));
        }
        if self.bootstrap.owner_stop_pending() {
            return self.finish_owner_stopped();
        }
        let captures = self.finish_cancelled()?;
        let roles = self.roles.as_ref().ok_or(CallerExecutionError::Bootstrap(
            CallerBootstrapError::TerminalTransition,
        ))?;
        let peaks = self
            .bootstrap
            .settled_cancellation_peaks(roles)
            .map_err(CallerExecutionError::Bootstrap)?;
        let outcome = match super::launch::stream_bytes(
            super::launch::CaptureStream::Stdout,
            captures.stdout,
            self.limit,
            self.harnesses,
        ) {
            Err(outcome) => outcome,
            Ok(stdout) => {
                drop(stdout);
                match super::launch::stream_bytes(
                    super::launch::CaptureStream::Stderr,
                    captures.stderr,
                    self.limit,
                    self.harnesses,
                ) {
                    Err(outcome) => outcome,
                    Ok(_) => {
                        return Err(CallerExecutionError::Bootstrap(
                            CallerBootstrapError::TerminalTransition,
                        ));
                    }
                }
            }
        };
        drop(self.report.take());
        Ok((
            BoundedLaunch {
                report: Ok(None),
                outcome,
                memory: MemoryObservation {
                    mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                    peak_resident_bytes: Some(peaks.tree_rss_bytes),
                },
            },
            peaks,
        ))
    }

    /// Preserve a genuine independent O resource/work stop even beside C cancellation or
    /// provisional report bytes. Actual normal roles, stream end and captures precede assembly.
    pub(super) fn finish_owner_stopped(
        &mut self,
    ) -> Result<(BoundedLaunch, MeasuredPeaks), CallerExecutionError> {
        if !self.bootstrap.owner_stop_pending() {
            return Err(CallerExecutionError::Bootstrap(
                CallerBootstrapError::TerminalTransition,
            ));
        }
        if !self.lease_closed {
            self.close_original_lease()?;
        }
        if self.roles.is_none() {
            self.roles = Some(
                self.bootstrap
                    .settle_launcher_chain(&self.clock, LauncherSettlementMode::ObserveOuterExit)
                    .map_err(CallerExecutionError::Bootstrap)?,
            );
        }
        let roles = self.roles.as_ref().ok_or(CallerExecutionError::Bootstrap(
            CallerBootstrapError::TerminalTransition,
        ))?;
        let (cause, peaks) = loop {
            if let Some(stop) = self
                .bootstrap
                .finish_owner_stop_after_roles(roles)
                .map_err(CallerExecutionError::Bootstrap)?
            {
                break stop;
            }
            self.pause_until(roles.cutoff())?;
        };
        // Actual capture joins still matter even when genuine resource/timeout precedence
        // discards their bytes and the provisional report. No capture flag substitutes for EOF.
        let captures = self
            .bootstrap
            .streams
            .settle(roles)
            .map_err(CallerExecutionError::Io)?;
        self.settled = true;
        drop(captures);
        drop(self.report.take());
        let outcome = match cause {
            OwnerStopCause::ResourceExhausted => LaunchOutcome::MemoryExhausted,
            OwnerStopCause::TimedOut => LaunchOutcome::TimedOut,
        };
        Ok((
            BoundedLaunch {
                report: Ok(None),
                outcome,
                memory: MemoryObservation {
                    mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                    peak_resident_bytes: Some(peaks.tree_rss_bytes),
                },
            },
            peaks,
        ))
    }

    /// The preparation-negative carries no measurements or report. The original O cause is
    /// projected only after actual normal O/L/creator/EOF and real capture joins by the retained
    /// earliest cutoff. Missing/abnormal custody leaves this owner unsettled and refuses.
    pub(super) fn finish_operational_failure(
        &mut self,
    ) -> Result<BoundedLaunchError, CallerExecutionError> {
        if !self.lease_closed {
            self.close_original_lease()?;
        }
        if self.roles.is_none() {
            self.roles = Some(
                self.bootstrap
                    .settle_launcher_chain(&self.clock, LauncherSettlementMode::ObserveOuterExit)
                    .map_err(CallerExecutionError::Bootstrap)?,
            );
        }
        let roles = self.roles.as_ref().ok_or(CallerExecutionError::Bootstrap(
            CallerBootstrapError::TerminalTransition,
        ))?;
        loop {
            if self
                .bootstrap
                .finish_operational_failure_after_roles(roles)
                .map_err(CallerExecutionError::Bootstrap)?
            {
                break;
            }
            self.pause_until(roles.cutoff())?;
        }
        let captures = self
            .bootstrap
            .streams
            .settle(roles)
            .map_err(CallerExecutionError::Io)?;
        drop(captures);
        drop(self.report.take());
        if Instant::now() >= roles.cutoff() {
            return Err(CallerExecutionError::Deadline(DeadlineError::Expired));
        }
        self.settled = true;
        let failure = self
            .bootstrap
            .operational_failure_after_roles()
            .map_err(CallerExecutionError::Bootstrap)?;
        Self::project_preparation_failure(failure)
    }

    fn project_preparation_failure(
        failure: super::outer_failure::FailureHeader,
    ) -> Result<BoundedLaunchError, CallerExecutionError> {
        use super::{
            cross_role_cause::{
                CauseCheckerProvenance, CauseCheckerRole, CauseIntegrityPredicate,
                CauseIntegritySource, CauseOperation, KaniCauseMetadataIntegrityError,
                RemoteCauseProvenance, RemoteCauseRole,
            },
            execute::{KaniStartupAdmissionCause, KaniStartupCapability},
            outer_failure::FailureRepresentation,
            startup_cause::{ProjectedStartupCause, RepresentationError},
        };
        // Exact preparation producers, not an errno/Display classification or a generic
        // permission to reuse policy AC39. Post-activation observation errors are distinct.
        let admission = match failure.operation {
            CauseOperation::ProcSetup => KaniStartupAdmissionCause::CapabilityUnavailable {
                capability: KaniStartupCapability::PrivateProc,
            },
            CauseOperation::ReportCreation
            | CauseOperation::ReportCollection
            | CauseOperation::LauncherObservation
            | CauseOperation::TreeObservation => KaniStartupAdmissionCause::MemoryEnforcement,
            CauseOperation::Identity
            | CauseOperation::OwnerProtection
            | CauseOperation::ControlPreparation
            | CauseOperation::MonitorSpawn => KaniStartupAdmissionCause::CapabilityUnavailable {
                capability: KaniStartupCapability::TrustedOwnerProtection,
            },
            CauseOperation::RoleBootstrap
            | CauseOperation::NamespaceSetup
            | CauseOperation::ControlEncoding
            | CauseOperation::ControlReception
            | CauseOperation::OuterSpawn
            | CauseOperation::OuterControl
            | CauseOperation::OuterWaitRetirement
            | CauseOperation::ResourceAccounting
            | CauseOperation::ReportSealing
            | CauseOperation::ReportDelivery
            | CauseOperation::MonitorClaim
            | CauseOperation::MonitorStop
            | CauseOperation::MonitorReap
            | CauseOperation::ExclusiveLease
            | CauseOperation::NativePolicyPreparation
            | CauseOperation::NativePolicyInstallation
            | CauseOperation::BackendSupervision
            | CauseOperation::BackendCompletion
            | CauseOperation::SettlementControl => {
                return Err(CallerExecutionError::Bootstrap(
                    CallerBootstrapError::TerminalReplyMismatch,
                ));
            }
        };
        let cause = match failure.representation {
            FailureRepresentation::Original { cause } => {
                match cause.project(RemoteCauseProvenance {
                    role: RemoteCauseRole::Outer,
                    operation: failure.operation,
                }) {
                    Ok(ProjectedStartupCause::Io { error, .. }) => error,
                    Ok(ProjectedStartupCause::Seccompiler { .. }) => {
                        return Err(CallerExecutionError::PolicyProjection(
                            RepresentationError::NonInstallationBackendCause,
                        ));
                    }
                    Err(error) => {
                        let predicate = match &error {
                            RepresentationError::OsKindMismatch => {
                                CauseIntegrityPredicate::OriginalOsKindMismatch
                            }
                            RepresentationError::MissingOsCode
                            | RepresentationError::OsPayloadMismatch => {
                                CauseIntegrityPredicate::MalformedCauseMetadata
                            }
                            RepresentationError::InvalidContextBound
                            | RepresentationError::Reservation(_)
                            | RepresentationError::ContextExceeded
                            | RepresentationError::Formatting
                            | RepresentationError::UnnamedIoKind
                            | RepresentationError::NonInstallationBackendCause
                            | RepresentationError::PolicyCauseMismatch => {
                                return Err(CallerExecutionError::PolicyProjection(error));
                            }
                        };
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            KaniCauseMetadataIntegrityError::new(
                                predicate,
                                CauseCheckerProvenance {
                                    role: Some(CauseCheckerRole::Caller),
                                    operation: Some(CauseOperation::ControlReception),
                                },
                                Some(CauseIntegritySource::Representation(error)),
                            ),
                        )
                    }
                }
            }
            FailureRepresentation::Integrity { predicate } => {
                // Authenticated O supplies its actual checking predicate, without original
                // replay. Its independently unknown check operation/source remain absent.
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    KaniCauseMetadataIntegrityError::new(
                        predicate,
                        CauseCheckerProvenance {
                            role: Some(CauseCheckerRole::Outer),
                            operation: None,
                        },
                        None,
                    ),
                )
            }
        };
        Ok(BoundedLaunchError::Unavailable { admission, cause })
    }

    /// Policy failure was authenticated from the actual positively retained pre-recipe I.
    /// Preserve its original B stamp/cause through actual lease EOF, I termination/M reap,
    /// normal O/L/creator proof and captures; independent genuine owner stops still win.
    pub(super) fn finish_policy_refused(
        &mut self,
    ) -> Result<PolicyTerminalOutcome, CallerExecutionError> {
        if !self.lease_closed {
            self.close_original_lease()?;
        }
        loop {
            let cutoff = self
                .clock
                .settlement_deadline()
                .map_err(CallerExecutionError::Deadline)?;
            if self
                .bootstrap
                .driver_control_step(&mut self.clock, Some(cutoff))
                .map_err(CallerExecutionError::Bootstrap)?
            {
                break;
            }
            self.pause_until(cutoff)?;
        }
        if self.bootstrap.owner_stop_pending() {
            let (launch, peaks) = self.finish_owner_stopped()?;
            return Ok(PolicyTerminalOutcome::OwnerStopped { launch, peaks });
        }
        if self.roles.is_none() {
            self.roles = Some(
                self.bootstrap
                    .settle_launcher_chain(&self.clock, LauncherSettlementMode::ObserveOuterExit)
                    .map_err(CallerExecutionError::Bootstrap)?,
            );
        }
        let roles = self.roles.as_ref().ok_or(CallerExecutionError::Bootstrap(
            CallerBootstrapError::TerminalTransition,
        ))?;
        loop {
            if self
                .bootstrap
                .finish_setup_refusal_after_roles(roles)
                .map_err(CallerExecutionError::Bootstrap)?
            {
                break;
            }
            self.pause_until(roles.cutoff())?;
        }
        let captures = self
            .bootstrap
            .streams
            .settle(roles)
            .map_err(CallerExecutionError::Io)?;
        self.settled = true;
        drop(captures);
        let (failure, _original_context) = self
            .bootstrap
            .settled_policy_refusal(roles)
            .map_err(CallerExecutionError::Bootstrap)?;
        let projected = super::startup_projection::project_policy(failure)
            .map_err(CallerExecutionError::PolicyProjection)?;
        // Public OS error is returned directly: wrapping it to attach private provenance would
        // erase raw_os_error. The authenticated original site remains retained in bootstrap.
        Ok(PolicyTerminalOutcome::Refused(
            BoundedLaunchError::Unavailable {
                admission: super::execute::KaniStartupAdmissionCause::CapabilityUnavailable {
                    capability: projected.capability,
                },
                cause: projected.cause,
            },
        ))
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

    /// Drive the genuine completed transaction without releasing the original owner on error.
    /// A report read is provisional: authentic owner-stop metadata supersedes it, and the final
    /// commit still precedes actual normal role, stream and capture settlement. A local read or
    /// transport error remains owned by the caller for its separate cancellation path.
    pub(super) fn finish_completed_transaction(
        &mut self,
    ) -> Result<(BoundedLaunch, MeasuredPeaks), CallerExecutionError> {
        if self.completed.is_none() {
            return Err(CallerExecutionError::Bootstrap(
                CallerBootstrapError::TerminalTransition,
            ));
        }
        if !self.lease_closed {
            self.close_original_lease()?;
        }
        let read = self.receive_completed_report();
        if self.bootstrap.owner_stop_pending() {
            return self.finish_owner_stopped();
        }
        read?;
        loop {
            let committed = self.advance_commit()?;
            if self.bootstrap.owner_stop_pending() {
                return self.finish_owner_stopped();
            }
            if committed {
                break;
            }
            let cutoff = self
                .clock
                .settlement_deadline()
                .map_err(CallerExecutionError::Deadline)?;
            self.pause_until(cutoff)?;
        }
        self.finish_completed()
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
                    .settle_launcher_chain(&self.clock, LauncherSettlementMode::ObserveOuterExit)
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
        self.settled = true;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani::run::{
        cross_role_cause::{CauseOperation, KaniCrossRoleCauseLoss},
        execute::KaniStartupAdmissionCause,
        outer_failure::{FailureHeader, FailureRepresentation},
        protocol::current_build_identity,
        role_deadline::{StopOrigin, StopStamp},
        startup_cause::StartupCause,
    };

    fn failure(original: &io::Error) -> FailureHeader {
        FailureHeader {
            identity: current_build_identity(),
            authority: serde_json::from_value(serde_json::to_value([0_u8; 32]).unwrap()).unwrap(),
            stop: StopStamp::capture(StopOrigin::Outer).unwrap(),
            operation: CauseOperation::ReportCreation,
            representation: FailureRepresentation::Original {
                cause: StartupCause::capture_io(original).unwrap(),
            },
        }
    }

    /// Trace: FR-034-AC-40
    #[test]
    fn operational_projection_preserves_errno_and_only_marks_actual_custom_loss() {
        // Projection primitive only: actor authentication, original producer kind gate and
        // whole-chain settlement are separate. No metadata token proves those obligations.
        let os = io::Error::from_raw_os_error(nix::libc::EACCES);
        let BoundedLaunchError::Unavailable { admission, cause } =
            CallerExecution::project_preparation_failure(failure(&os)).unwrap()
        else {
            panic!("negative projection changed public branch");
        };
        assert_eq!(admission, KaniStartupAdmissionCause::MemoryEnforcement);
        assert_eq!(cause.raw_os_error(), os.raw_os_error());
        assert_eq!(cause.kind(), os.kind());
        assert!(cause.get_ref().is_none());
        let original = io::Error::new(io::ErrorKind::Other, "actual custom source");
        let BoundedLaunchError::Unavailable { cause, .. } =
            CallerExecution::project_preparation_failure(failure(&original)).unwrap()
        else {
            panic!("custom loss changed public branch");
        };
        assert_eq!(cause.kind(), original.kind());
        assert_eq!(cause.raw_os_error(), None);
        assert!(cause
            .get_ref()
            .unwrap()
            .downcast_ref::<KaniCrossRoleCauseLoss>()
            .is_some());
        let free = io::Error::from(io::ErrorKind::Other);
        let BoundedLaunchError::Unavailable { cause, .. } =
            CallerExecution::project_preparation_failure(failure(&free)).unwrap()
        else {
            panic!("payload-free error changed public branch");
        };
        assert_eq!(cause.kind(), free.kind());
        assert!(cause.get_ref().is_none());
    }

    /// Trace: FR-034-AC-15, FR-034-AC-40
    #[test]
    fn operational_projection_rejects_a_non_preparation_producer_site() {
        let mut header = failure(&io::Error::from(io::ErrorKind::InvalidData));
        header.operation = CauseOperation::NativePolicyInstallation;
        assert!(matches!(
            CallerExecution::project_preparation_failure(header),
            Err(CallerExecutionError::Bootstrap(
                CallerBootstrapError::TerminalReplyMismatch
            ))
        ));
    }
}

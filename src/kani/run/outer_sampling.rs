//! O's actual retained L/tree observer, conservative ledger and anonymous collector.
//!
//! This owner creates no child during preparation. A real complete setup sample must authorize
//! the report writer before the nested monitor factory can consume it. Later ticks continue the
//! same observations while the backend runs; neither completion nor quiet pipe data bypasses RSS.
//! Final report sealing and evidence conclusions remain the whole-chain owner's separate duties.

use std::{
    io,
    os::fd::{AsFd, OwnedFd},
    time::Instant,
};

use super::{
    control::{
        role_pair, ControlError, FrameStorage, GuardianEndpoint, IncrementalReceive,
        IncrementalSend, PreparedFrame, RoleCaller, RoleEndpoint,
    },
    cross_role_cause::{CauseIntegrityPredicate, CauseOperation},
    memory::{LauncherMemory, MemoryObserver},
    namespace::{
        GatedClaim, GuardianIdentity, InnerSettlement, MonitorStopSettlement, OuterMonitorOwner,
        ReadyIdentityError,
    },
    outer_caller::{OuterCallerControl, OuterCallerReceive},
    outer_failure::{FailureHeader, FailureRepresentation, FailureState, NegativeCommit},
    outer_preparation::{SamplingParts, SamplingPreparation},
    outer_setup::PreparedOuter,
    protocol::BackendExit,
    report_storage::{ReportCollector, ReportError, SealedReport},
    resource_ledger::{ChargeError, MeasuredPeaks, MemoryTick, ResourceLedger},
    role_deadline::{
        DeadlineError, IdentityDeadline, RoleDeadline, StopOrigin, StopStamp, StopTimeline,
    },
    role_protocol::{
        CallerTerminalControl, InnerBootstrap, InnerOwnerControl, OuterPhaseCommand,
        OuterPhaseReply, OuterTerminalReply, OwnerStopCause, RunSettings, TerminalDisposition,
    },
    startup_cause::{PreparedStartupContext, RepresentationError, StartupCause},
    startup_envelope::{InnerEventHeader, InnerOwnerHeader, PolicyFailureCause, CONTEXT_BYTES},
};

#[derive(Debug)]
pub(super) enum SamplingError {
    /// Actual O-owned I/O producer site and the unchanged original error. The site is
    /// retained at the failing call, never inferred from ErrorKind or diagnostic text.
    Io {
        operation: CauseOperation,
        cause: io::Error,
    },
    Control(ControlError),
    Report {
        operation: CauseOperation,
        cause: ReportError,
    },
    Charge(ChargeError),
    Deadline(DeadlineError),
    InvalidMonitorTransition,
    InnerLeaseConsumed,
    PhaseAuthorityMismatch,
    UnexpectedPhase,
    SettlementReportMismatch,
    InnerIdentity(ReadyIdentityError),
    InnerCompletionAuthority,
    UnexpectedInnerEvent,
    InvalidTerminalTransition,
    TerminalAuthority,
    TerminalSize,
    TerminalDeadlineMismatch,
    PartialReportDescriptor,
}

impl SamplingError {
    /// Borrow the actual producer without cloning or surrendering its original local cause.
    /// This implemented capture covers these two I/O domains; other domains retain their
    /// original typed errors and still require their genuine operational transport integration.
    fn original_io(&self) -> Option<(CauseOperation, &io::Error)> {
        match self {
            Self::Io { operation, cause }
            | Self::Report {
                operation,
                cause: ReportError::Io(cause),
            } => Some((*operation, cause)),
            Self::Report { .. }
            | Self::Control(_)
            | Self::Charge(_)
            | Self::Deadline(_)
            | Self::InvalidMonitorTransition
            | Self::InnerLeaseConsumed
            | Self::PhaseAuthorityMismatch
            | Self::UnexpectedPhase
            | Self::SettlementReportMismatch
            | Self::InnerIdentity(_)
            | Self::InnerCompletionAuthority
            | Self::UnexpectedInnerEvent
            | Self::InvalidTerminalTransition
            | Self::TerminalAuthority
            | Self::TerminalSize
            | Self::TerminalDeadlineMismatch
            | Self::PartialReportDescriptor => None,
        }
    }

    /// Move only actual I/O producer values into the negative representation path. Other
    /// domains remain owned and explicit; this does not invent a kind, site or wrapper cause.
    pub(super) fn into_original_io(self) -> Result<(CauseOperation, io::Error), Self> {
        match self {
            Self::Io { operation, cause } => Ok((operation, cause)),
            Self::Report {
                operation,
                cause: ReportError::Io(cause),
            } => Ok((operation, cause)),
            original @ (Self::Report { .. }
            | Self::Control(_)
            | Self::Charge(_)
            | Self::Deadline(_)
            | Self::InvalidMonitorTransition
            | Self::InnerLeaseConsumed
            | Self::PhaseAuthorityMismatch
            | Self::UnexpectedPhase
            | Self::SettlementReportMismatch
            | Self::InnerIdentity(_)
            | Self::InnerCompletionAuthority
            | Self::UnexpectedInnerEvent
            | Self::InvalidTerminalTransition
            | Self::TerminalAuthority
            | Self::TerminalSize
            | Self::TerminalDeadlineMismatch
            | Self::PartialReportDescriptor) => Err(original),
        }
    }
}

impl std::fmt::Display for SamplingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "outer sampling refused: {self:?}")
    }
}

impl std::error::Error for SamplingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { cause, .. } => Some(cause),
            Self::Control(error) => Some(error),
            Self::Report { cause, .. } => Some(cause),
            Self::Charge(error) => Some(error),
            Self::Deadline(error) => Some(error),
            Self::InnerIdentity(error) => Some(error),
            Self::InvalidMonitorTransition
            | Self::InnerLeaseConsumed
            | Self::PhaseAuthorityMismatch
            | Self::UnexpectedPhase
            | Self::SettlementReportMismatch
            | Self::InnerCompletionAuthority
            | Self::UnexpectedInnerEvent
            | Self::InvalidTerminalTransition
            | Self::TerminalAuthority
            | Self::TerminalSize
            | Self::TerminalDeadlineMismatch
            | Self::PartialReportDescriptor => None,
        }
    }
}

pub(super) struct OuterSampling {
    launcher: LauncherMemory,
    tree: MemoryObserver,
    ledger: ResourceLedger,
    collector: ReportCollector,
    settings: RunSettings,
    stops: StopTimeline,
    owner_stop: Option<OwnerStopCause>,
    setup_refusal: Option<PolicyFailureCause>,
    caller_cancelled: bool,
    // Historical capability milestone, established only by observe_live's complete checked
    // within-ceiling setup observation. It is not current tick validity or Dispatch authority.
    observation_admitted: bool,
}

/// One actual O actor composes the existing production phases, I completion and terminal
/// transaction. All child custody remains in this same owner across a failed step; no automatic
/// progression replaces C's BeginMonitor/ClaimInner/ReleaseGate controls.
pub(super) struct OuterRunOwner {
    sampling: Option<OuterSampling>,
    monitor: Option<InnerMonitor>,
    phases: OuterPhases,
    caller_receive: OuterCallerReceive,
    completion: InnerCompletion,
    terminal_prepared: Option<TerminalPreparation>,
    terminal: Option<TerminalDelivery>,
    inner_settlement: Option<InnerSettlement>,
    startup_deadline: Instant,
    caller_frame_deadline: Option<Instant>,
    caller_pending: Option<OuterCallerControl>,
    cancellation: Option<CallerCancellation>,
    state: OuterRunState,
    poisoned: bool,
    // The failed actor step's actual producer instant, retained before helper publication.
    // It is distinct from an earlier completion/stop already retained in the same timeline.
    failure_event: Option<Result<StopStamp, DeadlineError>>,
    // Producer-owned state is retained separately so a failed clock comparison neither
    // erases the original event/error nor becomes an inferred admission/timeout fact.
    failure_state: Option<Result<FailureState, DeadlineError>>,
    failure_cause: Option<(CauseOperation, Result<FailureHeader, RepresentationError>)>,
    unclaimed_failure: Option<PreparationFailureDelivery>,
    // An owner stop before monitor preparation retains the original unexposed I endpoint.
    _unexposed_inner_endpoint: Option<GuardianEndpoint>,
}

enum OuterRunState {
    Startup,
    Backend,
    AwaitCompletedClose,
    AwaitInnerSettlement,
    AwaitReportEof,
    Terminal,
    Committed,
    OwnerStopped,
    CallerCancelled,
}

struct CallerCancellation {
    cutoff: Instant,
    stop: StopStamp,
    monitor: Option<MonitorStopSettlement>,
    storage: Option<FrameStorage>,
}

/// Private production progress only; none of these variants grants C evidence or cleanup credit.
pub(super) enum OuterRunProgress {
    Pending,
    Startup(PhaseProgress),
    BackendDispatched,
    BackendCompleted,
    ResourceExhausted,
    OwnerStopped { cause: OwnerStopCause },
    SetupRefused,
    TerminalCommitted,
}

/// Actual pre-monitor construction custody. Failed construction leaves all returned resources
/// and the original I endpoint here for the helper's negative settlement transaction. This state
/// supplies neither a no-child witness nor a measurement; no preparation error authorizes retry.
pub(super) struct OuterRunPreparation {
    resources: Option<OuterPreparationResources>,
    attempted: bool,
    failure_event: Option<Result<StopStamp, DeadlineError>>,
    // Producer-owned state is retained separately so a failed clock comparison neither
    // erases the original event/error nor becomes an inferred admission/timeout fact.
    failure_state: Option<Result<FailureState, DeadlineError>>,
    // Required metadata is recorded before any optional Display formatting. A failed capture
    // retains its actual typed checking error; absence is never a measurement or timeout label.
    failure_cause: Option<Result<FailureHeader, RepresentationError>>,
    failure_operation: Option<CauseOperation>,
    failure_delivery: Option<PreparationFailureDelivery>,
    failure_poisoned: bool,
}

/// Provisional helper progress only. The original error stays owned by the helper caller.
/// A genuine resource observation transfers the SAME actual sampling/monitor resources.
pub(super) enum PreparationFailureProgress {
    Pending,
    Committed,
    OwnerStopped(OuterRunOwner),
}

/// A genuine unclaimed startup failure carries no report/measurement/inner settlement.
/// OwnerStopped resumes the SAME retained actor after an actual independent resource sample.
pub(super) enum ActivatedFailureProgress {
    Pending,
    Committed,
    OwnerStopped,
}

struct PreparationFailureDelivery {
    cutoff: Instant,
    send: IncrementalSend,
    monitor_settled: bool,
    collector_stopped: bool,
    committed: bool,
}

struct OuterPreparationResources {
    sampling_preparation: Option<SamplingPreparation>,
    sampling: Option<OuterSampling>,
    inner_endpoint: Option<GuardianEndpoint>,
    phases: Option<OuterPhases>,
    caller_receive: Option<OuterCallerReceive>,
    completion: Option<InnerCompletion>,
    terminal: Option<TerminalPreparation>,
    monitor: Option<InnerMonitor>,
    startup_deadline: Option<Instant>,
}

impl OuterRunPreparation {
    /// Capture ownership without allocating a buffer, cloning settings or starting any role.
    pub(super) fn new(
        launcher: LauncherMemory,
        settings: RunSettings,
        inner_endpoint: GuardianEndpoint,
        negative_storage: FrameStorage,
    ) -> Self {
        Self {
            resources: Some(OuterPreparationResources {
                sampling_preparation: Some(SamplingPreparation::new(launcher, settings)),
                sampling: None,
                inner_endpoint: Some(inner_endpoint),
                phases: None,
                caller_receive: None,
                completion: None,
                terminal: Some(TerminalPreparation::new(negative_storage)),
                monitor: None,
                startup_deadline: None,
            }),
            attempted: false,
            failure_event: None,
            failure_state: None,
            failure_cause: None,
            failure_operation: None,
            failure_delivery: None,
            failure_poisoned: false,
        }
    }

    /// One attempted activation. A failed step returns its original error while this SAME
    /// preparation still owns every successfully returned object. No implicit settlement occurs.
    pub(super) fn prepare(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        caller_pin: &OwnedFd,
    ) -> Result<OuterRunOwner, SamplingError> {
        if self.attempted {
            return Err(SamplingError::InvalidMonitorTransition);
        }
        self.attempted = true;
        if let Err(error) = self.prepare_resources(outer, caller, caller_pin) {
            // O observes the failed construction here, before returning/publishing its cause
            // or doing diagnostic encoding. A clock error is retained without retry. If a real
            // memory stop already exists, capture_once preserves its earlier actual event.
            // A constructor work check is itself a stop producer. Retain its ORIGINAL
            // event (including capture failure), rather than sampling again at this return.
            let work_stop = self
                .resources
                .as_ref()
                .and_then(|resources| resources.sampling_preparation.as_ref())
                .and_then(SamplingPreparation::work_stop);
            self.failure_event = Some(match work_stop {
                Some(event) => event,
                None => match self
                    .resources
                    .as_mut()
                    .and_then(|resources| resources.sampling.as_mut())
                {
                    Some(sampling) => sampling.stops.capture_once(StopOrigin::Outer),
                    None => StopStamp::capture(StopOrigin::Outer),
                },
            });
            self.capture_failure_cause(&error);
            return Err(error);
        }
        self.activate_resources()
    }

    fn activate_resources(&mut self) -> Result<OuterRunOwner, SamplingError> {
        let resources = self
            .resources
            .take()
            .ok_or(SamplingError::InvalidMonitorTransition)?;
        match resources {
            OuterPreparationResources {
                sampling_preparation: None,
                sampling: Some(sampling),
                inner_endpoint,
                phases: Some(phases),
                caller_receive: Some(caller_receive),
                completion: Some(completion),
                terminal: Some(terminal),
                monitor,
                startup_deadline: Some(startup_deadline),
            } if sampling.owner_stop.is_some() || monitor.is_some() => {
                let stopped = sampling.owner_stop.is_some();
                Ok(OuterRunOwner {
                    sampling: Some(sampling),
                    monitor,
                    phases,
                    caller_receive,
                    completion,
                    terminal_prepared: Some(terminal),
                    terminal: None,
                    inner_settlement: None,
                    startup_deadline,
                    caller_frame_deadline: None,
                    caller_pending: None,
                    cancellation: None,
                    state: if stopped {
                        OuterRunState::OwnerStopped
                    } else {
                        OuterRunState::Startup
                    },
                    poisoned: false,
                    failure_event: None,
                    failure_state: None,
                    failure_cause: None,
                    unclaimed_failure: None,
                    _unexposed_inner_endpoint: inner_endpoint,
                })
            }
            resources => {
                self.resources = Some(resources);
                Err(SamplingError::InvalidMonitorTransition)
            }
        }
    }

    /// Original O construction-failure event. No failure, or unavailable capture, is not a
    /// permission to start settlement at receipt time. This method creates no new event.
    pub(super) fn failure_stop(&self) -> Result<Option<StopStamp>, DeadlineError> {
        self.failure_event.transpose()
    }

    /// Fixed original-cause custody only. Reading it grants no sender/site, no-child,
    /// measurement, terminal delivery or settlement authority.
    pub(super) fn failure_cause(&self) -> Option<&Result<FailureHeader, RepresentationError>> {
        self.failure_cause.as_ref()
    }

    fn capture_failure_cause(&mut self, error: &SamplingError) {
        let Some(Ok(stop)) = self.failure_event else {
            return;
        };
        let Some(resources) = self.resources.as_ref() else {
            return;
        };
        let settings = match (&resources.sampling, &resources.sampling_preparation) {
            (Some(sampling), _) => &sampling.settings,
            (None, Some(preparation)) => preparation.settings(),
            (None, None) => return,
        };
        let admitted = resources
            .sampling
            .as_ref()
            .is_some_and(|sampling| sampling.observation_admitted);
        self.failure_state = Some(FailureState::capture(settings, stop, admitted));
        let Some(Ok(state)) = self.failure_state else {
            return;
        };
        // Metadata capture borrows the same original error before diagnostics or publication.
        // No text buffer is needed, and a representation error never replaces that original.
        if let Some((operation, cause)) = retain_io_failure(error, settings, stop, state) {
            self.failure_operation = Some(operation);
            self.failure_cause = Some(cause);
        }
    }

    /// Finite negative progress after this SAME preparation failed. The helper must retain
    /// the unchanged original SamplingError throughout these borrowed steps. No normal return
    /// occurs on a missing buffer, partial send, failed observation or unconfirmed M custody.
    pub(super) fn failure_step(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<PreparationFailureProgress, SamplingError> {
        if self.failure_poisoned || !self.attempted || self.failure_event.is_none() {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        self.failure_poisoned = true;
        let result = self.advance_failure(outer, caller);
        if result.is_ok() {
            self.failure_poisoned = false;
        }
        result
    }

    fn advance_failure(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<PreparationFailureProgress, SamplingError> {
        if self
            .failure_delivery
            .as_ref()
            .is_some_and(|delivery| delivery.committed)
        {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        if self.failure_delivery.is_none() {
            self.prepare_failure_delivery()?;
        }
        let delivery = self
            .failure_delivery
            .as_mut()
            .ok_or(SamplingError::InvalidTerminalTransition)?;
        let cutoff = delivery.cutoff;
        if Instant::now() >= cutoff {
            return Err(SamplingError::Deadline(DeadlineError::Expired));
        }
        caller
            .transport()
            .refuse_observable_eof()
            .map_err(SamplingError::Control)?;
        outer
            .require_creator_live()
            .map_err(|error| SamplingError::Io {
                operation: CauseOperation::OwnerProtection,
                cause: io::Error::other(error),
            })?;
        let resources = self
            .resources
            .as_mut()
            .ok_or(SamplingError::InvalidMonitorTransition)?;
        if let Some(sampling) = resources.sampling.as_mut() {
            // A failed original observation is never recovered from historical .ok() peaks.
            // This is a new complete observation under the same full conservative ledger;
            // an independently established ceiling breach wins over the retained IO candidate.
            sampling.ledger.begin_observation();
            let tick = observe_live(
                &mut sampling.launcher,
                &mut sampling.tree,
                &mut sampling.ledger,
                &sampling.settings,
                &sampling.stops,
                outer,
                caller,
            )?;
            if matches!(tick, MemoryTick::Exhausted(_)) {
                sampling.owner_stop = Some(OwnerStopCause::ResourceExhausted);
                sampling
                    .stops
                    .capture_once(StopOrigin::Outer)
                    .map_err(SamplingError::Deadline)?;
                sampling
                    .collector
                    .begin_owner_stop()
                    .map_err(|cause| SamplingError::Report {
                        operation: CauseOperation::ReportCollection,
                        cause,
                    })?;
                // Drop only the zero-progress negative frame. A partial negative cannot be
                // replaced with a resource commit on the same stream.
                if delivery.send.has_partial_frame() {
                    return Err(SamplingError::UnexpectedPhase);
                }
                let retired = self
                    .failure_delivery
                    .take()
                    .ok_or(SamplingError::InvalidTerminalTransition)?;
                let cutoff = retired.cutoff;
                let monitor_settled = retired.monitor_settled;
                let collector_stopped = retired.collector_stopped;
                let committed = retired.committed;
                let storage = match retired.send.retire_unsent() {
                    Ok(storage) => storage,
                    Err(send) => {
                        self.failure_delivery = Some(PreparationFailureDelivery {
                            cutoff,
                            send,
                            monitor_settled,
                            collector_stopped,
                            committed,
                        });
                        return Err(SamplingError::UnexpectedPhase);
                    }
                };
                self.resources
                    .as_mut()
                    .and_then(|resources| resources.terminal.as_mut())
                    .ok_or(SamplingError::InvalidTerminalTransition)?
                    .commit = Some(storage);
                return self
                    .activate_resources()
                    .map(PreparationFailureProgress::OwnerStopped);
            }
        }
        // Preparation alone cannot spawn M: only the activated actor's authenticated
        // BeginMonitor calls spawn. Still use the retained monitor's real stop/reap step if a
        // returned monitor exists; an absent returned factory object is never a fake reap.
        if !delivery.monitor_settled {
            if let Some(monitor) = resources.monitor.as_mut() {
                if !matches!(monitor.state, InnerMonitorState::Prepared) {
                    return Err(SamplingError::InvalidMonitorTransition);
                }
                if monitor
                    .monitor
                    .stop_monitor_step(cutoff)
                    .map_err(|cause| SamplingError::Io {
                        operation: CauseOperation::MonitorStop,
                        cause,
                    })?
                    .is_none()
                {
                    return Ok(PreparationFailureProgress::Pending);
                }
            }
            delivery.monitor_settled = true;
            // No I helper was spawned during construction. Retire only this owned unexposed
            // endpoint after M's actual prepared command/writer or real Child custody settles.
            drop(resources.inner_endpoint.take());
        }
        let collector = match (&mut resources.sampling, &mut resources.sampling_preparation) {
            (Some(sampling), _) => Some(&mut sampling.collector),
            (None, Some(preparation)) => preparation.collector.as_mut(),
            (None, None) => return Err(SamplingError::InvalidMonitorTransition),
        };
        if let Some(collector) = collector {
            if !delivery.collector_stopped {
                // Entry only retires the owned writer. The actor checks its original cutoff
                // above; bounded draining below retains that same cutoff inside the collector.
                collector
                    .begin_owner_stop()
                    .map_err(|cause| SamplingError::Report {
                        operation: CauseOperation::ReportCollection,
                        cause,
                    })?;
                delivery.collector_stopped = true;
            }
            if !collector
                .drain_owner_stop(Some(cutoff))
                .map_err(|cause| SamplingError::Report {
                    operation: CauseOperation::ReportCollection,
                    cause,
                })?
            {
                return Ok(PreparationFailureProgress::Pending);
            }
        }
        // No collector was returned in the other branch; no pipe EOF/report or measurement
        // claim is invented. The actual no-M preparation boundary exposed no external writer.
        let sent = delivery
            .send
            .advance(&caller.transport(), &[], cutoff)
            .map_err(SamplingError::Control)?;
        if Instant::now() >= cutoff {
            return Err(SamplingError::Deadline(DeadlineError::Expired));
        }
        if sent {
            delivery.committed = true;
            Ok(PreparationFailureProgress::Committed)
        } else {
            Ok(PreparationFailureProgress::Pending)
        }
    }

    fn prepare_failure_delivery(&mut self) -> Result<(), SamplingError> {
        // Only this retained constructor can have elected original work expiry without
        // attempting an observation. Admission=false or missing peaks cannot select it.
        let constructor_stop = self
            .resources
            .as_ref()
            .and_then(|resources| resources.sampling_preparation.as_ref())
            .and_then(SamplingPreparation::work_stop);
        if let Some(event) = constructor_stop {
            let stop = event.map_err(SamplingError::Deadline)?;
            if self.failure_stop().map_err(SamplingError::Deadline)? != Some(stop) {
                return Err(SamplingError::InvalidTerminalTransition);
            }
            let resources = self
                .resources
                .as_mut()
                .ok_or(SamplingError::InvalidMonitorTransition)?;
            if resources.sampling.is_some() || resources.monitor.is_some() {
                return Err(SamplingError::InvalidTerminalTransition);
            }
            let preparation = resources
                .sampling_preparation
                .as_ref()
                .ok_or(SamplingError::InvalidMonitorTransition)?;
            let settings = preparation.settings();
            if !settings
                .work_deadline
                .expired_at(stop)
                .map_err(SamplingError::Deadline)?
            {
                return Err(SamplingError::InvalidTerminalTransition);
            }
            let header = super::outer_failure::ConstructorTimeoutHeader {
                identity: settings.identity,
                authority: settings.authority,
                stop,
            };
            let mut timeline =
                StopTimeline::prepare(settings.started).map_err(SamplingError::Deadline)?;
            timeline.observe(stop).map_err(SamplingError::Deadline)?;
            let cutoff = observation_deadline(settings, &timeline)?
                .ok_or(SamplingError::InvalidTerminalTransition)?;
            let storage = resources
                .terminal
                .as_mut()
                .and_then(|terminal| terminal.commit.take())
                .ok_or(SamplingError::InvalidTerminalTransition)?;
            let frame = storage
                .encode(&header.commit())
                .map_err(SamplingError::Control)?;
            self.failure_delivery = Some(PreparationFailureDelivery {
                cutoff,
                send: IncrementalSend::new(frame),
                monitor_settled: false,
                collector_stopped: false,
                committed: false,
            });
            return Ok(());
        }
        let header = match self.failure_cause.as_ref() {
            Some(Ok(header)) => *header,
            Some(Err(error)) => {
                // These are actual metadata-capture predicates, before optional formatting.
                // The unchanged original error and checking source stay owned in O; this
                // branch reports integrity and MUST NOT replay a guessed original cause.
                let predicate = match error {
                    RepresentationError::UnnamedIoKind => {
                        CauseIntegrityPredicate::UnnameableOriginalKind
                    }
                    RepresentationError::OsKindMismatch => {
                        CauseIntegrityPredicate::OriginalOsKindMismatch
                    }
                    RepresentationError::MissingOsCode | RepresentationError::OsPayloadMismatch => {
                        CauseIntegrityPredicate::MalformedCauseMetadata
                    }
                    RepresentationError::InvalidContextBound
                    | RepresentationError::Reservation(_)
                    | RepresentationError::ContextExceeded
                    | RepresentationError::Formatting
                    | RepresentationError::NonInstallationBackendCause
                    | RepresentationError::PolicyCauseMismatch => {
                        return Err(SamplingError::InvalidTerminalTransition);
                    }
                };
                let resources = self
                    .resources
                    .as_ref()
                    .ok_or(SamplingError::InvalidMonitorTransition)?;
                let settings = match (&resources.sampling, &resources.sampling_preparation) {
                    (Some(sampling), _) => &sampling.settings,
                    (None, Some(preparation)) => preparation.settings(),
                    (None, None) => return Err(SamplingError::InvalidMonitorTransition),
                };
                FailureHeader {
                    identity: settings.identity,
                    authority: settings.authority,
                    stop: self
                        .failure_stop()
                        .map_err(SamplingError::Deadline)?
                        .ok_or(SamplingError::InvalidTerminalTransition)?,
                    operation: self
                        .failure_operation
                        .ok_or(SamplingError::InvalidTerminalTransition)?,
                    state: self
                        .failure_state
                        .transpose()
                        .map_err(SamplingError::Deadline)?
                        .ok_or(SamplingError::InvalidTerminalTransition)?,
                    representation: FailureRepresentation::Integrity { predicate },
                }
            }
            None => return Err(SamplingError::InvalidTerminalTransition),
        };
        let resources = self
            .resources
            .as_mut()
            .ok_or(SamplingError::InvalidMonitorTransition)?;
        let cutoff = match (&mut resources.sampling, &mut resources.sampling_preparation) {
            (Some(sampling), _) => {
                sampling
                    .stops
                    .observe(header.stop)
                    .map_err(SamplingError::Deadline)?;
                observation_deadline(&sampling.settings, &sampling.stops)?
                    .ok_or(SamplingError::InvalidTerminalTransition)?
            }
            (None, Some(preparation)) => {
                let mut stop = StopTimeline::prepare(preparation.settings.started)
                    .map_err(SamplingError::Deadline)?;
                stop.observe(header.stop).map_err(SamplingError::Deadline)?;
                observation_deadline(&preparation.settings, &stop)?
                    .ok_or(SamplingError::InvalidTerminalTransition)?
            }
            (None, None) => return Err(SamplingError::InvalidMonitorTransition),
        };
        let terminal = resources
            .terminal
            .as_mut()
            .ok_or(SamplingError::InvalidTerminalTransition)?;
        // Only an actual returned preallocated frame is consumed. Missing/failed storage
        // never creates a replacement buffer, an allowance, or a successful negative receipt.
        let frame = terminal
            .commit
            .take()
            .ok_or(SamplingError::InvalidTerminalTransition)?
            .encode(&NegativeCommit::new(header, &[]))
            .map_err(SamplingError::Control)?;
        self.failure_delivery = Some(PreparationFailureDelivery {
            cutoff,
            send: IncrementalSend::new(frame),
            monitor_settled: false,
            collector_stopped: false,
            committed: false,
        });
        Ok(())
    }

    fn prepare_resources(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        caller_pin: &OwnedFd,
    ) -> Result<(), SamplingError> {
        let resources = self
            .resources
            .as_mut()
            .ok_or(SamplingError::InvalidMonitorTransition)?;
        let original = resources
            .sampling_preparation
            .as_ref()
            .ok_or(SamplingError::InvalidMonitorTransition)?;
        resources.startup_deadline = Some(
            original
                .settings()
                .startup_deadline()
                .map_err(SamplingError::Deadline)?,
        );
        resources.phases = Some(OuterPhases::prepare()?);
        resources.caller_receive =
            Some(OuterCallerReceive::prepare().map_err(SamplingError::Control)?);
        resources
            .caller_receive
            .as_ref()
            .ok_or(SamplingError::InvalidMonitorTransition)?
            .reserved_bytes()
            .map_err(SamplingError::Control)?;
        resources.completion = Some(InnerCompletion::prepare()?);
        resources
            .terminal
            .as_mut()
            .ok_or(SamplingError::InvalidTerminalTransition)?
            .prepare_descriptor()?;
        let preparation = resources
            .sampling_preparation
            .take()
            .ok_or(SamplingError::InvalidMonitorTransition)?;
        let parts = match preparation.prepare(outer) {
            Ok(parts) => parts,
            Err(failure) => {
                resources.sampling_preparation = Some(failure.preparation);
                return Err(failure.error);
            }
        };
        resources.sampling = Some(OuterSampling::from_parts(parts));
        let sampling = resources
            .sampling
            .as_mut()
            .ok_or(SamplingError::InvalidMonitorTransition)?;
        // Store complete sampling custody before its first fallible observation. Failure is
        // not a zero peak or a StartupTimeoutBeforeObservation claim.
        sampling.tick(outer, caller)?;
        if sampling.owner_stop.is_none() {
            resources.monitor = Some(sampling.prepare_inner_monitor(
                outer,
                caller_pin,
                &mut resources.inner_endpoint,
            )?);
        }
        Ok(())
    }
}

impl OuterRunOwner {
    /// Errors retain this same actor/actual monitor owner for whole-chain cancellation. The helper
    /// executor cannot turn a failed/partial terminal step into normal O exit or classification.
    pub(super) fn tick(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<OuterRunProgress, SamplingError> {
        if self.poisoned || matches!(self.state, OuterRunState::Committed) {
            return Err(SamplingError::InvalidMonitorTransition);
        }
        self.poisoned = true;
        let result = self.advance(outer, caller);
        if result.is_ok() {
            self.poisoned = false;
        } else {
            // Capture once at this actual failure boundary, before the helper can publish or
            // encode the original error. Failed clock capture is retained without retry.
            let event = StopStamp::capture(StopOrigin::Outer);
            self.failure_event = Some(match event {
                Ok(stamp) => {
                    let timeline = match (&mut self.sampling, &mut self.terminal) {
                        (Some(sampling), _) => Some(&mut sampling.stops),
                        (None, Some(terminal)) => Some(&mut terminal.sampling.stops),
                        (None, None) => None,
                    };
                    match timeline {
                        Some(timeline) => timeline.observe(stamp).map(|_| stamp),
                        None => Ok(stamp),
                    }
                }
                Err(error) => Err(error),
            });
            if let (Err(original), Some(Ok(stop))) = (&result, self.failure_event) {
                let settings = match (&self.sampling, &self.terminal) {
                    (Some(sampling), _) => Some(&sampling.settings),
                    (None, Some(terminal)) => Some(&terminal.sampling.settings),
                    (None, None) => None,
                };
                if let Some(settings) = settings {
                    // Required producer metadata precedes optional diagnostics. No new buffer
                    // or reconstructed error is created; the same original stays in result.
                    let admitted = match (&self.sampling, &self.terminal) {
                        (Some(sampling), _) => sampling.observation_admitted,
                        (None, Some(terminal)) => terminal.sampling.observation_admitted,
                        (None, None) => false,
                    };
                    self.failure_state = Some(FailureState::capture(settings, stop, admitted));
                    if let Some(Ok(state)) = self.failure_state {
                        self.failure_cause = retain_io_failure(original, settings, stop, state);
                    }
                }
            }
        }
        result
    }

    /// Borrow the actual first failed-step clock fact. No consumer may substitute receipt time
    /// for absent/failed capture, or interpret this stamp as measurement or role settlement.
    pub(super) fn failure_stop(&self) -> Result<Option<StopStamp>, DeadlineError> {
        self.failure_event.transpose()
    }

    /// Actual producer milestone/election only; caller authentication and settlement remain
    /// required before either fact selects any result. Failed comparison is retained explicitly.
    pub(super) fn failure_state(&self) -> Result<Option<FailureState>, DeadlineError> {
        self.failure_state.transpose()
    }

    /// Producer metadata alone supplies no independently authenticated admission state, public
    /// classification or settlement. Unsupported non-I/O domains retain their original error.
    pub(super) fn failure_cause(
        &self,
    ) -> Option<&(CauseOperation, Result<FailureHeader, RepresentationError>)> {
        self.failure_cause.as_ref()
    }

    /// Negative progress is available only before any actual I claim. The helper retains its
    /// unchanged original error while this same owner keeps real M/collector/control custody.
    /// Normal O exit remains provisional until C's actual outer/L/capture/creator settlement.
    pub(super) fn unclaimed_failure_step(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<ActivatedFailureProgress, SamplingError> {
        if !self.poisoned || !matches!(self.state, OuterRunState::Startup) {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        if self
            .monitor
            .as_mut()
            .ok_or(SamplingError::InvalidMonitorTransition)?
            .monitor
            .namespace()
            .claimed_init_terminated()
            .map_err(|cause| SamplingError::Io {
                operation: CauseOperation::MonitorClaim,
                cause,
            })?
            .is_some()
        {
            // The actual stored claim, not a coarse phase enum, excludes this branch even
            // when binding a newly stored I claim failed before the phase advanced.
            return Err(SamplingError::InvalidMonitorTransition);
        }
        if self
            .phases
            .pending_reply
            .as_ref()
            .is_some_and(|reply| reply.send.has_partial_frame())
        {
            return Err(SamplingError::UnexpectedPhase);
        }
        if self.unclaimed_failure.is_none() {
            let header = match self.failure_cause.as_ref() {
                Some((_, Ok(header))) => *header,
                // Unrepresentable causes retain the actual original/checking errors. This
                // slice cannot invent a cause or reuse a policy-only source-loss exception.
                Some((_, Err(_))) | None => return Err(SamplingError::InvalidTerminalTransition),
            };
            let sampling = self
                .sampling
                .as_ref()
                .ok_or(SamplingError::InvalidMonitorTransition)?;
            let cutoff = observation_deadline(&sampling.settings, &sampling.stops)?
                .ok_or(SamplingError::InvalidTerminalTransition)?;
            if Instant::now() >= cutoff {
                return Err(SamplingError::Deadline(DeadlineError::Expired));
            }
            // Zero-progress replies cannot authorize the caller. Any already complete queued
            // phase remains in C's original framer and must retain its real rights for cleanup.
            drop(self.phases.pending_reply.take());
            let frame = self
                .terminal_prepared
                .as_mut()
                .and_then(|prepared| prepared.commit.take())
                .ok_or(SamplingError::InvalidTerminalTransition)?
                .encode(&NegativeCommit::new(header, &[]))
                .map_err(SamplingError::Control)?;
            self.unclaimed_failure = Some(PreparationFailureDelivery {
                cutoff,
                send: IncrementalSend::new(frame),
                monitor_settled: false,
                collector_stopped: false,
                committed: false,
            });
        }
        let delivery = self
            .unclaimed_failure
            .as_mut()
            .ok_or(SamplingError::InvalidTerminalTransition)?;
        if delivery.committed || Instant::now() >= delivery.cutoff {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        let sampling = self
            .sampling
            .as_mut()
            .ok_or(SamplingError::InvalidMonitorTransition)?;
        sampling.ledger.begin_observation();
        // Fresh complete accounting is attempted before every finite reap/send step. An
        // unavailable new observation refuses this attempt; it never recovers stale peaks.
        let tick = sampling.tick_observation(outer, caller)?;
        if matches!(tick, MemoryTick::Exhausted(_)) {
            if delivery.send.has_partial_frame() {
                return Err(SamplingError::UnexpectedPhase);
            }
            let retired = self
                .unclaimed_failure
                .take()
                .ok_or(SamplingError::InvalidTerminalTransition)?;
            let PreparationFailureDelivery {
                cutoff,
                send,
                monitor_settled,
                collector_stopped,
                committed,
            } = retired;
            let storage = match send.retire_unsent() {
                Ok(storage) => storage,
                Err(send) => {
                    self.unclaimed_failure = Some(PreparationFailureDelivery {
                        cutoff,
                        send,
                        monitor_settled,
                        collector_stopped,
                        committed,
                    });
                    return Err(SamplingError::UnexpectedPhase);
                }
            };
            self.terminal_prepared
                .as_mut()
                .ok_or(SamplingError::InvalidTerminalTransition)?
                .commit = Some(storage);
            self.state = OuterRunState::OwnerStopped;
            self.poisoned = false;
            return Ok(ActivatedFailureProgress::OwnerStopped);
        }
        if !delivery.collector_stopped {
            sampling
                .collector
                .begin_owner_stop()
                .map_err(|cause| SamplingError::Report {
                    operation: CauseOperation::ReportCollection,
                    cause,
                })?;
            delivery.collector_stopped = true;
        }
        if !delivery.monitor_settled {
            if self
                .monitor
                .as_mut()
                .ok_or(SamplingError::InvalidMonitorTransition)?
                .monitor
                .stop_monitor_step(delivery.cutoff)
                .map_err(|cause| SamplingError::Io {
                    operation: CauseOperation::MonitorStop,
                    cause,
                })?
                .is_none()
            {
                return Ok(ActivatedFailureProgress::Pending);
            }
            // Positive actual M reap/NotCreated is separate from unclaimed-tree termination.
            // I has no claim token here; actual normal O termination later proves its tree.
            delivery.monitor_settled = true;
            drop(self._unexposed_inner_endpoint.take());
        }
        if Instant::now() >= delivery.cutoff {
            return Err(SamplingError::Deadline(DeadlineError::Expired));
        }
        if delivery
            .send
            .advance(&caller.transport(), &[], delivery.cutoff)
            .map_err(SamplingError::Control)?
        {
            if Instant::now() >= delivery.cutoff {
                return Err(SamplingError::Deadline(DeadlineError::Expired));
            }
            delivery.committed = true;
            self.state = OuterRunState::Committed;
            Ok(ActivatedFailureProgress::Committed)
        } else {
            Ok(ActivatedFailureProgress::Pending)
        }
    }

    fn advance(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<OuterRunProgress, SamplingError> {
        // The original C stream is consumed before I events/EOF or startup side effects.
        // A partial frame suspends those transitions, never accounting, liveness or the cutoff.
        let mut caller_control = self.caller_pending.take();
        if matches!(
            self.state,
            OuterRunState::Startup | OuterRunState::Backend | OuterRunState::AwaitCompletedClose
        ) {
            let sampling = self
                .sampling
                .as_mut()
                .ok_or(SamplingError::InvalidTerminalTransition)?;
            sampling.tick(outer, caller)?;
            if sampling.owner_stop.is_none()
                && sampling.setup_refusal.is_none()
                && caller_control.is_none()
            {
                let cutoff = observation_deadline(&sampling.settings, &sampling.stops)?;
                let cutoff = if matches!(self.state, OuterRunState::Startup) {
                    Some(cutoff.map_or(self.startup_deadline, |cutoff| {
                        cutoff.min(self.startup_deadline)
                    }))
                } else {
                    cutoff
                };
                let cutoff = match (cutoff, self.caller_frame_deadline) {
                    (Some(original), Some(frame)) => Some(original.min(frame)),
                    (original, None) => original,
                    (None, Some(frame)) => Some(frame),
                };
                caller_control = self
                    .caller_receive
                    .step(caller, cutoff)
                    .map_err(SamplingError::Control)?;
                if self.caller_receive.has_partial_frame() {
                    if self.caller_frame_deadline.is_none() {
                        let cap = Instant::now()
                            .checked_add(std::time::Duration::from_secs(3))
                            .ok_or(SamplingError::Deadline(DeadlineError::Unrepresentable))?;
                        self.caller_frame_deadline =
                            Some(cutoff.map_or(cap, |cutoff| cutoff.min(cap)));
                    }
                    return Ok(OuterRunProgress::Pending);
                }
                if caller_control.is_some() {
                    self.caller_frame_deadline = None;
                }
            }
        }
        if let Some(OuterCallerControl::Close(CallerTerminalControl::CancelClose {
            authority,
            deadline,
            stop,
        })) = caller_control.as_ref()
        {
            if stop.origin != StopOrigin::Caller {
                return Err(SamplingError::TerminalAuthority);
            }
            if self
                .phases
                .pending_reply
                .as_ref()
                .is_some_and(|reply| reply.send.has_partial_frame())
            {
                return Err(SamplingError::UnexpectedPhase);
            }
            let sampling = self
                .sampling
                .as_mut()
                .ok_or(SamplingError::InvalidTerminalTransition)?;
            let cutoff = adopt_caller_close(sampling, *authority, *deadline, *stop)?;
            sampling
                .collector
                .begin_owner_stop()
                .map_err(|cause| SamplingError::Report {
                    operation: CauseOperation::ReportCollection,
                    cause,
                })?;
            sampling.caller_cancelled = true;
            // No bytes from a zero-progress pending reply entered the stream. Never splice
            // a partial response, invent I Completed, or transfer original local C cause.
            drop(self.phases.pending_reply.take());
            let prepared = self
                .terminal_prepared
                .take()
                .ok_or(SamplingError::InvalidTerminalTransition)?;
            self.cancellation = Some(CallerCancellation {
                cutoff,
                stop: *stop,
                monitor: None,
                storage: prepared.commit,
            });
            self.state = OuterRunState::CallerCancelled;
            caller_control = None;
        }
        if matches!(self.state, OuterRunState::CallerCancelled) {
            return self.cancel_caller_step(outer, caller);
        }
        if self.sampling.as_ref().is_some_and(|sampling| {
            sampling.owner_stop.is_some() || sampling.setup_refusal.is_some()
        }) {
            // A later lower RSS sample cannot undo the genuine earlier owner stop or
            // reopen startup/Dispatch/report progress while the same owner settles children.
            self.state = OuterRunState::OwnerStopped;
        }
        if matches!(self.state, OuterRunState::OwnerStopped) {
            if self
                .phases
                .pending_reply
                .as_ref()
                .is_some_and(|reply| reply.send.has_partial_frame())
            {
                // A stop commit cannot replace or splice an already started C/O phase frame.
                // Refusal retains the actual role owners for the caller's bounded cancellation.
                return Err(SamplingError::UnexpectedPhase);
            }
            // Zero-progress replies never entered the stream. Any fully emitted reply remains
            // queued for C's same original framer; neither case authorizes more O phase work.
            drop(self.phases.pending_reply.take());
            // Keep complete actual observations and discarded-reader work during every real
            // M stop/reap attempt. M reap is separate from actual outer INIT termination.
            let sampling = self
                .sampling
                .as_mut()
                .ok_or(SamplingError::InvalidMonitorTransition)?;
            sampling.tick(outer, caller)?;
            let cutoff = observation_deadline(&sampling.settings, &sampling.stops)?
                .ok_or(SamplingError::InvalidTerminalTransition)?;
            if sampling.owner_stop.is_none() && sampling.setup_refusal.is_some() {
                // I already owns its real claim and the negative installer child. Preserve
                // its original C refusal/context channel until C closes the exclusive lease.
                // I's actual pidfd termination, not an EOF/frame/boolean, ends this wait.
                let monitor = self
                    .monitor
                    .as_mut()
                    .ok_or(SamplingError::InvalidMonitorTransition)?;
                if !monitor
                    .monitor
                    .namespace()
                    .init_terminated()
                    .map_err(|cause| SamplingError::Io {
                        operation: CauseOperation::MonitorStop,
                        cause,
                    })?
                {
                    return Ok(OuterRunProgress::SetupRefused);
                }
            }
            let settlement = match self.monitor.as_mut() {
                Some(monitor) => monitor.monitor.stop_monitor_step(cutoff).map_err(|cause| {
                    SamplingError::Io {
                        operation: CauseOperation::MonitorStop,
                        cause,
                    }
                })?,
                None => Some(MonitorStopSettlement::NotCreated),
            };
            let Some(settlement) = settlement else {
                return sampling.stop_progress();
            };
            // For the nonreport branch only, actual O termination later confirms its unclaimed
            // tree (FR034-279/285..292). No InnerSettlement, report EOF or seal is manufactured.
            let sampling = self
                .sampling
                .take()
                .ok_or(SamplingError::InvalidTerminalTransition)?;
            self.terminal = Some(
                self.terminal_prepared
                    .take()
                    .ok_or(SamplingError::InvalidTerminalTransition)?
                    .begin_owner_stop(sampling, settlement, cutoff)?,
            );
            self.state = OuterRunState::Terminal;
            return Ok(OuterRunProgress::Pending);
        }

        if matches!(self.state, OuterRunState::Terminal) {
            return match self
                .terminal
                .as_mut()
                .ok_or(SamplingError::InvalidTerminalTransition)?
                .tick(outer, caller, &mut self.caller_receive)?
            {
                TerminalProgress::Pending => Ok(OuterRunProgress::Pending),
                TerminalProgress::Committed => {
                    self.state = OuterRunState::Committed;
                    Ok(OuterRunProgress::TerminalCommitted)
                }
            };
        }
        let sampling = self
            .sampling
            .as_mut()
            .ok_or(SamplingError::InvalidTerminalTransition)?;
        match self.state {
            OuterRunState::Startup => {
                let progress = self.phases.tick(
                    sampling,
                    outer,
                    caller,
                    self.monitor
                        .as_mut()
                        .ok_or(SamplingError::InvalidMonitorTransition)?,
                    self.startup_deadline,
                    caller_control.take(),
                )?;
                if sampling.owner_stop.is_some() {
                    self.state = OuterRunState::OwnerStopped;
                    return sampling.stop_progress();
                }
                if matches!(progress, PhaseProgress::GateReleased) {
                    self.state = OuterRunState::Backend;
                }
                Ok(OuterRunProgress::Startup(progress))
            }
            OuterRunState::Backend => {
                match caller_control.take() {
                    Some(
                        control @ OuterCallerControl::Close(CallerTerminalControl::CompletedClose {
                            ..
                        }),
                    ) => {
                        // C may already hold genuine I Completed while O's separate I frame is
                        // queued. Retain the complete close, but grant no completion authority
                        // until this same actor authenticates its actual I completion stream.
                        self.caller_pending = Some(control);
                    }
                    Some(OuterCallerControl::Phase(_))
                    | Some(OuterCallerControl::ReadCompleted { .. })
                    | Some(OuterCallerControl::Close(CallerTerminalControl::ReadCompleted {
                        ..
                    }))
                    | Some(OuterCallerControl::Close(CallerTerminalControl::CancelClose {
                        ..
                    })) => {
                        return Err(SamplingError::UnexpectedPhase);
                    }
                    None => {}
                }
                match self.completion.tick(
                    sampling,
                    outer,
                    caller,
                    self.monitor
                        .as_ref()
                        .ok_or(SamplingError::InvalidMonitorTransition)?,
                )? {
                    CompletionProgress::Pending => Ok(OuterRunProgress::Pending),
                    CompletionProgress::Dispatched => Ok(OuterRunProgress::BackendDispatched),
                    CompletionProgress::Exhausted => {
                        self.state = OuterRunState::OwnerStopped;
                        sampling.stop_progress()
                    }
                    CompletionProgress::SetupRefused => {
                        self.state = OuterRunState::OwnerStopped;
                        Ok(OuterRunProgress::SetupRefused)
                    }
                    CompletionProgress::Completed(_) => {
                        self.state = OuterRunState::AwaitCompletedClose;
                        Ok(OuterRunProgress::BackendCompleted)
                    }
                }
            }
            OuterRunState::AwaitCompletedClose => {
                let (_tick, closed) = self
                    .terminal_prepared
                    .as_mut()
                    .ok_or(SamplingError::InvalidTerminalTransition)?
                    .receive_completed_close(
                        sampling,
                        &self.completion,
                        outer,
                        caller,
                        caller_control.take(),
                    )?;
                if sampling.owner_stop.is_some() {
                    return sampling.stop_progress();
                }
                if closed {
                    self.state = OuterRunState::AwaitInnerSettlement;
                }
                Ok(OuterRunProgress::Pending)
            }
            OuterRunState::AwaitInnerSettlement | OuterRunState::AwaitReportEof => {
                let cutoff = self
                    .terminal_prepared
                    .as_ref()
                    .ok_or(SamplingError::InvalidTerminalTransition)?
                    .settlement_deadline()?;
                if Instant::now() >= cutoff {
                    return Err(SamplingError::Deadline(DeadlineError::Expired));
                }
                sampling.tick(outer, caller)?;
                if sampling.owner_stop.is_some() {
                    return sampling.stop_progress();
                }
                if matches!(self.state, OuterRunState::AwaitInnerSettlement) {
                    self.inner_settlement = self
                        .monitor
                        .as_mut()
                        .ok_or(SamplingError::InvalidMonitorTransition)?
                        .poll_settled()?;
                    if self.inner_settlement.is_none() {
                        return Ok(OuterRunProgress::Pending);
                    }
                    self.state = OuterRunState::AwaitReportEof;
                }
                if !sampling.report_eof() {
                    return Ok(OuterRunProgress::Pending);
                }
                let sampling = self
                    .sampling
                    .take()
                    .ok_or(SamplingError::InvalidTerminalTransition)?;
                let settlement = self
                    .inner_settlement
                    .take()
                    .ok_or(SamplingError::InvalidTerminalTransition)?;
                let (report, sampling, settlement) =
                    sampling.seal_after_inner_settlement(settlement, outer, caller)?;
                self.terminal = Some(
                    self.terminal_prepared
                        .take()
                        .ok_or(SamplingError::InvalidTerminalTransition)?
                        .begin(report, sampling, settlement)?,
                );
                self.state = OuterRunState::Terminal;
                Ok(OuterRunProgress::Pending)
            }
            OuterRunState::Terminal
            | OuterRunState::Committed
            | OuterRunState::OwnerStopped
            | OuterRunState::CallerCancelled => Err(SamplingError::InvalidTerminalTransition),
        }
    }
    /// Original C cancellation has no report/measurement result. Its receipt is provisional:
    /// actual O normal exit still confirms the unclaimed outer tree, while M reap is separate.
    fn cancel_caller_step(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<OuterRunProgress, SamplingError> {
        let sampling = self
            .sampling
            .as_mut()
            .ok_or(SamplingError::InvalidTerminalTransition)?;
        let started = Instant::now();
        sampling.tick(outer, caller)?;
        if sampling.owner_stop.is_some() {
            // A real fresh independent resource/timeout candidate is never replaced by C's
            // local error. No cancellation bytes have entered the stream until the final send.
            self.state = OuterRunState::OwnerStopped;
            return sampling.stop_progress();
        }
        let cancellation = self
            .cancellation
            .as_mut()
            .ok_or(SamplingError::InvalidTerminalTransition)?;
        let cutoff = observation_deadline(&sampling.settings, &sampling.stops)?
            .ok_or(SamplingError::InvalidTerminalTransition)?;
        cancellation.cutoff = cancellation.cutoff.min(cutoff);
        if Instant::now() >= cancellation.cutoff {
            return Err(SamplingError::Deadline(DeadlineError::Expired));
        }
        if cancellation.monitor.is_none() {
            cancellation.monitor = match self.monitor.as_mut() {
                Some(monitor) => {
                    if monitor
                        .monitor
                        .namespace()
                        .claimed_init_terminated()
                        .map_err(|cause| SamplingError::Io {
                            operation: CauseOperation::MonitorStop,
                            cause,
                        })?
                        == Some(false)
                    {
                        // Only original lease EOF is primary here. No M signal or outer exit
                        // can masquerade as the claimed I's actual retained pidfd termination.
                        return Ok(OuterRunProgress::Pending);
                    }
                    monitor
                        .monitor
                        .stop_monitor_step(cancellation.cutoff)
                        .map_err(|cause| SamplingError::Io {
                            operation: CauseOperation::MonitorStop,
                            cause,
                        })?
                }
                None => Some(MonitorStopSettlement::NotCreated),
            };
            if cancellation.monitor.is_none() {
                return Ok(OuterRunProgress::Pending);
            }
        }
        let storage = cancellation
            .storage
            .take()
            .ok_or(SamplingError::InvalidTerminalTransition)?;
        let stop = sampling.stops.earliest().map_err(SamplingError::Deadline)?;
        match stop.origin {
            StopOrigin::Caller if stop == cancellation.stop => {}
            StopOrigin::Inner => {
                // This producer exists only after actual claimed-I completion authentication.
                // The retained original M/claimed pin already supplied positive termination
                // above; forwarding the earlier clock does not mint report completion custody.
                if !matches!(
                    self.completion.state,
                    InnerCompletionState::Acknowledging { .. } | InnerCompletionState::Completed(_)
                ) {
                    return Err(SamplingError::TerminalAuthority);
                }
            }
            StopOrigin::Caller | StopOrigin::Launcher | StopOrigin::Outer | StopOrigin::Backend => {
                return Err(SamplingError::TerminalAuthority);
            }
        }
        let frame = storage
            .encode(&OuterTerminalReply::Cancelled {
                authority: sampling.settings.authority,
                stop,
                peaks: sampling
                    .ledger
                    .measured_peaks()
                    .map_err(SamplingError::Charge)?,
            })
            .map_err(SamplingError::Control)?;
        let due = started
            .checked_add(super::owned::TICK)
            .ok_or(SamplingError::Deadline(DeadlineError::Unrepresentable))?;
        if Instant::now() >= due {
            cancellation.storage = Some(frame.into_storage());
            return Ok(OuterRunProgress::Pending);
        }
        if caller
            .transport()
            .send_terminal_once(&frame, cancellation.cutoff)
            .map_err(SamplingError::Control)?
        {
            self.state = OuterRunState::Committed;
            Ok(OuterRunProgress::TerminalCommitted)
        } else {
            cancellation.storage = Some(frame.into_storage());
            Ok(OuterRunProgress::Pending)
        }
    }
}

/// One allocation-free capture shared by construction and activated failures. This captures
/// only genuine producer I/O domains; caller state/authentication and projection stay separate.
fn retain_io_failure(
    error: &SamplingError,
    settings: &RunSettings,
    stop: StopStamp,
    state: FailureState,
) -> Option<(CauseOperation, Result<FailureHeader, RepresentationError>)> {
    let (operation, cause) = error.original_io()?;
    Some((
        operation,
        StartupCause::capture_io(cause).map(|cause| FailureHeader {
            identity: settings.identity,
            authority: settings.authority,
            stop,
            operation,
            state,
            representation: FailureRepresentation::Original { cause },
        }),
    ))
}

/// O's actual accounting survives collector sealing and descriptor delivery. This owner grants
/// no writer exposure or child creation, and its latest complete sample is not cleanup evidence.
pub(super) struct TerminalSampling {
    observation_admitted: bool,
    launcher: LauncherMemory,
    tree: MemoryObserver,
    ledger: ResourceLedger,
    settings: RunSettings,
    stops: StopTimeline,
}

impl TerminalSampling {
    /// Preserve every normal accounting tick while report delivery/read acknowledgement remains
    /// pending. The already consumed pipe collector cannot be drained or reopened here.
    pub(super) fn tick(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<MemoryTick, SamplingError> {
        self.ledger.begin_observation();
        let result = (|| {
            let tick = observe_live(
                &mut self.launcher,
                &mut self.tree,
                &mut self.ledger,
                &self.settings,
                &self.stops,
                outer,
                caller,
            )?;
            outer
                .require_creator_live()
                .map_err(|error| SamplingError::Io {
                    operation: CauseOperation::OwnerProtection,
                    cause: io::Error::other(error),
                })?;
            caller
                .transport()
                .refuse_observable_eof()
                .map_err(SamplingError::Control)?;
            if matches!(tick, MemoryTick::Exhausted(_)) {
                self.stops
                    .capture_once(StopOrigin::Outer)
                    .map_err(SamplingError::Deadline)?;
            }
            Ok(tick)
        })();
        if result.is_err() {
            self.ledger.begin_observation();
        }
        result
    }

    /// Actual complete normal observation and recorded peaks. Only the sample attached to the
    /// eventual successful complete commit is final; zero-progress attempts must sample again.
    /// Descriptor/read stages use this same actual source without granting final metrics credit.
    pub(super) fn observation_and_peaks(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<(MemoryTick, MeasuredPeaks), SamplingError> {
        let tick = self.tick(outer, caller)?;
        let peaks = self
            .ledger
            .measured_peaks()
            .map_err(SamplingError::Charge)?;
        Ok((tick, peaks))
    }
}

/// Retain O's original negative-send allocation before fallible run preparation. Descriptor
/// storage is prepared separately before any M/writer child. Actual live allocations remain
/// in O's observed RSS; report backing retains its full reservation.
pub(super) struct TerminalPreparation {
    descriptor: Option<FrameStorage>,
    commit: Option<FrameStorage>,
    close_deadline: Option<Instant>,
    poisoned: bool,
}

impl TerminalPreparation {
    fn new(commit: FrameStorage) -> Self {
        Self {
            descriptor: None,
            commit: Some(commit),
            close_deadline: None,
            poisoned: false,
        }
    }

    /// Keep the original negative-send reservation even when descriptor allocation fails.
    fn prepare_descriptor(&mut self) -> Result<(), SamplingError> {
        if self.descriptor.is_some() || self.commit.is_none() || self.poisoned {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        self.descriptor = Some(FrameStorage::prepare().map_err(SamplingError::Control)?);
        Ok(())
    }

    /// Receive C's settlement clock only with O's real authenticated I completion retained.
    /// Every partial frame attempt still observes L/tree and drains the report. Original None
    /// remains never-elapsing work; partial-control caps cannot mint/reset a settlement allowance.
    pub(super) fn receive_completed_close(
        &mut self,
        sampling: &mut OuterSampling,
        completion: &InnerCompletion,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        received: Option<OuterCallerControl>,
    ) -> Result<(MemoryTick, bool), SamplingError> {
        if self.poisoned
            || self.close_deadline.is_some()
            || !matches!(completion.state, InnerCompletionState::Completed(_))
        {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        self.poisoned = true;
        let tick = sampling.tick(outer, caller)?;
        if sampling.owner_stop.is_some() {
            self.poisoned = false;
            return Ok((tick, false));
        }
        let Some(received) = received else {
            self.poisoned = false;
            return Ok((tick, false));
        };
        let OuterCallerControl::Close(CallerTerminalControl::CompletedClose {
            authority,
            deadline,
            stop,
        }) = received
        else {
            return Err(SamplingError::InvalidTerminalTransition);
        };
        let deadline = adopt_caller_close(sampling, authority, deadline, stop)?;
        self.close_deadline = Some(deadline);
        self.poisoned = false;
        Ok((tick, true))
    }

    pub(super) fn settlement_deadline(&self) -> Result<Instant, SamplingError> {
        self.close_deadline
            .ok_or(SamplingError::InvalidTerminalTransition)
    }

    /// The cutoff is C's already authenticated original settlement deadline. This method
    /// creates no phase allowance; finite original T may only shorten that same cutoff.
    pub(super) fn begin(
        self,
        report: SealedReport,
        sampling: TerminalSampling,
        settlement: InnerSettlement,
    ) -> Result<TerminalDelivery, SamplingError> {
        let cutoff = self.settlement_deadline()?;
        if self.poisoned {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        let bytes = u64::try_from(report.bytes).map_err(|_| SamplingError::TerminalSize)?;
        if bytes > super::REPORT_CONTENT_BYTES {
            return Err(SamplingError::TerminalSize);
        }
        let deadline = sampling
            .settings
            .identity_deadline()
            .map_err(SamplingError::Deadline)?
            .map_or(cutoff, |original| cutoff.min(original));
        if Instant::now() >= deadline {
            return Err(SamplingError::Deadline(DeadlineError::Expired));
        }
        let authority = sampling.settings.authority;
        let descriptor = self
            .descriptor
            .ok_or(SamplingError::InvalidTerminalTransition)?
            .encode(&OuterTerminalReply::ReportDescriptor { authority, bytes })
            .map_err(SamplingError::Control)?;
        Ok(TerminalDelivery {
            sampling,
            state: TerminalState::DeliverDescriptor {
                report,
                send: IncrementalSend::new(descriptor),
            },
            commit: Some(
                self.commit
                    .ok_or(SamplingError::InvalidTerminalTransition)?,
            ),
            bytes: Some(bytes),
            deadline,
            disposition: TerminalDisposition::Report,
            cleanup: TerminalCleanupWitness::Claimed {
                _settlement: settlement,
                discarded_report: None,
            },
            caller_cancelled: false,
            poisoned: false,
        })
    }

    /// M's genuine stop/reap witness authorizes only a provisional nonreport commit. The actual
    /// discarded pipe and full backing remain owned until O exits; unclaimed I may still own a
    /// writer. C must prove actual outer INIT termination and every remaining owned settlement.
    fn begin_owner_stop(
        self,
        sampling: OuterSampling,
        monitor_stop: MonitorStopSettlement,
        deadline: Instant,
    ) -> Result<TerminalDelivery, SamplingError> {
        if self.poisoned || Instant::now() >= deadline {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        let disposition = match (sampling.owner_stop, sampling.setup_refusal) {
            (Some(cause), _) => TerminalDisposition::OwnerStop { cause },
            (None, Some(failure)) => TerminalDisposition::SetupRefused { failure },
            (None, None) => return Err(SamplingError::InvalidTerminalTransition),
        };
        let original = observation_deadline(&sampling.settings, &sampling.stops)?
            .ok_or(SamplingError::InvalidTerminalTransition)?;
        if deadline > original {
            return Err(SamplingError::TerminalDeadlineMismatch);
        }
        Ok(TerminalDelivery {
            sampling: TerminalSampling {
                observation_admitted: sampling.observation_admitted,
                launcher: sampling.launcher,
                tree: sampling.tree,
                ledger: sampling.ledger,
                settings: sampling.settings,
                stops: sampling.stops,
            },
            state: TerminalState::Commit,
            commit: Some(
                self.commit
                    .ok_or(SamplingError::InvalidTerminalTransition)?,
            ),
            bytes: None,
            deadline,
            disposition,
            cleanup: TerminalCleanupWitness::MonitorStop {
                _settlement: monitor_stop,
                discarded: sampling.collector,
            },
            caller_cancelled: false,
            poisoned: false,
        })
    }
}

enum TerminalState {
    DeliverDescriptor {
        report: SealedReport,
        send: IncrementalSend,
    },
    AwaitRead,
    Commit,
    Committed,
}

pub(super) enum TerminalProgress {
    Pending,
    /// Complete commit emission only. The executor may now exit normally; C must still prove
    /// actual normal O exit/reap through retained L, capture closure and creator settlement.
    Committed,
}

/// Actual settlement custody survives both accepted-report and abandoned-report transactions.
/// A claimed witness binds the original writer pipe and actual I/M settlement; it cannot be
/// replaced by the distinct monitor-stop witness or a resurrected collector after sealing.
enum TerminalCleanupWitness {
    Claimed {
        _settlement: InnerSettlement,
        // Zero-progress delivery abandonment retains O's actual sealed backing until O exits.
        // After full delivery C owns its duplicate; O cannot revoke it or release C's backing.
        discarded_report: Option<SealedReport>,
    },
    MonitorStop {
        _settlement: MonitorStopSettlement,
        discarded: ReportCollector,
    },
}

/// O's single terminal transaction. Every unsuccessful finite attempt returns to actual fresh
/// accounting, with no recursive commit ACK, repeated descriptor, new buffer or deadline reset.
pub(super) struct TerminalDelivery {
    sampling: TerminalSampling,
    state: TerminalState,
    commit: Option<FrameStorage>,
    bytes: Option<u64>,
    deadline: Instant,
    disposition: TerminalDisposition,
    cleanup: TerminalCleanupWitness,
    // Only a full authenticated CancelClose after descriptor delivery can abandon the original
    // read acknowledgment. The actual claimed settlement/backing stay in this same transaction.
    caller_cancelled: bool,
    poisoned: bool,
}

impl TerminalDelivery {
    pub(super) fn tick(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        receive: &mut OuterCallerReceive,
    ) -> Result<TerminalProgress, SamplingError> {
        if self.poisoned || matches!(self.state, TerminalState::Committed) {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        self.poisoned = true;
        let result = self.advance(outer, caller, receive);
        if result.is_ok() {
            self.poisoned = false;
        }
        result
    }

    /// Abandon report acceptance only after a fresh complete genuine resource observation.
    /// The actual I/M proof and conservative backing charge survive. C must discard any delivered
    /// report bytes after its authenticated stop disposition, then confirm whole-chain settlement.
    fn stop_sealed_report(&mut self) -> Result<(), SamplingError> {
        let TerminalCleanupWitness::Claimed {
            discarded_report, ..
        } = &mut self.cleanup
        else {
            return Err(SamplingError::InvalidTerminalTransition);
        };
        if discarded_report.is_some() || self.bytes.is_none() {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        match &self.state {
            TerminalState::DeliverDescriptor { send, .. } if send.has_partial_frame() => {
                // Keep the actual report, send progress and proof on refusal; never splice a
                // stop frame over a descriptor prefix (including already transferred rights).
                return Err(SamplingError::PartialReportDescriptor);
            }
            TerminalState::DeliverDescriptor { .. }
            | TerminalState::AwaitRead
            | TerminalState::Commit => {}
            TerminalState::Committed => return Err(SamplingError::InvalidTerminalTransition),
        }
        // TerminalSampling's actual observation already captured the Outer trigger once. An
        // earlier authentic completion may remain the first stop; no later resource restarts R.
        let cutoff = observation_deadline(&self.sampling.settings, &self.sampling.stops)?
            .ok_or(SamplingError::InvalidTerminalTransition)?;
        let deadline = self.deadline.min(cutoff);
        if Instant::now() >= deadline {
            return Err(SamplingError::Deadline(DeadlineError::Expired));
        }
        if matches!(self.state, TerminalState::DeliverDescriptor { .. }) {
            let TerminalState::DeliverDescriptor { report, .. } =
                std::mem::replace(&mut self.state, TerminalState::Commit)
            else {
                return Err(SamplingError::InvalidTerminalTransition);
            };
            // Only a zero-progress frame is retired. Preserve its real sealed backing; the
            // ledger never subtracts the original reservation even when C received no fd.
            *discarded_report = Some(report);
        }
        // AwaitRead remains AwaitRead: same bytes, authority, ACK and shortened original cutoff.
        self.deadline = deadline;
        self.disposition = TerminalDisposition::OwnerStop {
            cause: OwnerStopCause::ResourceExhausted,
        };
        Ok(())
    }

    fn advance(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        receive: &mut OuterCallerReceive,
    ) -> Result<TerminalProgress, SamplingError> {
        if Instant::now() >= self.deadline {
            return Err(SamplingError::Deadline(DeadlineError::Expired));
        }
        let sample_started = Instant::now();
        let (tick, peaks) = self.sampling.observation_and_peaks(outer, caller)?;
        match self.disposition {
            TerminalDisposition::Report => {
                if !matches!(
                    self.cleanup,
                    TerminalCleanupWitness::Claimed {
                        discarded_report: None,
                        ..
                    }
                ) {
                    return Err(SamplingError::InvalidTerminalTransition);
                }
                if matches!(tick, MemoryTick::Exhausted(_)) {
                    self.stop_sealed_report()?;
                }
            }
            TerminalDisposition::OwnerStop { .. } | TerminalDisposition::SetupRefused { .. } => {
                match &mut self.cleanup {
                    TerminalCleanupWitness::MonitorStop { discarded, .. } => {
                        if !matches!(self.state, TerminalState::Commit) || self.bytes.is_some() {
                            return Err(SamplingError::InvalidTerminalTransition);
                        }
                        discarded
                            .drain_owner_stop(Some(self.deadline))
                            .map_err(|cause| SamplingError::Report {
                                operation: CauseOperation::ReportCollection,
                                cause,
                            })?;
                    }
                    TerminalCleanupWitness::Claimed { .. } => {
                        // A delivered descriptor still owes its original bounded read ACK. This
                        // actual settled witness grants neither pipe resurrection nor another ACK.
                        if !matches!(self.state, TerminalState::AwaitRead | TerminalState::Commit)
                            || self.bytes.is_none()
                            || !matches!(
                                self.disposition,
                                TerminalDisposition::OwnerStop {
                                    cause: OwnerStopCause::ResourceExhausted
                                }
                            )
                        {
                            return Err(SamplingError::InvalidTerminalTransition);
                        }
                    }
                }
                if matches!(self.disposition, TerminalDisposition::SetupRefused { .. })
                    && matches!(tick, MemoryTick::Exhausted(_))
                {
                    self.sampling
                        .stops
                        .capture_once(StopOrigin::Outer)
                        .map_err(SamplingError::Deadline)?;
                    self.disposition = TerminalDisposition::OwnerStop {
                        cause: OwnerStopCause::ResourceExhausted,
                    };
                }
                // Persistent above-ceiling RSS preserves the genuine stop candidate; it cannot
                // suppress due observations or prevent the bounded provisional stop commit.
            }
        }
        match &mut self.state {
            TerminalState::DeliverDescriptor { report, send } => {
                if send
                    .advance(
                        &caller.transport(),
                        &[report.descriptor.as_fd()],
                        self.deadline,
                    )
                    .map_err(SamplingError::Control)?
                {
                    // Drop only O's already delivered sealed descriptor; C owns its genuine
                    // transferred duplicate. The original full backing reserve remains charged.
                    self.state = TerminalState::AwaitRead;
                }
                Ok(TerminalProgress::Pending)
            }
            TerminalState::AwaitRead => {
                let Some(received) = receive
                    .step(caller, Some(self.deadline))
                    .map_err(SamplingError::Control)?
                else {
                    return Ok(TerminalProgress::Pending);
                };
                match received {
                    OuterCallerControl::ReadCompleted { authority, bytes } => {
                        if authority != self.sampling.settings.authority {
                            return Err(SamplingError::TerminalAuthority);
                        }
                        if Some(bytes) != self.bytes {
                            return Err(SamplingError::TerminalSize);
                        }
                    }
                    OuterCallerControl::Close(CallerTerminalControl::CancelClose {
                        authority,
                        deadline,
                        stop,
                    }) => {
                        if stop.origin != StopOrigin::Caller
                            || !matches!(self.cleanup, TerminalCleanupWitness::Claimed { .. })
                        {
                            return Err(SamplingError::TerminalAuthority);
                        }
                        let deadline = adopt_close_clock(
                            &self.sampling.settings,
                            &mut self.sampling.stops,
                            authority,
                            deadline,
                            stop,
                        )?;
                        self.deadline = self.deadline.min(deadline);
                        self.caller_cancelled = true;
                    }
                    OuterCallerControl::Phase(_)
                    | OuterCallerControl::Close(CallerTerminalControl::CompletedClose { .. })
                    | OuterCallerControl::Close(CallerTerminalControl::ReadCompleted { .. }) => {
                        return Err(SamplingError::InvalidTerminalTransition);
                    }
                }
                self.state = TerminalState::Commit;
                Ok(TerminalProgress::Pending)
            }
            TerminalState::Commit => {
                // This fresh actual-source sample follows all seal/report descriptor delivery/
                // bounded-read points. Serialize only scalars into the pre-reserved O buffer.
                let storage = self
                    .commit
                    .take()
                    .ok_or(SamplingError::InvalidTerminalTransition)?;
                let stop = self
                    .sampling
                    .stops
                    .earliest()
                    .map_err(SamplingError::Deadline)?;
                let reply = if self.caller_cancelled
                    && matches!(self.disposition, TerminalDisposition::Report)
                {
                    // No read ACK, report acceptance or metrics are manufactured by a local
                    // read failure. Real I/M settlement was retained before the descriptor.
                    OuterTerminalReply::Cancelled {
                        authority: self.sampling.settings.authority,
                        stop,
                        peaks,
                    }
                } else {
                    OuterTerminalReply::Committed {
                        authority: self.sampling.settings.authority,
                        peaks,
                        disposition: self.disposition,
                        stop,
                    }
                };
                let frame = storage.encode(&reply).map_err(SamplingError::Control)?;
                // Do not freeze the early peak while serialization/preemption consumes a due
                // ordinary 20ms observer tick. Zero-progress returns to a fresh complete sample.
                // This check is immediately before the nonblocking syscall, not a promise that
                // scheduler or kernel execution cannot cross that point afterward.
                let due = sample_started
                    .checked_add(super::owned::TICK)
                    .ok_or(SamplingError::Deadline(DeadlineError::Unrepresentable))?;
                if Instant::now() >= due {
                    self.commit = Some(frame.into_storage());
                    return Ok(TerminalProgress::Pending);
                }
                if caller
                    .transport()
                    .send_terminal_once(&frame, self.deadline)
                    .map_err(SamplingError::Control)?
                {
                    self.state = TerminalState::Committed;
                    Ok(TerminalProgress::Committed)
                } else {
                    self.commit = Some(frame.into_storage());
                    Ok(TerminalProgress::Pending)
                }
            }
            TerminalState::Committed => Err(SamplingError::InvalidTerminalTransition),
        }
    }
}

/// Both complete close alternatives preserve the existing producer clock and original cutoff.
/// The actor independently requires genuine I completion or Caller-origin cancellation authority.
fn adopt_caller_close(
    sampling: &mut OuterSampling,
    authority: super::protocol::RunAuthority,
    deadline: RoleDeadline,
    stop: StopStamp,
) -> Result<Instant, SamplingError> {
    adopt_close_clock(
        &sampling.settings,
        &mut sampling.stops,
        authority,
        deadline,
        stop,
    )
}

fn adopt_close_clock(
    settings: &RunSettings,
    stops: &mut StopTimeline,
    authority: super::protocol::RunAuthority,
    deadline: RoleDeadline,
    stop: StopStamp,
) -> Result<Instant, SamplingError> {
    if authority != settings.authority {
        return Err(SamplingError::TerminalAuthority);
    }
    stops.observe(stop).map_err(SamplingError::Deadline)?;
    let earliest = stops
        .deadline(settings.settlement_reserve, settings.deadline)
        .map_err(SamplingError::Deadline)?;
    let deadline = if earliest
        .no_later_than(deadline)
        .map_err(SamplingError::Deadline)?
    {
        earliest
    } else {
        deadline
    };
    let original = settings.deadline;
    if let IdentityDeadline::Finite { deadline: bound } = original {
        if !deadline
            .no_later_than(bound)
            .map_err(SamplingError::Deadline)?
        {
            return Err(SamplingError::TerminalDeadlineMismatch);
        }
    }
    let deadline = deadline.local().map_err(SamplingError::Deadline)?;
    if matches!(original, IdentityDeadline::NeverElapses)
        && deadline.saturating_duration_since(Instant::now()) > super::role_deadline::SETTLE_RESERVE
    {
        return Err(SamplingError::TerminalDeadlineMismatch);
    }
    if Instant::now() >= deadline {
        return Err(SamplingError::Deadline(DeadlineError::Expired));
    }
    Ok(deadline)
}

/// One actual-source accounting operation shared by collecting and terminal states. No caller
/// observation, missing quantity, elapsed label or previous ledger peak substitutes for this read.
fn observation_deadline(
    settings: &RunSettings,
    stops: &StopTimeline,
) -> Result<Option<Instant>, SamplingError> {
    let original = settings
        .identity_deadline()
        .map_err(SamplingError::Deadline)?;
    let stop = stops
        .active_deadline(settings.settlement_reserve, settings.deadline)
        .map_err(SamplingError::Deadline)?;
    match stop {
        Some(stop) => {
            let cutoff = stop.local().map_err(SamplingError::Deadline)?;
            Ok(Some(
                original.map_or(cutoff, |original| original.min(cutoff)),
            ))
        }
        None => Ok(original),
    }
}

fn observe_live(
    launcher: &mut LauncherMemory,
    tree: &mut MemoryObserver,
    ledger: &mut ResourceLedger,
    settings: &RunSettings,
    stops: &StopTimeline,
    outer: &PreparedOuter<'_>,
    caller: &RoleEndpoint,
) -> Result<MemoryTick, SamplingError> {
    caller
        .transport()
        .refuse_observable_eof()
        .map_err(SamplingError::Control)?;
    outer
        .require_creator_live()
        .map_err(|error| SamplingError::Io {
            operation: CauseOperation::OwnerProtection,
            cause: io::Error::other(error),
        })?;
    let launcher = launcher.sample().map_err(|cause| SamplingError::Io {
        operation: CauseOperation::LauncherObservation,
        cause,
    })?;
    let deadline = observation_deadline(settings, stops)?;
    let tree = tree
        .observe_until(1, deadline)
        .map_err(|cause| SamplingError::Io {
            operation: CauseOperation::TreeObservation,
            cause,
        })?;
    ledger
        .observe(Some(launcher), Some(tree))
        .map_err(SamplingError::Charge)
}

/// O retains the actual nested monitor and its private I bootstrap in one owner. State advances
/// only through the same production gate/claim operations; no fixture fabricates an INIT claim.
pub(super) struct InnerMonitor {
    monitor: OuterMonitorOwner,
    control: RoleCaller,
    frame: IncrementalSend,
    outer_pin: OwnedFd,
    caller_pin: OwnedFd,
    caller_lease: Option<GuardianEndpoint>,
    state: InnerMonitorState,
}

enum InnerMonitorState {
    Prepared,
    SpawnAttempted,
    Spawned,
    Gated(GatedClaim),
    ReleaseAttempted,
    GateReleased,
    Bootstrapped,
}

impl InnerMonitor {
    pub(super) fn poll_settled(&mut self) -> Result<Option<InnerSettlement>, SamplingError> {
        self.monitor
            .poll_settled()
            .map_err(|cause| SamplingError::Io {
                operation: CauseOperation::MonitorReap,
                cause,
            })
    }

    /// Record the attempt before spawn: any error retains the actual monitor owner and cannot
    /// retry a second process. This owner must survive until the outer run confirms settlement.
    pub(super) fn spawn(
        &mut self,
        outer: &PreparedOuter<'_>,
        deadline: Instant,
    ) -> Result<(), SamplingError> {
        if !matches!(self.state, InnerMonitorState::Prepared) {
            return Err(SamplingError::InvalidMonitorTransition);
        }
        self.state = InnerMonitorState::SpawnAttempted;
        self.monitor
            .spawn(outer, deadline)
            .map_err(|cause| SamplingError::Io {
                operation: CauseOperation::MonitorSpawn,
                cause,
            })?;
        self.state = InnerMonitorState::Spawned;
        Ok(())
    }

    /// Continue a complete normal O accounting/collector tick before each finite claim attempt.
    /// The gate is still held after a successful result. This is not Ready or Dispatch evidence.
    pub(super) fn claim_tick(
        &mut self,
        sampling: &mut OuterSampling,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        startup_deadline: Instant,
    ) -> Result<(MemoryTick, bool), SamplingError> {
        if !matches!(self.state, InnerMonitorState::Spawned) {
            return Err(SamplingError::InvalidMonitorTransition);
        }
        let (tick, claim) =
            sampling.claim_monitor_tick(outer, caller, &mut self.monitor, startup_deadline)?;
        let claimed = claim.is_some();
        if let Some(claim) = claim {
            self.state = InnerMonitorState::Gated(claim);
        }
        Ok((tick, claimed))
    }

    /// Consume the genuine gated claim once. Authenticated bootstrap delivery uses subsequent
    /// finite normal actor ticks; attempted release or later sending proves no I settlement.
    pub(super) fn release_gate(
        &mut self,
        sampling: &mut OuterSampling,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        startup_deadline: Instant,
    ) -> Result<MemoryTick, SamplingError> {
        if !matches!(self.state, InnerMonitorState::Gated(_)) {
            return Err(SamplingError::InvalidMonitorTransition);
        }
        let tick = sampling.tick(outer, caller)?;
        if matches!(tick, MemoryTick::Exhausted(_)) || sampling.owner_stop.is_some() {
            return Ok(tick);
        }
        let InnerMonitorState::Gated(claim) =
            std::mem::replace(&mut self.state, InnerMonitorState::ReleaseAttempted)
        else {
            return Err(SamplingError::InvalidMonitorTransition);
        };
        let deadline = sampling
            .settings
            .startup_deadline()
            .map_err(SamplingError::Deadline)?;
        self.monitor
            .namespace()
            .release_bootstrap_gate(claim, Some(deadline), startup_deadline)
            .map_err(|cause| SamplingError::Io {
                operation: CauseOperation::MonitorClaim,
                cause,
            })?;
        self.state = InnerMonitorState::GateReleased;
        Ok(tick)
    }
    /// One finite bootstrap send step after the real gate release, with normal complete accounting
    /// before each attempt. I's temporarily unread socket cannot suspend RSS or collector progress.
    fn bootstrap_tick(
        &mut self,
        sampling: &mut OuterSampling,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        startup_deadline: Instant,
    ) -> Result<(MemoryTick, bool), SamplingError> {
        if !matches!(self.state, InnerMonitorState::GateReleased) {
            return Err(SamplingError::InvalidMonitorTransition);
        }
        let tick = sampling.tick(outer, caller)?;
        if matches!(tick, MemoryTick::Exhausted(_)) || sampling.owner_stop.is_some() {
            return Ok((tick, false));
        }
        let deadline = sampling
            .settings
            .startup_deadline()
            .map_err(SamplingError::Deadline)?
            .min(startup_deadline);
        let lease = self
            .caller_lease
            .as_ref()
            .ok_or(SamplingError::InnerLeaseConsumed)?;
        let sent = self
            .frame
            .advance(
                &self.control.transport(),
                &[
                    self.outer_pin.as_fd(),
                    self.caller_pin.as_fd(),
                    lease.as_fd(),
                ],
                deadline,
            )
            .map_err(SamplingError::Control)?;
        if sent {
            self.caller_lease.take();
            self.state = InnerMonitorState::Bootstrapped;
        }
        Ok((tick, sent))
    }
}

/// O independently receives actual I's backend events over the already retained private role
/// pair. C's lease and textual reports do not substitute for these authenticated completion facts.
pub(super) struct InnerCompletion {
    receive: IncrementalReceive,
    state: InnerCompletionState,
    frame_deadline: Option<Instant>,
    refusal_context: PreparedStartupContext,
    decode_scratch: super::guardian_decode::Scratch,
}

enum InnerCompletionState {
    AwaitDispatch,
    Dispatched,
    Acknowledging {
        outcome: BackendExit,
        send: IncrementalSend,
        deadline: Instant,
    },
    Completed(BackendExit),
    Refused,
}

pub(super) enum CompletionProgress {
    Pending,
    Dispatched,
    Completed(BackendExit),
    Exhausted,
    SetupRefused,
}

impl InnerCompletion {
    /// Allocate before M/writer exposure; subsequent decoding reuses these actual owned buffers.
    pub(super) fn prepare() -> Result<Self, SamplingError> {
        Ok(Self {
            receive: IncrementalReceive::prepare().map_err(SamplingError::Control)?,
            state: InnerCompletionState::AwaitDispatch,
            frame_deadline: None,
            decode_scratch: super::guardian_decode::Scratch::default(),
            refusal_context: PreparedStartupContext::new(CONTEXT_BYTES).map_err(|error| {
                SamplingError::Io {
                    operation: CauseOperation::ControlPreparation,
                    cause: io::Error::other(error),
                }
            })?,
        })
    }

    /// Fresh normal accounting precedes each one-chunk frame attempt, including partial Completed.
    /// A completed backend remains inside live I until the unchanged original C lease closes.
    pub(super) fn tick(
        &mut self,
        sampling: &mut OuterSampling,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        monitor: &InnerMonitor,
    ) -> Result<CompletionProgress, SamplingError> {
        let tick = sampling.tick(outer, caller)?;
        if matches!(tick, MemoryTick::Exhausted(_)) || sampling.owner_stop.is_some() {
            return Ok(CompletionProgress::Exhausted);
        }
        if !matches!(monitor.state, InnerMonitorState::Bootstrapped) {
            return Err(SamplingError::InvalidMonitorTransition);
        }
        if let InnerCompletionState::Completed(outcome) = self.state {
            return Ok(CompletionProgress::Completed(outcome));
        }
        if matches!(self.state, InnerCompletionState::Refused) {
            return Ok(CompletionProgress::SetupRefused);
        }
        if let InnerCompletionState::Acknowledging {
            outcome,
            send,
            deadline,
        } = &mut self.state
        {
            // The normal complete accounting tick above runs again on EVERY finite send attempt.
            // Buffered I bytes or merely preparing this frame never count as an observed event.
            if send
                .advance(&monitor.control.transport(), &[], *deadline)
                .map_err(SamplingError::Control)?
            {
                let outcome = *outcome;
                self.state = InnerCompletionState::Completed(outcome);
                return Ok(CompletionProgress::Completed(outcome));
            }
            return Ok(CompletionProgress::Pending);
        }
        // A normal never-elapsing backend may legitimately have no event for arbitrarily long.
        // Once the first frame byte is received, its finite control cap is retained, never reset.
        // Every attempt itself is nonblocking and followed by a fresh normal accounting tick.
        let cap = Instant::now()
            .checked_add(std::time::Duration::from_secs(3))
            .ok_or(SamplingError::Deadline(DeadlineError::Unrepresentable))?;
        let cap = self.frame_deadline.unwrap_or(cap);
        let deadline = sampling
            .settings
            .work_deadline()
            .map_err(SamplingError::Deadline)?
            .map_or(cap, |deadline| deadline.min(cap));
        let Some(received) = self
            .receive
            .advance_decode(
                &monitor.control.transport(),
                InnerOwnerHeader::rights_count,
                deadline,
                |payload| {
                    super::startup_envelope::decode_inner_owner(
                        payload,
                        &mut self.refusal_context,
                        &mut self.decode_scratch,
                    )
                },
            )
            .map_err(SamplingError::Control)?
        else {
            if self.frame_deadline.is_none() && self.receive.has_partial_frame() {
                self.frame_deadline = Some(deadline);
            }
            return Ok(CompletionProgress::Pending);
        };
        self.frame_deadline = None;
        let sender = received
            .credentials
            .ok_or(SamplingError::Control(ControlError::MissingCredentials))?;
        monitor
            .monitor
            .verify_ready(sender, 0)
            .map_err(SamplingError::InnerIdentity)?;
        match (&self.state, received.control) {
            (
                InnerCompletionState::AwaitDispatch,
                InnerOwnerHeader::Event(InnerEventHeader::Dispatched { authority }),
            ) if authority == sampling.settings.authority => {
                self.state = InnerCompletionState::Dispatched;
                Ok(CompletionProgress::Dispatched)
            }
            (
                InnerCompletionState::Dispatched,
                InnerOwnerHeader::Event(InnerEventHeader::Completed {
                    authority,
                    outcome,
                    stop,
                }),
            ) if authority == sampling.settings.authority => {
                if stop.origin != StopOrigin::Inner {
                    return Err(SamplingError::UnexpectedInnerEvent);
                }
                let stop = sampling
                    .stops
                    .observe(stop)
                    .map_err(SamplingError::Deadline)?;
                let cutoff = sampling
                    .stops
                    .deadline(
                        sampling.settings.settlement_reserve,
                        sampling.settings.deadline,
                    )
                    .map_err(SamplingError::Deadline)?
                    .local()
                    .map_err(SamplingError::Deadline)?;
                let deadline = deadline.min(cutoff);
                self.state = InnerCompletionState::Acknowledging {
                    outcome,
                    send: IncrementalSend::new(
                        PreparedFrame::encode(&InnerOwnerControl::CompletionObserved {
                            authority,
                            stop,
                        })
                        .map_err(SamplingError::Control)?,
                    ),
                    deadline,
                };
                Ok(CompletionProgress::Pending)
            }
            (
                InnerCompletionState::AwaitDispatch,
                InnerOwnerHeader::Refused {
                    identity,
                    authority,
                    stop,
                    failure,
                },
            ) => {
                if identity != sampling.settings.identity
                    || authority != sampling.settings.authority
                    || stop.origin != StopOrigin::Backend
                {
                    return Err(SamplingError::InnerCompletionAuthority);
                }
                sampling
                    .stops
                    .observe(stop)
                    .map_err(SamplingError::Deadline)?;
                sampling.setup_refusal = Some(failure);
                self.state = InnerCompletionState::Refused;
                sampling
                    .collector
                    .begin_owner_stop()
                    .map_err(|cause| SamplingError::Report {
                        operation: CauseOperation::ReportCollection,
                        cause,
                    })?;
                Ok(CompletionProgress::SetupRefused)
            }
            (
                InnerCompletionState::AwaitDispatch,
                InnerOwnerHeader::Event(InnerEventHeader::Dispatched { .. }),
            )
            | (
                InnerCompletionState::Dispatched,
                InnerOwnerHeader::Event(InnerEventHeader::Completed { .. }),
            ) => Err(SamplingError::InnerCompletionAuthority),
            (
                InnerCompletionState::AwaitDispatch
                | InnerCompletionState::Dispatched
                | InnerCompletionState::Acknowledging { .. }
                | InnerCompletionState::Completed(_),
                _,
            ) => Err(SamplingError::UnexpectedInnerEvent),
            (InnerCompletionState::Refused, _) => Err(SamplingError::UnexpectedInnerEvent),
        }
    }
}

/// One normal O startup controller, independent of the optional fixture feature. C drives the
/// same production prefix by data; every wait returns to complete ordinary RSS/collector sampling.
pub(super) struct OuterPhases {
    phase: OuterPhase,
    pending_reply: Option<PhaseReply>,
}

struct PhaseReply {
    send: IncrementalSend,
    pin: Option<OwnedFd>,
    next: OuterPhase,
    progress: PhaseProgress,
}

enum OuterPhase {
    BeforeMonitor,
    Bootstrap,
    Claiming,
    ClaimedGated,
    Bootstrapping,
    ClaimedBootstrap,
}

/// Private actor progress, not a public/fixture witness or proof of any role's settlement.
pub(super) enum PhaseProgress {
    Pending,
    MonitorSpawned,
    InnerClaimed,
    GateReleased,
    Exhausted,
}

impl OuterPhases {
    pub(super) fn prepare() -> Result<Self, SamplingError> {
        Ok(Self {
            phase: OuterPhase::BeforeMonitor,
            pending_reply: None,
        })
    }

    /// At most one bounded receive or claim step follows the same normal accounting tick.
    /// Partial controls cannot hold the collector or memory observer inside a blocking frame read.
    pub(super) fn tick(
        &mut self,
        sampling: &mut OuterSampling,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        monitor: &mut InnerMonitor,
        startup_deadline: Instant,
        received: Option<OuterCallerControl>,
    ) -> Result<PhaseProgress, SamplingError> {
        let startup_deadline = startup_deadline.min(
            sampling
                .settings
                .setup_deadline
                .local()
                .map_err(SamplingError::Deadline)?,
        );
        if received.is_some()
            && (self.pending_reply.is_some()
                || matches!(self.phase, OuterPhase::Bootstrapping | OuterPhase::Claiming))
        {
            return Err(SamplingError::UnexpectedPhase);
        }
        if let Some(reply) = self.pending_reply.as_mut() {
            let tick = sampling.tick(outer, caller)?;
            if matches!(tick, MemoryTick::Exhausted(_)) || sampling.owner_stop.is_some() {
                return Ok(PhaseProgress::Exhausted);
            }
            let deadline = sampling
                .settings
                .startup_deadline()
                .map_err(SamplingError::Deadline)?
                .min(startup_deadline);
            let rights = reply.pin.as_ref().map(|pin| [pin.as_fd()]);
            if reply
                .send
                .advance(
                    &caller.transport(),
                    rights.as_ref().map_or(&[], |rights| &rights[..]),
                    deadline,
                )
                .map_err(SamplingError::Control)?
            {
                let reply = self
                    .pending_reply
                    .take()
                    .ok_or(SamplingError::UnexpectedPhase)?;
                self.phase = reply.next;
                return Ok(reply.progress);
            }
            return Ok(PhaseProgress::Pending);
        }
        if matches!(self.phase, OuterPhase::Bootstrapping) {
            let (tick, sent) = monitor.bootstrap_tick(sampling, outer, caller, startup_deadline)?;
            if matches!(tick, MemoryTick::Exhausted(_)) || sampling.owner_stop.is_some() {
                return Ok(PhaseProgress::Exhausted);
            }
            if sent {
                self.queue_reply(
                    OuterPhaseReply::GateReleased {
                        authority: sampling.settings.authority,
                    },
                    None,
                    OuterPhase::ClaimedBootstrap,
                    PhaseProgress::GateReleased,
                )?;
            }
            return Ok(PhaseProgress::Pending);
        }
        if matches!(self.phase, OuterPhase::Claiming) {
            let (tick, claimed) = monitor.claim_tick(sampling, outer, caller, startup_deadline)?;
            if matches!(tick, MemoryTick::Exhausted(_)) || sampling.owner_stop.is_some() {
                return Ok(PhaseProgress::Exhausted);
            }
            if claimed {
                let (start, namespace, pin) =
                    monitor
                        .monitor
                        .inner_capability()
                        .map_err(|cause| SamplingError::Io {
                            operation: CauseOperation::MonitorClaim,
                            cause,
                        })?;
                self.queue_reply(
                    OuterPhaseReply::InnerClaimed {
                        authority: sampling.settings.authority,
                        start,
                        namespace,
                    },
                    Some(pin),
                    OuterPhase::ClaimedGated,
                    PhaseProgress::InnerClaimed,
                )?;
                return Ok(PhaseProgress::Pending);
            }
            return Ok(PhaseProgress::Pending);
        }
        let tick = sampling.tick(outer, caller)?;
        if matches!(tick, MemoryTick::Exhausted(_)) || sampling.owner_stop.is_some() {
            return Ok(PhaseProgress::Exhausted);
        }
        let deadline = sampling
            .settings
            .startup_deadline()
            .map_err(SamplingError::Deadline)?
            .min(startup_deadline);
        let Some(received) = received else {
            return Ok(PhaseProgress::Pending);
        };
        let OuterCallerControl::Phase(command) = received else {
            return Err(SamplingError::UnexpectedPhase);
        };
        if command.authority() != sampling.settings.authority {
            return Err(SamplingError::PhaseAuthorityMismatch);
        }
        match (&self.phase, command) {
            (OuterPhase::BeforeMonitor, OuterPhaseCommand::BeginMonitor { .. }) => {
                monitor.spawn(outer, deadline)?;
                let pin =
                    monitor
                        .monitor
                        .monitor_capability()
                        .map_err(|cause| SamplingError::Io {
                            operation: CauseOperation::MonitorSpawn,
                            cause,
                        })?;
                self.queue_reply(
                    OuterPhaseReply::MonitorSpawned {
                        authority: sampling.settings.authority,
                    },
                    Some(pin),
                    OuterPhase::Bootstrap,
                    PhaseProgress::MonitorSpawned,
                )?;
                Ok(PhaseProgress::Pending)
            }
            (OuterPhase::Bootstrap, OuterPhaseCommand::ClaimInner { .. }) => {
                self.phase = OuterPhase::Claiming;
                Ok(PhaseProgress::Pending)
            }
            (OuterPhase::ClaimedGated, OuterPhaseCommand::ReleaseGate { .. }) => {
                let tick = monitor.release_gate(sampling, outer, caller, startup_deadline)?;
                if matches!(tick, MemoryTick::Exhausted(_)) || sampling.owner_stop.is_some() {
                    return Ok(PhaseProgress::Exhausted);
                }
                self.phase = OuterPhase::Bootstrapping;
                Ok(PhaseProgress::Pending)
            }
            (
                OuterPhase::BeforeMonitor
                | OuterPhase::Bootstrap
                | OuterPhase::Claiming
                | OuterPhase::ClaimedGated
                | OuterPhase::Bootstrapping
                | OuterPhase::ClaimedBootstrap,
                _,
            ) => Err(SamplingError::UnexpectedPhase),
        }
    }
    fn queue_reply(
        &mut self,
        control: OuterPhaseReply,
        pin: Option<OwnedFd>,
        next: OuterPhase,
        progress: PhaseProgress,
    ) -> Result<(), SamplingError> {
        let frame = PreparedFrame::encode(&control).map_err(SamplingError::Control)?;
        self.pending_reply = Some(PhaseReply {
            send: IncrementalSend::new(frame),
            pin,
            next,
            progress,
        });
        Ok(())
    }
}

impl OuterSampling {
    /// Move the actual fully prepared resources. The retained preparation factory performs
    /// no observation; the actor stores these before its separate initial fallible RSS tick.
    fn from_parts(parts: SamplingParts) -> Self {
        let SamplingParts {
            launcher,
            tree,
            ledger,
            collector,
            settings,
            stops,
        } = parts;
        Self {
            launcher,
            tree,
            ledger,
            collector,
            settings,
            stops,
            owner_stop: None,
            setup_refusal: None,
            caller_cancelled: false,
            observation_admitted: false,
        }
    }

    fn stop_progress(&self) -> Result<OuterRunProgress, SamplingError> {
        match (self.owner_stop, self.setup_refusal) {
            (Some(cause), _) => Ok(OuterRunProgress::OwnerStopped { cause }),
            (None, Some(_)) => Ok(OuterRunProgress::SetupRefused),
            (None, None) => Err(SamplingError::InvalidMonitorTransition),
        }
    }

    /// One real observation followed by finite collector work. Independently observed resource
    /// exhaustion takes priority and never becomes successful proof from partially written bytes.
    /// An observation error remains an error; it cannot substitute zero for any role/worker RSS.
    pub(super) fn tick(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<MemoryTick, SamplingError> {
        self.ledger.begin_observation();
        let result = self.tick_observation(outer, caller);
        if result.is_err() {
            self.ledger.begin_observation();
        }
        result
    }

    fn tick_observation(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<MemoryTick, SamplingError> {
        let tick = observe_live(
            &mut self.launcher,
            &mut self.tree,
            &mut self.ledger,
            &self.settings,
            &self.stops,
            outer,
            caller,
        )?;
        if matches!(tick, MemoryTick::WithinCeiling(_)) {
            self.observation_admitted = true;
        }
        if self.owner_stop.is_none() {
            // Only a fresh complete observation may select a resource stop. It precedes the
            // clock check, so genuine memory excess wins simultaneous observed work expiry.
            let cause = if matches!(tick, MemoryTick::Exhausted(_)) {
                Some(OwnerStopCause::ResourceExhausted)
            } else if self
                .stops
                .active_deadline(self.settings.settlement_reserve, self.settings.deadline)
                .map_err(SamplingError::Deadline)?
                .is_none()
                && self
                    .settings
                    .work_deadline()
                    .map_err(SamplingError::Deadline)?
                    .is_some_and(|deadline| Instant::now() >= deadline)
            {
                // An already observed completion ended work; its settlement cannot be turned
                // into a later work timeout. Never-elapsing work has no manufactured cutoff.
                Some(OwnerStopCause::TimedOut)
            } else {
                None
            };
            if let Some(cause) = cause {
                self.stops
                    .capture_once(StopOrigin::Outer)
                    .map_err(SamplingError::Deadline)?;
                self.owner_stop = Some(cause);
                // Close only the collector's still-owned writer. This is not child settlement.
                self.collector
                    .begin_owner_stop()
                    .map_err(|cause| SamplingError::Report {
                        operation: CauseOperation::ReportCollection,
                        cause,
                    })?;
            }
        }
        let deadline = observation_deadline(&self.settings, &self.stops)?;
        if self.owner_stop.is_some() || self.setup_refusal.is_some() || self.caller_cancelled {
            // Do not let malformed/partial report bytes replace the already genuine owner-stop
            // candidate. Actual finite reader EOF still requires all actual writers to close.
            self.collector
                .drain_owner_stop(deadline)
                .map_err(|cause| SamplingError::Report {
                    operation: CauseOperation::ReportCollection,
                    cause,
                })?;
        } else {
            self.collector
                .drain_tick(deadline)
                .map_err(|cause| SamplingError::Report {
                    operation: CauseOperation::ReportCollection,
                    cause,
                })?;
        }
        outer
            .require_creator_live()
            .map_err(|error| SamplingError::Io {
                operation: CauseOperation::OwnerProtection,
                cause: io::Error::other(error),
            })?;
        caller
            .transport()
            .refuse_observable_eof()
            .map_err(SamplingError::Control)?;
        Ok(tick)
    }

    /// The ledger accepts this writer only after an actual within-ceiling setup tick. M owns its
    /// direct Child and temporary mapping copy separately; neither a recipe nor this call spawns.
    pub(super) fn prepare_monitor(
        &mut self,
        outer: &PreparedOuter<'_>,
        bootstrap: RoleEndpoint,
    ) -> Result<OuterMonitorOwner, SamplingError> {
        if self.owner_stop.is_some() {
            return Err(SamplingError::InvalidMonitorTransition);
        }
        OuterMonitorOwner::prepare(
            outer,
            &self.settings.helper,
            bootstrap,
            &mut self.collector,
            &self.ledger,
        )
        .map_err(|cause| SamplingError::Io {
            operation: CauseOperation::MonitorSpawn,
            cause,
        })
    }

    /// Prepare the actual I bootstrap and monitor as one retained production owner before spawn.
    /// The report pipe identity is taken directly from this O collector, never supplied by C or I.
    pub(super) fn prepare_inner_monitor(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller_pin: &OwnedFd,
        caller_lease: &mut Option<GuardianEndpoint>,
    ) -> Result<InnerMonitor, SamplingError> {
        if caller_lease.is_none() {
            return Err(SamplingError::InnerLeaseConsumed);
        }
        let (control, endpoint) = role_pair().map_err(SamplingError::Control)?;
        let frame = PreparedFrame::encode(&InnerBootstrap::Start {
            settings: self.settings.clone(),
            outer_namespace: outer.namespace(),
            report: self.collector.identity(),
            report_slot: super::report_storage::REPORT_SLOT,
        })
        .map_err(SamplingError::Control)?;
        let outer_pin =
            outer
                .descriptor()
                .try_clone_to_owned()
                .map_err(|cause| SamplingError::Io {
                    operation: CauseOperation::Identity,
                    cause,
                })?;
        let caller_pin = caller_pin.try_clone().map_err(|cause| SamplingError::Io {
            operation: CauseOperation::Identity,
            cause,
        })?;
        let monitor = self.prepare_monitor(outer, endpoint)?;
        Ok(InnerMonitor {
            monitor,
            control,
            frame: IncrementalSend::new(frame),
            outer_pin,
            caller_pin,
            caller_lease: caller_lease.take(),
            state: InnerMonitorState::Prepared,
        })
    }

    /// A fresh complete normal accounting tick precedes each finite startup read. This preserves
    /// the same outer-root observer while the retained monitor creates/maps its gated inner INIT.
    pub(super) fn claim_monitor_tick(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
        monitor: &mut OuterMonitorOwner,
        startup_deadline: std::time::Instant,
    ) -> Result<(MemoryTick, Option<GatedClaim>), SamplingError> {
        let tick = self.tick(outer, caller)?;
        if self.owner_stop.is_some() {
            return Ok((tick, None));
        }
        let claim = monitor
            .namespace()
            .claim_gated_tick(startup_deadline, &mut self.tree)
            .map_err(|cause| SamplingError::Io {
                operation: CauseOperation::MonitorClaim,
                cause,
            })?;
        Ok((tick, claim))
    }

    /// Real writer EOF is a separate condition from authenticated backend completion or I/M exit.
    /// The actor continues ordinary accounting/draining ticks until it observes this actual read.
    pub(super) fn report_eof(&self) -> bool {
        self.collector.eof()
    }

    /// Consume only the collector after genuine I/M settlement and actual writer EOF. Retained
    /// accounting remains active through sealed descriptor delivery and the bounded consumer read.
    /// Return the original authenticated settlement token for terminal cleanup custody. The
    /// pre-seal sample is not serialized as final metrics or accepted as cleanup evidence.
    pub(super) fn seal_after_inner_settlement(
        mut self,
        settlement: InnerSettlement,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<(SealedReport, TerminalSampling, InnerSettlement), SamplingError> {
        if self.owner_stop.is_some() {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        if !settlement.matches_report(self.collector.identity()) {
            return Err(SamplingError::SettlementReportMismatch);
        }
        // This is a complete normal pre-seal sample after actual I/M settlement. Accounting
        // continues after seals/delivery, so this cannot be mislabeled as final metrics.
        if matches!(self.tick(outer, caller)?, MemoryTick::Exhausted(_)) {
            return Err(SamplingError::Charge(ChargeError::ResourceExhausted));
        }
        self.ledger
            .require_writer_exposure(self.collector.identity(), self.collector.reserve())
            .map_err(SamplingError::Charge)?;
        let deadline = observation_deadline(&self.settings, &self.stops)?;
        let report = self
            .collector
            .seal(deadline)
            .map_err(|cause| SamplingError::Report {
                operation: CauseOperation::ReportSealing,
                cause,
            })?;
        Ok((
            report,
            TerminalSampling {
                observation_admitted: self.observation_admitted,
                launcher: self.launcher,
                tree: self.tree,
                ledger: self.ledger,
                settings: self.settings,
                stops: self.stops,
            },
            settlement,
        ))
    }

    /// Actual complete measured peaks only. This is not a settled evidence constructor.
    pub(super) fn measured_peaks(&self) -> Result<MeasuredPeaks, SamplingError> {
        self.ledger.measured_peaks().map_err(SamplingError::Charge)
    }
}

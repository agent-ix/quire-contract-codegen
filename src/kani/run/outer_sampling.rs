//! O's actual retained L/tree observer, conservative ledger and anonymous collector.
//!
//! This owner creates no child during preparation. A real complete setup sample must authorize
//! the report writer before the nested monitor factory can consume it. Later ticks continue the
//! same observations while the backend runs; neither completion nor quiet pipe data bypasses RSS.
//! Final report sealing and evidence conclusions remain the whole-chain owner's separate duties.

use std::{
    io,
    os::fd::{AsFd, OwnedFd},
    path::Path,
    time::Instant,
};

use super::{
    control::{
        role_pair, ControlError, FrameStorage, GuardianEndpoint, IncrementalReceive,
        IncrementalSend, PreparedFrame, RoleCaller, RoleEndpoint,
    },
    memory::{LauncherMemory, MemoryObserver},
    namespace::{
        GatedClaim, GuardianIdentity, InnerSettlement, OuterMonitorOwner, ReadyIdentityError,
    },
    outer_setup::PreparedOuter,
    protocol::{BackendExit, GuardianControl},
    report_storage::{ReportCollector, ReportError, SealedReport},
    resource_ledger::{ChargeError, MeasuredPeaks, MemoryTick, ResourceLedger},
    role_deadline::DeadlineError,
    role_protocol::{
        CallerTerminalControl, InnerBootstrap, InnerOwnerControl, OuterPhaseCommand,
        OuterPhaseReply, OuterTerminalReply, RunSettings,
    },
};

#[derive(Debug)]
pub(super) enum SamplingError {
    Observation(io::Error),
    Control(ControlError),
    Report(ReportError),
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
}

impl std::fmt::Display for SamplingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "outer sampling refused: {self:?}")
    }
}

impl std::error::Error for SamplingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Observation(error) => Some(error),
            Self::Control(error) => Some(error),
            Self::Report(error) => Some(error),
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
            | Self::TerminalSize => None,
        }
    }
}

pub(super) struct OuterSampling {
    launcher: LauncherMemory,
    tree: MemoryObserver,
    ledger: ResourceLedger,
    collector: ReportCollector,
    settings: RunSettings,
}

/// O's actual accounting survives collector sealing and descriptor delivery. This owner grants
/// no writer exposure or child creation, and its latest complete sample is not cleanup evidence.
pub(super) struct TerminalSampling {
    launcher: LauncherMemory,
    tree: MemoryObserver,
    ledger: ResourceLedger,
    settings: RunSettings,
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
                outer,
                caller,
            )?;
            outer
                .require_creator_live()
                .map_err(|error| SamplingError::Observation(io::Error::other(error)))?;
            caller
                .transport()
                .refuse_observable_eof()
                .map_err(SamplingError::Control)?;
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

/// Reserve all terminal protocol storage while O is still preparing, before any M/writer child.
/// Its actual allocations remain in O's observed RSS; report backing retains its full reservation.
pub(super) struct TerminalPreparation {
    descriptor: FrameStorage,
    commit: FrameStorage,
    read_ack: IncrementalReceive,
}

impl TerminalPreparation {
    pub(super) fn prepare() -> Result<Self, SamplingError> {
        Ok(Self {
            descriptor: FrameStorage::prepare().map_err(SamplingError::Control)?,
            commit: FrameStorage::prepare().map_err(SamplingError::Control)?,
            read_ack: IncrementalReceive::prepare().map_err(SamplingError::Control)?,
        })
    }

    /// The cutoff is the retained whole owner's ALREADY started settlement deadline. This method
    /// creates no phase allowance; finite original T may only shorten that same cutoff.
    pub(super) fn begin(
        self,
        report: SealedReport,
        sampling: TerminalSampling,
        cutoff: Instant,
    ) -> Result<TerminalDelivery, SamplingError> {
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
            .encode(&OuterTerminalReply::ReportDescriptor { authority, bytes })
            .map_err(SamplingError::Control)?;
        Ok(TerminalDelivery {
            sampling,
            state: TerminalState::DeliverDescriptor {
                report,
                send: IncrementalSend::new(descriptor),
            },
            commit: Some(self.commit),
            read_ack: self.read_ack,
            bytes,
            deadline,
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
    Exhausted,
    /// Complete commit emission only. The executor may now exit normally; C must still prove
    /// actual normal O exit/reap through retained L, capture closure and creator settlement.
    Committed,
}

/// O's single terminal transaction. Every unsuccessful finite attempt returns to actual fresh
/// accounting, with no recursive commit ACK, repeated descriptor, new buffer or deadline reset.
pub(super) struct TerminalDelivery {
    sampling: TerminalSampling,
    state: TerminalState,
    commit: Option<FrameStorage>,
    read_ack: IncrementalReceive,
    bytes: u64,
    deadline: Instant,
    poisoned: bool,
}

impl TerminalDelivery {
    pub(super) fn tick(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<TerminalProgress, SamplingError> {
        if self.poisoned || matches!(self.state, TerminalState::Committed) {
            return Err(SamplingError::InvalidTerminalTransition);
        }
        self.poisoned = true;
        let result = self.advance(outer, caller);
        if result.is_ok() {
            self.poisoned = false;
        }
        result
    }

    fn advance(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<TerminalProgress, SamplingError> {
        if Instant::now() >= self.deadline {
            return Err(SamplingError::Deadline(DeadlineError::Expired));
        }
        let sample_started = Instant::now();
        let (tick, peaks) = self.sampling.observation_and_peaks(outer, caller)?;
        if matches!(tick, MemoryTick::Exhausted(_)) {
            return Ok(TerminalProgress::Exhausted);
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
                let Some(received) = self
                    .read_ack
                    .advance::<CallerTerminalControl>(&caller.transport(), |_| 0, self.deadline)
                    .map_err(SamplingError::Control)?
                else {
                    return Ok(TerminalProgress::Pending);
                };
                let CallerTerminalControl::ReadCompleted { authority, bytes } = received.control;
                if authority != self.sampling.settings.authority {
                    return Err(SamplingError::TerminalAuthority);
                }
                if bytes != self.bytes {
                    return Err(SamplingError::TerminalSize);
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
                let frame = storage
                    .encode(&OuterTerminalReply::Committed {
                        authority: self.sampling.settings.authority,
                        peaks,
                    })
                    .map_err(SamplingError::Control)?;
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

/// One actual-source accounting operation shared by collecting and terminal states. No caller
/// observation, missing quantity, elapsed label or previous ledger peak substitutes for this read.
fn observe_live(
    launcher: &mut LauncherMemory,
    tree: &mut MemoryObserver,
    ledger: &mut ResourceLedger,
    settings: &RunSettings,
    outer: &PreparedOuter<'_>,
    caller: &RoleEndpoint,
) -> Result<MemoryTick, SamplingError> {
    caller
        .transport()
        .refuse_observable_eof()
        .map_err(SamplingError::Control)?;
    outer
        .require_creator_live()
        .map_err(|error| SamplingError::Observation(io::Error::other(error)))?;
    let launcher = launcher.sample().map_err(SamplingError::Observation)?;
    let deadline = settings
        .identity_deadline()
        .map_err(SamplingError::Deadline)?;
    let tree = tree
        .observe_until(1, deadline)
        .map_err(SamplingError::Observation)?;
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
            .map_err(SamplingError::Observation)
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
            .map_err(SamplingError::Observation)?;
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
        if matches!(tick, MemoryTick::Exhausted(_)) {
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
            .map_err(SamplingError::Observation)?;
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
        if matches!(tick, MemoryTick::Exhausted(_)) {
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
}

pub(super) enum CompletionProgress {
    Pending,
    Dispatched,
    Completed(BackendExit),
    Exhausted,
}

impl InnerCompletion {
    /// Allocate before M/writer exposure; subsequent decoding reuses these actual owned buffers.
    pub(super) fn prepare() -> Result<Self, SamplingError> {
        Ok(Self {
            receive: IncrementalReceive::prepare().map_err(SamplingError::Control)?,
            state: InnerCompletionState::AwaitDispatch,
            frame_deadline: None,
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
        if matches!(tick, MemoryTick::Exhausted(_)) {
            return Ok(CompletionProgress::Exhausted);
        }
        if !matches!(monitor.state, InnerMonitorState::Bootstrapped) {
            return Err(SamplingError::InvalidMonitorTransition);
        }
        if let InnerCompletionState::Completed(outcome) = self.state {
            return Ok(CompletionProgress::Completed(outcome));
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
            .advance::<GuardianControl>(&monitor.control.transport(), |_| 0, deadline)
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
            (InnerCompletionState::AwaitDispatch, GuardianControl::Dispatched { authority })
                if authority == sampling.settings.authority =>
            {
                self.state = InnerCompletionState::Dispatched;
                Ok(CompletionProgress::Dispatched)
            }
            (
                InnerCompletionState::Dispatched,
                GuardianControl::Completed { authority, outcome },
            ) if authority == sampling.settings.authority => {
                self.state = InnerCompletionState::Acknowledging {
                    outcome,
                    send: IncrementalSend::new(
                        PreparedFrame::encode(&InnerOwnerControl::CompletionObserved { authority })
                            .map_err(SamplingError::Control)?,
                    ),
                    deadline,
                };
                Ok(CompletionProgress::Pending)
            }
            (InnerCompletionState::AwaitDispatch, GuardianControl::Dispatched { .. })
            | (InnerCompletionState::Dispatched, GuardianControl::Completed { .. }) => {
                Err(SamplingError::InnerCompletionAuthority)
            }
            (
                InnerCompletionState::AwaitDispatch
                | InnerCompletionState::Dispatched
                | InnerCompletionState::Acknowledging { .. }
                | InnerCompletionState::Completed(_),
                _,
            ) => Err(SamplingError::UnexpectedInnerEvent),
        }
    }
}

/// One normal O startup controller, independent of the optional fixture feature. C drives the
/// same production prefix by data; every wait returns to complete ordinary RSS/collector sampling.
pub(super) struct OuterPhases {
    receive: IncrementalReceive,
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
            receive: IncrementalReceive::prepare().map_err(SamplingError::Control)?,
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
    ) -> Result<PhaseProgress, SamplingError> {
        let startup_deadline = startup_deadline.min(
            sampling
                .settings
                .setup_deadline
                .local()
                .map_err(SamplingError::Deadline)?,
        );
        if let Some(reply) = self.pending_reply.as_mut() {
            let tick = sampling.tick(outer, caller)?;
            if matches!(tick, MemoryTick::Exhausted(_)) {
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
            if matches!(tick, MemoryTick::Exhausted(_)) {
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
            if matches!(tick, MemoryTick::Exhausted(_)) {
                return Ok(PhaseProgress::Exhausted);
            }
            if claimed {
                let (start, namespace, pin) = monitor
                    .monitor
                    .inner_capability()
                    .map_err(SamplingError::Observation)?;
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
        if matches!(tick, MemoryTick::Exhausted(_)) {
            return Ok(PhaseProgress::Exhausted);
        }
        let deadline = sampling
            .settings
            .startup_deadline()
            .map_err(SamplingError::Deadline)?
            .min(startup_deadline);
        let Some(received) = self
            .receive
            .advance::<OuterPhaseCommand>(
                &caller.transport(),
                OuterPhaseCommand::rights_count,
                deadline,
            )
            .map_err(SamplingError::Control)?
        else {
            return Ok(PhaseProgress::Pending);
        };
        if received.control.authority() != sampling.settings.authority {
            return Err(SamplingError::PhaseAuthorityMismatch);
        }
        match (&self.phase, received.control) {
            (OuterPhase::BeforeMonitor, OuterPhaseCommand::BeginMonitor { .. }) => {
                monitor.spawn(outer, deadline)?;
                let pin = monitor
                    .monitor
                    .monitor_capability()
                    .map_err(SamplingError::Observation)?;
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
                if matches!(tick, MemoryTick::Exhausted(_)) {
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
    /// O alone owns every report storage descriptor. Setup installs the per-O capacity filter
    /// before any writer-bearing child, then checks actual private proc observation availability.
    pub(super) fn prepare(
        outer: &PreparedOuter<'_>,
        launcher: LauncherMemory,
        settings: RunSettings,
    ) -> Result<Self, SamplingError> {
        settings
            .setup_deadline
            .local()
            .map_err(SamplingError::Deadline)?;
        let deadline = settings
            .identity_deadline()
            .map_err(SamplingError::Deadline)?;
        let collector = ReportCollector::prepare(outer).map_err(SamplingError::Report)?;
        let mut tree =
            MemoryObserver::prepare(Path::new("/proc")).map_err(SamplingError::Observation)?;
        tree.restrict_census(settings.memory_bytes)
            .map_err(SamplingError::Observation)?;
        tree.bind_outer(outer).map_err(SamplingError::Observation)?;
        settings
            .setup_deadline
            .local()
            .map_err(SamplingError::Deadline)?;
        let ledger = ResourceLedger::prepare(
            settings.memory_bytes,
            settings.caller_run_buffers,
            collector.reserve(),
            collector.identity(),
            deadline,
        )
        .map_err(SamplingError::Charge)?;
        Ok(Self {
            launcher,
            tree,
            ledger,
            collector,
            settings,
        })
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
            outer,
            caller,
        )?;
        if matches!(tick, MemoryTick::Exhausted(_)) {
            return Ok(tick);
        }
        let deadline = self
            .settings
            .identity_deadline()
            .map_err(SamplingError::Deadline)?;
        self.collector
            .drain_tick(deadline)
            .map_err(SamplingError::Report)?;
        outer
            .require_creator_live()
            .map_err(|error| SamplingError::Observation(io::Error::other(error)))?;
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
        OuterMonitorOwner::prepare(
            outer,
            &self.settings.helper,
            bootstrap,
            &mut self.collector,
            &self.ledger,
        )
        .map_err(SamplingError::Observation)
    }

    /// Prepare the actual I bootstrap and monitor as one retained production owner before spawn.
    /// The report pipe identity is taken directly from this O collector, never supplied by C or I.
    pub(super) fn prepare_inner_monitor(
        &mut self,
        outer: &PreparedOuter<'_>,
        caller_pin: &OwnedFd,
        caller_lease: GuardianEndpoint,
    ) -> Result<InnerMonitor, SamplingError> {
        let (control, endpoint) = role_pair().map_err(SamplingError::Control)?;
        let frame = PreparedFrame::encode(&InnerBootstrap::Start {
            settings: self.settings.clone(),
            outer_namespace: outer.namespace(),
            report: self.collector.identity(),
            report_slot: super::report_storage::REPORT_SLOT,
        })
        .map_err(SamplingError::Control)?;
        let outer_pin = outer
            .descriptor()
            .try_clone_to_owned()
            .map_err(SamplingError::Observation)?;
        let caller_pin = caller_pin.try_clone().map_err(SamplingError::Observation)?;
        let monitor = self.prepare_monitor(outer, endpoint)?;
        Ok(InnerMonitor {
            monitor,
            control,
            frame: IncrementalSend::new(frame),
            outer_pin,
            caller_pin,
            caller_lease: Some(caller_lease),
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
        if matches!(tick, MemoryTick::Exhausted(_)) {
            return Ok((tick, None));
        }
        let claim = monitor
            .namespace()
            .claim_gated_tick(startup_deadline, &mut self.tree)
            .map_err(SamplingError::Observation)?;
        Ok((tick, claim))
    }

    /// Real writer EOF is a separate condition from authenticated backend completion or I/M exit.
    /// The actor continues ordinary accounting/draining ticks until it observes this actual read.
    pub(super) fn report_eof(&self) -> bool {
        self.collector.eof()
    }

    /// Consume only the collector after genuine I/M settlement and actual writer EOF. Retained
    /// accounting remains active through sealed descriptor delivery and the bounded consumer read.
    /// The pre-seal sample is not serialized as final metrics or accepted as cleanup evidence.
    pub(super) fn seal_after_inner_settlement(
        mut self,
        settlement: InnerSettlement,
        outer: &PreparedOuter<'_>,
        caller: &RoleEndpoint,
    ) -> Result<(SealedReport, TerminalSampling), SamplingError> {
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
        let deadline = self
            .settings
            .identity_deadline()
            .map_err(SamplingError::Deadline)?;
        let report = self
            .collector
            .seal(deadline)
            .map_err(SamplingError::Report)?;
        Ok((
            report,
            TerminalSampling {
                launcher: self.launcher,
                tree: self.tree,
                ledger: self.ledger,
                settings: self.settings,
            },
        ))
    }

    /// Actual complete measured peaks only. This is not a settled evidence constructor.
    pub(super) fn measured_peaks(&self) -> Result<MeasuredPeaks, SamplingError> {
        self.ledger.measured_peaks().map_err(SamplingError::Charge)
    }
}

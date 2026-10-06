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
        role_pair, ControlError, GuardianEndpoint, IncrementalReceive, IncrementalSend,
        PreparedFrame, RoleCaller, RoleEndpoint,
    },
    memory::{LauncherMemory, MemoryObserver},
    namespace::{GatedClaim, InnerSettlement, OuterMonitorOwner},
    outer_setup::PreparedOuter,
    report_storage::{ReportCollector, ReportError, SealedReport},
    resource_ledger::{ChargeError, MeasuredPeaks, MemoryTick, ResourceLedger},
    role_deadline::DeadlineError,
    role_protocol::{InnerBootstrap, OuterPhaseCommand, OuterPhaseReply, RunSettings},
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
            Self::InvalidMonitorTransition
            | Self::InnerLeaseConsumed
            | Self::PhaseAuthorityMismatch
            | Self::UnexpectedPhase
            | Self::SettlementReportMismatch => None,
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
        caller
            .transport()
            .refuse_observable_eof()
            .map_err(SamplingError::Control)?;
        outer
            .require_creator_live()
            .map_err(|error| SamplingError::Observation(io::Error::other(error)))?;
        let launcher = self.launcher.sample().map_err(SamplingError::Observation)?;
        let deadline = self
            .settings
            .identity_deadline()
            .map_err(SamplingError::Deadline)?;
        let tree = self
            .tree
            .observe_until(1, deadline)
            .map_err(SamplingError::Observation)?;
        let tick = self
            .ledger
            .observe(Some(launcher), Some(tree))
            .map_err(SamplingError::Charge)?;
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

    /// Consume the collector only after its own genuine I/M settlement and actual writer EOF.
    /// This returns sealed bytes/measurements to the retained O owner, not execution evidence or
    /// permission for C to return before actual O/L/thread settlement and bounded report reading.
    pub(super) fn seal_after_inner_settlement(
        self,
        settlement: InnerSettlement,
    ) -> Result<(SealedReport, MeasuredPeaks), SamplingError> {
        if !settlement.matches_report(self.collector.identity()) {
            return Err(SamplingError::SettlementReportMismatch);
        }
        self.ledger
            .require_writer_exposure(self.collector.identity(), self.collector.reserve())
            .map_err(SamplingError::Charge)?;
        let peaks = self
            .ledger
            .measured_peaks()
            .map_err(SamplingError::Charge)?;
        let deadline = self
            .settings
            .identity_deadline()
            .map_err(SamplingError::Deadline)?;
        let report = self
            .collector
            .seal(deadline)
            .map_err(SamplingError::Report)?;
        Ok((report, peaks))
    }

    /// Actual complete measured peaks only. This is not a settled evidence constructor.
    pub(super) fn measured_peaks(&self) -> Result<MeasuredPeaks, SamplingError> {
        self.ledger.measured_peaks().map_err(SamplingError::Charge)
    }
}

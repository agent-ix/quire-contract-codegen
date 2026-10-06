//! O's actual retained L/tree observer, conservative ledger and anonymous collector.
//!
//! This owner creates no child during preparation. A real complete setup sample must authorize
//! the report writer before the nested monitor factory can consume it. Later ticks continue the
//! same observations while the backend runs; neither completion nor quiet pipe data bypasses RSS.
//! Final report sealing and evidence conclusions remain the whole-chain owner's separate duties.

use std::{io, path::Path};

use super::{
    control::{ControlError, RoleEndpoint},
    memory::{LauncherMemory, MemoryObserver},
    namespace::OuterMonitorOwner,
    outer_setup::PreparedOuter,
    report_storage::{ReportCollector, ReportError},
    resource_ledger::{ChargeError, MeasuredPeaks, MemoryTick, ResourceLedger},
    role_deadline::DeadlineError,
    role_protocol::RunSettings,
};

#[derive(Debug)]
pub(super) enum SamplingError {
    Observation(io::Error),
    Control(ControlError),
    Report(ReportError),
    Charge(ChargeError),
    Deadline(DeadlineError),
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

impl OuterSampling {
    /// O alone owns every report storage descriptor. Setup installs the per-O capacity filter
    /// before any writer-bearing child, then checks actual private proc observation availability.
    pub(super) fn prepare(
        outer: &PreparedOuter<'_>,
        launcher: LauncherMemory,
        settings: RunSettings,
    ) -> Result<Self, SamplingError> {
        let deadline = settings.deadline.local().map_err(SamplingError::Deadline)?;
        let collector = ReportCollector::prepare(outer).map_err(SamplingError::Report)?;
        let mut tree =
            MemoryObserver::prepare(Path::new("/proc")).map_err(SamplingError::Observation)?;
        tree.restrict_census(settings.memory_bytes)
            .map_err(SamplingError::Observation)?;
        tree.bind_outer(outer).map_err(SamplingError::Observation)?;
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
            .deadline
            .local()
            .map_err(SamplingError::Deadline)?;
        let tree = self
            .tree
            .observe_before(1, deadline)
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
            .deadline
            .local()
            .map_err(SamplingError::Deadline)?;
        self.collector
            .drain_tick(Some(deadline))
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

    /// Actual complete measured peaks only. This is not a settled evidence constructor.
    pub(super) fn measured_peaks(&self) -> Result<MeasuredPeaks, SamplingError> {
        self.ledger.measured_peaks().map_err(SamplingError::Charge)
    }
}

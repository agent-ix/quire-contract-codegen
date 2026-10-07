//! Retained construction custody for O's sampling resources.
//!
//! One consuming attempt returns every actual prepared object or the same preparation owner
//! with its original error. This owns no child, exposes no writer and performs no observation.
//! A factory's internal failure remains that factory's responsibility: only returned objects
//! can be retained here. Neither retaining nor dropping this owner proves role settlement.

use std::path::Path;

use super::{
    cross_role_cause::CauseOperation,
    memory::{LauncherMemory, MemoryObserver},
    outer_sampling::SamplingError,
    outer_setup::PreparedOuter,
    report_storage::ReportCollector,
    resource_ledger::ResourceLedger,
    role_deadline::StopTimeline,
    role_protocol::RunSettings,
};

/// Actual construction state, not absent measurement or absent-child evidence.
pub(super) struct SamplingPreparation {
    pub(super) launcher: LauncherMemory,
    pub(super) settings: RunSettings,
    pub(super) collector: Option<ReportCollector>,
    pub(super) tree: Option<MemoryObserver>,
    pub(super) ledger: Option<ResourceLedger>,
    pub(super) stops: Option<StopTimeline>,
}

/// The original fully prepared resources, ready for the owner's separate first observation.
pub(super) struct SamplingParts {
    pub(super) launcher: LauncherMemory,
    pub(super) tree: MemoryObserver,
    pub(super) ledger: ResourceLedger,
    pub(super) collector: ReportCollector,
    pub(super) settings: RunSettings,
    pub(super) stops: StopTimeline,
}

/// Failed preparation retains every returned resource alongside the unchanged original error.
///
/// This private custody must remain inside the execution owner's settlement flow; it is not
/// a public cleanup handle or proof that any role was retired.
pub(super) struct PreparationFailure {
    pub(super) preparation: SamplingPreparation,
    pub(super) error: SamplingError,
}

impl SamplingPreparation {
    pub(super) fn new(launcher: LauncherMemory, settings: RunSettings) -> Self {
        Self {
            launcher,
            settings,
            collector: None,
            tree: None,
            ledger: None,
            stops: None,
        }
    }

    /// Borrow the original settings without transferring or cloning preparation custody.
    pub(super) fn settings(&self) -> &RunSettings {
        &self.settings
    }

    /// One attempt, preserving original deadlines and constructor order without retry/reset.
    pub(super) fn prepare(
        mut self,
        outer: &PreparedOuter<'_>,
    ) -> Result<SamplingParts, PreparationFailure> {
        if let Err(error) = self.prepare_resources(outer) {
            return Err(PreparationFailure {
                preparation: self,
                error,
            });
        }

        // Match the entire owner at once: a missing resource retains all other objects rather
        // than partially taking them before a subsequent fallible extraction.
        match self {
            Self {
                launcher,
                settings,
                collector: Some(collector),
                tree: Some(tree),
                ledger: Some(ledger),
                stops: Some(stops),
            } => Ok(SamplingParts {
                launcher,
                tree,
                ledger,
                collector,
                settings,
                stops,
            }),
            preparation => Err(PreparationFailure {
                preparation,
                error: SamplingError::InvalidMonitorTransition,
            }),
        }
    }

    fn prepare_resources(&mut self, outer: &PreparedOuter<'_>) -> Result<(), SamplingError> {
        self.settings
            .setup_deadline
            .local()
            .map_err(SamplingError::Deadline)?;
        let deadline = self
            .settings
            .identity_deadline()
            .map_err(SamplingError::Deadline)?;

        self.collector = Some(ReportCollector::prepare(outer).map_err(SamplingError::Report)?);
        self.tree = Some(
            MemoryObserver::prepare(Path::new("/proc")).map_err(|cause| SamplingError::Io {
                operation: CauseOperation::ProcSetup,
                cause,
            })?,
        );
        let tree = self
            .tree
            .as_mut()
            .ok_or(SamplingError::InvalidMonitorTransition)?;
        tree.restrict_census(self.settings.memory_bytes)
            .map_err(|cause| SamplingError::Io {
                operation: CauseOperation::TreeObservation,
                cause,
            })?;
        tree.bind_outer(outer).map_err(|cause| SamplingError::Io {
            operation: CauseOperation::Identity,
            cause,
        })?;

        self.settings
            .setup_deadline
            .local()
            .map_err(SamplingError::Deadline)?;
        let collector = self
            .collector
            .as_ref()
            .ok_or(SamplingError::InvalidMonitorTransition)?;
        self.ledger = Some(
            ResourceLedger::prepare(
                self.settings.memory_bytes,
                self.settings.caller_run_buffers,
                collector.reserve(),
                collector.identity(),
                deadline,
            )
            .map_err(SamplingError::Charge)?,
        );
        self.stops =
            Some(StopTimeline::prepare(self.settings.started).map_err(SamplingError::Deadline)?);
        Ok(())
    }
}

//! Checked original-ceiling ledger for O's actual concurrent whole-tree observation ticks.
//!
//! The O loop supplies independent positive L RSS and private-proc O-tree RSS. Missing quantities
//! are errors, never zero. Tree peak and conservative total remain separate facts; reserved report
//! backing and finite caller buffers are charged even when sparse, unused or mapped into RSS.

use std::{num::NonZeroU64, time::Instant};

use super::{
    report_storage::{BackingReserve, PipeIdentity},
    spawner::SPAWNER_STACK_BYTES,
};

#[derive(Clone, Copy, Debug)]
pub(super) struct ChargedSample {
    pub(super) tree_rss_bytes: u64,
    pub(super) conservative_bytes: u64,
}

/// Actual recorded production peaks. Construction requires at least one complete observation;
/// report reservation, the configured ceiling and an absent sample cannot supply either value.
#[derive(Clone, Copy, Debug)]
pub(super) struct MeasuredPeaks {
    pub(super) tree_rss_bytes: u64,
    pub(super) charged_bytes: u64,
}

/// Independent production memory stop; this is not a report-classification result.
#[derive(Clone, Copy, Debug)]
pub(super) enum MemoryTick {
    WithinCeiling(ChargedSample),
    Exhausted(ChargedSample),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ChargeError {
    Deadline,
    MissingLauncherRss,
    MissingOuterTreeRss,
    MissingCallerStackReservation,
    Unrepresentable,
    MissingSetupSample,
    ResourceExhausted,
    ReservationMismatch,
    MissingMeasuredPeak,
}

impl std::fmt::Display for ChargeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "whole-run charge refused: {self:?}")
    }
}

impl std::error::Error for ChargeError {}

pub(super) struct ResourceLedger {
    ceiling: NonZeroU64,
    caller_buffers: u64,
    backing: BackingReserve,
    pipe: PipeIdentity,
    deadline: Option<Instant>,
    last_sample: Option<MemoryTick>,
    peak_tree_rss: Option<u64>,
    peak_conservative: Option<u64>,
}

impl ResourceLedger {
    /// Construction validates arithmetic before any writer is exposed. C must declare the real
    /// finite upper bound of its controls/captures/stack allocations; stack presence alone does
    /// not establish that complete bound. The owner still needs an actual setup RSS tick.
    pub(super) fn prepare(
        ceiling: NonZeroU64,
        caller_buffers: u64,
        backing: BackingReserve,
        pipe: PipeIdentity,
        deadline: Option<Instant>,
    ) -> Result<Self, ChargeError> {
        if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            return Err(ChargeError::Deadline);
        }
        let minimum =
            u64::try_from(SPAWNER_STACK_BYTES).map_err(|_| ChargeError::Unrepresentable)?;
        if caller_buffers < minimum {
            return Err(ChargeError::MissingCallerStackReservation);
        }
        backing
            .charge(0, caller_buffers)
            .map_err(|_| ChargeError::Unrepresentable)?;
        Ok(Self {
            ceiling,
            caller_buffers,
            backing,
            pipe,
            deadline,
            last_sample: None,
            peak_tree_rss: None,
            peak_conservative: None,
        })
    }

    /// Revoke writer permission before upstream creator/proc/control checks begin. A failed
    /// observation before `observe` itself is called cannot leave an earlier setup tick usable.
    pub(super) fn begin_observation(&mut self) {
        self.last_sample = None;
    }

    /// Called at setup and EVERY original observer tick, before accepting backend completion.
    /// None means unavailable observation, not zero RSS. Actual zero must be established by the
    /// observer's unchanged fresh-identity/MM-release rules. A successful ledger call itself
    /// proves neither census completeness nor that the orchestrator maintained the tick schedule.
    pub(super) fn observe(
        &mut self,
        launcher_rss: Option<u64>,
        outer_tree_rss: Option<u64>,
    ) -> Result<MemoryTick, ChargeError> {
        // An unavailable later tick invalidates earlier exposure permission.
        self.begin_observation();
        let tree_rss_bytes = launcher_rss
            .ok_or(ChargeError::MissingLauncherRss)?
            .checked_add(outer_tree_rss.ok_or(ChargeError::MissingOuterTreeRss)?)
            .ok_or(ChargeError::Unrepresentable)?;
        let conservative_bytes = self
            .backing
            .charge(tree_rss_bytes, self.caller_buffers)
            .map_err(|_| ChargeError::Unrepresentable)?;
        self.peak_tree_rss = Some(self.peak_tree_rss.unwrap_or(0).max(tree_rss_bytes));
        self.peak_conservative = Some(self.peak_conservative.unwrap_or(0).max(conservative_bytes));
        let sample = ChargedSample {
            tree_rss_bytes,
            conservative_bytes,
        };
        let tick = if conservative_bytes > self.ceiling.get() {
            // Preserve the executor's memory-before-deadline priority for an independently
            // established overage. Neither this sample nor partial content may become a proof.
            MemoryTick::Exhausted(sample)
        } else if self
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            return Err(ChargeError::Deadline);
        } else {
            MemoryTick::WithinCeiling(sample)
        };
        self.last_sample = Some(tick);
        Ok(tick)
    }

    /// No writer may be exposed without an actual successful setup sample under this original
    /// ceiling/deadline and the exact collector's independently established reservation.
    pub(super) fn require_writer_exposure(
        &self,
        pipe: PipeIdentity,
        backing: BackingReserve,
    ) -> Result<(), ChargeError> {
        if pipe != self.pipe || backing != self.backing {
            return Err(ChargeError::ReservationMismatch);
        }
        match self.last_sample {
            Some(MemoryTick::Exhausted(_)) => return Err(ChargeError::ResourceExhausted),
            None => return Err(ChargeError::MissingSetupSample),
            Some(MemoryTick::WithinCeiling(_)) => {}
        }
        if self
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            return Err(ChargeError::Deadline);
        }
        Ok(())
    }

    /// This is measurement data, not permission to return evidence. The run owner separately
    /// authenticates completion and confirms all owned settlement before using either value.
    pub(super) fn measured_peaks(&self) -> Result<MeasuredPeaks, ChargeError> {
        Ok(MeasuredPeaks {
            tree_rss_bytes: self.peak_tree_rss.ok_or(ChargeError::MissingMeasuredPeak)?,
            charged_bytes: self
                .peak_conservative
                .ok_or(ChargeError::MissingMeasuredPeak)?,
        })
    }
}

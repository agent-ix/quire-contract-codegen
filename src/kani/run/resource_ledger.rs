//! Checked original-ceiling ledger for O's actual concurrent whole-tree observation ticks.
//!
//! The O loop supplies independent positive L RSS and private-proc O-tree RSS. Missing quantities
//! are errors, never zero. Tree peak and conservative total remain separate facts; reserved report
//! backing and finite caller buffers are charged even when sparse, unused or mapped into RSS.

use std::{num::NonZeroU64, time::Instant};

use serde::{Deserialize, Serialize};

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
macro_rules! measuredpeaks_record {
    ($($variant:ident => $visibility:vis $member:ident: $value:ty),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
        #[serde(deny_unknown_fields)]
        pub(super) struct MeasuredPeaks { $($visibility $member: $value),+ }
        #[derive(Clone, Copy)]
        pub(super) enum PeakField { $($variant),+ }
        impl PeakField {
            pub(super) fn declared_order() -> &'static [Self] { &[$(Self::$variant),+] }
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($member)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}
measuredpeaks_record! {
    TreeRssBytes => pub(super) tree_rss_bytes: u64,
    ChargedBytes => pub(super) charged_bytes: u64,
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

/// Producer-owned complete history is distinct from current exposure permission. The only
/// transition out of Unobserved is after both RSS inputs and the full named charge succeed.
/// This private state alone selects neither a timeout reason nor settled evidence.
#[derive(Clone, Copy)]
enum CompleteHistory {
    Unobserved,
    Recorded(MeasuredPeaks),
}

pub(super) struct ResourceLedger {
    ceiling: NonZeroU64,
    caller_buffers: u64,
    backing: BackingReserve,
    pipe: PipeIdentity,
    deadline: Option<Instant>,
    last_sample: Option<MemoryTick>,
    history: CompleteHistory,
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
            history: CompleteHistory::Unobserved,
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
        let peaks = match self.history {
            CompleteHistory::Unobserved => MeasuredPeaks {
                tree_rss_bytes,
                charged_bytes: conservative_bytes,
            },
            CompleteHistory::Recorded(previous) => MeasuredPeaks {
                tree_rss_bytes: previous.tree_rss_bytes.max(tree_rss_bytes),
                charged_bytes: previous.charged_bytes.max(conservative_bytes),
            },
        };
        // A complete charge remains historical fact even if the original deadline check
        // below refuses current exposure. Admission and positive absence must not be inferred
        // from last_sample or from this call's Result alone.
        self.history = CompleteHistory::Recorded(peaks);
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
        match self.history {
            CompleteHistory::Unobserved => Err(ChargeError::MissingMeasuredPeak),
            CompleteHistory::Recorded(peaks) => Ok(peaks),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Trace: FR-034-AC-32
    #[test]
    fn complete_history_survives_later_unavailable_samples_without_fabricated_initial_peak() {
        // Pure named-charge arithmetic, not actual proc completeness, O sampling or settlement.
        let caller = u64::try_from(SPAWNER_STACK_BYTES).unwrap();
        let backing = BackingReserve {
            pipe_bytes: 4096,
            memfd_bytes: 8192,
        };
        let pipe = PipeIdentity::from_wire_parts(1, 2);
        let mut ledger = ResourceLedger::prepare(
            NonZeroU64::new(u64::MAX).unwrap(),
            caller,
            backing,
            pipe,
            None,
        )
        .unwrap();
        assert_eq!(
            ledger.measured_peaks().unwrap_err(),
            ChargeError::MissingMeasuredPeak
        );
        assert_eq!(
            ledger.observe(None, Some(200)).unwrap_err(),
            ChargeError::MissingLauncherRss
        );
        assert_eq!(
            ledger.measured_peaks().unwrap_err(),
            ChargeError::MissingMeasuredPeak
        );
        assert!(matches!(
            ledger.observe(Some(100), Some(200)).unwrap(),
            MemoryTick::WithinCeiling(_)
        ));
        let recorded = ledger.measured_peaks().unwrap();
        assert_eq!(recorded.tree_rss_bytes, 300);
        assert_eq!(recorded.charged_bytes, backing.charge(300, caller).unwrap());
        assert_eq!(
            ledger.observe(Some(1000), None).unwrap_err(),
            ChargeError::MissingOuterTreeRss
        );
        assert_eq!(
            ledger.require_writer_exposure(pipe, backing).unwrap_err(),
            ChargeError::MissingSetupSample
        );
        assert_eq!(
            ledger.measured_peaks().unwrap().tree_rss_bytes,
            recorded.tree_rss_bytes
        );
        assert_eq!(
            ledger.measured_peaks().unwrap().charged_bytes,
            recorded.charged_bytes
        );
        assert!(matches!(
            ledger.observe(Some(10), Some(20)).unwrap(),
            MemoryTick::WithinCeiling(_)
        ));
        assert_eq!(
            ledger.measured_peaks().unwrap().tree_rss_bytes,
            recorded.tree_rss_bytes
        );
        assert_eq!(
            ledger.measured_peaks().unwrap().charged_bytes,
            recorded.charged_bytes
        );
    }
    /// Trace: FR-034-AC-32
    #[test]
    fn complete_charge_precedes_deadline_refusal_and_actual_overage_keeps_priority() {
        let caller = u64::try_from(SPAWNER_STACK_BYTES).unwrap();
        let backing = BackingReserve {
            pipe_bytes: 4096,
            memfd_bytes: 8192,
        };
        let pipe = PipeIdentity::from_wire_parts(1, 2);
        let mut ledger = ResourceLedger::prepare(
            NonZeroU64::new(u64::MAX).unwrap(),
            caller,
            backing,
            pipe,
            None,
        )
        .unwrap();
        // A deterministic already-expired pure-ledger state; no sleeps, role/proc witness,
        // producer admission or emitted evidence is inferred from this fixture.
        ledger.deadline = Some(Instant::now());
        assert_eq!(
            ledger.observe(Some(100), Some(200)).unwrap_err(),
            ChargeError::Deadline
        );
        let complete = ledger.measured_peaks().unwrap();
        assert_eq!(complete.tree_rss_bytes, 300);
        assert_eq!(complete.charged_bytes, caller + 300 + 4096 + 8192);
        assert_eq!(
            ledger.require_writer_exposure(pipe, backing).unwrap_err(),
            ChargeError::MissingSetupSample
        );
        ledger.ceiling = NonZeroU64::new(complete.charged_bytes - 1).unwrap();
        assert!(matches!(
            ledger.observe(Some(100), Some(200)).unwrap(),
            MemoryTick::Exhausted(_)
        ));
        assert_eq!(
            ledger.require_writer_exposure(pipe, backing).unwrap_err(),
            ChargeError::ResourceExhausted
        );
        assert_eq!(
            ledger.measured_peaks().unwrap().charged_bytes,
            complete.charged_bytes
        );
    }
}

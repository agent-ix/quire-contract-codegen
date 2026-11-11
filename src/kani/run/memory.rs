//! Peak memory the caller's environment provides for a finished run (FR-028-AC-21, ADR-007).
//!
//! The runner measures no memory itself and enforces no cap: the caller's environment (a cgroup,
//! `systemd-run`, a container) owns the cap. A [`PeakMemorySource`] is the seam through which an
//! environment reports the largest resident set of any one process of a finished run. A run
//! whose environment provides no peak records none; no estimate, zero or launcher-only figure
//! stands in for it.

use std::fmt::Debug;

use serde::Serialize;

/// The unit a [`PeakMemorySource`] reports in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryUnit {
    /// Bytes, as `ru_maxrss` is on macOS.
    Bytes,
    /// Kibibytes, as `ru_maxrss` is on Linux.
    Kibibytes,
}

/// One peak figure in the unit its source reports.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PeakReading {
    /// The figure, in `unit`.
    pub value: u64,
    /// The unit of `value`.
    pub unit: MemoryUnit,
}

impl PeakReading {
    /// The figure in bytes, or none when it does not fit a `u64`.
    fn bytes(self) -> Option<u64> {
        match self.unit {
            MemoryUnit::Bytes => Some(self.value),
            MemoryUnit::Kibibytes => self.value.checked_mul(1024),
        }
    }
}

/// A named source of the peak resident memory of a finished run.
pub trait PeakMemorySource: Debug + Send + Sync {
    /// The name the evidence records beside the peak.
    fn name(&self) -> &str;

    /// The largest resident set of any one process in the run whose launcher had process id
    /// `launcher`, or none when the environment provides none.
    fn peak(&self, launcher: u32) -> Option<PeakReading>;
}

/// The source of an environment that provides no peak: runs record none.
#[derive(Debug)]
pub(super) struct NoPeakMemory;

impl PeakMemorySource for NoPeakMemory {
    fn name(&self) -> &str {
        "none"
    }

    fn peak(&self, _launcher: u32) -> Option<PeakReading> {
        None
    }
}

/// The peak memory recorded by one launch, with the name of the source that provided it.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryObservation {
    /// The name of the source that provided the peak. Absent when none provided one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Largest resident set of any one process of the run, in bytes. Absent when the environment
    /// provided none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_resident_bytes: Option<u64>,
}

impl MemoryObservation {
    /// The observation of `source` for the run led by `launcher`, normalised to bytes.
    pub(super) fn observe(source: &dyn PeakMemorySource, launcher: u32) -> Self {
        match source.peak(launcher).and_then(PeakReading::bytes) {
            Some(bytes) => Self {
                source: Some(source.name().to_owned()),
                peak_resident_bytes: Some(bytes),
            },
            None => Self::default(),
        }
    }
}

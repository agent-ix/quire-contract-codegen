//! C's completed-backend result assembly after authenticated whole-chain settlement.
//!
//! The caller supplies genuine settled captures, backend exit, report candidate and complete
//! terminal measurements. This module neither settles roles nor selects timeout/resource stops,
//! and reading or assembling a report does not establish execution evidence.

use std::num::NonZeroUsize;

use crate::kani::output::report::KaniReportRefusal;

use super::{
    caller_streams::{PreparedCaptureText, SettledCaptures},
    launch::{stream_bytes, BoundedLaunch, BoundedLaunchError, CaptureStream, LaunchOutcome},
    memory::{MemoryMechanism, MemoryObservation},
    protocol::BackendExit,
    resource_ledger::MeasuredPeaks,
};

/// Assemble only an actual completed backend after the caller has positively confirmed role,
/// capture and creator settlement. `peaks` must come from its authenticated complete terminal
/// observation and normal owned O exit; no configured ceiling or missing sample substitutes.
/// The supplied text storage was reserved and charged before L. Capture failures discard the
/// report candidate and retain their existing stdout-first refusal precedence; no result or error
/// carries cleanup authority.
pub(super) fn assemble_completed(
    outcome: BackendExit,
    captures: SettledCaptures,
    text: PreparedCaptureText,
    report: Result<Option<Vec<u8>>, KaniReportRefusal>,
    peaks: MeasuredPeaks,
    limit: usize,
    harnesses: NonZeroUsize,
) -> Result<BoundedLaunch, BoundedLaunchError> {
    let memory = MemoryObservation {
        mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
        peak_resident_bytes: Some(peaks.tree_rss_bytes),
    };
    let stdout = match stream_bytes(CaptureStream::Stdout, captures.stdout, limit, harnesses) {
        Ok(bytes) => bytes,
        Err(outcome) => {
            return Ok(BoundedLaunch {
                report: Ok(None),
                outcome,
                memory,
            });
        }
    };
    let stderr = match stream_bytes(CaptureStream::Stderr, captures.stderr, limit, harnesses) {
        Ok(bytes) => bytes,
        Err(outcome) => {
            return Ok(BoundedLaunch {
                report: Ok(None),
                outcome,
                memory,
            });
        }
    };
    let text = text
        .decode_joined(&stdout, &stderr, limit)
        .map_err(BoundedLaunchError::Io)?;
    let (exited_successfully, exit_code) = match outcome {
        BackendExit::Code(code) => (code == 0, Some(code)),
        BackendExit::Signal(_) => (false, None),
    };
    Ok(BoundedLaunch {
        report,
        outcome: LaunchOutcome::Completed {
            exited_successfully,
            exit_code,
            text,
        },
        memory,
    })
}

// One execution: the request, refusal and evidence types and `execute_kani_obligation`.
pub(crate) mod execute;

// One content cap shared by collection, consumers and serialized production evidence.
pub(super) const REPORT_CONTENT_BYTES: u64 = 16 * 1_048_576;
// The harness of any kind this crate can run.
pub(crate) mod harness;
// Spawn, bounded capture, timeout and process-group kill.
pub(crate) mod launch;
// The unique report path, stale removal and bounded read.
pub(crate) mod report_file;
// The backend and its location.
pub(crate) mod tool;

// Backend-tree resident-memory observation.
pub(crate) mod memory;

mod namespace;

// Anonymous lease framing exists only on the supported guardian platform.
#[cfg(target_os = "linux")]
mod control;

// Actual creator-thread liveness, including the retained outside L role.
#[cfg(target_os = "linux")]
mod creator;

#[cfg(target_os = "linux")]
mod protocol;

#[cfg(target_os = "linux")]
mod guardian;

#[cfg(target_os = "linux")]
mod stages;

#[cfg(target_os = "linux")]
mod owned;

#[cfg(target_os = "linux")]
mod publication;

#[cfg(feature = "guardian-test-support")]
pub(crate) mod fixture;

pub(crate) fn guardian_entry() -> std::process::ExitCode {
    #[cfg(target_os = "linux")]
    {
        match guardian::run(protocol::current_build_identity()) {
            Ok(()) | Err(guardian::GuardianError::Control(control::ControlError::Eof)) => {
                std::process::ExitCode::SUCCESS
            }
            Err(
                guardian::GuardianError::Control(_)
                | guardian::GuardianError::Refusal(_)
                | guardian::GuardianError::Io(_),
            ) => std::process::ExitCode::FAILURE,
        }
    }
    #[cfg(not(target_os = "linux"))]
    std::process::ExitCode::FAILURE
}

// Internal original backend stdin capture precedes control-descriptor creation.
pub(crate) mod stdin;

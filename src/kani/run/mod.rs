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

#[cfg(target_os = "linux")]
mod backend_exec;
#[cfg(target_os = "linux")]
mod backend_installer;
#[cfg(target_os = "linux")]
mod backend_policy;
#[cfg(target_os = "linux")]
mod caller_bootstrap;
#[cfg(target_os = "linux")]
mod caller_driver;
#[cfg(target_os = "linux")]
mod caller_execution;
#[cfg(target_os = "linux")]
mod caller_prepare;
#[cfg(target_os = "linux")]
mod caller_result;
#[cfg(target_os = "linux")]
mod caller_streams;
#[cfg(target_os = "linux")]
mod helper_entry;
#[cfg(target_os = "linux")]
mod launcher_owner;

// Actual creator-thread liveness, including the retained outside L role.
#[cfg(target_os = "linux")]
mod creator;

// Linux role/setup/storage primitives are compiled with the actual helper library artifact.
// Registration alone does not replace the existing production launch orchestration.
#[cfg(target_os = "linux")]
mod outer_sampling;
#[cfg(target_os = "linux")]
mod outer_setup;
#[cfg(target_os = "linux")]
mod owner_protection;
#[cfg(target_os = "linux")]
mod pipe_policy;
#[cfg(target_os = "linux")]
mod report_storage;
#[cfg(target_os = "linux")]
mod resource_ledger;
#[cfg(target_os = "linux")]
mod role_bootstrap;
#[cfg(target_os = "linux")]
mod role_command;
#[cfg(target_os = "linux")]
mod role_deadline;
#[cfg(target_os = "linux")]
mod role_protocol;
#[cfg(target_os = "linux")]
mod spawner;
#[cfg(target_os = "linux")]
mod startup_cause;
#[cfg(target_os = "linux")]
mod startup_envelope;
#[cfg(target_os = "linux")]
mod startup_projection;

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
        // Every role selects only this same actual helper artifact. Unknown/excess argv
        // refuses before bootstrap; no role can bypass its authenticated retained owner.
        let role = match role_command::HelperRole::select(std::env::args_os().skip(1)) {
            Ok(role) => role,
            Err(_) => return std::process::ExitCode::FAILURE,
        };
        match helper_entry::run(role, protocol::current_build_identity()) {
            Ok(()) => std::process::ExitCode::SUCCESS,
            Err(_) => std::process::ExitCode::FAILURE,
        }
    }
    #[cfg(not(target_os = "linux"))]
    std::process::ExitCode::FAILURE
}

// Internal original backend stdin capture precedes control-descriptor creation.
pub(crate) mod stdin;

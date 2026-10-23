// One execution: the request, refusal and evidence types and `execute_kani_obligation`.
pub(crate) mod execute;
// The harness of any kind this crate can run.
pub(crate) mod harness;
// Spawn, bounded capture, timeout and process-group kill.
pub(crate) mod launch;
// Shared bounded direct-child process API built on the owned launcher.
pub mod process;
// The unique report path, stale removal and bounded read.
pub(crate) mod report_file;
// Private staging and paired Kani/native execution for routed scalar obligations.
pub(crate) mod scalar;
// The backend and its location.
pub(crate) mod tool;

// Backend-tree resident-memory observation.
pub(crate) mod memory;

mod namespace;

// Construction-time dependencies of the existing launch owners.
mod owners;

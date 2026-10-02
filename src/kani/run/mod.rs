// One execution: the request, refusal and evidence types and `execute_kani_obligation`.
pub(crate) mod execute;
// The harness of any kind this crate can run.
pub(crate) mod harness;
// Spawn, bounded capture, timeout and process-group kill.
pub(crate) mod launch;
// The unique report path, stale removal and bounded read.
pub(crate) mod report_file;
// The backend and its location.
pub(crate) mod tool;

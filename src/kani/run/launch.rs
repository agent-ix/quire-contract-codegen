//! Launching the Kani launcher process: spawn as the leader of its own process group, bounded
//! capture, the wall-clock budget and the group kill (FR-017, FR-028; ADR-007).

use std::{
    fmt, io,
    io::{Read, Write},
    num::NonZeroUsize,
    os::{fd::AsFd, unix::process::CommandExt},
    path::Path,
    process::{Child, ChildStdin, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use super::{
    memory::MemoryObservation,
    owners::{LaunchOwners, PollOperation},
};

use quire_contract_ir::kani::KaniCancellationSource;
use quire_exact::{Cancel, CancelCause};

use rustix::{
    event::{PollFd, PollFlags},
    fs::{fcntl_getfl, fcntl_setfl, OFlags},
    io::Errno,
    process::{waitid, Pid, WaitId, WaitIdOptions},
    time::Timespec,
};

/// The launcher's two output streams, each whole, as the process wrote them.
#[derive(Debug, Default)]
pub(crate) struct CapturedStreams {
    /// Everything the process wrote to stdout.
    pub(crate) stdout: Vec<u8>,
    /// Everything the process wrote to stderr.
    pub(crate) stderr: Vec<u8>,
}

impl CapturedStreams {
    /// Stdout and stderr, each decoded lossily and joined by one line feed: the console text
    /// `classify_kani_run` reads.
    pub(super) fn merged_text(&self) -> String {
        format!(
            "{}\n{}",
            String::from_utf8_lossy(&self.stdout),
            String::from_utf8_lossy(&self.stderr)
        )
    }
}

/// How the launcher's run within its caller-declared budget
/// (the wall-clock ceiling of its identity for a bounded backend launch)
/// concluded.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum LaunchOutcome {
    /// The process exited on its own within the budget.
    Completed {
        /// `ExitStatus::success()`.
        exited_successfully: bool,
        /// `ExitStatus::code()`.
        exit_code: Option<i32>,
        /// The two streams, each whole. The only copy: the merged text classification reads
        /// is built from it by [`CapturedStreams::merged_text`].
        streams: CapturedStreams,
    },
    /// The budget elapsed before the process exited. The launcher's whole process group has been
    /// killed and the launcher reaped.
    TimedOut,
    /// The caller-owned handle stopped the run with this retained source.
    Cancelled { source: KaniCancellationSource },
    /// A stream carried more than `limit` bytes. The run was stopped and the launcher's whole
    /// process group killed; no text is retained, because a truncated stream is evidence nobody
    /// can vouch for (FR-017-AC-14).
    OutputOverLimit {
        /// The stream that carried too much.
        stream: CaptureStream,
        /// The most it may carry: 8 MiB times `harnesses`.
        limit: usize,
        /// How many harnesses the process ran.
        harnesses: usize,
    },
    /// A stream could not be read to its end: its capture thread panicked, or a poll or read of
    /// the pipe failed. The run was stopped and the group killed; an unread stream is not an
    /// empty one, so no text is returned (FR-017-AC-25).
    OutputUnread {
        /// The stream that was not read.
        stream: CaptureStream,
        /// What went wrong, as text.
        detail: String,
    },
}

/// One of the launcher's two output streams.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaptureStream {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

impl fmt::Display for CaptureStream {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
        })
    }
}

/// Why a capture thread did not return its stream.
#[derive(Debug, Eq, PartialEq)]
enum CaptureFailure {
    /// More than the limit was read; only the bounded prefix is retained for non-Kani callers.
    OverLimit { prefix: Vec<u8> },
    /// The pipe could not be polled or read, or its thread panicked.
    Unread { detail: String },
}

/// What one capture thread returns: the whole stream, or why it could not.
type Captured = Result<Vec<u8>, CaptureFailure>;

/// Polling interval while waiting for the launcher to exit within its budget, and the longest a
/// capture thread goes between looking at its stop flag. Short enough that a tight caller-declared
/// timeout in a test is still observed promptly, long enough not to spin.
const LAUNCHER_POLL_INTERVAL: Duration = Duration::from_millis(20);

/// Most bytes each of the launcher's stdout and stderr may carry, per harness the process runs. A
/// stream over it is refused, never truncated: the verdict is in the exported report, but the
/// playback a falsified harness is attributed from is printed in the stream, and a cut stream
/// cannot say which blocks it lost.
pub(super) const CAPTURE_LIMIT: usize = 8 * 1024 * 1024;

/// Longest a capture thread keeps reading after it is told to stop. Whatever the launcher wrote
/// before it ended is already in the pipe and is read in microseconds; the limit only bounds a
/// straggler that keeps writing.
const STOP_DRAIN_LIMIT: Duration = Duration::from_millis(100);

/// Runs `command` to completion or kills its process group once `timeout` elapses, whichever
/// happens first, and **returns** within `timeout` plus a small constant (the poll interval and
/// `STOP_DRAIN_LIMIT`) either way; that bound does not depend on whether the kill reached
/// everything. A `timeout` too large to add to the current instant never elapses.
///
/// The launcher runs as the leader of its own process group, so a timeout kills CBMC and
/// every other descendant along with it instead of leaving them running past the budget; see
/// `LaunchTeardown::signal_group`. The group is no longer the terminal's foreground group, so a Ctrl-C
/// typed at the caller's terminal reaches the caller and not the launcher or its descendants:
/// a caller that is interrupted and exits without returning from this function leaves them
/// running until they finish on their own.
///
/// Stdout and stderr are drained on their own threads as soon as the process is spawned, the same
/// way `Command::output()` drains them internally: a full pipe buffer would otherwise stall the
/// child while this function is only polling `try_wait`, turning a bounded run into a hang of its
/// own. Each thread keeps its whole stream, up to 8 MiB, and polls its pipe
/// rather than blocking in `read`. A stream over the limit, a pipe that cannot be polled or read
/// and a thread that panics each stop the run at once and kill the group; the call then returns
/// [`LaunchOutcome::OutputOverLimit`] or [`LaunchOutcome::OutputUnread`] and no text.
///
/// Once the launcher is gone, however it went, its process group is killed (a descendant that
/// outlived a launcher that exited on its own is killed here too, as one is on a timeout), then
/// this function tells both threads to stop and joins them: a descendant that left the process
/// group may still hold the pipe's write end open. Each thread then reads what is already in the
/// pipe for at most `STOP_DRAIN_LIMIT` and returns, however fast a straggler keeps writing, so the
/// join adds at most that limit plus one poll interval to the return time.
///
/// This is the run of one harness; a batch multiplies the limit by its harness count.
#[cfg(test)]
fn run_launcher_with_timeout(command: Command, timeout: Duration) -> io::Result<LaunchOutcome> {
    run_launcher(command, timeout, NonZeroUsize::MIN)
}

/// [`run_launcher_with_timeout`] for a process that runs `harnesses` harnesses: each stream may
/// carry [`CAPTURE_LIMIT`] bytes for every one.
#[cfg(test)]
pub(super) fn run_launcher(
    command: Command,
    timeout: Duration,
    harnesses: NonZeroUsize,
) -> io::Result<LaunchOutcome> {
    run_monitored(
        command,
        timeout,
        harnesses,
        None,
        &Cancel::new(),
        &LaunchOwners::real(),
    )
}

/// A bounded launch together with the peak memory the environment provided for it.
pub(super) struct BoundedLaunch {
    pub(super) outcome: LaunchOutcome,
    pub(super) memory: MemoryObservation,
}

/// The operation that failed before a complete child outcome was available.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LaunchFailurePhase {
    Spawn,
    CaptureSetup,
    Wait,
}

#[derive(Debug)]
struct StagedLaunchError {
    phase: LaunchFailurePhase,
    source: io::Error,
}

impl fmt::Display for StagedLaunchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.source.fmt(formatter)
    }
}

impl std::error::Error for StagedLaunchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

fn staged_error(phase: LaunchFailurePhase, source: io::Error, shared_process: bool) -> io::Error {
    if shared_process {
        io::Error::new(source.kind(), StagedLaunchError { phase, source })
    } else {
        source
    }
}

/// Inspect the typed phase without interpreting an error message.
pub(crate) fn failure_phase(error: &io::Error) -> Option<LaunchFailurePhase> {
    error
        .get_ref()
        .and_then(|cause| cause.downcast_ref::<StagedLaunchError>())
        .map(|cause| cause.phase)
}

/// Shared run facts for a backend whose response has its own domain interpretation.
pub(crate) struct SharedLaunch {
    pub(crate) outcome: LaunchOutcome,
    pub(crate) started: bool,
    /// The launcher's process id when one was started.
    pub(crate) launcher: Option<u32>,
    pub(crate) input_failure: Option<String>,
    pub(crate) partial_stdout: Option<Vec<u8>>,
    pub(crate) partial_stderr: Option<Vec<u8>>,
}

impl SharedLaunch {
    fn not_started(outcome: LaunchOutcome) -> Self {
        Self {
            outcome,
            started: false,
            launcher: None,
            input_failure: None,
            partial_stdout: None,
            partial_stderr: None,
        }
    }
}

/// Run a direct selected executable through the same group-owned launcher as Kani.
pub(crate) fn run_bounded_process_internal(
    executable: &Path,
    arguments: &[std::ffi::OsString],
    input: Vec<u8>,
    wall_clock: Duration,
    output_limit: usize,
    deadline: Instant,
    cancel: &Cancel,
) -> io::Result<SharedLaunch> {
    let mut command = Command::new(executable);
    command.args(arguments);
    run_monitored_with(
        command,
        wall_clock,
        Some(deadline),
        cancel,
        &LaunchOwners::real(),
        Some(input),
        output_limit,
        NonZeroUsize::MIN,
    )
}

/// Run a backend within `wall_clock`, retaining the peak memory the environment provides.
pub(super) fn run_bounded_launcher(
    command: Command,
    wall_clock: Duration,
    harnesses: NonZeroUsize,
    deadline: Option<Instant>,
    cancel: &Cancel,
    owners: &LaunchOwners,
) -> io::Result<BoundedLaunch> {
    let run = run_monitored_with(
        command,
        wall_clock,
        deadline,
        cancel,
        owners,
        None,
        CAPTURE_LIMIT.saturating_mul(harnesses.get()),
        harnesses,
    )?;
    let memory = match run.launcher {
        Some(launcher) => MemoryObservation::observe(owners.peak.as_ref(), launcher),
        None => MemoryObservation::default(),
    };
    Ok(BoundedLaunch {
        outcome: run.outcome,
        memory,
    })
}

#[cfg(test)]
fn run_monitored(
    command: Command,
    timeout: Duration,
    harnesses: NonZeroUsize,
    supplied_deadline: Option<Instant>,
    cancel: &Cancel,
    owners: &LaunchOwners,
) -> io::Result<LaunchOutcome> {
    run_monitored_with(
        command,
        timeout,
        supplied_deadline,
        cancel,
        owners,
        None,
        CAPTURE_LIMIT.saturating_mul(harnesses.get()),
        harnesses,
    )
    .map(|run| run.outcome)
}

#[allow(clippy::too_many_arguments)] // Existing launch owners and resource boundaries stay explicit.
fn run_monitored_with(
    mut command: Command,
    timeout: Duration,
    supplied_deadline: Option<Instant>,
    cancel: &Cancel,
    owners: &LaunchOwners,
    input: Option<Vec<u8>>,
    limit: usize,
    harnesses: NonZeroUsize,
) -> io::Result<SharedLaunch> {
    let shared_process = input.is_some();
    // The launcher leads a new process group: one group signal reaches every descendant.
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0);
    if input.is_some() {
        command.stdin(Stdio::piped());
    }
    let deadline = supplied_deadline.or_else(|| Instant::now().checked_add(timeout));
    if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
        return Ok(SharedLaunch::not_started(LaunchOutcome::TimedOut));
    }
    if let Some(source) = observed_cancellation(cancel) {
        return Ok(SharedLaunch::not_started(LaunchOutcome::Cancelled {
            source,
        }));
    }
    let mut child = command
        .spawn()
        .map_err(|error| staged_error(LaunchFailurePhase::Spawn, error, shared_process))?;
    let launcher = child.id();
    drop(command);
    let (Some(stdout), Some(stderr)) = (child.stdout.take(), child.stderr.take()) else {
        owners.teardown.signal_group(&mut child);
        let _ = owners.teardown.reap(&mut child);
        return Err(staged_error(
            LaunchFailurePhase::CaptureSetup,
            io::Error::other("a piped standard stream was not captured"),
            shared_process,
        ));
    };
    let flags = CaptureFlags {
        stop: Arc::new(AtomicBool::new(false)),
        failed: Arc::new(AtomicBool::new(false)),
    };
    let stdout_reader = spawn_capture(stdout, &flags, limit, Arc::clone(&owners.stdout));
    let stderr_reader = spawn_capture(stderr, &flags, limit, Arc::clone(&owners.stderr));
    let input_stop = Arc::new(AtomicBool::new(false));
    let (input_writer, mut input_failure) = match input {
        Some(bytes) => match child.stdin.take() {
            Some(pipe) => match spawn_input(
                pipe,
                bytes,
                Arc::clone(&input_stop),
                Arc::clone(&flags.failed),
            ) {
                Ok(writer) => (Some(writer), None),
                Err(error) => {
                    flags.failed.store(true, Ordering::Release);
                    (None, Some(error.to_string()))
                }
            },
            None => {
                flags.failed.store(true, Ordering::Release);
                (None, Some("piped stdin was not captured".to_owned()))
            }
        },
        None => (None, None),
    };

    let exited = wait_until(&child, deadline, &flags.failed, cancel);
    input_stop.store(true, Ordering::Release);
    // The group is signalled while the launcher is still unreaped, so the unreaped launcher
    // reserves the group id (FR-017-AC-68).
    owners.teardown.signal_group(&mut child);
    let reaped = owners.teardown.reap(&mut child);
    flags.stop.store(true, Ordering::Release);
    let stdout_bytes = finish_capture(stdout_reader);
    let stderr_bytes = finish_capture(stderr_reader);
    let (mut partial_stdout, stdout_failure) = match stdout_bytes {
        Ok(bytes) => (Some(bytes), None),
        Err(CaptureFailure::OverLimit { prefix }) => (
            Some(prefix),
            Some(CaptureFailure::OverLimit { prefix: Vec::new() }),
        ),
        Err(failure) => (None, Some(failure)),
    };
    let (mut partial_stderr, stderr_failure) = match stderr_bytes {
        Ok(bytes) => (Some(bytes), None),
        Err(CaptureFailure::OverLimit { prefix }) => (
            Some(prefix),
            Some(CaptureFailure::OverLimit { prefix: Vec::new() }),
        ),
        Err(failure) => (None, Some(failure)),
    };
    if let Some(writer) = input_writer {
        match writer.join() {
            Ok(Err(error)) => input_failure = Some(error.to_string()),
            Err(_) => input_failure = Some("the stdin writer panicked".to_owned()),
            Ok(Ok(false)) if matches!(exited.as_ref(), Ok(WaitConclusion::Completed)) => {
                input_failure = Some("stdin was not written whole before process exit".to_owned());
            }
            Ok(Ok(_)) => {}
        }
    }
    let exited =
        exited.map_err(|error| staged_error(LaunchFailurePhase::Wait, error, shared_process))?;
    let reaped =
        reaped.map_err(|error| staged_error(LaunchFailurePhase::Wait, error, shared_process))?;

    for (stream, failure) in [
        (CaptureStream::Stdout, stdout_failure),
        (CaptureStream::Stderr, stderr_failure),
    ] {
        if let Some(failure) = failure {
            return Ok(SharedLaunch {
                outcome: capture_failure_outcome(stream, failure, limit, harnesses),
                started: true,
                launcher: Some(launcher),
                input_failure,
                partial_stdout,
                partial_stderr,
            });
        }
    }

    let outcome = match exited {
        WaitConclusion::Completed => LaunchOutcome::Completed {
            exited_successfully: reaped.success(),
            exit_code: reaped.code(),
            streams: CapturedStreams {
                stdout: partial_stdout
                    .take()
                    .ok_or_else(|| io::Error::other("stdout capture missing"))?,
                stderr: partial_stderr
                    .take()
                    .ok_or_else(|| io::Error::other("stderr capture missing"))?,
            },
        },
        WaitConclusion::TimedOut => LaunchOutcome::TimedOut,
        WaitConclusion::Cancelled { source } => LaunchOutcome::Cancelled { source },
    };
    Ok(SharedLaunch {
        outcome,
        started: true,
        launcher: Some(launcher),
        input_failure,
        partial_stdout,
        partial_stderr,
    })
}

/// The bytes of `stream`, or the outcome that refuses the run because they were not all read or
/// were more than `limit`.
#[cfg(test)]
fn stream_bytes(
    stream: CaptureStream,
    captured: Captured,
    limit: usize,
    harnesses: NonZeroUsize,
) -> Result<Vec<u8>, LaunchOutcome> {
    captured.map_err(|failure| capture_failure_outcome(stream, failure, limit, harnesses))
}

fn capture_failure_outcome(
    stream: CaptureStream,
    failure: CaptureFailure,
    limit: usize,
    harnesses: NonZeroUsize,
) -> LaunchOutcome {
    match failure {
        CaptureFailure::OverLimit { .. } => LaunchOutcome::OutputOverLimit {
            stream,
            limit,
            harnesses: harnesses.get(),
        },
        CaptureFailure::Unread { detail } => LaunchOutcome::OutputUnread { stream, detail },
    }
}

/// The two flags the capture threads and the run share.
struct CaptureFlags {
    /// Set by the run once the launcher is gone: read what is in the pipe and return.
    stop: Arc<AtomicBool>,
    /// Set by a capture thread that failed: the run can no longer be trusted, stop waiting.
    failed: Arc<AtomicBool>,
}

/// The settled reason for stopping the unreaped launcher.
#[derive(Debug)]
enum WaitConclusion {
    Completed,
    TimedOut,
    Cancelled { source: KaniCancellationSource },
}

/// Polls the unreaped launcher until it exits, the deadline passes, a capture fails or the
/// caller cancels. An exited launcher remains unreaped until the group has been signalled:
/// otherwise its pid, the group id, could be recycled and a later signal could reach an
/// unrelated group.
fn wait_until(
    child: &Child,
    deadline: Option<Instant>,
    failed: &AtomicBool,
    cancel: &Cancel,
) -> io::Result<WaitConclusion> {
    let pid = i32::try_from(child.id())
        .ok()
        .and_then(Pid::from_raw)
        .ok_or_else(|| io::Error::other("the launcher's process id is not a valid pid"))?;
    loop {
        // An exited launcher cannot turn an already elapsed ceiling into completion.
        if failed.load(Ordering::Acquire)
            || deadline.is_some_and(|deadline| Instant::now() >= deadline)
        {
            return Ok(WaitConclusion::TimedOut);
        }
        if let Some(source) = observed_cancellation(cancel) {
            return Ok(WaitConclusion::Cancelled { source });
        }
        match waitid(
            WaitId::Pid(pid),
            WaitIdOptions::EXITED | WaitIdOptions::NOHANG | WaitIdOptions::NOWAIT,
        ) {
            Ok(Some(_)) => return Ok(WaitConclusion::Completed),
            Ok(None) | Err(Errno::INTR) => {}
            Err(errno) => return Err(errno.into()),
        }
        thread::sleep(LAUNCHER_POLL_INTERVAL);
    }
}

/// Starts a thread that reads `pipe` to its end, or until `stop` is set and nothing more is
/// waiting in it. A failure sets `failed`, so the run stops waiting for a launcher whose output
/// nobody is reading any more.
fn spawn_capture<R>(
    pipe: R,
    flags: &CaptureFlags,
    limit: usize,
    poller: Arc<dyn PollOperation>,
) -> thread::JoinHandle<Captured>
where
    R: Read + AsFd + Send + 'static,
{
    let stop = Arc::clone(&flags.stop);
    let failure = FlagOnFailure {
        flag: Arc::clone(&flags.failed),
        failed: true,
    };
    thread::spawn(move || {
        // The whole guard moves into the thread: a closure naming only `failure.failed` would
        // capture that field and drop the guard, and set the flag, here.
        let mut failure = failure;
        let captured = capture(pipe, &stop, limit, STOP_DRAIN_LIMIT, |fds, timeout| {
            poller.poll(fds, timeout)
        });
        failure.failed = captured.is_err();
        captured
    })
}

/// Write exact input on a nonblocking pipe so a child that never reads stdin cannot hold cleanup.
fn spawn_input(
    mut pipe: ChildStdin,
    bytes: Vec<u8>,
    stop: Arc<AtomicBool>,
    failed: Arc<AtomicBool>,
) -> io::Result<thread::JoinHandle<io::Result<bool>>> {
    let flags = fcntl_getfl(&pipe)?;
    fcntl_setfl(&pipe, flags | OFlags::NONBLOCK)?;
    Ok(thread::spawn(move || {
        let mut failure = FlagOnFailure {
            flag: failed,
            failed: true,
        };
        let mut offset = 0;
        let result = loop {
            if offset == bytes.len() {
                break Ok(true);
            }
            if stop.load(Ordering::Acquire) {
                break Ok(false);
            }
            let end = bytes.len().min(offset.saturating_add(64 * 1024));
            match pipe.write(&bytes[offset..end]) {
                Ok(0) => break Err(io::Error::from(io::ErrorKind::WriteZero)),
                Ok(written) => offset += written,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    let mut ready = [PollFd::new(&pipe, PollFlags::OUT)];
                    let wait = Timespec::try_from(LAUNCHER_POLL_INTERVAL).unwrap_or_default();
                    match rustix::event::poll(&mut ready, Some(&wait)) {
                        Ok(_) | Err(Errno::INTR) => {}
                        Err(error) => break Err(error.into()),
                    }
                }
                Err(_) if stop.load(Ordering::Acquire) => break Ok(false),
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                Err(error) => break Err(error),
            }
        };
        failure.failed = result.is_err();
        result
    }))
}

/// Sets `flag` when dropped while `failed`, which a capture thread is until it has returned its
/// stream: a thread that panics unwinds through this and so stops the run's wait too, instead of
/// leaving the run waiting on a launcher whose output nobody reads.
struct FlagOnFailure {
    flag: Arc<AtomicBool>,
    failed: bool,
}

impl Drop for FlagOnFailure {
    fn drop(&mut self) {
        if self.failed {
            self.flag.store(true, Ordering::Release);
        }
    }
}

/// The stream a capture thread returned, or why it did not: a thread that panicked is an unread
/// stream, never an empty one.
fn finish_capture(reader: thread::JoinHandle<Captured>) -> Captured {
    reader.join().unwrap_or_else(|_| {
        Err(CaptureFailure::Unread {
            detail: "the capture thread panicked".to_owned(),
        })
    })
}

/// Reads `pipe` until EOF, or until `stop` is set and the pipe has been read dry or
/// `drain_limit` has passed, returning every byte read. Everything written before `stop` was set
/// is already in the pipe when the flag is seen, so the drain after it loses none of it.
///
/// More than `limit` bytes is [`CaptureFailure::OverLimit`] with only the bounded prefix, and
/// exactly `limit` is returned whole. A failed poll or read is [`CaptureFailure::Unread`]. Kani
/// never treats an over-limit prefix as complete output or proof evidence.
fn capture<R: Read + AsFd>(
    mut pipe: R,
    stop: &AtomicBool,
    limit: usize,
    drain_limit: Duration,
    mut poll_pipe: impl FnMut(&mut [PollFd<'_>], Option<&Timespec>) -> Result<usize, Errno>,
) -> Captured {
    let mut kept = Vec::new();
    let mut chunk = [0_u8; 64 * 1024];
    let interval = Timespec::try_from(LAUNCHER_POLL_INTERVAL).unwrap_or_default();
    let no_wait = Timespec::default();
    let mut drain_until: Option<Instant> = None;
    loop {
        if drain_until.is_none() && stop.load(Ordering::Acquire) {
            drain_until = Instant::now().checked_add(drain_limit);
            if drain_until.is_none() {
                break;
            }
        }
        if drain_until.is_some_and(|until| Instant::now() >= until) {
            break;
        }
        let wait = if drain_until.is_some() {
            &no_wait
        } else {
            &interval
        };
        let mut fds = [PollFd::new(&pipe, PollFlags::IN)];
        match poll_pipe(&mut fds, Some(wait)) {
            Ok(0) if drain_until.is_some() => break,
            Ok(0) => continue,
            Ok(_) => {}
            Err(Errno::INTR) => continue,
            Err(errno) => {
                return Err(CaptureFailure::Unread {
                    detail: format!("poll failed: {errno}"),
                })
            }
        }
        match pipe.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => {
                let remaining = limit.saturating_sub(kept.len());
                kept.extend(chunk.iter().take(read.min(remaining)));
                if read > remaining {
                    return Err(CaptureFailure::OverLimit { prefix: kept });
                }
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => {
                return Err(CaptureFailure::Unread {
                    detail: format!("read failed: {error}"),
                })
            }
        }
    }
    Ok(kept)
}

/// Observe the caller's retained first event, not a clock or a meter charge.
fn observed_cancellation(cancel: &Cancel) -> Option<KaniCancellationSource> {
    cancel.cause().map(|cause| match cause {
        CancelCause::Requested => KaniCancellationSource::Requested,
        CancelCause::Deadline => KaniCancellationSource::Deadline,
    })
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;
    use rustix::event::poll;
    use std::{fs, io::Write};

    use super::super::owners::LaunchTeardown;
    use super::{super::owners::SystemTeardown, *};
    use crate::core::test_support::discover_scratch;

    #[trace("FR-028-AC-2")]
    #[test]
    fn zero_wall_ceiling_cannot_accept_an_already_exited_launcher() {
        let deadline = Instant::now();
        let mut child = Command::new("sh")
            .args(["-c", "exit 0"])
            .process_group(0)
            .spawn()
            .unwrap();
        let pid = Pid::from_raw(i32::try_from(child.id()).unwrap()).unwrap();
        // Wait for a positive exit signal without reaping it. This models completion during
        // observation, after the zero ceiling elapsed; it needs no scheduling assumption.
        assert!(waitid(
            WaitId::Pid(pid),
            WaitIdOptions::EXITED | WaitIdOptions::NOWAIT,
        )
        .unwrap()
        .is_some());
        let conclusion = wait_until(
            &child,
            Some(deadline),
            &AtomicBool::new(false),
            &Cancel::new(),
        )
        .unwrap();
        SystemTeardown.signal_group(&mut child);
        child.wait().unwrap();
        assert!(
            matches!(conclusion, WaitConclusion::TimedOut),
            "an expired zero ceiling must time out, observed {conclusion:?}"
        );
    }

    #[trace("FR-028-AC-2")]
    #[test]
    fn expired_dispatch_deadline_starts_no_helper() {
        let directory = discover_scratch("expired-helper-dispatch");
        let marker = directory.join("helper-started");
        let mut command = Command::new("sh");
        command.args(["-c", "touch \"$1\"", "sh"]).arg(&marker);
        let outcome = run_monitored(
            command,
            Duration::from_secs(5),
            NonZeroUsize::MIN,
            Some(Instant::now()),
            &Cancel::new(),
            &LaunchOwners::real(),
        )
        .unwrap();
        assert!(matches!(outcome, LaunchOutcome::TimedOut));
        assert!(
            !marker.exists(),
            "expired preparation allowance must not spawn a helper"
        );
        fs::remove_dir_all(directory).unwrap();
    }

    /// The launcher's descendants are killed when the budget elapses, not only the immediate
    /// child, against a real process tree with a genuine grandchild. `sh -c '(sh -c "echo $$ >
    /// pidfile ; exec sleep 30") & wait'` runs an outer `sh` (the pid `run_launcher_with_timeout`
    /// itself sees) that forks a distinct grandchild shell, which records its pid and execs into
    /// `sleep 30`; the outer shell only waits. Running the grandchild through `exec` directly in
    /// the outer shell would record the direct child's own pid, which plain `child.kill()`
    /// already kills, and so would prove nothing about the group kill.
    ///
    /// The budget is tried up a ladder because a budget shorter than the time the OS takes to
    /// fork the grandchild ends the run before any grandchild exists; a rung whose grandchild
    /// never wrote its pidfile is inconclusive and the next rung is tried, so the test passes
    /// only on a recorded grandchild that is gone.
    #[trace("FR-017-AC-17", "TC-027")]
    #[cfg(target_os = "linux")]
    #[test]
    fn a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child() {
        let mut last_surviving_grandchild = None;
        for &budget in GRANDCHILD_KILL_TIMEOUT_LADDER.iter() {
            let directory = discover_scratch("launcher-timeout-grandchild");
            let pidfile = directory.join("pid");
            let mut command = Command::new("sh");
            // `sleep 30` (not the ladder's own scale) so the grandchild is still alive at
            // every rung's deadline: this test's timeout classification must come from the
            // budget elapsing, never from the grandchild finishing on its own.
            command.arg("-c").arg(format!(
                "(sh -c 'echo $$ > {} ; exec sleep 30') & wait",
                pidfile.display()
            ));
            let launch = run_launcher_with_timeout(command, budget).unwrap();
            assert!(
                matches!(launch, LaunchOutcome::TimedOut),
                "a run past its budget must classify as timed out"
            );

            // A missing, empty, or unparseable pidfile means the budget ended before the
            // grandchild recorded its pid: that rung proves nothing about the group kill (there
            // is no grandchild to have been killed), so it is inconclusive and the ladder moves
            // to the next, larger budget. Only a recorded pid that is then gone counts as a kill.
            let grandchild_pid: Option<i32> = fs::read_to_string(&pidfile)
                .ok()
                .and_then(|contents| contents.trim().parse().ok());
            let grandchild_survived = grandchild_pid.is_some_and(|pid| !process_gone_within(pid));
            let killed = grandchild_pid.is_some() && !grandchild_survived;

            if grandchild_survived {
                let pid = grandchild_pid.expect("survived implies a parsed pid");
                // This rung lost the race: the grandchild is still alive past the budget.
                // Kill it directly so a losing rung never leaves an orphaned `sleep 30`
                // behind, whether this is a retry or the last rung about to fail.
                let _ = Command::new("kill")
                    .args(["-KILL", &pid.to_string()])
                    .status();
                last_surviving_grandchild = Some(pid);
            } else {
                last_surviving_grandchild = None;
            }
            let _ = fs::remove_dir_all(directory);

            if killed {
                return;
            }
            // Survived this rung, or never started one: indistinguishable, from here, between
            // "genuinely not killed" and "forked after this rung's snapshot" — retry at the
            // next, larger budget rather than assert either reading.
        }

        panic!(
            "the timed-out run's grandchild ({last_surviving_grandchild:?}) was still present \
             in `/proc`, or never recorded its pid, even at the largest budget in \
             GRANDCHILD_KILL_TIMEOUT_LADDER ({GRANDCHILD_KILL_TIMEOUT_LADDER:?}); \
             kill_process_tree must reach every member of the launcher's process group"
        );
    }

    /// Whether `pid` is dead, waiting up to [`GRANDCHILD_REAP_WAIT`] for it to become so.
    ///
    /// Two facts about a SIGKILLed process make a bare `/proc/<pid>` existence check wrong.
    /// The kill is asynchronous: `kill_process_tree` has returned once the signal is sent, not
    /// once the target has stopped running, so the process can still read as running for a few
    /// milliseconds. And the grandchild is reparented to init when its parent dies, which
    /// reaps it in its own time (measured here: PID 1 left killed grandchildren as zombies
    /// for seconds), so `/proc/<pid>` outlives a process that is already dead. Dead means
    /// absent or in state `Z`/`X`; anything else after the wait is a survivor.
    #[cfg(target_os = "linux")]
    fn process_gone_within(pid: i32) -> bool {
        let deadline = Instant::now() + GRANDCHILD_REAP_WAIT;
        loop {
            let state = fs::read_to_string(format!("/proc/{pid}/stat"))
                .ok()
                .and_then(|stat| {
                    stat.rsplit_once(')')
                        .and_then(|(_, rest)| rest.trim_start().chars().next())
                });
            if matches!(state, None | Some('Z') | Some('X')) {
                return true;
            }
            if Instant::now() >= deadline {
                return false;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// How long a delivered SIGKILL is given to take effect before its target counts as a
    /// survivor. Far longer than signal delivery takes; a process still running after it was
    /// not killed.
    #[cfg(target_os = "linux")]
    const GRANDCHILD_REAP_WAIT: Duration = Duration::from_secs(2);

    /// Successive budgets [`a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child`]
    /// tries, smallest first: the common case (an unloaded or lightly loaded run) settles on
    /// the first, cheap rung, and only a run actually contending for CPU climbs further. The
    /// top rung is a generous safety margin chosen as a judgment call, not a number backed by
    /// direct local measurement — this fix's own local validation environment could not
    /// reproduce the original flake at all — sized so that exhausting it is a real finding
    /// about `kill_process_tree`, not an unlucky scheduling instant.
    #[cfg(target_os = "linux")]
    const GRANDCHILD_KILL_TIMEOUT_LADDER: [Duration; 5] = [
        Duration::from_millis(200),
        Duration::from_millis(500),
        Duration::from_secs(1),
        Duration::from_secs(2),
        Duration::from_secs(5),
    ];

    /// A process that has left the launcher's process group keeps its inherited copy of the
    /// pipes' write end open and is out of reach of the group kill: a `setsid sleep 45`, started
    /// in a subshell and left behind when the outer shell `exec`s into `sleep 45` (the direct
    /// child), is such a process. Waiting for the capture threads to see EOF would block until
    /// that orphan's 45 s sleep ended on its own, which is unbounded; the call must return after
    /// the drain limit, long before that (asserted at 20 s, wide enough for a loaded host and
    /// under the 45 s). A capture that ignored the stop flag would fail this at about 45 s.
    ///
    /// The launcher waits for the orphan to record its pid before it `exec`s, and a rung whose
    /// budget ended before that is inconclusive (the orphan may still have been in the group),
    /// so the budget is tried up [`GRANDCHILD_KILL_TIMEOUT_LADDER`]. The orphan is killed by pid
    /// before the assertions run, so no `sleep 45` outlives the test, passing or failing.
    #[trace("FR-017-AC-16", "TC-027")]
    #[cfg(target_os = "linux")]
    #[test]
    fn a_process_orphaned_just_before_the_kill_does_not_block_this_calls_own_return() {
        for &budget in GRANDCHILD_KILL_TIMEOUT_LADDER.iter() {
            let directory = discover_scratch("launcher-timeout-escaped-orphan");
            let pidfile = directory.join("pid");
            let mut command = Command::new("sh");
            command.arg("-c").arg(format!(
                "( setsid sh -c 'echo $$ > {pid}; exec sleep 45' & ); \
                 until [ -s {pid} ]; do sleep 0.02; done; exec sleep 45",
                pid = pidfile.display()
            ));
            let started = Instant::now();
            let outcome = run_launcher_with_timeout(command, budget).unwrap();
            let elapsed = started.elapsed();
            let orphan_started = kill_recorded_process(&pidfile);
            let _ = fs::remove_dir_all(directory);

            if !orphan_started {
                // The budget ended before the orphan was out of the group: it proves nothing.
                continue;
            }
            assert!(
                matches!(outcome, LaunchOutcome::TimedOut),
                "a run past its budget must classify as timed out"
            );
            assert!(
                elapsed < Duration::from_secs(20),
                "a {budget:?} budget must not take anywhere near the escaped process's own 45s \
                 sleep: took {elapsed:?}"
            );
            return;
        }
        panic!(
            "the escaped process never recorded its pid within the largest budget in \
             GRANDCHILD_KILL_TIMEOUT_LADDER ({GRANDCHILD_KILL_TIMEOUT_LADDER:?})"
        );
    }

    /// Kills the process whose pid `pidfile` records, waiting up to [`GRANDCHILD_REAP_WAIT`] for
    /// the record to appear, and reports whether there was one.
    #[cfg(target_os = "linux")]
    fn kill_recorded_process(pidfile: &std::path::Path) -> bool {
        let deadline = Instant::now() + GRANDCHILD_REAP_WAIT;
        loop {
            let recorded = fs::read_to_string(pidfile)
                .ok()
                .and_then(|contents| contents.trim().parse::<i32>().ok());
            if let Some(pid) = recorded {
                let _ = Command::new("kill")
                    .args(["-KILL", &pid.to_string()])
                    .status();
                return true;
            }
            if Instant::now() >= deadline {
                return false;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Regression coverage for the polling/draining plumbing `run_launcher_with_timeout` added:
    /// a process that exits within its budget still reports its real exit status and combined
    /// output, unchanged from what `Command::output()` used to hand back directly. No criterion
    /// traces this specifically — the Kani lane's real `cargo-kani` runs
    /// (`tests/kani_obligations.rs`) already exercise this completed path end to end; this is
    /// only faster, hermetic coverage of the same plumbing.
    #[test]
    fn a_run_finishing_within_its_budget_reports_its_own_exit_status_and_output() {
        let mut command = Command::new("sh");
        command.arg("-c").arg("printf out; printf err 1>&2; exit 3");
        let outcome = run_launcher_with_timeout(command, Duration::from_secs(5)).unwrap();
        match outcome {
            LaunchOutcome::Completed {
                exited_successfully,
                exit_code,
                streams,
            } => {
                assert!(!exited_successfully);
                assert_eq!(exit_code, Some(3));
                assert_eq!(streams.stdout, b"out");
                assert_eq!(streams.stderr, b"err");
                assert_eq!(streams.merged_text(), "out\nerr");
            }
            other => panic!("a fast process must complete, not end as {other:?}"),
        }
    }

    /// The merged text classification reads is built from the kept streams alone, byte for byte
    /// as the launch used to join them: lossy stdout, one line feed, lossy stderr.
    ///
    #[trace("FR-017-AC-36", "TC-043")]
    #[test]
    fn the_merged_text_is_the_lossy_streams_joined_by_one_line_feed() {
        let mut command = Command::new("sh");
        command
            .arg("-c")
            .arg("printf 'a\\377b'; printf 'c\\n' 1>&2; exit 1");
        let LaunchOutcome::Completed { streams, .. } =
            run_launcher_with_timeout(command, Duration::from_secs(5)).unwrap()
        else {
            panic!("a fast process completes");
        };
        assert_eq!(streams.stdout, b"a\xffb");
        assert_eq!(streams.stderr, b"c\n");
        assert_eq!(streams.merged_text(), "a\u{fffd}b\nc\n");
    }

    /// A stream of more than the limit is refused whole, never truncated to a tail and returned;
    /// a stream of exactly the limit is returned whole.
    #[trace("FR-017-AC-14", "TC-043")]
    #[test]
    fn tc_043_a_capture_over_its_limit_is_refused_and_one_at_its_limit_is_whole() {
        let limit = 4096;
        for (written, expected) in [(limit, true), (limit + 1, false)] {
            let (reader, mut writer) = io::pipe().unwrap();
            let producer = thread::spawn(move || {
                // The write end is dropped on return, so the reader sees the end of the stream.
                writer.write_all(&vec![b'x'; written]).unwrap();
            });
            let captured = capture(
                reader,
                &AtomicBool::new(false),
                limit,
                STOP_DRAIN_LIMIT,
                poll,
            );
            producer.join().unwrap();
            match (expected, captured) {
                (true, Ok(kept)) => assert_eq!(kept.len(), limit, "exactly the limit is whole"),
                (false, Err(CaptureFailure::OverLimit { prefix })) => {
                    assert_eq!(prefix.len(), limit);
                }
                (_, other) => panic!("{written} bytes against a limit of {limit}: {other:?}"),
            }
        }
    }

    /// Runs `capture` on its own thread and returns what it kept, or `None` if it had not
    /// returned within `within` or failed, so a capture that never stops fails the test instead
    /// of hanging it.
    fn capture_within(
        reader: io::PipeReader,
        stop: &Arc<AtomicBool>,
        drain_limit: Duration,
        within: Duration,
    ) -> Option<Vec<u8>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let stop = Arc::clone(stop);
        thread::spawn(move || {
            // Unbounded: these tests are about when the capture stops, and a straggler writing
            // flat out would otherwise cross the limit first.
            let _ = sender.send(capture(reader, &stop, usize::MAX, drain_limit, poll).ok());
        });
        receiver.recv_timeout(within).ok().flatten()
    }

    /// Bytes already in the pipe when the stop flag is seen are returned even though a write end
    /// is still open.
    #[trace("FR-017-AC-16", "TC-027")]
    #[test]
    fn a_capture_thread_told_to_stop_returns_what_is_already_in_the_pipe() {
        let (reader, mut writer) = io::pipe().unwrap();
        writer.write_all(b"written before stop").unwrap();
        let stop = Arc::new(AtomicBool::new(true));
        let kept = capture_within(reader, &stop, STOP_DRAIN_LIMIT, Duration::from_secs(10));
        assert_eq!(kept.as_deref(), Some(&b"written before stop"[..]));
        drop(writer);
    }

    /// A capture thread that is idle in its poll, with a write end still open and nothing more
    /// coming, stops once the flag is set later.
    #[trace("FR-017-AC-16", "TC-027")]
    #[test]
    fn a_capture_thread_blocked_on_an_open_idle_pipe_stops_when_the_flag_is_set() {
        let (reader, writer) = io::pipe().unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        let setter = {
            let stop = Arc::clone(&stop);
            thread::spawn(move || {
                thread::sleep(3 * LAUNCHER_POLL_INTERVAL);
                stop.store(true, Ordering::Release);
            })
        };
        let kept = capture_within(reader, &stop, STOP_DRAIN_LIMIT, Duration::from_secs(10));
        setter.join().unwrap();
        assert_eq!(kept, Some(Vec::new()), "the capture must stop on the flag");
        drop(writer);
    }

    /// The drain after the stop flag ends at its limit even when bytes are still waiting: with a
    /// zero limit nothing is read.
    #[trace("FR-017-AC-16", "TC-027")]
    #[test]
    fn a_capture_thread_stops_reading_when_its_drain_limit_has_passed() {
        let (reader, mut writer) = io::pipe().unwrap();
        writer.write_all(b"waiting in the pipe").unwrap();
        let stop = Arc::new(AtomicBool::new(true));
        let kept = capture_within(reader, &stop, Duration::ZERO, Duration::from_secs(10));
        assert_eq!(kept, Some(Vec::new()), "read past a zero drain limit");
        drop(writer);
    }

    /// A straggler that writes without pause cannot hold the capture thread past the drain
    /// limit after the flag is set.
    #[trace("FR-017-AC-16", "TC-027")]
    #[test]
    fn a_capture_thread_stops_within_the_drain_limit_while_a_straggler_keeps_writing() {
        let (reader, mut writer) = io::pipe().unwrap();
        let running = Arc::new(AtomicBool::new(true));
        let straggler = {
            let running = Arc::clone(&running);
            thread::spawn(move || {
                while running.load(Ordering::Acquire) {
                    if writer.write_all(&[b'x'; 4096]).is_err() {
                        break;
                    }
                }
            })
        };
        let stop = Arc::new(AtomicBool::new(false));
        let setter = {
            let stop = Arc::clone(&stop);
            thread::spawn(move || {
                thread::sleep(3 * LAUNCHER_POLL_INTERVAL);
                stop.store(true, Ordering::Release);
            })
        };
        let kept = capture_within(
            reader,
            &stop,
            STOP_DRAIN_LIMIT,
            STOP_DRAIN_LIMIT + Duration::from_secs(10),
        );
        running.store(false, Ordering::Release);
        setter.join().unwrap();
        straggler.join().unwrap();
        assert!(kept.is_some(), "the drain limit must end the capture");
    }

    /// A launcher that prints `bytes` bytes to `stream`, then `then` (a shell command).
    fn printer(stream: CaptureStream, bytes: usize, then: &str) -> Command {
        let redirect = match stream {
            CaptureStream::Stdout => "",
            CaptureStream::Stderr => " 1>&2",
        };
        let mut command = Command::new("sh");
        command
            .arg("-c")
            .arg(format!("head -c {bytes} /dev/zero{redirect}; {then}"));
        command
    }

    /// A launcher whose stdout, then whose stderr, carries more than the limit is stopped and
    /// refused naming the stream, the limit and the harness count; one that prints exactly the
    /// limit completes with its real exit status and the whole stream. Nothing is truncated and
    /// then classified.
    #[trace("FR-017-AC-14", "TC-043")]
    #[test]
    fn tc_043_a_launcher_stream_over_the_limit_is_refused_and_one_at_the_limit_completes() {
        for stream in [CaptureStream::Stdout, CaptureStream::Stderr] {
            let outcome = run_launcher_with_timeout(
                printer(stream, CAPTURE_LIMIT + 1, "exit 0"),
                Duration::from_secs(60),
            )
            .unwrap();
            assert!(
                matches!(
                    outcome,
                    LaunchOutcome::OutputOverLimit { stream: named, limit, harnesses: 1 }
                        if named == stream && limit == CAPTURE_LIMIT
                ),
                "{stream}: one byte over the limit is refused"
            );
            let LaunchOutcome::Completed {
                exit_code, streams, ..
            } = run_launcher_with_timeout(
                printer(stream, CAPTURE_LIMIT, "exit 3"),
                Duration::from_secs(60),
            )
            .unwrap()
            else {
                panic!("{stream}: exactly the limit completes");
            };
            assert_eq!(exit_code, Some(3));
            assert_eq!(
                streams.merged_text().len(),
                CAPTURE_LIMIT + 1,
                "{stream}: the whole stream"
            );
        }
    }

    /// A batch process may carry the limit for each member and no more: the limit is multiplied
    /// by the harness count, and the refusal names that count.
    #[trace("FR-017-AC-14", "TC-043")]
    #[test]
    fn tc_043_a_batch_stream_is_bounded_by_the_limit_times_the_member_count() {
        let members = NonZeroUsize::new(3).unwrap();
        let outcome = run_launcher(
            printer(CaptureStream::Stdout, 3 * CAPTURE_LIMIT + 1, "exit 0"),
            Duration::from_secs(60),
            members,
        )
        .unwrap();
        assert!(
            matches!(
                outcome,
                LaunchOutcome::OutputOverLimit { stream: CaptureStream::Stdout, limit, harnesses: 3 }
                    if limit == 3 * CAPTURE_LIMIT
            ),
            "three members' limit is three times one"
        );
        let outcome = run_launcher(
            printer(CaptureStream::Stderr, 3 * CAPTURE_LIMIT, "exit 0"),
            Duration::from_secs(60),
            members,
        )
        .unwrap();
        assert!(
            matches!(outcome, LaunchOutcome::Completed { .. }),
            "exactly three members' limit completes"
        );
    }

    /// An over-limit launcher that would otherwise run on is stopped and its whole process group
    /// is killed, not only refused: the call returns while the launcher is still short of its
    /// marker (it is killed at once, not left to run on and found over the limit afterwards), and
    /// its grandchild is gone.
    #[trace("FR-017-AC-14", "TC-043")]
    #[cfg(target_os = "linux")]
    #[test]
    fn tc_043_an_over_limit_run_is_stopped_and_kills_the_launcher_group() {
        let directory = discover_scratch("over-limit-kill");
        let pidfile = directory.join("pid");
        let marker = directory.join("ran-on");
        // The launcher shell forks a grandchild (recording its pid), prints past the limit, sleeps
        // ten seconds, writes its marker and waits for the grandchild. A run that is stopped when
        // the stream goes over never reaches the marker; one that waits for the launcher to end
        // on its own writes it, and only a kill of the whole group ends the grandchild. The
        // launcher is killed within milliseconds of the stream going over, so the ten seconds are
        // headroom for a loaded host, not time the test spends.
        let mut command = Command::new("sh");
        command.arg("-c").arg(format!(
            "sleep 45 & echo $! > {}; head -c {} /dev/zero; sleep 10; touch {}; wait",
            pidfile.display(),
            CAPTURE_LIMIT + 1,
            marker.display()
        ));
        let outcome = run_launcher_with_timeout(command, Duration::from_secs(120)).unwrap();
        assert!(matches!(outcome, LaunchOutcome::OutputOverLimit { .. }));
        assert!(
            !marker.exists(),
            "the launcher ran on after its stream went over the limit"
        );
        let grandchild: i32 = fs::read_to_string(&pidfile)
            .unwrap()
            .trim()
            .parse()
            .expect("the launcher recorded its grandchild's pid before it printed");
        assert!(
            process_gone_within(grandchild),
            "the over-limit launcher's grandchild {grandchild} is still running"
        );
        let _ = fs::remove_dir_all(directory);
    }

    /// A reader that panics when it is read, over a pipe with bytes waiting so that its poll says
    /// readable.
    struct PanickingPipe(io::PipeReader);

    impl Read for PanickingPipe {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            panic!("a capture thread that panics");
        }
    }

    impl AsFd for PanickingPipe {
        fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
            self.0.as_fd()
        }
    }

    /// A reader whose read fails, over a pipe with bytes waiting.
    struct FailingPipe(io::PipeReader);

    impl Read for FailingPipe {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("a pipe that cannot be read"))
        }
    }

    impl AsFd for FailingPipe {
        fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
            self.0.as_fd()
        }
    }

    /// A poll that fails.
    fn failing_poll(_: &mut [PollFd<'_>], _: Option<&Timespec>) -> Result<usize, Errno> {
        Err(Errno::IO)
    }

    /// A capture thread that panics, a read that errs and a poll that errs each refuse the stream
    /// as unread: none of them returns the empty text, or what had been read so far, as the
    /// stream.
    #[trace("FR-017-AC-25", "TC-043")]
    #[test]
    fn tc_043_a_capture_that_panics_or_whose_poll_or_read_errs_is_unread_not_empty() {
        let waiting = || {
            let (reader, mut writer) = io::pipe().unwrap();
            writer.write_all(b"bytes waiting").unwrap();
            // Keep the write end open: the stream has not ended.
            (reader, writer)
        };
        let flags = CaptureFlags {
            stop: Arc::new(AtomicBool::new(false)),
            failed: Arc::new(AtomicBool::new(false)),
        };

        let (reader, _writer) = waiting();
        let panicked = finish_capture(spawn_capture(
            PanickingPipe(reader),
            &flags,
            CAPTURE_LIMIT,
            Arc::clone(&LaunchOwners::real().stdout),
        ));
        assert!(
            matches!(&panicked, Err(CaptureFailure::Unread { detail }) if detail.contains("panicked")),
            "{panicked:?}"
        );
        assert!(
            flags.failed.load(Ordering::Acquire),
            "a failed capture stops the run's wait"
        );

        let (reader, _writer) = waiting();
        let read_failed = capture(
            FailingPipe(reader),
            &AtomicBool::new(false),
            CAPTURE_LIMIT,
            STOP_DRAIN_LIMIT,
            poll,
        );
        assert!(
            matches!(&read_failed, Err(CaptureFailure::Unread { detail }) if detail.contains("read failed")),
            "{read_failed:?}"
        );

        let (reader, _writer) = waiting();
        let poll_failed = capture(
            reader,
            &AtomicBool::new(false),
            CAPTURE_LIMIT,
            STOP_DRAIN_LIMIT,
            failing_poll,
        );
        assert!(
            matches!(&poll_failed, Err(CaptureFailure::Unread { detail }) if detail.contains("poll failed")),
            "{poll_failed:?}"
        );
    }

    /// An unread stream refuses the run with the stream named; it never becomes text.
    #[trace("FR-017-AC-25", "TC-043")]
    #[test]
    fn tc_043_an_unread_stream_names_the_stream_in_the_launch_outcome() {
        let unread = || CaptureFailure::Unread {
            detail: "gone".to_owned(),
        };
        let one = NonZeroUsize::MIN;
        assert!(matches!(
            stream_bytes(CaptureStream::Stderr, Err(unread()), 5, one),
            Err(LaunchOutcome::OutputUnread { stream: CaptureStream::Stderr, detail })
                if detail == "gone"
        ));
        assert!(matches!(
            stream_bytes(
                CaptureStream::Stdout,
                Err(CaptureFailure::OverLimit { prefix: vec![] }),
                5,
                one
            ),
            Err(LaunchOutcome::OutputOverLimit {
                stream: CaptureStream::Stdout,
                limit: 5,
                harnesses: 1
            })
        ));
        assert!(matches!(
            stream_bytes(CaptureStream::Stdout, Ok(b"ok".to_vec()), 5, one),
            Ok(bytes) if bytes == b"ok"
        ));
    }

    /// A launcher that exits on its own, leaving a real grandchild in its process group, has
    /// that grandchild killed by the time the run returns, as it does on a timeout. `sleep 45 &`
    /// is forked by the launcher shell, which writes its pid and exits without waiting.
    #[trace("FR-017-AC-24", "TC-043")]
    #[cfg(target_os = "linux")]
    #[test]
    fn tc_043_a_launcher_that_exits_on_its_own_has_its_grandchild_killed() {
        let directory = discover_scratch("exit-grandchild");
        let pidfile = directory.join("pid");
        let mut command = Command::new("sh");
        command
            .arg("-c")
            .arg(format!("sleep 45 & echo $! > {}", pidfile.display()));
        let outcome = run_launcher_with_timeout(command, Duration::from_secs(60)).unwrap();
        assert!(
            matches!(
                outcome,
                LaunchOutcome::Completed {
                    exit_code: Some(0),
                    ..
                }
            ),
            "the launcher exited on its own, within its budget"
        );
        let grandchild: i32 = fs::read_to_string(&pidfile)
            .unwrap()
            .trim()
            .parse()
            .expect("the launcher recorded its grandchild's pid before it exited");
        assert!(
            process_gone_within(grandchild),
            "the grandchild {grandchild} outlived the run that left it in the launcher's group"
        );
        let _ = fs::remove_dir_all(directory);
    }

    /// A timeout too large to add to the current instant means no deadline, not a panic.
    #[trace("FR-017-AC-15", "TC-027")]
    #[test]
    fn a_timeout_of_duration_max_never_elapses_and_does_not_panic() {
        let mut command = Command::new("sh");
        command.arg("-c").arg("printf done");
        let outcome = run_launcher_with_timeout(command, Duration::MAX).unwrap();
        assert!(matches!(
            outcome,
            LaunchOutcome::Completed { exit_code: Some(0), ref streams, .. } if streams.merged_text().contains("done")
        ));
    }

    /// The return allowance past a wall-clock timeout (FR-017-AC-66): the capture drain limit
    /// [`STOP_DRAIN_LIMIT`] (100 ms), the poll interval [`LAUNCHER_POLL_INTERVAL`] (20 ms) and
    /// scheduling slack.
    const RETURN_ALLOWANCE: Duration = Duration::from_secs(1);

    /// A stand-in that prints its own process id and process group id.
    fn identity_printer() -> Command {
        let mut command = Command::new("sh");
        command
            .arg("-c")
            .arg("echo $$; ps -o pgid= -p $$ | tr -d ' '");
        command
    }

    fn printed_ids(stdout: &[u8]) -> (i32, i32) {
        let text = String::from_utf8_lossy(stdout);
        let mut lines = text.lines();
        let pid = lines.next().unwrap().trim().parse().unwrap();
        let pgid = lines.next().unwrap().trim().parse().unwrap();
        (pid, pgid)
    }

    /// The launcher leads a new process group by a call that exists on Linux and macOS: its
    /// group id is its own process id and differs from the caller's. A stand-in left in the
    /// caller's group is the control and reports the caller's group id.
    #[trace("FR-017-AC-65", "TC-043")]
    #[test]
    fn tc_043_the_launcher_leads_its_own_process_group() {
        let caller_group = rustix::process::getpgrp().as_raw_nonzero().get();
        let LaunchOutcome::Completed { streams, .. } =
            run_launcher_with_timeout(identity_printer(), Duration::from_secs(10)).unwrap()
        else {
            panic!("the stand-in exits on its own");
        };
        let (pid, pgid) = printed_ids(&streams.stdout);
        assert_eq!(pgid, pid, "the launcher leads its own group");
        assert_ne!(pgid, caller_group, "the group is not the caller's");

        let control = identity_printer().output().unwrap();
        let (control_pid, control_pgid) = printed_ids(&control.stdout);
        assert_eq!(
            control_pgid, caller_group,
            "the control stays in the caller's group"
        );
        assert_ne!(control_pgid, control_pid);
    }

    /// Heartbeat scripts: the descendant is a child of the launcher and the grandchild a child of
    /// the descendant, all in the launcher's group, each appending a byte every 50 ms.
    fn write_heartbeat_scripts(directory: &std::path::Path) -> (Command, [std::path::PathBuf; 2]) {
        let grandchild = directory.join("grandchild.sh");
        let descendant = directory.join("descendant.sh");
        fs::write(
            &grandchild,
            "while :; do printf x >> \"$1\"; sleep 0.05; done\n",
        )
        .unwrap();
        fs::write(
            &descendant,
            format!(
                "sh '{}' \"$2\" &\nwhile :; do printf x >> \"$1\"; sleep 0.05; done\n",
                grandchild.display()
            ),
        )
        .unwrap();
        let heartbeats = [
            directory.join("descendant.beat"),
            directory.join("grandchild.beat"),
        ];
        let mut command = Command::new("sh");
        command
            .arg("-c")
            .arg("sh \"$0\" \"$1\" \"$2\" & wait")
            .arg(&descendant)
            .args(&heartbeats);
        (command, heartbeats)
    }

    fn size(path: &std::path::Path) -> u64 {
        fs::metadata(path).map_or(0, |metadata| metadata.len())
    }

    /// On a timeout the launcher, its descendant and that descendant's grandchild are all
    /// killed, and the run returns within the timeout plus [`RETURN_ALLOWANCE`]; both heartbeat
    /// files stop growing.
    #[trace("FR-017-AC-66", "TC-043")]
    #[test]
    fn tc_043_a_timeout_kills_the_whole_group_and_returns_within_the_allowance() {
        let directory = discover_scratch("group-kill-heartbeat");
        let (command, heartbeats) = write_heartbeat_scripts(&directory);
        let timeout = Duration::from_secs(1);
        let started = Instant::now();
        let outcome = run_launcher_with_timeout(command, timeout).unwrap();
        let elapsed = started.elapsed();
        assert!(matches!(outcome, LaunchOutcome::TimedOut), "{outcome:?}");
        assert!(
            elapsed < timeout + RETURN_ALLOWANCE,
            "returned after {elapsed:?}"
        );
        let at_return = heartbeats.each_ref().map(|path| size(path));
        assert!(
            at_return.iter().all(|bytes| *bytes > 0),
            "both processes were alive: {at_return:?}"
        );
        thread::sleep(Duration::from_millis(250));
        let later = heartbeats.each_ref().map(|path| size(path));
        assert_eq!(
            later, at_return,
            "both heartbeats stopped with the group kill"
        );
        fs::remove_dir_all(directory).unwrap();
    }

    /// Control for the heartbeat reading: killing only the launcher's own process id leaves the
    /// descendants running and both heartbeat files growing, so the group kill above is what
    /// stops them.
    #[trace("FR-017-AC-66", "TC-043")]
    #[test]
    fn tc_043_killing_only_the_launcher_leaves_the_heartbeats_growing() {
        let directory = discover_scratch("launcher-only-kill-heartbeat");
        let (mut command, heartbeats) = write_heartbeat_scripts(&directory);
        command.process_group(0);
        let mut child = command.spawn().unwrap();
        thread::sleep(Duration::from_millis(300));
        child.kill().unwrap();
        child.wait().unwrap();
        let after_kill = heartbeats.each_ref().map(|path| size(path));
        thread::sleep(Duration::from_millis(300));
        let later = heartbeats.each_ref().map(|path| size(path));
        let group = Pid::from_raw(i32::try_from(child.id()).unwrap()).unwrap();
        let _ = rustix::process::kill_process_group(group, rustix::process::Signal::KILL);
        assert!(
            later
                .iter()
                .zip(after_kill)
                .all(|(now, before)| *now > before),
            "the descendants outlive a launcher-only kill: {after_kill:?} then {later:?}"
        );
        fs::remove_dir_all(directory).unwrap();
    }

    /// A descendant in the group that holds the output pipe open for five seconds does not hold
    /// the return: the group kill closes it and the call returns inside the allowance.
    #[trace("FR-017-AC-66", "TC-043")]
    #[test]
    fn tc_043_a_descendant_holding_the_output_pipe_does_not_delay_the_return() {
        let mut command = Command::new("sh");
        command.arg("-c").arg("sleep 5 & wait");
        let timeout = Duration::from_millis(500);
        let started = Instant::now();
        let outcome = run_launcher_with_timeout(command, timeout).unwrap();
        let elapsed = started.elapsed();
        assert!(matches!(outcome, LaunchOutcome::TimedOut), "{outcome:?}");
        assert!(
            elapsed < timeout + RETURN_ALLOWANCE,
            "returned after {elapsed:?}"
        );
    }

    /// Control: a descendant that ends its own heartbeat by exiting before the timeout stops
    /// writing at its own exit, so the files are read after a real stop.
    #[trace("FR-017-AC-66", "TC-043")]
    #[test]
    fn tc_043_a_descendant_that_exits_before_the_timeout_ends_its_own_heartbeat() {
        let directory = discover_scratch("self-ending-heartbeat");
        let beat = directory.join("self.beat");
        let mut command = Command::new("sh");
        command
            .arg("-c")
            .arg("( for _ in 1 2 3; do printf x >> \"$0\"; sleep 0.05; done ) & sleep 30")
            .arg(&beat);
        let outcome = run_launcher_with_timeout(command, Duration::from_secs(1)).unwrap();
        assert!(matches!(outcome, LaunchOutcome::TimedOut), "{outcome:?}");
        assert_eq!(size(&beat), 3);
        fs::remove_dir_all(directory).unwrap();
    }

    /// Records the order of the group signal and the reap around the real operations. At the
    /// signal it probes the launcher's own pid, which a zombie still reserves and a reaped child
    /// does not, so the record reflects the process table and not only the call order.
    #[derive(Debug, Default)]
    struct RecordingTeardown {
        events: std::sync::Mutex<Vec<&'static str>>,
    }

    impl LaunchTeardown for RecordingTeardown {
        fn signal_group(&self, child: &mut Child) {
            let pid = rustix::process::Pid::from_raw(i32::try_from(child.id()).unwrap()).unwrap();
            let reserved = rustix::process::test_kill_process(pid).is_ok();
            self.events.lock().unwrap().push(if reserved {
                "signal_group"
            } else {
                "signal_group_after_reap"
            });
            SystemTeardown.signal_group(child);
        }

        fn reap(&self, child: &mut Child) -> io::Result<std::process::ExitStatus> {
            self.events.lock().unwrap().push("reap");
            SystemTeardown.reap(child)
        }
    }

    fn owners_recording(recorder: &Arc<RecordingTeardown>) -> LaunchOwners {
        let real = LaunchOwners::real();
        LaunchOwners {
            stdout: Arc::clone(&real.stdout),
            stderr: Arc::clone(&real.stderr),
            teardown: Arc::clone(recorder) as Arc<dyn LaunchTeardown>,
            peak: Arc::clone(&real.peak),
        }
    }

    /// On a timeout, and on a launcher that exits on its own, the group is signalled before the
    /// launcher is reaped (FR-017-AC-68). The recorder probes the process table at the signal,
    /// so a reap-first order records `signal_group_after_reap` and fails this test.
    #[trace("FR-017-AC-68", "TC-043")]
    #[test]
    fn tc_043_the_group_is_signalled_before_the_launcher_is_reaped() {
        for (script, timeout) in [
            ("sleep 30", Duration::from_millis(300)),
            ("exit 0", Duration::from_secs(10)),
        ] {
            let recorder = Arc::new(RecordingTeardown::default());
            let mut command = Command::new("sh");
            command.arg("-c").arg(script);
            run_monitored(
                command,
                timeout,
                NonZeroUsize::MIN,
                None,
                &Cancel::new(),
                &owners_recording(&recorder),
            )
            .unwrap();
            assert_eq!(
                *recorder.events.lock().unwrap(),
                ["signal_group", "reap"],
                "script {script:?}"
            );
        }

        // The control: the same recorder driven reap-first, as a mutant would, reports that the
        // launcher was already gone when the group was signalled.
        let recorder = RecordingTeardown::default();
        let mut child = Command::new("sh")
            .args(["-c", "exit 0"])
            .process_group(0)
            .spawn()
            .unwrap();
        recorder.reap(&mut child).unwrap();
        recorder.signal_group(&mut child);
        assert_eq!(
            *recorder.events.lock().unwrap(),
            ["reap", "signal_group_after_reap"]
        );
    }
}

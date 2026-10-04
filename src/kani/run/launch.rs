//! Launching the Kani launcher process: spawn, bounded capture, the wall-clock budget and the
//! process-group kill (FR-017).

use std::{
    fmt, io,
    io::Read,
    num::NonZeroUsize,
    os::{fd::AsFd, unix::process::CommandExt},
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use rustix::{
    event::{poll, PollFd, PollFlags},
    io::Errno,
    process::{kill_process_group, waitid, Pid, Signal, WaitId, WaitIdOptions},
    time::Timespec,
};

/// How the launcher's run within its caller-declared budget
/// ([`KaniExecutionRequest::timeout`](super::execute::KaniExecutionRequest::timeout))
/// concluded.
#[derive(Debug)]
#[non_exhaustive]
pub enum LaunchOutcome {
    /// The process exited on its own within the budget.
    Completed {
        /// `ExitStatus::success()`.
        exited_successfully: bool,
        /// `ExitStatus::code()`.
        exit_code: Option<i32>,
        /// Combined stdout and stderr, newline-joined, matching `classify_kani_run`'s input shape.
        text: String,
    },
    /// The budget elapsed before the process exited. The launcher's whole process group has been
    /// killed and the launcher reaped; a descendant that left the group is not killed, and this
    /// call does not wait for it.
    TimedOut,
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
    /// More than the limit was read.
    OverLimit,
    /// The pipe could not be polled or read, or its thread panicked.
    Unread { detail: String },
}

/// What one capture thread returns: the whole stream, or why it could not.
type Captured = Result<Vec<u8>, CaptureFailure>;

/// The `poll(2)` a capture thread waits with. A parameter so that a failed poll, which no real pipe
/// produces on demand, is exercised by a test.
type PollFn = fn(&mut [PollFd<'_>], Option<&Timespec>) -> Result<usize, Errno>;

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
/// `kill_process_tree`. The group is no longer the terminal's foreground group, so a Ctrl-C
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
pub fn run_launcher_with_timeout(command: Command, timeout: Duration) -> io::Result<LaunchOutcome> {
    run_launcher(command, timeout, NonZeroUsize::MIN)
}

/// [`run_launcher_with_timeout`] for a process that runs `harnesses` harnesses: each stream may
/// carry [`CAPTURE_LIMIT`] bytes for every one.
pub(super) fn run_launcher(
    mut command: Command,
    timeout: Duration,
    harnesses: NonZeroUsize,
) -> io::Result<LaunchOutcome> {
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0);
    let mut child = command.spawn()?;
    let (Some(stdout), Some(stderr)) = (child.stdout.take(), child.stderr.take()) else {
        let _ = child.kill();
        let _ = child.wait();
        return Err(io::Error::other("a piped standard stream was not captured"));
    };
    let limit = CAPTURE_LIMIT.saturating_mul(harnesses.get());
    let flags = CaptureFlags {
        stop: Arc::new(AtomicBool::new(false)),
        failed: Arc::new(AtomicBool::new(false)),
    };
    let stdout_reader = spawn_capture(stdout, &flags, limit);
    let stderr_reader = spawn_capture(stderr, &flags, limit);

    let deadline = Instant::now().checked_add(timeout);
    let exited = wait_until(&child, deadline, &flags.failed);
    // The group is signalled while the leader, exited or not, is still unreaped (see
    // `wait_until`); only then is it reaped.
    kill_process_tree(&mut child);
    let reaped = child.wait();
    flags.stop.store(true, Ordering::Release);
    let stdout_bytes = finish_capture(stdout_reader);
    let stderr_bytes = finish_capture(stderr_reader);
    let exited = exited?;
    let reaped = reaped?;

    let stdout_bytes = match stream_bytes(CaptureStream::Stdout, stdout_bytes, limit, harnesses) {
        Ok(bytes) => bytes,
        Err(refusal) => return Ok(refusal),
    };
    let stderr_bytes = match stream_bytes(CaptureStream::Stderr, stderr_bytes, limit, harnesses) {
        Ok(bytes) => bytes,
        Err(refusal) => return Ok(refusal),
    };

    if exited {
        Ok(LaunchOutcome::Completed {
            exited_successfully: reaped.success(),
            exit_code: reaped.code(),
            text: format!(
                "{}\n{}",
                String::from_utf8_lossy(&stdout_bytes),
                String::from_utf8_lossy(&stderr_bytes)
            ),
        })
    } else {
        Ok(LaunchOutcome::TimedOut)
    }
}

/// The bytes of `stream`, or the outcome that refuses the run because they were not all read or
/// were more than `limit`.
fn stream_bytes(
    stream: CaptureStream,
    captured: Captured,
    limit: usize,
    harnesses: NonZeroUsize,
) -> Result<Vec<u8>, LaunchOutcome> {
    captured.map_err(|failure| match failure {
        CaptureFailure::OverLimit => LaunchOutcome::OutputOverLimit {
            stream,
            limit,
            harnesses: harnesses.get(),
        },
        CaptureFailure::Unread { detail } => LaunchOutcome::OutputUnread { stream, detail },
    })
}

/// The two flags the capture threads and the run share.
struct CaptureFlags {
    /// Set by the run once the launcher is gone: read what is in the pipe and return.
    stop: Arc<AtomicBool>,
    /// Set by a capture thread that failed: the run can no longer be trusted, stop waiting.
    failed: Arc<AtomicBool>,
}

/// Polls `child` until it has exited (`true`), `deadline` passes (`false`) or a capture thread
/// sets `failed` (`false`; the caller reports the failure, not a timeout). A `deadline` of `None`
/// never passes.
///
/// An exited launcher is **not reaped**: `waitid` with `NOWAIT` reports it and leaves it a zombie,
/// so its pid, which is its process group's id, stays allocated until the caller has signalled the
/// group and then reaped it. Reaping first (`try_wait`) would free the id while a straggler of the
/// group might not exist any more, and the group kill could then reach an unrelated process group
/// that had been given the recycled id.
fn wait_until(child: &Child, deadline: Option<Instant>, failed: &AtomicBool) -> io::Result<bool> {
    let pid = i32::try_from(child.id())
        .ok()
        .and_then(Pid::from_raw)
        .ok_or_else(|| io::Error::other("the launcher's process id is not a valid pid"))?;
    loop {
        match waitid(
            WaitId::Pid(pid),
            WaitIdOptions::EXITED | WaitIdOptions::NOHANG | WaitIdOptions::NOWAIT,
        ) {
            Ok(Some(_)) => return Ok(true),
            Ok(None) | Err(Errno::INTR) => {}
            Err(errno) => return Err(errno.into()),
        }
        if failed.load(Ordering::Acquire)
            || deadline.is_some_and(|deadline| Instant::now() >= deadline)
        {
            return Ok(false);
        }
        thread::sleep(LAUNCHER_POLL_INTERVAL);
    }
}

/// Starts a thread that reads `pipe` to its end, or until `stop` is set and nothing more is
/// waiting in it. A failure sets `failed`, so the run stops waiting for a launcher whose output
/// nobody is reading any more.
fn spawn_capture<R>(pipe: R, flags: &CaptureFlags, limit: usize) -> thread::JoinHandle<Captured>
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
        let captured = capture(pipe, &stop, limit, STOP_DRAIN_LIMIT, poll);
        failure.failed = captured.is_err();
        captured
    })
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
/// More than `limit` bytes is [`CaptureFailure::OverLimit`] and exactly `limit` is returned
/// whole; a failed poll or read is [`CaptureFailure::Unread`]. Nothing is truncated and nothing
/// that failed is returned as what had been read so far.
fn capture<R: Read + AsFd>(
    mut pipe: R,
    stop: &AtomicBool,
    limit: usize,
    drain_limit: Duration,
    poll_pipe: PollFn,
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
                kept.extend(chunk.iter().take(read));
                if kept.len() > limit {
                    return Err(CaptureFailure::OverLimit);
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

/// Kills the launcher and every process in its process group.
///
/// This exists so that a finished run, whether it timed out, was stopped or exited on its own,
/// does not leave its descendants behind: Kani's launcher
/// forks `kani-driver`, which forks CBMC, and CBMC is the solver a budget most needs to stop. A
/// `child.kill()` alone would leave it running after the launcher is gone. The launcher is started
/// as the leader of its own process group ([`run_launcher_with_timeout`]), every descendant
/// inherits that group, and one signal to the group reaches them all, however deep, with no
/// snapshot of the process tree that could miss a process forked a moment later.
///
/// A descendant that leaves the group (`setsid`, `setpgid`) is not reached. Nothing in Kani's
/// process tree does, and this call does not wait for one either way.
fn kill_process_tree(child: &mut Child) {
    if let Some(group) = i32::try_from(child.id()).ok().and_then(Pid::from_raw) {
        let _ = kill_process_group(group, Signal::KILL);
    }
    let _ = child.kill();
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write};

    use super::*;
    use crate::kani::test_support::discover_scratch;

    /// The launcher's descendants are killed when the budget elapses, not only the immediate
    /// child, against a real process tree with a genuine grandchild. `sh -c '(sh -c "echo $$ >
    /// pidfile ; exec sleep 30") & wait'` runs an outer `sh` (the pid `run_launcher_with_timeout`
    /// itself sees) that forks a distinct grandchild shell, which records its pid and execs into
    /// `sleep 30`; the outer shell only waits. Running the grandchild through `exec` directly in
    /// the outer shell would record the direct child's own pid, which plain `child.kill()`
    /// already kills, and so would prove nothing about the group kill.
    ///
    /// The budget is tried up a ladder because a budget shorter than the time the OS takes to
    /// fork the grandchild ends the run before any grandchild exists; a grandchild that died
    /// before writing its pidfile counts as killed.
    ///
    /// Trace: FR-017-AC-17, TC-027
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

            // A missing, empty, or unparseable pidfile means the grandchild was killed before
            // it finished recording its own pid — informationally at least as strong as
            // confirming its pid is gone from `/proc`, not a test failure. Treat it as "not
            // surviving" and move on rather than panicking, which would escape the retry
            // ladder entirely and report a fast, successful kill as a test crash.
            let grandchild_pid: Option<i32> = fs::read_to_string(&pidfile)
                .ok()
                .and_then(|contents| contents.trim().parse().ok());
            let grandchild_survived = grandchild_pid.is_some_and(|pid| !process_gone_within(pid));

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

            if !grandchild_survived {
                return;
            }
            // Survived this rung: indistinguishable, from here, between "genuinely not
            // killed" and "forked after this rung's snapshot" — retry at the next, larger
            // budget rather than assert either reading.
        }

        panic!(
            "the timed-out run's grandchild ({last_surviving_grandchild:?}) was still present \
             in `/proc` even at the largest budget in GRANDCHILD_KILL_TIMEOUT_LADDER \
             ({GRANDCHILD_KILL_TIMEOUT_LADDER:?}); kill_process_tree must reach every member of \
             the launcher's process group"
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

    /// A process orphaned just before the kill keeps its inherited copy of the pipes' write end
    /// open: `( sleep 45 & )` orphans a `sleep 45` at once, while `exec sleep 45` replaces the
    /// outer shell, the direct child. Waiting for the capture threads to see EOF would block
    /// until the orphan's `sleep 45` ended on its own, which is unbounded; the call must return
    /// long before that (asserted at 20 s, wide enough for a loaded host, still under the 45 s).
    ///
    /// Trace: FR-017-AC-16, TC-027
    #[test]
    fn a_process_orphaned_just_before_the_kill_does_not_block_this_calls_own_return() {
        let mut command = Command::new("sh");
        command.arg("-c").arg("( sleep 45 & ) ; exec sleep 45");
        let started = Instant::now();
        let outcome = run_launcher_with_timeout(command, Duration::from_millis(200)).unwrap();
        assert!(
            matches!(outcome, LaunchOutcome::TimedOut),
            "a run past its budget must classify as timed out"
        );
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "a 200ms budget must not take anywhere near the orphaned process's own 45s sleep: \
             took {:?}",
            started.elapsed()
        );
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
                text,
            } => {
                assert!(!exited_successfully);
                assert_eq!(exit_code, Some(3));
                assert!(text.contains("out"));
                assert!(text.contains("err"));
            }
            other => panic!("a fast process must complete, not end as {other:?}"),
        }
    }

    /// A stream of more than the limit is refused whole, never truncated to a tail and returned;
    /// a stream of exactly the limit is returned whole.
    ///
    /// Trace: FR-017-AC-14, TC-043
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
                (false, Err(CaptureFailure::OverLimit)) => {}
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
    ///
    /// Trace: FR-017-AC-16, TC-027
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
    ///
    /// Trace: FR-017-AC-16, TC-027
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
    ///
    /// Trace: FR-017-AC-16, TC-027
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
    ///
    /// Trace: FR-017-AC-16, TC-027
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
    ///
    /// Trace: FR-017-AC-14, TC-043
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
                exit_code, text, ..
            } = run_launcher_with_timeout(
                printer(stream, CAPTURE_LIMIT, "exit 3"),
                Duration::from_secs(60),
            )
            .unwrap()
            else {
                panic!("{stream}: exactly the limit completes");
            };
            assert_eq!(exit_code, Some(3));
            assert_eq!(text.len(), CAPTURE_LIMIT + 1, "{stream}: the whole stream");
        }
    }

    /// A batch process may carry the limit for each member and no more: the limit is multiplied
    /// by the harness count, and the refusal names that count.
    ///
    /// Trace: FR-017-AC-14, TC-043
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
    ///
    /// Trace: FR-017-AC-14, TC-043
    #[cfg(target_os = "linux")]
    #[test]
    fn tc_043_an_over_limit_run_is_stopped_and_kills_the_launcher_group() {
        let directory = discover_scratch("over-limit-kill");
        let pidfile = directory.join("pid");
        let marker = directory.join("ran-on");
        // The launcher shell forks a grandchild (recording its pid), prints past the limit, sleeps
        // two seconds, writes its marker and waits for the grandchild. A run that is stopped when
        // the stream goes over never reaches the marker; one that waits for the launcher to end
        // on its own writes it, and only a kill of the whole group ends the grandchild.
        let mut command = Command::new("sh");
        command.arg("-c").arg(format!(
            "sleep 45 & echo $! > {}; head -c {} /dev/zero; sleep 2; touch {}; wait",
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
    ///
    /// Trace: FR-017-AC-25, TC-043
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
        let panicked = finish_capture(spawn_capture(PanickingPipe(reader), &flags, CAPTURE_LIMIT));
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
    ///
    /// Trace: FR-017-AC-25, TC-043
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
                Err(CaptureFailure::OverLimit),
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
    ///
    /// Trace: FR-017-AC-24, TC-043
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
    ///
    /// Trace: FR-017-AC-15, TC-027
    #[test]
    fn a_timeout_of_duration_max_never_elapses_and_does_not_panic() {
        let mut command = Command::new("sh");
        command.arg("-c").arg("printf done");
        let outcome = run_launcher_with_timeout(command, Duration::MAX).unwrap();
        assert!(matches!(
            outcome,
            LaunchOutcome::Completed { exit_code: Some(0), ref text, .. } if text.contains("done")
        ));
    }
}

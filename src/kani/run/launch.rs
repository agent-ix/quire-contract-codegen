//! Launching the Kani launcher process: spawn, bounded capture, the wall-clock budget and the
//! process-group kill (FR-017).

use std::{
    io,
    io::Read,
    os::{fd::AsFd, unix::process::CommandExt},
    process::{Child, Command, ExitStatus, Stdio},
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
    process::{kill_process_group, Pid, Signal},
    time::Timespec,
};

/// How the launcher's run within its caller-declared budget
/// ([`KaniExecutionRequest::timeout`](super::execute::KaniExecutionRequest::timeout))
/// concluded.
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
}

/// Polling interval while waiting for the launcher to exit within its budget, and the longest a
/// capture thread goes between looking at its stop flag. Short enough that a tight caller-declared
/// timeout in a test is still observed promptly, long enough not to spin.
const LAUNCHER_POLL_INTERVAL: Duration = Duration::from_millis(20);

/// Most bytes kept from each of the launcher's stdout and stderr. Kani prints its concrete
/// playback last, so when a stream is longer its tail is what is kept; the verdict is not in
/// the stream at all, it is in the exported report.
/// A stream is always drained to the end so the child never blocks on a full pipe; only what is
/// retained is bounded.
const CAPTURE_LIMIT: usize = 8 * 1024 * 1024;

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
/// own. Each thread keeps at most `CAPTURE_LIMIT` bytes and polls its pipe rather than blocking
/// in `read`, so once the launcher is gone this function tells both threads to stop and joins
/// them: a descendant that left the process group may still hold the pipe's write end open. Each thread then reads what is
/// already in the pipe for at most `STOP_DRAIN_LIMIT` and returns, however fast a straggler
/// keeps writing, so the join adds at most that limit plus one poll interval to the return time.
pub fn run_launcher_with_timeout(
    mut command: Command,
    timeout: Duration,
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
    let stop = Arc::new(AtomicBool::new(false));
    let stdout_reader = spawn_capture(stdout, &stop);
    let stderr_reader = spawn_capture(stderr, &stop);

    let deadline = Instant::now().checked_add(timeout);
    let waited = wait_until(&mut child, deadline);
    if !matches!(waited, Ok(Some(_))) {
        kill_process_tree(&mut child);
        let _ = child.wait();
    }
    stop.store(true, Ordering::Release);
    let stdout_bytes = stdout_reader.join().unwrap_or_default();
    let stderr_bytes = stderr_reader.join().unwrap_or_default();

    match waited? {
        Some(status) => Ok(LaunchOutcome::Completed {
            exited_successfully: status.success(),
            exit_code: status.code(),
            text: format!(
                "{}\n{}",
                String::from_utf8_lossy(&stdout_bytes),
                String::from_utf8_lossy(&stderr_bytes)
            ),
        }),
        None => Ok(LaunchOutcome::TimedOut),
    }
}

/// Polls `child` until it exits (`Some`) or `deadline` passes (`None`). A `deadline` of `None`
/// never passes.
fn wait_until(child: &mut Child, deadline: Option<Instant>) -> io::Result<Option<ExitStatus>> {
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(Some(status));
        }
        if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            return Ok(None);
        }
        thread::sleep(LAUNCHER_POLL_INTERVAL);
    }
}

/// Starts a thread that reads `pipe` to its end, or until `stop` is set and nothing more is
/// waiting in it.
fn spawn_capture<R>(pipe: R, stop: &Arc<AtomicBool>) -> thread::JoinHandle<Vec<u8>>
where
    R: Read + AsFd + Send + 'static,
{
    let stop = Arc::clone(stop);
    thread::spawn(move || capture_tail(pipe, &stop, CAPTURE_LIMIT, STOP_DRAIN_LIMIT))
}

/// Reads `pipe` until EOF, or until `stop` is set and the pipe has been read dry or
/// `drain_limit` has passed, returning its last `limit` bytes at most. Everything written
/// before `stop` was set is already in the pipe when the flag is seen, so the drain after it
/// loses none of it.
fn capture_tail<R: Read + AsFd>(
    mut pipe: R,
    stop: &AtomicBool,
    limit: usize,
    drain_limit: Duration,
) -> Vec<u8> {
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
        match poll(&mut fds, Some(wait)) {
            Ok(0) if drain_until.is_some() => break,
            Ok(0) => continue,
            Ok(_) => {}
            Err(Errno::INTR) => continue,
            Err(_) => break,
        }
        match pipe.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => {
                kept.extend_from_slice(&chunk[..read]);
                if kept.len() > limit.saturating_mul(2) {
                    kept.drain(..kept.len() - limit);
                }
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => break,
        }
    }
    if kept.len() > limit {
        kept.drain(..kept.len() - limit);
    }
    kept
}

/// Kills the launcher and every process in its process group.
///
/// This exists so that a timed-out run does not leave its descendants behind: Kani's launcher
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
    /// in about 200ms.
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
            started.elapsed() < Duration::from_secs(2),
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
            LaunchOutcome::TimedOut => panic!("a fast process must not be reported as timed out"),
        }
    }

    /// A stream longer than the capture limit is drained to its end but only its tail is kept,
    /// because Kani prints the playback it ends with last; the launcher's own memory is bounded by the limit,
    /// not by what the child prints.
    ///
    /// Trace: FR-017-AC-14, TC-027
    #[test]
    fn a_stream_longer_than_the_capture_limit_keeps_only_its_tail() {
        let (reader, mut writer) = io::pipe().unwrap();
        let producer = thread::spawn(move || {
            for _ in 0..100 {
                writer.write_all(&[b'x'; 1000]).unwrap();
            }
            writer.write_all(b"VERIFICATION:- SUCCESSFUL").unwrap();
        });
        let kept = capture_tail(reader, &AtomicBool::new(false), 4096, STOP_DRAIN_LIMIT);
        producer.join().unwrap();
        assert_eq!(kept.len(), 4096);
        assert!(kept.ends_with(b"VERIFICATION:- SUCCESSFUL"));
    }

    /// Runs `capture_tail` on its own thread and returns what it kept, or `None` if it had not
    /// returned within `within`, so a capture that never stops fails the test instead of
    /// hanging it.
    fn capture_within(
        reader: io::PipeReader,
        stop: &Arc<AtomicBool>,
        drain_limit: Duration,
        within: Duration,
    ) -> Option<Vec<u8>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let stop = Arc::clone(stop);
        thread::spawn(move || {
            let _ = sender.send(capture_tail(reader, &stop, CAPTURE_LIMIT, drain_limit));
        });
        receiver.recv_timeout(within).ok()
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

    /// A launcher that prints far more than the capture limit and then exits still reports its
    /// real exit status and the end of its output, with the retained text bounded.
    ///
    /// Trace: FR-017-AC-14, TC-027
    #[test]
    fn a_launcher_printing_more_than_the_limit_completes_with_bounded_text() {
        let mut command = Command::new("sh");
        command.arg("-c").arg(format!(
            "head -c {} /dev/zero | tr '\\0' x; printf '\\nVERIFICATION:- SUCCESSFUL'",
            3 * CAPTURE_LIMIT
        ));
        let LaunchOutcome::Completed {
            exit_code, text, ..
        } = run_launcher_with_timeout(command, Duration::from_secs(60)).unwrap()
        else {
            panic!("the launcher exits on its own within its budget");
        };
        assert_eq!(exit_code, Some(0));
        assert!(text.len() <= CAPTURE_LIMIT + 2);
        assert!(text.contains("VERIFICATION:- SUCCESSFUL"));
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

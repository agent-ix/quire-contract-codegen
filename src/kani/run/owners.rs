//! Immutable dependencies of one bounded execution: capture polling, the group signal and reap
//! of the launcher, and the environment's peak-memory source.

use std::{
    fmt::Debug,
    io,
    process::{Child, ExitStatus},
    sync::{Arc, OnceLock},
    thread,
    time::{Duration, Instant},
};

use rustix::{
    event::{poll, PollFd},
    io::Errno,
    process::{kill_process_group, waitid, Pid, Signal, WaitId, WaitIdOptions},
    time::Timespec,
};

use super::memory::{NoPeakMemory, PeakMemorySource};

/// Poll a real capture pipe; the caller retains the descriptor and lifecycle.
pub(super) trait PollOperation: Debug + Send + Sync {
    fn poll(&self, fds: &mut [PollFd<'_>], timeout: Option<&Timespec>) -> Result<usize, Errno>;
}

#[derive(Debug)]
pub(super) struct SystemPoll;

impl PollOperation for SystemPoll {
    fn poll(&self, fds: &mut [PollFd<'_>], timeout: Option<&Timespec>) -> Result<usize, Errno> {
        poll(fds, timeout)
    }
}

/// How long a stopped launcher may take to be seen exited before the run is refused.
const REAP_LIMIT: Duration = Duration::from_secs(5);

/// Interval between looks at a stopped launcher.
const REAP_POLL_INTERVAL: Duration = Duration::from_millis(20);

/// The two teardown operations of a run, kept apart so a test can record their order
/// (FR-017-AC-68): the group is signalled while the launcher is still unreaped, so the
/// launcher still reserves the group id.
pub(super) trait LaunchTeardown: Debug + Send + Sync {
    /// Kill the launcher's whole process group.
    fn signal_group(&self, child: &mut Child);

    /// Reap the stopped launcher, bounded.
    fn reap(&self, child: &mut Child) -> io::Result<ExitStatus>;
}

#[derive(Debug)]
pub(super) struct SystemTeardown;

impl LaunchTeardown for SystemTeardown {
    fn signal_group(&self, child: &mut Child) {
        if let Some(group) = i32::try_from(child.id()).ok().and_then(Pid::from_raw) {
            let _ = kill_process_group(group, Signal::KILL);
        }
        let _ = child.kill();
    }

    /// Reap only after observing exit; never turn an unconfirmed exit into an unbounded wait.
    fn reap(&self, child: &mut Child) -> io::Result<ExitStatus> {
        let pid = i32::try_from(child.id())
            .ok()
            .and_then(Pid::from_raw)
            .ok_or_else(|| io::Error::other("invalid launcher pid"))?;
        let deadline = Instant::now() + REAP_LIMIT;
        loop {
            match waitid(
                WaitId::Pid(pid),
                WaitIdOptions::EXITED | WaitIdOptions::NOHANG | WaitIdOptions::NOWAIT,
            ) {
                Ok(Some(_)) => return child.wait(),
                Ok(None) | Err(Errno::INTR) => {}
                Err(error) => return Err(error.into()),
            }
            if Instant::now() >= deadline {
                return Err(io::Error::other("launcher termination was not confirmed"));
            }
            thread::sleep(REAP_POLL_INTERVAL);
        }
    }
}

/// Construction-time composition; clones retain the same dependencies and grouping identity.
#[derive(Debug)]
pub(super) struct LaunchOwners {
    pub(super) stdout: Arc<dyn PollOperation>,
    pub(super) stderr: Arc<dyn PollOperation>,
    pub(super) teardown: Arc<dyn LaunchTeardown>,
    pub(super) peak: Arc<dyn PeakMemorySource>,
}

impl LaunchOwners {
    /// All ordinary installations share the real composition, preserving their process grouping.
    pub(super) fn real() -> Arc<Self> {
        static REAL: OnceLock<Arc<LaunchOwners>> = OnceLock::new();
        Arc::clone(REAL.get_or_init(|| {
            Arc::new(Self {
                stdout: Arc::new(SystemPoll),
                stderr: Arc::new(SystemPoll),
                teardown: Arc::new(SystemTeardown),
                peak: Arc::new(NoPeakMemory),
            })
        }))
    }

    /// The same composition reporting its peak through `peak`.
    pub(super) fn with_peak(&self, peak: Arc<dyn PeakMemorySource>) -> Arc<Self> {
        Arc::new(Self {
            stdout: Arc::clone(&self.stdout),
            stderr: Arc::clone(&self.stderr),
            teardown: Arc::clone(&self.teardown),
            peak,
        })
    }
}

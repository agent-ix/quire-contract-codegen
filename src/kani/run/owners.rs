//! Immutable dependencies of one bounded execution's existing observation and teardown owners.

use std::{
    fmt::Debug,
    io,
    sync::{Arc, OnceLock},
};

use rustix::{
    event::{poll, PollFd},
    io::Errno,
    time::Timespec,
};

use super::memory::MemoryObserver;

/// Sample through the genuine observer, retaining its bound process identities and RSS history.
pub(super) trait MemorySampling: Debug + Send + Sync {
    fn observe(&self, observer: &mut MemoryObserver, root: u32) -> io::Result<u64>;
}

#[derive(Debug)]
pub(super) struct ProcessMemory;

impl MemorySampling for ProcessMemory {
    fn observe(&self, observer: &mut MemoryObserver, root: u32) -> io::Result<u64> {
        observer.observe(root)
    }
}

/// Poll a real capture pipe or owned init pidfd; the caller retains the descriptor and lifecycle.
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

/// Construction-time composition; clones retain the same dependencies and grouping identity.
#[derive(Debug)]
pub(super) struct LaunchOwners {
    pub(super) memory: Arc<dyn MemorySampling>,
    pub(super) stdout: Arc<dyn PollOperation>,
    pub(super) stderr: Arc<dyn PollOperation>,
    pub(super) confirmation: Arc<dyn PollOperation>,
}

impl LaunchOwners {
    /// All ordinary installations share the real composition, preserving their process grouping.
    pub(super) fn real() -> Arc<Self> {
        static REAL: OnceLock<Arc<LaunchOwners>> = OnceLock::new();
        Arc::clone(REAL.get_or_init(|| {
            Arc::new(Self {
                memory: Arc::new(ProcessMemory),
                stdout: Arc::new(SystemPoll),
                stderr: Arc::new(SystemPoll),
                confirmation: Arc::new(SystemPoll),
            })
        }))
    }
}

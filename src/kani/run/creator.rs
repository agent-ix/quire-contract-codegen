//! Actual creating-thread and launcher liveness capabilities for the outer-owner startup.
//!
//! A process pidfd cannot detect a nonleader creator's death. Conversely Linux delays a leader
//! pidfd's exit notification while siblings remain alive. The caller therefore creates L from a
//! retained dedicated nonleader thread and transfers that thread's actual pidfd before setup.

use std::{
    io,
    os::fd::{AsFd, BorrowedFd, OwnedFd},
};

use rustix::{
    event::{poll, PollFd, PollFlags},
    process::{pidfd_open, Pid, PidfdFlags, Signal},
    time::Timespec,
};

/// Kernel capability for the actual dedicated spawning thread, never a caller-selected number.
pub(super) struct CreatorThread {
    tid: Pid,
    pin: OwnedFd,
}

impl CreatorThread {
    /// Called inside the dedicated spawner before creating L.
    pub(super) fn capture() -> io::Result<Self> {
        let raw = nix::unistd::gettid().as_raw();
        let tid = Pid::from_raw(raw)
            .ok_or_else(|| io::Error::other("creating thread has no positive kernel identity"))?;
        let process = i32::try_from(std::process::id())
            .map_err(|_| io::Error::other("caller process identity exceeds the platform range"))?;
        if raw == process {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "outer launcher requires a retained nonleader creating thread",
            ));
        }
        // Linux UAPI defines PIDFD_THREAD as O_EXCL. Rustix explicitly admits externally defined
        // pidfd flags; no raw syscall, guessed flag value or process-pidfd fallback is used.
        let flags =
            PidfdFlags::from_bits_retain(rustix::fs::OFlags::EXCL.bits()) | PidfdFlags::NONBLOCK;
        let pin = pidfd_open(tid, flags)?;
        require_live(&pin)?;
        Ok(Self { tid, pin })
    }

    pub(super) fn tid(&self) -> Pid {
        self.tid
    }

    pub(super) fn descriptor(&self) -> BorrowedFd<'_> {
        self.pin.as_fd()
    }

    pub(super) fn require_live(&self) -> io::Result<()> {
        require_live(&self.pin)
    }
}

/// Checks the actual retained capability, including an outside parent invisible as PID0.
pub(super) fn require_live(pin: impl AsFd) -> io::Result<()> {
    let mut events = [PollFd::new(&pin, PollFlags::IN)];
    poll(&mut events, Some(&Timespec::default()))?;
    let received = events[0].revents();
    if received.intersects(PollFlags::IN | PollFlags::HUP) {
        return Err(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "the positively owned creator has terminated",
        ));
    }
    if !received.is_empty() {
        return Err(io::Error::other(
            "creator liveness capability cannot be observed",
        ));
    }
    Ok(())
}

/// Arm, inspect, then check actual creator liveness; repeat after any credential transition.
pub(super) fn arm_parent_death(pin: impl AsFd) -> io::Result<()> {
    rustix::process::set_parent_process_death_signal(Some(Signal::KILL))?;
    if rustix::process::parent_process_death_signal()? != Some(Signal::KILL) {
        return Err(io::Error::other(
            "parent-death protection was not installed",
        ));
    }
    require_live(pin)
}

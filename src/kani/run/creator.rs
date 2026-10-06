//! Actual creating-thread and launcher liveness capabilities for the outer-owner startup.
//!
//! A process pidfd cannot detect a nonleader creator's death. Conversely Linux delays a leader
//! pidfd's exit notification while siblings remain alive. The caller therefore creates L from a
//! retained dedicated nonleader thread and transfers that thread's actual pidfd before setup.

use std::{
    fs::File,
    io::{self, Read},
    os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd},
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

/// Authenticate the nonleader creator before L changes its namespace/proc view. The process
/// capability has already been matched to the exclusive bootstrap peer by the role transport.
/// Actual pidfd fdinfo binds the thread number; a live thread's pinned status binds its TGID.
/// A bare caller-selected TID or getppid alone never establishes creating-thread authority.
pub(super) fn validate_parent_thread(thread: &OwnedFd, process: &OwnedFd) -> io::Result<()> {
    require_live(thread)?;
    require_live(process)?;
    let thread_metadata = rustix::fs::fstat(thread)?;
    let process_metadata = rustix::fs::fstat(process)?;
    if thread_metadata.st_dev != process_metadata.st_dev {
        return Err(io::Error::other("creating thread is not an actual pidfd"));
    }
    let thread_pid = descriptor_pid(thread)?;
    let process_pid = descriptor_pid(process)?;
    if thread_pid == process_pid
        || process_pid != nix::unistd::getppid().as_raw()
        || thread_pid <= 0
        || process_pid <= 0
    {
        return Err(io::Error::other(
            "creating thread is not the nonleader parent",
        ));
    }
    let status = read_bounded(&format!("/proc/{thread_pid}/status"))?;
    if unique_number(&status, b"Pid:")? != thread_pid
        || unique_number(&status, b"Tgid:")? != process_pid
    {
        return Err(io::Error::other(
            "creating thread belongs to another caller",
        ));
    }
    // The original thread must still be live after reading its status: a replacement task's
    // matching number cannot satisfy the retained original capability.
    require_live(thread)?;
    require_live(process)
}

fn descriptor_pid(descriptor: &OwnedFd) -> io::Result<i32> {
    let info = read_bounded(&format!("/proc/self/fdinfo/{}", descriptor.as_raw_fd()))?;
    unique_number(&info, b"Pid:")
}

fn read_bounded(path: &str) -> io::Result<Vec<u8>> {
    const PROC_BYTES: usize = 16_384;
    let file = File::open(path)?;
    let mut bytes = Vec::new();
    file.take(16_385).read_to_end(&mut bytes)?;
    if bytes.len() > PROC_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "creator proc record exceeds bound",
        ));
    }
    Ok(bytes)
}

fn unique_number(record: &[u8], field: &[u8]) -> io::Result<i32> {
    let mut found = None;
    for line in record.split(|byte| *byte == b'\n') {
        if let Some(value) = line.strip_prefix(field) {
            let value = std::str::from_utf8(value.trim_ascii())
                .map_err(|_| io::Error::other("creator identity is not numeric"))?
                .parse::<i32>()
                .map_err(|_| io::Error::other("creator identity is not numeric"))?;
            if found.replace(value).is_some() {
                return Err(io::Error::other("creator identity field repeated"));
            }
        }
    }
    found.ok_or_else(|| io::Error::other("creator identity field absent"))
}

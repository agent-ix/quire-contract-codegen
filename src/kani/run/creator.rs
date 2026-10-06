//! Actual creating-thread and launcher liveness capabilities for the outer-owner startup.
//!
//! A process pidfd cannot detect a nonleader creator's death. Conversely Linux delays a leader
//! pidfd's exit notification while siblings remain alive. The caller therefore creates L from a
//! retained dedicated nonleader thread and transfers that thread's actual pidfd before setup.

use std::{
    fmt::Write as _,
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

/// C's identity records and numeric proc paths are reserved before L exists. The same scratch
/// is reused for both pidfd records and the child status, without retaining data across reads.
pub(super) struct PreparedIdentity {
    bytes: Vec<u8>,
    path: String,
}

impl PreparedIdentity {
    pub(super) fn prepare() -> io::Result<Self> {
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(PROC_BYTES + 1)
            .map_err(io::Error::other)?;
        bytes.resize(PROC_BYTES + 1, 0);
        let mut path = String::new();
        // All generated paths contain only a fixed prefix/suffix and one positive i32 number.
        // Reserve the complete maximum before any role spawn; arbitrary paths are not accepted.
        path.try_reserve_exact(64).map_err(io::Error::other)?;
        Ok(Self { bytes, path })
    }

    pub(super) fn reserved_bytes(&self) -> io::Result<u64> {
        let bytes = self
            .bytes
            .capacity()
            .checked_add(self.path.capacity())
            .ok_or_else(|| io::Error::other("caller identity reservation overflow"))?;
        u64::try_from(bytes)
            .map_err(|_| io::Error::other("caller identity reservation exceeds platform"))
    }

    /// Bind the actual received process capability to its retained parent in C's proc view.
    /// Neither a recycled numeric PID nor a record from another parent satisfies both live pins.
    pub(super) fn validate_child_process(
        &mut self,
        child: &OwnedFd,
        parent: &OwnedFd,
    ) -> io::Result<i32> {
        require_live(child)?;
        require_live(parent)?;
        let child_metadata = rustix::fs::fstat(child)?;
        let parent_metadata = rustix::fs::fstat(parent)?;
        if child_metadata.st_dev != parent_metadata.st_dev {
            return Err(io::Error::other("child capability is not an actual pidfd"));
        }
        let child_pid = self.descriptor_pid(child)?;
        let parent_pid = self.descriptor_pid(parent)?;
        if child_pid <= 0 || parent_pid <= 0 || child_pid == parent_pid {
            return Err(io::Error::other(
                "child capability has no distinct live process identity",
            ));
        }
        self.path.clear();
        write!(&mut self.path, "/proc/{child_pid}/status").map_err(io::Error::other)?;
        let status = self.read_record()?;
        if unique_number(status, b"Pid:")? != child_pid
            || unique_number(status, b"Tgid:")? != child_pid
            || unique_number(status, b"PPid:")? != parent_pid
        {
            return Err(io::Error::other(
                "child capability is not the retained parent's process",
            ));
        }
        require_live(child)?;
        require_live(parent)?;
        Ok(child_pid)
    }

    pub(super) fn child_namespace(
        &mut self,
        pid: i32,
    ) -> io::Result<super::outer_setup::NamespaceIdentity> {
        if pid <= 0 {
            return Err(io::Error::other(
                "namespace child identity must be positive",
            ));
        }
        self.path.clear();
        write!(&mut self.path, "/proc/{pid}/ns/pid").map_err(io::Error::other)?;
        super::outer_setup::NamespaceIdentity::read(&self.path)
    }

    pub(super) fn child_start(&mut self, pid: i32) -> io::Result<u64> {
        if pid <= 0 {
            return Err(io::Error::other("claimed child identity must be positive"));
        }
        self.path.clear();
        write!(&mut self.path, "/proc/{pid}/stat").map_err(io::Error::other)?;
        super::memory::process_start_record(self.read_record()?)
    }

    fn descriptor_pid(&mut self, descriptor: &OwnedFd) -> io::Result<i32> {
        self.path.clear();
        write!(
            &mut self.path,
            "/proc/self/fdinfo/{}",
            descriptor.as_raw_fd()
        )
        .map_err(io::Error::other)?;
        unique_number(self.read_record()?, b"Pid:")
    }

    fn read_record(&mut self) -> io::Result<&[u8]> {
        let mut file = File::open(&self.path)?;
        let mut length = 0;
        while length < self.bytes.len() {
            match file.read(&mut self.bytes[length..]) {
                Ok(0) => break,
                Ok(count) => length += count,
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                Err(error) => return Err(error),
            }
        }
        if length > PROC_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "creator proc record exceeds bound",
            ));
        }
        Ok(&self.bytes[..length])
    }
}

const PROC_BYTES: usize = 16_384;

fn descriptor_pid(descriptor: &OwnedFd) -> io::Result<i32> {
    let info = read_bounded(&format!("/proc/self/fdinfo/{}", descriptor.as_raw_fd()))?;
    unique_number(&info, b"Pid:")
}

fn read_bounded(path: &str) -> io::Result<Vec<u8>> {
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

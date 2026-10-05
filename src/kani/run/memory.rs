//! Linux procfs observation of aggregate backend-tree resident memory (FR-028-AC-21).
//!
//! The peak is the largest resident total observed at a polling instant. It is not an
//! allocation limit or an estimate of memory between observations. Descendants are retained
//! by pid and start time after leaving the launcher's process group; a reused pid is never
//! counted or signalled as the old process.

use std::{
    collections::BTreeMap,
    fs, io,
    io::Read,
    path::{Path, PathBuf},
};

#[cfg(target_os = "linux")]
use rustix::process::{pidfd_open, pidfd_send_signal, PidfdFlags};
use rustix::process::{Pid, Signal};
use serde::Serialize;

/// The mechanism that actually enforced the run's memory ceiling.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryMechanism {
    /// Poll the aggregate resident bytes of the launcher and its observed descendants.
    LinuxProcfsTreeRss,
}

/// Memory observed by the mechanism during one launch.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryObservation {
    /// The actual mechanism used, rather than a requested mechanism.
    pub mechanism: MemoryMechanism,
    /// Largest aggregate resident memory observed, in bytes. Absent until a tree was observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_resident_bytes: Option<u64>,
}

/// A procfs process's identity and ancestry. The start tick prevents pid-reuse confusion.
struct Process {
    pid: u32,
    parent: u32,
    group: u32,
    start: u64,
}

pub(super) struct MemoryObserver {
    root: PathBuf,
    known: BTreeMap<u32, u64>,
    observation: MemoryObservation,
}

impl MemoryObserver {
    /// Check mechanism availability before the backend is spawned.
    pub(super) fn prepare(root: &Path) -> io::Result<Self> {
        if !cfg!(target_os = "linux") {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "tree resident-memory observation requires Linux procfs",
            ));
        }
        let observer = Self {
            root: root.to_path_buf(),
            known: BTreeMap::new(),
            observation: MemoryObservation {
                mechanism: MemoryMechanism::LinuxProcfsTreeRss,
                peak_resident_bytes: None,
            },
        };
        // Reading a directory alone is insufficient: verify both ancestry and resident bytes.
        let processes = observer.processes()?;
        let own = std::process::id();
        let own_process = processes
            .iter()
            .find(|process| process.pid == own)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Unsupported,
                    "procfs does not expose the caller's process",
                )
            })?;
        observer
            .resident_bytes(own, own_process.start)?
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::Unsupported, "procfs caller identity changed")
            })?;
        #[cfg(target_os = "linux")]
        {
            let pid = i32::try_from(own)
                .ok()
                .and_then(Pid::from_raw)
                .ok_or_else(|| io::Error::other("the caller has no valid pid"))?;
            pidfd_open(pid, PidfdFlags::empty())?;
        }
        Ok(observer)
    }

    pub(super) fn observation(&self) -> MemoryObservation {
        self.observation.clone()
    }

    pub(super) fn observe(&mut self, launcher: u32) -> io::Result<u64> {
        let processes = self.processes()?;
        self.known.retain(|pid, start| {
            processes
                .iter()
                .any(|process| process.pid == *pid && process.start == *start)
        });
        // Group membership finds a descendant even if its parent exited between polls. Parent
        // membership also finds descendants that changed their group or session.
        loop {
            let mut added = false;
            for process in &processes {
                if (process.pid == launcher
                    || process.group == launcher
                    || self.known.contains_key(&process.parent))
                    && !self.known.contains_key(&process.pid)
                {
                    self.known.insert(process.pid, process.start);
                    added = true;
                }
            }
            if !added {
                break;
            }
        }
        let mut total = 0u64;
        let mut observed = false;
        for (pid, start) in &self.known {
            match self.resident_bytes(*pid, *start) {
                Ok(Some(bytes)) => {
                    total = total.saturating_add(bytes);
                    observed = true;
                }
                Ok(None) => {}
                Err(error) if process_disappeared(&error) => {}
                Err(error) => return Err(error),
            }
        }
        if observed {
            self.observation.peak_resident_bytes =
                Some(self.observation.peak_resident_bytes.unwrap_or(0).max(total));
        }
        Ok(total)
    }

    /// Kill descendants that escaped the group, checking start time before signalling them.
    pub(super) fn kill_known(&self) {
        for (pid, start) in &self.known {
            #[cfg(target_os = "linux")]
            {
                let Some(signal_pid) = i32::try_from(*pid).ok().and_then(Pid::from_raw) else {
                    continue;
                };
                // Claim the process handle before checking ancestry. If its pid is recycled
                // after that check, the signal still addresses this handle's process.
                let Ok(handle) = pidfd_open(signal_pid, PidfdFlags::empty()) else {
                    continue;
                };
                let stat = self.root.join(pid.to_string()).join("stat");
                let Ok(text) = fs::read_to_string(stat) else {
                    continue;
                };
                let Ok(process) = parse_process(&text) else {
                    continue;
                };
                if process.start == *start {
                    let _ = pidfd_send_signal(&handle, Signal::KILL);
                }
            }
        }
    }

    fn processes(&self) -> io::Result<Vec<Process>> {
        let mut processes = Vec::new();
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            if entry
                .file_name()
                .to_str()
                .and_then(|name| name.parse::<u32>().ok())
                .is_none()
            {
                continue;
            }
            match fs::read_to_string(entry.path().join("stat")) {
                Ok(text) => processes.push(parse_process(&text)?),
                Err(error) if process_disappeared(&error) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(processes)
    }

    fn resident_bytes(&self, pid: u32, start: u64) -> io::Result<Option<u64>> {
        let directory = self.root.join(pid.to_string());
        // Pin the status file before rechecking identity. A later pid reuse cannot redirect
        // this open file to the replacement process; an exited process may instead yield ESRCH.
        let mut status_file = fs::File::open(directory.join("status"))?;
        let process = parse_process(&fs::read_to_string(directory.join("stat"))?)?;
        if process.start != start {
            return Ok(None);
        }
        let mut status = String::new();
        status_file.read_to_string(&mut status)?;
        for line in status.lines() {
            if let Some(rss) = line.strip_prefix("VmRSS:") {
                let mut words = rss.split_whitespace();
                let kib = words
                    .next()
                    .and_then(|word| word.parse::<u64>().ok())
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "invalid procfs resident memory")
                    })?;
                if words.next() != Some("kB") || words.next().is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "invalid procfs resident-memory unit",
                    ));
                }
                return kib.checked_mul(1024).map(Some).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "procfs resident memory overflows bytes",
                    )
                });
            }
        }
        // Exited processes have no address space and no VmRSS entry.
        if status
            .lines()
            .any(|line| line.starts_with("State:") && line.split_whitespace().nth(1) == Some("Z"))
        {
            return Ok(Some(0));
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "procfs has no resident-memory value",
        ))
    }
}

fn process_disappeared(error: &io::Error) -> bool {
    error.kind() == io::ErrorKind::NotFound
        || error.raw_os_error() == Some(rustix::io::Errno::SRCH.raw_os_error())
}

fn parse_process(text: &str) -> io::Result<Process> {
    let malformed = || {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid procfs process ancestry",
        )
    };
    let (pid, _) = text.split_once(' ').ok_or_else(malformed)?;
    // A comm field can contain spaces and ')' characters; the final ')' ends that field.
    let (_, tail) = text.rsplit_once(')').ok_or_else(malformed)?;
    let mut fields = tail.split_whitespace();
    fields.next().ok_or_else(malformed)?;
    let parent = fields
        .next()
        .and_then(|value| value.parse().ok())
        .ok_or_else(malformed)?;
    let group = fields
        .next()
        .and_then(|value| value.parse().ok())
        .ok_or_else(malformed)?;
    let start = fields
        .nth(16)
        .and_then(|value| value.parse().ok())
        .ok_or_else(malformed)?;
    Ok(Process {
        pid: pid.parse().map_err(|_| malformed())?,
        parent,
        group,
        start,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Trace: FR-028-AC-21.
    #[test]
    fn resident_memory_of_a_reused_pid_is_excluded_from_the_backend_sample() {
        let root = crate::kani::test_support::discover_scratch("memory-pid-reuse");
        let directory = root.join("42");
        fs::create_dir(&directory).unwrap();
        let mut fields = ["0"; 20];
        fields[0] = "S";
        fields[1] = "1";
        fields[2] = "42";
        fields[19] = "200";
        fs::write(
            directory.join("stat"),
            format!("42 (replacement) {}", fields.join(" ")),
        )
        .unwrap();
        fs::write(directory.join("status"), "VmRSS:\t65536 kB\n").unwrap();
        let observer = MemoryObserver {
            root: root.clone(),
            known: BTreeMap::new(),
            observation: MemoryObservation {
                mechanism: MemoryMechanism::LinuxProcfsTreeRss,
                peak_resident_bytes: None,
            },
        };
        assert_eq!(observer.resident_bytes(42, 100).unwrap(), None);
        assert_eq!(
            observer.resident_bytes(42, 200).unwrap(),
            Some(64 * 1024 * 1024)
        );
        fs::remove_dir_all(root).unwrap();
    }

    /// Trace: FR-028-AC-21.
    #[test]
    fn available_observer_does_not_invent_a_peak_before_observing_a_tree() {
        let observer = MemoryObserver::prepare(Path::new("/proc")).unwrap();
        assert_eq!(
            observer.observation().mechanism,
            MemoryMechanism::LinuxProcfsTreeRss
        );
        assert_eq!(observer.observation().peak_resident_bytes, None);
    }
}

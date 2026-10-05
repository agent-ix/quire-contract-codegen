//! Linux procfs observation of the sum of backend-process resident memory (FR-028-AC-21).
//!
//! The peak is the largest resident total observed at a polling instant. It is not an
//! allocation limit or an estimate of memory between observations. Descendants are retained
//! by pid and start time after leaving the launcher's process group; a reused pid is never
//! counted or signalled as the old process.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    io::Read,
    path::{Path, PathBuf},
};

use rustix::process::Pid;
#[cfg(target_os = "linux")]
use rustix::process::{pidfd_open, PidfdFlags};
use serde::Serialize;

/// The mechanism that actually enforced the run's memory ceiling.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryMechanism {
    /// Own all descendants with a PID namespace and poll their conservative per-process RSS sum.
    LinuxPidNamespaceProcfsTreeRss,
}

/// Memory observed by the mechanism during one launch.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryObservation {
    /// The actual mechanism used, rather than a requested mechanism.
    pub mechanism: MemoryMechanism,
    /// Largest observed sum of per-process RSS, in bytes; shared pages count once per process.
    /// This conservative metric can exceed the physical footprint. Absent until observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_resident_bytes: Option<u64>,
}

/// A procfs process's identity and ancestry. The start tick prevents pid-reuse confusion.
struct Process {
    pid: u32,
    parent: u32,
    start: u64,
    virtual_bytes: u64,
    resident_pages: u64,
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
                mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                peak_resident_bytes: None,
            },
        };
        // Reading a directory alone is insufficient: verify both ancestry and resident bytes.
        let own = std::process::id();
        let own_process = parse_process(&fs::read_to_string(
            root.join(own.to_string()).join("stat"),
        )?)?;
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

    pub(super) fn bind_root(&mut self, pid: u32, start: u64) -> io::Result<()> {
        let process = parse_process(&fs::read_to_string(
            self.root.join(pid.to_string()).join("stat"),
        )?)?;
        if process.start != start {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "owned namespace init identity changed",
            ));
        }
        self.resident_bytes(pid, start)?.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "owned init RSS identity changed",
            )
        })?;
        self.known.insert(pid, start);
        Ok(())
    }

    pub(super) fn observation(&self) -> MemoryObservation {
        self.observation.clone()
    }

    pub(super) fn observe(&mut self, launcher: u32) -> io::Result<u64> {
        let processes = self.processes(launcher)?;
        self.known.retain(|pid, start| {
            *pid == launcher
                || processes
                    .iter()
                    .any(|process| process.pid == *pid && process.start == *start)
        });
        for process in &processes {
            self.known.insert(process.pid, process.start);
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

    /// Walk only the owned namespace subtree, including children created by worker threads.
    fn processes(&self, launcher: u32) -> io::Result<Vec<Process>> {
        let mut pending: Vec<(u32, Option<(u32, u64)>)> = vec![(launcher, None)];
        let mut seen = BTreeSet::new();
        let mut processes = Vec::new();
        while let Some((pid, parent)) = pending.pop() {
            if !seen.insert(pid) {
                continue;
            }
            let directory = self.root.join(pid.to_string());
            let process = match fs::read_to_string(directory.join("stat")) {
                Ok(text) => parse_process(&text)?,
                Err(error) if process_disappeared(&error) => continue,
                Err(error) => return Err(error),
            };
            if pid == launcher
                && self
                    .known
                    .get(&pid)
                    .is_some_and(|start| *start != process.start)
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "owned namespace init was recycled",
                ));
            }
            if let Some((parent_pid, parent_start)) = parent {
                if process.parent != parent_pid {
                    continue;
                }
                let parent_stat =
                    match fs::read_to_string(self.root.join(parent_pid.to_string()).join("stat")) {
                        Ok(text) => parse_process(&text)?,
                        Err(error) if process_disappeared(&error) => continue,
                        Err(error) => return Err(error),
                    };
                if parent_stat.start != parent_start {
                    continue;
                }
            }
            let tasks = match fs::read_dir(directory.join("task")) {
                Ok(tasks) => tasks,
                Err(error) if process_disappeared(&error) => {
                    // A reaped child can vanish after its stat was read. A live leader whose
                    // workers still exist is different: missing task ancestry stays fail-closed.
                    match fs::read_to_string(directory.join("stat")) {
                        Err(error) if process_disappeared(&error) => continue,
                        Err(error) => return Err(error),
                        Ok(stat) => {
                            let fresh = parse_process(&stat)?;
                            if fresh.start != process.start {
                                continue;
                            }
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "live owned task ancestry unavailable",
                            ));
                        }
                    }
                }
                Err(error) => return Err(error),
            };
            for task in tasks {
                let task = task?;
                match fs::read_to_string(task.path().join("children")) {
                    Ok(children) => {
                        for child in children.split_whitespace() {
                            pending.push((
                                child.parse::<u32>().map_err(|_| {
                                    io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "invalid owned child pid",
                                    )
                                })?,
                                Some((pid, process.start)),
                            ));
                        }
                    }
                    Err(error) if process_disappeared(&error) => {}
                    Err(error) => return Err(error),
                }
            }
            processes.push(process);
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
        if let Some(bytes) = status_rss(&status)? {
            return Ok(Some(bytes));
        }
        let threads = status
            .lines()
            .find_map(|line| line.strip_prefix("Threads:"))
            .and_then(|value| value.trim().parse::<u64>().ok())
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "procfs has no thread count")
            })?;
        if threads > 1 {
            // An exited leader can have no mm while workers still share the live address space.
            // Use one worker's RSS, never sum threads that share the same mm.
            let workers = fs::read_dir(directory.join("task")).map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("live worker RSS unavailable: {error}"),
                )
            })?;
            for worker in workers {
                let worker = worker.map_err(|error| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("live worker enumeration failed: {error}"),
                    )
                })?;
                let mut pinned = match fs::File::open(worker.path().join("status")) {
                    Ok(pinned) => pinned,
                    Err(error) if process_disappeared(&error) => continue,
                    Err(error) => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("live worker status unavailable: {error}"),
                        ))
                    }
                };
                let latest = parse_process(&fs::read_to_string(directory.join("stat"))?)?;
                if latest.start != start {
                    return Ok(None);
                }
                let mut text = String::new();
                match pinned.read_to_string(&mut text) {
                    Ok(_) => {}
                    Err(error) if process_disappeared(&error) => continue,
                    Err(error) => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("live worker RSS unavailable: {error}"),
                        ))
                    }
                }
                if let Some(bytes) = status_rss(&text)? {
                    return Ok(Some(bytes));
                }
            }
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "live worker RSS unavailable",
            ));
        }
        // Linux can release task->mm before publishing a zombie state. Its status then has
        // no VmRSS; stat reports zero virtual bytes and resident pages. Verify both from a
        // fresh, identity-matched stat instead of treating an unexplained missing field as zero.
        let latest = parse_process(&fs::read_to_string(directory.join("stat"))?)?;
        if latest.start != start {
            return Ok(None);
        }
        if latest.virtual_bytes == 0 && latest.resident_pages == 0 {
            return Ok(Some(0));
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "procfs has no resident-memory value",
        ))
    }
}

fn status_rss(status: &str) -> io::Result<Option<u64>> {
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
    Ok(None)
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
    let parent: u32 = fields
        .next()
        .and_then(|value| value.parse().ok())
        .ok_or_else(malformed)?;
    let _group: u32 = fields
        .next()
        .and_then(|value| value.parse().ok())
        .ok_or_else(malformed)?;
    let start = fields
        .nth(16)
        .and_then(|value| value.parse().ok())
        .ok_or_else(malformed)?;
    let virtual_bytes = fields
        .next()
        .and_then(|value| value.parse().ok())
        .ok_or_else(malformed)?;
    let resident_pages = fields
        .next()
        .and_then(|value| value.parse().ok())
        .ok_or_else(malformed)?;
    Ok(Process {
        pid: pid.parse().map_err(|_| malformed())?,
        parent,
        start,
        virtual_bytes,
        resident_pages,
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
        let mut fields = ["0"; 22];
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
                mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
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
    fn released_address_space_is_observed_before_zombie_status_but_missing_rss_is_refused() {
        let root = crate::kani::test_support::discover_scratch("memory-address-space-release");
        let directory = root.join("42");
        fs::create_dir_all(directory.join("task/42")).unwrap();
        fs::write(directory.join("task/42/children"), []).unwrap();
        let mut fields = ["0"; 22];
        fields[0] = "R";
        fields[1] = "1";
        fields[2] = "42";
        fields[19] = "200";
        let stat = |fields: &[&str]| format!("42 (exiting) {}", fields.join(" "));
        fs::write(directory.join("stat"), stat(&fields)).unwrap();
        fs::write(
            directory.join("status"),
            "State:\tR (running)\nThreads:\t1\n",
        )
        .unwrap();
        let mut observer = MemoryObserver {
            root: root.clone(),
            known: BTreeMap::new(),
            observation: MemoryObservation {
                mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                peak_resident_bytes: None,
            },
        };
        assert_eq!(observer.observe(42).unwrap(), 0);
        assert_eq!(observer.observation().peak_resident_bytes, Some(0));
        fields[20] = "4096";
        fields[21] = "1";
        fs::write(directory.join("stat"), stat(&fields)).unwrap();
        assert_eq!(
            observer.observe(42).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }

    /// Trace: FR-028-AC-21.
    #[test]
    fn available_observer_does_not_invent_a_peak_before_observing_a_tree() {
        let observer = MemoryObserver::prepare(Path::new("/proc")).unwrap();
        assert_eq!(
            observer.observation().mechanism,
            MemoryMechanism::LinuxPidNamespaceProcfsTreeRss
        );
        assert_eq!(observer.observation().peak_resident_bytes, None);
    }
    /// Trace: FR-028-AC-21.
    #[test]
    fn missing_task_ancestry_refuses_a_zombie_leader_with_live_workers() {
        let root = crate::kani::test_support::discover_scratch("memory-task-exit");
        let directory = root.join("42");
        fs::create_dir(&directory).unwrap();
        let mut fields = ["0"; 22];
        fields[0] = "Z";
        fields[1] = "1";
        fields[2] = "42";
        fields[19] = "200";
        let stat = |fields: &[&str]| format!("42 (exiting) {}", fields.join(" "));
        fs::write(directory.join("stat"), stat(&fields)).unwrap();
        let observer = MemoryObserver {
            root: root.clone(),
            known: BTreeMap::from([(42, 200)]),
            observation: MemoryObservation {
                mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                peak_resident_bytes: None,
            },
        };
        fs::write(
            directory.join("status"),
            "State:\tZ (zombie)\nThreads:\t2\n",
        )
        .unwrap();
        assert_eq!(
            observer.processes(42).err().unwrap().kind(),
            io::ErrorKind::InvalidData
        );
        fields[0] = "R";
        fs::write(directory.join("stat"), stat(&fields)).unwrap();
        assert_eq!(
            observer.processes(42).err().unwrap().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }

    /// Trace: FR-028-AC-21.
    #[test]
    fn released_leader_mm_uses_live_worker_rss_or_refuses_observation() {
        let root = crate::kani::test_support::discover_scratch("memory-live-worker");
        let directory = root.join("42");
        fs::create_dir_all(directory.join("task/43")).unwrap();
        let mut fields = ["0"; 22];
        fields[0] = "Z";
        fields[1] = "1";
        fields[2] = "42";
        fields[19] = "200";
        fs::write(
            directory.join("stat"),
            format!("42 (leader) {}", fields.join(" ")),
        )
        .unwrap();
        fs::write(directory.join("status"), "Threads:\t2\n").unwrap();
        fs::write(directory.join("task/43/status"), "VmRSS:\t65536 kB\n").unwrap();
        let observer = MemoryObserver {
            root: root.clone(),
            known: BTreeMap::new(),
            observation: MemoryObservation {
                mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                peak_resident_bytes: None,
            },
        };
        assert_eq!(
            observer.resident_bytes(42, 200).unwrap(),
            Some(64 * 1024 * 1024)
        );
        fs::remove_dir_all(directory.join("task")).unwrap();
        assert_eq!(
            observer.resident_bytes(42, 200).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }
}

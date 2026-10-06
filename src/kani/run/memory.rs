//! Linux procfs observation of the sum of backend-process resident memory (FR-028-AC-21).
//!
//! Sampling walks the owned namespace init's task-child ancestry, with start identities
//! excluding PID reuse. This observer signals no process; NamespaceOwner owns teardown.
//! The peak is the largest conservative sum of per-process RSS observed at a polling instant,
//! counting shared pages per process. It is no allocation limit or intersample peak estimate.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    time::Instant,
};

#[cfg(target_os = "linux")]
use rustix::process::{pidfd_open, Pid, PidfdFlags};
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
#[derive(Debug)]
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
    observation_deadline: Option<Instant>,
    census_entries: usize,
}

impl MemoryObserver {
    /// Check mechanism availability before the backend is spawned.
    #[cfg(not(target_os = "linux"))]
    pub(super) fn prepare(_root: &Path) -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "tree resident-memory observation requires Linux procfs",
        ))
    }

    #[cfg(target_os = "linux")]
    pub(super) fn prepare(root: &Path) -> io::Result<Self> {
        let observer = Self {
            observation_deadline: None,
            census_entries: usize::MAX,
            root: root.to_path_buf(),
            known: BTreeMap::new(),
            observation: MemoryObservation {
                mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                peak_resident_bytes: None,
            },
        };
        // Reading a directory alone is insufficient: verify both ancestry and resident bytes.
        let own = std::process::id();
        let own_process = parse_process(&read_proc_file(root.join(own.to_string()).join("stat"))?)?;
        observer
            .resident_bytes(own, own_process.start)?
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::Unsupported, "procfs caller identity changed")
            })?;
        // CONFIG_PROC_CHILDREN is required: an empty file is valid, an absent live task's
        // file is not. Probe before dispatch, then keep checking this during owned traversal.
        let own_tasks = root.join(own.to_string()).join("task");
        for task in fs::read_dir(&own_tasks)? {
            let task = task?;
            read_task_children(&task.path())?;
        }
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

    #[cfg(target_os = "linux")]
    pub(super) fn bind_root(&mut self, pid: u32, start: u64) -> io::Result<()> {
        let process = parse_process(&read_proc_file(
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
        if !self.known.contains_key(&pid) {
            self.check_census_size(
                self.known
                    .len()
                    .checked_add(1)
                    .ok_or_else(census_overflow)?,
            )?;
        }
        self.known.insert(pid, start);
        Ok(())
    }

    /// Bound census records against the original product ceiling. This is a finite workspace
    /// admission bound, not an RSS estimate: the ledger still measures actual O allocations.
    /// Each individual collection is bounded; duplicate ancestry cannot grow the pending queue
    /// without limit before its next observation. Small ceilings still admit the root observation
    /// so independently measured exhaustion keeps its existing classification priority.
    #[cfg(target_os = "linux")]
    pub(super) fn restrict_census(&mut self, ceiling: std::num::NonZeroU64) -> io::Result<()> {
        let record_bytes =
            u64::try_from(std::mem::size_of::<Process>()).map_err(io::Error::other)?;
        let entries = ceiling.get().checked_div(record_bytes).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "census record has no finite size",
            )
        })?;
        self.census_entries = usize::try_from(entries.max(1)).map_err(io::Error::other)?;
        self.check_census_size(self.known.len())
    }

    fn check_census_size(&self, entries: usize) -> io::Result<()> {
        if entries > self.census_entries {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "owned census exceeds its finite workspace admission bound",
            ))
        } else {
            Ok(())
        }
    }

    /// Bind the actual prepared outer PID1 as the whole contained-tree root. L is outside this
    /// private proc view and is sampled separately from its retained original files/pidfd.
    #[cfg(target_os = "linux")]
    pub(super) fn bind_outer(
        &mut self,
        outer: &super::outer_setup::PreparedOuter<'_>,
    ) -> io::Result<()> {
        outer.require_creator_live().map_err(io::Error::other)?;
        let pid = std::process::id();
        if pid != 1 {
            return Err(io::Error::other(
                "outer observation is not running in actual PID1",
            ));
        }
        let process = parse_process(&read_proc_file(self.root.join("1/stat"))?)?;
        self.bind_root(pid, process.start)?;
        outer.require_creator_live().map_err(io::Error::other)
    }

    pub(super) fn observation(&self) -> MemoryObservation {
        self.observation.clone()
    }

    /// O retains the original deadline through each census/RSS operation. A failed partial
    /// traversal supplies no sample. Resetting this private guard changes neither the original
    /// deadline nor the owner's work/settlement allocation, and does not alter prior peak data.
    #[cfg(target_os = "linux")]
    pub(super) fn observe_before(&mut self, launcher: u32, deadline: Instant) -> io::Result<u64> {
        self.observation_deadline = Some(deadline);
        let result = self.observe(launcher);
        self.observation_deadline = None;
        result
    }

    fn check_observation_deadline(&self) -> io::Result<()> {
        if self
            .observation_deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "original census deadline elapsed",
            ))
        } else {
            Ok(())
        }
    }

    pub(super) fn observe(&mut self, launcher: u32) -> io::Result<u64> {
        self.check_observation_deadline()?;
        let mut processes = self.processes(launcher)?;
        // One retained-census membership lookup per known identity; an O(N^2) rescan could
        // consume the original window before any fresh RSS/control/collector progress.
        processes.sort_unstable_by_key(|process| (process.pid, process.start));
        self.check_observation_deadline()?;
        let deadline = self.observation_deadline;
        self.known.retain(|pid, start| {
            // On expiry preserve remaining identities, then refuse this incomplete sample.
            // Retention does not establish observed zero, a new peak or permission to dispatch.
            deadline.is_some_and(|deadline| Instant::now() >= deadline)
                || *pid == launcher
                || processes
                    .binary_search_by_key(&(*pid, *start), |process| (process.pid, process.start))
                    .is_ok()
        });
        self.check_observation_deadline()?;
        for process in &processes {
            self.check_observation_deadline()?;
            if !self.known.contains_key(&process.pid) {
                self.check_census_size(
                    self.known
                        .len()
                        .checked_add(1)
                        .ok_or_else(census_overflow)?,
                )?;
            }
            self.known.insert(process.pid, process.start);
        }
        let mut total = 0u64;
        let mut observed = false;
        for (pid, start) in &self.known {
            self.check_observation_deadline()?;
            match self.resident_bytes(*pid, *start) {
                Ok(Some(bytes)) => {
                    total = total.checked_add(bytes).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "owned resident-memory overflow")
                    })?;
                    observed = true;
                }
                Ok(None) => {}
                Err(error) if process_disappeared(&error) => {}
                Err(error) => return Err(error),
            }
        }
        self.check_observation_deadline()?;
        if observed {
            self.observation.peak_resident_bytes =
                Some(self.observation.peak_resident_bytes.unwrap_or(0).max(total));
        }
        Ok(total)
    }

    /// Walk only the owned namespace subtree, including children created by worker threads.
    fn processes(&self, launcher: u32) -> io::Result<Vec<Process>> {
        let mut pending: Vec<(u32, Option<(u32, u64)>)> = Vec::new();
        reserve_census_push(&mut pending, self.census_entries)?;
        pending.push((launcher, None));
        let mut seen = BTreeSet::new();
        let mut processes = Vec::new();
        let mut task_entries = 0usize;
        while let Some((pid, parent)) = pending.pop() {
            self.check_observation_deadline()?;
            if seen.contains(&pid) {
                continue;
            }
            self.check_census_size(seen.len().checked_add(1).ok_or_else(census_overflow)?)?;
            seen.insert(pid);
            let directory = self.root.join(pid.to_string());
            let process = match read_proc_file(directory.join("stat")) {
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
                    match read_proc_file(self.root.join(parent_pid.to_string()).join("stat")) {
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
                    match read_proc_file(directory.join("stat")) {
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
                self.check_observation_deadline()?;
                task_entries = task_entries.checked_add(1).ok_or_else(census_overflow)?;
                self.check_census_size(task_entries)?;
                let task = task?;
                match read_task_children(&task.path()) {
                    Ok(Some(children)) => {
                        for child in children.split_whitespace() {
                            self.check_observation_deadline()?;
                            reserve_census_push(&mut pending, self.census_entries)?;
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
                    Ok(None) => {}
                    Err(error) => return Err(error),
                }
            }
            reserve_census_push(&mut processes, self.census_entries)?;
            processes.push(process);
        }
        Ok(processes)
    }

    fn resident_bytes(&self, pid: u32, start: u64) -> io::Result<Option<u64>> {
        self.resident_bytes_with(pid, start, |task| fs::read_dir(task))
    }

    fn resident_bytes_with(
        &self,
        pid: u32,
        start: u64,
        read_tasks: impl FnOnce(&Path) -> io::Result<fs::ReadDir>,
    ) -> io::Result<Option<u64>> {
        self.check_observation_deadline()?;
        let directory = self.root.join(pid.to_string());
        // Pin the status file before rechecking identity. A later pid reuse cannot redirect
        // this open file to the replacement process; an exited process may instead yield ESRCH.
        let mut status_file = match fs::File::open(directory.join("status")) {
            Ok(file) => file,
            Err(error) if process_disappeared(&error) => {
                return Self::missing_status(&directory, start)
            }
            Err(error) => return Err(error),
        };
        let mut stat_file = match fs::File::open(directory.join("stat")) {
            Ok(file) => file,
            Err(error) if process_disappeared(&error) => return Ok(None),
            Err(error) => return Err(error),
        };
        let stat = match read_proc_record(&mut stat_file) {
            Ok(stat) => stat,
            Err(error) if process_disappeared(&error) => return Ok(None),
            Err(error) => return Err(error),
        };
        let process = parse_process(&stat)?;
        if process.start != start {
            return Ok(None);
        }
        let status = match read_proc_record(&mut status_file) {
            Ok(status) => status,
            Err(error) if process_disappeared(&error) => {
                return Self::missing_status(&directory, start)
            }
            Err(error) => return Err(error),
        };
        if let Some(bytes) = status_rss(&status)? {
            return Ok(Some(bytes));
        }
        let threads = status_threads(&status)?;
        if threads > 1 {
            // An exited leader can have no mm while workers still share the live address space.
            // Use one worker's RSS, never sum threads that share the same mm.
            let workers = match read_tasks(&directory.join("task")) {
                Ok(workers) => workers,
                Err(error) if process_disappeared(&error) => {
                    if !Self::still_live_with_identity(
                        &directory,
                        start,
                        &mut status_file,
                        &mut stat_file,
                    )? {
                        return Ok(None);
                    }
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("live worker RSS unavailable: {error}"),
                    ));
                }
                Err(error) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("live worker RSS unavailable: {error}"),
                    ))
                }
            };
            let mut released = 0u64;
            let mut worker_entries = 0usize;
            for worker in workers {
                self.check_observation_deadline()?;
                worker_entries = worker_entries.checked_add(1).ok_or_else(census_overflow)?;
                self.check_census_size(worker_entries)?;
                let worker = match worker {
                    Ok(worker) => worker,
                    Err(error) if process_disappeared(&error) => {
                        if !Self::still_live_with_identity(
                            &directory,
                            start,
                            &mut status_file,
                            &mut stat_file,
                        )? {
                            return Ok(None);
                        }
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("live worker enumeration failed: {error}"),
                        ));
                    }
                    Err(error) => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("live worker enumeration failed: {error}"),
                        ))
                    }
                };
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
                let Some(latest) = read_process_if_present(&directory.join("stat"))? else {
                    return Ok(None);
                };
                if latest.start != start {
                    return Ok(None);
                }
                let text = match read_proc_record(&mut pinned) {
                    Ok(text) => text,
                    Err(error) if process_disappeared(&error) => continue,
                    Err(error) => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("live worker RSS unavailable: {error}"),
                        ))
                    }
                };
                if let Some(bytes) = status_rss(&text)? {
                    return Ok(Some(bytes));
                }
                let task_stat = match read_proc_file(worker.path().join("stat")) {
                    Ok(stat) => parse_process(&stat)?,
                    Err(error) if process_disappeared(&error) => continue,
                    Err(error) => return Err(error),
                };
                if task_stat.virtual_bytes != 0 || task_stat.resident_pages != 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "live worker has no RSS",
                    ));
                }
                released = released.saturating_add(1);
            }
            let Some(fresh_stat) = read_process_if_present(&directory.join("stat"))? else {
                return Ok(None);
            };
            if fresh_stat.start != start {
                return Ok(None);
            }
            let fresh_status = match read_proc_file(directory.join("status")) {
                Ok(status) => status,
                Err(error) if process_disappeared(&error) => {
                    return Self::missing_status(&directory, start)
                }
                Err(error) => return Err(error),
            };
            let fresh_threads = status_threads(&fresh_status)?;
            if released > 0
                && released >= fresh_threads
                && fresh_stat.virtual_bytes == 0
                && fresh_stat.resident_pages == 0
            {
                return Ok(Some(0));
            }
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "live worker RSS unavailable",
            ));
        }
        // Linux can release task->mm before publishing a zombie state. Its status then has
        // no VmRSS; stat reports zero virtual bytes and resident pages. Verify both from a
        // fresh, identity-matched stat instead of treating an unexplained missing field as zero.
        let Some(latest) = read_process_if_present(&directory.join("stat"))? else {
            return Ok(None);
        };
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

    fn still_live_with_identity(
        directory: &Path,
        start: u64,
        status_file: &mut fs::File,
        stat_file: &mut fs::File,
    ) -> io::Result<bool> {
        // Read the pinned files again: a path now naming a replacement cannot make the old
        // process appear live. Then check the current path, which vanishes once it is reaped.
        for file in [&mut *status_file, &mut *stat_file] {
            match file.seek(SeekFrom::Start(0)) {
                Ok(_) => {}
                Err(error) if process_disappeared(&error) => return Ok(false),
                Err(error) => return Err(error),
            }
        }
        let _status = match read_proc_record(status_file) {
            Ok(bytes) if bytes.is_empty() => {
                return Self::empty_pinned_process_file(directory, start)
            }
            Ok(bytes) => bytes,
            Err(error) if process_disappeared(&error) => return Ok(false),
            Err(error) => return Err(error),
        };
        let stat = match read_proc_record(stat_file) {
            Ok(bytes) if bytes.is_empty() => {
                return Self::empty_pinned_process_file(directory, start)
            }
            Ok(bytes) => bytes,
            Err(error) if process_disappeared(&error) => return Ok(false),
            Err(error) => return Err(error),
        };
        if parse_process(&stat)?.start != start {
            return Ok(false);
        }
        let Some(current) = read_process_if_present(&directory.join("stat"))? else {
            return Ok(false);
        };
        Ok(current.start == start)
    }

    fn empty_pinned_process_file(directory: &Path, start: u64) -> io::Result<bool> {
        let current = read_process_if_present(&directory.join("stat"))?;
        if current.is_some_and(|process| process.start == start) {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "live pinned procfs identity unavailable",
            ))
        } else {
            Ok(false)
        }
    }

    fn missing_status(directory: &Path, start: u64) -> io::Result<Option<u64>> {
        if read_process_if_present(&directory.join("stat"))?
            .is_some_and(|process| process.start == start)
        {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "live process status unavailable",
            ))
        } else {
            Ok(None)
        }
    }
}

/// L observations retained before O replaces procfs. O cannot reopen L through its private proc.
/// Bootstrap must authenticate the actual L creator pin and these originally opened descriptors.
#[cfg(target_os = "linux")]
pub(super) struct LauncherMemory {
    pin: std::os::fd::OwnedFd,
    stat: fs::File,
    status: fs::File,
    pid: u32,
    start: u64,
}

#[cfg(target_os = "linux")]
impl LauncherMemory {
    pub(super) fn bind(
        pin: std::os::fd::OwnedFd,
        mut stat: fs::File,
        status: fs::File,
    ) -> io::Result<Self> {
        super::creator::require_live(&pin)?;
        let identity = parse_process(&read_pinned_role_file(&mut stat)?)?;
        let mut observation = Self {
            pin,
            stat,
            status,
            pid: identity.pid,
            start: identity.start,
        };
        observation.sample()?;
        Ok(observation)
    }

    /// Single-thread L's actual retained identity must remain observable; missing RSS is never
    /// zero unless a fresh identity-matched stat proves that its address space was released.
    pub(super) fn sample(&mut self) -> io::Result<u64> {
        super::creator::require_live(&self.pin)?;
        let before = parse_process(&read_pinned_role_file(&mut self.stat)?)?;
        let status = read_pinned_role_file(&mut self.status)?;
        let after = parse_process(&read_pinned_role_file(&mut self.stat)?)?;
        if before.pid != self.pid
            || after.pid != self.pid
            || before.start != self.start
            || after.start != self.start
            || status_role_identity(&status, b"Pid:")? != self.pid
            || status_role_identity(&status, b"Tgid:")? != self.pid
            || status_threads(&status)? != 1
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "launcher identity or single-thread observation changed",
            ));
        }
        super::creator::require_live(&self.pin)?;
        if let Some(bytes) = status_rss(&status)? {
            return Ok(bytes);
        }
        if after.virtual_bytes == 0 && after.resident_pages == 0 {
            return Ok(0);
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "live launcher RSS unavailable",
        ))
    }
}

#[cfg(target_os = "linux")]
fn status_role_identity(status: &[u8], key: &[u8]) -> io::Result<u32> {
    let malformed = || {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "retained launcher status identity is unavailable or ambiguous",
        )
    };
    let mut fields = status
        .split(|byte| *byte == b'\n')
        .filter_map(|line| line.strip_prefix(key));
    let value = fields.next().ok_or_else(malformed)?;
    if fields.next().is_some() {
        return Err(malformed());
    }
    std::str::from_utf8(value)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .filter(|value| *value != 0)
        .ok_or_else(malformed)
}

fn census_overflow() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "owned census size overflow")
}

/// Grow only within the finite record bound, using fallible geometric reservation. A refused
/// push supplies no partial census, RSS peak or permission to expose the backend writer.
fn reserve_census_push<T>(records: &mut Vec<T>, limit: usize) -> io::Result<()> {
    let required = records.len().checked_add(1).ok_or_else(census_overflow)?;
    if required > limit {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "owned census exceeds its finite workspace admission bound",
        ));
    }
    if required > records.capacity() {
        let capacity = records
            .capacity()
            .checked_mul(2)
            .unwrap_or(limit)
            .max(required)
            .min(limit);
        records
            .try_reserve_exact(
                capacity
                    .checked_sub(records.len())
                    .ok_or_else(census_overflow)?,
            )
            .map_err(io::Error::other)?;
        if records.capacity() > limit {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "census allocation exceeds its finite reservation bound",
            ));
        }
    }
    Ok(())
}

// Proc records are untrusted observation input. Refuse an oversized record rather than
// allocating until EOF, accepting a truncated ancestry/RSS sample, or substituting zero.
// This bounds one record only; the O owner separately bounds its census workspace.
const PROC_RECORD_BYTES: usize = 16_384;

fn read_proc_file(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    read_proc_record(&mut fs::File::open(path)?)
}

fn read_proc_record(file: &mut fs::File) -> io::Result<Vec<u8>> {
    let limit = PROC_RECORD_BYTES.checked_add(1).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "procfs record bound overflow")
    })?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(limit).map_err(io::Error::other)?;
    file.take(u64::try_from(limit).map_err(io::Error::other)?)
        .read_to_end(&mut bytes)?;
    if bytes.len() > PROC_RECORD_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "procfs observation record exceeds its finite input bound",
        ));
    }
    Ok(bytes)
}

#[cfg(target_os = "linux")]
fn read_pinned_role_file(file: &mut fs::File) -> io::Result<Vec<u8>> {
    file.seek(SeekFrom::Start(0))?;
    let bytes = read_proc_record(file)?;
    if bytes.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "retained launcher observation missing",
        ));
    }
    Ok(bytes)
}

fn read_process_if_present(path: &Path) -> io::Result<Option<Process>> {
    match read_proc_file(path) {
        Ok(stat) => parse_process(&stat).map(Some),
        Err(error) if process_disappeared(&error) => Ok(None),
        Err(error) => Err(error),
    }
}

fn status_rss(status: &[u8]) -> io::Result<Option<u64>> {
    for line in status.split(|byte| *byte == b'\n') {
        if let Some(rss) = line.strip_prefix(b"VmRSS:") {
            let rss = std::str::from_utf8(rss)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
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

fn status_threads(status: &[u8]) -> io::Result<u64> {
    status
        .split(|byte| *byte == b'\n')
        .find_map(|line| line.strip_prefix(b"Threads:"))
        .and_then(|value| std::str::from_utf8(value).ok())
        .and_then(|value| value.trim().parse::<u64>().ok())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "procfs has no thread count"))
}

fn read_task_children(task: &Path) -> io::Result<Option<String>> {
    match read_proc_file(task.join("children")) {
        Ok(children) => String::from_utf8(children)
            .map(Some)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error)),
        Err(error) if process_disappeared(&error) => match fs::metadata(task) {
            Err(error) if process_disappeared(&error) => Ok(None),
            Err(error) => Err(error),
            Ok(_) => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "live task children observation unavailable",
            )),
        },
        Err(error) => Err(error),
    }
}

fn process_disappeared(error: &io::Error) -> bool {
    error.kind() == io::ErrorKind::NotFound
        || error.raw_os_error() == Some(rustix::io::Errno::SRCH.raw_os_error())
}

/// Reuse the byte-safe proc-stat identity decoder for the retained caller's actual INIT claim.
#[cfg(target_os = "linux")]
pub(super) fn process_start_record(record: &[u8]) -> io::Result<u64> {
    parse_process(record).map(|process| process.start)
}

fn parse_process(text: &[u8]) -> io::Result<Process> {
    let malformed = || {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid procfs process ancestry",
        )
    };
    let separator = text
        .iter()
        .position(|byte| *byte == b' ')
        .ok_or_else(malformed)?;
    let pid = std::str::from_utf8(text.get(..separator).ok_or_else(malformed)?)
        .map_err(|_| malformed())?;
    // comm is opaque bytes and can contain spaces, ')' and non-UTF8 program names. Only
    // the numeric identity/RSS fields after its final ')' require UTF8/ASCII decoding.
    let closing = text
        .iter()
        .rposition(|byte| *byte == b')')
        .ok_or_else(malformed)?;
    let tail = std::str::from_utf8(text.get(closing + 1..).ok_or_else(malformed)?)
        .map_err(|_| malformed())?;
    let mut fields = tail.split_whitespace();
    fields.next().ok_or_else(malformed)?;
    let parent: u32 = fields
        .next()
        .and_then(|value| value.parse().ok())
        .ok_or_else(malformed)?;
    // Linux leaves this unused group field at -1 when an exiting task has no sighand.
    // Ownership is still checked from the PID, start tick and parent, not this group value.
    let _group: i32 = fields
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

    /// Trace: FR-028-AC-21, FR-034-AC-17
    #[test]
    fn opaque_process_names_preserve_identity_and_rss_but_invalid_numbers_refuse() {
        let mut fields = ["0"; 22];
        fields[0] = "S";
        fields[1] = "1";
        fields[2] = "42";
        fields[19] = "200";
        fields[20] = "1048576";
        fields[21] = "16";
        let mut bytes = b"42 (raw-\xff ) name) ".to_vec();
        bytes.extend_from_slice(fields.join(" ").as_bytes());
        let process = parse_process(&bytes).unwrap();
        assert_eq!(process.pid, 42);
        assert_eq!(process.parent, 1);
        assert_eq!(process.start, 200);
        assert_eq!(process.virtual_bytes, 1048576);
        assert_eq!(process.resident_pages, 16);
        assert_eq!(
            status_rss(b"Name:\traw-\xff\nVmRSS:\t64 kB\nThreads:\t1\n").unwrap(),
            Some(64 * 1024)
        );
        assert_eq!(
            status_threads(b"Name:\traw-\xff\nThreads:\t1\n").unwrap(),
            1
        );
        bytes.push(0xff);
        assert_eq!(
            parse_process(&bytes).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            status_rss(b"Name:\tvalid\nVmRSS:\t\xff kB\n")
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            status_threads(b"Threads:\t\xff\n").unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }

    /// Trace: FR-028-AC-21.
    #[test]
    fn released_sighand_signed_group_preserves_owned_rss_and_identity_checks() {
        let root = crate::kani::test_support::discover_scratch("memory-signed-group");
        let directory = root.join("42");
        fs::create_dir_all(directory.join("task/42")).unwrap();
        fs::write(directory.join("task/42/children"), []).unwrap();
        let mut fields = ["0"; 22];
        fields[0] = "S";
        fields[1] = "1";
        fields[2] = "42";
        fields[19] = "200";
        fields[20] = "1048576";
        fields[21] = "16";
        let stat = |fields: &[&str]| format!("42 (owned) {}", fields.join(" "));
        fs::write(directory.join("stat"), stat(&fields)).unwrap();
        fs::write(directory.join("status"), "VmRSS:\t64 kB\nThreads:\t1\n").unwrap();
        let mut observer = MemoryObserver {
            observation_deadline: None,
            census_entries: usize::MAX,
            root: root.clone(),
            known: BTreeMap::from([(42, 200)]),
            observation: MemoryObservation {
                mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                peak_resident_bytes: None,
            },
        };
        assert_eq!(observer.observe(42).unwrap(), 64 * 1024);

        // do_task_stat can keep pgid=-1 when the exiting task has lost its sighand.
        fields[0] = "X";
        fields[1] = "0";
        fields[2] = "-1";
        fields[20] = "0";
        fields[21] = "0";
        fs::write(directory.join("stat"), stat(&fields)).unwrap();
        fs::write(directory.join("status"), "State:\tX (dead)\nThreads:\t1\n").unwrap();
        assert_eq!(observer.observe(42).unwrap(), 0);
        assert_eq!(observer.observation().peak_resident_bytes, Some(64 * 1024));

        fields[1] = "not-a-parent";
        fs::write(directory.join("stat"), stat(&fields)).unwrap();
        assert_eq!(
            observer.observe(42).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fields[1] = "0";
        fields[19] = "201";
        fs::write(directory.join("stat"), stat(&fields)).unwrap();
        assert_eq!(
            observer.observe(42).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }

    /// Trace: FR-028-AC-21, FR-034-AC-32.
    #[cfg(target_os = "linux")]
    #[test]
    fn oversized_census_refuses_without_accepting_partial_rss_or_changing_peak() {
        let root = crate::kani::test_support::discover_scratch("memory-census-bound");
        let directory = root.join("42");
        fs::create_dir_all(directory.join("task/42")).unwrap();
        let mut fields = ["0"; 22];
        fields[0] = "S";
        fields[1] = "1";
        fields[19] = "200";
        fields[20] = "1048576";
        fields[21] = "16";
        fs::write(
            directory.join("stat"),
            format!("42 (owned) {}", fields.join(" ")),
        )
        .unwrap();
        fs::write(directory.join("status"), "VmRSS:\t64 kB\nThreads:\t1\n").unwrap();
        let children = directory.join("task/42/children");
        fs::write(&children, []).unwrap();
        let mut observer = MemoryObserver {
            observation_deadline: None,
            census_entries: usize::MAX,
            root: root.clone(),
            known: BTreeMap::from([(42, 200)]),
            observation: MemoryObservation {
                mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                peak_resident_bytes: None,
            },
        };
        observer
            .restrict_census(
                std::num::NonZeroU64::new(u64::try_from(std::mem::size_of::<Process>()).unwrap())
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(observer.observe(42).unwrap(), 64 * 1024);
        // Repeated ancestry cannot inflate the pending queue before deduplication.
        fs::write(&children, "43 43").unwrap();
        assert_eq!(
            observer.observe(42).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(observer.observation().peak_resident_bytes, Some(64 * 1024));
        fs::write(&children, []).unwrap();
        fs::create_dir_all(directory.join("task/43")).unwrap();
        fs::write(directory.join("task/43/children"), []).unwrap();
        assert_eq!(
            observer.observe(42).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(observer.observation().peak_resident_bytes, Some(64 * 1024));
        fs::remove_dir_all(root).unwrap();
    }

    /// Trace: FR-028-AC-21, FR-034-AC-32.
    #[test]
    fn oversized_proc_record_refuses_before_accepting_valid_prefix() {
        let root = crate::kani::test_support::discover_scratch("memory-record-bound");
        fs::create_dir_all(&root).unwrap();
        let path = root.join("status");
        let mut bytes = b"VmRSS:\t64 kB\nThreads:\t1\n".to_vec();
        bytes.resize(PROC_RECORD_BYTES, b' ');
        fs::write(&path, &bytes).unwrap();
        assert_eq!(
            status_rss(&read_proc_file(&path).unwrap()).unwrap(),
            Some(64 * 1024)
        );
        bytes.push(b' ');
        fs::write(&path, &bytes).unwrap();
        assert_eq!(
            read_proc_file(&path).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::write(root.join("children"), &bytes).unwrap();
        assert_eq!(
            read_task_children(&root).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::write(root.join("children"), []).unwrap();
        assert_eq!(read_task_children(&root).unwrap(), Some(String::new()));
        fs::remove_dir_all(root).unwrap();
    }

    /// Trace: FR-028-AC-21.
    // Requires actual Linux procfs/pidfd/namespace mechanism availability.
    #[cfg(target_os = "linux")]
    #[test]
    fn unavailable_children_observation_refuses_and_only_disappeared_tasks_are_skipped() {
        use std::os::unix::fs::symlink;
        let root = crate::kani::test_support::discover_scratch("memory-no-children");
        let own = std::process::id();
        let directory = root.join(own.to_string());
        let task = directory.join(format!("task/{own}"));
        fs::create_dir_all(&task).unwrap();
        for file in ["stat", "status"] {
            symlink(format!("/proc/{own}/{file}"), directory.join(file)).unwrap();
        }
        assert_eq!(
            MemoryObserver::prepare(&root).err().unwrap().kind(),
            io::ErrorKind::Unsupported
        );
        fs::write(task.join("children"), []).unwrap();
        assert!(MemoryObserver::prepare(&root).is_ok());
        fs::remove_file(task.join("children")).unwrap();
        assert_eq!(
            read_task_children(&task).unwrap_err().kind(),
            io::ErrorKind::Unsupported
        );
        fs::remove_dir(&task).unwrap();
        assert_eq!(read_task_children(&task).unwrap(), None);
        fs::remove_dir_all(root).unwrap();
    }

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
            observation_deadline: None,
            census_entries: usize::MAX,
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
            observation_deadline: None,
            census_entries: usize::MAX,
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
    // Requires actual Linux procfs/pidfd/namespace mechanism availability.
    #[cfg(target_os = "linux")]
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
            observation_deadline: None,
            census_entries: usize::MAX,
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
            observation_deadline: None,
            census_entries: usize::MAX,
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
        // Every remaining thread can pass exit_mm before Threads drops to one.
        fs::create_dir(directory.join("task/42")).unwrap();
        for tid in [42, 43] {
            fs::write(
                directory.join(format!("task/{tid}/status")),
                "Threads:\t2\n",
            )
            .unwrap();
            fs::write(
                directory.join(format!("task/{tid}/stat")),
                format!("{tid} (released) {}", fields.join(" ")),
            )
            .unwrap();
        }
        assert_eq!(observer.resident_bytes(42, 200).unwrap(), Some(0));
        // A remaining task with a live mm and unreadable RSS still fails closed.
        fields[20] = "4096";
        fields[21] = "1";
        fs::write(
            directory.join("task/43/stat"),
            format!("43 (live) {}", fields.join(" ")),
        )
        .unwrap();
        assert_eq!(
            observer.resident_bytes(42, 200).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(directory.join("task")).unwrap();
        assert_eq!(
            observer.resident_bytes(42, 200).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }

    /// Trace: FR-028-AC-21.
    #[test]
    fn vanished_process_during_worker_listing_is_skipped_but_live_missing_task_is_refused() {
        let root = crate::kani::test_support::discover_scratch("memory-worker-list-race");
        let directory = root.join("42");
        fs::create_dir(&directory).unwrap();
        let mut fields = ["0"; 22];
        fields[0] = "R";
        fields[1] = "1";
        fields[2] = "42";
        fields[19] = "200";
        let stat = format!("42 (exiting) {}", fields.join(" "));
        fs::write(directory.join("stat"), &stat).unwrap();
        fs::write(directory.join("status"), "Threads:\t2\n").unwrap();
        let observer = MemoryObserver {
            observation_deadline: None,
            census_entries: usize::MAX,
            root: root.clone(),
            known: BTreeMap::new(),
            observation: MemoryObservation {
                mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                peak_resident_bytes: None,
            },
        };
        assert_eq!(
            observer
                .resident_bytes_with(42, 200, |task| {
                    fs::remove_file(directory.join("stat"))?;
                    fs::read_dir(task)
                })
                .unwrap(),
            None
        );
        fs::write(directory.join("stat"), &stat).unwrap();
        fields[19] = "201";
        let replacement = format!("42 (replacement) {}", fields.join(" "));
        assert_eq!(
            observer
                .resident_bytes_with(42, 200, |task| {
                    fs::write(directory.join("stat"), &replacement)?;
                    fs::read_dir(task)
                })
                .unwrap(),
            None
        );
        fs::write(directory.join("stat"), &stat).unwrap();
        assert_eq!(
            observer
                .resident_bytes_with(42, 200, |task| fs::read_dir(task))
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidData
        );
        fs::remove_dir_all(root).unwrap();
    }
}

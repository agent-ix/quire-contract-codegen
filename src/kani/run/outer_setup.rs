//! Single-thread L namespace creation and actual outer-PID1 private-proc setup.
//!
//! These operations run in the matched helper roles, never in the multithreaded library caller.
//! Namespace descriptors bind verification to actual kernel objects. An unchanged mount namespace
//! refuses before any mount; no failure path changes host policy or selects another recipe.

use std::{
    fs::{self, File},
    io::{self, Read},
    os::{
        fd::{AsFd, BorrowedFd, OwnedFd},
        unix::fs::MetadataExt,
    },
};

use nix::{
    mount::{mount, MsFlags},
    sched::{unshare, CloneFlags},
};
use rustix::process::{pidfd_open, PidfdFlags};
use serde::{Deserialize, Serialize};

use super::{
    control::{ControlError, RoleEndpoint},
    creator,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct NamespaceIdentity {
    device: u64,
    inode: u64,
}

impl NamespaceIdentity {
    pub(super) fn read(path: &str) -> io::Result<Self> {
        let descriptor = File::open(path)?;
        let metadata = descriptor.metadata()?;
        Ok(Self {
            device: metadata.dev(),
            inode: metadata.ino(),
        })
    }
}

/// Original host-proc observations retained before O replaces its private /proc mount.
pub(super) struct LauncherNamespace {
    pub(super) original_mount: NamespaceIdentity,
    pub(super) original_pid: NamespaceIdentity,
    pub(super) original_network: NamespaceIdentity,
    pub(super) launcher_pin: OwnedFd,
    pub(super) launcher_stat: File,
    pub(super) launcher_status: File,
    original_proc: File,
    launcher_tasks: File,
}

impl LauncherNamespace {
    /// Recheck L's original task directory, even after O replaced their shared mount's proc view.
    pub(super) fn require_single_thread(&self) -> Result<(), SetupError> {
        let tasks = rustix::fs::Dir::read_from(&self.launcher_tasks)
            .map_err(|error| SetupError::TaskCensus(error.into()))?;
        let mut found = false;
        for task in tasks {
            let task = task.map_err(|error| SetupError::TaskCensus(error.into()))?;
            if matches!(task.file_name().to_bytes(), b"." | b"..") {
                continue;
            }
            if found {
                return Err(SetupError::NotSingleThreaded);
            }
            found = true;
        }
        if !found {
            return Err(SetupError::NotSingleThreaded);
        }
        Ok(())
    }

    /// Look up only the positively retained Child's PID namespace through the original proc root.
    pub(super) fn child_pid_namespace(&self, child: u32) -> io::Result<NamespaceIdentity> {
        self.child_namespace(child, "pid")
    }

    pub(super) fn child_network_namespace(&self, child: u32) -> io::Result<NamespaceIdentity> {
        self.child_namespace(child, "net")
    }

    fn child_namespace(&self, child: u32, name: &str) -> io::Result<NamespaceIdentity> {
        let descriptor = rustix::fs::openat(
            &self.original_proc,
            format!("{child}/ns/{name}"),
            rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::CLOEXEC,
            rustix::fs::Mode::empty(),
        )?;
        let metadata = rustix::fs::fstat(&descriptor)?;
        Ok(NamespaceIdentity {
            device: metadata.st_dev,
            inode: metadata.st_ino,
        })
    }
}

/// Minted only after actual outer-PID1, mapped identity, arm and private-proc verification.
pub(super) struct PreparedOuter<'bootstrap> {
    namespace: NamespaceIdentity,
    network: NamespaceIdentity,
    launcher: OwnedFd,
    outer: OwnedFd,
    bootstrap: &'bootstrap RoleEndpoint,
}

impl PreparedOuter<'_> {
    pub(super) fn namespace(&self) -> NamespaceIdentity {
        self.namespace
    }

    pub(super) fn network(&self) -> NamespaceIdentity {
        self.network
    }

    pub(super) fn descriptor(&self) -> BorrowedFd<'_> {
        self.outer.as_fd()
    }

    pub(super) fn bootstrap(&self) -> &RoleEndpoint {
        self.bootstrap
    }

    pub(super) fn require_creator_live(&self) -> Result<(), SetupError> {
        creator::require_live(&self.launcher).map_err(SetupError::Creator)?;
        self.bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(SetupError::Bootstrap)
    }
}

#[derive(Debug)]
pub(super) enum SetupError {
    Creator(io::Error),
    Bootstrap(ControlError),
    TaskCensus(io::Error),
    NotSingleThreaded,
    NamespaceIdentity(io::Error),
    LauncherIdentity(io::Error),
    Unshare(nix::errno::Errno),
    UnshareNetwork(nix::errno::Errno),
    NetworkNamespaceUnchanged,
    Setgroups(io::Error),
    UidMap(io::Error),
    GidMap(io::Error),
    MappingMismatch,
    MountNamespaceUnchanged,
    NotOuterInit,
    PidNamespaceUnchanged,
    Session(nix::errno::Errno),
    MakeMountsPrivate(nix::errno::Errno),
    MountProc(nix::errno::Errno),
    ProcIdentity(io::Error),
    ProcNamespaceMismatch,
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "outer namespace setup refused: {self:?}")
    }
}

impl std::error::Error for SetupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Creator(error)
            | Self::TaskCensus(error)
            | Self::NamespaceIdentity(error)
            | Self::LauncherIdentity(error)
            | Self::Setgroups(error)
            | Self::UidMap(error)
            | Self::GidMap(error)
            | Self::ProcIdentity(error) => Some(error),
            Self::Bootstrap(error) => Some(error),
            Self::Unshare(error)
            | Self::UnshareNetwork(error)
            | Self::Session(error)
            | Self::MakeMountsPrivate(error)
            | Self::MountProc(error) => Some(error),
            Self::NotSingleThreaded
            | Self::MappingMismatch
            | Self::MountNamespaceUnchanged
            | Self::NotOuterInit
            | Self::PidNamespaceUnchanged
            | Self::NetworkNamespaceUnchanged
            | Self::ProcNamespaceMismatch => None,
        }
    }
}

pub(super) fn require_single_thread() -> Result<(), SetupError> {
    let mut tasks = fs::read_dir("/proc/self/task").map_err(SetupError::TaskCensus)?;
    tasks
        .next()
        .transpose()
        .map_err(SetupError::TaskCensus)?
        .ok_or(SetupError::NotSingleThreaded)?;
    if tasks
        .next()
        .transpose()
        .map_err(SetupError::TaskCensus)?
        .is_some()
    {
        return Err(SetupError::NotSingleThreaded);
    }
    Ok(())
}

/// L arms and checks the actual creating-thread pin before setup and after mapping changes.
pub(super) fn prepare_launcher(
    creator_pin: &OwnedFd,
    bootstrap: &RoleEndpoint,
    caller_uid: u32,
    caller_gid: u32,
) -> Result<LauncherNamespace, SetupError> {
    require_single_thread()?;
    // No setuid/setgid helper transition is accepted as the original caller's mapping.
    if rustix::process::getuid().as_raw() != caller_uid
        || rustix::process::geteuid().as_raw() != caller_uid
        || rustix::process::getgid().as_raw() != caller_gid
        || rustix::process::getegid().as_raw() != caller_gid
    {
        return Err(SetupError::MappingMismatch);
    }
    creator::arm_parent_death(creator_pin).map_err(SetupError::Creator)?;
    bootstrap
        .transport()
        .refuse_observable_eof()
        .map_err(SetupError::Bootstrap)?;
    let original_mount =
        NamespaceIdentity::read("/proc/self/ns/mnt").map_err(SetupError::NamespaceIdentity)?;
    let original_pid =
        NamespaceIdentity::read("/proc/self/ns/pid").map_err(SetupError::NamespaceIdentity)?;
    let original_network =
        NamespaceIdentity::read("/proc/self/ns/net").map_err(SetupError::NamespaceIdentity)?;
    let launcher_pin = pidfd_open(rustix::process::getpid(), PidfdFlags::NONBLOCK)
        .map_err(|error| SetupError::LauncherIdentity(error.into()))?;
    let launcher_stat = File::open("/proc/self/stat").map_err(SetupError::LauncherIdentity)?;
    let launcher_status = File::open("/proc/self/status").map_err(SetupError::LauncherIdentity)?;
    // L and O initially share their NEWNS. O's fresh proc mount cannot redirect these retained
    // L-only directory descriptions to O's PID view; neither directory is sent to O/I/backend.
    let original_proc = File::open("/proc").map_err(SetupError::LauncherIdentity)?;
    let launcher_tasks = File::open("/proc/self/task").map_err(SetupError::LauncherIdentity)?;
    unshare(CloneFlags::CLONE_NEWUSER | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS)
        .map_err(SetupError::Unshare)?;
    let current_mount =
        NamespaceIdentity::read("/proc/self/ns/mnt").map_err(SetupError::NamespaceIdentity)?;
    if current_mount == original_mount {
        return Err(SetupError::MountNamespaceUnchanged);
    }
    fs::write("/proc/self/setgroups", b"deny\n").map_err(SetupError::Setgroups)?;
    fs::write("/proc/self/uid_map", format!("0 {caller_uid} 1\n")).map_err(SetupError::UidMap)?;
    fs::write("/proc/self/gid_map", format!("0 {caller_gid} 1\n")).map_err(SetupError::GidMap)?;
    if read_small("/proc/self/setgroups")
        .map_err(SetupError::Setgroups)?
        .trim_ascii()
        != b"deny"
        || !exact_mapping("/proc/self/uid_map", caller_uid).map_err(SetupError::UidMap)?
        || !exact_mapping("/proc/self/gid_map", caller_gid).map_err(SetupError::GidMap)?
        || rustix::process::getuid().as_raw() != 0
        || rustix::process::geteuid().as_raw() != 0
        || rustix::process::getgid().as_raw() != 0
        || rustix::process::getegid().as_raw() != 0
    {
        return Err(SetupError::MappingMismatch);
    }
    // Mapping/credential changes can clear PDEATH. Reinstallation plus the actual nonleader pin
    // closes the before-arm race; getppid(TGID) cannot substitute for that thread capability.
    creator::arm_parent_death(creator_pin).map_err(SetupError::Creator)?;
    bootstrap
        .transport()
        .refuse_observable_eof()
        .map_err(SetupError::Bootstrap)?;
    // Network isolation is a distinct mandatory capability, with its original syscall cause.
    // Mapping and creating-thread rearming have completed; O does not exist at this boundary.
    unshare(CloneFlags::CLONE_NEWNET).map_err(SetupError::UnshareNetwork)?;
    let current_network =
        NamespaceIdentity::read("/proc/self/ns/net").map_err(SetupError::NamespaceIdentity)?;
    if current_network == original_network {
        return Err(SetupError::NetworkNamespaceUnchanged);
    }
    creator::require_live(creator_pin).map_err(SetupError::Creator)?;
    bootstrap
        .transport()
        .refuse_observable_eof()
        .map_err(SetupError::Bootstrap)?;
    Ok(LauncherNamespace {
        original_mount,
        original_pid,
        original_network,
        launcher_pin,
        launcher_stat,
        launcher_status,
        original_proc,
        launcher_tasks,
    })
}

/// O arms actual L liveness before private mounts or ANY inner child creation.
pub(super) fn prepare_outer<'bootstrap>(
    launcher_pin: OwnedFd,
    bootstrap: &'bootstrap RoleEndpoint,
    original_mount: NamespaceIdentity,
    original_pid: NamespaceIdentity,
    original_network: NamespaceIdentity,
) -> Result<PreparedOuter<'bootstrap>, SetupError> {
    require_single_thread()?;
    if rustix::process::getpid().as_raw_nonzero().get() != 1
        || rustix::process::getuid().as_raw() != 0
        || rustix::process::geteuid().as_raw() != 0
        || rustix::process::getgid().as_raw() != 0
        || rustix::process::getegid().as_raw() != 0
    {
        return Err(SetupError::NotOuterInit);
    }
    // The outside parent appears as PID0 in O. The retained real L pidfd supplies liveness.
    creator::arm_parent_death(&launcher_pin).map_err(SetupError::Creator)?;
    bootstrap
        .transport()
        .refuse_observable_eof()
        .map_err(SetupError::Bootstrap)?;
    let current_mount =
        NamespaceIdentity::read("/proc/self/ns/mnt").map_err(SetupError::NamespaceIdentity)?;
    if current_mount == original_mount {
        return Err(SetupError::MountNamespaceUnchanged);
    }
    let current_pid =
        NamespaceIdentity::read("/proc/self/ns/pid").map_err(SetupError::NamespaceIdentity)?;
    if current_pid == original_pid {
        return Err(SetupError::PidNamespaceUnchanged);
    }
    let current_network =
        NamespaceIdentity::read("/proc/self/ns/net").map_err(SetupError::NamespaceIdentity)?;
    if current_network == original_network {
        return Err(SetupError::NetworkNamespaceUnchanged);
    }
    nix::unistd::setsid().map_err(SetupError::Session)?;
    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE | MsFlags::MS_REC,
        None::<&str>,
    )
    .map_err(SetupError::MakeMountsPrivate)?;
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOEXEC,
        None::<&str>,
    )
    .map_err(SetupError::MountProc)?;
    let proc_pid = NamespaceIdentity::read("/proc/1/ns/pid").map_err(SetupError::ProcIdentity)?;
    if proc_pid != current_pid {
        return Err(SetupError::ProcNamespaceMismatch);
    }
    let status = read_small("/proc/self/status").map_err(SetupError::ProcIdentity)?;
    let mut pid = None;
    let mut namespace_pids = None;
    for line in status.split(|byte| *byte == b'\n') {
        if let Some(value) = line.strip_prefix(b"Pid:") {
            if pid.replace(value.trim_ascii()).is_some() {
                return Err(SetupError::ProcNamespaceMismatch);
            }
        }
        if let Some(value) = line.strip_prefix(b"NSpid:") {
            if namespace_pids.replace(value.trim_ascii()).is_some() {
                return Err(SetupError::ProcNamespaceMismatch);
            }
        }
    }
    if pid != Some(b"1".as_slice()) || namespace_pids != Some(b"1".as_slice()) {
        return Err(SetupError::ProcNamespaceMismatch);
    }
    creator::require_live(&launcher_pin).map_err(SetupError::Creator)?;
    bootstrap
        .transport()
        .refuse_observable_eof()
        .map_err(SetupError::Bootstrap)?;
    let outer = pidfd_open(rustix::process::getpid(), PidfdFlags::NONBLOCK)
        .map_err(|error| SetupError::LauncherIdentity(error.into()))?;
    Ok(PreparedOuter {
        namespace: current_pid,
        network: current_network,
        launcher: launcher_pin,
        outer,
        bootstrap,
    })
}

fn read_small(path: &str) -> io::Result<Vec<u8>> {
    // Kernel map/status files are bounded inputs, not arbitrary paths supplied by the caller.
    let mut text = Vec::new();
    File::open(path)?.take(16_385).read_to_end(&mut text)?;
    if text.len() > 16_384 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "namespace metadata exceeds its bound",
        ));
    }
    Ok(text)
}

fn exact_mapping(path: &str, original: u32) -> io::Result<bool> {
    let mapping = read_small(path)?;
    let mapping = std::str::from_utf8(&mapping)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "mapping numbers are not ASCII"))?;
    let mut fields = mapping.split_whitespace();
    Ok(fields.next() == Some("0")
        && fields.next().and_then(|field| field.parse::<u32>().ok()) == Some(original)
        && fields.next() == Some("1")
        && fields.next().is_none())
}

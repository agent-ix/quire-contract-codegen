//! Per-run PID-namespace ownership: gated startup, claimed init, and confirmed teardown.
//!
//! Bubblewrap is mandatory. The caller's process never becomes a subreaper. Before dispatch,
//! the unreaped monitor pins the dedicated startup process group. After dispatch, the init
//! pidfd owns every descendant, including orphaned, session-escaped and nested-namespace tasks.
//!
//! The gate releases only trusted guardian bootstrap. Positive backend Dispatch belongs to the
//! separately authenticated caller lease; gate EOF itself supplies no backend authorization.

#[cfg(target_os = "linux")]
use command_fds::{CommandFdExt, FdMapping};
#[cfg(target_os = "linux")]
use rustix::{
    event::{poll, PollFd, PollFlags},
    pipe::{pipe_with, PipeFlags},
    process::{kill_process_group, pidfd_open, pidfd_send_signal, Pid, PidfdFlags, Signal},
    time::Timespec,
};
use serde::{Deserialize, Serialize};
#[cfg(any(test, target_os = "linux"))]
use std::process::Command;
use std::{
    ffi::{OsStr, OsString},
    path::Path,
};
#[cfg(target_os = "linux")]
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    os::fd::OwnedFd,
    path::PathBuf,
    process::Child,
    time::Instant,
};

#[cfg(all(test, target_os = "linux"))]
use std::time::Duration;

/// An internal command recipe with explicit environment inheritance and inherited stdin.
/// Capture owns stdout/stderr; no caller-configured stream is silently reconstructed.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct BackendCommand {
    program: OsString,
    arguments: Vec<OsString>,
    directory: Option<OsString>,
    environment: Vec<(OsString, OsString)>,
}

impl BackendCommand {
    pub(super) fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            program: program.as_ref().to_owned(),
            arguments: Vec::new(),
            directory: None,
            environment: std::env::vars_os().collect(),
        }
    }
    pub(super) fn arg(&mut self, argument: impl AsRef<OsStr>) -> &mut Self {
        self.arguments.push(argument.as_ref().to_owned());
        self
    }
    pub(super) fn args(&mut self, arguments: &[String]) -> &mut Self {
        self.arguments.extend(arguments.iter().map(OsString::from));
        self
    }
    pub(super) fn env(&mut self, name: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> &mut Self {
        self.environment
            .push((name.as_ref().to_owned(), value.as_ref().to_owned()));
        self
    }
    pub(super) fn current_dir(&mut self, directory: impl AsRef<Path>) -> &mut Self {
        self.directory = Some(directory.as_ref().as_os_str().to_owned());
        self
    }

    /// Actual retained recipe allocation in C, including unused Vec/OsString capacity. I's
    /// later Command construction is inside the owned RSS tree; this measures only C's recipe.
    #[cfg(target_os = "linux")]
    pub(super) fn reserved_bytes(&self) -> io::Result<u64> {
        let overflow = || {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "backend recipe allocation cannot be represented",
            )
        };
        let mut total = std::mem::size_of::<Self>()
            .checked_add(self.program.capacity())
            .and_then(|total| {
                self.arguments
                    .capacity()
                    .checked_mul(std::mem::size_of::<OsString>())
                    .and_then(|arguments| total.checked_add(arguments))
            })
            .and_then(|total| {
                self.environment
                    .capacity()
                    .checked_mul(std::mem::size_of::<(OsString, OsString)>())
                    .and_then(|environment| total.checked_add(environment))
            })
            .ok_or_else(overflow)?;
        for value in self.arguments.iter().chain(self.directory.iter()).chain(
            self.environment
                .iter()
                .flat_map(|(name, value)| [name, value]),
        ) {
            total = total.checked_add(value.capacity()).ok_or_else(overflow)?;
        }
        u64::try_from(total).map_err(|_| overflow())
    }

    /// Apply the same argv, working directory and inherited-environment additions to a command.
    #[cfg(any(test, target_os = "linux"))]
    pub(super) fn configure(&self, command: &mut Command) {
        command.args(&self.arguments);
        if let Some(directory) = &self.directory {
            command.current_dir(Path::new(directory));
        }
        command
            .env_clear()
            .envs(self.environment.iter().map(|(name, value)| (name, value)));
    }

    #[cfg(target_os = "linux")]
    pub(super) fn into_command(self) -> Command {
        let mut command = Command::new(&self.program);
        self.configure(&mut command);
        command
    }

    /// Real POSIX command capture for report tests, with no resource-enforcement attestation.
    #[cfg(test)]
    #[cfg(not(target_os = "linux"))]
    pub(super) fn report_test_command(&self) -> Command {
        let mut command = Command::new(&self.program);
        self.configure(&mut command);
        command
    }
}

#[cfg(target_os = "linux")]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StartupInfo {
    #[serde(rename = "child-pid")]
    child: u32,
    #[serde(rename = "mnt-namespace")]
    _mount: u64,
    #[serde(rename = "pid-namespace")]
    namespace: u64,
}

#[cfg(target_os = "linux")]
struct StartupReader {
    file: File,
    bytes: Vec<u8>,
    eof: bool,
}

#[cfg(target_os = "linux")]
impl StartupReader {
    const INPUT_BYTES: usize = 4096;

    fn prepare(file: File) -> io::Result<Self> {
        let flags = rustix::fs::fcntl_getfl(&file)?;
        rustix::fs::fcntl_setfl(&file, flags | rustix::fs::OFlags::NONBLOCK)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(
                Self::INPUT_BYTES
                    .checked_add(1)
                    .ok_or_else(|| unavailable("startup information bound overflow"))?,
            )
            .map_err(io::Error::other)?;
        Ok(Self {
            file,
            bytes,
            eof: false,
        })
    }

    /// At most one finite read, never a completion wait. Only actual writer EOF permits parsing.
    fn advance(&mut self) -> io::Result<Option<StartupInfo>> {
        if !self.eof {
            let mut chunk = [0; 1024];
            match self.file.read(&mut chunk) {
                Ok(0) => self.eof = true,
                Ok(length) => {
                    let next = self
                        .bytes
                        .len()
                        .checked_add(length)
                        .ok_or_else(|| unavailable("startup information size overflow"))?;
                    if next > Self::INPUT_BYTES {
                        return Err(unavailable(
                            "namespace startup information exceeds its bound",
                        ));
                    }
                    self.bytes.extend_from_slice(&chunk[..length]);
                    return Ok(None);
                }
                Err(error)
                    if matches!(
                        error.kind(),
                        io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                    ) =>
                {
                    return Ok(None)
                }
                Err(error) => return Err(error),
            }
        }
        serde_json::from_slice(&self.bytes)
            .map(Some)
            .map_err(|error| unavailable(format!("namespace startup failed: {error}")))
    }
}

#[cfg(target_os = "linux")]
struct InitClaim {
    pid: u32,
    start: u64,
    handle: OwnedFd,
    namespace: PathBuf,
}

/// O's retained actual monitor custody. Every post-spawn failure leaves the same unreaped Child
/// and any acquired pin in this owner; M/group exit never attests outer or unclaimed I teardown.
#[cfg(target_os = "linux")]
pub(super) struct OuterMonitorOwner {
    command: Option<Command>,
    child: Option<Child>,
    pin: Option<OwnedFd>,
    namespace: NamespaceOwner,
}

#[cfg(target_os = "linux")]
impl OuterMonitorOwner {
    pub(super) fn prepare(
        outer: &super::outer_setup::PreparedOuter<'_>,
        helper: &Path,
        bootstrap: super::control::RoleEndpoint,
        collector: &mut super::report_storage::ReportCollector,
        ledger: &super::resource_ledger::ResourceLedger,
    ) -> io::Result<Self> {
        let (command, namespace) =
            NamespaceOwner::prepare_outer(outer, helper, bootstrap, collector, ledger)?;
        Ok(Self {
            command: Some(command),
            child: None,
            pin: None,
            namespace,
        })
    }

    /// Original O authority/deadline are checked again immediately before the sole spawn attempt.
    /// No failure authorizes replacement, and no successful signal or Drop supplies settlement.
    pub(super) fn spawn(
        &mut self,
        outer: &super::outer_setup::PreparedOuter<'_>,
        deadline: Instant,
    ) -> io::Result<()> {
        outer.require_creator_live().map_err(io::Error::other)?;
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "nested monitor spawn deadline elapsed",
            ));
        }
        let mut command = self
            .command
            .take()
            .ok_or_else(|| unavailable("nested monitor spawn already attempted"))?;
        self.child = Some(command.spawn()?);
        // Actual Child custody precedes dropping the Command's intended-only report writer copy.
        drop(command);
        let child = self
            .child
            .as_ref()
            .ok_or_else(|| unavailable("nested monitor Child is absent"))?;
        let pid = valid_pid(child.id())?;
        self.pin = Some(pidfd_open(pid, PidfdFlags::NONBLOCK)?);
        self.namespace.attach(child)?;
        super::creator::require_live(
            self.pin
                .as_ref()
                .ok_or_else(|| unavailable("nested monitor pin is absent"))?,
        )?;
        outer.require_creator_live().map_err(io::Error::other)
    }

    /// Clone only this retained direct Child's actual capability for an authenticated O reply.
    pub(super) fn monitor_capability(&self) -> io::Result<OwnedFd> {
        let pin = self
            .pin
            .as_ref()
            .ok_or_else(|| unavailable("nested monitor pin is absent"))?;
        super::creator::require_live(pin)?;
        pin.try_clone()
    }

    pub(super) fn inner_capability(
        &self,
    ) -> io::Result<(u64, super::outer_setup::NamespaceIdentity, OwnedFd)> {
        let claim = self
            .namespace
            .init
            .as_ref()
            .ok_or_else(|| unavailable("inner INIT is unclaimed"))?;
        super::creator::require_live(&claim.handle)?;
        let namespace =
            super::outer_setup::NamespaceIdentity::read(&format!("/proc/{}/ns/pid", claim.pid))?;
        if fs::read_link(format!("/proc/{}/ns/pid", claim.pid))? != claim.namespace {
            return Err(unavailable("claimed inner namespace changed"));
        }
        let pin = claim.handle.try_clone()?;
        super::creator::require_live(&claim.handle)?;
        Ok((claim.start, namespace, pin))
    }

    pub(super) fn namespace(&mut self) -> &mut NamespaceOwner {
        &mut self.namespace
    }

    pub(super) fn child(&mut self) -> Option<&mut Child> {
        self.child.as_mut()
    }
}

/// Produced only after INIT identity and memory-root binding both succeed with the gate retained.
#[cfg(target_os = "linux")]
pub(super) struct GatedClaim {
    pid: u32,
    start: u64,
}

#[cfg(target_os = "linux")]
pub(super) struct NamespaceOwner {
    gate: Option<File>,
    #[cfg(target_os = "linux")]
    info: StartupReader,
    wrapper: Option<Pid>,
    init: Option<InitClaim>,
    cleaned: bool,
}

#[cfg(target_os = "linux")]
impl NamespaceOwner {
    #[cfg(feature = "guardian-test-support")]
    pub(super) fn fixture_claim(&self) -> io::Result<(u32, u64, &OwnedFd, &Path)> {
        let claim = self
            .init
            .as_ref()
            .ok_or_else(|| unavailable("fixture INIT is unclaimed"))?;
        Ok((claim.pid, claim.start, &claim.handle, &claim.namespace))
    }

    #[cfg(feature = "guardian-test-support")]
    pub(super) fn fixture_gate_retained(&self) -> bool {
        self.gate.is_some()
    }

    pub(super) fn prepare(
        helper: &Path,
        #[cfg(target_os = "linux")] endpoint: super::control::GuardianEndpoint,
    ) -> io::Result<(Command, Self)> {
        #[cfg(target_os = "linux")]
        let (gate_read, gate_write) = pipe_with(PipeFlags::CLOEXEC)?;
        #[cfg(target_os = "linux")]
        let (info_read, info_write) = pipe_with(PipeFlags::CLOEXEC)?;
        // Parent FDs remain CLOEXEC. The authoritative adapter maps only this child after
        // fork, with no allocations/locks; concurrent unrelated spawns inherit no controls.
        let mut command = Command::new("bwrap");
        command
            .args([
                "--unshare-user",
                "--unshare-pid",
                "--as-pid-1",
                "--new-session",
                "--die-with-parent",
                "--bind",
                "/",
                "/",
                "--dev-bind",
                "/dev",
                "/dev",
                "--proc",
                "/proc",
                "--info-fd",
            ])
            .arg("3")
            .arg("--block-fd")
            .arg("4")
            .arg("--")
            .arg(helper);
        #[cfg(target_os = "linux")]
        {
            command
                .fd_mappings(vec![
                    FdMapping {
                        parent_fd: endpoint.into_child_mapping(),
                        child_fd: 0,
                    },
                    FdMapping {
                        parent_fd: info_write,
                        child_fd: 3,
                    },
                    FdMapping {
                        parent_fd: gate_read,
                        child_fd: 4,
                    },
                ])
                .map_err(|error| {
                    unavailable(format!("namespace control mapping failed: {error}"))
                })?;
            Ok((
                command,
                Self {
                    gate: Some(File::from(gate_write)),
                    info: StartupReader::prepare(File::from(info_read))?,
                    wrapper: None,
                    init: None,
                    cleaned: false,
                },
            ))
        }
    }

    /// Attach while the monitor is still unreaped; gated init cannot yet change its group.
    /// Prepare nested M only inside the actual armed O, after the collector's checked writer
    /// exposure. The returned Command owns its child-only writer copy until actual spawn; O's
    /// retained monitor owner must drop that Command after storing its actual Child. This method
    /// supplies no teardown conclusion: C separately requires actual outer INIT termination.
    pub(super) fn prepare_outer(
        outer: &super::outer_setup::PreparedOuter<'_>,
        helper: &Path,
        bootstrap: super::control::RoleEndpoint,
        collector: &mut super::report_storage::ReportCollector,
        ledger: &super::resource_ledger::ResourceLedger,
    ) -> io::Result<(Command, Self)> {
        use std::os::unix::process::CommandExt;

        outer.require_creator_live().map_err(io::Error::other)?;
        if !helper.is_absolute() {
            return Err(unavailable("nested helper path is not absolute"));
        }
        let (gate_read, gate_write) = pipe_with(PipeFlags::CLOEXEC)?;
        let (info_read, info_write) = pipe_with(PipeFlags::CLOEXEC)?;
        // Only this ledger-authorized take transfers O's actual original report writer. Neither
        // C nor L has a writer, and the separate I lease arrives later over authenticated bootstrap.
        let writer = collector
            .take_spawn_writer(ledger)
            .map_err(io::Error::other)?;
        let mut command = Command::new("bwrap");
        command
            .args([
                "--unshare-user",
                "--unshare-pid",
                "--as-pid-1",
                "--new-session",
                "--bind",
                "/",
                "/",
                "--dev-bind",
                "/dev",
                "/dev",
                "--proc",
                "/proc",
                "--info-fd",
                "3",
                "--block-fd",
                "4",
                "--",
            ])
            .arg(helper)
            .process_group(0);
        command
            .fd_mappings(vec![
                FdMapping {
                    parent_fd: bootstrap.into_child_mapping(),
                    child_fd: 0,
                },
                FdMapping {
                    parent_fd: info_write,
                    child_fd: 3,
                },
                FdMapping {
                    parent_fd: gate_read,
                    child_fd: 4,
                },
                FdMapping {
                    parent_fd: writer,
                    child_fd: super::report_storage::REPORT_SLOT,
                },
            ])
            .map_err(|error| unavailable(format!("nested control mapping failed: {error}")))?;
        outer.require_creator_live().map_err(io::Error::other)?;
        Ok((
            command,
            Self {
                gate: Some(File::from(gate_write)),
                info: StartupReader::prepare(File::from(info_read))?,
                wrapper: None,
                init: None,
                cleaned: false,
            },
        ))
    }

    pub(super) fn attach(&mut self, child: &Child) -> io::Result<()> {
        self.wrapper = Some(valid_pid(child.id())?);
        Ok(())
    }

    /// Claim and bind the namespace INIT while retaining its bootstrap gate.
    #[cfg(target_os = "linux")]
    pub(super) fn claim_gated(
        &mut self,
        deadline: Option<Instant>,
        startup_deadline: Instant,
        observer: &mut super::memory::MemoryObserver,
    ) -> io::Result<GatedClaim> {
        let info = self.startup_information(deadline, startup_deadline)?;
        let claim = self.bind_gated_information(info, observer)?;
        if Instant::now() >= startup_deadline {
            return Err(startup_expiry(deadline));
        }
        Ok(claim)
    }

    fn bind_gated_information(
        &mut self,
        info: StartupInfo,
        observer: &mut super::memory::MemoryObserver,
    ) -> io::Result<GatedClaim> {
        self.claim_init(info.child, Some(info.namespace))?;
        let start = self
            .init
            .as_ref()
            .map(|claim| claim.start)
            .ok_or_else(|| unavailable("namespace init claim missing"))?;
        observer.bind_root(info.child, start)?;
        Ok(GatedClaim {
            pid: info.child,
            start,
        })
    }

    /// Release only trusted bootstrap, after the retained INIT claim and observer binding.
    pub(super) fn release_bootstrap_gate(
        &mut self,
        claim: GatedClaim,
        deadline: Option<Instant>,
        startup_deadline: Instant,
    ) -> io::Result<u32> {
        if !self
            .init
            .as_ref()
            .is_some_and(|init| init.pid == claim.pid && init.start == claim.start)
        {
            return Err(unavailable("namespace bootstrap requires its INIT claim"));
        }
        if Instant::now() >= startup_deadline {
            return Err(startup_expiry(deadline));
        }
        self.gate
            .as_mut()
            .ok_or_else(|| unavailable("namespace startup gate missing"))?
            .write_all(&[1])?;
        self.gate.take();
        Ok(claim.pid)
    }

    /// Validate a candidate only against the still-owned monitor and actual kernel identity.
    #[cfg(target_os = "linux")]
    fn claim_init(&mut self, child: u32, reported_namespace: Option<u64>) -> io::Result<()> {
        let handle = pidfd_open(valid_pid(child)?, PidfdFlags::empty())?;
        let directory = PathBuf::from("/proc").join(child.to_string());
        let status = fs::read_to_string(directory.join("status"))?;
        let parent = status
            .lines()
            .find_map(|line| line.strip_prefix("PPid:"))
            .and_then(|pid| pid.trim().parse::<i32>().ok());
        let nspid = status
            .lines()
            .find_map(|line| line.strip_prefix("NSpid:"))
            .and_then(|ids| ids.split_whitespace().last());
        let namespace = fs::read_link(directory.join("ns/pid"))?;
        let caller_namespace = fs::read_link("/proc/self/ns/pid")?;
        if parent != self.wrapper.map(|pid| pid.as_raw_nonzero().get())
            || nspid != Some("1")
            || namespace == caller_namespace
            || reported_namespace.is_some_and(|reported| namespace != format!("pid:[{reported}]"))
        {
            return Err(unavailable(
                "namespace init identity did not match its owned monitor",
            ));
        }
        let stat = fs::read_to_string(directory.join("stat"))?;
        let start = stat
            .rsplit_once(')')
            .and_then(|(_, fields)| fields.split_whitespace().nth(19))
            .and_then(|ticks| ticks.parse::<u64>().ok())
            .ok_or_else(|| unavailable("namespace init start identity missing"))?;
        let mut readiness = [PollFd::new(&handle, PollFlags::IN)];
        poll(
            &mut readiness,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )?;
        if !readiness[0].revents().is_empty() {
            return Err(unavailable("namespace init died before claim"));
        }
        // Retain the claimed handle before releasing the gate. No later pid reuse changes it.
        self.init = Some(InitClaim {
            pid: child,
            start,
            handle,
            namespace,
        });
        Ok(())
    }

    /// Revalidate the retained INIT chain and its new session after authenticated bootstrap.
    #[cfg(target_os = "linux")]
    pub(super) fn verify_ready(
        &self,
        sender: super::control::PeerCredentials,
        mapped_uid: u32,
    ) -> Result<(), ReadyIdentityError> {
        let claim = self.init.as_ref().ok_or(ReadyIdentityError::Unclaimed)?;
        if u32::try_from(sender.pid).ok() != Some(claim.pid)
            || sender.uid != rustix::process::getuid().as_raw()
            || sender.gid != rustix::process::getgid().as_raw()
        {
            return Err(ReadyIdentityError::SenderMismatch);
        }
        let directory = PathBuf::from("/proc").join(claim.pid.to_string());
        let status = bounded_proc_text(&directory.join("status"), 16_384)?;
        verify_ready_status(
            status.as_bytes(),
            self.wrapper
                .map(|pid| pid.as_raw_pid())
                .ok_or(ReadyIdentityError::ChainMismatch)?,
        )?;
        if fs::read_link(directory.join("ns/pid"))? != claim.namespace {
            return Err(ReadyIdentityError::ChainMismatch);
        }
        let stat = bounded_proc_text(&directory.join("stat"), 4096)?;
        verify_ready_stat(stat.as_bytes(), claim.pid, claim.start)?;
        let mappings = bounded_proc_text(&directory.join("uid_map"), 4096)?;
        verify_ready_mapping(mappings.as_bytes(), mapped_uid)?;
        if self.init_terminated()? {
            return Err(ReadyIdentityError::InitTerminated);
        }
        Ok(())
    }

    /// Pidfd readiness alone, rather than control EOF, confirms namespace INIT termination.
    #[cfg(target_os = "linux")]
    pub(super) fn init_terminated(&self) -> io::Result<bool> {
        let claim = self
            .init
            .as_ref()
            .ok_or_else(|| unavailable("INIT remains unclaimed"))?;
        let mut ready = [PollFd::new(&claim.handle, PollFlags::IN)];
        poll(&mut ready, Some(&Timespec::default()))?;
        if ready[0]
            .revents()
            .intersects(PollFlags::ERR | PollFlags::NVAL)
        {
            return Err(io::Error::other("INIT pidfd observation failed"));
        }
        Ok(ready[0].revents().contains(PollFlags::IN))
    }

    /// Recovery reads only the live retained monitor's bounded direct task children.
    #[cfg(target_os = "linux")]
    fn recover_owned_init(&mut self) -> io::Result<()> {
        use std::collections::BTreeSet;

        const TASKS: usize = 16;
        const CHILDREN: usize = 64;
        const CHILD_BYTES: u64 = 4096;
        let monitor = self
            .wrapper
            .ok_or_else(|| unavailable("owned monitor missing"))?;
        let monitor_handle = pidfd_open(monitor, PidfdFlags::empty())?;
        let mut live = [PollFd::new(&monitor_handle, PollFlags::IN)];
        poll(&mut live, Some(&Timespec::default()))?;
        if !live[0].revents().is_empty() {
            return Err(unavailable("owned monitor died before INIT recovery"));
        }
        let directory = PathBuf::from("/proc")
            .join(monitor.as_raw_nonzero().get().to_string())
            .join("task");
        let mut candidates = BTreeSet::new();
        for (index, task) in fs::read_dir(directory)?.enumerate() {
            if index == TASKS {
                return Err(unavailable("owned monitor task recovery exceeds bound"));
            }
            let mut bytes = Vec::new();
            File::open(task?.path().join("children"))?
                .take(CHILD_BYTES + 1)
                .read_to_end(&mut bytes)?;
            if u64::try_from(bytes.len()).map_err(|_| unavailable("owned children size invalid"))?
                > CHILD_BYTES
            {
                return Err(unavailable(
                    "owned monitor children recovery exceeds byte bound",
                ));
            }
            let text = std::str::from_utf8(&bytes)
                .map_err(|_| unavailable("owned monitor children encoding invalid"))?;
            for child in text.split_whitespace() {
                let child = child
                    .parse::<u32>()
                    .map_err(|_| unavailable("owned monitor child identity invalid"))?;
                candidates.insert(child);
                if candidates.len() > CHILDREN {
                    return Err(unavailable(
                        "owned monitor children recovery exceeds count bound",
                    ));
                }
            }
        }
        // The monitor may have exited during collection. Its reparented children confer no claim.
        poll(&mut live, Some(&Timespec::default()))?;
        if !live[0].revents().is_empty() {
            return Err(unavailable("owned monitor died during INIT recovery"));
        }
        for child in candidates {
            if self.claim_init(child, None).is_ok() {
                return Ok(());
            }
        }
        Err(unavailable(
            "owned INIT recovery unavailable; teardown unconfirmed",
        ))
    }

    #[cfg(target_os = "linux")]
    fn startup_information(
        &mut self,
        identity_deadline: Option<Instant>,
        startup_deadline: Instant,
    ) -> io::Result<StartupInfo> {
        self.read_info(startup_deadline).map_err(|error| {
            if error.kind() == io::ErrorKind::TimedOut {
                startup_expiry(identity_deadline)
            } else {
                error
            }
        })
    }

    #[cfg(target_os = "linux")]
    fn read_info(&mut self, deadline: Instant) -> io::Result<StartupInfo> {
        loop {
            if Instant::now() >= deadline {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "namespace startup exceeded its allowance",
                ));
            }
            if let Some(info) = self.info.advance()? {
                return Ok(info);
            }
            let mut ready = [PollFd::new(&self.info.file, PollFlags::IN)];
            poll(
                &mut ready,
                Some(&Timespec {
                    tv_sec: 0,
                    tv_nsec: 20_000_000,
                }),
            )?;
        }
    }

    /// O uses one nonblocking startup step per normal observer tick. Waiting for bwrap's info
    /// EOF may not suspend whole-tree accounting or report/control work until the setup cap.
    pub(super) fn claim_gated_tick(
        &mut self,
        startup_deadline: Instant,
        observer: &mut super::memory::MemoryObserver,
    ) -> io::Result<Option<GatedClaim>> {
        if self.init.is_some() {
            return Err(unavailable("namespace INIT already claimed"));
        }
        if Instant::now() >= startup_deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "namespace startup exceeded its allowance",
            ));
        }
        let Some(info) = self.info.advance()? else {
            return Ok(None);
        };
        let claim = self.bind_gated_information(info, observer)?;
        if Instant::now() >= startup_deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "namespace startup exceeded its allowance",
            ));
        }
        Ok(Some(claim))
    }

    /// Kill init on every conclusion, then confirm kernel namespace teardown before accepting it.
    pub(super) fn cleanup(&mut self) -> io::Result<()> {
        if self.cleaned {
            return Ok(());
        }
        let mut failure = None;
        #[cfg(target_os = "linux")]
        if self.init.is_none() && self.wrapper.is_some() {
            // Exact INIT recovery takes precedence while the bootstrap gate remains retained.
            if let Err(error) = self.recover_owned_init() {
                failure = Some(error);
            }
        }
        #[cfg(target_os = "linux")]
        if let Some(InitClaim { handle, .. }) = &self.init {
            if let Err(error) = pidfd_send_signal(handle, Signal::KILL) {
                if error != rustix::io::Errno::SRCH {
                    failure = Some(io::Error::from(error));
                }
            }
        }
        // Unclaimed startup cancellation signals its pinned group before any gate close.
        // With exact INIT ownership, leave the retained monitor alive to report/reap that INIT.
        if self.init.is_none() {
            if let Some(wrapper) = self.wrapper {
                if let Err(error) = kill_process_group(wrapper, Signal::KILL) {
                    if error != rustix::io::Errno::SRCH {
                        failure = Some(io::Error::from(error));
                    }
                }
            }
        }
        if let Some(InitClaim { handle, .. }) = &self.init {
            let mut ready = [PollFd::new(handle, PollFlags::IN)];
            poll(
                &mut ready,
                Some(&Timespec {
                    tv_sec: 5,
                    tv_nsec: 0,
                }),
            )?;
            if !ready[0].revents().contains(PollFlags::IN) {
                return Err(io::Error::other("namespace teardown was not confirmed"));
            }
        }
        if let Some(error) = failure {
            return Err(error);
        }
        self.gate.take();
        self.cleaned = true;
        Ok(())
    }
}

#[cfg(target_os = "linux")]
impl Drop for NamespaceOwner {
    fn drop(&mut self) {
        if self.cleaned {
            return;
        }
        // Abandonment initiates pinned cancellation without blocking Drop on observation.
        // Only explicit cleanup confirms teardown. The gate stays owned until after signals.
        #[cfg(target_os = "linux")]
        if let Some(InitClaim { handle, .. }) = &self.init {
            let _ = pidfd_send_signal(handle, Signal::KILL);
        }
        if let Some(wrapper) = self.wrapper {
            let _ = kill_process_group(wrapper, Signal::KILL);
        }
    }
}

/// Both actual ownership topologies authenticate the same unchanged guardian lease stages.
/// The legacy direct monitor and the new retained C→L→O→M→I owner supply genuine identities.
#[cfg(target_os = "linux")]
pub(super) trait GuardianIdentity {
    fn verify_ready(
        &self,
        sender: super::control::PeerCredentials,
        mapped_uid: u32,
    ) -> Result<(), ReadyIdentityError>;
}

#[cfg(target_os = "linux")]
impl GuardianIdentity for NamespaceOwner {
    fn verify_ready(
        &self,
        sender: super::control::PeerCredentials,
        mapped_uid: u32,
    ) -> Result<(), ReadyIdentityError> {
        NamespaceOwner::verify_ready(self, sender, mapped_uid)
    }
}

#[cfg(target_os = "linux")]
pub(super) fn verify_ready_status(record: &[u8], monitor: i32) -> Result<(), ReadyIdentityError> {
    let text = std::str::from_utf8(record).map_err(|_| ReadyIdentityError::ChainMismatch)?;
    let mut parent = None;
    let mut nspid = None;
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("PPid:") {
            if parent
                .replace(
                    value
                        .trim()
                        .parse::<i32>()
                        .map_err(|_| ReadyIdentityError::ChainMismatch)?,
                )
                .is_some()
            {
                return Err(ReadyIdentityError::ChainMismatch);
            }
        }
        if let Some(value) = line.strip_prefix("NSpid:") {
            if nspid.replace(value.split_whitespace().last()).is_some() {
                return Err(ReadyIdentityError::ChainMismatch);
            }
        }
    }
    if parent != Some(monitor) || nspid.flatten() != Some("1") {
        return Err(ReadyIdentityError::ChainMismatch);
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub(super) fn verify_ready_stat(
    record: &[u8],
    pid: u32,
    start: u64,
) -> Result<(), ReadyIdentityError> {
    let closing = record
        .iter()
        .rposition(|byte| *byte == b')')
        .ok_or(ReadyIdentityError::ChainMismatch)?;
    let tail = std::str::from_utf8(
        record
            .get(closing + 1..)
            .ok_or(ReadyIdentityError::ChainMismatch)?,
    )
    .map_err(|_| ReadyIdentityError::ChainMismatch)?;
    let mut fields = tail.split_whitespace();
    fields.next().ok_or(ReadyIdentityError::ChainMismatch)?;
    fields.next().ok_or(ReadyIdentityError::ChainMismatch)?;
    let group = fields.next().and_then(|value| value.parse::<u32>().ok());
    let session = fields.next().and_then(|value| value.parse::<u32>().ok());
    let tty = fields.next().and_then(|value| value.parse::<i64>().ok());
    if fields.nth(14).and_then(|value| value.parse::<u64>().ok()) != Some(start) {
        return Err(ReadyIdentityError::ChainMismatch);
    }
    if group != Some(pid)
        || session != Some(pid)
        || tty != Some(0)
        || group == u32::try_from(rustix::process::getpgrp().as_raw_pid()).ok()
        || session
            == u32::try_from(
                rustix::process::getsid(None)
                    .map_err(io::Error::from)?
                    .as_raw_pid(),
            )
            .ok()
    {
        return Err(ReadyIdentityError::SessionNotIsolated);
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub(super) fn verify_ready_mapping(
    record: &[u8],
    mapped_uid: u32,
) -> Result<(), ReadyIdentityError> {
    let text = std::str::from_utf8(record).map_err(|_| ReadyIdentityError::UidMappingMismatch)?;
    let host_uid = rustix::process::getuid().as_raw();
    let mut actual_mapping = None;
    for (index, mapping) in text.lines().enumerate() {
        if index == 32 {
            return Err(ReadyIdentityError::UidMappingMismatch);
        }
        let mut columns = mapping.split_whitespace();
        let inside = columns.next().and_then(|value| value.parse::<u32>().ok());
        let outside = columns.next().and_then(|value| value.parse::<u32>().ok());
        let length = columns.next().and_then(|value| value.parse::<u32>().ok());
        let (Some(inside), Some(outside), Some(length)) = (inside, outside, length) else {
            return Err(ReadyIdentityError::UidMappingMismatch);
        };
        if columns.next().is_some() {
            return Err(ReadyIdentityError::UidMappingMismatch);
        }
        if let Some(offset) = host_uid
            .checked_sub(outside)
            .filter(|offset| *offset < length)
        {
            if actual_mapping.replace(inside.checked_add(offset)).is_some() {
                return Err(ReadyIdentityError::UidMappingMismatch);
            }
        }
    }
    if actual_mapping.flatten() != Some(mapped_uid) {
        return Err(ReadyIdentityError::UidMappingMismatch);
    }
    Ok(())
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub(super) enum ReadyIdentityError {
    Unclaimed,
    SenderMismatch,
    ChainMismatch,
    SessionNotIsolated,
    UidMappingMismatch,
    InitTerminated,
    Observation(io::Error),
}

#[cfg(target_os = "linux")]
impl From<io::Error> for ReadyIdentityError {
    fn from(error: io::Error) -> Self {
        Self::Observation(error)
    }
}

#[cfg(target_os = "linux")]
impl std::fmt::Display for ReadyIdentityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Observation(error) => write!(formatter, "guardian identity observation: {error}"),
            other => write!(formatter, "{other:?}"),
        }
    }
}

#[cfg(target_os = "linux")]
impl std::error::Error for ReadyIdentityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Observation(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(target_os = "linux")]
fn bounded_proc_text(path: &Path, limit: usize) -> io::Result<String> {
    let mut bytes = Vec::new();
    File::open(path)?
        .take(u64::try_from(limit).map_err(|_| unavailable("procfs read bound invalid"))? + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > limit {
        return Err(unavailable("owned procfs identity exceeds read bound"));
    }
    String::from_utf8(bytes).map_err(|_| unavailable("owned procfs identity encoding invalid"))
}

#[cfg(target_os = "linux")]
fn startup_expiry(identity_deadline: Option<Instant>) -> io::Error {
    if identity_deadline.is_some_and(|deadline| Instant::now() >= deadline) {
        io::Error::new(
            io::ErrorKind::TimedOut,
            "identity wall-clock ceiling elapsed during namespace startup",
        )
    } else {
        unavailable("namespace startup allowance elapsed before the identity ceiling")
    }
}

#[cfg(target_os = "linux")]
fn unavailable(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::Unsupported, message.into())
}

#[cfg(target_os = "linux")]
fn valid_pid(pid: u32) -> io::Result<Pid> {
    i32::try_from(pid)
        .ok()
        .and_then(Pid::from_raw)
        .ok_or_else(|| unavailable("invalid namespace process id"))
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::*;
    use crate::kani::test_support::discover_scratch;
    use std::os::unix::process::CommandExt;
    use std::process::Stdio;

    /// Trace: FR-034-AC-34.
    #[test]
    fn startup_reader_returns_between_partial_reads_and_requires_actual_writer_eof() {
        let (reader, writer) = pipe_with(PipeFlags::CLOEXEC).unwrap();
        let mut reader = StartupReader::prepare(File::from(reader)).unwrap();
        let mut writer = File::from(writer);
        writer.write_all(b"{\"child-pid\":2,").unwrap();
        assert!(reader.advance().unwrap().is_none());
        assert!(reader.advance().unwrap().is_none());
        writer
            .write_all(b"\"mnt-namespace\":12,\"pid-namespace\":23}")
            .unwrap();
        assert!(reader.advance().unwrap().is_none());
        // Complete JSON is still not actual EOF while the one real writer remains open.
        assert!(reader.advance().unwrap().is_none());
        drop(writer);
        let info = reader.advance().unwrap().unwrap();
        assert_eq!(info.child, 2);
        assert_eq!(info.namespace, 23);
        assert_eq!(info._mount, 12);
    }

    fn ready(handle: &OwnedFd) -> bool {
        let mut fds = [PollFd::new(handle, PollFlags::IN)];
        poll(
            &mut fds,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }),
        )
        .unwrap();
        fds[0].revents().contains(PollFlags::IN)
    }

    fn wait_for(path: &Path) {
        let deadline = Instant::now() + Duration::from_secs(5);
        while !path.exists() {
            assert!(
                Instant::now() < deadline,
                "fixture handshake was not reached: {}",
                path.display()
            );
            std::thread::yield_now();
        }
    }

    fn children(pid: u32) -> Vec<u32> {
        fs::read_to_string(format!("/proc/{pid}/task/{pid}/children"))
            .unwrap()
            .split_whitespace()
            .map(|pid| pid.parse().unwrap())
            .collect()
    }

    fn child_with_nspid(children: &[u32], local_pid: &str) -> (Option<u32>, Vec<String>) {
        let mut found = None;
        let mut observed = Vec::with_capacity(children.len());
        for &pid in children {
            match fs::read_to_string(format!("/proc/{pid}/status")) {
                Ok(status) => {
                    let nspid = status.lines().find(|line| line.starts_with("NSpid:"));
                    observed.push(format!("host_pid={pid} {nspid:?}"));
                    if nspid.is_some_and(|ids| ids.split_whitespace().last() == Some(local_pid)) {
                        found = Some(pid);
                    }
                }
                Err(error) => observed.push(format!("host_pid={pid} status_error={error}")),
            }
        }
        (found, observed)
    }

    /// Trace: FR-028-AC-2, FR-028-AC-21.
    #[test]
    fn startup_cap_refuses_without_claiming_the_identity_wall_ceiling_elapsed() {
        let (_lease, endpoint) = super::super::control::private_pair().unwrap();
        let (_command, mut owner) =
            NamespaceOwner::prepare(Path::new("/unused-helper"), endpoint).unwrap();
        let allowance = Instant::now();
        let error = owner
            .startup_information(Some(allowance + Duration::from_secs(600)), allowance)
            .err()
            .unwrap();
        assert_eq!(error.kind(), io::ErrorKind::Unsupported);
        let error = owner
            .startup_information(Some(allowance), allowance)
            .err()
            .unwrap();
        assert_eq!(error.kind(), io::ErrorKind::TimedOut);
    }

    /// Trace: FR-028-AC-21.
    #[test]
    fn gated_startup_abort_kills_init_before_gate_eof_and_never_dispatches_backend() {
        let directory = discover_scratch("namespace-startup-abort");
        let marker = directory.join("backend-started");
        let mut recipe = BackendCommand::new("sh");
        recipe.args(&[
            "-c".to_owned(),
            "touch \"$1\"".to_owned(),
            "sh".to_owned(),
            marker.display().to_string(),
        ]);
        let (mut command, mut owner) = NamespaceOwner::prepare(recipe).unwrap();
        command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .process_group(0);
        let mut child = command.spawn().unwrap();
        drop(command);
        owner.attach(&child).unwrap();
        let info = owner
            .read_info(Instant::now() + Duration::from_secs(5))
            .unwrap();
        let init = pidfd_open(valid_pid(info.child).unwrap(), PidfdFlags::empty()).unwrap();
        assert!(
            children(info.child).is_empty(),
            "backend must remain gated before claim"
        );
        assert!(!marker.exists());
        // This is the pre-claim cancellation path used for malformed control information.
        owner.cleanup().unwrap();
        assert!(
            ready(&init),
            "startup init termination must be confirmed before return"
        );
        child.wait().unwrap();
        assert!(
            !marker.exists(),
            "gate EOF during abort must never execute backend"
        );
        fs::remove_dir_all(directory).unwrap();
    }

    /// Trace: FR-028-AC-21, FR-017-AC-24.
    #[test]
    fn completed_monitor_cleanup_kills_an_orphan_and_its_fork_after_the_last_sample() {
        let directory = discover_scratch("namespace-late-fork");
        let ready_file = directory.join("orphan-ready");
        let adopt = directory.join("adopt-now");
        let release = directory.join("fork-now");
        let forked = directory.join("forked");
        let script = r#"import os,pathlib,signal,sys
root=pathlib.Path(sys.argv[1])
intermediate=os.fork()
if intermediate:
    os.waitpid(intermediate,0)
    while not (root/'backend-exit').exists():
        os.sched_yield()
    os._exit(0)
if os.fork():
    # Keep the orphan attached to this parent until the caller has seen its ready file.
    while not (root/'adopt-now').exists():
        os.sched_yield()
    os._exit(0)
os.setsid()
# Rename publishes the complete PID before the caller can observe the ready path.
(root/'orphan-ready.tmp').write_text(str(os.getpid()))
(root/'orphan-ready.tmp').replace(root/'orphan-ready')
while os.getppid()!=1:
    os.sched_yield()
while not (root/'fork-now').exists():
    os.sched_yield()
if not os.fork():
    (root/'forked').touch()
    signal.pause()
signal.pause()
"#;
        let mut recipe = BackendCommand::new("python3");
        recipe.args(&[
            "-c".to_owned(),
            script.to_owned(),
            directory.display().to_string(),
        ]);
        let (mut command, mut owner) = NamespaceOwner::prepare(recipe).unwrap();
        command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .process_group(0);
        let mut child = command.spawn().unwrap();
        drop(command);
        owner.attach(&child).unwrap();
        let mut observer =
            super::super::memory::MemoryObserver::prepare(Path::new("/proc")).unwrap();
        let init = owner
            .dispatch(Some(Instant::now() + Duration::from_secs(5)), &mut observer)
            .unwrap();
        wait_for(&ready_file);
        observer.observe(init).unwrap(); // The later fork is deliberately not sampled.
        let local_orphan = fs::read_to_string(&ready_file).unwrap();
        assert!(
            !local_orphan.is_empty() && local_orphan.bytes().all(|byte| byte.is_ascii_digit()),
            "orphan-ready must contain a nonempty ASCII-digit NSpid, got {local_orphan:?}"
        );
        // The held parent constructs a ready-before-adoption window for this test.
        assert!(
            child_with_nspid(&children(init), &local_orphan).0.is_none(),
            "held orphan parent must prevent adoption before release"
        );
        fs::write(adopt, []).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        let orphan = loop {
            let (candidate, observed) = child_with_nspid(&children(init), &local_orphan);
            if let Some(orphan) = candidate {
                break orphan;
            }
            assert!(
                Instant::now() < deadline,
                "orphan NSpid {local_orphan} was not adopted by INIT {init}; observed children: {observed:?}"
            );
            std::thread::yield_now();
        };
        let orphan_handle = pidfd_open(valid_pid(orphan).unwrap(), PidfdFlags::empty()).unwrap();
        fs::write(release, []).unwrap();
        wait_for(&forked);
        let grandchild = children(orphan)[0];
        let grandchild_handle =
            pidfd_open(valid_pid(grandchild).unwrap(), PidfdFlags::empty()).unwrap();
        assert!(!ready(&orphan_handle));
        assert!(!ready(&grandchild_handle));
        // Pin INIT while the backend still holds it alive. Opening its numeric PID
        // after backend exit races with INIT's legitimate termination.
        let init_handle = pidfd_open(valid_pid(init).unwrap(), PidfdFlags::empty()).unwrap();
        fs::write(directory.join("backend-exit"), []).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if rustix::process::waitid(
                rustix::process::WaitId::Pid(valid_pid(child.id()).unwrap()),
                rustix::process::WaitIdOptions::EXITED
                    | rustix::process::WaitIdOptions::NOHANG
                    | rustix::process::WaitIdOptions::NOWAIT,
            )
            .unwrap()
            .is_some()
            {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "outside monitor did not report initial backend completion"
            );
            std::thread::yield_now();
        }
        owner.cleanup().unwrap();
        let orphan_stopped = ready(&orphan_handle);
        let grandchild_stopped = ready(&grandchild_handle);
        // Record the owning cleanup's result first. If a cleanup mutant leaves descendants,
        // fixture teardown uses the already-owned init, without turning that result into a pass.
        if !orphan_stopped || !grandchild_stopped {
            pidfd_send_signal(&init_handle, Signal::KILL).unwrap();
            let mut fds = [PollFd::new(&init_handle, PollFlags::IN)];
            poll(
                &mut fds,
                Some(&Timespec {
                    tv_sec: 5,
                    tv_nsec: 0,
                }),
            )
            .unwrap();
            assert!(fds[0].revents().contains(PollFlags::IN));
        }
        assert_eq!(child.wait().unwrap().code(), Some(0));
        fs::remove_dir_all(directory).unwrap();
        assert!(
            orphan_stopped,
            "completed-run cleanup must kill the adopted orphan"
        );
        assert!(
            grandchild_stopped,
            "completed-run cleanup must kill the unsampled fork"
        );
    }
}

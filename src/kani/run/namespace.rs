//! Per-run PID-namespace ownership: gated startup, claimed init, and confirmed teardown.
//!
//! Bubblewrap is mandatory. The caller's process never becomes a subreaper. Before dispatch,
//! the unreaped monitor pins the dedicated startup process group. After dispatch, the init
//! pidfd owns every descendant, including orphaned, session-escaped and nested-namespace tasks.
//!
//! This ownership and teardown covers in-process conclusions and startup error/unwind paths.
//! Abrupt caller death (SIGKILL, abort, OOM kill) before the gate/PDEATH chain is fully armed
//! can close the gate and release an unowned backend. Caller-death supervision is deferred
//! to IR-639; this module does not claim protection for that startup window.

use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io,
    os::fd::OwnedFd,
    path::{Path, PathBuf},
    process::{Child, Command},
    time::Instant,
};

#[cfg(target_os = "linux")]
use std::{
    fs,
    io::{Read, Write},
    time::Duration,
};

#[cfg(target_os = "linux")]
use command_fds::{CommandFdExt, FdMapping};
#[cfg(target_os = "linux")]
use rustix::pipe::{pipe_with, PipeFlags};
#[cfg(target_os = "linux")]
use rustix::process::{pidfd_open, pidfd_send_signal, PidfdFlags};
use rustix::{
    event::{poll, PollFd, PollFlags},
    process::{kill_process_group, Pid, Signal},
    time::Timespec,
};
#[cfg(target_os = "linux")]
use serde::Deserialize;

/// An internal command recipe with explicit environment inheritance and inherited stdin.
/// Capture owns stdout/stderr; no caller-configured stream is silently reconstructed.
pub(super) struct BackendCommand {
    program: OsString,
    arguments: Vec<OsString>,
    directory: Option<PathBuf>,
    environment: Vec<(OsString, OsString)>,
}

impl BackendCommand {
    pub(super) fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            program: program.as_ref().to_owned(),
            arguments: Vec::new(),
            directory: None,
            environment: Vec::new(),
        }
    }
    #[cfg(test)]
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
        self.directory = Some(directory.as_ref().to_owned());
        self
    }
    /// Apply the same argv, working directory and inherited-environment additions to a command.
    fn configure(&self, command: &mut Command) {
        command.args(&self.arguments);
        if let Some(directory) = &self.directory {
            command.current_dir(directory);
        }
        command.envs(self.environment.iter().map(|(name, value)| (name, value)));
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

pub(super) struct NamespaceOwner {
    gate: Option<File>,
    #[cfg(target_os = "linux")]
    info: File,
    wrapper: Option<Pid>,
    init: Option<(u32, u64, OwnedFd)>,
    cleaned: bool,
}

impl NamespaceOwner {
    pub(super) fn prepare(recipe: BackendCommand) -> io::Result<(Command, Self)> {
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
            .arg(&recipe.program);
        recipe.configure(&mut command);
        #[cfg(target_os = "linux")]
        {
            command
                .fd_mappings(vec![
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
                    info: File::from(info_read),
                    wrapper: None,
                    init: None,
                    cleaned: false,
                },
            ))
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(unavailable("PID-namespace ownership requires Linux"))
        }
    }

    /// Attach while the monitor is still unreaped; gated init cannot yet change its group.
    pub(super) fn attach(&mut self, child: &Child) -> io::Result<()> {
        self.wrapper = Some(valid_pid(child.id())?);
        Ok(())
    }

    /// Claim the namespace init before releasing any backend instruction.
    #[cfg(target_os = "linux")]
    pub(super) fn dispatch(
        &mut self,
        deadline: Option<Instant>,
        observer: &mut super::memory::MemoryObserver,
    ) -> io::Result<u32> {
        let startup_limit = Instant::now() + Duration::from_secs(5);
        let startup_deadline =
            deadline.map_or(startup_limit, |deadline| deadline.min(startup_limit));
        let info = self.startup_information(deadline, startup_deadline)?;
        let handle = pidfd_open(valid_pid(info.child)?, PidfdFlags::empty())?;
        let directory = PathBuf::from("/proc").join(info.child.to_string());
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
            || namespace != format!("pid:[{}]", info.namespace)
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
        self.init = Some((info.child, start, handle));
        observer.bind_root(info.child, start)?;
        if Instant::now() >= startup_deadline {
            return Err(startup_expiry(deadline));
        }
        self.gate
            .as_mut()
            .ok_or_else(|| unavailable("namespace startup gate missing"))?
            .write_all(&[1])?;
        self.gate.take();
        Ok(info.child)
    }

    /// Unsupported targets cannot dispatch a PID-namespace backend.
    #[cfg(not(target_os = "linux"))]
    pub(super) fn dispatch(
        &mut self,
        _: Option<Instant>,
        _: &mut super::memory::MemoryObserver,
    ) -> io::Result<u32> {
        Err(unavailable("PID namespaces unavailable"))
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
        let mut bytes = Vec::new();
        loop {
            if Instant::now() >= deadline {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "namespace startup exceeded its allowance",
                ));
            }
            let mut ready = [PollFd::new(&self.info, PollFlags::IN)];
            poll(
                &mut ready,
                Some(&Timespec {
                    tv_sec: 0,
                    tv_nsec: 20_000_000,
                }),
            )?;
            if ready[0].revents().is_empty() {
                continue;
            }
            let mut chunk = [0; 1024];
            let length = self.info.read(&mut chunk)?;
            if length == 0 {
                break;
            }
            if bytes.len().saturating_add(length) > 4096 {
                return Err(unavailable(
                    "namespace startup information exceeds its bound",
                ));
            }
            bytes.extend_from_slice(&chunk[..length]);
        }
        let info: StartupInfo = serde_json::from_slice(&bytes)
            .map_err(|error| unavailable(format!("namespace startup failed: {error}")))?;
        Ok(info)
    }

    /// Kill init on every conclusion, then confirm kernel namespace teardown before accepting it.
    pub(super) fn cleanup(&mut self) -> io::Result<()> {
        if self.cleaned {
            return Ok(());
        }
        let mut failure = None;
        #[cfg(target_os = "linux")]
        if let Some((_, _, handle)) = &self.init {
            if let Err(error) = pidfd_send_signal(handle, Signal::KILL) {
                if error != rustix::io::Errno::SRCH {
                    failure = Some(io::Error::from(error));
                }
            }
        }
        // Group kill MUST precede every possible gate close, even when init signalling failed.
        if let Some(wrapper) = self.wrapper {
            if let Err(error) = kill_process_group(wrapper, Signal::KILL) {
                if error != rustix::io::Errno::SRCH {
                    failure = Some(io::Error::from(error));
                }
            }
            #[cfg(target_os = "linux")]
            if self.init.is_none() {
                // On malformed info or early monitor death, inspect only membership of the
                // still-pinned startup group. This exceptional scan is never the polling path.
                confirm_startup_group_dead(wrapper)?;
            }
        }
        if let Some((_, _, handle)) = &self.init {
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

impl Drop for NamespaceOwner {
    fn drop(&mut self) {
        // Retry own-group SIGKILL before fields close on an exceptional cleanup failure.
        // The API refuses unconfirmed cleanup; it never accepts a proof in that state.
        let _ = self.cleanup();
        if !self.cleaned {
            if let Some(wrapper) = self.wrapper {
                let _ = kill_process_group(wrapper, Signal::KILL);
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn confirm_startup_group_dead(group: Pid) -> io::Result<()> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let mut alive = false;
        for entry in fs::read_dir("/proc")? {
            let entry = entry?;
            if entry
                .file_name()
                .to_str()
                .and_then(|name| name.parse::<u32>().ok())
                .is_none()
            {
                continue;
            }
            let text = match fs::read_to_string(entry.path().join("stat")) {
                Ok(text) => text,
                Err(error)
                    if error.kind() == io::ErrorKind::NotFound
                        || error.raw_os_error() == Some(rustix::io::Errno::SRCH.raw_os_error()) =>
                {
                    continue
                }
                Err(error) => return Err(error),
            };
            let Some((_, fields)) = text.rsplit_once(')') else {
                continue;
            };
            let mut fields = fields.split_whitespace();
            let state = fields.next();
            fields.next();
            let process_group = fields.next().and_then(|group| group.parse::<i32>().ok());
            if process_group == Some(group.as_raw_nonzero().get())
                && state != Some("Z")
                && state != Some("X")
            {
                alive = true;
            }
        }
        if !alive {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(io::Error::other(
                "startup group termination was not confirmed",
            ));
        }
        match kill_process_group(group, Signal::KILL) {
            Ok(()) | Err(rustix::io::Errno::SRCH) => {}
            Err(error) => return Err(error.into()),
        }
        std::thread::sleep(Duration::from_millis(20));
    }
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

fn unavailable(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::Unsupported, message.into())
}

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

    /// Trace: FR-028-AC-2, FR-028-AC-21.
    #[test]
    fn startup_cap_refuses_without_claiming_the_identity_wall_ceiling_elapsed() {
        let (_command, mut owner) = NamespaceOwner::prepare(BackendCommand::new("sh")).unwrap();
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
    os._exit(0)
os.setsid()
while os.getppid()!=1:
    os.sched_yield()
(root/'orphan-ready').write_text(str(os.getpid()))
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
        let orphan = children(init)
            .into_iter()
            .find(|pid| {
                fs::read_to_string(format!("/proc/{pid}/status")).is_ok_and(|status| {
                    status
                        .lines()
                        .find_map(|line| line.strip_prefix("NSpid:"))
                        .is_some_and(|ids| {
                            ids.split_whitespace().last() == Some(local_orphan.trim())
                        })
                })
            })
            .unwrap();
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

//! Joined nonleader C spawning thread with parent-retained actual launcher custody.
//!
//! The actual returned L Child is recorded before any fallible identity or publication work.
//! Normal thread return waits for positive L reap; dropping a
//! JoinHandle or Child is never recorded as settlement. The production owner must drive settlement
//! and join on every path, retaining this object on any unconfirmed cleanup result.

use std::{
    io,
    os::{fd::OwnedFd, unix::process::CommandExt},
    process::{Child, Command},
    sync::{Arc, Condvar, Mutex, MutexGuard},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use rustix::process::{pidfd_open, Pid, PidfdFlags};

use super::creator::CreatorThread;

/// Explicit per-run stack reservation; accounting must include this before thread creation.
pub(super) const SPAWNER_STACK_BYTES: usize = 2 * 1_048_576;
const TICK: Duration = Duration::from_millis(20);

/// The actual local operation that failed, retaining its original owned I/O cause. Only
/// Command names Command::spawn; no errno, diagnostic text or timeout selects that origin.
#[derive(Debug)]
pub(super) enum SpawnFailure {
    Command(io::Error),
    Creator(io::Error),
    LauncherIdentity(io::Error),
    LauncherPin(io::Error),
    Startup(io::Error),
}

impl SpawnFailure {
    /// Move the same cause into the public boundary after caller-owned settlement. The original
    /// private site must be retained separately if needed; this performs no classification.
    pub(super) fn into_io(self) -> io::Error {
        match self {
            Self::Command(error)
            | Self::Creator(error)
            | Self::LauncherIdentity(error)
            | Self::LauncherPin(error)
            | Self::Startup(error) => error,
        }
    }
}

impl std::fmt::Display for SpawnFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Command(error) => write!(formatter, "launcher command spawn: {error}"),
            Self::Creator(error) => write!(formatter, "launcher creator: {error}"),
            Self::LauncherIdentity(error) => write!(formatter, "launcher identity: {error}"),
            Self::LauncherPin(error) => write!(formatter, "launcher pin: {error}"),
            Self::Startup(error) => write!(formatter, "launcher startup: {error}"),
        }
    }
}

impl std::error::Error for SpawnFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Command(error)
            | Self::Creator(error)
            | Self::LauncherIdentity(error)
            | Self::LauncherPin(error)
            | Self::Startup(error) => Some(error),
        }
    }
}

enum Startup {
    Preparing,
    Ready,
    Failed,
}

struct Custody {
    startup: Startup,
    failure: Option<SpawnFailure>,
    creator: Option<CreatorThread>,
    launcher: Option<Child>,
    launcher_pin: Option<OwnedFd>,
    cancel_requested: bool,
    signal_sent: bool,
    creation_finished: bool,
    reaped: bool,
}

struct Shared {
    custody: Mutex<Custody>,
    changed: Condvar,
}

/// Snapshot consists only of duplicates of positively established kernel capabilities.
pub(super) struct SpawnIdentity {
    pub(super) creator_tid: Pid,
    pub(super) creator_pin: OwnedFd,
    pub(super) launcher_pid: Pid,
    pub(super) launcher_pin: OwnedFd,
}

pub(super) struct RetainedSpawner {
    shared: Arc<Shared>,
    thread: Option<JoinHandle<()>>,
}

impl RetainedSpawner {
    /// Named retained custody state plus the explicitly requested creator stack. Opaque std/
    /// pthread allocator, TLS and stack-guard overhead are not represented as measured capacity.
    pub(super) fn reserved_bytes() -> io::Result<u64> {
        let bytes = std::mem::size_of::<Shared>()
            .checked_add(SPAWNER_STACK_BYTES)
            .ok_or_else(|| io::Error::other("creator custody reservation overflow"))?;
        u64::try_from(bytes).map_err(io::Error::other)
    }

    /// Caller must reserve the explicit stack and this bounded custody allocation before entry.
    /// No helper identity or inherited descriptor is synthesized here: Command is the real L.
    pub(super) fn spawn(mut command: Command) -> Result<Self, SpawnFailure> {
        let shared = Arc::new(Shared {
            custody: Mutex::new(Custody {
                startup: Startup::Preparing,
                failure: None,
                creator: None,
                launcher: None,
                launcher_pin: None,
                cancel_requested: false,
                signal_sent: false,
                creation_finished: false,
                reaped: false,
            }),
            changed: Condvar::new(),
        });
        let retained = Arc::clone(&shared);
        let thread = thread::Builder::new()
            .name("kani-creator".to_owned())
            .stack_size(SPAWNER_STACK_BYTES)
            .spawn(move || {
                let creator = CreatorThread::capture().map_err(SpawnFailure::Creator);
                command.process_group(0);
                // Do not hold the custody mutex during Command's exec-error handshake. The
                // original owner can record deadline cancellation without waiting on that IO.
                // Until this returns, Preparing is not evidence of absence or settlement.
                let spawned = match &creator {
                    Ok(_) => Some(command.spawn().map_err(SpawnFailure::Command)),
                    Err(_) => None,
                };
                let mut custody = lock_custody(&retained);
                let spawned = match spawned {
                    Some(Ok(child)) => {
                        custody.launcher = Some(child);
                        Ok(())
                    }
                    Some(Err(error)) => Err(error),
                    None => Ok(()),
                };
                custody.creation_finished = true;
                let result = (|| {
                    let creator = creator?;
                    custody.creator = Some(creator);
                    // setpgid runs safely in the child before exec, separating L/O from C/group
                    // death. O itself must not request process_group(0), because it calls setsid.
                    spawned?;
                    let child = custody.launcher.as_ref().ok_or_else(|| {
                        SpawnFailure::Startup(io::Error::other(
                            "launcher custody missing after spawn",
                        ))
                    })?;
                    let pid = child_pid(child).map_err(SpawnFailure::LauncherIdentity)?;
                    custody.launcher_pin = Some(
                        pidfd_open(pid, PidfdFlags::NONBLOCK)
                            .map_err(|error| SpawnFailure::LauncherPin(error.into()))?,
                    );
                    if custody.cancel_requested {
                        return Err(SpawnFailure::Startup(io::Error::new(
                            io::ErrorKind::Interrupted,
                            "launcher startup already cancelled",
                        )));
                    }
                    Ok::<_, SpawnFailure>(())
                })();
                match result {
                    Ok(()) => custody.startup = Startup::Ready,
                    Err(error) => {
                        custody.startup = Startup::Failed;
                        custody.failure = Some(error);
                    }
                }
                drop(command); // intended child mappings cannot retain caller lease copies.
                retained.changed.notify_all();
                while !custody.reaped {
                    custody = match retained.changed.wait(custody) {
                        Ok(custody) => custody,
                        Err(poisoned) => {
                            let mut custody = poisoned.into_inner();
                            custody.startup = Startup::Failed;
                            custody.failure = Some(SpawnFailure::Startup(io::Error::other(
                                "launcher custody poisoned",
                            )));
                            custody
                        }
                    };
                }
            })
            .map_err(SpawnFailure::Creator)?;
        Ok(Self {
            shared,
            thread: Some(thread),
        })
    }

    /// Errors preserve this owner, including an actual Child created before a pin/open failure.
    pub(super) fn wait_started(&self, deadline: Instant) -> Result<SpawnIdentity, SpawnFailure> {
        let mut custody = lock_custody(&self.shared);
        loop {
            match custody.startup {
                Startup::Ready => {
                    let creator = custody.creator.as_ref().ok_or_else(|| {
                        SpawnFailure::Startup(io::Error::other("creator custody missing"))
                    })?;
                    creator.require_live().map_err(SpawnFailure::Creator)?;
                    let child = custody.launcher.as_ref().ok_or_else(|| {
                        SpawnFailure::Startup(io::Error::other("launcher custody missing"))
                    })?;
                    let launcher_pin = custody.launcher_pin.as_ref().ok_or_else(|| {
                        SpawnFailure::Startup(io::Error::other("launcher capability missing"))
                    })?;
                    return Ok(SpawnIdentity {
                        creator_tid: creator.tid(),
                        creator_pin: rustix::io::fcntl_dupfd_cloexec(creator.descriptor(), 3)
                            .map_err(|error| SpawnFailure::Creator(error.into()))?,
                        launcher_pid: child_pid(child).map_err(SpawnFailure::LauncherIdentity)?,
                        launcher_pin: rustix::io::fcntl_dupfd_cloexec(launcher_pin, 3)
                            .map_err(|error| SpawnFailure::LauncherPin(error.into()))?,
                    });
                }
                Startup::Failed => {
                    return Err(custody.failure.take().unwrap_or_else(|| {
                        SpawnFailure::Startup(io::Error::other("launcher startup refused"))
                    }));
                }
                Startup::Preparing => {}
            }
            if self.thread.as_ref().is_some_and(JoinHandle::is_finished) {
                return Err(SpawnFailure::Startup(io::Error::other(
                    "creating thread terminated before startup",
                )));
            }
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .filter(|remaining| !remaining.is_zero())
                .ok_or_else(|| {
                    SpawnFailure::Startup(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "launcher startup deadline",
                    ))
                })?;
            custody = match self
                .shared
                .changed
                .wait_timeout(custody, remaining.min(TICK))
            {
                Ok((custody, _)) => custody,
                Err(poisoned) => {
                    let (mut custody, _) = poisoned.into_inner();
                    custody.startup = Startup::Failed;
                    custody.failure = Some(SpawnFailure::Startup(io::Error::other(
                        "launcher custody poisoned",
                    )));
                    custody
                }
            };
        }
    }

    /// Call only after the original owner has settled independently retained O/I authorities.
    /// Positive Child reap, including startup without a Child, releases the retained creator.
    pub(super) fn settle_launcher(&self, deadline: Instant) -> io::Result<()> {
        loop {
            {
                let mut custody = lock_custody(&self.shared);
                if custody.reaped {
                    return Ok(());
                }
                custody.cancel_requested = true;
                let signal_sent = custody.signal_sent;
                match custody.launcher.as_mut() {
                    Some(child) => {
                        if child.try_wait()?.is_some() {
                            custody.reaped = true;
                        } else if !signal_sent {
                            match child.kill() {
                                Ok(()) => {}
                                Err(error)
                                    if error.raw_os_error()
                                        == Some(rustix::io::Errno::SRCH.raw_os_error()) => {}
                                Err(error) => return Err(error),
                            }
                            // Retire acknowledgement can let L exit between try_wait and kill.
                            // ESRCH ends the one signal attempt, never the retained Child wait.
                            custody.signal_sent = true;
                        }
                    }
                    None => {
                        // A poisoned/failed startup flag alone cannot prove that the spawning
                        // thread has returned from its exec handshake. Preserve custody until
                        // actual creation completion or positively observed thread termination.
                        if custody.creation_finished
                            || self.thread.as_ref().is_some_and(JoinHandle::is_finished)
                        {
                            custody.reaped = true;
                        }
                    }
                }
                if custody.reaped {
                    self.shared.changed.notify_all();
                    return Ok(());
                }
            }
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .filter(|remaining| !remaining.is_zero())
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::TimedOut, "launcher reap unconfirmed")
                })?;
            thread::park_timeout(remaining.min(TICK));
        }
    }

    /// Joining is allowed only after actual launcher settlement; failure retains the handle.
    pub(super) fn join(&mut self, deadline: Instant) -> io::Result<()> {
        if !lock_custody(&self.shared).reaped {
            return Err(io::Error::other("launcher settlement not confirmed"));
        }
        while self
            .thread
            .as_ref()
            .is_some_and(|thread| !thread.is_finished())
        {
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .filter(|remaining| !remaining.is_zero())
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::TimedOut, "creator join unconfirmed")
                })?;
            thread::park_timeout(remaining.min(TICK));
        }
        if let Some(thread) = self.thread.take() {
            thread
                .join()
                .map_err(|_| io::Error::other("creating thread panicked"))?;
        }
        Ok(())
    }
}

fn lock_custody(shared: &Shared) -> MutexGuard<'_, Custody> {
    match shared.custody.lock() {
        Ok(custody) => custody,
        Err(poisoned) => {
            // Recovery preserves actual cleanup authority, never successful startup authority.
            let mut custody = poisoned.into_inner();
            custody.startup = Startup::Failed;
            custody.failure = Some(SpawnFailure::Startup(io::Error::other(
                "launcher custody poisoned",
            )));
            custody
        }
    }
}

fn child_pid(child: &Child) -> io::Result<Pid> {
    let raw = i32::try_from(child.id())
        .map_err(|_| io::Error::other("actual launcher PID exceeds platform range"))?;
    Pid::from_raw(raw).ok_or_else(|| io::Error::other("actual launcher PID is not positive"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[derive(Debug)]
    struct OriginalPayload(u64);

    impl std::fmt::Display for OriginalPayload {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "local creator payload {}", self.0)
        }
    }

    impl std::error::Error for OriginalPayload {}

    /// Trace: FR-034-AC-15, FR-034-AC-38.
    #[test]
    fn typed_spawn_origin_keeps_original_local_os_kind_and_errno() {
        // Cause projection only: these values do not claim a Command/kernel operation failed.
        let constructors: [fn(io::Error) -> SpawnFailure; 5] = [
            SpawnFailure::Command,
            SpawnFailure::Creator,
            SpawnFailure::LauncherIdentity,
            SpawnFailure::LauncherPin,
            SpawnFailure::Startup,
        ];
        for (expected, construct) in constructors.into_iter().enumerate() {
            let original = io::Error::from_raw_os_error(rustix::io::Errno::ACCESS.raw_os_error());
            let kind = original.kind();
            let errno = original.raw_os_error();
            let failure = construct(original);
            let actual = match &failure {
                SpawnFailure::Command(_) => 0,
                SpawnFailure::Creator(_) => 1,
                SpawnFailure::LauncherIdentity(_) => 2,
                SpawnFailure::LauncherPin(_) => 3,
                SpawnFailure::Startup(_) => 4,
            };
            assert_eq!(actual, expected);
            let source = failure
                .source()
                .unwrap()
                .downcast_ref::<io::Error>()
                .unwrap();
            assert_eq!(source.kind(), kind);
            assert_eq!(source.raw_os_error(), errno);
            let returned = failure.into_io();
            assert_eq!(returned.kind(), kind);
            assert_eq!(returned.raw_os_error(), errno);
            assert!(returned.get_ref().is_none());
        }
    }

    /// Trace: FR-034-AC-15, FR-034-AC-38.
    #[test]
    fn startup_failure_moves_custom_cause_without_replacing_payload_or_kind() {
        let original = io::Error::new(io::ErrorKind::Interrupted, OriginalPayload(41));
        let address = original
            .get_ref()
            .unwrap()
            .downcast_ref::<OriginalPayload>()
            .unwrap() as *const OriginalPayload;
        let failure = SpawnFailure::Startup(original);
        assert!(matches!(&failure, SpawnFailure::Startup(_)));
        let returned = failure.into_io();
        assert_eq!(returned.kind(), io::ErrorKind::Interrupted);
        assert_eq!(returned.raw_os_error(), None);
        let payload = returned
            .get_ref()
            .unwrap()
            .downcast_ref::<OriginalPayload>()
            .unwrap();
        assert_eq!(payload.0, 41);
        assert!(std::ptr::eq(payload, address));
    }
}

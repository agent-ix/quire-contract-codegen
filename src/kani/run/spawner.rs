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

enum Startup {
    Preparing,
    Ready,
    Failed,
}

struct Custody {
    startup: Startup,
    failure: Option<io::Error>,
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
    /// Caller must reserve the explicit stack and this bounded custody allocation before entry.
    /// No helper identity or inherited descriptor is synthesized here: Command is the real L.
    pub(super) fn spawn(mut command: Command) -> io::Result<Self> {
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
                let creator = CreatorThread::capture();
                command.process_group(0);
                // Do not hold the custody mutex during Command's exec-error handshake. The
                // original owner can record deadline cancellation without waiting on that IO.
                // Until this returns, Preparing is not evidence of absence or settlement.
                let spawned = match &creator {
                    Ok(_) => Some(command.spawn()),
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
                    let child = custody
                        .launcher
                        .as_ref()
                        .ok_or_else(|| io::Error::other("launcher custody missing after spawn"))?;
                    let pid = child_pid(child)?;
                    custody.launcher_pin = Some(pidfd_open(pid, PidfdFlags::NONBLOCK)?);
                    if custody.cancel_requested {
                        return Err(io::Error::new(
                            io::ErrorKind::Interrupted,
                            "launcher startup already cancelled",
                        ));
                    }
                    Ok::<_, io::Error>(())
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
                            custody.failure = Some(io::Error::other("launcher custody poisoned"));
                            custody
                        }
                    };
                }
            })?;
        Ok(Self {
            shared,
            thread: Some(thread),
        })
    }

    /// Errors preserve this owner, including an actual Child created before a pin/open failure.
    pub(super) fn wait_started(&self, deadline: Instant) -> io::Result<SpawnIdentity> {
        let mut custody = lock_custody(&self.shared);
        loop {
            match custody.startup {
                Startup::Ready => {
                    let creator = custody
                        .creator
                        .as_ref()
                        .ok_or_else(|| io::Error::other("creator custody missing"))?;
                    creator.require_live()?;
                    let child = custody
                        .launcher
                        .as_ref()
                        .ok_or_else(|| io::Error::other("launcher custody missing"))?;
                    let launcher_pin = custody
                        .launcher_pin
                        .as_ref()
                        .ok_or_else(|| io::Error::other("launcher capability missing"))?;
                    return Ok(SpawnIdentity {
                        creator_tid: creator.tid(),
                        creator_pin: rustix::io::fcntl_dupfd_cloexec(creator.descriptor(), 3)?,
                        launcher_pid: child_pid(child)?,
                        launcher_pin: rustix::io::fcntl_dupfd_cloexec(launcher_pin, 3)?,
                    });
                }
                Startup::Failed => {
                    return Err(custody
                        .failure
                        .take()
                        .unwrap_or_else(|| io::Error::other("launcher startup refused")));
                }
                Startup::Preparing => {}
            }
            if self.thread.as_ref().is_some_and(JoinHandle::is_finished) {
                return Err(io::Error::other(
                    "creating thread terminated before startup",
                ));
            }
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .filter(|remaining| !remaining.is_zero())
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::TimedOut, "launcher startup deadline")
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
                    custody.failure = Some(io::Error::other("launcher custody poisoned"));
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
                            child.kill()?;
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
            custody.failure = Some(io::Error::other("launcher custody poisoned"));
            custody
        }
    }
}

fn child_pid(child: &Child) -> io::Result<Pid> {
    let raw = i32::try_from(child.id())
        .map_err(|_| io::Error::other("actual launcher PID exceeds platform range"))?;
    Pid::from_raw(raw).ok_or_else(|| io::Error::other("actual launcher PID is not positive"))
}

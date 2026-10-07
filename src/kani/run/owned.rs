//! Production retained ownership, authenticated completion and pre-escalation lease observation.

use std::{
    io,
    num::NonZeroUsize,
    os::unix::{fs::PermissionsExt, process::CommandExt},
    path::Path,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use super::{
    control::{CallerLease, ControlError},
    launch::{
        finish_capture, reap_stopped_launcher, stream_bytes, BoundedLaunch, BoundedLaunchError,
        CaptureFlags, CaptureStream, Captured, GuardianFailureKind, LaunchOutcome, PreparedCapture,
        CAPTURE_LIMIT,
    },
    memory::MemoryObserver,
    namespace::{BackendCommand, GatedClaim, NamespaceOwner, ReadyIdentityError},
    protocol::{BackendExit, GuardianRefusal},
    publication::{Publication, Stage},
    report_file::read_report,
    stages::{Bootstrap, ClaimedBootstrap, Dispatched, InitReady, StageError},
    stdin::OriginalStdin,
};

pub(super) const TICK: Duration = Duration::from_millis(20);
const LEASE_CLOSE_CAP: Duration = Duration::from_millis(250);

/// Owns every independent cancellation/settlement authority; the lease owns none of these.
struct RunOwner {
    monitor: Child,
    namespace: NamespaceOwner,
    stdout: Option<thread::JoinHandle<Captured>>,
    stderr: Option<thread::JoinHandle<Captured>>,
    flags: CaptureFlags,
    deadline: Option<Instant>,
    publication: Arc<Publication>,
    capture_reserved_bytes: u64,
}

/// Recorded before independent INIT cancellation, with the live RunOwner still retained.
enum LeaseCloseObservation {
    ConfirmedTermination,
    EscalationRequired,
    Unavailable(io::Error),
}

struct Settled {
    monitor: io::Result<std::process::ExitStatus>,
    cleanup: io::Result<()>,
    stdout: Captured,
    stderr: Captured,
}

impl RunOwner {
    fn spawn(
        mut command: Command,
        namespace: NamespaceOwner,
        deadline: Option<Instant>,
        limit: usize,
        publication: Arc<Publication>,
    ) -> io::Result<Self> {
        let prepared_stdout = PreparedCapture::prepare(limit)?;
        let prepared_stderr = PreparedCapture::prepare(limit)?;
        let capture_reserved_bytes = prepared_stdout
            .reserved_bytes()?
            .checked_add(prepared_stderr.reserved_bytes()?)
            .ok_or_else(|| io::Error::other("capture reservation cannot be represented"))?;
        command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .process_group(0);
        let child = command.spawn()?;
        drop(command); // Child-only mapping copies cannot keep the exclusive lease alive.
        let mut owner = Self {
            monitor: child,
            namespace,
            stdout: None,
            stderr: None,
            deadline,
            publication,
            capture_reserved_bytes,
            flags: CaptureFlags {
                stop: Arc::new(AtomicBool::new(false)),
                failed: Arc::new(AtomicBool::new(false)),
            },
        };
        if let Err(error) = owner.namespace.attach(&owner.monitor) {
            let _ = owner.monitor.kill();
            let _ = reap_stopped_launcher(&mut owner.monitor);
            return Err(error);
        }
        let (Some(stdout), Some(stderr)) =
            (owner.monitor.stdout.take(), owner.monitor.stderr.take())
        else {
            let _ = owner.settle();
            return Err(io::Error::other(
                "guardian bounded streams were not captured",
            ));
        };
        let readers = (|| {
            owner.stdout = Some(prepared_stdout.spawn(stdout, &owner.flags)?);
            owner.stderr = Some(prepared_stderr.spawn(stderr, &owner.flags)?);
            Ok::<(), io::Error>(())
        })();
        if let Err(error) = readers {
            let _ = owner.settle();
            return Err(error);
        }
        Ok(owner)
    }

    /// The only production lease-close observation. No signal or monitor reap occurs here.
    fn close_lease_and_observe(&self, lease: CallerLease) -> LeaseCloseObservation {
        drop(lease);
        self.publication.lease_closed();
        self.publication.publish(Stage::LeaseClosing);
        let cap = Instant::now() + LEASE_CLOSE_CAP;
        let deadline = self.deadline.map_or(cap, |deadline| deadline.min(cap));
        loop {
            match self.namespace.init_terminated() {
                Ok(true) => return LeaseCloseObservation::ConfirmedTermination,
                Err(error) => return LeaseCloseObservation::Unavailable(error),
                Ok(false) => {}
            }
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                return LeaseCloseObservation::EscalationRequired;
            };
            thread::sleep(remaining.min(TICK));
        }
    }

    /// Independent escalation follows the recorded lease observation, or urgent resource refusal.
    fn settle(&mut self) -> Settled {
        let cleanup = self.namespace.cleanup();
        let monitor = reap_stopped_launcher(&mut self.monitor);
        if monitor.is_err() {
            // The unreaped Child still pins this monitor identity. A failed normal reap must
            // not strand it after successful namespace cleanup suppressed NamespaceOwner Drop.
            let _ = self.monitor.kill();
            let _ = reap_stopped_launcher(&mut self.monitor);
        }
        self.flags.stop.store(true, Ordering::Release);
        let stdout = self
            .stdout
            .take()
            .map(finish_capture)
            .unwrap_or(Ok(Vec::new()));
        let stderr = self
            .stderr
            .take()
            .map(finish_capture)
            .unwrap_or(Ok(Vec::new()));
        Settled {
            monitor,
            cleanup,
            stdout,
            stderr,
        }
    }
}

impl Drop for RunOwner {
    fn drop(&mut self) {
        // NamespaceOwner's Drop initiates pinned cancellation; it does not confirm or reap.
        self.flags.stop.store(true, Ordering::Release);
    }
}

enum Stop {
    Completed(BackendExit),
    TimedOut,
    MemoryExhausted,
    MemoryUnobserved(String),
    CaptureFailed,
}

pub(super) fn run(
    recipe: BackendCommand,
    helper: &Path,
    stdin: &OriginalStdin,
    report_path: &Path,
    ceilings: crate::kani::identity::ProofCeilings,
    harnesses: NonZeroUsize,
    deadline: Option<Instant>,
    observer: &mut MemoryObserver,
) -> Result<BoundedLaunch, BoundedLaunchError> {
    let helper = resolve_helper(helper)?;
    let limit = CAPTURE_LIMIT.saturating_mul(harnesses.get());
    let started = match start_sequence(
        recipe,
        &helper,
        stdin,
        report_path,
        deadline,
        observer,
        limit,
        Stage::Dispatched,
    ) {
        Ok(started) => started,
        Err(error)
            if expired(deadline)
                && !matches!(
                    error,
                    BoundedLaunchError::Guardian {
                        kind: GuardianFailureKind::CleanupUnconfirmed,
                        ..
                    }
                ) =>
        {
            return Ok(BoundedLaunch {
                outcome: LaunchOutcome::TimedOut,
                memory: observer.observation(),
                report: Ok(None),
            });
        }
        Err(error) => return Err(error),
    };
    let Started {
        owner: Some(mut owner),
        state: SequenceState::Dispatched(dispatched),
        init: Some(init),
        publication,
    } = started
    else {
        return Err(refusal(
            GuardianFailureKind::UnexpectedControl,
            "incomplete production stage sequence".to_owned(),
        ));
    };
    // The complete sequence transfers its same publication ownership with the run handles.
    owner.publication = publication;
    let waiting = wait(
        &owner,
        &dispatched,
        init,
        ceilings.memory_bytes.get(),
        observer,
    );
    let mut stop = match waiting {
        Ok(stop) => stop,
        Err(error) => {
            drop(dispatched.into_lease());
            let settled = owner.settle();
            require_cleanup(&settled)?;
            if expired(deadline) {
                return Ok(BoundedLaunch {
                    outcome: LaunchOutcome::TimedOut,
                    memory: observer.observation(),
                    report: Ok(None),
                });
            }
            return Err(stage_refusal(error));
        }
    };
    // Only authenticated completed backend bytes are retained. Classification remains after cleanup.
    let report = if matches!(stop, Stop::Completed(_)) {
        read_report(report_path, deadline)
    } else {
        Ok(None)
    };
    if expired(deadline) {
        stop = Stop::TimedOut;
    }
    let observation = if matches!(stop, Stop::Completed(_)) {
        Some(owner.close_lease_and_observe(dispatched.into_lease()))
    } else {
        // Resource/observation/capture failure retains immediate independent INIT cancellation.
        drop(dispatched.into_lease());
        None
    };
    let settled = owner.settle();
    require_cleanup(&settled)?;
    if expired(deadline) && matches!(stop, Stop::Completed(_)) {
        stop = Stop::TimedOut;
    }
    if let Some(observation) = observation {
        match observation {
            LeaseCloseObservation::ConfirmedTermination => {}
            LeaseCloseObservation::EscalationRequired => {
                if expired(deadline) {
                    stop = Stop::TimedOut;
                } else {
                    return Err(refusal(
                        GuardianFailureKind::GuardianTerminated,
                        "guardian did not terminate through its closed lease before escalation"
                            .to_owned(),
                    ));
                }
            }
            LeaseCloseObservation::Unavailable(error) => {
                return Err(refusal(
                    GuardianFailureKind::CleanupUnconfirmed,
                    error.to_string(),
                ))
            }
        }
    }
    let outcome = match stop {
        Stop::MemoryExhausted => LaunchOutcome::MemoryExhausted,
        Stop::MemoryUnobserved(detail) => LaunchOutcome::MemoryUnobserved { detail },
        Stop::TimedOut => {
            // As in the existing capture driver, capture refusal owns a concurrent wall expiry.
            for (stream, captured) in [
                (CaptureStream::Stdout, settled.stdout),
                (CaptureStream::Stderr, settled.stderr),
            ] {
                if let Err(outcome) = stream_bytes(stream, captured, limit, harnesses) {
                    return Ok(BoundedLaunch {
                        outcome,
                        memory: observer.observation(),
                        report: Ok(None),
                    });
                }
            }
            LaunchOutcome::TimedOut
        }
        Stop::Completed(exit) => {
            if !settled
                .monitor
                .as_ref()
                .is_ok_and(|status| status.success())
            {
                return Err(refusal(
                    GuardianFailureKind::GuardianTerminated,
                    "guardian monitor reported unsuccessful termination".to_owned(),
                ));
            }
            let stdout = match stream_bytes(CaptureStream::Stdout, settled.stdout, limit, harnesses)
            {
                Ok(bytes) => bytes,
                Err(outcome) => {
                    return Ok(BoundedLaunch {
                        outcome,
                        memory: observer.observation(),
                        report: Ok(None),
                    })
                }
            };
            let stderr = match stream_bytes(CaptureStream::Stderr, settled.stderr, limit, harnesses)
            {
                Ok(bytes) => bytes,
                Err(outcome) => {
                    return Ok(BoundedLaunch {
                        outcome,
                        memory: observer.observation(),
                        report: Ok(None),
                    })
                }
            };
            LaunchOutcome::Completed {
                exited_successfully: exit == BackendExit::Code(0),
                exit_code: match exit {
                    BackendExit::Code(code) => Some(code),
                    BackendExit::Signal(_) => None,
                },
                text: format!(
                    "{}\n{}",
                    String::from_utf8_lossy(&stdout),
                    String::from_utf8_lossy(&stderr)
                ),
            }
        }
        Stop::CaptureFailed => {
            for (stream, captured) in [
                (CaptureStream::Stdout, settled.stdout),
                (CaptureStream::Stderr, settled.stderr),
            ] {
                if let Err(outcome) = stream_bytes(stream, captured, limit, harnesses) {
                    return Ok(BoundedLaunch {
                        outcome,
                        memory: observer.observation(),
                        report: Ok(None),
                    });
                }
            }
            return Err(refusal(
                GuardianFailureKind::ControlUnavailable,
                "capture failure flag had no settled failure".to_owned(),
            ));
        }
    };
    let retained_report = if matches!(outcome, LaunchOutcome::Completed { .. }) {
        report
    } else {
        Ok(None)
    };
    Ok(BoundedLaunch {
        outcome,
        memory: observer.observation(),
        report: retained_report,
    })
}

fn resolve_helper(helper: &Path) -> Result<std::path::PathBuf, BoundedLaunchError> {
    // Resolve the supplied file directly; a relative path never enters executable PATH search.
    let helper = if helper.is_absolute() {
        helper.to_owned()
    } else {
        std::env::current_dir()
            .map_err(BoundedLaunchError::Io)?
            .join(helper)
    };
    let helper_metadata = std::fs::metadata(&helper).map_err(|error| {
        refusal(
            if error.kind() == io::ErrorKind::NotFound {
                GuardianFailureKind::MissingHelper
            } else {
                GuardianFailureKind::UnusableHelper
            },
            error.to_string(),
        )
    })?;
    if !helper_metadata.is_file() || helper_metadata.permissions().mode() & 0o111 == 0 {
        return Err(refusal(
            GuardianFailureKind::UnusableHelper,
            "configured guardian must be an explicit executable file".to_owned(),
        ));
    }
    Ok(helper)
}

/// The single production sequence. A fixture selects a prefix as ordinary data.
enum SequenceState {
    BeforeMonitor {
        command: Command,
        namespace: NamespaceOwner,
        bootstrap: Bootstrap,
    },
    Bootstrap(Bootstrap),
    ClaimedGated {
        bootstrap: Bootstrap,
        gated: GatedClaim,
    },
    ClaimedBootstrap(ClaimedBootstrap),
    InitReady(InitReady),
    Dispatched(Dispatched),
}

impl SequenceState {
    fn stage(&self) -> Stage {
        match self {
            Self::BeforeMonitor { .. } => Stage::BeforeMonitor,
            Self::Bootstrap(_) => Stage::Bootstrap,
            Self::ClaimedGated { .. } => Stage::ClaimedGated,
            Self::ClaimedBootstrap(_) => Stage::ClaimedBootstrap,
            Self::InitReady(_) => Stage::InitReady,
            Self::Dispatched(_) => Stage::Dispatched,
        }
    }
}

struct Started {
    owner: Option<RunOwner>,
    state: SequenceState,
    init: Option<u32>,
    publication: Arc<Publication>,
}

fn start_sequence(
    recipe: BackendCommand,
    helper: &Path,
    stdin: &OriginalStdin,
    report_path: &Path,
    deadline: Option<Instant>,
    observer: &mut MemoryObserver,
    limit: usize,
    until: Stage,
) -> Result<Started, BoundedLaunchError> {
    let (bootstrap, endpoint) = Bootstrap::new(deadline).map_err(stage_refusal)?;
    let (command, namespace) = NamespaceOwner::prepare(helper, endpoint)
        .map_err(|error| refusal(GuardianFailureKind::ControlUnavailable, error.to_string()))?;
    let publication = Arc::new(Publication::default());
    let mut state = SequenceState::BeforeMonitor {
        command,
        namespace,
        bootstrap,
    };
    let mut owner: Option<RunOwner> = None;
    let mut init = None;
    let mut recipe = Some(recipe);
    loop {
        publication.publish(state.stage());
        if publication.stage() != Some(state.stage()) {
            drop(state);
            if let Some(retained) = &mut owner {
                require_cleanup(&retained.settle())?;
            }
            return Err(refusal(
                GuardianFailureKind::UnexpectedControl,
                "shared stage publication unavailable".to_owned(),
            ));
        }
        if state.stage() == until {
            return Ok(Started {
                owner,
                state,
                init,
                publication,
            });
        }
        let advance = (|| {
            Ok::<_, BoundedLaunchError>(match state {
                SequenceState::BeforeMonitor {
                    command,
                    namespace,
                    bootstrap,
                } => {
                    owner = Some(
                        RunOwner::spawn(
                            command,
                            namespace,
                            deadline,
                            limit,
                            Arc::clone(&publication),
                        )
                        .map_err(BoundedLaunchError::Io)?,
                    );
                    SequenceState::Bootstrap(bootstrap)
                }
                SequenceState::Bootstrap(bootstrap) => {
                    let retained = retained_owner(&mut owner)?;
                    let gated = retained
                        .namespace
                        .claim_gated(deadline, bootstrap.setup_deadline(), observer)
                        .map_err(|error| {
                            refusal(GuardianFailureKind::InitIdentityMismatch, error.to_string())
                        })?;
                    SequenceState::ClaimedGated { bootstrap, gated }
                }
                SequenceState::ClaimedGated { bootstrap, gated } => {
                    let retained = retained_owner(&mut owner)?;
                    init = Some(
                        retained
                            .namespace
                            .release_bootstrap_gate(gated, deadline, bootstrap.setup_deadline())
                            .map_err(|error| {
                                refusal(
                                    GuardianFailureKind::InitIdentityMismatch,
                                    error.to_string(),
                                )
                            })?,
                    );
                    SequenceState::ClaimedBootstrap(bootstrap.claimed().map_err(stage_refusal)?)
                }
                SequenceState::ClaimedBootstrap(claimed) => {
                    let retained = retained_owner(&mut owner)?;
                    SequenceState::InitReady(
                        claimed
                            .authenticate(&retained.namespace)
                            .map_err(stage_refusal)?,
                    )
                }
                SequenceState::InitReady(ready) => {
                    let retained = retained_owner(&mut owner)?;
                    let command = recipe.take().ok_or_else(|| {
                        refusal(
                            GuardianFailureKind::UnexpectedControl,
                            "backend recipe already consumed".to_owned(),
                        )
                    })?;
                    let dispatched = ready
                        .send_dispatch(command, stdin, vec![report_path.as_os_str().to_owned()])
                        .and_then(|pending| pending.acknowledge(&retained.namespace))
                        .map_err(stage_refusal)?;
                    SequenceState::Dispatched(dispatched)
                }
                SequenceState::Dispatched(_) => {
                    return Err(refusal(
                        GuardianFailureKind::UnexpectedControl,
                        "requested stage is outside startup sequence".to_owned(),
                    ))
                }
            })
        })();
        match advance {
            Ok(next) => state = next,
            Err(error) => {
                if let Some(retained) = &mut owner {
                    require_cleanup(&retained.settle())?;
                }
                return Err(error);
            }
        }
    }
}

fn retained_owner(owner: &mut Option<RunOwner>) -> Result<&mut RunOwner, BoundedLaunchError> {
    owner.as_mut().ok_or_else(|| {
        refusal(
            GuardianFailureKind::UnexpectedControl,
            "stage has no retained monitor owner".to_owned(),
        )
    })
}

fn wait(
    owner: &RunOwner,
    dispatched: &Dispatched,
    init: u32,
    ceiling: u64,
    observer: &mut MemoryObserver,
) -> Result<Stop, StageError> {
    loop {
        match observer.observe(init) {
            Ok(bytes) if bytes > ceiling => return Ok(Stop::MemoryExhausted),
            Ok(_) => {}
            Err(error) => return Ok(Stop::MemoryUnobserved(error.to_string())),
        }
        if expired(owner.deadline) {
            return Ok(Stop::TimedOut);
        }
        if owner.flags.failed.load(Ordering::Acquire) {
            return Ok(Stop::CaptureFailed);
        }
        if let Some(exit) = dispatched.completion(&owner.namespace, owner.deadline)? {
            return Ok(Stop::Completed(exit.outcome));
        }
        thread::sleep(TICK);
    }
}

fn expired(deadline: Option<Instant>) -> bool {
    deadline.is_some_and(|deadline| Instant::now() >= deadline)
}

fn require_cleanup(settled: &Settled) -> Result<(), BoundedLaunchError> {
    if let Err(error) = &settled.cleanup {
        return Err(refusal(
            GuardianFailureKind::CleanupUnconfirmed,
            error.to_string(),
        ));
    }
    if let Err(error) = &settled.monitor {
        return Err(refusal(
            GuardianFailureKind::CleanupUnconfirmed,
            error.to_string(),
        ));
    }
    Ok(())
}

fn refusal(kind: GuardianFailureKind, detail: String) -> BoundedLaunchError {
    BoundedLaunchError::Guardian { kind, detail }
}

fn stage_refusal(error: StageError) -> BoundedLaunchError {
    let kind = match &error {
        StageError::Control(error) => match error {
            ControlError::Io(_) => GuardianFailureKind::ControlUnavailable,
            ControlError::Eof => GuardianFailureKind::GuardianTerminated,
            ControlError::Deadline => GuardianFailureKind::ControlDeadline,
            ControlError::EncodedBytesExceeded => GuardianFailureKind::ControlLimit,
            ControlError::InvalidEncoding(_)
            | ControlError::Truncated
            | ControlError::UnknownAncillary
            | ControlError::ProgressPoisoned => GuardianFailureKind::MalformedControl,
            ControlError::PartialTerminalSend { .. } => GuardianFailureKind::MalformedControl,
            ControlError::TrailingTerminalBytes => GuardianFailureKind::MalformedControl,
            ControlError::MissingCredentials
            | ControlError::RepeatedCredentials
            | ControlError::ChangedCredentials
            | ControlError::UnexpectedCredentials
            | ControlError::CreatorMismatch => GuardianFailureKind::SenderMismatch,
            ControlError::ExcessRights
            | ControlError::RightsNotCloexec
            | ControlError::RightsCount { .. } => GuardianFailureKind::InvalidDescriptors,
        },
        StageError::Identity(error) => match error {
            ReadyIdentityError::SenderMismatch => GuardianFailureKind::SenderMismatch,
            ReadyIdentityError::SessionNotIsolated => GuardianFailureKind::SessionNotIsolated,
            ReadyIdentityError::UidMappingMismatch => GuardianFailureKind::UidMappingMismatch,
            ReadyIdentityError::InitTerminated => GuardianFailureKind::GuardianTerminated,
            ReadyIdentityError::Unclaimed
            | ReadyIdentityError::ChainMismatch
            | ReadyIdentityError::Observation(_) => GuardianFailureKind::InitIdentityMismatch,
        },
        StageError::Authority(_) => GuardianFailureKind::ControlUnavailable,
        StageError::BuildIdentityMismatch => GuardianFailureKind::BuildIdentityMismatch,
        StageError::AuthorityMismatch => GuardianFailureKind::AuthorityMismatch,
        StageError::UnexpectedControl => GuardianFailureKind::UnexpectedControl,
        StageError::MissingSender => GuardianFailureKind::SenderMismatch,
        StageError::GuardianRefused(reason) => match reason {
            GuardianRefusal::NotNamespaceInit => GuardianFailureKind::InitIdentityMismatch,
            GuardianRefusal::SessionNotIsolated => GuardianFailureKind::SessionNotIsolated,
            GuardianRefusal::CreatorUidMismatch => GuardianFailureKind::UidMappingMismatch,
            GuardianRefusal::BuildIdentityMismatch => GuardianFailureKind::BuildIdentityMismatch,
            GuardianRefusal::UnexpectedControl => GuardianFailureKind::UnexpectedControl,
            GuardianRefusal::ReplayedAuthority => GuardianFailureKind::AuthorityMismatch,
            GuardianRefusal::InvalidControl => GuardianFailureKind::MalformedControl,
            GuardianRefusal::BackendSpawnFailed => GuardianFailureKind::BackendSpawnFailed,
            GuardianRefusal::BackendObservationFailed => {
                GuardianFailureKind::BackendObservationFailed
            }
        },
    };
    refusal(kind, format!("{error}"))
}

#[cfg(feature = "guardian-test-support")]
#[path = "fixture_execution.rs"]
mod fixtures;

#[cfg(feature = "guardian-test-support")]
pub(super) fn fixture(
    request: super::fixture::GuardianFixtureRequest<'_>,
) -> Result<super::fixture::GuardianFixtureObservation, super::fixture::GuardianFixtureError> {
    fixtures::run(request)
}

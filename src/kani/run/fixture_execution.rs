//! Fixture-only raw reporting/coordination around the same private production stages.

use super::super::{
    control::Transport, fixture::*, memory::MemoryObserver, namespace::BackendCommand,
};
use super::{
    resolve_helper, retained_owner, start_sequence, BoundedLaunchError, LeaseCloseObservation,
    Publication, RunOwner, SequenceState, Stage, Started, CAPTURE_LIMIT,
};
use rustix::{
    event::{poll, PollFd, PollFlags},
    fs::{fstat, OFlags},
    io::{fcntl_dupfd_cloexec, fcntl_getfd, fcntl_getfl, fcntl_setfd, fcntl_setfl, FdFlags},
    process::{
        getpgrp, getpid, getsid, kill_process, kill_process_group, pidfd_open, pidfd_send_signal,
        Pid, PidfdFlags, Signal,
    },
    time::Timespec,
};
use serde::Deserialize;
use std::{
    fs::{self, OpenOptions},
    io::{self, Read},
    os::fd::{AsFd, OwnedFd},
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

const PROC_BYTES: usize = 8192;
const WITNESS_BYTES: usize = 4096;
const WORKER_CHILDREN: usize = 64;

struct Reporter {
    auxiliary: OwnedFd,
    stdout_cloexec: bool,
    auxiliary_cloexec: bool,
    descriptor: GuardianFixtureDescriptor,
}

impl Reporter {
    fn prepare() -> Result<Self, GuardianFixtureError> {
        let stdout = std::io::stdout();
        let _ = rustix::net::sockopt::socket_type(&stdout)
            .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Reporter))?;
        let auxiliary = fcntl_dupfd_cloexec(&stdout, 3).map_err(io_error)?;
        fcntl_setfd(&stdout, FdFlags::CLOEXEC).map_err(io_error)?;
        let flags = fcntl_getfl(&auxiliary).map_err(io_error)?;
        fcntl_setfl(&auxiliary, flags | OFlags::NONBLOCK).map_err(io_error)?;
        let stdout_cloexec = fcntl_getfd(&stdout)
            .map_err(io_error)?
            .contains(FdFlags::CLOEXEC);
        let auxiliary_cloexec = fcntl_getfd(&auxiliary)
            .map_err(io_error)?
            .contains(FdFlags::CLOEXEC);
        if !stdout_cloexec || !auxiliary_cloexec {
            return Err(GuardianFixtureError::Coordination(
                GuardianFixtureFailure::Reporter,
            ));
        }
        let descriptor = descriptor_identity(&auxiliary)?;
        Ok(Self {
            descriptor,
            auxiliary,
            stdout_cloexec,
            auxiliary_cloexec,
        })
    }
}

pub(super) fn run(
    request: GuardianFixtureRequest<'_>,
) -> Result<GuardianFixtureObservation, GuardianFixtureError> {
    // Standard stdout is the sole safe inherited entry. No reporter mapping reaches any child.
    let reporter = Reporter::prepare()?;
    let helper = resolve_helper(request.guardian_path).map_err(production_error)?;
    let mut observer = MemoryObserver::prepare(Path::new("/proc"))?;
    let recipe = backend_recipe(&request);
    let until = match &request.scenario {
        GuardianFixtureScenario::ExactDeath { prefix, .. } => stage(*prefix),
        GuardianFixtureScenario::PendingDispatch { .. } => Stage::InitReady,
        GuardianFixtureScenario::Dispatched { .. } => Stage::Dispatched,
    };
    // A pending fixture retains the actual recipe for the unchanged production frame-send step.
    let pending_recipe = matches!(
        &request.scenario,
        GuardianFixtureScenario::PendingDispatch { .. }
    )
    .then(|| backend_recipe(&request));
    let started = start_sequence(
        recipe,
        &helper,
        request.original_stdin,
        request.report_path,
        Some(request.deadline),
        &mut observer,
        CAPTURE_LIMIT.saturating_mul(request.harnesses.get()),
        until,
    )
    .map_err(production_error)?;
    if let GuardianFixtureScenario::ExactDeath { prefix, death } = &request.scenario {
        let sent = exact_death(&started, &reporter, *prefix, *death, request.deadline);
        let Started {
            mut owner, state, ..
        } = started;
        drop(state);
        if let Some(owner) = &mut owner {
            let _ = owner.settle();
        }
        return Err(sent.err().unwrap_or(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Stage,
        )));
    }
    let Started {
        mut owner,
        state,
        publication,
        ..
    } = started;
    let retained = retained_owner(&mut owner).map_err(production_error)?;
    let observed = live(
        retained,
        state,
        Arc::clone(&publication),
        pending_recipe,
        &request,
        &mut observer,
        reporter.descriptor,
    );
    let raw = match observed {
        Ok(raw) => raw,
        Err(error) => RawObservation {
            lease_closing_published: publication.stage() == Some(Stage::LeaseClosing),
            lease: None,
            completed_close_ordinal: publication
                .close_order()
                .and_then(|order| order.completed_close),
            publication_ordinal: publication.close_order().map(|order| order.publication),
            worker_terminated: None,
            worker_identity: None,
            worker_descriptor: None,
            monitor_reporter_present: None,
            init_reporter_present: None,
            worker_reporter_present: None,
            backend_marker_present: None,
            coordination_failure: Some(match error {
                GuardianFixtureError::Coordination(failure) => failure,
                _ => GuardianFixtureFailure::Stage,
            }),
        },
    };
    // Every live success/refusal/observation failure immediately follows with the same cleanup.
    let settled = retained.settle();
    let cleanup = match (&settled.cleanup, &settled.monitor) {
        (Ok(()), Ok(_)) => GuardianFixtureCleanup::Confirmed,
        (Err(error), _) | (_, Err(error)) => GuardianFixtureCleanup::Unconfirmed {
            detail: error.to_string(),
        },
    };
    Ok(GuardianFixtureObservation {
        lease_closing_published: raw.lease_closing_published,
        lease: raw.lease,
        completed_close_ordinal: raw.completed_close_ordinal,
        publication_ordinal: raw.publication_ordinal,
        worker_terminated: raw.worker_terminated,
        worker_identity: raw.worker_identity,
        worker_descriptor: raw.worker_descriptor,
        monitor_reporter_present: raw.monitor_reporter_present,
        init_reporter_present: raw.init_reporter_present,
        worker_reporter_present: raw.worker_reporter_present,
        backend_marker_present: raw.backend_marker_present,
        coordination_failure: raw.coordination_failure,
        cleanup,
    })
}

fn backend_recipe(request: &GuardianFixtureRequest<'_>) -> BackendCommand {
    let mut recipe = BackendCommand::new(request.program);
    for argument in request.arguments {
        recipe.arg(argument);
    }
    recipe.current_dir(request.directory);
    for (name, value) in request.environment {
        recipe.env(name, value);
    }
    recipe
}

fn stage(prefix: GuardianFixturePrefix) -> Stage {
    match prefix {
        GuardianFixturePrefix::BeforeMonitor => Stage::BeforeMonitor,
        GuardianFixturePrefix::Bootstrap => Stage::Bootstrap,
        GuardianFixturePrefix::ClaimedGated => Stage::ClaimedGated,
        GuardianFixturePrefix::ClaimedBootstrap => Stage::ClaimedBootstrap,
        GuardianFixturePrefix::InitReady => Stage::InitReady,
    }
}

fn exact_death(
    started: &Started,
    reporter: &Reporter,
    prefix: GuardianFixturePrefix,
    death: GuardianFixtureDeath,
    deadline: Instant,
) -> Result<(), GuardianFixtureError> {
    if started.publication.stage() != Some(stage(prefix)) {
        return Err(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Stage,
        ));
    }
    let caller = identity(std::process::id())?;
    if death == GuardianFixtureDeath::CallerGroup
        && (getpgrp() != getpid() || getsid(None).map_err(io_error)? != getpid())
    {
        return Err(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::CallerGroup,
        ));
    }
    let (pin, descriptor, gate_retained) = match &started.owner {
        None => (
            GuardianFixturePin::NoInit,
            None,
            matches!(started.state, SequenceState::BeforeMonitor { .. }),
        ),
        Some(owner) if prefix == GuardianFixturePrefix::Bootstrap => {
            let before = identity(owner.monitor.id())?;
            if before.parent != caller.pid {
                return Err(GuardianFixtureError::Coordination(
                    GuardianFixtureFailure::Stage,
                ));
            }
            let handle =
                pidfd_open(valid_pid(before.pid)?, PidfdFlags::empty()).map_err(io_error)?;
            if identity(before.pid)? != before || terminated(&handle)? {
                return Err(GuardianFixtureError::Coordination(
                    GuardianFixtureFailure::Stage,
                ));
            }
            let descriptor = descriptor_identity(&handle)?;
            (
                GuardianFixturePin::Monitor {
                    identity: before,
                    descriptor,
                },
                Some(handle),
                owner.namespace.fixture_gate_retained(),
            )
        }
        Some(owner) => {
            let (pid, start, original, namespace) = owner.namespace.fixture_claim()?;
            let observed = identity(pid)?;
            if observed.start != start
                || Path::new(&observed.namespace) != namespace
                || terminated(original)?
            {
                return Err(GuardianFixtureError::Coordination(
                    GuardianFixtureFailure::Stage,
                ));
            }
            let handle = original.try_clone()?;
            let descriptor = descriptor_identity(&handle)?;
            (
                GuardianFixturePin::Init {
                    identity: observed,
                    descriptor,
                },
                Some(handle),
                owner.namespace.fixture_gate_retained(),
            )
        }
    };
    let monitor_reporter_present = started
        .owner
        .as_ref()
        .map(|owner| reporter_present(owner.monitor.id(), reporter.descriptor))
        .transpose()?;
    let init_reporter_present = match &pin {
        GuardianFixturePin::Init { identity, .. } => {
            Some(reporter_present(identity.pid, reporter.descriptor)?)
        }
        GuardianFixturePin::NoInit | GuardianFixturePin::Monitor { .. } => None,
    };
    let witness = GuardianFixtureDeathWitness {
        prefix,
        caller,
        pin,
        gate_retained,
        stdout_cloexec: reporter.stdout_cloexec,
        auxiliary_cloexec: reporter.auxiliary_cloexec,
        reporter_descriptor: reporter.descriptor,
        monitor_reporter_present,
        init_reporter_present,
    };
    let encoded = serde_json::to_vec(&witness)
        .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Stage))?;
    if encoded.len() > WITNESS_BYTES {
        return Err(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Overflow,
        ));
    }
    let rights: Vec<_> = descriptor.as_ref().map(AsFd::as_fd).into_iter().collect();
    Transport::fixture_reporter(&reporter.auxiliary)
        .send(&witness, &rights, deadline)
        .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Reporter))?;
    match death {
        GuardianFixtureDeath::Caller => kill_process(getpid(), Signal::KILL).map_err(io_error)?,
        GuardianFixtureDeath::CallerGroup => {
            kill_process_group(getpgrp(), Signal::KILL).map_err(io_error)?
        }
    }
    // No successful intentional-death operation returns, even if a broken platform returns here.
    std::process::exit(97)
}

struct RawObservation {
    lease_closing_published: bool,
    lease: Option<GuardianFixtureLeaseObservation>,
    completed_close_ordinal: Option<u64>,
    publication_ordinal: Option<u64>,
    worker_terminated: Option<bool>,
    worker_identity: Option<GuardianFixtureIdentity>,
    worker_descriptor: Option<GuardianFixtureDescriptor>,
    monitor_reporter_present: Option<bool>,
    init_reporter_present: Option<bool>,
    worker_reporter_present: Option<bool>,
    backend_marker_present: Option<bool>,
    coordination_failure: Option<GuardianFixtureFailure>,
}

fn live(
    owner: &mut RunOwner,
    state: SequenceState,
    publication: Arc<Publication>,
    pending_recipe: Option<BackendCommand>,
    request: &GuardianFixtureRequest<'_>,
    observer: &mut MemoryObserver,
    reporter: GuardianFixtureDescriptor,
) -> Result<RawObservation, GuardianFixtureError> {
    let (marker, worker, lease, continuation) = match (&request.scenario, state) {
        (
            GuardianFixtureScenario::PendingDispatch { backend_marker },
            SequenceState::InitReady(ready),
        ) => {
            let (_, _, original, _) = owner.namespace.fixture_claim()?;
            let pin = original.try_clone()?;
            check_resources(owner, observer, request)?;
            stop_init(owner, &pin, request.deadline)?;
            check_resources(owner, observer, request)?;
            let recipe = pending_recipe.ok_or(GuardianFixtureError::Coordination(
                GuardianFixtureFailure::Stage,
            ))?;
            let resume = continuation(Arc::clone(&publication), pin, request.deadline)?;
            let pending = match ready.send_dispatch(
                recipe,
                request.original_stdin,
                vec![request.report_path.as_os_str().to_owned()],
            ) {
                Ok(pending) => pending,
                Err(error) => {
                    resume.stop.store(true, Ordering::Release);
                    let _ = resume.thread.join();
                    return Err(production_error(super::stage_refusal(error)));
                }
            };
            (backend_marker, None, pending.into_lease(), Some(resume))
        }
        (
            GuardianFixtureScenario::Dispatched {
                backend_marker,
                worker_acknowledgement,
            },
            SequenceState::Dispatched(dispatched),
        ) => {
            let pinned = wait_worker(owner, worker_acknowledgement, request, observer)?;
            (backend_marker, Some(pinned), dispatched.into_lease(), None)
        }
        _ => {
            return Err(GuardianFixtureError::Coordination(
                GuardianFixtureFailure::Stage,
            ))
        }
    };
    // Observation errors are sealed as unavailability; they cannot detach an owned continuation.
    let monitor_reporter_present = reporter_present(owner.monitor.id(), reporter).ok();
    let init_reporter_present = owner
        .namespace
        .fixture_claim()
        .ok()
        .and_then(|(init, _, _, _)| reporter_present(init, reporter).ok());
    let worker_reporter_present = worker
        .as_ref()
        .and_then(|worker| reporter_present(worker.identity.pid, reporter).ok());
    let observation = owner.close_lease_and_observe(lease);
    // Seal raw facts before settle can independently signal INIT or kill an acknowledged worker.
    let order = publication.close_order();
    let (worker_terminated, worker_failure) = match worker
        .as_ref()
        .map(|worker| terminated(&worker.handle))
        .transpose()
    {
        Ok(observed) => (observed, None),
        Err(_) => (None, Some(GuardianFixtureFailure::Worker)),
    };
    let backend_marker_present = match fs::metadata(marker) {
        Ok(_) => Some(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Some(false),
        Err(_) => None,
    };
    let coordination_failure = if let Some(resume) = continuation {
        resume.stop.store(true, Ordering::Release);
        match resume.thread.join() {
            Ok(Ok(())) => None,
            Ok(Err(failure)) => Some(failure),
            Err(_) => Some(GuardianFixtureFailure::Resume),
        }
    } else {
        None
    };
    let lease = Some(match observation {
        LeaseCloseObservation::ConfirmedTermination => {
            GuardianFixtureLeaseObservation::ConfirmedTermination
        }
        LeaseCloseObservation::EscalationRequired => {
            GuardianFixtureLeaseObservation::EscalationRequired
        }
        LeaseCloseObservation::Unavailable(error) => GuardianFixtureLeaseObservation::Unavailable {
            detail: error.to_string(),
        },
    });
    Ok(RawObservation {
        lease_closing_published: publication.stage() == Some(Stage::LeaseClosing),
        lease,
        completed_close_ordinal: order.and_then(|order| order.completed_close),
        publication_ordinal: order.map(|order| order.publication),
        worker_terminated,
        worker_identity: worker.as_ref().map(|worker| worker.identity.clone()),
        worker_descriptor: worker.as_ref().map(|worker| worker.descriptor),
        monitor_reporter_present,
        init_reporter_present,
        worker_reporter_present,
        backend_marker_present,
        coordination_failure: coordination_failure
            .or(worker_failure)
            .or_else(|| {
                (monitor_reporter_present.is_none() || init_reporter_present.is_none())
                    .then_some(GuardianFixtureFailure::Stage)
            })
            .or_else(|| {
                (worker.is_some() && worker_reporter_present.is_none())
                    .then_some(GuardianFixtureFailure::Worker)
            })
            .or_else(|| {
                backend_marker_present
                    .is_none()
                    .then_some(GuardianFixtureFailure::Stage)
            })
            .or_else(|| {
                order
                    .is_none()
                    .then_some(GuardianFixtureFailure::Publication)
            }),
    })
}

struct Continuation {
    stop: Arc<AtomicBool>,
    thread: thread::JoinHandle<Result<(), GuardianFixtureFailure>>,
}
fn continuation(
    publication: Arc<Publication>,
    pin: OwnedFd,
    deadline: Instant,
) -> Result<Continuation, GuardianFixtureError> {
    let stop = Arc::new(AtomicBool::new(false));
    let stopped = Arc::clone(&stop);
    let thread = thread::Builder::new()
        .name("guardian-fixture-resume".to_owned())
        .spawn(move || loop {
            if publication.stage() == Some(Stage::LeaseClosing)
                && publication.close_order().is_some()
            {
                return pidfd_send_signal(&pin, Signal::CONT)
                    .map_err(|_| GuardianFixtureFailure::Resume);
            }
            if stopped.load(Ordering::Acquire) || Instant::now() >= deadline {
                return Err(GuardianFixtureFailure::Publication);
            }
            thread::park_timeout(Duration::from_millis(1));
        })?;
    Ok(Continuation { stop, thread })
}

fn stop_init(
    owner: &RunOwner,
    pin: &OwnedFd,
    deadline: Instant,
) -> Result<(), GuardianFixtureError> {
    let (pid, start, _, namespace) = owner.namespace.fixture_claim()?;
    pidfd_send_signal(pin, Signal::STOP).map_err(io_error)?;
    loop {
        let observed = identity(pid)?;
        if observed.start != start
            || Path::new(&observed.namespace) != namespace
            || terminated(pin)?
        {
            return Err(GuardianFixtureError::Coordination(
                GuardianFixtureFailure::Stop,
            ));
        }
        let bytes = bounded(&PathBuf::from(format!("/proc/{pid}/stat")))?;
        let close = bytes.iter().rposition(|byte| *byte == b')').ok_or(
            GuardianFixtureError::Coordination(GuardianFixtureFailure::Stop),
        )?;
        if bytes.get(close + 2) == Some(&b'T') {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(GuardianFixtureError::Coordination(
                GuardianFixtureFailure::Stop,
            ));
        }
        thread::yield_now();
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkerAcknowledgement {
    namespace_pid: u32,
    start: u64,
}

struct PinnedWorker {
    handle: OwnedFd,
    identity: GuardianFixtureIdentity,
    descriptor: GuardianFixtureDescriptor,
}

fn wait_worker(
    owner: &RunOwner,
    path: &Path,
    request: &GuardianFixtureRequest<'_>,
    observer: &mut MemoryObserver,
) -> Result<PinnedWorker, GuardianFixtureError> {
    let deadline = request.deadline;
    let (init, _, _, namespace) = owner.namespace.fixture_claim()?;
    loop {
        check_resources(owner, observer, request)?;
        match bounded(path) {
            Ok(bytes) => {
                let acknowledgement: WorkerAcknowledgement = serde_json::from_slice(&bytes)
                    .map_err(|_| {
                        GuardianFixtureError::Coordination(GuardianFixtureFailure::Worker)
                    })?;
                let children =
                    bounded(&PathBuf::from(format!("/proc/{init}/task/{init}/children")))?;
                let text = std::str::from_utf8(&children).map_err(|_| {
                    GuardianFixtureError::Coordination(GuardianFixtureFailure::Worker)
                })?;
                for (index, child) in text.split_whitespace().enumerate() {
                    if index >= WORKER_CHILDREN {
                        return Err(GuardianFixtureError::Coordination(
                            GuardianFixtureFailure::Overflow,
                        ));
                    }
                    let pid: u32 = child.parse().map_err(|_| {
                        GuardianFixtureError::Coordination(GuardianFixtureFailure::Worker)
                    })?;
                    let observed = identity(pid)?;
                    if observed.parent != init
                        || Path::new(&observed.namespace) != namespace
                        || observed.start != acknowledgement.start
                    {
                        continue;
                    }
                    let status = bounded(&PathBuf::from(format!("/proc/{pid}/status")))?;
                    // Process names in status are opaque bytes; decode only numeric NSpid.
                    let namespace_pid = status
                        .split(|byte| *byte == b'\n')
                        .find_map(|line| line.strip_prefix(b"NSpid:"))
                        .and_then(|value| std::str::from_utf8(value).ok())
                        .and_then(|value| value.split_whitespace().last())
                        .and_then(|value| value.parse::<u32>().ok());
                    if namespace_pid != Some(acknowledgement.namespace_pid) {
                        continue;
                    }
                    let pin = pidfd_open(valid_pid(pid)?, PidfdFlags::empty()).map_err(io_error)?;
                    if identity(pid)? == observed && !terminated(&pin)? {
                        let descriptor = descriptor_identity(&pin)?;
                        return Ok(PinnedWorker {
                            handle: pin,
                            identity: observed,
                            descriptor,
                        });
                    }
                    return Err(GuardianFixtureError::Coordination(
                        GuardianFixtureFailure::Worker,
                    ));
                }
            }
            Err(GuardianFixtureError::Io(error))
                if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        if Instant::now() >= deadline {
            return Err(GuardianFixtureError::Coordination(
                GuardianFixtureFailure::Deadline,
            ));
        }
        thread::yield_now();
    }
}

fn check_resources(
    owner: &RunOwner,
    observer: &mut MemoryObserver,
    request: &GuardianFixtureRequest<'_>,
) -> Result<(), GuardianFixtureError> {
    if owner.flags.failed.load(Ordering::Acquire) {
        return Err(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Resource,
        ));
    }
    if Instant::now() >= request.deadline {
        return Err(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Deadline,
        ));
    }
    let (init, _, _, _) = owner.namespace.fixture_claim()?;
    match observer.observe(init) {
        Ok(bytes) if bytes <= request.ceilings.memory_bytes.get() => Ok(()),
        Ok(_) | Err(_) => Err(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Resource,
        )),
    }
}

fn bounded(path: &Path) -> Result<Vec<u8>, GuardianFixtureError> {
    let mut bytes = Vec::new();
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(
            i32::try_from((OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC).bits())
                .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Stage))?,
        )
        .open(path)?;
    if !file.metadata()?.is_file() {
        return Err(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Stage,
        ));
    }
    file.take(
        u64::try_from(PROC_BYTES)
            .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Overflow))?
            + 1,
    )
    .read_to_end(&mut bytes)?;
    if bytes.len() > PROC_BYTES {
        return Err(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Overflow,
        ));
    }
    Ok(bytes)
}

fn identity(pid: u32) -> Result<GuardianFixtureIdentity, GuardianFixtureError> {
    let directory = PathBuf::from(format!("/proc/{pid}"));
    let stat = bounded(&directory.join("stat"))?;
    let closing =
        stat.iter()
            .rposition(|byte| *byte == b')')
            .ok_or(GuardianFixtureError::Coordination(
                GuardianFixtureFailure::Stage,
            ))?;
    let tail = std::str::from_utf8(stat.get(closing + 1..).ok_or(
        GuardianFixtureError::Coordination(GuardianFixtureFailure::Stage),
    )?)
    .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Stage))?;
    let fields: Vec<_> = tail.split_whitespace().take(21).collect();
    let number = |index: usize| {
        fields.get(index).ok_or(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Stage,
        ))
    };
    let parent = number(1)?
        .parse()
        .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Stage))?;
    let group = number(2)?
        .parse()
        .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Stage))?;
    let session = number(3)?
        .parse()
        .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Stage))?;
    let start = number(19)?
        .parse()
        .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Stage))?;
    let namespace = fs::read_link(directory.join("ns/pid"))?
        .into_os_string()
        .into_string()
        .map_err(|_| GuardianFixtureError::Coordination(GuardianFixtureFailure::Stage))?;
    Ok(GuardianFixtureIdentity {
        pid,
        start,
        parent,
        namespace,
        group,
        session,
    })
}

fn reporter_present(
    pid: u32,
    reporter: GuardianFixtureDescriptor,
) -> Result<bool, GuardianFixtureError> {
    let expected = format!("socket:[{}]", reporter.inode);
    for (index, entry) in fs::read_dir(format!("/proc/{pid}/fd"))?.enumerate() {
        if index >= 128 {
            return Err(GuardianFixtureError::Coordination(
                GuardianFixtureFailure::Overflow,
            ));
        }
        let entry = entry?;
        match fs::read_link(entry.path()) {
            Ok(link) if link == Path::new(&expected) => return Ok(true),
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(false)
}

fn descriptor_identity(pin: &OwnedFd) -> Result<GuardianFixtureDescriptor, GuardianFixtureError> {
    let stat = fstat(pin).map_err(io_error)?;
    Ok(GuardianFixtureDescriptor {
        device: stat.st_dev,
        inode: stat.st_ino,
    })
}
fn terminated(pin: &OwnedFd) -> Result<bool, GuardianFixtureError> {
    let mut events = [PollFd::new(pin, PollFlags::IN)];
    poll(&mut events, Some(&Timespec::default())).map_err(io_error)?;
    if events[0]
        .revents()
        .intersects(PollFlags::ERR | PollFlags::NVAL)
    {
        return Err(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Stage,
        ));
    }
    Ok(events[0].revents().contains(PollFlags::IN))
}
fn valid_pid(pid: u32) -> Result<Pid, GuardianFixtureError> {
    i32::try_from(pid)
        .ok()
        .and_then(Pid::from_raw)
        .ok_or(GuardianFixtureError::Coordination(
            GuardianFixtureFailure::Stage,
        ))
}
fn io_error(error: rustix::io::Errno) -> GuardianFixtureError {
    GuardianFixtureError::Io(error.into())
}
fn production_error(error: BoundedLaunchError) -> GuardianFixtureError {
    match error {
        BoundedLaunchError::Io(error) => GuardianFixtureError::Io(error),
        BoundedLaunchError::Guardian { kind, detail } => {
            GuardianFixtureError::Guardian { kind, detail }
        }
        BoundedLaunchError::Unavailable(error) => GuardianFixtureError::Io(error),
    }
}

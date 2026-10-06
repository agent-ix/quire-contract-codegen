//! Namespace INIT bootstrap, authenticated Dispatch and kernel-owned descendant supervision.
//!
//! This process never replaces itself with the backend. Its termination remains INIT death,
//! which tears down even unsampled, reparented and nested-namespace descendants. No diagnostic
//! bytes enter stdout/stderr: both are inherited solely by the backend's bounded captures.

use std::{
    ffi::OsString,
    fs::{self, File},
    io::{self, Read},
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use command_fds::{CommandFdExt, FdMapping};

use rustix::process::{getpid, getuid, waitpid, Pid, WaitOptions};

use super::{
    control::{ControlError, IncrementalReceive, IncrementalSend, PreparedFrame, Transport},
    protocol::{
        BackendExit, BuildIdentity, CallerControl, GuardianControl, GuardianRefusal, StdinControl,
    },
};

const BOOTSTRAP_CAP: Duration = Duration::from_secs(3);
const SUPERVISION_TICK: Duration = Duration::from_millis(20);
const REAP_WORK: usize = 64;
const ARTIFACT_COUNT: usize = 8;

#[derive(Debug)]
pub(super) enum GuardianError {
    Control(ControlError),
    Refusal(GuardianRefusal),
    Io(io::Error),
}

impl std::fmt::Display for GuardianError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Control(error) => write!(formatter, "{error}"),
            Self::Refusal(reason) => write!(formatter, "{reason:?}"),
            Self::Io(error) => write!(formatter, "guardian backend I/O: {error}"),
        }
    }
}

impl std::error::Error for GuardianError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Control(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::Refusal(_) => None,
        }
    }
}

impl From<ControlError> for GuardianError {
    fn from(error: ControlError) -> Self {
        Self::Control(error)
    }
}

impl From<io::Error> for GuardianError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Called only by this package's helper entry with its actual compiled library identity.
pub(super) fn run(identity: BuildIdentity) -> Result<(), GuardianError> {
    let stdin = std::io::stdin();
    let transport = Transport::from_guardian_stdin(&stdin)?;
    let deadline = Instant::now() + BOOTSTRAP_CAP;
    let result = supervise(&transport, identity, deadline);
    if let Err(GuardianError::Refusal(reason)) = &result {
        // Typed control is the sole diagnostic channel. A closed peer needs no diagnostic.
        let delivered = transport
            .send(&GuardianControl::Refused { reason: *reason }, &[], deadline)
            .is_ok();
        if delivered && *reason == GuardianRefusal::BuildIdentityMismatch {
            // Do not race the refusal frame with our own EOF. The live caller can consume its
            // typed refusal and close its exclusive lease; no refused path can authorize work.
            // Both the original bootstrap cap and the finite pending-control work remain in force.
            let mut refused_records = 0;
            while refused_records < 8 && Instant::now() < deadline {
                match transport.pending_control(SUPERVISION_TICK) {
                    Ok(false) => continue,
                    Ok(true) => {
                        refused_records += 1;
                        if transport
                            .receive::<CallerControl>(CallerControl::rights_count, deadline)
                            .is_err()
                        {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
    result
}

/// The one authenticated C/I admission path prepares the exact original backend command.
/// Its child has not been spawned: entry-specific report mapping can still be added safely.
pub(super) struct BackendAdmission {
    command: Command,
    authority: super::protocol::RunAuthority,
    artifacts: GuardianArtifacts,
}

impl BackendAdmission {
    /// Only authenticated I entry may export the safely reopened original O report writer to
    /// the exact backend child. Parent ownership stays CLOEXEC; no unrelated exec inherits it.
    fn into_pipe_command(mut self, writer: &File) -> Result<Self, GuardianError> {
        let descriptor = writer.try_clone()?;
        self.command
            .fd_mappings(vec![FdMapping {
                parent_fd: descriptor.into(),
                child_fd: super::report_storage::REPORT_SLOT,
            }])
            .map_err(|error| {
                io::Error::other(format!("report writer child mapping failed: {error}"))
            })?;
        Ok(self)
    }
}

/// Real I's authenticated O-origin entry shares the exact C lease admission recipe. Policy
/// installation remains the separately SPEC-gated backend boundary; this prepares no child.
pub(super) fn prepare_inner_backend(
    input: &super::role_bootstrap::InnerInput,
) -> Result<BackendAdmission, GuardianError> {
    super::creator::require_live(&input.outer_pin)?;
    super::creator::require_live(&input.caller_pin)?;
    input.outer_bootstrap.transport().refuse_observable_eof()?;
    let deadline = input
        .settings
        .startup_deadline()
        .map_err(io::Error::other)?
        .min(
            input
                .settings
                .setup_deadline
                .local()
                .map_err(io::Error::other)?,
        );
    let admitted = admit_backend(
        &input.caller_lease.transport(),
        input.settings.identity,
        deadline,
    )?;
    if admitted.authority != input.settings.authority {
        return Err(GuardianError::Refusal(GuardianRefusal::ReplayedAuthority));
    }
    super::creator::require_live(&input.outer_pin)?;
    super::creator::require_live(&input.caller_pin)?;
    input.outer_bootstrap.transport().refuse_observable_eof()?;
    admitted.into_pipe_command(&input.writer)
}

fn admit_backend(
    transport: &Transport<'_>,
    identity: BuildIdentity,
    startup_deadline: Instant,
) -> Result<BackendAdmission, GuardianError> {
    let init = getpid();
    if init.as_raw_nonzero().get() != 1 {
        return Err(GuardianError::Refusal(GuardianRefusal::NotNamespaceInit));
    }
    if !isolated_init_session()? {
        return Err(GuardianError::Refusal(GuardianRefusal::SessionNotIsolated));
    }
    // Credentials are translated by the guardian's user namespace. An unmapped outside UID
    // becomes the overflow UID and cannot establish the actual mapped original-caller UID.
    let creator = transport.creator_credentials()?;
    if creator.uid != getuid().as_raw() {
        return Err(GuardianError::Refusal(GuardianRefusal::CreatorUidMismatch));
    }
    let hello =
        transport.receive::<CallerControl>(CallerControl::rights_count, startup_deadline)?;
    let CallerControl::Hello {
        identity: expected,
        authority,
    } = hello.control
    else {
        return Err(GuardianError::Refusal(GuardianRefusal::UnexpectedControl));
    };
    if expected != identity {
        return Err(GuardianError::Refusal(
            GuardianRefusal::BuildIdentityMismatch,
        ));
    }
    transport.send(
        &GuardianControl::Ready {
            identity,
            authority,
            mapped_uid: getuid().as_raw(),
            creator_pid: creator.pid,
        },
        &[],
        startup_deadline,
    )?;
    let dispatch =
        transport.receive::<CallerControl>(CallerControl::rights_count, startup_deadline)?;
    let CallerControl::Dispatch {
        authority: received_authority,
        command,
        stdin,
        cleanup_paths,
    } = dispatch.control
    else {
        return Err(GuardianError::Refusal(GuardianRefusal::UnexpectedControl));
    };
    if received_authority != authority {
        return Err(GuardianError::Refusal(GuardianRefusal::ReplayedAuthority));
    }
    if cleanup_paths.len() > ARTIFACT_COUNT {
        return Err(GuardianError::Refusal(GuardianRefusal::InvalidControl));
    }
    let artifacts = GuardianArtifacts(cleanup_paths);
    let mut backend = command.into_command();
    match stdin {
        StdinControl::Open => {
            let descriptor = dispatch
                .rights
                .into_iter()
                .next()
                .ok_or(GuardianError::Refusal(GuardianRefusal::InvalidControl))?;
            backend.stdin(Stdio::from(descriptor));
        }
        StdinControl::Closed => {
            // Control fd0 is CLOEXEC. Inherit leaves fd0 closed at backend exec entry.
            backend.stdin(Stdio::inherit());
        }
    }
    backend.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    Ok(BackendAdmission {
        command: backend,
        authority,
        artifacts,
    })
}

fn supervise(
    transport: &Transport<'_>,
    identity: BuildIdentity,
    startup_deadline: Instant,
) -> Result<(), GuardianError> {
    let admitted = admit_backend(transport, identity, startup_deadline)?;
    supervise_admitted(transport, admitted, startup_deadline)
}

/// Single-thread I's real direct-backend wait state, shared by the ordinary supervision core
/// and the O/C control actor. A reaped PID matches only the actual unreaped spawned backend;
/// duplicate completion is refusal, not a second successful backend exit.
struct BackendReaper {
    pid: Pid,
    completed: bool,
}

/// I retains the actual direct Child and both trusted channels. The safe backend installer
/// supplies that Child only after installing its policy at the same positively owned PID;
/// this actor neither starts an unfiltered backend nor invents completion from a spawn recipe.
pub(super) struct InnerBackend {
    input: super::role_bootstrap::InnerInput,
    child: std::process::Child,
    reaper: Option<BackendReaper>,
    _artifacts: GuardianArtifacts,
    owner_receive: IncrementalReceive,
    state: InnerBackendState,
}

enum InnerBackendState {
    Dispatching(InnerEvent),
    Running,
    Completing { event: InnerEvent, observed: bool },
    AwaitLeaseClose,
}

/// Each event has two independent finite send cursors. A stopped receiver cannot block I's
/// primary original lease check or O's independently running accounting/collector actor.
struct InnerEvent {
    caller: IncrementalSend,
    outer: IncrementalSend,
    caller_sent: bool,
    outer_sent: bool,
    deadline: Instant,
}

impl InnerEvent {
    fn prepare(control: &GuardianControl, deadline: Instant) -> Result<Self, GuardianError> {
        Ok(Self {
            caller: IncrementalSend::new(PreparedFrame::encode(control)?),
            outer: IncrementalSend::new(PreparedFrame::encode(control)?),
            caller_sent: false,
            outer_sent: false,
            deadline,
        })
    }

    fn advance(
        &mut self,
        input: &super::role_bootstrap::InnerInput,
        authorize_caller: bool,
    ) -> Result<bool, GuardianError> {
        // O gets its genuine I-origin event directly; C's receipt never substitutes for it.
        if !self.outer_sent {
            self.outer_sent =
                self.outer
                    .advance(&input.outer_bootstrap.transport(), &[], self.deadline)?;
        }
        if authorize_caller && !self.caller_sent {
            self.caller_sent =
                self.caller
                    .advance(&input.caller_lease.transport(), &[], self.deadline)?;
        }
        Ok(self.outer_sent && self.caller_sent)
    }
}

pub(super) enum InnerBackendProgress {
    Running,
    /// Real C lease EOF. I entry must terminate; this value alone proves no namespace settlement.
    LeaseClosed,
}

impl InnerBackend {
    /// Prepare the Dispatch event before arbitrary code starts. The final exec installer owns
    /// the actual process start; this bounded reservation can fail without creating a backend.
    pub(super) fn prepare_dispatch_event(
        admitted: &BackendAdmission,
        deadline: Instant,
    ) -> Result<PreparedInnerDispatch, GuardianError> {
        Ok(PreparedInnerDispatch {
            event: InnerEvent::prepare(
                &GuardianControl::Dispatched {
                    authority: admitted.authority,
                },
                deadline,
            )?,
            owner_receive: IncrementalReceive::prepare()?,
        })
    }

    /// Ownership is stored before the next fallible pin/control/reap operation. No error result
    /// may drop the actual spawned Child and then pretend it was never created.
    pub(super) fn from_spawned(
        input: super::role_bootstrap::InnerInput,
        admitted: BackendAdmission,
        child: std::process::Child,
        dispatch: PreparedInnerDispatch,
    ) -> Self {
        Self {
            input,
            child,
            reaper: None,
            _artifacts: admitted.artifacts,
            owner_receive: dispatch.owner_receive,
            state: InnerBackendState::Dispatching(dispatch.event),
        }
    }

    /// One finite actor step. Caller EOF remains primary even with queued output/events; no
    /// post-Dispatch caller frame can authorize new work. Backend completion leaves I alive
    /// until real lease close, preserving namespace-owned descendant teardown.
    pub(super) fn tick(&mut self) -> Result<InnerBackendProgress, GuardianError> {
        match self
            .input
            .caller_lease
            .transport()
            .pending_control(Duration::ZERO)
        {
            Err(ControlError::Eof) => return Ok(InnerBackendProgress::LeaseClosed),
            Err(error) => return Err(error.into()),
            Ok(true) => return Err(GuardianError::Refusal(GuardianRefusal::UnexpectedControl)),
            Ok(false) => {}
        }
        super::creator::require_live(&self.input.outer_pin)?;
        super::creator::require_live(&self.input.caller_pin)?;
        self.input
            .outer_bootstrap
            .transport()
            .refuse_observable_eof()?;
        if !matches!(
            self.state,
            InnerBackendState::Completing {
                observed: false,
                ..
            }
        ) && self
            .input
            .outer_bootstrap
            .transport()
            .pending_control(Duration::ZERO)?
        {
            // An early or duplicate acknowledgment is an unauthorized lifecycle record.
            return Err(GuardianError::Refusal(GuardianRefusal::UnexpectedControl));
        }
        if self.reaper.is_none() {
            self.reaper = Some(BackendReaper::from_child(&self.child)?);
        }
        match &mut self.state {
            InnerBackendState::Dispatching(event) => {
                if event.advance(&self.input, true)? {
                    self.state = InnerBackendState::Running;
                }
            }
            InnerBackendState::Running => {
                let reaper = self.reaper.as_mut().ok_or(GuardianError::Refusal(
                    GuardianRefusal::BackendObservationFailed,
                ))?;
                if let Some(outcome) = reaper.tick()? {
                    let cap = Instant::now()
                        .checked_add(super::role_deadline::SETTLE_RESERVE)
                        .ok_or(GuardianError::Refusal(GuardianRefusal::InvalidControl))?;
                    let deadline = self
                        .input
                        .settings
                        .identity_deadline()
                        .map_err(io::Error::other)?
                        .map_or(cap, |original| original.min(cap));
                    self.state = InnerBackendState::Completing {
                        event: InnerEvent::prepare(
                            &GuardianControl::Completed {
                                authority: self.input.settings.authority,
                                outcome,
                            },
                            deadline,
                        )?,
                        observed: false,
                    };
                }
            }
            InnerBackendState::Completing { event, observed } => {
                self.reaper
                    .as_mut()
                    .ok_or(GuardianError::Refusal(
                        GuardianRefusal::BackendObservationFailed,
                    ))?
                    .tick()?;
                event.advance(&self.input, false)?;
                if !event.outer_sent
                    && self
                        .input
                        .outer_bootstrap
                        .transport()
                        .pending_control(Duration::ZERO)?
                {
                    return Err(GuardianError::Refusal(GuardianRefusal::UnexpectedControl));
                }
                if event.outer_sent && !*observed {
                    if let Some(received) = self
                        .owner_receive
                        .advance::<super::role_protocol::InnerOwnerControl>(
                        &self.input.outer_bootstrap.transport(),
                        super::role_protocol::InnerOwnerControl::rights_count,
                        event.deadline,
                    )? {
                        let super::role_protocol::InnerOwnerControl::CompletionObserved {
                            authority,
                        } = received.control;
                        if authority != self.input.settings.authority {
                            return Err(GuardianError::Refusal(GuardianRefusal::ReplayedAuthority));
                        }
                        *observed = true;
                    }
                }
                if *observed
                    && self
                        .input
                        .outer_bootstrap
                        .transport()
                        .pending_control(Duration::ZERO)?
                {
                    return Err(GuardianError::Refusal(GuardianRefusal::UnexpectedControl));
                }
                if *observed && event.advance(&self.input, true)? {
                    self.state = InnerBackendState::AwaitLeaseClose;
                }
            }
            InnerBackendState::AwaitLeaseClose => {
                self.reaper
                    .as_mut()
                    .ok_or(GuardianError::Refusal(
                        GuardianRefusal::BackendObservationFailed,
                    ))?
                    .tick()?;
            }
        }
        match self.input.caller_lease.transport().refuse_observable_eof() {
            Ok(()) => {}
            Err(ControlError::Eof) => return Ok(InnerBackendProgress::LeaseClosed),
            Err(error) => return Err(error.into()),
        }
        Ok(InnerBackendProgress::Running)
    }
}

/// Private pre-spawn reservation, not a Dispatched fact or a backend ownership handle.
pub(super) struct PreparedInnerDispatch {
    event: InnerEvent,
    owner_receive: IncrementalReceive,
}

impl BackendReaper {
    fn from_child(child: &std::process::Child) -> Result<Self, GuardianError> {
        let pid = i32::try_from(child.id())
            .ok()
            .and_then(Pid::from_raw)
            .ok_or(GuardianError::Refusal(
                GuardianRefusal::BackendObservationFailed,
            ))?;
        Ok(Self {
            pid,
            completed: false,
        })
    }

    /// Bounded real waitpid work; descendant exits do not become direct-backend completion.
    fn tick(&mut self) -> Result<Option<BackendExit>, GuardianError> {
        for _ in 0..REAP_WORK {
            let (pid, status) = match waitpid(None, WaitOptions::NOHANG) {
                Ok(Some(status)) => status,
                Ok(None) | Err(rustix::io::Errno::CHILD) => return Ok(None),
                Err(rustix::io::Errno::INTR) => continue,
                Err(error) => return Err(GuardianError::Io(error.into())),
            };
            if pid != self.pid {
                continue;
            }
            if self.completed {
                return Err(GuardianError::Refusal(
                    GuardianRefusal::BackendObservationFailed,
                ));
            }
            let outcome = if let Some(code) = status.exit_status() {
                BackendExit::Code(code)
            } else if let Some(signal) = status.terminating_signal() {
                BackendExit::Signal(signal)
            } else {
                return Err(GuardianError::Refusal(
                    GuardianRefusal::BackendObservationFailed,
                ));
            };
            self.completed = true;
            return Ok(Some(outcome));
        }
        Ok(None)
    }
}

fn supervise_admitted(
    transport: &Transport<'_>,
    admitted: BackendAdmission,
    startup_deadline: Instant,
) -> Result<(), GuardianError> {
    let BackendAdmission {
        command: mut backend,
        authority,
        artifacts: _artifacts,
    } = admitted;
    transport.refuse_observable_eof()?;
    let child = backend
        .spawn()
        .map_err(|_| GuardianError::Refusal(GuardianRefusal::BackendSpawnFailed))?;
    let mut reaper = BackendReaper::from_child(&child)?;
    transport.send(
        &GuardianControl::Dispatched { authority },
        &[],
        startup_deadline,
    )?;
    loop {
        // No post-Dispatch input is authorized. Pending controls refuse; EOF exits INIT.
        match transport.pending_control(SUPERVISION_TICK) {
            Err(ControlError::Eof) => return Ok(()),
            Err(error) => return Err(error.into()),
            Ok(true) => return Err(GuardianError::Refusal(GuardianRefusal::UnexpectedControl)),
            Ok(false) => {}
        }
        if let Some(outcome) = reaper.tick()? {
            transport.send(
                &GuardianControl::Completed { authority, outcome },
                &[],
                Instant::now() + BOOTSTRAP_CAP,
            )?;
        }
        // After completion remain INIT and retain descendants until the caller closes its lease.
        // The live caller bounded-reads the report before close; a vanished caller owns no result.
    }
}

struct GuardianArtifacts(Vec<OsString>);

fn isolated_init_session() -> io::Result<bool> {
    let mut bytes = Vec::new();
    fs::File::open("/proc/self/stat")?
        .take(4097)
        .read_to_end(&mut bytes)?;
    if bytes.len() > 4096 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "guardian stat exceeds bound",
        ));
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "guardian stat encoding invalid")
    })?;
    let fields = text
        .rsplit_once(')')
        .map(|(_, fields)| fields)
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "guardian stat fields missing")
        })?;
    let mut fields = fields.split_whitespace().skip(2);
    // An outside process group/session can appear as zero in this PID namespace. Observe it
    // without treating zero as a valid nonzero process identity.
    Ok(fields.next() == Some("1") && fields.next() == Some("1") && fields.next() == Some("0"))
}

impl Drop for GuardianArtifacts {
    fn drop(&mut self) {
        for path in &self.0 {
            let _ = fs::remove_file(Path::new(path));
        }
    }
}

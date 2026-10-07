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
    control::{
        ControlError, FrameStorage, IncrementalReceive, IncrementalSend, PreparedFrame,
        PreparedReceive, Transport,
    },
    protocol::{
        BackendExit, BuildIdentity, CallerControl, GuardianControl, GuardianRefusal, StdinControl,
    },
    role_deadline::{StopOrigin, StopStamp, StopTimeline},
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
    super::owner_protection::protect_inner()?;
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

/// Actual first-party I authentication before policy installation. This value grants no Ready
/// or Dispatch; it retains only the already checked original C/build/mapping/run identity.
pub(super) struct AuthenticatedHello {
    identity: BuildIdentity,
    authority: super::protocol::RunAuthority,
    mapped_uid: u32,
    creator_pid: i32,
}

impl AuthenticatedHello {
    pub(super) fn authority(&self) -> super::protocol::RunAuthority {
        self.authority
    }

    /// The production installer path cannot publish I Ready from spawn/EOF or a caller-supplied
    /// boolean. Only its actual same-PID child's authenticated policy admission mints this token.
    pub(super) fn publish_installed_ready(
        self,
        admission: super::backend_installer::InstallerAdmission,
        transport: &Transport<'_>,
        deadline: Instant,
    ) -> Result<ReadyAdmission, GuardianError> {
        if admission.authority() != self.authority {
            return Err(GuardianError::Refusal(GuardianRefusal::ReplayedAuthority));
        }
        self.publish_ready(transport, deadline)
    }

    /// The new I entry must authenticate policy-ready before this transition. This shared
    /// operation checks lease/identity readiness only; it does not establish policy success.
    fn publish_ready(
        self,
        transport: &Transport<'_>,
        deadline: Instant,
    ) -> Result<ReadyAdmission, GuardianError> {
        transport.refuse_observable_eof()?;
        transport.send(
            &GuardianControl::Ready {
                identity: self.identity,
                authority: self.authority,
                mapped_uid: self.mapped_uid,
                creator_pid: self.creator_pid,
            },
            &[],
            deadline,
        )?;
        Ok(ReadyAdmission {
            authority: self.authority,
        })
    }
}

pub(super) struct ReadyAdmission {
    authority: super::protocol::RunAuthority,
}

#[derive(Debug)]
pub(super) enum InnerAdmissionError {
    Guardian(GuardianError),
    Installer(super::backend_installer::InstallerError),
    MissingOwnedState,
}

impl std::fmt::Display for InnerAdmissionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "inner backend admission refused: {self:?}")
    }
}

impl std::error::Error for InnerAdmissionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Guardian(error) => Some(error),
            Self::Installer(error) => Some(error),
            Self::MissingOwnedState => None,
        }
    }
}

enum InnerAdmissionState {
    Prepared,
    SendingStart,
    AwaitPolicy,
    SendingOuterRefusal,
    SendingRefusal,
    Refused,
    AwaitDispatch(ReadyAdmission),
    SendingExec,
    Dispatched,
    Transferred,
}

pub(super) enum InnerAdmissionProgress {
    Pending,
    Ready,
    Dispatched,
    Refused,
    /// Actual exclusive C lease EOF before Dispatch; no buffered frame may authorize work.
    LeaseClosed,
}

/// I owns the actual trusted installer throughout admission, including every failed post-spawn
/// operation. Positive policy receipt precedes I Ready, and C Dispatch precedes exact Exec.
pub(super) struct PendingInnerBackend {
    caller_reception: PreparedCallerReception,
    input: Option<super::role_bootstrap::InnerInput>,
    installer: super::backend_installer::InstallerOwner,
    hello: Option<AuthenticatedHello>,
    dispatch: Option<PreparedInnerDispatch>,
    artifacts: Option<GuardianArtifacts>,
    handoff_child: Option<std::process::Child>,
    refusal_storage: Option<FrameStorage>,
    refusal_send: Option<IncrementalSend>,
    outer_refusal_storage: Option<FrameStorage>,
    outer_refusal_send: Option<IncrementalSend>,
    policy_refusal: Option<(super::startup_envelope::PolicyFailureCause, StopStamp)>,
    stops: StopTimeline,
    deadline: Instant,
    state: InnerAdmissionState,
}

impl PendingInnerBackend {
    /// All fallible buffers/protection/authentication are prepared before the first child. An
    /// error here creates no backend; later errors retain the actual child in this same owner.
    pub(super) fn prepare(
        input: super::role_bootstrap::InnerInput,
    ) -> Result<Self, InnerAdmissionError> {
        super::owner_protection::protect_inner()
            .map_err(GuardianError::Io)
            .map_err(InnerAdmissionError::Guardian)?;
        let deadline = input
            .settings
            .startup_deadline()
            .map_err(io::Error::other)
            .map_err(GuardianError::Io)
            .map_err(InnerAdmissionError::Guardian)?;
        let mut caller_reception =
            PreparedCallerReception::prepare().map_err(InnerAdmissionError::Guardian)?;
        let hello = authenticate_hello(
            &input.caller_lease.transport(),
            input.settings.identity,
            deadline,
            &mut caller_reception,
        )
        .map_err(InnerAdmissionError::Guardian)?;
        if hello.authority != input.settings.authority {
            return Err(InnerAdmissionError::Guardian(GuardianError::Refusal(
                GuardianRefusal::ReplayedAuthority,
            )));
        }
        let dispatch = InnerBackend::prepare_dispatch_authority(hello.authority, deadline)
            .map_err(InnerAdmissionError::Guardian)?;
        let installer = super::backend_installer::InstallerOwner::prepare(&input)
            .map_err(InnerAdmissionError::Installer)?;
        let refusal_storage = FrameStorage::prepare()
            .map_err(GuardianError::Control)
            .map_err(InnerAdmissionError::Guardian)?;
        let outer_refusal_storage = FrameStorage::prepare()
            .map_err(GuardianError::Control)
            .map_err(InnerAdmissionError::Guardian)?;
        let stops = StopTimeline::prepare(input.settings.started)
            .map_err(io::Error::other)
            .map_err(GuardianError::Io)
            .map_err(InnerAdmissionError::Guardian)?;
        Ok(Self {
            caller_reception,
            input: Some(input),
            installer,
            hello: Some(hello),
            dispatch: Some(dispatch),
            artifacts: None,
            handoff_child: None,
            refusal_storage: Some(refusal_storage),
            refusal_send: None,
            outer_refusal_storage: Some(outer_refusal_storage),
            outer_refusal_send: None,
            policy_refusal: None,
            stops,
            deadline,
            state: InnerAdmissionState::Prepared,
        })
    }

    pub(super) fn tick(&mut self) -> Result<InnerAdmissionProgress, InnerAdmissionError> {
        let input = self
            .input
            .as_ref()
            .ok_or(InnerAdmissionError::MissingOwnedState)?;
        match input.caller_lease.transport().refuse_observable_eof() {
            Ok(()) => {}
            Err(ControlError::Eof) => return Ok(InnerAdmissionProgress::LeaseClosed),
            Err(error) => return Err(InnerAdmissionError::Guardian(GuardianError::Control(error))),
        }
        super::creator::require_live(&input.outer_pin)
            .map_err(GuardianError::Io)
            .map_err(InnerAdmissionError::Guardian)?;
        super::creator::require_live(&input.caller_pin)
            .map_err(GuardianError::Io)
            .map_err(InnerAdmissionError::Guardian)?;
        input
            .outer_bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(GuardianError::Control)
            .map_err(InnerAdmissionError::Guardian)?;
        super::owner_protection::require_protected()
            .map_err(GuardianError::Io)
            .map_err(InnerAdmissionError::Guardian)?;
        let cutoff = match self.state {
            InnerAdmissionState::SendingOuterRefusal
            | InnerAdmissionState::SendingRefusal
            | InnerAdmissionState::Refused => self
                .stops
                .deadline(input.settings.settlement_reserve, input.settings.deadline)
                .and_then(super::role_deadline::RoleDeadline::local)
                .map_err(io::Error::other)
                .map_err(GuardianError::Io)
                .map_err(InnerAdmissionError::Guardian)?,
            InnerAdmissionState::Prepared
            | InnerAdmissionState::SendingStart
            | InnerAdmissionState::AwaitPolicy
            | InnerAdmissionState::AwaitDispatch(_)
            | InnerAdmissionState::SendingExec
            | InnerAdmissionState::Dispatched
            | InnerAdmissionState::Transferred => self.deadline,
        };
        if Instant::now() >= cutoff {
            return Err(InnerAdmissionError::Guardian(GuardianError::Control(
                ControlError::Deadline,
            )));
        }
        match &self.state {
            InnerAdmissionState::Prepared => {
                self.installer
                    .spawn()
                    .map_err(InnerAdmissionError::Installer)?;
                self.state = InnerAdmissionState::SendingStart;
            }
            InnerAdmissionState::SendingStart => {
                if self
                    .installer
                    .send_start(input, self.deadline)
                    .map_err(InnerAdmissionError::Installer)?
                {
                    self.state = InnerAdmissionState::AwaitPolicy;
                }
            }
            InnerAdmissionState::AwaitPolicy => {
                let admission = match self.installer.receive_policy_ready(input, self.deadline) {
                    Ok(admission) => admission,
                    Err(super::backend_installer::InstallerError::PolicyRefused {
                        failure,
                        stop,
                    }) => {
                        // The actual installer remains in this owner. Capture original metadata
                        // before any fallible forwarding work, with no I receipt-time trigger.
                        self.policy_refusal = Some((failure, stop));
                        self.state = InnerAdmissionState::SendingOuterRefusal;
                        self.stops
                            .observe(stop)
                            .map_err(io::Error::other)
                            .map_err(GuardianError::Io)
                            .map_err(InnerAdmissionError::Guardian)?;
                        let storage = self
                            .refusal_storage
                            .take()
                            .ok_or(InnerAdmissionError::MissingOwnedState)?;
                        let refusal = super::startup_envelope::InstallerReply::Refused {
                            identity: input.settings.identity,
                            authority: input.settings.authority,
                            stop,
                            failure,
                            context: self.installer.failure_context().as_bytes(),
                        };
                        let frame = storage
                            .encode(&refusal)
                            .map_err(GuardianError::Control)
                            .map_err(InnerAdmissionError::Guardian)?;
                        self.refusal_send = Some(IncrementalSend::new(frame));
                        let outer_frame = self
                            .outer_refusal_storage
                            .take()
                            .ok_or(InnerAdmissionError::MissingOwnedState)?
                            .encode(&refusal)
                            .map_err(GuardianError::Control)
                            .map_err(InnerAdmissionError::Guardian)?;
                        self.outer_refusal_send = Some(IncrementalSend::new(outer_frame));
                        return Ok(InnerAdmissionProgress::Pending);
                    }
                    Err(error) => return Err(InnerAdmissionError::Installer(error)),
                };
                if let Some(admission) = admission {
                    let hello = self
                        .hello
                        .take()
                        .ok_or(InnerAdmissionError::MissingOwnedState)?;
                    let ready = hello
                        .publish_installed_ready(
                            admission,
                            &input.caller_lease.transport(),
                            self.deadline,
                        )
                        .map_err(InnerAdmissionError::Guardian)?;
                    self.state = InnerAdmissionState::AwaitDispatch(ready);
                    return Ok(InnerAdmissionProgress::Ready);
                }
            }
            InnerAdmissionState::SendingOuterRefusal => {
                // Preserve the genuine negative producer event at O before C can close its
                // exclusive lease. This is the same bounded Refused record, not completion.
                if self
                    .outer_refusal_send
                    .as_mut()
                    .ok_or(InnerAdmissionError::MissingOwnedState)?
                    .advance(&input.outer_bootstrap.transport(), &[], cutoff)
                    .map_err(GuardianError::Control)
                    .map_err(InnerAdmissionError::Guardian)?
                {
                    self.outer_refusal_send = None;
                    self.state = InnerAdmissionState::SendingRefusal;
                }
            }
            InnerAdmissionState::SendingRefusal => {
                if self
                    .refusal_send
                    .as_mut()
                    .ok_or(InnerAdmissionError::MissingOwnedState)?
                    .advance(&input.caller_lease.transport(), &[], cutoff)
                    .map_err(GuardianError::Control)
                    .map_err(InnerAdmissionError::Guardian)?
                {
                    self.refusal_send = None;
                    self.state = InnerAdmissionState::Refused;
                    return Ok(InnerAdmissionProgress::Refused);
                }
            }
            // Never publish Ready or transfer the child after a negative transaction. The
            // real I lease/outer liveness checks above remain active throughout settlement.
            InnerAdmissionState::Refused => return Ok(InnerAdmissionProgress::Refused),
            InnerAdmissionState::AwaitDispatch(ready) => {
                // This is the original strict C-lease receiver, so EOF wins over buffered
                // authorization. No private installer control substitutes for C Dispatch.
                let recipe = receive_dispatch(
                    &input.caller_lease.transport(),
                    ReadyAdmission {
                        authority: ready.authority,
                    },
                    self.deadline,
                    &mut self.caller_reception,
                )
                .map_err(InnerAdmissionError::Guardian)?;
                self.installer
                    .queue_exec(recipe.command, recipe.stdin)
                    .map_err(InnerAdmissionError::Installer)?;
                self.artifacts = Some(recipe.artifacts);
                self.state = InnerAdmissionState::SendingExec;
            }
            InnerAdmissionState::SendingExec => {
                if self
                    .installer
                    .send_exec(input, self.deadline)
                    .map_err(InnerAdmissionError::Installer)?
                {
                    self.state = InnerAdmissionState::Dispatched;
                    return Ok(InnerAdmissionProgress::Dispatched);
                }
            }
            InnerAdmissionState::Dispatched => return Ok(InnerAdmissionProgress::Dispatched),
            InnerAdmissionState::Transferred => return Err(InnerAdmissionError::MissingOwnedState),
        }
        Ok(InnerAdmissionProgress::Pending)
    }

    /// Move the same actual child into the normal I reaper only after genuine Dispatch. All
    /// required fields are checked while ownership is retained; no fallible pin happens later.
    pub(super) fn take_running(&mut self) -> Result<InnerBackend, InnerAdmissionError> {
        if !matches!(self.state, InnerAdmissionState::Dispatched)
            || self.input.is_none()
            || self.dispatch.is_none()
            || self.artifacts.is_none()
        {
            return Err(InnerAdmissionError::MissingOwnedState);
        }
        self.handoff_child = Some(
            self.installer
                .take_dispatched_child()
                .map_err(InnerAdmissionError::Installer)?,
        );
        let input = self
            .input
            .take()
            .ok_or(InnerAdmissionError::MissingOwnedState)?;
        let dispatch = self
            .dispatch
            .take()
            .ok_or(InnerAdmissionError::MissingOwnedState)?;
        let artifacts = self
            .artifacts
            .take()
            .ok_or(InnerAdmissionError::MissingOwnedState)?;
        let child = self
            .handoff_child
            .take()
            .ok_or(InnerAdmissionError::MissingOwnedState)?;
        self.state = InnerAdmissionState::Transferred;
        Ok(InnerBackend::from_installed_child(
            input, child, artifacts, dispatch,
        ))
    }
}

/// The original C/I frame, workspace and native label stay owned across Hello and Dispatch.
/// The production I owner prepares them before its first installer Child is created.
struct PreparedCallerReception {
    frame: PreparedReceive,
    scratch: super::guardian_decode::Scratch,
    native_tag: super::native_os_decode::NativeOsTag,
}

impl PreparedCallerReception {
    fn prepare() -> Result<Self, GuardianError> {
        Ok(Self {
            frame: PreparedReceive::prepare()?,
            scratch: super::guardian_decode::Scratch::default(),
            native_tag: super::native_os_decode::native_tag().map_err(io::Error::other)?,
        })
    }
}

fn authenticate_hello(
    transport: &Transport<'_>,
    identity: BuildIdentity,
    startup_deadline: Instant,
    reception: &mut PreparedCallerReception,
) -> Result<AuthenticatedHello, GuardianError> {
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
    let tag = reception.native_tag;
    let hello = transport.receive_prepared_decode(
        &mut reception.frame,
        super::caller_control_decode::CallerReply::rights_count,
        startup_deadline,
        |payload| {
            super::caller_control_decode::decode(payload, &mut reception.scratch, tag)
                .map_err(ControlError::InvalidGrammar)
        },
    )?;
    let super::caller_control_decode::CallerReply::Hello {
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
    Ok(AuthenticatedHello {
        identity,
        authority,
        mapped_uid: getuid().as_raw(),
        creator_pid: creator.pid,
    })
}

fn receive_dispatch(
    transport: &Transport<'_>,
    ready: ReadyAdmission,
    startup_deadline: Instant,
    reception: &mut PreparedCallerReception,
) -> Result<DispatchRecipe, GuardianError> {
    let authority = ready.authority;
    let tag = reception.native_tag;
    let dispatch = transport.receive_prepared_decode(
        &mut reception.frame,
        super::caller_control_decode::CallerReply::rights_count,
        startup_deadline,
        |payload| {
            super::caller_control_decode::decode(payload, &mut reception.scratch, tag)
                .map_err(ControlError::InvalidGrammar)
        },
    )?;
    let super::caller_control_decode::CallerReply::Dispatch {
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
    if cleanup_paths.count() > ARTIFACT_COUNT {
        return Err(GuardianError::Refusal(GuardianRefusal::InvalidControl));
    }
    // All frame, run/state and rights checks precede owned native materialization. The
    // borrowed facts keep this SAME prepared payload and rights live until conversion ends.
    transport.refuse_observable_eof()?;
    if Instant::now() >= startup_deadline {
        return Err(ControlError::Deadline.into());
    }
    let command = super::recipe_decode::materialize(command, tag, &mut reception.scratch)
        .map_err(original_materialization_error)?;
    let cleanup_paths =
        super::recipe_decode::materialize_values(cleanup_paths, tag, &mut reception.scratch)
            .map_err(original_materialization_error)?;
    transport.refuse_observable_eof()?;
    if Instant::now() >= startup_deadline {
        return Err(ControlError::Deadline.into());
    }
    let stdin = match stdin {
        StdinControl::Open => {
            let descriptor = dispatch
                .rights
                .pop()
                .ok_or(GuardianError::Refusal(GuardianRefusal::InvalidControl))?;
            super::stdin::OriginalStdin::Open(descriptor)
        }
        StdinControl::Closed => super::stdin::OriginalStdin::Closed,
    };
    Ok(DispatchRecipe {
        command,
        stdin,
        authority,
        artifacts: GuardianArtifacts(cleanup_paths),
    })
}

fn original_materialization_error(
    error: super::recipe_decode::MaterializationError,
) -> GuardianError {
    match error {
        super::recipe_decode::MaterializationError::Grammar(error) => {
            GuardianError::Control(ControlError::InvalidGrammar(error))
        }
        super::recipe_decode::MaterializationError::Allocation(error) => {
            // Move the ACTUAL local allocator source. No cross-role reconstruction, kind
            // normalization, policy-origin relabel or prose-derived classification occurs.
            GuardianError::Io(io::Error::other(error))
        }
    }
}

/// Exact typed metadata and original stdin survive Dispatch without constructing a backend
/// Command in I. The new installer owner forwards these same values to its retained actual PID.
struct DispatchRecipe {
    command: super::namespace::BackendCommand,
    stdin: super::stdin::OriginalStdin,
    authority: super::protocol::RunAuthority,
    artifacts: GuardianArtifacts,
}

impl DispatchRecipe {
    fn into_command(self) -> BackendAdmission {
        let mut backend = self.command.into_command();
        match self.stdin {
            super::stdin::OriginalStdin::Open(descriptor) => {
                backend.stdin(Stdio::from(descriptor));
            }
            super::stdin::OriginalStdin::Closed => {
                backend.stdin(Stdio::inherit());
            }
        }
        backend.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        BackendAdmission {
            command: backend,
            authority: self.authority,
            artifacts: self.artifacts,
        }
    }
}

fn admit_backend(
    transport: &Transport<'_>,
    identity: BuildIdentity,
    startup_deadline: Instant,
) -> Result<BackendAdmission, GuardianError> {
    let mut reception = PreparedCallerReception::prepare()?;
    let hello = authenticate_hello(transport, identity, startup_deadline, &mut reception)?;
    let ready = hello.publish_ready(transport, startup_deadline)?;
    receive_dispatch(transport, ready, startup_deadline, &mut reception)
        .map(DispatchRecipe::into_command)
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
    stops: Option<StopTimeline>,
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
        Self::prepare_dispatch_authority(admitted.authority, deadline)
    }

    fn prepare_dispatch_authority(
        authority: super::protocol::RunAuthority,
        deadline: Instant,
    ) -> Result<PreparedInnerDispatch, GuardianError> {
        Ok(PreparedInnerDispatch {
            event: InnerEvent::prepare(&GuardianControl::Dispatched { authority }, deadline)?,
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
        Self::from_installed_child(input, child, admitted.artifacts, dispatch)
    }

    fn from_installed_child(
        input: super::role_bootstrap::InnerInput,
        child: std::process::Child,
        artifacts: GuardianArtifacts,
        dispatch: PreparedInnerDispatch,
    ) -> Self {
        Self {
            input,
            child,
            reaper: None,
            _artifacts: artifacts,
            owner_receive: dispatch.owner_receive,
            state: InnerBackendState::Dispatching(dispatch.event),
            stops: None,
        }
    }

    /// One finite actor step. Caller EOF remains primary even with queued output/events; no
    /// post-Dispatch caller frame can authorize new work. Backend completion leaves I alive
    /// until real lease close, preserving namespace-owned descendant teardown.
    pub(super) fn tick(&mut self) -> Result<InnerBackendProgress, GuardianError> {
        if self.stops.is_none() {
            self.stops =
                Some(StopTimeline::prepare(self.input.settings.started).map_err(io::Error::other)?);
        }
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
        super::owner_protection::require_protected()?;
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
        if let Some(deadline) = self
            .stops
            .as_ref()
            .ok_or(GuardianError::Refusal(GuardianRefusal::InvalidControl))?
            .active_deadline(
                self.input.settings.settlement_reserve,
                self.input.settings.deadline,
            )
            .map_err(io::Error::other)?
        {
            deadline.local().map_err(io::Error::other)?;
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
                    let stops = self
                        .stops
                        .as_mut()
                        .ok_or(GuardianError::Refusal(GuardianRefusal::InvalidControl))?;
                    let stop = stops
                        .capture_once(StopOrigin::Inner)
                        .map_err(io::Error::other)?;
                    let deadline = stops
                        .deadline(
                            self.input.settings.settlement_reserve,
                            self.input.settings.deadline,
                        )
                        .map_err(io::Error::other)?
                        .local()
                        .map_err(io::Error::other)?;
                    self.state = InnerBackendState::Completing {
                        event: InnerEvent::prepare(
                            &GuardianControl::Completed {
                                authority: self.input.settings.authority,
                                outcome,
                                stop,
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
                    if let Some(received) = self.owner_receive.advance_decode(
                        &self.input.outer_bootstrap.transport(),
                        super::role_protocol::InnerOwnerControl::rights_count,
                        event.deadline,
                        |payload| {
                            super::inner_reply_decode::owner(
                                payload,
                                &mut self.input.decode_scratch,
                            )
                            .map_err(super::control::ControlError::InvalidGrammar)
                        },
                    )? {
                        let super::role_protocol::InnerOwnerControl::CompletionObserved {
                            authority,
                            stop,
                        } = received.control;
                        if authority != self.input.settings.authority {
                            return Err(GuardianError::Refusal(GuardianRefusal::ReplayedAuthority));
                        }
                        let stops = self
                            .stops
                            .as_mut()
                            .ok_or(GuardianError::Refusal(GuardianRefusal::InvalidControl))?;
                        stops.observe(stop).map_err(io::Error::other)?;
                        let cutoff = stops
                            .deadline(
                                self.input.settings.settlement_reserve,
                                self.input.settings.deadline,
                            )
                            .map_err(io::Error::other)?
                            .local()
                            .map_err(io::Error::other)?;
                        event.deadline = event.deadline.min(cutoff);
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
                &GuardianControl::Completed {
                    authority,
                    outcome,
                    stop: StopStamp::capture(StopOrigin::Inner).map_err(io::Error::other)?,
                },
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

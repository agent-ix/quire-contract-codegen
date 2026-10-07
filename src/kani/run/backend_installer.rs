//! I's retained actual Child at the trusted, same-PID policy/recipe boundary.
//!
//! Every fallible post-spawn operation borrows this owner; an error never discards the Child or
//! its live process capability. Spawn and EOF cannot establish installation. The executor must
//! settle I's entire namespace on every refused admission before publishing a C conclusion.

use std::{
    fs::File,
    io,
    os::fd::{AsFd, OwnedFd},
    process::{Child, Command},
    time::Instant,
};

use rustix::process::{pidfd_open, Pid, PidfdFlags};

use super::{
    control::{
        role_pair, ControlError, IncrementalReceive, IncrementalSend, PreparedFrame, RoleCaller,
        RoleEndpoint, RoleEntry,
    },
    creator::{self, PreparedIdentity},
    protocol::{BuildIdentity, RunAuthority, StdinControl},
    role_bootstrap::InnerInput,
    role_command::HelperRole,
    role_deadline::{DeadlineError, StopOrigin, StopStamp, StopTimeline},
    role_protocol::{BackendInstallerControl, RunSettings},
    startup_cause::{PreparedStartupContext, RepresentationError},
    startup_envelope::{self, InstallerReply, InstallerReplyHeader, PolicyFailureCause},
    stdin::OriginalStdin,
};

#[derive(Debug)]
pub(super) enum InstallerError {
    Io(io::Error),
    Control(ControlError),
    Report(super::report_storage::ReportError),
    Representation(RepresentationError),
    Deadline(DeadlineError),
    PolicyRefused {
        failure: PolicyFailureCause,
        stop: StopStamp,
    },
    UnexpectedState,
    MissingChild,
    MissingPin,
    InvalidPid,
    SenderMismatch,
    AuthorityMismatch,
    BuildIdentityMismatch,
}

/// Concrete trusted-entry failures remain local owned causes until the typed startup envelope
/// has been authenticated and transported. No transport EOF is policy success or exec success.
#[derive(Debug)]
pub(super) enum InstallerEntryError {
    Io(io::Error),
    Control(ControlError),
    Report(super::report_storage::ReportError),
    Deadline(DeadlineError),
    Representation(RepresentationError),
    Policy {
        cause: super::backend_policy::BackendPolicyError,
        stop: Result<StopStamp, DeadlineError>,
    },
    InvalidRole,
    IdentityMismatch,
    UnexpectedControl,
    UnexpectedState,
    MissingWriter,
}

impl std::fmt::Display for InstallerEntryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "trusted backend entry refused: {self:?}")
    }
}

impl std::error::Error for InstallerEntryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Control(error) => Some(error),
            Self::Report(error) => Some(error),
            Self::Deadline(error) => Some(error),
            Self::Representation(error) => Some(error),
            Self::Policy { cause, .. } => Some(cause),
            Self::InvalidRole
            | Self::IdentityMismatch
            | Self::UnexpectedControl
            | Self::UnexpectedState
            | Self::MissingWriter => None,
        }
    }
}

enum EntryState {
    Authenticated,
    Installing,
    Installed,
    AwaitExec,
    ExecReceived,
    Failed,
}

/// Actual single-thread sanitized helper, positively owned by I. Its startup descriptor remains
/// fd0 CLOEXEC through exact same-PID exec; all other trusted capabilities are owned CLOEXEC.
pub(super) struct InstallerEntry {
    control: RoleEndpoint,
    settings: RunSettings,
    inner_pin: OwnedFd,
    caller_pin: OwnedFd,
    writer: Option<File>,
    startup_deadline: Instant,
    ready: Option<IncrementalSend>,
    exec_receive: IncrementalReceive,
    failure_context: PreparedStartupContext,
    state: EntryState,
}

impl InstallerEntry {
    pub(super) fn receive(
        identity: BuildIdentity,
        initial_deadline: Instant,
    ) -> Result<Self, InstallerEntryError> {
        super::outer_setup::require_single_thread()
            .map_err(|error| InstallerEntryError::Io(io::Error::other(error)))?;
        if rustix::process::getpid().as_raw_nonzero().get() <= 1
            || !rustix::process::getppid().is_some_and(|pid| pid.as_raw_nonzero().get() == 1)
            || rustix::process::getuid().as_raw() != 0
            || rustix::process::getgid().as_raw() != 0
        {
            return Err(InstallerEntryError::InvalidRole);
        }
        let entry = RoleEntry::from_entry_stdin().map_err(InstallerEntryError::Control)?;
        let received = entry
            .receive::<BackendInstallerControl>(
                BackendInstallerControl::rights_count,
                initial_deadline,
            )
            .map_err(InstallerEntryError::Control)?;
        let BackendInstallerControl::Start { settings, report } = received.control else {
            return Err(InstallerEntryError::UnexpectedControl);
        };
        let [inner_pin, caller_pin, writer]: [OwnedFd; 3] =
            received.rights.try_into().map_err(|rights: Vec<OwnedFd>| {
                InstallerEntryError::Control(ControlError::RightsCount {
                    expected: 3,
                    received: rights.len(),
                })
            })?;
        let control = entry
            .authenticate(&inner_pin)
            .map_err(InstallerEntryError::Control)?;
        let peer = control
            .transport()
            .creator_credentials()
            .map_err(InstallerEntryError::Control)?;
        if peer.pid != 1 || peer.uid != 0 || peer.gid != 0 || settings.identity != identity {
            return Err(InstallerEntryError::IdentityMismatch);
        }
        let startup_deadline = settings
            .startup_deadline()
            .map_err(InstallerEntryError::Deadline)?
            .min(initial_deadline);
        creator::require_live(&inner_pin).map_err(InstallerEntryError::Io)?;
        creator::require_live(&caller_pin).map_err(InstallerEntryError::Io)?;
        control
            .transport()
            .refuse_observable_eof()
            .map_err(InstallerEntryError::Control)?;
        let writer = File::from(writer);
        super::report_storage::verify_owned_writer(&report, &writer)
            .map_err(InstallerEntryError::Report)?;
        Ok(Self {
            control,
            settings,
            inner_pin,
            caller_pin,
            writer: Some(writer),
            startup_deadline,
            ready: None,
            exec_receive: IncrementalReceive::prepare().map_err(InstallerEntryError::Control)?,
            failure_context: PreparedStartupContext::new(startup_envelope::CONTEXT_BYTES)
                .map_err(InstallerEntryError::Representation)?,
            state: EntryState::Authenticated,
        })
    }

    /// Actual policy precedes the only positive PolicyReady frame. An irreversible partial
    /// installation cannot retry, execute a recipe or synthesize successful admission.
    pub(super) fn install(&mut self) -> Result<(), InstallerEntryError> {
        if !matches!(self.state, EntryState::Authenticated) {
            return Err(InstallerEntryError::UnexpectedState);
        }
        self.require_live()?;
        self.state = EntryState::Installing;
        if let Err(cause) = super::backend_policy::install() {
            // Preserve the actual failure even if CLOCK_MONOTONIC capture itself refuses. Such
            // a missing stamp cannot be replaced with receipt time or a new settlement window.
            let stop = StopStamp::capture(StopOrigin::Backend);
            self.state = EntryState::Failed;
            return Err(InstallerEntryError::Policy { cause, stop });
        }
        self.require_live()?;
        let ready = PreparedFrame::encode(&InstallerReply::PolicyReady {
            identity: self.settings.identity,
            authority: self.settings.authority,
        })
        .map_err(InstallerEntryError::Control)?;
        self.ready = Some(IncrementalSend::new(ready));
        self.state = EntryState::Installed;
        Ok(())
    }

    /// Capture the concrete original cause without prose classification or unbounded formatting.
    /// A clock/representation failure remains refusal; it cannot fabricate a successful Ready.
    pub(super) fn prepare_policy_refusal(
        &mut self,
        error: &InstallerEntryError,
    ) -> Result<PreparedFrame, InstallerEntryError> {
        if !matches!(self.state, EntryState::Failed) {
            return Err(InstallerEntryError::UnexpectedState);
        }
        let InstallerEntryError::Policy { cause, stop } = error else {
            return Err(InstallerEntryError::UnexpectedState);
        };
        let stop = (*stop).map_err(InstallerEntryError::Deadline)?;
        if stop.origin != StopOrigin::Backend {
            return Err(InstallerEntryError::IdentityMismatch);
        }
        use super::backend_policy::BackendPolicyError;
        let failure = match cause {
            BackendPolicyError::UnsupportedArchitecture => {
                PolicyFailureCause::UnsupportedArchitecture
            }
            BackendPolicyError::InvalidProgram => PolicyFailureCause::InvalidProgram,
            BackendPolicyError::NotBackend => PolicyFailureCause::NotBackend,
            BackendPolicyError::ProtectionUnverified => PolicyFailureCause::ProtectionUnverified,
            BackendPolicyError::Preparation(error) => PolicyFailureCause::Preparation {
                cause: self
                    .failure_context
                    .capture_io(error)
                    .map_err(InstallerEntryError::Representation)?,
            },
            BackendPolicyError::Privilege(error) => PolicyFailureCause::Privilege {
                cause: self
                    .failure_context
                    .capture_io(error)
                    .map_err(InstallerEntryError::Representation)?,
            },
            #[cfg(any(
                target_arch = "x86_64",
                target_arch = "aarch64",
                target_arch = "riscv64"
            ))]
            BackendPolicyError::Filter(error) => PolicyFailureCause::Filter {
                cause: self
                    .failure_context
                    .capture_seccompiler(error)
                    .map_err(InstallerEntryError::Representation)?,
            },
        };
        if matches!(
            failure,
            PolicyFailureCause::UnsupportedArchitecture
                | PolicyFailureCause::InvalidProgram
                | PolicyFailureCause::NotBackend
                | PolicyFailureCause::ProtectionUnverified
        ) {
            self.failure_context
                .capture_context(cause)
                .map_err(InstallerEntryError::Representation)?;
        }
        PreparedFrame::encode(&InstallerReply::Refused {
            identity: self.settings.identity,
            authority: self.settings.authority,
            stop,
            failure,
            context: self.failure_context.context_bytes(),
        })
        .map_err(InstallerEntryError::Control)
    }

    fn require_live(&self) -> Result<(), InstallerEntryError> {
        if Instant::now() >= self.startup_deadline {
            return Err(InstallerEntryError::Deadline(DeadlineError::Expired));
        }
        self.settings
            .startup_deadline()
            .map_err(InstallerEntryError::Deadline)?;
        creator::require_live(&self.inner_pin).map_err(InstallerEntryError::Io)?;
        creator::require_live(&self.caller_pin).map_err(InstallerEntryError::Io)?;
        self.control
            .transport()
            .refuse_observable_eof()
            .map_err(InstallerEntryError::Control)
    }

    /// Send only the positive frame already prepared after installation; no early EOF can stand
    /// in for this actual sender/run-bound receipt at I. The owner event loop supplies the same
    /// original finite setup cutoff and checks live authority before every nonblocking attempt.
    pub(super) fn send_ready(&mut self, deadline: Instant) -> Result<bool, InstallerEntryError> {
        if !matches!(self.state, EntryState::Installed) {
            return Err(InstallerEntryError::UnexpectedState);
        }
        self.require_live()?;
        let sent = self
            .ready
            .as_mut()
            .ok_or(InstallerEntryError::UnexpectedState)?
            .advance(&self.control.transport(), &[], deadline)
            .map_err(InstallerEntryError::Control)?;
        if sent {
            self.ready = None;
            self.state = EntryState::AwaitExec;
        }
        Ok(sent)
    }

    /// Decode only the genuine next I-origin Exec after installation/positive readiness. The
    /// exclusive creator check, exact descriptor count and original run precede recipe use.
    pub(super) fn receive_exec(
        &mut self,
        deadline: Instant,
    ) -> Result<Option<InstallerExecRecipe>, InstallerEntryError> {
        if !matches!(self.state, EntryState::AwaitExec) {
            return Err(InstallerEntryError::UnexpectedState);
        }
        self.require_live()?;
        let Some(received) = self
            .exec_receive
            .advance::<BackendInstallerControl>(
                &self.control.transport(),
                BackendInstallerControl::rights_count,
                deadline,
            )
            .map_err(InstallerEntryError::Control)?
        else {
            return Ok(None);
        };
        let BackendInstallerControl::Exec {
            authority,
            command,
            stdin,
        } = received.control
        else {
            return Err(InstallerEntryError::UnexpectedControl);
        };
        if authority != self.settings.authority {
            return Err(InstallerEntryError::IdentityMismatch);
        }
        let stdin = match stdin {
            StdinControl::Open => OriginalStdin::Open(
                received
                    .rights
                    .pop()
                    .ok_or(InstallerEntryError::UnexpectedControl)?,
            ),
            StdinControl::Closed => OriginalStdin::Closed,
        };
        self.state = EntryState::ExecReceived;
        Ok(Some(InstallerExecRecipe {
            authority,
            command,
            stdin,
        }))
    }

    /// Drop every trusted endpoint/pin as this entry is consumed, before arbitrary recipe exec.
    /// The separately authenticated stdio bootstrap fd0 stays CLOEXEC and stable through exec.
    /// Only original stdin, capture streams and the verified report writer map into the recipe.
    pub(super) fn prepare_exec(
        mut self,
        recipe: InstallerExecRecipe,
    ) -> Result<super::backend_exec::PreparedBackendExec, InstallerEntryError> {
        if !matches!(self.state, EntryState::ExecReceived)
            || recipe.authority != self.settings.authority
        {
            return Err(InstallerEntryError::UnexpectedState);
        }
        self.require_live()?;
        let writer = self
            .writer
            .take()
            .ok_or(InstallerEntryError::MissingWriter)?;
        super::backend_exec::prepare(recipe.command, recipe.stdin, writer)
            .map_err(InstallerEntryError::Io)
    }
}

/// Exact recipe from the authenticated installed state, not a second public request or handle.
pub(super) struct InstallerExecRecipe {
    authority: RunAuthority,
    command: super::namespace::BackendCommand,
    stdin: OriginalStdin,
}

impl From<io::Error> for InstallerError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<ControlError> for InstallerError {
    fn from(error: ControlError) -> Self {
        Self::Control(error)
    }
}

impl std::fmt::Display for InstallerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "backend installation admission refused: {self:?}"
        )
    }
}

impl std::error::Error for InstallerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Control(error) => Some(error),
            Self::Report(error) => Some(error),
            Self::Representation(error) => Some(error),
            Self::Deadline(error) => Some(error),
            Self::PolicyRefused { .. } => None,
            Self::UnexpectedState
            | Self::MissingChild
            | Self::MissingPin
            | Self::InvalidPid
            | Self::SenderMismatch
            | Self::AuthorityMismatch
            | Self::BuildIdentityMismatch => None,
        }
    }
}

enum InstallerState {
    Prepared,
    SpawnAttempted,
    Spawned,
    AwaitPolicy,
    PolicyReady,
    SendingExec,
    ExecQueued,
    ChildTransferred,
}

/// Minted only after authenticated positive policy readiness from the retained actual Child.
/// This token permits I Ready for this run, not backend execution or a cleanup conclusion.
pub(super) struct InstallerAdmission {
    authority: RunAuthority,
}

impl InstallerAdmission {
    pub(super) fn authority(&self) -> RunAuthority {
        self.authority
    }
}

/// Single-thread I retains the actual sanitized installer from spawn through authenticated
/// policy readiness and exact recipe delivery. O concurrently observes this actual subtree.
pub(super) struct InstallerOwner {
    command: Option<Command>,
    child: Option<Child>,
    pin: Option<OwnedFd>,
    inner_pin: OwnedFd,
    control: RoleCaller,
    start: IncrementalSend,
    receive: IncrementalReceive,
    exec: Option<IncrementalSend>,
    exec_stdin: Option<OwnedFd>,
    identities: PreparedIdentity,
    identity: BuildIdentity,
    authority: RunAuthority,
    state: InstallerState,
    failure_context: PreparedStartupContext,
    stops: StopTimeline,
}

impl InstallerOwner {
    /// Allocate finite role controls before creating the actual child. The report identity
    /// comes from original O bootstrap, not from blessing whichever descriptor is now open.
    pub(super) fn prepare(input: &InnerInput) -> Result<Self, InstallerError> {
        super::owner_protection::require_protected()?;
        creator::require_live(&input.outer_pin)?;
        creator::require_live(&input.caller_pin)?;
        input.caller_lease.transport().refuse_observable_eof()?;
        input.outer_bootstrap.transport().refuse_observable_eof()?;
        super::report_storage::verify_owned_writer(&input.report, &input.writer)
            .map_err(InstallerError::Report)?;
        let (control, endpoint) = role_pair()?;
        let command = HelperRole::Backend.command(&input.settings.helper, endpoint)?;
        let inner_pin = pidfd_open(rustix::process::getpid(), PidfdFlags::NONBLOCK)
            .map_err(|error| InstallerError::Io(error.into()))?;
        Ok(Self {
            command: Some(command),
            child: None,
            pin: None,
            inner_pin,
            control,
            start: IncrementalSend::new(PreparedFrame::encode(&BackendInstallerControl::Start {
                settings: input.settings.clone(),
                report: input.report,
            })?),
            receive: IncrementalReceive::prepare()?,
            exec: None,
            exec_stdin: None,
            identities: PreparedIdentity::prepare()?,
            identity: input.settings.identity,
            authority: input.settings.authority,
            state: InstallerState::Prepared,
            failure_context: PreparedStartupContext::new(startup_envelope::CONTEXT_BYTES)
                .map_err(InstallerError::Representation)?,
            stops: StopTimeline::prepare(input.settings.started)
                .map_err(InstallerError::Deadline)?,
        })
    }

    /// The actual Child enters retained ownership before any fallible pin/identity operation.
    pub(super) fn spawn(&mut self) -> Result<(), InstallerError> {
        if !matches!(self.state, InstallerState::Prepared) {
            return Err(InstallerError::UnexpectedState);
        }
        let mut command = self.command.take().ok_or(InstallerError::UnexpectedState)?;
        self.state = InstallerState::SpawnAttempted;
        let child = command.spawn()?;
        self.child = Some(child);
        self.state = InstallerState::Spawned;
        drop(command);
        let child = self.child.as_ref().ok_or(InstallerError::MissingChild)?;
        let raw = i32::try_from(child.id()).map_err(|_| InstallerError::InvalidPid)?;
        let pid = Pid::from_raw(raw).ok_or(InstallerError::InvalidPid)?;
        self.pin = Some(
            pidfd_open(pid, PidfdFlags::NONBLOCK)
                .map_err(|error| InstallerError::Io(error.into()))?,
        );
        let actual = self.identities.validate_child_process(
            self.pin.as_ref().ok_or(InstallerError::MissingPin)?,
            &self.inner_pin,
        )?;
        if actual != raw {
            return Err(InstallerError::InvalidPid);
        }
        Ok(())
    }

    pub(super) fn send_start(
        &mut self,
        input: &InnerInput,
        deadline: Instant,
    ) -> Result<bool, InstallerError> {
        if !matches!(self.state, InstallerState::Spawned)
            || input.settings.authority != self.authority
        {
            return Err(InstallerError::UnexpectedState);
        }
        input.caller_lease.transport().refuse_observable_eof()?;
        creator::require_live(&input.outer_pin)?;
        creator::require_live(self.pin.as_ref().ok_or(InstallerError::MissingPin)?)?;
        let sent = self.start.advance(
            &self.control.transport(),
            &[
                self.inner_pin.as_fd(),
                input.caller_pin.as_fd(),
                input.writer.as_fd(),
            ],
            deadline,
        )?;
        if sent {
            self.state = InstallerState::AwaitPolicy;
        }
        Ok(sent)
    }

    /// Exact actual Child credentials, live pin and original run/build bind positive policy
    /// admission. No EOF, spawn status or earlier receipt can pass this transition.
    pub(super) fn receive_policy_ready(
        &mut self,
        input: &InnerInput,
        deadline: Instant,
    ) -> Result<Option<InstallerAdmission>, InstallerError> {
        if !matches!(self.state, InstallerState::AwaitPolicy) {
            return Err(InstallerError::UnexpectedState);
        }
        input.caller_lease.transport().refuse_observable_eof()?;
        creator::require_live(&input.outer_pin)?;
        creator::require_live(&input.caller_pin)?;
        let context = &mut self.failure_context;
        let Some(received) = self.receive.advance_decode(
            &self.control.transport(),
            InstallerReplyHeader::rights_count,
            deadline,
            |payload| startup_envelope::decode(payload, context),
        )?
        else {
            return Ok(None);
        };
        let sender = received.credentials.ok_or(InstallerError::SenderMismatch)?;
        let child = self.child.as_ref().ok_or(InstallerError::MissingChild)?;
        let actual = i32::try_from(child.id()).map_err(|_| InstallerError::InvalidPid)?;
        if sender.pid != actual
            || sender.uid != rustix::process::getuid().as_raw()
            || sender.gid != rustix::process::getgid().as_raw()
        {
            return Err(InstallerError::SenderMismatch);
        }
        if self.identities.validate_child_process(
            self.pin.as_ref().ok_or(InstallerError::MissingPin)?,
            &self.inner_pin,
        )? != actual
        {
            return Err(InstallerError::InvalidPid);
        }
        if received.control.identity() != self.identity {
            return Err(InstallerError::BuildIdentityMismatch);
        }
        if received.control.authority() != self.authority {
            return Err(InstallerError::AuthorityMismatch);
        }
        if let InstallerReplyHeader::Refused { stop, failure, .. } = received.control {
            if stop.origin != StopOrigin::Backend {
                return Err(InstallerError::AuthorityMismatch);
            }
            self.stops.observe(stop).map_err(InstallerError::Deadline)?;
            return Err(InstallerError::PolicyRefused { failure, stop });
        }
        self.state = InstallerState::PolicyReady;
        Ok(Some(InstallerAdmission {
            authority: self.authority,
        }))
    }

    /// Borrowed original context is provisional until the caller/outer owner authenticates the
    /// forwarded failure and confirms whole-chain settlement. It grants no public conclusion.
    pub(super) fn failure_context(&self) -> &str {
        self.failure_context.context()
    }

    /// Only the actual C/I Dispatch receiver calls this transition. The exact recipe and stdin
    /// move once to the installed process; no backend command is spawned or reconstructed in I.
    pub(super) fn queue_exec(
        &mut self,
        command: super::namespace::BackendCommand,
        stdin: OriginalStdin,
    ) -> Result<(), InstallerError> {
        if !matches!(self.state, InstallerState::PolicyReady) {
            return Err(InstallerError::UnexpectedState);
        }
        creator::require_live(self.pin.as_ref().ok_or(InstallerError::MissingPin)?)?;
        let control = BackendInstallerControl::Exec {
            authority: self.authority,
            command,
            stdin: match &stdin {
                OriginalStdin::Open(_) => StdinControl::Open,
                OriginalStdin::Closed => StdinControl::Closed,
            },
        };
        let frame = PreparedFrame::encode(&control)?;
        self.exec_stdin = match stdin {
            OriginalStdin::Open(descriptor) => Some(descriptor),
            OriginalStdin::Closed => None,
        };
        self.exec = Some(IncrementalSend::new(frame));
        self.state = InstallerState::SendingExec;
        Ok(())
    }

    /// One bounded nonblocking send step; the real caller lease is checked before every byte
    /// authorization attempt, including a partially queued Exec. EOF cannot authorize execution.
    pub(super) fn send_exec(
        &mut self,
        input: &InnerInput,
        deadline: Instant,
    ) -> Result<bool, InstallerError> {
        if !matches!(self.state, InstallerState::SendingExec) {
            return Err(InstallerError::UnexpectedState);
        }
        input.caller_lease.transport().refuse_observable_eof()?;
        creator::require_live(&input.outer_pin)?;
        creator::require_live(&input.caller_pin)?;
        creator::require_live(self.pin.as_ref().ok_or(InstallerError::MissingPin)?)?;
        let right = self.exec_stdin.as_ref().map(AsFd::as_fd);
        let sent = self
            .exec
            .as_mut()
            .ok_or(InstallerError::UnexpectedState)?
            .advance(&self.control.transport(), right.as_slice(), deadline)?;
        if sent {
            self.exec_stdin = None;
            self.exec = None;
            self.state = InstallerState::ExecQueued;
        }
        Ok(sent)
    }

    /// Transfer the same actual Child to I's ordinary reaper only after exact Exec was queued.
    /// This grants no claim that arbitrary recipe exec succeeded; failed exec uses NoVerdict.
    pub(super) fn take_dispatched_child(&mut self) -> Result<Child, InstallerError> {
        if !matches!(self.state, InstallerState::ExecQueued) {
            return Err(InstallerError::UnexpectedState);
        }
        let child = self.child.take().ok_or(InstallerError::MissingChild)?;
        self.state = InstallerState::ChildTransferred;
        Ok(child)
    }
}

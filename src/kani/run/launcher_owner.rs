//! Actual single-thread L ownership of O's Child and authenticated arm handoff (FR-034).
//!
//! Namespace creation belongs to the authenticated bootstrap primitive. This owner is retained
//! BEFORE any O spawn, records its unreaped Child BEFORE fallible pin/control work, and accepts
//! only an arm from that same positively owned O. It neither spawns bwrap nor owns the I writer.

use std::{
    io,
    os::{
        fd::{AsFd, OwnedFd},
        unix::process::ExitStatusExt,
    },
    process::{Child, Command, ExitStatus},
    thread,
    time::{Duration, Instant},
};

use rustix::process::{pidfd_open, Pid, PidfdFlags, Signal};

use super::{
    control::{role_pair, ControlError, IncrementalReceive, PreparedFrame, RoleCaller},
    creator,
    outer_setup::{NamespaceIdentity, PreparedOuter, SetupError},
    protocol::BackendExit,
    role_bootstrap::{BootstrapError, PreparedLauncher},
    role_command::HelperRole,
    role_deadline::{DeadlineError, IdentityDeadline, StopOrigin, StopTimeline},
    role_protocol::{
        LauncherControl, LauncherReply, LauncherSettlementMode, OuterArmReply, OuterBootstrap,
        OuterChildSettlement, RunSettings,
    },
};

#[derive(Debug)]
pub(super) enum LauncherError {
    Bootstrap(BootstrapError),
    Io(io::Error),
    Control(ControlError),
    Setup(SetupError),
    Deadline(DeadlineError),
    MissingParentArm,
    SpawnAlreadyAttempted,
    AlreadyArmed,
    MissingChild,
    MissingPin,
    InvalidChildPid,
    EndpointsConsumed,
    MissingSender,
    SenderMismatch,
    ArmMismatch,
    CapabilityMismatch,
    SettlementAuthorityMismatch,
    SettlementDeadlineMismatch,
    UnexpectedSettlementControl,
    InvalidOuterExit,
    SettlementAlreadyAttempted,
}

impl std::fmt::Display for LauncherError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "outer child ownership refused: {self:?}")
    }
}

impl std::error::Error for LauncherError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Bootstrap(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::Control(error) => Some(error),
            Self::Setup(error) => Some(error),
            Self::Deadline(error) => Some(error),
            Self::MissingParentArm
            | Self::SpawnAlreadyAttempted
            | Self::AlreadyArmed
            | Self::MissingChild
            | Self::MissingPin
            | Self::InvalidChildPid
            | Self::EndpointsConsumed
            | Self::MissingSender
            | Self::SenderMismatch
            | Self::ArmMismatch
            | Self::CapabilityMismatch
            | Self::SettlementAuthorityMismatch
            | Self::SettlementDeadlineMismatch
            | Self::UnexpectedSettlementControl
            | Self::InvalidOuterExit
            | Self::SettlementAlreadyAttempted => None,
        }
    }
}

pub(super) struct LauncherOwner {
    pub(super) input: PreparedLauncher,
    bootstrap: RoleCaller,
    command: Option<Command>,
    child: Option<Child>,
    pin: Option<OwnedFd>,
    start_frame: PreparedFrame,
    observation_frame: PreparedFrame,
    arm: Option<NamespaceIdentity>,
    cancellation_attempted: bool,
    exit: Option<ExitStatus>,
    settlement_attempted: bool,
    settlement_receive: IncrementalReceive,
}

impl LauncherOwner {
    pub(super) fn prepare(input: PreparedLauncher) -> Result<Self, LauncherError> {
        input
            .settings
            .startup_deadline()
            .map_err(LauncherError::Deadline)?;
        let (bootstrap, endpoint) = role_pair().map_err(LauncherError::Control)?;
        let command = HelperRole::Outer
            .command(&input.settings.helper, endpoint)
            .map_err(LauncherError::Io)?;
        let start_frame = PreparedFrame::encode(&OuterBootstrap::Start {
            settings: input.settings.clone(),
            original_mount: input.namespace.original_mount,
            original_pid: input.namespace.original_pid,
            original_network: input.namespace.original_network,
        })
        .map_err(LauncherError::Control)?;
        let observation_frame = PreparedFrame::encode(&OuterBootstrap::LauncherObservation {
            authority: input.settings.authority,
        })
        .map_err(LauncherError::Control)?;
        Ok(Self {
            input,
            bootstrap,
            command: Some(command),
            child: None,
            pin: None,
            start_frame,
            observation_frame,
            arm: None,
            cancellation_attempted: false,
            exit: None,
            settlement_attempted: false,
            settlement_receive: IncrementalReceive::prepare().map_err(LauncherError::Control)?,
        })
    }

    /// Every failure leaves actual Child custody in this existing owner for bounded settlement.
    pub(super) fn spawn(&mut self) -> Result<(), LauncherError> {
        if self.settlement_attempted {
            return Err(LauncherError::SettlementAlreadyAttempted);
        }
        self.require_parent()?;
        let mut command = self
            .command
            .take()
            .ok_or(LauncherError::SpawnAlreadyAttempted)?;
        let child = command.spawn().map_err(LauncherError::Io)?;
        self.child = Some(child);
        // Drop the command's child-only stdin copy before accepting bootstrap/arm observations.
        drop(command);
        let child = self.child.as_ref().ok_or(LauncherError::MissingChild)?;
        let pid = i32::try_from(child.id())
            .ok()
            .and_then(Pid::from_raw)
            .ok_or(LauncherError::InvalidChildPid)?;
        self.pin = Some(
            pidfd_open(pid, PidfdFlags::NONBLOCK)
                .map_err(|error| LauncherError::Io(error.into()))?,
        );
        let pin = self.pin.as_ref().ok_or(LauncherError::MissingPin)?;
        creator::require_live(pin).map_err(LauncherError::Io)?;
        let deadline = self
            .input
            .settings
            .startup_deadline()
            .map_err(LauncherError::Deadline)?
            .min(
                self.input
                    .settings
                    .setup_deadline
                    .local()
                    .map_err(LauncherError::Deadline)?,
            );
        let outer = self
            .input
            .outer_endpoint
            .as_ref()
            .ok_or(LauncherError::EndpointsConsumed)?;
        let inner = self
            .input
            .inner_endpoint
            .as_ref()
            .ok_or(LauncherError::EndpointsConsumed)?;
        self.bootstrap
            .transport()
            .send_prepared(
                &self.start_frame,
                &[
                    self.input.namespace.launcher_pin.as_fd(),
                    self.input.caller_pin.as_fd(),
                    outer.as_fd(),
                    inner.as_fd(),
                ],
                deadline,
            )
            .map_err(LauncherError::Control)?;
        self.input.outer_endpoint.take();
        self.input.inner_endpoint.take();
        self.bootstrap
            .transport()
            .send_prepared(
                &self.observation_frame,
                &[
                    self.input.namespace.launcher_stat.as_fd(),
                    self.input.namespace.launcher_status.as_fd(),
                ],
                deadline,
            )
            .map_err(LauncherError::Control)?;
        Ok(())
    }

    /// Initiate cancellation of the one actual positively owned O. This is not settlement;
    /// the retained Child must separately yield a real exit status before L can return normally.
    pub(super) fn cancel_outer(&mut self) -> Result<(), LauncherError> {
        if self.poll_outer_exit()?.is_some() || self.cancellation_attempted {
            return Ok(());
        }
        self.cancellation_attempted = true;
        if let Some(pin) = &self.pin {
            match rustix::process::pidfd_send_signal(pin, Signal::KILL) {
                Ok(()) | Err(rustix::io::Errno::SRCH) => Ok(()),
                Err(error) => Err(LauncherError::Io(error.into())),
            }
        } else {
            // Spawn records this unreaped Child before pidfd_open. If that capability call
            // failed, std Child remains exact owned cancellation/reaping authority, not a
            // guessed PID or permission to continue an unsupported run.
            let child = self.child.as_mut().ok_or(LauncherError::MissingChild)?;
            match child.kill() {
                Ok(()) => Ok(()),
                Err(error)
                    if error.raw_os_error() == Some(rustix::io::Errno::SRCH.raw_os_error()) =>
                {
                    Ok(())
                }
                Err(error) => Err(LauncherError::Io(error)),
            }
        }
    }

    /// Only the unreaped Child's actual wait result proves O exit. A ready pidfd, EOF, signal
    /// request or an error does not invent a successful reap; repeated polls retain the result.
    pub(super) fn poll_outer_exit(&mut self) -> Result<Option<ExitStatus>, LauncherError> {
        if let Some(exit) = self.exit {
            return Ok(Some(exit));
        }
        let Some(child) = self.child.as_mut() else {
            return Ok(None);
        };
        self.exit = child.try_wait().map_err(LauncherError::Io)?;
        Ok(self.exit)
    }

    /// Consume one authenticated original-C cleanup request and retain O custody through reap.
    /// A reply is sent only after the real Child wait; L remains live until C acknowledges it.
    /// Neither this request nor the reply creates another settlement window.
    pub(super) fn settle_for_caller(
        &mut self,
        request: LauncherControl,
    ) -> Result<(), LauncherError> {
        if self.settlement_attempted {
            return Err(LauncherError::SettlementAlreadyAttempted);
        }
        self.settlement_attempted = true;
        let LauncherControl::Settle {
            authority,
            deadline,
            mode,
        } = request
        else {
            return Err(LauncherError::UnexpectedSettlementControl);
        };
        if authority != self.input.settings.authority {
            return Err(LauncherError::SettlementAuthorityMismatch);
        }
        let original = self.input.settings.deadline;
        if let IdentityDeadline::Finite { deadline: bound } = original {
            if !deadline
                .no_later_than(bound)
                .map_err(LauncherError::Deadline)?
            {
                return Err(LauncherError::SettlementDeadlineMismatch);
            }
        }
        let deadline = deadline.local().map_err(LauncherError::Deadline)?;
        if matches!(original, IdentityDeadline::NeverElapses)
            && deadline.saturating_duration_since(Instant::now())
                > super::role_deadline::SETTLE_RESERVE
        {
            return Err(LauncherError::SettlementDeadlineMismatch);
        }
        // The executor decoded this command on the authenticated original C endpoint, under
        // its existing finite per-frame cap. Only the transferred original/first-stop cutoff
        // controls settlement; the checks above cannot refresh or extend it.
        let custody = if self.child.is_none() {
            OuterChildSettlement::NotCreated
        } else {
            if matches!(mode, LauncherSettlementMode::CancelOuter) {
                self.cancel_outer()?;
            }
            let exit = loop {
                if let Some(exit) = self.poll_outer_exit()? {
                    break exit;
                }
                let remaining = deadline
                    .checked_duration_since(Instant::now())
                    .filter(|remaining| !remaining.is_zero())
                    .ok_or(LauncherError::Deadline(DeadlineError::Expired))?;
                thread::park_timeout(remaining.min(Duration::from_millis(20)));
            };
            let outcome = match (exit.code(), exit.signal()) {
                (Some(code), None) => BackendExit::Code(code),
                (None, Some(signal)) => BackendExit::Signal(signal),
                _ => return Err(LauncherError::InvalidOuterExit),
            };
            OuterChildSettlement::Reaped { outcome }
        };
        self.input
            .bootstrap
            .transport()
            .send(
                &LauncherReply::OuterSettled {
                    identity: self.input.settings.identity,
                    authority: self.input.settings.authority,
                    custody,
                },
                &[],
                deadline,
            )
            .map_err(LauncherError::Control)?;
        let acknowledged = self
            .input
            .bootstrap
            .transport()
            .receive::<LauncherControl>(LauncherControl::rights_count, deadline)
            .map_err(LauncherError::Control)?;
        let LauncherControl::Retire { authority } = acknowledged.control else {
            return Err(LauncherError::UnexpectedSettlementControl);
        };
        if authority != self.input.settings.authority {
            return Err(LauncherError::SettlementAuthorityMismatch);
        }
        Ok(())
    }

    /// After arm, L owns no work budget and never applies its expired setup cap to ordinary
    /// admitted backend work. It observes the real creator-thread/lease and retains O's Child
    /// until C's authenticated original settlement transaction positively reaps and retires it.
    pub(super) fn run_until_retired(&mut self) -> Result<(), LauncherError> {
        let mut frame_cutoff = None;
        loop {
            self.input
                .namespace
                .require_single_thread()
                .map_err(LauncherError::Setup)?;
            if rustix::process::parent_process_death_signal()
                .map_err(|error| LauncherError::Io(error.into()))?
                != Some(Signal::KILL)
            {
                return Err(LauncherError::MissingParentArm);
            }
            // Check the positively retained actual creating thread, not merely C's TGID.
            let creator_state = creator::require_live(&self.input.creator_pin);
            let caller_state = self
                .input
                .bootstrap
                .transport()
                .pending_control(Duration::ZERO);
            if creator_state.is_err() || matches!(caller_state, Err(ControlError::Eof)) {
                let mut stop = StopTimeline::prepare(self.input.settings.started)
                    .map_err(LauncherError::Deadline)?;
                stop.capture_once(StopOrigin::Launcher)
                    .map_err(LauncherError::Deadline)?;
                let cutoff = stop
                    .deadline(
                        self.input.settings.settlement_reserve,
                        self.input.settings.deadline,
                    )
                    .and_then(super::role_deadline::RoleDeadline::local)
                    .map_err(LauncherError::Deadline)?;
                self.cancel_outer()?;
                while self.poll_outer_exit()?.is_none() {
                    let remaining = cutoff
                        .checked_duration_since(Instant::now())
                        .filter(|remaining| !remaining.is_zero())
                        .ok_or(LauncherError::Deadline(DeadlineError::Expired))?;
                    thread::park_timeout(remaining.min(Duration::from_millis(20)));
                }
                if Instant::now() >= cutoff {
                    return Err(LauncherError::Deadline(DeadlineError::Expired));
                }
                // This is actual L child settlement only, with no public conclusion. A failed
                // creator inspection is not rewritten as proof of caller death/EOF.
                return match creator_state {
                    Err(error) => Err(LauncherError::Io(error)),
                    Ok(()) => Err(LauncherError::Control(ControlError::Eof)),
                };
            }
            let pending = caller_state.map_err(LauncherError::Control)?;
            if let Some(original) = self
                .input
                .settings
                .identity_deadline()
                .map_err(LauncherError::Deadline)?
            {
                if Instant::now() >= original {
                    return Err(LauncherError::Deadline(DeadlineError::Expired));
                }
            }
            if pending || self.settlement_receive.has_partial_frame() {
                // Freeze a finite frame-receive bound on its first observed data; retries never
                // restart it. The decoded C request must still carry the genuine earlier cutoff.
                let cutoff = match frame_cutoff {
                    Some(cutoff) => cutoff,
                    None => {
                        let cap = Instant::now()
                            .checked_add(self.input.settings.settlement_reserve)
                            .ok_or(LauncherError::Deadline(DeadlineError::Unrepresentable))?;
                        let cutoff = self
                            .input
                            .settings
                            .identity_deadline()
                            .map_err(LauncherError::Deadline)?
                            .map_or(cap, |original| original.min(cap));
                        frame_cutoff = Some(cutoff);
                        cutoff
                    }
                };
                if let Some(received) = self
                    .settlement_receive
                    .advance::<LauncherControl>(
                        &self.input.bootstrap.transport(),
                        LauncherControl::rights_count,
                        cutoff,
                    )
                    .map_err(LauncherError::Control)?
                {
                    // SO_PEERCRED pins the exclusive endpoint; every received record also binds
                    // C's actual PID/UID/GID. No rights or other command can request retirement.
                    let sender = received.credentials.ok_or(LauncherError::MissingSender)?;
                    let actual = self
                        .input
                        .bootstrap
                        .transport()
                        .creator_credentials()
                        .map_err(LauncherError::Control)?;
                    if sender.pid != actual.pid
                        || sender.uid != actual.uid
                        || sender.gid != actual.gid
                    {
                        return Err(LauncherError::SenderMismatch);
                    }
                    return self.settle_for_caller(received.control);
                }
            }
            thread::park_timeout(Duration::from_millis(20));
        }
    }

    pub(super) fn confirm_arm(&mut self) -> Result<NamespaceIdentity, LauncherError> {
        if self.arm.is_some() {
            return Err(LauncherError::AlreadyArmed);
        }
        self.require_parent()?;
        let child = self.child.as_ref().ok_or(LauncherError::MissingChild)?;
        let expected = self.pin.as_ref().ok_or(LauncherError::MissingPin)?;
        creator::require_live(expected).map_err(LauncherError::Io)?;
        let deadline = self
            .input
            .settings
            .startup_deadline()
            .map_err(LauncherError::Deadline)?
            .min(
                self.input
                    .settings
                    .setup_deadline
                    .local()
                    .map_err(LauncherError::Deadline)?,
            );
        let received = self
            .bootstrap
            .transport()
            .receive::<OuterArmReply>(OuterArmReply::rights_count, deadline)
            .map_err(LauncherError::Control)?;
        let sender = received.credentials.ok_or(LauncherError::MissingSender)?;
        if i32::try_from(child.id()).ok() != Some(sender.pid) || sender.uid != 0 || sender.gid != 0
        {
            return Err(LauncherError::SenderMismatch);
        }
        let OuterArmReply::Armed {
            identity,
            authority,
            namespace,
            network,
            mapped_uid,
            mapped_gid,
        } = received.control;
        if identity != self.input.settings.identity
            || authority != self.input.settings.authority
            || mapped_uid != 0
            || mapped_gid != 0
            || namespace == self.input.namespace.original_pid
            || network == self.input.namespace.original_network
        {
            return Err(LauncherError::ArmMismatch);
        }
        let [actual]: [OwnedFd; 1] =
            received.rights.try_into().map_err(|rights: Vec<OwnedFd>| {
                LauncherError::Control(ControlError::RightsCount {
                    expected: 1,
                    received: rights.len(),
                })
            })?;
        let actual_identity =
            rustix::fs::fstat(&actual).map_err(|error| LauncherError::Io(error.into()))?;
        let expected_identity =
            rustix::fs::fstat(expected).map_err(|error| LauncherError::Io(error.into()))?;
        let observed_namespace = self
            .input
            .namespace
            .child_pid_namespace(child.id())
            .map_err(LauncherError::Io)?;
        if actual_identity.st_dev != expected_identity.st_dev
            || actual_identity.st_ino != expected_identity.st_ino
            || observed_namespace != namespace
            || self
                .input
                .namespace
                .child_network_namespace(child.id())
                .map_err(LauncherError::Io)?
                != network
        {
            return Err(LauncherError::CapabilityMismatch);
        }
        creator::require_live(expected).map_err(LauncherError::Io)?;
        self.require_parent()?;
        self.arm = Some(namespace);
        Ok(namespace)
    }

    fn require_parent(&self) -> Result<(), LauncherError> {
        self.input
            .namespace
            .require_single_thread()
            .map_err(LauncherError::Setup)?;
        self.input
            .settings
            .setup_deadline
            .local()
            .map_err(LauncherError::Deadline)?;
        self.input
            .settings
            .startup_deadline()
            .map_err(LauncherError::Deadline)?;
        if rustix::process::parent_process_death_signal()
            .map_err(|error| LauncherError::Io(error.into()))?
            != Some(Signal::KILL)
        {
            return Err(LauncherError::MissingParentArm);
        }
        creator::require_live(&self.input.creator_pin).map_err(LauncherError::Io)?;
        self.input
            .bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(LauncherError::Control)
    }
}

/// Only an actual prepared O can publish its arm capability; bytes themselves prove no arm.
pub(super) fn publish_outer_arm(
    guard: &PreparedOuter<'_>,
    settings: &RunSettings,
    caller: &super::control::RoleEndpoint,
    deadline: Instant,
) -> Result<(), LauncherError> {
    guard.require_creator_live().map_err(LauncherError::Setup)?;
    let reply = PreparedFrame::encode(&OuterArmReply::Armed {
        identity: settings.identity,
        authority: settings.authority,
        namespace: guard.namespace(),
        network: guard.network(),
        mapped_uid: 0,
        mapped_gid: 0,
    })
    .map_err(LauncherError::Control)?;
    guard
        .bootstrap()
        .transport()
        .send_prepared(&reply, &[guard.descriptor()], deadline)
        .map_err(LauncherError::Control)?;
    guard.require_creator_live().map_err(LauncherError::Setup)?;
    caller
        .transport()
        .send_prepared(&reply, &[guard.descriptor()], deadline)
        .map_err(LauncherError::Control)
}

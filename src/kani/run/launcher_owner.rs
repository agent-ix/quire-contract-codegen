//! Actual single-thread L ownership of O's Child and authenticated arm handoff (FR-034).
//!
//! Namespace creation belongs to the authenticated bootstrap primitive. This owner is retained
//! BEFORE any O spawn, records its unreaped Child BEFORE fallible pin/control work, and accepts
//! only an arm from that same positively owned O. It neither spawns bwrap nor owns the I writer.

use std::{
    io,
    os::fd::{AsFd, OwnedFd},
    process::{Child, Command},
    time::Instant,
};

use rustix::process::{pidfd_open, Pid, PidfdFlags, Signal};

use super::{
    control::{role_pair, ControlError, PreparedFrame, RoleCaller},
    creator,
    outer_setup::{NamespaceIdentity, PreparedOuter, SetupError},
    role_bootstrap::{BootstrapError, PreparedLauncher},
    role_command::HelperRole,
    role_deadline::DeadlineError,
    role_protocol::{OuterArmReply, OuterBootstrap, RunSettings},
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
            | Self::CapabilityMismatch => None,
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
}

impl LauncherOwner {
    pub(super) fn prepare(input: PreparedLauncher) -> Result<Self, LauncherError> {
        input
            .settings
            .deadline
            .local()
            .map_err(LauncherError::Deadline)?;
        let (bootstrap, endpoint) = role_pair().map_err(LauncherError::Control)?;
        let command = HelperRole::Outer
            .command(&input.settings.helper, endpoint)
            .map_err(LauncherError::Io)?;
        let start_frame = PreparedFrame::encode(&OuterBootstrap::Start {
            settings: input.settings.clone(),
            original_mount: input.namespace.original_mount,
            original_pid: input.namespace.original_pid,
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
        })
    }

    /// Every failure leaves actual Child custody in this existing owner for bounded settlement.
    pub(super) fn spawn(&mut self) -> Result<(), LauncherError> {
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
            .deadline
            .local()
            .map_err(LauncherError::Deadline)?;
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
            .deadline
            .local()
            .map_err(LauncherError::Deadline)?;
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
            mapped_uid,
            mapped_gid,
        } = received.control;
        if identity != self.input.settings.identity
            || authority != self.input.settings.authority
            || mapped_uid != 0
            || mapped_gid != 0
            || namespace == self.input.namespace.original_pid
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
            .deadline
            .local()
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

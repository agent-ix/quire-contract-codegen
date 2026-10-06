//! C's retained actual launcher and exclusive bootstrap capabilities (FR-034).
//!
//! Preparation creates no helper. Once launch begins, every error leaves the same owner in the
//! caller's hands. Only the original RunOwner may settle the outer/inner chain, then reap L and
//! join its actual creating thread. An error, dropped handle or closed bootstrap proves no reap.

use std::{
    io,
    os::fd::{AsFd, OwnedFd},
    process::{Command, Stdio},
    sync::Arc,
    time::Instant,
};

use rustix::process::{pidfd_open, PidfdFlags};

use super::{
    caller_streams::CallerStreams,
    control::{
        role_pair, ControlError, FrameStorage, GuardianEndpoint, PreparedFrame, PreparedReceive,
        RoleCaller,
    },
    creator,
    outer_setup::NamespaceIdentity,
    protocol::{current_build_identity, BuildIdentity, RunAuthority},
    publication::Publication,
    report_storage::PreparedReportRead,
    role_command::HelperRole,
    role_deadline::DeadlineError,
    role_protocol::{LauncherControl, OuterArmReply, RunSettings},
    spawner::{RetainedSpawner, SpawnIdentity},
    stages::{Bootstrap, PreparedDispatch},
    stdin::OriginalStdin,
};

#[derive(Debug)]
pub(super) enum CallerBootstrapError {
    Io(io::Error),
    Control(ControlError),
    Deadline(DeadlineError),
    SettingsMismatch,
    ReservationUnrepresentable,
    LaunchAlreadyAttempted,
    MissingLauncher,
    EndpointsConsumed,
    ArmAlreadyReceived,
    MissingSender,
    ArmMismatch,
    MissingOuterPin,
    CapabilityMismatch,
}

impl std::fmt::Display for CallerBootstrapError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "caller role bootstrap refused: {self:?}")
    }
}

impl std::error::Error for CallerBootstrapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Control(error) => Some(error),
            Self::Deadline(error) => Some(error),
            Self::SettingsMismatch
            | Self::ReservationUnrepresentable
            | Self::LaunchAlreadyAttempted
            | Self::MissingLauncher
            | Self::EndpointsConsumed
            | Self::ArmAlreadyReceived
            | Self::MissingSender
            | Self::ArmMismatch
            | Self::MissingOuterPin
            | Self::CapabilityMismatch => None,
        }
    }
}

/// The original I-lease writer is retained separately from both role-control writers.
pub(super) struct CallerBootstrap {
    pub(super) inner_bootstrap: Bootstrap,
    pub(super) outer_control: RoleCaller,
    launcher_control: RoleCaller,
    caller_pin: OwnedFd,
    inner_endpoint: Option<OwnedFd>,
    outer_endpoint: Option<OwnedFd>,
    start_frame: PreparedFrame,
    pub(super) report_read: Option<PreparedReportRead>,
    pub(super) dispatch: Option<PreparedDispatch>,
    pub(super) stdin: OriginalStdin,
    pub(super) streams: CallerStreams,
    pub(super) publication: Arc<Publication>,
    receive: PreparedReceive,
    named_buffers: u64,
    command: Option<Command>,
    spawner: Option<RetainedSpawner>,
    identity: Option<SpawnIdentity>,
    deadline: Instant,
    build_identity: BuildIdentity,
    authority: RunAuthority,
    caller_uid: u32,
    caller_gid: u32,
    original_namespace: NamespaceIdentity,
    outer_pin: Option<OwnedFd>,
    outer_namespace: Option<NamespaceIdentity>,
}

impl CallerBootstrap {
    /// Assemble actual named C run buffers before creating L. Settings cannot supply or override
    /// this charge: it is computed from the retained reservations and serialized once afterwards.
    /// Opaque incidental std/libc runtime allocations are not assigned a fabricated capacity.
    pub(super) fn prepare(
        mut settings: RunSettings,
        bootstrap: Bootstrap,
        inner_endpoint: GuardianEndpoint,
        dispatch: PreparedDispatch,
        stdin: OriginalStdin,
        capture_limit: usize,
        publication: Arc<Publication>,
    ) -> Result<Self, CallerBootstrapError> {
        if settings.authority != bootstrap.authority()
            || settings.identity != current_build_identity()
            || settings.caller_uid != rustix::process::getuid().as_raw()
            || settings.caller_gid != rustix::process::getgid().as_raw()
        {
            return Err(CallerBootstrapError::SettingsMismatch);
        }
        let deadline = settings
            .deadline
            .local()
            .map_err(CallerBootstrapError::Deadline)?;
        let build_identity = settings.identity;
        let authority = settings.authority;
        let caller_uid = settings.caller_uid;
        let caller_gid = settings.caller_gid;
        let original_namespace =
            NamespaceIdentity::read("/proc/self/ns/pid").map_err(CallerBootstrapError::Io)?;
        let (launcher_control, launcher_endpoint) =
            role_pair().map_err(CallerBootstrapError::Control)?;
        let (outer_control, outer_endpoint) = role_pair().map_err(CallerBootstrapError::Control)?;
        let caller_pin = pidfd_open(rustix::process::getpid(), PidfdFlags::NONBLOCK)
            .map_err(|error| CallerBootstrapError::Io(error.into()))?;
        let (streams, stdout, stderr) =
            CallerStreams::prepare(capture_limit).map_err(CallerBootstrapError::Io)?;
        let report_read = PreparedReportRead::prepare()
            .map_err(|error| CallerBootstrapError::Io(io::Error::other(error)))?;
        let receive = PreparedReceive::prepare().map_err(CallerBootstrapError::Control)?;
        let frame_storage = FrameStorage::prepare().map_err(CallerBootstrapError::Control)?;
        let helper_capacity = u64::try_from(settings.helper.capacity())
            .map_err(|_| CallerBootstrapError::ReservationUnrepresentable)?;
        let mut command = HelperRole::Launcher
            .command(&settings.helper, launcher_endpoint)
            .map_err(CallerBootstrapError::Io)?;
        command
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr));
        let mut named_buffers = u64::try_from(std::mem::size_of::<Self>())
            .map_err(|_| CallerBootstrapError::ReservationUnrepresentable)?;
        let reservations = [
            RetainedSpawner::reserved_bytes().map_err(CallerBootstrapError::Io)?,
            frame_storage
                .reserved_bytes()
                .map_err(CallerBootstrapError::Control)?,
            receive
                .reserved_bytes()
                .map_err(CallerBootstrapError::Control)?,
            bootstrap
                .reserved_bytes()
                .map_err(|error| CallerBootstrapError::Io(io::Error::other(error)))?,
            dispatch
                .reserved_bytes()
                .map_err(|error| CallerBootstrapError::Io(io::Error::other(error)))?,
            report_read
                .reserved_bytes()
                .map_err(|error| CallerBootstrapError::Io(io::Error::other(error)))?,
            streams.reserved_bytes(),
            helper_capacity,
            u64::try_from(std::mem::size_of::<Publication>())
                .map_err(|_| CallerBootstrapError::ReservationUnrepresentable)?,
        ];
        for bytes in reservations {
            named_buffers = named_buffers
                .checked_add(bytes)
                .ok_or(CallerBootstrapError::ReservationUnrepresentable)?;
        }
        settings.caller_run_buffers = named_buffers;
        let start_frame = frame_storage
            .encode(&LauncherControl::Start { settings })
            .map_err(CallerBootstrapError::Control)?;
        Ok(Self {
            inner_bootstrap: bootstrap,
            outer_control,
            launcher_control,
            caller_pin,
            inner_endpoint: Some(inner_endpoint.into_child_mapping()),
            outer_endpoint: Some(outer_endpoint.into_child_mapping()),
            start_frame,
            report_read: Some(report_read),
            dispatch: Some(dispatch),
            stdin,
            streams,
            publication,
            receive,
            named_buffers,
            command: Some(command),
            spawner: None,
            identity: None,
            deadline,
            build_identity,
            authority,
            caller_uid,
            caller_gid,
            original_namespace,
            outer_pin: None,
            outer_namespace: None,
        })
    }

    /// Stores actual spawner custody BEFORE any wait/pin/control error. A failed handshake must
    /// be settled by this retained owner; no retry or replacement helper is attempted here.
    pub(super) fn launch(&mut self) -> Result<(), CallerBootstrapError> {
        if Instant::now() >= self.deadline {
            return Err(CallerBootstrapError::Deadline(DeadlineError::Expired));
        }
        let command = self
            .command
            .take()
            .ok_or(CallerBootstrapError::LaunchAlreadyAttempted)?;
        self.spawner = Some(RetainedSpawner::spawn(command).map_err(CallerBootstrapError::Io)?);
        self.streams.start().map_err(CallerBootstrapError::Io)?;
        let identity = self
            .spawner
            .as_ref()
            .ok_or(CallerBootstrapError::MissingLauncher)?
            .wait_started(self.deadline)
            .map_err(CallerBootstrapError::Io)?;
        self.identity = Some(identity);
        let identity = self
            .identity
            .as_ref()
            .ok_or(CallerBootstrapError::MissingLauncher)?;
        let inner = self
            .inner_endpoint
            .as_ref()
            .ok_or(CallerBootstrapError::EndpointsConsumed)?;
        let outer = self
            .outer_endpoint
            .as_ref()
            .ok_or(CallerBootstrapError::EndpointsConsumed)?;
        self.launcher_control
            .transport()
            .send_prepared(
                &self.start_frame,
                &[
                    identity.creator_pin.as_fd(),
                    self.caller_pin.as_fd(),
                    inner.as_fd(),
                    outer.as_fd(),
                ],
                self.deadline,
            )
            .map_err(CallerBootstrapError::Control)?;
        // Only the endpoint copy is transferred. C retains the sole original I-lease writer and
        // its independent O-control writer throughout the later live-caller lease observation.
        self.inner_endpoint.take();
        self.outer_endpoint.take();
        Ok(())
    }

    /// C accepts only the actual live child of its retained L, authenticated as the O sender.
    /// The received capability is retained before any fallible identity inspection; a failed
    /// publication never authorizes a replacement O or discards the existing role owner's pins.
    pub(super) fn confirm_outer_arm(&mut self) -> Result<NamespaceIdentity, CallerBootstrapError> {
        if self.outer_pin.is_some() {
            return Err(CallerBootstrapError::ArmAlreadyReceived);
        }
        let launcher = self
            .identity
            .as_ref()
            .ok_or(CallerBootstrapError::MissingLauncher)?;
        creator::require_live(&launcher.launcher_pin).map_err(CallerBootstrapError::Io)?;
        let received = self
            .outer_control
            .transport()
            .receive_prepared::<OuterArmReply>(
                &mut self.receive,
                OuterArmReply::rights_count,
                self.deadline,
            )
            .map_err(CallerBootstrapError::Control)?;
        let pin = received
            .rights
            .pop()
            .ok_or(CallerBootstrapError::MissingOuterPin)?;
        self.outer_pin = Some(pin);
        let pin = self
            .outer_pin
            .as_ref()
            .ok_or(CallerBootstrapError::MissingOuterPin)?;
        let sender = received
            .credentials
            .ok_or(CallerBootstrapError::MissingSender)?;
        let OuterArmReply::Armed {
            identity,
            authority,
            namespace,
            mapped_uid,
            mapped_gid,
        } = received.control;
        if identity != self.build_identity
            || authority != self.authority
            || mapped_uid != 0
            || mapped_gid != 0
            || sender.uid != self.caller_uid
            || sender.gid != self.caller_gid
            || namespace == self.original_namespace
        {
            return Err(CallerBootstrapError::ArmMismatch);
        }
        let pid = creator::validate_child_process(pin, &launcher.launcher_pin)
            .map_err(CallerBootstrapError::Io)?;
        if sender.pid != pid
            || NamespaceIdentity::read(&format!("/proc/{pid}/ns/pid"))
                .map_err(CallerBootstrapError::Io)?
                != namespace
        {
            return Err(CallerBootstrapError::CapabilityMismatch);
        }
        creator::require_live(pin).map_err(CallerBootstrapError::Io)?;
        creator::require_live(&launcher.launcher_pin).map_err(CallerBootstrapError::Io)?;
        if Instant::now() >= self.deadline {
            return Err(CallerBootstrapError::Deadline(DeadlineError::Expired));
        }
        self.outer_namespace = Some(namespace);
        Ok(namespace)
    }

    pub(super) fn named_buffer_reservation(&self) -> u64 {
        self.named_buffers
    }

    pub(super) fn launcher(&self) -> Result<&RetainedSpawner, CallerBootstrapError> {
        self.spawner
            .as_ref()
            .ok_or(CallerBootstrapError::MissingLauncher)
    }

    pub(super) fn launcher_identity(&self) -> Result<&SpawnIdentity, CallerBootstrapError> {
        self.identity
            .as_ref()
            .ok_or(CallerBootstrapError::MissingLauncher)
    }
}

//! C's retained actual launcher and exclusive bootstrap capabilities (FR-034).
//!
//! Preparation creates no helper. Once launch begins, every error leaves the same owner in the
//! caller's hands. Only the original RunOwner may settle the outer/inner chain, then reap L and
//! join its actual creating thread. An error, dropped handle or closed bootstrap proves no reap.

use std::{
    cell::RefCell,
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
    namespace::{GuardianIdentity, ReadyIdentityError},
    outer_setup::NamespaceIdentity,
    protocol::{current_build_identity, BuildIdentity, GuardianRefusal, RunAuthority},
    publication::{Publication, Stage},
    report_storage::PreparedReportRead,
    role_command::HelperRole,
    role_deadline::DeadlineError,
    role_protocol::{
        LauncherControl, LauncherReply, OuterArmReply, OuterPhaseCommand, OuterPhaseReply,
        RunSettings,
    },
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
    LauncherRefused(GuardianRefusal),
    LauncherReplyMismatch,
    UnexpectedPhase,
    PhaseReplyMismatch,
    MissingMonitorPin,
    MissingInnerPin,
    InnerBootstrapConsumed,
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
            | Self::CapabilityMismatch
            | Self::LauncherRefused(_)
            | Self::LauncherReplyMismatch
            | Self::UnexpectedPhase
            | Self::PhaseReplyMismatch
            | Self::MissingMonitorPin
            | Self::MissingInnerPin
            | Self::InnerBootstrapConsumed => None,
        }
    }
}

/// The original I-lease writer is retained separately from both role-control writers.
pub(super) struct CallerBootstrap {
    inner_bootstrap: Option<Bootstrap>,
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
    identity_records: RefCell<creator::PreparedIdentity>,
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
    outer_pid: Option<i32>,
    monitor_pin: Option<OwnedFd>,
    inner_pin: Option<OwnedFd>,
    phase_frames: [PreparedFrame; 3],
    phase: CallerPhase,
    inner_identity: Option<(i32, u64, NamespaceIdentity)>,
}

enum CallerPhase {
    AwaitArm,
    BeforeMonitor,
    AwaitMonitor,
    Bootstrap,
    AwaitClaim,
    ClaimedGated,
    AwaitGate,
    ClaimedBootstrap,
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
        let identity_records =
            creator::PreparedIdentity::prepare().map_err(CallerBootstrapError::Io)?;
        let phase_frames = [
            PreparedFrame::encode(&OuterPhaseCommand::BeginMonitor { authority }),
            PreparedFrame::encode(&OuterPhaseCommand::ClaimInner { authority }),
            PreparedFrame::encode(&OuterPhaseCommand::ReleaseGate { authority }),
        ];
        let [begin, claim, release] = phase_frames;
        let phase_frames = [
            begin.map_err(CallerBootstrapError::Control)?,
            claim.map_err(CallerBootstrapError::Control)?,
            release.map_err(CallerBootstrapError::Control)?,
        ];
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
            identity_records
                .reserved_bytes()
                .map_err(CallerBootstrapError::Io)?,
            helper_capacity,
            u64::try_from(std::mem::size_of::<Publication>())
                .map_err(|_| CallerBootstrapError::ReservationUnrepresentable)?,
        ];
        for bytes in reservations {
            named_buffers = named_buffers
                .checked_add(bytes)
                .ok_or(CallerBootstrapError::ReservationUnrepresentable)?;
        }
        for frame in &phase_frames {
            named_buffers = named_buffers
                .checked_add(
                    frame
                        .reserved_bytes()
                        .map_err(CallerBootstrapError::Control)?,
                )
                .ok_or(CallerBootstrapError::ReservationUnrepresentable)?;
        }
        settings.caller_run_buffers = named_buffers;
        let start_frame = frame_storage
            .encode(&LauncherControl::Start { settings })
            .map_err(CallerBootstrapError::Control)?;
        Ok(Self {
            inner_bootstrap: Some(bootstrap),
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
            identity_records: RefCell::new(identity_records),
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
            outer_pid: None,
            monitor_pin: None,
            inner_pin: None,
            phase_frames,
            phase: CallerPhase::AwaitArm,
            inner_identity: None,
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
        let received = self
            .launcher_control
            .transport()
            .receive_prepared::<LauncherReply>(
                &mut self.receive,
                LauncherReply::rights_count,
                self.deadline,
            )
            .map_err(CallerBootstrapError::Control)?;
        let sender = received
            .credentials
            .ok_or(CallerBootstrapError::MissingSender)?;
        if sender.pid != identity.launcher_pid.as_raw_pid()
            || sender.uid != self.caller_uid
            || sender.gid != self.caller_gid
        {
            return Err(CallerBootstrapError::LauncherReplyMismatch);
        }
        creator::require_live(&identity.launcher_pin).map_err(CallerBootstrapError::Io)?;
        match received.control {
            LauncherReply::Ready {
                identity,
                authority,
            } if identity == self.build_identity && authority == self.authority => Ok(()),
            LauncherReply::Refused {
                authority, reason, ..
            } if authority == self.authority => Err(CallerBootstrapError::LauncherRefused(reason)),
            LauncherReply::Ready { .. } | LauncherReply::Refused { .. } => {
                Err(CallerBootstrapError::LauncherReplyMismatch)
            }
        }
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
        let pid = self
            .identity_records
            .get_mut()
            .validate_child_process(pin, &launcher.launcher_pin)
            .map_err(CallerBootstrapError::Io)?;
        if sender.pid != pid
            || self
                .identity_records
                .get_mut()
                .child_namespace(pid)
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
        self.outer_pid = Some(pid);
        self.phase = CallerPhase::BeforeMonitor;
        Ok(namespace)
    }

    pub(super) fn begin_monitor(&mut self) -> Result<(), CallerBootstrapError> {
        if !matches!(self.phase, CallerPhase::BeforeMonitor) {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.phase = CallerPhase::AwaitMonitor;
        self.outer_control
            .transport()
            .send_prepared(&self.phase_frames[0], &[], self.deadline)
            .map_err(CallerBootstrapError::Control)
    }

    pub(super) fn claim_inner(&mut self) -> Result<(), CallerBootstrapError> {
        if !matches!(self.phase, CallerPhase::Bootstrap) {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.phase = CallerPhase::AwaitClaim;
        self.outer_control
            .transport()
            .send_prepared(&self.phase_frames[1], &[], self.deadline)
            .map_err(CallerBootstrapError::Control)
    }

    pub(super) fn release_gate(&mut self) -> Result<(), CallerBootstrapError> {
        if !matches!(self.phase, CallerPhase::ClaimedGated) {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.phase = CallerPhase::AwaitGate;
        self.outer_control
            .transport()
            .send_prepared(&self.phase_frames[2], &[], self.deadline)
            .map_err(CallerBootstrapError::Control)
    }

    /// Receive the reply into the already charged buffers. Every received process capability is
    /// installed in this owner BEFORE any fallible identity check, so errors cannot discard custody.
    pub(super) fn confirm_phase(&mut self) -> Result<(), CallerBootstrapError> {
        if !matches!(
            self.phase,
            CallerPhase::AwaitMonitor | CallerPhase::AwaitClaim | CallerPhase::AwaitGate
        ) {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        let received = self
            .outer_control
            .transport()
            .receive_prepared::<OuterPhaseReply>(
                &mut self.receive,
                OuterPhaseReply::rights_count,
                self.deadline,
            )
            .map_err(CallerBootstrapError::Control)?;
        // Select only the expected typed reply; a reordered frame never authorizes a transition.
        let next = match (&self.phase, &received.control) {
            (CallerPhase::AwaitMonitor, OuterPhaseReply::MonitorSpawned { .. }) => {
                self.monitor_pin = received.rights.pop();
                CallerPhase::Bootstrap
            }
            (CallerPhase::AwaitClaim, OuterPhaseReply::InnerClaimed { .. }) => {
                self.inner_pin = received.rights.pop();
                CallerPhase::ClaimedGated
            }
            (CallerPhase::AwaitGate, OuterPhaseReply::GateReleased { .. }) => {
                CallerPhase::ClaimedBootstrap
            }
            _ => return Err(CallerBootstrapError::UnexpectedPhase),
        };
        let sender = received
            .credentials
            .ok_or(CallerBootstrapError::MissingSender)?;
        if sender.pid
            != self
                .outer_pid
                .ok_or(CallerBootstrapError::MissingOuterPin)?
            || sender.uid != self.caller_uid
            || sender.gid != self.caller_gid
            || received.control.authority() != self.authority
        {
            return Err(CallerBootstrapError::PhaseReplyMismatch);
        }
        let outer = self
            .outer_pin
            .as_ref()
            .ok_or(CallerBootstrapError::MissingOuterPin)?;
        creator::require_live(outer).map_err(CallerBootstrapError::Io)?;
        match received.control {
            OuterPhaseReply::MonitorSpawned { .. } => {
                let monitor = self
                    .monitor_pin
                    .as_ref()
                    .ok_or(CallerBootstrapError::MissingMonitorPin)?;
                self.identity_records
                    .get_mut()
                    .validate_child_process(monitor, outer)
                    .map_err(CallerBootstrapError::Io)?;
            }
            OuterPhaseReply::InnerClaimed {
                start, namespace, ..
            } => {
                let monitor = self
                    .monitor_pin
                    .as_ref()
                    .ok_or(CallerBootstrapError::MissingMonitorPin)?;
                let inner = self
                    .inner_pin
                    .as_ref()
                    .ok_or(CallerBootstrapError::MissingInnerPin)?;
                let pid = self
                    .identity_records
                    .get_mut()
                    .validate_child_process(inner, monitor)
                    .map_err(CallerBootstrapError::Io)?;
                if self
                    .identity_records
                    .get_mut()
                    .child_start(pid)
                    .map_err(CallerBootstrapError::Io)?
                    != start
                    || self
                        .identity_records
                        .get_mut()
                        .child_namespace(pid)
                        .map_err(CallerBootstrapError::Io)?
                        != namespace
                    || Some(namespace) == self.outer_namespace
                    || namespace == self.original_namespace
                {
                    return Err(CallerBootstrapError::CapabilityMismatch);
                }
                creator::require_live(inner).map_err(CallerBootstrapError::Io)?;
                self.inner_identity = Some((pid, start, namespace));
            }
            OuterPhaseReply::GateReleased { .. } => {
                creator::require_live(
                    self.inner_pin
                        .as_ref()
                        .ok_or(CallerBootstrapError::MissingInnerPin)?,
                )
                .map_err(CallerBootstrapError::Io)?;
            }
        }
        creator::require_live(outer).map_err(CallerBootstrapError::Io)?;
        if Instant::now() >= self.deadline {
            return Err(CallerBootstrapError::Deadline(DeadlineError::Expired));
        }
        let stage = match &next {
            CallerPhase::Bootstrap => Stage::Bootstrap,
            CallerPhase::ClaimedGated => Stage::ClaimedGated,
            CallerPhase::ClaimedBootstrap => Stage::ClaimedBootstrap,
            CallerPhase::AwaitArm
            | CallerPhase::BeforeMonitor
            | CallerPhase::AwaitMonitor
            | CallerPhase::AwaitClaim
            | CallerPhase::AwaitGate => return Err(CallerBootstrapError::UnexpectedPhase),
        };
        self.phase = next;
        self.publication.publish(stage);
        Ok(())
    }

    /// Take only the real original C/I lease state after authenticated actual gate release.
    /// The same existing typed Hello/Ready/Dispatch stages run against this retained chain owner.
    pub(super) fn take_inner_bootstrap(&mut self) -> Result<Bootstrap, CallerBootstrapError> {
        if !matches!(self.phase, CallerPhase::ClaimedBootstrap) {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.inner_bootstrap
            .take()
            .ok_or(CallerBootstrapError::InnerBootstrapConsumed)
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

impl GuardianIdentity for CallerBootstrap {
    fn verify_ready(
        &self,
        sender: super::control::PeerCredentials,
        mapped_uid: u32,
    ) -> Result<(), ReadyIdentityError> {
        let launcher = self
            .identity
            .as_ref()
            .ok_or(ReadyIdentityError::Unclaimed)?;
        creator::require_live(&launcher.creator_pin)?;
        creator::require_live(&launcher.launcher_pin)?;
        let expected = self.inner_identity.ok_or(ReadyIdentityError::Unclaimed)?;
        let inner = self
            .inner_pin
            .as_ref()
            .ok_or(ReadyIdentityError::Unclaimed)?;
        let monitor = self
            .monitor_pin
            .as_ref()
            .ok_or(ReadyIdentityError::Unclaimed)?;
        let mut records = self
            .identity_records
            .try_borrow_mut()
            .map_err(|_| io::Error::other("caller identity records are already borrowed"))?;
        records.verify_ready(inner, monitor, expected, sender, mapped_uid)?;
        creator::require_live(
            self.outer_pin
                .as_ref()
                .ok_or(ReadyIdentityError::Unclaimed)?,
        )?;
        creator::require_live(&launcher.creator_pin)?;
        creator::require_live(&launcher.launcher_pin)?;
        Ok(())
    }
}

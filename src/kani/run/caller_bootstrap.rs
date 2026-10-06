//! C's retained actual launcher and exclusive bootstrap capabilities (FR-034).
//!
//! Preparation creates no helper. Once launch begins, every error leaves the same owner in the
//! caller's hands. Only the original RunOwner may settle the outer/inner chain, then reap L and
//! join its actual creating thread. An error, dropped handle or closed bootstrap proves no reap.

use std::{
    cell::RefCell,
    fmt::{self, Write as _},
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
        RoleCaller, TerminalReceive,
    },
    creator,
    namespace::{GuardianIdentity, ReadyIdentityError},
    outer_setup::NamespaceIdentity,
    protocol::{current_build_identity, BackendExit, BuildIdentity, GuardianRefusal, RunAuthority},
    publication::{Publication, Stage},
    report_storage::{PreparedReportRead, ReportError},
    resource_ledger::MeasuredPeaks,
    role_command::HelperRole,
    role_deadline::{DeadlineError, ExecutionClock, IdentityDeadline, RoleDeadline},
    role_protocol::{
        CallerTerminalControl, LauncherControl, LauncherReply, LauncherSettlementMode,
        OuterArmReply, OuterChildSettlement, OuterPhaseCommand, OuterPhaseReply,
        OuterTerminalReply, RunSettings,
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
    Report(ReportError),
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
    SettlementAlreadyAttempted,
    SettlementReplyMismatch,
    OuterTerminationUnconfirmed,
    CleanupDetailConsumed,
    TerminalTransition,
    TerminalReplyMismatch,
    OuterExitAbnormal,
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
            Self::Report(error) => Some(error),
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
            | Self::InnerBootstrapConsumed
            | Self::SettlementAlreadyAttempted
            | Self::SettlementReplyMismatch
            | Self::OuterTerminationUnconfirmed
            | Self::CleanupDetailConsumed
            | Self::TerminalTransition
            | Self::TerminalReplyMismatch
            | Self::OuterExitAbnormal => None,
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
    terminal_receive: TerminalReceive,
    read_ack_storage: Option<FrameStorage>,
    terminal_phase: CallerTerminalPhase,
    identity_records: RefCell<creator::PreparedIdentity>,
    named_buffers: u64,
    command: Option<Command>,
    spawner: Option<RetainedSpawner>,
    identity: Option<SpawnIdentity>,
    deadline: Instant,
    identity_deadline: Option<Instant>,
    identity_clock: IdentityDeadline,
    settlement_transfer: Option<(Instant, RoleDeadline)>,
    build_identity: BuildIdentity,
    authority: RunAuthority,
    caller_uid: u32,
    caller_gid: u32,
    original_namespace: NamespaceIdentity,
    original_network: NamespaceIdentity,
    settlement_storage: Option<FrameStorage>,
    retirement_frame: PreparedFrame,
    outer_settled: Option<OuterChildSettlement>,
    cleanup_detail: Option<PreparedCleanupDetail>,
    outer_pin: Option<OwnedFd>,
    outer_namespace: Option<NamespaceIdentity>,
    outer_network: Option<NamespaceIdentity>,
    outer_pid: Option<i32>,
    monitor_pin: Option<OwnedFd>,
    inner_pin: Option<OwnedFd>,
    phase_frames: [PreparedFrame; 3],
    phase: CallerPhase,
    inner_identity: Option<(i32, u64, NamespaceIdentity)>,
}

// Existing merged settlement diagnostic ceiling, not another caller-tunable product bound.
const CLEANUP_DETAIL_BYTES: usize = 4096;

/// A bounded diagnostic owns no process/thread/cleanup authority. Its complete allocation exists
/// before L, and formatting never allocates another full error String after settlement failure.
pub(super) struct PreparedCleanupDetail {
    text: String,
}

impl PreparedCleanupDetail {
    fn prepare() -> io::Result<Self> {
        let mut text = String::new();
        text.try_reserve_exact(CLEANUP_DETAIL_BYTES)
            .map_err(io::Error::other)?;
        Ok(Self { text })
    }

    fn reserved_bytes(&self) -> io::Result<u64> {
        u64::try_from(self.text.capacity()).map_err(io::Error::other)
    }

    pub(super) fn seal(mut self, detail: fmt::Arguments<'_>) -> String {
        // Reaching the finite diagnostic cap stops formatting at a UTF8 boundary. A diagnostic
        // truncation cannot choose the failure kind, claim settlement or manufacture evidence.
        let _ = self.write_fmt(detail);
        self.text
    }
}

impl fmt::Write for PreparedCleanupDetail {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        let remaining = CLEANUP_DETAIL_BYTES
            .checked_sub(self.text.len())
            .ok_or(fmt::Error)?;
        let length = value.floor_char_boundary(remaining.min(value.len()));
        self.text.push_str(value.get(..length).ok_or(fmt::Error)?);
        if length != value.len() {
            return Err(fmt::Error);
        }
        Ok(())
    }
}

/// Minted only after actual O custody, L reap and the retained creator's join. It grants no
/// report/evidence acceptance; output readers still require their own bounded settlement.
pub(super) struct CallerRoleSettlement {
    cutoff: Instant,
    authority: RunAuthority,
}

impl CallerRoleSettlement {
    pub(super) fn cutoff(&self) -> Instant {
        self.cutoff
    }
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

enum CallerTerminalPhase {
    AwaitDescriptor,
    AwaitCommit,
    ReceivedCommit(MeasuredPeaks),
    Finished,
    Refused,
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
        clock: &ExecutionClock,
    ) -> Result<Self, CallerBootstrapError> {
        if settings.authority != bootstrap.authority()
            || settings.identity != current_build_identity()
            || settings.caller_uid != rustix::process::getuid().as_raw()
            || settings.caller_gid != rustix::process::getgid().as_raw()
        {
            return Err(CallerBootstrapError::SettingsMismatch);
        }
        settings
            .bind_clock(clock)
            .map_err(CallerBootstrapError::Deadline)?;
        // C retains its actual original Instant; decoding its own role wire clock would round
        // it and make subsequent comparisons with the original ExecutionClock invalid.
        let identity_deadline = clock.original_deadline();
        let identity_clock = settings.deadline;
        let deadline = clock
            .work_deadline()
            .map_or(bootstrap.setup_deadline(), |deadline| {
                deadline.min(bootstrap.setup_deadline())
            });
        // A supplied numeric label cannot replace the genuine original C setup clock.
        settings.setup_deadline =
            RoleDeadline::from_original(deadline).map_err(CallerBootstrapError::Deadline)?;
        let build_identity = settings.identity;
        let authority = settings.authority;
        let caller_uid = settings.caller_uid;
        let caller_gid = settings.caller_gid;
        let original_namespace =
            NamespaceIdentity::read("/proc/self/ns/pid").map_err(CallerBootstrapError::Io)?;
        let original_network =
            NamespaceIdentity::read("/proc/self/ns/net").map_err(CallerBootstrapError::Io)?;
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
        let terminal_receive = TerminalReceive::prepare().map_err(CallerBootstrapError::Control)?;
        let read_ack_storage = FrameStorage::prepare().map_err(CallerBootstrapError::Control)?;
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
        let cleanup_detail = PreparedCleanupDetail::prepare().map_err(CallerBootstrapError::Io)?;
        let settlement_storage = FrameStorage::prepare().map_err(CallerBootstrapError::Control)?;
        let retirement_frame = PreparedFrame::encode(&LauncherControl::Retire { authority })
            .map_err(CallerBootstrapError::Control)?;
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
            u64::try_from(std::mem::size_of::<ExecutionClock>())
                .map_err(|_| CallerBootstrapError::ReservationUnrepresentable)?,
            terminal_receive
                .reserved_bytes()
                .map_err(CallerBootstrapError::Control)?,
            read_ack_storage
                .reserved_bytes()
                .map_err(CallerBootstrapError::Control)?,
            cleanup_detail
                .reserved_bytes()
                .map_err(CallerBootstrapError::Io)?,
            settlement_storage
                .reserved_bytes()
                .map_err(CallerBootstrapError::Control)?,
            retirement_frame
                .reserved_bytes()
                .map_err(CallerBootstrapError::Control)?,
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
            terminal_receive,
            read_ack_storage: Some(read_ack_storage),
            terminal_phase: CallerTerminalPhase::AwaitDescriptor,
            identity_records: RefCell::new(identity_records),
            named_buffers,
            command: Some(command),
            spawner: None,
            identity: None,
            deadline,
            identity_deadline,
            identity_clock,
            settlement_transfer: None,
            build_identity,
            authority,
            caller_uid,
            caller_gid,
            original_namespace,
            original_network,
            settlement_storage: Some(settlement_storage),
            retirement_frame,
            outer_settled: None,
            cleanup_detail: Some(cleanup_detail),
            outer_pin: None,
            outer_namespace: None,
            outer_network: None,
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
            LauncherReply::OuterSettled { .. } => {
                return Err(CallerBootstrapError::LauncherReplyMismatch)
            }
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
            network,
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
            || network == self.original_network
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
            || self
                .identity_records
                .get_mut()
                .child_network_namespace(pid)
                .map_err(CallerBootstrapError::Io)?
                != network
        {
            return Err(CallerBootstrapError::CapabilityMismatch);
        }
        creator::require_live(pin).map_err(CallerBootstrapError::Io)?;
        creator::require_live(&launcher.launcher_pin).map_err(CallerBootstrapError::Io)?;
        if Instant::now() >= self.deadline {
            return Err(CallerBootstrapError::Deadline(DeadlineError::Expired));
        }
        self.outer_namespace = Some(namespace);
        self.outer_network = Some(network);
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
                    || Some(
                        self.identity_records
                            .get_mut()
                            .child_network_namespace(pid)
                            .map_err(CallerBootstrapError::Io)?,
                    ) != self.outer_network
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

    /// Take the actual pre-L diagnostic reservation once. The whole-chain owner selects the
    /// existing CleanupUnconfirmed kind only after the real settlement attempt; this String
    /// formatter alone carries no authority and confirms no cleanup.
    pub(super) fn take_cleanup_detail(
        &mut self,
    ) -> Result<PreparedCleanupDetail, CallerBootstrapError> {
        self.cleanup_detail
            .take()
            .ok_or(CallerBootstrapError::CleanupDetailConsumed)
    }

    /// The whole RunOwner invokes this only after authenticated backend Completed and actual
    /// original I lease close. Descriptor reception retains strict EOF precedence. Reading bytes
    /// grants no classification: the final complete metrics frame and all settlement still follow.
    pub(super) fn read_terminal_report(
        &mut self,
        clock: &ExecutionClock,
    ) -> Result<Option<Vec<u8>>, CallerBootstrapError> {
        if !matches!(self.terminal_phase, CallerTerminalPhase::AwaitDescriptor)
            || !matches!(self.phase, CallerPhase::ClaimedBootstrap)
            || clock.original_deadline() != self.identity_deadline
        {
            return Err(CallerBootstrapError::TerminalTransition);
        }
        self.terminal_phase = CallerTerminalPhase::Refused;
        let (cutoff, deadline) = self.settlement_clock(clock)?;
        let start = self
            .read_ack_storage
            .take()
            .ok_or(CallerBootstrapError::TerminalTransition)?
            .encode(&CallerTerminalControl::CompletedClose {
                authority: self.authority,
                deadline,
                stop: clock.stop_stamp().map_err(CallerBootstrapError::Deadline)?,
            })
            .map_err(CallerBootstrapError::Control)?;
        self.outer_control
            .transport()
            .send_prepared(&start, &[], cutoff)
            .map_err(CallerBootstrapError::Control)?;
        let ack_storage = start.into_storage();
        let received = self
            .outer_control
            .transport()
            .receive_prepared::<OuterTerminalReply>(
                &mut self.receive,
                OuterTerminalReply::rights_count,
                cutoff,
            )
            .map_err(CallerBootstrapError::Control)?;
        let sender = received
            .credentials
            .ok_or(CallerBootstrapError::MissingSender)?;
        let OuterTerminalReply::ReportDescriptor { authority, bytes } = received.control else {
            return Err(CallerBootstrapError::TerminalReplyMismatch);
        };
        let descriptor = received
            .rights
            .pop()
            .ok_or(CallerBootstrapError::TerminalReplyMismatch)?;
        if sender.pid
            != self
                .outer_pid
                .ok_or(CallerBootstrapError::MissingOuterPin)?
            || sender.uid != self.caller_uid
            || sender.gid != self.caller_gid
            || authority != self.authority
        {
            return Err(CallerBootstrapError::TerminalReplyMismatch);
        }
        creator::require_live(
            self.outer_pin
                .as_ref()
                .ok_or(CallerBootstrapError::MissingOuterPin)?,
        )
        .map_err(CallerBootstrapError::Io)?;
        let expected_bytes =
            usize::try_from(bytes).map_err(|_| CallerBootstrapError::ReservationUnrepresentable)?;
        let read = self
            .report_read
            .take()
            .ok_or(CallerBootstrapError::TerminalTransition)?;
        let report = read
            .read_received(descriptor, expected_bytes, Some(cutoff))
            .map_err(CallerBootstrapError::Report)?;
        let ack = ack_storage
            .encode(&CallerTerminalControl::ReadCompleted { authority, bytes })
            .map_err(CallerBootstrapError::Control)?;
        self.outer_control
            .transport()
            .send_prepared(&ack, &[], cutoff)
            .map_err(CallerBootstrapError::Control)?;
        self.terminal_phase = CallerTerminalPhase::AwaitCommit;
        Ok(report)
    }

    /// The final-only decoder may drain a complete queued commit after expected normal O exit.
    /// This returns only progress: cached peaks remain private/provisional until actual O custody
    /// and normal child wait/reap are authenticated. Partial/malformed frames never supply peaks.
    pub(super) fn receive_terminal_commit(
        &mut self,
        clock: &mut ExecutionClock,
    ) -> Result<bool, CallerBootstrapError> {
        if !matches!(self.terminal_phase, CallerTerminalPhase::AwaitCommit)
            || clock.original_deadline() != self.identity_deadline
        {
            return Err(CallerBootstrapError::TerminalTransition);
        }
        let (cutoff, _) = self.settlement_clock(clock)?;
        self.terminal_phase = CallerTerminalPhase::Refused;
        let Some(received) = self
            .terminal_receive
            .advance::<OuterTerminalReply>(&self.outer_control.transport(), cutoff)
            .map_err(CallerBootstrapError::Control)?
        else {
            self.terminal_phase = CallerTerminalPhase::AwaitCommit;
            return Ok(false);
        };
        let sender = received
            .credentials
            .ok_or(CallerBootstrapError::MissingSender)?;
        let OuterTerminalReply::Committed {
            authority,
            peaks,
            stop,
        } = received.control
        else {
            return Err(CallerBootstrapError::TerminalReplyMismatch);
        };
        if sender.pid
            != self
                .outer_pid
                .ok_or(CallerBootstrapError::MissingOuterPin)?
            || sender.uid != self.caller_uid
            || sender.gid != self.caller_gid
            || authority != self.authority
        {
            return Err(CallerBootstrapError::TerminalReplyMismatch);
        }
        clock
            .adopt_stop(stop, self.identity_clock)
            .map_err(CallerBootstrapError::Deadline)?;
        // The authenticated L owns its unreaped actual O Child while this commit is decoded.
        // O may already have exited; original sender binding is not replaced by PID reopening.
        creator::require_live(&self.launcher_identity()?.launcher_pin)
            .map_err(CallerBootstrapError::Io)?;
        self.terminal_phase = CallerTerminalPhase::ReceivedCommit(peaks);
        Ok(true)
    }

    /// Even a syntactically complete commit cannot supply measurements after abnormal O exit.
    /// Require actual authenticated child wait/reap, creator join and absence of extra stream
    /// bytes. Capture settlement and original-deadline classification remain RunOwner duties.
    pub(super) fn finish_terminal_after_roles(
        &mut self,
        roles: &CallerRoleSettlement,
    ) -> Result<Option<MeasuredPeaks>, CallerBootstrapError> {
        if roles.authority != self.authority {
            return Err(CallerBootstrapError::TerminalReplyMismatch);
        }
        if !matches!(
            self.outer_settled,
            Some(OuterChildSettlement::Reaped {
                outcome: BackendExit::Code(0)
            })
        ) {
            return Err(CallerBootstrapError::OuterExitAbnormal);
        }
        let CallerTerminalPhase::ReceivedCommit(peaks) = self.terminal_phase else {
            return Err(CallerBootstrapError::TerminalTransition);
        };
        if !self
            .terminal_receive
            .confirm_end(&self.outer_control.transport(), roles.cutoff)
            .map_err(CallerBootstrapError::Control)?
        {
            return Ok(None);
        }
        self.terminal_phase = CallerTerminalPhase::Finished;
        Ok(Some(peaks))
    }

    /// Actual O-child custody is established before L reap/creator join. This method is called
    /// by the original whole-chain owner with its existing first-stop/original cutoff; it does
    /// not close the independent I lease or authorize proof/report acceptance.
    pub(super) fn settle_launcher_chain(
        &mut self,
        clock: &ExecutionClock,
        mode: LauncherSettlementMode,
    ) -> Result<CallerRoleSettlement, CallerBootstrapError> {
        if clock.original_deadline() != self.identity_deadline {
            return Err(CallerBootstrapError::SettingsMismatch);
        }
        let (cutoff, deadline) = self.settlement_clock(clock)?;
        let storage = self
            .settlement_storage
            .take()
            .ok_or(CallerBootstrapError::SettlementAlreadyAttempted)?;
        let frame = storage
            .encode(&LauncherControl::Settle {
                authority: self.authority,
                deadline,
                mode,
            })
            .map_err(CallerBootstrapError::Control)?;
        self.launcher_control
            .transport()
            .send_prepared(&frame, &[], cutoff)
            .map_err(CallerBootstrapError::Control)?;
        let received = self
            .launcher_control
            .transport()
            .receive_prepared::<LauncherReply>(
                &mut self.receive,
                LauncherReply::rights_count,
                cutoff,
            )
            .map_err(CallerBootstrapError::Control)?;
        let launcher = self
            .identity
            .as_ref()
            .ok_or(CallerBootstrapError::MissingLauncher)?;
        let sender = received
            .credentials
            .ok_or(CallerBootstrapError::MissingSender)?;
        if sender.pid != launcher.launcher_pid.as_raw_pid()
            || sender.uid != self.caller_uid
            || sender.gid != self.caller_gid
        {
            return Err(CallerBootstrapError::SettlementReplyMismatch);
        }
        let LauncherReply::OuterSettled {
            identity,
            authority,
            custody,
        } = received.control
        else {
            return Err(CallerBootstrapError::SettlementReplyMismatch);
        };
        if identity != self.build_identity || authority != self.authority {
            return Err(CallerBootstrapError::SettlementReplyMismatch);
        }
        if let Some(pin) = &self.outer_pin {
            if matches!(custody, OuterChildSettlement::NotCreated) {
                return Err(CallerBootstrapError::SettlementReplyMismatch);
            }
            // A malformed arm candidate is retained for descriptor cleanup, but never grants
            // process authority. Only completed actual child/pidfd binding set outer_pid.
            if self.outer_pid.is_some()
                && !super::creator::terminated(pin).map_err(CallerBootstrapError::Io)?
            {
                return Err(CallerBootstrapError::OuterTerminationUnconfirmed);
            }
        }
        self.outer_settled = Some(custody);
        self.launcher_control
            .transport()
            .send_prepared(&self.retirement_frame, &[], cutoff)
            .map_err(CallerBootstrapError::Control)?;
        let spawner = self
            .spawner
            .as_mut()
            .ok_or(CallerBootstrapError::MissingLauncher)?;
        spawner
            .settle_launcher(cutoff)
            .map_err(CallerBootstrapError::Io)?;
        spawner.join(cutoff).map_err(CallerBootstrapError::Io)?;
        if Instant::now() >= cutoff {
            return Err(CallerBootstrapError::Deadline(DeadlineError::Expired));
        }
        Ok(CallerRoleSettlement {
            cutoff,
            authority: self.authority,
        })
    }

    /// Preserve the one encoded finite bound, or encode the original None first-stop cutoff
    /// once. Neither later read acknowledgment nor L settlement may recompute/reset that clock.
    fn settlement_clock(
        &mut self,
        clock: &ExecutionClock,
    ) -> Result<(Instant, RoleDeadline), CallerBootstrapError> {
        if clock.original_deadline() != self.identity_deadline {
            return Err(CallerBootstrapError::SettingsMismatch);
        }
        let cutoff = clock
            .settlement_deadline()
            .map_err(CallerBootstrapError::Deadline)?;
        let deadline = clock
            .stop_deadline(self.identity_clock)
            .map_err(CallerBootstrapError::Deadline)?;
        if let Some((retained, previous)) = self.settlement_transfer {
            if cutoff > retained
                || !deadline
                    .no_later_than(previous)
                    .map_err(CallerBootstrapError::Deadline)?
            {
                return Err(CallerBootstrapError::SettingsMismatch);
            }
        }
        self.settlement_transfer = Some((cutoff, deadline));
        Ok((cutoff, deadline))
    }

    /// The original identity clock is independent from the finite startup cap. None has its
    /// original never-elapsing meaning, never a deadline synthesized at backend completion.
    pub(super) fn identity_deadline(&self) -> Option<Instant> {
        self.identity_deadline
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

#[cfg(test)]
mod tests {
    use super::{PreparedCleanupDetail, CLEANUP_DETAIL_BYTES};

    /// Trace: FR-034-AC-38
    #[test]
    fn settlement_detail_uses_its_prepared_capacity_and_preserves_utf8_at_the_cap() {
        let prepared = PreparedCleanupDetail::prepare().unwrap();
        let capacity = prepared.text.capacity();
        let input = "界".repeat(CLEANUP_DETAIL_BYTES);
        let detail = prepared.seal(format_args!("owned role: {input}"));
        assert!(detail.len() <= CLEANUP_DETAIL_BYTES);
        assert_eq!(detail.capacity(), capacity);
        assert!(detail.starts_with("owned role: "));
        assert!(detail.len() > CLEANUP_DETAIL_BYTES - 3);
        assert!(detail.ends_with('界'));
    }

    /// Trace: FR-034-AC-38
    #[test]
    fn short_settlement_detail_preserves_actual_diagnostic_fields() {
        let prepared = PreparedCleanupDetail::prepare().unwrap();
        let capacity = prepared.text.capacity();
        let detail = prepared.seal(format_args!("O={} L={} wait={}", 17, 11, "pending"));
        assert_eq!(detail, "O=17 L=11 wait=pending");
        assert_eq!(detail.capacity(), capacity);
    }
}

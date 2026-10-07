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
        role_pair, ControlError, FrameStorage, GuardianEndpoint, IncrementalReceive,
        IncrementalSend, PreparedFrame, PreparedReceive, RoleCaller, TerminalReceive,
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
        OuterStartupControl, OuterTerminalReply, OwnerStopCause, RunSettings,
    },
    spawner::{RetainedSpawner, SpawnIdentity},
    stages::{Bootstrap, InitReady, PreparedDispatch, StageError, VerifiedInitReady},
    stdin::OriginalStdin,
};

#[derive(Debug)]
pub(super) enum CallerBootstrapError {
    Io(io::Error),
    Control(ControlError),
    Deadline(DeadlineError),
    Report(ReportError),
    Stage(StageError),
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
    OwnerStopObserved,
    PolicyRefusalObserved,
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
            Self::Stage(error) => Some(error),
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
            | Self::OuterExitAbnormal
            | Self::OwnerStopObserved
            | Self::PolicyRefusalObserved => None,
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
    start_frame: Option<PreparedFrame>,
    launcher_start: Option<IncrementalSend>,
    launcher_start_sent: bool,
    launcher_ready: bool,
    launcher_receive: IncrementalReceive,
    launcher_failed: bool,
    startup_failed: bool,
    startup_finished: bool,
    pub(super) report_read: Option<PreparedReportRead>,
    pub(super) dispatch: Option<PreparedDispatch>,
    pub(super) stdin: OriginalStdin,
    pub(super) streams: CallerStreams,
    pub(super) publication: Arc<Publication>,
    receive: PreparedReceive,
    inner_receive: IncrementalReceive,
    outer_receive: IncrementalReceive,
    phase_send: Option<IncrementalSend>,
    phase_failed: bool,
    inner_hello: Option<IncrementalSend>,
    inner_hello_sent: bool,
    inner_auth_failed: bool,
    inner_ready: Option<VerifiedInitReady>,
    startup_context: super::startup_cause::PreparedStartupContext,
    policy_refusal: Option<super::startup_envelope::PolicyFailureCause>,
    policy_refusal_stop: Option<super::role_deadline::StopStamp>,
    pending_setup_refusal: Option<super::startup_envelope::PolicyFailureCause>,
    policy_refusal_settled: bool,
    terminal_receive: TerminalReceive,
    read_ack_storage: Option<FrameStorage>,
    cancel_storage: Option<FrameStorage>,
    cancel_close: Option<IncrementalSend>,
    cancel_close_sent: bool,
    terminal_phase: CallerTerminalPhase,
    identity_records: RefCell<creator::PreparedIdentity>,
    named_buffers: u64,
    memory_bytes: std::num::NonZeroU64,
    pending_owner_stop: Option<PendingOwnerStop>,
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
    phase_frames: [Option<PreparedFrame>; 3],
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

#[derive(Clone, Copy, Eq, PartialEq)]
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

/// Explicit C-owned phase selection. A supplied action is not a fixture pause or permission
/// to advance a different boundary; O still authenticates the existing exact private command.
#[derive(Clone, Copy)]
pub(super) enum CallerPhaseAction {
    BeginMonitor,
    ClaimInner,
    ReleaseGate,
}

impl CallerPhaseAction {
    fn transition(self) -> (usize, CallerPhase, CallerPhase) {
        match self {
            Self::BeginMonitor => (0, CallerPhase::BeforeMonitor, CallerPhase::AwaitMonitor),
            Self::ClaimInner => (1, CallerPhase::Bootstrap, CallerPhase::AwaitClaim),
            Self::ReleaseGate => (2, CallerPhase::ClaimedGated, CallerPhase::AwaitGate),
        }
    }
}

enum OwnerStopReceiver {
    Startup,
    Terminal,
}

struct PendingOwnerStop {
    cause: OwnerStopCause,
    peaks: MeasuredPeaks,
    receiver: OwnerStopReceiver,
}

enum CallerTerminalPhase {
    AwaitDescriptor,
    AwaitCommit,
    ReceivedCommit(MeasuredPeaks),
    Finished,
    Refused,
}

impl CallerBootstrap {
    /// The actual prepared finite startup cutoff, never a fresh cap at the caller driver.
    pub(super) fn startup_cutoff(&self) -> Instant {
        self.deadline
    }

    /// The same original O stream is multiplexed during acknowledged backend execution. A
    /// complete authenticated stop only shortens the clock; this does not accept any result.
    pub(super) fn driver_control_step(
        &mut self,
        clock: &mut ExecutionClock,
        cutoff: Option<Instant>,
    ) -> Result<bool, CallerBootstrapError> {
        self.receive_startup_control(clock, cutoff, false)
    }

    pub(super) fn driver_capture_stop(
        &self,
        clock: &mut ExecutionClock,
    ) -> Result<Instant, CallerBootstrapError> {
        if clock.original_deadline() != self.identity_deadline {
            return Err(CallerBootstrapError::SettingsMismatch);
        }
        clock
            .capture_caller_stop(self.identity_clock)
            .map_err(CallerBootstrapError::Deadline)
    }

    pub(super) fn driver_adopt_completion(
        &self,
        clock: &mut ExecutionClock,
        stop: super::role_deadline::StopStamp,
    ) -> Result<Instant, CallerBootstrapError> {
        if clock.original_deadline() != self.identity_deadline
            || stop.origin != super::role_deadline::StopOrigin::Inner
        {
            return Err(CallerBootstrapError::SettingsMismatch);
        }
        clock
            .adopt_stop(stop, self.identity_clock)
            .map_err(CallerBootstrapError::Deadline)
    }

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
        let memory_bytes = settings.memory_bytes;
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
        let inner_receive = IncrementalReceive::prepare().map_err(CallerBootstrapError::Control)?;
        let startup_context = super::startup_cause::PreparedStartupContext::new(
            super::startup_envelope::CONTEXT_BYTES,
        )
        .map_err(|error| CallerBootstrapError::Io(io::Error::other(error)))?;
        let outer_receive = IncrementalReceive::prepare().map_err(CallerBootstrapError::Control)?;
        let launcher_receive =
            IncrementalReceive::prepare().map_err(CallerBootstrapError::Control)?;
        let terminal_receive = TerminalReceive::prepare().map_err(CallerBootstrapError::Control)?;
        let read_ack_storage = FrameStorage::prepare().map_err(CallerBootstrapError::Control)?;
        let cancel_storage = FrameStorage::prepare().map_err(CallerBootstrapError::Control)?;
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
            super::caller_driver::CallerDriver::metadata_reservation()
                .map_err(|error| CallerBootstrapError::Io(io::Error::other(error)))?,
            super::caller_execution::CallerExecution::additional_metadata()?,
            super::role_protocol::cancellation_decode_bytes()
                .map_err(CallerBootstrapError::Control)?,
            u64::try_from(startup_context.reserved_bytes())
                .map_err(|_| CallerBootstrapError::ReservationUnrepresentable)?,
            super::startup_envelope::inner_startup_decode_bytes()
                .map_err(CallerBootstrapError::Control)?,
            super::startup_envelope::inner_event_decode_bytes()
                .map_err(CallerBootstrapError::Control)?,
            super::stages::CallerLeaseClient::metadata_reservation()
                .map_err(CallerBootstrapError::Stage)?,
            super::role_protocol::outer_startup_decode_bytes()
                .map_err(CallerBootstrapError::Control)?,
            super::role_protocol::report_decode_bytes().map_err(CallerBootstrapError::Control)?,
            u64::try_from(std::mem::size_of::<ExecutionClock>())
                .map_err(|_| CallerBootstrapError::ReservationUnrepresentable)?,
            launcher_receive
                .reserved_bytes()
                .map_err(CallerBootstrapError::Control)?,
            outer_receive
                .reserved_bytes()
                .map_err(CallerBootstrapError::Control)?,
            inner_receive
                .reserved_bytes()
                .map_err(CallerBootstrapError::Control)?,
            terminal_receive
                .reserved_bytes()
                .map_err(CallerBootstrapError::Control)?,
            cancel_storage
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
            start_frame: Some(start_frame),
            launcher_start: None,
            launcher_start_sent: false,
            launcher_ready: false,
            launcher_receive,
            launcher_failed: false,
            startup_failed: false,
            startup_finished: false,
            report_read: Some(report_read),
            dispatch: Some(dispatch),
            stdin,
            streams,
            publication,
            receive,
            inner_receive,
            outer_receive,
            phase_send: None,
            phase_failed: false,
            inner_hello: None,
            inner_hello_sent: false,
            inner_auth_failed: false,
            inner_ready: None,
            startup_context,
            policy_refusal: None,
            policy_refusal_stop: None,
            pending_setup_refusal: None,
            policy_refusal_settled: false,
            terminal_receive,
            read_ack_storage: Some(read_ack_storage),
            cancel_storage: Some(cancel_storage),
            cancel_close: None,
            cancel_close_sent: false,
            terminal_phase: CallerTerminalPhase::AwaitDescriptor,
            identity_records: RefCell::new(identity_records),
            named_buffers,
            memory_bytes,
            pending_owner_stop: None,
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
            phase_frames: phase_frames.map(Some),
            phase: CallerPhase::AwaitArm,
            inner_identity: None,
        })
    }

    /// One ordinary C startup iteration over the retained L/O/I sources. This drives the exact
    /// existing phase operations in order, while partial sends/receives return to the caller's
    /// original clock/capture loop. No control wait creates another budget or pauses O sampling.
    /// The positive result owns the original authenticated I lease, not a replacement endpoint.
    pub(super) fn startup_step(
        &mut self,
        clock: &mut ExecutionClock,
        cutoff: Instant,
    ) -> Result<Option<InitReady>, CallerBootstrapError> {
        if self.startup_failed || self.startup_finished {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.startup_failed = true;
        let result = self.advance_startup(clock, cutoff.min(self.deadline));
        if result.is_ok() {
            self.startup_failed = false;
        }
        result
    }

    fn advance_startup(
        &mut self,
        clock: &mut ExecutionClock,
        cutoff: Instant,
    ) -> Result<Option<InitReady>, CallerBootstrapError> {
        if Instant::now() >= cutoff {
            return Err(CallerBootstrapError::Control(ControlError::Deadline));
        }
        if self.spawner.is_none() {
            // The actual owned spawner is installed before any fallible startup wait. Failure
            // poisons this driver; neither retry nor an error return replaces that owner.
            self.begin_launch_until(cutoff)?;
            return Ok(None);
        }
        if !self.launcher_ready {
            self.launch_step(cutoff)?;
        }
        if self.launcher_start_sent && matches!(self.phase, CallerPhase::AwaitArm) {
            // L Ready and O Armed are independent sources. A quiet/partial L message cannot
            // make C enter a blocking O authentication wait, or vice versa.
            self.confirm_outer_arm_step(cutoff)?;
        }
        if !self.launcher_ready || matches!(self.phase, CallerPhase::AwaitArm) {
            return Ok(None);
        }
        if self.receive_startup_control(clock, Some(cutoff), true)? {
            return Err(CallerBootstrapError::OwnerStopObserved);
        }
        if self.outer_receive.has_partial_frame() {
            return Ok(None);
        }
        match self.phase {
            CallerPhase::BeforeMonitor => {
                self.send_phase_step(CallerPhaseAction::BeginMonitor, cutoff)?;
            }
            CallerPhase::Bootstrap => {
                self.send_phase_step(CallerPhaseAction::ClaimInner, cutoff)?;
            }
            CallerPhase::ClaimedGated => {
                self.send_phase_step(CallerPhaseAction::ReleaseGate, cutoff)?;
            }
            CallerPhase::AwaitMonitor | CallerPhase::AwaitClaim | CallerPhase::AwaitGate => {
                if self.phase_send.is_some() {
                    let action = match self.phase {
                        CallerPhase::AwaitMonitor => CallerPhaseAction::BeginMonitor,
                        CallerPhase::AwaitClaim => CallerPhaseAction::ClaimInner,
                        CallerPhase::AwaitGate => CallerPhaseAction::ReleaseGate,
                        CallerPhase::AwaitArm
                        | CallerPhase::BeforeMonitor
                        | CallerPhase::Bootstrap
                        | CallerPhase::ClaimedGated
                        | CallerPhase::ClaimedBootstrap => {
                            return Err(CallerBootstrapError::UnexpectedPhase);
                        }
                    };
                    self.send_phase_step(action, cutoff)?;
                }
            }
            CallerPhase::ClaimedBootstrap => {
                if self.inner_ready.is_none() && !self.authenticate_inner_step(clock, cutoff)? {
                    return Ok(None);
                }
                self.require_startup_owners(cutoff)?;
                let ready = self.take_authenticated_inner()?;
                self.startup_finished = true;
                return Ok(Some(ready));
            }
            CallerPhase::AwaitArm => return Err(CallerBootstrapError::UnexpectedPhase),
        }
        Ok(None)
    }

    /// One original charged stream consumer for phases and provisional O stop clocks. Complete
    /// OwnerStop after EOF can only shorten the original clock and poison further startup; it
    /// grants no phase/report/evidence. All ordinary phase frames retain strict EOF precedence.
    fn receive_startup_control(
        &mut self,
        clock: &mut ExecutionClock,
        cutoff: Option<Instant>,
        allow_phase: bool,
    ) -> Result<bool, CallerBootstrapError> {
        if clock.original_deadline() != self.identity_deadline
            || self.pending_owner_stop.is_some()
            || self.pending_setup_refusal.is_some()
        {
            return Err(CallerBootstrapError::TerminalTransition);
        }
        let launcher = self.launcher_identity()?;
        creator::require_live(&launcher.creator_pin).map_err(CallerBootstrapError::Io)?;
        creator::require_live(&launcher.launcher_pin).map_err(CallerBootstrapError::Io)?;
        self.launcher_control
            .transport()
            .refuse_observable_eof()
            .map_err(CallerBootstrapError::Control)?;
        if self.outer_pin.is_none() || self.outer_pid.is_none() {
            return Err(CallerBootstrapError::MissingOuterPin);
        }
        let Some(received) = self
            .outer_receive
            .advance_clock_only_optional(
                &self.outer_control.transport(),
                OuterStartupControl::rights_count,
                cutoff,
                super::role_protocol::decode_outer_startup,
                OuterStartupControl::clock_only,
            )
            .map_err(CallerBootstrapError::Control)?
        else {
            return Ok(false);
        };
        let sender = received
            .credentials
            .ok_or(CallerBootstrapError::MissingSender)?;
        let control = received.control;
        let pin = received.rights.pop();
        match control {
            OuterStartupControl::Phase(reply) => {
                if !allow_phase || self.phase_send.is_some() {
                    return Err(CallerBootstrapError::UnexpectedPhase);
                }
                let cutoff = cutoff.ok_or(CallerBootstrapError::UnexpectedPhase)?;
                self.require_startup_owners(cutoff)?;
                self.accept_phase_reply(reply, Some(sender), pin, cutoff)?;
                Ok(false)
            }
            OuterStartupControl::OwnerStop {
                authority,
                peaks,
                stop,
                cause,
            } => {
                // O identity was positively bound at Armed and L still retains its actual Child.
                // Do not reopen an exited PID or treat EOF itself as stop authority.
                if sender.pid
                    != self
                        .outer_pid
                        .ok_or(CallerBootstrapError::MissingOuterPin)?
                    || sender.uid != self.caller_uid
                    || sender.gid != self.caller_gid
                    || authority != self.authority
                    || pin.is_some()
                    || peaks.charged_bytes < peaks.tree_rss_bytes
                    || (matches!(cause, OwnerStopCause::ResourceExhausted)
                        && peaks.charged_bytes <= self.memory_bytes.get())
                {
                    return Err(CallerBootstrapError::TerminalReplyMismatch);
                }
                clock
                    .adopt_stop(stop, self.identity_clock)
                    .map_err(CallerBootstrapError::Deadline)?;
                let original = clock
                    .settlement_deadline()
                    .map_err(CallerBootstrapError::Deadline)?;
                if Instant::now() >= original {
                    return Err(CallerBootstrapError::Deadline(DeadlineError::Expired));
                }
                self.pending_owner_stop = Some(PendingOwnerStop {
                    cause,
                    peaks,
                    receiver: OwnerStopReceiver::Startup,
                });
                self.phase_failed = true;
                self.inner_auth_failed = true;
                // No final/proof token is minted. The caller must settle under this same shortened
                // clock and then prove Code0 O wait plus L/captures/creator, including stream EOF.
                Ok(true)
            }
            OuterStartupControl::SetupRefused {
                authority,
                peaks,
                stop,
                failure,
            } => {
                // I's complete strict negative reply must already be retained, before C closes
                // the lease that lets I terminate and O finish this nonreport transaction.
                if sender.pid
                    != self
                        .outer_pid
                        .ok_or(CallerBootstrapError::MissingOuterPin)?
                    || sender.uid != self.caller_uid
                    || sender.gid != self.caller_gid
                    || authority != self.authority
                    || pin.is_some()
                    || self.policy_refusal != Some(failure)
                    || self.policy_refusal_stop != Some(stop)
                    || peaks.charged_bytes < peaks.tree_rss_bytes
                {
                    return Err(CallerBootstrapError::TerminalReplyMismatch);
                }
                let cutoff = clock
                    .adopt_stop(stop, self.identity_clock)
                    .map_err(CallerBootstrapError::Deadline)?;
                if Instant::now() >= cutoff {
                    return Err(CallerBootstrapError::Deadline(DeadlineError::Expired));
                }
                self.pending_setup_refusal = Some(failure);
                self.phase_failed = true;
                self.inner_auth_failed = true;
                Ok(true)
            }
        }
    }

    /// During irreversible cancellation, inspect only the complete authenticated original stop
    /// on the SAME framer before waiting for L. The caller supplies its already active original
    /// settlement cutoff; no phase can advance and no partial frame is restarted or skipped.
    pub(super) fn receive_owner_stop_clock(
        &mut self,
        clock: &mut ExecutionClock,
        cutoff: Instant,
    ) -> Result<bool, CallerBootstrapError> {
        self.startup_failed = true;
        self.phase_failed = true;
        self.inner_auth_failed = true;
        self.receive_startup_control(clock, Some(cutoff), false)
    }

    /// Provisional candidate remains unavailable until real normal O custody and original role
    /// settlement. Captures and classification are still the whole caller owner's later duties.
    pub(super) fn finish_owner_stop_after_roles(
        &mut self,
        roles: &CallerRoleSettlement,
    ) -> Result<Option<(OwnerStopCause, MeasuredPeaks)>, CallerBootstrapError> {
        if roles.authority != self.authority || self.pending_owner_stop.is_none() {
            return Err(CallerBootstrapError::TerminalTransition);
        }
        if !matches!(
            self.outer_settled,
            Some(OuterChildSettlement::Reaped {
                outcome: BackendExit::Code(0)
            })
        ) {
            return Err(CallerBootstrapError::OuterExitAbnormal);
        }
        let ended = match self
            .pending_owner_stop
            .as_ref()
            .ok_or(CallerBootstrapError::TerminalTransition)?
            .receiver
        {
            OwnerStopReceiver::Startup => self
                .outer_receive
                .confirm_end(&self.outer_control.transport(), roles.cutoff),
            OwnerStopReceiver::Terminal => self
                .terminal_receive
                .confirm_end(&self.outer_control.transport(), roles.cutoff),
        }
        .map_err(CallerBootstrapError::Control)?;
        if !ended {
            return Ok(None);
        }
        let pending = self
            .pending_owner_stop
            .take()
            .ok_or(CallerBootstrapError::TerminalTransition)?;
        self.terminal_phase = CallerTerminalPhase::Finished;
        Ok(Some((pending.cause, pending.peaks)))
    }

    /// Finish only an authenticated original policy-negative transaction after the actual normal
    /// O/L/creator role proof. Captures still belong to the caller and no evidence is minted.
    pub(super) fn finish_setup_refusal_after_roles(
        &mut self,
        roles: &CallerRoleSettlement,
    ) -> Result<bool, CallerBootstrapError> {
        if roles.authority != self.authority
            || Instant::now() >= roles.cutoff
            || self.pending_setup_refusal.is_none()
            || self.pending_setup_refusal != self.policy_refusal
        {
            return Err(CallerBootstrapError::TerminalTransition);
        }
        if !matches!(
            self.outer_settled,
            Some(OuterChildSettlement::Reaped {
                outcome: BackendExit::Code(0),
            })
        ) {
            return Err(CallerBootstrapError::OuterExitAbnormal);
        }
        if !self
            .outer_receive
            .confirm_end(&self.outer_control.transport(), roles.cutoff)
            .map_err(CallerBootstrapError::Control)?
        {
            return Ok(false);
        }
        self.pending_setup_refusal = None;
        self.policy_refusal_settled = true;
        Ok(true)
    }

    /// Stores actual spawner custody BEFORE any wait/pin/control error. A failed handshake must
    /// be settled by this retained owner; no retry or replacement helper is attempted here.
    pub(super) fn begin_launch(&mut self) -> Result<(), CallerBootstrapError> {
        self.begin_launch_until(self.deadline)
    }

    fn begin_launch_until(&mut self, cutoff: Instant) -> Result<(), CallerBootstrapError> {
        let cutoff = cutoff.min(self.deadline);
        if Instant::now() >= cutoff {
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
            .wait_started(cutoff)
            .map_err(CallerBootstrapError::Io)?;
        self.identity = Some(identity);
        Ok(())
    }

    pub(super) fn launch(&mut self) -> Result<(), CallerBootstrapError> {
        self.begin_launch()?;
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
                self.start_frame
                    .as_ref()
                    .ok_or(CallerBootstrapError::LaunchAlreadyAttempted)?,
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
        self.launcher_start_sent = true;
        let received = self
            .launcher_control
            .transport()
            .receive_prepared::<LauncherReply>(
                &mut self.receive,
                LauncherReply::rights_count,
                self.deadline,
            )
            .map_err(CallerBootstrapError::Control)?;
        let control = received.control;
        let sender = received.credentials;
        self.accept_launcher_ready(control, sender)
    }

    fn accept_launcher_ready(
        &mut self,
        control: LauncherReply,
        sender: Option<super::control::PeerCredentials>,
    ) -> Result<(), CallerBootstrapError> {
        let identity = self.launcher_identity()?;
        let sender = sender.ok_or(CallerBootstrapError::MissingSender)?;
        if sender.pid != identity.launcher_pid.as_raw_pid()
            || sender.uid != self.caller_uid
            || sender.gid != self.caller_gid
        {
            return Err(CallerBootstrapError::LauncherReplyMismatch);
        }
        creator::require_live(&identity.launcher_pin).map_err(CallerBootstrapError::Io)?;
        match control {
            LauncherReply::Ready {
                identity,
                authority,
            } if identity == self.build_identity && authority == self.authority => {
                self.launcher_ready = true;
                Ok(())
            }
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

    /// One genuine C/L bootstrap send or receive step. The actual creator/Child identity is
    /// already retained by begin_launch. No O frame may block reception of this parallel source;
    /// partial rights/bytes remain owned, and errors prevent any second launch/admission attempt.
    pub(super) fn launch_step(&mut self, cutoff: Instant) -> Result<bool, CallerBootstrapError> {
        if self.launcher_failed || self.launcher_ready {
            return Err(CallerBootstrapError::LaunchAlreadyAttempted);
        }
        self.launcher_failed = true;
        let result = self.advance_launch(cutoff.min(self.deadline));
        if result.is_ok() {
            self.launcher_failed = false;
        }
        result
    }

    fn advance_launch(&mut self, cutoff: Instant) -> Result<bool, CallerBootstrapError> {
        if Instant::now() >= cutoff {
            return Err(CallerBootstrapError::Control(ControlError::Deadline));
        }
        let identity = self
            .identity
            .as_ref()
            .ok_or(CallerBootstrapError::MissingLauncher)?;
        creator::require_live(&identity.creator_pin).map_err(CallerBootstrapError::Io)?;
        creator::require_live(&identity.launcher_pin).map_err(CallerBootstrapError::Io)?;
        self.launcher_control
            .transport()
            .refuse_observable_eof()
            .map_err(CallerBootstrapError::Control)?;
        if !self.launcher_start_sent {
            if self.launcher_start.is_none() {
                let frame = self
                    .start_frame
                    .take()
                    .ok_or(CallerBootstrapError::LaunchAlreadyAttempted)?;
                self.launcher_start = Some(IncrementalSend::new(frame));
            }
            let inner = self
                .inner_endpoint
                .as_ref()
                .ok_or(CallerBootstrapError::EndpointsConsumed)?;
            let outer = self
                .outer_endpoint
                .as_ref()
                .ok_or(CallerBootstrapError::EndpointsConsumed)?;
            let sent = self
                .launcher_start
                .as_mut()
                .ok_or(CallerBootstrapError::LaunchAlreadyAttempted)?
                .advance(
                    &self.launcher_control.transport(),
                    &[
                        identity.creator_pin.as_fd(),
                        self.caller_pin.as_fd(),
                        inner.as_fd(),
                        outer.as_fd(),
                    ],
                    cutoff,
                )
                .map_err(CallerBootstrapError::Control)?;
            if sent {
                self.launcher_start = None;
                self.launcher_start_sent = true;
                self.inner_endpoint.take();
                self.outer_endpoint.take();
            }
            return Ok(false);
        }
        let Some(received) = self
            .launcher_receive
            .advance::<LauncherReply>(
                &self.launcher_control.transport(),
                LauncherReply::rights_count,
                cutoff,
            )
            .map_err(CallerBootstrapError::Control)?
        else {
            return Ok(false);
        };
        let control = received.control;
        let sender = received.credentials;
        self.accept_launcher_ready(control, sender)?;
        if Instant::now() >= cutoff {
            return Err(CallerBootstrapError::Control(ControlError::Deadline));
        }
        Ok(true)
    }

    /// C accepts only the actual live child of its retained L, authenticated as the O sender.
    /// The received capability is retained before any fallible identity inspection; a failed
    /// publication never authorizes a replacement O or discards the existing role owner's pins.
    pub(super) fn confirm_outer_arm(&mut self) -> Result<NamespaceIdentity, CallerBootstrapError> {
        if self.phase_failed || self.outer_receive.has_partial_frame() || self.outer_pin.is_some() {
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
        let control = received.control;
        let sender = received.credentials;
        self.accept_outer_arm(control, sender, pin, self.deadline)
    }

    /// Incremental arm reception does not block the parallel L startup channel. A partial
    /// authentic frame retains its received rights in the pre-L O receiver until the next step.
    pub(super) fn confirm_outer_arm_step(
        &mut self,
        cutoff: Instant,
    ) -> Result<Option<NamespaceIdentity>, CallerBootstrapError> {
        if self.phase_failed
            || !matches!(self.phase, CallerPhase::AwaitArm)
            || self.outer_pin.is_some()
        {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.phase_failed = true;
        let result = self.advance_outer_arm(cutoff.min(self.deadline));
        if result.is_ok() {
            self.phase_failed = false;
        }
        result
    }

    fn advance_outer_arm(
        &mut self,
        cutoff: Instant,
    ) -> Result<Option<NamespaceIdentity>, CallerBootstrapError> {
        if Instant::now() >= cutoff {
            return Err(CallerBootstrapError::Control(ControlError::Deadline));
        }
        let launcher = self.launcher_identity()?;
        creator::require_live(&launcher.creator_pin).map_err(CallerBootstrapError::Io)?;
        creator::require_live(&launcher.launcher_pin).map_err(CallerBootstrapError::Io)?;
        self.launcher_control
            .transport()
            .refuse_observable_eof()
            .map_err(CallerBootstrapError::Control)?;
        let Some(received) = self
            .outer_receive
            .advance::<OuterArmReply>(
                &self.outer_control.transport(),
                OuterArmReply::rights_count,
                cutoff,
            )
            .map_err(CallerBootstrapError::Control)?
        else {
            return Ok(None);
        };
        let pin = received
            .rights
            .pop()
            .ok_or(CallerBootstrapError::MissingOuterPin)?;
        let control = received.control;
        let sender = received.credentials;
        self.accept_outer_arm(control, sender, pin, cutoff)
            .map(Some)
    }

    fn accept_outer_arm(
        &mut self,
        control: OuterArmReply,
        sender: Option<super::control::PeerCredentials>,
        pin: OwnedFd,
        cutoff: Instant,
    ) -> Result<NamespaceIdentity, CallerBootstrapError> {
        let launcher = self
            .identity
            .as_ref()
            .ok_or(CallerBootstrapError::MissingLauncher)?;
        self.outer_pin = Some(pin);
        let pin = self
            .outer_pin
            .as_ref()
            .ok_or(CallerBootstrapError::MissingOuterPin)?;
        let sender = sender.ok_or(CallerBootstrapError::MissingSender)?;
        let OuterArmReply::Armed {
            identity,
            authority,
            namespace,
            network,
            mapped_uid,
            mapped_gid,
        } = control;
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
        if Instant::now() >= cutoff {
            return Err(CallerBootstrapError::Deadline(DeadlineError::Expired));
        }
        self.outer_namespace = Some(namespace);
        self.outer_network = Some(network);
        self.outer_pid = Some(pid);
        self.phase = CallerPhase::BeforeMonitor;
        Ok(namespace)
    }

    pub(super) fn begin_monitor(&mut self) -> Result<(), CallerBootstrapError> {
        if self.phase_failed || !matches!(self.phase, CallerPhase::BeforeMonitor) {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.phase = CallerPhase::AwaitMonitor;
        self.outer_control
            .transport()
            .send_prepared(
                self.phase_frames
                    .get(0)
                    .and_then(Option::as_ref)
                    .ok_or(CallerBootstrapError::UnexpectedPhase)?,
                &[],
                self.deadline,
            )
            .map_err(CallerBootstrapError::Control)
    }

    pub(super) fn claim_inner(&mut self) -> Result<(), CallerBootstrapError> {
        if self.phase_failed || !matches!(self.phase, CallerPhase::Bootstrap) {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.phase = CallerPhase::AwaitClaim;
        self.outer_control
            .transport()
            .send_prepared(
                self.phase_frames
                    .get(1)
                    .and_then(Option::as_ref)
                    .ok_or(CallerBootstrapError::UnexpectedPhase)?,
                &[],
                self.deadline,
            )
            .map_err(CallerBootstrapError::Control)
    }

    pub(super) fn release_gate(&mut self) -> Result<(), CallerBootstrapError> {
        if self.phase_failed || !matches!(self.phase, CallerPhase::ClaimedGated) {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.phase = CallerPhase::AwaitGate;
        self.outer_control
            .transport()
            .send_prepared(
                self.phase_frames
                    .get(2)
                    .and_then(Option::as_ref)
                    .ok_or(CallerBootstrapError::UnexpectedPhase)?,
                &[],
                self.deadline,
            )
            .map_err(CallerBootstrapError::Control)
    }

    /// One explicit phase-send attempt. All bytes were encoded/reserved before L, and every
    /// unsuccessful attempt returns to the C multiplexer instead of blocking its other owners.
    pub(super) fn send_phase_step(
        &mut self,
        action: CallerPhaseAction,
        cutoff: Instant,
    ) -> Result<bool, CallerBootstrapError> {
        if self.phase_failed {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.phase_failed = true;
        let result = self.advance_phase_send(action, cutoff.min(self.deadline));
        if result.is_ok() {
            self.phase_failed = false;
        }
        result
    }

    fn advance_phase_send(
        &mut self,
        action: CallerPhaseAction,
        cutoff: Instant,
    ) -> Result<bool, CallerBootstrapError> {
        if !self.launcher_ready || self.launcher_failed {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        let (slot, before, awaiting) = action.transition();
        if self.phase_send.is_none() {
            if self.phase != before {
                return Err(CallerBootstrapError::UnexpectedPhase);
            }
            let frame = self
                .phase_frames
                .get_mut(slot)
                .and_then(Option::take)
                .ok_or(CallerBootstrapError::UnexpectedPhase)?;
            self.phase_send = Some(IncrementalSend::new(frame));
            self.phase = awaiting;
        } else if self.phase != awaiting {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.require_startup_owners(cutoff)?;
        let sent = self
            .phase_send
            .as_mut()
            .ok_or(CallerBootstrapError::UnexpectedPhase)?
            .advance(&self.outer_control.transport(), &[], cutoff)
            .map_err(CallerBootstrapError::Control)?;
        if sent {
            self.phase_send = None;
        }
        Ok(sent)
    }

    /// One reply-read attempt from the actual O sender. Rights are retained by this same owner
    /// before identity inspection. Partial frames keep their actual storage/capabilities until
    /// another C iteration; the caller supplies the earliest original work/stop cutoff each time.
    pub(super) fn confirm_phase_step(
        &mut self,
        cutoff: Instant,
    ) -> Result<bool, CallerBootstrapError> {
        if self.phase_failed || self.phase_send.is_some() {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.phase_failed = true;
        let result = self.advance_phase_reply(cutoff.min(self.deadline));
        if result.is_ok() {
            self.phase_failed = false;
        }
        result
    }

    fn advance_phase_reply(&mut self, cutoff: Instant) -> Result<bool, CallerBootstrapError> {
        if !matches!(
            self.phase,
            CallerPhase::AwaitMonitor | CallerPhase::AwaitClaim | CallerPhase::AwaitGate
        ) {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.require_startup_owners(cutoff)?;
        let Some(received) = self
            .outer_receive
            .advance::<OuterPhaseReply>(
                &self.outer_control.transport(),
                OuterPhaseReply::rights_count,
                cutoff,
            )
            .map_err(CallerBootstrapError::Control)?
        else {
            return Ok(false);
        };
        let control = received.control;
        let sender = received.credentials;
        let pin = received.rights.pop();
        self.accept_phase_reply(control, sender, pin, cutoff)?;
        Ok(true)
    }

    fn require_startup_owners(&self, cutoff: Instant) -> Result<(), CallerBootstrapError> {
        self.require_live_roles(Some(cutoff))
    }

    fn require_live_roles(&self, cutoff: Option<Instant>) -> Result<(), CallerBootstrapError> {
        if cutoff.is_some_and(|cutoff| Instant::now() >= cutoff) {
            return Err(CallerBootstrapError::Control(ControlError::Deadline));
        }
        let launcher = self.launcher_identity()?;
        creator::require_live(&launcher.creator_pin).map_err(CallerBootstrapError::Io)?;
        creator::require_live(&launcher.launcher_pin).map_err(CallerBootstrapError::Io)?;
        creator::require_live(
            self.outer_pin
                .as_ref()
                .ok_or(CallerBootstrapError::MissingOuterPin)?,
        )
        .map_err(CallerBootstrapError::Io)?;
        self.launcher_control
            .transport()
            .refuse_observable_eof()
            .map_err(CallerBootstrapError::Control)?;
        self.outer_control
            .transport()
            .refuse_observable_eof()
            .map_err(CallerBootstrapError::Control)
    }

    /// Receive the reply into the already charged buffers. Every received process capability is
    /// installed in this owner BEFORE any fallible identity check, so errors cannot discard custody.
    pub(super) fn confirm_phase(&mut self) -> Result<(), CallerBootstrapError> {
        if self.phase_failed
            || self.phase_send.is_some()
            || self.outer_receive.has_partial_frame()
            || !matches!(
                self.phase,
                CallerPhase::AwaitMonitor | CallerPhase::AwaitClaim | CallerPhase::AwaitGate
            )
        {
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
        let control = received.control;
        let sender = received.credentials;
        let pin = received.rights.pop();
        self.accept_phase_reply(control, sender, pin, self.deadline)
    }

    fn accept_phase_reply(
        &mut self,
        control: OuterPhaseReply,
        sender: Option<super::control::PeerCredentials>,
        pin: Option<OwnedFd>,
        cutoff: Instant,
    ) -> Result<(), CallerBootstrapError> {
        // Select only the expected typed reply; a reordered frame never authorizes a transition.
        let next = match (&self.phase, &control) {
            (CallerPhase::AwaitMonitor, OuterPhaseReply::MonitorSpawned { .. }) => {
                self.monitor_pin = pin;
                CallerPhase::Bootstrap
            }
            (CallerPhase::AwaitClaim, OuterPhaseReply::InnerClaimed { .. }) => {
                self.inner_pin = pin;
                CallerPhase::ClaimedGated
            }
            (CallerPhase::AwaitGate, OuterPhaseReply::GateReleased { .. }) => {
                CallerPhase::ClaimedBootstrap
            }
            _ => return Err(CallerBootstrapError::UnexpectedPhase),
        };
        let sender = sender.ok_or(CallerBootstrapError::MissingSender)?;
        if sender.pid
            != self
                .outer_pid
                .ok_or(CallerBootstrapError::MissingOuterPin)?
            || sender.uid != self.caller_uid
            || sender.gid != self.caller_gid
            || control.authority() != self.authority
        {
            return Err(CallerBootstrapError::PhaseReplyMismatch);
        }
        let outer = self
            .outer_pin
            .as_ref()
            .ok_or(CallerBootstrapError::MissingOuterPin)?;
        creator::require_live(outer).map_err(CallerBootstrapError::Io)?;
        match control {
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
        if Instant::now() >= cutoff {
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

    /// At most one nonblocking C/I control step, using only pre-L reserved storage. The outer
    /// caller loop multiplexes its retained L/O channels between these attempts and supplies
    /// the earliest original work/stop cutoff; this method creates no three-second wait.
    pub(super) fn authenticate_inner_step(
        &mut self,
        clock: &mut ExecutionClock,
        cutoff: Instant,
    ) -> Result<bool, CallerBootstrapError> {
        if self.inner_auth_failed {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        self.inner_auth_failed = true;
        let result = self.advance_inner_authentication(clock, cutoff);
        if result.is_ok() {
            self.inner_auth_failed = false;
        }
        result
    }

    fn advance_inner_authentication(
        &mut self,
        clock: &mut ExecutionClock,
        cutoff: Instant,
    ) -> Result<bool, CallerBootstrapError> {
        if !matches!(self.phase, CallerPhase::ClaimedBootstrap) || self.inner_ready.is_some() {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        let cutoff = cutoff.min(self.deadline);
        if Instant::now() >= cutoff {
            return Err(CallerBootstrapError::Control(ControlError::Deadline));
        }
        self.require_startup_owners(cutoff)?;
        let bootstrap = self
            .inner_bootstrap
            .as_mut()
            .ok_or(CallerBootstrapError::InnerBootstrapConsumed)?;
        if !self.inner_hello_sent {
            if self.inner_hello.is_none() {
                self.inner_hello = Some(IncrementalSend::new(
                    bootstrap
                        .take_hello()
                        .map_err(CallerBootstrapError::Stage)?,
                ));
            }
            let sent = self
                .inner_hello
                .as_mut()
                .ok_or(CallerBootstrapError::UnexpectedPhase)?
                .advance(&bootstrap.transport(), &[], cutoff)
                .map_err(CallerBootstrapError::Control)?;
            if sent {
                self.inner_hello = None;
                self.inner_hello_sent = true;
            }
            return Ok(false);
        }
        let Some(received) = self
            .inner_receive
            .advance_decode(
                &bootstrap.transport(),
                super::startup_envelope::InnerStartupHeader::rights_count,
                cutoff,
                |payload| {
                    super::startup_envelope::decode_inner_startup(
                        payload,
                        &mut self.startup_context,
                    )
                },
            )
            .map_err(CallerBootstrapError::Control)?
        else {
            return Ok(false);
        };
        let sender = received
            .credentials
            .ok_or(CallerBootstrapError::MissingSender)?;
        let control = match received.control {
            super::startup_envelope::InnerStartupHeader::Ready(control) => control,
            super::startup_envelope::InnerStartupHeader::Refused {
                identity,
                authority,
                stop,
                failure,
            } => {
                if identity != self.build_identity
                    || authority != self.authority
                    || stop.origin != super::role_deadline::StopOrigin::Backend
                    || clock.original_deadline() != self.identity_deadline
                {
                    return Err(CallerBootstrapError::PhaseReplyMismatch);
                }
                // Actual retained I/monitor/O/C chain and mapped credentials are required even
                // for a negative reply. A reason string or same backend PID cannot bind I.
                self.verify_ready(sender, 0)
                    .map_err(StageError::Identity)
                    .map_err(CallerBootstrapError::Stage)?;
                let settlement = clock
                    .adopt_stop(stop, self.identity_clock)
                    .map_err(CallerBootstrapError::Deadline)?;
                if Instant::now() >= settlement {
                    return Err(CallerBootstrapError::Control(ControlError::Deadline));
                }
                self.policy_refusal = Some(failure);
                self.policy_refusal_stop = Some(stop);
                return Err(CallerBootstrapError::PolicyRefusalObserved);
            }
        };
        let ready = super::stages::verify_init_ready(self, control, sender, self.authority)
            .map_err(CallerBootstrapError::Stage)?;
        self.inner_ready = Some(ready);
        self.require_startup_owners(cutoff)?;
        self.publication.publish(Stage::InitReady);
        Ok(true)
    }

    /// The SAME pre-L charged I framer transfers logically from Ready to ACK/Completed.
    /// No blocking receive, second consumer or post-L buffer allocation enters C's loop.
    pub(super) fn receive_inner_event_step(
        &mut self,
        client: &mut super::stages::CallerLeaseClient,
        cutoff: Option<Instant>,
    ) -> Result<super::stages::CallerLeaseProgress, CallerBootstrapError> {
        self.require_live_roles(cutoff)?;
        let Some(received) = self
            .inner_receive
            .advance_decode_optional(
                &client.transport(),
                super::startup_envelope::InnerEventHeader::rights_count,
                cutoff,
                super::startup_envelope::decode_inner_event,
            )
            .map_err(CallerBootstrapError::Control)?
        else {
            return Ok(super::stages::CallerLeaseProgress::Pending);
        };
        let sender = received
            .credentials
            .ok_or(CallerBootstrapError::MissingSender)?;
        let control = received.control;
        client
            .accept_event(self, control, sender)
            .map_err(CallerBootstrapError::Stage)
    }

    /// Negative setup metadata is provisional until actual role settlement. Borrowing the
    /// retained original context here still grants no evidence or output-capture conclusion.
    pub(super) fn settled_policy_refusal(
        &self,
        roles: &CallerRoleSettlement,
    ) -> Result<(super::startup_envelope::PolicyFailureCause, &str), CallerBootstrapError> {
        if roles.authority != self.authority
            || Instant::now() >= roles.cutoff
            || !self.policy_refusal_settled
        {
            return Err(CallerBootstrapError::SettlementReplyMismatch);
        }
        let cause = self
            .policy_refusal
            .ok_or(CallerBootstrapError::UnexpectedPhase)?;
        Ok((cause, self.startup_context.context()))
    }

    /// Publish the actual original C trigger to O BEFORE the caller closes its I lease.
    /// The same pre-L close buffer is consumed once. A pending partial phase command cannot
    /// be replaced/spliced; refusing it retains all send/lease/process owners for settlement.
    pub(super) fn cancel_close_step(
        &mut self,
        clock: &mut ExecutionClock,
    ) -> Result<bool, CallerBootstrapError> {
        if self.cancel_close_sent {
            return Ok(true);
        }
        if clock.original_deadline() != self.identity_deadline
            || self
                .phase_send
                .as_ref()
                .is_some_and(IncrementalSend::has_partial_frame)
        {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        if self.cancel_close.is_none() {
            clock
                .capture_caller_stop(self.identity_clock)
                .map_err(CallerBootstrapError::Deadline)?;
            let (_, deadline) = self.settlement_clock(clock)?;
            let frame = self
                .cancel_storage
                .take()
                .ok_or(CallerBootstrapError::TerminalTransition)?
                .encode(&CallerTerminalControl::CancelClose {
                    authority: self.authority,
                    deadline,
                    stop: clock
                        .caller_stop_stamp()
                        .map_err(CallerBootstrapError::Deadline)?,
                })
                .map_err(CallerBootstrapError::Control)?;
            self.cancel_close = Some(IncrementalSend::new(frame));
            // Only a zero-progress phase frame can be retired. No complete phase reply is
            // promoted after cancellation and the same receive framer retains its actual bytes.
            self.phase_send.take();
            self.phase_failed = true;
            self.inner_auth_failed = true;
        }
        let cutoff = clock
            .settlement_deadline()
            .map_err(CallerBootstrapError::Deadline)?;
        let sent = self
            .cancel_close
            .as_mut()
            .ok_or(CallerBootstrapError::TerminalTransition)?
            .advance(&self.outer_control.transport(), &[], cutoff)
            .map_err(CallerBootstrapError::Control)?;
        if sent {
            self.cancel_close.take();
            self.cancel_close_sent = true;
        }
        Ok(sent)
    }

    /// Close only C's still-owned pre-Dispatch lease while retaining every process, capture and
    /// creator authority in this owner. This performs cancellation, not termination proof. Once
    /// Ready transfers the lease to CallerLeaseClient, its actual owner must close it instead.
    pub(super) fn close_bootstrap_lease(&mut self) -> Result<(), CallerBootstrapError> {
        let bootstrap = self
            .inner_bootstrap
            .take()
            .ok_or(CallerBootstrapError::InnerBootstrapConsumed)?;
        self.inner_auth_failed = true;
        self.phase_failed = true;
        self.inner_hello = None;
        drop(bootstrap.into_lease());
        // These read-only ordinals describe the actual close above. No observer callback, I/O,
        // feature branch or controller pause can run between close and publication.
        self.publication.lease_closed();
        self.publication.publish(Stage::LeaseClosing);
        Ok(())
    }

    /// Consume only the already verified stage. Failed/pending receive attempts never take
    /// the actual lease out of this retained whole-chain owner.
    pub(super) fn take_authenticated_inner(&mut self) -> Result<InitReady, CallerBootstrapError> {
        if self.inner_ready.is_none() || self.inner_bootstrap.is_none() {
            return Err(CallerBootstrapError::UnexpectedPhase);
        }
        let ready = self
            .inner_ready
            .take()
            .ok_or(CallerBootstrapError::UnexpectedPhase)?;
        let bootstrap = self
            .inner_bootstrap
            .take()
            .ok_or(CallerBootstrapError::InnerBootstrapConsumed)?;
        match bootstrap.into_authenticated(ready) {
            Ok(ready) => Ok(ready),
            Err((bootstrap, error)) => {
                self.inner_bootstrap = Some(bootstrap);
                self.inner_auth_failed = true;
                Err(CallerBootstrapError::Stage(error))
            }
        }
    }

    /// Take only the real original C/I lease state after authenticated actual gate release.
    /// The same existing typed Hello/Ready/Dispatch stages run against this retained chain owner.
    pub(super) fn take_inner_bootstrap(&mut self) -> Result<Bootstrap, CallerBootstrapError> {
        if !matches!(self.phase, CallerPhase::ClaimedBootstrap)
            || self.inner_hello.is_some()
            || self.inner_hello_sent
            || self.inner_auth_failed
        {
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
    /// original I lease close. The SAME original O framer can receive either a strict-EOF
    /// descriptor or a complete clock-only stop. An actual stop returns OwnerStopObserved;
    /// it does not authorize any report or phase. Reading bytes
    /// grants no classification: the final complete metrics frame and all settlement still follow.
    pub(super) fn read_terminal_report(
        &mut self,
        clock: &mut ExecutionClock,
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
        let (header, sender, descriptor) = loop {
            let received = self
                .outer_receive
                .advance_clock_only_decode(
                    &self.outer_control.transport(),
                    super::role_protocol::ReportStartHeader::rights_count,
                    cutoff,
                    super::role_protocol::decode_report_start,
                    super::role_protocol::ReportStartHeader::clock_only,
                )
                .map_err(CallerBootstrapError::Control)?;
            if let Some(received) = received {
                break (
                    received.control,
                    received
                        .credentials
                        .ok_or(CallerBootstrapError::MissingSender)?,
                    received.rights.pop(),
                );
            }
            self.outer_control
                .transport()
                .wait_terminal_readable(cutoff)
                .map_err(CallerBootstrapError::Control)?;
        };
        if sender.pid
            != self
                .outer_pid
                .ok_or(CallerBootstrapError::MissingOuterPin)?
            || sender.uid != self.caller_uid
            || sender.gid != self.caller_gid
        {
            return Err(CallerBootstrapError::TerminalReplyMismatch);
        }
        let (authority, bytes, descriptor) = match header {
            super::role_protocol::ReportStartHeader::Descriptor { authority, bytes } => {
                if authority != self.authority {
                    return Err(CallerBootstrapError::TerminalReplyMismatch);
                }
                (
                    authority,
                    bytes,
                    descriptor.ok_or(CallerBootstrapError::TerminalReplyMismatch)?,
                )
            }
            super::role_protocol::ReportStartHeader::OwnerStop {
                authority,
                peaks,
                stop,
                cause,
            } => {
                if descriptor.is_some() {
                    return Err(CallerBootstrapError::TerminalReplyMismatch);
                }
                self.retain_terminal_stop(
                    clock,
                    authority,
                    peaks,
                    stop,
                    cause,
                    OwnerStopReceiver::Startup,
                )?;
                return Err(CallerBootstrapError::OwnerStopObserved);
            }
        };
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
            .advance_decode(
                &self.outer_control.transport(),
                cutoff,
                super::role_protocol::decode_terminal_commit,
            )
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
            disposition,
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
            || peaks.charged_bytes < peaks.tree_rss_bytes
        {
            return Err(CallerBootstrapError::TerminalReplyMismatch);
        }
        match disposition {
            super::role_protocol::TerminalDisposition::OwnerStop { cause } => {
                self.retain_terminal_stop(
                    clock,
                    authority,
                    peaks,
                    stop,
                    cause,
                    OwnerStopReceiver::Terminal,
                )?;
                return Ok(true);
            }
            super::role_protocol::TerminalDisposition::SetupRefused { .. } => {
                return Err(CallerBootstrapError::TerminalReplyMismatch)
            }
            super::role_protocol::TerminalDisposition::Report => {}
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

    /// Retain an authenticated candidate only. A report FD/read may already exist: its bytes
    /// remain provisional and the caller must discard them after this actual stop transaction.
    fn retain_terminal_stop(
        &mut self,
        clock: &mut ExecutionClock,
        authority: RunAuthority,
        peaks: MeasuredPeaks,
        stop: super::role_deadline::StopStamp,
        cause: OwnerStopCause,
        receiver: OwnerStopReceiver,
    ) -> Result<(), CallerBootstrapError> {
        if authority != self.authority
            || self.pending_owner_stop.is_some()
            || peaks.charged_bytes < peaks.tree_rss_bytes
            || (matches!(cause, OwnerStopCause::ResourceExhausted)
                && peaks.charged_bytes <= self.memory_bytes.get())
        {
            return Err(CallerBootstrapError::TerminalReplyMismatch);
        }
        let cutoff = clock
            .adopt_stop(stop, self.identity_clock)
            .map_err(CallerBootstrapError::Deadline)?;
        if Instant::now() >= cutoff {
            return Err(CallerBootstrapError::Deadline(DeadlineError::Expired));
        }
        creator::require_live(&self.launcher_identity()?.launcher_pin)
            .map_err(CallerBootstrapError::Io)?;
        self.pending_owner_stop = Some(PendingOwnerStop {
            cause,
            peaks,
            receiver,
        });
        self.terminal_phase = CallerTerminalPhase::Refused;
        Ok(())
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

    /// Private production selection only; no report or evidence is authorized by this fact.
    pub(super) fn owner_stop_pending(&self) -> bool {
        self.pending_owner_stop.is_some()
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

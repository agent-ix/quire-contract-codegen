//! Authenticated L bootstrap before namespace setup or any outer/inner child creation.
//!
//! Frame decoding alone grants no role authority. The actual original C process capability must
//! match the exclusive kernel peer; its nonleader creator capability must belong to that process.
//! Separate I and O endpoints remain separate owned objects, never original lease-writer copies.

use std::{fs::File, io, os::fd::OwnedFd, time::Instant};

use super::{
    control::{
        ControlError, GuardianEndpoint, IncrementalReceive, PreparedReceive, RoleEndpoint,
        RoleEntry,
    },
    creator,
    memory::LauncherMemory,
    outer_setup::{self, LauncherNamespace, NamespaceIdentity, PreparedOuter, SetupError},
    protocol::{BuildIdentity, GuardianRefusal},
    role_deadline::DeadlineError,
    role_protocol::{LauncherReply, RunSettings},
    spawner::SPAWNER_STACK_BYTES,
};

/// Actual resources surviving authenticated L setup. This state creates no child itself.
pub(super) struct PreparedLauncher {
    pub(super) settings: RunSettings,
    pub(super) namespace: LauncherNamespace,
    pub(super) bootstrap: RoleEndpoint,
    pub(super) caller_pin: OwnedFd,
    pub(super) creator_pin: OwnedFd,
    pub(super) inner_endpoint: Option<GuardianEndpoint>,
    pub(super) outer_endpoint: Option<RoleEndpoint>,
    pub(super) control_receive: IncrementalReceive,
    pub(super) decode_scratch: super::guardian_decode::Scratch,
}

#[derive(Debug)]
pub(super) enum BootstrapError {
    Control(ControlError),
    Report(super::report_storage::ReportError),
    Creator(io::Error),
    Setup(SetupError),
    Deadline(DeadlineError),
    BuildIdentityMismatch,
    HelperNotAbsolute,
    OriginalIdentityMismatch,
    CallerStackNotCharged,
    ChargeUnrepresentable,
    UnexpectedControl,
    ReplayedAuthority,
    LauncherObservation(io::Error),
    LauncherObservationConsumed,
    InnerIdentityMismatch,
    OuterNamespaceMismatch,
    WriterMappingMismatch,
    InheritedParentDeath,
}

impl std::fmt::Display for BootstrapError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "role bootstrap refused: {self:?}")
    }
}

impl std::error::Error for BootstrapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Control(error) => Some(error),
            Self::Report(error) => Some(error),
            Self::Creator(error) | Self::LauncherObservation(error) => Some(error),
            Self::Setup(error) => Some(error),
            Self::Deadline(error) => Some(error),
            Self::BuildIdentityMismatch
            | Self::HelperNotAbsolute
            | Self::OriginalIdentityMismatch
            | Self::CallerStackNotCharged
            | Self::ChargeUnrepresentable
            | Self::UnexpectedControl
            | Self::ReplayedAuthority
            | Self::LauncherObservationConsumed
            | Self::InnerIdentityMismatch
            | Self::OuterNamespaceMismatch
            | Self::WriterMappingMismatch
            | Self::InheritedParentDeath => None,
        }
    }
}

/// Called only in the package's single-thread L entry, with a finite initial bootstrap deadline.
/// The original transferred deadline replaces no time: every subsequent setup check uses it.
pub(super) fn prepare_launcher(
    identity: BuildIdentity,
    initial_deadline: Instant,
) -> Result<PreparedLauncher, BootstrapError> {
    let entry = RoleEntry::from_entry_stdin().map_err(BootstrapError::Control)?;
    let mut frame = PreparedReceive::prepare().map_err(BootstrapError::Control)?;
    let mut decode_scratch = super::guardian_decode::Scratch::default();
    let received = entry
        .receive_prepared_decode(
            &mut frame,
            super::bootstrap_control_decode::LauncherInput::rights_count,
            initial_deadline,
            |payload| {
                super::bootstrap_control_decode::launcher(payload, &mut decode_scratch)
                    .map_err(ControlError::InvalidGrammar)
            },
        )
        .map_err(BootstrapError::Control)?;
    let super::bootstrap_control_decode::LauncherInput::Start { settings } = received.control
    else {
        return Err(BootstrapError::UnexpectedControl);
    };
    // Original rights stay owned by the original frame until authentication and fallible
    // settings construction complete. No owned helper pathname is made from an unbound peer.
    let caller_pin = received
        .rights
        .get(1)
        .ok_or(BootstrapError::UnexpectedControl)?;
    let bootstrap = entry
        .authenticate(caller_pin)
        .map_err(BootstrapError::Control)?;
    let original_deadline = super::role_protocol::startup_deadline_from_parts(
        settings.deadline,
        settings.started,
        settings.settlement_reserve,
        settings.work_deadline,
        settings.setup_deadline,
    )
    .map_err(BootstrapError::Deadline)?
    .min(initial_deadline);
    if settings.identity != identity {
        // This authenticated C→L pair exists before unshare or any child. Preserve typed stale-
        // artifact refusal rather than timing out the unrelated C→O channel when no O exists.
        bootstrap
            .transport()
            .send(
                &LauncherReply::Refused {
                    identity,
                    authority: settings.authority,
                    reason: GuardianRefusal::BuildIdentityMismatch,
                },
                &[],
                original_deadline,
            )
            .map_err(BootstrapError::Control)?;
        // Let C consume the refusal before our own EOF makes buffered bytes ineligible. No
        // authorization is accepted here; the caller closes this bootstrap during settlement.
        while Instant::now() < original_deadline {
            match bootstrap
                .transport()
                .pending_control(std::time::Duration::from_millis(20))
            {
                Ok(false) => {}
                Ok(true) => return Err(BootstrapError::UnexpectedControl),
                Err(ControlError::Eof) => break,
                Err(error) => return Err(BootstrapError::Control(error)),
            }
        }
        return Err(BootstrapError::BuildIdentityMismatch);
    }
    let creator_pin = received
        .rights
        .first()
        .ok_or(BootstrapError::UnexpectedControl)?;
    creator::validate_parent_thread(creator_pin, caller_pin).map_err(BootstrapError::Creator)?;
    bootstrap
        .transport()
        .refuse_observable_eof()
        .map_err(BootstrapError::Control)?;
    let settings =
        super::settings_materialize::materialize(settings).map_err(materialization_error)?;
    if Instant::now() >= original_deadline {
        return Err(BootstrapError::Deadline(DeadlineError::Expired));
    }
    creator::validate_parent_thread(creator_pin, caller_pin).map_err(BootstrapError::Creator)?;
    bootstrap
        .transport()
        .refuse_observable_eof()
        .map_err(BootstrapError::Control)?;
    // Reverse wire-order pops preserve the original ancillary buffer's allocation for Settle/Retire.
    let outer = received
        .rights
        .pop()
        .ok_or(BootstrapError::UnexpectedControl)?;
    let inner = received
        .rights
        .pop()
        .ok_or(BootstrapError::UnexpectedControl)?;
    let caller_pin = received
        .rights
        .pop()
        .ok_or(BootstrapError::UnexpectedControl)?;
    let creator_pin = received
        .rights
        .pop()
        .ok_or(BootstrapError::UnexpectedControl)?;
    if !settings.helper.is_absolute() {
        return Err(BootstrapError::HelperNotAbsolute);
    }
    let original = bootstrap
        .transport()
        .creator_credentials()
        .map_err(BootstrapError::Control)?;
    if original.uid != settings.caller_uid || original.gid != settings.caller_gid {
        return Err(BootstrapError::OriginalIdentityMismatch);
    }
    let minimum =
        u64::try_from(SPAWNER_STACK_BYTES).map_err(|_| BootstrapError::ChargeUnrepresentable)?;
    if settings.caller_run_buffers < minimum {
        return Err(BootstrapError::CallerStackNotCharged);
    }
    settings
        .startup_deadline()
        .map_err(BootstrapError::Deadline)?;
    creator::validate_parent_thread(&creator_pin, &caller_pin).map_err(BootstrapError::Creator)?;
    let inner_endpoint =
        GuardianEndpoint::from_received(inner, &caller_pin).map_err(BootstrapError::Control)?;
    let outer_endpoint =
        RoleEndpoint::from_received(outer, &caller_pin).map_err(BootstrapError::Control)?;
    let namespace = outer_setup::prepare_launcher(
        &creator_pin,
        &bootstrap,
        settings.caller_uid,
        settings.caller_gid,
    )
    .map_err(BootstrapError::Setup)?;
    settings
        .startup_deadline()
        .map_err(BootstrapError::Deadline)?;
    bootstrap
        .transport()
        .send(
            &LauncherReply::Ready {
                identity,
                authority: settings.authority,
            },
            &[],
            original_deadline,
        )
        .map_err(BootstrapError::Control)?;
    Ok(PreparedLauncher {
        settings,
        namespace,
        bootstrap,
        caller_pin,
        creator_pin,
        inner_endpoint: Some(inner_endpoint),
        outer_endpoint: Some(outer_endpoint),
        control_receive: IncrementalReceive::from_prepared(frame),
        decode_scratch,
    })
}

/// Owned authenticated O entry data. Keeping the L bootstrap separate preserves its EOF meaning;
/// the independent C final-control endpoint remains available after original I lease closure.
pub(super) struct OuterInput {
    pub(super) settings: RunSettings,
    pub(super) caller_pin: OwnedFd,
    pub(super) caller_control: RoleEndpoint,
    pub(super) inner_endpoint: GuardianEndpoint,
    launcher_pin: OwnedFd,
    bootstrap: RoleEndpoint,
    original_mount: NamespaceIdentity,
    original_pid: NamespaceIdentity,
    original_network: NamespaceIdentity,
    launcher_observation: Option<(File, File)>,
    negative_storage: super::control::FrameStorage,
}

impl OuterInput {
    /// Receive only the two allocated L-origin frames. No M or writer-bearing child exists.
    pub(super) fn receive(
        identity: BuildIdentity,
        initial_deadline: Instant,
    ) -> Result<Self, BootstrapError> {
        let entry = RoleEntry::from_entry_stdin().map_err(BootstrapError::Control)?;
        let mut frame = PreparedReceive::prepare().map_err(BootstrapError::Control)?;
        let mut scratch = super::guardian_decode::Scratch::default();
        let received = entry
            .receive_prepared_decode(
                &mut frame,
                super::bootstrap_control_decode::OuterInput::rights_count,
                initial_deadline,
                |payload| {
                    super::bootstrap_control_decode::outer(payload, &mut scratch)
                        .map_err(ControlError::InvalidGrammar)
                },
            )
            .map_err(BootstrapError::Control)?;
        let super::bootstrap_control_decode::OuterInput::Start {
            settings,
            original_mount,
            original_pid,
            original_network,
        } = received.control
        else {
            return Err(BootstrapError::UnexpectedControl);
        };
        let launcher_pin = received
            .rights
            .first()
            .ok_or(BootstrapError::UnexpectedControl)?;
        let bootstrap = entry
            .authenticate(launcher_pin)
            .map_err(BootstrapError::Control)?;
        if settings.identity != identity {
            return Err(BootstrapError::BuildIdentityMismatch);
        }
        let original = super::role_protocol::startup_deadline_from_parts(
            settings.deadline,
            settings.started,
            settings.settlement_reserve,
            settings.work_deadline,
            settings.setup_deadline,
        )
        .map_err(BootstrapError::Deadline)?
        .min(initial_deadline);
        let caller_pin = received
            .rights
            .get(1)
            .ok_or(BootstrapError::UnexpectedControl)?;
        creator::require_live(launcher_pin).map_err(BootstrapError::Creator)?;
        creator::require_live(caller_pin).map_err(BootstrapError::Creator)?;
        bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(BootstrapError::Control)?;
        let settings =
            super::settings_materialize::materialize(settings).map_err(materialization_error)?;
        if Instant::now() >= original {
            return Err(BootstrapError::Deadline(DeadlineError::Expired));
        }
        creator::require_live(launcher_pin).map_err(BootstrapError::Creator)?;
        creator::require_live(caller_pin).map_err(BootstrapError::Creator)?;
        bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(BootstrapError::Control)?;
        let inner = received
            .rights
            .pop()
            .ok_or(BootstrapError::UnexpectedControl)?;
        let control = received
            .rights
            .pop()
            .ok_or(BootstrapError::UnexpectedControl)?;
        let caller_pin = received
            .rights
            .pop()
            .ok_or(BootstrapError::UnexpectedControl)?;
        let launcher_pin = received
            .rights
            .pop()
            .ok_or(BootstrapError::UnexpectedControl)?;
        if !settings.helper.is_absolute() {
            return Err(BootstrapError::HelperNotAbsolute);
        }
        creator::require_live(&launcher_pin).map_err(BootstrapError::Creator)?;
        creator::require_live(&caller_pin).map_err(BootstrapError::Creator)?;
        let caller_control =
            RoleEndpoint::from_received(control, &caller_pin).map_err(BootstrapError::Control)?;
        let inner_endpoint =
            GuardianEndpoint::from_received(inner, &caller_pin).map_err(BootstrapError::Control)?;
        let original = settings
            .startup_deadline()
            .map_err(BootstrapError::Deadline)?;
        let observation = bootstrap
            .transport()
            .receive_prepared_decode(
                &mut frame,
                super::bootstrap_control_decode::OuterInput::rights_count,
                original.min(initial_deadline),
                |payload| {
                    super::bootstrap_control_decode::outer(payload, &mut scratch)
                        .map_err(ControlError::InvalidGrammar)
                },
            )
            .map_err(BootstrapError::Control)?;
        let super::bootstrap_control_decode::OuterInput::LauncherObservation { authority } =
            observation.control
        else {
            return Err(BootstrapError::UnexpectedControl);
        };
        if authority != settings.authority {
            return Err(BootstrapError::ReplayedAuthority);
        }
        let status = observation
            .rights
            .pop()
            .ok_or(BootstrapError::UnexpectedControl)?;
        let stat = observation
            .rights
            .pop()
            .ok_or(BootstrapError::UnexpectedControl)?;
        for descriptor in [&stat, &status] {
            if rustix::fs::fstatfs(descriptor)
                .map_err(|error| BootstrapError::LauncherObservation(error.into()))?
                .f_type
                != rustix::fs::PROC_SUPER_MAGIC
                || rustix::fs::FileType::from_raw_mode(
                    rustix::fs::fstat(descriptor)
                        .map_err(|error| BootstrapError::LauncherObservation(error.into()))?
                        .st_mode,
                ) != rustix::fs::FileType::RegularFile
                || rustix::fs::fcntl_getfl(descriptor)
                    .map_err(|error| BootstrapError::LauncherObservation(error.into()))?
                    & rustix::fs::OFlags::ACCMODE
                    != rustix::fs::OFlags::RDONLY
            {
                return Err(BootstrapError::LauncherObservation(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "launcher observations are not read-only proc files",
                )));
            }
        }
        bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(BootstrapError::Control)?;
        creator::require_live(&launcher_pin).map_err(BootstrapError::Creator)?;
        // Both authenticated original frames are consumed, their facts materialized and all
        // actual rights transferred. Preserve this real allocation for later negative output.
        let negative_storage = frame
            .into_frame_storage()
            .map_err(|_original| BootstrapError::UnexpectedControl)?;
        Ok(Self {
            settings,
            caller_pin,
            caller_control,
            inner_endpoint,
            launcher_pin,
            bootstrap,
            original_mount,
            original_pid,
            original_network,
            launcher_observation: Some((File::from(stat), File::from(status))),
            negative_storage,
        })
    }

    /// Split owned transports from the separately borrowed setup context. The actual guard
    /// borrows only that context, so moving I's endpoint into its intended child needs neither
    /// self-referential storage nor an extra endpoint alias. Original L proc descriptions are
    /// consumed before O replaces its private proc; missing data cannot become zero RSS.
    pub(super) fn into_parts(mut self) -> Result<OuterParts, BootstrapError> {
        let deadline = self
            .settings
            .startup_deadline()
            .map_err(BootstrapError::Deadline)?;
        let (stat, status) = self
            .launcher_observation
            .take()
            .ok_or(BootstrapError::LauncherObservationConsumed)?;
        let pin = self
            .launcher_pin
            .try_clone()
            .map_err(BootstrapError::Creator)?;
        let launcher_memory =
            LauncherMemory::bind(pin, stat, status).map_err(BootstrapError::LauncherObservation)?;
        Ok(OuterParts {
            settings: self.settings,
            caller_pin: self.caller_pin,
            caller_control: self.caller_control,
            inner_endpoint: self.inner_endpoint,
            launcher_memory,
            negative_storage: self.negative_storage,
            setup: OuterSetup {
                launcher_pin: self.launcher_pin,
                bootstrap: self.bootstrap,
                original_mount: self.original_mount,
                original_pid: self.original_pid,
                original_network: self.original_network,
                deadline,
            },
        })
    }
}

pub(super) struct OuterParts {
    pub(super) settings: RunSettings,
    pub(super) caller_pin: OwnedFd,
    pub(super) caller_control: RoleEndpoint,
    pub(super) inner_endpoint: GuardianEndpoint,
    pub(super) launcher_memory: LauncherMemory,
    pub(super) negative_storage: super::control::FrameStorage,
    pub(super) setup: OuterSetup,
}

/// This exclusive bootstrap and actual parent pin outlive every guard used to spawn M.
pub(super) struct OuterSetup {
    launcher_pin: OwnedFd,
    bootstrap: RoleEndpoint,
    original_mount: NamespaceIdentity,
    original_pid: NamespaceIdentity,
    original_network: NamespaceIdentity,
    deadline: Instant,
}

impl OuterSetup {
    pub(super) fn prepare(&self) -> Result<PreparedOuter<'_>, BootstrapError> {
        self.require_deadline()?;
        let pin = self
            .launcher_pin
            .try_clone()
            .map_err(BootstrapError::Creator)?;
        let prepared = outer_setup::prepare_outer(
            pin,
            &self.bootstrap,
            self.original_mount,
            self.original_pid,
            self.original_network,
        )
        .map_err(BootstrapError::Setup)?;
        self.require_deadline()?;
        Ok(prepared)
    }

    pub(super) fn deadline(&self) -> Instant {
        self.deadline
    }

    fn require_deadline(&self) -> Result<(), BootstrapError> {
        if Instant::now() >= self.deadline {
            Err(BootstrapError::Deadline(DeadlineError::Expired))
        } else {
            Ok(())
        }
    }
}

/// I's authenticated O-origin bootstrap, separate from its original exclusive C lease.
/// This state grants no backend Dispatch; that remains the unchanged typed C/I protocol.
pub(super) struct InnerInput {
    /// Reuse the original bootstrap parser workspace through actual O acknowledgement.
    pub(super) decode_scratch: super::guardian_decode::Scratch,
    pub(super) settings: RunSettings,
    pub(super) outer_pin: OwnedFd,
    pub(super) caller_pin: OwnedFd,
    pub(super) outer_bootstrap: RoleEndpoint,
    pub(super) caller_lease: GuardianEndpoint,
    pub(super) writer: File,
    /// Original O identity retained after safe I acquisition, never resampled from a new writer.
    pub(super) report: super::report_storage::PipeIdentity,
}

impl InnerInput {
    /// Runs only at actual single-thread inner PID1 entry before creating any backend.
    /// O's original pipe identity is authenticated before acquiring/closing the exact slot.
    pub(super) fn receive(
        identity: BuildIdentity,
        initial_deadline: Instant,
    ) -> Result<Self, BootstrapError> {
        outer_setup::require_single_thread().map_err(BootstrapError::Setup)?;
        if rustix::process::getpid().as_raw_nonzero().get() != 1
            || rustix::process::getuid().as_raw() != 0
            || rustix::process::getgid().as_raw() != 0
        {
            return Err(BootstrapError::InnerIdentityMismatch);
        }
        let entry = RoleEntry::from_entry_stdin().map_err(BootstrapError::Control)?;
        let mut frame = PreparedReceive::prepare().map_err(BootstrapError::Control)?;
        let mut scratch = super::guardian_decode::Scratch::default();
        let received = entry
            .receive_prepared_decode(
                &mut frame,
                super::bootstrap_control_decode::InnerInput::rights_count,
                initial_deadline,
                |payload| {
                    super::bootstrap_control_decode::inner(payload, &mut scratch)
                        .map_err(ControlError::InvalidGrammar)
                },
            )
            .map_err(BootstrapError::Control)?;
        let super::bootstrap_control_decode::InnerInput::Start {
            settings,
            outer_namespace,
            report,
            report_slot,
        } = received.control;
        let outer_pin = received
            .rights
            .first()
            .ok_or(BootstrapError::UnexpectedControl)?;
        let outer_bootstrap = entry
            .authenticate(outer_pin)
            .map_err(BootstrapError::Control)?;
        if settings.identity != identity {
            return Err(BootstrapError::BuildIdentityMismatch);
        }
        if report_slot != super::report_storage::REPORT_SLOT {
            return Err(BootstrapError::WriterMappingMismatch);
        }
        let original = super::role_protocol::startup_deadline_from_parts(
            settings.deadline,
            settings.started,
            settings.settlement_reserve,
            settings.work_deadline,
            settings.setup_deadline,
        )
        .map_err(BootstrapError::Deadline)?
        .min(initial_deadline);
        let caller_pin = received
            .rights
            .get(1)
            .ok_or(BootstrapError::UnexpectedControl)?;
        creator::require_live(outer_pin).map_err(BootstrapError::Creator)?;
        creator::require_live(caller_pin).map_err(BootstrapError::Creator)?;
        let creator = outer_bootstrap
            .transport()
            .creator_credentials()
            .map_err(BootstrapError::Control)?;
        if creator.uid != 0 || creator.gid != 0 {
            return Err(BootstrapError::OriginalIdentityMismatch);
        }
        if NamespaceIdentity::read("/proc/self/ns/pid").map_err(BootstrapError::Creator)?
            == outer_namespace
        {
            return Err(BootstrapError::OuterNamespaceMismatch);
        }
        outer_bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(BootstrapError::Control)?;
        let settings =
            super::settings_materialize::materialize(settings).map_err(materialization_error)?;
        if Instant::now() >= original {
            return Err(BootstrapError::Deadline(DeadlineError::Expired));
        }
        creator::require_live(outer_pin).map_err(BootstrapError::Creator)?;
        creator::require_live(caller_pin).map_err(BootstrapError::Creator)?;
        outer_bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(BootstrapError::Control)?;
        let lease = received
            .rights
            .pop()
            .ok_or(BootstrapError::UnexpectedControl)?;
        let caller_pin = received
            .rights
            .pop()
            .ok_or(BootstrapError::UnexpectedControl)?;
        let outer_pin = received
            .rights
            .pop()
            .ok_or(BootstrapError::UnexpectedControl)?;
        if !settings.helper.is_absolute() {
            return Err(BootstrapError::HelperNotAbsolute);
        }
        settings
            .setup_deadline
            .local()
            .map_err(BootstrapError::Deadline)?;
        settings
            .startup_deadline()
            .map_err(BootstrapError::Deadline)?;
        creator::require_live(&outer_pin).map_err(BootstrapError::Creator)?;
        creator::require_live(&caller_pin).map_err(BootstrapError::Creator)?;
        let creator = outer_bootstrap
            .transport()
            .creator_credentials()
            .map_err(BootstrapError::Control)?;
        if creator.uid != 0 || creator.gid != 0 {
            return Err(BootstrapError::OriginalIdentityMismatch);
        }
        if NamespaceIdentity::read("/proc/self/ns/pid").map_err(BootstrapError::Creator)?
            == outer_namespace
        {
            return Err(BootstrapError::OuterNamespaceMismatch);
        }
        let caller_lease =
            GuardianEndpoint::from_received(lease, &caller_pin).map_err(BootstrapError::Control)?;
        outer_bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(BootstrapError::Control)?;
        // O's positively owned namespace contains I independently of M. Fatal inherited inner
        // PDEATH cannot replace the live-C lease EOF oracle or abort its pre-escalation facts.
        rustix::process::set_parent_process_death_signal(None)
            .map_err(|error| BootstrapError::Creator(error.into()))?;
        if rustix::process::parent_process_death_signal()
            .map_err(|error| BootstrapError::Creator(error.into()))?
            .is_some()
        {
            return Err(BootstrapError::InheritedParentDeath);
        }
        creator::require_live(&outer_pin).map_err(BootstrapError::Creator)?;
        let writer =
            super::report_storage::acquire_inner_writer(&report).map_err(BootstrapError::Report)?;
        settings
            .startup_deadline()
            .map_err(BootstrapError::Deadline)?;
        creator::require_live(&outer_pin).map_err(BootstrapError::Creator)?;
        creator::require_live(&caller_pin).map_err(BootstrapError::Creator)?;
        outer_bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(BootstrapError::Control)?;
        Ok(Self {
            decode_scratch: scratch,
            settings,
            outer_pin,
            caller_pin,
            outer_bootstrap,
            caller_lease,
            writer,
            report,
        })
    }
}

fn materialization_error(error: super::recipe_decode::MaterializationError) -> BootstrapError {
    match error {
        super::recipe_decode::MaterializationError::Grammar(error) => {
            BootstrapError::Control(ControlError::InvalidGrammar(error))
        }
        super::recipe_decode::MaterializationError::Allocation(error) => {
            BootstrapError::Creator(io::Error::other(error))
        }
    }
}

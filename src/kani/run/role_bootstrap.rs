//! Authenticated L bootstrap before namespace setup or any outer/inner child creation.
//!
//! Frame decoding alone grants no role authority. The actual original C process capability must
//! match the exclusive kernel peer; its nonleader creator capability must belong to that process.
//! Separate I and O endpoints remain separate owned objects, never original lease-writer copies.

use std::{fs::File, io, os::fd::OwnedFd, time::Instant};

use super::{
    control::{ControlError, GuardianEndpoint, RoleEndpoint, RoleEntry},
    creator,
    memory::LauncherMemory,
    outer_setup::{self, LauncherNamespace, NamespaceIdentity, PreparedOuter, SetupError},
    protocol::{BuildIdentity, GuardianRefusal},
    role_deadline::DeadlineError,
    role_protocol::{InnerBootstrap, LauncherControl, LauncherReply, OuterBootstrap, RunSettings},
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
    let received = entry
        .receive::<LauncherControl>(LauncherControl::rights_count, initial_deadline)
        .map_err(BootstrapError::Control)?;
    let LauncherControl::Start { settings } = received.control;
    let [creator_pin, caller_pin, inner, outer]: [OwnedFd; 4] =
        received.rights.try_into().map_err(|rights: Vec<OwnedFd>| {
            BootstrapError::Control(ControlError::RightsCount {
                expected: 4,
                received: rights.len(),
            })
        })?;
    let bootstrap = entry
        .authenticate(&caller_pin)
        .map_err(BootstrapError::Control)?;
    let original_deadline = settings
        .deadline
        .local()
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
        .deadline
        .local()
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
        .deadline
        .local()
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
    launcher_observation: Option<(File, File)>,
}

impl OuterInput {
    /// Receive only the two allocated L-origin frames. No M or writer-bearing child exists.
    pub(super) fn receive(
        identity: BuildIdentity,
        initial_deadline: Instant,
    ) -> Result<Self, BootstrapError> {
        let entry = RoleEntry::from_entry_stdin().map_err(BootstrapError::Control)?;
        let received = entry
            .receive::<OuterBootstrap>(OuterBootstrap::rights_count, initial_deadline)
            .map_err(BootstrapError::Control)?;
        let OuterBootstrap::Start {
            settings,
            original_mount,
            original_pid,
        } = received.control
        else {
            return Err(BootstrapError::UnexpectedControl);
        };
        let [launcher_pin, caller_pin, control, inner]: [OwnedFd; 4] =
            received.rights.try_into().map_err(|rights: Vec<OwnedFd>| {
                BootstrapError::Control(ControlError::RightsCount {
                    expected: 4,
                    received: rights.len(),
                })
            })?;
        let bootstrap = entry
            .authenticate(&launcher_pin)
            .map_err(BootstrapError::Control)?;
        if settings.identity != identity {
            return Err(BootstrapError::BuildIdentityMismatch);
        }
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
            .deadline
            .local()
            .map_err(BootstrapError::Deadline)?;
        let observation = bootstrap
            .transport()
            .receive::<OuterBootstrap>(OuterBootstrap::rights_count, original.min(initial_deadline))
            .map_err(BootstrapError::Control)?;
        let OuterBootstrap::LauncherObservation { authority } = observation.control else {
            return Err(BootstrapError::UnexpectedControl);
        };
        if authority != settings.authority {
            return Err(BootstrapError::ReplayedAuthority);
        }
        let [stat, status]: [OwnedFd; 2] =
            observation
                .rights
                .try_into()
                .map_err(|rights: Vec<OwnedFd>| {
                    BootstrapError::Control(ControlError::RightsCount {
                        expected: 2,
                        received: rights.len(),
                    })
                })?;
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
        Ok(Self {
            settings,
            caller_pin,
            caller_control,
            inner_endpoint,
            launcher_pin,
            bootstrap,
            original_mount,
            original_pid,
            launcher_observation: Some((File::from(stat), File::from(status))),
        })
    }

    /// Split owned transports from the separately borrowed setup context. The actual guard
    /// borrows only that context, so moving I's endpoint into its intended child needs neither
    /// self-referential storage nor an extra endpoint alias. Original L proc descriptions are
    /// consumed before O replaces its private proc; missing data cannot become zero RSS.
    pub(super) fn into_parts(mut self) -> Result<OuterParts, BootstrapError> {
        let deadline = self
            .settings
            .deadline
            .local()
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
            setup: OuterSetup {
                launcher_pin: self.launcher_pin,
                bootstrap: self.bootstrap,
                original_mount: self.original_mount,
                original_pid: self.original_pid,
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
    pub(super) setup: OuterSetup,
}

/// This exclusive bootstrap and actual parent pin outlive every guard used to spawn M.
pub(super) struct OuterSetup {
    launcher_pin: OwnedFd,
    bootstrap: RoleEndpoint,
    original_mount: NamespaceIdentity,
    original_pid: NamespaceIdentity,
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
    pub(super) settings: RunSettings,
    pub(super) outer_pin: OwnedFd,
    pub(super) caller_pin: OwnedFd,
    pub(super) outer_bootstrap: RoleEndpoint,
    pub(super) caller_lease: GuardianEndpoint,
    pub(super) writer: File,
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
        let received = entry
            .receive::<InnerBootstrap>(InnerBootstrap::rights_count, initial_deadline)
            .map_err(BootstrapError::Control)?;
        if !received.control.expected_report_mapping() {
            return Err(BootstrapError::WriterMappingMismatch);
        }
        let InnerBootstrap::Start {
            settings,
            outer_namespace,
            report,
            report_slot: _,
        } = received.control;
        let [outer_pin, caller_pin, lease]: [OwnedFd; 3] =
            received.rights.try_into().map_err(|rights: Vec<OwnedFd>| {
                BootstrapError::Control(ControlError::RightsCount {
                    expected: 3,
                    received: rights.len(),
                })
            })?;
        let outer_bootstrap = entry
            .authenticate(&outer_pin)
            .map_err(BootstrapError::Control)?;
        if settings.identity != identity {
            return Err(BootstrapError::BuildIdentityMismatch);
        }
        if !settings.helper.is_absolute() {
            return Err(BootstrapError::HelperNotAbsolute);
        }
        settings
            .deadline
            .local()
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
            .deadline
            .local()
            .map_err(BootstrapError::Deadline)?;
        creator::require_live(&outer_pin).map_err(BootstrapError::Creator)?;
        creator::require_live(&caller_pin).map_err(BootstrapError::Creator)?;
        outer_bootstrap
            .transport()
            .refuse_observable_eof()
            .map_err(BootstrapError::Control)?;
        Ok(Self {
            settings,
            outer_pin,
            caller_pin,
            outer_bootstrap,
            caller_lease,
            writer,
        })
    }
}

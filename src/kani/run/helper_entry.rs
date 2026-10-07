//! Execution of the actual matched helper's private L/O/I/backend roles.
//!
//! Each actor owns its real child and controls throughout its run. An abnormal helper return
//! grants C neither role settlement nor evidence; C must independently confirm its retained
//! whole chain. Normal O exit occurs only after the complete authenticated terminal commit.

use std::{
    thread,
    time::{Duration, Instant},
};

use super::{
    backend_installer::{self, InstallerEntryError},
    guardian::{
        GuardianError, InnerAdmissionError, InnerAdmissionProgress, InnerBackendProgress,
        PendingInnerBackend,
    },
    launcher_owner::{self, LauncherError, LauncherOwner},
    outer_sampling::{OuterRunOwner, OuterRunProgress, SamplingError},
    protocol::BuildIdentity,
    role_bootstrap::{self, BootstrapError, InnerInput, OuterInput, OuterParts},
    role_command::HelperRole,
};

// The existing finite helper bootstrap cap, clipped by authenticated original settings before
// setup. It is never a backend work or settlement allowance and is never restarted on retry.
const ENTRY_SETUP_CAP: Duration = Duration::from_secs(3);
const ACTOR_TICK: Duration = Duration::from_millis(20);

#[derive(Debug)]
pub(super) enum HelperEntryError {
    Bootstrap(BootstrapError),
    Launcher(LauncherError),
    Sampling(SamplingError),
    Admission(InnerAdmissionError),
    Guardian(GuardianError),
    Installer(InstallerEntryError),
    ClockUnrepresentable,
}

impl std::fmt::Display for HelperEntryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "matched helper role failed: {self:?}")
    }
}

impl std::error::Error for HelperEntryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Bootstrap(error) => Some(error),
            Self::Launcher(error) => Some(error),
            Self::Sampling(error) => Some(error),
            Self::Admission(error) => Some(error),
            Self::Guardian(error) => Some(error),
            Self::Installer(error) => Some(error),
            Self::ClockUnrepresentable => None,
        }
    }
}

pub(super) fn run(role: HelperRole, identity: BuildIdentity) -> Result<(), HelperEntryError> {
    let initial = Instant::now()
        .checked_add(ENTRY_SETUP_CAP)
        .ok_or(HelperEntryError::ClockUnrepresentable)?;
    match role {
        HelperRole::Launcher => {
            let input = role_bootstrap::prepare_launcher(identity, initial)
                .map_err(HelperEntryError::Bootstrap)?;
            let mut owner = LauncherOwner::prepare(input).map_err(HelperEntryError::Launcher)?;
            owner.spawn().map_err(HelperEntryError::Launcher)?;
            owner.confirm_arm().map_err(HelperEntryError::Launcher)?;
            owner
                .run_until_retired()
                .map_err(HelperEntryError::Launcher)
        }
        HelperRole::Outer => run_outer(identity, initial),
        HelperRole::Inner => run_inner(identity, initial),
        HelperRole::Backend => {
            backend_installer::run_entry(identity, initial).map_err(HelperEntryError::Installer)
        }
    }
}

fn run_outer(identity: BuildIdentity, initial: Instant) -> Result<(), HelperEntryError> {
    let input = OuterInput::receive(identity, initial).map_err(HelperEntryError::Bootstrap)?;
    let OuterParts {
        settings,
        caller_pin,
        caller_control,
        inner_endpoint,
        launcher_memory,
        setup,
    } = input.into_parts().map_err(HelperEntryError::Bootstrap)?;
    let guard = setup.prepare().map_err(HelperEntryError::Bootstrap)?;
    // Actual O has armed its original L parent and prepared private namespaces before either
    // endpoint receives its positively owned capability. No M exists at this boundary.
    launcher_owner::publish_outer_arm(&guard, &settings, &caller_control, setup.deadline())
        .map_err(HelperEntryError::Launcher)?;
    let mut owner = OuterRunOwner::prepare(
        &guard,
        &caller_control,
        &caller_pin,
        inner_endpoint,
        launcher_memory,
        settings,
    )
    .map_err(HelperEntryError::Sampling)?;
    loop {
        match owner
            .tick(&guard, &caller_control)
            .map_err(HelperEntryError::Sampling)?
        {
            OuterRunProgress::TerminalCommitted => return Ok(()),
            OuterRunProgress::Pending
            | OuterRunProgress::Startup(_)
            | OuterRunProgress::BackendDispatched
            | OuterRunProgress::BackendCompleted
            | OuterRunProgress::ResourceExhausted
            | OuterRunProgress::OwnerStopped { .. } => {}
        }
        // The actor performs its due observation checks before each terminal send. This wait
        // schedules the next finite step; it does not suspend accounting inside a blocked send.
        thread::park_timeout(ACTOR_TICK);
    }
}

fn run_inner(identity: BuildIdentity, initial: Instant) -> Result<(), HelperEntryError> {
    let input = InnerInput::receive(identity, initial).map_err(HelperEntryError::Bootstrap)?;
    let mut owner = PendingInnerBackend::prepare(input).map_err(HelperEntryError::Admission)?;
    loop {
        match owner.tick().map_err(HelperEntryError::Admission)? {
            InnerAdmissionProgress::LeaseClosed => return Ok(()),
            InnerAdmissionProgress::Dispatched => break,
            InnerAdmissionProgress::Pending
            | InnerAdmissionProgress::Ready
            | InnerAdmissionProgress::Refused => {}
        }
        thread::park_timeout(ACTOR_TICK);
    }
    // The SAME retained actual installer Child transfers only after real C Dispatch/Exec.
    let mut backend = owner.take_running().map_err(HelperEntryError::Admission)?;
    loop {
        match backend.tick().map_err(HelperEntryError::Guardian)? {
            InnerBackendProgress::LeaseClosed => return Ok(()),
            InnerBackendProgress::Running => {}
        }
        thread::park_timeout(ACTOR_TICK);
    }
}

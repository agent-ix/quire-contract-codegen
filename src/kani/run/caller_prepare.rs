//! Prepare the retained caller run without creating a helper or authorizing a backend.
//!
//! The caller supplies the resolved helper and already captured original stdin. All named
//! controls, captures and recipe storage are reserved by CallerBootstrap before L can exist.

use std::{num::NonZeroUsize, path::PathBuf, sync::Arc, time::Instant};

use crate::kani::identity::ProofCeilings;

use super::{
    caller_bootstrap::{CallerBootstrap, CallerBootstrapError},
    launch::CAPTURE_LIMIT,
    namespace::BackendCommand,
    protocol::current_build_identity,
    publication::Publication,
    role_deadline::{DeadlineError, ExecutionClock, IdentityDeadline, RoleDeadline},
    role_protocol::RunSettings,
    stages::{Bootstrap, StageError},
    stdin::OriginalStdin,
};

/// Preparation failures retain their original typed source for the caller's admission mapping.
#[derive(Debug)]
pub(super) enum PreparationError {
    Stage(StageError),
    Deadline(DeadlineError),
    CallerBootstrap(CallerBootstrapError),
    CaptureSizeOverflow,
    PreRoleExpired,
}

impl std::fmt::Display for PreparationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "caller preparation refused: {self:?}")
    }
}

impl std::error::Error for PreparationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Stage(error) => Some(error),
            Self::Deadline(error) => Some(error),
            Self::CallerBootstrap(error) => Some(error),
            Self::CaptureSizeOverflow | Self::PreRoleExpired => None,
        }
    }
}

/// Reserve the exact supplied recipe and caller buffers without spawning or classifying a run.
/// The activation owner must recheck the original clock before creating L.
pub(super) fn prepare(
    recipe: BackendCommand,
    resolved_helper: PathBuf,
    stdin: OriginalStdin,
    ceilings: ProofCeilings,
    harnesses: NonZeroUsize,
    clock: &ExecutionClock,
    publication: Arc<Publication>,
) -> Result<CallerBootstrap, PreparationError> {
    if clock.work_expired(Instant::now()) {
        return Err(PreparationError::PreRoleExpired);
    }
    let capture_limit = CAPTURE_LIMIT
        .checked_mul(harnesses.get())
        .ok_or(PreparationError::CaptureSizeOverflow)?;
    let (bootstrap, inner_endpoint) =
        Bootstrap::new(clock.work_deadline()).map_err(PreparationError::Stage)?;
    // The anonymous report operand is already part of the supplied recipe. No pathname report
    // or surviving-owner cleanup list is introduced by this preparation boundary.
    let dispatch = bootstrap
        .prepare_dispatch(recipe, &stdin, Vec::new())
        .map_err(PreparationError::Stage)?;
    let settings = RunSettings {
        helper: resolved_helper,
        identity: current_build_identity(),
        authority: bootstrap.authority(),
        deadline: IdentityDeadline::from_original(clock.original_deadline())
            .map_err(PreparationError::Deadline)?,
        started: clock.started(),
        settlement_reserve: clock.reserve(),
        work_deadline: IdentityDeadline::from_original(clock.work_deadline())
            .map_err(PreparationError::Deadline)?,
        setup_deadline: RoleDeadline::from_original(bootstrap.setup_deadline())
            .map_err(PreparationError::Deadline)?,
        caller_uid: rustix::process::getuid().as_raw(),
        caller_gid: rustix::process::getgid().as_raw(),
        memory_bytes: ceilings.memory_bytes,
        // CallerBootstrap overwrites this seed with its checked retained reservations before
        // encoding Start. It is never an observation or a transmitted charge supplied here.
        caller_run_buffers: 0,
    };
    let caller = CallerBootstrap::prepare(
        settings,
        bootstrap,
        inner_endpoint,
        dispatch,
        stdin,
        capture_limit,
        publication,
        clock,
    )
    .map_err(PreparationError::CallerBootstrap)?;
    if clock.work_expired(Instant::now()) {
        return Err(PreparationError::PreRoleExpired);
    }
    Ok(caller)
}

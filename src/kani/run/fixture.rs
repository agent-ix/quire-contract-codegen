//! Opt-in TC-049 observation contract. The harness judges raw facts after owned cleanup.

use super::{launch::GuardianFailureKind, stdin::OriginalStdin};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    io,
    path::{Path, PathBuf},
    time::Instant,
};

/// Exact shared startup prefix for intentional original-caller death.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardianFixturePrefix {
    /// Pair/setup exists; no monitor has been spawned.
    BeforeMonitor,
    /// Monitor exists; INIT is unclaimed and the gate is retained.
    Bootstrap,
    /// Validated INIT and ready observer exist while the gate is retained.
    ClaimedGated,
    /// Gate was released only for trusted bootstrap; Ready is not authenticated.
    ClaimedBootstrap,
    /// Actual matching peer/session/observer are ready; no Dispatch was sent.
    InitReady,
}

/// Deliberate death targets only this fixture's positively verified ownership.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardianFixtureDeath {
    /// Kill this original-caller process.
    Caller,
    /// Kill this caller's dedicated group, whose leader is this caller.
    CallerGroup,
}

/// One operation selects observation or an exact startup prefix; it exports no run handles.
pub enum GuardianFixtureScenario {
    /// Send the sealed prefix witness and one actual owned pin, then self-kill.
    ExactDeath {
        prefix: GuardianFixturePrefix,
        death: GuardianFixtureDeath,
    },
    /// Stop actual Ready INIT, queue normal Dispatch, close the lease and resume on publication.
    PendingDispatch { backend_marker: PathBuf },
    /// Observe closed-lease INIT/escaped-worker termination before independent escalation.
    Dispatched {
        backend_marker: PathBuf,
        worker_acknowledgement: PathBuf,
    },
}

/// Explicit real helper/backend inputs, charged to one original monotonic deadline.
pub struct GuardianFixtureRequest<'a> {
    /// Matched actual package helper; never discovered through PATH.
    pub guardian_path: &'a Path,
    /// Real backend executable and raw argument bytes.
    pub program: &'a std::ffi::OsStr,
    /// Raw backend arguments.
    pub arguments: &'a [OsString],
    /// Actual backend working directory.
    pub directory: &'a Path,
    /// Environment additions applied to the ordinary inherited environment.
    pub environment: &'a [(OsString, OsString)],
    /// Explicit original backend stdin, captured before controls exist.
    pub original_stdin: &'a OriginalStdin,
    /// Assigned report path; it is not created by bootstrap.
    pub report_path: &'a Path,
    /// Original deadline, shared by all coordination and cleanup observations.
    pub deadline: Instant,
    /// Ordinary whole-run resource authority; fixture observation never grants a proof result.
    pub ceilings: crate::kani::identity::ProofCeilings,
    /// Members sharing this one launcher's capture and resource ownership set.
    pub harnesses: std::num::NonZeroUsize,
    /// Typed fixture selection.
    pub scenario: GuardianFixtureScenario,
}

/// Typed fixture-only coordination failure; none establishes passing lifecycle evidence.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardianFixtureFailure {
    /// Reporter flags/type could not be established before spawn.
    Reporter,
    /// The original budget expired.
    Deadline,
    /// Ordinary whole-run memory or observation authority required urgent cancellation.
    Resource,
    /// Owned INIT never reached positive stopped state.
    Stop,
    /// LeaseClosing publication was absent or unavailable.
    Publication,
    /// Owned continuation did not successfully resume INIT.
    Resume,
    /// Positive worker acknowledgement or its owned pin was unavailable.
    Worker,
    /// Fixture caller did not own a dedicated process group.
    CallerGroup,
    /// A required stage or pin observation was unavailable.
    Stage,
    /// Bounded observation/report storage was exceeded.
    Overflow,
}

/// An operation refusal, retaining typed production failure discriminants.
#[derive(Debug)]
pub enum GuardianFixtureError {
    /// This operation requires actual Linux namespaces and pidfds.
    UnsupportedPlatform,
    /// Descriptor/procfs/owned observation failed.
    Io(io::Error),
    /// Fixture coordination failed without claiming successful cancellation.
    Coordination(GuardianFixtureFailure),
    /// Ordinary production stage refused.
    Guardian {
        kind: GuardianFailureKind,
        detail: String,
    },
}

impl std::fmt::Display for GuardianFixtureError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "guardian fixture I/O: {error}"),
            Self::Guardian { kind, detail } => {
                write!(formatter, "guardian fixture {kind:?}: {detail}")
            }
            Self::Coordination(failure) => {
                write!(formatter, "guardian fixture coordination: {failure:?}")
            }
            Self::UnsupportedPlatform => formatter.write_str("guardian fixture requires Linux"),
        }
    }
}
impl std::error::Error for GuardianFixtureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}
impl From<io::Error> for GuardianFixtureError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Raw actual kernel/procfs identity; a reported PID never replaces the transferred pin.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardianFixtureIdentity {
    /// Host-visible process ID at sealing.
    pub pid: u32,
    /// Kernel start tick, excluding reuse.
    pub start: u64,
    /// Actual host-visible parent at sealing.
    pub parent: u32,
    /// Actual namespace link identity.
    pub namespace: String,
    /// Host-visible process group.
    pub group: i32,
    /// Host-visible session.
    pub session: i32,
}

/// Descriptor identity of the actual cloned, validated pin.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardianFixtureDescriptor {
    /// Actual descriptor device.
    pub device: u64,
    /// Actual descriptor inode.
    pub inode: u64,
}

/// The single right's authority, or truthful absence before any spawn.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum GuardianFixturePin {
    /// No monitor or INIT was spawned; no descriptor accompanies this witness.
    NoInit,
    /// INIT remains unclaimed; only the actual unreaped monitor pin accompanies it.
    Monitor {
        identity: GuardianFixtureIdentity,
        descriptor: GuardianFixtureDescriptor,
    },
    /// The descriptor clones the already-validated actual owned INIT pin.
    Init {
        identity: GuardianFixtureIdentity,
        descriptor: GuardianFixtureDescriptor,
    },
}

/// Immutable exact-boundary witness queued before fixture self-death.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardianFixtureDeathWitness {
    /// Actual shared prefix published before reporting.
    pub prefix: GuardianFixturePrefix,
    /// Actual original caller identity, authenticated independently by the harness's Child.
    pub caller: GuardianFixtureIdentity,
    /// Truthful ownership and exact transferred right identity.
    pub pin: GuardianFixturePin,
    /// Actual gate ownership at sealing.
    pub gate_retained: bool,
    /// Actual original reporter stdout CLOEXEC before any spawn.
    pub stdout_cloexec: bool,
    /// Actual non-stdio auxiliary reporter CLOEXEC before any spawn.
    pub auxiliary_cloexec: bool,
    /// Actual reporter socket descriptor identity, independent of its descriptor number.
    pub reporter_descriptor: GuardianFixtureDescriptor,
    /// Reporter presence in the actual owned monitor before intentional death; absent pre-spawn.
    pub monitor_reporter_present: Option<bool>,
    /// Reporter presence in actual claimed gated INIT or execed guardian before intentional death.
    pub init_reporter_present: Option<bool>,
}

/// Typed raw lease observation recorded before independent INIT escalation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum GuardianFixtureLeaseObservation {
    /// Actual owned INIT pidfd signalled termination.
    ConfirmedTermination,
    /// Observation cap ended while actual INIT remained live.
    EscalationRequired,
    /// Kernel observation failed; no termination inferred.
    Unavailable { detail: String },
}

/// Cleanup result is separate from the earlier raw lifecycle facts.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum GuardianFixtureCleanup {
    /// Actual owned namespace cleanup and monitor reaping confirmed.
    Confirmed,
    /// Cleanup or reaping remained unavailable.
    Unconfirmed { detail: String },
}

/// Returned only after immediate unchanged cleanup; contains no cancellation or process handle.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardianFixtureObservation {
    /// Actual unconditional LeaseClosing stage was published before these facts were sealed.
    pub lease_closing_published: bool,
    /// Production close result sealed before escalation.
    pub lease: Option<GuardianFixtureLeaseObservation>,
    /// Completed actual-close event ordinal sealed at publication; never late-filled.
    pub completed_close_ordinal: Option<u64>,
    /// Actual LeaseClosing publication ordinal, if available.
    pub publication_ordinal: Option<u64>,
    /// Actual pinned acknowledged worker termination before escalation.
    pub worker_terminated: Option<bool>,
    /// Pre-close identity of the positively acknowledged owned worker; no process handle escapes.
    pub worker_identity: Option<GuardianFixtureIdentity>,
    /// Actual retained worker pidfd descriptor identity at acknowledgement.
    pub worker_descriptor: Option<GuardianFixtureDescriptor>,
    /// Actual reporter socket presence in the retained monitor before lease closure.
    pub monitor_reporter_present: Option<bool>,
    /// Actual reporter socket presence in claimed INIT before lease closure.
    pub init_reporter_present: Option<bool>,
    /// Actual reporter socket presence in the positively acknowledged worker before closure.
    pub worker_reporter_present: Option<bool>,
    /// Actual backend marker presence before escalation.
    pub backend_marker_present: Option<bool>,
    /// Missing or failed fixture coordination, separately from lifecycle facts.
    pub coordination_failure: Option<GuardianFixtureFailure>,
    /// Independent subsequent cleanup result.
    pub cleanup: GuardianFixtureCleanup,
}

/// Runs TC-049's sole opt-in private-boundary operation using the ordinary shared stages.
///
/// Test-only: callers must deliberately build this library, caller fixture and helper from the
/// same consumer manifest and feature inputs. Production dependency exclusion belongs to IR-649.
/// Intentional-death selections report through startup stdout safely duplicated before spawn;
/// success self-kills and never returns. Live selections return raw facts only after cleanup.
/// No public lease, ownership handle, cancellation entry or cleanup-deferring callback exists.
pub fn observe_guardian_fixture(
    request: GuardianFixtureRequest<'_>,
) -> Result<GuardianFixtureObservation, GuardianFixtureError> {
    #[cfg(target_os = "linux")]
    {
        super::owned::fixture(request)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = request;
        Err(GuardianFixtureError::UnsupportedPlatform)
    }
}

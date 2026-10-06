//! Closed bootstrap/control frames for L/O, separate from I's original exclusive caller lease.
//!
//! Descriptor counts are part of each variant. The role executor authenticates the actual creator
//! pin before trusting any bootstrap data; decoding these frames alone grants no spawning,
//! report-writer or backend authority. Final report delivery does not use the consumed I lease.

use std::{num::NonZeroU64, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::{
    outer_setup::NamespaceIdentity,
    protocol::{BackendExit, BuildIdentity, GuardianRefusal, RunAuthority},
    report_storage::{PipeIdentity, REPORT_SLOT},
    role_deadline::{ExecutionClock, IdentityDeadline, RoleDeadline},
};

/// C-origin settings, forwarded without replacing the original deadline or run authority.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RunSettings {
    /// The original explicitly supplied helper, resolved by C without executable PATH search.
    pub(super) helper: PathBuf,
    pub(super) identity: BuildIdentity,
    pub(super) authority: RunAuthority,
    pub(super) deadline: IdentityDeadline,
    /// Original C-derived cutoff T-R_eff, not a fresh role-local work allowance.
    pub(super) work_deadline: IdentityDeadline,
    /// C’s original finite bootstrap cap, bounded by the original identity deadline.
    pub(super) setup_deadline: RoleDeadline,
    pub(super) caller_uid: u32,
    pub(super) caller_gid: u32,
    pub(super) memory_bytes: NonZeroU64,
    pub(super) caller_run_buffers: u64,
}

impl RunSettings {
    /// Trusted C binds both clocks before serializing its authenticated start frame. Neither L
    /// nor O computes a new reserve from its shorter remaining time.
    pub(super) fn bind_clock(
        &mut self,
        clock: &ExecutionClock,
    ) -> Result<(), super::role_deadline::DeadlineError> {
        self.deadline = IdentityDeadline::from_original(clock.original_deadline())?;
        self.work_deadline = IdentityDeadline::from_original(clock.work_deadline())?;
        Ok(())
    }

    /// None preserves the original never-elapsing admission (including checked_add overflow).
    /// A finite transferred deadline that actually expires remains an error, never None.
    pub(super) fn identity_deadline(
        &self,
    ) -> Result<Option<std::time::Instant>, super::role_deadline::DeadlineError> {
        self.deadline.local()
    }

    pub(super) fn startup_deadline(
        &self,
    ) -> Result<std::time::Instant, super::role_deadline::DeadlineError> {
        let cap = self.setup_deadline.local()?;
        Ok(self
            .work_deadline()?
            .map_or(cap, |deadline| deadline.min(cap)))
    }

    pub(super) fn work_deadline(
        &self,
    ) -> Result<Option<std::time::Instant>, super::role_deadline::DeadlineError> {
        self.work_deadline.local()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum LauncherControl {
    /// Rights: actual creating-thread pin, actual C process pin, I lease endpoint, O endpoint.
    Start { settings: RunSettings },
    /// Same original settlement cutoff; this is not a fresh allowance at L.
    Settle {
        authority: RunAuthority,
        deadline: RoleDeadline,
        mode: LauncherSettlementMode,
    },
    /// C has consumed actual O custody while L remained live on the original endpoint.
    Retire { authority: RunAuthority },
}

impl LauncherControl {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 4,
            Self::Settle { .. } | Self::Retire { .. } => 0,
        }
    }
}

/// Private whole-owner settlement selection, never a public caller cancellation handle.
#[derive(Clone, Copy, Deserialize, Serialize)]
pub(super) enum LauncherSettlementMode {
    ObserveOuterExit,
    CancelOuter,
}

/// Trusted L reports only custody established from its retained actual Child wait result.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum OuterChildSettlement {
    NotCreated,
    Reaped { outcome: BackendExit },
}

/// Authenticated L-origin bootstrap status. Neither status grants backend authorization.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum LauncherReply {
    Ready {
        identity: BuildIdentity,
        authority: RunAuthority,
    },
    OuterSettled {
        identity: BuildIdentity,
        authority: RunAuthority,
        custody: OuterChildSettlement,
    },
    Refused {
        identity: BuildIdentity,
        authority: RunAuthority,
        reason: GuardianRefusal,
    },
}

impl LauncherReply {
    pub(super) fn rights_count(&self) -> usize {
        0
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum OuterBootstrap {
    /// Rights: actual L process pin, actual C process pin, O endpoint, I lease endpoint.
    Start {
        settings: RunSettings,
        original_mount: NamespaceIdentity,
        original_pid: NamespaceIdentity,
        original_network: NamespaceIdentity,
    },
    /// Rights: actual L stat/status files, opened before private proc replacement.
    LauncherObservation { authority: RunAuthority },
}

impl OuterBootstrap {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 4,
            Self::LauncherObservation { .. } => 2,
        }
    }
}

/// Actual O-origin arm publication, received separately by L and C with one actual O pidfd.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum OuterArmReply {
    Armed {
        identity: BuildIdentity,
        authority: RunAuthority,
        namespace: NamespaceIdentity,
        network: NamespaceIdentity,
        mapped_uid: u32,
        mapped_gid: u32,
    },
}

impl OuterArmReply {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Armed { .. } => 1,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum InnerBootstrap {
    /// Rights: actual O process pin, actual C process pin, original I lease endpoint.
    Start {
        settings: RunSettings,
        outer_namespace: NamespaceIdentity,
        report: PipeIdentity,
        report_slot: i32,
    },
}

impl InnerBootstrap {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 3,
        }
    }

    /// Slot equality is necessary, never sufficient: O/run authentication and original pipe
    /// identity verification must precede writer acquisition.
    pub(super) fn expected_report_mapping(&self) -> bool {
        match self {
            Self::Start { report_slot, .. } => *report_slot == REPORT_SLOT,
        }
    }
}

/// Ordinary private C→O startup transitions. Each authorization names the original run;
/// no command changes the original deadline, grants Dispatch or carries fixture observations.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum OuterPhaseCommand {
    BeginMonitor { authority: RunAuthority },
    ClaimInner { authority: RunAuthority },
    ReleaseGate { authority: RunAuthority },
}

impl OuterPhaseCommand {
    pub(super) fn rights_count(&self) -> usize {
        0
    }

    pub(super) fn authority(&self) -> RunAuthority {
        match self {
            Self::BeginMonitor { authority }
            | Self::ClaimInner { authority }
            | Self::ReleaseGate { authority } => *authority,
        }
    }
}

/// O-origin startup replies carry real process capabilities, never numbers standing in for pins.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum OuterPhaseReply {
    MonitorSpawned {
        authority: RunAuthority,
    },
    InnerClaimed {
        authority: RunAuthority,
        start: u64,
        namespace: NamespaceIdentity,
    },
    GateReleased {
        authority: RunAuthority,
    },
}

impl OuterPhaseReply {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::MonitorSpawned { .. } | Self::InnerClaimed { .. } => 1,
            Self::GateReleased { .. } => 0,
        }
    }

    pub(super) fn authority(&self) -> RunAuthority {
        match self {
            Self::MonitorSpawned { authority }
            | Self::InnerClaimed { authority, .. }
            | Self::GateReleased { authority } => *authority,
        }
    }
}

/// O has actually authenticated I's complete backend event. Queueing the I-origin event alone
/// grants no permission to publish C Completed and let original lease close destroy its sender.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum InnerOwnerControl {
    CompletionObserved { authority: RunAuthority },
}

impl InnerOwnerControl {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::CompletionObserved { .. } => 0,
        }
    }
}

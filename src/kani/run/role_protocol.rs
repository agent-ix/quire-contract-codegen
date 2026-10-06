//! Closed bootstrap/control frames for L/O, separate from I's original exclusive caller lease.
//!
//! Descriptor counts are part of each variant. The role executor authenticates the actual creator
//! pin before trusting any bootstrap data; decoding these frames alone grants no spawning,
//! report-writer or backend authority. Final report delivery does not use the consumed I lease.

use std::{num::NonZeroU64, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::{
    outer_setup::NamespaceIdentity,
    protocol::{BuildIdentity, GuardianRefusal, RunAuthority},
    report_storage::{PipeIdentity, REPORT_SLOT},
    role_deadline::RoleDeadline,
};

/// C-origin settings, forwarded without replacing the original deadline or run authority.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RunSettings {
    /// The original explicitly supplied helper, resolved by C without executable PATH search.
    pub(super) helper: PathBuf,
    pub(super) identity: BuildIdentity,
    pub(super) authority: RunAuthority,
    pub(super) deadline: RoleDeadline,
    pub(super) caller_uid: u32,
    pub(super) caller_gid: u32,
    pub(super) memory_bytes: NonZeroU64,
    pub(super) caller_run_buffers: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum LauncherControl {
    /// Rights: actual creating-thread pin, actual C process pin, I lease endpoint, O endpoint.
    Start { settings: RunSettings },
}

impl LauncherControl {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 4,
        }
    }
}

/// Authenticated L-origin bootstrap status. Neither status grants backend authorization.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum LauncherReply {
    Ready {
        identity: BuildIdentity,
        authority: RunAuthority,
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

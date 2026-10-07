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
    resource_ledger::MeasuredPeaks,
    role_deadline::{ExecutionClock, IdentityDeadline, MonotonicInstant, RoleDeadline, StopStamp},
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
    /// Mandatory C kernel-clock lower bound, captured before any creating thread/L starts.
    pub(super) started: MonotonicInstant,
    /// The same original C-derived R_eff, never a receiving role's new budget.
    pub(super) settlement_reserve: std::time::Duration,
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
        self.started = clock.started();
        self.settlement_reserve = clock.reserve();
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
        if self.settlement_reserve > super::role_deadline::SETTLE_RESERVE
            || (matches!(self.deadline, IdentityDeadline::NeverElapses)
                && self.settlement_reserve != super::role_deadline::SETTLE_RESERVE)
        {
            return Err(super::role_deadline::DeadlineError::InvalidClock);
        }
        self.started.require_started()?;
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

/// I's temporary authenticated channel to the same-PID trusted backend installer. The original
/// environment stays metadata until policy admission and C's genuine positive Dispatch.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum BackendInstallerControl {
    /// Rights: actual I process pin, original C process pin, owned original O report writer.
    Start {
        settings: RunSettings,
        report: PipeIdentity,
    },
    /// Sent only after genuine C/I Dispatch. It starts no second backend process or budget.
    Exec {
        authority: RunAuthority,
        command: super::namespace::BackendCommand,
        stdin: super::protocol::StdinControl,
    },
}

impl BackendInstallerControl {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 3,
            Self::Exec { stdin, .. } => stdin.rights_count(),
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
    CompletionObserved {
        authority: RunAuthority,
        stop: StopStamp,
    },
}

impl InnerOwnerControl {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::CompletionObserved { .. } => 0,
        }
    }
}

/// Private terminal branch authority. An owner stop never attests I completion or report bytes.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum TerminalDisposition {
    Report,
    OwnerStop {
        cause: OwnerStopCause,
    },
    /// Genuine pre-Dispatch policy refusal; measurements never become public evidence.
    SetupRefused {
        failure: super::startup_envelope::PolicyFailureCause,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(super) enum OwnerStopCause {
    ResourceExhausted,
    TimedOut,
}

/// O's terminal transaction remains separate from the already consumed I lease. A decoded
/// commit is provisional until actual normal O/L/thread/capture settlement and stream-end checks.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum OuterTerminalReply {
    ReportDescriptor {
        authority: RunAuthority,
        bytes: u64,
    },
    Committed {
        authority: RunAuthority,
        peaks: MeasuredPeaks,
        stop: StopStamp,
        disposition: TerminalDisposition,
    },
}

impl OuterTerminalReply {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::ReportDescriptor { .. } => 1,
            Self::Committed { .. } => 0,
        }
    }
}

/// Sent only after C has validated seals/type/size and completed its original bounded report read.
/// This precedes the final metrics commit and cannot create a recursive acknowledgment window.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum CallerTerminalControl {
    /// C authenticated I Completed and closed its original lease. This carries its already
    /// started settlement cutoff, including the original None admission's one first-stop R.
    CompletedClose {
        authority: RunAuthority,
        deadline: RoleDeadline,
        stop: StopStamp,
    },
    ReadCompleted {
        authority: RunAuthority,
        bytes: u64,
    },
}

/// Exact alternatives on C's one original startup stream. OwnerStop can shorten the original
/// clock under irreversible cancellation; it cannot authorize any phase or attest cleanup.
pub(super) enum OuterStartupControl {
    Phase(OuterPhaseReply),
    OwnerStop {
        authority: RunAuthority,
        peaks: MeasuredPeaks,
        stop: StopStamp,
        cause: OwnerStopCause,
    },
    SetupRefused {
        authority: RunAuthority,
        peaks: MeasuredPeaks,
        stop: StopStamp,
        failure: super::startup_envelope::PolicyFailureCause,
    },
}

impl OuterStartupControl {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Phase(reply) => reply.rights_count(),
            Self::OwnerStop { .. } | Self::SetupRefused { .. } => 0,
        }
    }

    pub(super) fn clock_only(&self) -> bool {
        matches!(self, Self::OwnerStop { .. } | Self::SetupRefused { .. })
    }
}

#[derive(Clone, Copy, Deserialize)]
enum StartupReplyKind {
    MonitorSpawned,
    InnerClaimed,
    GateReleased,
    Committed,
}

// Flat borrowed/scalar decoding avoids serde's internally-tagged Content accumulator. First
// select the tag without owning ignored fields, then enforce that selected variant's exact map.
#[derive(Deserialize)]
struct StartupReplySelector {
    kind: StartupReplyKind,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SimplePhaseReply {
    kind: StartupReplyKind,
    authority: RunAuthority,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ClaimedPhaseReply {
    kind: StartupReplyKind,
    authority: RunAuthority,
    start: u64,
    namespace: NamespaceIdentity,
}

#[derive(Deserialize)]
enum StopDispositionKind {
    OwnerStop,
    SetupRefused,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StopDisposition {
    kind: StopDispositionKind,
    cause: Option<OwnerStopCause>,
    failure: Option<super::startup_envelope::PolicyFailureCause>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StopCommittedReply {
    kind: StartupReplyKind,
    authority: RunAuthority,
    peaks: MeasuredPeaks,
    stop: StopStamp,
    disposition: StopDisposition,
}

/// Decode only a phase reply or the existing complete typed nonreport commit. No fallback to
/// report/Ready/Dispatch parsing exists. Preflight bounds integer/string parser scratch paths;
/// every second pass checks the exact selected schema, duplicate fields and complete JSON EOF.
pub(super) fn decode_outer_startup(
    payload: &[u8],
) -> Result<OuterStartupControl, super::control::ControlError> {
    use super::control::ControlError;
    use serde::de::Error as _;
    let invalid = || {
        ControlError::InvalidEncoding(serde_json::Error::custom("unexpected outer startup reply"))
    };
    super::startup_cause::check_scratch_free_json(payload)
        .map_err(|error| ControlError::InvalidEncoding(serde_json::Error::custom(error)))?;
    let selected: StartupReplySelector =
        serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
    match selected.kind {
        StartupReplyKind::MonitorSpawned | StartupReplyKind::GateReleased => {
            let reply: SimplePhaseReply =
                serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
            let phase = match reply.kind {
                StartupReplyKind::MonitorSpawned => OuterPhaseReply::MonitorSpawned {
                    authority: reply.authority,
                },
                StartupReplyKind::GateReleased => OuterPhaseReply::GateReleased {
                    authority: reply.authority,
                },
                StartupReplyKind::InnerClaimed | StartupReplyKind::Committed => {
                    return Err(invalid())
                }
            };
            Ok(OuterStartupControl::Phase(phase))
        }
        StartupReplyKind::InnerClaimed => {
            let reply: ClaimedPhaseReply =
                serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
            if !matches!(reply.kind, StartupReplyKind::InnerClaimed) {
                return Err(invalid());
            }
            Ok(OuterStartupControl::Phase(OuterPhaseReply::InnerClaimed {
                authority: reply.authority,
                start: reply.start,
                namespace: reply.namespace,
            }))
        }
        StartupReplyKind::Committed => {
            let reply: StopCommittedReply =
                serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
            if !matches!(reply.kind, StartupReplyKind::Committed) {
                return Err(invalid());
            }
            match (
                reply.disposition.kind,
                reply.disposition.cause,
                reply.disposition.failure,
            ) {
                (StopDispositionKind::OwnerStop, Some(cause), None) => {
                    Ok(OuterStartupControl::OwnerStop {
                        authority: reply.authority,
                        peaks: reply.peaks,
                        stop: reply.stop,
                        cause,
                    })
                }
                (StopDispositionKind::SetupRefused, None, Some(failure)) => {
                    Ok(OuterStartupControl::SetupRefused {
                        authority: reply.authority,
                        peaks: reply.peaks,
                        stop: reply.stop,
                        failure,
                    })
                }
                _ => Err(invalid()),
            }
        }
    }
}

/// Fixed owning scalar decode/storage reservation, separate from the already retained frame
/// payload/right buffers. No String/Vec/Content accumulator belongs to this schema decoder.
pub(super) fn outer_startup_decode_bytes() -> Result<u64, super::control::ControlError> {
    let packet = [
        std::mem::size_of::<SimplePhaseReply>(),
        std::mem::size_of::<ClaimedPhaseReply>(),
        std::mem::size_of::<StopCommittedReply>(),
    ]
    .into_iter()
    .max()
    .ok_or(super::control::ControlError::EncodedBytesExceeded)?;
    let total = packet
        .checked_add(std::mem::size_of::<StartupReplySelector>())
        .and_then(|bytes| bytes.checked_add(std::mem::size_of::<OuterStartupControl>()))
        .ok_or(super::control::ControlError::EncodedBytesExceeded)?;
    u64::try_from(total).map_err(|_| super::control::ControlError::EncodedBytesExceeded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani::run::role_deadline::StopOrigin;

    /// Trace: FR-034-AC-15, FR-034-AC-35, FR-034-AC-38, FR-034-AC-39
    #[test]
    fn negative_commit_is_distinct_from_owner_stop_and_requires_original_cause() {
        let authority = RunAuthority::fresh().unwrap();
        let stop = StopStamp::capture(StopOrigin::Backend).unwrap();
        let failure = super::super::startup_envelope::PolicyFailureCause::ProtectionUnverified;
        let original = serde_json::to_value(OuterTerminalReply::Committed {
            authority,
            peaks: MeasuredPeaks {
                tree_rss_bytes: 17,
                charged_bytes: 31,
            },
            stop,
            disposition: TerminalDisposition::SetupRefused { failure },
        })
        .unwrap();
        let OuterStartupControl::SetupRefused {
            authority: actual_authority,
            peaks,
            stop: actual_stop,
            failure: actual_failure,
        } = decode_outer_startup(&serde_json::to_vec(&original).unwrap()).unwrap()
        else {
            panic!("negative commit changed disposition");
        };
        assert_eq!(actual_authority, authority);
        assert_eq!(actual_stop, stop);
        assert_eq!(actual_failure, failure);
        assert_eq!(peaks.charged_bytes, 31);
        assert_eq!(peaks.tree_rss_bytes, 17);
        let mut missing = original.clone();
        missing["disposition"]
            .as_object_mut()
            .unwrap()
            .remove("failure");
        assert!(decode_outer_startup(&serde_json::to_vec(&missing).unwrap()).is_err());
        let mut conflicting = original.clone();
        conflicting["disposition"]["cause"] = "TimedOut".into();
        assert!(decode_outer_startup(&serde_json::to_vec(&conflicting).unwrap()).is_err());
        let mut renamed = original;
        renamed["disposition"]["kind"] = "OwnerStop".into();
        assert!(decode_outer_startup(&serde_json::to_vec(&renamed).unwrap()).is_err());
    }

    /// Trace: FR-034-AC-15, FR-034-AC-38.
    #[test]
    fn provisional_stop_decoder_requires_exact_complete_metadata_and_never_accepts_report() {
        let authority = RunAuthority::fresh().unwrap();
        let stop = StopStamp::capture(StopOrigin::Outer).unwrap();
        let original = serde_json::to_value(OuterTerminalReply::Committed {
            authority,
            peaks: MeasuredPeaks {
                tree_rss_bytes: 17,
                charged_bytes: 31,
            },
            stop,
            disposition: TerminalDisposition::OwnerStop {
                cause: OwnerStopCause::TimedOut,
            },
        })
        .unwrap();
        for field in ["kind", "authority", "peaks", "stop", "disposition"] {
            let mut missing = original.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(
                decode_outer_startup(&serde_json::to_vec(&missing).unwrap()).is_err(),
                "{field}"
            );
            let mut wrong = original.clone();
            wrong[field] = serde_json::Value::Null;
            assert!(
                decode_outer_startup(&serde_json::to_vec(&wrong).unwrap()).is_err(),
                "{field}"
            );
        }
        let mut extra = original.clone();
        extra["unknown"] = true.into();
        assert!(decode_outer_startup(&serde_json::to_vec(&extra).unwrap()).is_err());
        let mut report = original.clone();
        report["disposition"] = serde_json::json!({ "kind": "Report" });
        assert!(decode_outer_startup(&serde_json::to_vec(&report).unwrap()).is_err());
        let bytes = serde_json::to_vec(&original).unwrap();
        let mut trailing = bytes.clone();
        trailing.extend_from_slice(b"{}");
        assert!(decode_outer_startup(&trailing).is_err());
        let encoded = String::from_utf8(bytes).unwrap();
        let duplicate = encoded.replacen(
            "\"kind\":\"Committed\"",
            "\"kind\":\"Committed\",\"kind\":\"Committed\"",
            1,
        );
        assert!(decode_outer_startup(duplicate.as_bytes()).is_err());
        let escaped = encoded.replacen("Committed", "\\u0043ommitted", 1);
        assert!(decode_outer_startup(escaped.as_bytes()).is_err());
    }
}

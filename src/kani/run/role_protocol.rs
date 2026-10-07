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
    report_storage::PipeIdentity,
    resource_ledger::MeasuredPeaks,
    role_deadline::{ExecutionClock, IdentityDeadline, MonotonicInstant, RoleDeadline, StopStamp},
};

// The same declaration supplies both the owning settings type and its positional order.
macro_rules! run_settings {
    ($($variant:ident => $(#[$attribute:meta])* $field:ident: $value:ty),+ $(,)?) => {
        /// C-origin settings, forwarded without replacing the original deadline or run authority.
        #[derive(Clone, Deserialize, Serialize)]
        #[serde(deny_unknown_fields)]
        pub(super) struct RunSettings { $($(#[$attribute])* pub(super) $field: $value),+ }
        #[derive(Clone, Copy)]
        pub(super) enum SettingsField { $($variant),+ }
        impl SettingsField {
            pub(super) fn declared_order() -> &'static [Self] { &[$(Self::$variant),+] }
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($field)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}
run_settings! {
    Helper =>
        /// The original explicitly supplied helper, resolved by C without executable PATH search.
        helper: PathBuf,
    Identity => identity: BuildIdentity,
    Authority => authority: RunAuthority,
    Deadline => deadline: IdentityDeadline,
    Started =>
        /// Mandatory C kernel-clock lower bound, captured before any creating thread/L starts.
        started: MonotonicInstant,
    SettlementReserve =>
        /// The same original C-derived R_eff, never a receiving role's new budget.
        settlement_reserve: std::time::Duration,
    WorkDeadline =>
        /// Original C-derived cutoff T-R_eff, not a fresh role-local work allowance.
        work_deadline: IdentityDeadline,
    SetupDeadline =>
        /// C’s original finite bootstrap cap, bounded by the original identity deadline.
        setup_deadline: RoleDeadline,
    CallerUid => caller_uid: u32,
    CallerGid => caller_gid: u32,
    MemoryBytes => memory_bytes: NonZeroU64,
    CallerRunBuffers => caller_run_buffers: u64,
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
        startup_deadline_from_parts(
            self.deadline,
            self.started,
            self.settlement_reserve,
            self.work_deadline,
            self.setup_deadline,
        )
    }

    pub(super) fn work_deadline(
        &self,
    ) -> Result<Option<std::time::Instant>, super::role_deadline::DeadlineError> {
        self.work_deadline.local()
    }
}

/// The same original transferred clock check for owned settings and authenticated borrowed
/// settings before pathname allocation. It creates no new time window or admission milestone.
pub(super) fn startup_deadline_from_parts(
    deadline: IdentityDeadline,
    started: MonotonicInstant,
    reserve: std::time::Duration,
    work: IdentityDeadline,
    setup: RoleDeadline,
) -> Result<std::time::Instant, super::role_deadline::DeadlineError> {
    if reserve > super::role_deadline::SETTLE_RESERVE
        || (matches!(deadline, IdentityDeadline::NeverElapses)
            && reserve != super::role_deadline::SETTLE_RESERVE)
    {
        return Err(super::role_deadline::DeadlineError::InvalidClock);
    }
    started.require_started()?;
    let cap = setup.local()?;
    Ok(work.local()?.map_or(cap, |deadline| deadline.min(cap)))
}

// The owning declaration supplies both emitted fields and parsed variant field order.
macro_rules! bootstrap_controls {
    ($(#[$enum_attribute:meta])* $name:ident, $kind:ident { $($(#[$attribute:meta])* $variant:ident $( { $($field:ident: $value:ty),+ $(,)? } )?),+ $(,)? }) => {
        $(#[$enum_attribute])*
        #[derive(Deserialize, Serialize)]
        #[serde(tag = "kind", deny_unknown_fields)]
        pub(super) enum $name { $($(#[$attribute])* $variant $( { $($field: $value),+ } )?),+ }
        #[derive(Clone, Copy)]
        pub(super) enum $kind { $($variant),+ }
        impl $kind {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
            pub(super) fn declared_fields(self) -> &'static [&'static str] {
                match self { $(Self::$variant => &[$($(stringify!($field)),+)?]),+ }
            }
        }
    };
}

bootstrap_controls! { LauncherControl, LauncherControlKind {
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
}}

impl LauncherControl {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 4,
            Self::Settle { .. } | Self::Retire { .. } => 0,
        }
    }
}

macro_rules! settlement_modes {
    ($($variant:ident),+ $(,)?) => {
        /// Private whole-owner settlement selection, never a public caller cancellation handle.
        #[derive(Clone, Copy, Deserialize, Serialize)]
        pub(super) enum LauncherSettlementMode { $($variant),+ }
        impl LauncherSettlementMode {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}
settlement_modes!(ObserveOuterExit, CancelOuter);

bootstrap_controls! {
    /// Trusted L reports only custody established from its retained actual Child wait result.
    #[derive(Clone, Copy, Debug)]
    OuterChildSettlement, OuterChildSettlementKind {
        NotCreated,
        Reaped { outcome: BackendExit },
    }
}

bootstrap_controls! {
    /// Authenticated L-origin bootstrap status. Neither status grants backend authorization.
    LauncherReply, LauncherReplyKind {
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
}}

impl LauncherReply {
    pub(super) fn rights_count(&self) -> usize {
        0
    }
}

bootstrap_controls! { OuterBootstrap, OuterBootstrapKind {
    /// Rights: actual L process pin, actual C process pin, O endpoint, I lease endpoint.
    Start {
        settings: RunSettings,
        original_mount: NamespaceIdentity,
        original_pid: NamespaceIdentity,
        original_network: NamespaceIdentity,
    },
    /// Rights: actual L stat/status files, opened before private proc replacement.
    LauncherObservation { authority: RunAuthority },
}}

impl OuterBootstrap {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 4,
            Self::LauncherObservation { .. } => 2,
        }
    }
}

bootstrap_controls! {
    /// Actual O-origin arm publication, received separately by L and C with one actual O pidfd.
    OuterArmReply, OuterArmReplyKind {
    Armed {
        identity: BuildIdentity,
        authority: RunAuthority,
        namespace: NamespaceIdentity,
        network: NamespaceIdentity,
        mapped_uid: u32,
        mapped_gid: u32,
    },
}}

impl OuterArmReply {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Armed { .. } => 1,
        }
    }
}

bootstrap_controls! { InnerBootstrap, InnerBootstrapKind {
    /// Rights: actual O process pin, actual C process pin, original I lease endpoint.
    Start {
        settings: RunSettings,
        outer_namespace: NamespaceIdentity,
        report: PipeIdentity,
        report_slot: i32,
    },
}}

impl InnerBootstrap {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 3,
        }
    }
}

/// I's temporary authenticated channel to the same-PID trusted backend installer. The original
/// environment stays metadata until policy admission and C's genuine positive Dispatch.
macro_rules! backend_installer_controls {
    ($($(#[$attribute:meta])* $variant:ident { $($field:ident: $value:ty),+ $(,)? }),+ $(,)?) => {
        #[derive(Deserialize, Serialize)]
        #[serde(tag = "kind", deny_unknown_fields)]
        pub(super) enum BackendInstallerControl { $($(#[$attribute])* $variant { $($field: $value),+ }),+ }
        #[derive(Clone, Copy)]
        pub(super) enum InstallerControlKind { $($variant),+ }
        impl InstallerControlKind {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
            pub(super) fn declared_fields(self) -> &'static [&'static str] {
                match self { $(Self::$variant => &[$(stringify!($field)),+]),+ }
            }
        }
    };
}
backend_installer_controls! {
    /// Rights: actual I process pin, original C process pin, owned original O report writer.
    Start { settings: RunSettings, report: PipeIdentity },
    /// Sent only after genuine C/I Dispatch. It starts no second backend process or budget.
    Exec { authority: RunAuthority, command: super::namespace::BackendCommand, stdin: super::protocol::StdinControl },
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

bootstrap_controls! {
    /// O-origin startup replies carry real process capabilities, never numbers standing in for pins.
    OuterPhaseReply, OuterPhaseReplyKind {
        MonitorSpawned { authority: RunAuthority },
        InnerClaimed { authority: RunAuthority, start: u64, namespace: NamespaceIdentity },
        GateReleased { authority: RunAuthority },
    }
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

bootstrap_controls! {
    /// O has actually authenticated I's complete backend event. Queueing the I-origin event alone
    /// grants no permission to publish C Completed and let original lease close destroy its sender.
    InnerOwnerControl, InnerOwnerControlKind {
        CompletionObserved { authority: RunAuthority, stop: StopStamp },
    }
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

macro_rules! owner_stop_causes {
    ($($variant:ident),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
        pub(super) enum OwnerStopCause { $($variant),+ }
        impl OwnerStopCause {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}

owner_stop_causes! { ResourceExhausted, TimedOut }

/// O's terminal transaction remains separate from the already consumed I lease. A decoded
/// commit is provisional until actual normal O/L/thread/capture settlement and stream-end checks.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum OuterTerminalReply {
    /// Provisional receipt of authenticated caller cancellation after actual M settlement.
    /// Required peaks come from successful complete O observations; they remain provisional.
    /// No report or classification authority is created; C still confirms actual normal
    /// O/L/capture/creator settlement inside the original cutoff.
    Cancelled {
        authority: RunAuthority,
        peaks: MeasuredPeaks,
        stop: StopStamp,
    },
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
            Self::Committed { .. } | Self::Cancelled { .. } => 0,
        }
    }
}

/// C's existing bounded close/read transaction on the independent original O channel.
/// Cancellation is distinct from authenticated I completion and cannot admit a report. The
/// existing read acknowledgment creates no recursive acknowledgment window.
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum CallerTerminalControl {
    /// C irreversibly stopped the original run and closed its actual I lease. The original C
    /// first-stop stamp/cutoff is carried without transporting or reclassifying its local error.
    CancelClose {
        authority: RunAuthority,
        deadline: RoleDeadline,
        stop: StopStamp,
    },
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

/// Exact provisional alternatives for C's irreversible cancellation path only. Actor-owned
/// authentication and whole-chain settlement remain mandatory; no report admission follows.
pub(super) enum CancellationHeader {
    Cancelled {
        authority: RunAuthority,
        peaks: MeasuredPeaks,
        stop: StopStamp,
    },
    OwnerStop {
        authority: RunAuthority,
        peaks: MeasuredPeaks,
        stop: StopStamp,
        cause: OwnerStopCause,
    },
}

impl CancellationHeader {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Cancelled { .. } | Self::OwnerStop { .. } => 0,
        }
    }
}

/// Cancellation-only progress on C's same original startup receive cursor. A queued phase
/// carries only cleanup custody here: it grants no phase advancement, Dispatch or report read.
/// The actor authenticates the original O/run and exact in-flight phase before taking any rights.
pub(super) enum CancellationProgress {
    ConstructorTimeout(super::outer_failure::ConstructorTimeoutHeader),
    Phase(OuterPhaseReply),
    Terminal(CancellationHeader),
}

impl CancellationProgress {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::ConstructorTimeout(_) => 0,
            Self::Phase(phase) => phase.rights_count(),
            Self::Terminal(terminal) => terminal.rights_count(),
        }
    }
}

/// State-specific cleanup-only subset of the sole fixed O->C union decoder.
pub(super) fn decode_cancellation_progress(
    payload: &[u8],
    context: &mut super::startup_cause::PreparedStartupContext,
    scratch: &mut super::guardian_decode::Scratch,
) -> Result<CancellationProgress, super::control::ControlError> {
    super::outer_reply::cancellation_progress(payload, context, scratch)
}

/// Exact scalar close-only view of O's existing sole fixed C-control grammar.
/// No phase/read ACK or actor authorization follows this parsed value.
pub(super) fn decode_caller_close(
    payload: &[u8],
    scratch: &mut super::guardian_decode::Scratch,
) -> Result<CallerTerminalControl, super::control::ControlError> {
    super::outer_caller::decode_close(payload, scratch)
}

/// Exact cancellation receipt or genuine owner-stop only; no phase/report permission.
pub(super) fn decode_cancellation_commit(
    payload: &[u8],
    context: &mut super::startup_cause::PreparedStartupContext,
    scratch: &mut super::guardian_decode::Scratch,
) -> Result<CancellationHeader, super::control::ControlError> {
    super::outer_reply::cancellation_commit(payload, context, scratch)
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

macro_rules! outer_reply_kinds {
    ($($variant:ident),+ $(,)?) => {
        #[derive(Clone, Copy, Deserialize)]
        pub(super) enum OuterReplyKind { $($variant),+ }
        impl OuterReplyKind {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}

// One complete O->C frame-tag inventory. Each receiver still selects its own exact context;
// adding a parsed tag does not authorize that tag in startup/report/cancellation.
outer_reply_kinds! { MonitorSpawned, InnerClaimed, GateReleased, Committed, ReportDescriptor, Cancelled }

/// Original startup subset of the sole fixed union; terminal/report frames remain refused.
pub(super) fn decode_outer_startup(
    payload: &[u8],
    context: &mut super::startup_cause::PreparedStartupContext,
    scratch: &mut super::guardian_decode::Scratch,
) -> Result<OuterStartupControl, super::control::ControlError> {
    match super::outer_reply::decode(payload, context, scratch)? {
        super::outer_reply::OuterReply::Startup(reply) => Ok(reply),
        super::outer_reply::OuterReply::Failure(_)
        | super::outer_reply::OuterReply::ConstructorTimeout(_) => Err(
            super::control::ControlError::InvalidGrammar(super::guardian_decode::DecodeError::new(
                super::guardian_decode::DecodeSite::Field,
                super::guardian_decode::DecodeCause::InvalidValue,
            )),
        ),
    }
}

/// Exact alternatives after actual I completion/lease close. A nonreport stop remains
/// provisional and cannot stand for a delivered report descriptor or bounded read completion.
pub(super) enum ReportStartHeader {
    Descriptor {
        authority: RunAuthority,
        bytes: u64,
    },
    OwnerStop {
        authority: RunAuthority,
        peaks: MeasuredPeaks,
        stop: StopStamp,
        cause: OwnerStopCause,
    },
}

impl ReportStartHeader {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Descriptor { .. } => 1,
            Self::OwnerStop { .. } => 0,
        }
    }
    pub(super) fn clock_only(&self) -> bool {
        matches!(self, Self::OwnerStop { .. })
    }
}

/// Descriptor or genuine owner-stop only, on the actual report-start cursor.
pub(super) fn decode_report_start(
    payload: &[u8],
    context: &mut super::startup_cause::PreparedStartupContext,
    scratch: &mut super::guardian_decode::Scratch,
) -> Result<ReportStartHeader, super::control::ControlError> {
    super::outer_reply::report_start(payload, context, scratch)
}

/// Exact Report/OwnerStop terminal subset with no parse-and-retry or fallback decoder.
pub(super) fn decode_terminal_commit(
    payload: &[u8],
    context: &mut super::startup_cause::PreparedStartupContext,
    scratch: &mut super::guardian_decode::Scratch,
) -> Result<OuterTerminalReply, super::control::ControlError> {
    super::outer_reply::terminal_commit(payload, context, scratch)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parsed {
        ($name:ident, $result:ty) => {
            fn $name(payload: &[u8]) -> Result<$result, super::super::control::ControlError> {
                super::$name(
                    payload,
                    &mut super::super::startup_cause::PreparedStartupContext::new(0).unwrap(),
                    &mut super::super::guardian_decode::Scratch::default(),
                )
            }
        };
    }
    fn decode_caller_close(
        payload: &[u8],
    ) -> Result<CallerTerminalControl, super::super::control::ControlError> {
        super::decode_caller_close(
            payload,
            &mut super::super::guardian_decode::Scratch::default(),
        )
    }

    parsed!(decode_outer_startup, OuterStartupControl);
    parsed!(decode_cancellation_progress, CancellationProgress);
    parsed!(decode_cancellation_commit, CancellationHeader);
    parsed!(decode_report_start, ReportStartHeader);
    parsed!(decode_terminal_commit, OuterTerminalReply);
    use crate::kani::run::role_deadline::StopOrigin;

    /// Trace: FR-034-AC-15, FR-034-AC-33, FR-034-AC-34, FR-034-AC-38.
    #[test]
    fn cancellation_progress_preserves_exact_queued_phase_cleanup_rights_only() {
        let authority = RunAuthority::fresh().unwrap();
        // Fixed schema values only; no namespace observation or role custody is claimed.
        let namespace: NamespaceIdentity =
            serde_json::from_value(serde_json::json!({ "device": 1, "inode": 2 })).unwrap();
        for (reply, expected, expected_rights) in [
            (OuterPhaseReply::MonitorSpawned { authority }, 0, 1),
            (
                OuterPhaseReply::InnerClaimed {
                    authority,
                    start: 23,
                    namespace,
                },
                1,
                1,
            ),
            (OuterPhaseReply::GateReleased { authority }, 2, 0),
        ] {
            let original = serde_json::to_value(reply).unwrap();
            let bytes = serde_json::to_vec(&original).unwrap();
            let progress = decode_cancellation_progress(&bytes).unwrap();
            assert_eq!(progress.rights_count(), expected_rights);
            let CancellationProgress::Phase(phase) = progress else {
                panic!("queued phase became terminal receipt");
            };
            assert_eq!(phase.authority(), authority);
            let actual = match phase {
                OuterPhaseReply::MonitorSpawned { .. } => 0,
                OuterPhaseReply::InnerClaimed {
                    start,
                    namespace: actual,
                    ..
                } => {
                    assert_eq!(start, 23);
                    assert_eq!(actual, namespace);
                    1
                }
                OuterPhaseReply::GateReleased { .. } => 2,
            };
            assert_eq!(actual, expected);
            assert!(decode_cancellation_commit(&bytes).is_err());
            assert!(decode_report_start(&bytes).is_err());
            assert!(decode_terminal_commit(&bytes).is_err());
            for field in original.as_object().unwrap().keys() {
                let mut missing = original.clone();
                missing.as_object_mut().unwrap().remove(field);
                assert!(
                    decode_cancellation_progress(&serde_json::to_vec(&missing).unwrap()).is_err()
                );
                let mut malformed = original.clone();
                malformed[field] = serde_json::Value::Null;
                assert!(
                    decode_cancellation_progress(&serde_json::to_vec(&malformed).unwrap()).is_err()
                );
            }
            let mut extra = original.clone();
            extra["bytes"] = 1.into();
            assert!(decode_cancellation_progress(&serde_json::to_vec(&extra).unwrap()).is_err());
            let encoded = serde_json::to_string(&original).unwrap();
            let conflicting = encoded.replacen("{", "{\"kind\":\"Cancelled\",", 1);
            assert!(decode_cancellation_progress(conflicting.as_bytes()).is_err());
            let mut trailing = bytes;
            trailing.extend_from_slice(b"{}");
            assert!(decode_cancellation_progress(&trailing).is_err());
            assert!(decode_cancellation_progress(&trailing[..trailing.len() / 2]).is_err());
        }
        let stop = StopStamp::capture(StopOrigin::Caller).unwrap();
        let peaks = MeasuredPeaks {
            tree_rss_bytes: 7,
            charged_bytes: 19,
        };
        let cancelled = serde_json::to_vec(&OuterTerminalReply::Cancelled {
            authority,
            peaks,
            stop,
        })
        .unwrap();
        let receipt = decode_cancellation_progress(&cancelled).unwrap();
        assert_eq!(receipt.rights_count(), 0);
        assert!(
            matches!(receipt, CancellationProgress::Terminal(CancellationHeader::Cancelled {
            authority: actual, peaks: actual_peaks, stop: actual_stop,
        }) if actual == authority && actual_stop == stop
            && actual_peaks.tree_rss_bytes == peaks.tree_rss_bytes
            && actual_peaks.charged_bytes == peaks.charged_bytes)
        );
        for cause in [OwnerStopCause::ResourceExhausted, OwnerStopCause::TimedOut] {
            let stop_commit = serde_json::to_vec(&OuterTerminalReply::Committed {
                authority,
                stop,
                peaks: MeasuredPeaks {
                    tree_rss_bytes: 7,
                    charged_bytes: 11,
                },
                disposition: TerminalDisposition::OwnerStop { cause },
            })
            .unwrap();
            let progress = decode_cancellation_progress(&stop_commit).unwrap();
            assert_eq!(progress.rights_count(), 0);
            assert!(
                matches!(progress, CancellationProgress::Terminal(CancellationHeader::OwnerStop {
                authority: actual, stop: actual_stop, cause: actual_cause, ..
            }) if actual == authority && actual_stop == stop && actual_cause == cause)
            );
        }
        for refused in [
            OuterTerminalReply::ReportDescriptor {
                authority,
                bytes: 1,
            },
            OuterTerminalReply::Committed {
                authority,
                stop,
                peaks: MeasuredPeaks {
                    tree_rss_bytes: 7,
                    charged_bytes: 11,
                },
                disposition: TerminalDisposition::Report,
            },
            OuterTerminalReply::Committed {
                authority,
                stop,
                peaks: MeasuredPeaks {
                    tree_rss_bytes: 7,
                    charged_bytes: 11,
                },
                disposition: TerminalDisposition::SetupRefused {
                    failure:
                        super::super::startup_envelope::PolicyFailureCause::ProtectionUnverified,
                },
            },
        ] {
            assert!(decode_cancellation_progress(&serde_json::to_vec(&refused).unwrap()).is_err());
        }
    }

    /// Trace: FR-034-AC-15, FR-034-AC-33, FR-034-AC-34, FR-034-AC-38.
    #[test]
    fn close_decoder_keeps_cancellation_distinct_and_requires_exact_original_clock_fields() {
        let authority = RunAuthority::fresh().unwrap();
        let stop = StopStamp::capture(StopOrigin::Caller).unwrap();
        let deadline = RoleDeadline::from_original(std::time::Instant::now()).unwrap();
        for (control, cancelled) in [
            (
                CallerTerminalControl::CancelClose {
                    authority,
                    deadline,
                    stop,
                },
                true,
            ),
            (
                CallerTerminalControl::CompletedClose {
                    authority,
                    deadline,
                    stop,
                },
                false,
            ),
        ] {
            let original = serde_json::to_value(control).unwrap();
            let decoded = decode_caller_close(&serde_json::to_vec(&original).unwrap()).unwrap();
            let (actual_authority, actual_deadline, actual_stop, actual_cancelled) = match decoded {
                CallerTerminalControl::CancelClose {
                    authority,
                    deadline,
                    stop,
                } => (authority, deadline, stop, true),
                CallerTerminalControl::CompletedClose {
                    authority,
                    deadline,
                    stop,
                } => (authority, deadline, stop, false),
                CallerTerminalControl::ReadCompleted { .. } => panic!("close became a read ACK"),
            };
            assert_eq!(actual_authority, authority);
            assert_eq!(actual_deadline, deadline);
            assert_eq!(actual_stop, stop);
            assert_eq!(actual_cancelled, cancelled);
            for field in ["kind", "authority", "deadline", "stop"] {
                let mut missing = original.clone();
                missing.as_object_mut().unwrap().remove(field);
                assert!(decode_caller_close(&serde_json::to_vec(&missing).unwrap()).is_err());
                let mut malformed = original.clone();
                malformed[field] = serde_json::Value::Null;
                assert!(decode_caller_close(&serde_json::to_vec(&malformed).unwrap()).is_err());
            }
            let mut extra = original.clone();
            extra["bytes"] = 1.into();
            assert!(decode_caller_close(&serde_json::to_vec(&extra).unwrap()).is_err());
            let encoded = serde_json::to_string(&original).unwrap();
            let duplicate = encoded.replacen("{", "{\"kind\":\"CancelClose\",", 1);
            assert!(decode_caller_close(duplicate.as_bytes()).is_err());
        }
        let ack = serde_json::to_vec(&CallerTerminalControl::ReadCompleted {
            authority,
            bytes: 0,
        })
        .unwrap();
        assert!(decode_caller_close(&ack).is_err());
    }

    /// Trace: FR-034-AC-15, FR-034-AC-33, FR-034-AC-34, FR-034-AC-38.
    #[test]
    fn cancellation_receipt_requires_exact_peaks_without_report_authority() {
        let authority = RunAuthority::fresh().unwrap();
        let stop = StopStamp::capture(StopOrigin::Caller).unwrap();
        let peaks = MeasuredPeaks {
            tree_rss_bytes: 7,
            charged_bytes: 19,
        };
        let receipt = OuterTerminalReply::Cancelled {
            authority,
            peaks,
            stop,
        };
        assert_eq!(receipt.rights_count(), 0);
        let original = serde_json::to_value(receipt).unwrap();
        let bytes = serde_json::to_vec(&original).unwrap();
        let header = decode_cancellation_commit(&bytes).unwrap();
        assert_eq!(header.rights_count(), 0);
        let CancellationHeader::Cancelled {
            authority: actual,
            peaks: actual_peaks,
            stop: actual_stop,
        } = header
        else {
            panic!("cancellation invented an owner stop");
        };
        assert_eq!(actual, authority);
        assert_eq!(actual_stop, stop);
        assert_eq!(actual_peaks.tree_rss_bytes, peaks.tree_rss_bytes);
        assert_eq!(actual_peaks.charged_bytes, peaks.charged_bytes);
        assert!(decode_outer_startup(&bytes).is_err());
        assert!(decode_report_start(&bytes).is_err());
        assert!(decode_terminal_commit(&bytes).is_err());
        for field in ["kind", "authority", "peaks", "stop"] {
            let mut missing = original.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(decode_cancellation_commit(&serde_json::to_vec(&missing).unwrap()).is_err());
            let mut malformed = original.clone();
            malformed[field] = serde_json::Value::Null;
            assert!(decode_cancellation_commit(&serde_json::to_vec(&malformed).unwrap()).is_err());
        }
        for field in ["cause", "bytes", "disposition", "deadline"] {
            let mut extra = original.clone();
            extra[field] = serde_json::Value::Null;
            assert!(decode_cancellation_commit(&serde_json::to_vec(&extra).unwrap()).is_err());
        }
        for field in ["tree_rss_bytes", "charged_bytes"] {
            let mut missing = original.clone();
            missing["peaks"].as_object_mut().unwrap().remove(field);
            let encoded = serde_json::to_vec(&missing).unwrap();
            assert!(decode_cancellation_commit(&encoded).is_err());
            assert!(decode_cancellation_progress(&encoded).is_err());
            let mut malformed = original.clone();
            malformed["peaks"][field] = serde_json::Value::Null;
            let encoded = serde_json::to_vec(&malformed).unwrap();
            assert!(decode_cancellation_commit(&encoded).is_err());
            assert!(decode_cancellation_progress(&encoded).is_err());
        }
        let mut extra_peak = original.clone();
        extra_peak["peaks"]["unknown"] = 1.into();
        assert!(decode_cancellation_commit(&serde_json::to_vec(&extra_peak).unwrap()).is_err());
        let encoded = serde_json::to_string(&original).unwrap();
        let conflicting = encoded.replacen("{", "{\"kind\":\"Committed\",", 1);
        assert!(decode_cancellation_commit(conflicting.as_bytes()).is_err());
        let mut trailing = bytes;
        trailing.extend_from_slice(b"{}");
        assert!(decode_cancellation_commit(&trailing).is_err());
        for cause in [OwnerStopCause::ResourceExhausted, OwnerStopCause::TimedOut] {
            let peaks = MeasuredPeaks {
                tree_rss_bytes: 7,
                charged_bytes: 19,
            };
            let commit = serde_json::to_vec(&OuterTerminalReply::Committed {
                authority,
                peaks,
                stop,
                disposition: TerminalDisposition::OwnerStop { cause },
            })
            .unwrap();
            let CancellationHeader::OwnerStop {
                authority: actual,
                peaks: actual_peaks,
                stop: actual_stop,
                cause: actual_cause,
            } = decode_cancellation_commit(&commit).unwrap()
            else {
                panic!("genuine owner stop changed branch");
            };
            assert_eq!(actual, authority);
            assert_eq!(actual_peaks.tree_rss_bytes, peaks.tree_rss_bytes);
            assert_eq!(actual_peaks.charged_bytes, peaks.charged_bytes);
            assert_eq!(actual_stop, stop);
            assert_eq!(actual_cause, cause);
        }
        for refused in [
            serde_json::to_vec(&OuterTerminalReply::Committed {
                authority,
                peaks: MeasuredPeaks {
                    tree_rss_bytes: 7,
                    charged_bytes: 19,
                },
                stop,
                disposition: TerminalDisposition::SetupRefused {
                    failure:
                        super::super::startup_envelope::PolicyFailureCause::ProtectionUnverified,
                },
            })
            .unwrap(),
            serde_json::to_vec(&OuterPhaseReply::MonitorSpawned { authority }).unwrap(),
            serde_json::to_vec(&OuterTerminalReply::ReportDescriptor {
                authority,
                bytes: 1,
            })
            .unwrap(),
            serde_json::to_vec(&OuterTerminalReply::Committed {
                authority,
                peaks: MeasuredPeaks {
                    tree_rss_bytes: 7,
                    charged_bytes: 19,
                },
                stop,
                disposition: TerminalDisposition::Report,
            })
            .unwrap(),
        ] {
            assert!(decode_cancellation_commit(&refused).is_err());
        }
    }

    /// Trace: FR-034-AC-15, FR-034-AC-32, FR-034-AC-33, FR-034-AC-38
    #[test]
    fn report_receiver_distinguishes_descriptor_final_report_and_late_stop() {
        let authority = RunAuthority::fresh().unwrap();
        let stop = StopStamp::capture(StopOrigin::Outer).unwrap();
        let descriptor = serde_json::to_vec(&OuterTerminalReply::ReportDescriptor {
            authority,
            bytes: 73,
        })
        .unwrap();
        let ReportStartHeader::Descriptor {
            authority: actual,
            bytes,
        } = decode_report_start(&descriptor).unwrap()
        else {
            panic!("descriptor changed branch");
        };
        assert_eq!(actual, authority);
        assert_eq!(bytes, 73);
        assert!(decode_terminal_commit(&descriptor).is_err());
        let mut report = serde_json::to_value(OuterTerminalReply::Committed {
            authority,
            peaks: MeasuredPeaks {
                tree_rss_bytes: 19,
                charged_bytes: 41,
            },
            stop,
            disposition: TerminalDisposition::Report,
        })
        .unwrap();
        assert!(matches!(
            decode_terminal_commit(&serde_json::to_vec(&report).unwrap()).unwrap(),
            OuterTerminalReply::Committed {
                disposition: TerminalDisposition::Report,
                ..
            }
        ));
        assert!(decode_report_start(&serde_json::to_vec(&report).unwrap()).is_err());
        report["disposition"] =
            serde_json::json!({ "kind": "OwnerStop", "cause": "ResourceExhausted" });
        let encoded = serde_json::to_vec(&report).unwrap();
        let ReportStartHeader::OwnerStop {
            authority: actual,
            peaks,
            stop: actual_stop,
            cause,
        } = decode_report_start(&encoded).unwrap()
        else {
            panic!("late resource stop changed branch");
        };
        assert_eq!(actual, authority);
        assert_eq!(actual_stop, stop);
        assert_eq!(cause, OwnerStopCause::ResourceExhausted);
        assert_eq!(peaks.charged_bytes, 41);
        assert!(matches!(
            decode_terminal_commit(&encoded).unwrap(),
            OuterTerminalReply::Committed {
                disposition: TerminalDisposition::OwnerStop {
                    cause: OwnerStopCause::ResourceExhausted
                },
                ..
            }
        ));
        for field in ["authority", "peaks", "stop", "disposition"] {
            let mut missing = report.clone();
            missing.as_object_mut().unwrap().remove(field);
            let bytes = serde_json::to_vec(&missing).unwrap();
            assert!(decode_report_start(&bytes).is_err(), "{field}");
            assert!(decode_terminal_commit(&bytes).is_err(), "{field}");
        }
        report["bytes"] = 73.into();
        let conflict = serde_json::to_vec(&report).unwrap();
        assert!(decode_report_start(&conflict).is_err());
        assert!(decode_terminal_commit(&conflict).is_err());
    }

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

//! One fixed full-union walk on the existing startup receive payload.
//!
//! Shared disposition and scalar schemas use the same cursor and scratch. This module owns no
//! receive cursor, sender/site authority, state transition, cutoff or settlement operation.
//! A negative reply is provisional clock/error data, never permission for phase or Dispatch.

use std::mem::{size_of, size_of_val};

use super::{
    control::ControlError,
    guardian_decode::{DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Scratch, Text},
    outer_failure::{FailureHeader, FailureRepresentation, FailureState},
    outer_failure_decode::{self, Disposition},
    outer_setup::NamespaceIdentity,
    protocol::{BuildIdentity, RunAuthority},
    resource_ledger::MeasuredPeaks,
    role_control_scalar_decode,
    role_deadline::StopStamp,
    role_protocol::{
        CancellationHeader, CancellationProgress, OuterPhaseReply, OuterReplyKind as ReplyKind,
        OuterStartupControl, OuterTerminalReply, ReportStartHeader, TerminalDisposition,
    },
    role_scalar_decode,
    startup_cause::PreparedStartupContext,
};

pub(super) enum OuterReply {
    Startup(OuterStartupControl),
    Failure(FailureHeader),
}

// Parsed frame facts are shared, while each public(super) entrypoint below admits only its
// original state-specific subset. This object owns no receive cursor or actor authority.
enum Frame {
    Startup(OuterStartupControl),
    Failure(FailureHeader),
    Terminal(OuterTerminalReply),
}

impl OuterReply {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Startup(reply) => reply.rights_count(),
            Self::Failure(reply) => reply.rights_count(),
        }
    }

    /// Clock-only data still requires actual owner authentication, retention and settlement.
    pub(super) fn clock_only(&self) -> bool {
        match self {
            Self::Startup(reply) => reply.clock_only(),
            Self::Failure(_) => true,
        }
    }
}

#[derive(Clone, Copy)]
enum ReplyField {
    Kind,
    Identity,
    Authority,
    Stop,
    Disposition,
    Start,
    Namespace,
    Peaks,
    Bytes,
}

impl ReplyField {
    fn from_text(text: Text<'_>) -> Result<Self, DecodeError> {
        if text.equals("kind") {
            Ok(Self::Kind)
        } else if text.equals("identity") {
            Ok(Self::Identity)
        } else if text.equals("authority") {
            Ok(Self::Authority)
        } else if text.equals("stop") {
            Ok(Self::Stop)
        } else if text.equals("disposition") {
            Ok(Self::Disposition)
        } else if text.equals("start") {
            Ok(Self::Start)
        } else if text.equals("namespace") {
            Ok(Self::Namespace)
        } else if text.equals("peaks") {
            Ok(Self::Peaks)
        } else if text.equals("bytes") {
            Ok(Self::Bytes)
        } else {
            Err(field_error(DecodeCause::UnknownField))
        }
    }
}

#[derive(Default)]
struct ReplyFields {
    kind: Option<ReplyKind>,
    identity: Option<BuildIdentity>,
    authority: Option<RunAuthority>,
    stop: Option<StopStamp>,
    disposition: Option<Disposition>,
    start: Option<u64>,
    namespace: Option<NamespaceIdentity>,
    peaks: Option<MeasuredPeaks>,
    bytes: Option<u64>,
}

fn field_error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}
fn required<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| field_error(DecodeCause::MissingField))
}

impl ReplyFields {
    fn finish(self) -> Result<Frame, DecodeError> {
        let authority = required(self.authority)?;
        match required(self.kind)? {
            ReplyKind::MonitorSpawned | ReplyKind::GateReleased => {
                if self.identity.is_some()
                    || self.stop.is_some()
                    || self.disposition.is_some()
                    || self.start.is_some()
                    || self.namespace.is_some()
                    || self.peaks.is_some()
                    || self.bytes.is_some()
                {
                    return Err(field_error(DecodeCause::UnknownField));
                }
                let phase = match required(self.kind)? {
                    ReplyKind::MonitorSpawned => OuterPhaseReply::MonitorSpawned { authority },
                    ReplyKind::GateReleased => OuterPhaseReply::GateReleased { authority },
                    ReplyKind::InnerClaimed
                    | ReplyKind::Committed
                    | ReplyKind::ReportDescriptor
                    | ReplyKind::Cancelled => return Err(field_error(DecodeCause::InvalidValue)),
                };
                Ok(Frame::Startup(OuterStartupControl::Phase(phase)))
            }
            ReplyKind::InnerClaimed => {
                if self.identity.is_some()
                    || self.stop.is_some()
                    || self.disposition.is_some()
                    || self.peaks.is_some()
                    || self.bytes.is_some()
                {
                    return Err(field_error(DecodeCause::UnknownField));
                }
                Ok(Frame::Startup(OuterStartupControl::Phase(
                    OuterPhaseReply::InnerClaimed {
                        authority,
                        start: required(self.start)?,
                        namespace: required(self.namespace)?,
                    },
                )))
            }
            ReplyKind::ReportDescriptor => {
                if self.identity.is_some()
                    || self.stop.is_some()
                    || self.disposition.is_some()
                    || self.start.is_some()
                    || self.namespace.is_some()
                    || self.peaks.is_some()
                {
                    return Err(field_error(DecodeCause::UnknownField));
                }
                Ok(Frame::Terminal(OuterTerminalReply::ReportDescriptor {
                    authority,
                    bytes: required(self.bytes)?,
                }))
            }
            ReplyKind::Cancelled => {
                if self.identity.is_some()
                    || self.disposition.is_some()
                    || self.start.is_some()
                    || self.namespace.is_some()
                    || self.bytes.is_some()
                {
                    return Err(field_error(DecodeCause::UnknownField));
                }
                Ok(Frame::Terminal(OuterTerminalReply::Cancelled {
                    authority,
                    peaks: required(self.peaks)?,
                    stop: required(self.stop)?,
                }))
            }
            ReplyKind::Committed => {
                if self.start.is_some() || self.namespace.is_some() || self.bytes.is_some() {
                    return Err(field_error(DecodeCause::UnknownField));
                }
                let stop = required(self.stop)?;
                match required(self.disposition)? {
                    Disposition::Report => {
                        if self.identity.is_some() {
                            return Err(field_error(DecodeCause::UnknownField));
                        }
                        Ok(Frame::Terminal(OuterTerminalReply::Committed {
                            authority,
                            peaks: required(self.peaks)?,
                            stop,
                            disposition: TerminalDisposition::Report,
                        }))
                    }
                    Disposition::OperationalFailure {
                        operation,
                        state,
                        representation,
                    } => {
                        if self.peaks.is_some() {
                            return Err(field_error(DecodeCause::UnknownField));
                        }
                        Ok(Frame::Failure(FailureHeader {
                            identity: required(self.identity)?,
                            authority,
                            stop,
                            operation,
                            state,
                            representation,
                        }))
                    }
                    Disposition::OwnerStop { cause } => {
                        if self.identity.is_some() {
                            return Err(field_error(DecodeCause::UnknownField));
                        }
                        Ok(Frame::Startup(OuterStartupControl::OwnerStop {
                            authority,
                            peaks: required(self.peaks)?,
                            stop,
                            cause,
                        }))
                    }
                    Disposition::SetupRefused { failure } => {
                        if self.identity.is_some() {
                            return Err(field_error(DecodeCause::UnknownField));
                        }
                        Ok(Frame::Startup(OuterStartupControl::SetupRefused {
                            authority,
                            peaks: required(self.peaks)?,
                            stop,
                            failure,
                        }))
                    }
                }
            }
        }
    }
}

/// One exact typed union walk, with no selector skip that precedes a required cause visitor.
/// Same original framer supplies this bounded payload; authentication/state remain actor duties.
fn frame(
    payload: &[u8],
    context: &mut PreparedStartupContext,
    scratch: &mut Scratch,
) -> Result<Frame, ControlError> {
    context.begin_metadata_frame();
    let mut parse = || -> Result<Frame, DecodeError> {
        let mut decoder = Decoder::new(payload, scratch)?;
        let mut object = decoder.begin_object()?;
        let mut fields = ReplyFields::default();
        macro_rules! once {
            ($slot:expr, $value:expr) => {{
                if $slot.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                $slot = Some($value?);
            }};
        }
        while let Some(name) = decoder.next_field(&mut object)? {
            match ReplyField::from_text(name)? {
                ReplyField::Kind => {
                    once!(
                        fields.kind,
                        ReplyKind::metadata_text(decoder.unit_variant()?)
                            .ok_or_else(|| field_error(DecodeCause::InvalidValue))
                    )
                }
                ReplyField::Identity => {
                    once!(fields.identity, role_scalar_decode::identity(&mut decoder))
                }
                ReplyField::Authority => once!(
                    fields.authority,
                    role_scalar_decode::authority(&mut decoder)
                ),
                ReplyField::Stop => once!(fields.stop, role_scalar_decode::stop(&mut decoder)),
                ReplyField::Disposition => once!(
                    fields.disposition,
                    outer_failure_decode::disposition(&mut decoder, context)
                ),
                ReplyField::Start => once!(fields.start, decoder.unsigned()),
                ReplyField::Namespace => once!(
                    fields.namespace,
                    role_control_scalar_decode::namespace(&mut decoder)
                ),
                ReplyField::Peaks => once!(
                    fields.peaks,
                    role_control_scalar_decode::peaks(&mut decoder)
                ),
                ReplyField::Bytes => once!(fields.bytes, decoder.unsigned()),
            }
        }
        let reply = fields.finish()?;
        decoder.finish()?;
        Ok(reply)
    };
    let result = parse();
    outer_failure_decode::checked(result, context).map_err(|source| context.grammar_error(source))
}

fn unexpected_frame() -> ControlError {
    ControlError::InvalidGrammar(field_error(DecodeCause::InvalidValue))
}

/// Original startup subset only. Complete cancellation/report data cannot advance a phase.
pub(super) fn decode(
    payload: &[u8],
    context: &mut PreparedStartupContext,
    scratch: &mut Scratch,
) -> Result<OuterReply, ControlError> {
    match frame(payload, context, scratch)? {
        Frame::Startup(reply) => Ok(OuterReply::Startup(reply)),
        Frame::Failure(header) => Ok(OuterReply::Failure(header)),
        Frame::Terminal(_) => Err(unexpected_frame()),
    }
}

fn cancellation(frame: Frame) -> Result<CancellationHeader, ControlError> {
    match frame {
        Frame::Terminal(OuterTerminalReply::Cancelled {
            authority,
            peaks,
            stop,
        }) => Ok(CancellationHeader::Cancelled {
            authority,
            peaks,
            stop,
        }),
        Frame::Startup(OuterStartupControl::OwnerStop {
            authority,
            peaks,
            stop,
            cause,
        }) => Ok(CancellationHeader::OwnerStop {
            authority,
            peaks,
            stop,
            cause,
        }),
        Frame::Startup(
            OuterStartupControl::Phase(_) | OuterStartupControl::SetupRefused { .. },
        )
        | Frame::Failure(_)
        | Frame::Terminal(
            OuterTerminalReply::ReportDescriptor { .. } | OuterTerminalReply::Committed { .. },
        ) => Err(unexpected_frame()),
    }
}

/// Exact cancellation receipt/real owner-stop data; never report/setup/phase authority.
pub(super) fn cancellation_commit(
    payload: &[u8],
    context: &mut PreparedStartupContext,
    scratch: &mut Scratch,
) -> Result<CancellationHeader, ControlError> {
    cancellation(frame(payload, context, scratch)?)
}

/// Queued complete phase fields are decoded solely for the caller's cleanup state.
pub(super) fn cancellation_progress(
    payload: &[u8],
    context: &mut PreparedStartupContext,
    scratch: &mut Scratch,
) -> Result<CancellationProgress, ControlError> {
    match frame(payload, context, scratch)? {
        Frame::Startup(OuterStartupControl::Phase(phase)) => Ok(CancellationProgress::Phase(phase)),
        other => cancellation(other).map(CancellationProgress::Terminal),
    }
}

/// Report read begins only with an actual descriptor or genuine owner-stop transaction.
pub(super) fn report_start(
    payload: &[u8],
    context: &mut PreparedStartupContext,
    scratch: &mut Scratch,
) -> Result<ReportStartHeader, ControlError> {
    match frame(payload, context, scratch)? {
        Frame::Terminal(OuterTerminalReply::ReportDescriptor { authority, bytes }) => {
            Ok(ReportStartHeader::Descriptor { authority, bytes })
        }
        Frame::Startup(OuterStartupControl::OwnerStop {
            authority,
            peaks,
            stop,
            cause,
        }) => Ok(ReportStartHeader::OwnerStop {
            authority,
            peaks,
            stop,
            cause,
        }),
        Frame::Startup(
            OuterStartupControl::Phase(_) | OuterStartupControl::SetupRefused { .. },
        )
        | Frame::Failure(_)
        | Frame::Terminal(
            OuterTerminalReply::Cancelled { .. } | OuterTerminalReply::Committed { .. },
        ) => Err(unexpected_frame()),
    }
}

/// Report commit or real owner-stop only; neither cancellation nor setup refusal is a report.
pub(super) fn terminal_commit(
    payload: &[u8],
    context: &mut PreparedStartupContext,
    scratch: &mut Scratch,
) -> Result<OuterTerminalReply, ControlError> {
    match frame(payload, context, scratch)? {
        Frame::Terminal(
            reply @ OuterTerminalReply::Committed {
                disposition: TerminalDisposition::Report,
                ..
            },
        ) => Ok(reply),
        Frame::Startup(OuterStartupControl::OwnerStop {
            authority,
            peaks,
            stop,
            cause,
        }) => Ok(OuterTerminalReply::Committed {
            authority,
            peaks,
            stop,
            disposition: TerminalDisposition::OwnerStop { cause },
        }),
        Frame::Startup(
            OuterStartupControl::Phase(_) | OuterStartupControl::SetupRefused { .. },
        )
        | Frame::Failure(_)
        | Frame::Terminal(
            OuterTerminalReply::Cancelled { .. }
            | OuterTerminalReply::ReportDescriptor { .. }
            | OuterTerminalReply::Committed {
                disposition:
                    TerminalDisposition::OwnerStop { .. } | TerminalDisposition::SetupRefused { .. },
                ..
            },
        ) => Err(unexpected_frame()),
    }
}

/// Fixed union schema records only; primitive, scalar, cause and shared disposition charges are separate.
pub(super) fn decode_bytes() -> Result<u64, ControlError> {
    let terms = [
        size_of::<ReplyFields>(),
        size_of::<ReplyKind>(),
        size_of::<ReplyField>(),
        size_of::<Frame>(),
        size_of::<OuterReply>(),
        size_of::<Result<Frame, DecodeError>>(),
        size_of::<CancellationHeader>(),
        size_of::<CancellationProgress>(),
        size_of::<ReportStartHeader>(),
        size_of::<OuterTerminalReply>(),
        size_of::<Result<OuterReply, ControlError>>(),
        size_of::<Result<CancellationHeader, ControlError>>(),
        size_of::<Result<CancellationProgress, ControlError>>(),
        size_of::<Result<ReportStartHeader, ControlError>>(),
        size_of::<Result<OuterTerminalReply, ControlError>>(),
        size_of::<ObjectState>(),
        size_of::<Text<'static>>(),
        size_of::<FailureState>(),
        size_of::<FailureRepresentation>(),
    ];
    let mut total =
        u64::try_from(size_of_val(&terms)).map_err(|_| ControlError::EncodedBytesExceeded)?;
    for term in terms {
        total = total
            .checked_add(u64::try_from(term).map_err(|_| ControlError::EncodedBytesExceeded)?)
            .ok_or(ControlError::EncodedBytesExceeded)?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    // Protocol selection primitives only, not real role/authentication/cleanup acceptance.
    use std::io;

    use super::*;
    use crate::kani::run::{
        cross_role_cause::{CauseIntegrityPredicate, CauseOperation},
        outer_failure::{FailureRepresentation, NegativeCommit},
        outer_setup::NamespaceIdentity,
        protocol::{current_build_identity, RunAuthority},
        resource_ledger::MeasuredPeaks,
        role_deadline::{StopOrigin, StopStamp},
        role_protocol::{OuterPhaseReply, OuterTerminalReply, OwnerStopCause, TerminalDisposition},
        startup_envelope::PolicyFailureCause,
    };

    fn parse(
        payload: &[u8],
        context: &mut PreparedStartupContext,
    ) -> Result<OuterReply, ControlError> {
        decode(
            payload,
            context,
            &mut super::super::guardian_decode::Scratch::default(),
        )
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn single_union_walk_keeps_earlier_unknown_kind_before_later_packet_faults() {
        use crate::kani::run::guardian_decode::{DecodeCause, DecodeError};

        let header = failure();
        let valid = serde_json::to_string(&NegativeCommit::new(header, &[])).unwrap();
        let original = match header.representation {
            FailureRepresentation::Original { cause } => cause,
            FailureRepresentation::Integrity { .. } => panic!("expected original cause"),
        };
        let cause_json = serde_json::to_string(&original).unwrap();
        let mut unknown_value = serde_json::to_value(original).unwrap();
        unknown_value["Io"]["kind"] = "UnknownKind".into();
        let unknown = valid.replace(&cause_json, &serde_json::to_string(&unknown_value).unwrap());
        assert_ne!(unknown, valid);
        for packet in [
            unknown.replace("\"context\":[]", "\"context\":["),
            format!("{unknown} trailing"),
        ] {
            let mut context = PreparedStartupContext::new(0).unwrap();
            let error = parse(packet.as_bytes(), &mut context)
                .err()
                .expect("invalid kind was accepted");
            let ControlError::CauseMetadataGrammar { predicate, source } = &error else {
                panic!("actual first cause checking fact was replaced by a selector error")
            };
            assert_eq!(*predicate, CauseIntegrityPredicate::UnknownKindMetadata);
            assert_eq!(source.cause(), DecodeCause::InvalidValue);
            assert!(std::ptr::eq(
                source,
                std::error::Error::source(&error)
                    .unwrap()
                    .downcast_ref::<DecodeError>()
                    .unwrap()
            ));
        }
    }

    fn decode_outer_startup(payload: &[u8]) -> Result<OuterStartupControl, ControlError> {
        super::super::role_protocol::decode_outer_startup(
            payload,
            &mut PreparedStartupContext::new(0).unwrap(),
            &mut Scratch::default(),
        )
    }

    fn authority() -> RunAuthority {
        serde_json::from_value(serde_json::to_value([0_u8; 32]).unwrap()).unwrap()
    }

    fn failure() -> FailureHeader {
        let mut context = PreparedStartupContext::new(0).unwrap();
        FailureHeader {
            identity: current_build_identity(),
            authority: authority(),
            stop: StopStamp::capture(StopOrigin::Outer).unwrap(),
            operation: CauseOperation::ProcSetup,
            state: super::outer_failure::FailureState {
                observation_admitted: false,
                original_work_expired: false,
            },
            representation: FailureRepresentation::Original {
                cause: context
                    .capture_io(&io::Error::from_raw_os_error(1))
                    .unwrap(),
            },
        }
    }

    fn refuses(value: &serde_json::Value) {
        let mut context = PreparedStartupContext::new(16).unwrap();
        assert!(parse(&serde_json::to_vec(value).unwrap(), &mut context).is_err());
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn existing_phase_and_negative_commit_preserve_their_own_rights_and_clock_policy() {
        let authority = authority();
        // Pure schema identity, not a claimed real namespace or process capability.
        let namespace: NamespaceIdentity =
            serde_json::from_value(serde_json::json!({"device": 7, "inode": 11})).unwrap();
        for phase in [
            OuterPhaseReply::MonitorSpawned { authority },
            OuterPhaseReply::InnerClaimed {
                authority,
                start: 17,
                namespace,
            },
            OuterPhaseReply::GateReleased { authority },
        ] {
            let bytes = serde_json::to_vec(&phase).unwrap();
            let mut context = PreparedStartupContext::new(0).unwrap();
            let decoded = parse(&bytes, &mut context).unwrap();
            assert_eq!(decoded.rights_count(), phase.rights_count());
            assert!(!decoded.clock_only());
            let OuterReply::Startup(OuterStartupControl::Phase(actual)) = decoded else {
                panic!("phase became negative clock data");
            };
            match (phase, actual) {
                (
                    OuterPhaseReply::MonitorSpawned { authority },
                    OuterPhaseReply::MonitorSpawned { authority: actual },
                )
                | (
                    OuterPhaseReply::GateReleased { authority },
                    OuterPhaseReply::GateReleased { authority: actual },
                ) => assert_eq!(authority, actual),
                (
                    OuterPhaseReply::InnerClaimed {
                        authority,
                        start,
                        namespace,
                    },
                    OuterPhaseReply::InnerClaimed {
                        authority: actual,
                        start: actual_start,
                        namespace: actual_namespace,
                    },
                ) => {
                    assert_eq!(authority, actual);
                    assert_eq!(start, actual_start);
                    assert_eq!(namespace, actual_namespace);
                }
                _ => panic!("selected phase changed kind"),
            }
        }
        let stop = StopStamp::capture(StopOrigin::Outer).unwrap();
        for disposition in [
            TerminalDisposition::OwnerStop {
                cause: OwnerStopCause::TimedOut,
            },
            TerminalDisposition::SetupRefused {
                failure: PolicyFailureCause::ProtectionUnverified,
            },
        ] {
            let bytes = serde_json::to_vec(&OuterTerminalReply::Committed {
                authority,
                peaks: MeasuredPeaks {
                    tree_rss_bytes: 17,
                    charged_bytes: 31,
                },
                stop,
                disposition,
            })
            .unwrap();
            let mut context = PreparedStartupContext::new(0).unwrap();
            let decoded = parse(&bytes, &mut context).unwrap();
            assert_eq!(decoded.rights_count(), 0);
            assert!(decoded.clock_only());
            match decoded {
                OuterReply::Startup(OuterStartupControl::OwnerStop {
                    authority: actual,
                    peaks,
                    stop: actual_stop,
                    cause,
                }) => {
                    assert_eq!(actual, authority);
                    assert_eq!(actual_stop, stop);
                    assert_eq!(peaks.charged_bytes, 31);
                    assert_eq!(cause, OwnerStopCause::TimedOut);
                }
                OuterReply::Startup(OuterStartupControl::SetupRefused {
                    authority: actual,
                    peaks,
                    stop: actual_stop,
                    failure,
                }) => {
                    assert_eq!(actual, authority);
                    assert_eq!(actual_stop, stop);
                    assert_eq!(peaks.charged_bytes, 31);
                    assert_eq!(failure, PolicyFailureCause::ProtectionUnverified);
                }
                OuterReply::Startup(OuterStartupControl::Phase(_)) | OuterReply::Failure(_) => {
                    panic!("measured negative changed domain")
                }
            }
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn full_union_preserves_owning_unit_maps_and_nullable_opposite_stop_members() {
        let authority = authority();
        let peaks = MeasuredPeaks {
            tree_rss_bytes: 17,
            charged_bytes: 31,
        };
        let stop = StopStamp::capture(StopOrigin::Outer).unwrap();
        for (disposition, opposite) in [
            (
                TerminalDisposition::OwnerStop {
                    cause: OwnerStopCause::TimedOut,
                },
                "failure",
            ),
            (
                TerminalDisposition::SetupRefused {
                    failure: PolicyFailureCause::ProtectionUnverified,
                },
                "cause",
            ),
        ] {
            let mut packet = serde_json::to_value(OuterTerminalReply::Committed {
                authority,
                peaks,
                stop,
                disposition,
            })
            .unwrap();
            // The measured former scalar/Option grammar admits these forms. Its obsolete
            // serde body has now been removed; this unit exercises the selected fixed parser,
            // while the owning tag derive separately remains the unit-map grammar oracle.
            packet["kind"] = serde_json::json!({"Committed": null});
            let name = packet["disposition"]["kind"].as_str().unwrap().to_owned();
            packet["disposition"]["kind"] = serde_json::json!({name: null});
            packet["disposition"][opposite] = serde_json::Value::Null;
            assert!(matches!(
                serde_json::from_value::<ReplyKind>(packet["kind"].clone()).unwrap(),
                ReplyKind::Committed
            ));
            let bytes = serde_json::to_vec(&packet).unwrap();
            let mut context = PreparedStartupContext::new(0).unwrap();
            let new = parse(&bytes, &mut context).unwrap();
            match (disposition, new) {
                (
                    TerminalDisposition::OwnerStop { cause },
                    OuterReply::Startup(OuterStartupControl::OwnerStop {
                        cause: actual,
                        authority: actual_authority,
                        peaks: actual_peaks,
                        stop: actual_stop,
                    }),
                ) => {
                    assert_eq!(actual, cause);
                    assert_eq!(actual_authority, authority);
                    assert_eq!(actual_peaks, peaks);
                    assert_eq!(actual_stop, stop);
                }
                (
                    TerminalDisposition::SetupRefused { failure },
                    OuterReply::Startup(OuterStartupControl::SetupRefused {
                        failure: actual,
                        authority: actual_authority,
                        peaks: actual_peaks,
                        stop: actual_stop,
                    }),
                ) => {
                    assert_eq!(actual, failure);
                    assert_eq!(actual_authority, authority);
                    assert_eq!(actual_peaks, peaks);
                    assert_eq!(actual_stop, stop);
                }
                _ => panic!("original admitted stop changed branch"),
            }
            // A unit map with a non-null body is not an alternate representation.
            packet["kind"] = serde_json::json!({"Committed": false});
            let bytes = serde_json::to_vec(&packet).unwrap();
            assert!(decode_outer_startup(&bytes).is_err());
            assert!(parse(&bytes, &mut context).is_err());
            // Duplicate explicit-null Option fields must not disappear as duplicate absence.
            packet["kind"] = serde_json::json!("Committed");
            let bytes = serde_json::to_string(&packet).unwrap();
            let member = format!("\"{opposite}\":null");
            let doubled = bytes.replacen(&member, &format!("{member},{member}"), 1);
            assert_ne!(doubled, bytes);
            assert!(decode_outer_startup(doubled.as_bytes()).is_err());
            assert!(parse(doubled.as_bytes(), &mut context).is_err());
            assert!(outer_failure_decode::decode(
                &serde_json::to_vec(&packet).unwrap(),
                &mut Scratch::default(),
                &mut context
            )
            .is_err());
        }
    }

    /// Trace: FR-034-AC-15, FR-034-AC-39
    #[test]
    fn receipts_without_context_preserve_the_pending_original_policy_diagnostic() {
        let mut context = PreparedStartupContext::new(64).unwrap();
        let original = io::Error::new(io::ErrorKind::PermissionDenied, "actual I policy context");
        context.capture_io(&original).unwrap();
        let reserved = context.reserved_bytes();
        let authority = authority();
        let stop = StopStamp::capture(StopOrigin::Backend).unwrap();
        let peaks = MeasuredPeaks {
            tree_rss_bytes: 17,
            charged_bytes: 31,
        };
        let refused = serde_json::to_vec(&OuterTerminalReply::Committed {
            authority,
            peaks,
            stop,
            disposition: TerminalDisposition::SetupRefused {
                failure: PolicyFailureCause::ProtectionUnverified,
            },
        })
        .unwrap();
        *context.metadata_fault_slot() = Some(CauseIntegrityPredicate::MalformedCauseMetadata);
        let decoded = parse(&refused, &mut context).unwrap();
        assert!(matches!(
            decoded,
            OuterReply::Startup(OuterStartupControl::SetupRefused { .. })
        ));
        assert_eq!(context.context(), original.to_string());
        assert_eq!(context.reserved_bytes(), reserved);
        assert!(context.metadata_fault_slot().is_none());
        let cancelled = serde_json::to_vec(&OuterTerminalReply::Cancelled {
            authority,
            peaks,
            stop,
        })
        .unwrap();
        let actual =
            cancellation_commit(&cancelled, &mut context, &mut Scratch::default()).unwrap();
        assert!(matches!(actual, CancellationHeader::Cancelled { .. }));
        assert_eq!(context.context(), original.to_string());
        assert_eq!(context.reserved_bytes(), reserved);
        assert!(context.metadata_fault_slot().is_none());
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn operational_branches_are_provisional_and_stay_out_of_ordinary_decoders() {
        let original = failure();
        for header in [
            original,
            FailureHeader {
                representation: FailureRepresentation::Integrity {
                    predicate: CauseIntegrityPredicate::MalformedCauseMetadata,
                },
                ..original
            },
        ] {
            let bytes = serde_json::to_vec(&NegativeCommit::new(header, &[255])).unwrap();
            let mut context = PreparedStartupContext::new(1).unwrap();
            let capacity = context.reserved_bytes();
            let decoded = parse(&bytes, &mut context).unwrap();
            assert_eq!(decoded.rights_count(), 0);
            assert!(decoded.clock_only());
            let OuterReply::Failure(actual) = decoded else {
                panic!("failure became phase data");
            };
            assert_eq!(actual, header);
            assert!(context.context().is_empty());
            assert_eq!(context.reserved_bytes(), capacity);
            assert!(decode_outer_startup(&bytes).is_err());
            assert!(crate::kani::run::role_protocol::decode_report_start(
                &bytes,
                &mut context,
                &mut Scratch::default()
            )
            .is_err());
            assert!(crate::kani::run::role_protocol::decode_terminal_commit(
                &bytes,
                &mut context,
                &mut Scratch::default()
            )
            .is_err());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn selector_and_chosen_schema_reject_missing_null_unknown_duplicate_and_mixed_fields() {
        let original = serde_json::to_value(NegativeCommit::new(failure(), &[])).unwrap();
        for nested in [false, true] {
            for replacement in [
                None,
                Some(serde_json::Value::Null),
                Some(serde_json::json!(true)),
                Some(serde_json::json!("unknown")),
            ] {
                let mut changed = original.clone();
                let object = if nested {
                    &mut changed["disposition"]
                } else {
                    &mut changed
                };
                match replacement {
                    Some(replacement) => {
                        object
                            .as_object_mut()
                            .unwrap()
                            .insert("kind".into(), replacement);
                    }
                    None => {
                        object.as_object_mut().unwrap().remove("kind");
                    }
                }
                refuses(&changed);
            }
            let tag = if nested {
                "OperationalFailure"
            } else {
                "Committed"
            };
            let encoded = serde_json::to_string(&original).unwrap();
            let member = format!("\"kind\":\"{tag}\"");
            let duplicate = encoded.replacen(&member, &format!("{member},{member}"), 1);
            assert_ne!(duplicate, encoded);
            let mut context = PreparedStartupContext::new(0).unwrap();
            assert!(parse(duplicate.as_bytes(), &mut context).is_err());
        }
        for replacement in [
            None,
            Some(serde_json::Value::Null),
            Some(serde_json::json!(1)),
        ] {
            let mut changed = original.clone();
            match replacement {
                Some(replacement) => {
                    changed["disposition"] = replacement;
                }
                None => {
                    changed.as_object_mut().unwrap().remove("disposition");
                }
            }
            refuses(&changed);
        }
        let encoded = serde_json::to_string(&original).unwrap();
        let member = format!("\"disposition\":{}", original["disposition"]);
        let duplicate = encoded.replacen(&member, &format!("{member},{member}"), 1);
        assert_ne!(duplicate, encoded);
        let mut context = PreparedStartupContext::new(0).unwrap();
        assert!(parse(duplicate.as_bytes(), &mut context).is_err());
        for tag in [
            "Report",
            "ReportDescriptor",
            "Cancelled",
            "Ready",
            "Dispatched",
        ] {
            let mut changed = original.clone();
            changed["kind"] = serde_json::json!(tag);
            refuses(&changed);
            let mut changed = original.clone();
            changed["disposition"]["kind"] = serde_json::json!(tag);
            refuses(&changed);
        }
        for field in ["peaks", "report_bytes", "unknown"] {
            let mut changed = original.clone();
            changed[field] = serde_json::json!(0);
            refuses(&changed);
        }
        let mut conflict = original.clone();
        conflict["disposition"]["kind"] = serde_json::json!("OwnerStop");
        refuses(&conflict);
        let encoded = serde_json::to_vec(&original).unwrap();
        let mut trailing = encoded;
        trailing.extend_from_slice(b"{}");
        let mut context = PreparedStartupContext::new(0).unwrap();
        assert!(parse(&trailing, &mut context).is_err());
    }
}

//! O's one charged C-control receive cursor across phase, close and report-read controls.
//!
//! This parser grants no actor transition. Its caller authenticates the retained original creator,
//! run, stop origin and original cutoff, and performs normal accounting before every finite step.
//! Partial frames remain in this same strict receiver; no phase selects another stream cursor.

use std::time::Instant;

use super::{
    control::{ControlError, IncrementalReceive, PreparedReceived, RoleEndpoint},
    guardian_decode::{DecodeCause, DecodeError, DecodeSite, Decoder, Scratch, Text},
    protocol::RunAuthority,
    role_deadline::{RoleDeadline, StopStamp},
    role_protocol::{CallerTerminalControl, OuterPhaseCommand},
    role_scalar_decode,
};

/// Parsed alternatives only; the actual actor rejects controls inappropriate for its state.
pub(super) enum OuterCallerControl {
    Phase(OuterPhaseCommand),
    /// Contains only CompletedClose or CancelClose, never ReadCompleted.
    Close(CallerTerminalControl),
    ReadCompleted {
        authority: RunAuthority,
        bytes: u64,
    },
}

/// Exactly one original incremental receive allocation, with irreversible transport refusal.
pub(super) struct OuterCallerReceive {
    receive: IncrementalReceive,
    poisoned: bool,
    scratch: Scratch,
}

impl OuterCallerReceive {
    pub(super) fn prepare() -> Result<Self, ControlError> {
        Ok(Self {
            receive: IncrementalReceive::prepare()?,
            poisoned: false,
            scratch: Scratch::default(),
        })
    }

    /// One bounded nonblocking syscall at most. None preserves the original never-elapsing
    /// cutoff; a supplied finite cutoff is used unchanged. Existing strict EOF/ancillary rules
    /// stay active, and complete parsed controls still require actor-owned authorization.
    pub(super) fn step(
        &mut self,
        caller: &RoleEndpoint,
        cutoff: Option<Instant>,
    ) -> Result<Option<OuterCallerControl>, ControlError> {
        if self.poisoned {
            return Err(ControlError::ProgressPoisoned);
        }
        self.poisoned = true;
        let received = self.receive.advance_decode_optional(
            &caller.transport(),
            |_| 0,
            cutoff,
            |payload| decode_control(payload, &mut self.scratch),
        )?;
        let control = match received {
            Some(received) => {
                // Exclusive creator authentication uses the retained safe endpoint/creator
                // guard outside this module. Missing SCM credentials are not missing authority;
                // an unexpected supplied sender credential is still a transport refusal.
                if received.credentials.is_some() {
                    return Err(ControlError::UnexpectedCredentials);
                }
                Some(received.control)
            }
            None => None,
        };
        self.poisoned = false;
        Ok(control)
    }

    /// Actual unread frame progress only, retained even after a refusal.
    pub(super) fn has_partial_frame(&self) -> bool {
        self.receive.has_partial_frame()
    }

    /// Existing receive payload/right reservation plus actual fixed wrapper and scalar decoder
    /// records/results. Resident scratch is included in Self exactly once; primitive transient
    /// records exclude that resident term. Initializer/compiled native-stack highwater remains
    /// an integration measurement obligation, rather than a claim from these layout sums.
    pub(super) fn reserved_bytes(&self) -> Result<u64, ControlError> {
        let primitive =
            super::guardian_decode::decode_bytes().map_err(ControlError::InvalidGrammar)?;
        let resident_scratch = u64::try_from(std::mem::size_of::<Scratch>())
            .map_err(|_| ControlError::EncodedBytesExceeded)?;
        let transient = primitive
            .checked_sub(resident_scratch)
            .ok_or(ControlError::EncodedBytesExceeded)?;
        let nested = role_scalar_decode::decode_bytes().map_err(ControlError::InvalidGrammar)?;
        let fixed = std::mem::size_of::<Self>()
            .checked_sub(std::mem::size_of::<IncrementalReceive>())
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<ControlFields>()))
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<ControlField>()))
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<Option<OuterCallerControl>>()))
            .and_then(|bytes| {
                bytes.checked_add(std::mem::size_of::<
                    Option<PreparedReceived<'static, OuterCallerControl>>,
                >())
            })
            .and_then(|bytes| u64::try_from(bytes).ok())
            .and_then(|bytes| bytes.checked_add(transient))
            .ok_or(ControlError::EncodedBytesExceeded)?;
        self.receive
            .reserved_bytes()?
            .checked_add(fixed)
            .and_then(|bytes| bytes.checked_add(nested))
            .ok_or(ControlError::EncodedBytesExceeded)
    }
}

macro_rules! control_kinds {
    ($($variant:ident),+ $(,)?) => {
        #[derive(Clone, Copy)]
        enum ControlKind { $($variant),+ }
        impl ControlKind {
            fn from_text(text: Text<'_>) -> Result<Self, DecodeError> {
                $(if text.equals(stringify!($variant)) { return Ok(Self::$variant); })+
                Err(field_error(DecodeCause::InvalidValue))
            }
        }
    };
}

control_kinds! { BeginMonitor, ClaimInner, ReleaseGate, CompletedClose, CancelClose, ReadCompleted }

#[derive(Clone, Copy)]
enum ControlField {
    Kind,
    Authority,
    Deadline,
    Stop,
    Bytes,
}

impl ControlField {
    fn from_text(text: Text<'_>) -> Result<Self, DecodeError> {
        if text.equals("kind") {
            Ok(Self::Kind)
        } else if text.equals("authority") {
            Ok(Self::Authority)
        } else if text.equals("deadline") {
            Ok(Self::Deadline)
        } else if text.equals("stop") {
            Ok(Self::Stop)
        } else if text.equals("bytes") {
            Ok(Self::Bytes)
        } else {
            Err(field_error(DecodeCause::UnknownField))
        }
    }
}

#[derive(Default)]
struct ControlFields {
    kind: Option<ControlKind>,
    authority: Option<RunAuthority>,
    deadline: Option<RoleDeadline>,
    stop: Option<StopStamp>,
    bytes: Option<u64>,
}

fn field_error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}

fn required<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| field_error(DecodeCause::MissingField))
}

impl ControlFields {
    fn finish(self) -> Result<OuterCallerControl, DecodeError> {
        let kind = required(self.kind)?;
        let authority = required(self.authority)?;
        match kind {
            ControlKind::BeginMonitor | ControlKind::ClaimInner | ControlKind::ReleaseGate => {
                if self.deadline.is_some() || self.stop.is_some() || self.bytes.is_some() {
                    return Err(field_error(DecodeCause::UnknownField));
                }
                let phase = match kind {
                    ControlKind::BeginMonitor => OuterPhaseCommand::BeginMonitor { authority },
                    ControlKind::ClaimInner => OuterPhaseCommand::ClaimInner { authority },
                    ControlKind::ReleaseGate => OuterPhaseCommand::ReleaseGate { authority },
                    ControlKind::CompletedClose
                    | ControlKind::CancelClose
                    | ControlKind::ReadCompleted => {
                        return Err(field_error(DecodeCause::InvalidValue))
                    }
                };
                Ok(OuterCallerControl::Phase(phase))
            }
            ControlKind::CompletedClose | ControlKind::CancelClose => {
                if self.bytes.is_some() {
                    return Err(field_error(DecodeCause::UnknownField));
                }
                let deadline = required(self.deadline)?;
                let stop = required(self.stop)?;
                let close = match kind {
                    ControlKind::CompletedClose => CallerTerminalControl::CompletedClose {
                        authority,
                        deadline,
                        stop,
                    },
                    ControlKind::CancelClose => CallerTerminalControl::CancelClose {
                        authority,
                        deadline,
                        stop,
                    },
                    ControlKind::BeginMonitor
                    | ControlKind::ClaimInner
                    | ControlKind::ReleaseGate
                    | ControlKind::ReadCompleted => {
                        return Err(field_error(DecodeCause::InvalidValue))
                    }
                };
                Ok(OuterCallerControl::Close(close))
            }
            ControlKind::ReadCompleted => {
                if self.deadline.is_some() || self.stop.is_some() {
                    return Err(field_error(DecodeCause::UnknownField));
                }
                Ok(OuterCallerControl::ReadCompleted {
                    authority,
                    bytes: required(self.bytes)?,
                })
            }
        }
    }
}

/// Strict single schema walk; no selector pass or ignored-value prevalidation precedes it.
fn decode_control(
    payload: &[u8],
    scratch: &mut Scratch,
) -> Result<OuterCallerControl, ControlError> {
    let mut parse = || -> Result<OuterCallerControl, DecodeError> {
        let mut decoder = Decoder::new(payload, scratch)?;
        let mut object = decoder.begin_object()?;
        let mut fields = ControlFields::default();
        while let Some(name) = decoder.next_field(&mut object)? {
            match ControlField::from_text(name)? {
                ControlField::Kind => {
                    if fields.kind.is_some() {
                        return Err(field_error(DecodeCause::DuplicateField));
                    }
                    fields.kind = Some(ControlKind::from_text(decoder.string()?)?);
                }
                ControlField::Authority => {
                    if fields.authority.is_some() {
                        return Err(field_error(DecodeCause::DuplicateField));
                    }
                    fields.authority = Some(role_scalar_decode::authority(&mut decoder)?);
                }
                ControlField::Deadline => {
                    if fields.deadline.is_some() {
                        return Err(field_error(DecodeCause::DuplicateField));
                    }
                    fields.deadline = Some(role_scalar_decode::deadline(&mut decoder)?);
                }
                ControlField::Stop => {
                    if fields.stop.is_some() {
                        return Err(field_error(DecodeCause::DuplicateField));
                    }
                    fields.stop = Some(role_scalar_decode::stop(&mut decoder)?);
                }
                ControlField::Bytes => {
                    if fields.bytes.is_some() {
                        return Err(field_error(DecodeCause::DuplicateField));
                    }
                    fields.bytes = Some(decoder.unsigned()?);
                }
            }
        }
        let control = fields.finish()?;
        decoder.finish()?;
        Ok(control)
    };
    parse().map_err(ControlError::InvalidGrammar)
}

#[cfg(test)]
mod tests {
    use super::super::{
        control::role_pair,
        role_deadline::{RoleDeadline, StopOrigin, StopStamp},
    };
    use super::*;

    fn parse_control(payload: &[u8]) -> Result<OuterCallerControl, ControlError> {
        decode_control(payload, &mut Scratch::default())
    }

    /// Trace: FR-034-AC-15, FR-034-AC-33, FR-034-AC-34, FR-034-AC-38.
    #[test]
    fn union_decoder_preserves_each_exact_phase_close_and_read_kind() {
        let authority = RunAuthority::fresh().unwrap();
        for (command, expected) in [
            (OuterPhaseCommand::BeginMonitor { authority }, 0),
            (OuterPhaseCommand::ClaimInner { authority }, 1),
            (OuterPhaseCommand::ReleaseGate { authority }, 2),
        ] {
            let original = serde_json::to_value(command).unwrap();
            let OuterCallerControl::Phase(phase) =
                parse_control(&serde_json::to_vec(&original).unwrap()).unwrap()
            else {
                panic!("phase changed branch");
            };
            assert_eq!(phase.authority(), authority);
            let actual = match phase {
                OuterPhaseCommand::BeginMonitor { .. } => 0,
                OuterPhaseCommand::ClaimInner { .. } => 1,
                OuterPhaseCommand::ReleaseGate { .. } => 2,
            };
            assert_eq!(actual, expected);
            let mut extra = original.clone();
            extra["bytes"] = 0.into();
            assert!(parse_control(&serde_json::to_vec(&extra).unwrap()).is_err());
            for field in ["kind", "authority"] {
                let mut missing = original.clone();
                missing.as_object_mut().unwrap().remove(field);
                assert!(parse_control(&serde_json::to_vec(&missing).unwrap()).is_err());
                let mut wrong = original.clone();
                wrong[field] = serde_json::Value::Null;
                assert!(parse_control(&serde_json::to_vec(&wrong).unwrap()).is_err());
            }
        }
        let stop = StopStamp::capture(StopOrigin::Caller).unwrap();
        let deadline = RoleDeadline::from_original(Instant::now()).unwrap();
        for (control, expected_cancel) in [
            (
                CallerTerminalControl::CompletedClose {
                    authority,
                    deadline,
                    stop,
                },
                false,
            ),
            (
                CallerTerminalControl::CancelClose {
                    authority,
                    deadline,
                    stop,
                },
                true,
            ),
        ] {
            let OuterCallerControl::Close(close) =
                parse_control(&serde_json::to_vec(&control).unwrap()).unwrap()
            else {
                panic!("close changed branch");
            };
            let (actual, actual_deadline, actual_stop, cancelled) = match close {
                CallerTerminalControl::CompletedClose {
                    authority,
                    deadline,
                    stop,
                } => (authority, deadline, stop, false),
                CallerTerminalControl::CancelClose {
                    authority,
                    deadline,
                    stop,
                } => (authority, deadline, stop, true),
                CallerTerminalControl::ReadCompleted { .. } => panic!("read ACK became close"),
            };
            assert_eq!(actual, authority);
            assert_eq!(actual_deadline, deadline);
            assert_eq!(actual_stop, stop);
            assert_eq!(cancelled, expected_cancel);
        }
        let original = serde_json::to_value(CallerTerminalControl::ReadCompleted {
            authority,
            bytes: 23,
        })
        .unwrap();
        let OuterCallerControl::ReadCompleted {
            authority: actual,
            bytes,
        } = parse_control(&serde_json::to_vec(&original).unwrap()).unwrap()
        else {
            panic!("read ACK changed branch");
        };
        assert_eq!(actual, authority);
        assert_eq!(bytes, 23);
        for field in ["kind", "authority", "bytes"] {
            let mut missing = original.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(parse_control(&serde_json::to_vec(&missing).unwrap()).is_err());
            let mut wrong = original.clone();
            wrong[field] = serde_json::Value::Null;
            assert!(parse_control(&serde_json::to_vec(&wrong).unwrap()).is_err());
        }
        let mut extra = original.clone();
        extra["stop"] = serde_json::to_value(stop).unwrap();
        assert!(parse_control(&serde_json::to_vec(&extra).unwrap()).is_err());
        let encoded = serde_json::to_string(&original).unwrap();
        let duplicate = encoded.replacen("{", "{\"kind\":\"BeginMonitor\",", 1);
        assert!(parse_control(duplicate.as_bytes()).is_err());
    }

    /// Trace: FR-034-AC-15, FR-034-AC-33, FR-034-AC-34, FR-034-AC-38.
    #[test]
    fn one_original_receiver_retains_partial_frame_and_refuses_eof_without_clock_reset() {
        // Ordinary local nonblocking transport only; no role/namespace/cleanup authority claimed.
        let (sender, endpoint) = role_pair().unwrap();
        let mut receive = OuterCallerReceive::prepare().unwrap();
        assert!(receive.step(&endpoint, None).unwrap().is_none());
        assert!(!receive.has_partial_frame());
        let authority = RunAuthority::fresh().unwrap();
        let cutoff = Instant::now()
            .checked_add(std::time::Duration::from_secs(3))
            .unwrap();
        sender
            .transport()
            .send(&OuterPhaseCommand::BeginMonitor { authority }, &[], cutoff)
            .unwrap();
        assert!(receive.step(&endpoint, None).unwrap().is_none());
        assert!(receive.has_partial_frame());
        let Some(OuterCallerControl::Phase(OuterPhaseCommand::BeginMonitor { authority: actual })) =
            receive.step(&endpoint, Some(cutoff)).unwrap()
        else {
            panic!("same partial phase was not completed");
        };
        assert_eq!(actual, authority);
        assert!(!receive.has_partial_frame());
        sender
            .transport()
            .send(
                &CallerTerminalControl::ReadCompleted {
                    authority,
                    bytes: 31,
                },
                &[],
                cutoff,
            )
            .unwrap();
        assert!(receive.step(&endpoint, Some(cutoff)).unwrap().is_none());
        assert!(receive.has_partial_frame());
        drop(sender);
        assert!(matches!(
            receive.step(&endpoint, Some(cutoff)),
            Err(ControlError::Eof)
        ));
        assert!(receive.has_partial_frame());
        assert!(matches!(
            receive.step(&endpoint, None),
            Err(ControlError::ProgressPoisoned)
        ));
        let (_sender, endpoint) = role_pair().unwrap();
        let mut expired = OuterCallerReceive::prepare().unwrap();
        assert!(matches!(
            expired.step(&endpoint, Some(Instant::now())),
            Err(ControlError::Deadline)
        ));
    }
}

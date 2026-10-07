//! O's one charged C-control receive cursor across phase, close and report-read controls.
//!
//! This parser grants no actor transition. Its caller authenticates the retained original creator,
//! run, stop origin and original cutoff, and performs normal accounting before every finite step.
//! Partial frames remain in this same strict receiver; no phase selects another stream cursor.

use std::time::Instant;

use serde::Deserialize;

use super::{
    control::{ControlError, IncrementalReceive, PreparedReceived, RoleEndpoint},
    protocol::RunAuthority,
    role_protocol::{
        cancellation_decode_bytes, decode_caller_close, CallerTerminalControl, OuterPhaseCommand,
    },
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
}

impl OuterCallerReceive {
    pub(super) fn prepare() -> Result<Self, ControlError> {
        Ok(Self {
            receive: IncrementalReceive::prepare()?,
            poisoned: false,
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
            decode_control,
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
    /// records/results. No second framer, dynamic context or increased product bound is charged.
    pub(super) fn reserved_bytes(&self) -> Result<u64, ControlError> {
        let nested = cancellation_decode_bytes()?;
        let fixed = std::mem::size_of::<Self>()
            .checked_sub(std::mem::size_of::<IncrementalReceive>())
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<ControlSelector>()))
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<PhaseRecord>()))
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<ReadRecord>()))
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<Option<OuterCallerControl>>()))
            .and_then(|bytes| {
                bytes.checked_add(std::mem::size_of::<
                    Option<PreparedReceived<'static, OuterCallerControl>>,
                >())
            })
            .and_then(|bytes| u64::try_from(bytes).ok())
            .ok_or(ControlError::EncodedBytesExceeded)?;
        self.receive
            .reserved_bytes()?
            .checked_add(fixed)
            .and_then(|bytes| bytes.checked_add(nested))
            .ok_or(ControlError::EncodedBytesExceeded)
    }
}

#[derive(Deserialize)]
enum ControlKind {
    BeginMonitor,
    ClaimInner,
    ReleaseGate,
    CompletedClose,
    CancelClose,
    ReadCompleted,
}

#[derive(Deserialize)]
struct ControlSelector {
    kind: ControlKind,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PhaseRecord {
    kind: ControlKind,
    authority: RunAuthority,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadRecord {
    kind: ControlKind,
    authority: RunAuthority,
    bytes: u64,
}

fn decode_control(payload: &[u8]) -> Result<OuterCallerControl, ControlError> {
    use serde::de::Error as _;
    super::startup_cause::check_scratch_free_json(payload)
        .map_err(|error| ControlError::InvalidEncoding(serde_json::Error::custom(error)))?;
    let selector: ControlSelector =
        serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
    match selector.kind {
        ControlKind::BeginMonitor | ControlKind::ClaimInner | ControlKind::ReleaseGate => {
            let record: PhaseRecord =
                serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
            let phase = match record.kind {
                ControlKind::BeginMonitor => OuterPhaseCommand::BeginMonitor {
                    authority: record.authority,
                },
                ControlKind::ClaimInner => OuterPhaseCommand::ClaimInner {
                    authority: record.authority,
                },
                ControlKind::ReleaseGate => OuterPhaseCommand::ReleaseGate {
                    authority: record.authority,
                },
                ControlKind::CompletedClose
                | ControlKind::CancelClose
                | ControlKind::ReadCompleted => {
                    return Err(ControlError::InvalidEncoding(serde_json::Error::custom(
                        "unexpected outer phase kind",
                    )));
                }
            };
            Ok(OuterCallerControl::Phase(phase))
        }
        ControlKind::CompletedClose | ControlKind::CancelClose => {
            decode_caller_close(payload).map(OuterCallerControl::Close)
        }
        ControlKind::ReadCompleted => {
            let record: ReadRecord =
                serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
            if !matches!(record.kind, ControlKind::ReadCompleted) {
                return Err(ControlError::InvalidEncoding(serde_json::Error::custom(
                    "unexpected outer read kind",
                )));
            }
            Ok(OuterCallerControl::ReadCompleted {
                authority: record.authority,
                bytes: record.bytes,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        control::role_pair,
        role_deadline::{RoleDeadline, StopOrigin, StopStamp},
    };
    use super::*;

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
                decode_control(&serde_json::to_vec(&original).unwrap()).unwrap()
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
            assert!(decode_control(&serde_json::to_vec(&extra).unwrap()).is_err());
            for field in ["kind", "authority"] {
                let mut missing = original.clone();
                missing.as_object_mut().unwrap().remove(field);
                assert!(decode_control(&serde_json::to_vec(&missing).unwrap()).is_err());
                let mut wrong = original.clone();
                wrong[field] = serde_json::Value::Null;
                assert!(decode_control(&serde_json::to_vec(&wrong).unwrap()).is_err());
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
                decode_control(&serde_json::to_vec(&control).unwrap()).unwrap()
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
        } = decode_control(&serde_json::to_vec(&original).unwrap()).unwrap()
        else {
            panic!("read ACK changed branch");
        };
        assert_eq!(actual, authority);
        assert_eq!(bytes, 23);
        for field in ["kind", "authority", "bytes"] {
            let mut missing = original.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(decode_control(&serde_json::to_vec(&missing).unwrap()).is_err());
            let mut wrong = original.clone();
            wrong[field] = serde_json::Value::Null;
            assert!(decode_control(&serde_json::to_vec(&wrong).unwrap()).is_err());
        }
        let mut extra = original.clone();
        extra["stop"] = serde_json::to_value(stop).unwrap();
        assert!(decode_control(&serde_json::to_vec(&extra).unwrap()).is_err());
        let encoded = serde_json::to_string(&original).unwrap();
        let duplicate = encoded.replacen("{", "{\"kind\":\"BeginMonitor\",", 1);
        assert!(decode_control(duplicate.as_bytes()).is_err());
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

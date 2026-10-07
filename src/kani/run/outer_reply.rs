//! Fixed tag selection on the existing startup receive payload.
//!
//! One selected strict decoder owns the remaining schema and JSON EOF. This module owns no
//! receive cursor, sender/site authority, state transition, cutoff or settlement operation.
//! A negative reply is provisional clock/error data, never permission for phase or Dispatch.

use std::mem::size_of;

use serde::{de::Error as _, Deserialize};

use super::{
    control::ControlError,
    outer_failure::{self, FailureHeader},
    role_protocol::{decode_outer_startup, OuterStartupControl},
    startup_cause::{check_scratch_free_json, PreparedStartupContext},
};

pub(super) enum OuterReply {
    Startup(OuterStartupControl),
    Failure(FailureHeader),
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

#[derive(Clone, Copy, Deserialize)]
enum ReplyKind {
    MonitorSpawned,
    InnerClaimed,
    GateReleased,
    Committed,
}

#[derive(Clone, Copy, Deserialize)]
enum DispositionKind {
    OwnerStop,
    SetupRefused,
    OperationalFailure,
}

// Flat scalar selectors deliberately ignore other fields WITHOUT owning them. The selected
// existing strict decoder then rejects unknown/conflicting fields and enforces all field types.
#[derive(Deserialize)]
struct ReplySelector {
    kind: ReplyKind,
    disposition: Option<DispositionSelector>,
}

#[derive(Deserialize)]
struct DispositionSelector {
    kind: DispositionKind,
}

/// Select once without speculative decoding, cursor changes or a normal-reply fallback.
pub(super) fn decode(
    payload: &[u8],
    context: &mut PreparedStartupContext,
) -> Result<OuterReply, ControlError> {
    check_scratch_free_json(payload)
        .map_err(|error| ControlError::InvalidEncoding(serde_json::Error::custom(error)))?;
    let selected: ReplySelector =
        serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
    match selected.kind {
        ReplyKind::MonitorSpawned | ReplyKind::InnerClaimed | ReplyKind::GateReleased => {
            decode_outer_startup(payload).map(OuterReply::Startup)
        }
        ReplyKind::Committed => {
            let disposition = selected.disposition.ok_or_else(|| {
                ControlError::InvalidEncoding(serde_json::Error::missing_field("disposition"))
            })?;
            match disposition.kind {
                DispositionKind::OwnerStop | DispositionKind::SetupRefused => {
                    decode_outer_startup(payload).map(OuterReply::Startup)
                }
                DispositionKind::OperationalFailure => {
                    outer_failure::decode(payload, context).map(OuterReply::Failure)
                }
            }
        }
    }
}

/// New selector and result-wrapper storage only. Existing branch decoder storage is separate.
pub(super) fn decode_bytes() -> Result<u64, ControlError> {
    let branch_payload = size_of::<OuterStartupControl>().max(size_of::<FailureHeader>());
    let wrapper = size_of::<OuterReply>()
        .checked_sub(branch_payload)
        .ok_or(ControlError::EncodedBytesExceeded)?;
    let bytes = size_of::<ReplySelector>()
        .checked_add(size_of::<DispositionSelector>())
        .and_then(|bytes| bytes.checked_add(wrapper))
        .ok_or(ControlError::EncodedBytesExceeded)?;
    u64::try_from(bytes).map_err(|_| ControlError::EncodedBytesExceeded)
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
        assert!(decode(&serde_json::to_vec(value).unwrap(), &mut context).is_err());
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
            let decoded = decode(&bytes, &mut context).unwrap();
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
            let decoded = decode(&bytes, &mut context).unwrap();
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
            let decoded = decode(&bytes, &mut context).unwrap();
            assert_eq!(decoded.rights_count(), 0);
            assert!(decoded.clock_only());
            let OuterReply::Failure(actual) = decoded else {
                panic!("failure became phase data");
            };
            assert_eq!(actual, header);
            assert!(context.context().is_empty());
            assert_eq!(context.reserved_bytes(), capacity);
            assert!(decode_outer_startup(&bytes).is_err());
            assert!(crate::kani::run::role_protocol::decode_report_start(&bytes).is_err());
            assert!(crate::kani::run::role_protocol::decode_terminal_commit(&bytes).is_err());
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
            assert!(decode(duplicate.as_bytes(), &mut context).is_err());
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
        assert!(decode(duplicate.as_bytes(), &mut context).is_err());
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
        assert!(decode(&trailing, &mut context).is_err());
    }
}

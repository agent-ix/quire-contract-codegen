// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Exact trusted-installer reply grammar on one owner-held scratch/cursor (FR-034).
//!
//! Parsed headers authorize no installer, clock or result. This module preserves the existing
//! per-variant field and cause-domain checks; the actor owns authentication, original causes,
//! projection and settlement. Required context structure is independent of optional diagnostics.

use std::mem::{size_of, size_of_val};

use super::{
    guardian_decode::{DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Scratch, Text},
    outer_failure_decode,
    protocol::{BuildIdentity, RunAuthority},
    role_control_scalar_decode,
    role_deadline::StopStamp,
    role_scalar_decode,
    startup_cause::{PreparedStartupContext, StartupCause},
    startup_envelope::{InstallerReplyHeader, PolicyFailureCause, ReplyKind},
};

fn field_error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}

fn required<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| field_error(DecodeCause::MissingField))
}

#[derive(Clone, Copy)]
enum Field {
    Kind,
    Identity,
    Authority,
    Stop,
    Failure,
    Context,
}

impl Field {
    fn from_text(text: Text<'_>) -> Result<Self, DecodeError> {
        if text.equals("kind") {
            Ok(Self::Kind)
        } else if text.equals("identity") {
            Ok(Self::Identity)
        } else if text.equals("authority") {
            Ok(Self::Authority)
        } else if text.equals("stop") {
            Ok(Self::Stop)
        } else if text.equals("failure") {
            Ok(Self::Failure)
        } else if text.equals("context") {
            Ok(Self::Context)
        } else {
            Err(field_error(DecodeCause::UnknownField))
        }
    }
}

#[derive(Default)]
struct Fields {
    kind: Option<ReplyKind>,
    identity: Option<BuildIdentity>,
    authority: Option<RunAuthority>,
    stop: Option<StopStamp>,
    failure: Option<PolicyFailureCause>,
    context: Option<()>,
}

impl Fields {
    fn finish(self) -> Result<InstallerReplyHeader, DecodeError> {
        let identity = required(self.identity)?;
        let authority = required(self.authority)?;
        match required(self.kind)? {
            ReplyKind::PolicyReady => {
                if self.stop.is_some() || self.failure.is_some() || self.context.is_some() {
                    return Err(field_error(DecodeCause::UnknownField));
                }
                Ok(InstallerReplyHeader::PolicyReady {
                    identity,
                    authority,
                })
            }
            ReplyKind::Refused => {
                required(self.context)?;
                let failure = required(self.failure)?;
                // Preserve the existing ReplySeed domain checks, without public cause replay.
                match failure {
                    PolicyFailureCause::Preparation {
                        cause: StartupCause::Io(_),
                    }
                    | PolicyFailureCause::Privilege {
                        cause: StartupCause::Io(_),
                    }
                    | PolicyFailureCause::Filter {
                        cause: StartupCause::Seccompiler(_),
                    }
                    | PolicyFailureCause::UnsupportedArchitecture
                    | PolicyFailureCause::InvalidProgram
                    | PolicyFailureCause::NotBackend
                    | PolicyFailureCause::ProtectionUnverified => {}
                    PolicyFailureCause::Preparation {
                        cause: StartupCause::Seccompiler(_),
                    }
                    | PolicyFailureCause::Privilege {
                        cause: StartupCause::Seccompiler(_),
                    }
                    | PolicyFailureCause::Filter {
                        cause: StartupCause::Io(_),
                    } => {
                        return Err(field_error(DecodeCause::InvalidValue));
                    }
                }
                Ok(InstallerReplyHeader::Refused {
                    identity,
                    authority,
                    stop: required(self.stop)?,
                    failure,
                })
            }
        }
    }
}

macro_rules! once {
    ($slot:expr, $value:expr) => {{
        if $slot.is_some() {
            return Err(field_error(DecodeCause::DuplicateField));
        }
        $slot = Some($value?);
    }};
}

/// Decode the whole existing installer schema, retaining the first actual checking fault.
pub(super) fn decode(
    payload: &[u8],
    scratch: &mut Scratch,
    context: &mut PreparedStartupContext,
) -> Result<InstallerReplyHeader, DecodeError> {
    context.clear();
    let result = decode_inner(payload, scratch, context);
    outer_failure_decode::checked(result, context)
}

fn decode_inner(
    payload: &[u8],
    scratch: &mut Scratch,
    context: &mut PreparedStartupContext,
) -> Result<InstallerReplyHeader, DecodeError> {
    let mut decoder = Decoder::new(payload, scratch)?;
    let mut object = decoder.begin_object()?;
    let mut fields = Fields::default();
    while let Some(name) = decoder.next_field(&mut object)? {
        match Field::from_text(name)? {
            Field::Kind => once!(
                fields.kind,
                ReplyKind::metadata_text(decoder.unit_variant()?)
                    .ok_or_else(|| field_error(DecodeCause::InvalidValue))
            ),
            Field::Identity => once!(fields.identity, role_scalar_decode::identity(&mut decoder)),
            Field::Authority => once!(
                fields.authority,
                role_scalar_decode::authority(&mut decoder)
            ),
            Field::Stop => once!(fields.stop, role_scalar_decode::stop(&mut decoder)),
            Field::Failure => once!(
                fields.failure,
                role_control_scalar_decode::policy(&mut decoder, context.metadata_fault_slot())
            ),
            Field::Context => once!(fields.context, context.decode_context(&mut decoder)),
        }
    }
    let header = fields.finish()?;
    decoder.finish()?;
    Ok(header)
}

/// Additional fixed installer schema records only. Primitive, scalar, policy, cause and context
/// staging/resident capacities are charged separately. This fixed call graph has no payload-driven
/// native recursion; compiler temporaries, initializer placement and stack highwater are UNMEASURED.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<Fields>(),
        size_of::<Field>(),
        size_of::<ReplyKind>(),
        size_of::<InstallerReplyHeader>(),
        size_of::<Result<InstallerReplyHeader, DecodeError>>(),
        size_of::<&mut PreparedStartupContext>(),
        // Enclosing root state/name remain live across the separately charged nested schema.
        size_of::<ObjectState>(),
        size_of::<Text<'static>>(),
    ];
    let storage = || DecodeError::new(DecodeSite::Storage, DecodeCause::StorageBound);
    let table = u64::try_from(size_of_val(&terms)).map_err(|_| storage())?;
    terms.into_iter().try_fold(table, |sum, term| {
        sum.checked_add(u64::try_from(term).map_err(|_| storage())?)
            .ok_or_else(storage)
    })
}

#[cfg(test)]
mod tests {
    // Grammar units only: no authenticated installer/state/original public error replay credit.
    use std::io;

    use super::*;
    use crate::kani::run::{
        cross_role_cause::CauseIntegrityPredicate,
        protocol::current_build_identity,
        role_deadline::{MonotonicInstant, StopOrigin},
        startup_cause::StartupSeccompilerCause,
        startup_envelope::InstallerReply,
    };

    fn authority() -> RunAuthority {
        RunAuthority::from_wire_bytes([9; 32])
    }

    fn stop() -> StopStamp {
        StopStamp::from_wire_parts(StopOrigin::Inner, MonotonicInstant::from_wire_parts(1, 2))
    }

    fn refusal(failure: PolicyFailureCause, context: &[u8]) -> InstallerReply<'_> {
        InstallerReply::Refused {
            identity: current_build_identity(),
            authority: authority(),
            stop: stop(),
            failure,
            context,
        }
    }

    fn failure() -> PolicyFailureCause {
        PolicyFailureCause::Preparation {
            cause: StartupCause::capture_io(&io::Error::from_raw_os_error(1)).unwrap(),
        }
    }

    fn assert_refused(header: InstallerReplyHeader, expected: PolicyFailureCause) {
        match header {
            InstallerReplyHeader::Refused {
                identity,
                authority: actual,
                stop: actual_stop,
                failure,
            } => {
                assert_eq!(identity, current_build_identity());
                assert_eq!(actual, authority());
                assert_eq!(actual_stop, stop());
                assert_eq!(failure, expected);
            }
            InstallerReplyHeader::PolicyReady { .. } => panic!("refusal became ready"),
        }
    }

    fn refuses(bytes: &[u8], predicate: CauseIntegrityPredicate) -> DecodeError {
        let mut scratch = Scratch::default();
        let mut context = PreparedStartupContext::new(16).unwrap();
        let source = decode(bytes, &mut scratch, &mut context).err().unwrap();
        assert_eq!(*context.metadata_fault_slot(), Some(predicate));
        source
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn serialized_ready_and_refused_headers_preserve_all_parsed_facts() {
        let mut scratch = Scratch::default();
        let mut context = PreparedStartupContext::new(16).unwrap();
        let ready = InstallerReply::PolicyReady {
            identity: current_build_identity(),
            authority: authority(),
        };
        let bytes = serde_json::to_vec(&ready).unwrap();
        match decode(&bytes, &mut scratch, &mut context).unwrap() {
            InstallerReplyHeader::PolicyReady {
                identity,
                authority: actual,
            } => {
                assert_eq!(identity, current_build_identity());
                assert_eq!(actual, authority());
            }
            InstallerReplyHeader::Refused { .. } => panic!("ready became refusal"),
        }
        let io = StartupCause::capture_io(&io::Error::from(io::ErrorKind::InvalidData)).unwrap();
        for expected in [
            failure(),
            PolicyFailureCause::Privilege { cause: io },
            PolicyFailureCause::Filter {
                cause: StartupCause::Seccompiler(StartupSeccompilerCause::EmptyFilter),
            },
            PolicyFailureCause::UnsupportedArchitecture,
            PolicyFailureCause::InvalidProgram,
            PolicyFailureCause::NotBackend,
            PolicyFailureCause::ProtectionUnverified,
        ] {
            let bytes = serde_json::to_vec(&refusal(expected, b"actual text")).unwrap();
            assert_refused(
                decode(&bytes, &mut scratch, &mut context).unwrap(),
                expected,
            );
            assert_eq!(context.context(), "actual text");
            assert!(context.metadata_fault_slot().is_none());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn derived_header_unit_tag_forms_are_preserved_with_exact_null_map_bodies() {
        for reply in [
            InstallerReply::PolicyReady {
                identity: current_build_identity(),
                authority: authority(),
            },
            refusal(failure(), &[]),
        ] {
            let mut value = serde_json::to_value(reply).unwrap();
            let kind = value["kind"].as_str().unwrap().to_owned();
            value["kind"] = serde_json::json!({(kind.clone()): null});
            let bytes = serde_json::to_vec(&value).unwrap();
            let mut scratch = Scratch::default();
            let mut context = PreparedStartupContext::new(16).unwrap();
            let header = decode(&bytes, &mut scratch, &mut context).unwrap();
            if kind == "Refused" {
                assert_refused(header, failure());
            } else {
                match header {
                    InstallerReplyHeader::PolicyReady {
                        identity,
                        authority: actual,
                    } => {
                        assert_eq!(identity, current_build_identity());
                        assert_eq!(actual, authority());
                    }
                    InstallerReplyHeader::Refused { .. } => {
                        panic!("unit tag selected wrong branch")
                    }
                }
            }
            for body in [
                serde_json::json!(false),
                serde_json::json!({}),
                serde_json::json!([]),
            ] {
                value["kind"] = serde_json::json!({(kind.clone()): body});
                refuses(
                    &serde_json::to_vec(&value).unwrap(),
                    CauseIntegrityPredicate::MalformedCauseMetadata,
                );
            }
            value["kind"] = serde_json::json!({"Refused": null, "PolicyReady": null});
            refuses(
                &serde_json::to_vec(&value).unwrap(),
                CauseIntegrityPredicate::MalformedCauseMetadata,
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn every_required_field_rejects_missing_null_duplicate_unknown_or_wrong_type() {
        let original = serde_json::to_value(refusal(failure(), &[])).unwrap();
        for field in [
            "kind",
            "identity",
            "authority",
            "stop",
            "failure",
            "context",
        ] {
            let mut changed = original.clone();
            changed.as_object_mut().unwrap().remove(field);
            refuses(
                &serde_json::to_vec(&changed).unwrap(),
                CauseIntegrityPredicate::IncompleteCauseMetadata,
            );
            for wrong in [serde_json::Value::Null, serde_json::json!(42)] {
                let mut changed = original.clone();
                changed[field] = wrong;
                refuses(
                    &serde_json::to_vec(&changed).unwrap(),
                    CauseIntegrityPredicate::MalformedCauseMetadata,
                );
            }
            let bytes = serde_json::to_string(&original).unwrap();
            let doubled = bytes.replacen('{', &format!("{{\"{field}\":{},", original[field]), 1);
            assert_eq!(
                refuses(
                    doubled.as_bytes(),
                    CauseIntegrityPredicate::MalformedCauseMetadata
                )
                .cause(),
                DecodeCause::DuplicateField
            );
        }
        let mut changed = original.clone();
        changed["foreign"] = serde_json::json!(0);
        refuses(
            &serde_json::to_vec(&changed).unwrap(),
            CauseIntegrityPredicate::MalformedCauseMetadata,
        );
        let ready = serde_json::to_value(InstallerReply::PolicyReady {
            identity: current_build_identity(),
            authority: authority(),
        })
        .unwrap();
        for field in ["stop", "failure", "context"] {
            let mut extra = ready.clone();
            extra[field] = original[field].clone();
            assert_eq!(
                refuses(
                    &serde_json::to_vec(&extra).unwrap(),
                    CauseIntegrityPredicate::MalformedCauseMetadata
                )
                .cause(),
                DecodeCause::UnknownField
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn stronger_installer_site_cause_pairs_refuse_without_cause_projection() {
        let io = StartupCause::capture_io(&io::Error::from_raw_os_error(1)).unwrap();
        let dependency = StartupCause::Seccompiler(StartupSeccompilerCause::EmptyFilter);
        for wrong in [
            PolicyFailureCause::Preparation { cause: dependency },
            PolicyFailureCause::Privilege { cause: dependency },
            PolicyFailureCause::Filter { cause: io },
        ] {
            let bytes = serde_json::to_vec(&refusal(wrong, &[])).unwrap();
            assert_eq!(
                refuses(&bytes, CauseIntegrityPredicate::MalformedCauseMetadata).cause(),
                DecodeCause::InvalidValue
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn unknown_cause_kind_precedes_later_malformed_required_context() {
        let original = serde_json::to_value(refusal(failure(), &[])).unwrap();
        let mut cause = original["failure"].clone();
        cause["Preparation"]["cause"]["Io"]["kind"] = serde_json::json!("UnknownOriginalKind");
        let bytes = format!("{{\"kind\":\"Refused\",\"identity\":{},\"authority\":{},\"stop\":{},\"failure\":{cause},\"context\":[false]}}?", original["identity"], original["authority"], original["stop"]);
        assert_eq!(
            refuses(
                bytes.as_bytes(),
                CauseIntegrityPredicate::UnknownKindMetadata
            )
            .cause(),
            DecodeCause::InvalidValue
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn omitted_diagnostics_cannot_bypass_required_bytes_or_enclosing_eof() {
        let mut scratch = Scratch::default();
        let mut context = PreparedStartupContext::new(2).unwrap();
        for diagnostic in [&[255_u8][..], b"overbound", &[0xe2, 0x82][..]] {
            let bytes = serde_json::to_vec(&refusal(failure(), diagnostic)).unwrap();
            assert_refused(
                decode(&bytes, &mut scratch, &mut context).unwrap(),
                failure(),
            );
            assert!(context.context().is_empty());
            assert!(context.metadata_fault_slot().is_none());
        }
        for malformed in [
            serde_json::json!([255, false]),
            serde_json::json!([255, 256]),
            serde_json::json!([255, -1]),
        ] {
            let mut value = serde_json::to_value(refusal(failure(), &[])).unwrap();
            value["context"] = malformed;
            refuses(
                &serde_json::to_vec(&value).unwrap(),
                CauseIntegrityPredicate::MalformedCauseMetadata,
            );
        }
        let bytes = serde_json::to_vec(&refusal(failure(), &[255])).unwrap();
        let incomplete = bytes.get(..bytes.len() - 1).unwrap();
        assert_eq!(
            refuses(incomplete, CauseIntegrityPredicate::IncompleteCauseMetadata).cause(),
            DecodeCause::UnexpectedEnd
        );
        let mut trailing = bytes;
        trailing.push(b'?');
        assert_eq!(
            refuses(&trailing, CauseIntegrityPredicate::MalformedCauseMetadata).cause(),
            DecodeCause::TrailingBytes
        );
    }
}

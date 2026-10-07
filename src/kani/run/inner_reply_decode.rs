// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! One exact inner-role reply grammar on the original owner-held cursor (FR-034).
//!
//! Headers are parsed facts only. The actor selects startup/event/owner subsets and authenticates
//! actual roles, original clocks, lease and settlement. Context-free frames retain pending original
//! diagnostic text; every frame resets only its first metadata-checking fault.

use std::mem::{size_of, size_of_val};

use super::{
    guardian_decode::{DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Scratch, Text},
    installer_reply_decode, outer_failure_decode,
    protocol::{BackendExit, BackendExitTag, BuildIdentity, GuardianControl, RunAuthority},
    role_control_scalar_decode,
    role_deadline::StopStamp,
    role_scalar_decode,
    startup_cause::PreparedStartupContext,
    startup_envelope::{InnerEventHeader, InnerFrameKind, PolicyFailureCause},
};

/// Parsed union only; wrappers retain the existing phase-specific accepted subsets.
pub(super) enum InnerReply {
    Ready(GuardianControl),
    Event(InnerEventHeader),
    Refused {
        identity: BuildIdentity,
        authority: RunAuthority,
        stop: StopStamp,
        failure: PolicyFailureCause,
    },
}

fn error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}

fn required<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| error(DecodeCause::MissingField))
}

fn reject(present: &[bool]) -> Result<(), DecodeError> {
    if present.iter().any(|present| *present) {
        Err(error(DecodeCause::UnknownField))
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Field {
    Kind,
    Identity,
    Authority,
    MappedUid,
    CreatorPid,
    Outcome,
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
        } else if text.equals("mapped_uid") {
            Ok(Self::MappedUid)
        } else if text.equals("creator_pid") {
            Ok(Self::CreatorPid)
        } else if text.equals("outcome") {
            Ok(Self::Outcome)
        } else if text.equals("stop") {
            Ok(Self::Stop)
        } else if text.equals("failure") {
            Ok(Self::Failure)
        } else if text.equals("context") {
            Ok(Self::Context)
        } else {
            Err(error(DecodeCause::UnknownField))
        }
    }
}

#[derive(Default)]
struct Fields {
    kind: Option<InnerFrameKind>,
    identity: Option<BuildIdentity>,
    authority: Option<RunAuthority>,
    mapped_uid: Option<u32>,
    creator_pid: Option<i32>,
    outcome: Option<BackendExit>,
    stop: Option<StopStamp>,
    failure: Option<PolicyFailureCause>,
    context: Option<()>,
}

impl Fields {
    fn finish(self) -> Result<InnerReply, DecodeError> {
        let authority = required(self.authority)?;
        match required(self.kind)? {
            InnerFrameKind::Ready => {
                reject(&[
                    self.outcome.is_some(),
                    self.stop.is_some(),
                    self.failure.is_some(),
                    self.context.is_some(),
                ])?;
                Ok(InnerReply::Ready(GuardianControl::Ready {
                    identity: required(self.identity)?,
                    authority,
                    mapped_uid: required(self.mapped_uid)?,
                    creator_pid: required(self.creator_pid)?,
                }))
            }
            InnerFrameKind::Dispatched => {
                reject(&[
                    self.identity.is_some(),
                    self.mapped_uid.is_some(),
                    self.creator_pid.is_some(),
                    self.outcome.is_some(),
                    self.stop.is_some(),
                    self.failure.is_some(),
                    self.context.is_some(),
                ])?;
                Ok(InnerReply::Event(InnerEventHeader::Dispatched {
                    authority,
                }))
            }
            InnerFrameKind::Completed => {
                reject(&[
                    self.identity.is_some(),
                    self.mapped_uid.is_some(),
                    self.creator_pid.is_some(),
                    self.failure.is_some(),
                    self.context.is_some(),
                ])?;
                Ok(InnerReply::Event(InnerEventHeader::Completed {
                    authority,
                    outcome: required(self.outcome)?,
                    stop: required(self.stop)?,
                }))
            }
            InnerFrameKind::Refused => {
                reject(&[
                    self.mapped_uid.is_some(),
                    self.creator_pid.is_some(),
                    self.outcome.is_some(),
                ])?;
                let identity = required(self.identity)?;
                let stop = required(self.stop)?;
                required(self.context)?;
                let failure = required(self.failure)?;
                let failure = installer_reply_decode::admitted_failure(failure)?;
                Ok(InnerReply::Refused {
                    identity,
                    authority,
                    stop,
                    failure,
                })
            }
        }
    }
}

macro_rules! once {
    ($slot:expr, $value:expr) => {{
        if $slot.is_some() {
            return Err(error(DecodeCause::DuplicateField));
        }
        $slot = Some($value?);
    }};
}

fn signed_i32(decoder: &mut Decoder<'_, '_>) -> Result<i32, DecodeError> {
    i32::try_from(decoder.signed()?)
        .map_err(|_| DecodeError::new(DecodeSite::Signed, DecodeCause::IntegerOverflow))
}

fn mapped_uid(decoder: &mut Decoder<'_, '_>) -> Result<u32, DecodeError> {
    u32::try_from(decoder.unsigned()?)
        .map_err(|_| DecodeError::new(DecodeSite::Unsigned, DecodeCause::IntegerOverflow))
}

fn outcome(decoder: &mut Decoder<'_, '_>) -> Result<BackendExit, DecodeError> {
    let mut object = decoder.begin_object()?;
    let name = decoder
        .next_field(&mut object)?
        .ok_or_else(|| error(DecodeCause::MissingField))?;
    let tag =
        BackendExitTag::metadata_text(name).ok_or_else(|| error(DecodeCause::InvalidValue))?;
    let value = signed_i32(decoder)?;
    let outcome = match tag {
        BackendExitTag::Code => BackendExit::Code(value),
        BackendExitTag::Signal => BackendExit::Signal(value),
    };
    if decoder.next_field(&mut object)?.is_some() {
        return Err(error(DecodeCause::InvalidValue));
    }
    Ok(outcome)
}

/// Consume the whole schema with the supplied scratch and actual per-frame first-fault slot.
pub(super) fn decode(
    payload: &[u8],
    scratch: &mut Scratch,
    context: &mut PreparedStartupContext,
) -> Result<InnerReply, DecodeError> {
    context.begin_metadata_frame();
    let result = decode_inner(payload, scratch, context);
    outer_failure_decode::checked(result, context)
}

fn decode_inner(
    payload: &[u8],
    scratch: &mut Scratch,
    context: &mut PreparedStartupContext,
) -> Result<InnerReply, DecodeError> {
    let mut decoder = Decoder::new(payload, scratch)?;
    let mut object = decoder.begin_object()?;
    let mut fields = Fields::default();
    while let Some(name) = decoder.next_field(&mut object)? {
        match Field::from_text(name)? {
            Field::Kind => once!(
                fields.kind,
                InnerFrameKind::metadata_text(decoder.unit_variant()?)
                    .ok_or_else(|| error(DecodeCause::InvalidValue))
            ),
            Field::Identity => once!(fields.identity, role_scalar_decode::identity(&mut decoder)),
            Field::Authority => once!(
                fields.authority,
                role_scalar_decode::authority(&mut decoder)
            ),
            Field::MappedUid => once!(fields.mapped_uid, mapped_uid(&mut decoder)),
            Field::CreatorPid => once!(fields.creator_pid, signed_i32(&mut decoder)),
            Field::Outcome => once!(fields.outcome, outcome(&mut decoder)),
            Field::Stop => once!(fields.stop, role_scalar_decode::stop(&mut decoder)),
            Field::Failure => once!(
                fields.failure,
                role_control_scalar_decode::policy(&mut decoder, context.metadata_fault_slot())
            ),
            Field::Context => once!(fields.context, context.decode_context(&mut decoder)),
        }
    }
    let reply = fields.finish()?;
    decoder.finish()?;
    Ok(reply)
}

/// Checked schema delta only; existing primitive/scalar/policy/cause/context charges are separate.
/// Fixed call depth is not compiler frame or initializer/native-stack highwater evidence.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<Fields>(),
        size_of::<Field>(),
        size_of::<InnerFrameKind>(),
        size_of::<BackendExitTag>(),
        size_of::<InnerReply>(),
        size_of::<InnerEventHeader>(),
        size_of::<Result<InnerReply, DecodeError>>(),
        size_of::<BackendExit>(),
        size_of::<ObjectState>(),
        size_of::<Text<'static>>(),
        size_of::<&mut PreparedStartupContext>(),
        size_of::<[bool; 7]>(),
        size_of::<&[bool]>(),
        size_of::<std::slice::Iter<'static, bool>>(),
        size_of::<&bool>(),
        size_of::<i32>(),
        size_of::<u32>(),
        size_of::<i64>(),
        size_of::<u64>(),
        size_of::<Result<i32, DecodeError>>(),
        size_of::<Result<u32, DecodeError>>(),
        size_of::<Result<BackendExit, DecodeError>>(),
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
    // Only grammar/custody of parsed facts, not genuine Ready/Dispatch/completion or cleanup.
    use std::io;

    use super::*;
    use crate::kani::run::{
        cross_role_cause::CauseIntegrityPredicate,
        protocol::current_build_identity,
        role_deadline::{MonotonicInstant, StopOrigin},
        startup_cause::StartupCause,
        startup_envelope::InstallerReply,
    };

    fn authority() -> RunAuthority {
        RunAuthority::from_wire_bytes([3; 32])
    }
    fn stop() -> StopStamp {
        StopStamp::from_wire_parts(StopOrigin::Inner, MonotonicInstant::from_wire_parts(1, 2))
    }
    fn failure() -> PolicyFailureCause {
        PolicyFailureCause::Privilege {
            cause: StartupCause::capture_io(&io::Error::from_raw_os_error(1)).unwrap(),
        }
    }
    fn fixtures() -> Vec<serde_json::Value> {
        vec![
            serde_json::to_value(GuardianControl::Ready {
                identity: current_build_identity(),
                authority: authority(),
                mapped_uid: u32::MAX,
                creator_pid: i32::MIN,
            })
            .unwrap(),
            serde_json::to_value(GuardianControl::Dispatched {
                authority: authority(),
            })
            .unwrap(),
            serde_json::to_value(GuardianControl::Completed {
                authority: authority(),
                outcome: BackendExit::Signal(-2),
                stop: stop(),
            })
            .unwrap(),
            serde_json::to_value(InstallerReply::Refused {
                identity: current_build_identity(),
                authority: authority(),
                stop: stop(),
                failure: failure(),
                context: b"pending",
            })
            .unwrap(),
        ]
    }

    fn assert_parsed(reply: InnerReply, kind: &str) {
        match (reply, kind) {
            (
                InnerReply::Ready(GuardianControl::Ready {
                    identity,
                    authority: actual,
                    mapped_uid,
                    creator_pid,
                }),
                "Ready",
            ) => {
                assert_eq!(identity, current_build_identity());
                assert_eq!(actual, authority());
                assert_eq!(mapped_uid, u32::MAX);
                assert_eq!(creator_pid, i32::MIN);
            }
            (
                InnerReply::Event(InnerEventHeader::Dispatched { authority: actual }),
                "Dispatched",
            ) => assert_eq!(actual, authority()),
            (
                InnerReply::Event(InnerEventHeader::Completed {
                    authority: actual,
                    outcome,
                    stop: stamp,
                }),
                "Completed",
            ) => {
                assert_eq!(actual, authority());
                assert_eq!(outcome, BackendExit::Signal(-2));
                assert_eq!(stamp, stop());
            }
            (
                InnerReply::Refused {
                    identity,
                    authority: actual,
                    stop: stamp,
                    failure: actual_failure,
                },
                "Refused",
            ) => {
                assert_eq!(identity, current_build_identity());
                assert_eq!(actual, authority());
                assert_eq!(stamp, stop());
                assert_eq!(actual_failure, failure());
            }
            _ => panic!("decoded header does not match typed fixture"),
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
    fn actual_encoded_branches_preserve_parsed_facts_and_pending_diagnostics() {
        let mut scratch = Scratch::default();
        let mut context = PreparedStartupContext::new(16).unwrap();
        context.capture_context(&"pending");
        let capacity = context.reserved_bytes();
        for value in fixtures() {
            *context.metadata_fault_slot() = Some(CauseIntegrityPredicate::UnknownKindMetadata);
            let bytes = serde_json::to_vec(&value).unwrap();
            assert_parsed(
                decode(&bytes, &mut scratch, &mut context).unwrap(),
                value["kind"].as_str().unwrap(),
            );
            assert_eq!(context.context(), "pending");
            assert_eq!(context.reserved_bytes(), capacity);
            assert!(context.metadata_fault_slot().is_none());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn every_required_and_forbidden_field_has_independent_adverse_controls() {
        let cases = fixtures();
        let mut all_fields = serde_json::Map::new();
        for case in &cases {
            all_fields.extend(case.as_object().unwrap().clone());
        }
        for original in cases {
            for (field, original_value) in original.as_object().unwrap() {
                let mut missing = original.clone();
                missing.as_object_mut().unwrap().remove(field);
                refuses(
                    &serde_json::to_vec(&missing).unwrap(),
                    CauseIntegrityPredicate::IncompleteCauseMetadata,
                );
                for wrong in [serde_json::Value::Null, serde_json::json!(false)] {
                    let mut changed = original.clone();
                    changed[field] = wrong;
                    refuses(
                        &serde_json::to_vec(&changed).unwrap(),
                        CauseIntegrityPredicate::MalformedCauseMetadata,
                    );
                }
                let bytes = serde_json::to_string(&original).unwrap();
                let duplicate = bytes.replacen('{', &format!("{{\"{field}\":{original_value},"), 1);
                assert_eq!(
                    refuses(
                        duplicate.as_bytes(),
                        CauseIntegrityPredicate::MalformedCauseMetadata
                    )
                    .cause(),
                    DecodeCause::DuplicateField
                );
            }
            for (field, value) in &all_fields {
                if !original.as_object().unwrap().contains_key(field) {
                    let mut extra = original.clone();
                    extra[field] = value.clone();
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
            let mut extra = original;
            extra["foreign"] = serde_json::json!(false);
            refuses(
                &serde_json::to_vec(&extra).unwrap(),
                CauseIntegrityPredicate::MalformedCauseMetadata,
            );
        }
        refuses(
            br#"{"kind":"Refused","reason":"InvalidControl"}"#,
            CauseIntegrityPredicate::MalformedCauseMetadata,
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn owning_unit_map_tags_preserve_branch_without_other_tag_or_body_acceptance() {
        for original in fixtures() {
            let kind = original["kind"].as_str().unwrap();
            let mut changed = original.clone();
            changed["kind"] = serde_json::json!({(kind): null});
            let mut scratch = Scratch::default();
            let mut context = PreparedStartupContext::new(16).unwrap();
            assert_parsed(
                decode(
                    &serde_json::to_vec(&changed).unwrap(),
                    &mut scratch,
                    &mut context,
                )
                .unwrap(),
                kind,
            );
            for malformed in [
                serde_json::json!({(kind): false}),
                serde_json::json!({(kind): {}}),
                serde_json::json!({"Ready": null, "Completed": null}),
            ] {
                changed["kind"] = malformed;
                refuses(
                    &serde_json::to_vec(&changed).unwrap(),
                    CauseIntegrityPredicate::MalformedCauseMetadata,
                );
            }
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn exit_and_identity_numbers_retain_exact_width_and_owning_integer_grammar() {
        let completed = &fixtures()[2];
        let mut scratch = Scratch::default();
        let mut context = PreparedStartupContext::new(16).unwrap();
        for expected in [
            BackendExit::Code(i32::MIN),
            BackendExit::Code(i32::MAX),
            BackendExit::Signal(i32::MIN),
            BackendExit::Signal(i32::MAX),
        ] {
            let mut changed = completed.clone();
            changed["outcome"] = serde_json::to_value(expected).unwrap();
            let InnerReply::Event(InnerEventHeader::Completed {
                outcome: actual, ..
            }) = decode(
                &serde_json::to_vec(&changed).unwrap(),
                &mut scratch,
                &mut context,
            )
            .unwrap()
            else {
                panic!("exit value changed branch");
            };
            assert_eq!(actual, expected);
        }
        for tag in ["Code", "Signal"] {
            for number in [
                "-0",
                "-2147483649",
                "2147483648",
                "1.5",
                "null",
                "false",
                "\"0\"",
            ] {
                let mut changed = completed.clone();
                changed["outcome"] = serde_json::json!({(tag): 0});
                let bytes = serde_json::to_string(&changed)
                    .unwrap()
                    .replace(&format!("\"{tag}\":0"), &format!("\"{tag}\":{number}"));
                refuses(
                    bytes.as_bytes(),
                    CauseIntegrityPredicate::MalformedCauseMetadata,
                );
            }
        }
        for invalid in [
            serde_json::json!({}),
            serde_json::json!({"Other": 0}),
            serde_json::json!({"Code": 0, "Signal": 9}),
            serde_json::json!("Code"),
        ] {
            let mut changed = completed.clone();
            changed["outcome"] = invalid;
            assert!(decode(
                &serde_json::to_vec(&changed).unwrap(),
                &mut scratch,
                &mut context
            )
            .is_err());
        }
        for (field, numbers) in [
            ("mapped_uid", &["-1", "4294967296", "1.5"][..]),
            ("creator_pid", &["-0", "2147483648", "-2147483649"][..]),
        ] {
            for number in numbers {
                let mut ready = fixtures()[0].clone();
                ready[field] = serde_json::json!(0);
                let bytes = serde_json::to_string(&ready)
                    .unwrap()
                    .replace(&format!("\"{field}\":0"), &format!("\"{field}\":{number}"));
                refuses(
                    bytes.as_bytes(),
                    CauseIntegrityPredicate::MalformedCauseMetadata,
                );
            }
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn first_unknown_cause_and_required_context_structure_survive_optional_text_omission() {
        let original = fixtures()[3].clone();
        let mut failure = original["failure"].clone();
        failure["Privilege"]["cause"]["Io"]["kind"] = serde_json::json!("UnknownKind");
        let bytes = format!("{{\"kind\":\"Refused\",\"identity\":{},\"authority\":{},\"stop\":{},\"failure\":{failure},\"context\":[false]}}?", original["identity"], original["authority"], original["stop"]);
        assert_eq!(
            refuses(
                bytes.as_bytes(),
                CauseIntegrityPredicate::UnknownKindMetadata
            )
            .cause(),
            DecodeCause::InvalidValue
        );
        let mut scratch = Scratch::default();
        let mut context = PreparedStartupContext::new(2).unwrap();
        for text in [
            serde_json::json!([255]),
            serde_json::json!([226, 130]),
            serde_json::json!([97, 98, 99]),
        ] {
            let mut changed = original.clone();
            changed["context"] = text;
            assert_parsed(
                decode(
                    &serde_json::to_vec(&changed).unwrap(),
                    &mut scratch,
                    &mut context,
                )
                .unwrap(),
                "Refused",
            );
            assert!(context.context().is_empty());
            assert!(context.metadata_fault_slot().is_none());
        }
        for text in [
            serde_json::json!([255, false]),
            serde_json::json!([255, 256]),
        ] {
            let mut changed = original.clone();
            changed["context"] = text;
            refuses(
                &serde_json::to_vec(&changed).unwrap(),
                CauseIntegrityPredicate::MalformedCauseMetadata,
            );
        }
        let mut bytes = serde_json::to_vec(&original).unwrap();
        bytes.push(b'?');
        assert_eq!(
            refuses(&bytes, CauseIntegrityPredicate::MalformedCauseMetadata).cause(),
            DecodeCause::TrailingBytes
        );
        bytes.pop();
        bytes.pop();
        assert_eq!(
            refuses(&bytes, CauseIntegrityPredicate::IncompleteCauseMetadata).cause(),
            DecodeCause::UnexpectedEnd
        );
    }
}

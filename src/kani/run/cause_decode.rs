// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Strict original-cause metadata over the guardian's actual fixed decoder (FR-034).
//!
//! The owner supplies the current cursor, sole scratch and first-fault slot. This module parses
//! only the existing cause schema: no framing, context, producer/site authentication, projection
//! or optional diagnostic rendering. Owning declarations supply kind/payload/variant labels.

use std::{
    mem::{size_of, size_of_val},
    os::raw::c_long,
};

use super::{
    cross_role_cause::CauseIntegrityPredicate,
    guardian_decode::{
        record as named_record, ArrayState, DecodeCause, DecodeError, DecodeSite, Decoder,
        ObjectState, Text, ValueKind,
    },
    startup_cause::{
        StartupCause, StartupCauseTag, StartupIoCause, StartupIoField, StartupIoKind,
        StartupPayload, StartupSeccompilerCause, StartupSeccompilerTag,
    },
};

fn record(slot: &mut Option<CauseIntegrityPredicate>, predicate: CauseIntegrityPredicate) {
    if slot.is_none() {
        *slot = Some(predicate);
    }
}

/// Preserve the actual error object and first observed schema/syntax checking fact.
fn checked<T>(
    result: Result<T, DecodeError>,
    fault: &mut Option<CauseIntegrityPredicate>,
) -> Result<T, DecodeError> {
    result.map_err(|error| {
        match error.cause() {
            DecodeCause::MissingField | DecodeCause::UnexpectedEnd => {
                record(fault, CauseIntegrityPredicate::IncompleteCauseMetadata);
            }
            DecodeCause::InputBound | DecodeCause::StorageBound => {
                // The original input/reservation owner must handle these actual bounds.
                // This schema owns no additional cap and has no allocated predicate for them.
            }
            DecodeCause::EmptyInput
            | DecodeCause::UnexpectedToken
            | DecodeCause::InvalidSeparator
            | DecodeCause::InvalidEscape
            | DecodeCause::InvalidUnicode
            | DecodeCause::InvalidUtf8
            | DecodeCause::InvalidNumber
            | DecodeCause::IntegerOverflow
            | DecodeCause::TrailingBytes
            | DecodeCause::UnknownField
            | DecodeCause::DuplicateField
            | DecodeCause::InvalidValue => {
                record(fault, CauseIntegrityPredicate::MalformedCauseMetadata);
            }
        }
        error
    })
}

fn field_error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}

fn required<'input>(
    decoder: &mut Decoder<'input, '_>,
    object: &mut ObjectState,
) -> Result<Text<'input>, DecodeError> {
    decoder
        .next_field(object)?
        .ok_or_else(|| field_error(DecodeCause::MissingField))
}

fn close_single(
    decoder: &mut Decoder<'_, '_>,
    object: &mut ObjectState,
) -> Result<(), DecodeError> {
    if decoder.next_field(object)?.is_some() {
        return Err(field_error(DecodeCause::InvalidValue));
    }
    Ok(())
}

/// Consume one existing external-tag cause value; containing-envelope EOF remains owner-held.
pub(super) fn decode(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
) -> Result<StartupCause, DecodeError> {
    let result = decode_cause(decoder, fault);
    checked(result, fault)
}

/// Only the policy's original ordinary Deserialize delegate admits derived record/enum forms.
/// The generic metadata seed remains map-only with string-only kind/payload/unit tokens.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Grammar {
    MetadataSeed,
    PolicyDerived,
}

/// Parse the original policy delegate's ordinary-derived cause grammar on the same cursor.
pub(super) fn decode_policy(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
) -> Result<StartupCause, DecodeError> {
    let result = cause_value(decoder, fault, Grammar::PolicyDerived);
    checked(result, fault)
}

fn decode_cause(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
) -> Result<StartupCause, DecodeError> {
    cause_value(decoder, fault, Grammar::MetadataSeed)
}

fn cause_value(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
    grammar: Grammar,
) -> Result<StartupCause, DecodeError> {
    let mut object = decoder.begin_object()?;
    let tag = StartupCauseTag::metadata_text(required(decoder, &mut object)?)
        .ok_or_else(|| field_error(DecodeCause::UnknownField))?;
    let cause = match tag {
        StartupCauseTag::Io => StartupCause::Io(io_value(decoder, fault, grammar)?),
        StartupCauseTag::Seccompiler => {
            StartupCause::Seccompiler(installation(decoder, fault, grammar)?)
        }
    };
    close_single(decoder, &mut object)?;
    Ok(cause)
}

#[derive(Default)]
struct IoFields {
    kind: Option<StartupIoKind>,
    // Outer Option records actual member presence; inner None is its explicit JSON null.
    raw_os_error: Option<Option<i32>>,
    payload: Option<StartupPayload>,
}

impl IoFields {
    fn finish(self) -> Result<StartupIoCause, DecodeError> {
        let missing = || field_error(DecodeCause::MissingField);
        Ok(StartupIoCause::from_metadata(
            self.kind.ok_or_else(missing)?,
            self.raw_os_error.ok_or_else(missing)?,
            self.payload.ok_or_else(missing)?,
        ))
    }
}

/// Consume one strict I/O record with all three fields mandatory, including explicit errno null.
pub(super) fn decode_io(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
) -> Result<StartupIoCause, DecodeError> {
    let result = io_value(decoder, fault, Grammar::MetadataSeed);
    checked(result, fault)
}

// Resolve the owning enum label before its unit body, preserving an earlier kind fault even
// when later null/closing syntax is malformed. This is only the policy-derived unit grammar.
fn derived_unit<'input, T>(
    decoder: &mut Decoder<'input, '_>,
    lookup: impl FnOnce(Text<'input>) -> Result<T, DecodeError>,
) -> Result<T, DecodeError> {
    match decoder.peek_kind()? {
        ValueKind::String => lookup(decoder.string()?),
        ValueKind::Object => {
            let mut object = decoder.begin_object()?;
            let value = lookup(required(decoder, &mut object)?)?;
            decoder.null()?;
            close_single(decoder, &mut object)?;
            Ok(value)
        }
        ValueKind::Array | ValueKind::Number | ValueKind::Boolean | ValueKind::Null => {
            Err(field_error(DecodeCause::UnexpectedToken))
        }
    }
}

fn io_value(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
    grammar: Grammar,
) -> Result<StartupIoCause, DecodeError> {
    let mut fields = IoFields::default();
    let mut visit = |decoder: &mut Decoder<'_, '_>, field| {
        match field {
            StartupIoField::Kind => {
                if fields.kind.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                let mut lookup = |name| {
                    StartupIoKind::metadata_text(name).ok_or_else(|| {
                        record(fault, CauseIntegrityPredicate::UnknownKindMetadata);
                        field_error(DecodeCause::InvalidValue)
                    })
                };
                fields.kind = Some(match grammar {
                    Grammar::MetadataSeed => lookup(decoder.string()?)?,
                    Grammar::PolicyDerived => derived_unit(decoder, lookup)?,
                });
            }
            StartupIoField::Errno => {
                if fields.raw_os_error.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                fields.raw_os_error = Some(if decoder.peek_kind()? == ValueKind::Null {
                    decoder.null()?;
                    None
                } else {
                    Some(i32::try_from(decoder.signed()?).map_err(|_| {
                        DecodeError::new(DecodeSite::Signed, DecodeCause::IntegerOverflow)
                    })?)
                });
            }
            StartupIoField::Payload => {
                if fields.payload.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                let lookup = |name| {
                    StartupPayload::metadata_text(name)
                        .ok_or_else(|| field_error(DecodeCause::InvalidValue))
                };
                fields.payload = Some(match grammar {
                    Grammar::MetadataSeed => lookup(decoder.string()?)?,
                    Grammar::PolicyDerived => derived_unit(decoder, lookup)?,
                });
            }
        }
        Ok(())
    };
    match grammar {
        Grammar::PolicyDerived => named_record(
            decoder,
            StartupIoField::declared_order(),
            StartupIoField::metadata_text,
            visit,
        )?,
        Grammar::MetadataSeed => {
            let mut object = decoder.begin_object()?;
            while let Some(name) = decoder.next_field(&mut object)? {
                visit(
                    decoder,
                    StartupIoField::metadata_text(name)
                        .ok_or_else(|| field_error(DecodeCause::UnknownField))?,
                )?;
            }
        }
    }
    fields.finish()
}

// The installation string-versus-map discriminator is supplied by the owning primitive.
// It must not prevalidate a whole subtree before a prior kind checking fact is observed.
fn installation(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
    grammar: Grammar,
) -> Result<StartupSeccompilerCause, DecodeError> {
    let result = installation_record(decoder, fault, grammar);
    checked(result, fault)
}

fn installation_record(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
    grammar: Grammar,
) -> Result<StartupSeccompilerCause, DecodeError> {
    if decoder.peek_kind()? == ValueKind::String {
        return match StartupSeccompilerTag::metadata_text(decoder.string()?) {
            Some(StartupSeccompilerTag::EmptyFilter) => Ok(StartupSeccompilerCause::EmptyFilter),
            Some(
                StartupSeccompilerTag::Prctl
                | StartupSeccompilerTag::Seccomp
                | StartupSeccompilerTag::ThreadSync,
            )
            | None => Err(field_error(DecodeCause::InvalidValue)),
        };
    }
    let mut object = decoder.begin_object()?;
    let tag = StartupSeccompilerTag::metadata_text(required(decoder, &mut object)?)
        .ok_or_else(|| field_error(DecodeCause::UnknownField))?;
    let cause = match tag {
        // The currently strict cause codec admits this unit variant ONLY as a string.
        StartupSeccompilerTag::EmptyFilter => {
            if grammar == Grammar::MetadataSeed {
                return Err(field_error(DecodeCause::InvalidValue));
            }
            decoder.null()?;
            StartupSeccompilerCause::EmptyFilter
        }
        StartupSeccompilerTag::Prctl => {
            StartupSeccompilerCause::Prctl(io_value(decoder, fault, grammar)?)
        }
        StartupSeccompilerTag::Seccomp => {
            StartupSeccompilerCause::Seccomp(io_value(decoder, fault, grammar)?)
        }
        StartupSeccompilerTag::ThreadSync => StartupSeccompilerCause::ThreadSync {
            pid: thread_sync(decoder, grammar)?,
        },
    };
    close_single(decoder, &mut object)?;
    Ok(cause)
}

struct ThreadSyncFields {
    pid: Option<c_long>,
}

fn thread_sync(decoder: &mut Decoder<'_, '_>, grammar: Grammar) -> Result<c_long, DecodeError> {
    if grammar == Grammar::PolicyDerived && decoder.peek_kind()? == ValueKind::Array {
        let mut array = decoder.begin_array()?;
        if !decoder.next_element(&mut array)? {
            return Err(field_error(DecodeCause::MissingField));
        }
        let pid = c_long::try_from(decoder.signed()?)
            .map_err(|_| DecodeError::new(DecodeSite::Signed, DecodeCause::IntegerOverflow))?;
        if decoder.next_element(&mut array)? {
            return Err(field_error(DecodeCause::InvalidValue));
        }
        return Ok(pid);
    }
    let mut object = decoder.begin_object()?;
    let mut fields = ThreadSyncFields { pid: None };
    while let Some(name) = decoder.next_field(&mut object)? {
        if !name.equals("pid") {
            return Err(field_error(DecodeCause::UnknownField));
        }
        if fields.pid.is_some() {
            return Err(field_error(DecodeCause::DuplicateField));
        }
        fields.pid = Some(
            c_long::try_from(decoder.signed()?)
                .map_err(|_| DecodeError::new(DecodeSite::Signed, DecodeCause::IntegerOverflow))?,
        );
    }
    fields
        .pid
        .ok_or_else(|| field_error(DecodeCause::MissingField))
}

/// Fixed simultaneous schema delta beyond the primitive workspace/decoder reservation.
///
/// Maximum call path: decode -> installation -> decode_io OR thread_sync. No payload-dependent
/// schema recursion or nested Decoder is introduced. Three simultaneously live ObjectStates
/// are required; the primitive already charges one, so this delta adds two ObjectStates.
/// An I/O key and its string value may coexist; primitive charges one Text, so add one Text.
/// Include core cause records and checked-result envelopes, accumulators, selectors, fault-slot
/// reborrows and scalar work not already charged by the primitive. The branch-disjoint I/O and
/// ThreadSync accumulators are conservatively both included as actual declared record types.
/// Main separately charges the existing fault slot and primitive records; no Scratch/payload/
/// framer/context/primitive Decoder duplication. Layout terms do not prove native stack high-water.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<ObjectState>(),
        size_of::<ObjectState>(),
        size_of::<Text<'static>>(),
        size_of::<IoFields>(),
        size_of::<ArrayState>(),
        size_of::<ObjectState>(), // policy unit-map body coexists with its I/O record.
        size_of::<Grammar>(),
        size_of::<&[StartupIoField]>(),
        size_of::<ThreadSyncFields>(),
        size_of::<StartupIoField>(),
        size_of::<StartupCauseTag>(),
        size_of::<StartupSeccompilerTag>(),
        size_of::<StartupCause>(),
        size_of::<StartupSeccompilerCause>(),
        size_of::<StartupIoCause>(),
        size_of::<Result<StartupCause, DecodeError>>(),
        size_of::<Result<StartupSeccompilerCause, DecodeError>>(),
        size_of::<Result<StartupIoCause, DecodeError>>(),
        size_of::<&mut Option<CauseIntegrityPredicate>>(),
        size_of::<&mut Option<CauseIntegrityPredicate>>(),
        size_of::<&mut Option<CauseIntegrityPredicate>>(),
        size_of::<Option<i32>>(),
        size_of::<i32>(),
        size_of::<c_long>(),
    ];
    let storage = || DecodeError::new(DecodeSite::Storage, DecodeCause::StorageBound);
    let bytes = u64::try_from(size_of_val(&terms)).map_err(|_| storage())?;
    terms.into_iter().try_fold(bytes, |bytes, term| {
        bytes
            .checked_add(u64::try_from(term).map_err(|_| storage())?)
            .ok_or_else(storage)
    })
}

#[cfg(test)]
mod tests {
    // Actual schema/source units only, not sender-domain/authentication/whole-AC40 evidence.
    use super::super::guardian_decode::Scratch;
    use super::*;

    fn parse(
        bytes: &[u8],
        fault: &mut Option<CauseIntegrityPredicate>,
    ) -> Result<StartupCause, DecodeError> {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(bytes, &mut scratch)?;
        let cause = decode(&mut decoder, fault)?;
        decoder.finish()?;
        Ok(cause)
    }

    fn refuses(bytes: &[u8], expected: CauseIntegrityPredicate) -> DecodeError {
        let mut fault = None;
        let error = parse(bytes, &mut fault).unwrap_err();
        assert_eq!(fault, Some(expected));
        error
    }

    fn io(kind: StartupIoKind, raw: Option<i32>, payload: StartupPayload) -> StartupIoCause {
        StartupIoCause::from_metadata(kind, raw, payload)
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn actual_owning_serializer_shapes_roundtrip_without_os_projection() {
        let simple = io(
            StartupIoKind::InvalidData,
            None,
            StartupPayload::NoCustomPayload,
        );
        let os = io(
            StartupIoKind::PermissionDenied,
            Some(1),
            StartupPayload::NoCustomPayload,
        );
        for cause in [
            StartupCause::Io(simple),
            StartupCause::Io(os),
            StartupCause::Io(io(
                StartupIoKind::OsDerived,
                None,
                StartupPayload::NoCustomPayload,
            )),
            StartupCause::Io(io(
                StartupIoKind::Other,
                None,
                StartupPayload::DirectTryReserve,
            )),
            StartupCause::Io(io(
                StartupIoKind::Other,
                None,
                StartupPayload::UnrepresentedCustom,
            )),
            StartupCause::Seccompiler(StartupSeccompilerCause::EmptyFilter),
            StartupCause::Seccompiler(StartupSeccompilerCause::Prctl(simple)),
            StartupCause::Seccompiler(StartupSeccompilerCause::Seccomp(os)),
            StartupCause::Seccompiler(StartupSeccompilerCause::ThreadSync { pid: -7 }),
        ] {
            let bytes = serde_json::to_vec(&cause).unwrap();
            let mut fault = None;
            assert_eq!(parse(&bytes, &mut fault).unwrap(), cause);
            assert_eq!(fault, None);
        }
        // OsDerived/null is syntactically admitted metadata, not permission to project it.
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn escaped_labels_and_reordered_members_use_existing_owning_authorities() {
        let bytes = br#"{"\u0049o":{"payload":"NoCustom\u0050ayload","raw_os_error":null,"k\u0069nd":"O\u0074her"}}"#;
        let mut fault = None;
        assert_eq!(
            parse(bytes, &mut fault).unwrap(),
            StartupCause::Io(io(
                StartupIoKind::Other,
                None,
                StartupPayload::NoCustomPayload
            ))
        );
        assert_eq!(fault, None);
        let mut fault = None;
        assert_eq!(
            parse(br#"{"Seccompiler":"Empty\u0046ilter"}"#, &mut fault).unwrap(),
            StartupCause::Seccompiler(StartupSeccompilerCause::EmptyFilter)
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn missing_members_are_incomplete_but_explicit_errno_null_is_present() {
        let original = serde_json::to_value(StartupCause::Io(io(
            StartupIoKind::Other,
            None,
            StartupPayload::NoCustomPayload,
        )))
        .unwrap();
        for name in ["kind", "raw_os_error", "payload"] {
            let mut missing = original.clone();
            missing["Io"].as_object_mut().unwrap().remove(name);
            assert_eq!(
                refuses(
                    &serde_json::to_vec(&missing).unwrap(),
                    CauseIntegrityPredicate::IncompleteCauseMetadata
                )
                .cause(),
                DecodeCause::MissingField
            );
        }
        for bytes in [
            b"{}".as_slice(),
            br#"{"Seccompiler":{}}"#,
            br#"{"Seccompiler":{"ThreadSync":{}}}"#,
        ] {
            assert_eq!(
                refuses(bytes, CauseIntegrityPredicate::IncompleteCauseMetadata).cause(),
                DecodeCause::MissingField
            );
        }
        let mut fault = None;
        assert_eq!(
            parse(&serde_json::to_vec(&original).unwrap(), &mut fault).unwrap(),
            StartupCause::Io(io(
                StartupIoKind::Other,
                None,
                StartupPayload::NoCustomPayload
            ))
        );
        assert_eq!(fault, None);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn unknown_kind_is_first_before_later_syntax_and_preexisting_fault_is_retained() {
        // No subtree prevalidation may mask the earlier actually encountered unknown kind.
        for bytes in [
            br#"{"Io":{"kind":"unknown","raw_os_error":null,"payload":"NoCustomPayload"}}"#
                .as_slice(),
            br#"{"Seccompiler":{"Prctl":{"kind":"unknown","raw_os_error":"#,
        ] {
            let error = refuses(bytes, CauseIntegrityPredicate::UnknownKindMetadata);
            assert_eq!(error.cause(), DecodeCause::InvalidValue);
            assert!(std::error::Error::source(&error).is_none());
        }
        let mut fault = Some(CauseIntegrityPredicate::UnknownKindMetadata);
        assert!(parse(br#"{"Io":null}"#, &mut fault).is_err());
        assert_eq!(fault, Some(CauseIntegrityPredicate::UnknownKindMetadata));
        let mut fault = Some(CauseIntegrityPredicate::MalformedCauseMetadata);
        assert!(parse(br#"{"Io":{"kind":"unknown"}}"#, &mut fault).is_err());
        assert_eq!(fault, Some(CauseIntegrityPredicate::MalformedCauseMetadata));
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn every_io_field_refuses_duplicates_and_wrong_types_without_unknown_kind_inference() {
        for bytes in [
            br#"{"Io":{"kind":null,"raw_os_error":null,"payload":"NoCustomPayload"}}"#.as_slice(),
            br#"{"Io":{"kind":1,"raw_os_error":null,"payload":"NoCustomPayload"}}"#,
            br#"{"Io":{"kind":"Other","raw_os_error":"bad","payload":"NoCustomPayload"}}"#,
            br#"{"Io":{"kind":"Other","raw_os_error":false,"payload":"NoCustomPayload"}}"#,
            br#"{"Io":{"kind":"Other","raw_os_error":null,"payload":null}}"#,
            br#"{"Io":{"kind":"Other","raw_os_error":null,"payload":"unknown"}}"#,
            br#"{"Io":{"kind":"Other","kind":"Other","raw_os_error":null,"payload":"NoCustomPayload"}}"#,
            br#"{"Io":{"kind":"Other","raw_os_error":null,"raw_os_error":null,"payload":"NoCustomPayload"}}"#,
            br#"{"Io":{"kind":"Other","raw_os_error":null,"payload":"NoCustomPayload","payload":"NoCustomPayload"}}"#,
            br#"{"Io":{"kind":"Other","raw_os_error":null,"payload":"NoCustomPayload","extra":0}}"#,
        ] {
            refuses(bytes, CauseIntegrityPredicate::MalformedCauseMetadata);
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn signed_errno_width_is_checked_without_normalizing_semantic_metadata() {
        for raw in [i32::MIN, i32::MAX, 0] {
            let expected = StartupCause::Io(io(
                StartupIoKind::Other,
                Some(raw),
                StartupPayload::UnrepresentedCustom,
            ));
            let mut fault = None;
            assert_eq!(
                parse(&serde_json::to_vec(&expected).unwrap(), &mut fault).unwrap(),
                expected
            );
            assert_eq!(fault, None);
        }
        for raw in [
            "2147483648",
            "-2147483649",
            "9223372036854775808",
            "1.0",
            "1e0",
        ] {
            let bytes = format!(
                r#"{{"Io":{{"kind":"Other","raw_os_error":{raw},"payload":"NoCustomPayload"}}}}"#
            );
            refuses(
                bytes.as_bytes(),
                CauseIntegrityPredicate::MalformedCauseMetadata,
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn native_threadsync_width_and_exact_single_pid_record_are_checked() {
        for pid in [c_long::MIN, c_long::MAX, 0] {
            let expected = StartupCause::Seccompiler(StartupSeccompilerCause::ThreadSync { pid });
            let mut fault = None;
            assert_eq!(
                parse(&serde_json::to_vec(&expected).unwrap(), &mut fault).unwrap(),
                expected
            );
            assert_eq!(fault, None);
        }
        for bytes in [
            br#"{"Seccompiler":{"ThreadSync":{"pid":null}}}"#.as_slice(),
            br#"{"Seccompiler":{"ThreadSync":{"pid":"7"}}}"#,
            br#"{"Seccompiler":{"ThreadSync":{"pid":9223372036854775808}}}"#,
            br#"{"Seccompiler":{"ThreadSync":{"pid":7,"pid":7}}}"#,
            br#"{"Seccompiler":{"ThreadSync":{"pid":7,"extra":0}}}"#,
        ] {
            refuses(bytes, CauseIntegrityPredicate::MalformedCauseMetadata);
        }
        if size_of::<c_long>() < size_of::<i64>() {
            refuses(
                br#"{"Seccompiler":{"ThreadSync":{"pid":2147483648}}}"#,
                CauseIntegrityPredicate::MalformedCauseMetadata,
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn external_variants_preserve_unit_string_only_and_refuse_conflicting_shapes() {
        for bytes in [
            b"null".as_slice(),
            b"[]",
            br#"{"Io":null}"#,
            br#"{"Seccompiler":"Prctl"}"#,
            br#"{"Seccompiler":"unknown"}"#,
            br#"{"Seccompiler":{"EmptyFilter":null}}"#,
            br#"{"Seccompiler":{"unknown":null}}"#,
            br#"{"Seccompiler":{"ThreadSync":{"pid":7},"Prctl":{}}}"#,
            br#"{"Seccompiler":"EmptyFilter","Io":{}}"#,
        ] {
            refuses(bytes, CauseIntegrityPredicate::MalformedCauseMetadata);
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn partial_cause_is_incomplete_but_envelope_trailing_bytes_are_not_reclassified() {
        for bytes in [
            b"{".as_slice(),
            br#"{"Io":{"kind":"#,
            br#"{"Io":{"kind":"Other","raw_os_error":null,"payload":"NoCustomPayload"}"#,
        ] {
            assert_eq!(
                refuses(bytes, CauseIntegrityPredicate::IncompleteCauseMetadata).cause(),
                DecodeCause::UnexpectedEnd
            );
        }
        let expected = StartupCause::Seccompiler(StartupSeccompilerCause::EmptyFilter);
        let mut bytes = serde_json::to_vec(&expected).unwrap();
        bytes.extend_from_slice(b" true");
        let mut fault = None;
        assert_eq!(
            parse(&bytes, &mut fault).unwrap_err().cause(),
            DecodeCause::TrailingBytes
        );
        assert_eq!(fault, None);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn direct_io_entry_consumes_one_record_and_preserves_containing_cursor() {
        let expected = io(StartupIoKind::Other, None, StartupPayload::NoCustomPayload);
        let mut bytes = b"[".to_vec();
        serde_json::to_writer(&mut bytes, &expected).unwrap();
        bytes.extend_from_slice(b",7]");
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(&bytes, &mut scratch).unwrap();
        let mut array = decoder.begin_array().unwrap();
        assert!(decoder.next_element(&mut array).unwrap());
        let mut fault = None;
        assert_eq!(decode_io(&mut decoder, &mut fault).unwrap(), expected);
        assert_eq!(fault, None);
        assert!(decoder.next_element(&mut array).unwrap());
        assert_eq!(decoder.byte().unwrap(), 7);
        assert!(!decoder.next_element(&mut array).unwrap());
        decoder.finish().unwrap();
    }
}

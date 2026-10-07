// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Strict negative-envelope grammar over one caller-owned decoder and scratch (FR-034).
//!
//! Headers are unverified parsed facts. This module owns no authentication, clock, phase,
//! projection, frame or settlement. Required context bytes are distinct from optional UTF-8
//! diagnostic retention. The owner retains the actual fixed error and first checking predicate.

use std::mem::{size_of, size_of_val};

use super::{
    cause_decode,
    cross_role_cause::{CauseIntegrityPredicate, CauseOperation},
    guardian_decode::{DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Scratch, Text},
    outer_failure::{FailureHeader, FailureRepresentation, FailureRepresentationTag, FailureState},
    protocol::{BuildIdentity, RunAuthority},
    role_deadline::StopStamp,
    role_scalar_decode,
    startup_cause::{PreparedStartupContext, StartupCause},
};

fn field_error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}

fn missing<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| field_error(DecodeCause::MissingField))
}

// These selectors reflect record member names only; owning enums supply variant inventories.
macro_rules! fields {
    ($name:ident { $($variant:ident => $label:literal),+ $(,)? }) => {
        #[derive(Clone, Copy)]
        enum $name { $($variant),+ }
        impl $name {
            fn from_text(text: Text<'_>) -> Result<Self, DecodeError> {
                $(if text.equals($label) { return Ok(Self::$variant); })+
                Err(field_error(DecodeCause::UnknownField))
            }
        }
    };
}

macro_rules! read_once {
    ($slot:expr, $value:expr) => {{
        if $slot.is_some() {
            return Err(field_error(DecodeCause::DuplicateField));
        }
        $slot = Some($value?);
    }};
}

fields!(CommitField { Kind => "kind", Identity => "identity", Authority => "authority", Stop => "stop", Disposition => "disposition" });
fields!(DispositionField { Kind => "kind", Operation => "operation", State => "state", Representation => "representation", Context => "context" });
fields!(StateField { ObservationAdmitted => "observation_admitted", OriginalWorkExpired => "original_work_expired" });

#[derive(Default)]
struct CommitFields {
    kind: Option<()>,
    identity: Option<BuildIdentity>,
    authority: Option<RunAuthority>,
    stop: Option<StopStamp>,
    disposition: Option<Disposition>,
}

#[derive(Default)]
struct DispositionFields {
    kind: Option<()>,
    operation: Option<CauseOperation>,
    state: Option<FailureState>,
    representation: Option<FailureRepresentation>,
    context: Option<()>,
}

#[derive(Default)]
struct StateFields {
    observation_admitted: Option<bool>,
    original_work_expired: Option<bool>,
}

struct Disposition {
    operation: CauseOperation,
    state: FailureState,
    representation: FailureRepresentation,
}

fn tag(decoder: &mut Decoder<'_, '_>, expected: &str) -> Result<(), DecodeError> {
    if decoder.string()?.equals(expected) {
        Ok(())
    } else {
        Err(field_error(DecodeCause::InvalidValue))
    }
}

/// Parse the entire current negative schema; no returned fact authorizes a transaction.
pub(super) fn decode(
    payload: &[u8],
    scratch: &mut Scratch,
    context: &mut PreparedStartupContext,
) -> Result<FailureHeader, DecodeError> {
    context.clear();
    let result = decode_inner(payload, scratch, context);
    if let Err(error) = result {
        let predicate = match error.cause() {
            DecodeCause::MissingField | DecodeCause::UnexpectedEnd => {
                Some(CauseIntegrityPredicate::IncompleteCauseMetadata)
            }
            DecodeCause::InputBound | DecodeCause::StorageBound => None,
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
            | DecodeCause::InvalidValue => Some(CauseIntegrityPredicate::MalformedCauseMetadata),
        };
        let slot = context.metadata_fault_slot();
        if slot.is_none() {
            *slot = predicate;
        }
        return Err(error);
    }
    result
}

fn decode_inner(
    payload: &[u8],
    scratch: &mut Scratch,
    context: &mut PreparedStartupContext,
) -> Result<FailureHeader, DecodeError> {
    let mut decoder = Decoder::new(payload, scratch)?;
    let mut object = decoder.begin_object()?;
    let mut fields = CommitFields::default();
    while let Some(name) = decoder.next_field(&mut object)? {
        match CommitField::from_text(name)? {
            CommitField::Kind => read_once!(fields.kind, tag(&mut decoder, "Committed")),
            CommitField::Identity => {
                read_once!(fields.identity, role_scalar_decode::identity(&mut decoder))
            }
            CommitField::Authority => read_once!(
                fields.authority,
                role_scalar_decode::authority(&mut decoder)
            ),
            CommitField::Stop => read_once!(fields.stop, role_scalar_decode::stop(&mut decoder)),
            CommitField::Disposition => {
                read_once!(fields.disposition, disposition(&mut decoder, context))
            }
        }
    }
    missing(fields.kind)?;
    let disposition = missing(fields.disposition)?;
    let header = FailureHeader {
        identity: missing(fields.identity)?,
        authority: missing(fields.authority)?,
        stop: missing(fields.stop)?,
        operation: disposition.operation,
        state: disposition.state,
        representation: disposition.representation,
    };
    decoder.finish()?;
    Ok(header)
}

fn disposition(
    decoder: &mut Decoder<'_, '_>,
    context: &mut PreparedStartupContext,
) -> Result<Disposition, DecodeError> {
    let mut object = decoder.begin_object()?;
    let mut fields = DispositionFields::default();
    while let Some(name) = decoder.next_field(&mut object)? {
        match DispositionField::from_text(name)? {
            DispositionField::Kind => read_once!(fields.kind, tag(decoder, "OperationalFailure")),
            DispositionField::Operation => {
                read_once!(fields.operation, role_scalar_decode::operation(decoder))
            }
            DispositionField::State => read_once!(fields.state, state(decoder)),
            DispositionField::Representation => {
                read_once!(fields.representation, representation(decoder, context))
            }
            DispositionField::Context => {
                read_once!(fields.context, context.decode_context(decoder))
            }
        }
    }
    missing(fields.kind)?;
    missing(fields.context)?;
    Ok(Disposition {
        operation: missing(fields.operation)?,
        state: missing(fields.state)?,
        representation: missing(fields.representation)?,
    })
}

fn state(decoder: &mut Decoder<'_, '_>) -> Result<FailureState, DecodeError> {
    let mut object = decoder.begin_object()?;
    let mut fields = StateFields::default();
    while let Some(name) = decoder.next_field(&mut object)? {
        match StateField::from_text(name)? {
            StateField::ObservationAdmitted => {
                read_once!(fields.observation_admitted, decoder.boolean())
            }
            StateField::OriginalWorkExpired => {
                read_once!(fields.original_work_expired, decoder.boolean())
            }
        }
    }
    Ok(FailureState {
        observation_admitted: missing(fields.observation_admitted)?,
        original_work_expired: missing(fields.original_work_expired)?,
    })
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

fn representation(
    decoder: &mut Decoder<'_, '_>,
    context: &mut PreparedStartupContext,
) -> Result<FailureRepresentation, DecodeError> {
    let mut outer = decoder.begin_object()?;
    let tag = FailureRepresentationTag::metadata_text(required(decoder, &mut outer)?)
        .ok_or_else(|| field_error(DecodeCause::UnknownField))?;
    let mut body = decoder.begin_object()?;
    let field = required(decoder, &mut body)?;
    let representation = match tag {
        FailureRepresentationTag::Original => {
            if !field.equals("cause") {
                return Err(field_error(DecodeCause::UnknownField));
            }
            let cause = cause_decode::decode(decoder, context.metadata_fault_slot())?;
            match cause {
                StartupCause::Io(_) => FailureRepresentation::Original { cause },
                StartupCause::Seccompiler(_) => return Err(field_error(DecodeCause::InvalidValue)),
            }
        }
        FailureRepresentationTag::Integrity => {
            if !field.equals("predicate") {
                return Err(field_error(DecodeCause::UnknownField));
            }
            FailureRepresentation::Integrity {
                predicate: role_scalar_decode::predicate(decoder)?,
            }
        }
    };
    close_single(decoder, &mut body)?;
    close_single(decoder, &mut outer)?;
    Ok(representation)
}

/// Checked additional fixed schema/context staging records, excluding separately charged storage.
/// Root/disposition/representation/body are a fixed call graph. Cause/scalar decoder deltas and
/// the primitive's one object/text/array are charged separately; no payload-driven recursion.
/// Compiler temporaries, initializer transients and actual native-stack highwater are unmeasured.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<CommitFields>(),
        size_of::<DispositionFields>(),
        size_of::<StateFields>(),
        size_of::<CommitField>(),
        size_of::<DispositionField>(),
        size_of::<StateField>(),
        size_of::<Disposition>(),
        size_of::<FailureHeader>(),
        size_of::<FailureState>(),
        size_of::<FailureRepresentation>(),
        size_of::<FailureRepresentationTag>(),
        // Four schema object states and four names, less the primitive's first object/text.
        size_of::<ObjectState>(),
        size_of::<ObjectState>(),
        size_of::<ObjectState>(),
        size_of::<Text<'static>>(),
        size_of::<Text<'static>>(),
        size_of::<Text<'static>>(),
        size_of::<&mut Decoder<'static, 'static>>(),
        size_of::<&mut PreparedStartupContext>(),
        size_of::<&str>(),
        size_of::<Result<FailureHeader, DecodeError>>(),
        size_of::<Result<Disposition, DecodeError>>(),
        size_of::<Result<FailureState, DecodeError>>(),
        size_of::<Result<FailureRepresentation, DecodeError>>(),
        size_of::<Option<Text<'static>>>(),
        size_of::<Option<CauseIntegrityPredicate>>(),
        // Actual inline decode_context staging. ArrayState itself is already primitive-charged.
        size_of::<[u8; 4]>(),
        size_of::<usize>(),
        size_of::<bool>(),
        size_of::<u8>(),
        size_of::<&mut u8>(),
        size_of::<&[u8]>(),
        size_of::<&str>(),
        size_of::<Result<&str, std::str::Utf8Error>>(),
        size_of::<usize>(),
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
    // Grammar units only: serialization supplies parsed data, never owner authentication.
    use std::io;

    use super::*;
    use crate::kani::run::{
        outer_failure::NegativeCommit,
        protocol::current_build_identity,
        role_deadline::{MonotonicInstant, StopOrigin},
        startup_cause::StartupSeccompilerCause,
    };

    fn header() -> FailureHeader {
        FailureHeader {
            identity: current_build_identity(),
            authority: RunAuthority::from_wire_bytes([7; 32]),
            stop: StopStamp::from_wire_parts(
                StopOrigin::Outer,
                MonotonicInstant::from_wire_parts(3, 4),
            ),
            operation: CauseOperation::ProcSetup,
            state: FailureState {
                observation_admitted: false,
                original_work_expired: true,
            },
            representation: FailureRepresentation::Original {
                cause: StartupCause::capture_io(&io::Error::from_raw_os_error(1)).unwrap(),
            },
        }
    }

    fn value() -> serde_json::Value {
        serde_json::to_value(NegativeCommit::new(header(), b"diagnostic")).unwrap()
    }

    fn refuses(value: &serde_json::Value, expected: CauseIntegrityPredicate) {
        let mut context = PreparedStartupContext::new(16).unwrap();
        let mut scratch = Scratch::default();
        let bytes = serde_json::to_vec(value).unwrap();
        assert!(decode(&bytes, &mut scratch, &mut context).is_err());
        assert_eq!(*context.metadata_fault_slot(), Some(expected));
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn serialized_negative_variants_retain_exact_header_and_optional_context() {
        let mut scratch = Scratch::default();
        let mut context = PreparedStartupContext::new(16).unwrap();
        let reserved = context.reserved_bytes();
        let original = header();
        let integrity = FailureHeader {
            representation: FailureRepresentation::Integrity {
                predicate: CauseIntegrityPredicate::RequiredRepresentationFormattingFailed,
            },
            ..original
        };
        let no_errno = FailureHeader {
            representation: FailureRepresentation::Original {
                cause: StartupCause::capture_io(&io::Error::from(io::ErrorKind::InvalidData))
                    .unwrap(),
            },
            ..original
        };
        for expected in [original, integrity, no_errno] {
            let bytes = serde_json::to_vec(&NegativeCommit::new(expected, b"actual text")).unwrap();
            assert_eq!(
                decode(&bytes, &mut scratch, &mut context).unwrap(),
                expected
            );
            assert_eq!(context.context(), "actual text");
            assert_eq!(context.reserved_bytes(), reserved);
            assert!(context.metadata_fault_slot().is_none());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn required_members_refuse_missing_null_duplicate_unknown_and_wrong_types() {
        // Every member of the newly owned records has the same independent adverse controls.
        // Identity/time/cause subrecords retain their separately owned grammar tests.
        let records = [
            (
                "",
                &["kind", "identity", "authority", "stop", "disposition"][..],
            ),
            (
                "/disposition",
                &["kind", "operation", "state", "representation", "context"][..],
            ),
            (
                "/disposition/state",
                &["observation_admitted", "original_work_expired"][..],
            ),
            ("/disposition/representation/Original", &["cause"][..]),
        ];
        for (path, members) in records {
            for member in members {
                let mut absent = value();
                absent
                    .pointer_mut(path)
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .remove(*member);
                refuses(&absent, CauseIntegrityPredicate::IncompleteCauseMetadata);
                for wrong in [serde_json::Value::Null, serde_json::json!(42)] {
                    let mut malformed = value();
                    malformed.pointer_mut(path).unwrap()[*member] = wrong;
                    refuses(&malformed, CauseIntegrityPredicate::MalformedCauseMetadata);
                }
                let mut duplicate = value();
                // Raw duplicate JSON is necessary: a Value map cannot contain duplicate keys.
                let record = duplicate.pointer_mut(path).unwrap();
                let original = serde_json::to_string(record).unwrap();
                let member_value = serde_json::to_string(&record[*member]).unwrap();
                let doubled = original.replacen('{', &format!("{{\"{member}\":{member_value},"), 1);
                let whole = serde_json::to_string(&duplicate)
                    .unwrap()
                    .replacen(&original, &doubled, 1);
                let mut scratch = Scratch::default();
                let mut context = PreparedStartupContext::new(16).unwrap();
                assert_eq!(
                    decode(whole.as_bytes(), &mut scratch, &mut context)
                        .unwrap_err()
                        .cause(),
                    DecodeCause::DuplicateField
                );
            }
            let mut unknown = value();
            unknown.pointer_mut(path).unwrap()["foreign"] = serde_json::json!(true);
            refuses(&unknown, CauseIntegrityPredicate::MalformedCauseMetadata);
        }
        let mut wrong_tag = value();
        wrong_tag["kind"] = serde_json::json!("Completed");
        refuses(&wrong_tag, CauseIntegrityPredicate::MalformedCauseMetadata);
        wrong_tag = value();
        wrong_tag["disposition"]["kind"] = serde_json::json!("Report");
        refuses(&wrong_tag, CauseIntegrityPredicate::MalformedCauseMetadata);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn representation_requires_one_variant_and_io_only_original_with_explicit_errno() {
        let mut absent_errno = value();
        absent_errno["disposition"]["representation"]["Original"]["cause"]["Io"]
            .as_object_mut()
            .unwrap()
            .remove("raw_os_error");
        refuses(
            &absent_errno,
            CauseIntegrityPredicate::IncompleteCauseMetadata,
        );
        let non_io = FailureHeader {
            representation: FailureRepresentation::Original {
                cause: StartupCause::Seccompiler(StartupSeccompilerCause::EmptyFilter),
            },
            ..header()
        };
        refuses(
            &serde_json::to_value(NegativeCommit::new(non_io, &[])).unwrap(),
            CauseIntegrityPredicate::MalformedCauseMetadata,
        );
        for representation in [serde_json::json!({}), serde_json::json!({"Integrity": {}})] {
            let mut changed = value();
            changed["disposition"]["representation"] = representation;
            refuses(&changed, CauseIntegrityPredicate::IncompleteCauseMetadata);
        }
        for representation in [
            serde_json::json!({"Integrity": {"predicate": null}}),
            serde_json::json!({"Integrity": {"predicate": "UnknownPredicate"}}),
            serde_json::json!({"Other": {}}),
            serde_json::json!({"Integrity": {"predicate": "MalformedCauseMetadata", "extra": 1}}),
            serde_json::json!({"Original": value()["disposition"]["representation"]["Original"].clone(), "Integrity": {"predicate": "MalformedCauseMetadata"}}),
        ] {
            let mut changed = value();
            changed["disposition"]["representation"] = representation;
            refuses(&changed, CauseIntegrityPredicate::MalformedCauseMetadata);
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn first_unknown_kind_precedes_later_malformed_context_and_trailing_syntax() {
        // Explicit ordering puts the cause before context; no selector pre-scan may mask it.
        let original = value();
        let mut disposition = original["disposition"].clone();
        disposition["representation"]["Original"]["cause"]["Io"]["kind"] =
            serde_json::json!("UnknownKind");
        let bytes = format!(
            "{{\"kind\":\"Committed\",\"identity\":{},\"authority\":{},\"stop\":{},\"disposition\":{{\"kind\":\"OperationalFailure\",\"operation\":{},\"state\":{},\"representation\":{},\"context\":[false]}}}}?",
            original["identity"], original["authority"], original["stop"], disposition["operation"], disposition["state"], disposition["representation"],
        );
        let mut scratch = Scratch::default();
        let mut context = PreparedStartupContext::new(16).unwrap();
        assert_eq!(
            decode(bytes.as_bytes(), &mut scratch, &mut context)
                .unwrap_err()
                .cause(),
            DecodeCause::InvalidValue
        );
        assert_eq!(
            *context.metadata_fault_slot(),
            Some(CauseIntegrityPredicate::UnknownKindMetadata)
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn omitted_diagnostics_still_require_all_bytes_and_complete_envelope() {
        let mut scratch = Scratch::default();
        let mut context = PreparedStartupContext::new(2).unwrap();
        for diagnostic in [&[255_u8][..], b"overbound", &[0xe2, 0x82][..]] {
            let bytes = serde_json::to_vec(&NegativeCommit::new(header(), diagnostic)).unwrap();
            assert_eq!(
                decode(&bytes, &mut scratch, &mut context).unwrap(),
                header()
            );
            assert!(context.context().is_empty());
            assert!(context.metadata_fault_slot().is_none());
        }
        for diagnostic in [
            serde_json::json!([255, false]),
            serde_json::json!([255, 256]),
            serde_json::json!([255, -1]),
        ] {
            let mut changed = value();
            changed["disposition"]["context"] = diagnostic;
            refuses(&changed, CauseIntegrityPredicate::MalformedCauseMetadata);
        }
        let bytes = serde_json::to_vec(&NegativeCommit::new(header(), &[255])).unwrap();
        let encoded = std::str::from_utf8(&bytes).unwrap();
        for malformed in [
            encoded.replace("[255]", "[255}"),
            encoded.replace("[255]", "[255,]"),
            encoded.replacen("\"context\":[255]", "\"context\":[255],\"extra\":0", 1),
        ] {
            assert!(decode(malformed.as_bytes(), &mut scratch, &mut context).is_err());
            assert_eq!(
                *context.metadata_fault_slot(),
                Some(CauseIntegrityPredicate::MalformedCauseMetadata)
            );
        }
        let mut trailing = bytes.clone();
        trailing.push(b'?');
        assert_eq!(
            decode(&trailing, &mut scratch, &mut context)
                .unwrap_err()
                .cause(),
            DecodeCause::TrailingBytes
        );
        assert_eq!(
            *context.metadata_fault_slot(),
            Some(CauseIntegrityPredicate::MalformedCauseMetadata)
        );
        let incomplete = bytes.get(..bytes.len() - 1).unwrap();
        assert_eq!(
            decode(incomplete, &mut scratch, &mut context)
                .unwrap_err()
                .cause(),
            DecodeCause::UnexpectedEnd
        );
        assert_eq!(
            *context.metadata_fault_slot(),
            Some(CauseIntegrityPredicate::IncompleteCauseMetadata)
        );
        let mut width = value();
        width["authority"][0] = serde_json::json!(256);
        refuses(&width, CauseIntegrityPredicate::MalformedCauseMetadata);
    }
}

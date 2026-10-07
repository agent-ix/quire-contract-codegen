// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Borrowed settings and raw bootstrap scalar facts on the original control cursor.
//!
//! Parsing grants no clock, process, descriptor or admission authority. The owner authenticates
//! the enclosing frame before materializing the UTF-8 helper path. Only IdentityDeadline's
//! source-selected internal-tag content is staged as a borrowed span for late tag selection.

use std::{
    mem::{size_of, size_of_val},
    num::NonZeroU64,
    time::Duration,
};

use super::{
    guardian_decode::{
        record, ArrayState, DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Text,
        ValueKind, ValueSlice,
    },
    protocol::{BuildIdentity, RunAuthority},
    report_storage::{PipeIdentity, PipeIdentityField},
    role_deadline::{IdentityDeadline, IdentityDeadlineKind, MonotonicInstant, RoleDeadline},
    role_protocol::SettingsField,
    role_scalar_decode,
};

/// Original borrowed UTF-8 helper and parsed settings; no owned pathname or authority is made.
#[derive(Clone, Copy)]
pub(super) struct SettingsBytes<'input> {
    pub(super) helper: Text<'input>,
    pub(super) identity: BuildIdentity,
    pub(super) authority: RunAuthority,
    pub(super) deadline: IdentityDeadline,
    pub(super) started: MonotonicInstant,
    pub(super) settlement_reserve: Duration,
    pub(super) work_deadline: IdentityDeadline,
    pub(super) setup_deadline: RoleDeadline,
    pub(super) caller_uid: u32,
    pub(super) caller_gid: u32,
    pub(super) memory_bytes: NonZeroU64,
    pub(super) caller_run_buffers: u64,
}

#[derive(Default)]
struct SettingsFields<'input> {
    helper: Option<Text<'input>>,
    identity: Option<BuildIdentity>,
    authority: Option<RunAuthority>,
    deadline: Option<IdentityDeadline>,
    started: Option<MonotonicInstant>,
    settlement_reserve: Option<Duration>,
    work_deadline: Option<IdentityDeadline>,
    setup_deadline: Option<RoleDeadline>,
    caller_uid: Option<u32>,
    caller_gid: Option<u32>,
    memory_bytes: Option<NonZeroU64>,
    caller_run_buffers: Option<u64>,
}

fn error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}

fn required<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| error(DecodeCause::MissingField))
}

macro_rules! once {
    ($slot:expr, $value:expr) => {{
        if $slot.is_some() {
            return Err(error(DecodeCause::DuplicateField));
        }
        $slot = Some($value?);
    }};
}

fn unsigned32(decoder: &mut Decoder<'_, '_>) -> Result<u32, DecodeError> {
    u32::try_from(decoder.unsigned()?)
        .map_err(|_| DecodeError::new(DecodeSite::Unsigned, DecodeCause::IntegerOverflow))
}

fn settings_field<'input>(
    decoder: &mut Decoder<'input, '_>,
    field: SettingsField,
    fields: &mut SettingsFields<'input>,
) -> Result<(), DecodeError> {
    match field {
        SettingsField::Helper => once!(fields.helper, decoder.string()),
        SettingsField::Identity => once!(fields.identity, role_scalar_decode::identity(decoder)),
        SettingsField::Authority => once!(fields.authority, role_scalar_decode::authority(decoder)),
        SettingsField::Deadline => once!(fields.deadline, identity_deadline(decoder)),
        SettingsField::Started => once!(fields.started, role_scalar_decode::monotonic(decoder)),
        SettingsField::SettlementReserve => once!(fields.settlement_reserve, duration(decoder)),
        SettingsField::WorkDeadline => once!(fields.work_deadline, identity_deadline(decoder)),
        SettingsField::SetupDeadline => {
            once!(fields.setup_deadline, role_scalar_decode::deadline(decoder))
        }
        SettingsField::CallerUid => once!(fields.caller_uid, unsigned32(decoder)),
        SettingsField::CallerGid => once!(fields.caller_gid, unsigned32(decoder)),
        SettingsField::MemoryBytes => once!(
            fields.memory_bytes,
            NonZeroU64::new(decoder.unsigned()?).ok_or_else(|| error(DecodeCause::InvalidValue))
        ),
        SettingsField::CallerRunBuffers => once!(fields.caller_run_buffers, decoder.unsigned()),
    }
    Ok(())
}

/// Consume one ordinary settings object or exact positional sequence using its owning catalog.
pub(super) fn settings<'input>(
    decoder: &mut Decoder<'input, '_>,
) -> Result<SettingsBytes<'input>, DecodeError> {
    let mut fields = SettingsFields::default();
    record(
        decoder,
        SettingsField::declared_order(),
        SettingsField::metadata_text,
        |decoder, field| settings_field(decoder, field, &mut fields),
    )?;
    Ok(SettingsBytes {
        helper: required(fields.helper)?,
        identity: required(fields.identity)?,
        authority: required(fields.authority)?,
        deadline: required(fields.deadline)?,
        started: required(fields.started)?,
        settlement_reserve: required(fields.settlement_reserve)?,
        work_deadline: required(fields.work_deadline)?,
        setup_deadline: required(fields.setup_deadline)?,
        caller_uid: required(fields.caller_uid)?,
        caller_gid: required(fields.caller_gid)?,
        memory_bytes: required(fields.memory_bytes)?,
        caller_run_buffers: required(fields.caller_run_buffers)?,
    })
}

#[derive(Default)]
struct DeadlineFields<'input> {
    kind: Option<IdentityDeadlineKind>,
    deadline: Option<ValueSlice<'input>>,
    body_fault: Option<DecodeCause>,
    extra_sequence: bool,
}

fn deadline_kind(decoder: &mut Decoder<'_, '_>) -> Result<IdentityDeadlineKind, DecodeError> {
    IdentityDeadlineKind::metadata_text(decoder.string()?)
        .ok_or_else(|| error(DecodeCause::InvalidValue))
}

fn deadline_map<'input>(
    decoder: &mut Decoder<'input, '_>,
    fields: &mut DeadlineFields<'input>,
) -> Result<(), DecodeError> {
    let mut object = decoder.begin_object()?;
    while let Some(name) = decoder.next_field(&mut object)? {
        if name.equals("kind") {
            once!(fields.kind, deadline_kind(decoder));
        } else {
            // TaggedContentVisitor consumes arbitrary content before selecting the variant.
            // Only this no-cause schema permits staging; it grants no fault-slot precedence.
            // Selected Content depth is preserved; numeric conversion parity remains unproven.
            let value = decoder.content_value()?;
            let known = IdentityDeadlineKind::Finite
                .declared_fields()
                .iter()
                .any(|field| name.equals(field));
            let fault = if known {
                if fields.deadline.is_some() {
                    Some(DecodeCause::DuplicateField)
                } else {
                    fields.deadline = Some(value);
                    None
                }
            } else {
                Some(DecodeCause::UnknownField)
            };
            if fields.body_fault.is_none() {
                fields.body_fault = fault;
            }
        }
    }
    Ok(())
}

fn deadline_sequence<'input>(
    decoder: &mut Decoder<'input, '_>,
    fields: &mut DeadlineFields<'input>,
) -> Result<(), DecodeError> {
    let mut array = decoder.begin_array()?;
    if !decoder.next_element(&mut array)? {
        return Err(error(DecodeCause::MissingField));
    }
    fields.kind = Some(deadline_kind(decoder)?);
    while decoder.next_element(&mut array)? {
        let value = decoder.content_value()?;
        if fields.deadline.is_none() {
            fields.deadline = Some(value);
        } else {
            fields.extra_sequence = true;
        }
    }
    Ok(())
}

/// Parse internal-tag content, then select a single borrowed child on the SAME workspace.
/// NeverElapses ignores map members, but its enclosing sequence end refuses any remainder.
pub(super) fn identity_deadline(
    decoder: &mut Decoder<'_, '_>,
) -> Result<IdentityDeadline, DecodeError> {
    let mut fields = DeadlineFields::default();
    let sequence = match decoder.peek_kind()? {
        ValueKind::Object => {
            deadline_map(decoder, &mut fields)?;
            false
        }
        ValueKind::Array => {
            deadline_sequence(decoder, &mut fields)?;
            true
        }
        ValueKind::String | ValueKind::Number | ValueKind::Boolean | ValueKind::Null => {
            return Err(error(DecodeCause::UnexpectedToken));
        }
    };
    match required(fields.kind)? {
        IdentityDeadlineKind::NeverElapses => {
            if sequence && fields.deadline.is_some() {
                return Err(error(DecodeCause::InvalidValue));
            }
            Ok(IdentityDeadline::NeverElapses)
        }
        IdentityDeadlineKind::Finite => {
            if let Some(fault) = fields.body_fault {
                return Err(error(fault));
            }
            if fields.extra_sequence {
                return Err(error(DecodeCause::InvalidValue));
            }
            let mut child = decoder.nested(required(fields.deadline)?)?;
            let deadline = role_scalar_decode::deadline(&mut child)?;
            child.finish()?;
            Ok(IdentityDeadline::Finite { deadline })
        }
    }
}

#[derive(Clone, Copy)]
enum DurationField {
    Secs,
    Nanos,
}

#[derive(Default)]
struct DurationFields {
    secs: Option<u64>,
    nanos: Option<u32>,
}

/// Consume serde's Duration record, checking carry overflow before its normalizing constructor.
pub(super) fn duration(decoder: &mut Decoder<'_, '_>) -> Result<Duration, DecodeError> {
    let mut fields = DurationFields::default();
    record(
        decoder,
        &[DurationField::Secs, DurationField::Nanos],
        |name| {
            if name.equals("secs") {
                Some(DurationField::Secs)
            } else if name.equals("nanos") {
                Some(DurationField::Nanos)
            } else {
                None
            }
        },
        |decoder, field| {
            match field {
                DurationField::Secs => once!(fields.secs, decoder.unsigned()),
                DurationField::Nanos => once!(fields.nanos, unsigned32(decoder)),
            }
            Ok(())
        },
    )?;
    let secs = required(fields.secs)?;
    let nanos = required(fields.nanos)?;
    secs.checked_add(u64::from(nanos / 1_000_000_000))
        .ok_or_else(|| error(DecodeCause::IntegerOverflow))?;
    Ok(Duration::new(secs, nanos))
}

#[derive(Default)]
struct PipeFields {
    device: Option<u64>,
    inode: Option<u64>,
}

/// Parsed pipe numbers only; actual descriptor verification remains with its owner.
pub(super) fn pipe_identity(decoder: &mut Decoder<'_, '_>) -> Result<PipeIdentity, DecodeError> {
    let mut fields = PipeFields::default();
    record(
        decoder,
        PipeIdentityField::declared_order(),
        PipeIdentityField::metadata_text,
        |decoder, field| {
            match field {
                PipeIdentityField::Device => once!(fields.device, decoder.unsigned()),
                PipeIdentityField::Inode => once!(fields.inode, decoder.unsigned()),
            }
            Ok(())
        },
    )?;
    Ok(PipeIdentity::from_wire_parts(
        required(fields.device)?,
        required(fields.inode)?,
    ))
}

/// Checked additional fixed schema layouts, not a compiler stack or allocation highwater proof.
/// Existing Scratch/frame/output capacities and reused scalar/primitive schema reservations
/// are excluded. Settings -> identity deadline -> one sequential raw-deadline child is the fixed
/// deepest selected path; generic nested content uses the primitive's existing iterative scan.
/// Initializer/return-place/copy transients and actual native stack remain unmeasured.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<SettingsBytes<'static>>(),
        size_of::<SettingsFields<'static>>(),
        size_of::<DeadlineFields<'static>>(),
        size_of::<DurationFields>(),
        size_of::<PipeFields>(),
        size_of::<SettingsField>(),
        size_of::<IdentityDeadlineKind>(),
        size_of::<DurationField>(),
        size_of::<PipeIdentityField>(),
        size_of::<Result<SettingsBytes<'static>, DecodeError>>(),
        size_of::<Result<IdentityDeadline, DecodeError>>(),
        size_of::<Result<Duration, DecodeError>>(),
        size_of::<Result<PipeIdentity, DecodeError>>(),
        size_of::<Option<DecodeCause>>(),
        size_of::<Option<ValueSlice<'static>>>(),
        size_of::<Text<'static>>(),
        size_of::<ObjectState>(),
        size_of::<ArrayState>(),
        size_of::<Decoder<'static, 'static>>(),
        size_of::<&mut Decoder<'static, 'static>>(),
        size_of::<&SettingsField>(),
        size_of::<&str>(),
        size_of::<u64>(),
        size_of::<u32>(),
        size_of::<bool>(),
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
    use super::*;
    use crate::kani::run::{
        guardian_decode::Scratch, protocol::current_build_identity, role_protocol::RunSettings,
    };
    use serde_json::{json, Value};

    fn original() -> RunSettings {
        RunSettings {
            helper: "/helper\n\u{03bb}".into(),
            identity: current_build_identity(),
            authority: RunAuthority::from_wire_bytes([7; 32]),
            deadline: IdentityDeadline::Finite {
                deadline: RoleDeadline::from_wire_parts(42, 3),
            },
            started: MonotonicInstant::from_wire_parts(30, 4),
            settlement_reserve: Duration::from_nanos(1_500_000_007),
            work_deadline: IdentityDeadline::NeverElapses,
            setup_deadline: RoleDeadline::from_wire_parts(41, 5),
            caller_uid: 12,
            caller_gid: 13,
            memory_bytes: NonZeroU64::new(123).unwrap(),
            caller_run_buffers: 17,
        }
    }

    fn settings_equal(value: Value) {
        let original: RunSettings = serde_json::from_value(value.clone()).unwrap();
        let bytes = serde_json::to_vec(&value).unwrap();
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(&bytes, &mut scratch).unwrap();
        let actual = settings(&mut decoder).unwrap();
        decoder.finish().unwrap();
        assert_eq!(
            actual.helper.chars().collect::<String>(),
            original.helper.to_str().unwrap()
        );
        assert_eq!(actual.identity, original.identity);
        assert_eq!(actual.authority, original.authority);
        assert_eq!(
            serde_json::to_value(actual.deadline).unwrap(),
            serde_json::to_value(original.deadline).unwrap()
        );
        assert_eq!(actual.started, original.started);
        assert_eq!(actual.settlement_reserve, original.settlement_reserve);
        assert_eq!(
            serde_json::to_value(actual.work_deadline).unwrap(),
            serde_json::to_value(original.work_deadline).unwrap()
        );
        assert_eq!(
            serde_json::to_value(actual.setup_deadline).unwrap(),
            serde_json::to_value(original.setup_deadline).unwrap()
        );
        assert_eq!(actual.caller_uid, original.caller_uid);
        assert_eq!(actual.caller_gid, original.caller_gid);
        assert_eq!(actual.memory_bytes, original.memory_bytes);
        assert_eq!(actual.caller_run_buffers, original.caller_run_buffers);
    }

    fn positional(value: &Value) -> Value {
        json!([
            value["helper"],
            value["identity"],
            value["authority"],
            value["deadline"],
            value["started"],
            value["settlement_reserve"],
            value["work_deadline"],
            value["setup_deadline"],
            value["caller_uid"],
            value["caller_gid"],
            value["memory_bytes"],
            value["caller_run_buffers"]
        ])
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn settings_object_and_positional_values_preserve_all_original_fields() {
        let value = serde_json::to_value(original()).unwrap();
        settings_equal(value.clone());
        settings_equal(positional(&value));
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn settings_required_types_and_exact_sequence_length_refuse() {
        let value = serde_json::to_value(original()).unwrap();
        let mut missing = value.clone();
        missing.as_object_mut().unwrap().remove("helper");
        let mut zero = value.clone();
        zero["memory_bytes"] = json!(0);
        let mut native_path = value.clone();
        native_path["helper"] = json!({"Unix": [1]});
        let mut wide_uid = value.clone();
        wide_uid["caller_uid"] = json!(u64::from(u32::MAX) + 1);
        let mut short = positional(&value);
        let _ = short.as_array_mut().unwrap().pop();
        let mut extra_position = positional(&value);
        extra_position.as_array_mut().unwrap().push(Value::Null);
        let mut unknown = value;
        unknown["extra"] = json!(true);
        for value in [
            missing,
            zero,
            native_path,
            wide_uid,
            unknown,
            short,
            extra_position,
        ] {
            assert!(serde_json::from_value::<RunSettings>(value.clone()).is_err());
            let bytes = serde_json::to_vec(&value).unwrap();
            let mut scratch = Scratch::default();
            let mut decoder = Decoder::new(&bytes, &mut scratch).unwrap();
            assert!(settings(&mut decoder).is_err());
        }
    }

    fn parsed_duration(bytes: &[u8]) -> Result<Duration, DecodeError> {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(bytes, &mut scratch)?;
        let value = duration(&mut decoder)?;
        decoder.finish()?;
        Ok(value)
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn duration_carries_nanos_and_refuses_overflow_wrong_types_and_extra_members() {
        for bytes in [
            br#"{"nanos":1500000007,"secs":2}"#.as_slice(),
            br#"[2,1500000007]"#.as_slice(),
        ] {
            let original: Duration = serde_json::from_slice(bytes).unwrap();
            assert_eq!(original, Duration::new(3, 500_000_007));
            assert_eq!(parsed_duration(bytes).unwrap(), original);
        }
        for bytes in [
            br#"[18446744073709551615,1000000000]"#.as_slice(),
            br#"[1]"#.as_slice(),
            br#"[1,2,3]"#.as_slice(),
            br#"{"secs":1,"nanos":2,"nanos":3}"#.as_slice(),
            br#"{"secs":1,"nanos":2,"extra":0}"#.as_slice(),
            br#"{"secs":1,"nanos":4294967296}"#.as_slice(),
            br#"{"secs":1,"nanos":null}"#.as_slice(),
        ] {
            assert!(serde_json::from_slice::<Duration>(bytes).is_err());
            assert!(parsed_duration(bytes).is_err());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn ignored_deadline_content_depth_matches_original_parent_before_and_after_tag() {
        for depth in [125, 126, 127, 128] {
            let body = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
            for bytes in [
                format!(r#"{{"extra":{body},"kind":"NeverElapses"}}"#),
                format!(r#"{{"kind":"NeverElapses","extra":{body}}}"#),
            ] {
                let original = serde_json::from_slice::<IdentityDeadline>(bytes.as_bytes());
                let fixed = parsed_deadline(bytes.as_bytes());
                assert_eq!(fixed.is_ok(), original.is_ok(), "depth {depth}");
                if let Ok(original) = original {
                    assert_eq!(fixed.unwrap(), original);
                } else {
                    assert_eq!(fixed.unwrap_err().cause(), DecodeCause::RecursionLimit);
                }
            }
        }
    }

    fn parsed_deadline(bytes: &[u8]) -> Result<IdentityDeadline, DecodeError> {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(bytes, &mut scratch)?;
        let value = identity_deadline(&mut decoder)?;
        decoder.finish()?;
        Ok(value)
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn deadline_late_tags_unit_map_ignored_content_and_empty_sequence_match_owning_serde() {
        for bytes in [
            br#"{"deadline":[2,1000000000],"kind":"Finite"}"#.as_slice(),
            br#"["Finite",[2,1000000000]]"#.as_slice(),
            br#"{"extra":[{"nested":true}],"deadline":false,"deadline":null,"kind":"NeverElapses"}"#.as_slice(),
            br#"["NeverElapses"]"#.as_slice(),
        ] {
            let original: IdentityDeadline = serde_json::from_slice(bytes).unwrap();
            assert_eq!(serde_json::to_value(parsed_deadline(bytes).unwrap()).unwrap(), serde_json::to_value(original).unwrap());
        }
        // Raw clock nanos survive parsing unchanged; only the owner validates clock semantics.
        let IdentityDeadline::Finite { deadline } =
            parsed_deadline(br#"["Finite",[2,1000000000]]"#).unwrap()
        else {
            panic!("finite variant lost");
        };
        assert_eq!(
            serde_json::to_value(deadline).unwrap(),
            json!({"seconds":2,"nanoseconds":1000000000})
        );
        for bytes in [
            br#"["NeverElapses",1]"#.as_slice(),
            br#"["Finite",[2,3],1]"#.as_slice(),
            br#"{"kind":"Finite"}"#.as_slice(),
            br#"{"deadline":[2,3],"extra":0,"kind":"Finite"}"#.as_slice(),
            br#"{"kind":"NeverElapses","kind":"NeverElapses"}"#.as_slice(),
            br#"{"kind":{"NeverElapses":null}}"#.as_slice(),
            br#"{"kind":"NeverElapses","ignored":[false,]}"#.as_slice(),
        ] {
            assert!(serde_json::from_slice::<IdentityDeadline>(bytes).is_err());
            assert!(parsed_deadline(bytes).is_err());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn pipe_identity_preserves_owning_object_and_sequence_and_refuses_extras() {
        for bytes in [
            br#"{"inode":4,"device":3}"#.as_slice(),
            br#"[3,4]"#.as_slice(),
        ] {
            let original: PipeIdentity = serde_json::from_slice(bytes).unwrap();
            let mut scratch = Scratch::default();
            let mut decoder = Decoder::new(bytes, &mut scratch).unwrap();
            assert_eq!(pipe_identity(&mut decoder).unwrap(), original);
            decoder.finish().unwrap();
            assert_eq!(original, PipeIdentity::from_wire_parts(3, 4));
        }
        for bytes in [
            br#"[3]"#.as_slice(),
            br#"[3,4,5]"#.as_slice(),
            br#"{"device":3,"inode":4,"device":5}"#.as_slice(),
        ] {
            assert!(serde_json::from_slice::<PipeIdentity>(bytes).is_err());
            let mut scratch = Scratch::default();
            let mut decoder = Decoder::new(bytes, &mut scratch).unwrap();
            assert!(pipe_identity(&mut decoder).is_err());
        }
    }
}

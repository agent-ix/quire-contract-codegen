// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Original launcher/arm reply grammar, without process or settlement authority.
//!
//! One original frame and supplied workspace produce existing fixed domain records. Only the
//! internally tagged child-custody scalar stages generic content before its late tag selection.

use std::mem::{size_of, size_of_val};

use super::{
    guardian_decode::{
        ArrayState, DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Scratch, Text,
        ValueKind, ValueSlice,
    },
    inner_reply_decode,
    outer_setup::NamespaceIdentity,
    protocol::{BuildIdentity, GuardianRefusal, RunAuthority},
    role_control_scalar_decode,
    role_protocol::{
        LauncherReply, LauncherReplyKind, OuterArmReply, OuterArmReplyKind, OuterChildSettlement,
        OuterChildSettlementKind, OuterPhaseReply, OuterPhaseReplyKind,
    },
    role_scalar_decode,
};

fn error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}
fn required<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| error(DecodeCause::MissingField))
}
fn unsigned32(decoder: &mut Decoder<'_, '_>) -> Result<u32, DecodeError> {
    u32::try_from(decoder.unsigned()?)
        .map_err(|_| DecodeError::new(DecodeSite::Unsigned, DecodeCause::IntegerOverflow))
}
fn refusal(decoder: &mut Decoder<'_, '_>) -> Result<GuardianRefusal, DecodeError> {
    GuardianRefusal::metadata_text(decoder.unit_variant()?)
        .ok_or_else(|| error(DecodeCause::InvalidValue))
}

// One semantic delegate binding table. Variant membership/order are supplied solely by the
// owning Kind declarations; this union order grants no branch or role permission.
macro_rules! reply_bindings {
    ($decoder:ident; $($field:ident: $value:ty => $parse:expr),+ $(,)?) => {
        #[derive(Default)]
        struct ReplyFields { $($field: Option<$value>),+ }
        impl ReplyFields {
            fn read(&mut self, $decoder: &mut Decoder<'_, '_>, matches: impl Fn(&str) -> bool) -> Result<(), DecodeError> {
                $(if matches(stringify!($field)) {
                    if self.$field.is_some() { return Err(error(DecodeCause::DuplicateField)); }
                    self.$field = Some($parse?);
                    return Ok(());
                })+
                Err(error(DecodeCause::UnknownField))
            }
            fn validate(&self, allowed: &[&str]) -> Result<(), DecodeError> {
                for name in allowed {
                    let present = $(if *name == stringify!($field) { self.$field.is_some() } else)+ { false };
                    if !present { return Err(error(DecodeCause::MissingField)); }
                }
                $(if self.$field.is_some() && !allowed.contains(&stringify!($field)) {
                    return Err(error(DecodeCause::UnknownField));
                })+
                Ok(())
            }
        }
    };
}
reply_bindings! { decoder;
    identity: BuildIdentity => role_scalar_decode::identity(decoder),
    authority: RunAuthority => role_scalar_decode::authority(decoder),
    start: u64 => decoder.unsigned(),
    custody: OuterChildSettlement => custody(decoder),
    reason: GuardianRefusal => refusal(decoder),
    namespace: NamespaceIdentity => role_control_scalar_decode::namespace(decoder),
    network: NamespaceIdentity => role_control_scalar_decode::namespace(decoder),
    mapped_uid: u32 => unsigned32(decoder),
    mapped_gid: u32 => unsigned32(decoder),
}

fn kind<'input, K>(
    decoder: &mut Decoder<'input, '_>,
    lookup: fn(Text<'input>) -> Option<K>,
) -> Result<K, DecodeError> {
    lookup(decoder.string()?).ok_or_else(|| error(DecodeCause::InvalidValue))
}
fn frame<'input, K: Copy>(
    payload: &'input [u8],
    scratch: &mut Scratch,
    lookup: fn(Text<'input>) -> Option<K>,
    declared: fn(K) -> &'static [&'static str],
) -> Result<(K, ReplyFields), DecodeError> {
    let mut decoder = Decoder::new(payload, scratch)?;
    let mut selected = None;
    let mut fields = ReplyFields::default();
    match decoder.peek_kind()? {
        ValueKind::Object => {
            let mut object = decoder.begin_object()?;
            while let Some(name) = decoder.next_field(&mut object)? {
                if name.equals("kind") {
                    if selected.is_some() {
                        return Err(error(DecodeCause::DuplicateField));
                    }
                    selected = Some(kind(&mut decoder, lookup)?);
                } else {
                    fields.read(&mut decoder, |field| name.equals(field))?;
                }
            }
        }
        ValueKind::Array => {
            let mut array = decoder.begin_array()?;
            if !decoder.next_element(&mut array)? {
                return Err(error(DecodeCause::MissingField));
            }
            let tag = kind(&mut decoder, lookup)?;
            selected = Some(tag);
            for name in declared(tag) {
                if !decoder.next_element(&mut array)? {
                    return Err(error(DecodeCause::MissingField));
                }
                fields.read(&mut decoder, |field| *name == field)?;
            }
            if decoder.next_element(&mut array)? {
                return Err(error(DecodeCause::InvalidValue));
            }
        }
        ValueKind::String | ValueKind::Number | ValueKind::Boolean | ValueKind::Null => {
            return Err(error(DecodeCause::UnexpectedToken))
        }
    }
    let selected = required(selected)?;
    fields.validate(declared(selected))?;
    decoder.finish()?;
    Ok((selected, fields))
}

/// Decode the entire original L reply into its existing fixed record, without custody proof.
pub(super) fn launcher(
    payload: &[u8],
    scratch: &mut Scratch,
) -> Result<LauncherReply, DecodeError> {
    let (kind, fields) = frame(
        payload,
        scratch,
        LauncherReplyKind::metadata_text,
        LauncherReplyKind::declared_fields,
    )?;
    match kind {
        LauncherReplyKind::Ready => Ok(LauncherReply::Ready {
            identity: required(fields.identity)?,
            authority: required(fields.authority)?,
        }),
        LauncherReplyKind::OuterSettled => Ok(LauncherReply::OuterSettled {
            identity: required(fields.identity)?,
            authority: required(fields.authority)?,
            custody: required(fields.custody)?,
        }),
        LauncherReplyKind::Refused => Ok(LauncherReply::Refused {
            identity: required(fields.identity)?,
            authority: required(fields.authority)?,
            reason: required(fields.reason)?,
        }),
    }
}

/// Decode original O Armed facts; live pin/mapping/namespace validation remains actor-owned.
pub(super) fn arm(payload: &[u8], scratch: &mut Scratch) -> Result<OuterArmReply, DecodeError> {
    let (kind, fields) = frame(
        payload,
        scratch,
        OuterArmReplyKind::metadata_text,
        OuterArmReplyKind::declared_fields,
    )?;
    match kind {
        OuterArmReplyKind::Armed => Ok(OuterArmReply::Armed {
            identity: required(fields.identity)?,
            authority: required(fields.authority)?,
            namespace: required(fields.namespace)?,
            network: required(fields.network)?,
            mapped_uid: required(fields.mapped_uid)?,
            mapped_gid: required(fields.mapped_gid)?,
        }),
    }
}

/// Existing phase-only receive grammar. The current public actor uses the complete startup
/// union; these retained low-level interfaces use the SAME fixed record visitor and owning
/// declarations, with no fallback, phase authorization or pin manufactured by this parser.
pub(super) fn phase(payload: &[u8], scratch: &mut Scratch) -> Result<OuterPhaseReply, DecodeError> {
    let (kind, fields) = frame(
        payload,
        scratch,
        OuterPhaseReplyKind::metadata_text,
        OuterPhaseReplyKind::declared_fields,
    )?;
    match kind {
        OuterPhaseReplyKind::MonitorSpawned => Ok(OuterPhaseReply::MonitorSpawned {
            authority: required(fields.authority)?,
        }),
        OuterPhaseReplyKind::InnerClaimed => Ok(OuterPhaseReply::InnerClaimed {
            authority: required(fields.authority)?,
            start: required(fields.start)?,
            namespace: required(fields.namespace)?,
        }),
        OuterPhaseReplyKind::GateReleased => Ok(OuterPhaseReply::GateReleased {
            authority: required(fields.authority)?,
        }),
    }
}

#[derive(Default)]
struct CustodyFields<'input> {
    kind: Option<OuterChildSettlementKind>,
    outcome: Option<ValueSlice<'input>>,
    body_fault: Option<DecodeCause>,
    extra_sequence: bool,
}
fn custody_map<'input>(
    decoder: &mut Decoder<'input, '_>,
    fields: &mut CustodyFields<'input>,
) -> Result<(), DecodeError> {
    let mut object = decoder.begin_object()?;
    while let Some(name) = decoder.next_field(&mut object)? {
        if name.equals("kind") {
            if fields.kind.is_some() {
                return Err(error(DecodeCause::DuplicateField));
            }
            fields.kind = Some(kind(decoder, OuterChildSettlementKind::metadata_text)?);
        } else {
            // Owning TaggedContentVisitor first validates generic content. NotCreated may
            // ignore any valid map member; Reaped subsequently checks its one exact schema.
            let value = decoder.value()?;
            let known = OuterChildSettlementKind::Reaped
                .declared_fields()
                .iter()
                .any(|field| name.equals(field));
            let fault = if known {
                if fields.outcome.is_some() {
                    Some(DecodeCause::DuplicateField)
                } else {
                    fields.outcome = Some(value);
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
fn custody_sequence<'input>(
    decoder: &mut Decoder<'input, '_>,
    fields: &mut CustodyFields<'input>,
) -> Result<(), DecodeError> {
    let mut array = decoder.begin_array()?;
    if !decoder.next_element(&mut array)? {
        return Err(error(DecodeCause::MissingField));
    }
    fields.kind = Some(kind(decoder, OuterChildSettlementKind::metadata_text)?);
    while decoder.next_element(&mut array)? {
        let value = decoder.value()?;
        if fields.outcome.is_none() {
            fields.outcome = Some(value);
        } else {
            fields.extra_sequence = true;
        }
    }
    Ok(())
}

/// Consume parsed child-custody metadata only; no absence, creation, reaping or exit is inferred.
pub(super) fn custody(decoder: &mut Decoder<'_, '_>) -> Result<OuterChildSettlement, DecodeError> {
    let mut fields = CustodyFields::default();
    let sequence = match decoder.peek_kind()? {
        ValueKind::Object => {
            custody_map(decoder, &mut fields)?;
            false
        }
        ValueKind::Array => {
            custody_sequence(decoder, &mut fields)?;
            true
        }
        ValueKind::String | ValueKind::Number | ValueKind::Boolean | ValueKind::Null => {
            return Err(error(DecodeCause::UnexpectedToken))
        }
    };
    match required(fields.kind)? {
        OuterChildSettlementKind::NotCreated => {
            if sequence && fields.outcome.is_some() {
                return Err(error(DecodeCause::InvalidValue));
            }
            Ok(OuterChildSettlement::NotCreated)
        }
        OuterChildSettlementKind::Reaped => {
            if let Some(fault) = fields.body_fault {
                return Err(error(fault));
            }
            if fields.extra_sequence {
                return Err(error(DecodeCause::InvalidValue));
            }
            let mut child = decoder.nested(required(fields.outcome)?)?;
            let outcome = inner_reply_decode::outcome(&mut child)?;
            child.finish()?;
            Ok(OuterChildSettlement::Reaped { outcome })
        }
    }
}

/// Checked additional schema layouts, excluding shared Scratch/frame and nested helper charges.
/// Fixed deepest path is frame -> custody -> one sequential outcome child on SAME workspace.
/// Generic content scanning is iterative. Initializer/copy/return-place and native highwater
/// remain unmeasured; sizeof terms establish neither actual stack nor complete accounting.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<ReplyFields>(),
        size_of::<CustodyFields<'static>>(),
        size_of::<LauncherReply>(),
        size_of::<OuterArmReply>(),
        size_of::<OuterPhaseReply>(),
        size_of::<OuterPhaseReplyKind>(),
        size_of::<Option<OuterPhaseReplyKind>>(),
        size_of::<Result<OuterPhaseReply, DecodeError>>(),
        size_of::<(OuterPhaseReplyKind, ReplyFields)>(),
        size_of::<Result<(OuterPhaseReplyKind, ReplyFields), DecodeError>>(),
        size_of::<OuterChildSettlement>(),
        size_of::<Result<LauncherReply, DecodeError>>(),
        size_of::<Result<OuterArmReply, DecodeError>>(),
        size_of::<Result<OuterChildSettlement, DecodeError>>(),
        size_of::<(LauncherReplyKind, ReplyFields)>(),
        size_of::<(OuterArmReplyKind, ReplyFields)>(),
        size_of::<Result<(LauncherReplyKind, ReplyFields), DecodeError>>(),
        size_of::<Result<(OuterArmReplyKind, ReplyFields), DecodeError>>(),
        size_of::<Option<LauncherReplyKind>>(),
        size_of::<Option<OuterArmReplyKind>>(),
        size_of::<OuterChildSettlementKind>(),
        size_of::<GuardianRefusal>(),
        size_of::<Option<DecodeCause>>(),
        size_of::<Option<ValueSlice<'static>>>(),
        size_of::<ObjectState>(),
        size_of::<ArrayState>(),
        size_of::<Text<'static>>(),
        size_of::<Decoder<'static, 'static>>(),
        size_of::<&mut Decoder<'static, 'static>>(),
        size_of::<&'static [&'static str]>(),
        size_of::<&'static str>(),
        size_of::<fn(Text<'static>) -> Option<LauncherReplyKind>>(),
        size_of::<fn(LauncherReplyKind) -> &'static [&'static str]>(),
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
    // Representation oracles only; these values never stand for actual pins or Child waits.
    use super::*;
    use crate::kani::run::protocol::{current_build_identity, BackendExit};
    use serde_json::{json, Value};

    #[derive(Clone, Copy)]
    enum Scope {
        Launcher,
        Arm,
        Phase,
    }
    fn fixtures() -> Vec<(Scope, Value, &'static [&'static str])> {
        let identity = current_build_identity();
        let authority = RunAuthority::from_wire_bytes([8; 32]);
        vec![
            (
                Scope::Phase,
                serde_json::to_value(OuterPhaseReply::MonitorSpawned { authority }).unwrap(),
                OuterPhaseReplyKind::MonitorSpawned.declared_fields(),
            ),
            (
                Scope::Phase,
                serde_json::to_value(OuterPhaseReply::InnerClaimed {
                    authority,
                    start: u64::MAX,
                    namespace: NamespaceIdentity::from_wire_parts(3, 4),
                })
                .unwrap(),
                OuterPhaseReplyKind::InnerClaimed.declared_fields(),
            ),
            (
                Scope::Phase,
                serde_json::to_value(OuterPhaseReply::GateReleased { authority }).unwrap(),
                OuterPhaseReplyKind::GateReleased.declared_fields(),
            ),
            (
                Scope::Launcher,
                serde_json::to_value(LauncherReply::Ready {
                    identity,
                    authority,
                })
                .unwrap(),
                LauncherReplyKind::Ready.declared_fields(),
            ),
            (
                Scope::Launcher,
                serde_json::to_value(LauncherReply::OuterSettled {
                    identity,
                    authority,
                    custody: OuterChildSettlement::Reaped {
                        outcome: BackendExit::Signal(9),
                    },
                })
                .unwrap(),
                LauncherReplyKind::OuterSettled.declared_fields(),
            ),
            (
                Scope::Launcher,
                serde_json::to_value(LauncherReply::Refused {
                    identity,
                    authority,
                    reason: GuardianRefusal::InvalidControl,
                })
                .unwrap(),
                LauncherReplyKind::Refused.declared_fields(),
            ),
            (
                Scope::Arm,
                serde_json::to_value(OuterArmReply::Armed {
                    identity,
                    authority,
                    namespace: NamespaceIdentity::from_wire_parts(3, 4),
                    network: NamespaceIdentity::from_wire_parts(5, 6),
                    mapped_uid: 7,
                    mapped_gid: u32::MAX,
                })
                .unwrap(),
                OuterArmReplyKind::Armed.declared_fields(),
            ),
        ]
    }
    fn expected(scope: Scope, bytes: &[u8]) -> Option<(Value, usize)> {
        match scope {
            Scope::Launcher => serde_json::from_slice::<LauncherReply>(bytes)
                .ok()
                .map(|value| (serde_json::to_value(&value).unwrap(), value.rights_count())),
            Scope::Phase => serde_json::from_slice::<OuterPhaseReply>(bytes)
                .ok()
                .map(|value| (serde_json::to_value(&value).unwrap(), value.rights_count())),
            Scope::Arm => serde_json::from_slice::<OuterArmReply>(bytes)
                .ok()
                .map(|value| (serde_json::to_value(&value).unwrap(), value.rights_count())),
        }
    }
    fn actual(scope: Scope, bytes: &[u8]) -> Result<(Value, usize), DecodeError> {
        let mut scratch = Scratch::default();
        match scope {
            Scope::Launcher => {
                let value = launcher(bytes, &mut scratch)?;
                Ok((serde_json::to_value(&value).unwrap(), value.rights_count()))
            }
            Scope::Phase => {
                let value = phase(bytes, &mut scratch)?;
                Ok((serde_json::to_value(&value).unwrap(), value.rights_count()))
            }
            Scope::Arm => {
                let value = arm(bytes, &mut scratch)?;
                Ok((serde_json::to_value(&value).unwrap(), value.rights_count()))
            }
        }
    }
    fn positive(scope: Scope, value: &Value) {
        let bytes = serde_json::to_vec(value).unwrap();
        assert_eq!(
            actual(scope, &bytes).unwrap(),
            expected(scope, &bytes).expect("owning positive input")
        );
    }
    fn negative(scope: Scope, bytes: &[u8]) {
        assert!(
            expected(scope, bytes).is_none(),
            "owning decoder must refuse"
        );
        assert!(actual(scope, bytes).is_err(), "fixed decoder must refuse");
    }
    fn sequence(value: &Value, fields: &[&str]) -> Value {
        Value::Array(
            std::iter::once(value["kind"].clone())
                .chain(fields.iter().map(|field| value[*field].clone()))
                .collect(),
        )
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn original_launcher_variants_and_arm_match_values_rights_and_owning_sequence_order() {
        for (scope, value, fields) in fixtures() {
            positive(scope, &value);
            positive(scope, &sequence(&value, fields));
            let mut late = value.clone();
            let tag = late.as_object_mut().unwrap().remove("kind").unwrap();
            let body = serde_json::to_string(&late).unwrap();
            let bytes = format!(
                "{},\"kind\":{}}}",
                body.strip_suffix('}').unwrap(),
                serde_json::to_string(&tag).unwrap()
            );
            assert_eq!(
                actual(scope, bytes.as_bytes()).unwrap(),
                expected(scope, bytes.as_bytes()).unwrap()
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn every_required_member_duplicate_opposite_null_arity_and_enclosing_syntax_refuse() {
        for (scope, value, fields) in fixtures() {
            for name in std::iter::once("kind").chain(fields.iter().copied()) {
                let mut missing = value.clone();
                missing.as_object_mut().unwrap().remove(name);
                negative(scope, &serde_json::to_vec(&missing).unwrap());
                let mut null = value.clone();
                null[name] = Value::Null;
                negative(scope, &serde_json::to_vec(&null).unwrap());
                let original = serde_json::to_string(&value).unwrap();
                let duplicate = format!(
                    "{{{}:{},{}",
                    serde_json::to_string(name).unwrap(),
                    serde_json::to_string(&value[name]).unwrap(),
                    original.strip_prefix('{').unwrap()
                );
                negative(scope, duplicate.as_bytes());
            }
            let mut unknown = value.clone();
            unknown["extra"] = Value::Null;
            negative(scope, &serde_json::to_vec(&unknown).unwrap());
            let forbidden = if fields.contains(&"custody") {
                "reason"
            } else {
                "custody"
            };
            let mut opposite = value.clone();
            opposite[forbidden] = Value::Null;
            negative(scope, &serde_json::to_vec(&opposite).unwrap());
            let mut map_tag = value.clone();
            map_tag["kind"] = json!({(value["kind"].as_str().unwrap()):null});
            negative(scope, &serde_json::to_vec(&map_tag).unwrap());
            let mut short = sequence(&value, fields);
            let _ = short.as_array_mut().unwrap().pop();
            negative(scope, &serde_json::to_vec(&short).unwrap());
            let mut extra = sequence(&value, fields);
            extra.as_array_mut().unwrap().push(Value::Null);
            negative(scope, &serde_json::to_vec(&extra).unwrap());
            let complete = serde_json::to_vec(&value).unwrap();
            negative(scope, &complete[..complete.len() - 1]);
            let mut trailing = complete;
            trailing.extend_from_slice(b"{}");
            negative(scope, &trailing);
        }
    }

    fn parsed_custody(bytes: &[u8]) -> Result<OuterChildSettlement, DecodeError> {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(bytes, &mut scratch)?;
        let value = custody(&mut decoder)?;
        decoder.finish()?;
        Ok(value)
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn custody_unit_ignored_map_and_reaped_late_kind_match_actual_owning_serde() {
        for bytes in [
            br#"{"outcome":false,"outcome":[{"arbitrary":null}],"other":0,"kind":"NotCreated"}"#
                .as_slice(),
            br#"["NotCreated"]"#.as_slice(),
            br#"{"outcome":{"Code":0},"kind":"Reaped"}"#.as_slice(),
            br#"["Reaped",{"Signal":9}]"#.as_slice(),
        ] {
            let expected: OuterChildSettlement = serde_json::from_slice(bytes).unwrap();
            let actual = parsed_custody(bytes).unwrap();
            assert_eq!(
                serde_json::to_value(actual).unwrap(),
                serde_json::to_value(expected).unwrap()
            );
        }
        assert!(matches!(
            parsed_custody(br#"{"kind":"NotCreated","outcome":false}"#).unwrap(),
            OuterChildSettlement::NotCreated
        ));
        assert!(matches!(
            parsed_custody(br#"{"kind":"Reaped","outcome":{"Code":0}}"#).unwrap(),
            OuterChildSettlement::Reaped {
                outcome: BackendExit::Code(0)
            }
        ));
        for bytes in [
            br#"["NotCreated",null]"#.as_slice(),
            br#"["Reaped"]"#.as_slice(),
            br#"["Reaped",{"Code":1},0]"#.as_slice(),
            br#"{"kind":"Reaped","outcome":false}"#.as_slice(),
            br#"{"kind":"Reaped","outcome":{"Code":2147483648}}"#.as_slice(),
            br#"{"kind":"Reaped","outcome":{"Signal":-0}}"#.as_slice(),
            br#"{"kind":"Reaped","outcome":{"Code":1,"Signal":9}}"#.as_slice(),
            br#"{"kind":"Reaped","outcome":{"Code":1},"extra":0}"#.as_slice(),
            br#"{"kind":"Reaped","outcome":{"Code":1},"outcome":{"Code":2}}"#.as_slice(),
            br#"{"kind":"NotCreated","kind":"NotCreated"}"#.as_slice(),
            br#"{"kind":{"NotCreated":null}}"#.as_slice(),
            br#"{"kind":"NotCreated","ignored":[false,]}"#.as_slice(),
        ] {
            assert!(serde_json::from_slice::<OuterChildSettlement>(bytes).is_err());
            assert!(parsed_custody(bytes).is_err());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn ordinary_refusal_unit_map_and_nested_namespace_sequences_preserve_raw_values() {
        for (scope, mut value, _) in fixtures() {
            if let Some(reason) = value.get_mut("reason") {
                *reason = json!({"InvalidControl":null});
            }
            for name in ["namespace", "network"] {
                if let Some(namespace) = value.get_mut(name) {
                    *namespace = json!([namespace["device"], namespace["inode"]]);
                }
            }
            positive(scope, &value);
        }
        let (scope, value, _) = fixtures()
            .into_iter()
            .find(|(_, value, _)| value.get("reason").is_some())
            .unwrap();
        for reason in [
            json!({"InvalidControl":true}),
            json!({"InvalidControl":null,"UnexpectedControl":null}),
            json!(1),
        ] {
            let mut wrong = value.clone();
            wrong["reason"] = reason;
            negative(scope, &serde_json::to_vec(&wrong).unwrap());
        }
        let (scope, value, _) = fixtures()
            .into_iter()
            .find(|(scope, _, _)| matches!(scope, Scope::Arm))
            .unwrap();
        for field in ["mapped_uid", "mapped_gid"] {
            let mut wide = value.clone();
            wide[field] = json!(u64::from(u32::MAX) + 1);
            negative(scope, &serde_json::to_vec(&wide).unwrap());
        }
    }
}

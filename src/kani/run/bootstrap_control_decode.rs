// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Borrowed original L/O/I bootstrap controls on a single supplied workspace.
//!
//! Returned fields and descriptor counts are parsed facts only. Actual sender/run/rights,
//! clocks, retained process custody and settings materialization belong to the role owner.

use std::mem::{size_of, size_of_val};

use super::{
    guardian_decode::{
        ArrayState, DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Scratch, Text,
        ValueKind,
    },
    outer_setup::NamespaceIdentity,
    protocol::RunAuthority,
    report_storage::PipeIdentity,
    role_control_scalar_decode,
    role_deadline::RoleDeadline,
    role_protocol::{
        InnerBootstrapKind, LauncherControlKind, LauncherSettlementMode, OuterBootstrapKind,
    },
    role_scalar_decode,
    settings_decode::{self, SettingsBytes},
};

/// Parsed original launcher control, without authority to create or settle a role.
#[derive(Clone, Copy)]
pub(super) enum LauncherInput<'input> {
    Start {
        settings: SettingsBytes<'input>,
    },
    Settle {
        authority: RunAuthority,
        deadline: RoleDeadline,
        mode: LauncherSettlementMode,
    },
    Retire {
        authority: RunAuthority,
    },
}
impl LauncherInput<'_> {
    /// Existing variant descriptor policy; no received descriptor is authenticated here.
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 4,
            Self::Settle { .. } | Self::Retire { .. } => 0,
        }
    }
}

/// Parsed original outer bootstrap, without live namespace or launcher observation authority.
#[derive(Clone, Copy)]
pub(super) enum OuterInput<'input> {
    Start {
        settings: SettingsBytes<'input>,
        original_mount: NamespaceIdentity,
        original_pid: NamespaceIdentity,
        original_network: NamespaceIdentity,
    },
    LauncherObservation {
        authority: RunAuthority,
    },
}
impl OuterInput<'_> {
    /// Existing variant descriptor policy; numbers do not substitute for actual retained rights.
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 4,
            Self::LauncherObservation { .. } => 2,
        }
    }
}

/// Parsed original inner bootstrap; report slot and pipe numbers remain unverified.
#[derive(Clone, Copy)]
pub(super) enum InnerInput<'input> {
    Start {
        settings: SettingsBytes<'input>,
        outer_namespace: NamespaceIdentity,
        report: PipeIdentity,
        report_slot: i32,
    },
}
impl InnerInput<'_> {
    /// Existing three-right policy, not a report writer or process ownership claim.
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 3,
        }
    }
}

fn error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}
fn required<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| error(DecodeCause::MissingField))
}

fn mode(decoder: &mut Decoder<'_, '_>) -> Result<LauncherSettlementMode, DecodeError> {
    LauncherSettlementMode::metadata_text(decoder.unit_variant()?)
        .ok_or_else(|| error(DecodeCause::InvalidValue))
}
fn report_slot(decoder: &mut Decoder<'_, '_>) -> Result<i32, DecodeError> {
    i32::try_from(decoder.signed()?)
        .map_err(|_| DecodeError::new(DecodeSite::Signed, DecodeCause::IntegerOverflow))
}

// Semantic delegate bindings are defined once. Variant membership and positional order come
// exclusively from the owning Kind declarations, not this union's declaration order.
macro_rules! body_bindings {
    ($input:lifetime; $decoder:ident; $($field:ident: $value:ty => $parse:expr),+ $(,)?) => {
        #[derive(Default)]
        struct BodyFields<$input> { $($field: Option<$value>),+ }
        impl<$input> BodyFields<$input> {
            fn read(
                &mut self,
                $decoder: &mut Decoder<$input, '_>,
                matches: impl Fn(&str) -> bool,
            ) -> Result<(), DecodeError> {
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
body_bindings! { 'input; decoder;
    settings: SettingsBytes<'input> => settings_decode::settings(decoder),
    authority: RunAuthority => role_scalar_decode::authority(decoder),
    deadline: RoleDeadline => role_scalar_decode::deadline(decoder),
    mode: LauncherSettlementMode => mode(decoder),
    original_mount: NamespaceIdentity => role_control_scalar_decode::namespace(decoder),
    original_pid: NamespaceIdentity => role_control_scalar_decode::namespace(decoder),
    original_network: NamespaceIdentity => role_control_scalar_decode::namespace(decoder),
    outer_namespace: NamespaceIdentity => role_control_scalar_decode::namespace(decoder),
    report: PipeIdentity => settings_decode::pipe_identity(decoder),
    report_slot: i32 => report_slot(decoder),
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
) -> Result<(K, BodyFields<'input>), DecodeError> {
    let mut decoder = Decoder::new(payload, scratch)?;
    let mut selected = None;
    let mut body = BodyFields::default();
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
                    body.read(&mut decoder, |field| name.equals(field))?;
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
                body.read(&mut decoder, |field| *name == field)?;
            }
            if decoder.next_element(&mut array)? {
                return Err(error(DecodeCause::InvalidValue));
            }
        }
        ValueKind::String | ValueKind::Number | ValueKind::Boolean | ValueKind::Null => {
            return Err(error(DecodeCause::UnexpectedToken));
        }
    }
    let selected = required(selected)?;
    body.validate(declared(selected))?;
    decoder.finish()?;
    Ok((selected, body))
}

/// Decode one complete original C-to-L control; no frame becomes owned or authenticated.
pub(super) fn launcher<'input>(
    payload: &'input [u8],
    scratch: &mut Scratch,
) -> Result<LauncherInput<'input>, DecodeError> {
    let (kind, fields) = frame(
        payload,
        scratch,
        LauncherControlKind::metadata_text,
        LauncherControlKind::declared_fields,
    )?;
    match kind {
        LauncherControlKind::Start => Ok(LauncherInput::Start {
            settings: required(fields.settings)?,
        }),
        LauncherControlKind::Settle => Ok(LauncherInput::Settle {
            authority: required(fields.authority)?,
            deadline: required(fields.deadline)?,
            mode: required(fields.mode)?,
        }),
        LauncherControlKind::Retire => Ok(LauncherInput::Retire {
            authority: required(fields.authority)?,
        }),
    }
}

/// Decode one complete original L-to-O bootstrap; descriptor custody remains with its owner.
pub(super) fn outer<'input>(
    payload: &'input [u8],
    scratch: &mut Scratch,
) -> Result<OuterInput<'input>, DecodeError> {
    let (kind, fields) = frame(
        payload,
        scratch,
        OuterBootstrapKind::metadata_text,
        OuterBootstrapKind::declared_fields,
    )?;
    match kind {
        OuterBootstrapKind::Start => Ok(OuterInput::Start {
            settings: required(fields.settings)?,
            original_mount: required(fields.original_mount)?,
            original_pid: required(fields.original_pid)?,
            original_network: required(fields.original_network)?,
        }),
        OuterBootstrapKind::LauncherObservation => Ok(OuterInput::LauncherObservation {
            authority: required(fields.authority)?,
        }),
    }
}

/// Decode one complete original O-to-I bootstrap; no pipe/slot check or acquisition occurs.
pub(super) fn inner<'input>(
    payload: &'input [u8],
    scratch: &mut Scratch,
) -> Result<InnerInput<'input>, DecodeError> {
    let (kind, fields) = frame(
        payload,
        scratch,
        InnerBootstrapKind::metadata_text,
        InnerBootstrapKind::declared_fields,
    )?;
    match kind {
        InnerBootstrapKind::Start => Ok(InnerInput::Start {
            settings: required(fields.settings)?,
            outer_namespace: required(fields.outer_namespace)?,
            report: required(fields.report)?,
            report_slot: required(fields.report_slot)?,
        }),
    }
}

/// Checked new fixed layouts only; shared Scratch/frame/nested schema terms are excluded.
/// The fixed path frame -> settings -> identity deadline -> sequential raw-deadline child has
/// no payload recursion. Initializer, copy/return-place transients and native stack highwater
/// remain unmeasured; this sum supplies no whole-accounting or owned-capacity proof.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<BodyFields<'static>>(),
        size_of::<LauncherInput<'static>>(),
        size_of::<OuterInput<'static>>(),
        size_of::<InnerInput<'static>>(),
        size_of::<Result<LauncherInput<'static>, DecodeError>>(),
        size_of::<Result<OuterInput<'static>, DecodeError>>(),
        size_of::<Result<InnerInput<'static>, DecodeError>>(),
        size_of::<(LauncherControlKind, BodyFields<'static>)>(),
        size_of::<(OuterBootstrapKind, BodyFields<'static>)>(),
        size_of::<(InnerBootstrapKind, BodyFields<'static>)>(),
        size_of::<Result<(LauncherControlKind, BodyFields<'static>), DecodeError>>(),
        size_of::<Result<(OuterBootstrapKind, BodyFields<'static>), DecodeError>>(),
        size_of::<Result<(InnerBootstrapKind, BodyFields<'static>), DecodeError>>(),
        size_of::<Option<LauncherControlKind>>(),
        size_of::<Option<OuterBootstrapKind>>(),
        size_of::<Option<InnerBootstrapKind>>(),
        size_of::<LauncherSettlementMode>(),
        size_of::<ObjectState>(),
        size_of::<ArrayState>(),
        size_of::<Text<'static>>(),
        size_of::<&mut Decoder<'static, 'static>>(),
        size_of::<&'static [&'static str]>(),
        size_of::<&'static str>(),
        size_of::<fn(Text<'static>) -> Option<LauncherControlKind>>(),
        size_of::<fn(LauncherControlKind) -> &'static [&'static str]>(),
        size_of::<i64>(),
        size_of::<i32>(),
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
    // Parsed representation only: no actual sender, live pin, pipe or settlement is involved.
    use super::*;
    use crate::kani::run::{
        protocol::current_build_identity,
        role_deadline::{IdentityDeadline, MonotonicInstant},
        role_protocol::{InnerBootstrap, LauncherControl, OuterBootstrap, RunSettings},
    };
    use serde_json::{json, Value};
    use std::{num::NonZeroU64, time::Duration};

    #[derive(Clone, Copy)]
    enum Scope {
        Launcher,
        Outer,
        Inner,
    }

    fn original_settings() -> RunSettings {
        RunSettings {
            helper: "/helper\n\u{03bb}".into(),
            identity: current_build_identity(),
            authority: RunAuthority::from_wire_bytes([9; 32]),
            deadline: IdentityDeadline::NeverElapses,
            started: MonotonicInstant::from_wire_parts(4, 5),
            settlement_reserve: Duration::from_nanos(500_000_003),
            work_deadline: IdentityDeadline::Finite {
                deadline: RoleDeadline::from_wire_parts(8, 9),
            },
            setup_deadline: RoleDeadline::from_wire_parts(7, 6),
            caller_uid: 12,
            caller_gid: 13,
            memory_bytes: NonZeroU64::new(100).unwrap(),
            caller_run_buffers: 14,
        }
    }

    fn owned_settings(value: SettingsBytes<'_>) -> RunSettings {
        RunSettings {
            helper: value.helper.chars().collect::<String>().into(),
            identity: value.identity,
            authority: value.authority,
            deadline: value.deadline,
            started: value.started,
            settlement_reserve: value.settlement_reserve,
            work_deadline: value.work_deadline,
            setup_deadline: value.setup_deadline,
            caller_uid: value.caller_uid,
            caller_gid: value.caller_gid,
            memory_bytes: value.memory_bytes,
            caller_run_buffers: value.caller_run_buffers,
        }
    }

    fn fixtures() -> Vec<(Scope, Value, &'static [&'static str])> {
        let authority = RunAuthority::from_wire_bytes([9; 32]);
        vec![
            (
                Scope::Launcher,
                serde_json::to_value(LauncherControl::Start {
                    settings: original_settings(),
                })
                .unwrap(),
                LauncherControlKind::Start.declared_fields(),
            ),
            (
                Scope::Launcher,
                serde_json::to_value(LauncherControl::Settle {
                    authority,
                    deadline: RoleDeadline::from_wire_parts(9, 10),
                    mode: LauncherSettlementMode::CancelOuter,
                })
                .unwrap(),
                LauncherControlKind::Settle.declared_fields(),
            ),
            (
                Scope::Launcher,
                serde_json::to_value(LauncherControl::Retire { authority }).unwrap(),
                LauncherControlKind::Retire.declared_fields(),
            ),
            (
                Scope::Outer,
                serde_json::to_value(OuterBootstrap::Start {
                    settings: original_settings(),
                    original_mount: NamespaceIdentity::from_wire_parts(1, 2),
                    original_pid: NamespaceIdentity::from_wire_parts(3, 4),
                    original_network: NamespaceIdentity::from_wire_parts(5, 6),
                })
                .unwrap(),
                OuterBootstrapKind::Start.declared_fields(),
            ),
            (
                Scope::Outer,
                serde_json::to_value(OuterBootstrap::LauncherObservation { authority }).unwrap(),
                OuterBootstrapKind::LauncherObservation.declared_fields(),
            ),
            (
                Scope::Inner,
                serde_json::to_value(InnerBootstrap::Start {
                    settings: original_settings(),
                    outer_namespace: NamespaceIdentity::from_wire_parts(7, 8),
                    report: PipeIdentity::from_wire_parts(9, 10),
                    report_slot: -123,
                })
                .unwrap(),
                InnerBootstrapKind::Start.declared_fields(),
            ),
        ]
    }

    fn expected(scope: Scope, bytes: &[u8]) -> Option<(Value, usize)> {
        match scope {
            Scope::Launcher => serde_json::from_slice::<LauncherControl>(bytes)
                .ok()
                .map(|value| (serde_json::to_value(&value).unwrap(), value.rights_count())),
            Scope::Outer => serde_json::from_slice::<OuterBootstrap>(bytes)
                .ok()
                .map(|value| (serde_json::to_value(&value).unwrap(), value.rights_count())),
            Scope::Inner => serde_json::from_slice::<InnerBootstrap>(bytes)
                .ok()
                .map(|value| (serde_json::to_value(&value).unwrap(), value.rights_count())),
        }
    }

    fn actual(scope: Scope, bytes: &[u8]) -> Result<(Value, usize), DecodeError> {
        let mut scratch = Scratch::default();
        match scope {
            Scope::Launcher => {
                let value = launcher(bytes, &mut scratch)?;
                let rights = value.rights_count();
                let owned = match value {
                    LauncherInput::Start { settings } => LauncherControl::Start {
                        settings: owned_settings(settings),
                    },
                    LauncherInput::Settle {
                        authority,
                        deadline,
                        mode,
                    } => LauncherControl::Settle {
                        authority,
                        deadline,
                        mode,
                    },
                    LauncherInput::Retire { authority } => LauncherControl::Retire { authority },
                };
                Ok((serde_json::to_value(owned).unwrap(), rights))
            }
            Scope::Outer => {
                let value = outer(bytes, &mut scratch)?;
                let rights = value.rights_count();
                let owned = match value {
                    OuterInput::Start {
                        settings,
                        original_mount,
                        original_pid,
                        original_network,
                    } => OuterBootstrap::Start {
                        settings: owned_settings(settings),
                        original_mount,
                        original_pid,
                        original_network,
                    },
                    OuterInput::LauncherObservation { authority } => {
                        OuterBootstrap::LauncherObservation { authority }
                    }
                };
                Ok((serde_json::to_value(owned).unwrap(), rights))
            }
            Scope::Inner => {
                let value = inner(bytes, &mut scratch)?;
                let rights = value.rights_count();
                let InnerInput::Start {
                    settings,
                    outer_namespace,
                    report,
                    report_slot,
                } = value;
                Ok((
                    serde_json::to_value(InnerBootstrap::Start {
                        settings: owned_settings(settings),
                        outer_namespace,
                        report,
                        report_slot,
                    })
                    .unwrap(),
                    rights,
                ))
            }
        }
    }

    fn assert_positive(scope: Scope, value: Value) {
        let bytes = serde_json::to_vec(&value).unwrap();
        let expected = expected(scope, &bytes).expect("owning positive input must decode");
        assert_eq!(actual(scope, &bytes).unwrap(), expected);
    }
    fn assert_negative(scope: Scope, bytes: &[u8]) {
        assert!(
            expected(scope, bytes).is_none(),
            "owning grammar must refuse control"
        );
        assert!(
            actual(scope, bytes).is_err(),
            "fixed grammar must refuse control"
        );
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
    fn every_bootstrap_variant_matches_owning_values_rights_and_positional_order() {
        for (scope, value, fields) in fixtures() {
            assert_positive(scope, value.clone());
            assert_positive(scope, sequence(&value, fields));
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
    fn nested_positional_records_and_mode_unit_null_map_keep_original_facts() {
        for (scope, mut value, _) in fixtures() {
            if let Some(settings) = value.get_mut("settings") {
                let original = settings.clone();
                *settings = json!([
                    original["helper"],
                    original["identity"],
                    original["authority"],
                    original["deadline"],
                    original["started"],
                    original["settlement_reserve"],
                    original["work_deadline"],
                    original["setup_deadline"],
                    original["caller_uid"],
                    original["caller_gid"],
                    original["memory_bytes"],
                    original["caller_run_buffers"]
                ]);
            }
            for name in [
                "original_mount",
                "original_pid",
                "original_network",
                "outer_namespace",
            ] {
                if let Some(namespace) = value.get_mut(name) {
                    *namespace = json!([namespace["device"], namespace["inode"]]);
                }
            }
            if let Some(report) = value.get_mut("report") {
                *report = json!([report["device"], report["inode"]]);
            }
            if let Some(mode) = value.get_mut("mode") {
                *mode = json!({"CancelOuter":null});
            }
            assert_positive(scope, value);
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn each_required_member_unknown_and_forbidden_null_sequence_arity_and_trailing_data_refuse() {
        for (scope, value, fields) in fixtures() {
            for name in std::iter::once("kind").chain(fields.iter().copied()) {
                let mut missing = value.clone();
                missing.as_object_mut().unwrap().remove(name);
                assert_negative(scope, &serde_json::to_vec(&missing).unwrap());
                let mut null = value.clone();
                null[name] = Value::Null;
                assert_negative(scope, &serde_json::to_vec(&null).unwrap());
                let encoded = serde_json::to_string(&value).unwrap();
                let duplicate = format!(
                    "{{{}:{},{}",
                    serde_json::to_string(name).unwrap(),
                    serde_json::to_string(&value[name]).unwrap(),
                    encoded.strip_prefix('{').unwrap()
                );
                assert_negative(scope, duplicate.as_bytes());
            }
            let mut unknown = value.clone();
            unknown["extra"] = Value::Null;
            assert_negative(scope, &serde_json::to_vec(&unknown).unwrap());
            let forbidden = if fields.contains(&"settings") {
                "authority"
            } else {
                "settings"
            };
            let mut opposite = value.clone();
            opposite[forbidden] = Value::Null;
            assert_negative(scope, &serde_json::to_vec(&opposite).unwrap());
            let mut unit_map = value.clone();
            unit_map["kind"] = json!({(value["kind"].as_str().unwrap()):null});
            assert_negative(scope, &serde_json::to_vec(&unit_map).unwrap());
            let mut short = sequence(&value, fields);
            let _ = short.as_array_mut().unwrap().pop();
            assert_negative(scope, &serde_json::to_vec(&short).unwrap());
            let mut extra = sequence(&value, fields);
            extra.as_array_mut().unwrap().push(Value::Null);
            assert_negative(scope, &serde_json::to_vec(&extra).unwrap());
            let mut trailing = serde_json::to_vec(&value).unwrap();
            trailing.extend_from_slice(b"{}");
            assert_negative(scope, &trailing);
            let complete = serde_json::to_vec(&value).unwrap();
            assert_negative(scope, &complete[..complete.len() - 1]);
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn original_mode_and_report_slot_types_refuse_without_slot_admission_policy() {
        let (scope, value, _) = fixtures()
            .into_iter()
            .find(|(_, value, _)| value.get("mode").is_some())
            .unwrap();
        for mode in [
            json!({"CancelOuter":false}),
            json!({"CancelOuter":null,"ObserveOuterExit":null}),
            json!(1),
        ] {
            let mut wrong = value.clone();
            wrong["mode"] = mode;
            assert_negative(scope, &serde_json::to_vec(&wrong).unwrap());
        }
        let (scope, value, _) = fixtures()
            .into_iter()
            .find(|(scope, _, _)| matches!(scope, Scope::Inner))
            .unwrap();
        assert_positive(scope, value.clone()); // -123 is a raw i32, not the expected-report-slot check.
        for slot in [json!(i64::from(i32::MAX) + 1), json!(-0.0), json!("3")] {
            let mut wrong = value.clone();
            wrong["report_slot"] = slot;
            assert_negative(scope, &serde_json::to_vec(&wrong).unwrap());
        }
    }
}

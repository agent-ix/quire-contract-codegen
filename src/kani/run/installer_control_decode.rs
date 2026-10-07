// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Borrowed I-origin installer controls over the actual original frame.
//!
//! These parsed facts grant no creator, policy, Dispatch, report or recipe authority. The
//! receiving installer authenticates original descriptors before owned materialization.

use std::mem::{size_of, size_of_val};

use super::{
    guardian_decode::{
        ArrayState, DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Scratch, Text,
        ValueKind,
    },
    native_os_decode::NativeOsTag,
    protocol::{RunAuthority, StdinControl},
    recipe_decode::{self, RecipeBytes},
    report_storage::PipeIdentity,
    role_protocol::InstallerControlKind,
    role_scalar_decode,
    settings_decode::{self, SettingsBytes},
};

pub(super) enum InstallerControl<'input> {
    Start {
        settings: SettingsBytes<'input>,
        report: PipeIdentity,
    },
    Exec {
        authority: RunAuthority,
        command: RecipeBytes<'input>,
        stdin: StdinControl,
    },
}

impl InstallerControl<'_> {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Start { .. } => 3,
            Self::Exec { stdin, .. } => stdin.rights_count(),
        }
    }
}

fn error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}
fn required<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| error(DecodeCause::MissingField))
}

macro_rules! control_fields {
    ($($variant:ident => $name:literal),+ $(,)?) => {
        #[derive(Clone, Copy)]
        enum Field { $($variant),+ }
        impl Field {
            fn text(text: Text<'_>) -> Result<Self, DecodeError> {
                $(if text.equals($name) { return Ok(Self::$variant); })+
                Err(error(DecodeCause::UnknownField))
            }
            fn name(name: &str) -> Result<Self, DecodeError> {
                match name { $($name => Ok(Self::$variant),)+ _ => Err(error(DecodeCause::UnknownField)) }
            }
        }
    };
}
control_fields! { Kind => "kind", Settings => "settings", Report => "report", Authority => "authority", Command => "command", Stdin => "stdin" }

#[derive(Default)]
struct Fields<'input> {
    kind: Option<InstallerControlKind>,
    settings: Option<SettingsBytes<'input>>,
    report: Option<PipeIdentity>,
    authority: Option<RunAuthority>,
    command: Option<RecipeBytes<'input>>,
    stdin: Option<StdinControl>,
}
impl<'input> Fields<'input> {
    fn finish(self) -> Result<InstallerControl<'input>, DecodeError> {
        match required(self.kind)? {
            InstallerControlKind::Start => {
                if self.authority.is_some() || self.command.is_some() || self.stdin.is_some() {
                    return Err(error(DecodeCause::UnknownField));
                }
                Ok(InstallerControl::Start {
                    settings: required(self.settings)?,
                    report: required(self.report)?,
                })
            }
            InstallerControlKind::Exec => {
                if self.settings.is_some() || self.report.is_some() {
                    return Err(error(DecodeCause::UnknownField));
                }
                Ok(InstallerControl::Exec {
                    authority: required(self.authority)?,
                    command: required(self.command)?,
                    stdin: required(self.stdin)?,
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

fn field<'input>(
    decoder: &mut Decoder<'input, '_>,
    field: Field,
    fields: &mut Fields<'input>,
    tag: NativeOsTag,
) -> Result<(), DecodeError> {
    match field {
        Field::Kind => once!(
            fields.kind,
            InstallerControlKind::metadata_text(decoder.string()?)
                .ok_or_else(|| error(DecodeCause::InvalidValue))
        ),
        Field::Settings => once!(fields.settings, settings_decode::settings(decoder)),
        Field::Report => once!(fields.report, settings_decode::pipe_identity(decoder)),
        Field::Authority => once!(fields.authority, role_scalar_decode::authority(decoder)),
        Field::Command => once!(fields.command, recipe_decode::decode(decoder, tag)),
        Field::Stdin => once!(
            fields.stdin,
            StdinControl::metadata_text(decoder.unit_variant()?)
                .ok_or_else(|| error(DecodeCause::InvalidValue))
        ),
    }
    Ok(())
}

pub(super) fn decode<'input>(
    payload: &'input [u8],
    scratch: &mut Scratch,
    tag: NativeOsTag,
) -> Result<InstallerControl<'input>, DecodeError> {
    let mut decoder = Decoder::new(payload, scratch)?;
    let mut fields = Fields::default();
    match decoder.peek_kind()? {
        ValueKind::Object => {
            let mut object = decoder.begin_object()?;
            while let Some(name) = decoder.next_field(&mut object)? {
                field(&mut decoder, Field::text(name)?, &mut fields, tag)?;
            }
        }
        ValueKind::Array => {
            let mut array = decoder.begin_array()?;
            if !decoder.next_element(&mut array)? {
                return Err(error(DecodeCause::MissingField));
            }
            let kind = InstallerControlKind::metadata_text(decoder.string()?)
                .ok_or_else(|| error(DecodeCause::InvalidValue))?;
            fields.kind = Some(kind);
            for name in kind.declared_fields() {
                if !decoder.next_element(&mut array)? {
                    return Err(error(DecodeCause::MissingField));
                }
                field(&mut decoder, Field::name(name)?, &mut fields, tag)?;
            }
            if decoder.next_element(&mut array)? {
                return Err(error(DecodeCause::InvalidValue));
            }
        }
        ValueKind::String | ValueKind::Number | ValueKind::Boolean | ValueKind::Null => {
            return Err(error(DecodeCause::UnexpectedToken))
        }
    }
    let control = fields.finish()?;
    decoder.finish()?;
    Ok(control)
}

/// Envelope-layout delta only; reused settings/recipe/scalar/primitive reservations are separate.
/// Input/output allocations, initializer temporaries and native-stack highwater are not sizeof proofs.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<Fields<'static>>(),
        size_of::<InstallerControl<'static>>(),
        size_of::<Result<InstallerControl<'static>, DecodeError>>(),
        size_of::<InstallerControlKind>(),
        size_of::<Field>(),
        size_of::<ObjectState>(),
        size_of::<ArrayState>(),
        size_of::<Text<'static>>(),
        size_of::<&[&str]>(),
        size_of::<ValueKind>(),
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
    // Owning grammar and exact output units only, not creator/policy/Dispatch/pipe proof.
    use super::*;
    use crate::kani::run::{
        namespace::BackendCommand,
        native_os_decode,
        protocol::current_build_identity,
        role_deadline::{IdentityDeadline, MonotonicInstant, RoleDeadline},
        role_protocol::{BackendInstallerControl, RunSettings},
        settings_materialize,
    };
    use std::{num::NonZeroU64, time::Duration};

    fn controls() -> [BackendInstallerControl; 2] {
        let settings = RunSettings {
            helper: "/helper\nλ".into(),
            identity: current_build_identity(),
            authority: RunAuthority::from_wire_bytes([3; 32]),
            deadline: IdentityDeadline::NeverElapses,
            started: MonotonicInstant::from_wire_parts(1, 2),
            settlement_reserve: Duration::from_secs(1),
            work_deadline: IdentityDeadline::NeverElapses,
            setup_deadline: RoleDeadline::from_wire_parts(4, 5),
            caller_uid: 7,
            caller_gid: 8,
            memory_bytes: NonZeroU64::new(123).unwrap(),
            caller_run_buffers: 321,
        };
        [
            BackendInstallerControl::Start {
                settings,
                report: PipeIdentity::from_wire_parts(9, 10),
            },
            BackendInstallerControl::Exec {
                authority: RunAuthority::from_wire_bytes([3; 32]),
                command: BackendCommand::from_received_parts(
                    "program".into(),
                    vec!["arg".into()],
                    None,
                    vec![("KEY".into(), "value".into())],
                ),
                stdin: StdinControl::Open,
            },
        ]
    }

    fn canonical(bytes: &[u8]) {
        let expected: BackendInstallerControl = serde_json::from_slice(bytes).unwrap();
        let mut scratch = Scratch::default();
        let tag = native_os_decode::native_tag().unwrap();
        let parsed = decode(bytes, &mut scratch, tag).unwrap();
        assert_eq!(parsed.rights_count(), expected.rights_count());
        let actual = match parsed {
            InstallerControl::Start { settings, report } => BackendInstallerControl::Start {
                settings: settings_materialize::materialize(settings).unwrap(),
                report,
            },
            InstallerControl::Exec {
                authority,
                command,
                stdin,
            } => BackendInstallerControl::Exec {
                authority,
                command: recipe_decode::materialize(command, tag, &mut scratch).unwrap(),
                stdin,
            },
        };
        assert_eq!(
            serde_json::to_value(actual).unwrap(),
            serde_json::to_value(expected).unwrap()
        );
    }
    fn refuses(bytes: &[u8]) {
        assert!(serde_json::from_slice::<BackendInstallerControl>(bytes).is_err());
        let mut scratch = Scratch::default();
        assert!(decode(bytes, &mut scratch, native_os_decode::native_tag().unwrap()).is_err());
    }

    /// Trace: FR-034-AC-15, FR-034-AC-17
    #[test]
    fn original_installer_object_and_tagged_sequences_materialize_exact_fields() {
        for control in controls() {
            let value = serde_json::to_value(control).unwrap();
            canonical(&serde_json::to_vec(&value).unwrap());
            let kind = if value["kind"] == "Start" {
                InstallerControlKind::Start
            } else {
                InstallerControlKind::Exec
            };
            let mut sequence = vec![value["kind"].clone()];
            sequence.extend(
                kind.declared_fields()
                    .iter()
                    .map(|field| value[*field].clone()),
            );
            canonical(&serde_json::to_vec(&sequence).unwrap());
            for count in 0..sequence.len() {
                refuses(&serde_json::to_vec(&sequence[..count]).unwrap());
            }
            sequence.push(serde_json::Value::Null);
            refuses(&serde_json::to_vec(&sequence).unwrap());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn installer_missing_duplicate_opposite_and_kind_unit_fields_refuse() {
        for control in controls() {
            let value = serde_json::to_value(control).unwrap();
            for name in value.as_object().unwrap().keys() {
                let mut missing = value.clone();
                missing.as_object_mut().unwrap().remove(name);
                refuses(&serde_json::to_vec(&missing).unwrap());
            }
            let mut opposite = value.clone();
            let name = if value["kind"] == "Start" {
                "authority"
            } else {
                "report"
            };
            opposite[name] = serde_json::Value::Null;
            refuses(&serde_json::to_vec(&opposite).unwrap());
            let mut kind_unit = value.clone();
            let name = value["kind"].as_str().unwrap();
            kind_unit["kind"] = serde_json::json!({ name: null });
            refuses(&serde_json::to_vec(&kind_unit).unwrap());
        }
        refuses(br#"{"kind":"Exec","kind":"Exec"}"#);
    }
}

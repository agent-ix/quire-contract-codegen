// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Borrowed CallerControl facts on one original frame/cursor (FR-034).
//!
//! The owning actor retains credentials, descriptors, original clocks and frame lifetime, then
//! authenticates before exact owned recipe materialization. This module supplies no admission,
//! cleanup, allocation cap, error replay or execution authority.

use std::mem::{size_of, size_of_val};

use super::{
    guardian_decode::{DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Scratch, Text},
    native_os_decode::NativeOsTag,
    protocol::{BuildIdentity, CallerControlKind, RunAuthority, StdinControl},
    recipe_decode::{self, RecipeBytes, RecipeSequence},
    role_scalar_decode,
};

/// Borrowed schema only; frame and ancillary custody remain with the original receiver.
pub(super) enum CallerReply<'input> {
    Hello {
        identity: BuildIdentity,
        authority: RunAuthority,
    },
    Dispatch {
        authority: RunAuthority,
        command: RecipeBytes<'input>,
        stdin: StdinControl,
        cleanup_paths: RecipeSequence<'input>,
    },
}

impl CallerReply<'_> {
    /// Expected descriptor count from the owning stdin schema, not descriptor validation.
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Hello { .. } => 0,
            Self::Dispatch { stdin, .. } => stdin.rights_count(),
        }
    }
}

fn error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}
fn required<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| error(DecodeCause::MissingField))
}

#[derive(Clone, Copy)]
enum Field {
    Kind,
    Identity,
    Authority,
    Command,
    Stdin,
    CleanupPaths,
}
impl Field {
    fn from_text(text: Text<'_>) -> Result<Self, DecodeError> {
        if text.equals("kind") {
            Ok(Self::Kind)
        } else if text.equals("identity") {
            Ok(Self::Identity)
        } else if text.equals("authority") {
            Ok(Self::Authority)
        } else if text.equals("command") {
            Ok(Self::Command)
        } else if text.equals("stdin") {
            Ok(Self::Stdin)
        } else if text.equals("cleanup_paths") {
            Ok(Self::CleanupPaths)
        } else {
            Err(error(DecodeCause::UnknownField))
        }
    }
}

#[derive(Default)]
struct Fields<'input> {
    kind: Option<CallerControlKind>,
    identity: Option<BuildIdentity>,
    authority: Option<RunAuthority>,
    command: Option<RecipeBytes<'input>>,
    stdin: Option<StdinControl>,
    cleanup_paths: Option<RecipeSequence<'input>>,
}
impl<'input> Fields<'input> {
    fn finish(self) -> Result<CallerReply<'input>, DecodeError> {
        let authority = required(self.authority)?;
        match required(self.kind)? {
            CallerControlKind::Hello => {
                if self.command.is_some() || self.stdin.is_some() || self.cleanup_paths.is_some() {
                    return Err(error(DecodeCause::UnknownField));
                }
                Ok(CallerReply::Hello {
                    identity: required(self.identity)?,
                    authority,
                })
            }
            CallerControlKind::Dispatch => {
                if self.identity.is_some() {
                    return Err(error(DecodeCause::UnknownField));
                }
                Ok(CallerReply::Dispatch {
                    authority,
                    command: required(self.command)?,
                    stdin: required(self.stdin)?,
                    cleanup_paths: required(self.cleanup_paths)?,
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

/// Decode the whole original caller schema, with no context/fault-slot or owned allocations.
pub(super) fn decode<'input>(
    payload: &'input [u8],
    scratch: &mut Scratch,
    tag: NativeOsTag,
) -> Result<CallerReply<'input>, DecodeError> {
    let mut decoder = Decoder::new(payload, scratch)?;
    let mut object = decoder.begin_object()?;
    let mut fields = Fields::default();
    while let Some(name) = decoder.next_field(&mut object)? {
        match Field::from_text(name)? {
            // Internally tagged CallerControl requires a string; no external-unit alternative.
            Field::Kind => once!(
                fields.kind,
                CallerControlKind::metadata_text(decoder.string()?)
                    .ok_or_else(|| error(DecodeCause::InvalidValue))
            ),
            Field::Identity => once!(fields.identity, role_scalar_decode::identity(&mut decoder)),
            Field::Authority => once!(
                fields.authority,
                role_scalar_decode::authority(&mut decoder)
            ),
            Field::Command => once!(fields.command, recipe_decode::decode(&mut decoder, tag)),
            Field::Stdin => once!(
                fields.stdin,
                StdinControl::metadata_text(decoder.unit_variant()?)
                    .ok_or_else(|| error(DecodeCause::InvalidValue))
            ),
            Field::CleanupPaths => once!(
                fields.cleanup_paths,
                recipe_decode::values(&mut decoder, tag)
            ),
        }
    }
    let reply = fields.finish()?;
    decoder.finish()?;
    Ok(reply)
}

/// Checked envelope-layout delta; primitive/scalar/native/recipe reservations are separate.
/// Borrowed frames/output capacity and resident Scratch are excluded. Compiler initialization,
/// return-place/transient storage and native-stack highwater remain unmeasured.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<Fields<'static>>(),
        size_of::<Field>(),
        size_of::<CallerControlKind>(),
        size_of::<CallerReply<'static>>(),
        size_of::<Result<CallerReply<'static>, DecodeError>>(),
        size_of::<StdinControl>(),
        size_of::<&mut Decoder<'static, 'static>>(),
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
    // Representation units only: no authentication, descriptor possession or exec admission.
    use std::{
        ffi::OsString,
        os::unix::ffi::{OsStrExt, OsStringExt},
    };

    use super::*;
    use crate::kani::run::{
        namespace::BackendCommand,
        native_os_decode,
        protocol::{current_build_identity, CallerControl},
    };

    fn authority() -> RunAuthority {
        RunAuthority::from_wire_bytes([5; 32])
    }
    fn recipe() -> BackendCommand {
        BackendCommand::from_received_parts(
            OsString::from_vec(vec![b'p', 255, 0]),
            vec![OsString::from_vec(vec![128, 0]), OsString::new()],
            Some(OsString::from_vec(vec![b'd', 254])),
            vec![(
                OsString::from_vec(vec![b'K', 253]),
                OsString::from_vec(vec![252, 0]),
            )],
        )
    }
    fn hello() -> CallerControl {
        CallerControl::Hello {
            identity: current_build_identity(),
            authority: authority(),
        }
    }
    fn dispatch(stdin: StdinControl) -> CallerControl {
        CallerControl::Dispatch {
            authority: authority(),
            command: recipe(),
            stdin,
            cleanup_paths: vec![OsString::from_vec(vec![b'x', 251]), OsString::new()],
        }
    }
    fn value(control: CallerControl) -> serde_json::Value {
        serde_json::to_value(control).unwrap()
    }
    fn refuses(bytes: &[u8]) {
        // Differential oracle is the original owning Deserialize, not a mirrored schema.
        assert!(serde_json::from_slice::<CallerControl>(bytes).is_err());
        let mut scratch = Scratch::default();
        assert!(decode(bytes, &mut scratch, native_os_decode::native_tag().unwrap()).is_err());
    }
    fn original_bytes(
        bytes: native_os_decode::NativeOsBytes<'_>,
        scratch: &mut Scratch,
    ) -> Vec<u8> {
        let mut output = vec![0; bytes.byte_count()];
        native_os_decode::write_bytes(bytes, &mut output, scratch).unwrap();
        output
    }

    /// Trace: FR-034-AC-15, FR-034-AC-17
    #[test]
    fn owning_serialization_preserves_hello_dispatch_rights_and_original_native_recipe() {
        let mut scratch = Scratch::default();
        let bytes = serde_json::to_vec(&hello()).unwrap();
        assert!(matches!(
            serde_json::from_slice::<CallerControl>(&bytes).unwrap(),
            CallerControl::Hello { .. }
        ));
        let parsed = decode(
            &bytes,
            &mut scratch,
            native_os_decode::native_tag().unwrap(),
        )
        .unwrap();
        assert_eq!(parsed.rights_count(), 0);
        let CallerReply::Hello {
            identity,
            authority: actual,
        } = parsed
        else {
            panic!("Hello changed branch");
        };
        assert_eq!(identity, current_build_identity());
        assert_eq!(actual, authority());
        for stdin in [StdinControl::Open, StdinControl::Closed] {
            let bytes = serde_json::to_vec(&dispatch(stdin)).unwrap();
            let CallerControl::Dispatch {
                command: owning,
                cleanup_paths: owning_paths,
                ..
            } = serde_json::from_slice::<CallerControl>(&bytes).unwrap()
            else {
                panic!("owning Dispatch changed branch");
            };
            let parsed = decode(
                &bytes,
                &mut scratch,
                native_os_decode::native_tag().unwrap(),
            )
            .unwrap();
            assert_eq!(parsed.rights_count(), stdin.rights_count());
            let CallerReply::Dispatch {
                authority: actual,
                command,
                stdin: actual_stdin,
                cleanup_paths,
            } = parsed
            else {
                panic!("Dispatch changed branch");
            };
            assert_eq!(actual, authority());
            assert_eq!(actual_stdin, stdin);
            let owning_value = serde_json::to_value(owning).unwrap();
            let program: OsString =
                serde_json::from_value(owning_value["program"].clone()).unwrap();
            assert_eq!(
                original_bytes(command.program, &mut scratch),
                program.as_bytes()
            );
            let directory: OsString =
                serde_json::from_value(owning_value["directory"].clone()).unwrap();
            assert_eq!(
                original_bytes(command.directory.unwrap(), &mut scratch),
                directory.as_bytes()
            );
            let arguments: Vec<OsString> =
                serde_json::from_slice(command.arguments.encoded()).unwrap();
            let expected_arguments: Vec<OsString> =
                serde_json::from_value(owning_value["arguments"].clone()).unwrap();
            assert_eq!(arguments, expected_arguments);
            assert_eq!(command.arguments.count(), arguments.len());
            let environment: Vec<(OsString, OsString)> =
                serde_json::from_slice(command.environment.encoded()).unwrap();
            let expected_environment: Vec<(OsString, OsString)> =
                serde_json::from_value(owning_value["environment"].clone()).unwrap();
            assert_eq!(environment, expected_environment);
            assert_eq!(command.environment.count(), environment.len());
            let paths: Vec<OsString> = serde_json::from_slice(cleanup_paths.encoded()).unwrap();
            assert_eq!(paths, owning_paths);
            assert_eq!(cleanup_paths.count(), paths.len());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn internal_kind_maps_refuse_while_ordinary_stdin_unit_maps_remain_admitted() {
        let mut scratch = Scratch::default();
        for original in [value(hello()), value(dispatch(StdinControl::Open))] {
            let name = original["kind"].as_str().unwrap();
            let mut changed = original.clone();
            changed["kind"] = serde_json::json!({(name): null});
            refuses(&serde_json::to_vec(&changed).unwrap());
        }
        for stdin in [StdinControl::Open, StdinControl::Closed] {
            let mut changed = value(dispatch(stdin));
            let name = changed["stdin"].as_str().unwrap().to_owned();
            changed["stdin"] = serde_json::json!({(name.clone()): null});
            let bytes = serde_json::to_vec(&changed).unwrap();
            let CallerControl::Dispatch { stdin: owning, .. } =
                serde_json::from_slice::<CallerControl>(&bytes).unwrap()
            else {
                panic!("stdin map changed owning branch");
            };
            let parsed = decode(
                &bytes,
                &mut scratch,
                native_os_decode::native_tag().unwrap(),
            )
            .unwrap();
            assert_eq!(parsed.rights_count(), stdin.rights_count());
            let CallerReply::Dispatch { stdin: actual, .. } = parsed else {
                panic!("stdin map changed branch");
            };
            assert_eq!(actual, owning);
            assert_eq!(actual, stdin);
            changed["stdin"] = serde_json::json!({(name): false});
            refuses(&serde_json::to_vec(&changed).unwrap());
            changed["stdin"] = serde_json::json!({"Open": null, "Closed": null});
            refuses(&serde_json::to_vec(&changed).unwrap());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn envelope_required_duplicate_unknown_opposite_and_eof_guards_match_owner() {
        let cases = [value(hello()), value(dispatch(StdinControl::Closed))];
        let mut all = serde_json::Map::new();
        for case in &cases {
            all.extend(case.as_object().unwrap().clone());
        }
        for original in cases {
            for (field, member) in original.as_object().unwrap() {
                let mut changed = original.clone();
                changed.as_object_mut().unwrap().remove(field);
                refuses(&serde_json::to_vec(&changed).unwrap());
                for wrong in [serde_json::Value::Null, serde_json::json!(false)] {
                    let mut changed = original.clone();
                    changed[field] = wrong;
                    refuses(&serde_json::to_vec(&changed).unwrap());
                }
                let bytes = serde_json::to_string(&original).unwrap();
                let duplicate = bytes.replacen('{', &format!("{{\"{field}\":{member},"), 1);
                refuses(duplicate.as_bytes());
            }
            for (field, member) in &all {
                if !original.as_object().unwrap().contains_key(field) {
                    let mut changed = original.clone();
                    changed[field] = member.clone();
                    refuses(&serde_json::to_vec(&changed).unwrap());
                }
            }
            let mut changed = original.clone();
            changed["foreign"] = serde_json::json!(true);
            refuses(&serde_json::to_vec(&changed).unwrap());
            let mut bytes = serde_json::to_vec(&original).unwrap();
            bytes.push(b'?');
            refuses(&bytes);
            bytes.pop();
            bytes.pop();
            refuses(&bytes);
        }
    }

    /// Trace: FR-034-AC-15, FR-034-AC-17
    #[test]
    fn delegated_recipe_preserves_optional_directory_and_refuses_bad_native_tuple_bytes() {
        let original = value(dispatch(StdinControl::Closed));
        let mut scratch = Scratch::default();
        for absent in [false, true] {
            let mut changed = original.clone();
            if absent {
                changed["command"]
                    .as_object_mut()
                    .unwrap()
                    .remove("directory");
            } else {
                changed["command"]["directory"] = serde_json::Value::Null;
            }
            let bytes = serde_json::to_vec(&changed).unwrap();
            assert!(serde_json::from_slice::<CallerControl>(&bytes).is_ok());
            let CallerReply::Dispatch { command, .. } = decode(
                &bytes,
                &mut scratch,
                native_os_decode::native_tag().unwrap(),
            )
            .unwrap() else {
                panic!("directory changed branch");
            };
            assert!(command.directory.is_none());
        }
        let native = original["command"]["program"].clone();
        for tuple in [
            serde_json::json!([]),
            serde_json::json!([native.clone()]),
            serde_json::json!([native.clone(), native.clone(), native.clone()]),
        ] {
            let mut changed = original.clone();
            changed["command"]["environment"] = serde_json::json!([tuple]);
            refuses(&serde_json::to_vec(&changed).unwrap());
        }
        let label = native.as_object().unwrap().keys().next().unwrap();
        let mut changed = original.clone();
        changed["command"]["program"] = serde_json::json!({(label): [256]});
        refuses(&serde_json::to_vec(&changed).unwrap());
        let mut changed = original;
        changed["command"]["directory"] = serde_json::Value::Null;
        let bytes = serde_json::to_string(&changed).unwrap().replacen(
            "\"directory\":null",
            "\"directory\":null,\"directory\":null",
            1,
        );
        refuses(bytes.as_bytes());
    }
}

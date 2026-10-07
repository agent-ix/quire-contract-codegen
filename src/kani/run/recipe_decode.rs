// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Borrowed exact recipe facts from the original guardian control frame.
//!
//! No recipe becomes owned here. The receiving actor authenticates its complete frame and
//! retained descriptors before materializing these native bytes. Counts describe logical
//! values, not allocation capacities or a native-stack bound.

use std::{
    collections::TryReserveError,
    error::Error,
    ffi::OsString,
    fmt,
    mem::{size_of, size_of_val},
    os::unix::ffi::OsStringExt,
};

use super::{
    guardian_decode::{
        ArrayState, CursorMark, DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState,
        Scratch, ValueKind,
    },
    namespace::{BackendCommand, BackendCommandField},
    native_os_decode::{self, NativeOsBytes, NativeOsTag},
};

/// One already visited array, still borrowing the original immutable control payload.
#[derive(Clone, Copy)]
pub(super) struct RecipeSequence<'input> {
    encoded: &'input [u8],
    count: usize,
}

impl<'input> RecipeSequence<'input> {
    pub(super) const fn encoded(self) -> &'input [u8] {
        self.encoded
    }

    pub(super) const fn count(self) -> usize {
        self.count
    }
}

/// Exact parsed values; no field carries pathname, execution or admission authority.
#[derive(Clone, Copy)]
pub(super) struct RecipeBytes<'input> {
    pub(super) program: NativeOsBytes<'input>,
    pub(super) arguments: RecipeSequence<'input>,
    pub(super) directory: Option<NativeOsBytes<'input>>,
    pub(super) environment: RecipeSequence<'input>,
}

#[derive(Default)]
struct RecipeFields<'input> {
    program: Option<NativeOsBytes<'input>>,
    arguments: Option<RecipeSequence<'input>>,
    // The owning derived Option admits a missing directory and explicit null. This outer
    // Option records field presence so duplicate null remains a refusal.
    directory: Option<Option<NativeOsBytes<'input>>>,
    environment: Option<RecipeSequence<'input>>,
}

pub(super) fn decode<'input>(
    decoder: &mut Decoder<'input, '_>,
    tag: NativeOsTag,
) -> Result<RecipeBytes<'input>, DecodeError> {
    let mut object = decoder.begin_object()?;
    let mut fields = RecipeFields::default();
    while let Some(field) = decoder.next_field(&mut object)? {
        match BackendCommandField::metadata_text(field)
            .ok_or_else(|| fault(DecodeCause::UnknownField))?
        {
            BackendCommandField::Program => {
                refuse_duplicate(fields.program.is_some())?;
                fields.program = Some(native_os_decode::decode(decoder, tag)?);
            }
            BackendCommandField::Arguments => {
                refuse_duplicate(fields.arguments.is_some())?;
                fields.arguments = Some(sequence(decoder, tag, SequenceKind::Arguments)?);
            }
            BackendCommandField::Directory => {
                refuse_duplicate(fields.directory.is_some())?;
                fields.directory = Some(if matches!(decoder.peek_kind()?, ValueKind::Null) {
                    decoder.null()?;
                    None
                } else {
                    Some(native_os_decode::decode(decoder, tag)?)
                });
            }
            BackendCommandField::Environment => {
                refuse_duplicate(fields.environment.is_some())?;
                fields.environment = Some(sequence(decoder, tag, SequenceKind::Environment)?);
            }
        }
    }
    Ok(RecipeBytes {
        program: fields
            .program
            .ok_or_else(|| fault(DecodeCause::MissingField))?,
        arguments: fields
            .arguments
            .ok_or_else(|| fault(DecodeCause::MissingField))?,
        directory: fields.directory.flatten(),
        environment: fields
            .environment
            .ok_or_else(|| fault(DecodeCause::MissingField))?,
    })
}

/// Visit an ordinary native-value array, including the existing cleanup-path collection.
/// The owning actor separately enforces its existing artifact count before materialization.
pub(super) fn values<'input>(
    decoder: &mut Decoder<'input, '_>,
    tag: NativeOsTag,
) -> Result<RecipeSequence<'input>, DecodeError> {
    sequence(decoder, tag, SequenceKind::Arguments)
}

#[derive(Clone, Copy)]
enum SequenceKind {
    Arguments,
    Environment,
}

fn sequence<'input>(
    decoder: &mut Decoder<'input, '_>,
    tag: NativeOsTag,
    kind: SequenceKind,
) -> Result<RecipeSequence<'input>, DecodeError> {
    let mark = decoder.mark();
    let mut array = decoder.begin_array()?;
    let mut count = 0_usize;
    while decoder.next_element(&mut array)? {
        if matches!(kind, SequenceKind::Environment) {
            // The owning Vec<(OsString, OsString)> admits exactly a two-element tuple.
            let mut tuple = decoder.begin_array()?;
            if !decoder.next_element(&mut tuple)? {
                return Err(fault(DecodeCause::MissingField));
            }
            native_os_decode::decode(decoder, tag)?;
            if !decoder.next_element(&mut tuple)? {
                return Err(fault(DecodeCause::MissingField));
            }
            native_os_decode::decode(decoder, tag)?;
            if decoder.next_element(&mut tuple)? {
                return Err(fault(DecodeCause::InvalidValue));
            }
        } else {
            native_os_decode::decode(decoder, tag)?;
        }
        count = count
            .checked_add(1)
            .ok_or_else(|| fault(DecodeCause::StorageBound))?;
    }
    Ok(RecipeSequence {
        encoded: decoder.consumed_since(mark)?,
        count,
    })
}

/// Actual local allocation or immutable-span check failure; no cause is reconstructed.
#[derive(Debug)]
pub(super) enum MaterializationError {
    Grammar(DecodeError),
    Allocation(TryReserveError),
}

impl fmt::Display for MaterializationError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Grammar(error) => error.fmt(output),
            Self::Allocation(error) => error.fmt(output),
        }
    }
}

impl Error for MaterializationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Grammar(error) => Some(error),
            Self::Allocation(error) => Some(error),
        }
    }
}

impl From<DecodeError> for MaterializationError {
    fn from(error: DecodeError) -> Self {
        Self::Grammar(error)
    }
}

/// Materialize ONLY after the actor has authenticated the full original frame and retained
/// its descriptors. Each fallible reserve preserves its actual local TryReserveError. Logical
/// counts do not establish allocation capacities; the owning actor accounts actual retained
/// recipe capacities/role RSS separately, without a CONTROL_BYTES allocation-bound claim.
pub(super) fn materialize(
    recipe: RecipeBytes<'_>,
    tag: NativeOsTag,
    scratch: &mut Scratch,
) -> Result<BackendCommand, MaterializationError> {
    let program = native_value(recipe.program, scratch)?;
    let arguments = materialize_values(recipe.arguments, tag, scratch)?;
    let directory = recipe
        .directory
        .map(|value| native_value(value, scratch))
        .transpose()?;
    let mut environment = Vec::new();
    environment
        .try_reserve_exact(recipe.environment.count)
        .map_err(MaterializationError::Allocation)?;
    {
        let mut decoder = Decoder::new(recipe.environment.encoded, scratch)?;
        let mut array = decoder.begin_array()?;
        while decoder.next_element(&mut array)? {
            if environment.len() >= recipe.environment.count {
                return Err(fault(DecodeCause::StorageBound).into());
            }
            let mut tuple = decoder.begin_array()?;
            if !decoder.next_element(&mut tuple)? {
                return Err(fault(DecodeCause::MissingField).into());
            }
            let name = native_os_decode::decode(&mut decoder, tag)?;
            if !decoder.next_element(&mut tuple)? {
                return Err(fault(DecodeCause::MissingField).into());
            }
            let value = native_os_decode::decode(&mut decoder, tag)?;
            if decoder.next_element(&mut tuple)? {
                return Err(fault(DecodeCause::InvalidValue).into());
            }
            let name = decoder.with_scratch(|scratch| native_value(name, scratch))?;
            let value = decoder.with_scratch(|scratch| native_value(value, scratch))?;
            environment.push((name, value));
        }
        decoder.finish()?;
    }
    if environment.len() != recipe.environment.count {
        return Err(fault(DecodeCause::StorageBound).into());
    }
    Ok(BackendCommand::from_received_parts(
        program,
        arguments,
        directory,
        environment,
    ))
}

/// Materialize the already validated ordinary native collection after actor authentication.
/// Existing semantic collection limits (such as artifact count) stay with the actor.
pub(super) fn materialize_values(
    sequence: RecipeSequence<'_>,
    tag: NativeOsTag,
    scratch: &mut Scratch,
) -> Result<Vec<OsString>, MaterializationError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(sequence.count)
        .map_err(MaterializationError::Allocation)?;
    {
        let mut decoder = Decoder::new(sequence.encoded, scratch)?;
        let mut array = decoder.begin_array()?;
        while decoder.next_element(&mut array)? {
            if values.len() >= sequence.count {
                return Err(fault(DecodeCause::StorageBound).into());
            }
            let value = native_os_decode::decode(&mut decoder, tag)?;
            values.push(decoder.with_scratch(|scratch| native_value(value, scratch))?);
        }
        decoder.finish()?;
    }
    if values.len() != sequence.count {
        return Err(fault(DecodeCause::StorageBound).into());
    }
    Ok(values)
}

fn native_value(
    value: NativeOsBytes<'_>,
    scratch: &mut Scratch,
) -> Result<OsString, MaterializationError> {
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(value.byte_count())
        .map_err(MaterializationError::Allocation)?;
    bytes.resize(value.byte_count(), 0);
    native_os_decode::write_bytes(value, &mut bytes, scratch)?;
    Ok(OsString::from_vec(bytes))
}

fn refuse_duplicate(seen: bool) -> Result<(), DecodeError> {
    if seen {
        return Err(fault(DecodeCause::DuplicateField));
    }
    Ok(())
}

const fn fault(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}

/// Declared fixed schema storage beyond the supplied primitive/native-value workspaces.
/// This does not prove compiler initializer/return-place/native-stack highwater or output caps.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<RecipeFields<'static>>(),
        size_of::<RecipeBytes<'static>>(),
        size_of::<RecipeSequence<'static>>(),
        size_of::<Result<RecipeBytes<'static>, DecodeError>>(),
        size_of::<Result<RecipeSequence<'static>, DecodeError>>(),
        size_of::<ObjectState>(),
        size_of::<ArrayState>(),
        size_of::<ArrayState>(),
        size_of::<CursorMark<'static>>(),
        size_of::<BackendCommandField>(),
        size_of::<SequenceKind>(),
        size_of::<ValueKind>(),
        size_of::<NativeOsTag>(),
        size_of::<usize>(),
        size_of::<Result<(), DecodeError>>(),
        // Sequential materialization owns these actual fixed headers/errors while its
        // retained output capacities are measured separately by BackendCommand/role RSS.
        size_of::<Vec<u8>>(),
        size_of::<Vec<OsString>>(),
        size_of::<Vec<(OsString, OsString)>>(),
        size_of::<OsString>(),
        size_of::<OsString>(),
        size_of::<OsString>(),
        size_of::<Option<OsString>>(),
        size_of::<BackendCommand>(),
        size_of::<MaterializationError>(),
        size_of::<TryReserveError>(),
        size_of::<Result<OsString, MaterializationError>>(),
        size_of::<Result<BackendCommand, MaterializationError>>(),
        // The outer sequence cursor remains live during native span materialization's
        // inner cursor, using the SAME Scratch. Primitive reservation covers one cursor.
        size_of::<Decoder<'static, 'static>>(),
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
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    use super::super::{guardian_decode::Scratch, namespace::BackendCommand};
    use super::*;

    fn parse<'input>(
        payload: &'input [u8],
        scratch: &mut Scratch,
    ) -> Result<RecipeBytes<'input>, DecodeError> {
        let tag = native_os_decode::native_tag().unwrap();
        let mut decoder = Decoder::new(payload, scratch)?;
        let recipe = decode(&mut decoder, tag)?;
        decoder.finish()?;
        Ok(recipe)
    }

    fn fixture() -> BackendCommand {
        BackendCommand::from_received_parts(
            OsString::from_vec(b"program-\xff\0".to_vec()),
            vec![OsString::from_vec(b"argument-\xfe".to_vec())],
            Some(OsString::from_vec(b"directory-\xfd".to_vec())),
            vec![(
                OsString::from_vec(b"name-\xfc".to_vec()),
                OsString::from_vec(b"value-\xfb\0".to_vec()),
            )],
        )
    }

    /// Trace: FR-034-AC-15, FR-034-AC-17
    #[test]
    fn native_recipe_facts_keep_original_spans_and_exact_non_utf8_bytes() {
        let payload = serde_json::to_vec(&fixture()).unwrap();
        let original: BackendCommand = serde_json::from_slice(&payload).unwrap();
        assert_eq!(serde_json::to_vec(&original).unwrap(), payload);
        let mut scratch = Scratch::default();
        let recipe = parse(&payload, &mut scratch).unwrap();
        let mut program = vec![0; recipe.program.byte_count()];
        native_os_decode::write_bytes(recipe.program, &mut program, &mut scratch).unwrap();
        assert_eq!(program, b"program-\xff\0");
        assert_eq!(recipe.arguments.count(), 1);
        assert_eq!(recipe.environment.count(), 1);
        for span in [recipe.arguments.encoded(), recipe.environment.encoded()] {
            assert!(payload
                .windows(span.len())
                .any(|window| std::ptr::eq(window.as_ptr(), span.as_ptr())));
        }
        let directory = recipe.directory.expect("genuine captured directory");
        let mut bytes = vec![0; directory.byte_count()];
        native_os_decode::write_bytes(directory, &mut bytes, &mut scratch).unwrap();
        assert_eq!(bytes, b"directory-\xfd");
        let materialized = materialize(
            recipe,
            native_os_decode::native_tag().unwrap(),
            &mut scratch,
        )
        .unwrap();
        assert_eq!(serde_json::to_vec(&materialized).unwrap(), payload);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn recipe_option_grammar_and_tuple_arity_match_the_owning_deserializer() {
        let mut value = serde_json::to_value(fixture()).unwrap();
        let object = value.as_object_mut().unwrap();
        object.remove("directory");
        let missing = serde_json::to_vec(&value).unwrap();
        let mut scratch = Scratch::default();
        assert!(serde_json::from_slice::<BackendCommand>(&missing).is_ok());
        assert!(parse(&missing, &mut scratch).unwrap().directory.is_none());
        value["directory"] = serde_json::Value::Null;
        let null = serde_json::to_vec(&value).unwrap();
        assert!(serde_json::from_slice::<BackendCommand>(&null).is_ok());
        assert!(parse(&null, &mut scratch).unwrap().directory.is_none());

        let duplicate =
            String::from_utf8(null.clone())
                .unwrap()
                .replacen("{", "{\"directory\":null,", 1);
        assert!(serde_json::from_slice::<BackendCommand>(duplicate.as_bytes()).is_err());
        assert!(matches!(
            parse(duplicate.as_bytes(), &mut scratch),
            Err(error) if error.cause() == DecodeCause::DuplicateField
        ));
        for tuple in [
            serde_json::json!([]),
            serde_json::json!([value["program"].clone()]),
            serde_json::json!([
                value["program"].clone(),
                value["program"].clone(),
                value["program"].clone()
            ]),
        ] {
            value["environment"] = serde_json::json!([tuple]);
            let malformed = serde_json::to_vec(&value).unwrap();
            assert!(serde_json::from_slice::<BackendCommand>(&malformed).is_err());
            assert!(parse(&malformed, &mut scratch).is_err());
        }
    }
}

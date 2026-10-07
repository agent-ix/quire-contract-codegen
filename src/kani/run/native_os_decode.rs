// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Borrowed native recipe-byte facts from the authoritative OsStr serializer (FR-034).
//!
//! No recipe admission/authentication or owned string allocation occurs here. The owner retains
//! the original frame and supplies output storage only after authentication. Native labels are
//! captured from Serialize, not mirrored from dependency declarations.

use std::{
    error::Error,
    ffi::OsStr,
    fmt,
    mem::{size_of, size_of_val},
};

use serde::{
    ser::{Impossible, Serializer},
    Serialize,
};

use super::guardian_decode::{
    ArrayState, CursorMark, DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Scratch,
    Text,
};

/// Native external-variant label captured from the actual authoritative Serialize call.
#[derive(Clone, Copy)]
pub(super) struct NativeOsTag {
    variant: &'static str,
}

/// Actual tag-extraction refusal; no diagnostic formatting or dynamic error payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum NativeFormatError {
    UnsupportedSerialization,
}

impl fmt::Display for NativeFormatError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSerialization => output.write_str("unsupported native serialization"),
        }
    }
}
impl Error for NativeFormatError {}
impl serde::ser::Error for NativeFormatError {
    fn custom<T: fmt::Display>(_message: T) -> Self {
        Self::UnsupportedSerialization
    }
}

struct TagSerializer;

macro_rules! refuse {
    ($($name:ident($($arg:ident: $ty:ty),*) -> $result:ty;)+) => {
        $(fn $name(self, $($arg: $ty),*) -> Result<$result, NativeFormatError> {
            Err(NativeFormatError::UnsupportedSerialization)
        })+
    };
}

impl Serializer for TagSerializer {
    type Ok = NativeOsTag;
    type Error = NativeFormatError;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    refuse! {
        serialize_bool(_value: bool) -> Self::Ok;
        serialize_i8(_value: i8) -> Self::Ok;
        serialize_i16(_value: i16) -> Self::Ok;
        serialize_i32(_value: i32) -> Self::Ok;
        serialize_i64(_value: i64) -> Self::Ok;
        serialize_i128(_value: i128) -> Self::Ok;
        serialize_u8(_value: u8) -> Self::Ok;
        serialize_u16(_value: u16) -> Self::Ok;
        serialize_u32(_value: u32) -> Self::Ok;
        serialize_u64(_value: u64) -> Self::Ok;
        serialize_u128(_value: u128) -> Self::Ok;
        serialize_f32(_value: f32) -> Self::Ok;
        serialize_f64(_value: f64) -> Self::Ok;
        serialize_char(_value: char) -> Self::Ok;
        serialize_str(_value: &str) -> Self::Ok;
        serialize_bytes(_value: &[u8]) -> Self::Ok;
        serialize_none() -> Self::Ok;
        serialize_unit() -> Self::Ok;
        serialize_unit_struct(_name: &'static str) -> Self::Ok;
        serialize_unit_variant(_name: &'static str, _index: u32, _variant: &'static str) -> Self::Ok;
        serialize_seq(_length: Option<usize>) -> Self::SerializeSeq;
        serialize_tuple(_length: usize) -> Self::SerializeTuple;
        serialize_tuple_struct(_name: &'static str, _length: usize) -> Self::SerializeTupleStruct;
        serialize_tuple_variant(_name: &'static str, _index: u32, _variant: &'static str, _length: usize) -> Self::SerializeTupleVariant;
        serialize_map(_length: Option<usize>) -> Self::SerializeMap;
        serialize_struct(_name: &'static str, _length: usize) -> Self::SerializeStruct;
        serialize_struct_variant(_name: &'static str, _index: u32, _variant: &'static str, _length: usize) -> Self::SerializeStructVariant;
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(NativeFormatError::UnsupportedSerialization)
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(NativeFormatError::UnsupportedSerialization)
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(NativeOsTag { variant })
    }
    fn collect_str<T: ?Sized + fmt::Display>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        // Override serde's allocating default; never invoke Display.
        Err(NativeFormatError::UnsupportedSerialization)
    }
    fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        I::Item: Serialize,
    {
        Err(NativeFormatError::UnsupportedSerialization)
    }
    fn collect_map<K: Serialize, V: Serialize, I: IntoIterator<Item = (K, V)>>(
        self,
        _iter: I,
    ) -> Result<Self::Ok, Self::Error> {
        Err(NativeFormatError::UnsupportedSerialization)
    }
}

/// Capture the actual native label without visiting the payload or constructing JSON.
pub(super) fn native_tag() -> Result<NativeOsTag, NativeFormatError> {
    OsStr::new("").serialize(TagSerializer)
}

/// Original validated encoded array span and actual byte count, not owned recipe storage.
#[derive(Clone, Copy)]
pub(super) struct NativeOsBytes<'input> {
    encoded: &'input [u8],
    count: usize,
}
impl NativeOsBytes<'_> {
    pub(super) fn byte_count(&self) -> usize {
        self.count
    }
}

fn error(site: DecodeSite, cause: DecodeCause) -> DecodeError {
    DecodeError::new(site, cause)
}

/// Consume one exact native external byte-array value on the original cursor.
pub(super) fn decode<'input>(
    decoder: &mut Decoder<'input, '_>,
    tag: NativeOsTag,
) -> Result<NativeOsBytes<'input>, DecodeError> {
    let mut object = decoder.begin_object()?;
    let name = decoder
        .next_field(&mut object)?
        .ok_or_else(|| error(DecodeSite::Field, DecodeCause::MissingField))?;
    if !name.equals(tag.variant) {
        return Err(error(DecodeSite::Field, DecodeCause::InvalidValue));
    }
    let start = decoder.mark();
    let mut array = decoder.begin_array()?;
    let mut count = 0_usize;
    while decoder.next_element(&mut array)? {
        decoder.byte()?;
        count = count
            .checked_add(1)
            .ok_or_else(|| error(DecodeSite::Storage, DecodeCause::StorageBound))?;
    }
    let encoded = decoder.consumed_since(start)?;
    if decoder.next_field(&mut object)?.is_some() {
        return Err(error(DecodeSite::Field, DecodeCause::InvalidValue));
    }
    Ok(NativeOsBytes { encoded, count })
}

/// Materialize an already validated immutable span into exact caller-owned storage.
/// Frame authentication and output allocation/ownership remain the calling actor's duties.
pub(super) fn write_bytes(
    bytes: NativeOsBytes<'_>,
    output: &mut [u8],
    scratch: &mut Scratch,
) -> Result<(), DecodeError> {
    if output.len() != bytes.count {
        return Err(error(DecodeSite::Storage, DecodeCause::StorageBound));
    }
    let mut decoder = Decoder::new(bytes.encoded, scratch)?;
    let mut array = decoder.begin_array()?;
    let mut index = 0_usize;
    while decoder.next_element(&mut array)? {
        let byte = decoder.byte()?;
        let slot = output
            .get_mut(index)
            .ok_or_else(|| error(DecodeSite::Storage, DecodeCause::StorageBound))?;
        *slot = byte;
        index = index
            .checked_add(1)
            .ok_or_else(|| error(DecodeSite::Storage, DecodeCause::StorageBound))?;
    }
    if index != bytes.count {
        return Err(error(DecodeSite::Storage, DecodeCause::StorageBound));
    }
    decoder.finish()
}

/// Checked fixed schema/serializer/materialization records only, beyond primitive storage.
/// Existing Scratch, frame and output capacity are excluded. Layout terms do not establish
/// compiler initializer/return-place/transient storage or native-stack highwater.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<TagSerializer>(),
        // Actual OsStr serializer call borrows native payload/name without visiting bytes.
        size_of::<&OsStr>(),
        size_of::<&[u8]>(),
        size_of::<&'static str>(),
        size_of::<u32>(),
        size_of::<NativeOsTag>(),
        size_of::<NativeFormatError>(),
        size_of::<Result<NativeOsTag, NativeFormatError>>(),
        size_of::<NativeOsBytes<'static>>(),
        size_of::<Result<NativeOsBytes<'static>, DecodeError>>(),
        size_of::<Result<(), DecodeError>>(),
        size_of::<CursorMark<'static>>(),
        size_of::<ObjectState>(),
        size_of::<ArrayState>(),
        size_of::<Text<'static>>(),
        size_of::<&mut Decoder<'static, 'static>>(),
        size_of::<&mut [u8]>(),
        size_of::<&mut u8>(),
        size_of::<usize>(),
        size_of::<usize>(),
        size_of::<u8>(),
    ];
    let storage = || error(DecodeSite::Storage, DecodeCause::StorageBound);
    let table = u64::try_from(size_of_val(&terms)).map_err(|_| storage())?;
    terms.into_iter().try_fold(table, |sum, term| {
        sum.checked_add(u64::try_from(term).map_err(|_| storage())?)
            .ok_or_else(storage)
    })
}

#[cfg(test)]
mod tests {
    // Native representation units only: no frame authentication/owned recipe/resource acceptance.
    use std::{
        ffi::OsString,
        os::unix::ffi::{OsStrExt, OsStringExt},
    };

    use super::*;

    fn parsed<'input>(
        bytes: &'input [u8],
        scratch: &mut Scratch,
    ) -> Result<NativeOsBytes<'input>, DecodeError> {
        let mut decoder = Decoder::new(bytes, scratch)?;
        let native = decode(&mut decoder, native_tag().unwrap())?;
        decoder.finish()?;
        Ok(native)
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn real_native_serialization_and_deserialization_preserve_non_utf8_nul_and_empty() {
        let mut scratch = Scratch::default();
        for original in [
            OsString::new(),
            OsString::from_vec(vec![0, 255, 128, b'x']),
            OsString::from("text"),
        ] {
            let encoded = serde_json::to_vec(original.as_os_str()).unwrap();
            let authoritative: OsString = serde_json::from_slice(&encoded).unwrap();
            assert_eq!(authoritative, original);
            let bytes = parsed(&encoded, &mut scratch).unwrap();
            assert_eq!(bytes.byte_count(), original.as_bytes().len());
            let mut output = vec![0; bytes.byte_count()];
            write_bytes(bytes, &mut output, &mut scratch).unwrap();
            assert_eq!(output, original.as_bytes());
            assert_eq!(OsString::from_vec(output), authoritative);
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn native_tag_comes_from_real_serializer_and_schema_refuses_owning_adversaries() {
        let encoded = serde_json::to_vec(OsStr::new("")).unwrap();
        let tag = native_tag().unwrap();
        let mut scratch = Scratch::default();
        // Obtain the emitted label using the real borrowed primitive, not a mirrored catalog.
        let mut decoder = Decoder::new(&encoded, &mut scratch).unwrap();
        let mut object = decoder.begin_object().unwrap();
        assert!(decoder
            .next_field(&mut object)
            .unwrap()
            .unwrap()
            .equals(tag.variant));
        let label = serde_json::to_string(tag.variant).unwrap();
        for invalid in [
            "null".to_owned(),
            "\"text\"".to_owned(),
            "[]".to_owned(),
            "{}".to_owned(),
            "{\"ForeignNative\":[]}".to_owned(),
            // Actual owning Deserialize rejects the other platform's native representation.
            "{\"Windows\":[]}".to_owned(),
            format!("{{{label}:null}}"),
            format!("{{{label}:\"text\"}}"),
            format!("{{{label}:{{}}}}"),
            format!("{{{label}:[],\"extra\":[]}}"),
            format!("{{{label}:[],{label}:[]}}"),
            format!("{{{label}:[256]}}"),
            format!("{{{label}:[-1]}}"),
            format!("{{{label}:[-0]}}"),
            format!("{{{label}:[1.5]}}"),
            format!("{{{label}:[18446744073709551616]}}"),
            format!("{{{label}:[false]}}"),
            format!("{{{label}:[\"1\"]}}"),
            format!("{{{label}:[]}}?"),
            format!("{{{label}:[1]"),
        ] {
            assert!(serde_json::from_str::<OsString>(&invalid).is_err());
            assert!(parsed(invalid.as_bytes(), &mut scratch).is_err());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn escaped_tag_and_whitespace_span_materialize_without_outer_object_copy() {
        let tag = native_tag().unwrap();
        let first = tag.variant.chars().next().unwrap();
        let rest: String = tag.variant.chars().skip(1).collect();
        let encoded = format!(
            "{{\"\\u{:04x}{rest}\": \n [0, 255, 65] \t}}",
            u32::from(first)
        );
        let authoritative: OsString = serde_json::from_str(&encoded).unwrap();
        let mut scratch = Scratch::default();
        let bytes = parsed(encoded.as_bytes(), &mut scratch).unwrap();
        let mut output = [7_u8; 3];
        write_bytes(bytes, &mut output, &mut scratch).unwrap();
        assert_eq!(output.as_slice(), authoritative.as_bytes());
        // Span retains just the already visited original array, including preceding whitespace.
        assert!(bytes.encoded.ends_with(b"]"));
        assert!(!bytes.encoded.contains(&b'{'));
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn wrong_materialization_length_refuses_before_any_output_write() {
        let original = OsString::from_vec(vec![255, 0, 1]);
        let encoded = serde_json::to_vec(&original).unwrap();
        let mut scratch = Scratch::default();
        let bytes = parsed(&encoded, &mut scratch).unwrap();
        let mut short = [77; 2];
        let mut long = [77; 4];
        assert_eq!(
            write_bytes(bytes, &mut short, &mut scratch)
                .unwrap_err()
                .cause(),
            DecodeCause::StorageBound
        );
        assert_eq!(short, [77; 2]);
        assert_eq!(
            write_bytes(bytes, &mut long, &mut scratch)
                .unwrap_err()
                .cause(),
            DecodeCause::StorageBound
        );
        assert_eq!(long, [77; 4]);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn unsupported_tag_serializer_never_formats_supplied_diagnostics() {
        struct MustNotFormat;
        impl fmt::Display for MustNotFormat {
            fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
                panic!("unexpected Display invocation")
            }
        }
        assert_eq!(
            <NativeFormatError as serde::ser::Error>::custom(MustNotFormat),
            NativeFormatError::UnsupportedSerialization
        );
        assert_eq!(
            TagSerializer.collect_str(&MustNotFormat).err().unwrap(),
            NativeFormatError::UnsupportedSerialization
        );
        assert_eq!(
            TagSerializer.serialize_bool(false).err().unwrap(),
            NativeFormatError::UnsupportedSerialization
        );
        assert!(NativeFormatError::UnsupportedSerialization
            .source()
            .is_none());
    }
}

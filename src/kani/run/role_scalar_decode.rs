// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Fixed role scalar schemas over the guardian's borrowed decoder (FR-034).
//!
//! Returned identities, authority bytes and absolute time components are unverified parsed facts.
//! This module owns no clock, authentication, phase, original deadline, source projection or
//! frame. The same owner-held cursor/scratch is used throughout; the envelope owner finishes it.

use std::mem::{size_of, size_of_val};

use super::{
    cross_role_cause::{CauseIntegrityPredicate, CauseOperation},
    guardian_decode::{DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Text},
    protocol::{BuildIdentity, GuardianLifecycle, GuardianProtocol, RunAuthority},
    role_deadline::{MonotonicInstant, RoleDeadline, StopOrigin, StopStamp},
};

fn field_error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}

fn missing<T>(value: Option<T>) -> Result<T, DecodeError> {
    value.ok_or_else(|| field_error(DecodeCause::MissingField))
}

#[derive(Clone, Copy)]
enum IdentityField {
    Artifact,
    Protocol,
    Lifecycle,
}

impl IdentityField {
    fn from_text(name: Text<'_>) -> Result<Self, DecodeError> {
        if name.equals("artifact") {
            Ok(Self::Artifact)
        } else if name.equals("protocol") {
            Ok(Self::Protocol)
        } else if name.equals("lifecycle") {
            Ok(Self::Lifecycle)
        } else {
            Err(field_error(DecodeCause::UnknownField))
        }
    }
}

#[derive(Default)]
struct IdentityFields {
    artifact: Option<[u8; 32]>,
    protocol: Option<GuardianProtocol>,
    lifecycle: Option<GuardianLifecycle>,
}

/// Consume one exact BuildIdentity record without claiming current-build equality.
pub(super) fn identity(decoder: &mut Decoder<'_, '_>) -> Result<BuildIdentity, DecodeError> {
    let mut object = decoder.begin_object()?;
    let mut fields = IdentityFields::default();
    while let Some(name) = decoder.next_field(&mut object)? {
        match IdentityField::from_text(name)? {
            IdentityField::Artifact => {
                if fields.artifact.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                fields.artifact = Some(bytes32(decoder)?);
            }
            IdentityField::Protocol => {
                if fields.protocol.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                let value = GuardianProtocol::metadata_text(decoder.string()?)
                    .ok_or_else(|| field_error(DecodeCause::InvalidValue))?;
                fields.protocol = Some(value);
            }
            IdentityField::Lifecycle => {
                if fields.lifecycle.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                let value = GuardianLifecycle::metadata_text(decoder.string()?)
                    .ok_or_else(|| field_error(DecodeCause::InvalidValue))?;
                fields.lifecycle = Some(value);
            }
        }
    }
    Ok(BuildIdentity {
        artifact: missing(fields.artifact)?,
        protocol: missing(fields.protocol)?,
        lifecycle: missing(fields.lifecycle)?,
    })
}

fn bytes32(decoder: &mut Decoder<'_, '_>) -> Result<[u8; 32], DecodeError> {
    let mut array = decoder.begin_array()?;
    let mut bytes = [0_u8; 32];
    let mut index = 0_usize;
    while decoder.next_element(&mut array)? {
        let slot = bytes
            .get_mut(index)
            .ok_or_else(|| DecodeError::new(DecodeSite::Array, DecodeCause::InvalidValue))?;
        *slot = decoder.byte()?;
        index = index
            .checked_add(1)
            .ok_or_else(|| DecodeError::new(DecodeSite::Array, DecodeCause::IntegerOverflow))?;
    }
    if index != bytes.len() {
        return Err(DecodeError::new(
            DecodeSite::Array,
            DecodeCause::InvalidValue,
        ));
    }
    Ok(bytes)
}

/// Consume the existing exact authority array; these bytes establish no run or peer authority.
pub(super) fn authority(decoder: &mut Decoder<'_, '_>) -> Result<RunAuthority, DecodeError> {
    Ok(RunAuthority::from_wire_bytes(bytes32(decoder)?))
}

#[derive(Clone, Copy)]
enum TimeField {
    Seconds,
    Nanoseconds,
}

impl TimeField {
    fn from_text(name: Text<'_>) -> Result<Self, DecodeError> {
        if name.equals("seconds") {
            Ok(Self::Seconds)
        } else if name.equals("nanoseconds") {
            Ok(Self::Nanoseconds)
        } else {
            Err(field_error(DecodeCause::UnknownField))
        }
    }
}

#[derive(Default)]
struct TimeFields {
    seconds: Option<u64>,
    nanoseconds: Option<u32>,
}

struct TimeParts {
    seconds: u64,
    nanoseconds: u32,
}

fn time_parts(decoder: &mut Decoder<'_, '_>) -> Result<TimeParts, DecodeError> {
    let mut object = decoder.begin_object()?;
    let mut fields = TimeFields::default();
    while let Some(name) = decoder.next_field(&mut object)? {
        match TimeField::from_text(name)? {
            TimeField::Seconds => {
                if fields.seconds.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                fields.seconds = Some(decoder.unsigned()?);
            }
            TimeField::Nanoseconds => {
                if fields.nanoseconds.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                fields.nanoseconds = Some(u32::try_from(decoder.unsigned()?).map_err(|_| {
                    DecodeError::new(DecodeSite::Unsigned, DecodeCause::IntegerOverflow)
                })?);
            }
        }
    }
    Ok(TimeParts {
        seconds: missing(fields.seconds)?,
        nanoseconds: missing(fields.nanoseconds)?,
    })
}

/// Consume raw component widths only; the owning clock check decides validity and freshness.
pub(super) fn monotonic(decoder: &mut Decoder<'_, '_>) -> Result<MonotonicInstant, DecodeError> {
    let parts = time_parts(decoder)?;
    Ok(MonotonicInstant::from_wire_parts(
        parts.seconds,
        parts.nanoseconds,
    ))
}

/// Consume raw deadline components without creating or resetting any clock/window.
pub(super) fn deadline(decoder: &mut Decoder<'_, '_>) -> Result<RoleDeadline, DecodeError> {
    let parts = time_parts(decoder)?;
    Ok(RoleDeadline::from_wire_parts(
        parts.seconds,
        parts.nanoseconds,
    ))
}

#[derive(Clone, Copy)]
enum StopField {
    Origin,
    Instant,
}

impl StopField {
    fn from_text(name: Text<'_>) -> Result<Self, DecodeError> {
        if name.equals("origin") {
            Ok(Self::Origin)
        } else if name.equals("instant") {
            Ok(Self::Instant)
        } else {
            Err(field_error(DecodeCause::UnknownField))
        }
    }
}

#[derive(Default)]
struct StopFields {
    origin: Option<StopOrigin>,
    instant: Option<MonotonicInstant>,
}

/// Consume one raw stamp; origin and instant remain claims requiring the owner's actual checks.
pub(super) fn stop(decoder: &mut Decoder<'_, '_>) -> Result<StopStamp, DecodeError> {
    let mut object = decoder.begin_object()?;
    let mut fields = StopFields::default();
    while let Some(name) = decoder.next_field(&mut object)? {
        match StopField::from_text(name)? {
            StopField::Origin => {
                if fields.origin.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                fields.origin = Some(
                    StopOrigin::metadata_text(decoder.string()?)
                        .ok_or_else(|| field_error(DecodeCause::InvalidValue))?,
                );
            }
            StopField::Instant => {
                if fields.instant.is_some() {
                    return Err(field_error(DecodeCause::DuplicateField));
                }
                fields.instant = Some(monotonic(decoder)?);
            }
        }
    }
    Ok(StopStamp::from_wire_parts(
        missing(fields.origin)?,
        missing(fields.instant)?,
    ))
}

/// Consume one existing operation label; no role/site cross-product permission is inferred.
pub(super) fn operation(decoder: &mut Decoder<'_, '_>) -> Result<CauseOperation, DecodeError> {
    CauseOperation::metadata_text(decoder.string()?)
        .ok_or_else(|| field_error(DecodeCause::InvalidValue))
}

/// Consume one existing predicate label; this does not establish that the predicate occurred.
pub(super) fn predicate(
    decoder: &mut Decoder<'_, '_>,
) -> Result<CauseIntegrityPredicate, DecodeError> {
    CauseIntegrityPredicate::metadata_text(decoder.string()?)
        .ok_or_else(|| field_error(DecodeCause::InvalidValue))
}

/// Actual fixed schema delta beyond the already charged primitive records.
///
/// Call depth is stop -> monotonic -> time_parts, or identity -> bytes32; no payload recursion.
/// Add one ObjectState and one Text beyond the primitive's existing ones. Its existing ArrayState
/// covers identity's sequential array. Actual accumulator/result/selector/array/index terms and
/// this function's fixed sum table are included; no Scratch/Decoder/frame/payload duplication.
/// Main charges enclosing decoded records separately; layout sums do not prove native-stack size.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<ObjectState>(),
        size_of::<Text<'static>>(),
        size_of::<IdentityFields>(),
        size_of::<TimeFields>(),
        size_of::<TimeParts>(),
        size_of::<StopFields>(),
        size_of::<IdentityField>(),
        size_of::<TimeField>(),
        size_of::<StopField>(),
        size_of::<[u8; 32]>(),
        size_of::<usize>(),
        size_of::<u32>(),
        size_of::<BuildIdentity>(),
        size_of::<RunAuthority>(),
        size_of::<MonotonicInstant>(),
        size_of::<RoleDeadline>(),
        size_of::<StopStamp>(),
        size_of::<CauseOperation>(),
        size_of::<CauseIntegrityPredicate>(),
        size_of::<Result<TimeParts, DecodeError>>(),
        size_of::<Result<MonotonicInstant, DecodeError>>(),
        size_of::<Result<StopStamp, DecodeError>>(),
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
    // These assert scalar schema grammar only, not clock/authentication/whole-role acceptance.
    use super::super::{guardian_decode::Scratch, role_deadline::DeadlineError};
    use super::*;
    use serde::Serialize;

    fn parse<T>(
        bytes: &[u8],
        read: fn(&mut Decoder<'_, '_>) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError> {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(bytes, &mut scratch)?;
        let value = read(&mut decoder)?;
        decoder.finish()?;
        Ok(value)
    }

    fn roundtrip<T: Serialize + PartialEq + std::fmt::Debug>(
        value: T,
        read: fn(&mut Decoder<'_, '_>) -> Result<T, DecodeError>,
    ) {
        assert_eq!(
            parse(&serde_json::to_vec(&value).unwrap(), read).unwrap(),
            value
        );
    }

    fn record_adverses<T: Serialize + std::fmt::Debug>(
        value: T,
        read: fn(&mut Decoder<'_, '_>) -> Result<T, DecodeError>,
    ) {
        let original = serde_json::to_value(value).unwrap();
        let fields = original
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        for field in fields {
            let mut missing = original.clone();
            missing.as_object_mut().unwrap().remove(&field);
            assert_eq!(
                parse(&serde_json::to_vec(&missing).unwrap(), read)
                    .unwrap_err()
                    .cause(),
                DecodeCause::MissingField
            );
            let mut null = original.clone();
            null[&field] = serde_json::Value::Null;
            assert!(
                parse(&serde_json::to_vec(&null).unwrap(), read).is_err(),
                "null field admitted"
            );
        }
        let mut unknown = original.clone();
        unknown["extra"] = serde_json::Value::Null;
        assert_eq!(
            parse(&serde_json::to_vec(&unknown).unwrap(), read)
                .unwrap_err()
                .cause(),
            DecodeCause::UnknownField
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn actual_scalar_dto_serializer_roundtrips_preserve_unverified_facts() {
        let build = BuildIdentity {
            artifact: std::array::from_fn(|index| u8::try_from(index).unwrap()),
            protocol: GuardianProtocol::PrivateBoundedLease,
            lifecycle: GuardianLifecycle::NamespaceInitTypedDispatchLeaseEof,
        };
        roundtrip(build, identity);
        roundtrip(RunAuthority::from_wire_bytes(build.artifact), authority);
        roundtrip(
            MonotonicInstant::from_wire_parts(u64::MAX, u32::MAX),
            monotonic,
        );
        roundtrip(RoleDeadline::from_wire_parts(u64::MAX, u32::MAX), deadline);
        roundtrip(
            StopStamp::from_wire_parts(
                StopOrigin::Backend,
                MonotonicInstant::from_wire_parts(u64::MAX, u32::MAX),
            ),
            stop,
        );
        roundtrip(CauseOperation::ControlReception, operation);
        roundtrip(CauseIntegrityPredicate::MalformedCauseMetadata, predicate);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn every_record_field_is_required_nonnull_and_closed() {
        let build = BuildIdentity {
            artifact: [0; 32],
            protocol: GuardianProtocol::PrivateBoundedLease,
            lifecycle: GuardianLifecycle::NamespaceInitTypedDispatchLeaseEof,
        };
        record_adverses(build, identity);
        record_adverses(MonotonicInstant::from_wire_parts(9, 7), monotonic);
        record_adverses(RoleDeadline::from_wire_parts(9, 7), deadline);
        record_adverses(
            StopStamp::from_wire_parts(StopOrigin::Inner, MonotonicInstant::from_wire_parts(9, 7)),
            stop,
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn exact_byte_arrays_refuse_short_extra_nonbyte_and_width_overflow() {
        for array in [vec![0_u64; 31], vec![0; 33]] {
            assert_eq!(
                parse(&serde_json::to_vec(&array).unwrap(), authority)
                    .unwrap_err()
                    .cause(),
                DecodeCause::InvalidValue
            );
        }
        let mut overflow = vec![0_u64; 32];
        overflow[31] = 256;
        assert_eq!(
            parse(&serde_json::to_vec(&overflow).unwrap(), authority)
                .unwrap_err()
                .cause(),
            DecodeCause::IntegerOverflow
        );
        for byte in [
            serde_json::json!(-1),
            serde_json::json!(1.0),
            serde_json::json!(null),
            serde_json::json!("0"),
        ] {
            // Construct only the actual local DTO adverse array, with no source-schema copy.
            let mut array = serde_json::to_value([0_u8; 32]).unwrap();
            array[0] = byte;
            assert!(
                parse(&serde_json::to_vec(&array).unwrap(), authority).is_err(),
                "non-byte array element admitted"
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn clock_components_check_width_but_do_not_claim_clock_validity() {
        let raw = br#"{"seconds":18446744073709551615,"nanoseconds":4294967295}"#;
        assert_eq!(
            parse(raw, monotonic).unwrap(),
            MonotonicInstant::from_wire_parts(u64::MAX, u32::MAX)
        );
        let parsed = parse(raw, deadline).unwrap();
        assert_eq!(parsed, RoleDeadline::from_wire_parts(u64::MAX, u32::MAX));
        // This existing owner's pure bound comparison rejects invalid nanos before clock capture.
        assert_eq!(
            parsed.no_later_than(parsed),
            Err(DeadlineError::InvalidClock)
        );
        for bytes in [
            br#"{"seconds":18446744073709551616,"nanoseconds":0}"#.as_slice(),
            br#"{"seconds":0,"nanoseconds":4294967296}"#,
        ] {
            assert_eq!(
                parse(bytes, monotonic).unwrap_err().cause(),
                DecodeCause::IntegerOverflow
            );
        }
        for bytes in [
            br#"{"seconds":-1,"nanoseconds":0}"#.as_slice(),
            br#"{"seconds":0,"nanoseconds":1.5}"#,
            br#"{"seconds":1e0,"nanoseconds":0}"#,
        ] {
            assert_eq!(
                parse(bytes, deadline).unwrap_err().cause(),
                DecodeCause::InvalidNumber
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn duplicate_record_fields_refuse_before_the_duplicate_value_is_read() {
        for bytes in [
            br#"{"seconds":1,"seconds":null,"nanoseconds":0}"#.as_slice(),
            br#"{"seconds":1,"nanoseconds":0,"nanoseconds":null}"#,
        ] {
            assert_eq!(
                parse(bytes, monotonic).unwrap_err().cause(),
                DecodeCause::DuplicateField
            );
        }
        for bytes in [
            br#"{"origin":"Inner","origin":null,"instant":{"seconds":1,"nanoseconds":0}}"#
                .as_slice(),
            br#"{"origin":"Inner","instant":{"seconds":1,"nanoseconds":0},"instant":null}"#,
        ] {
            assert_eq!(
                parse(bytes, stop).unwrap_err().cause(),
                DecodeCause::DuplicateField
            );
        }
        let build = BuildIdentity {
            artifact: [0; 32],
            protocol: GuardianProtocol::PrivateBoundedLease,
            lifecycle: GuardianLifecycle::NamespaceInitTypedDispatchLeaseEof,
        };
        let mut encoded = serde_json::to_vec(&build).unwrap();
        assert_eq!(encoded.pop(), Some(b'}'));
        for field in ["artifact", "protocol", "lifecycle"] {
            let mut duplicate = encoded.clone();
            duplicate.extend_from_slice(format!(r#",{field:?}:null}}"#).as_bytes());
            assert_eq!(
                parse(&duplicate, identity).unwrap_err().cause(),
                DecodeCause::DuplicateField
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn labels_use_owning_metadata_and_accept_escaped_known_spelling() {
        assert_eq!(
            parse(br#""Control\u0052eception""#, operation).unwrap(),
            CauseOperation::ControlReception
        );
        assert_eq!(
            parse(br#""MalformedCause\u004detadata""#, predicate).unwrap(),
            CauseIntegrityPredicate::MalformedCauseMetadata
        );
        assert_eq!(
            parse(br#""unknown""#, operation).unwrap_err().cause(),
            DecodeCause::InvalidValue
        );
        assert_eq!(
            parse(br#""unknown""#, predicate).unwrap_err().cause(),
            DecodeCause::InvalidValue
        );
        let bytes = br#"{"origin":"\u0049nner","instant":{"seconds":1,"nanoseconds":0}}"#;
        assert_eq!(
            parse(bytes, stop).unwrap(),
            StopStamp::from_wire_parts(StopOrigin::Inner, MonotonicInstant::from_wire_parts(1, 0))
        );
        let build = BuildIdentity {
            artifact: [0; 32],
            protocol: GuardianProtocol::PrivateBoundedLease,
            lifecycle: GuardianLifecycle::NamespaceInitTypedDispatchLeaseEof,
        };
        let encoded = serde_json::to_string(&build)
            .unwrap()
            .replace("PrivateBoundedLease", r"PrivateBounded\u004cease")
            .replace(
                "NamespaceInitTypedDispatchLeaseEof",
                r"NamespaceInitTypedDispatchLease\u0045of",
            );
        assert_eq!(parse(encoded.as_bytes(), identity).unwrap(), build);
        let mut bad = serde_json::to_value(build).unwrap();
        for field in ["protocol", "lifecycle"] {
            bad[field] = serde_json::json!("unknown");
            assert_eq!(
                parse(&serde_json::to_vec(&bad).unwrap(), identity)
                    .unwrap_err()
                    .cause(),
                DecodeCause::InvalidValue
            );
            bad = serde_json::to_value(build).unwrap();
        }
        assert_eq!(
            parse(
                br#"{"origin":"unknown","instant":{"seconds":1,"nanoseconds":0}}"#,
                stop
            )
            .unwrap_err()
            .cause(),
            DecodeCause::InvalidValue
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn containing_cursor_is_retained_and_trailing_or_partial_records_refuse() {
        let mut scratch = Scratch::default();
        let mut decoder =
            Decoder::new(br#"[{"seconds":1,"nanoseconds":0},7]"#, &mut scratch).unwrap();
        let mut array = decoder.begin_array().unwrap();
        assert!(decoder.next_element(&mut array).unwrap());
        assert_eq!(
            monotonic(&mut decoder).unwrap(),
            MonotonicInstant::from_wire_parts(1, 0)
        );
        assert!(decoder.next_element(&mut array).unwrap());
        assert_eq!(decoder.byte().unwrap(), 7);
        assert!(!decoder.next_element(&mut array).unwrap());
        decoder.finish().unwrap();
        assert_eq!(
            parse(br#"{"seconds":1,"nanoseconds":0} true"#, monotonic)
                .unwrap_err()
                .cause(),
            DecodeCause::TrailingBytes
        );
        assert_eq!(
            parse(br#"{"seconds":1,"nanoseconds":0"#, monotonic)
                .unwrap_err()
                .cause(),
            DecodeCause::UnexpectedEnd
        );
    }
}

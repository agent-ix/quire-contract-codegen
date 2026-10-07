//! Strict required-cause seeds with fixed first-fault custody.
//!
//! The whole-envelope owner supplies the same deserializer, lexical preflight, fault-slot reset
//! and final EOF checks. Parsed facts grant no producer/site/state or projection authority.
//! This module neither renders optional diagnostic context nor reconstructs an original error.

use std::{fmt, mem::size_of, os::raw::c_long};

use serde::{de, de::DeserializeSeed};

use super::{
    control::ControlError,
    cross_role_cause::CauseIntegrityPredicate,
    startup_cause::{
        StartupCause, StartupIoCause, StartupIoKind, StartupPayload, StartupSeccompilerCause,
    },
};

fn record(slot: &mut Option<CauseIntegrityPredicate>, predicate: CauseIntegrityPredicate) {
    if slot.is_none() {
        *slot = Some(predicate);
    }
}

fn refused<E: de::Error>(
    slot: &mut Option<CauseIntegrityPredicate>,
    predicate: CauseIntegrityPredicate,
) -> E {
    record(slot, predicate);
    E::custom("required cause metadata refused")
}

/// Borrow the owner's first-fault slot; no deserializer, buffer or metadata authority is owned.
pub(super) struct CauseSeed<'a> {
    fault: &'a mut Option<CauseIntegrityPredicate>,
}

impl<'a> CauseSeed<'a> {
    pub(super) fn new(fault: &'a mut Option<CauseIntegrityPredicate>) -> Self {
        Self { fault }
    }
}

impl<'de> DeserializeSeed<'de> for CauseSeed<'_> {
    type Value = StartupCause;

    fn deserialize<D: de::Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        let fault = self.fault;
        decoder
            .deserialize_map(CauseVisitor { fault: &mut *fault })
            .map_err(|error| {
                record(fault, CauseIntegrityPredicate::MalformedCauseMetadata);
                error
            })
    }
}

struct CauseVisitor<'a> {
    fault: &'a mut Option<CauseIntegrityPredicate>,
}

impl<'de> de::Visitor<'de> for CauseVisitor<'_> {
    type Value = StartupCause;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("one required external cause tag")
    }

    fn visit_str<E: de::Error>(self, _value: &str) -> Result<Self::Value, E> {
        Err(refused(
            self.fault,
            CauseIntegrityPredicate::MalformedCauseMetadata,
        ))
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let tag = map
            .next_key_seed(TokenSeed)?
            .ok_or_else(|| refused(self.fault, CauseIntegrityPredicate::IncompleteCauseMetadata))?;
        let cause = match tag {
            "Io" => StartupCause::Io(map.next_value_seed(IoSeed {
                fault: &mut *self.fault,
            })?),
            "Seccompiler" => StartupCause::Seccompiler(map.next_value_seed(InstallationSeed {
                fault: &mut *self.fault,
            })?),
            _ => {
                return Err(refused(
                    self.fault,
                    CauseIntegrityPredicate::MalformedCauseMetadata,
                ))
            }
        };
        if map.next_key_seed(TokenSeed)?.is_some() {
            return Err(refused(
                self.fault,
                CauseIntegrityPredicate::MalformedCauseMetadata,
            ));
        }
        Ok(cause)
    }
}

// Whole-envelope preflight excludes escaped strings; unescaped JSON tokens can remain borrowed.
// Unknown tokens never become an owned diagnostic String in this helper's refusal messages.
struct TokenSeed;

impl<'de> DeserializeSeed<'de> for TokenSeed {
    type Value = &'de str;

    fn deserialize<D: de::Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_str(self)
    }
}

impl<'de> de::Visitor<'de> for TokenSeed {
    type Value = &'de str;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("borrowed required metadata token")
    }

    fn visit_borrowed_str<E: de::Error>(self, value: &'de str) -> Result<Self::Value, E> {
        Ok(value)
    }

    fn visit_str<E: de::Error>(self, _value: &str) -> Result<Self::Value, E> {
        Err(E::custom("required metadata token is not borrowed"))
    }
}

struct IoSeed<'a> {
    fault: &'a mut Option<CauseIntegrityPredicate>,
}

#[derive(Default)]
struct IoFields {
    kind: Option<StartupIoKind>,
    raw_os_error: Option<Option<i32>>,
    payload: Option<StartupPayload>,
}

impl<'de> DeserializeSeed<'de> for IoSeed<'_> {
    type Value = StartupIoCause;

    fn deserialize<D: de::Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_map(self)
    }
}

impl<'de> de::Visitor<'de> for IoSeed<'_> {
    type Value = StartupIoCause;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("required I/O cause members")
    }

    fn visit_str<E: de::Error>(self, _value: &str) -> Result<Self::Value, E> {
        Err(refused(
            self.fault,
            CauseIntegrityPredicate::MalformedCauseMetadata,
        ))
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut fields = IoFields::default();
        while let Some(field) = map.next_key_seed(TokenSeed)? {
            match field {
                "kind" => {
                    if fields.kind.is_some() {
                        return Err(refused(
                            self.fault,
                            CauseIntegrityPredicate::MalformedCauseMetadata,
                        ));
                    }
                    let name = map.next_value_seed(TokenSeed)?;
                    fields.kind = Some(StartupIoKind::metadata_name(name).ok_or_else(|| {
                        refused(self.fault, CauseIntegrityPredicate::UnknownKindMetadata)
                    })?);
                }
                "raw_os_error" => {
                    if fields.raw_os_error.is_some() {
                        return Err(refused(
                            self.fault,
                            CauseIntegrityPredicate::MalformedCauseMetadata,
                        ));
                    }
                    fields.raw_os_error = Some(map.next_value::<Option<i32>>()?);
                }
                "payload" => {
                    if fields.payload.is_some() {
                        return Err(refused(
                            self.fault,
                            CauseIntegrityPredicate::MalformedCauseMetadata,
                        ));
                    }
                    let name = map.next_value_seed(TokenSeed)?;
                    fields.payload =
                        Some(StartupPayload::metadata_name(name).ok_or_else(|| {
                            refused(self.fault, CauseIntegrityPredicate::MalformedCauseMetadata)
                        })?);
                }
                _ => {
                    return Err(refused(
                        self.fault,
                        CauseIntegrityPredicate::MalformedCauseMetadata,
                    ))
                }
            }
        }
        let kind = fields
            .kind
            .ok_or_else(|| refused(self.fault, CauseIntegrityPredicate::IncompleteCauseMetadata))?;
        let raw_os_error = fields
            .raw_os_error
            .ok_or_else(|| refused(self.fault, CauseIntegrityPredicate::IncompleteCauseMetadata))?;
        let payload = fields
            .payload
            .ok_or_else(|| refused(self.fault, CauseIntegrityPredicate::IncompleteCauseMetadata))?;
        Ok(StartupIoCause::from_metadata(kind, raw_os_error, payload))
    }
}

struct InstallationSeed<'a> {
    fault: &'a mut Option<CauseIntegrityPredicate>,
}

impl<'de> DeserializeSeed<'de> for InstallationSeed<'_> {
    type Value = StartupSeccompilerCause;

    fn deserialize<D: de::Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_any(self)
    }
}

impl<'de> de::Visitor<'de> for InstallationSeed<'_> {
    type Value = StartupSeccompilerCause;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("one existing installation cause")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "EmptyFilter" => Ok(StartupSeccompilerCause::EmptyFilter),
            _ => Err(refused(
                self.fault,
                CauseIntegrityPredicate::MalformedCauseMetadata,
            )),
        }
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let tag = map
            .next_key_seed(TokenSeed)?
            .ok_or_else(|| refused(self.fault, CauseIntegrityPredicate::IncompleteCauseMetadata))?;
        let cause = match tag {
            "Prctl" => StartupSeccompilerCause::Prctl(map.next_value_seed(IoSeed {
                fault: &mut *self.fault,
            })?),
            "Seccomp" => StartupSeccompilerCause::Seccomp(map.next_value_seed(IoSeed {
                fault: &mut *self.fault,
            })?),
            "ThreadSync" => StartupSeccompilerCause::ThreadSync {
                pid: map.next_value_seed(ThreadSyncSeed {
                    fault: &mut *self.fault,
                })?,
            },
            _ => {
                return Err(refused(
                    self.fault,
                    CauseIntegrityPredicate::MalformedCauseMetadata,
                ))
            }
        };
        if map.next_key_seed(TokenSeed)?.is_some() {
            return Err(refused(
                self.fault,
                CauseIntegrityPredicate::MalformedCauseMetadata,
            ));
        }
        Ok(cause)
    }
}

struct ThreadSyncSeed<'a> {
    fault: &'a mut Option<CauseIntegrityPredicate>,
}

#[derive(Default)]
struct ThreadSyncFields {
    pid: Option<c_long>,
}

impl<'de> DeserializeSeed<'de> for ThreadSyncSeed<'_> {
    type Value = c_long;

    fn deserialize<D: de::Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_map(self)
    }
}

impl<'de> de::Visitor<'de> for ThreadSyncSeed<'_> {
    type Value = c_long;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("required native thread synchronization pid")
    }

    fn visit_str<E: de::Error>(self, _value: &str) -> Result<Self::Value, E> {
        Err(refused(
            self.fault,
            CauseIntegrityPredicate::MalformedCauseMetadata,
        ))
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut fields = ThreadSyncFields::default();
        while let Some(field) = map.next_key_seed(TokenSeed)? {
            if field != "pid" || fields.pid.is_some() {
                return Err(refused(
                    self.fault,
                    CauseIntegrityPredicate::MalformedCauseMetadata,
                ));
            }
            fields.pid = Some(map.next_value::<c_long>()?);
        }
        fields
            .pid
            .ok_or_else(|| refused(self.fault, CauseIntegrityPredicate::IncompleteCauseMetadata))
    }
}

/// Fixed seed/accumulator/result/fault storage only, excluding existing decoder/frame/context.
/// A returned serde error may own an incidental message allocation. Its capacity is inaccessible
/// here; this reservation proves no dynamic error capacity bound or whole-runtime stack bound.
pub(super) fn decode_bytes() -> Result<u64, ControlError> {
    let bytes = size_of::<CauseSeed<'_>>()
        .checked_add(size_of::<CauseVisitor<'_>>())
        .and_then(|bytes| bytes.checked_add(size_of::<IoSeed<'_>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<InstallationSeed<'_>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<ThreadSyncSeed<'_>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<TokenSeed>()))
        .and_then(|bytes| bytes.checked_add(size_of::<IoFields>()))
        .and_then(|bytes| bytes.checked_add(size_of::<ThreadSyncFields>()))
        .and_then(|bytes| bytes.checked_add(size_of::<StartupCause>()))
        .and_then(|bytes| bytes.checked_add(size_of::<StartupIoCause>()))
        .and_then(|bytes| bytes.checked_add(size_of::<StartupSeccompilerCause>()))
        .and_then(|bytes| bytes.checked_add(size_of::<Option<CauseIntegrityPredicate>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<Option<&str>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<&str>()))
        .and_then(|bytes| bytes.checked_add(size_of::<Option<i32>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<c_long>()))
        .ok_or(ControlError::EncodedBytesExceeded)?;
    u64::try_from(bytes).map_err(|_| ControlError::EncodedBytesExceeded)
}

#[cfg(test)]
mod tests {
    // Pure required-schema/first-fault units: no AC40, role/auth, settlement or producer coverage.
    use super::*;

    fn parse(
        bytes: &[u8],
        fault: &mut Option<CauseIntegrityPredicate>,
    ) -> Result<StartupCause, serde_json::Error> {
        let mut decoder = serde_json::Deserializer::from_slice(bytes);
        let cause = CauseSeed::new(fault).deserialize(&mut decoder)?;
        decoder.end()?;
        Ok(cause)
    }

    fn refuses(bytes: &[u8], predicate: CauseIntegrityPredicate) {
        let mut fault = None;
        assert!(parse(bytes, &mut fault).is_err());
        assert_eq!(fault, Some(predicate));
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn existing_serializer_io_and_installation_shapes_roundtrip_with_explicit_errno_absence() {
        let io = StartupIoCause::from_metadata(
            StartupIoKind::InvalidData,
            None,
            StartupPayload::NoCustomPayload,
        );
        let os = StartupIoCause::from_metadata(
            StartupIoKind::PermissionDenied,
            Some(1),
            StartupPayload::NoCustomPayload,
        );
        for cause in [
            StartupCause::Io(io),
            StartupCause::Io(os),
            StartupCause::Seccompiler(StartupSeccompilerCause::EmptyFilter),
            StartupCause::Seccompiler(StartupSeccompilerCause::Prctl(io)),
            StartupCause::Seccompiler(StartupSeccompilerCause::Seccomp(os)),
            StartupCause::Seccompiler(StartupSeccompilerCause::ThreadSync { pid: -7 }),
        ] {
            let bytes = serde_json::to_vec(&cause).unwrap();
            let mut fault = None;
            assert_eq!(parse(&bytes, &mut fault).unwrap(), cause);
            assert_eq!(fault, None);
        }
        let os_derived = StartupCause::Io(StartupIoCause::from_metadata(
            StartupIoKind::OsDerived,
            None,
            StartupPayload::NoCustomPayload,
        ));
        let mut fault = None;
        assert_eq!(
            parse(&serde_json::to_vec(&os_derived).unwrap(), &mut fault).unwrap(),
            os_derived
        );
        assert_eq!(fault, None); // Existing projector, not schema decoding, owns MissingOsCode.
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn unknown_kind_keeps_first_specific_receiver_fault_without_normalizing_an_original() {
        let bytes = br#"{"Io":{"kind":"unknown","raw_os_error":null,"payload":"NoCustomPayload"}}"#;
        refuses(bytes, CauseIntegrityPredicate::UnknownKindMetadata);
        let mut fault = None;
        let mut decoder = serde_json::Deserializer::from_slice(bytes);
        let error = CauseSeed::new(&mut fault)
            .deserialize(&mut decoder)
            .unwrap_err();
        assert!(error.is_data());
        assert_eq!(fault, Some(CauseIntegrityPredicate::UnknownKindMetadata));
        let mut fault = Some(CauseIntegrityPredicate::UnknownKindMetadata);
        assert!(parse(br#"{"Io":null}"#, &mut fault).is_err());
        assert_eq!(fault, Some(CauseIntegrityPredicate::UnknownKindMetadata));
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn each_required_io_member_and_threadsync_pid_have_precise_missing_custody() {
        let original = serde_json::to_value(StartupCause::Io(StartupIoCause::from_metadata(
            StartupIoKind::Other,
            None,
            StartupPayload::UnrepresentedCustom,
        )))
        .unwrap();
        for field in ["kind", "raw_os_error", "payload"] {
            let mut missing = original.clone();
            missing["Io"].as_object_mut().unwrap().remove(field);
            refuses(
                &serde_json::to_vec(&missing).unwrap(),
                CauseIntegrityPredicate::IncompleteCauseMetadata,
            );
        }
        for bytes in [
            &b"{}"[..],
            &br#"{"Seccompiler":{}}"#[..],
            &br#"{"Seccompiler":{"ThreadSync":{}}}"#[..],
        ] {
            refuses(bytes, CauseIntegrityPredicate::IncompleteCauseMetadata);
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn malformed_types_widths_payload_fields_and_external_tags_are_not_missing_or_unknown_kind() {
        for bytes in [
            &b"null"[..],
            &b"[]"[..],
            &br#"{"Io":null}"#[..],
            &br#"{"Io":{"kind":null,"raw_os_error":null,"payload":"NoCustomPayload"}}"#[..],
            &br#"{"Io":{"kind":1,"raw_os_error":null,"payload":"NoCustomPayload"}}"#[..],
            &br#"{"Io":{"kind":"Other","raw_os_error":2147483648,"payload":"NoCustomPayload"}}"#[..],
            &br#"{"Io":{"kind":"Other","raw_os_error":"bad","payload":"NoCustomPayload"}}"#[..],
            &br#"{"Io":{"kind":"Other","raw_os_error":null,"payload":null}}"#[..],
            &br#"{"Io":{"kind":"Other","raw_os_error":null,"payload":"unknown"}}"#[..],
            &br#"{"Io":{"kind":"Other","kind":"Other","raw_os_error":null,"payload":"NoCustomPayload"}}"#[..],
            &br#"{"Io":{"kind":"Other","raw_os_error":null,"raw_os_error":null,"payload":"NoCustomPayload"}}"#[..],
            &br#"{"Io":{"kind":"Other","raw_os_error":null,"payload":"NoCustomPayload","payload":"NoCustomPayload"}}"#[..],
            &br#"{"Io":{"kind":"Other","raw_os_error":null,"payload":"NoCustomPayload","extra":0}}"#[..],
            &br#"{"Seccompiler":"unknown"}"#[..],
            &br#"{"Seccompiler":{"ThreadSync":{"pid":null}}}"#[..],
            &br#"{"Seccompiler":{"ThreadSync":{"pid":9223372036854775808}}}"#[..],
            &br#"{"Seccompiler":{"ThreadSync":{"pid":7,"pid":7}}}"#[..],
            &br#"{"Seccompiler":{"ThreadSync":{"pid":7,"extra":0}}}"#[..],
            &br#"{"Seccompiler":{"EmptyFilter":null}}"#[..],
            &br#"{"Seccompiler":{"ThreadSync":{"pid":7},"Prctl":{}}}"#[..],
            &br#"{"Seccompiler":"EmptyFilter","Io":{}}"#[..],
        ] {
            refuses(bytes, CauseIntegrityPredicate::MalformedCauseMetadata);
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn whole_owner_eof_check_refuses_trailing_input_without_relabeling_seed_success() {
        let mut fault = None;
        let error = parse(br#"{"Seccompiler":"EmptyFilter"}{}"#, &mut fault).unwrap_err();
        assert!(error.is_syntax());
        assert_eq!(fault, None); // Owner classifies actual framing/EOF, not this required seed.
    }
}

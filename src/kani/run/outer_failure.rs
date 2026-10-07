//! Sole bounded decoder for O's negative operational commit.
//!
//! Typed data supplies no sender, site, stop, measurement or settlement authority. The owner
//! selects this schema on its existing receive cursor and authenticates every retained fact.
//! Required byte structure remains strict even when optional diagnostic text is omitted.

use std::{fmt, mem::size_of};

use serde::{de, de::DeserializeSeed, de::Error as _, Deserialize, Serialize};

use super::{
    control::ControlError,
    cross_role_cause::{CauseIntegrityPredicate, CauseOperation},
    protocol::{BuildIdentity, RunAuthority},
    role_deadline::StopStamp,
    startup_cause::{
        check_scratch_free_json, PreparedStartupContext, StartupBytesSeed, StartupCause,
    },
};

/// Original replay and a required-representation fault are disjoint typed data.
/// The sender must supply an actual I/O cause; this decoder rejects the dependency domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) enum FailureRepresentation {
    Original { cause: StartupCause },
    Integrity { predicate: CauseIntegrityPredicate },
}

/// Fixed negative metadata; no measured peak, report descriptor or child-state proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct FailureHeader {
    pub(super) identity: BuildIdentity,
    pub(super) authority: RunAuthority,
    pub(super) stop: StopStamp,
    pub(super) operation: CauseOperation,
    pub(super) representation: FailureRepresentation,
}

impl FailureHeader {
    pub(super) fn rights_count(&self) -> usize {
        0
    }
}

/// Typed output uses the existing negative commit tag, without copying measured fields.
#[derive(Serialize)]
#[serde(tag = "kind")]
pub(super) enum NegativeCommit<'a> {
    Committed {
        identity: BuildIdentity,
        authority: RunAuthority,
        stop: StopStamp,
        disposition: NegativeDisposition<'a>,
    },
}

#[derive(Serialize)]
#[serde(tag = "kind")]
pub(super) enum NegativeDisposition<'a> {
    OperationalFailure {
        operation: CauseOperation,
        representation: FailureRepresentation,
        context: &'a [u8],
    },
}

impl<'a> NegativeCommit<'a> {
    /// Borrow only; selecting a genuine cause and authenticating its site remain owner duties.
    pub(super) fn new(header: FailureHeader, context: &'a [u8]) -> Self {
        Self::Committed {
            identity: header.identity,
            authority: header.authority,
            stop: header.stop,
            disposition: NegativeDisposition::OperationalFailure {
                operation: header.operation,
                representation: header.representation,
                context,
            },
        }
    }
}

#[derive(Deserialize)]
enum CommitKind {
    Committed,
}

#[derive(Deserialize)]
enum DispositionKind {
    OperationalFailure,
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum CommitField {
    Kind,
    Identity,
    Authority,
    Stop,
    Disposition,
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum DispositionField {
    Kind,
    Operation,
    Representation,
    Context,
}

#[derive(Default)]
struct CommitFields {
    kind: Option<CommitKind>,
    identity: Option<BuildIdentity>,
    authority: Option<RunAuthority>,
    stop: Option<StopStamp>,
    disposition: Option<DispositionMetadata>,
}

#[derive(Default)]
struct DispositionFields {
    kind: Option<DispositionKind>,
    operation: Option<CauseOperation>,
    representation: Option<FailureRepresentation>,
    context: bool,
}

struct DispositionMetadata {
    operation: CauseOperation,
    representation: FailureRepresentation,
}

struct CommitSeed<'a>(&'a mut PreparedStartupContext);
struct DispositionSeed<'a>(&'a mut PreparedStartupContext);

impl<'de> DeserializeSeed<'de> for CommitSeed<'_> {
    type Value = FailureHeader;

    fn deserialize<D: de::Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_map(self)
    }
}

impl<'de> de::Visitor<'de> for CommitSeed<'_> {
    type Value = FailureHeader;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("one exact negative operational commit")
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut fields = CommitFields::default();
        while let Some(field) = map.next_key::<CommitField>()? {
            match field {
                CommitField::Kind => {
                    if fields.kind.is_some() {
                        return Err(A::Error::duplicate_field("kind"));
                    }
                    fields.kind = Some(map.next_value()?);
                }
                CommitField::Identity => {
                    if fields.identity.is_some() {
                        return Err(A::Error::duplicate_field("identity"));
                    }
                    fields.identity = Some(map.next_value()?);
                }
                CommitField::Authority => {
                    if fields.authority.is_some() {
                        return Err(A::Error::duplicate_field("authority"));
                    }
                    fields.authority = Some(map.next_value()?);
                }
                CommitField::Stop => {
                    if fields.stop.is_some() {
                        return Err(A::Error::duplicate_field("stop"));
                    }
                    fields.stop = Some(map.next_value()?);
                }
                CommitField::Disposition => {
                    if fields.disposition.is_some() {
                        return Err(A::Error::duplicate_field("disposition"));
                    }
                    fields.disposition = Some(map.next_value_seed(DispositionSeed(self.0))?);
                }
            }
        }
        let CommitKind::Committed = fields.kind.ok_or_else(|| A::Error::missing_field("kind"))?;
        let disposition = fields
            .disposition
            .ok_or_else(|| A::Error::missing_field("disposition"))?;
        Ok(FailureHeader {
            identity: fields
                .identity
                .ok_or_else(|| A::Error::missing_field("identity"))?,
            authority: fields
                .authority
                .ok_or_else(|| A::Error::missing_field("authority"))?,
            stop: fields.stop.ok_or_else(|| A::Error::missing_field("stop"))?,
            operation: disposition.operation,
            representation: disposition.representation,
        })
    }
}

impl<'de> DeserializeSeed<'de> for DispositionSeed<'_> {
    type Value = DispositionMetadata;

    fn deserialize<D: de::Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        decoder.deserialize_map(self)
    }
}

impl<'de> de::Visitor<'de> for DispositionSeed<'_> {
    type Value = DispositionMetadata;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("one exact operational failure disposition")
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut fields = DispositionFields::default();
        while let Some(field) = map.next_key::<DispositionField>()? {
            match field {
                DispositionField::Kind => {
                    if fields.kind.is_some() {
                        return Err(A::Error::duplicate_field("kind"));
                    }
                    fields.kind = Some(map.next_value()?);
                }
                DispositionField::Operation => {
                    if fields.operation.is_some() {
                        return Err(A::Error::duplicate_field("operation"));
                    }
                    fields.operation = Some(map.next_value()?);
                }
                DispositionField::Representation => {
                    if fields.representation.is_some() {
                        return Err(A::Error::duplicate_field("representation"));
                    }
                    fields.representation = Some(map.next_value()?);
                }
                DispositionField::Context => {
                    if fields.context {
                        return Err(A::Error::duplicate_field("context"));
                    }
                    map.next_value_seed(self.0.bytes_seed())?;
                    fields.context = true;
                }
            }
        }
        let DispositionKind::OperationalFailure =
            fields.kind.ok_or_else(|| A::Error::missing_field("kind"))?;
        if !fields.context {
            return Err(A::Error::missing_field("context"));
        }
        let representation = fields
            .representation
            .ok_or_else(|| A::Error::missing_field("representation"))?;
        match representation {
            FailureRepresentation::Original {
                cause: StartupCause::Io(_),
            }
            | FailureRepresentation::Integrity { .. } => {}
            FailureRepresentation::Original {
                cause: StartupCause::Seccompiler(_),
            } => {
                return Err(A::Error::custom("operational original cause is not I/O"));
            }
        }
        Ok(DispositionMetadata {
            operation: fields
                .operation
                .ok_or_else(|| A::Error::missing_field("operation"))?,
            representation,
        })
    }
}

/// Decode only this negative schema, on borrowed bounded payload and retained context.
/// A successful parse authenticates no fact and changes no cursor or actor state.
pub(super) fn decode(
    payload: &[u8],
    context: &mut PreparedStartupContext,
) -> Result<FailureHeader, ControlError> {
    context.clear();
    check_scratch_free_json(payload).map_err(|error| {
        ControlError::InvalidEncoding(<serde_json::Error as de::Error>::custom(error))
    })?;
    let mut decoder = serde_json::Deserializer::from_slice(payload);
    let header = CommitSeed(context)
        .deserialize(&mut decoder)
        .map_err(ControlError::InvalidEncoding)?;
    decoder.end().map_err(ControlError::InvalidEncoding)?;
    Ok(header)
}

/// Additional fixed decoder objects only; retained frame/right/context allocations are separate.
/// Every term names actual schema/seed/result or scalar staging storage, with checked arithmetic.
pub(super) fn decode_bytes() -> Result<u64, ControlError> {
    let bytes = size_of::<CommitFields>()
        .checked_add(size_of::<DispositionFields>())
        .and_then(|bytes| bytes.checked_add(size_of::<CommitSeed<'_>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<DispositionSeed<'_>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<StartupBytesSeed<'_>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<FailureHeader>()))
        .and_then(|bytes| bytes.checked_add(size_of::<DispositionMetadata>()))
        .and_then(|bytes| bytes.checked_add(size_of::<FailureRepresentation>()))
        .and_then(|bytes| bytes.checked_add(size_of::<Option<CommitField>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<Option<DispositionField>>()))
        .and_then(|bytes| {
            bytes.checked_add(size_of::<
                serde_json::Deserializer<serde_json::de::SliceRead<'_>>,
            >())
        })
        .and_then(|bytes| bytes.checked_add(size_of::<[u8; 4]>()))
        .and_then(|bytes| bytes.checked_add(size_of::<usize>()))
        .and_then(|bytes| bytes.checked_add(size_of::<bool>()))
        .and_then(|bytes| bytes.checked_add(size_of::<Option<u8>>()))
        .and_then(|bytes| bytes.checked_add(size_of::<Option<u64>>()))
        .ok_or(ControlError::EncodedBytesExceeded)?;
    u64::try_from(bytes).map_err(|_| ControlError::EncodedBytesExceeded)
}

#[cfg(test)]
mod tests {
    // Schema/source units only: no authenticated site, role, outcome or settlement claim.
    use std::io;

    use super::*;
    use crate::kani::run::{
        protocol::current_build_identity, role_deadline::StopOrigin,
        startup_cause::StartupSeccompilerCause,
    };

    fn original() -> FailureHeader {
        let mut context = PreparedStartupContext::new(0).unwrap();
        FailureHeader {
            identity: current_build_identity(),
            authority: serde_json::from_value(serde_json::to_value([0_u8; 32]).unwrap()).unwrap(),
            stop: StopStamp::capture(StopOrigin::Outer).unwrap(),
            operation: CauseOperation::ProcSetup,
            representation: FailureRepresentation::Original {
                cause: context
                    .capture_io(&io::Error::from_raw_os_error(1))
                    .unwrap(),
            },
        }
    }

    fn value(header: FailureHeader) -> serde_json::Value {
        serde_json::to_value(NegativeCommit::new(header, &[])).unwrap()
    }

    fn refuses(value: &serde_json::Value) {
        let mut context = PreparedStartupContext::new(16).unwrap();
        assert!(decode(&serde_json::to_vec(value).unwrap(), &mut context).is_err());
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn negative_schema_retains_metadata_and_integrity_has_no_original_replay() {
        let original = original();
        let mut context = PreparedStartupContext::new(16).unwrap();
        let capacity = context.reserved_bytes();
        let bytes = serde_json::to_vec(&NegativeCommit::new(original, b"actual text")).unwrap();
        let decoded = decode(&bytes, &mut context).unwrap();
        assert_eq!(decoded, original);
        assert_eq!(decoded.rights_count(), 0);
        assert_eq!(context.context(), "actual text");
        assert_eq!(context.reserved_bytes(), capacity);

        let integrity = FailureHeader {
            representation: FailureRepresentation::Integrity {
                predicate: CauseIntegrityPredicate::RequiredRepresentationFormattingFailed,
            },
            ..original
        };
        let bytes = serde_json::to_vec(&NegativeCommit::new(integrity, &[])).unwrap();
        assert_eq!(decode(&bytes, &mut context).unwrap(), integrity);
        assert!(context.context().is_empty());
        // The same actual negative decoder must distinguish explicit no-errno from
        // omitted required metadata, rather than inherit serde's Option default.
        let mut absent_errno = value(original);
        absent_errno["disposition"]["representation"]["Original"]["cause"]["Io"]
            .as_object_mut()
            .unwrap()
            .remove("raw_os_error");
        refuses(&absent_errno);
        let payload_free =
            StartupCause::capture_io(&io::Error::from(io::ErrorKind::InvalidData)).unwrap();
        let no_errno = FailureHeader {
            representation: FailureRepresentation::Original {
                cause: payload_free,
            },
            ..original
        };
        let bytes = serde_json::to_vec(&NegativeCommit::new(no_errno, &[])).unwrap();
        assert_eq!(decode(&bytes, &mut context).unwrap(), no_errno);
        let mut conflicting = value(integrity);
        conflicting["disposition"]["representation"]["Integrity"]["cause"] =
            value(original)["disposition"]["representation"]["Original"]["cause"].clone();
        refuses(&conflicting);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn every_required_field_rejects_missing_null_wrong_type_and_duplicate() {
        let original = value(original());
        for nested in [false, true] {
            let fields: &[&str] = if nested {
                &["kind", "operation", "representation", "context"]
            } else {
                &["kind", "identity", "authority", "stop", "disposition"]
            };
            for field in fields {
                for replacement in [
                    None,
                    Some(serde_json::Value::Null),
                    Some(serde_json::json!(true)),
                ] {
                    let mut changed = original.clone();
                    let object = if nested {
                        &mut changed["disposition"]
                    } else {
                        &mut changed
                    };
                    let object = object.as_object_mut().unwrap();
                    match replacement {
                        Some(replacement) => {
                            object.insert((*field).into(), replacement);
                        }
                        None => {
                            object.remove(*field);
                        }
                    }
                    refuses(&changed);
                }
                let encoded = serde_json::to_string(&original).unwrap();
                let object = if nested {
                    &original["disposition"]
                } else {
                    &original
                };
                let member = format!("\"{field}\":{}", object[*field]);
                let duplicated = encoded.replacen(&member, &format!("{member},{member}"), 1);
                assert_ne!(duplicated, encoded);
                let mut context = PreparedStartupContext::new(16).unwrap();
                assert!(decode(duplicated.as_bytes(), &mut context).is_err());
            }
        }
        for nested in [false, true] {
            let mut changed = original.clone();
            let object = if nested {
                &mut changed["disposition"]
            } else {
                &mut changed
            };
            object
                .as_object_mut()
                .unwrap()
                .insert("unknown".into(), serde_json::json!(0));
            refuses(&changed);
        }
        let mut missing_payload = original;
        missing_payload["disposition"]["representation"]["Original"]["cause"]["Io"]
            .as_object_mut()
            .unwrap()
            .remove("payload");
        refuses(&missing_payload);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn original_and_integrity_representation_require_their_own_closed_metadata() {
        let original = value(original());
        for path in [
            "/disposition/representation/Original",
            "/disposition/representation/Original/cause/Io",
        ] {
            let fields: &[&str] = if path.ends_with("/Io") {
                &["kind", "payload"]
            } else {
                &["cause"]
            };
            for field in fields {
                for replacement in [
                    None,
                    Some(serde_json::Value::Null),
                    Some(serde_json::json!(true)),
                ] {
                    let mut changed = original.clone();
                    let object = changed.pointer_mut(path).unwrap().as_object_mut().unwrap();
                    match replacement {
                        Some(replacement) => {
                            object.insert((*field).into(), replacement);
                        }
                        None => {
                            object.remove(*field);
                        }
                    }
                    refuses(&changed);
                }
            }
        }
        for field in ["kind", "payload"] {
            let mut changed = original.clone();
            changed["disposition"]["representation"]["Original"]["cause"]["Io"][field] =
                serde_json::json!("unknown");
            refuses(&changed);
        }
        let header = FailureHeader {
            representation: FailureRepresentation::Integrity {
                predicate: CauseIntegrityPredicate::MalformedCauseMetadata,
            },
            ..self::original()
        };
        let integrity = value(header);
        for replacement in [
            None,
            Some(serde_json::Value::Null),
            Some(serde_json::json!("unknown")),
            Some(serde_json::json!(true)),
        ] {
            let mut changed = integrity.clone();
            let object = changed["disposition"]["representation"]["Integrity"]
                .as_object_mut()
                .unwrap();
            match replacement {
                Some(replacement) => {
                    object.insert("predicate".into(), replacement);
                }
                None => {
                    object.remove("predicate");
                }
            }
            refuses(&changed);
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn alternate_domains_extra_fields_and_incomplete_frames_cannot_be_negative_commits() {
        let original = value(original());
        for tag in [
            "Report",
            "Phase",
            "Cancelled",
            "Ready",
            "Dispatched",
            "unknown",
        ] {
            let mut changed = original.clone();
            changed["kind"] = serde_json::json!(tag);
            refuses(&changed);
            let mut changed = original.clone();
            changed["disposition"]["kind"] = serde_json::json!(tag);
            refuses(&changed);
        }
        for field in ["peaks", "report_bytes", "descriptor", "charged_peak"] {
            for nested in [false, true] {
                let mut changed = original.clone();
                let object = if nested {
                    &mut changed["disposition"]
                } else {
                    &mut changed
                };
                object
                    .as_object_mut()
                    .unwrap()
                    .insert(field.into(), serde_json::json!(0));
                refuses(&changed);
            }
        }
        let mut changed = original.clone();
        changed["disposition"]["representation"] =
            serde_json::to_value(FailureRepresentation::Original {
                cause: StartupCause::Seccompiler(StartupSeccompilerCause::EmptyFilter),
            })
            .unwrap();
        refuses(&changed);
        let encoded = serde_json::to_vec(&original).unwrap();
        let mut context = PreparedStartupContext::new(16).unwrap();
        assert!(decode(&encoded[..encoded.len() - 1], &mut context).is_err());
        let mut trailing = encoded;
        trailing.extend_from_slice(b"{}");
        assert!(decode(&trailing, &mut context).is_err());
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn optional_text_omission_still_consumes_strict_required_byte_structure() {
        let header = original();
        let mut context = PreparedStartupContext::new(1).unwrap();
        let capacity = context.reserved_bytes();
        for diagnostic in [&b"long"[..], &[255][..], &[0xc2][..]] {
            let bytes = serde_json::to_vec(&NegativeCommit::new(header, diagnostic)).unwrap();
            assert_eq!(decode(&bytes, &mut context).unwrap(), header);
            assert!(context.context().is_empty());
            assert_eq!(context.reserved_bytes(), capacity);
        }
        for diagnostic in [
            serde_json::json!([255, 256]),
            serde_json::json!([255, null]),
            serde_json::json!([255, "bad"]),
            serde_json::json!([255, -1]),
            serde_json::json!([255, 1.5]),
        ] {
            let mut changed = value(header);
            changed["disposition"]["context"] = diagnostic;
            refuses(&changed);
        }
    }
}

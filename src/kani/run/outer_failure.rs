//! Sole bounded decoder for O's negative operational commit.
//!
//! Typed data supplies no sender, site, stop, measurement or settlement authority. The owner
//! selects this schema on its existing receive cursor and authenticates every retained fact.
//! Required byte structure remains strict even when optional diagnostic text is omitted.

use serde::{Deserialize, Serialize};

use super::{
    control::ControlError,
    cross_role_cause::{CauseIntegrityPredicate, CauseOperation},
    protocol::{BuildIdentity, RunAuthority},
    role_deadline::{DeadlineError, StopStamp},
    role_protocol::RunSettings,
    startup_cause::{PreparedStartupContext, StartupCause},
};

/// Original replay and a required-representation fault are disjoint typed data.
/// The sender must supply an actual I/O cause; this decoder rejects the dependency domain.
macro_rules! failure_representations {
    ($($variant:ident { $field:ident: $value:ty }),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
        #[serde(deny_unknown_fields)]
        pub(super) enum FailureRepresentation {
            $($variant { $field: $value }),+
        }

        #[derive(Clone, Copy)]
        pub(super) enum FailureRepresentationTag {
            $($variant),+
        }

        impl FailureRepresentationTag {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}

failure_representations! {
    Original { cause: StartupCause },
    Integrity { predicate: CauseIntegrityPredicate },
}

/// Genuine producer-owned observation capability and original work-clock election. Neither
/// field supplies a current sample, a negative cause, positive Dispatch or settlement proof.
macro_rules! failurestate_record {
    ($($variant:ident => $visibility:vis $member:ident: $value:ty),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
        #[serde(deny_unknown_fields)]
        pub(super) struct FailureState { $($visibility $member: $value),+ }
        #[derive(Clone, Copy)]
        pub(super) enum FailureStateField { $($variant),+ }
        impl FailureStateField {
            pub(super) fn declared_order() -> &'static [Self] { &[$(Self::$variant),+] }
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($member)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}
failurestate_record! {
    ObservationAdmitted => pub(super) observation_admitted: bool,
    OriginalWorkExpired => pub(super) original_work_expired: bool,
}

impl FailureState {
    pub(super) fn capture(
        settings: &RunSettings,
        stop: StopStamp,
        observation_admitted: bool,
    ) -> Result<Self, DeadlineError> {
        Ok(Self {
            observation_admitted,
            original_work_expired: settings.work_deadline.expired_at(stop)?,
        })
    }
}

/// Fixed negative metadata; no measured peak, report descriptor or child-state proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct FailureHeader {
    pub(super) identity: BuildIdentity,
    pub(super) authority: RunAuthority,
    pub(super) stop: StopStamp,
    pub(super) operation: CauseOperation,
    pub(super) state: FailureState,
    pub(super) representation: FailureRepresentation,
}

impl FailureHeader {
    pub(super) fn rights_count(&self) -> usize {
        0
    }
}

/// Parsed constructor-work timeout facts, never an absence or settlement witness. Only O's
/// retained pre-observation constructor work producer may emit this distinct disposition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ConstructorTimeoutHeader {
    pub(super) identity: BuildIdentity,
    pub(super) authority: RunAuthority,
    pub(super) stop: StopStamp,
}

#[derive(Serialize)]
#[serde(tag = "kind")]
pub(super) enum ConstructorTimeoutCommit {
    Committed {
        identity: BuildIdentity,
        authority: RunAuthority,
        stop: StopStamp,
        disposition: ConstructorTimeoutDisposition,
    },
}

macro_rules! constructor_timeout_disposition {
    ($variant:ident) => {
        #[derive(Serialize)]
        #[serde(tag = "kind")]
        pub(super) enum ConstructorTimeoutDisposition {
            $variant,
        }
        impl ConstructorTimeoutDisposition {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                text.equals(stringify!($variant)).then_some(Self::$variant)
            }
        }
    };
}
constructor_timeout_disposition! { StartupTimeoutBeforeObservation }

impl ConstructorTimeoutHeader {
    pub(super) fn commit(self) -> ConstructorTimeoutCommit {
        ConstructorTimeoutCommit::Committed {
            identity: self.identity,
            authority: self.authority,
            stop: self.stop,
            disposition: ConstructorTimeoutDisposition::StartupTimeoutBeforeObservation,
        }
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
        state: FailureState,
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
                state: header.state,
                representation: header.representation,
                context,
            },
        }
    }
}

/// Decode only the negative schema with the caller's already retained fixed workspace.
/// Actor authentication, original clocks and positive settlement remain independent duties.
pub(super) fn decode(
    payload: &[u8],
    scratch: &mut super::guardian_decode::Scratch,
    context: &mut PreparedStartupContext,
) -> Result<FailureHeader, ControlError> {
    super::outer_failure_decode::decode(payload, scratch, context)
        .map_err(|source| context.grammar_error(source))
}

/// Actual fixed envelope schema delta; primitive/scalar/cause workspaces are charged separately.
pub(super) fn decode_bytes() -> Result<u64, ControlError> {
    super::outer_failure_decode::decode_bytes().map_err(ControlError::InvalidGrammar)
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

    fn parse(
        payload: &[u8],
        context: &mut PreparedStartupContext,
    ) -> Result<FailureHeader, ControlError> {
        decode(
            payload,
            &mut super::super::guardian_decode::Scratch::default(),
            context,
        )
    }

    fn original() -> FailureHeader {
        let mut context = PreparedStartupContext::new(0).unwrap();
        FailureHeader {
            identity: current_build_identity(),
            authority: serde_json::from_value(serde_json::to_value([0_u8; 32]).unwrap()).unwrap(),
            stop: StopStamp::capture(StopOrigin::Outer).unwrap(),
            operation: CauseOperation::ProcSetup,
            state: FailureState {
                observation_admitted: false,
                original_work_expired: false,
            },
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
        assert!(parse(&serde_json::to_vec(value).unwrap(), &mut context).is_err());
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn negative_schema_retains_metadata_and_integrity_has_no_original_replay() {
        let original = original();
        let mut context = PreparedStartupContext::new(16).unwrap();
        let capacity = context.reserved_bytes();
        let bytes = serde_json::to_vec(&NegativeCommit::new(original, b"actual text")).unwrap();
        let decoded = parse(&bytes, &mut context).unwrap();
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
        assert_eq!(parse(&bytes, &mut context).unwrap(), integrity);
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
        assert_eq!(parse(&bytes, &mut context).unwrap(), no_errno);
        let mut conflicting = value(integrity);
        conflicting["disposition"]["representation"]["Integrity"]["cause"] =
            value(original)["disposition"]["representation"]["Original"]["cause"].clone();
        refuses(&conflicting);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn actual_negative_decoder_distinguishes_required_kind_presence_and_type_faults() {
        let header = original();
        let mut unknown = value(header);
        unknown["disposition"]["representation"]["Original"]["cause"]["Io"]["kind"] =
            serde_json::json!("UnknownOriginalKind");
        let mut missing = value(header);
        missing["disposition"]["representation"]["Original"]["cause"]["Io"]
            .as_object_mut()
            .unwrap()
            .remove("raw_os_error");
        let mut malformed = value(header);
        malformed["disposition"]["representation"]["Original"]["cause"]["Io"]["payload"] =
            serde_json::Value::Null;
        let mut context = PreparedStartupContext::new(0).unwrap();
        let capacity = context.reserved_bytes();
        for (input, expected) in [
            (unknown, CauseIntegrityPredicate::UnknownKindMetadata),
            (missing, CauseIntegrityPredicate::IncompleteCauseMetadata),
            (malformed, CauseIntegrityPredicate::MalformedCauseMetadata),
        ] {
            let bytes = serde_json::to_vec(&input).unwrap();
            let error = parse(&bytes, &mut context).unwrap_err();
            let ControlError::CauseMetadataGrammar { predicate, source } = &error else {
                panic!("required cause fault lost its typed predicate")
            };
            assert_eq!(*predicate, expected);
            let expected_source = match expected {
                CauseIntegrityPredicate::UnknownKindMetadata => {
                    super::super::guardian_decode::DecodeCause::InvalidValue
                }
                CauseIntegrityPredicate::IncompleteCauseMetadata => {
                    super::super::guardian_decode::DecodeCause::MissingField
                }
                CauseIntegrityPredicate::MalformedCauseMetadata => {
                    super::super::guardian_decode::DecodeCause::UnexpectedToken
                }
                other => panic!("unexpected tested predicate: {other:?}"),
            };
            assert_eq!(source.cause(), expected_source);
            let actual_source = std::error::Error::source(&error)
                .unwrap()
                .downcast_ref::<super::super::guardian_decode::DecodeError>()
                .unwrap();
            assert!(std::ptr::eq(source, actual_source));
            assert_eq!(context.reserved_bytes(), capacity);
            // A following valid whole decode resets the fault; diagnostic omission alone
            // never authorizes a prior failed required cause or replays its predicate.
            let valid = serde_json::to_vec(&NegativeCommit::new(header, &[])).unwrap();
            assert_eq!(parse(&valid, &mut context).unwrap(), header);
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn every_required_field_rejects_missing_null_wrong_type_and_duplicate() {
        let original = value(original());
        for nested in [false, true] {
            let fields: &[&str] = if nested {
                &["kind", "operation", "state", "representation", "context"]
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
                assert!(parse(duplicated.as_bytes(), &mut context).is_err());
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
        assert!(parse(&encoded[..encoded.len() - 1], &mut context).is_err());
        let mut trailing = encoded;
        trailing.extend_from_slice(b"{}");
        assert!(parse(&trailing, &mut context).is_err());
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn optional_text_omission_still_consumes_strict_required_byte_structure() {
        let header = original();
        let mut context = PreparedStartupContext::new(1).unwrap();
        let capacity = context.reserved_bytes();
        for diagnostic in [&b"long"[..], &[255][..], &[0xc2][..]] {
            let bytes = serde_json::to_vec(&NegativeCommit::new(header, diagnostic)).unwrap();
            assert_eq!(parse(&bytes, &mut context).unwrap(), header);
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

#[cfg(test)]
mod state_tests {
    use super::*;

    /// Trace: FR-034-AC-15, FR-034-AC-38
    #[test]
    fn failure_milestone_and_work_election_are_required_typed_facts() {
        // Schema primitive only: genuine producer setup observation, authentication and
        // actual cleanup are integration obligations, not established by these booleans.
        let valid = serde_json::json!({
            "observation_admitted": true, "original_work_expired": false
        });
        let decoded: FailureState = serde_json::from_value(valid.clone()).unwrap();
        assert!(decoded.observation_admitted);
        assert!(!decoded.original_work_expired);
        for name in ["observation_admitted", "original_work_expired"] {
            for replacement in [
                None,
                Some(serde_json::Value::Null),
                Some(serde_json::json!(1)),
                Some(serde_json::json!("false")),
            ] {
                let mut changed = valid.clone();
                let fields = changed.as_object_mut().unwrap();
                if let Some(value) = replacement {
                    fields.insert(name.into(), value);
                } else {
                    fields.remove(name);
                }
                assert!(serde_json::from_value::<FailureState>(changed).is_err());
            }
        }
        let mut extra = valid;
        extra["peak"] = serde_json::json!(0);
        assert!(serde_json::from_value::<FailureState>(extra).is_err());
    }
}

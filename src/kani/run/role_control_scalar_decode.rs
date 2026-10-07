// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Raw role-control schemas on the owner's sole borrowed decoder (FR-034).
//!
//! These values establish no namespace identity, memory observation, policy-domain match,
//! authentication, deadline or outcome. Owning declarations supply enum labels; the envelope
//! owner retains the scratch, context, first-fault slot and complete-input responsibility.

use std::mem::{size_of, size_of_val};

use super::{
    cause_decode,
    cross_role_cause::CauseIntegrityPredicate,
    guardian_decode::{
        record, ArrayState, DecodeCause, DecodeError, DecodeSite, Decoder, ObjectState, Text,
        ValueKind,
    },
    outer_setup::{NamespaceField, NamespaceIdentity},
    resource_ledger::{MeasuredPeaks, PeakField},
    role_protocol::OwnerStopCause,
    startup_envelope::{PolicyFailureCause, PolicyFailureTag},
};

fn error(cause: DecodeCause) -> DecodeError {
    DecodeError::new(DecodeSite::Field, cause)
}

fn required<'input>(
    decoder: &mut Decoder<'input, '_>,
    object: &mut ObjectState,
) -> Result<Text<'input>, DecodeError> {
    decoder
        .next_field(object)?
        .ok_or_else(|| error(DecodeCause::MissingField))
}

fn close_single(
    decoder: &mut Decoder<'_, '_>,
    object: &mut ObjectState,
) -> Result<(), DecodeError> {
    if decoder.next_field(object)?.is_some() {
        return Err(error(DecodeCause::InvalidValue));
    }
    Ok(())
}

/// Consume exactly the existing policy schema, preserving the first required-metadata fault.
/// Unit strings and single unit-null maps are alternatives of the same derived enum grammar.
pub(super) fn policy(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
) -> Result<PolicyFailureCause, DecodeError> {
    let result = policy_value(decoder, fault);
    if let Err(source) = result {
        let predicate = match source.cause() {
            DecodeCause::MissingField | DecodeCause::UnexpectedEnd => {
                Some(CauseIntegrityPredicate::IncompleteCauseMetadata)
            }
            DecodeCause::InputBound | DecodeCause::StorageBound => None,
            DecodeCause::EmptyInput
            | DecodeCause::UnexpectedToken
            | DecodeCause::InvalidSeparator
            | DecodeCause::InvalidEscape
            | DecodeCause::InvalidUnicode
            | DecodeCause::InvalidUtf8
            | DecodeCause::InvalidNumber
            | DecodeCause::IntegerOverflow
            | DecodeCause::RecursionLimit
            | DecodeCause::TrailingBytes
            | DecodeCause::UnknownField
            | DecodeCause::DuplicateField
            | DecodeCause::InvalidValue => Some(CauseIntegrityPredicate::MalformedCauseMetadata),
        };
        if fault.is_none() {
            *fault = predicate;
        }
        return Err(source);
    }
    result
}

fn unit(decoder: &mut Decoder<'_, '_>, tagged_map: bool) -> Result<(), DecodeError> {
    if tagged_map {
        decoder.null()?;
    }
    Ok(())
}

fn policy_body(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
    tagged_map: bool,
) -> Result<super::startup_cause::StartupCause, DecodeError> {
    if !tagged_map {
        return Err(error(DecodeCause::InvalidValue));
    }
    if decoder.peek_kind()? == ValueKind::Array {
        let mut body = decoder.begin_array()?;
        if !decoder.next_element(&mut body)? {
            return Err(error(DecodeCause::MissingField));
        }
        let cause = cause_decode::decode_policy(decoder, fault)?;
        if decoder.next_element(&mut body)? {
            return Err(error(DecodeCause::InvalidValue));
        }
        return Ok(cause);
    }
    let mut body = decoder.begin_object()?;
    if !required(decoder, &mut body)?.equals("cause") {
        return Err(error(DecodeCause::UnknownField));
    }
    let cause = cause_decode::decode_policy(decoder, fault)?;
    close_single(decoder, &mut body)?;
    Ok(cause)
}

fn policy_value(
    decoder: &mut Decoder<'_, '_>,
    fault: &mut Option<CauseIntegrityPredicate>,
) -> Result<PolicyFailureCause, DecodeError> {
    let mut outer = match decoder.peek_kind()? {
        ValueKind::Object => Some(decoder.begin_object()?),
        ValueKind::String => None,
        ValueKind::Array | ValueKind::Number | ValueKind::Boolean | ValueKind::Null => {
            return Err(error(DecodeCause::UnexpectedToken));
        }
    };
    let name = match outer.as_mut() {
        Some(object) => required(decoder, object)?,
        None => decoder.string()?,
    };
    let tag =
        PolicyFailureTag::metadata_text(name).ok_or_else(|| error(DecodeCause::InvalidValue))?;
    let tagged_map = outer.is_some();
    let policy = match tag {
        PolicyFailureTag::UnsupportedArchitecture => {
            unit(decoder, tagged_map)?;
            PolicyFailureCause::UnsupportedArchitecture
        }
        PolicyFailureTag::InvalidProgram => {
            unit(decoder, tagged_map)?;
            PolicyFailureCause::InvalidProgram
        }
        PolicyFailureTag::NotBackend => {
            unit(decoder, tagged_map)?;
            PolicyFailureCause::NotBackend
        }
        PolicyFailureTag::ProtectionUnverified => {
            unit(decoder, tagged_map)?;
            PolicyFailureCause::ProtectionUnverified
        }
        PolicyFailureTag::Preparation => PolicyFailureCause::Preparation {
            cause: policy_body(decoder, fault, tagged_map)?,
        },
        PolicyFailureTag::Privilege => PolicyFailureCause::Privilege {
            cause: policy_body(decoder, fault, tagged_map)?,
        },
        PolicyFailureTag::Filter => PolicyFailureCause::Filter {
            cause: policy_body(decoder, fault, tagged_map)?,
        },
    };
    if let Some(object) = outer.as_mut() {
        close_single(decoder, object)?;
    }
    Ok(policy)
}

#[derive(Clone, Copy)]
enum PairField {
    First,
    Second,
}

#[derive(Default)]
struct PairFields {
    first: Option<u64>,
    second: Option<u64>,
}

fn pair<F: Copy>(
    decoder: &mut Decoder<'_, '_>,
    order: &[F],
    lookup: impl Fn(Text<'_>) -> Option<F>,
    select: impl Fn(F) -> PairField,
) -> Result<(u64, u64), DecodeError> {
    let mut fields = PairFields::default();
    record(decoder, order, lookup, |decoder, field| {
        let slot = match select(field) {
            PairField::First => &mut fields.first,
            PairField::Second => &mut fields.second,
        };
        if slot.is_some() {
            return Err(error(DecodeCause::DuplicateField));
        }
        *slot = Some(decoder.unsigned()?);
        Ok(())
    })?;
    Ok((
        fields
            .first
            .ok_or_else(|| error(DecodeCause::MissingField))?,
        fields
            .second
            .ok_or_else(|| error(DecodeCause::MissingField))?,
    ))
}

/// Consume raw namespace numbers, without observing a path or descriptor.
pub(super) fn namespace(decoder: &mut Decoder<'_, '_>) -> Result<NamespaceIdentity, DecodeError> {
    let (device, inode) = pair(
        decoder,
        NamespaceField::declared_order(),
        NamespaceField::metadata_text,
        |field| match field {
            NamespaceField::Device => PairField::First,
            NamespaceField::Inode => PairField::Second,
        },
    )?;
    Ok(NamespaceIdentity::from_wire_parts(device, inode))
}

/// Consume raw peak numbers; ordering, samples and ceiling claims remain owner obligations.
pub(super) fn peaks(decoder: &mut Decoder<'_, '_>) -> Result<MeasuredPeaks, DecodeError> {
    let (tree_rss_bytes, charged_bytes) = pair(
        decoder,
        PeakField::declared_order(),
        PeakField::metadata_text,
        |field| match field {
            PeakField::TreeRssBytes => PairField::First,
            PeakField::ChargedBytes => PairField::Second,
        },
    )?;
    Ok(MeasuredPeaks {
        tree_rss_bytes,
        charged_bytes,
    })
}

/// Consume the owning unit-enum grammar without electing a failure or timeout outcome.
pub(super) fn owner_stop(decoder: &mut Decoder<'_, '_>) -> Result<OwnerStopCause, DecodeError> {
    match decoder.peek_kind()? {
        ValueKind::String => OwnerStopCause::metadata_text(decoder.string()?)
            .ok_or_else(|| error(DecodeCause::InvalidValue)),
        ValueKind::Object => {
            let mut object = decoder.begin_object()?;
            let cause = OwnerStopCause::metadata_text(required(decoder, &mut object)?)
                .ok_or_else(|| error(DecodeCause::InvalidValue))?;
            decoder.null()?;
            close_single(decoder, &mut object)?;
            Ok(cause)
        }
        ValueKind::Array | ValueKind::Number | ValueKind::Boolean | ValueKind::Null => {
            Err(error(DecodeCause::UnexpectedToken))
        }
    }
}

/// Checked fixed records beyond separately charged primitive/cause/scalar workspaces.
/// The policy call graph has fixed depth, not payload-driven recursion. Actual compiler stack
/// highwater, initializer transients and record overlap are unmeasured, not sizeof guarantees.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<Option<ObjectState>>(), // Outer retained while the fixed policy body is live.
        size_of::<Text<'static>>(),
        size_of::<PolicyFailureTag>(),
        size_of::<Option<PolicyFailureTag>>(),
        size_of::<PolicyFailureCause>(),
        size_of::<Result<PolicyFailureCause, DecodeError>>(),
        size_of::<&mut Decoder<'static, 'static>>(),
        size_of::<&mut Option<CauseIntegrityPredicate>>(),
        size_of::<Option<CauseIntegrityPredicate>>(),
        size_of::<bool>(),
        size_of::<PairField>(),
        size_of::<PairFields>(),
        size_of::<ArrayState>(),
        size_of::<&[NamespaceField]>(),
        size_of::<&[PeakField]>(),
        size_of::<fn(Text<'_>) -> Result<PairField, DecodeError>>(),
        size_of::<&mut Option<u64>>(),
        size_of::<(u64, u64)>(),
        size_of::<NamespaceIdentity>(),
        size_of::<MeasuredPeaks>(),
        size_of::<OwnerStopCause>(),
        size_of::<Option<OwnerStopCause>>(),
        size_of::<Result<(u64, u64), DecodeError>>(),
        size_of::<Result<PairField, DecodeError>>(),
        size_of::<Result<NamespaceIdentity, DecodeError>>(),
        size_of::<Result<MeasuredPeaks, DecodeError>>(),
        size_of::<Result<OwnerStopCause, DecodeError>>(),
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
    // Local grammar units only; raw decoded values authorize no owner/policy/measurement claim.
    use std::io;

    use super::*;
    use crate::kani::run::{
        guardian_decode::Scratch,
        startup_cause::{StartupCause, StartupSeccompilerCause},
    };

    fn parse<T>(
        bytes: &[u8],
        read: fn(&mut Decoder<'_, '_>) -> Result<T, DecodeError>,
    ) -> Result<T, DecodeError> {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(bytes, &mut scratch)?;
        let parsed = read(&mut decoder)?;
        decoder.finish()?;
        Ok(parsed)
    }

    fn parse_policy(bytes: &[u8]) -> Result<PolicyFailureCause, DecodeError> {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(bytes, &mut scratch)?;
        let parsed = policy(&mut decoder, &mut None)?;
        decoder.finish()?;
        Ok(parsed)
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn every_policy_branch_preserves_raw_cause_and_derived_unit_forms() {
        let io = StartupCause::capture_io(&io::Error::from_raw_os_error(1)).unwrap();
        let dependency = StartupCause::Seccompiler(StartupSeccompilerCause::EmptyFilter);
        for expected in [
            PolicyFailureCause::UnsupportedArchitecture,
            PolicyFailureCause::InvalidProgram,
            PolicyFailureCause::NotBackend,
            PolicyFailureCause::ProtectionUnverified,
            PolicyFailureCause::Preparation { cause: io },
            PolicyFailureCause::Privilege { cause: io },
            PolicyFailureCause::Filter { cause: dependency },
            // Unusual but well-formed pairs stay raw for the authenticated owner to check.
            PolicyFailureCause::Preparation { cause: dependency },
            PolicyFailureCause::Filter { cause: io },
        ] {
            let bytes = serde_json::to_vec(&expected).unwrap();
            assert_eq!(parse_policy(&bytes).unwrap(), expected);
            if let serde_json::Value::String(name) = serde_json::to_value(expected).unwrap() {
                let map = format!("{{{}:null}}", serde_json::to_string(&name).unwrap());
                assert_eq!(parse_policy(map.as_bytes()).unwrap(), expected);
                for body in ["0", "false", "{}", "[]", "\"null\""] {
                    let malformed = format!("{{{}:{body}}}", serde_json::to_string(&name).unwrap());
                    assert!(parse_policy(malformed.as_bytes()).is_err());
                }
            } else {
                assert!(matches!(
                    expected,
                    PolicyFailureCause::Preparation { .. }
                        | PolicyFailureCause::Privilege { .. }
                        | PolicyFailureCause::Filter { .. }
                ));
            }
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn policy_body_and_external_tag_refuse_missing_duplicate_unknown_and_wrong_types() {
        let original = PolicyFailureCause::Preparation {
            cause: StartupCause::capture_io(&io::Error::from(io::ErrorKind::InvalidData)).unwrap(),
        };
        let mut value = serde_json::to_value(original).unwrap();
        value["Preparation"]["cause"]["Io"]
            .as_object_mut()
            .unwrap()
            .remove("raw_os_error");
        assert!(parse_policy(&serde_json::to_vec(&value).unwrap()).is_err());
        for bytes in [
            &br#"{}"#[..],
            br#"{"Preparation":{}}"#,
            br#"{"Preparation":null}"#,
            br#"{"Preparation":{"cause":null}}"#,
            br#"{"Preparation":{"foreign":null}}"#,
            br#""Preparation""#,
            br#"{"Other":null}"#,
            br#"{"InvalidProgram":null,"NotBackend":null}"#,
            br#"["InvalidProgram"]"#,
            br#"null"#,
        ] {
            assert!(parse_policy(bytes).is_err());
        }
        let cause = serde_json::to_string(&match original {
            PolicyFailureCause::Preparation { cause } => cause,
            _ => panic!("fixture must contain its original cause"),
        })
        .unwrap();
        for bytes in [
            format!("{{\"Preparation\":{{\"cause\":{cause},\"cause\":{cause}}}}}"),
            format!("{{\"Preparation\":{{\"cause\":{cause},\"foreign\":0}}}}"),
        ] {
            assert!(parse_policy(bytes.as_bytes()).is_err());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn policy_preserves_first_unknown_kind_before_later_syntax_and_specific_prior_fault() {
        let mut scratch = Scratch::default();
        let bytes = br#"{"Filter":{"cause":{"Io":{"kind":"UnknownKind","raw_os_error":null,"payload":"KindOnly"}},"later": [}}"#;
        let mut decoder = Decoder::new(bytes, &mut scratch).unwrap();
        let mut fault = None;
        assert_eq!(
            policy(&mut decoder, &mut fault).unwrap_err().cause(),
            DecodeCause::InvalidValue
        );
        assert_eq!(fault, Some(CauseIntegrityPredicate::UnknownKindMetadata));
        let mut decoder = Decoder::new(br#"{"Preparation":{}}"#, &mut scratch).unwrap();
        fault = Some(CauseIntegrityPredicate::PolicySiteCauseMismatch);
        assert_eq!(
            policy(&mut decoder, &mut fault).unwrap_err().cause(),
            DecodeCause::MissingField
        );
        assert_eq!(
            fault,
            Some(CauseIntegrityPredicate::PolicySiteCauseMismatch)
        );
        let mut decoder = Decoder::new(br#"{"Preparation":{}}"#, &mut scratch).unwrap();
        fault = None;
        assert!(policy(&mut decoder, &mut fault).is_err());
        assert_eq!(
            fault,
            Some(CauseIntegrityPredicate::IncompleteCauseMetadata)
        );
    }

    fn pair_refuses(bytes: &[u8], is_namespace: bool) {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(bytes, &mut scratch).unwrap();
        if is_namespace {
            assert!(namespace(&mut decoder)
                .and_then(|_| decoder.finish())
                .is_err());
        } else {
            assert!(peaks(&mut decoder).and_then(|_| decoder.finish()).is_err());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn namespace_and_peaks_require_exact_unsigned_fields_without_domain_inference() {
        let namespace_expected = NamespaceIdentity::from_wire_parts(u64::MAX, 0);
        let bytes = serde_json::to_vec(&namespace_expected).unwrap();
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(&bytes, &mut scratch).unwrap();
        assert_eq!(namespace(&mut decoder).unwrap(), namespace_expected);
        decoder.finish().unwrap();
        let peak_expected = MeasuredPeaks {
            tree_rss_bytes: u64::MAX,
            charged_bytes: 0,
        };
        let bytes = serde_json::to_vec(&peak_expected).unwrap();
        let mut decoder = Decoder::new(&bytes, &mut scratch).unwrap();
        let decoded = peaks(&mut decoder).unwrap();
        assert_eq!(decoded.tree_rss_bytes, peak_expected.tree_rss_bytes);
        assert_eq!(decoded.charged_bytes, peak_expected.charged_bytes);
        decoder.finish().unwrap();
        for (is_namespace, first, second) in [
            (true, "device", "inode"),
            (false, "tree_rss_bytes", "charged_bytes"),
        ] {
            for (member, other) in [(first, second), (second, first)] {
                pair_refuses(format!("{{\"{other}\":1}}").as_bytes(), is_namespace);
                for wrong in [
                    "null",
                    "false",
                    "\"1\"",
                    "-1",
                    "1.5",
                    "18446744073709551616",
                    "[]",
                    "{}",
                ] {
                    pair_refuses(
                        format!("{{\"{member}\":{wrong},\"{other}\":1}}").as_bytes(),
                        is_namespace,
                    );
                }
                pair_refuses(
                    format!("{{\"{member}\":1,\"{member}\":2,\"{other}\":3}}").as_bytes(),
                    is_namespace,
                );
            }
            pair_refuses(
                format!("{{\"{first}\":1,\"{second}\":2,\"foreign\":0}}").as_bytes(),
                is_namespace,
            );
            pair_refuses(
                format!("{{\"{first}\":1,\"{second}\":2}}?").as_bytes(),
                is_namespace,
            );
            pair_refuses(
                format!("{{\"{first}\":1,\"{second}\":2").as_bytes(),
                is_namespace,
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn owner_stop_preserves_unit_forms_and_escaped_names_without_outcome_authority() {
        let mut scratch = Scratch::default();
        for expected in [OwnerStopCause::ResourceExhausted, OwnerStopCause::TimedOut] {
            let bytes = serde_json::to_vec(&expected).unwrap();
            let mut decoder = Decoder::new(&bytes, &mut scratch).unwrap();
            assert_eq!(owner_stop(&mut decoder).unwrap(), expected);
            decoder.finish().unwrap();
            let name: String = serde_json::from_slice(&bytes).unwrap();
            let map = format!("{{{}:null}}", serde_json::to_string(&name).unwrap());
            let mut decoder = Decoder::new(map.as_bytes(), &mut scratch).unwrap();
            assert_eq!(owner_stop(&mut decoder).unwrap(), expected);
            decoder.finish().unwrap();
        }
        let mut decoder = Decoder::new(br#""\u0054imedOut""#, &mut scratch).unwrap();
        assert_eq!(owner_stop(&mut decoder).unwrap(), OwnerStopCause::TimedOut);
        decoder.finish().unwrap();
        for bytes in [
            &br#""Other""#[..],
            br#"{"TimedOut":false}"#,
            br#"{"TimedOut":null,"ResourceExhausted":null}"#,
            br#"{}"#,
            br#"null"#,
            br#"[]"#,
            br#""TimedOut"?"#,
        ] {
            let mut decoder = Decoder::new(bytes, &mut scratch).unwrap();
            assert!(owner_stop(&mut decoder)
                .and_then(|_| decoder.finish())
                .is_err());
        }
    }
    /// Trace: FR-034-AC-15
    #[test]
    fn positional_namespace_and_peaks_match_the_owning_record_grammar() {
        let bytes = b"[7,11]";
        let owning: NamespaceIdentity = serde_json::from_slice(bytes).unwrap();
        assert_eq!(parse(bytes, namespace).unwrap(), owning);
        let owning: MeasuredPeaks = serde_json::from_slice(bytes).unwrap();
        let actual = parse(bytes, peaks).unwrap();
        assert_eq!(actual.tree_rss_bytes, owning.tree_rss_bytes);
        assert_eq!(actual.charged_bytes, owning.charged_bytes);
        for bytes in [
            b"[]".as_slice(),
            b"[7]",
            b"[7,11,0]",
            b"[-1,11]",
            b"[7,true]",
        ] {
            assert!(serde_json::from_slice::<NamespaceIdentity>(bytes).is_err());
            assert!(parse(bytes, namespace).is_err());
            assert!(serde_json::from_slice::<MeasuredPeaks>(bytes).is_err());
            assert!(parse(bytes, peaks).is_err());
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn policy_derived_cause_forms_match_the_original_delegate_without_widening_seed_grammar() {
        for bytes in [
            br#"{"Preparation":[{"Io":[{"InvalidData":null},null,{"NoCustomPayload":null}]}]}"#.as_slice(),
            br#"{"Filter":[{"Seccompiler":{"EmptyFilter":null}}]}"#,
            br#"{"Filter":[{"Seccompiler":{"ThreadSync":[7]}}]}"#,
            br#"{"Filter":{"cause":{"Seccompiler":{"Prctl":["PermissionDenied",1,"NoCustomPayload"]}}}}"#,
        ] {
            let owning: PolicyFailureCause = serde_json::from_slice(bytes).unwrap();
            assert_eq!(parse_policy(bytes).unwrap(), owning);
        }
        for bytes in [
            br#"{"Preparation":[]}"#.as_slice(),
            br#"{"Preparation":[{"Io":["Other",null]}]}"#,
            br#"{"Filter":[{"Seccompiler":{"ThreadSync":[7,8]}}]}"#,
            br#"{"Preparation":[{"Io":["Other",null,"NoCustomPayload"]},null]}"#,
        ] {
            assert!(serde_json::from_slice::<PolicyFailureCause>(bytes).is_err());
            assert!(parse_policy(bytes).is_err());
        }
        // The original generic map seed is intentionally a different selected authority.
        let mut scratch = super::super::guardian_decode::Scratch::default();
        let mut decoder =
            Decoder::new(br#"{"Io":["Other",null,"NoCustomPayload"]}"#, &mut scratch).unwrap();
        assert!(cause_decode::decode(&mut decoder, &mut None).is_err());
    }
    /// Trace: FR-034-AC-15
    #[test]
    fn policy_unknown_kind_precedes_later_unit_body_syntax() {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(
            br#"{"Preparation":[{"Io":[{"UnknownKind":false},null,"NoCustomPayload"]}]}"#,
            &mut scratch,
        )
        .unwrap();
        let mut fault = None;
        let error = policy(&mut decoder, &mut fault).unwrap_err();
        assert_eq!(error.cause(), DecodeCause::InvalidValue);
        assert_eq!(fault, Some(CauseIntegrityPredicate::UnknownKindMetadata));
    }
}

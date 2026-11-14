// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Bounded scalar playback decoding and operand-domain validation (FR-032-AC-2/3).

use std::collections::BTreeSet;

use crate::kani::{
    abi::integer_scalar_operand_count,
    identity::{ScalarGuardDomain, ScalarObligationIdentity},
    output::playback::Playback,
};
use crate::replay::witness::DecodeFailure;

/// Maximum selected scalar playback bytes accepted by the decoder.
pub const SCALAR_PLAYBACK_BYTES: usize = 8 * 1024 * 1024;

/// Decodes typed integer scalar playback in the identity's persisted symbolic draw order.
///
/// Accepts only integer add, subtract, multiply and negate, with their required operand arity
/// and valid inclusive ranges. Literal singleton operands remain separate positions. The selected
/// single-harness transcript is limited to 8 MiB by this decoder, even if the output reader
/// accepted a larger input. Every draw
/// must contain exactly eight little-endian bytes and agree with Kani's decoded-value comment.
/// All draws are decoded before the first operand outside its persisted range is refused.
///
/// This validates playback against the supplied identity; it does not authenticate that identity
/// against a proving package or original proof context, execute an oracle, or build a replay plan.
/// A replay builder must authenticate its context and schema before calling this decoder.
///
/// # Errors
///
/// Returns arity, width and rendered-value refusal codes, or
/// `cg_scalar_witness_operation_unsupported`, `cg_scalar_witness_schema_arity_mismatch`,
/// `cg_scalar_witness_schema_range_invalid`, `cg_scalar_witness_schema_duplicate_binding`,
/// `cg_witness_decode_bytes_limit`, `cg_witness_harness_identity_mismatch`, or
/// `cg_scalar_witness_operand_out_of_domain`. Domain failures name the first operand in draw order.
pub fn decode_scalar_falsification(
    identity: &ScalarObligationIdentity,
    playback: &Playback<'_>,
) -> Result<Vec<i64>, DecodeFailure> {
    // An operation outside the integer profile has no persisted schema to check; its refusal
    // follows the byte ceiling and the harness identity.
    let arity = scalar_operand_count(identity);
    if let Some(arity) = arity {
        validate_schema(identity, arity)?;
    }
    let harness = identity.harness_symbol.as_str();
    let module = identity.module_symbol.as_str();
    let failure = |code, context: &str| DecodeFailure::new(code, harness, context);
    if playback.source_len > SCALAR_PLAYBACK_BYTES {
        return Err(failure(
            "cg_witness_decode_bytes_limit",
            &format!("{module};bytes={SCALAR_PLAYBACK_BYTES}"),
        ));
    }
    let qualified = format!("{module}::{harness}");
    if playback.harness != qualified {
        return Err(DecodeFailure::new(
            "cg_witness_harness_identity_mismatch",
            &qualified,
            playback.harness,
        ));
    }
    let arity = arity.ok_or_else(|| {
        failure(
            "cg_scalar_witness_operation_unsupported",
            &identity.operation_identity,
        )
    })?;
    let inputs = &identity.guard_inputs;
    if playback.entries.len() != arity + inputs.len() {
        return Err(failure("kani_witness_arity_mismatch", playback.check_text));
    }
    let (operand_entries, guard_entries) = playback.entries.split_at(arity);
    let values = identity
        .arguments
        .iter()
        .zip(operand_entries)
        .map(|(argument, (comment, bytes))| {
            let bytes: [u8; 8] = bytes
                .as_slice()
                .try_into()
                .map_err(|_| failure("kani_witness_width_mismatch", &argument.identifier))?;
            let value = i64::from_le_bytes(bytes);
            if comment.integer != Some(value) {
                return Err(failure(
                    "kani_witness_comment_mismatch",
                    &argument.identifier,
                ));
            }
            Ok(value)
        })
        .collect::<Result<Vec<_>, DecodeFailure>>()?;
    for (argument, value) in identity.arguments.iter().zip(&values) {
        if !(argument.minimum..=argument.maximum).contains(value) {
            return Err(failure(
                "cg_scalar_witness_operand_out_of_domain",
                &argument.identifier,
            ));
        }
    }
    for (input, (comment, bytes)) in inputs.iter().zip(guard_entries) {
        let identifier = input.identifier.as_str();
        let in_domain = match input.domain {
            ScalarGuardDomain::Boolean => {
                let [byte] = bytes.as_slice() else {
                    return Err(failure("kani_witness_width_mismatch", identifier));
                };
                if comment.boolean != Some(*byte == 1) && *byte <= 1 {
                    return Err(failure("kani_witness_comment_mismatch", identifier));
                }
                *byte <= 1
            }
            ScalarGuardDomain::Integer { minimum, maximum } => {
                let bytes: [u8; 8] = bytes
                    .as_slice()
                    .try_into()
                    .map_err(|_| failure("kani_witness_width_mismatch", identifier))?;
                let value = i64::from_le_bytes(bytes);
                if comment.integer != Some(value) {
                    return Err(failure("kani_witness_comment_mismatch", identifier));
                }
                (minimum..=maximum).contains(&value)
            }
        };
        if !in_domain {
            return Err(failure("cg_scalar_witness_guard_out_of_domain", identifier));
        }
    }
    Ok(values)
}

fn validate_schema(
    identity: &ScalarObligationIdentity,
    arity: usize,
) -> Result<usize, DecodeFailure> {
    let harness = identity.harness_symbol.as_str();
    let module = identity.module_symbol.as_str();
    let failure = |code, context: &str| DecodeFailure::new(code, harness, context);
    if identity.arguments.len() != arity {
        return Err(failure("cg_scalar_witness_schema_arity_mismatch", module));
    }
    if let Some(float_scalar) = &identity.float_scalar {
        let (minimum, maximum) = match float_scalar.width.as_str() {
            "float32" => (0, i64::from(u32::MAX)),
            "float64" => (i64::MIN, i64::MAX),
            _ => (1, 0),
        };
        let bound = identity
            .arguments
            .iter()
            .all(|argument| argument.minimum == minimum && argument.maximum == maximum);
        if minimum > maximum || !bound || float_scalar.operator != identity.operation_identity {
            return Err(failure(
                "cg_scalar_witness_schema_float_mismatch",
                &float_scalar.operator,
            ));
        }
    }
    if let Some(enum_scalar) = &identity.enum_scalar {
        let count = i64::try_from(enum_scalar.members.len()).unwrap_or(i64::MAX);
        let bound = identity
            .arguments
            .iter()
            .all(|argument| argument.minimum == 0 && argument.maximum == count - 1);
        if count == 0 || !bound || enum_scalar.operator != identity.operation_identity {
            return Err(failure(
                "cg_scalar_witness_schema_enum_mismatch",
                &enum_scalar.qualified_name.join("::"),
            ));
        }
    }
    let mut seen = BTreeSet::new();
    for argument in &identity.arguments {
        if argument.minimum > argument.maximum {
            return Err(failure(
                "cg_scalar_witness_schema_range_invalid",
                &argument.identifier,
            ));
        }
        if !seen.insert(argument.identifier.as_str()) {
            return Err(failure(
                "cg_scalar_witness_schema_duplicate_binding",
                &argument.identifier,
            ));
        }
    }
    for input in &identity.guard_inputs {
        let inverted = matches!(
            input.domain,
            ScalarGuardDomain::Integer { minimum, maximum } if minimum > maximum
        );
        if inverted {
            return Err(failure(
                "cg_scalar_witness_schema_range_invalid",
                &input.identifier,
            ));
        }
        if !seen.insert(input.identifier.as_str()) {
            return Err(failure(
                "cg_scalar_witness_schema_duplicate_binding",
                &input.identifier,
            ));
        }
    }
    Ok(arity)
}

/// Operand count of the persisted scalar schema: an enum identity carries exactly one physical
/// `i64` index per comparison operand, a float identity one `i64` bit pattern per operand, every other identity follows the integer profile.
fn scalar_operand_count(identity: &ScalarObligationIdentity) -> Option<usize> {
    if identity.enum_scalar.is_some() {
        return Some(2);
    }
    if let Some(float_scalar) = &identity.float_scalar {
        return Some(float_scalar.operands.len());
    }
    integer_scalar_operand_count(&identity.operation_identity)
}

/// One decoded enum comparison counterexample: each operand's declared member.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumWitness {
    /// Declaration node the members belong to.
    pub declaration: quire_contract_model::CheckedNodeId,
    /// Comparison operator identity.
    pub operator: String,
    /// Left operand rank in the declared member order.
    pub left_index: i64,
    /// Right operand rank in the declared member order.
    pub right_index: i64,
    /// Left operand member node.
    pub left_member: String,
    /// Right operand member node.
    pub right_member: String,
}

/// Decodes an enum comparison counterexample into the declared members it names.
///
/// Reuses the scalar draw checks (one `i64` per operand, comment agreement, range
/// `0..member count`), so an out-of-range, missing, duplicate or foreign binding is refused with
/// the scalar refusal codes and never maps to a member.
///
/// # Errors
///
/// Every `decode_scalar_falsification` refusal, plus `cg_scalar_witness_enum_identity_missing`
/// for an identity that is not an enum comparison.
pub fn decode_enum_falsification(
    identity: &ScalarObligationIdentity,
    playback: &Playback<'_>,
) -> Result<EnumWitness, DecodeFailure> {
    let enum_scalar = identity.enum_scalar.as_ref().ok_or_else(|| {
        DecodeFailure::new(
            "cg_scalar_witness_enum_identity_missing",
            identity.harness_symbol.as_str(),
            &identity.operation_identity,
        )
    })?;
    let values = decode_scalar_falsification(identity, playback)?;
    let member = |index: i64| {
        usize::try_from(index)
            .ok()
            .and_then(|position| enum_scalar.members.get(position))
            .cloned()
            .ok_or_else(|| {
                DecodeFailure::new(
                    "cg_scalar_witness_operand_out_of_domain",
                    identity.harness_symbol.as_str(),
                    &enum_scalar.qualified_name.join("::"),
                )
            })
    };
    let [left_index, right_index] = values[..] else {
        return Err(DecodeFailure::new(
            "kani_witness_arity_mismatch",
            identity.harness_symbol.as_str(),
            &enum_scalar.qualified_name.join("::"),
        ));
    };
    Ok(EnumWitness {
        declaration: enum_scalar.declaration.clone(),
        operator: enum_scalar.operator.clone(),
        left_index,
        right_index,
        left_member: member(left_index)?,
        right_member: member(right_index)?,
    })
}

/// One decoded float comparison counterexample: each operand parameter's exact bit pattern.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloatWitness {
    /// Comparison operator identity.
    pub operator: String,
    /// Operand width, `float32` or `float64`.
    pub width: String,
    /// Each operand as the QSL parameter name and its bit pattern, in operand order. A `float32`
    /// pattern is below `2^32`; a `float64` pattern is the drawn `i64` reinterpreted, never
    /// converted through a host float or a checked signed conversion.
    pub operands: Vec<(String, u64)>,
}

/// Decodes a float comparison counterexample into the bit patterns it names.
///
/// Reuses the scalar draw checks (one `i64` per operand, comment agreement, the width's range), so
/// an out-of-range, missing, duplicate or foreign binding is refused with the scalar refusal codes.
///
/// # Errors
///
/// Every `decode_scalar_falsification` refusal, plus `cg_scalar_witness_float_identity_missing`
/// for an identity that is not a float comparison.
pub fn decode_float_falsification(
    identity: &ScalarObligationIdentity,
    playback: &Playback<'_>,
) -> Result<FloatWitness, DecodeFailure> {
    let float_scalar = identity.float_scalar.as_ref().ok_or_else(|| {
        DecodeFailure::new(
            "cg_scalar_witness_float_identity_missing",
            identity.harness_symbol.as_str(),
            &identity.operation_identity,
        )
    })?;
    let values = decode_scalar_falsification(identity, playback)?;
    Ok(FloatWitness {
        operator: float_scalar.operator.clone(),
        width: float_scalar.width.clone(),
        operands: identity
            .arguments
            .iter()
            .zip(values)
            .map(|(argument, value)| {
                (
                    argument.identifier.clone(),
                    u64::from_ne_bytes(value.to_ne_bytes()),
                )
            })
            .collect(),
    })
}

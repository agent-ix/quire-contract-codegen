// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Bounded scalar playback decoding and operand-domain validation (FR-032-AC-2/3).

use std::collections::BTreeSet;

use crate::kani::{
    abi::integer_scalar_operand_count, identity::ScalarObligationIdentity,
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
    let arity = validate_schema(identity)?;
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
    if playback.entries.len() != arity {
        return Err(failure("kani_witness_arity_mismatch", playback.check_text));
    }
    let values = identity
        .arguments
        .iter()
        .zip(&playback.entries)
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
    Ok(values)
}

fn validate_schema(identity: &ScalarObligationIdentity) -> Result<usize, DecodeFailure> {
    let harness = identity.harness_symbol.as_str();
    let module = identity.module_symbol.as_str();
    let failure = |code, context: &str| DecodeFailure::new(code, harness, context);
    let arity = integer_scalar_operand_count(&identity.operation_identity).ok_or_else(|| {
        failure(
            "cg_scalar_witness_operation_unsupported",
            &identity.operation_identity,
        )
    })?;
    if identity.arguments.len() != arity {
        return Err(failure("cg_scalar_witness_schema_arity_mismatch", module));
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
    Ok(arity)
}

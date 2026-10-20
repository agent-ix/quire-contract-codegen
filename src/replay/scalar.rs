// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Bounded scalar playback decoding and operand-domain validation (FR-032-AC-2/3).

use std::collections::BTreeSet;

use crate::kani::{
    identity::ScalarObligationIdentity,
    output::playback::{read_block, select_assertion_block, DecodeFailure},
};

const PLAYBACK_BYTES: usize = 8 * 1024 * 1024;

/// Decodes integer scalar assertion playback in the identity's persisted symbolic draw order.
///
/// Accepts only integer add, subtract, multiply and negate, with their required operand arity
/// and valid inclusive ranges. Literal singleton operands remain separate positions. The selected
/// single-harness transcript is limited to 8 MiB before the Kani adapter parses it. Every draw
/// must contain exactly eight little-endian bytes and agree with Kani's decoded-value comment.
/// All draws are decoded before the first operand outside its persisted range is refused.
///
/// This validates playback against the supplied identity; it does not authenticate that identity
/// against a proving package or original proof context, execute an oracle, or build a replay plan.
/// A replay builder must authenticate its context and schema before calling this decoder.
///
/// # Errors
///
/// Returns the adapter's existing malformed-block, arity, width and comment refusal codes, or
/// `cg_scalar_witness_operation_unsupported`, `cg_scalar_witness_schema_arity_mismatch`,
/// `cg_scalar_witness_schema_range_invalid`, `cg_scalar_witness_schema_duplicate_binding`,
/// `cg_witness_decode_bytes_limit`, `cg_witness_harness_identity_mismatch`, or
/// `cg_scalar_witness_operand_out_of_domain`. Domain failures name the first operand in draw order.
pub fn decode_scalar_falsification(
    identity: &ScalarObligationIdentity,
    transcript: &str,
) -> Result<Vec<i64>, DecodeFailure> {
    let harness = identity.harness_symbol.as_str();
    let module = identity.module_symbol.as_str();
    let failure = |code, context: &str| DecodeFailure::new(code, harness, context);
    let arity = match identity.operation_identity.as_str() {
        "quire.op.integer.add" | "quire.op.integer.sub" | "quire.op.integer.mul" => 2,
        "quire.op.integer.negate" => 1,
        operation => {
            return Err(failure(
                "cg_scalar_witness_operation_unsupported",
                operation,
            ))
        }
    };
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
    if transcript.len() > PLAYBACK_BYTES {
        return Err(failure(
            "cg_witness_decode_bytes_limit",
            &format!("{module};bytes={PLAYBACK_BYTES}"),
        ));
    }
    let block = select_assertion_block(transcript, harness, module)?;
    // The fixed byte ceiling bounds adapter allocation. Read all entries so extra draws retain
    // the arity refusal, rather than being reported as a caller-configured value limit.
    let playback = read_block(block, harness, module, usize::MAX)?;
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
            if comment.parse::<i64>() != Ok(value) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::contract_package::{
            routed_contract_lowering_profile, ContractDriverResultSelection,
            ContractRequestPosition,
        },
        kani::test_support::proof_ceilings::proof_ceilings,
        routed::{
            capability::{BackendDescriptor, Candidate, ProviderOrigin},
            generate::{
                generate_routed, GenerationContexts, KaniGenerationContext, KindOutput,
                RoutedGenerationItem,
            },
        },
    };
    use ix_trace_rs::trace;
    use qsl_replay::{compile_package, DependencyInput, ReplayLimits, SourceIdentity, StageLimits};
    use quire_contract_model::{
        CheckedOccurrenceRole, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
        CheckedPackageV2ReadResult, ContractDependencies, ContractOccurrenceKeyV1,
        ContractPackageReadResult, ContractPackageWireV1,
    };

    fn identity(
        operation: &str,
        parameters: &str,
        result: &str,
        expression: &str,
    ) -> ScalarObligationIdentity {
        let source = format!(
            "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction subject using v({parameters}): {result} pure {{ {expression} }}\n"
        );
        let compiled = compile_package(
            SourceIdentity::new("test", "scalar-playback.native", "git", "fixture"),
            "scalar-playback.native",
            source.as_bytes(),
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("QSL compiles the authored scalar source");
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        let checked = match CheckedPackageV2::read(
            compiled.bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence,
        ) {
            CheckedPackageV2ReadResult::Admitted(package) => package,
            other => panic!("IR admits the emitted package: {other:?}"),
        };
        let applications = checked
            .graph()
            .nodes
            .iter()
            .filter(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some(operation)
            })
            .collect::<Vec<_>>();
        let [application] = applications.as_slice() else {
            panic!("one authored scalar application")
        };
        let node_id = application.node_id.clone();
        let conversion = checked
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(serde_json::Value::as_str)
                    == Some("quire.op.numeric.narrow")
                    && node.body.pointer("/arguments/0/target")
                        == Some(&serde_json::to_value(&node_id).unwrap())
            })
            .expect("authored result narrowing");
        let occurrence = |node_id: &quire_contract_model::CheckedNodeId| {
            let entries = checked
                .source_map()
                .iter()
                .filter(|entry| {
                    &entry.node_id == node_id && entry.role == CheckedOccurrenceRole::Expression
                })
                .collect::<Vec<_>>();
            let [entry] = entries.as_slice() else {
                panic!("one authored expression occurrence")
            };
            ContractOccurrenceKeyV1 {
                node_id: node_id.clone(),
                role: entry.role.clone(),
                ordinal: entry.ordinal,
            }
        };
        let selection = ContractDriverResultSelection::OwnResult {
            application: occurrence(&node_id),
            conversion: Some(occurrence(&conversion.node_id)),
        };
        let positions = [
            ContractRequestPosition::DriverItem {
                node_id: node_id.clone(),
                result_selection: selection.clone(),
            },
            ContractRequestPosition::ContextRoot {
                node_id: conversion.node_id.clone(),
            },
        ];
        let lowered = checked.lower(
            &[node_id.clone(), conversion.node_id.clone()],
            &routed_contract_lowering_profile(),
        );
        let admitted = match ContractPackageWireV1::read(
            lowered.package.canonical_bytes().unwrap(),
            CheckedPackageReadLimits::bounded(),
            &ContractDependencies::new(),
        ) {
            ContractPackageReadResult::Admitted(package) => package,
            other => panic!("IR admits routed package: {other:?}"),
        };
        let route = RoutedGenerationItem::from_descriptor(
            0,
            node_id,
            Candidate {
                identity: "kani".to_owned(),
            },
            &BackendDescriptor {
                identity: "kani".to_owned(),
                advertised: Vec::new(),
                origin: ProviderOrigin::Linked,
                domains: None,
                bounds: None,
            },
        )
        .expect("linked Kani route")
        .with_scalar_selection(&admitted, selection);
        let generated = generate_routed(
            &admitted,
            &positions,
            &[route],
            &GenerationContexts {
                kani: Some(KaniGenerationContext {
                    ceilings: proof_ceilings(),
                    subject_path: "crate::subject",
                    unwind: 3,
                }),
            },
        )
        .expect("actual routed scalar generation");
        let [item] = generated.items.as_slice() else {
            panic!("one routed item")
        };
        let KindOutput::Kani {
            harness: Some(harness),
            ..
        } = &item.output
        else {
            panic!("routed scalar harness: {:?}", item.output)
        };
        harness.identity.clone()
    }

    fn sum() -> ScalarObligationIdentity {
        identity(
            "quire.op.integer.add",
            "x: Int[0, 9], y: Int[10, 20]",
            "Int[0, 29]",
            "x + y",
        )
    }

    fn transcript(identity: &ScalarObligationIdentity, values: &[i64]) -> String {
        let entries = values
            .iter()
            .map(|value| format!("// {value}\nvec!{:?},\n", value.to_le_bytes()))
            .collect::<String>();
        format!(
            "/// Test generated for harness `{}::{}`\n/// Check for `assertion`: \"scalar parity\"\nfn playback() {{\nlet concrete_vals: Vec<Vec<u8>> = vec![\n{entries}];\nkani::concrete_playback_run(concrete_vals, harness);\n}}",
            identity.module_symbol, identity.harness_symbol,
        )
    }

    fn refusal(identity: &ScalarObligationIdentity, text: &str, code: &str) -> DecodeFailure {
        let error = decode_scalar_falsification(identity, text).expect_err("must refuse playback");
        assert_eq!(error.code, code);
        error
    }

    #[trace("FR-032-AC-2", "FR-032-AC-3")]
    #[test]
    fn routed_draw_order_is_preserved_with_distinct_domains() {
        let identity = sum();
        assert_eq!(identity.operation_identity, "quire.op.integer.add");
        assert_eq!(
            identity
                .arguments
                .iter()
                .map(|arg| (arg.minimum, arg.maximum))
                .collect::<Vec<_>>(),
            [(0, 9), (10, 20)],
        );
        assert_eq!(
            decode_scalar_falsification(&identity, &transcript(&identity, &[3, 17])),
            Ok(vec![3, 17])
        );
        let error = refusal(
            &identity,
            &transcript(&identity, &[17, 3]),
            "cg_scalar_witness_operand_out_of_domain",
        );
        assert_eq!(error.context, identity.arguments[0].identifier);
        let product = identity_for_product();
        assert_eq!(
            decode_scalar_falsification(&product, &transcript(&product, &[4, 130])),
            Ok(vec![4, 130])
        );
    }

    fn identity_for_product() -> ScalarObligationIdentity {
        identity(
            "quire.op.integer.mul",
            "x: Int[1, 9], y: Int[100, 200]",
            "Int[100, 1800]",
            "x * y",
        )
    }

    #[trace("FR-032-AC-2", "FR-032-AC-3")]
    #[test]
    fn negate_and_literal_singleton_keep_every_draw() {
        let negate = identity(
            "quire.op.integer.negate",
            "x: Int[1, 9]",
            "Int[-9, -1]",
            "-x",
        );
        assert_eq!(
            decode_scalar_falsification(&negate, &transcript(&negate, &[7])),
            Ok(vec![7])
        );
        let literal = identity(
            "quire.op.integer.add",
            "x: Int[0, 9]",
            "Int[0, 10]",
            "x + 1",
        );
        assert_eq!(
            (literal.arguments[1].minimum, literal.arguments[1].maximum),
            (1, 1)
        );
        assert_eq!(
            decode_scalar_falsification(&literal, &transcript(&literal, &[4, 1])),
            Ok(vec![4, 1])
        );
        refusal(
            &literal,
            &transcript(&literal, &[4]),
            "kani_witness_arity_mismatch",
        );
        for value in [0, 2] {
            let error = refusal(
                &literal,
                &transcript(&literal, &[4, value]),
                "cg_scalar_witness_operand_out_of_domain",
            );
            assert_eq!(error.context, literal.arguments[1].identifier);
        }
    }

    #[trace("FR-032-AC-2", "FR-032-AC-3")]
    #[test]
    fn subtraction_decodes_signed_extreme_operands_without_evaluating_them() {
        let identity = identity(
            "quire.op.integer.sub",
            "x: Int[-9223372036854775808, 9223372036854775807], y: Int[0, 0]",
            "Int[-9223372036854775808, 9223372036854775807]",
            "x - y",
        );
        for values in [[i64::MIN, 0], [i64::MAX, 0], [-1, 0]] {
            assert_eq!(
                decode_scalar_falsification(&identity, &transcript(&identity, &values)),
                Ok(values.to_vec())
            );
        }
    }

    #[trace("FR-032-AC-3")]
    #[test]
    fn inclusive_endpoints_and_adjacent_values_use_each_operands_range() {
        let identity = sum();
        for values in [[0, 10], [9, 20], [0, 20], [9, 10]] {
            assert_eq!(
                decode_scalar_falsification(&identity, &transcript(&identity, &values)),
                Ok(values.to_vec())
            );
        }
        for (values, index) in [
            ([-1, 10], 0),
            ([10, 10], 0),
            ([0, 9], 1),
            ([0, 21], 1),
            ([-1, 21], 0),
        ] {
            let error = refusal(
                &identity,
                &transcript(&identity, &values),
                "cg_scalar_witness_operand_out_of_domain",
            );
            assert_eq!(error.context, identity.arguments[index].identifier);
        }
    }

    #[trace("FR-032-AC-2")]
    #[test]
    fn harness_arity_width_and_comment_failures_are_distinct() {
        let identity = sum();
        let good = transcript(&identity, &[3, 17]);
        let wrong_harness = good.replace(
            &format!("{}::{}", identity.module_symbol, identity.harness_symbol),
            "other::harness",
        );
        refusal(
            &identity,
            &wrong_harness,
            "cg_witness_harness_identity_mismatch",
        );
        for values in [vec![], vec![3], vec![3, 17, 19]] {
            refusal(
                &identity,
                &transcript(&identity, &values),
                "kani_witness_arity_mismatch",
            );
        }
        for bytes in [
            "[3]",
            "[3, 0, 0, 0, 0, 0, 0]",
            "[3, 0, 0, 0, 0, 0, 0, 0, 0]",
        ] {
            refusal(
                &identity,
                &good.replace("[3, 0, 0, 0, 0, 0, 0, 0]", bytes),
                "kani_witness_width_mismatch",
            );
        }
        refusal(
            &identity,
            &good.replace("// 3\n", "// 4\n"),
            "kani_witness_comment_mismatch",
        );
        refusal(
            &identity,
            &good.replace("// 3\n", "// invalid\n"),
            "kani_witness_comment_mismatch",
        );
    }

    #[trace("FR-032-AC-2")]
    #[test]
    fn assertion_selection_and_malformed_blocks_use_adapter_refusals() {
        let identity = sum();
        let good = transcript(&identity, &[3, 17]);
        let cover = good.replace("`assertion`", "`cover`");
        assert_eq!(
            decode_scalar_falsification(&identity, &format!("{cover}\n{good}")),
            Ok(vec![3, 17])
        );
        for (text, code) in [
            ("no block".to_owned(), "kani_witness_harness_missing"),
            (cover, "kani_witness_cover_refused"),
            (
                format!("{good}\n{good}"),
                "kani_witness_multiple_assertions_refused",
            ),
            (
                good.replace("`assertion`", "`other`"),
                "kani_witness_check_kind_refused",
            ),
            (
                good.replace("/// Check for", "/// Missing"),
                "kani_witness_check_missing",
            ),
            (
                good.replace("\"scalar parity\"", "missing quote"),
                "kani_witness_check_text_missing",
            ),
            (
                good.replace("let concrete_vals", "let missing"),
                "kani_witness_concrete_vals_missing",
            ),
            (
                good.replace("];\nkani", ";\nkani"),
                "kani_witness_concrete_vals_malformed",
            ),
            (good.replace("// 3\n", ""), "kani_witness_comment_missing"),
            (
                good.replace("[3, 0, 0, 0, 0, 0, 0, 0]", "[999]"),
                "kani_witness_byte_invalid",
            ),
        ] {
            refusal(&identity, &text, code);
        }
    }

    #[trace("FR-032-AC-2", "FR-032-AC-3")]
    #[test]
    fn schema_then_byte_ceiling_then_decode_then_domain_precedence() {
        let identity = sum();
        let oversized = "x".repeat(PLAYBACK_BYTES + 1);
        refusal(&identity, &oversized, "cg_witness_decode_bytes_limit");
        let mut at_limit = transcript(&identity, &[3, 17]);
        at_limit.push_str(&" ".repeat(PLAYBACK_BYTES - at_limit.len()));
        assert_eq!(
            decode_scalar_falsification(&identity, &at_limit),
            Ok(vec![3, 17])
        );
        let malformed = transcript(&identity, &[-1, 17]).replace("// 17\n", "// 18\n");
        refusal(&identity, &malformed, "kani_witness_comment_mismatch");
        let mut invalid = identity.clone();
        invalid.arguments[0].minimum = 10;
        let error = refusal(
            &invalid,
            &oversized,
            "cg_scalar_witness_schema_range_invalid",
        );
        assert_eq!(error.context, invalid.arguments[0].identifier);
    }

    #[trace("FR-032-AC-2", "FR-032-AC-3")]
    #[test]
    fn unsupported_operator_and_invalid_persisted_schema_refuse_before_parsing() {
        let identity = sum();
        for operation in [
            "quire.op.boolean.implies",
            "quire.op.integer.div",
            "unknown",
        ] {
            let mut unsupported = identity.clone();
            unsupported.operation_identity = operation.to_owned();
            refusal(
                &unsupported,
                "no block",
                "cg_scalar_witness_operation_unsupported",
            );
        }
        let mut wrong_arity = identity.clone();
        wrong_arity.arguments.pop();
        refusal(
            &wrong_arity,
            "no block",
            "cg_scalar_witness_schema_arity_mismatch",
        );
        let mut duplicate = identity.clone();
        duplicate.arguments[1].identifier = duplicate.arguments[0].identifier.clone();
        refusal(
            &duplicate,
            "no block",
            "cg_scalar_witness_schema_duplicate_binding",
        );
    }
}

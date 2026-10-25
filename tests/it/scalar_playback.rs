// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-032 scalar playback through the public decoder and genuine routed identities.

use crate::common::proof_ceilings::proof_ceilings;
use ix_trace_rs::trace;
use qsl_replay::{compile_package, DependencyInput, ReplayLimits, SourceIdentity, StageLimits};
use quire_contract_codegen::{
    decode_scalar_falsification, generate_routed, parse_scalar_playback,
    routed_contract_lowering_profile, BackendDescriptor, Candidate, ContractDriverResultSelection,
    ContractRequestPosition, DecodeFailure, GenerationContexts, KaniGenerationContext,
    KaniScalarObligationHarness, KindOutput, ProviderOrigin, RoutedGenerationItem, ScalarLimits,
    ScalarObligationIdentity, ScalarOriginalLimitsError,
};
use quire_contract_model::{
    CheckedOccurrenceRole, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm, ContractDependencies, ContractOccurrenceKeyV1,
    ContractPackageReadResult, ContractPackageWireV1,
};

const PLAYBACK_BYTES: usize = 8 * 1024 * 1024;

fn decode_from_transcript(
    identity: &ScalarObligationIdentity,
    transcript: &str,
) -> Result<Vec<i64>, DecodeFailure> {
    let parsed = parse_scalar_playback(identity, transcript)?;
    decode_scalar_falsification(identity, &parsed)
}

fn operation_identity(body: &CheckedTerm) -> Option<&str> {
    let CheckedTerm::Application { operation, .. } = body else {
        return None;
    };
    operation.get("identity")?.as_str()
}

fn harness(
    operation: &str,
    parameters: &str,
    result: &str,
    expression: &str,
) -> KaniScalarObligationHarness {
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
        .filter(|node| operation_identity(&node.body) == Some(operation))
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
            operation_identity(&node.body) == Some("quire.op.numeric.narrow")
                && matches!(&node.body, CheckedTerm::Application { arguments, .. }
                    if matches!(arguments.first(), Some(CheckedTerm::Reference { target }) if target == &node_id))
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
    (**harness).clone()
}

fn identity(
    operation: &str,
    parameters: &str,
    result: &str,
    expression: &str,
) -> ScalarObligationIdentity {
    harness(operation, parameters, result, expression).identity
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
    let error = decode_from_transcript(identity, text).expect_err("must refuse playback");
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
        decode_from_transcript(&identity, &transcript(&identity, &[3, 17])),
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
        decode_from_transcript(&product, &transcript(&product, &[4, 130])),
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
        decode_from_transcript(&negate, &transcript(&negate, &[7])),
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
        decode_from_transcript(&literal, &transcript(&literal, &[4, 1])),
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
            decode_from_transcript(&identity, &transcript(&identity, &values)),
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
            decode_from_transcript(&identity, &transcript(&identity, &values)),
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
        decode_from_transcript(&identity, &format!("{cover}\n{good}")),
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
        decode_from_transcript(&identity, &at_limit),
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
        "quire.op.integer.lt",
        "quire.op.integer.eq",
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

#[trace("FR-032-AC-3")]
#[test]
fn original_renderer_limits_are_retained_with_genuine_routed_source() {
    let harness = harness(
        "quire.op.integer.add",
        "x: Int[0, 9]",
        "Int[0, 10]",
        "x + 1",
    );
    let limits = harness
        .original_scalar_limits()
        .expect("generation retained its actual meter limits");
    assert_eq!(
        limits,
        ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        }
    );
    assert!(harness.rust.contents.contains("rt::Meter::new(rt::ScalarLimits {\ninteger_bits: u64::MAX,\ndecimal_digits: u64::MAX,\nscale_expansion: u64::MAX,\ntext_input_bytes: u64::MAX,\ntext_scalars: u64::MAX,\nnormalized_scalars: u64::MAX,\nunit_edges: u64::MAX,\nvalue_occurrences: u64::MAX,\nwork_units: u64::MAX,\nresult_units: u64::MAX,\n})"));
    assert_eq!(
        harness
            .identity
            .arguments
            .iter()
            .map(|arg| (arg.minimum, arg.maximum))
            .collect::<Vec<_>>(),
        [(0, 9), (1, 1)]
    );
    assert!(harness
        .rust
        .contents
        .contains("exact >= i128::from(0_i64) && exact <= i128::from(10_i64)"));
}

#[trace("FR-032-AC-3")]
#[test]
fn original_limits_refuse_changed_retained_identity_source_or_record() {
    let harness = harness(
        "quire.op.integer.sub",
        "x: Int[0, 9], y: Int[0, 9]",
        "Int[-9, 9]",
        "x - y",
    );
    assert_eq!(
        harness.original_scalar_limits().unwrap().integer_bits,
        u64::MAX
    );
    let mut identity_changed = harness.clone();
    identity_changed.identity.arguments[0].maximum = 8;
    let mut source_changed = harness.clone();
    source_changed
        .rust
        .contents
        .push_str("\n// changed retained source");
    let mut record_changed = harness.clone();
    record_changed.record.contents.push(' ');
    for changed in [identity_changed, source_changed, record_changed] {
        assert_eq!(
            changed.original_scalar_limits(),
            Err(ScalarOriginalLimitsError::GeneratedArtifactChanged)
        );
    }
    assert_eq!(
        harness.original_scalar_limits().unwrap().integer_bits,
        u64::MAX
    );
}

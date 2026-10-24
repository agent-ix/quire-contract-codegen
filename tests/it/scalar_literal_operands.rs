// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-032 operand provenance at the QSL-emitted V2 scalar generation seam.
//! These controls do not exercise the still-gated scalar replay builder.

use crate::common::proof_ceilings::proof_ceilings;
use ix_trace_rs::trace;
use qsl_replay::{compile_package, DependencyInput, ReplayLimits, SourceIdentity, StageLimits};
use quire_contract_codegen::{
    generate_routed, routed_contract_lowering_profile, BackendDescriptor, Candidate,
    ClaimDisposition, ContractDriverResultSelection, ContractRequestPosition, GeneratedScalarClaim,
    GenerationContexts, KaniGenerationContext, KindOutput, ProviderOrigin, RoutedGenerationItem,
};
use quire_contract_model::{
    CheckedOccurrenceRole, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult, CheckedScalarOperandChild,
    CheckedScalarOperandRange, CheckedSemanticNodeV2, CheckedTerm, ContractDependencies,
    ContractOccurrenceKeyV1, ContractPackageReadResult, ContractPackageWireV1, LiteralKind,
};

fn compiled(parameters: &str, result: &str, expression: &str) -> CheckedPackageV2 {
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\n\
         profile v = \"quire.value.complete/v1\";\n\
         function subject using v({parameters}): {result} pure {{ {expression} }}\n"
    );
    let compiled = compile_package(
        SourceIdentity::new("test", "scalar-literals.native", "git", "fixture"),
        "scalar-literals.native",
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL compiles the authored scalar source");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(
        compiled.bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("IR admits the genuine QSL package: {other:?}"),
    }
}

fn application<'a>(package: &'a CheckedPackageV2, identity: &str) -> &'a CheckedSemanticNodeV2 {
    let nodes = package
        .graph()
        .nodes
        .iter()
        .filter(|node| operation_identity(node) == Some(identity))
        .collect::<Vec<_>>();
    let [node] = nodes.as_slice() else {
        panic!("one authored {identity} application: {nodes:?}");
    };
    node
}

fn application_arguments(node: &CheckedSemanticNodeV2) -> &[CheckedTerm] {
    let CheckedTerm::Application { arguments, .. } = &node.body else {
        panic!("selected node is an application");
    };
    arguments
}

fn operation_identity(node: &CheckedSemanticNodeV2) -> Option<&str> {
    let CheckedTerm::Application { operation, .. } = &node.body else {
        return None;
    };
    operation.get("identity")?.as_str()
}

fn assert_generated(
    package: &CheckedPackageV2,
    node: &CheckedSemanticNodeV2,
    expected_draws: Option<&[(i64, i64)]>,
) -> Box<GeneratedScalarClaim> {
    let occurrence = |node: &CheckedSemanticNodeV2| {
        let entries = node
            .occurrences
            .iter()
            .filter(|entry| entry.role == CheckedOccurrenceRole::Expression)
            .collect::<Vec<_>>();
        let [entry] = entries.as_slice() else {
            panic!("one authentic expression occurrence");
        };
        ContractOccurrenceKeyV1 {
            node_id: node.node_id.clone(),
            role: entry.role.clone(),
            ordinal: entry.ordinal,
        }
    };
    let conversion = package.graph().nodes.iter().find(|candidate| {
        operation_identity(candidate) == Some("quire.op.numeric.narrow")
            && matches!(
                application_arguments(candidate).first(),
                Some(CheckedTerm::Reference { target }) if target == &node.node_id
            )
    });
    let selection = ContractDriverResultSelection::OwnResult {
        application: occurrence(node),
        conversion: conversion.map(occurrence),
    };
    let mut positions = vec![ContractRequestPosition::DriverItem {
        node_id: node.node_id.clone(),
        result_selection: selection.clone(),
    }];
    if let Some(conversion) = conversion {
        positions.push(ContractRequestPosition::ContextRoot {
            node_id: conversion.node_id.clone(),
        });
    }
    let roots = positions
        .iter()
        .map(|position| position.node_id().clone())
        .collect::<Vec<_>>();
    let lowered = package.lower(&roots, &routed_contract_lowering_profile());
    let admitted = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().unwrap(),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => package,
        other => panic!("IR admits the routed scalar package: {other:?}"),
    };
    let route = RoutedGenerationItem::from_descriptor(
        0,
        node.node_id.clone(),
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
    .unwrap()
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
    .expect("generate routed scalar oracle");
    let [item] = generated.items.as_slice() else {
        panic!("one routed scalar item");
    };
    let KindOutput::Kani { harness, .. } = &item.output else {
        panic!("the linked scalar Kani generator handled the item");
    };
    match expected_draws {
        Some(expected) => {
            let harness = harness.as_ref().expect("arithmetic scalar harness");
            assert_eq!(
                harness
                    .identity
                    .arguments
                    .iter()
                    .map(|arg| (arg.minimum, arg.maximum))
                    .collect::<Vec<_>>(),
                expected,
                "the generated harness retains every operand's own range in draw order",
            );
        }
        None => assert!(
            harness.is_none(),
            "equality has an oracle but no scalar Kani renderer"
        ),
    }
    let claim_map = generated.claim_map.as_ref().expect("scalar claim map");
    let [claim] = claim_map.items.as_slice() else {
        panic!("one selected scalar claim");
    };
    let ClaimDisposition::Generated(generated) = &claim.result else {
        panic!(
            "the authored scalar operands must generate: {:?}",
            claim.result
        );
    };
    assert_eq!(claim.node_id, node.node_id);
    assert_eq!(
        Some(claim.operation.identity.as_str()),
        operation_identity(node)
    );
    assert!(generated.oracle_source.contains(&generated.symbol));
    generated.clone()
}

fn assert_inline(
    package: &CheckedPackageV2,
    node: &CheckedSemanticNodeV2,
    ordinal: usize,
    value: i128,
) {
    let Some(CheckedTerm::Literal {
        type_id,
        value_kind: LiteralKind::Integer,
        value: literal,
    }) = application_arguments(node).get(ordinal)
    else {
        panic!("inline integer literal, never a graph reference");
    };
    assert_eq!(literal, &value.to_string());
    let integer = package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == type_id)
        .expect("literal type belongs to the checked graph");
    assert_eq!(integer.node_tag.as_ref(), "scalar_type");
    assert_eq!(integer.semantic_form.as_ref(), "integer");
    if operation_identity(node) != Some("quire.op.integer.eq") {
        let [occurrence] = node.occurrences.as_slice() else {
            panic!("one authentic application occurrence");
        };
        let operands = package
            .scalar_application_operands(&node.node_id, occurrence)
            .expect("IR projects the actual scalar operands");
        let operand = &operands[ordinal];
        assert_eq!(operand.ordinal, u64::try_from(ordinal).unwrap());
        assert_eq!(
            operand.range,
            CheckedScalarOperandRange {
                lower: value,
                upper: value
            }
        );
        assert_eq!(
            operand.child,
            CheckedScalarOperandChild::InlineLiteral {
                application: node.node_id.clone(),
                occurrence: occurrence.clone(),
                ordinal: u64::try_from(ordinal).unwrap(),
            }
        );
    }
}

#[trace("TC-047", "FR-032-AC-3", "FR-032-AC-9")]
#[test]
fn add_keeps_the_plain_literal_inline_through_scalar_generation() {
    let package = compiled("x: Int[0, 9]", "Int[0, 10]", "x + 1");
    let add = application(&package, "quire.op.integer.add");
    assert_inline(&package, add, 1, 1);
    assert_generated(&package, add, Some(&[(0, 9), (1, 1)]));
}

#[trace("TC-047", "FR-032-AC-9")]
#[test]
fn equality_with_a_plain_integer_peer_keeps_the_literal_inline() {
    let package = compiled("x: Integer", "Boolean", "x = 1");
    let eq = application(&package, "quire.op.integer.eq");
    assert_inline(&package, eq, 1, 1);
    assert_generated(&package, eq, None);
}

#[trace("TC-047", "FR-032-AC-3", "FR-032-AC-9")]
#[test]
fn negate_keeps_the_plain_literal_inline_through_scalar_generation() {
    let package = compiled("", "Int[-9, 0]", "-1");
    let negate = application(&package, "quire.op.integer.negate");
    assert_inline(&package, negate, 0, 1);
    assert_generated(&package, negate, Some(&[(1, 1)]));
}

#[trace("TC-047", "FR-032-AC-9")]
#[test]
fn equality_with_a_bounded_peer_keeps_the_narrowed_literal_as_a_graph_child() {
    let package = compiled("x: Int[0, 9]", "Boolean", "x = 1");
    let eq = application(&package, "quire.op.integer.eq");
    let Some(CheckedTerm::Reference { target: child }) = application_arguments(eq).get(1) else {
        panic!("bounded equality operand is a graph reference");
    };
    let narrow = application(&package, "quire.op.numeric.narrow");
    assert_eq!(
        child, &narrow.node_id,
        "the operand is the composite narrowing application"
    );
    let Some(CheckedTerm::Reference { target: literal_id }) = application_arguments(narrow).first()
    else {
        panic!("narrowed literal is a graph reference");
    };
    let literal = package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == literal_id)
        .expect("narrow's literal child");
    let CheckedTerm::Literal { value, .. } = &literal.body else {
        panic!("narrow's child is a literal");
    };
    assert_eq!(value, "1");
    let generated = assert_generated(&package, eq, None);
    assert!(
        generated.checked_bounds.is_empty(),
        "equality adds no scalar bound parameters for the composite operand"
    );
}

// SPDX-License-Identifier: AGPL-3.0-or-later
//! Authentic inline Integer operands for the composite equality generator.

use super::admitted_enum_equality::{checked, generate_items};
use ix_trace_rs::trace;
use quire_contract_codegen::{
    ClaimDisposition, CompositeEqualityItem, CompositeEqualityRefusal, EqualityOperandDescriptor,
    EqualityOperatorKind,
};
use quire_contract_model::{CheckedNodeTag, CheckedTerm, LiteralKind};

const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
    profile v = \"quire.value.complete/v1\";\n\
    function literal_equal using v(): Boolean pure { 2 = 2 }\n";

#[trace("FR-018-AC-47", "TC-029")]
#[test]
fn genuine_inline_integer_literals_generate_and_wrong_descriptor_refuses() {
    let compiled = qsl_replay::compile_admitted(
        qsl_replay::CompileUnit {
            source: qsl_replay::SourceIdentity::new("test", "inline-integer-equality.native"),
            package_name: "inline-integer-equality".to_owned(),
            path: "inline-integer-equality.native",
            bytes: SOURCE.as_bytes(),
        },
        [],
        &qsl_replay::DependencyInput::default(),
        qsl_replay::StageLimits::default(),
        qsl_replay::ReplayLimits::default(),
    )
    .expect("genuine QSL source compiles");
    let package = checked(&compiled, &[]);
    let expression = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.node_tag.as_ref() == CheckedNodeTag::Expression.as_wire()
                && node.semantic_form.as_ref() == "binary"
        })
        .expect("genuine checked equality expression");
    let CheckedTerm::Application { arguments, .. } = &expression.body else {
        panic!("binary expression has an application body")
    };
    let [CheckedTerm::Literal {
        type_id: left,
        value_kind: LiteralKind::Integer,
        ..
    }, CheckedTerm::Literal {
        type_id: right,
        value_kind: LiteralKind::Integer,
        ..
    }] = &arguments[..]
    else {
        panic!("the checked source retains both inline literals: {arguments:?}")
    };
    assert_eq!(left, right);
    assert!(package.graph().nodes.iter().any(|node| {
        node.node_id == *left
            && node.node_tag.as_ref() == CheckedNodeTag::ScalarType.as_wire()
            && node.semantic_form.as_ref() == "integer"
    }));
    let healthy = CompositeEqualityItem {
        node_id: expression.node_id.clone(),
        operator: EqualityOperatorKind::Equal,
        left: EqualityOperandDescriptor::typed(left.clone()),
        right: EqualityOperandDescriptor::typed(right.clone()),
    };
    let baseline = generate_items(&package, None, std::slice::from_ref(&healthy), &[])
        .expect("the real package lowers for generation");
    let [claim] = &baseline.claim_map.items[..] else {
        panic!("one baseline claim")
    };
    assert!(
        matches!(claim.result, ClaimDisposition::Generated(_)),
        "inline Integer literals must generate: {:?}",
        claim.result
    );
    let ClaimDisposition::Generated(generated) = &claim.result else {
        panic!("the source-backed equality must generate")
    };
    let names = format!(
        "pub use self::{{{} as environment_literal_equal, {} as oracle_literal_equal}};\n",
        generated.environment_symbol, generated.oracle_symbol
    );
    super::exact_scalar_agreement::run_agreement_cases(
        "inline-integer-equality",
        baseline
            .artifacts
            .iter()
            .map(|artifact| (artifact.path.as_str(), artifact.contents.as_str())),
        &[("names.rs", names.as_str())],
        "tests/inline_integer_equality_support/agreement_cases.rs",
    );

    let boolean = expression.semantic_type.clone();
    assert_ne!(boolean, *left);
    let wrong = CompositeEqualityItem {
        left: EqualityOperandDescriptor::typed(boolean.clone()),
        ..healthy.clone()
    };
    let mixed = generate_items(
        &package,
        None,
        &[healthy, wrong],
        std::slice::from_ref(&boolean),
    )
    .expect("both descriptors receive a disposition");
    assert_eq!(mixed.claim_map.items.len(), 2);
    let generated = mixed
        .claim_map
        .items
        .iter()
        .find(|claim| matches!(claim.result, ClaimDisposition::Generated(_)))
        .expect("healthy sibling remains generated");
    assert_eq!(&generated.result, &baseline.claim_map.items[0].result);
    let refused = mixed
        .claim_map
        .items
        .iter()
        .find(|claim| matches!(claim.result, ClaimDisposition::Refused { .. }))
        .expect("mistyped descriptor refuses");
    assert!(matches!(
        &refused.result,
        ClaimDisposition::Refused {
            refusal: CompositeEqualityRefusal::OperandTypeMismatch {
                position: 0,
                expected,
                found: Some(found)
            }
        } if expected == &boolean && found == left
    ));
}

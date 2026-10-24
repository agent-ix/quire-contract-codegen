//! FR-018-AC-2, AC-8 (the `check_type` guard half), AC-9 and AC-11 (the
//! complementary-outcomes half): generated composite-equality oracles agree
//! with an independently assembled direct Contract Runtime call, across
//! record, tuple, option, collection and recursive shapes; a malformed
//! environment is refused with no charge; a denied charge surfaces as
//! `Outcome::Incomplete`, never a completed Boolean; and the two operator
//! variants over one expression node produce complementary outcomes.
//!
//! Nothing generated is committed. This module generates the corpus crate now
//! and runs all original vectors from `tests/composite_equality_support/cases/`
//! against their own genuine source contexts through the scratch-crate runner.

use ix_trace_rs::trace;

use super::exact_scalar_agreement::run_agreement_cases;

/// The crate the generator emits now for the corpus request compiles and every
/// agreement case passes against it. Each case drives the generated oracle and
/// a direct runtime call assembled from the request's own descriptor, so an
/// emitted operator, operand order or conversion that disagrees with the
/// descriptor fails here.
#[trace(
    "FR-018-AC-2",
    "FR-018-AC-8",
    "FR-018-AC-9",
    "FR-018-AC-10",
    "FR-018-AC-11",
    "TC-029"
)]
#[test]
fn tc_029_generated_composite_equality_crate_agrees_with_direct_runtime() {
    for case in fixture::CASES {
        let compiled = fixture::compile(case);
        let package = super::admitted_enum_equality::checked(&compiled, &[]);
        let mut item = original_item(&package, case.starts_with("conv"));
        let mut items = vec![item.clone()];
        if *case == "record" {
            item.operator = quire_contract_codegen::EqualityOperatorKind::NotEqual;
            items.push(item);
        }
        let oracles = super::admitted_enum_equality::generate_items(
            &package,
            Some(compiled.context()),
            &items,
            &[],
        )
        .expect("authentic original agreement package");
        assert_eq!(oracles.claim_map.items.len(), items.len(), "{case}");
        let aliases: &[(&str, quire_contract_codegen::EqualityOperatorKind)] = match *case {
            "record" => &[
                (
                    "e_record_equal",
                    quire_contract_codegen::EqualityOperatorKind::Equal,
                ),
                (
                    "e_record_not_equal",
                    quire_contract_codegen::EqualityOperatorKind::NotEqual,
                ),
            ],
            "tuple" => &[(
                "e_tuple_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "option" => &[(
                "e_option_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "collection" => &[(
                "e_collection_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "recursive" => &[(
                "e_self_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "nested" => &[(
                "e_pair_of_points_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "text" => &[(
                "e_text_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "enum" => &[(
                "e_enum_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "conv" => &[(
                "e_conv_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "conv_charge" => &[(
                "e_conv_charge_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "conv_rat_rat" => &[(
                "e_conv_rat_rat_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "conv_rat_int" => &[(
                "e_conv_rat_int_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "conv_dec_rat" => &[(
                "e_conv_dec_rat_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "conv_dec_dec" => &[(
                "e_conv_dec_dec_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            "conv_dec_int" => &[(
                "e_conv_dec_int_equal",
                quire_contract_codegen::EqualityOperatorKind::Equal,
            )],
            _ => panic!("the finite source case set is closed"),
        };
        let mut names = String::new();
        for (alias, operator) in aliases {
            let identity = match operator {
                quire_contract_codegen::EqualityOperatorKind::Equal => "equality.equal",
                quire_contract_codegen::EqualityOperatorKind::NotEqual => "equality.not_equal",
            };
            let claim = oracles
                .claim_map
                .items
                .iter()
                .find(|claim| claim.operation.identity == identity)
                .expect("original operation request");
            let quire_contract_codegen::ClaimDisposition::Generated(generated) = &claim.result
            else {
                panic!("{case} source refused: {:?}", claim.result)
            };
            names.push_str(&format!(
                "pub use self::{{{} as environment_{alias}, {} as oracle_{alias}}};\n",
                generated.environment_symbol, generated.oracle_symbol
            ));
        }
        run_agreement_cases(
            &format!("composite-equality-agreement-{case}"),
            oracles
                .artifacts
                .iter()
                .map(|artifact| (artifact.path.as_str(), artifact.contents.as_str())),
            &[("names.rs", names.as_str()), ("case-mode", case)],
            &format!("tests/composite_equality_support/cases/{case}.rs"),
        );
    }
}

#[path = "../composite_equality_support/authentic_native.rs"]
pub(super) mod fixture;

pub(super) fn original_item(
    package: &quire_contract_model::CheckedPackageV2,
    converted: bool,
) -> quire_contract_codegen::CompositeEqualityItem {
    use quire_contract_codegen::{
        CompositeEqualityItem, EqualityOperandDescriptor, EqualityOperatorKind,
    };
    use quire_contract_model::{ApplicationOperator, CheckedTerm};
    let node_id = super::admitted_enum_equality::application(package, "f");
    let expression = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == node_id)
        .expect("original equality");
    let CheckedTerm::Application { arguments, .. } = &expression.body else {
        panic!("original equality application");
    };
    assert_eq!(arguments.len(), 2);
    let source_type = |term: &CheckedTerm| {
        let CheckedTerm::Reference { target } = term else {
            panic!("original operand reference");
        };
        let mut target = target.clone();
        let mut visited = std::collections::BTreeSet::new();
        loop {
            assert!(
                visited.insert(target.clone()),
                "original conversion path is acyclic"
            );
            let node = package
                .graph()
                .nodes
                .iter()
                .find(|node| node.node_id == target)
                .expect("original operand node");
            let CheckedTerm::Application {
                operator: ApplicationOperator::Convert,
                arguments,
                ..
            } = &node.body
            else {
                break node.semantic_type.clone();
            };
            if node.node_tag.as_ref() != "expression" {
                break node.semantic_type.clone();
            }
            let Some(CheckedTerm::Reference { target: next }) = arguments.first() else {
                panic!("original conversion operand reference");
            };
            target = next.clone();
        }
    };
    let left = source_type(&arguments[0]);
    let right = source_type(&arguments[1]);
    CompositeEqualityItem {
        node_id,
        operator: EqualityOperatorKind::Equal,
        left: if converted {
            EqualityOperandDescriptor::converted(left, right.clone())
        } else {
            EqualityOperandDescriptor::typed(left)
        },
        right: EqualityOperandDescriptor::typed(right),
    }
}

// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exact scalar generation over the original QSpec model variant comparison fixture.

#[path = "../model_enum_support/fixture.rs"]
mod fixture;

use ix_trace_rs::trace;
use quire_contract_codegen::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, ClaimDisposition,
    ExactScalarOperation, EXACT_SCALAR_CRATE_NAME,
};
use quire_exact::ComparisonOperator;
use quire_specification::{ACME_COLORS_COMPARISON_SOURCE, ACME_COLORS_DOMAIN_PACKAGE};

#[trace("TC-024", "FR-014-AC-2")]
#[trace("TC-024", "FR-014-AC-6")]
#[test]
fn original_model_enum_eq_ne_generate_and_execute_all_four_truth_arms() {
    let (compiled, package) =
        fixture::compile(ACME_COLORS_COMPARISON_SOURCE, ACME_COLORS_DOMAIN_PACKAGE);
    assert_eq!(fixture::colors(&compiled, &package).len(), 2);
    let functions = [
        "redEqualsRed",
        "redEqualsGreen",
        "redDiffersFromRed",
        "redDiffersFromGreen",
    ];
    let nodes = functions.map(|function| fixture::equality_application(&package, function));
    let items = derive_exact_scalar_items(&package, &nodes)
        .into_iter()
        .zip([
            ComparisonOperator::Equal,
            ComparisonOperator::Equal,
            ComparisonOperator::NotEqual,
            ComparisonOperator::NotEqual,
        ])
        .map(|(item, operator)| {
            let item = item.expect("an admitted original model variant comparison derives");
            assert_eq!(
                item.operation,
                ExactScalarOperation::EnumComparison { operator }
            );
            item
        })
        .collect::<Vec<_>>();
    let oracles = generate_exact_scalar_oracles(&package, &items)
        .expect("original model variant comparisons generate");
    let aliases = nodes
        .iter()
        .zip(functions)
        .map(|(node, name)| {
            let claim = oracles
                .claim_map
                .items
                .iter()
                .find(|claim| &claim.node_id == node)
                .expect("the exact requested application has a claim");
            let ClaimDisposition::Generated(generated) = &claim.result else {
                panic!("model enum comparison refused: {claim:?}");
            };
            (generated.symbol.clone(), name.to_owned())
        })
        .collect::<Vec<_>>();
    let names = super::exact_scalar_agreement::names_file(EXACT_SCALAR_CRATE_NAME, &aliases, "");
    // The agreement fixture consumes the native reader; it is not part of the emitted oracle.
    let artifacts = oracles.artifacts.iter().map(|artifact| {
        let contents = if artifact.path == "Cargo.toml" {
            artifact.contents.replacen("[dependencies]\n", concat!(
                "[dependencies]\n",
                "quire-contract-model = { git = \"https://github.com/agent-ix/quire-contract-ir\", branch = \"main\" }\n"
            ), 1)
        } else { artifact.contents.clone() };
        (artifact.path.clone(), contents)
    }).collect::<Vec<_>>();
    super::exact_scalar_agreement::run_agreement_cases(
        "original-model-enum-comparisons",
        artifacts
            .iter()
            .map(|(path, contents)| (path.as_str(), contents.as_str())),
        &[
            ("names.rs", &names),
            ("colors.native", ACME_COLORS_COMPARISON_SOURCE),
            ("colors.json", ACME_COLORS_DOMAIN_PACKAGE),
        ],
        "tests/model_enum_support/agreement_cases.rs",
    );
}

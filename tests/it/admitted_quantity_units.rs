// SPDX-License-Identifier: AGPL-3.0-or-later
//! Genuine quantity owner sources and the retained quantity refusal boundary.

use super::admitted_enum_equality::{checked, direct_source_item, generate_items};
use ix_trace_rs::trace;
use qsl_replay::{
    compile_admitted, AdmittedCompilation, DependencyInput, ReplayLimits, SourceIdentity,
    StageLimits,
};
use quire_contract_codegen::{ClaimDisposition, CompositeEqualityRefusal};

const SOURCE: &str = r#"
language "ix:native" edition "1-draft";
profile v = "quire.value.complete/v1";
dimension Length;
unit m : Length = rational(1, 1);
unit km : Length = rational(1000, 1) * m;
union Choice { Mark(Boolean), Empty }
union OtherChoice { Mark(Boolean), Empty }
union WithQuantity { Sample(Quantity<m>), Empty }
function same_m using v(a: Quantity<m>, b: Quantity<m>): Boolean pure { a = b }
function same_km using v(a: Quantity<km>, b: Quantity<km>): Boolean pure { a = b }
function same_choice using v(a: Choice, b: Choice): Boolean pure { a = b }
function same_other using v(a: OtherChoice, b: OtherChoice): Boolean pure { a = b }
function same_nested_quantity using v(a: WithQuantity, b: WithQuantity): Boolean pure { a = b }
function healthy using v(a: Int[0,9], b: Int[0,9]): Boolean pure { a = b }
"#;

fn compile() -> AdmittedCompilation {
    compile_admitted(
        SourceIdentity::new("agent-ix", "quantity-owner-source", "git", "1"),
        "quantity-owner.native",
        SOURCE.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("genuine quantity and union source must compile without a quantity literal")
}

#[trace("TC-029", "FR-018-AC-18", "FR-018-AC-38")]
#[test]
fn genuine_quantity_items_keep_their_refusal_and_healthy_sibling() {
    let compiled = compile();
    let package = checked(&compiled, &[]);
    let items =
        ["same_m", "same_km", "healthy"].map(|function| direct_source_item(&package, function));
    for context in [None, Some(compiled.context())] {
        let generated = generate_items(&package, context, &items, &[])
            .expect("quantity refusal stays per-item");
        for item in &items[..2] {
            let claim = generated
                .claim_map
                .items
                .iter()
                .find(|claim| claim.node_id == item.node_id)
                .expect("the quantity item is retained in the claim map");
            assert!(
                matches!(
                    &claim.result,
                    ClaimDisposition::Refused {
                        refusal: CompositeEqualityRefusal::Unsupported {
                            node_tag: "quantity",
                            ..
                        }
                    }
                ),
                "{:?}",
                claim.result
            );
        }
        let healthy = generated
            .claim_map
            .items
            .iter()
            .find(|claim| claim.node_id == items[2].node_id)
            .expect("the healthy sibling is retained");
        assert!(matches!(&healthy.result, ClaimDisposition::Generated(_)));
    }
}

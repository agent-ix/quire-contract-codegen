//! FR-022 per-occurrence requirement records (IR-304 slice (a)).

use ix_trace_rs::trace;
use quire_contract_codegen::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, negotiate_kani_obligations,
    ContractDriverResultSelection, ContractRequestPosition, KaniObligationOutcome,
    KaniObligationRequest, ObligationItem,
};

use crate::kani_obligations::package;
use crate::routed_generation::{admitted_routing_input, route};

/// The persisted record of the harness a direct FR-015 caller gets for `x + 1` over
/// `x: Int[0, 9]` into `Int[0, 10]`.
fn direct_increment_record() -> String {
    let mut builder = package::qsl_shaped_package();
    builder.expression_occurrences();
    let (checked, ids) = builder.admit_resolved();
    let node_id = ids.resolve(&package::code_id(package::QSL_INC));
    let routed = [route(0, &node_id)];
    let admitted = admitted_routing_input(&checked, &routed);
    let item = derive_exact_scalar_items(&checked, std::slice::from_ref(&node_id))
        .into_iter()
        .next()
        .expect("one requested node is derived")
        .expect("the increment has a source-derived descriptor");
    let claim_map = generate_exact_scalar_oracles(&checked, &[item])
        .expect("the increment renders")
        .claim_map;
    let selection: &ContractDriverResultSelection = admitted
        .positions
        .iter()
        .find_map(|position| match position {
            ContractRequestPosition::DriverItem {
                result_selection, ..
            } => Some(result_selection),
            _ => None,
        })
        .expect("a driver position");
    let items = [ObligationItem::ScalarClaim {
        source: &checked,
        package: &admitted.package,
        claim_map: &claim_map,
        node_id: &node_id,
        claim_index: 0,
        result_selection: selection,
    }];
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items: &items,
        subject_path: "crate::subject",
        unwind: 1,
    })
    .expect("FR-015 accepts the request");
    let KaniObligationOutcome::Emitted {
        scalar_harnesses, ..
    } = outcome
    else {
        panic!("the request is rejected");
    };
    let [harness] = scalar_harnesses.as_slice() else {
        panic!("one harness, got {scalar_harnesses:?}");
    };
    harness.record.contents.clone()
}

#[trace("FR-022-AC-31", "TC-033")]
#[test]
fn direct_increment_record_equals_the_committed_golden() {
    let golden = include_str!("../fixtures/kani-scalar/direct-increment-record.json");
    assert_eq!(direct_increment_record(), golden);
}

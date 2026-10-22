//! Authentic source admission helpers; no prebaked graph or minted node identities.

use quire_contract_model::{CheckedNodeId, ContractPackageV1};

pub(super) const HEALTHY: &str = include_str!("fixtures/commutative.native");
pub(super) const MUTANT: &str = include_str!("fixtures/wrong_addend.native");

pub(super) fn admit(source: &str) -> (ContractPackageV1, CheckedNodeId) {
    use qsl_replay::{
        compile_admitted, DependencyInput, ReplayLimits, SourceIdentity, StageLimits,
    };
    use quire_contract_model::{
        CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
        CheckedPackageV2ReadResult,
    };
    let admitted = compile_admitted(
        SourceIdentity::new("a", "u", "git", "1"),
        "rational-predicate",
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("authentic QSL source admits");
    let wire: serde_json::Value = serde_json::from_slice(admitted.package().bytes()).unwrap();
    let mut evidence = CheckedPackageEvidence::new();
    for feature in wire["lock"]["required_features"].as_array().unwrap() {
        evidence.support_feature(feature.as_str().unwrap());
    }
    let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
        admitted.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) else {
        panic!("strict owner reader must admit the source output");
    };
    let function = package
        .graph()
        .nodes
        .iter()
        .find(|n| n.node_tag.as_ref() == "function")
        .unwrap()
        .node_id
        .clone();
    let lowered = package.lower(
        &[function.clone()],
        &quire_contract_codegen::routed_contract_lowering_profile(),
    );
    (
        lowered
            .package
            .wire()
            .expect("owner function closure lowers")
            .clone(),
        function,
    )
}

//! The byte-ceiling lowering fixtures of FR-014-AC-40, FR-018-AC-20 and FR-021-AC-23.
//!
//! Contract IR fails every requested record for `bytes` when the canonical length of the lowered
//! contract package of a call exceeds the ceiling the checked package was read under. A package
//! read under such a ceiling must still be admitted, so each fixture measures both lengths and
//! uses a request set whose per-request accounting makes the lowered package longer. Contract IR
//! V1 represents each reached graph node only once even when a record has a large closure.

use std::collections::BTreeSet;

use quire_contract_model::{
    CheckedNodeId, CheckedNodeTag, CheckedPackageReadLimits, CheckedPackageV2,
    CompleteLoweringProfileV2, CompleteLoweringRecordV2,
};

/// The largest ceiling a fixture is read under, far above any fixture's lowered package.
pub const LARGEST_CEILING: u64 = 16 * 1024 * 1024;

/// The limits a fixture is read under with `bytes` as its byte ceiling.
pub fn limits_under(bytes: u64) -> CheckedPackageReadLimits {
    CheckedPackageReadLimits {
        bytes,
        ..CheckedPackageReadLimits::bounded()
    }
}

/// The lowering profile of the generators that read a V2 package without requiring bounds:
/// the tags Contract IR lowers for an expression, a type or a value. The fixtures' nodes are
/// expressions over values and scalar types, which every generator's own profile lowers alike,
/// so the length measured here is the length of the call the generator itself makes; the
/// fixtures then assert the ceiling one byte below it refuses and the length itself does not.
pub fn measuring_profile(require_bounds: bool, identity: &str) -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
        reference: quire_contract_model::LoweringProfileRef {
            authority: "agent-ix".into(),
            identity: identity.into(),
        },
        supported_tags: BTreeSet::from([
            CheckedNodeTag::ScalarType,
            CheckedNodeTag::CompositeType,
            CheckedNodeTag::BoundedDomain,
            CheckedNodeTag::Value,
            CheckedNodeTag::Expression,
            CheckedNodeTag::Claim,
            CheckedNodeTag::Correspondence,
        ]),
        require_bounds,
        work_limit: 65_536,
    }
}

/// The canonical length of the lowered contract package Contract IR returns for `requested`
/// from `package`, which must have been read under a ceiling the lowered package fits.
pub fn lowered_package_length(
    package: &CheckedPackageV2,
    requested: &[CheckedNodeId],
    profile: &CompleteLoweringProfileV2,
) -> u64 {
    let lowering = package.lower(requested, profile);
    assert_eq!(
        lowering.records.len(),
        requested.len(),
        "one lowering record per requested node"
    );
    let lowered_count = lowering
        .records
        .iter()
        .filter(|record| matches!(record, CompleteLoweringRecordV2::Lowered { .. }))
        .count();
    assert_eq!(
        lowered_count,
        requested.len(),
        "only {lowered_count}/{} requested nodes lowered; first non-lowered record: {:?}",
        requested.len(),
        lowering
            .records
            .iter()
            .find(|record| !matches!(record, CompleteLoweringRecordV2::Lowered { .. }))
    );
    let package_nodes = lowering
        .package
        .lowered()
        .iter()
        .map(|node| &node.node.node_id)
        .chain(
            lowering
                .package
                .dependencies()
                .iter()
                .map(|dependency| &dependency.node.node_id),
        )
        .collect::<BTreeSet<_>>();
    for (requested_id, record) in requested.iter().zip(&lowering.records) {
        let CompleteLoweringRecordV2::Lowered { node } = record else {
            unreachable!("all records were checked as lowered");
        };
        assert_eq!(&node.node.node_id, requested_id);
        assert!(
            package_nodes.contains(requested_id),
            "requested node absent from lowered package"
        );
        assert!(
            node.dependencies
                .iter()
                .all(|dependency| package_nodes.contains(dependency)),
            "lowered node closure is incomplete for {requested_id:?}"
        );
    }
    let bytes = lowering
        .package
        .canonical_bytes()
        .expect("the lowered package fits the ceiling the package was read under");
    u64::try_from(bytes.len()).expect("a length fits u64")
}

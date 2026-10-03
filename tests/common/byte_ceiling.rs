//! The byte-ceiling lowering fixtures of FR-014-AC-40, FR-018-AC-20 and FR-021-AC-23.
//!
//! Contract IR fails every requested record for `bytes` when the canonical length of the lowered
//! contract package of a call exceeds the ceiling the checked package was read under. A package
//! read under such a ceiling must still be admitted, so the fixtures need a lowered package that
//! is longer than the checked package: a chain of nodes each reaching the one before lists its
//! whole closure in its `dependencies`, which grows quadratically while the checked package
//! grows linearly.

use std::collections::BTreeSet;

use quire_contract_model::{
    CheckedNodeId, CheckedNodeTag, CheckedPackageReadLimits, CheckedPackageV2,
    CompleteLoweringProfileV2,
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
pub fn measuring_profile(require_bounds: bool) -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
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
    let bytes = lowering
        .package
        .canonical_bytes()
        .expect("the lowered package fits the ceiling the package was read under");
    u64::try_from(bytes.len()).expect("a length fits u64")
}

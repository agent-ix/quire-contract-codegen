//! Reader admission for fixtures whose producers assign structural node IDs.
//!
//! The fixture builder keeps the same IDs in the authored wire and in expected
//! values. A body edit does not change a node's source position or identity.

use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult,
};
use serde_json::Value;

/// The exact wire submitted to the reader and its admission result.
pub struct ResolvedWire {
    pub wire: Value,
    pub outcome: CheckedPackageV2ReadResult,
    pub ids: FixtureIds,
}

/// Producer-assigned fixture IDs need no post-read substitution.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FixtureIds;

impl FixtureIds {
    /// Keep an expected node ID identical to the producer's graph ID.
    pub fn resolve(&self, node: &CheckedNodeId) -> CheckedNodeId {
        node.clone()
    }

    /// Expected typed references already use the producer's graph IDs.
    pub fn resolve_value(&self, _value: &mut Value) {}
}

/// Structural package identity is its name and exact lock; body changes do not rekey it.
pub(super) fn refresh_identity(wire: &mut Value) {
    assert!(wire["name"].is_string(), "fixture package needs a name");
    assert!(wire["lock"].is_object(), "fixture package needs a lock");
}

/// Submit the producer's wire unchanged so the reader is the admission authority.
pub fn resolve_wire(
    raw: &Value,
    evidence: &CheckedPackageEvidence,
    limits: CheckedPackageReadLimits,
) -> ResolvedWire {
    let wire = raw.clone();
    let bytes = serde_json::to_vec(&wire).expect("fixture wire serializes");
    let outcome = CheckedPackageV2::read(&bytes, limits, evidence);
    ResolvedWire {
        wire,
        outcome,
        ids: FixtureIds,
    }
}

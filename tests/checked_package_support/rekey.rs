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

/// Author empty operand rows for name-only fixture nodes, then let the reader admit the wire.
/// Existing operand rows and all producer identities remain unchanged.
pub fn resolve_wire(
    raw: &Value,
    evidence: &CheckedPackageEvidence,
    limits: CheckedPackageReadLimits,
) -> ResolvedWire {
    let mut wire = raw.clone();
    author_name_only_operand_rows(&mut wire);
    let bytes = serde_json::to_vec(&wire).expect("fixture wire serializes");
    let outcome = CheckedPackageV2::read(&bytes, limits, evidence);
    ResolvedWire {
        wire,
        outcome,
        ids: FixtureIds,
    }
}

/// Supply the required empty rows for this producer's name-only fixture graph.
/// Existing rows, slot identities and document-index identities are never rewritten.
pub fn author_name_only_operand_rows(wire: &mut Value) {
    if let Some(graph) = wire
        .get_mut("semantic_graph")
        .and_then(Value::as_object_mut)
    {
        if !graph.contains_key("operand_rows") {
            if let Some(nodes) = graph.get("nodes").and_then(Value::as_array) {
                if nodes.iter().all(|node| {
                    node["node_id"]["path"]
                        .as_array()
                        .is_some_and(|path| path.iter().all(Value::is_string))
                }) {
                    let rows = vec![Vec::<u64>::new(); nodes.len()];
                    graph.insert("operand_rows".to_owned(), serde_json::json!(rows));
                }
            }
        }
    }
}

//! Test-fixture key repair driven only by the checked reader's typed expected key.
//!
//! A package builder retains its readable source IDs. This module repairs a copy of its wire,
//! so building another fixture cannot change IDs observed by a parallel test. The reader remains
//! the sole authority for structural and application key derivation.

use std::collections::{BTreeMap, BTreeSet};

use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageRefusalCause,
    CheckedPackageV2, CheckedPackageV2ReadResult,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MAX_CUMULATIVE_BYTES: u64 = 1_024 * 1_048_576;
const MAX_READER_WORK: u64 = 256_000_000;
const NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";

/// The final wire, the reader outcome at that wire, and its fixture-local ID substitutions.
pub struct ResolvedWire {
    pub wire: Value,
    pub outcome: CheckedPackageV2ReadResult,
    pub ids: FixtureIds,
}

/// Maps a fixture's readable node IDs to the identities the reader required.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FixtureIds(BTreeMap<String, String>);

impl FixtureIds {
    /// Resolves an already-constructed test item against this one package.
    pub fn resolve(&self, node: &CheckedNodeId) -> CheckedNodeId {
        CheckedNodeId {
            domain: node.domain.clone(),
            digest: self.digest(&node.digest).into_boxed_str(),
        }
    }

    fn digest(&self, source: &str) -> String {
        let mut current = source;
        let mut visited = BTreeSet::new();
        while let Some(next) = self.0.get(current) {
            assert!(
                visited.insert(current),
                "fixture key substitution cycle at {current}"
            );
            current = next;
        }
        current.to_owned()
    }

    fn insert(&mut self, old: &CheckedNodeId, expected: &CheckedNodeId) {
        assert_eq!(old.domain, expected.domain, "fixture key domain changed");
        assert_ne!(old, expected, "stale-key refusal made no progress");
        let old_digest = old.digest.to_string();
        let expected_digest = expected.digest.to_string();
        assert!(
            !self.0.contains_key(&old_digest),
            "reader repeated a stale key for {old_digest}"
        );
        self.0.insert(old_digest.clone(), expected_digest);
        let resolved = self.digest(&old_digest);
        assert_ne!(resolved, old_digest, "fixture key substitution cycle");
    }
}

/// Rebuilds projection and package identity after a node-ID substitution.
fn refresh_identity(wire: &mut Value) {
    let projection = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture graph nodes")
        .iter()
        .cloned()
        .map(|mut node| {
            node.as_object_mut()
                .expect("fixture node")
                .remove("occurrences");
            node
        })
        .collect();
    wire["identity_preimage"]["identity_projection"] = Value::Array(projection);
    let preimage = serde_json::to_vec(&wire["identity_preimage"]).expect("fixture preimage");
    wire["package_id"]["digest"] = json!(format!("{:x}", Sha256::digest(preimage)));
}

/// Replaces only typed node references. An unrelated literal or declaration string can equal a
/// node digest byte for byte, but it is still semantic data and must not change with the key.
fn substitute(value: &mut Value, ids: &FixtureIds) {
    match value {
        Value::Array(items) => {
            for item in items {
                substitute(item, ids);
            }
        }
        Value::Object(object) => {
            if object.get("domain").and_then(Value::as_str) == Some(NODE_DOMAIN) {
                let digest = object
                    .get("digest")
                    .and_then(Value::as_str)
                    .expect("typed fixture node reference has a digest");
                let resolved = ids.digest(digest);
                object["digest"] = json!(resolved);
                return;
            }
            for member in object.values_mut() {
                substitute(member, ids);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

/// Application dependencies are in key order. A substituted key can cross another key in
/// that order, so refresh only this derived list after a substitution. Preserve duplicate
/// references so the reader can refuse them. The first attempt keeps the builder's original bytes
/// and negative fixtures.
fn reorder_application_dependencies(wire: &mut Value) {
    fn contains_application(value: &Value) -> bool {
        match value {
            Value::Array(items) => items.iter().any(contains_application),
            Value::Object(object) => {
                object.get("term").and_then(Value::as_str) == Some("application")
                    || object.values().any(contains_application)
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => false,
        }
    }

    for node in wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("fixture graph nodes")
    {
        if !contains_application(&node["body"]) {
            continue;
        }
        let dependencies = node["dependencies"].as_array_mut().expect("dependencies");
        let parsed = dependencies
            .iter()
            .cloned()
            .map(serde_json::from_value::<CheckedNodeId>)
            .collect::<Result<Vec<_>, _>>();
        if let Ok(mut parsed) = parsed {
            parsed.sort();
            *dependencies = parsed
                .into_iter()
                .map(|id| serde_json::to_value(id).expect("node id serializes"))
                .collect();
        }
    }
}

/// Repairs only stale keys for which IR retained an expected key. Every other refusal, including
/// a stale key with no derivable expected key, is returned unchanged for the caller to assert.
/// The finite reader is charged at most twice per source node and under cumulative byte/work
/// ceilings.
pub fn resolve_wire(
    raw: &Value,
    evidence: &CheckedPackageEvidence,
    limits: CheckedPackageReadLimits,
) -> ResolvedWire {
    let node_count = raw["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture graph nodes")
        .len();
    let attempts = node_count
        .checked_mul(2)
        .expect("fixture node-count cap")
        .max(1);
    let mut ids = FixtureIds::default();
    let mut seen = BTreeSet::new();
    let mut bytes_charged = 0u64;
    let mut work_charged = 0u64;
    for _ in 0..attempts {
        let mut wire = raw.clone();
        substitute(&mut wire, &ids);
        if !ids.0.is_empty() {
            reorder_application_dependencies(&mut wire);
        }
        refresh_identity(&mut wire);
        let bytes = serde_json::to_vec(&wire).expect("fixture wire");
        let bytes_len = u64::try_from(bytes.len()).expect("fixture bytes fit u64");
        bytes_charged = bytes_charged
            .checked_add(bytes_len)
            .expect("fixture byte counter");
        work_charged = work_charged
            .checked_add(limits.work)
            .expect("fixture work counter");
        assert!(
            bytes_charged <= MAX_CUMULATIVE_BYTES && work_charged <= MAX_READER_WORK,
            "fixture key repair exceeded its byte/work cap"
        );
        assert!(
            seen.insert(bytes.clone()),
            "fixture key repair made no progress"
        );
        let outcome = CheckedPackageV2::read(&bytes, limits, evidence);
        if let CheckedPackageV2ReadResult::Refused(refusal) = &outcome {
            if refusal.cause == Some(CheckedPackageRefusalCause::StaleNodeKey) {
                if let (Some(old), Some(expected)) = (&refusal.locus, refusal.expected_node_id()) {
                    ids.insert(old, expected);
                    continue;
                }
            }
        }
        return ResolvedWire { wire, outcome, ids };
    }
    panic!("fixture key repair exceeded twice its node count")
}

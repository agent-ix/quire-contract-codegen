//! Test-fixture key repair driven by Contract IR's public candidate API and reader refusals.
//!
//! A package builder retains its readable source IDs. This module repairs a copy of its wire,
//! so building another fixture cannot change IDs observed by a parallel test. The reader remains
//! the final authority for admission.

use std::collections::{BTreeMap, BTreeSet};

use quire_contract_model::{
    derive_application_node_id, derive_structural_candidates, ApplicationNodeDraft, CheckedNodeId,
    CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageRefusalCause, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedSemanticNodeV2,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MAX_CUMULATIVE_BYTES: u64 = 1_024 * 1_048_576;
const MAX_READER_WORK: u64 = 256_000_000;
const NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";
const APPLICATION_NODE_VERSION: &str = "quire.application-node/v1";

/// The final wire, the reader outcome at that wire, and its fixture-local ID substitutions.
pub struct ResolvedWire {
    pub wire: Value,
    pub outcome: CheckedPackageV2ReadResult,
    pub ids: FixtureIds,
}

/// Maps a fixture's readable node IDs to the identities the reader required.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FixtureIds(BTreeMap<String, String>);

fn structural_candidates(
    raw: &Value,
    limits: CheckedPackageReadLimits,
) -> Option<(FixtureIds, BTreeMap<String, String>)> {
    let raw_nodes: Vec<CheckedSemanticNodeV2> =
        serde_json::from_value(raw["semantic_graph"]["nodes"].clone()).ok()?;
    let mut ids = FixtureIds::default();
    for node in &raw_nodes {
        if node.node_tag.as_ref() == "scalar_type"
            && matches!(node.semantic_form.as_ref(), "boolean" | "integer")
        {
            let candidate =
                derive_structural_candidates(std::slice::from_ref(node), limits.bytes, limits.work)
                    .ok()?;
            if let Some(scalar) = candidate.nodes().first() {
                if &node.node_id != scalar.candidate_node_id() {
                    ids.insert(&node.node_id, scalar.candidate_node_id());
                }
            }
        }
    }
    let mut graph = raw["semantic_graph"]["nodes"].clone();
    substitute(&mut graph, &ids);
    let nodes: Vec<CheckedSemanticNodeV2> = serde_json::from_value(graph).ok()?;
    // Malformed negative fixtures have no structural candidates. Let the
    // reader report their exact refusal after the other fixture keys settle.
    let candidates = derive_structural_candidates(&nodes, limits.bytes, limits.work).ok()?;
    for candidate in candidates.nodes() {
        let old =
            &nodes[usize::try_from(candidate.position()).expect("fixture node position")].node_id;
        let expected = candidate.candidate_node_id();
        if old != expected {
            ids.insert(old, expected);
        }
    }
    let groups = candidates
        .groups()
        .iter()
        .map(|group| {
            (
                group.retained_label().to_owned(),
                group.candidate_label().to_owned(),
            )
        })
        .collect();
    Some((ids, groups))
}

/// Settle application candidates, including references to other applications, after
/// structural references are repaired.
fn application_candidates(raw: &Value, ids: &mut FixtureIds, limits: CheckedPackageReadLimits) {
    let node_count = raw["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture graph nodes")
        .len();
    // An application can reference another application. Re-derive against the
    // newly substituted graph until those dependencies settle, using only the
    // public pre-read candidate API. The bounded reader loop remains the final
    // authority for malformed and semantically refused fixtures.
    for _ in 0..node_count {
        let mut graph = raw["semantic_graph"]["nodes"].clone();
        substitute(&mut graph, ids);
        let Ok(nodes) = serde_json::from_value::<Vec<CheckedSemanticNodeV2>>(graph) else {
            return;
        };
        let mut changed = false;
        for node in &nodes {
            let raw_body = serde_json::to_value(&node.body).expect("fixture body serializes");
            if raw_body["term"] != "application" {
                continue;
            }
            let Ok(candidate) =
                derive_application_node_id(ApplicationNodeDraft::new(node), None, limits.bytes)
            else {
                continue;
            };
            if &node.node_id != candidate.candidate_node_id() {
                ids.insert(&node.node_id, candidate.candidate_node_id());
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

impl FixtureIds {
    /// Resolves an already-constructed test item against this one package.
    pub fn resolve(&self, node: &CheckedNodeId) -> CheckedNodeId {
        CheckedNodeId {
            domain: node.domain.clone(),
            digest: self.digest(&node.digest).into_boxed_str(),
        }
    }

    /// Resolve typed node references in a test-authored expected value.
    pub fn resolve_value(&self, value: &mut Value) {
        substitute(value, self);
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
pub(super) fn refresh_identity(wire: &mut Value) {
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

/// Install a public pre-read candidate into every typed reference in a fixture wire.
pub(super) fn install_candidate(
    wire: &mut Value,
    provisional: &CheckedNodeId,
    candidate: &CheckedNodeId,
) {
    let mut ids = FixtureIds::default();
    ids.insert(provisional, candidate);
    substitute(wire, &ids);
    reorder_application_dependencies(wire);
    refresh_identity(wire);
}

/// Replaces only typed node references. An unrelated literal or declaration string can equal a
/// node digest byte for byte, but it is still semantic data and must not change with the key.
fn substitute(value: &mut Value, ids: &FixtureIds) {
    substitute_mapping(value, &ids.0);
}

fn substitute_mapping(value: &mut Value, ids: &BTreeMap<String, String>) {
    match value {
        Value::Array(items) => {
            for item in items {
                substitute_mapping(item, ids);
            }
        }
        Value::Object(object) => {
            if object.get("domain").and_then(Value::as_str) == Some(NODE_DOMAIN) {
                let digest = object
                    .get("digest")
                    .and_then(Value::as_str)
                    .expect("typed fixture node reference has a digest");
                let mut resolved = digest;
                let mut visited = BTreeSet::new();
                while let Some(next) = ids.get(resolved) {
                    assert!(visited.insert(resolved), "fixture key substitution cycle");
                    resolved = next.as_str();
                }
                object["digest"] = json!(resolved);
                return;
            }
            for member in object.values_mut() {
                substitute_mapping(member, ids);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

/// Re-derives every application key from the currently substituted wire and propagates all
/// changed candidates through references until the application subgraph reaches a fixed point.
/// Non-application IDs may change on a later reader attempt; the caller repeats this pass after
/// applying those accumulated reader-authorized substitutions.
fn repair_application_candidates(wire: &mut Value, byte_limit: u64, ids: &mut FixtureIds) -> usize {
    let mut candidate_ids = BTreeMap::new();
    let nodes = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture graph nodes")
        .len();
    for round in 0..nodes.saturating_mul(2).max(1) {
        let updates = application_key_updates(wire, byte_limit);
        if updates.is_empty() {
            return round;
        }
        for (old, new) in updates {
            if old != new {
                ids.insert(
                    &CheckedNodeId {
                        domain: NODE_DOMAIN.into(),
                        digest: old.clone().into_boxed_str(),
                    },
                    &CheckedNodeId {
                        domain: NODE_DOMAIN.into(),
                        digest: new.clone().into_boxed_str(),
                    },
                );
                if let Some(previous) = candidate_ids.insert(old.clone(), new.clone()) {
                    assert_eq!(previous, new, "application key candidate changed in place");
                }
            }
        }
        substitute_mapping(wire, &candidate_ids);
        reorder_application_dependencies(wire);
    }
    panic!("application-key candidate repair did not reach a fixed point")
}

fn application_key_updates(wire: &Value, byte_limit: u64) -> Vec<(String, String)> {
    let nodes = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture graph nodes");
    let mut groups: BTreeMap<&str, Vec<CheckedNodeId>> = BTreeMap::new();
    for node in nodes {
        if let Some(label) = node.get("recursion_group").and_then(Value::as_str) {
            groups
                .entry(label)
                .or_default()
                .push(serde_json::from_value(node["node_id"].clone()).expect("group node id"));
        }
    }

    let mut updates = Vec::new();
    for node in nodes {
        if node["body"].get("term").and_then(Value::as_str) != Some("application") {
            continue;
        }
        let group = node
            .get("recursion_group")
            .and_then(Value::as_str)
            .and_then(|label| groups.get(label));
        let old = node["node_id"]["digest"]
            .as_str()
            .expect("application node digest")
            .to_owned();
        let preimage = application_preimage(node, group.map(Vec::as_slice).unwrap_or(&[]));
        let new = quire_canonical::sha256(&preimage, quire_canonical::Limits::new(byte_limit))
            .expect("application preimage is within fixture reader byte limit")
            .to_string();
        if old != new {
            updates.push((old, new));
        }
    }
    updates
}

fn application_preimage(node: &Value, group: &[CheckedNodeId]) -> Value {
    let recursion = node
        .get("recursion_group")
        .and_then(Value::as_str)
        .and_then(|_| {
            let node_id: CheckedNodeId =
                serde_json::from_value(node["node_id"].clone()).expect("application node id");
            group
                .iter()
                .position(|member| member == &node_id)
                .map(|ordinal| json!({"size": group.len(), "ordinal": ordinal}))
        });
    json!({
        "version": APPLICATION_NODE_VERSION,
        "node_tag": node["node_tag"],
        "semantic_form": node["semantic_form"],
        "semantic_type": node["semantic_type"],
        "declaration": node.get("declaration").cloned().unwrap_or(Value::Null),
        "recursion": recursion,
        "body": group_references(&node["body"], group),
    })
}

fn group_references(body: &Value, group: &[CheckedNodeId]) -> Value {
    fn rewrite(term: &mut Value, group: &[CheckedNodeId]) {
        match term.get("term").and_then(Value::as_str) {
            Some("reference") => {
                let target = term
                    .get("target")
                    .cloned()
                    .and_then(|value| serde_json::from_value::<CheckedNodeId>(value).ok());
                if let Some(ordinal) = target
                    .as_ref()
                    .and_then(|target| group.iter().position(|member| member == target))
                {
                    *term = json!({"term": "group_reference", "ordinal": ordinal});
                }
            }
            Some("application") => {
                if let Some(arguments) = term.get_mut("arguments").and_then(Value::as_array_mut) {
                    for argument in arguments {
                        rewrite(argument, group);
                    }
                }
            }
            Some("aggregate") => {
                if let Some(members) = term.get_mut("members").and_then(Value::as_array_mut) {
                    for member in members {
                        rewrite(member, group);
                    }
                }
            }
            Some("binding") => {
                if let Some(value) = term.get_mut("value") {
                    rewrite(value, group);
                }
            }
            _ => {}
        }
    }

    if group.is_empty() {
        return body.clone();
    }
    let mut rewritten = body.clone();
    rewrite(&mut rewritten, group);
    rewritten
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
    let (mut ids, groups) = structural_candidates(raw, limits).unwrap_or_default();
    application_candidates(raw, &mut ids, limits);
    let mut seen = BTreeSet::new();
    let mut bytes_charged = 0u64;
    let mut work_charged = 0u64;
    for attempt in 0..attempts {
        let mut wire = raw.clone();
        substitute(&mut wire, &ids);
        for node in wire["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("fixture graph nodes")
        {
            if let Some(label) = node["recursion_group"].as_str() {
                if let Some(candidate) = groups.get(label) {
                    node["recursion_group"] = json!(candidate);
                }
            }
        }
        if !ids.0.is_empty() {
            reorder_application_dependencies(&mut wire);
            repair_application_candidates(&mut wire, limits.bytes, &mut ids);
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
            "fixture key repair exceeded its byte/work cap after {attempt} attempts, {bytes_charged} bytes, {work_charged} work, {} substitutions",
            ids.0.len()
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Application-key candidates are recomputed after earlier candidates change references.
    /// The second node therefore needs another pass after the first node's ID is replaced.
    #[test]
    fn application_candidates_converge_across_a_reference_chain() {
        let node_ref = |digest: &str| json!({"domain": NODE_DOMAIN, "digest": digest});
        let node = |digest: &str, body: Value| {
            json!({
                "node_id": node_ref(digest),
                "node_tag": "expression",
                "semantic_form": "call",
                "semantic_type": node_ref(&"a".repeat(64)),
                "dependencies": [],
                "body": body,
            })
        };
        let first = node(
            &"1".repeat(64),
            json!({"term": "application", "operator": "call", "arguments": []}),
        );
        let second = node(
            &"2".repeat(64),
            json!({
                "term": "application",
                "operator": "call",
                "arguments": [
                    {"term": "reference", "target": node_ref(&"1".repeat(64))},
                    {"term": "reference", "target": node_ref(&"3".repeat(64))},
                ],
            }),
        );
        let parameter = json!({
            "node_id": node_ref(&"3".repeat(64)),
            "node_tag": "value",
            "semantic_form": "parameter",
            "semantic_type": node_ref(&"a".repeat(64)),
            "dependencies": [],
            "body": {"term": "aggregate", "members": []},
        });
        let mut wire = json!({"semantic_graph": {"nodes": [first, second, parameter]}});

        let original_first_id = wire["semantic_graph"]["nodes"][0]["node_id"]["digest"]
            .as_str()
            .expect("original first digest")
            .to_owned();
        let mut ids = FixtureIds::default();
        let rounds = repair_application_candidates(&mut wire, 1 << 20, &mut ids);
        assert!(
            rounds >= 2,
            "the dependent key needs a second candidate pass"
        );
        let nodes = wire["semantic_graph"]["nodes"]
            .as_array()
            .expect("synthetic nodes");
        let first_id = nodes[0]["node_id"]["digest"]
            .as_str()
            .expect("first digest");
        assert_eq!(
            nodes[1]["body"]["arguments"][0]["target"]["digest"].as_str(),
            Some(first_id)
        );
        assert_eq!(ids.digest(&original_first_id), first_id);
        assert!(application_key_updates(&wire, 1 << 20).is_empty());

        let second_id_before_parameter_substitution = nodes[1]["node_id"]["digest"]
            .as_str()
            .expect("second digest")
            .to_owned();
        substitute_mapping(
            &mut wire,
            &BTreeMap::from([("3".repeat(64), "4".repeat(64))]),
        );
        repair_application_candidates(&mut wire, 1 << 20, &mut ids);
        let nodes = wire["semantic_graph"]["nodes"]
            .as_array()
            .expect("synthetic nodes after substitution");
        assert_ne!(
            nodes[1]["node_id"]["digest"].as_str(),
            Some(second_id_before_parameter_substitution.as_str()),
            "a later non-application ID substitution refreshes the dependent application key"
        );
        assert!(application_key_updates(&wire, 1 << 20).is_empty());
    }
}

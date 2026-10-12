//! Test-fixture key repair driven by Contract IR's public candidate API and reader refusals.
//!
//! A package builder retains its readable source IDs. This module repairs a copy of its wire,
//! so building another fixture cannot change IDs observed by a parallel test. The reader remains
//! the final authority for admission.

use std::collections::{BTreeMap, BTreeSet};

use quire_contract_model::{
    derive_application_node_id, derive_structural_candidates, ApplicationIdentityContext,
    ApplicationNodeDraft, CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageRefusalCause, CheckedPackageV2, CheckedPackageV2ReadResult,
    CheckedSemanticNodeV2,
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

fn structural_candidates(
    raw: &Value,
    limits: CheckedPackageReadLimits,
    mut ids: FixtureIds,
) -> Option<(FixtureIds, BTreeMap<String, String>)> {
    let raw_nodes: Vec<CheckedSemanticNodeV2> =
        serde_json::from_value(raw["semantic_graph"]["nodes"].clone()).ok()?;
    for node in &raw_nodes {
        if node.node_tag.as_ref() == "scalar_type"
            && matches!(node.semantic_form.as_ref(), "boolean" | "integer")
        {
            let candidate =
                derive_structural_candidates(std::slice::from_ref(node), limits.bytes, limits.work)
                    .ok()?;
            if let Some(scalar) = candidate.nodes().first() {
                let current = ids.resolve(&node.node_id);
                if &current != scalar.candidate_node_id() {
                    ids.insert(&current, scalar.candidate_node_id());
                }
            }
        }
    }
    // Structural identities can depend on other structural identities: a frame reaches its
    // object, and that object reaches a bound. Derive each round from the already substituted
    // graph, installing only public candidates for the IDs in that round. The reader below
    // remains the final admission authority, including for malformed negative fixtures.
    for _ in 0..raw_nodes.len() {
        let mut graph = raw["semantic_graph"]["nodes"].clone();
        substitute(&mut graph, &ids);
        reorder_frame_entries(&raw["semantic_graph"]["nodes"], &mut graph);
        let nodes: Vec<CheckedSemanticNodeV2> = serde_json::from_value(graph).ok()?;
        // Malformed negative fixtures have no structural candidates. Let the
        // reader report their exact refusal after the other fixture keys settle.
        let candidates = derive_structural_candidates(&nodes, limits.bytes, limits.work).ok()?;
        let mut changed = false;
        for candidate in candidates.nodes() {
            let old = &nodes[usize::try_from(candidate.position()).expect("fixture node position")]
                .node_id;
            let expected = candidate.candidate_node_id();
            if old != expected {
                ids.insert(old, expected);
                changed = true;
            }
        }
        if !changed {
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
            return Some((ids, groups));
        }
    }
    None
}

/// Settle application candidates, including references to other applications, after
/// structural references are repaired.
fn application_candidates(
    raw: &Value,
    ids: &mut FixtureIds,
    limits: CheckedPackageReadLimits,
) -> usize {
    let node_count = raw["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture graph nodes")
        .len();
    // An application can reference another application. Re-derive against the
    // newly substituted graph until those dependencies settle, using only the
    // public pre-read candidate API. The bounded reader loop remains the final
    // authority for malformed and semantically refused fixtures.
    for round in 0..node_count {
        let mut graph = raw["semantic_graph"]["nodes"].clone();
        substitute(&mut graph, ids);
        let Ok(nodes) = serde_json::from_value::<Vec<CheckedSemanticNodeV2>>(graph) else {
            return round;
        };
        let mut changed = false;
        for node in &nodes {
            let raw_body = serde_json::to_value(&node.body).expect("fixture body serializes");
            if raw_body["term"] != "application" {
                continue;
            }
            // A predecessor may have acquired its candidate earlier in this pass. Refresh this
            // node against the evolving map so a graph-ordered chain settles in one pass.
            let mut draft = serde_json::to_value(node).expect("fixture node serializes");
            substitute(&mut draft, ids);
            let Ok(draft) = serde_json::from_value::<CheckedSemanticNodeV2>(draft) else {
                return round;
            };
            let context_nodes = if draft.recursion_group.is_some() {
                let mut graph = raw["semantic_graph"]["nodes"].clone();
                substitute(&mut graph, ids);
                serde_json::from_value::<Vec<CheckedSemanticNodeV2>>(graph).ok()
            } else {
                None
            };
            let context = context_nodes.as_ref().and_then(|nodes| {
                ApplicationIdentityContext::try_new(nodes, ApplicationNodeDraft::new(&draft)).ok()
            });
            if draft.recursion_group.is_some() && context.is_none() {
                continue;
            }
            let Ok(candidate) = derive_application_node_id(
                ApplicationNodeDraft::new(&draft),
                context.as_ref(),
                limits.bytes,
            ) else {
                continue;
            };
            if &draft.node_id != candidate.candidate_node_id() {
                ids.insert(&draft.node_id, candidate.candidate_node_id());
                changed = true;
            }
        }
        if !changed {
            return round + 1;
        }
    }
    node_count
}

/// Settle interleaved structural and application references with the public candidate APIs.
fn pre_read_candidates(
    raw: &Value,
    limits: CheckedPackageReadLimits,
) -> (FixtureIds, BTreeMap<String, String>) {
    let node_count = raw["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture graph nodes")
        .len();
    let (mut ids, mut groups) =
        structural_candidates(raw, limits, FixtureIds::default()).unwrap_or_default();
    // A structural function can reference an application, and that application can reference
    // another function. Alternate the two public candidate families against evolving fixture
    // IDs before asking the reader; otherwise the reader repairs long interleaved chains one
    // key at a time and can exhaust its unchanged work cap before reaching admission.
    for _ in 0..node_count {
        let before = ids.0.len();
        let _ = application_candidates(raw, &mut ids, limits);
        if let Some((settled, labels)) = structural_candidates(raw, limits, ids.clone()) {
            ids = settled;
            groups = labels;
        }
        if ids.0.len() == before {
            break;
        }
    }
    (ids, groups)
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
                object["digest"] = json!(ids.digest(digest));
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

/// Typed-key substitutions can invert well-formed frame entry order. Sort only an array whose
/// authored keys were strictly ascending; malformed order remains for the reader to refuse.
fn reorder_frame_entries(raw_nodes: &Value, resolved_nodes: &mut Value) {
    fn entry_key(entry: &Value, member: &str) -> Option<(String, String)> {
        let node = if member == "modifies" {
            &entry["declaration"]
        } else {
            entry
        };
        if node["domain"] != NODE_DOMAIN {
            return None;
        }
        let name = if member == "modifies" {
            match entry["kind"].as_str()? {
                "relationship" if entry.get("name").is_none() => String::new(),
                "field" => entry["name"].as_str()?.to_owned(),
                _ => return None,
            }
        } else {
            String::new()
        };
        Some((node["digest"].as_str()?.to_owned(), name))
    }
    let (Some(raw_nodes), Some(resolved_nodes)) =
        (raw_nodes.as_array(), resolved_nodes.as_array_mut())
    else {
        return;
    };
    for (raw, resolved) in raw_nodes.iter().zip(resolved_nodes) {
        if raw["node_tag"] != "state"
            || raw["semantic_form"] != "frame"
            || raw["body"]["term"] != "frame"
        {
            continue;
        }
        for member in ["modifies", "creates", "deletes"] {
            let (Some(authored), Some(current)) = (
                raw["body"][member].as_array(),
                resolved["body"][member].as_array_mut(),
            ) else {
                continue;
            };
            let Some(authored_keys) = authored
                .iter()
                .map(|entry| entry_key(entry, member))
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            if !authored_keys.windows(2).all(|pair| pair[0] < pair[1])
                || !current
                    .iter()
                    .all(|entry| entry_key(entry, member).is_some())
            {
                continue;
            }
            current.sort_by_key(|entry| entry_key(entry, member).expect("checked frame key"));
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
    let (mut ids, groups) = pre_read_candidates(raw, limits);
    let mut seen = BTreeSet::new();
    let mut bytes_charged = 0u64;
    let mut work_charged = 0u64;
    for attempt in 0..attempts {
        let _ = application_candidates(raw, &mut ids, limits);
        let mut wire = raw.clone();
        substitute(&mut wire, &ids);
        reorder_frame_entries(
            &raw["semantic_graph"]["nodes"],
            &mut wire["semantic_graph"]["nodes"],
        );
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

    /// Trace: FR-021-AC-34, TC-031. A function key changes after its body application's
    /// public candidate changes, and the caller application then consumes that function key.
    #[test]
    fn interleaved_function_and_application_candidates_settle_before_reader() {
        let key = |code: u32| CheckedNodeId {
            domain: NODE_DOMAIN.into(),
            digest: format!("{code:064x}").into(),
        };
        let reference = |target: &CheckedNodeId| json!({"term": "reference", "target": target});
        let node = |code: u32, tag: &str, form: &str, body: Value| {
            json!({
                "node_id": key(code),
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": tag,
                "semantic_form": form,
                "semantic_type": key(9),
                "dependencies": [],
                "occurrences": [],
                "body": body,
            })
        };
        let raw = json!({"semantic_graph": {"nodes": [
            node(9, "scalar_type", "integer", json!({"term": "aggregate", "members": []})),
            node(4, "expression", "binary", json!({"term": "application", "arguments": [
                {"term": "literal", "value_kind": "integer", "value": "1"}
            ]})),
            node(3, "function", "pure_function", reference(&key(4))),
            node(2, "expression", "call", json!({"term": "application", "arguments": [
                reference(&key(3))
            ]})),
            node(1, "function", "pure_function", reference(&key(2))),
        ]}});
        let limits = CheckedPackageReadLimits::bounded();
        let (first, _) = structural_candidates(&raw, limits, FixtureIds::default())
            .expect("initial public structural candidates");
        let mut after_app = first.clone();
        application_candidates(&raw, &mut after_app, limits);
        let (after_structural, _) = structural_candidates(&raw, limits, after_app.clone())
            .expect("structural candidates after application substitution");
        assert!(
            after_structural.0.len() > after_app.0.len(),
            "one-shot structural admission misses the function whose body key changed"
        );
        let (settled, _) = pre_read_candidates(&raw, limits);
        assert!(settled.0.len() > after_structural.0.len());
        let mut stable = settled.clone();
        application_candidates(&raw, &mut stable, limits);
        let (stable, _) = structural_candidates(&raw, limits, stable)
            .expect("stable public structural candidates");
        assert_eq!(stable, settled);
    }

    /// Trace: FR-037-AC-1, TC-052. A later non-application substitution
    /// refreshes the dependent public application candidate.
    #[test]
    fn application_candidates_converge_across_a_reference_chain() {
        let node_ref = |digest: &str| json!({"domain": NODE_DOMAIN, "digest": digest});
        let node = |digest: &str, body: Value| {
            json!({
                "node_id": node_ref(digest),
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "expression",
                "semantic_form": "call",
                "semantic_type": node_ref(&"a".repeat(64)),
                "dependencies": [],
                "occurrences": [],
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
            "schema_version": "quire.checked-semantic-graph/v2",
            "node_tag": "value",
            "semantic_form": "parameter",
            "semantic_type": node_ref(&"a".repeat(64)),
            "dependencies": [],
            "occurrences": [],
            "body": {"term": "aggregate", "members": []},
        });
        let raw = json!({"semantic_graph": {"nodes": [first, second, parameter]}});
        let limits = CheckedPackageReadLimits::bounded();
        let mut ids = FixtureIds::default();
        assert!(application_candidates(&raw, &mut ids, limits) > 0);
        let mut settled = raw["semantic_graph"]["nodes"].clone();
        substitute(&mut settled, &ids);
        let settled: Vec<CheckedSemanticNodeV2> =
            serde_json::from_value(settled).expect("settled synthetic graph");
        assert_eq!(
            settled[1].node_id,
            *derive_application_node_id(ApplicationNodeDraft::new(&settled[1]), None, limits.bytes)
                .expect("public dependent candidate")
                .candidate_node_id()
        );
        let before = settled[1].node_id.clone();
        ids.insert(
            &serde_json::from_value(node_ref(&"3".repeat(64))).expect("old parameter"),
            &serde_json::from_value(node_ref(&"4".repeat(64))).expect("new parameter"),
        );
        assert!(application_candidates(&raw, &mut ids, limits) > 0);
        let mut settled = raw["semantic_graph"]["nodes"].clone();
        substitute(&mut settled, &ids);
        let settled: Vec<CheckedSemanticNodeV2> =
            serde_json::from_value(settled).expect("resettled synthetic graph");
        assert_ne!(settled[1].node_id, before);
        assert_eq!(
            settled[1].node_id,
            *derive_application_node_id(ApplicationNodeDraft::new(&settled[1]), None, limits.bytes)
                .expect("refreshed public candidate")
                .candidate_node_id()
        );
    }

    /// Trace: FR-037-AC-1, TC-052. Group identity uses the complete ordered
    /// snapshot; an absent or partial context cannot supply the same candidate.
    #[test]
    fn grouped_application_candidates_require_complete_context() {
        let node_ref = |digest: &str| json!({"domain": NODE_DOMAIN, "digest": digest});
        let nodes: Vec<CheckedSemanticNodeV2> = serde_json::from_value(json!([
            {
                "node_id": node_ref(&"1".repeat(64)),
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "expression",
                "semantic_form": "call",
                "semantic_type": node_ref(&"a".repeat(64)),
                "recursion_group": "group",
                "dependencies": [],
                "occurrences": [],
                "body": {"term": "application", "operator": "call", "arguments": []}
            },
            {
                "node_id": node_ref(&"2".repeat(64)),
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "expression",
                "semantic_form": "call",
                "semantic_type": node_ref(&"a".repeat(64)),
                "recursion_group": "group",
                "dependencies": [],
                "occurrences": [],
                "body": {"term": "application", "operator": "call", "arguments": [
                    {"term": "reference", "target": node_ref(&"1".repeat(64))}
                ]}
            }
        ]))
        .expect("grouped drafts");
        let draft = ApplicationNodeDraft::new(&nodes[1]);
        let limits = CheckedPackageReadLimits::bounded();
        assert!(derive_application_node_id(draft, None, limits.bytes).is_err());
        let complete = ApplicationIdentityContext::try_new(&nodes, draft)
            .expect("complete ordered group context");
        let expected = derive_application_node_id(draft, Some(&complete), limits.bytes)
            .expect("complete public candidate");
        let partial = ApplicationIdentityContext::try_new(&nodes[1..], draft)
            .expect("partial draft is structurally valid but lacks predecessor");
        let incomplete = derive_application_node_id(draft, Some(&partial), limits.bytes);
        assert!(
            incomplete.is_err()
                || incomplete.unwrap().candidate_node_id() != expected.candidate_node_id()
        );

        let raw = json!({"semantic_graph": {"nodes": nodes}});
        let mut ids = FixtureIds::default();
        assert!(application_candidates(&raw, &mut ids, limits) > 0);
        let mut settled = raw["semantic_graph"]["nodes"].clone();
        substitute(&mut settled, &ids);
        let settled: Vec<CheckedSemanticNodeV2> =
            serde_json::from_value(settled).expect("settled grouped drafts");
        for node in &settled {
            let context =
                ApplicationIdentityContext::try_new(&settled, ApplicationNodeDraft::new(node))
                    .expect("complete settled context");
            let candidate = derive_application_node_id(
                ApplicationNodeDraft::new(node),
                Some(&context),
                limits.bytes,
            )
            .expect("settled grouped candidate");
            assert_eq!(&node.node_id, candidate.candidate_node_id());
        }
    }

    /// Trace: FR-037-AC-1, TC-052. Rekeying changes digest order, while an authored
    /// out-of-order frame negative must still reach the reader unchanged.
    #[test]
    fn frame_entry_rekey_sorts_only_authored_ordered_arrays() {
        let node = |entries: Vec<Value>| {
            json!({"node_tag": "state", "semantic_form": "frame", "body": {
                "term": "frame", "modifies": entries, "creates": [], "deletes": []
            }})
        };
        let entry = |digest: &str| {
            json!({
                "kind": "relationship",
                "declaration": {"domain": NODE_DOMAIN, "digest": digest}
            })
        };
        let raw = json!([node(vec![entry("01"), entry("02")])]);
        let mut resolved = raw.clone();
        resolved[0]["body"]["modifies"] = json!([entry("09"), entry("03")]);
        reorder_frame_entries(&raw, &mut resolved);
        assert_eq!(
            resolved[0]["body"]["modifies"],
            json!([entry("03"), entry("09")])
        );

        let raw_negative = json!([node(vec![entry("02"), entry("01")])]);
        let mut negative = json!([node(vec![entry("09"), entry("03")])]);
        reorder_frame_entries(&raw_negative, &mut negative);
        assert_eq!(
            negative[0]["body"]["modifies"],
            json!([entry("09"), entry("03")])
        );
    }

    /// Trace: FR-037-AC-1, TC-052. A predecessor-first chain consumes each public
    /// candidate within one changing graph pass and settles to those candidate IDs.
    #[test]
    fn predecessor_first_candidates_settle_in_one_changing_pass() {
        let key = |code: u32| CheckedNodeId {
            domain: NODE_DOMAIN.into(),
            digest: format!("{code:064x}").into(),
        };
        let reference = |target: &CheckedNodeId| json!({"term": "reference", "target": target});
        let provisional = [key(1), key(2), key(3)];
        let mut nodes = Vec::new();
        for (position, node_id) in provisional.iter().enumerate() {
            let argument = if position == 0 {
                json!({"term": "literal", "value_kind": "integer", "value": "1"})
            } else {
                reference(&provisional[position - 1])
            };
            nodes.push(json!({
                "node_id": node_id,
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "expression",
                "semantic_form": "binary",
                "semantic_type": key(9),
                "dependencies": [],
                "occurrences": [],
                "body": {"term": "application", "arguments": [argument]},
            }));
        }
        let raw = json!({"semantic_graph": {"nodes": nodes}});
        let limits = CheckedPackageReadLimits::bounded();
        let mut ids = FixtureIds::default();
        assert_eq!(application_candidates(&raw, &mut ids, limits), 2);

        let mut resolved = raw["semantic_graph"]["nodes"].clone();
        substitute(&mut resolved, &ids);
        let resolved: Vec<CheckedSemanticNodeV2> =
            serde_json::from_value(resolved).expect("resolved fixture nodes");
        for (original, node) in provisional.iter().zip(&resolved) {
            assert_ne!(&node.node_id, original);
            let candidate =
                derive_application_node_id(ApplicationNodeDraft::new(node), None, limits.bytes)
                    .expect("public candidate for settled application");
            assert_eq!(&node.node_id, candidate.candidate_node_id());
        }
    }
}

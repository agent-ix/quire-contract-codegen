// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Association controls over unmodified published packages. These suite-only
//! traces do not claim generated-code semantics or admission coverage.

use super::CheckedGraph;
use crate::core::contract_package::routed_contract_lowering_profile;
use ix_trace_rs::trace;
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm, ContractDependencies, ContractPackageReadResult,
    ContractPackageV1, ContractPackageWireV1,
};
use std::{fs, path::PathBuf, ptr};

fn published_package(fixture: &str) -> CheckedPackageV2 {
    let root = PathBuf::from(std::env::var_os("QSPEC_DIR").expect("published QSpec checkout"))
        .join("proposals/checked-package-v2");
    let bytes = fs::read(root.join("fixtures").join(fixture)).expect("published fixture bytes");
    // Preserve the published document while canonicalizing its JSON encoding.
    let document: serde_json::Value = serde_json::from_slice(&bytes).expect("published JSON");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let selections: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("selected-documents.json"))
            .expect("published fixture document selections"),
    )
    .expect("published selection JSON");
    let fixture_path = format!("fixtures/{fixture}");
    if let Some(selected_path) = selections["selected_documents"][fixture_path.as_str()].as_str() {
        let bytes = fs::read(root.join(selected_path)).expect("published selected domain document");
        let selected: serde_json::Value =
            serde_json::from_slice(&bytes).expect("selected domain JSON");
        evidence
            .insert_domain_package_document(
                selected["package"]["identity"]
                    .as_str()
                    .expect("selected domain identity"),
                bytes,
            )
            .expect("retain published evidence");
    }
    let definitions: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("extension-definitions.json")).expect("published extensions"),
    )
    .expect("published extension JSON");
    for entry in definitions["definitions"]
        .as_array()
        .expect("extension definitions")
    {
        if document["lock"]["definition_selections"]
            .as_array()
            .expect("selected definitions")
            .contains(&entry["reference"])
        {
            evidence
                .insert_extension_definition(
                    serde_json::from_value(entry["reference"].clone())
                        .expect("owner extension ref"),
                    serde_json::from_value(entry["definition"].clone())
                        .expect("owner extension definition"),
                )
                .expect("retain published extension definition");
        }
    }
    let canonical =
        crate::core::canonical::content_bytes(&document).expect("canonical published fixture");
    match CheckedPackageV2::read(&canonical, CheckedPackageReadLimits::bounded(), &evidence) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("unmodified published fixture {fixture} must admit: {other:?}"),
    }
}

fn roots(source: &CheckedPackageV2) -> Vec<CheckedNodeId> {
    source
        .graph()
        .nodes
        .iter()
        .map(|node| node.node_id.clone())
        .collect()
}

fn read_lowered(bytes: &[u8], source: &CheckedPackageV2) -> ContractPackageV1 {
    match ContractPackageWireV1::read_with_source(
        bytes,
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
        source,
    ) {
        Ok(ContractPackageReadResult::Admitted(package)) => *package,
        other => panic!("strict reader must admit the actual lowering: {other:?}"),
    }
}

fn assert_associations(source: &CheckedPackageV2, package: &ContractPackageV1) {
    let graph = CheckedGraph::new(package);
    assert!(
        graph.len() > 0,
        "lowering must represent genuine source nodes"
    );
    assert_eq!(graph.len(), package.wire().semantic_graph.nodes.len());
    let mut expected: Vec<_> = package.wire().semantic_graph.nodes.iter().collect();
    expected.sort_unstable_by(|left, right| left.node_id.cmp(&right.node_id));
    let actual: Vec<_> = graph.nodes().collect();
    assert_eq!(
        actual, expected,
        "iteration follows complete node-key order"
    );
    assert!(actual
        .windows(2)
        .all(|pair| pair[0].node_id < pair[1].node_id));
    let mut terms = 0;
    for node in actual {
        let key = &node.node_id;
        // The source search is independent of both lowered-package accessors.
        let original = source
            .graph()
            .nodes
            .iter()
            .find(|original| &original.node_id == key)
            .expect("represented key must exist in the original admitted source");
        assert_eq!(
            serde_json::to_value(node).expect("lowered node serializes"),
            serde_json::to_value(original).expect("source node serializes"),
            "complete metadata association for {key:?}"
        );
        let selected = graph.node(key).expect("iterated key is queryable");
        assert!(
            ptr::eq(selected, node),
            "iteration and lookup borrow the same node"
        );
        assert!(ptr::eq(selected, package.node(key).expect("owner node")));
        assert_eq!(
            graph.term(key),
            source.term(key),
            "source term association for {key:?}"
        );
        match (graph.term(key), package.term(key)) {
            (Some(term), Some(owned)) => {
                terms += 1;
                assert!(
                    ptr::eq(term, owned),
                    "term is borrowed from this package's owner"
                );
            }
            (None, None) => {}
            _ => panic!("graph and owner disagree on optional body for {key:?}"),
        }
    }
    assert!(
        terms > 0,
        "association assertions must inspect admitted typed bodies"
    );
}

#[trace("TC-024", "TC-029", "TC-031")]
#[test]
fn published_source_associations_survive_lowering() {
    for fixture in [
        "positive-operation-identities.json",
        "positive-all-families.json",
        "positive-recursive-records.json",
    ] {
        let source = published_package(fixture);
        let mut visited = 0;
        let source_graph = CheckedGraph::from_checked_with_node_visit(&source, || visited += 1);
        assert_eq!(visited, source.graph().nodes.len());
        assert_eq!(source_graph.len(), source.graph().nodes.len());
        for node in &source.graph().nodes {
            assert!(ptr::eq(
                source_graph.node(&node.node_id).expect("source node"),
                node
            ));
            assert_eq!(source_graph.term(&node.node_id), source.term(&node.node_id));
        }
        let lowered = source.lower(&roots(&source), &routed_contract_lowering_profile());
        let producer = lowered.package.wire().expect("producer's admitted package");
        assert_associations(&source, producer);
    }
}

#[trace("TC-024", "TC-029", "TC-031")]
#[test]
fn reversed_and_duplicate_requests_keep_keyed_node_and_term_associations() {
    let source = published_package("positive-two-owners-a.json");
    let requested = roots(&source);
    assert!(
        requested.len() > 1,
        "reversal must exercise multiple genuine roots"
    );
    let baseline = source.lower(&requested, &routed_contract_lowering_profile());
    let baseline_package = baseline.package.wire().expect("baseline package");
    assert_associations(&source, baseline_package);
    let mut reversed = requested.clone();
    reversed.reverse();
    let mut duplicated = reversed.clone();
    duplicated.extend(requested.iter().cloned());
    for reordered in [reversed, duplicated] {
        let lowered = source.lower(&reordered, &routed_contract_lowering_profile());
        let reader = read_lowered(
            lowered
                .package
                .canonical_bytes()
                .expect("reordered lowering bytes"),
            &source,
        );
        for package in [lowered.package.wire().expect("reordered producer"), &reader] {
            assert_associations(&source, package);
            let graph = CheckedGraph::new(package);
            let baseline_graph = CheckedGraph::new(baseline_package);
            assert_eq!(
                graph.nodes().collect::<Vec<_>>(),
                baseline_graph.nodes().collect::<Vec<_>>()
            );
            for node in baseline_graph.nodes() {
                assert_eq!(
                    graph.term(&node.node_id),
                    baseline_graph.term(&node.node_id)
                );
            }
        }
    }
}

#[trace("TC-024", "TC-029", "TC-031")]
#[test]
fn genuine_other_owner_and_unrepresented_source_keys_do_not_cross_package_authority() {
    let source_a = published_package("positive-two-owners-a.json");
    let source_b = published_package("positive-two-owners-b.json");
    let lowered_a = source_a.lower(&roots(&source_a), &routed_contract_lowering_profile());
    let lowered_b = source_b.lower(&roots(&source_b), &routed_contract_lowering_profile());
    let read_a = read_lowered(
        lowered_a.package.canonical_bytes().expect("owner A bytes"),
        &source_a,
    );
    let read_b = read_lowered(
        lowered_b.package.canonical_bytes().expect("owner B bytes"),
        &source_b,
    );
    for (package_a, package_b) in [
        (
            lowered_a.package.wire().expect("producer A"),
            lowered_b.package.wire().expect("producer B"),
        ),
        (&read_a, &read_b),
    ] {
        assert_associations(&source_a, package_a);
        assert_associations(&source_b, package_b);
        let graph_a = CheckedGraph::new(package_a);
        let graph_b = CheckedGraph::new(package_b);
        let foreign = source_b
            .graph()
            .nodes
            .iter()
            .find(|node| {
                graph_b.node(&node.node_id).is_some()
                    && !source_a
                        .graph()
                        .nodes
                        .iter()
                        .any(|a| a.node_id == node.node_id)
            })
            .expect("published owner B has a represented key absent from owner A");
        assert_eq!(graph_a.node(&foreign.node_id), None);
        assert_eq!(graph_a.term(&foreign.node_id), None);
        assert!(
            graph_a
                .nodes()
                .all(|node| graph_b.node(&node.node_id).is_none()),
            "structural package names keep every owner A key outside owner B"
        );
    }

    let value = source_a
        .graph()
        .nodes
        .iter()
        .find(|node| {
            matches!(
                source_a.term(&node.node_id),
                Some(CheckedTerm::Application { .. })
            )
        })
        .expect("published owner A has an arithmetic value root");
    let subset = source_a.lower(
        std::slice::from_ref(&value.node_id),
        &routed_contract_lowering_profile(),
    );
    let subset_reader = read_lowered(
        subset.package.canonical_bytes().expect("subset bytes"),
        &source_a,
    );
    for package in [
        subset.package.wire().expect("subset producer"),
        &subset_reader,
    ] {
        assert_associations(&source_a, package);
        let graph = CheckedGraph::new(package);
        let absent = source_a
            .graph()
            .nodes
            .iter()
            .find(|node| {
                !package
                    .wire()
                    .semantic_graph
                    .nodes
                    .iter()
                    .any(|represented| represented.node_id == node.node_id)
            })
            .expect("a single arithmetic value does not represent every source declaration");
        assert_eq!(graph.node(&absent.node_id), None);
        assert_eq!(graph.term(&absent.node_id), None);
    }
}

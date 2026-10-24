// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Admitted control for the boundary between typed term children and opaque
//! operation metadata. The fixture extension is authored here; it carries no
//! claim of QSL compilation or generated oracle behavior.

use super::{narrowing, references, Graph, Narrowing};
use crate::core::canonical;
use ix_trace_rs::trace;
use quire_canonical::Limits;
use quire_contract_model::{
    derive_application_node_id, encode_checked_package_v2, ApplicationIdentityContext,
    ApplicationNodeDraft, ApplicationOperator, CheckedCapability, CheckedNodeId,
    CheckedPackageEvidence, CheckedPackageIdentityPreimageRefV2, CheckedPackageIdentityPreimageV2,
    CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult, CheckedSemanticGraphV2,
    CheckedSemanticId, CheckedSourceMapEntry, CheckedTerm,
};
use serde_json::{json, Value};
use std::{fs, path::PathBuf};

const BYTES: u64 = 16_777_216;

// Exact full-key substitution in an authored draft; no IDs are fabricated.
fn replace_key(value: &mut Value, old: &Value, new: &Value) {
    if value == old {
        *value = new.clone();
        return;
    }
    match value {
        Value::Array(items) => {
            for item in items {
                replace_key(item, old, new);
            }
        }
        Value::Object(items) => {
            for item in items.values_mut() {
                replace_key(item, old, new);
            }
        }
        _ => {}
    }
}

// Public owner API only: derive the changed application's key, then replace
// occurrences of its exact former key in the authored draft.
fn rekey_application(wire: &mut Value, old: &CheckedNodeId) -> CheckedNodeId {
    let graph: CheckedSemanticGraphV2 =
        serde_json::from_value(wire["semantic_graph"].clone()).unwrap();
    let draft = ApplicationNodeDraft::new(graph.nodes.iter().find(|n| &n.node_id == old).unwrap());
    let context = ApplicationIdentityContext::try_new(&graph.nodes, draft).unwrap();
    let key = derive_application_node_id(draft, Some(&context), BYTES)
        .unwrap()
        .candidate_node_id()
        .clone();
    replace_key(
        wire,
        &serde_json::to_value(old).unwrap(),
        &serde_json::to_value(&key).unwrap(),
    );
    key
}

fn encode_draft(wire: &Value) -> Vec<u8> {
    let graph: CheckedSemanticGraphV2 =
        serde_json::from_value(wire["semantic_graph"].clone()).unwrap();
    let preimage: CheckedPackageIdentityPreimageV2 =
        serde_json::from_value(wire["identity_preimage"].clone()).unwrap();
    let borrowed = CheckedPackageIdentityPreimageRefV2 {
        version: &preimage.version,
        edition: &preimage.edition,
        profile_selections: &preimage.profile_selections,
        definition_selections: &preimage.definition_selections,
        model_selections: &preimage.model_selections,
        required_features: &preimage.required_features,
        dependency_selections: &preimage.dependency_selections,
        identity_projection: &graph.nodes,
    };
    let mut id: CheckedSemanticId = serde_json::from_value(wire["package_id"].clone()).unwrap();
    id.digest = canonical::content_digest_with_limits(&borrowed, Limits::new(BYTES))
        .unwrap()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
        .into();
    encode_checked_package_v2(
        &borrowed,
        &id,
        &serde_json::from_value(wire["lock"].clone()).unwrap(),
        &graph,
        &serde_json::from_value::<Vec<CheckedSourceMapEntry>>(wire["source_map"].clone()).unwrap(),
        &serde_json::from_value::<Vec<CheckedCapability>>(wire["capability_report"].clone())
            .unwrap(),
        &serde_json::from_value(wire["diagnostics"].clone()).unwrap(),
        Limits::new(BYTES),
    )
    .unwrap()
}

fn read(bytes: &[u8], evidence: &CheckedPackageEvidence, label: &str) -> CheckedPackageV2 {
    match CheckedPackageV2::read(bytes, CheckedPackageReadLimits::bounded(), evidence) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("{label}_NOT_ADMITTED {other:?}"),
    }
}

// Suite-only: FR-014-AC-27 also requires generated bounds and descriptor
// behavior, which this direct narrowing control does not exercise.
#[trace("TC-024")]
#[test]
fn opaque_operation_member_does_not_add_a_consumer_to_true_narrowing() {
    let root = PathBuf::from(std::env::var_os("QSPEC_DIR").expect("published QSpec checkout"))
        .join("proposals/checked-package-v2");
    let mut wire: Value = serde_json::from_slice(
        &fs::read(root.join("fixtures/positive-operation-identities.json")).unwrap(),
    )
    .unwrap();
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    for selection in wire["lock"]["model_selections"].as_array().unwrap() {
        assert_eq!(selection["identity"], "acme/orders");
        evidence
            .insert_domain_package_document(
                selection["digest"].as_str().unwrap(),
                fs::read(root.join("domain-package-acme-orders.json")).unwrap(),
            )
            .unwrap();
    }
    let original_bytes = canonical::content_bytes(&wire).unwrap();
    let original = read(&original_bytes, &evidence, "ORIGINAL");
    let target = original.graph().nodes.iter().find(|n| matches!(original.term(&n.node_id),
        Some(CheckedTerm::Application { operation, .. }) if operation["identity"] == "quire.op.integer.mul")).unwrap();
    let bound = original
        .graph()
        .nodes
        .iter()
        .find(|n| {
            n.node_tag.as_ref() == "bounded_domain" && n.semantic_form.as_ref() == "integer_range"
        })
        .unwrap();
    let bound_body = serde_json::to_value(&bound.body).unwrap();
    assert_eq!(bound_body["members"][0]["value"]["value"], "-9");
    assert_eq!(bound_body["members"][1]["value"]["value"], "9");
    let catalogue: Value = serde_json::from_str(
        quire_verification_contracts::operation_catalog::CHECKED_OPERATION_CATALOG_V1,
    )
    .unwrap();
    let entry = catalogue["operations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["identity"] == "quire.op.numeric.narrow")
        .unwrap();
    assert_eq!(entry["operator"], "convert");
    assert_eq!(entry["operands"], json!(["exact_numeric"]));
    assert_eq!(entry["result"], "member");
    assert_eq!(entry["member"], "type_argument");
    assert_eq!(entry["laws"], json!([]));
    assert_eq!(entry["mode"], Value::Null);
    assert_eq!(entry["leaves"], Value::Null);
    // Retain the published member record's actual shape, changing its target.
    let mut member = original
        .graph()
        .nodes
        .iter()
        .filter_map(|n| match &n.body {
            CheckedTerm::Application { operation, .. } => operation.get("member"),
            _ => None,
        })
        .find(|m| m["kind"] == entry["member"])
        .unwrap()
        .clone();
    member["declaration"] = serde_json::to_value(&bound.node_id).unwrap();
    let mut narrow = target.clone();
    narrow.semantic_form = "conversion".into();
    narrow.semantic_type = bound.node_id.clone();
    narrow.body = serde_json::from_value(json!({
        "term": "application", "operator": entry["operator"],
        "operation": {"identity": entry["identity"], "laws": entry["laws"],
            "mode": entry["mode"], "member": member, "leaves": []},
        "result_type": bound.node_id, "arguments": [{"term":"reference", "target":target.node_id}]
    }))
    .expect("authored narrowing is a typed checked term");
    narrow.dependencies = vec![target.node_id.clone(), bound.node_id.clone()];
    narrow.dependencies.sort();
    // The source key here is only a provisional selector on a detached draft.
    // A public candidate is installed before the node enters the draft graph.
    let key = derive_application_node_id(ApplicationNodeDraft::new(&narrow), None, BYTES)
        .unwrap()
        .candidate_node_id()
        .clone();
    narrow.node_id = key.clone();
    assert!(original.graph().nodes.iter().all(|n| n.node_id != key));
    wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::to_value(&narrow).unwrap());
    // This is an authored fixture extension, not a claim that QSL compiled it.
    // Reuse a genuine published expression locus for the additional occurrence.
    for occurrence in original
        .source_map()
        .iter()
        .filter(|s| s.node_id == target.node_id)
    {
        let mut occurrence = occurrence.clone();
        occurrence.node_id = key.clone();
        wire["source_map"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::to_value(occurrence).unwrap());
    }
    let baseline_bytes = encode_draft(&wire);
    let baseline = read(&baseline_bytes, &evidence, "TRUE_NARROW_BASELINE");
    let opaque_node = baseline.graph().nodes.iter().find(|n| matches!(baseline.term(&n.node_id),
        Some(CheckedTerm::Application { operation, .. }) if operation["identity"] == "quire.op.integer.add")).unwrap();
    let CheckedTerm::Application { operation, .. } = &opaque_node.body else {
        panic!("admitted opaque node is an application");
    };
    assert!(operation["member"].is_null());
    let node = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|n| n["node_id"] == serde_json::to_value(&opaque_node.node_id).unwrap())
        .unwrap();
    node["body"]["operation"]["member"] = json!({"term":"reference", "target":target.node_id});
    let opaque_key = rekey_application(&mut wire, &opaque_node.node_id);
    let mutated_bytes = encode_draft(&wire);
    let mutated = read(&mutated_bytes, &evidence, "TRUE_NARROW_OPAQUE");
    assert_eq!(
        baseline.graph().nodes.len(),
        original.graph().nodes.len() + 1
    );
    assert_eq!(mutated.graph().nodes.len(), baseline.graph().nodes.len());
    for (package, add_key) in [(&baseline, &opaque_node.node_id), (&mutated, &opaque_key)] {
        let graph = Graph::from_checked(package);
        let Some(CheckedTerm::Application {
            operator: ApplicationOperator::Convert,
            operation,
            arguments,
            result_type,
        }) = graph.term(&key)
        else {
            panic!("genuine narrowing application must survive strict admission")
        };
        assert_eq!(operation["identity"], entry["identity"]);
        assert_eq!(result_type, &bound.node_id);
        assert!(
            matches!(arguments.as_slice(), [CheckedTerm::Reference { target: id }]
            if id == &target.node_id)
        );
        assert!(
            references(graph.term(&key), &target.node_id),
            "true argument consumes the target"
        );
        match narrowing(&graph, &target.node_id) {
            Narrowing::To(bounds) => {
                assert_eq!(bounds.len(), 1, "one genuine narrowing bound");
                assert_eq!(bounds[0].node_id, bound.node_id);
                assert_eq!(bounds[0], bound);
            }
            Narrowing::Mixed => panic!("metadata must not create a second semantic consumer"),
            Narrowing::Unnarrowed => panic!("true narrowing argument must remain a consumer"),
        }
        assert!(
            !references(graph.term(add_key), &target.node_id),
            "opaque operation metadata is not a semantic child"
        );
        assert_eq!(
            graph.node(add_key).expect("admitted add").dependencies,
            opaque_node.dependencies,
            "opaque metadata does not change graph dependencies"
        );
        assert_eq!(
            graph
                .node(&target.node_id)
                .expect("admitted target")
                .dependencies,
            target.dependencies
        );
        assert_eq!(
            graph.node(&key).expect("admitted conversion").dependencies,
            narrow.dependencies
        );
    }
    let Some(CheckedTerm::Application { operation, .. }) = mutated.term(&opaque_key) else {
        panic!("opaque metadata not retained")
    };
    assert_eq!(
        operation["member"],
        json!({"term":"reference", "target":target.node_id})
    );
    // Positive discriminator: a real argument reference, unlike metadata,
    // creates an additional semantic consumer alongside the true narrowing.
    let node = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|node| node["node_id"] == serde_json::to_value(&opaque_key).unwrap())
        .unwrap();
    node["body"]["arguments"][0] = json!({"term":"reference", "target":target.node_id});
    node["dependencies"] = json!([target.node_id]);
    let semantic_key = rekey_application(&mut wire, &opaque_key);
    let semantic_bytes = encode_draft(&wire);
    let semantic = read(&semantic_bytes, &evidence, "TRUE_NARROW_SEMANTIC");
    assert_eq!(semantic.graph().nodes.len(), baseline.graph().nodes.len());
    let graph = Graph::from_checked(&semantic);
    assert!(references(graph.term(&key), &target.node_id));
    assert!(
        references(graph.term(&semantic_key), &target.node_id),
        "a true argument reference must create another consumer"
    );
    assert!(
        matches!(narrowing(&graph, &target.node_id), Narrowing::Mixed),
        "a real non-conversion consumer makes the narrowing shared"
    );
    assert_eq!(
        graph
            .node(&semantic_key)
            .expect("semantic add")
            .dependencies,
        vec![target.node_id.clone()]
    );
    let Some(CheckedTerm::Application {
        operation,
        arguments,
        ..
    }) = graph.term(&semantic_key)
    else {
        panic!("semantic sibling must remain an application")
    };
    assert!(matches!(&arguments[0], CheckedTerm::Reference {target: id} if id == &target.node_id));
    assert_eq!(
        operation["member"],
        json!({"term":"reference", "target":target.node_id})
    );
}

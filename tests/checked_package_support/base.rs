// The base CheckedPackage V2 document the exact-scalar and composite-equality package builders
// start from. `include!`d by both.

/// Source-assigned paths of the base package's four nominal nodes.
struct BaseNodeIds {
    enum_type: String,
    enum_member: String,
    unit: String,
    dimension: String,
}

const BASE_PACKAGE_NAME: &str = "cg-fixture";

fn base_node_ref(path: &str) -> serde_json::Value {
    serde_json::json!({
        "package": BASE_PACKAGE_NAME,
        "path": path.split('/').collect::<Vec<_>>(),
        "ordinal": 0
    })
}

fn base_owner() -> serde_json::Value {
    serde_json::json!({"kind": "source", "authority": "agent-ix", "identity": "example"})
}

fn base_node_ids() -> BaseNodeIds {
    BaseNodeIds {
        enum_type: "Example/Phase".into(),
        enum_member: "Example/Phase/OPEN".into(),
        dimension: "Example/Distance".into(),
        unit: "Example/pace".into(),
    }
}

/// The base CheckedPackage V2 wire document: an ordered enum `Example.Phase { OPEN, SHUT }`, its
/// `OPEN` member, a `Distance` dimension and a `pace` unit of it. The reader retains their
/// producer-assigned paths independent of body edits.
fn base_package() -> serde_json::Value {
    const GRAPH: &str = "quire.checked-semantic-graph/v2";
    let ids = base_node_ids();
    let node_ref = |digest: &str| base_node_ref(digest);
    // Definitions and source documents have their own closed identity types.
    let definition =
        |identity: &str| serde_json::json!({"authority": "agent-ix", "identity": identity});
    let source_ref = |identity: &str| definition(identity);
    let edition = serde_json::json!({
        "role": "edition",
        "definition": definition("quire-edition"),
    });
    let model = definition("example-model");
    let source = source_ref("example");
    let declared = serde_json::json!([{"role": "declaration", "ordinal": 0}]);

    let nodes = vec![
        serde_json::json!({
            "node_id": node_ref(&ids.enum_member), "schema_version": GRAPH,
            "node_tag": "value", "semantic_form": "enum_value",
            "semantic_type": node_ref(&ids.enum_type), "dependencies": [node_ref(&ids.enum_type)],
            "occurrences": declared,
            "body": {"term": "literal", "type": node_ref(&ids.enum_type), "value_kind": "enum", "value": "OPEN"},
        }),
        serde_json::json!({
            "node_id": node_ref(&ids.enum_type), "schema_version": GRAPH,
            "node_tag": "scalar_type", "semantic_form": "enum",
            "semantic_type": node_ref(&ids.enum_type),
            "declaration": {"qualified_name": ["Example", "Phase"]},
            "dependencies": [], "occurrences": declared,
            "body": {"term": "enum", "ordered": true, "members": ["OPEN", "SHUT"]},
            "owner": base_owner(),
        }),
        serde_json::json!({
            "node_id": node_ref(&ids.unit), "schema_version": GRAPH,
            "node_tag": "scalar_type", "semantic_form": "unit",
            "semantic_type": node_ref(&ids.dimension),
            "declaration": {"qualified_name": ["Example", "pace"]},
            "dependencies": [node_ref(&ids.dimension)], "occurrences": declared,
            "body": {"term": "unit", "dimension_node_id": node_ref(&ids.dimension),
                "target_unit_node_id": null,
                "scale": {"numerator": "1", "denominator": "1"},
                "offset": {"numerator": "0", "denominator": "1"}},
            "owner": base_owner(),
        }),
        serde_json::json!({
            "node_id": node_ref(&ids.dimension), "schema_version": GRAPH,
            "node_tag": "scalar_type", "semantic_form": "dimension",
            "semantic_type": node_ref(&ids.dimension),
            "declaration": {"qualified_name": ["Example", "Distance"]},
            "dependencies": [], "occurrences": declared,
            "body": {"term": "dimension", "terms": []},
            "owner": base_owner(),
        }),
    ];
    let source_map = [&ids.enum_member, &ids.enum_type, &ids.unit, &ids.dimension]
        .iter()
        .enumerate()
        .map(|(start, digest)| {
            serde_json::json!({
                "node_id": node_ref(digest), "role": "declaration", "ordinal": 0,
                "regions": [{"source": source, "start": start, "end": start + 1}],
            })
        })
        .collect::<Vec<_>>();

    serde_json::json!({
        "contract_version": "quire.checked-package/v2",
        "name": BASE_PACKAGE_NAME,
        "lock": {
            "sources": [source, source_ref("example-units")],
            "edition": edition,
            "profile_selections": [],
            "definition_selections": [model],
            "required_features": ["quire.value.complete/v1"],
            "dependency_selections": [],
        },
        "semantic_graph": {"graph_version": GRAPH, "nodes": nodes},
        "source_map": source_map,
        "capability_report": [{"feature": "quire.value.complete/v1", "disposition": "available"}],
        "diagnostics": {
            "catalog": definition("catalog"),
            "entries": [],
        },
    })
}

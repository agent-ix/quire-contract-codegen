// The base CheckedPackage V2 document the exact-scalar and composite-equality package builders
// start from. `include!`d by both.

/// The node ids of the base package's four nominal nodes. Each is the digest Contract IR itself
/// computes over the node's nominal identity preimage (`NominalIdentityPreimage::digest`), so the
/// ids are derived here, never written down.
struct BaseNodeIds {
    enum_type: String,
    enum_member: String,
    unit: String,
    dimension: String,
}

const BASE_NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";

fn base_node_ref(digest: &str) -> serde_json::Value {
    serde_json::json!({"domain": BASE_NODE_DOMAIN, "digest": digest})
}

fn base_owner() -> serde_json::Value {
    serde_json::json!({"kind": "definition", "authority": "agent-ix", "identity": "example-model"})
}

fn base_enum_declaration_preimage() -> serde_json::Value {
    serde_json::json!({
        "version": "quire.enum-declaration-node/v1", "owner": base_owner(),
        "qualified_declaration": ["Example", "Phase"],
        "ordered": true, "members": ["OPEN", "SHUT"],
    })
}

fn base_enum_member_preimage(enum_type: &str) -> serde_json::Value {
    serde_json::json!({
        "version": "quire.enum-member-node/v1",
        "declaration_node_id": base_node_ref(enum_type), "case": "OPEN",
    })
}

fn base_dimension_preimage() -> serde_json::Value {
    serde_json::json!({
        "version": "quire.dimension-node/v1", "owner": base_owner(),
        "qualified_declaration": ["Example", "Distance"], "terms": [],
    })
}

fn base_unit_preimage(dimension: &str) -> serde_json::Value {
    serde_json::json!({
        "version": "quire.unit-node/v1", "owner": base_owner(),
        "qualified_declaration": ["Example", "pace"],
        "dimension_node_id": base_node_ref(dimension), "target_unit_node_id": null,
        "scale": {"numerator": "1", "denominator": "1"},
        "offset": {"numerator": "0", "denominator": "1"},
    })
}

/// Contract IR's own digest of one nominal identity preimage.
fn base_preimage_digest(preimage: serde_json::Value) -> String {
    serde_json::from_value::<quire_contract_model::NominalIdentityPreimage>(preimage)
        .expect("a well-formed nominal identity preimage")
        .digest()
        .expect("a nominal identity preimage digests")
}

fn base_node_ids() -> BaseNodeIds {
    let enum_type = base_preimage_digest(base_enum_declaration_preimage());
    let dimension = base_preimage_digest(base_dimension_preimage());
    BaseNodeIds {
        enum_member: base_preimage_digest(base_enum_member_preimage(&enum_type)),
        unit: base_preimage_digest(base_unit_preimage(&dimension)),
        enum_type,
        dimension,
    }
}

/// The base CheckedPackage V2 wire document: an ordered enum `Example.Phase { OPEN, SHUT }`, its
/// `OPEN` member, a `Distance` dimension and a `pace` unit of it, each keyed by the digest of its
/// nominal identity preimage. The builder appends its own nodes and re-derives
/// `identity_preimage.identity_projection` and `package_id` from the finished graph (`wire()`), so
/// both start empty here.
fn base_package() -> serde_json::Value {
    const GRAPH: &str = "quire.checked-semantic-graph/v2";
    let ids = base_node_ids();
    let node_ref = |digest: &str| base_node_ref(digest);
    let artifact = |identity: &str, namespace: &str, domain: &str, digest: char| {
        serde_json::json!({
            "authority": "agent-ix",
            "identity": identity,
            "revision": {"namespace": namespace, "value": "1"},
            "digest_domain": domain,
            "digest": digest.to_string().repeat(64),
        })
    };
    let edition = serde_json::json!({
        "role": "edition",
        "definition": artifact("quire-edition", "semver", "quire.definition.bytes/v1", '1'),
    });
    let model = artifact("example-model", "git", "quire.definition.bytes/v1", '4');
    let source = artifact("example", "git", "quire.source.bytes/v1", '2');
    let aggregate = serde_json::json!({"term": "aggregate", "members": []});
    let declared = serde_json::json!([{"role": "declaration", "ordinal": 0}]);

    let nodes = vec![
        serde_json::json!({
            "node_id": node_ref(&ids.enum_member), "schema_version": GRAPH,
            "node_tag": "value", "semantic_form": "enum_value",
            "semantic_type": node_ref(&ids.enum_type), "dependencies": [node_ref(&ids.enum_type)],
            "occurrences": declared,
            "nominal_identity_preimage": base_enum_member_preimage(&ids.enum_type),
            "body": {"term": "literal", "type": node_ref(&ids.enum_type), "value_kind": "enum", "value": "OPEN"},
        }),
        serde_json::json!({
            "node_id": node_ref(&ids.enum_type), "schema_version": GRAPH,
            "node_tag": "scalar_type", "semantic_form": "enum",
            "semantic_type": node_ref(&ids.enum_type),
            "declaration": {"qualified_name": ["Example", "Phase"]},
            "dependencies": [], "occurrences": declared,
            "nominal_identity_preimage": base_enum_declaration_preimage(),
            "body": aggregate,
        }),
        serde_json::json!({
            "node_id": node_ref(&ids.unit), "schema_version": GRAPH,
            "node_tag": "scalar_type", "semantic_form": "unit",
            "semantic_type": node_ref(&ids.dimension),
            "declaration": {"qualified_name": ["Example", "pace"]},
            "dependencies": [node_ref(&ids.dimension)], "occurrences": declared,
            "nominal_identity_preimage": base_unit_preimage(&ids.dimension),
            "body": aggregate,
        }),
        serde_json::json!({
            "node_id": node_ref(&ids.dimension), "schema_version": GRAPH,
            "node_tag": "scalar_type", "semantic_form": "dimension",
            "semantic_type": node_ref(&ids.dimension),
            "declaration": {"qualified_name": ["Example", "Distance"]},
            "dependencies": [], "occurrences": declared,
            "nominal_identity_preimage": base_dimension_preimage(),
            "body": aggregate,
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
        "identity_preimage": {
            "version": "quire.checked-package-id/v2",
            "edition": edition,
            "profile_selections": [],
            "definition_selections": [model],
            "model_selections": [],
            "required_features": ["quire.value.complete/v1"],
            "dependency_selections": [],
            "identity_projection": [],
        },
        "package_id": {"domain": "quire.package.semantic/v2", "algorithm": "sha256", "digest": ""},
        "lock": {
            "sources": [source, artifact("example-units", "git", "quire.source.bytes/v1", '6')],
            "edition": edition,
            "profile_selections": [],
            "definition_selections": [model],
            "model_selections": [],
            "required_features": ["quire.value.complete/v1"],
            "dependency_selections": [],
        },
        "semantic_graph": {"graph_version": GRAPH, "nodes": nodes},
        "source_map": source_map,
        "capability_report": [{"feature": "quire.value.complete/v1", "disposition": "available"}],
        "diagnostics": {
            "catalog": {
                "authority": "agent-ix",
                "identity": "catalog",
                "revision": {"namespace": "draft", "value": "1"},
                "digest_domain": "quire.definition.bytes/v1",
                "digest": "3".repeat(64),
            },
            "entries": [],
        },
    })
}

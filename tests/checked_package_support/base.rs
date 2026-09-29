// The base CheckedPackage V2 document the exact-scalar and composite-equality package builders
// start from. `include!`d by both, so it defines one function and nothing else.

/// The base CheckedPackage V2 wire document: an ordered enum `Example.Status { READY, DONE }`, its
/// `READY` member, a `Length` dimension and a `metre` unit of it, each keyed by the digest of its
/// nominal identity preimage. The builder appends its own nodes and re-derives
/// `identity_preimage.identity_projection` and `package_id` from the finished graph (`wire()`), so
/// both start empty here.
fn base_package() -> serde_json::Value {
    const NODE: &str = "quire.checked-semantic-node/v1";
    const GRAPH: &str = "quire.checked-semantic-graph/v2";
    const ENUM_MEMBER: &str = "42ba51e7e622d99f292a5e6dcc196bd216efb98b33755910e54af4d65078d032";
    const ENUM_TYPE: &str = "7928f1e1b570335b404c8d21c66da8a3b8e37e434b0ebc622f80285488811562";
    const UNIT: &str = "79637623a46d29e884b62c6fa292aeb29d41e4ecc4e800b4d7ee910a3eaf23a4";
    const DIMENSION: &str = "b6cc14ab93b670cb0fc74a80dd18131ef7b06e3eee6a730e5ca092266314e22b";

    let node_ref = |digest: &str| serde_json::json!({"domain": NODE, "digest": digest});
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
    let owner = serde_json::json!({"kind": "definition", "authority": "agent-ix", "identity": "example-model"});
    let aggregate = serde_json::json!({"term": "aggregate", "members": []});
    let declared = serde_json::json!([{"role": "declaration", "ordinal": 0}]);

    let nodes = vec![
        serde_json::json!({
            "node_id": node_ref(ENUM_MEMBER), "schema_version": GRAPH,
            "node_tag": "value", "semantic_form": "enum_value",
            "semantic_type": node_ref(ENUM_TYPE), "dependencies": [node_ref(ENUM_TYPE)],
            "occurrences": declared,
            "nominal_identity_preimage": {
                "version": "quire.enum-member-node/v1",
                "declaration_node_id": node_ref(ENUM_TYPE), "case": "READY",
            },
            "body": {"term": "literal", "type": node_ref(ENUM_TYPE), "value_kind": "enum", "value": "READY"},
        }),
        serde_json::json!({
            "node_id": node_ref(ENUM_TYPE), "schema_version": GRAPH,
            "node_tag": "scalar_type", "semantic_form": "enum",
            "semantic_type": node_ref(ENUM_TYPE),
            "declaration": {"qualified_name": ["Example", "Status"]},
            "dependencies": [], "occurrences": declared,
            "nominal_identity_preimage": {
                "version": "quire.enum-declaration-node/v1", "owner": owner,
                "qualified_declaration": ["Example", "Status"],
                "ordered": true, "members": ["READY", "DONE"],
            },
            "body": aggregate,
        }),
        serde_json::json!({
            "node_id": node_ref(UNIT), "schema_version": GRAPH,
            "node_tag": "scalar_type", "semantic_form": "unit",
            "semantic_type": node_ref(DIMENSION),
            "declaration": {"qualified_name": ["Example", "metre"]},
            "dependencies": [node_ref(DIMENSION)], "occurrences": declared,
            "nominal_identity_preimage": {
                "version": "quire.unit-node/v1", "owner": owner,
                "qualified_declaration": ["Example", "metre"],
                "dimension_node_id": node_ref(DIMENSION), "target_unit_node_id": null,
                "scale": {"numerator": "1", "denominator": "1"},
                "offset": {"numerator": "0", "denominator": "1"},
            },
            "body": aggregate,
        }),
        serde_json::json!({
            "node_id": node_ref(DIMENSION), "schema_version": GRAPH,
            "node_tag": "scalar_type", "semantic_form": "dimension",
            "semantic_type": node_ref(DIMENSION),
            "declaration": {"qualified_name": ["Example", "Length"]},
            "dependencies": [], "occurrences": declared,
            "nominal_identity_preimage": {
                "version": "quire.dimension-node/v1", "owner": owner,
                "qualified_declaration": ["Example", "Length"], "terms": [],
            },
            "body": aggregate,
        }),
    ];
    let source_map = [ENUM_MEMBER, ENUM_TYPE, UNIT, DIMENSION]
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

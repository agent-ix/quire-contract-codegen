//! Admitted CheckedPackage V2 fixtures for composite/structural equality
//! generation (FR-018).
//!
//! Built on the same vendored `positive-nominal-identities.json` base as
//! `exact_scalar_support::package`, with `scalar_type`, `composite_type`,
//! `bounded_domain` and `expression` nodes appended under readable
//! zero-padded keys. Composite/collection bodies use the encoding
//! `quire_contract_codegen::composite_equality` documents: `record` and
//! `tuple` as an `aggregate` of `binding`/`reference` terms, `option` as a
//! one-member `aggregate` of a `reference`, `sequence`/`set`/`bag`/
//! `ordered_set` as an element `reference` then a `collection_bounds`
//! `reference`, and `collection_bounds` as two canonical decimal `integer`
//! literals.

#![allow(dead_code)] // Each test binary uses a different subset.

use quire_contract_codegen::{
    CompositeEqualityItem, EqualityOperandDescriptor, EqualityOperatorKind,
};
use quire_contract_ir::{
    CheckedArtifactLocator, CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";

/// The vendored fixture's admitted `scalar_type`/`enum` node (`Example.Status`,
/// members `READY`/`DONE`): reused rather than re-derived, since its nominal
/// identity preimage's digest must equal its own node id.
pub const ENUM_TYPE_DIGEST: &str =
    "7928f1e1b570335b404c8d21c66da8a3b8e37e434b0ebc622f80285488811562";

pub fn enum_type_id() -> CheckedNodeId {
    id(ENUM_TYPE_DIGEST)
}

/// A readable node key: the code, zero-padded to a 64-digit digest.
pub fn key(code: u32) -> String {
    format!("{code:0>64}")
}

pub fn id(digest: &str) -> CheckedNodeId {
    serde_json::from_value(node_ref(digest)).expect("node id")
}

pub fn code_id(code: u32) -> CheckedNodeId {
    id(&key(code))
}

fn node_ref(digest: &str) -> Value {
    json!({"domain": NODE_DOMAIN, "digest": digest})
}

pub fn reference(code: u32) -> Value {
    json!({"term": "reference", "target": node_ref(&key(code))})
}

pub fn binding(name: &str, code: u32) -> Value {
    json!({"term": "binding", "name": name, "value": reference(code)})
}

pub fn aggregate(members: Vec<Value>) -> Value {
    json!({"term": "aggregate", "members": members})
}

pub fn literal(kind: &str, value: &str) -> Value {
    json!({"term": "literal", "value_kind": kind, "value": value})
}

pub fn integer_literal(value: i64) -> Value {
    literal("integer", &value.to_string())
}

fn application(operator: &str, arguments: Vec<Value>) -> Value {
    json!({"term": "application", "operator": operator, "arguments": arguments})
}

/// A well-formed two-argument `binary` application body: content is
/// irrelevant to the generator, which reads the operand types from the
/// request descriptor, not from the body.
pub fn binary_body() -> Value {
    application(
        "binary",
        vec![literal("boolean", "true"), literal("boolean", "true")],
    )
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// A V2 package under construction.
pub struct PackageBuilder {
    value: Value,
}

impl Default for PackageBuilder {
    fn default() -> Self {
        let text = include_str!("../fixtures/exact_scalar/positive-nominal-identities.json");
        Self {
            value: serde_json::from_str(text).expect("vendored fixture is JSON"),
        }
    }
}

impl PackageBuilder {
    pub fn node(
        &mut self,
        digest: &str,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
    ) -> &mut Self {
        self.node_in_group(digest, tag, form, semantic_type, body, None)
    }

    /// A node whose body-reference cycle needs an explicit `recursion_group`:
    /// every node on one cycle must share the same non-empty group string.
    pub fn node_in_group(
        &mut self,
        digest: &str,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
        recursion_group: Option<&str>,
    ) -> &mut Self {
        let nodes = self.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("nodes");
        let mut node = json!({
            "node_id": node_ref(digest),
            "schema_version": "quire.checked-semantic-graph/v2",
            "node_tag": tag,
            "semantic_form": form,
            "semantic_type": node_ref(semantic_type),
            "dependencies": [],
            "occurrences": [{"role": "declaration", "ordinal": 0}],
            "body": body,
        });
        if let Some(group) = recursion_group {
            node["recursion_group"] = json!(group);
        }
        nodes.push(node);
        let source = self.value["lock"]["sources"][0].clone();
        let map = self.value["source_map"].as_array_mut().expect("source map");
        let start = map.len();
        map.push(json!({
            "node_id": node_ref(digest),
            "role": "declaration",
            "ordinal": 0,
            "regions": [{
                "source": source,
                "start": start,
                "end": start + 1,
            }],
        }));
        self
    }

    pub fn code(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: u32,
        body: Value,
    ) -> &mut Self {
        self.node(&key(code), tag, form, &key(semantic_type), body)
    }

    pub fn code_in_group(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: u32,
        body: Value,
        recursion_group: &str,
    ) -> &mut Self {
        self.node_in_group(
            &key(code),
            tag,
            form,
            &key(semantic_type),
            body,
            Some(recursion_group),
        )
    }

    /// The wire document with its identity projection and package id refreshed.
    pub fn wire(&self) -> Value {
        let mut package = self.value.clone();
        let projection = package["semantic_graph"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .cloned()
            .map(|mut node| {
                node.as_object_mut().expect("node").remove("occurrences");
                node
            })
            .collect::<Vec<_>>();
        package["identity_preimage"]["identity_projection"] = Value::Array(projection);
        let preimage = serde_json::to_vec(&package["identity_preimage"]).expect("preimage");
        package["package_id"]["digest"] = json!(sha256_hex(&preimage));
        package
    }

    pub fn admit(&self) -> CheckedPackageV2 {
        let wire = self.wire();
        let bytes = serde_json::to_vec(&wire).expect("canonical bytes");
        match CheckedPackageV2::read(
            &bytes,
            CheckedPackageReadLimits::bounded(),
            &evidence(&wire),
        ) {
            CheckedPackageV2ReadResult::Admitted(package) => *package,
            other => panic!("expected V2 admission, got {other:?}"),
        }
    }
}

fn locator(artifact: &Value) -> CheckedArtifactLocator {
    let text = |value: &Value| -> Box<str> { value.as_str().expect("locator member").into() };
    CheckedArtifactLocator {
        authority: text(&artifact["authority"]),
        identity: text(&artifact["identity"]),
        revision_namespace: text(&artifact["revision"]["namespace"]),
        revision_value: text(&artifact["revision"]["value"]),
        domain: text(&artifact["digest_domain"]),
    }
}

fn evidence(package: &Value) -> CheckedPackageEvidence {
    let lock = &package["lock"];
    let mut artifacts = lock["sources"].as_array().cloned().unwrap_or_default();
    artifacts.push(lock["edition"]["definition"].clone());
    artifacts.extend(
        lock["definition_selections"]
            .as_array()
            .cloned()
            .unwrap_or_default(),
    );
    artifacts.push(package["diagnostics"]["catalog"].clone());
    let mut evidence = CheckedPackageEvidence::new();
    for artifact in &artifacts {
        evidence.insert_artifact_digest(
            locator(artifact),
            artifact["digest"].as_str().expect("digest"),
        );
    }
    evidence.support_feature("quire.value.complete/v1");
    evidence
}

// ---------------------------------------------------------------------------
// The composite/structural corpus
// ---------------------------------------------------------------------------

pub const T_BOOLEAN: u32 = 1;
pub const T_INTEGER: u32 = 2;
pub const T_TEXT: u32 = 3;
pub const T_FLOAT64: u32 = 4;
pub const BD_TEXT: u32 = 6;
pub const T_INTEGER_BOUNDED: u32 = 7;
pub const BD_INTEGER: u32 = 8;
pub const T_DECIMAL_SMALL: u32 = 9;
pub const BD_DECIMAL: u32 = 10;

pub const R_POINT: u32 = 20;
pub const R_FLOAT: u32 = 21;
pub const CB_SMALL: u32 = 22;
pub const SEQ_R_FLOAT: u32 = 23;
pub const TUP_PAIR: u32 = 24;
pub const OPT_INT: u32 = 25;
pub const R_DUP: u32 = 27;
pub const REF_TYPE: u32 = 28;
pub const R_SELF: u32 = 29;
pub const OPT_SELF: u32 = 30;
pub const R_PAIR_OF_POINTS: u32 = 31;
pub const SEQ_INT: u32 = 32;

pub const M_BARE: u32 = 40;
pub const F_BARE: u32 = 41;
pub const S_BARE: u32 = 42;
pub const T_BARE: u32 = 43;

pub const E_RECORD: u32 = 100;
pub const E_NESTED_IEEE: u32 = 101;
pub const E_TUPLE: u32 = 102;
pub const E_OPTION: u32 = 103;
pub const E_TEXT: u32 = 104;
pub const E_ENUM: u32 = 105;
pub const E_DUP: u32 = 106;
pub const E_BAD_CONVERT: u32 = 107;
pub const E_REFERENCE: u32 = 108;
pub const E_CALL: u32 = 109;
pub const E_SELF: u32 = 110;
pub const E_CONV: u32 = 111;
pub const E_COLLECTION: u32 = 112;
pub const E_PAIR_OF_POINTS: u32 = 113;
pub const E_CONV_CHARGE: u32 = 114;

/// A package carrying the full composite/structural equality corpus.
pub fn corpus_package() -> PackageBuilder {
    let mut builder = PackageBuilder::default();

    builder
        .code(
            T_BOOLEAN,
            "scalar_type",
            "boolean",
            T_BOOLEAN,
            aggregate(vec![]),
        )
        .code(
            T_INTEGER,
            "scalar_type",
            "integer",
            T_INTEGER,
            aggregate(vec![]),
        )
        .code(T_TEXT, "scalar_type", "text", T_TEXT, aggregate(vec![]))
        .code(
            T_FLOAT64,
            "scalar_type",
            "float64",
            T_FLOAT64,
            aggregate(vec![]),
        )
        .code(
            BD_TEXT,
            "bounded_domain",
            "text_bounds",
            T_TEXT,
            aggregate(vec![
                integer_literal(0),
                integer_literal(16),
                literal("text", "nfc"),
            ]),
        )
        .code(
            T_INTEGER_BOUNDED,
            "scalar_type",
            "integer",
            T_INTEGER_BOUNDED,
            aggregate(vec![]),
        )
        .code(
            BD_INTEGER,
            "bounded_domain",
            "integer_range",
            T_INTEGER_BOUNDED,
            aggregate(vec![integer_literal(-100), integer_literal(100)]),
        )
        .code(
            T_DECIMAL_SMALL,
            "scalar_type",
            "decimal",
            T_DECIMAL_SMALL,
            aggregate(vec![]),
        )
        .code(
            BD_DECIMAL,
            "bounded_domain",
            "decimal_range",
            T_DECIMAL_SMALL,
            aggregate(vec![
                integer_literal(-100),
                integer_literal(100),
                integer_literal(0),
                integer_literal(0),
                literal("text", "nearest-even"),
            ]),
        );

    builder
        .code(
            R_POINT,
            "composite_type",
            "record",
            T_BOOLEAN,
            aggregate(vec![binding("x", T_INTEGER), binding("y", T_INTEGER)]),
        )
        .code(
            R_FLOAT,
            "composite_type",
            "record",
            T_BOOLEAN,
            aggregate(vec![binding("f", T_FLOAT64)]),
        )
        .code(
            CB_SMALL,
            "bounded_domain",
            "collection_bounds",
            T_BOOLEAN,
            aggregate(vec![integer_literal(0), integer_literal(8)]),
        )
        .code(
            SEQ_R_FLOAT,
            "composite_type",
            "sequence",
            T_BOOLEAN,
            aggregate(vec![reference(R_FLOAT), reference(CB_SMALL)]),
        )
        .code(
            TUP_PAIR,
            "composite_type",
            "tuple",
            T_BOOLEAN,
            aggregate(vec![reference(T_INTEGER_BOUNDED), reference(T_TEXT)]),
        )
        .code(
            OPT_INT,
            "composite_type",
            "option",
            T_BOOLEAN,
            aggregate(vec![reference(T_INTEGER)]),
        )
        .code(
            R_DUP,
            "composite_type",
            "record",
            T_BOOLEAN,
            aggregate(vec![binding("x", T_INTEGER), binding("x", T_INTEGER)]),
        )
        .code(
            REF_TYPE,
            "composite_type",
            "reference",
            T_BOOLEAN,
            aggregate(vec![]),
        )
        .code_in_group(
            R_SELF,
            "composite_type",
            "record",
            T_BOOLEAN,
            aggregate(vec![binding("next", OPT_SELF)]),
            "self-cycle",
        )
        .code_in_group(
            OPT_SELF,
            "composite_type",
            "option",
            T_BOOLEAN,
            aggregate(vec![reference(R_SELF)]),
            "self-cycle",
        )
        .code(
            R_PAIR_OF_POINTS,
            "composite_type",
            "record",
            T_BOOLEAN,
            aggregate(vec![binding("a", R_POINT), binding("b", R_POINT)]),
        )
        .code(
            SEQ_INT,
            "composite_type",
            "sequence",
            T_BOOLEAN,
            aggregate(vec![reference(T_INTEGER), reference(CB_SMALL)]),
        );

    builder
        .code(
            M_BARE,
            "model",
            "model_import",
            T_BOOLEAN,
            aggregate(vec![]),
        )
        .code(
            F_BARE,
            "function",
            "pure_function",
            T_BOOLEAN,
            aggregate(vec![]),
        )
        .code(
            S_BARE,
            "state",
            "state_clause",
            T_BOOLEAN,
            aggregate(vec![]),
        )
        .code(
            T_BARE,
            "temporal",
            "temporal_clause",
            T_BOOLEAN,
            aggregate(vec![]),
        );

    builder
        .code(E_RECORD, "expression", "binary", T_BOOLEAN, binary_body())
        .code(
            E_NESTED_IEEE,
            "expression",
            "binary",
            T_BOOLEAN,
            binary_body(),
        )
        .code(E_TUPLE, "expression", "binary", T_BOOLEAN, binary_body())
        .code(E_OPTION, "expression", "binary", T_BOOLEAN, binary_body())
        .code(E_TEXT, "expression", "binary", T_BOOLEAN, binary_body())
        .code(E_ENUM, "expression", "binary", T_BOOLEAN, binary_body())
        .code(E_DUP, "expression", "binary", T_BOOLEAN, binary_body())
        .code(
            E_BAD_CONVERT,
            "expression",
            "binary",
            T_BOOLEAN,
            binary_body(),
        )
        .code(
            E_REFERENCE,
            "expression",
            "binary",
            T_BOOLEAN,
            binary_body(),
        )
        .code(E_CALL, "expression", "call", T_BOOLEAN, binary_body())
        .code(E_SELF, "expression", "binary", T_BOOLEAN, binary_body())
        .code(E_CONV, "expression", "binary", T_BOOLEAN, binary_body())
        .code(
            E_COLLECTION,
            "expression",
            "binary",
            T_BOOLEAN,
            binary_body(),
        )
        .code(
            E_PAIR_OF_POINTS,
            "expression",
            "binary",
            T_BOOLEAN,
            binary_body(),
        )
        .code(
            E_CONV_CHARGE,
            "expression",
            "binary",
            T_BOOLEAN,
            binary_body(),
        );

    builder
}

// ---------------------------------------------------------------------------
// Item builders
// ---------------------------------------------------------------------------

pub fn typed(code: u32) -> EqualityOperandDescriptor {
    EqualityOperandDescriptor::typed(code_id(code))
}

pub fn converted(source: u32, target: u32) -> EqualityOperandDescriptor {
    EqualityOperandDescriptor::converted(code_id(source), code_id(target))
}

pub fn item(
    node: u32,
    operator: EqualityOperatorKind,
    left: EqualityOperandDescriptor,
    right: EqualityOperandDescriptor,
) -> CompositeEqualityItem {
    CompositeEqualityItem {
        node_id: code_id(node),
        operator,
        left,
        right,
    }
}

/// The corpus of items that generate successfully: the committed golden
/// crate and the agreement/compile tests are driven from this list.
/// Deterministic order is not load-bearing here; the generator re-sorts by
/// descriptor key.
pub fn golden_items() -> Vec<CompositeEqualityItem> {
    vec![
        item(
            E_RECORD,
            EqualityOperatorKind::Equal,
            typed(R_POINT),
            typed(R_POINT),
        ),
        item(
            E_RECORD,
            EqualityOperatorKind::NotEqual,
            typed(R_POINT),
            typed(R_POINT),
        ),
        item(
            E_TUPLE,
            EqualityOperatorKind::Equal,
            typed(TUP_PAIR),
            typed(TUP_PAIR),
        ),
        item(
            E_OPTION,
            EqualityOperatorKind::Equal,
            typed(OPT_INT),
            typed(OPT_INT),
        ),
        item(
            E_TEXT,
            EqualityOperatorKind::Equal,
            typed(T_TEXT),
            typed(T_TEXT),
        ),
        item(
            E_ENUM,
            EqualityOperatorKind::Equal,
            EqualityOperandDescriptor::typed(enum_type_id()),
            EqualityOperandDescriptor::typed(enum_type_id()),
        ),
        item(
            E_COLLECTION,
            EqualityOperatorKind::Equal,
            typed(SEQ_INT),
            typed(SEQ_INT),
        ),
        item(
            E_PAIR_OF_POINTS,
            EqualityOperatorKind::Equal,
            typed(R_PAIR_OF_POINTS),
            typed(R_PAIR_OF_POINTS),
        ),
        item(
            E_SELF,
            EqualityOperatorKind::Equal,
            typed(R_SELF),
            typed(R_SELF),
        ),
        item(
            E_CONV,
            EqualityOperatorKind::Equal,
            converted(T_INTEGER_BOUNDED, T_INTEGER),
            typed(T_INTEGER),
        ),
        item(
            E_CONV_CHARGE,
            EqualityOperatorKind::Equal,
            converted(T_INTEGER_BOUNDED, T_DECIMAL_SMALL),
            typed(T_DECIMAL_SMALL),
        ),
    ]
}

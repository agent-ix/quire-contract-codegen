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

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

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

/// `validate_application_keys`'s own preimage version tag (quire-contract-ir
/// dfd8bd78, crates/quire-contract-model/src/checked_package/v2/operations.rs).
const APPLICATION_NODE_VERSION: &str = "quire.application-node/v1";

/// Every code this module ever builds a node for, mapped to its real node
/// id: the computed application digest for an application-bodied node (see
/// [`PackageBuilder::application_code`]), or the readable placeholder
/// [`key`] for a plain node built by [`PackageBuilder::code`]/
/// [`PackageBuilder::code_in_group`] (`validate_application_keys` never
/// re-derives those, so `key(code)` really is their id). [`code_id`] reads
/// it so a caller building `golden_items()`/expected node ids without a
/// `&mut PackageBuilder` in hand still gets the same id IR would.
fn application_registry() -> &'static Mutex<BTreeMap<u32, String>> {
    static REGISTRY: OnceLock<Mutex<BTreeMap<u32, String>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(BTreeMap::new()))
}

/// Registers `code -> digest` once via [`application_registry`]. A second
/// registration for the same `code` must name the same `digest` -- a
/// differing one means two distinct fixture nodes accidentally share one
/// `code`, which silently overwriting would hide: every later [`code_id`]
/// call for that `code`, and every assertion built on it (including a
/// negative one like `!lib.contains(code_id(code).digest.as_ref())`), would
/// then resolve to whichever node happened to register last, without
/// telling a caller the code it asked for isn't the one it thinks it is.
fn register_code(code: u32, digest: String) {
    let mut registry = application_registry().lock().expect("registry lock");
    match registry.get(&code) {
        Some(existing) => assert_eq!(
            *existing, digest,
            "code {code} is already registered as {existing}, cannot also register it as \
             {digest} -- two distinct fixture nodes share one code"
        ),
        None => {
            registry.insert(code, digest);
        }
    }
}

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

/// The node id IR actually assigns for `code`, read from [`application_registry`].
/// Ensures the registry is populated by building the corpus once (discarding
/// the builder) if this is the first call in the process -- `corpus_package`
/// registers every code this module defines via `code`/`code_in_group`/
/// `application_code`, so one build is enough for the whole test binary. No
/// code this module's tests request is ever deliberately left unbuilt, so an
/// unregistered code here is always a fixture defect -- silently falling
/// back to a placeholder would hide it behind whichever assertion the wrong
/// code happened to still satisfy, so this panics instead.
pub fn code_id(code: u32) -> CheckedNodeId {
    if !application_registry()
        .lock()
        .expect("registry lock")
        .contains_key(&code)
    {
        corpus_package();
    }
    let digest = application_registry()
        .lock()
        .expect("registry lock")
        .get(&code)
        .cloned();
    match digest {
        Some(digest) => id(&digest),
        None => panic!(
            "code {code} is not registered by any PackageBuilder constructor -- build it via \
             `code`/`code_in_group`/`application_code` before requesting its id"
        ),
    }
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

/// The corpus's own scalar-type node for a literal `value_kind`. Contract IR
/// (a606059) requires `literal.type` as a member and validates only that it
/// resolves to a real node (FR-038-AC-17); every literal this module builds
/// types itself by kind, matching the vendored `positive-nominal-identities.
/// json` convention (`literal.type` == the node's own `semantic_type`).
fn literal_type(kind: &str) -> String {
    match kind {
        "boolean" => key(T_BOOLEAN),
        "integer" => key(T_INTEGER),
        "text" => key(T_TEXT),
        other => panic!("no corpus scalar type registered for literal kind {other}"),
    }
}

pub fn literal(kind: &str, value: &str) -> Value {
    json!({
        "term": "literal",
        "type": node_ref(&literal_type(kind)),
        "value_kind": kind,
        "value": value,
    })
}

pub fn integer_literal(value: i64) -> Value {
    literal("integer", &value.to_string())
}

/// Contract IR (a606059, FR-038-AC-17) requires `application.operation`
/// and `application.result_type` as members; IR-216's
/// `validate_operations` (quire-contract-ir dfd8bd78) checks `operation`
/// against the closed 135-entry `quire.checked-operation-catalog/v1`
/// (`tests/fixtures/checked-package/checked-package-v2/operation-catalog.json`),
/// so `operation` must name a real catalogued identity, not an opaque
/// placeholder. `operation` is still not read by this crate's own
/// generators (they classify a body by `term`/`operator`/`arguments` and the
/// request item's own descriptor, never by `operation`), so which
/// catalogued identity is used is otherwise irrelevant to what this crate
/// generates; `result_type` names the caller's own declared node type, a
/// node every caller of this helper has already registered via
/// `corpus_package`.
fn application(operator: &str, operation: Value, result_type: u32, arguments: Vec<Value>) -> Value {
    json!({
        "term": "application",
        "operator": operator,
        "operation": operation,
        "result_type": node_ref(&key(result_type)),
        "arguments": arguments,
    })
}

/// The catalogued `operation` member for `quire.op.boolean.eq`: no laws, no
/// mode, no member.
fn boolean_eq() -> Value {
    json!({
        "identity": "quire.op.boolean.eq",
        "laws": [],
        "mode": null,
        "member": null,
        "leaves": [],
    })
}

/// One `literal` operand naming its own declared type via `literal.type`,
/// exactly the member IR-216 already requires and validates only by
/// resolving it to a real node (see [`literal`]'s own doc comment) -- never
/// cross-checked against `value_kind`, so `value_kind`/`value` are a fixed,
/// inert placeholder and `type_ref` alone carries the operand's type.
///
/// This must stay a `literal`, not a `reference`: a `reference` operand's
/// family is resolved and enforced against `boolean.eq`'s own declared
/// `boolean` operand family by IR's `argument_family`/`check_operands`
/// admission check, and a scalar/composite *type* node is never in that
/// family, so a package built from `reference` operands is refused
/// `OperatorIneligible` at *admission*, before this generator ever runs.
fn typed_operand(type_ref: Value) -> Value {
    json!({"term": "literal", "type": type_ref, "value_kind": "integer", "value": "0"})
}

/// A well-formed two-argument `binary` application body: each operand names
/// its own declared type ([`typed_operand`]). The generator still builds the
/// runtime call entirely from the request descriptor, never from a resolved
/// operand *value*, but since
/// `quire_contract_codegen::composite_equality::check_operand_types` (FR-018
/// Behavior: "disagrees with its descriptor's arity or operand types"),
/// every caller of this function must pass the same two type codes its
/// descriptor declares as `source_type`, or the item refuses before
/// generation rather than after. Catalogued as `quire.op.boolean.eq`
/// (binary, two boolean operands): a real, closed-catalog identity every
/// caller can share, structurally conformant regardless of what the caller
/// actually means by the node -- IR's operand-family check never resolves a
/// family for a `literal` operand at all, so it never enforces that
/// declared `boolean` family against these placeholders. `result_type`
/// defaults to `T_BOOLEAN`, the body's actual result type;
/// [`binary_body_with_result`] overrides it only where two callers would
/// otherwise share one (type, type) pair and collide on one preimage digest.
pub fn binary_body(left_type: u32, right_type: u32) -> Value {
    binary_body_with_result(left_type, right_type, T_BOOLEAN)
}

/// [`binary_body`], with an explicit `result_type` disambiguator. `IR-216`
/// validates only that `result_type` resolves to a real node (see
/// [`application`]'s doc comment), so any already-registered code serves.
pub fn binary_body_with_result(left_type: u32, right_type: u32, result_type: u32) -> Value {
    application(
        "binary",
        boolean_eq(),
        result_type,
        vec![
            typed_operand(node_ref(&key(left_type))),
            typed_operand(node_ref(&key(right_type))),
        ],
    )
}

/// [`binary_body`] for an operand type not registered through
/// [`PackageBuilder::code`] -- the vendored `Example.Status` enum node,
/// named by its own digest rather than a placeholder `key(code)`.
pub fn binary_body_digest(left_digest: &str, right_digest: &str) -> Value {
    application(
        "binary",
        boolean_eq(),
        T_BOOLEAN,
        vec![
            typed_operand(node_ref(left_digest)),
            typed_operand(node_ref(right_digest)),
        ],
    )
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Contract IR (a606059, FR-208 `DeclarationTagRules`/`DeclarationOccurrenceRule`)
/// forbids `declaration` on `expression`/`relation`/`state`/`temporal`/
/// `correspondence` nodes and on `value`/`enum_value` nodes, and otherwise
/// requires it exactly when the node carries a `declaration`-role occurrence
/// — which every node built by this module does. The qualified name is not
/// cross-checked against anything else the reader validates (only that each
/// segment is a nonempty ASCII identifier), so a name derived from the
/// node's own digest is sufficient and stays unique by construction.
fn declaration_for(tag: &str, form: &str, digest: &str) -> Option<Value> {
    let forbidden = matches!(
        tag,
        "expression" | "relation" | "state" | "temporal" | "correspondence"
    ) || (tag == "value" && form == "enum_value");
    if forbidden {
        None
    } else {
        Some(json!({"qualified_name": [format!("n{digest}")]}))
    }
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
        self.node_in_group_labeled(
            digest,
            digest,
            tag,
            form,
            semantic_type,
            body,
            recursion_group,
        )
    }

    /// As [`Self::node_in_group`], but the declaration's qualified name is
    /// derived from `label` rather than `digest`. Needed for an
    /// application-bodied node: IR-216's `validate_application_keys`
    /// re-derives `digest` from a preimage that itself embeds `declaration`,
    /// so `digest` cannot be known before `declaration` is built from
    /// something else -- [`Self::application_code`] uses the node's own
    /// `code` as that something else.
    fn node_in_group_labeled(
        &mut self,
        digest: &str,
        label: &str,
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
        if let Some(declaration) = declaration_for(tag, form, label) {
            node["declaration"] = declaration;
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

    /// Registers one application-bodied `expression` node with the real
    /// `node_id` IR-216's `validate_application_keys` re-derives: the
    /// SHA-256 digest of `{version, node_tag, semantic_form, semantic_type,
    /// declaration, recursion, body}` over sorted-key JSON bytes
    /// (quire-contract-ir dfd8bd78,
    /// crates/quire-contract-model/src/checked_package/v2/operations.rs).
    /// `digest` is not known until `declaration` -- itself part of the
    /// preimage -- is built, so `declaration` is derived from `code` (via
    /// [`declaration_for`]'s `label` parameter) rather than from the digest
    /// this call computes. Every node this module builds via this method is
    /// `expression`-tagged, which `declaration_for` forbids a declaration
    /// on, so `declaration` is always `None` in practice; the preimage still
    /// includes the `None` to match IR's own shape exactly. Also records
    /// `code -> digest` in the module's application registry so
    /// [`code_id`] can look the same digest up without rebuilding the node.
    pub fn application_code(&mut self, code: u32, form: &str, body: Value) -> &mut Self {
        const TAG: &str = "expression";
        let label = code.to_string();
        let declaration = declaration_for(TAG, form, &label);
        let preimage = json!({
            "version": APPLICATION_NODE_VERSION,
            "node_tag": TAG,
            "semantic_form": form,
            "semantic_type": node_ref(&key(T_BOOLEAN)),
            "declaration": declaration,
            "recursion": Value::Null,
            "body": body,
        });
        let digest = sha256_hex(&serde_json::to_vec(&preimage).expect("preimage"));
        register_code(code, digest.clone());
        self.node_in_group_labeled(&digest, &label, TAG, form, &key(T_BOOLEAN), body, None)
    }

    pub fn code(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: u32,
        body: Value,
    ) -> &mut Self {
        register_code(code, key(code));
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
        register_code(code, key(code));
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
/// `Rational[-10, 10; 1, 1]`: a `convert<T>` source admitted into
/// `T_RATIONAL_WIDE` (`admits_equality_conversion`'s `Rational -> Rational`
/// row).
pub const T_RATIONAL_NARROW: u32 = 12;
pub const BD_RATIONAL_NARROW: u32 = 13;
/// `Rational[-100, 100; 1, 5]`: wide enough to admit `T_RATIONAL_NARROW`
/// (`Rational -> Rational`) and `T_DECIMAL_SMALL` (`Decimal -> Rational`).
pub const T_RATIONAL_WIDE: u32 = 14;
pub const BD_RATIONAL_WIDE: u32 = 15;
/// `Rational[-50, 50; 1, 1]`: denominator pinned to `1`, so
/// `admits_equality_conversion`'s `Rational -> Integer/Int/Decimal` row
/// admits `T_INTEGER`.
pub const T_RATIONAL_INT: u32 = 16;
pub const BD_RATIONAL_INT: u32 = 17;
/// `Decimal[-1000, 1000; 0, 2]`: wide enough to admit `T_DECIMAL_SMALL`
/// (`Decimal -> Decimal`).
pub const T_DECIMAL_WIDE: u32 = 18;
pub const BD_DECIMAL_WIDE: u32 = 19;

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
/// Not registered in `corpus_package()`: used only as a `key()`/`code_id()`
/// input to build a standalone `TypeEnvironment` for FR-018-AC-6's negative
/// control, exactly as `R_FLOAT` is reused for its positive one.
pub const R_NOT_FLOAT: u32 = 33;

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
/// FR-018 Behavior's "disagrees with its descriptor's ... operand types"
/// (codegen#82): body operands both reference `T_INTEGER`; every request in
/// this module's tests over this node uses a different descriptor type, so
/// `check_operand_types` refuses it before generation.
pub const E_OPERAND_MISMATCH: u32 = 115;
/// `admits_equality_conversion`'s `Rational -> Rational` row (codegen#83).
pub const E_CONV_RAT_RAT: u32 = 116;
/// `admits_equality_conversion`'s `Rational -> Integer/Int/Decimal` row,
/// exercised against `Integer` (codegen#83).
pub const E_CONV_RAT_INT: u32 = 117;
/// `admits_equality_conversion`'s `Decimal -> Rational` row (codegen#83).
pub const E_CONV_DEC_RAT: u32 = 118;
/// `admits_equality_conversion`'s `Decimal -> Decimal` row (codegen#83).
pub const E_CONV_DEC_DEC: u32 = 119;
/// `admits_equality_conversion`'s `Decimal -> Integer/Int` row, exercised
/// against `Integer` (codegen#83).
pub const E_CONV_DEC_INT: u32 = 120;

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
        )
        .code(
            T_RATIONAL_NARROW,
            "scalar_type",
            "rational",
            T_RATIONAL_NARROW,
            aggregate(vec![]),
        )
        .code(
            BD_RATIONAL_NARROW,
            "bounded_domain",
            "rational_range",
            T_RATIONAL_NARROW,
            aggregate(vec![
                integer_literal(-10),
                integer_literal(10),
                integer_literal(1),
                integer_literal(1),
            ]),
        )
        .code(
            T_RATIONAL_WIDE,
            "scalar_type",
            "rational",
            T_RATIONAL_WIDE,
            aggregate(vec![]),
        )
        .code(
            BD_RATIONAL_WIDE,
            "bounded_domain",
            "rational_range",
            T_RATIONAL_WIDE,
            aggregate(vec![
                integer_literal(-100),
                integer_literal(100),
                integer_literal(1),
                integer_literal(5),
            ]),
        )
        .code(
            T_RATIONAL_INT,
            "scalar_type",
            "rational",
            T_RATIONAL_INT,
            aggregate(vec![]),
        )
        .code(
            BD_RATIONAL_INT,
            "bounded_domain",
            "rational_range",
            T_RATIONAL_INT,
            aggregate(vec![
                integer_literal(-50),
                integer_literal(50),
                integer_literal(1),
                integer_literal(1),
            ]),
        )
        .code(
            T_DECIMAL_WIDE,
            "scalar_type",
            "decimal",
            T_DECIMAL_WIDE,
            aggregate(vec![]),
        )
        .code(
            BD_DECIMAL_WIDE,
            "bounded_domain",
            "decimal_range",
            T_DECIMAL_WIDE,
            aggregate(vec![
                integer_literal(-1000),
                integer_literal(1000),
                integer_literal(0),
                integer_literal(2),
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
        .application_code(E_RECORD, "binary", binary_body(R_POINT, R_POINT))
        .application_code(
            E_NESTED_IEEE,
            "binary",
            binary_body(SEQ_R_FLOAT, SEQ_R_FLOAT),
        )
        .application_code(E_TUPLE, "binary", binary_body(TUP_PAIR, TUP_PAIR))
        .application_code(E_OPTION, "binary", binary_body(OPT_INT, OPT_INT))
        .application_code(E_TEXT, "binary", binary_body(T_TEXT, T_TEXT))
        .application_code(
            E_ENUM,
            "binary",
            binary_body_digest(ENUM_TYPE_DIGEST, ENUM_TYPE_DIGEST),
        )
        .application_code(E_DUP, "binary", binary_body(R_DUP, R_DUP))
        // Same (T_TEXT, T_TEXT) operand-reference pair as E_TEXT: a distinct
        // `result_type` keeps the two application preimages from colliding
        // (see `binary_body_with_result`'s doc comment).
        .application_code(
            E_BAD_CONVERT,
            "binary",
            binary_body_with_result(T_TEXT, T_TEXT, T_INTEGER),
        )
        .application_code(E_REFERENCE, "binary", binary_body(REF_TYPE, T_INTEGER))
        .application_code(E_CALL, "call", binary_body(T_INTEGER, T_INTEGER))
        .application_code(E_SELF, "binary", binary_body(R_SELF, R_SELF))
        .application_code(E_CONV, "binary", binary_body(T_INTEGER_BOUNDED, T_INTEGER))
        .application_code(E_COLLECTION, "binary", binary_body(SEQ_INT, SEQ_INT))
        .application_code(
            E_PAIR_OF_POINTS,
            "binary",
            binary_body(R_PAIR_OF_POINTS, R_PAIR_OF_POINTS),
        )
        .application_code(
            E_CONV_CHARGE,
            "binary",
            binary_body(T_INTEGER_BOUNDED, T_DECIMAL_SMALL),
        )
        // FR-018 Behavior's operand-type disagreement refusal (codegen#82):
        // every request over this node in this module's tests declares a
        // descriptor type other than T_INTEGER, so the body's own
        // (T_INTEGER, T_INTEGER) reference pair always disagrees.
        .application_code(
            E_OPERAND_MISMATCH,
            "binary",
            binary_body(T_INTEGER, T_INTEGER),
        )
        .application_code(
            E_CONV_RAT_RAT,
            "binary",
            binary_body(T_RATIONAL_NARROW, T_RATIONAL_WIDE),
        )
        .application_code(
            E_CONV_RAT_INT,
            "binary",
            binary_body(T_RATIONAL_INT, T_INTEGER),
        )
        .application_code(
            E_CONV_DEC_RAT,
            "binary",
            binary_body(T_DECIMAL_SMALL, T_RATIONAL_WIDE),
        )
        .application_code(
            E_CONV_DEC_DEC,
            "binary",
            binary_body(T_DECIMAL_SMALL, T_DECIMAL_WIDE),
        )
        .application_code(
            E_CONV_DEC_INT,
            "binary",
            binary_body(T_DECIMAL_SMALL, T_INTEGER),
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

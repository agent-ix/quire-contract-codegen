//! Admitted CheckedPackage V2 fixtures for composite/structural equality
//! generation (FR-018).
//!
//! Built on the same `tests/checked_package_support/base.rs` base as
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

#[path = "codes.rs"]
mod codes;
pub use codes::*;

pub const NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";

/// `validate_application_keys`'s own preimage version tag (Contract IR
/// crates/quire-contract-model/src/checked_package/v2/operations.rs).
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

/// The base package's enum declaration node key.
pub fn enum_type_digest() -> String {
    base_node_ids().enum_type
}

pub fn enum_type_id() -> CheckedNodeId {
    id(&enum_type_digest())
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

/// A bound-body member in checked-package v2's shape (FR-322): a `binding` term naming the
/// member and carrying its literal, not the bare literal.
pub fn bound_member(name: &str, value: Value) -> Value {
    json!({"term": "binding", "name": name, "value": value})
}

pub fn aggregate(members: Vec<Value>) -> Value {
    json!({"term": "aggregate", "members": members})
}

/// The corpus's own scalar-type node for a literal `value_kind`. Contract IR
/// requires `literal.type` as a member and validates only that it
/// resolves to a real node (FR-038-AC-17); every literal this module builds
/// types itself by kind, matching the base package's convention
/// (`literal.type` == the node's own `semantic_type`).
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

/// Contract IR (FR-038-AC-17) requires `application.operation`
/// and `application.result_type` as members; Contract IR's
/// `validate_operations` checks `operation` against the checked-operation
/// catalog `quire-verification-contracts` publishes, so `operation` must name a real catalogued identity, not an opaque
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
/// [`PackageBuilder::code`] -- the base package's `Example.Phase` enum node,
/// named by its own node key rather than a placeholder `key(code)`.
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

/// Contract IR (FR-208 `DeclarationTagRules`/`DeclarationOccurrenceRule`)
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

include!("../checked_package_support/base.rs");

impl Default for PackageBuilder {
    fn default() -> Self {
        Self {
            value: base_package(),
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
    /// (Contract IR,
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
// The composite/structural corpus (codes in `codes.rs`)
// ---------------------------------------------------------------------------

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
                bound_member("min", integer_literal(0)),
                bound_member("max", integer_literal(16)),
                bound_member("text_profile", literal("text", "nfc")),
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
            aggregate(vec![
                bound_member("min", integer_literal(-100)),
                bound_member("max", integer_literal(100)),
            ]),
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
                bound_member("coefficient_min", integer_literal(-100)),
                bound_member("coefficient_max", integer_literal(100)),
                bound_member("scale_min", integer_literal(0)),
                bound_member("scale_max", integer_literal(0)),
                bound_member("rounding", literal("text", "nearest-even")),
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
                bound_member("numerator_min", integer_literal(-10)),
                bound_member("numerator_max", integer_literal(10)),
                bound_member("denominator_min", integer_literal(1)),
                bound_member("denominator_max", integer_literal(1)),
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
                bound_member("numerator_min", integer_literal(-100)),
                bound_member("numerator_max", integer_literal(100)),
                bound_member("denominator_min", integer_literal(1)),
                bound_member("denominator_max", integer_literal(5)),
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
                bound_member("numerator_min", integer_literal(-50)),
                bound_member("numerator_max", integer_literal(50)),
                bound_member("denominator_min", integer_literal(1)),
                bound_member("denominator_max", integer_literal(1)),
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
                bound_member("coefficient_min", integer_literal(-1000)),
                bound_member("coefficient_max", integer_literal(1000)),
                bound_member("scale_min", integer_literal(0)),
                bound_member("scale_max", integer_literal(2)),
                bound_member("rounding", literal("text", "nearest-even")),
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
            aggregate(vec![
                bound_member("min", integer_literal(0)),
                bound_member("max", integer_literal(8)),
            ]),
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
            binary_body_digest(&enum_type_digest(), &enum_type_digest()),
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

/// The corpus of items that generate successfully: the agreement and
/// compile tests build their crate from this list.
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
        // codegen#83: one vector per remaining `admits_equality_conversion`
        // row (`Int -> *` is already exercised above by E_CONV/
        // E_CONV_CHARGE).
        item(
            E_CONV_RAT_RAT,
            EqualityOperatorKind::Equal,
            converted(T_RATIONAL_NARROW, T_RATIONAL_WIDE),
            typed(T_RATIONAL_WIDE),
        ),
        item(
            E_CONV_RAT_INT,
            EqualityOperatorKind::Equal,
            converted(T_RATIONAL_INT, T_INTEGER),
            typed(T_INTEGER),
        ),
        item(
            E_CONV_DEC_RAT,
            EqualityOperatorKind::Equal,
            converted(T_DECIMAL_SMALL, T_RATIONAL_WIDE),
            typed(T_RATIONAL_WIDE),
        ),
        item(
            E_CONV_DEC_DEC,
            EqualityOperatorKind::Equal,
            converted(T_DECIMAL_SMALL, T_DECIMAL_WIDE),
            typed(T_DECIMAL_WIDE),
        ),
        item(
            E_CONV_DEC_INT,
            EqualityOperatorKind::Equal,
            converted(T_DECIMAL_SMALL, T_INTEGER),
            typed(T_INTEGER),
        ),
    ]
}

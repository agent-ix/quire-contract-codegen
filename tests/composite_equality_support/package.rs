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

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

use quire_contract_codegen::{
    CompositeEqualityItem, EqualityOperandDescriptor, EqualityOperatorKind,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult,
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
fn registered_digest(code: u32) -> String {
    application_registry()
        .lock()
        .expect("registry lock")
        .get(&code)
        .cloned()
        .unwrap_or_else(|| panic!("code {code} is not registered yet"))
}

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

/// The operand family (FR-322 operation catalog) of a corpus type node, by code: what
/// Contract IR resolves for a `literal` operand typed by that node.
fn family_of(code: u32) -> &'static str {
    match code {
        T_BOOLEAN => "boolean",
        T_INTEGER | T_INTEGER_BOUNDED => "integer",
        T_TEXT => "text",
        T_DECIMAL_SMALL | T_DECIMAL_WIDE => "decimal",
        T_RATIONAL_NARROW | T_RATIONAL_WIDE | T_RATIONAL_INT => "rational",
        R_POINT | R_FLOAT | R_DUP | R_SELF | R_PAIR_OF_POINTS | R_WITH_REF => "structural",
        SEQ_R_FLOAT | SEQ_INT | TUP_PAIR | OPT_INT | OPT_SELF => "structural",
        REF_TYPE => "reference",
        other => panic!("no operand family registered for corpus type code {other}"),
    }
}

/// The `quire.op.<family>.eq` operation FR-093's `Equality` row lowers an equality over
/// `family` operands to: Contract IR checks every operand's family against the operation's
/// declared operands, so the identity must match the operands' own type. `text` carries its
/// `text_profile` law and mode, selected by [`PackageBuilder::select_definition`]; `structural`
/// carries no leaves here, see [`operation_for`].
fn equality_operation(family: &str) -> Value {
    let plain = |identity: &str| {
        json!({
            "identity": identity,
            "laws": [],
            "mode": null,
            "member": null,
            "leaves": [],
        })
    };
    match family {
        "text" => json!({
            "identity": "quire.op.text.eq",
            "laws": [{"role": "text_profile", "definition": text_profile_definition()}],
            "mode": {"kind": "text_profile", "value": "nfc"},
            "member": null,
            "leaves": [],
        }),
        other => plain(&format!("quire.op.{other}.eq")),
    }
}

/// The equality operation for operands of type `code`. A structural equality lists one leaf
/// per `text` leaf of the compared type (FR-322), each with the `text_profile` law and mode, and
/// none for a type with no text; Contract IR counts them against the type.
fn operation_for(code: u32) -> Value {
    let family = family_of(code);
    let mut operation = equality_operation(family);
    if family == "structural" {
        operation["leaves"] = match code {
            // `Tuple<Int, Text>`: the one text leaf is position 1.
            TUP_PAIR => json!([{
                "path": ["position:1"],
                "laws": [{"role": "text_profile", "definition": text_profile_definition()}],
                "mode": {"kind": "text_profile", "value": "nfc"},
            }]),
            _ => json!([]),
        };
    }
    operation
}

/// The catalog's `text_profile` law definition, read from the catalog's home.
fn text_profile_definition() -> Value {
    let catalog: Value = serde_json::from_str(
        quire_verification_contracts::operation_catalog::CHECKED_OPERATION_CATALOG_V1,
    )
    .expect("the operation catalog is JSON");
    catalog["law_roles"]["text_profile"][0].clone()
}

/// The first code of the parameter nodes: one `value`/`parameter` node per operand type, at
/// `PARAMETER_BASE + <type code>`.
const PARAMETER_BASE: u32 = 3000;
/// The parameter typed by the base package's enum declaration, which has no corpus code.
const ENUM_PARAMETER: u32 = 3999;
/// The first code of the `convert` nodes [`PackageBuilder::converting_equality`] adds: the
/// conversion for equality node `code` is node `CONVERSION_BASE + code`.
const CONVERSION_BASE: u32 = 4000;
/// The operand types a corpus equality compares, each of which gets a parameter node.
const PARAMETER_TYPES: [u32; 18] = [
    T_INTEGER,
    T_INTEGER_BOUNDED,
    T_TEXT,
    T_DECIMAL_SMALL,
    T_DECIMAL_WIDE,
    T_RATIONAL_NARROW,
    T_RATIONAL_WIDE,
    T_RATIONAL_INT,
    R_POINT,
    R_DUP,
    R_SELF,
    R_PAIR_OF_POINTS,
    R_WITH_REF,
    SEQ_R_FLOAT,
    SEQ_INT,
    TUP_PAIR,
    OPT_INT,
    REF_TYPE,
];

/// The code of the parameter node typed by `type_code`.
fn parameter_code(type_code: u32) -> u32 {
    PARAMETER_BASE + type_code
}

/// The body QSL emits for a parameter's `value` node (form `parameter`): an `aggregate` of its
/// `name` (a `text` literal) and its `level` (an `integer` literal) as `binding` members.
fn parameter_body(name: &str) -> Value {
    aggregate(vec![
        bound_member("name", literal("text", name)),
        bound_member("level", integer_literal(0)),
    ])
}

/// A `reference` term to the node with digest `digest`.
fn reference_to(digest: &str) -> Value {
    json!({"term": "reference", "target": node_ref(digest)})
}

/// A `reference` operand to the parameter typed by `type_code`. QSL keys every operand as its
/// own node and puts a `reference` to it in the equality's `arguments`; the generator reads the
/// operand's type from that node, and Contract IR resolves its family from the node's
/// `semantic_type`.
fn operand(type_code: u32) -> Value {
    reference(parameter_code(type_code))
}

/// A well-formed two-argument `binary` application body: each operand is a `reference` to a
/// parameter node typed by its own declared type ([`operand`]). The generator still builds the
/// runtime call entirely from the request descriptor, never from a resolved
/// operand *value*, but since
/// `quire_contract_codegen::composite_equality::check_operand_types` (FR-018
/// Behavior: "disagrees with its descriptor's arity or operand types"),
/// every caller of this function must pass the same two type codes its
/// descriptor declares as `source_type`, or the item refuses before
/// generation rather than after. The `operation` is the equality FR-093 lowers
/// the left operand's family to ([`operation_for`]): Contract IR checks each
/// operand's family against it, so a body whose operands are not of that
/// family is refused `OperatorIneligible` at admission. `result_type`
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
        operation_for(left_type),
        result_type,
        vec![operand(left_type), operand(right_type)],
    )
}

/// [`binary_body`] over two operands of the base package's `Example.Phase` enum, which has no
/// corpus code: its own `quire.op.enum.eq` is the operation.
pub fn binary_body_enum() -> Value {
    application(
        "binary",
        equality_operation("enum"),
        T_BOOLEAN,
        vec![reference(ENUM_PARAMETER), reference(ENUM_PARAMETER)],
    )
}

/// The body of a `quire.op.numeric.convert` application: `argument` converted to `target`,
/// with QSL's `type_argument` member naming the `target` type node as its declaration.
fn convert_body(argument: Value, target: u32) -> Value {
    application(
        "convert",
        json!({
            "identity": "quire.op.numeric.convert",
            "laws": [],
            "mode": null,
            "member": {"kind": "type_argument", "declaration": node_ref(&key(target))},
            "leaves": [],
        }),
        target,
        vec![argument],
    )
}

/// FR-322's application-node dependency join for `body`: every `reference` target and every
/// operation member `declaration` of an `application` term, deduplicated and digest-ascending.
/// `result_type` and a `literal`'s own `type` are not dependencies.
fn application_dependencies(body: &Value) -> Vec<String> {
    fn walk(value: &Value, targets: &mut BTreeSet<String>) {
        match value {
            Value::Object(map) => {
                match map.get("term").and_then(Value::as_str) {
                    Some("reference") => {
                        if let Some(digest) = map["target"]["digest"].as_str() {
                            targets.insert(digest.to_owned());
                        }
                    }
                    Some("application") => {
                        if let Some(digest) =
                            map["operation"]["member"]["declaration"]["digest"].as_str()
                        {
                            targets.insert(digest.to_owned());
                        }
                    }
                    _ => {}
                }
                for member in map.values() {
                    walk(member, targets);
                }
            }
            Value::Array(items) => items.iter().for_each(|item| walk(item, targets)),
            _ => {}
        }
    }
    let mut targets = BTreeSet::new();
    walk(body, &mut targets);
    targets.into_iter().collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Contract IR (FR-208 `DeclarationTagRules`/`DeclarationOccurrenceRule`)
/// forbids `declaration` on `expression`/`relation`/`state`/`temporal`/
/// `correspondence` nodes and on `value`/`enum_value` and `value`/`parameter` nodes (QSL emits a
/// parameter with no declaration), and otherwise requires it exactly when the node carries a
/// `declaration`-role occurrence — which every node built by this module does but a parameter,
/// whose occurrence role is `expression`. The qualified name is not
/// cross-checked against anything else the reader validates (only that each
/// segment is a nonempty ASCII identifier), so a name derived from the
/// node's own digest is sufficient and stays unique by construction.
fn declaration_for(tag: &str, form: &str, digest: &str) -> Option<Value> {
    let forbidden = matches!(
        tag,
        "expression" | "relation" | "state" | "temporal" | "correspondence"
    ) || (tag == "value" && matches!(form, "enum_value" | "parameter"));
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
        // The schema's `BodyBindingRules` fix a parameter's occurrence role.
        let role = if (tag, form) == ("value", "parameter") {
            "expression"
        } else {
            "declaration"
        };
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
            "occurrences": [{"role": role, "ordinal": 0}],
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
            "role": role,
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
        self.application_node(code, form, &key(T_BOOLEAN), body)
    }

    /// As [`Self::application_code`], with the node typed by `semantic_type`. FR-322's
    /// application-node dependency join fixes the node's `dependencies`: exactly the unique,
    /// digest-ascending `reference` targets and operation member declarations of its body.
    pub fn application_node(
        &mut self,
        code: u32,
        form: &str,
        semantic_type: &str,
        body: Value,
    ) -> &mut Self {
        self.application_node_tagged(code, "expression", form, semantic_type, body)
    }

    /// As [`Self::application_node`], with the node under `tag` rather than `expression`: the
    /// `temporal` clause and formula nodes carry an application body too.
    pub fn application_node_tagged(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
    ) -> &mut Self {
        let label = code.to_string();
        let declaration = declaration_for(tag, form, &label);
        let preimage = json!({
            "version": APPLICATION_NODE_VERSION,
            "node_tag": tag,
            "semantic_form": form,
            "semantic_type": node_ref(semantic_type),
            "declaration": declaration,
            "recursion": Value::Null,
            "body": body,
        });
        let digest = sha256_hex(&serde_json::to_vec(&preimage).expect("preimage"));
        register_code(code, digest.clone());
        let dependencies = application_dependencies(&body);
        self.node_in_group_labeled(&digest, &label, tag, form, semantic_type, body, None);
        let node = self.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .and_then(|nodes| nodes.last_mut())
            .expect("the node just added");
        node["dependencies"] = Value::Array(dependencies.iter().map(|d| node_ref(d)).collect());
        self
    }

    /// Adds `count` Boolean equalities, node `BYTE_CHAIN_BASE + k` comparing node
    /// `BYTE_CHAIN_BASE + k - 1` with itself (the first compares a literal with itself). Each node
    /// reaches every node before it, so a lowered node lists its whole chain in `dependencies`:
    /// the lowered contract package of a call requesting the chain grows quadratically in `count`
    /// while the checked package grows linearly (FR-018-AC-20).
    pub fn boolean_equality_chain(&mut self, count: u32) -> &mut Self {
        self.boolean_equality_chain_from(BYTE_CHAIN_BASE, count, "true")
    }

    /// As [`Self::boolean_equality_chain`], numbering the chain from `base` and starting it from
    /// the Boolean literal `seed`. Two chains in one package need different seeds: a node's id is
    /// derived from its body, and two first nodes over one literal would share an id.
    pub fn boolean_equality_chain_from(&mut self, base: u32, count: u32, seed: &str) -> &mut Self {
        let equality = |operand: Value| {
            application(
                "binary",
                equality_operation("boolean"),
                T_BOOLEAN,
                vec![operand.clone(), operand],
            )
        };
        self.application_code(base, "binary", equality(literal("boolean", seed)));
        for offset in 1..count {
            let previous = registered_digest(base + offset - 1);
            self.application_code(base + offset, "binary", equality(reference_to(&previous)));
        }
        self
    }

    /// An equality node `code` whose left operand is a conversion, spelled as QSL spells
    /// `convert<T>`: a `quire.op.numeric.convert` `expression` node of its own (node
    /// `CONVERSION_BASE + code`, typed `target`, with the `type_argument` member naming the
    /// `target` type node as its declaration) over the `source`-typed parameter, and a
    /// `reference` to that node as the equality's first argument. The right operand is the
    /// `right_type` parameter. The equality is then over two `target`-family operands, so
    /// Contract IR admits it where two operands of different families would be
    /// `OperatorIneligible`.
    pub fn converting_equality(
        &mut self,
        code: u32,
        source: u32,
        target: u32,
        right_type: u32,
    ) -> &mut Self {
        let conversion = CONVERSION_BASE + code;
        self.application_node(
            conversion,
            "conversion",
            &key(target),
            convert_body(operand(source), target),
        );
        let conversion_digest = registered_digest(conversion);
        self.application_code(
            code,
            "binary",
            application(
                "binary",
                operation_for(right_type),
                T_BOOLEAN,
                vec![reference_to(&conversion_digest), operand(right_type)],
            ),
        )
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

    /// Registers `definition` under `role` in `lock.profile_selections` and the identity
    /// preimage's copy (deduplicated): IR admits a `temporal_profile` law only when the lock
    /// selected its definition under that role.
    pub fn select_profile(&mut self, role: &str, definition: Value) -> &mut Self {
        let selection = json!({"role": role, "definition": definition});
        for path in ["lock", "identity_preimage"] {
            let selections = self.value[path]["profile_selections"]
                .as_array_mut()
                .expect("profile_selections");
            if !selections.contains(&selection) {
                selections.push(selection.clone());
            }
        }
        self
    }

    /// Registers `definition` in `lock.definition_selections` and the identity preimage's copy
    /// (deduplicated): Contract IR refuses an operation law whose definition the lock did not
    /// select.
    pub fn select_definition(&mut self, definition: Value) -> &mut Self {
        for path in ["lock", "identity_preimage"] {
            let selections = self.value[path]["definition_selections"]
                .as_array_mut()
                .expect("definition_selections");
            if !selections.contains(&definition) {
                selections.push(definition.clone());
            }
        }
        self
    }

    /// Replaces the body of the node `code` built by [`Self::code`].
    pub fn set_body(&mut self, code: u32, body: Value) -> &mut Self {
        let digest = key(code);
        let node = self.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("nodes")
            .iter_mut()
            .find(|node| node["node_id"]["digest"].as_str() == Some(digest.as_str()))
            .unwrap_or_else(|| panic!("no node for code {code}"));
        node["body"] = body;
        self
    }

    /// The reader's verdict on this package, admitted or refused, with the wire it read.
    pub fn read(&self) -> (CheckedPackageV2ReadResult, Value) {
        let wire = self.wire();
        let bytes = serde_json::to_vec(&wire).expect("canonical bytes");
        let result =
            CheckedPackageV2::read(&bytes, CheckedPackageReadLimits::bounded(), &evidence());
        (result, wire)
    }

    pub fn admit(&self) -> CheckedPackageV2 {
        self.admit_with(CheckedPackageReadLimits::bounded())
    }

    /// As [`Self::admit`], reading under `limits`.
    pub fn admit_with(&self, limits: CheckedPackageReadLimits) -> CheckedPackageV2 {
        let wire = self.wire();
        let bytes = serde_json::to_vec(&wire).expect("canonical bytes");
        match CheckedPackageV2::read(&bytes, limits, &evidence()) {
            CheckedPackageV2ReadResult::Admitted(package) => *package,
            other => panic!("expected V2 admission, got {other:?}"),
        }
    }
}

fn evidence() -> CheckedPackageEvidence {
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
}

// ---------------------------------------------------------------------------
// The composite/structural corpus (codes in `codes.rs`)
// ---------------------------------------------------------------------------

/// [`corpus_package`] plus [`E_REFERENCE_DIRECT`]: `quire.op.reference.eq` over two `REF_TYPE`
/// operands. Contract IR refuses it (`reference.eq`'s `conforming_reference` constraint needs a
/// `Reference<X>` naming a model object type of a selected document, and `REF_TYPE` names none),
/// so it cannot be in the corpus; `tc_029_ac7_a_direct_reference_operand_is_refused_by_ir_today`
/// pins the refusal.
pub fn direct_reference_package() -> PackageBuilder {
    let mut builder = corpus_package();
    builder.application_code(
        E_REFERENCE_DIRECT,
        "binary",
        binary_body(REF_TYPE, REF_TYPE),
    );
    builder
}

/// [`corpus_package`] with `TUP_PAIR`'s members replaced by references to `members` (the text
/// leaf must stay at position 1) and the variant nodes `extras` added first, so a test can
/// reference a `bounded_domain` node, or a scalar, as a member's type (FR-018-AC-16).
pub fn tuple_members_package(extras: &[u32], members: &[u32]) -> PackageBuilder {
    let mut builder = corpus_package();
    let text_members = |profile_member: bool| {
        let mut bounds = vec![
            bound_member("min", integer_literal(0)),
            bound_member("max", integer_literal(16)),
        ];
        if profile_member {
            bounds.push(bound_member("text_profile", literal("text", "nfc")));
        }
        aggregate(bounds)
    };
    for extra in extras {
        match *extra {
            BD_TEXT_SIBLING => builder.code(
                BD_TEXT_SIBLING,
                "bounded_domain",
                "text_bounds",
                T_TEXT,
                aggregate(vec![
                    bound_member("min", integer_literal(1)),
                    bound_member("max", integer_literal(5)),
                    bound_member("text_profile", literal("text", "nfc")),
                ]),
            ),
            BD_INTEGER_WRONG_FORM => builder.code(
                BD_INTEGER_WRONG_FORM,
                "bounded_domain",
                "text_bounds",
                T_INTEGER_BOUNDED,
                text_members(true),
            ),
            BD_BOOLEAN => builder.code(
                BD_BOOLEAN,
                "bounded_domain",
                "integer_range",
                T_BOOLEAN,
                aggregate(vec![
                    bound_member("min", integer_literal(0)),
                    bound_member("max", integer_literal(1)),
                ]),
            ),
            BD_ENUM => {
                register_code(BD_ENUM, key(BD_ENUM));
                builder.node(
                    &key(BD_ENUM),
                    "bounded_domain",
                    "integer_range",
                    &enum_type_digest(),
                    aggregate(vec![
                        bound_member("min", integer_literal(0)),
                        bound_member("max", integer_literal(1)),
                    ]),
                )
            }
            BD_OVER_COMPOSITE => builder.code(
                BD_OVER_COMPOSITE,
                "bounded_domain",
                "collection_bounds",
                R_POINT,
                text_members(false),
            ),
            BD_FLOAT_ROUNDING => builder.code(
                BD_FLOAT_ROUNDING,
                "bounded_domain",
                "float_rounding",
                T_FLOAT64,
                aggregate(vec![bound_member(
                    "rounding",
                    literal("text", "nearest-even"),
                )]),
            ),
            other => panic!("no variant node registered for code {other}"),
        };
    }
    builder.set_body(
        TUP_PAIR,
        aggregate(members.iter().map(|member| reference(*member)).collect()),
    );
    builder
}

/// [`corpus_package`] plus [`E_NESTED_CONV`]: the first operand is a conversion of a
/// conversion (`T_INTEGER_BOUNDED` to `T_RATIONAL_WIDE` to `T_DECIMAL_WIDE`), each its own node, so the
/// operand's source type is the innermost operand's type, not the inner conversion's result.
pub fn nested_conversion_package() -> PackageBuilder {
    let mut builder = corpus_package();
    let inner = CONVERSION_BASE + E_NESTED_CONV;
    let outer = inner + 1000;
    builder.application_node(
        inner,
        "conversion",
        &key(T_RATIONAL_WIDE),
        convert_body(operand(T_INTEGER_BOUNDED), T_RATIONAL_WIDE),
    );
    let inner_operand = reference_to(&registered_digest(inner));
    builder.application_node(
        outer,
        "conversion",
        &key(T_DECIMAL_WIDE),
        convert_body(inner_operand, T_DECIMAL_WIDE),
    );
    let outer_operand = reference_to(&registered_digest(outer));
    builder.application_code(
        E_NESTED_CONV,
        "binary",
        application(
            "binary",
            operation_for(T_DECIMAL_WIDE),
            T_BOOLEAN,
            vec![outer_operand, operand(T_DECIMAL_WIDE)],
        ),
    );
    builder
}

/// [`corpus_package`] plus [`E_APPLICATION_OPERAND`]: the first operand is a `rational.div`
/// application node over two integer parameters, typed `T_RATIONAL_WIDE`. It is not a
/// conversion, so its type is its own `semantic_type`, never its first argument's.
pub fn application_operand_package() -> PackageBuilder {
    let mut builder = corpus_package();
    let division = 6000 + E_APPLICATION_OPERAND;
    builder.application_node(
        division,
        "binary",
        &key(T_RATIONAL_WIDE),
        application(
            "binary",
            json!({
                "identity": "quire.op.rational.div",
                "laws": [],
                "mode": null,
                "member": null,
                "leaves": [],
            }),
            T_RATIONAL_WIDE,
            vec![operand(T_INTEGER), operand(T_INTEGER)],
        ),
    );
    let division_operand = reference_to(&registered_digest(division));
    builder.application_code(
        E_APPLICATION_OPERAND,
        "binary",
        application(
            "binary",
            operation_for(T_RATIONAL_WIDE),
            T_BOOLEAN,
            vec![division_operand, operand(T_RATIONAL_WIDE)],
        ),
    );
    builder
}

/// A package carrying the full composite/structural equality corpus.
pub fn corpus_package() -> PackageBuilder {
    let mut builder = PackageBuilder::default();

    builder.select_definition(text_profile_definition());
    for type_code in PARAMETER_TYPES {
        builder.code(
            parameter_code(type_code),
            "value",
            "parameter",
            type_code,
            parameter_body(&format!("p{type_code}")),
        );
    }
    builder.node(
        &key(ENUM_PARAMETER),
        "value",
        "parameter",
        &enum_type_digest(),
        parameter_body("phase"),
    );
    register_code(ENUM_PARAMETER, key(ENUM_PARAMETER));
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
            // The text position names the `text_bounds` node, as QSL emits a text type: the
            // checked operation catalog's leaf rule reads the profile pin from the type chain
            // and refuses a text leaf whose type pins none.
            aggregate(vec![reference(T_INTEGER_BOUNDED), reference(BD_TEXT)]),
        )
        .code(
            OPT_INT,
            "composite_type",
            "option",
            T_BOOLEAN,
            aggregate(vec![reference(T_INTEGER)]),
        )
        .code(
            R_WITH_REF,
            "composite_type",
            "record",
            T_BOOLEAN,
            aggregate(vec![binding("r", REF_TYPE)]),
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
        .code(S_BARE, "state", "snapshot", T_BOOLEAN, aggregate(vec![]));

    // IR admits a `temporal_clause` only as a `temporal`-operator application over a declared
    // `parameter` with one `temporal_profile` law and a `temporal` formula argument (the QSpec
    // temporal-clause rule), so `T_BARE` is that minimal clause. CG refuses it as an
    // unsupported temporal family. The formula is registered first: the clause's reference to
    // it reads its node id.
    let clause_profile = json!({
        "authority": "agent-ix",
        "identity": "quire.temporal.event-position.false-extension/v1",
    });
    builder.select_profile("temporal_profile", clause_profile.clone());
    builder.application_node_tagged(
        T_BARE_FORMULA,
        "temporal",
        "formula",
        &key(T_BOOLEAN),
        application(
            "temporal_formula",
            json!({
                "identity": "quire.op.temporal.true",
                "laws": [], "mode": null, "member": null, "leaves": [],
            }),
            T_BOOLEAN,
            Vec::new(),
        ),
    );
    let clause_formula = registered_digest(T_BARE_FORMULA);
    builder.application_node_tagged(
        T_BARE,
        "temporal",
        "temporal_clause",
        &key(T_BOOLEAN),
        application(
            "temporal",
            json!({
                "identity": "quire.op.temporal.clause",
                "laws": [{"role": "temporal_profile", "definition": clause_profile}],
                "mode": null, "member": null, "leaves": [],
            }),
            T_BOOLEAN,
            vec![
                reference(parameter_code(T_INTEGER)),
                literal("text", "c"),
                aggregate(vec![]),
                aggregate(vec![]),
                aggregate(vec![]),
                reference_to(&clause_formula),
            ],
        ),
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
        .application_code(E_ENUM, "binary", binary_body_enum())
        .application_code(E_DUP, "binary", binary_body(R_DUP, R_DUP))
        // Same (T_TEXT, T_TEXT) operand-reference pair as E_TEXT: a distinct
        // `result_type` keeps the two application preimages from colliding
        // (see `binary_body_with_result`'s doc comment).
        .application_code(
            E_BAD_CONVERT,
            "binary",
            binary_body_with_result(T_TEXT, T_TEXT, T_INTEGER),
        )
        .application_code(E_REFERENCE, "binary", binary_body(R_WITH_REF, R_WITH_REF))
        .application_code(E_CALL, "call", binary_body(T_INTEGER, T_INTEGER))
        .application_code(E_SELF, "binary", binary_body(R_SELF, R_SELF))
        .converting_equality(E_CONV, T_INTEGER_BOUNDED, T_INTEGER, T_INTEGER)
        .application_code(E_COLLECTION, "binary", binary_body(SEQ_INT, SEQ_INT))
        .application_code(
            E_PAIR_OF_POINTS,
            "binary",
            binary_body(R_PAIR_OF_POINTS, R_PAIR_OF_POINTS),
        )
        .converting_equality(
            E_CONV_CHARGE,
            T_INTEGER_BOUNDED,
            T_DECIMAL_SMALL,
            T_DECIMAL_SMALL,
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
        .converting_equality(
            E_CONV_RAT_RAT,
            T_RATIONAL_NARROW,
            T_RATIONAL_WIDE,
            T_RATIONAL_WIDE,
        )
        .converting_equality(E_CONV_RAT_INT, T_RATIONAL_INT, T_INTEGER, T_INTEGER)
        .converting_equality(
            E_CONV_DEC_RAT,
            T_DECIMAL_SMALL,
            T_RATIONAL_WIDE,
            T_RATIONAL_WIDE,
        )
        .converting_equality(
            E_CONV_DEC_DEC,
            T_DECIMAL_SMALL,
            T_DECIMAL_WIDE,
            T_DECIMAL_WIDE,
        )
        .converting_equality(E_CONV_DEC_INT, T_DECIMAL_SMALL, T_INTEGER, T_INTEGER);

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

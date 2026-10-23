//! Admitted CheckedPackage V2 fixtures for exact scalar generation.
//!
//! The base (`tests/checked_package_support/base.rs`) supplies an enum declaration,
//! one of its members, a dimension and a declared unit under their nominal
//! keys. Scalar types, values and expressions are appended under readable
//! zero-padded keys, and the package identity is re-derived from the
//! finished graph.
//!
//! Bounds are `bounded_domain` nodes keyed by the digest of their content and
//! `foreign` (see `Bound::key`/`Bound::foreign`), and listed in the
//! dependencies of each expression that uses them. Each expression's own
//! `bounds` reaches every `Bound` it declares plus, transitively, every
//! `Bound` those declare as `foreign` (a bound whose body embeds a literal of
//! another requires-bound kind reaches that kind's own bound too) -- so an
//! expression's declared bounds are a lower bound on what it reaches, not
//! the exact set. Bound bodies use the encoding
//! `quire_contract_codegen::exact_scalar` documents.

#![allow(dead_code)] // Each test binary uses a different subset.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

use super::scalar_fixture_types::{
    BooleanOperator, DecimalOperator, EqualityOperandKind, ExactScalarItem, ExactScalarOperation,
    IeeeArithmeticOperator, OrderingOperandKind, QuantityOperator, RationalOperator,
};
use quire_contract_codegen::oracle::operator_catalog::IntegerOperator;
use quire_contract_model::{
    derive_application_node_id, ApplicationNodeDraft, CheckedNodeId, CheckedPackageEvidence,
    CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult, CheckedSemanticNodeV2,
};
use quire_exact::{
    ComparisonOperator, DecimalType, DivisionProfile, IeeeComparison, IeeeWidth, Integer,
    IntegerDomain, IntegerInterval, OrderingOperator, RationalDomain, RoundingMode, TextProfile,
    TextType,
};
use quire_semantic_value::declaration::EqualityOperator;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[path = "../checked_package_support/rekey.rs"]
mod rekey;
pub use rekey::FixtureIds;

pub const NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";
const FIXTURE_MODEL: &str = "test/cg-exact-scalar";
const FIXTURE_OBJECT: &str = "ix://test/cg-exact-scalar/Account";
const FIXTURE_RELATIONSHIP: &str = "ix://test/cg-exact-scalar/relationship/Account-link-Account";

/// The selected declaration to which the corpus's relationship nodes refer.
fn fixture_model_document() -> Value {
    json!({
        "contractVersion": "2.0.0",
        "package": {"identity": FIXTURE_MODEL, "version": "1"},
        "constructs": [{
            "kind": {"module": FIXTURE_MODEL, "name": "entity"},
            "construct": {"meaning": "quire.meaning.model.object-type/v1"},
        }],
        "types": [{
            "identity": FIXTURE_OBJECT, "displayName": "Account",
            "kind": {"module": FIXTURE_MODEL, "name": "entity"},
            "roles": [], "constraints": [], "extensions": [], "unknownPolicy": "reject",
            "supertypes": [], "fields": [], "operations": [],
            "relationships": [{
                "identity": FIXTURE_RELATIONSHIP,
                "category": "structural", "composite": false, "direction": "bidirectional",
                "sourceEnd": {"type": FIXTURE_OBJECT, "role": "link",
                    "multiplicity": {"lower": 1, "upper": 1, "ordered": false, "unique": true}},
                "targetEnd": {"type": FIXTURE_OBJECT,
                    "multiplicity": {"lower": 1, "upper": 1, "ordered": false, "unique": true}},
                "origin": {"source": {"sourceIdentity": FIXTURE_OBJECT,
                    "path": "models/Account.md", "startLine": 1, "startColumn": 1}},
            }],
        }],
    })
}
/// The base package's enum declaration node key.
pub fn enum_type() -> String {
    base_node_ids().enum_type
}

/// The base package's `OPEN` enum member node key.
pub fn enum_member() -> String {
    base_node_ids().enum_member
}

/// The base package's declared unit node key.
pub fn unit_type() -> String {
    base_node_ids().unit
}

/// A readable node key: the code, zero-padded to a 64-digit digest.
pub fn key(code: u32) -> String {
    format!("{code:0>64}")
}

pub fn id(digest: &str) -> CheckedNodeId {
    serde_json::from_value(node_ref(digest)).expect("node id")
}

/// Every code this module ever builds a node for, mapped to its builder-side
/// id: the provisional application digest for an application-bodied node (see
/// [`PackageBuilder::application_code`]/[`PackageBuilder::application_bounded`]),
/// or the readable placeholder [`key`] for a plain node built by
/// [`PackageBuilder::code`]/[`PackageBuilder::bounded`]. [`code_id`] reads it
/// so callers can build items without a `&mut PackageBuilder`; after admission
/// [`FixtureIds::resolve`] maps it to the key the checked reader required.
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

/// The builder-side node id for `code`, read from [`application_registry`].
/// Ensures the registry is populated by building the corpus once (discarding
/// the builder) if this is the first call in the process -- `corpus_package`
/// registers every code this module defines via `code`/`bounded`/
/// `application_code`/`application_bounded`, so one build suffices for the
/// whole test binary. [`MISSING`] is the one deliberately-unregistered
/// sentinel (used to request a node id guaranteed absent from the graph);
/// any other code nobody ever registered is a fixture defect, not a
/// legitimate non-application node, so this panics rather than silently
/// returning a placeholder no assertion can then tell apart from a real
/// digest -- exactly the gap a wrong code previously passed through
/// vacuously.
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
        None if code == MISSING => id(&key(code)),
        None => panic!(
            "code {code} is not registered by any PackageBuilder constructor and is not the \
             deliberately-absent MISSING ({MISSING}) sentinel -- if it names a real node, build \
             it via `code`/`bounded`/`application_code`/`application_bounded`; if it is \
             deliberately absent, name it next to the MISSING check in `code_id`"
        ),
    }
}

fn node_ref(digest: &str) -> Value {
    json!({"domain": NODE_DOMAIN, "digest": digest})
}

pub fn reference(digest: &str) -> Value {
    json!({"term": "reference", "target": node_ref(digest)})
}

/// Contract IR (FR-208 `DeclarationTagRules`/`DeclarationOccurrenceRule`)
/// forbids `declaration` on `expression`/`relation`/`state`/`temporal`/
/// `correspondence` nodes and on `value`/`enum_value` nodes (and, measured
/// against the reader, on `value`/`parameter` nodes, which QSL emits with no
/// declaration), and otherwise
/// requires it exactly when the node carries a `declaration`-role occurrence
/// — which every node built by this module does except `value`/`parameter`
/// (role `expression`) and `state`/`frame` (role `generated`), both of which
/// the rule above already leaves without a declaration. The qualified name is not
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

/// Contract IR (FR-038-AC-17) requires `application.operation`
/// and `application.result_type` as members; Contract IR's `validate_operations`
/// checks `operation` against the checked-operation catalog
/// `quire-verification-contracts` publishes, so `operation` must name a real catalogued identity with a conformant
/// `operator`/`laws`/`mode`/`member`, not an opaque placeholder. This crate's
/// own generators still classify a body by `term`/`operator`/`arguments`
/// (never by `operation`), but for an exact-scalar claim the *confirmation*
/// of the caller's own descriptor now does read `operation.identity` and
/// `operation.mode` (and, for `IntegerDivision`, `operation.laws`) and
/// refuses to mark a claim `IrConfirmed` unless they agree with the
/// descriptor -- see [`corpus_operation`] for how each corpus expression
/// picks a catalogued `operation` that matches its own descriptor. (A
/// mismatched descriptor against one of these same catalogued nodes --
/// e.g. an `IntegerOperator::Add` request item against node 1004's
/// catalogued `quire.op.integer.mul` -- is exercised directly by the
/// generation tests, not by a corpus node here.) `result_type` names the
/// caller's own declared node type for this expression, a node every
/// caller of this helper has already registered.
pub fn application(
    operator: &str,
    mut operation: Value,
    result_type: &str,
    arguments: Vec<Value>,
) -> Value {
    if operation["member"]["kind"] == "type_argument"
        && operation["member"].get("declaration").is_none()
    {
        operation["member"]["declaration"] = node_ref(result_type);
    }
    json!({
        "term": "application",
        "operator": operator,
        "operation": operation,
        "result_type": node_ref(result_type),
        "arguments": arguments,
    })
}

/// Reference targets used to anchor bounds on operand nodes. The application
/// dependency join also includes an operation member declaration, added after
/// choosing the anchor. `result_type` and a literal's `type` stay excluded.
fn reference_targets(body: &Value) -> Vec<String> {
    fn walk(value: &Value, targets: &mut BTreeSet<String>) {
        match value {
            Value::Object(map) => {
                if map.get("term").and_then(Value::as_str) == Some("reference") {
                    if let Some(digest) = map.get("target").and_then(|t| t.get("digest")) {
                        if let Some(digest) = digest.as_str() {
                            targets.insert(digest.to_owned());
                        }
                    }
                }
                for member in map.values() {
                    walk(member, targets);
                }
            }
            Value::Array(items) => {
                for item in items {
                    walk(item, targets);
                }
            }
            _ => {}
        }
    }
    let mut targets = BTreeSet::new();
    walk(body, &mut targets);
    targets.into_iter().collect()
}

/// The `value`/`literal` node this corpus pairs with a scalar `semantic_type`
/// digest, e.g. `T_TEXT` -> `V_TEXT`: a safe [`PackageBuilder::application_bounded`]
/// fallback anchor when a body has no `reference` argument to anchor a bound
/// on, since (unlike a nominal `scalar_type` node) a `value` node is already
/// established to tolerate an extra dependency (`V_QUANTITY`'s own
/// `rational_range` bound, wired at construction in `corpus_package`).
fn value_for_type(semantic_type: &str) -> Option<String> {
    let pairs = [
        (T_BOOLEAN, V_BOOLEAN),
        (T_INTEGER, V_INTEGER),
        (T_RATIONAL, V_RATIONAL),
        (T_DECIMAL, V_DECIMAL),
        (T_FLOAT32, V_FLOAT32),
        (T_FLOAT64, V_FLOAT64),
        (T_TEXT, V_TEXT),
    ];
    for (ty, value) in pairs {
        if semantic_type == key(ty) {
            return Some(key(value));
        }
    }
    if semantic_type == unit_type() {
        return Some(key(V_QUANTITY));
    }
    None
}

/// A catalogued `operation` member with no laws, mode or member.
pub fn op(identity: &str) -> Value {
    op_full(identity, Vec::new(), None, None)
}

/// A catalogued `operation` member with explicit laws/mode/member.
pub fn op_full(
    identity: &str,
    laws: Vec<Value>,
    mode: Option<Value>,
    member: Option<Value>,
) -> Value {
    json!({
        "identity": identity,
        "laws": laws,
        "mode": mode,
        "member": member,
        "leaves": [],
    })
}

/// One `operation.laws` entry: a law role paired with a catalogued
/// definition artifact ref.
pub fn law(role: &str, definition: Value) -> Value {
    json!({"role": role, "definition": definition})
}

/// An `operation.mode` member: `{"kind", "value"}`. The result type must pin
/// this value to pass Contract IR's `check_mode_type` validation.
pub fn mode_kv(kind: &str, value: &str) -> Value {
    json!({"kind": kind, "value": value})
}

/// An `operation.member` naming only a `kind`: sufficient for the member kind
/// this module uses without a declaration (`profile_operator`), which IR
/// checks for presence and kind only.
pub fn member_kind(kind: &str) -> Value {
    json!({"kind": kind})
}

/// An `operation.member` of kind `type_argument`. IR closes this shape to
/// `{kind, declaration}`, and a catalogued `member` result form requires the
/// application's `result_type` to equal the `declaration` node.
pub fn type_argument_member(declaration: &str) -> Value {
    json!({"kind": "type_argument", "declaration": node_ref(declaration)})
}

/// A well-formed definition ref `{authority, identity}` for a profile-role law, which has no closed
/// catalog to select from: any well-formed ref the lock selects under that role admits.
pub fn artifact_ref(identity: &str) -> Value {
    json!({"authority": "agent-ix", "identity": identity})
}

/// The catalogued `role` law definition named `identity`, read from the operation catalog's home
/// (`quire-verification-contracts`), which `validate_operations` requires
/// `operation.laws[].definition` to equal exactly.
pub fn catalog_definition(role: &str, identity: &str) -> Value {
    let catalog: Value = serde_json::from_str(
        quire_verification_contracts::operation_catalog::CHECKED_OPERATION_CATALOG_V1,
    )
    .expect("the operation catalog is JSON");
    catalog["law_roles"][role]
        .as_array()
        .into_iter()
        .flatten()
        .find(|definition| definition["identity"] == identity)
        .cloned()
        .unwrap_or_else(|| panic!("the catalog has no {role} definition {identity}"))
}

/// The catalogued `integer_division` law definition selecting `profile`.
pub fn integer_division_definition(profile: DivisionProfile) -> Value {
    catalog_definition("integer_division", profile.definition_identity())
}

/// The catalog's `ieee_profile` law definition.
pub fn ieee_profile_definition() -> Value {
    catalog_definition("ieee_profile", "quire.value.ieee754-2019-default/v1")
}

/// The catalog's `text_profile` law definition.
pub fn text_profile_definition() -> Value {
    catalog_definition("text_profile", "quire.value.text.unicode-17.0.0/v1")
}

fn aggregate() -> Value {
    json!({"term": "aggregate", "members": []})
}

/// The corpus's own scalar-type node for a literal `value_kind`. Contract IR
/// requires `literal.type` as a member and validates only that it
/// resolves to a real node (FR-038-AC-17); it does not require the target to
/// equal the containing node's own `semantic_type`, and most hand-written
/// corpus fixtures that type themselves by kind rather than by a bound do
/// follow that convention (`literal.type` == the node's own `semantic_type`),
/// matching the base package's pattern. `corpus_package`'s
/// own `V_QUANTITY` is the one exception: its `literal.type` is `rational`
/// (this value's own `value_kind`) even though the node's `semantic_type` is
/// `unit_type()`, because Contract IR's lowering reaches `literal.type`
/// regardless of the containing node's declared type (see `V_QUANTITY`'s own
/// comment in `corpus_package`).
fn literal_type(kind: &str) -> String {
    match kind {
        "boolean" => key(T_BOOLEAN),
        "integer" => key(T_INTEGER),
        "rational" => key(T_RATIONAL),
        "decimal" => key(T_DECIMAL),
        "float32_bits" => key(T_FLOAT32),
        "float64_bits" => key(T_FLOAT64),
        "text" => key(T_TEXT),
        "enum" => enum_type(),
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

/// A bound-body member in checked-package v2's shape (FR-322): a `binding` term naming the
/// member and carrying its literal, not the bare literal.
pub fn member(name: &str, value: Value) -> Value {
    json!({"term": "binding", "name": name, "value": value})
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// A V2 package under construction.
pub struct PackageBuilder {
    value: Value,
    bounds: BTreeSet<String>,
    dedicated_operands: BTreeSet<String>,
}

fn admitted_fixture_ids() -> &'static Mutex<BTreeMap<String, FixtureIds>> {
    static IDS: OnceLock<Mutex<BTreeMap<String, FixtureIds>>> = OnceLock::new();
    IDS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

/// The resolver captured when this exact fixture package was admitted.
pub fn fixture_ids_for(package: &CheckedPackageV2) -> FixtureIds {
    admitted_fixture_ids()
        .lock()
        .expect("fixture IDs lock")
        .get(package.package_id().digest.as_ref())
        .cloned()
        .expect("fixture package was admitted by PackageBuilder")
}

/// Resolve claim-map IDs against the fixture package named in that map.
pub fn fixture_ids_for_digest(digest: &str) -> FixtureIds {
    admitted_fixture_ids()
        .lock()
        .expect("fixture IDs lock")
        .get(digest)
        .cloned()
        .expect("fixture package ID is registered")
}

/// The scalar corpus request with every provisional node ID resolved for this admitted package.
pub fn golden_items_for(package: &CheckedPackageV2) -> Vec<ExactScalarItem> {
    let ids = fixture_ids_for(package);
    golden_items()
        .into_iter()
        .map(|mut item| {
            item.node_id = ids.resolve(&item.node_id);
            item
        })
        .collect()
}

include!("../checked_package_support/base.rs");

impl Default for PackageBuilder {
    fn default() -> Self {
        let mut value = base_package();
        let document = fixture_model_document();
        let selection = json!({
            "identity": FIXTURE_MODEL,
            "digest_domain": "sha256-jcs",
            "digest": quire_canonical::sha256(
                &document,
                quire_canonical::Limits::new(CheckedPackageReadLimits::bounded().bytes),
            )
            .expect("canonical model document")
            .to_string(),
        });
        value["lock"]["model_selections"] = json!([selection]);
        value["identity_preimage"]["model_selections"] = value["lock"]["model_selections"].clone();
        Self {
            value,
            bounds: BTreeSet::new(),
            dedicated_operands: BTreeSet::new(),
        }
    }
}

impl PackageBuilder {
    /// Give source-authored scalar expressions the QSL occurrence role used by
    /// FR-049 result selection. Both graph and source-map occurrences remain
    /// aligned before the public checked reader admits this fixture.
    pub fn expression_occurrences(&mut self) -> &mut Self {
        let expressions = self.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("fixture nodes")
            .iter_mut()
            .filter(|node| node["node_tag"] == "expression")
            .map(|node| {
                for occurrence in node["occurrences"]
                    .as_array_mut()
                    .expect("fixture occurrences")
                {
                    occurrence["role"] = json!("expression");
                }
                node["node_id"].clone()
            })
            .collect::<Vec<_>>();
        for source in self.value["source_map"]
            .as_array_mut()
            .expect("fixture source map")
        {
            if expressions.contains(&source["node_id"]) {
                source["role"] = json!("expression");
            }
        }
        self
    }

    pub fn node(
        &mut self,
        digest: &str,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
    ) -> &mut Self {
        self.node_with(digest, tag, form, semantic_type, body, &[])
    }

    pub fn node_with(
        &mut self,
        digest: &str,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
        dependencies: &[String],
    ) -> &mut Self {
        self.node_with_label(digest, digest, tag, form, semantic_type, body, dependencies)
    }

    /// As [`Self::node_with`], but the declaration's qualified name is
    /// derived from `label` rather than `digest`. Needed for an
    /// application-bodied node: IR-216's `validate_application_keys`
    /// re-derives `digest` from a preimage that itself embeds
    /// `declaration`, so `digest` cannot be known before `declaration` is
    /// built from something else -- [`Self::application_code_with`] uses
    /// the node's own `code` as that something else.
    fn node_with_label(
        &mut self,
        digest: &str,
        label: &str,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
        dependencies: &[String],
    ) -> &mut Self {
        let dependencies = dependencies
            .iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|digest| node_ref(digest))
            .collect::<Vec<_>>();
        // The schema's `BodyBindingRules` fix the occurrence role of these forms.
        let role = match (tag, form) {
            ("scalar_type", "integer") if digest == key(T_INTEGER) => "generated",
            ("value", "parameter") => "expression",
            ("state", "frame") => "generated",
            ("state", "operation_anchor") => "anchor",
            ("state", "state_clause") => "claim",
            _ => "declaration",
        };
        let source = self.value["lock"]["sources"][0].clone();
        let nodes = self.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("nodes");
        let mut node = json!({
            "node_id": node_ref(digest),
            "schema_version": "quire.checked-semantic-graph/v2",
            "node_tag": tag,
            "semantic_form": form,
            "semantic_type": node_ref(semantic_type),
            "dependencies": dependencies,
            "occurrences": [{"role": role, "ordinal": 0}],
            "body": body,
        });
        if let Some(declaration) = (role != "generated")
            .then(|| declaration_for(tag, form, label))
            .flatten()
        {
            node["declaration"] = declaration;
            // These hand-built declarations belong to the selected `example` source.
            // Application bodies derive their identity from their operation instead;
            // the checked reader therefore requires no owner on those nodes.
            if node["body"]["term"] != "application" {
                node["owner"] = json!({
                    "kind": "source",
                    "authority": source["authority"],
                    "identity": source["identity"],
                });
            }
        } else if (tag, form) == ("relation", "relationship") {
            node["owner"] = json!({
                "kind": "model", "identity": FIXTURE_MODEL, "node": FIXTURE_RELATIONSHIP,
            });
        }
        nodes.push(node);
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

    pub fn code(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
    ) -> &mut Self {
        register_code(code, key(code));
        self.node(&key(code), tag, form, semantic_type, body)
    }

    /// A node that depends on `bounds`, adding each bound node once.
    pub fn bounded(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
        bounds: &[Bound],
    ) -> &mut Self {
        register_code(code, key(code));
        let keys = bounds
            .iter()
            .map(|bound| self.bound(bound))
            .collect::<Vec<_>>();
        self.node_with(&key(code), tag, form, semantic_type, body, &keys)
    }

    /// Registers an application under Contract IR's candidate identity. Its
    /// declaration uses the fixture code because the final key is not known yet.
    pub fn application_code(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
    ) -> &mut Self {
        self.application_code_with(code, tag, form, semantic_type, body, &[])
    }

    /// As [`Self::application_code`], with extra node dependencies (e.g.
    /// bound keys already resolved by the caller).
    pub fn application_code_with(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
        dependencies: &[String],
    ) -> &mut Self {
        let label = code.to_string();
        self.node_with_label(
            &key(code),
            &label,
            tag,
            form,
            semantic_type,
            body,
            dependencies,
        );
        let nodes = self.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("fixture nodes");
        let node = nodes.last_mut().expect("application node just appended");
        let draft: CheckedSemanticNodeV2 =
            serde_json::from_value(node.clone()).expect("complete application draft");
        let candidate = derive_application_node_id(
            ApplicationNodeDraft::new(&draft),
            None,
            CheckedPackageReadLimits::bounded().bytes,
        )
        .expect("application candidate");
        let digest = candidate.candidate_node_id().digest.to_string();
        node["node_id"] = node_ref(&digest);
        self.value["source_map"]
            .as_array_mut()
            .expect("fixture source map")
            .last_mut()
            .expect("application source map just appended")["node_id"] = node_ref(&digest);
        register_code(code, digest);
        self
    }

    /// A `value`/`literal` node with `operand(form)`'s own content, at a
    /// digest dedicated to `bound_keys` rather than `operand(form)`'s own
    /// shared one.
    ///
    /// IR-280's own reachable-bound search (`CompleteLoweringRecordV2::
    /// RequiresBound`/`AmbiguousBound`) walks the *whole* transitive
    /// dependency closure a requested node reaches, not just its own direct
    /// dependencies. Before IR-280, every corpus expression needing a bound
    /// carried it as its own extra dependency, so sharing one operand node
    /// (`V_INTEGER` and friends) across many expressions never mixed their
    /// bounds up. Now that a bound can only reach a node through some other
    /// node's dependency edge (`application_bounded`'s doc), routing every
    /// bound through the one shared operand would union them all onto every
    /// expression that references it -- `AmbiguousBound` where only one
    /// expression (`AMBIGUOUS`, deliberately) should ever see more than one.
    /// A distinct corpus expression needing a distinct bound set (or set of
    /// bounds, for `AMBIGUOUS` itself) therefore gets its own operand node,
    /// keyed by `(form, bound_keys)` so expressions sharing the identical
    /// bound set still safely share one anchor, exactly as before.
    pub fn dedicated_operand(&mut self, form: &str, bound_keys: &[String]) -> String {
        self.dedicated_operand_impl(form, bound_keys, None, None)
    }

    fn dedicated_typed_operand(&mut self, form: &str, bound_keys: &[String], ty: &str) -> String {
        self.dedicated_operand_impl(form, bound_keys, None, Some(ty))
    }

    /// As [`Self::dedicated_operand`], but salted with `tag` so a caller that
    /// needs an anchor nothing else in the corpus can ever share -- not even
    /// another expression with the identical `(form, bound_keys)` pair --
    /// can force a distinct digest without inventing a new bound. Used by
    /// `LITERAL_OPERAND`'s own anchor (see its call site's doc): without a
    /// salt, `LITERAL_OPERAND`'s `dedicated_operand("integer", [INT])` call
    /// collides with every other corpus expression that shares the identical
    /// `INT` bound set, so `CLAIM`/`CLAIM_ALT` -- wired as dependencies of
    /// that one anchor -- became reachable from all of them too.
    pub fn dedicated_operand_tagged(
        &mut self,
        form: &str,
        bound_keys: &[String],
        tag: &str,
    ) -> String {
        self.dedicated_operand_impl(form, bound_keys, Some(tag), None)
    }

    /// Shared implementation of [`Self::dedicated_operand`] and
    /// [`Self::dedicated_operand_tagged`]. `tag: None` reproduces the
    /// original untagged digest preimage exactly (a 3-element array), so
    /// every pre-existing call site keeps its original node id; only a
    /// caller that opts into `dedicated_operand_tagged` gets the salted,
    /// 4-element preimage and thus a new, distinct digest.
    fn dedicated_operand_impl(
        &mut self,
        form: &str,
        bound_keys: &[String],
        tag: Option<&str>,
        ty_override: Option<&str>,
    ) -> String {
        let mut sorted_keys = bound_keys.to_vec();
        sorted_keys.sort();
        let preimage = match (tag, ty_override) {
            (Some(tag), None) => json!(["dedicated-operand", form, sorted_keys, tag]),
            (None, Some(ty)) => json!(["dedicated-operand", form, sorted_keys, ty]),
            (None, None) => json!(["dedicated-operand", form, sorted_keys]),
            (Some(_), Some(_)) => panic!("a dedicated operand has one selector"),
        };
        let digest = sha256_hex(&serde_json::to_vec(&preimage).expect("dedicated operand key"));
        if self.dedicated_operands.insert(digest.clone()) {
            let (kind, value, ty) = match form {
                "integer" => ("integer", "3", key(T_INTEGER)),
                "rational" => ("rational", "1/2", key(T_RATIONAL)),
                "decimal" => ("decimal", "1.5", key(T_DECIMAL)),
                "float32" => ("float32_bits", "0", key(T_FLOAT32)),
                "float64" => ("float64_bits", "0", key(T_FLOAT64)),
                "text" => ("text", "a", key(T_TEXT)),
                other => panic!("dedicated_operand: no literal form known for {other}"),
            };
            let ty = ty_override.unwrap_or(&ty);
            let mut body = literal(kind, value);
            body["type"] = node_ref(ty);
            self.node(&digest, "value", "literal", ty, body);
        }
        digest
    }

    /// As [`Self::dedicated_operand`], for the "unit" form it does not
    /// cover: "unit" has no fixed literal kind that method's `form` match
    /// can name, because a unit operand is `V_QUANTITY`'s own shape (a
    /// `rational`-typed magnitude under [`unit_type`], not a scalar literal
    /// keyed by one of `T_INTEGER`/`T_RATIONAL`/etc.). This builds a node
    /// dedicated to `bound_keys` with that same shape, including
    /// `V_QUANTITY`'s own inherent `rational_range` bound (Contract IR's
    /// lowering reaches `literal.type` regardless of the containing node's
    /// declared type, so this needs one reachable the same way `V_QUANTITY`
    /// itself does, in addition to `bound_keys`, which the caller wires on
    /// separately).
    pub fn dedicated_unit_operand(&mut self, bound_keys: &[String]) -> String {
        let mut sorted_keys = bound_keys.to_vec();
        sorted_keys.sort();
        let digest = sha256_hex(
            &serde_json::to_vec(&json!(["dedicated-unit-operand", sorted_keys]))
                .expect("dedicated unit operand key"),
        );
        if self.dedicated_operands.insert(digest.clone()) {
            let rational_bound = self.bound(&RAT);
            self.node_with(
                &digest,
                "value",
                "literal",
                &unit_type(),
                literal("rational", "1"),
                &[rational_bound],
            );
        }
        digest
    }

    /// An application-bodied node that needs `bounds` reachable.
    ///
    /// IR-280's FR-322 application-node dependency join requires this node's
    /// own `dependencies` to be *exactly* the unique, digest-ascending
    /// `reference` targets of its body -- `bounds` cannot be listed there
    /// directly. Each bound is still registered
    /// once (as [`Self::bounded`] does) and wired reachable, but as an extra
    /// dependency of a node the join constraint does not cover: the body's
    /// own first `reference` target, or, when every argument is a literal
    /// (no `reference` term at all -- e.g. `TextAdmission`'s fixed literal
    /// argument), the `value`/`literal` node of `semantic_type`'s own family
    /// (e.g. `T_TEXT` -> `V_TEXT`, via [`value_for_type`]) -- never
    /// `semantic_type` itself, a nominal `scalar_type` node IR-280 also
    /// refuses an unrelated extra dependency on. Either way the anchor is a
    /// non-application node, so it can carry an extra dependency IR-280
    /// would refuse on this node itself.
    ///
    /// The reference argument this anchors on is expected to already be
    /// [`Self::dedicated_operand`]'s (or, when no bound can collide, the
    /// plain shared `operand(form)`) -- this method only anchors on
    /// whatever `body` already names; it does not choose between them. See
    /// [`Self::dedicated_operand`]'s own doc for why that choice matters.
    pub fn application_bounded(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
        bounds: &[Bound],
    ) -> &mut Self {
        self.application_bounded_anchored(code, tag, form, semantic_type, body, bounds, None)
    }

    /// As [`Self::application_bounded`], but the caller may pin the anchor
    /// explicitly instead of leaving it to this method's own inference.
    /// Needed whenever a body's `reference_targets` contains more than one
    /// digest and the caller already knows which one is the dedicated
    /// argument the bound belongs to: [`reference_targets`] returns them in
    /// ascending digest order (a `BTreeSet`, for IR-280's own join
    /// requirement), so falling back to `.first()` there picks whichever
    /// reference happens to sort first, not "the first argument" in any
    /// positional sense -- correct only when a body has at most one distinct
    /// reference target. `quantity.pow`'s body has two (a dedicated "unit"
    /// node and a dedicated "integer" node) and needs the "unit" one
    /// specifically, so its caller passes `anchor` rather than relying on
    /// digest order.
    pub fn application_bounded_anchored(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
        bounds: &[Bound],
        anchor: Option<&str>,
    ) -> &mut Self {
        let keys = bounds
            .iter()
            .map(|bound| self.bound(bound))
            .collect::<Vec<_>>();
        let references = reference_targets(&body);
        // A `type_argument` member's declaration is a dependency of the
        // application alongside its `reference` targets; it is never the
        // bound anchor.
        let mut dependencies = references.clone();
        if let Some(declaration) = body["operation"]["member"]["declaration"]["digest"].as_str() {
            dependencies.push(declaration.to_owned());
        }
        // Only computed when there is a bound to anchor: a caller passing no
        // `bounds` (e.g. the isolated TextAdmission reader negative) may
        // legitimately have neither a `reference` target nor a `semantic_type`
        // `value_for_type` recognizes, and must not panic over an anchor
        // nothing here will use.
        if !keys.is_empty() {
            let anchor = anchor.map(str::to_owned).unwrap_or_else(|| {
                references.first().cloned().unwrap_or_else(|| {
                    value_for_type(semantic_type).unwrap_or_else(|| {
                        panic!(
                            "application_bounded({code}): {} bound(s) requested, the body has no \
                             `reference` target to anchor them on, and semantic_type \
                             {semantic_type} names no known value node",
                            bounds.len()
                        )
                    })
                })
            });
            for key in &keys {
                self.add_dependency(&anchor, key);
            }
        }
        dependencies.sort();
        dependencies.dedup();
        self.application_code_with(code, tag, form, semantic_type, body, &dependencies);
        self
    }

    /// Registers `definition` in `lock.definition_selections` (deduplicated),
    /// so `validate_operations`'s law-selection check (Contract IR
    /// `checked_package/v2/operations.rs`) finds it selected for
    /// any node whose `operation.laws` names it. Mirrors the same push into
    /// `identity_preimage.definition_selections`: `validate_lock`'s
    /// `same_non_graph_lock` (Contract IR
    /// `checked_package/v2/mod.rs`) refuses `StaleDependency` at `"lock"`
    /// unless the preimage and the lock agree field-for-field, and this is
    /// the one field this fixture builder mutates after construction.
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

    /// Registers `definition` under `role` in `lock.profile_selections`
    /// (deduplicated), the clause/profile-role analogue of
    /// [`Self::select_definition`]: `validate_operations`'s check for a
    /// `temporal_profile`/`protocol_profile` law (Contract IR
    /// `checked_package/v2/operations.rs`) accepts any published definition
    /// of that role exactly when the lock selected it under this role --
    /// there is no fixed catalogued list for a profile role the way there is
    /// for `integer_division`/`ieee_profile`/`text_profile`.
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

    /// Adds `dependency` to the already-registered node `target`'s own
    /// `dependencies` edge list without changing its builder-side id. The
    /// reader may require a different final id after this change; `wire()`
    /// resolves it through the reader's typed stale-key refusal and rebuilds
    /// package identity from the current node objects on every call, so the appended edge is
    /// reflected consistently by the next `wire()`/`admit()`.
    pub fn add_dependency(&mut self, target: &str, dependency: &str) -> &mut Self {
        let nodes = self.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("nodes");
        let node = nodes
            .iter_mut()
            .find(|node| node["node_id"]["digest"].as_str() == Some(target))
            .unwrap_or_else(|| panic!("no node registered with digest {target}"));
        let dependencies = node["dependencies"].as_array_mut().expect("dependencies");
        let reference = node_ref(dependency);
        if !dependencies.contains(&reference) {
            dependencies.push(reference);
        }
        self
    }

    /// Remove one deliberately invalid node while retaining its isolated inputs.
    fn omit_code(&mut self, code: u32) {
        let digest = code_id(code).digest;
        let nodes = self.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("nodes");
        let before = nodes.len();
        nodes.retain(|node| node["node_id"]["digest"].as_str() != Some(digest.as_ref()));
        assert_eq!(nodes.len() + 1, before, "one fixture node for code {code}");
        let map = self.value["source_map"].as_array_mut().expect("source map");
        let before = map.len();
        map.retain(|entry| entry["node_id"]["digest"].as_str() != Some(digest.as_ref()));
        assert_eq!(map.len() + 1, before, "one source entry for code {code}");
    }

    /// Add `bound` once, returning its key. Any requires-bound kind foreign
    /// to `bound`'s own body (see `Bound::foreign`) is added first and wired
    /// as this node's dependency, so it stays reachable wherever `bound` is.
    pub fn bound(&mut self, bound: &Bound) -> String {
        let digest = bound.key();
        if self.bounds.insert(digest.clone()) {
            let foreign = bound
                .foreign()
                .iter()
                .map(|foreign| self.bound(foreign))
                .collect::<Vec<_>>();
            let body = bound.body();
            let bounded_type = bound.bounded_type();
            self.node_with(
                &digest,
                "bounded_domain",
                bound.form(),
                &bounded_type,
                body,
                &foreign,
            );
        }
        digest
    }

    /// The fixture's authored wire before ID and package-identity resolution.
    pub fn raw_wire(&self) -> &Value {
        &self.value
    }

    /// The wire document with its identity projection and package id refreshed.
    pub fn wire(&self) -> Value {
        self.resolved_wire(CheckedPackageReadLimits::bounded()).wire
    }

    /// This fixture's wire and the map from its readable IDs to admitted IDs.
    fn resolved_wire(&self, limits: CheckedPackageReadLimits) -> rekey::ResolvedWire {
        rekey::resolve_wire(&self.value, &evidence(), limits)
    }

    /// Admits the package and returns its per-artifact ID resolver.
    pub fn admit_resolved(&self) -> (CheckedPackageV2, FixtureIds) {
        self.admit_with_resolved(CheckedPackageReadLimits::bounded())
    }

    /// Admits under explicit reader limits and returns this artifact's ID resolver.
    pub fn admit_with_resolved(
        &self,
        limits: CheckedPackageReadLimits,
    ) -> (CheckedPackageV2, FixtureIds) {
        let rekey::ResolvedWire { outcome, ids, .. } = self.resolved_wire(limits);
        match outcome {
            CheckedPackageV2ReadResult::Admitted(package) => {
                admitted_fixture_ids()
                    .lock()
                    .expect("fixture IDs lock")
                    .insert(package.package_id().digest.to_string(), ids.clone());
                (*package, ids)
            }
            other => panic!("expected V2 admission, got {other:?}"),
        }
    }

    /// Returns the reader result and wire with the IDs used by that wire.
    pub fn read_resolved(&self) -> (CheckedPackageV2ReadResult, Value, FixtureIds) {
        let rekey::ResolvedWire { wire, outcome, ids } =
            self.resolved_wire(CheckedPackageReadLimits::bounded());
        (outcome, wire, ids)
    }

    pub fn admit(&self) -> CheckedPackageV2 {
        self.admit_with(CheckedPackageReadLimits::bounded())
    }

    /// The reader's verdict on this package, admitted or refused, with the wire it read.
    pub fn read(&self) -> (CheckedPackageV2ReadResult, Value) {
        let (result, wire, _) = self.read_resolved();
        (result, wire)
    }

    pub fn admit_with(&self, limits: CheckedPackageReadLimits) -> CheckedPackageV2 {
        self.admit_with_resolved(limits).0
    }
}

fn evidence() -> CheckedPackageEvidence {
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let document = fixture_model_document();
    let bytes = serde_json::to_vec(&document).expect("model document");
    let digest = quire_canonical::sha256(
        &document,
        quire_canonical::Limits::new(CheckedPackageReadLimits::bounded().bytes),
    )
    .expect("canonical model document")
    .to_string();
    evidence
        .insert_domain_package_document(digest, bytes)
        .expect("retain fixture model document");
    evidence
}

/// Read a deliberate mutation of an already admitted wire under its refreshed package identity.
pub fn read_mutated_wire(mut wire: Value) -> CheckedPackageV2ReadResult {
    rekey::refresh_identity(&mut wire);
    let bytes = serde_json::to_vec(&wire).expect("mutated fixture wire");
    CheckedPackageV2::read(&bytes, CheckedPackageReadLimits::bounded(), &evidence())
}

/// Re-key a mutated valid-shape wire before asserting its semantic reader refusal.
pub fn read_application_mutated_wire(
    mut wire: Value,
    provisional: &CheckedNodeId,
) -> (CheckedPackageV2ReadResult, CheckedNodeId) {
    let nodes: Vec<CheckedSemanticNodeV2> =
        serde_json::from_value(wire["semantic_graph"]["nodes"].clone()).expect("draft nodes");
    let draft = nodes
        .iter()
        .find(|node| &node.node_id == provisional)
        .expect("draft application");
    let candidate = derive_application_node_id(
        ApplicationNodeDraft::new(draft),
        None,
        CheckedPackageReadLimits::bounded().bytes,
    )
    .expect("application candidate")
    .candidate_node_id()
    .clone();
    rekey::install_candidate(&mut wire, provisional, &candidate);
    let bytes = serde_json::to_vec(&wire).expect("mutated fixture wire");
    (
        CheckedPackageV2::read(&bytes, CheckedPackageReadLimits::bounded(), &evidence()),
        candidate,
    )
}

// ---- bounds ------------------------------------------------------------------

/// One `bounded_domain` node on a corpus scalar type.
#[derive(Clone, Debug)]
pub enum Bound {
    /// `integer_range` on `T_INTEGER`.
    Integer(i64, i64),
    /// `rational_range` on `T_RATIONAL`: numerator and denominator intervals.
    Rational(i64, i64, i64, i64),
    /// `decimal_range` on `T_DECIMAL`.
    Decimal(i64, i64, u64, u64, &'static str),
    /// `float_rounding` on the named float type.
    Rounding(&'static str, &'static str),
    /// `text_bounds` on `T_TEXT`.
    Text(u64, u64, &'static str),
    /// Any form on any corpus type with any body.
    Raw {
        form: &'static str,
        bounded: &'static str,
        body: Value,
        /// Bounds this one's body must reach (see `Bound::foreign`).
        foreign: Vec<Bound>,
    },
}

fn integer_literal(value: impl ToString) -> Value {
    literal("integer", &value.to_string())
}

/// The `value_kind` of every `literal` term reachable within `value`, in first-encounter
/// order, deduplicated.
fn literal_kinds(value: &Value) -> Vec<String> {
    fn walk(value: &Value, kinds: &mut Vec<String>) {
        match value {
            Value::Object(members) => {
                if members.get("term").and_then(Value::as_str) == Some("literal") {
                    if let Some(kind) = members.get("value_kind").and_then(Value::as_str) {
                        if !kinds.iter().any(|found| found == kind) {
                            kinds.push(kind.to_owned());
                        }
                    }
                }
                for member in members.values() {
                    walk(member, kinds);
                }
            }
            Value::Array(items) => {
                for item in items {
                    walk(item, kinds);
                }
            }
            _ => {}
        }
    }
    let mut kinds = Vec::new();
    walk(value, &mut kinds);
    kinds
}

impl Bound {
    pub fn form(&self) -> &'static str {
        match self {
            Self::Integer(..) => "integer_range",
            Self::Rational(..) => "rational_range",
            Self::Decimal(..) => "decimal_range",
            Self::Rounding(..) => "float_rounding",
            Self::Text(..) => "text_bounds",
            Self::Raw { form, .. } => form,
        }
    }

    fn bounded_form(&self) -> &'static str {
        match self {
            Self::Integer(..) => "integer",
            Self::Rational(..) => "rational",
            Self::Decimal(..) => "decimal",
            Self::Rounding(float, _) => float,
            Self::Text(..) => "text",
            Self::Raw { bounded, .. } => bounded,
        }
    }

    pub fn bounded_type(&self) -> String {
        match self {
            Self::Decimal(_, _, _, _, rounding) => decimal_type_for_rounding(rounding),
            Self::Rounding(form, rounding) => float_type_for_rounding(form, rounding),
            _ => result_type(self.bounded_form()),
        }
    }

    /// Contract IR (FR-038-AC-17) requires `literal.type`, so every
    /// numeric or text literal embedded in a bound's own body — not just the
    /// value it bounds — is a real graph edge: `CheckedPackageV2::lower`
    /// walks it and, with `require_bounds`, refuses any reachable
    /// `integer`/`rational`/`decimal`/`text` scalar type that has no
    /// `bounded_domain` of its own in the same closure. A bound whose body
    /// carries interval or rounding parameters of a *different* requires-
    /// bound kind than the one it bounds must therefore also reach that
    /// kind's own bound, or the corpus expression that depends on it is
    /// refused before this crate's own bound-shape checks ever run.
    ///
    /// Derived from `body()`'s own literal kinds rather than hand-declared
    /// per variant, so the two can never drift apart: every literal kind
    /// `body()` embeds other than this bound's own `bounded_form()` names a
    /// shared canonical bound that must be foreign here. `Raw` is the one
    /// exception -- its body is caller-supplied and arbitrary, and its own
    /// `foreign` field may deliberately name something other than the shared
    /// canonical bound for its kind (see `WRONG_BOUND_FORM`'s bare,
    /// integer-free text satisfier, which exists specifically to avoid the
    /// shared `TEXT` constant's own embedded integer endpoints), so it is
    /// still caller-declared rather than derived.
    fn foreign(&self) -> Vec<Bound> {
        if let Self::Raw { foreign, .. } = self {
            return foreign.clone();
        }
        literal_kinds(&self.body())
            .into_iter()
            .filter(|kind| kind != self.bounded_form())
            .map(|kind| match kind.as_str() {
                "integer" => INT,
                "rational" => RAT,
                "decimal" => DEC,
                "text" => TEXT,
                other => panic!("no shared foreign bound registered for literal kind {other}"),
            })
            .collect()
    }

    pub fn body(&self) -> Value {
        let members = match self {
            Self::Integer(lower, upper) => vec![
                member("min", integer_literal(lower)),
                member("max", integer_literal(upper)),
            ],
            Self::Rational(nl, nu, dl, du) => vec![
                member("numerator_min", integer_literal(nl)),
                member("numerator_max", integer_literal(nu)),
                member("denominator_min", integer_literal(dl)),
                member("denominator_max", integer_literal(du)),
            ],
            Self::Decimal(lower, upper, min, max, rounding) => vec![
                member("coefficient_min", integer_literal(lower)),
                member("coefficient_max", integer_literal(upper)),
                member("scale_min", integer_literal(min)),
                member("scale_max", integer_literal(max)),
                member("rounding", literal("text", rounding)),
            ],
            Self::Rounding(_, rounding) => vec![member("rounding", literal("text", rounding))],
            Self::Text(min, max, profile) => vec![
                member("min", integer_literal(min)),
                member("max", integer_literal(max)),
                member("text_profile", literal("text", profile)),
            ],
            Self::Raw { body, .. } => return body.clone(),
        };
        json!({"term": "aggregate", "members": members})
    }

    /// The node key: a digest of the form, the bounded type, the body, and the keys of every
    /// foreign bound this one reaches. `foreign` is part of the key -- not just `body` -- so
    /// two `Raw` bounds with identical form/bounded/body but different declared `foreign`
    /// dependencies never collide in `PackageBuilder::bound`'s dedupe and silently keep only
    /// one's foreign dependency.
    pub fn key(&self) -> String {
        let foreign_keys = self.foreign().iter().map(Bound::key).collect::<Vec<_>>();
        let content = json!([self.form(), self.bounded_type(), self.body(), foreign_keys]);
        sha256_hex(&serde_json::to_vec(&content).expect("bound content"))
    }
}

/// An `text_bounds` bound over Integer: a bound of the wrong form for an integer operation.
pub fn wrong_form_bound() -> Bound {
    Bound::Raw {
        form: "text_bounds",
        bounded: "integer",
        body: Bound::Text(0, 4, "nfc").body(),
        // The embedded profile literal is `text`, foreign to this
        // bound's own `integer` type; without a reachable text bound
        // the corpus expression is refused at IR's lowering stage
        // before CG's own wrong-form check ever runs. A bare,
        // integer-free text satisfier is used rather than the
        // shared `TEXT` constant: `TEXT`'s own body embeds integer
        // endpoints and would supply a genuine, correctly-formed
        // `integer_range` bound, defeating this fixture's point.
        foreign: vec![Bound::Raw {
            form: "text_bounds",
            bounded: "text",
            body: json!({"term": "aggregate", "members": [member("text_profile", literal("text", "nfc"))]}),
            foreign: vec![],
        }],
    }
}

/// An integer range whose upper bound is spelled non-canonically.
pub fn unreadable_bound() -> Bound {
    Bound::Raw {
        form: "integer_range",
        bounded: "integer",
        body: json!({
            "term": "aggregate",
            "members": [
                member("min", literal("integer", "-5")),
                member("max", literal("integer", "05")),
            ],
        }),
        foreign: vec![],
    }
}

pub const INT: Bound = Bound::Integer(-1000, 1000);
pub const INT5: Bound = Bound::Integer(-5, 5);
pub const RAT: Bound = Bound::Rational(-1000, 1000, 1, 1000);
pub const DEC: Bound = Bound::Decimal(-1000, 1000, 0, 2, "nearest-even");
pub const TEXT: Bound = Bound::Text(0, 64, "nfc");

// ---- the scalar corpus -------------------------------------------------------

pub const T_BOOLEAN: u32 = 1;
pub const T_INTEGER: u32 = 2;
pub const T_RATIONAL: u32 = 3;
pub const T_DECIMAL: u32 = 4;
pub const T_DECIMAL_ZERO: u32 = 8;
pub const T_DECIMAL_AWAY: u32 = 9;
pub const T_FLOAT64_ZERO: u32 = 10;
pub const T_FLOAT64_POSITIVE: u32 = 11;
pub const T_FLOAT32: u32 = 5;
pub const T_FLOAT64: u32 = 6;
pub const T_TEXT: u32 = 7;
pub const V_BOOLEAN: u32 = 101;
pub const V_INTEGER: u32 = 102;
pub const V_RATIONAL: u32 = 103;
pub const V_DECIMAL: u32 = 104;
pub const V_FLOAT32: u32 = 105;
pub const V_FLOAT64: u32 = 106;
pub const V_TEXT: u32 = 107;
pub const V_QUANTITY: u32 = 108;
/// A value/literal node whose semantic type is an `integer_range` `bounded_domain` over the
/// integer scalar type: what a QSL parameter typed `Int[0, 9]` references. It is an Integer
/// operand (IR-297).
pub const V_BOUNDED_PARAM: u32 = 109;
/// A value whose semantic type is another value node, so neither a scalar type nor a
/// `bounded_domain` over one.
pub const V_UNTYPED: u32 = 110;

/// The corpus integer subtraction, whose right operand is an inline literal.
pub const LITERAL_OPERAND: u32 = 1003;

/// Non-generated nodes, each with the reason it is refused.
pub const COMPOSITE: u32 = 2001;
pub const FUNCTION: u32 = 2002;
pub const MODEL: u32 = 2003;
pub const RELATION: u32 = 2004;
pub const STATE: u32 = 2005;
pub const TEMPORAL: u32 = 2006;
pub const PROTOCOL: u32 = 2007;
/// A `claim`-tagged node wired only as an extra dependency of
/// `LITERAL_OPERAND` (issue #100 / FR-014-AC-5): it is never itself
/// requested, so it never gets its own oracle -- its sole purpose is to
/// sit in one requested item's reachable set so the generator's `claims`
/// field, structurally empty in every fixture node before this one, is
/// actually exercised.
pub const CLAIM: u32 = 2008;
/// A second, distinct `claim`-tagged node wired to the same item as
/// `CLAIM` above (issue #100 PR review): a single populated `claims` entry
/// left the field's ordering and truncation unfalsifiable -- a `.take(1)`
/// mutation and a `.reverse()` mutation on the generated field both stayed
/// green against a one-element array. Two entries in a determinate order
/// close that gap.
pub const CLAIM_ALT: u32 = 2009;
/// The `temporal` formula the `TEMPORAL` clause names as its formula argument.
pub const TEMPORAL_FORMULA: u32 = 2010;
/// The `parameter` the `TEMPORAL` clause ranges over.
pub const TEMPORAL_PARAMETER: u32 = 2012;
pub const CALLS_FUNCTION: u32 = 2011;
pub const WRONG_BODY: u32 = 2013;
pub const WRONG_OPERAND: u32 = 2014;
/// Well-formed integer additions requested with a refused request shape.
pub const DUPLICATED: u32 = 2015;
pub const WRONG_RESULT: u32 = 2016;
pub const WRONG_ARITY: u32 = 2017;
/// An integer addition reaching an unbounded integer type.
pub const UNBOUNDED: u32 = 2021;
/// An integer addition bounded in the IR, requested as mathematical.
pub const MATHEMATICAL: u32 = 2022;
/// A binary32 addition whose non-exact mode has no pinned result or operand type.
pub const MISSING_ROUNDING: u32 = 2023;
/// An integer addition reaching two integer ranges.
pub const AMBIGUOUS: u32 = 2024;
/// An integer addition whose range spells a bound non-canonically.
/// An integer addition whose only integer bound has the text form.
pub const WRONG_BOUND_FORM: u32 = 2026;
/// A floor division bounded `[-5, 5]`, requested over `[-1000, 1000]`.
pub const DOMAIN_MISMATCH: u32 = 2027;
/// The third quantity conversion node.
pub const QUANTITY_CONVERT_AWAY: u32 = 2028;
/// The quantity conversion nodes: each converts a quantity to its own unit type under the
/// `exact` mode that unit type pins.
pub const QUANTITY_CONVERSIONS: [u32; 3] = [1086, 1087, QUANTITY_CONVERT_AWAY];
/// An integer addition whose left operand is itself an application.
pub const EXPRESSION_OPERAND: u32 = 2029;
/// A quantity addition whose right operand is a literal.
pub const LITERAL_QUANTITY: u32 = 2030;
/// An integer addition whose right operand has no scalar type.
pub const UNTYPED_OPERAND: u32 = 2031;
/// An integer addition whose right operand is typed by an `integer_range` `bounded_domain`. It is
/// generated (IR-297), so it is requested by its own test and is in no golden item list.
pub const BOUNDED_OPERAND: u32 = 2034;
/// A rational division over two integer-typed *reference* operands: IR
/// admits it (`quire.op.rational.div`'s catalogued operand family,
/// `rational_promotable`, is `{integer, rational}`), but CG's own
/// `Shape::of` requires `[Rational, Rational]`, so it is refused through
/// the `reference` arm of `check_operand`, not the `literal` arm
/// `WRONG_OPERAND` exercises.
pub const WRONG_OPERAND_REFERENCE: u32 = 2032;
/// A deliberate mode/type mismatch, retained for the direct reader-negative
/// test. Contract IR refuses it before codegen can inspect a descriptor.
pub const MODE_MISMATCH: u32 = 2033;
pub const MISSING: u32 = 9999;

fn integer(value: i64) -> Integer {
    Integer::from(value)
}

pub fn interval(lower: i64, upper: i64) -> IntegerInterval {
    IntegerInterval::new(integer(lower), integer(upper)).expect("interval")
}

pub fn bounded(lower: i64, upper: i64) -> IntegerDomain {
    IntegerDomain::Bounded(interval(lower, upper))
}

pub fn rational_domain(nl: i64, nu: i64, dl: i64, du: i64) -> RationalDomain {
    RationalDomain::new(interval(nl, nu), interval(dl, du)).expect("domain")
}

pub fn decimal_type(
    lower: i64,
    upper: i64,
    min: u64,
    max: u64,
    rounding: RoundingMode,
) -> DecimalType {
    DecimalType::new(integer(lower), integer(upper), min, max, rounding).expect("decimal type")
}

/// One generated expression of the corpus.
pub struct Expression {
    pub code: u32,
    pub form: &'static str,
    pub operator: &'static str,
    pub operands: &'static [&'static str],
    pub result: &'static str,
    pub operation: ExactScalarOperation,
    pub bounds: Vec<Bound>,
}

/// The six `TextAdmission` codes ([`TEXT_ADMISSIONS`]): Contract IR refuses each
/// `IllTyped`/`OperatorIneligible` at admission today, so they are outside [`corpus`] (one such
/// node refuses the whole package). `quire.op.numeric.convert` over a text operand: no
/// catalogued `convert` identity takes `text`, and QSL lowers no text admission. Unblocked by a
/// catalogued text-admission operation (a spec change) or by dropping the operation.
/// `tc_024_text_admission_corpus_is_refused_by_ir_today` pins the exact refusal of each;
/// FR-014-AC-2's text-admission case has no generated-crate evidence until then.
pub fn refused_corpus() -> Vec<Expression> {
    use ExactScalarOperation as Op;
    let mut refused = Vec::new();
    for (code, profile) in TEXT_ADMISSIONS.into_iter().zip(TextProfile::ALL) {
        refused.push(expression(
            code,
            CONVERSION,
            TEXT1,
            "text",
            Op::TextAdmission {
                text_type: TextType::new(0, 4, profile).expect("text type"),
            },
            vec![Bound::Text(0, 4, profile.as_str())],
        ));
    }
    refused
}

/// [`corpus_package`] plus the one [`refused_corpus`] expression `code`.
pub fn refused_corpus_package(code: u32) -> PackageBuilder {
    let mut builder = corpus_package();
    let expression = refused_corpus()
        .into_iter()
        .find(|expression| expression.code == code)
        .unwrap_or_else(|| panic!("{code} is not a refused corpus expression"));
    add_corpus_expression(&mut builder, expression);
    builder
}

fn expression(
    code: u32,
    shape: (&'static str, &'static str),
    operands: &'static [&'static str],
    result: &'static str,
    operation: ExactScalarOperation,
    bounds: Vec<Bound>,
) -> Expression {
    Expression {
        code,
        form: shape.0,
        operator: shape.1,
        operands,
        result,
        operation,
        bounds,
    }
}

const BINARY: (&str, &str) = ("binary", "binary");
const UNARY: (&str, &str) = ("unary", "unary");
const CONVERSION: (&str, &str) = ("conversion", "convert");

const INT2: &[&str] = &["integer", "integer"];
const INT1: &[&str] = &["integer"];
const RAT2: &[&str] = &["rational", "rational"];
const RAT1: &[&str] = &["rational"];
const DEC2: &[&str] = &["decimal", "decimal"];
const DEC1: &[&str] = &["decimal"];
const F32_2: &[&str] = &["float32", "float32"];
const F64_2: &[&str] = &["float64", "float64"];
const F64_1: &[&str] = &["float64"];
const TEXT1: &[&str] = &["text"];
const TEXT2: &[&str] = &["text", "text"];
const ENUM2: &[&str] = &["enum", "enum"];
const QTY2: &[&str] = &["unit", "unit"];
const QTY_INT: &[&str] = &["unit", "integer"];
const QTY1: &[&str] = &["unit"];

/// Codes of the text, enum and quantity comparisons, in `ComparisonOperator::ALL` order.
pub const TEXT_COMPARISONS: [u32; 6] = [1111, 1112, 1072, 1113, 1114, 1115];
pub const ENUM_COMPARISONS: [u32; 6] = [1121, 1122, 1073, 1123, 1124, 1125];
pub const QUANTITY_COMPARISONS: [u32; 6] = [1131, 1132, 1084, 1133, 1134, 1135];
/// Codes of the text admissions, in `TextProfile::ALL` order, all `[0, 4]`.
pub const TEXT_ADMISSIONS: [u32; 6] = [1074, 1071, 1075, 1076, 1077, 1078];

/// Every generated family: each integer, rational and decimal operator, each
/// division law, each IEEE operator and comparison exercised, each text
/// profile, each comparison operator, and each bounded conversion target.
pub fn corpus() -> Vec<Expression> {
    use ExactScalarOperation as Op;
    let integer = |code, shape, operands, operator, domain: IntegerDomain, bound: Bound| {
        expression(
            code,
            shape,
            operands,
            "integer",
            Op::IntegerArithmetic { operator, domain },
            vec![bound],
        )
    };
    let division = |code, profile, lower, upper, bound: Bound| {
        expression(
            code,
            BINARY,
            INT2,
            "integer",
            Op::IntegerDivision {
                profile,
                domain: bounded(lower, upper),
            },
            vec![bound],
        )
    };
    let rational = |code, shape, operands, operator, domain, bounds| {
        expression(
            code,
            shape,
            operands,
            "rational",
            Op::RationalArithmetic {
                operator,
                domain: Some(domain),
            },
            bounds,
        )
    };
    let ordering = |code, operands, operator, kind, bound: Bound| {
        expression(
            code,
            BINARY,
            operands,
            "boolean",
            Op::Ordering {
                operator,
                operands: kind,
            },
            vec![bound],
        )
    };
    let decimal = |code, shape, operands, operator, target, bound: Bound| {
        expression(
            code,
            shape,
            operands,
            "decimal",
            Op::DecimalArithmetic { operator, target },
            vec![bound],
        )
    };
    let ieee = |code, operands, result, operator, width, rounding: RoundingMode| {
        expression(
            code,
            BINARY,
            operands,
            result,
            Op::IeeeArithmetic {
                operator,
                width,
                rounding,
            },
            vec![Bound::Rounding(result, rounding.as_str())],
        )
    };
    let ieee_comparison = |code, operands, comparison, width| {
        expression(
            code,
            BINARY,
            operands,
            "boolean",
            Op::IeeeComparison { comparison, width },
            Vec::new(),
        )
    };
    let wide_decimal = || decimal_type(-1000, 1000, 0, 2, RoundingMode::NearestEven);
    let quantity = |code, operands, operator| {
        expression(
            code,
            BINARY,
            operands,
            "unit",
            Op::QuantityArithmetic { operator },
            if operands == QTY_INT {
                vec![INT]
            } else {
                Vec::new()
            },
        )
    };

    let mut corpus = vec![
        expression(
            1140,
            CONVERSION,
            QTY1,
            "unit",
            Op::QuantityConversion,
            Vec::new(),
        ),
        integer(
            1001,
            BINARY,
            INT2,
            IntegerOperator::Add,
            bounded(-1000, 1000),
            INT,
        ),
        integer(
            1002,
            UNARY,
            INT1,
            IntegerOperator::Negate,
            bounded(-8, 7),
            Bound::Integer(-8, 7),
        ),
        integer(
            1003,
            BINARY,
            INT2,
            IntegerOperator::Subtract,
            bounded(-1000, 1000),
            INT,
        ),
        integer(
            1004,
            BINARY,
            INT2,
            IntegerOperator::Multiply,
            bounded(-1000, 1000),
            INT,
        ),
        division(1011, DivisionProfile::Truncating, -1000, 1000, INT),
        division(1012, DivisionProfile::Floor, -1000, 1000, INT),
        division(1013, DivisionProfile::Euclidean, -1000, 1000, INT),
        division(1014, DivisionProfile::Truncating, -5, 5, INT5),
        expression(
            1021,
            BINARY,
            INT2,
            "integer",
            Op::IntegerModulo {
                domain: bounded(-5, 5),
            },
            vec![INT5],
        ),
        rational(
            1031,
            BINARY,
            RAT2,
            RationalOperator::Add,
            rational_domain(-1000, 1000, 1, 1000),
            vec![RAT],
        ),
        rational(
            1032,
            BINARY,
            RAT2,
            RationalOperator::Divide,
            rational_domain(-10, 10, 1, 4),
            vec![Bound::Rational(-10, 10, 1, 4)],
        ),
        rational(
            1033,
            BINARY,
            INT2,
            RationalOperator::IntegerDivide,
            rational_domain(-1000, 1000, 1, 1000),
            vec![INT, RAT],
        ),
        rational(
            1034,
            BINARY,
            RAT2,
            RationalOperator::Subtract,
            rational_domain(-1000, 1000, 1, 1000),
            vec![RAT],
        ),
        rational(
            1035,
            BINARY,
            RAT2,
            RationalOperator::Multiply,
            rational_domain(-1000, 1000, 1, 1000),
            vec![RAT],
        ),
        rational(
            1036,
            UNARY,
            RAT1,
            RationalOperator::Negate,
            rational_domain(-1000, 1000, 1, 1000),
            vec![RAT],
        ),
        ordering(
            1041,
            INT2,
            OrderingOperator::Less,
            OrderingOperandKind::Integer,
            INT,
        ),
        ordering(
            1042,
            DEC2,
            OrderingOperator::LessOrEqual,
            OrderingOperandKind::Decimal,
            DEC,
        ),
        ordering(
            1043,
            RAT2,
            OrderingOperator::Greater,
            OrderingOperandKind::Rational,
            RAT,
        ),
        ordering(
            1044,
            INT2,
            OrderingOperator::LessOrEqual,
            OrderingOperandKind::Integer,
            INT,
        ),
        ordering(
            1045,
            INT2,
            OrderingOperator::GreaterOrEqual,
            OrderingOperandKind::Integer,
            INT,
        ),
        decimal(
            1051,
            BINARY,
            DEC2,
            DecimalOperator::Add,
            wide_decimal(),
            DEC,
        ),
        decimal(
            1052,
            BINARY,
            DEC2,
            DecimalOperator::Divide,
            decimal_type(-1000, 1000, 0, 2, RoundingMode::TowardZero),
            Bound::Decimal(-1000, 1000, 0, 2, "toward-zero"),
        ),
        decimal(
            1053,
            CONVERSION,
            DEC1,
            DecimalOperator::Round,
            decimal_type(-100, 100, 0, 0, RoundingMode::NearestAway),
            Bound::Decimal(-100, 100, 0, 0, "nearest-away"),
        ),
        decimal(
            1054,
            BINARY,
            DEC2,
            DecimalOperator::Subtract,
            wide_decimal(),
            DEC,
        ),
        decimal(
            1055,
            BINARY,
            DEC2,
            DecimalOperator::Multiply,
            wide_decimal(),
            DEC,
        ),
        decimal(
            1056,
            UNARY,
            DEC1,
            DecimalOperator::Negate,
            wide_decimal(),
            DEC,
        ),
        ieee(
            1061,
            F32_2,
            "float32",
            IeeeArithmeticOperator::Add,
            IeeeWidth::Binary32,
            RoundingMode::NearestEven,
        ),
        ieee(
            1062,
            F64_2,
            "float64",
            IeeeArithmeticOperator::Divide,
            IeeeWidth::Binary64,
            RoundingMode::TowardZero,
        ),
        ieee_comparison(1063, F64_2, IeeeComparison::TotalOrder, IeeeWidth::Binary64),
        expression(
            1064,
            CONVERSION,
            F64_1,
            "float32",
            Op::IeeeWidthConversion {
                source: IeeeWidth::Binary64,
                target: IeeeWidth::Binary32,
                rounding: RoundingMode::NearestEven,
            },
            vec![Bound::Rounding("float32", "nearest-even")],
        ),
        ieee(
            1065,
            F32_2,
            "float32",
            IeeeArithmeticOperator::Subtract,
            IeeeWidth::Binary32,
            RoundingMode::NearestEven,
        ),
        ieee(
            1066,
            F64_2,
            "float64",
            IeeeArithmeticOperator::Multiply,
            IeeeWidth::Binary64,
            RoundingMode::TowardPositive,
        ),
        ieee_comparison(
            1067,
            F32_2,
            IeeeComparison::NumericEqual,
            IeeeWidth::Binary32,
        ),
        ieee_comparison(
            1068,
            F64_2,
            IeeeComparison::BitIdentical,
            IeeeWidth::Binary64,
        ),
        quantity(1081, QTY2, QuantityOperator::Add),
        quantity(1082, QTY2, QuantityOperator::Multiply),
        quantity(1083, QTY_INT, QuantityOperator::Power),
        quantity(1088, QTY2, QuantityOperator::Subtract),
        quantity(1089, QTY2, QuantityOperator::Divide),
    ];
    for (index, operator) in ComparisonOperator::ALL.into_iter().enumerate() {
        corpus.push(expression(
            TEXT_COMPARISONS[index],
            BINARY,
            TEXT2,
            "boolean",
            Op::TextComparison { operator },
            vec![TEXT],
        ));
        corpus.push(expression(
            ENUM_COMPARISONS[index],
            BINARY,
            ENUM2,
            "boolean",
            Op::EnumComparison { operator },
            Vec::new(),
        ));
        corpus.push(expression(
            QUANTITY_COMPARISONS[index],
            BINARY,
            QTY2,
            "boolean",
            Op::QuantityComparison { operator },
            Vec::new(),
        ));
    }
    corpus.sort_by_key(|expression| expression.code);
    corpus
}

/// The operand value node of a scalar form.
fn operand(form: &str) -> String {
    match form {
        "integer" => key(V_INTEGER),
        "rational" => key(V_RATIONAL),
        "decimal" => key(V_DECIMAL),
        "float32" => key(V_FLOAT32),
        "float64" => key(V_FLOAT64),
        "text" => key(V_TEXT),
        "enum" => enum_member(),
        "unit" => key(V_QUANTITY),
        other => panic!("no operand node for {other}"),
    }
}

/// The type node of a scalar form.
fn result_type(form: &str) -> String {
    match form {
        "boolean" => key(T_BOOLEAN),
        "integer" => key(T_INTEGER),
        "rational" => key(T_RATIONAL),
        "decimal" => key(T_DECIMAL),
        "float32" => key(T_FLOAT32),
        "float64" => key(T_FLOAT64),
        "text" => key(T_TEXT),
        "unit" => unit_type(),
        other => panic!("no type node for {other}"),
    }
}

fn decimal_type_for_rounding(rounding: &str) -> String {
    key(match rounding {
        "nearest-even" => T_DECIMAL,
        "toward-zero" => T_DECIMAL_ZERO,
        "nearest-away" => T_DECIMAL_AWAY,
        other => panic!("no decimal fixture type for rounding {other}"),
    })
}

fn float_type_for_rounding(form: &str, rounding: &str) -> String {
    key(match (form, rounding) {
        ("float32", "nearest-even") => T_FLOAT32,
        ("float64", "nearest-even") => T_FLOAT64,
        ("float64", "toward-zero") => T_FLOAT64_ZERO,
        ("float64", "toward-positive") => T_FLOAT64_POSITIVE,
        _ => panic!("no {form} fixture type for rounding {rounding}"),
    })
}

/// `code` is folded into the second operand so callers that register
/// several distinct expression nodes sharing this body's shape
/// (operator/operation/result_type) still get distinct digests.
/// `bounds`/`dependencies` are not part of Contract IR's application identity;
/// only the `body` distinguishes these otherwise identical expressions, so nodes
/// that differ only by their bound set collide on id unless `body` itself
/// also varies. `operand_ref` is a caller-chosen node, never the plain
/// shared `V_INTEGER`, so a caller pairing this body with a bound gets a
/// reference [`PackageBuilder::dedicated_operand`] can safely anchor it on
/// (see that method's own doc for why the shared node cannot).
fn integer_pair_for_operand(code: u32, operand_ref: &str) -> Value {
    application(
        "binary",
        op("quire.op.integer.add"),
        &key(T_INTEGER),
        vec![
            reference(operand_ref),
            literal("integer", &code.to_string()),
        ],
    )
}

/// The catalogued `body.operator`/`operation` pair for one corpus
/// expression, matched on its own [`ExactScalarOperation`] descriptor
/// against the checked-operation catalog.
/// `operation` is not read by this crate's own generators -- they classify
/// a body by `term`/`operator`/`arguments` and the request item's own
/// descriptor -- so which catalogued identity denotes a given expression is
/// otherwise free; each arm below picks the catalog entry whose semantics
/// most directly match the descriptor, wiring its operand family, laws,
/// mode and member exactly as `validate_operations` checks them:
///
/// - Every arithmetic/ordering identity used here has `operator ==
///   entry.operator` because `corpus()`'s own `BINARY`/`UNARY`/`CONVERSION`
///   shape tuples already spell the same three catalog operator strings
///   (`binary`/`unary`/`convert`) -- the one exception is IEEE comparison,
///   whose only catalogued identities (`ieee.numeric_equal`/`total_order`/
///   `bit_identical`) are `call`, so that arm alone overrides `expr.operator`.
/// - `integer.div`/`.rem` and every `ieee.*`/`text.*` identity require a
///   law (`integer_division`/`ieee_profile`/`text_profile` respectively);
///   `corpus_package` selects the matching definitions into
///   `lock.definition_selections` up front so every one of these admits.
/// - `decimal.add/sub/mul/div`, every `ieee.*` arithmetic op and
///   `numeric.convert_rounding` are catalogued with a `rounding` mode;
///   `text.*` with a `text_profile` mode. Both kinds' `value` is unchecked
///   by IR here (see [`mode_kv`]'s own doc), so any readable string works.
fn corpus_operation(expr: &Expression) -> (&'static str, Value) {
    use ExactScalarOperation as Op;
    match &expr.operation {
        Op::Boolean { operator, .. } => (
            expr.operator,
            op(match operator {
                BooleanOperator::And => "quire.op.boolean.and",
                BooleanOperator::Or => "quire.op.boolean.or",
                BooleanOperator::Not => "quire.op.boolean.not",
                BooleanOperator::Implies => "quire.op.boolean.implies",
            }),
        ),
        Op::Equality { operator, operands } => (
            expr.operator,
            op(match (operands, operator) {
                (EqualityOperandKind::Boolean, EqualityOperator::Equal) => "quire.op.boolean.eq",
                (EqualityOperandKind::Boolean, EqualityOperator::NotEqual) => "quire.op.boolean.ne",
                (EqualityOperandKind::Integer, EqualityOperator::Equal) => "quire.op.integer.eq",
                (EqualityOperandKind::Integer, EqualityOperator::NotEqual) => "quire.op.integer.ne",
            }),
        ),
        Op::IntegerArithmetic { operator, .. } => {
            let identity = match operator {
                IntegerOperator::Add => "quire.op.integer.add",
                IntegerOperator::Subtract => "quire.op.integer.sub",
                IntegerOperator::Multiply => "quire.op.integer.mul",
                IntegerOperator::Negate => "quire.op.integer.negate",
            };
            (expr.operator, op(identity))
        }
        Op::IntegerDivision { profile, .. } => (
            expr.operator,
            op_full(
                "quire.op.integer.div",
                vec![law(
                    "integer_division",
                    integer_division_definition(*profile),
                )],
                None,
                None,
            ),
        ),
        Op::IntegerModulo { .. } => (expr.operator, op("quire.op.integer.mod")),
        Op::RationalArithmetic { operator, .. } => {
            let identity = match operator {
                RationalOperator::Add => "quire.op.rational.add",
                RationalOperator::Subtract => "quire.op.rational.sub",
                RationalOperator::Multiply => "quire.op.rational.mul",
                // `rational.div`'s operands are `rational_promotable`
                // (integer or rational), so it fits an integer/integer
                // pair too -- there is no separate catalogued "integer
                // division to a rational result" identity.
                RationalOperator::Divide | RationalOperator::IntegerDivide => {
                    "quire.op.rational.div"
                }
                RationalOperator::Negate => "quire.op.rational.negate",
            };
            (expr.operator, op(identity))
        }
        Op::Ordering {
            operator,
            operands: kind,
        } => {
            let family = match kind {
                OrderingOperandKind::Integer => "integer",
                OrderingOperandKind::Rational => "rational",
                OrderingOperandKind::Decimal => "decimal",
            };
            let suffix = ordering_suffix(*operator);
            let identity: &'static str = match (family, suffix) {
                ("integer", "lt") => "quire.op.integer.lt",
                ("integer", "le") => "quire.op.integer.le",
                ("integer", "gt") => "quire.op.integer.gt",
                ("integer", "ge") => "quire.op.integer.ge",
                ("rational", "lt") => "quire.op.rational.lt",
                ("rational", "le") => "quire.op.rational.le",
                ("rational", "gt") => "quire.op.rational.gt",
                ("rational", "ge") => "quire.op.rational.ge",
                ("decimal", "lt") => "quire.op.decimal.lt",
                ("decimal", "le") => "quire.op.decimal.le",
                ("decimal", "gt") => "quire.op.decimal.gt",
                ("decimal", "ge") => "quire.op.decimal.ge",
                _ => unreachable!("every (family, suffix) pair is covered above"),
            };
            (expr.operator, op(identity))
        }
        Op::DecimalArithmetic { operator, target } => match operator {
            DecimalOperator::Add => (
                expr.operator,
                op_full(
                    "quire.op.decimal.add",
                    vec![],
                    Some(mode_kv("rounding", target.rounding().as_str())),
                    None,
                ),
            ),
            DecimalOperator::Subtract => (
                expr.operator,
                op_full(
                    "quire.op.decimal.sub",
                    vec![],
                    Some(mode_kv("rounding", target.rounding().as_str())),
                    None,
                ),
            ),
            DecimalOperator::Multiply => (
                expr.operator,
                op_full(
                    "quire.op.decimal.mul",
                    vec![],
                    Some(mode_kv("rounding", target.rounding().as_str())),
                    None,
                ),
            ),
            DecimalOperator::Divide => (
                expr.operator,
                op_full(
                    "quire.op.decimal.div",
                    vec![],
                    Some(mode_kv("rounding", target.rounding().as_str())),
                    None,
                ),
            ),
            DecimalOperator::Negate => (expr.operator, op("quire.op.decimal.negate")),
            DecimalOperator::Round => (
                expr.operator,
                op_full(
                    "quire.op.numeric.convert_rounding",
                    vec![],
                    Some(mode_kv("rounding", target.rounding().as_str())),
                    Some(member_kind("type_argument")),
                ),
            ),
        },
        Op::IeeeArithmetic {
            operator,
            width,
            rounding,
        } => {
            let identity = match (width, operator) {
                (IeeeWidth::Binary32, IeeeArithmeticOperator::Add) => "quire.op.ieee.float32.add",
                (IeeeWidth::Binary32, IeeeArithmeticOperator::Subtract) => {
                    "quire.op.ieee.float32.sub"
                }
                (IeeeWidth::Binary32, IeeeArithmeticOperator::Multiply) => {
                    "quire.op.ieee.float32.mul"
                }
                (IeeeWidth::Binary32, IeeeArithmeticOperator::Divide) => {
                    "quire.op.ieee.float32.div"
                }
                (IeeeWidth::Binary64, IeeeArithmeticOperator::Add) => "quire.op.ieee.float64.add",
                (IeeeWidth::Binary64, IeeeArithmeticOperator::Subtract) => {
                    "quire.op.ieee.float64.sub"
                }
                (IeeeWidth::Binary64, IeeeArithmeticOperator::Multiply) => {
                    "quire.op.ieee.float64.mul"
                }
                (IeeeWidth::Binary64, IeeeArithmeticOperator::Divide) => {
                    "quire.op.ieee.float64.div"
                }
            };
            (
                expr.operator,
                op_full(
                    identity,
                    vec![law("ieee_profile", ieee_profile_definition())],
                    Some(mode_kv("rounding", rounding.as_str())),
                    None,
                ),
            )
        }
        Op::IeeeComparison { comparison, .. } => {
            let identity = match comparison {
                IeeeComparison::NumericEqual => "quire.op.ieee.numeric_equal",
                IeeeComparison::TotalOrder => "quire.op.ieee.total_order",
                IeeeComparison::BitIdentical => "quire.op.ieee.bit_identical",
            };
            (
                // Overrides `expr.operator` ("binary" per `corpus()`'s own
                // shape tuple): the catalog's only IEEE-comparison
                // identities are `call`, and `body.operator` must equal
                // the catalogued entry's own `operator` or IR refuses
                // `operation-class-mismatch`. `expr.form` (the node's own
                // `semantic_form`, unrelated to this field) stays "binary".
                "call",
                op_full(
                    identity,
                    vec![law("ieee_profile", ieee_profile_definition())],
                    None,
                    None,
                ),
            )
        }
        Op::IeeeWidthConversion {
            source,
            target,
            rounding,
        } => match (source, target) {
            (IeeeWidth::Binary64, IeeeWidth::Binary32) => (
                expr.operator,
                op_full(
                    "quire.op.ieee.to_float32",
                    vec![law("ieee_profile", ieee_profile_definition())],
                    Some(mode_kv("rounding", rounding.as_str())),
                    Some(member_kind("type_argument")),
                ),
            ),
            (IeeeWidth::Binary32, IeeeWidth::Binary64) => (
                expr.operator,
                op_full(
                    "quire.op.ieee.to_float64",
                    vec![law("ieee_profile", ieee_profile_definition())],
                    None,
                    Some(member_kind("type_argument")),
                ),
            ),
            (source, target) => panic!("no catalog identity for ieee {source:?} -> {target:?}"),
        },
        // No catalogued `convert` identity accepts a `text` operand family
        // (the corpus loop below swaps this expression's own operand for a
        // literal, so IR's operand-family check never resolves one to
        // check); `numeric.convert`'s `exact_numeric` operand expectation
        // is therefore never actually exercised, only its `convert`
        // operator and `type_argument` member shape.
        Op::TextAdmission { .. } => (
            expr.operator,
            op_full(
                "quire.op.numeric.convert",
                vec![],
                None,
                Some(member_kind("type_argument")),
            ),
        ),
        Op::TextComparison { operator } => (
            expr.operator,
            op_full(
                text_family_identity("text", *operator),
                vec![law("text_profile", text_profile_definition())],
                Some(mode_kv("text_profile", "nfc")),
                None,
            ),
        ),
        Op::EnumComparison { operator } => {
            (expr.operator, op(text_family_identity("enum", *operator)))
        }
        Op::QuantityConversion => (
            expr.operator,
            op_full(
                "quire.op.quantity.convert",
                vec![],
                Some(mode_kv("rounding", "exact")),
                Some(member_kind("type_argument")),
            ),
        ),
        Op::QuantityArithmetic { operator } => {
            let identity = match operator {
                QuantityOperator::Add => "quire.op.quantity.add",
                QuantityOperator::Subtract => "quire.op.quantity.sub",
                QuantityOperator::Multiply => "quire.op.quantity.mul",
                QuantityOperator::Divide => "quire.op.quantity.div",
                QuantityOperator::Power => "quire.op.quantity.pow",
            };
            (expr.operator, op(identity))
        }
        Op::QuantityComparison { operator } => (
            expr.operator,
            op(text_family_identity("quantity", *operator)),
        ),
    }
}

/// `lt`/`le`/`gt`/`ge` per [`OrderingOperator`] variant, shared by
/// [`corpus_operation`]'s integer/rational/decimal `Ordering` arm.
fn ordering_suffix(operator: OrderingOperator) -> &'static str {
    match operator {
        OrderingOperator::Less => "lt",
        OrderingOperator::LessOrEqual => "le",
        OrderingOperator::Greater => "gt",
        OrderingOperator::GreaterOrEqual => "ge",
    }
}

/// `quire.op.<family>.<suffix>` for one of the catalog's six comparison
/// identities per family (`eq`/`ne`/`lt`/`le`/`gt`/`ge`), matching
/// [`ComparisonOperator`]'s six variants in the same order used by
/// `TEXT_COMPARISONS`/`ENUM_COMPARISONS`/`QUANTITY_COMPARISONS`.
fn text_family_identity(family: &str, operator: ComparisonOperator) -> &'static str {
    match (family, operator) {
        ("text", ComparisonOperator::Equal) => "quire.op.text.eq",
        ("text", ComparisonOperator::NotEqual) => "quire.op.text.ne",
        ("text", ComparisonOperator::Less) => "quire.op.text.lt",
        ("text", ComparisonOperator::LessOrEqual) => "quire.op.text.le",
        ("text", ComparisonOperator::Greater) => "quire.op.text.gt",
        ("text", ComparisonOperator::GreaterOrEqual) => "quire.op.text.ge",
        ("enum", ComparisonOperator::Equal) => "quire.op.enum.eq",
        ("enum", ComparisonOperator::NotEqual) => "quire.op.enum.ne",
        ("enum", ComparisonOperator::Less) => "quire.op.enum.lt",
        ("enum", ComparisonOperator::LessOrEqual) => "quire.op.enum.le",
        ("enum", ComparisonOperator::Greater) => "quire.op.enum.gt",
        ("enum", ComparisonOperator::GreaterOrEqual) => "quire.op.enum.ge",
        ("quantity", ComparisonOperator::Equal) => "quire.op.quantity.eq",
        ("quantity", ComparisonOperator::NotEqual) => "quire.op.quantity.ne",
        ("quantity", ComparisonOperator::Less) => "quire.op.quantity.lt",
        ("quantity", ComparisonOperator::LessOrEqual) => "quire.op.quantity.le",
        ("quantity", ComparisonOperator::Greater) => "quire.op.quantity.gt",
        ("quantity", ComparisonOperator::GreaterOrEqual) => "quire.op.quantity.ge",
        (family, operator) => panic!("no catalog identity for {family} {operator:?}"),
    }
}

/// Adds one corpus `expression` to `builder`: its operand nodes, its bound anchors and the
/// application node itself.
fn add_corpus_expression(builder: &mut PackageBuilder, expression: Expression) {
    use ExactScalarOperation as Op;
    // A form whose operand value is a fixed literal (`dedicated_operand`'s
    // own match) gets a bound-set-dedicated node instead of the plain
    // shared one whenever this expression declares a bound: see
    // `dedicated_operand`'s own doc for why sharing would otherwise union
    // unrelated expressions' bounds onto one reachable closure. "enum"
    // operands are never paired with a varying bound in this corpus
    // (every `EnumComparison` vector below declares `Vec::new()`), so
    // they keep the plain shared node unconditionally. "unit" operands
    // *do* get bounds here (`quantity.pow`'s `INT` domain), so they are
    // dedicated the same way via [`PackageBuilder::dedicated_unit_operand`]
    // whenever `bound_keys` is non-empty, rather than kept on the shared
    // `V_QUANTITY` node.
    // Text admission is an isolated reader negative for operator eligibility.
    // Its bounds play no part before that refusal and would add unrelated
    // structural constraints to the fixture.
    let bound_keys: Vec<String> = if matches!(expression.operation, Op::TextAdmission { .. }) {
        Vec::new()
    } else {
        expression
            .bounds
            .iter()
            .map(|bound| builder.bound(bound))
            .collect()
    };
    let pinned_type = match &expression.operation {
        Op::DecimalArithmetic { target, .. } => {
            Some(decimal_type_for_rounding(target.rounding().as_str()))
        }
        Op::IeeeArithmetic { rounding, .. } => Some(float_type_for_rounding(
            expression.result,
            rounding.as_str(),
        )),
        Op::IeeeWidthConversion { rounding, .. } => Some(float_type_for_rounding(
            expression.result,
            rounding.as_str(),
        )),
        _ => None,
    };
    let mut unit_anchor: Option<String> = None;
    let mut arguments = expression
        .operands
        .iter()
        .map(|form| {
            let node = if bound_keys.is_empty() || *form == "enum" {
                operand(form)
            } else if *form == "unit" {
                // Dedicated rather than the plain shared `V_QUANTITY`
                // (see the loop's own comment above): `V_QUANTITY` is
                // shared by all 13 quantity-typed corpus items, so a
                // bound wired onto it would be reachable from every one
                // of them, not just the item that declared it.
                let anchor = builder.dedicated_unit_operand(&bound_keys);
                unit_anchor = Some(anchor.clone());
                anchor
            } else if expression.code == LITERAL_OPERAND {
                // `LITERAL_OPERAND` needs an anchor nothing else in the
                // corpus can ever share, because `CLAIM`/`CLAIM_ALT` are
                // wired as dependencies of this exact node further below.
                // The plain `dedicated_operand` call is keyed only by
                // `(form, bound_keys)`, and nine other bounded-integer
                // corpus expressions (add, mul, lt, le, ge, the three
                // division profiles, and `quantity.pow`) share the
                // identical `("integer", [INT])` pair, so without this
                // salt `CLAIM`/`CLAIM_ALT` would be reachable from all of
                // them too, not just from `LITERAL_OPERAND`.
                builder.dedicated_operand_tagged(form, &bound_keys, "literal-operand-claim-anchor")
            } else if *form == expression.result && pinned_type.is_some() {
                builder.dedicated_typed_operand(
                    form,
                    &bound_keys,
                    pinned_type.as_deref().expect("pinned type"),
                )
            } else {
                builder.dedicated_operand(form, &bound_keys)
            };
            reference(&node)
        })
        .collect::<Vec<_>>();
    if expression.code == LITERAL_OPERAND {
        arguments[1] = literal("integer", "3");
    }
    // Code 1014 is `division(1014, Truncating, -5, 5, INT5)`: same
    // profile, operands and result as code 1011's
    // `division(1011, Truncating, -1000, 1000, INT)`. The two exist to
    // be distinguished only by which bound is reachable (`INT` vs
    // `INT5`), but bounds aren't part of the node-id preimage, so
    // without a body difference they'd collide on digest and IR's
    // `validate_graph` would refuse the whole package as a duplicate
    // node id. A literal second operand keeps 1014 a genuine, distinct
    // node without touching the domain-mismatch behavior under test.
    if expression.code == 1014 {
        arguments[1] = literal("integer", "1014");
    }
    // No catalogued `convert` identity accepts a `text` operand family
    // (see `corpus_operation`'s `TextAdmission` arm), so Contract IR refuses
    // these nodes at admission: they are built only for `refused_corpus_package`.
    // The operand is a literal.
    if matches!(expression.operation, Op::TextAdmission { .. }) {
        // `corpus_operation`'s `TextAdmission` arm ignores which
        // `TextProfile` this expression names (see its own comment),
        // so every `TEXT_ADMISSIONS` code would otherwise get the
        // identical `operator`/`operation`/`result_type` and, with
        // this same fixed literal, an identical body -- and thus an
        // identical node id, which IR's `validate_graph` refuses as a
        // duplicate. Folding `code` into the literal keeps each one a
        // distinct node without touching what's actually exercised
        // (the literal's family/value, not its exact text).
        //
        let Some(Bound::Text(_, _, profile)) = expression.bounds.first() else {
            panic!("text-admission negative has its declared text profile");
        };
        arguments = vec![literal("text", &format!("a{}-{profile}", expression.code))];
    }
    let (catalog_operator, mut operation) = corpus_operation(&expression);
    let node_semantic_type = pinned_type.unwrap_or_else(|| result_type(expression.result));
    // `corpus_operation` does not know the result type, which a
    // `type_argument` member must name as its declaration.
    if operation["member"] == member_kind("type_argument") {
        operation["member"] = type_argument_member(&node_semantic_type);
    }
    let node_bounds: &[Bound] = if matches!(expression.operation, Op::TextAdmission { .. }) {
        &[]
    } else {
        &expression.bounds
    };
    // `unit_anchor` (set above only when this expression's "unit" operand
    // was dedicated) is passed explicitly rather than left for
    // `application_bounded` to infer: `quantity.pow`'s body references
    // both a dedicated "unit" node and a dedicated "integer" node, and
    // `application_bounded`'s own inference picks whichever of the two
    // sorts first by digest -- not necessarily the dedicated "unit" node
    // -- which is exactly the bug this anchor makes explicit instead of
    // leaving to chance.
    builder.application_bounded_anchored(
        expression.code,
        "expression",
        expression.form,
        &node_semantic_type,
        application(catalog_operator, operation, &node_semantic_type, arguments),
        node_bounds,
        unit_anchor.as_deref(),
    );
}

/// A minimal checked wire proving a non-exact Float32 mode over bare Float32
/// types is refused before CG can classify its missing rounding bound.
pub fn missing_rounding_package() -> PackageBuilder {
    let mut builder = PackageBuilder::default();
    builder.code(
        T_FLOAT32,
        "scalar_type",
        "float32",
        &key(T_FLOAT32),
        aggregate(),
    );
    builder.code(
        V_FLOAT32,
        "value",
        "literal",
        &key(T_FLOAT32),
        literal("float32_bits", "0"),
    );
    builder.select_definition(ieee_profile_definition());
    builder.application_bounded(
        MISSING_ROUNDING,
        "expression",
        "binary",
        &key(T_FLOAT32),
        application(
            "binary",
            op_full(
                "quire.op.ieee.float32.add",
                vec![law("ieee_profile", ieee_profile_definition())],
                Some(mode_kv("rounding", "nearest-even")),
                None,
            ),
            &key(T_FLOAT32),
            vec![reference(&key(V_FLOAT32)), literal("float32_bits", "1")],
        ),
        &[],
    );
    builder
}

/// The complete package: types, values, the corpus, and refused nodes.
fn corpus_package_with_expression_operand() -> PackageBuilder {
    let mut builder = PackageBuilder::default();
    for (code, form) in [
        (T_BOOLEAN, "boolean"),
        (T_INTEGER, "integer"),
        (T_RATIONAL, "rational"),
        (T_DECIMAL, "decimal"),
        (T_FLOAT32, "float32"),
        (T_FLOAT64, "float64"),
        (T_TEXT, "text"),
    ] {
        let body = if matches!(form, "decimal" | "float32" | "float64") {
            json!({"term": "aggregate", "members": [member("rounding", literal("text", "nearest-even"))]})
        } else {
            aggregate()
        };
        builder.code(code, "scalar_type", form, &key(code), body);
    }
    for (code, rounding) in [
        (T_DECIMAL_ZERO, "toward-zero"),
        (T_DECIMAL_AWAY, "nearest-away"),
    ] {
        builder.code(
            code,
            "scalar_type",
            "decimal",
            &key(code),
            json!({"term": "aggregate", "members": [member("rounding", literal("text", rounding))]}),
        );
    }
    for (code, form, rounding) in [
        (T_FLOAT64_ZERO, "float64", "toward-zero"),
        (T_FLOAT64_POSITIVE, "float64", "toward-positive"),
    ] {
        builder.code(
            code,
            "scalar_type",
            form,
            &key(code),
            json!({"term": "aggregate", "members": [member("rounding", literal("text", rounding))]}),
        );
    }
    for (code, ty, kind, value) in [
        (V_INTEGER, key(T_INTEGER), "integer", "3"),
        (V_RATIONAL, key(T_RATIONAL), "rational", "1/2"),
        (V_DECIMAL, key(T_DECIMAL), "decimal", "1.5"),
        (V_FLOAT32, key(T_FLOAT32), "float32_bits", "0"),
        (V_FLOAT64, key(T_FLOAT64), "float64_bits", "0"),
        (V_TEXT, key(T_TEXT), "text", "a"),
    ] {
        builder.code(code, "value", "literal", &ty, literal(kind, value));
    }
    // `V_QUANTITY`'s own magnitude literal is `rational`-typed even though
    // the node's `semantic_type` is `unit_type()`; Contract IR's lowering
    // reaches that `literal.type` edge and, with `require_bounds`, requires
    // a `rational_range` bound somewhere in the same closure. Every corpus
    // expression that references this node needs one reachable, so it is
    // wired as `V_QUANTITY`'s own dependency rather than repeated per call
    // site.
    let rational_bound = builder.bound(&RAT);
    builder.node_with(
        &key(V_QUANTITY),
        "value",
        "literal",
        &unit_type(),
        literal("rational", "1"),
        &[rational_bound],
    );
    builder.code(
        V_BOOLEAN,
        "value",
        "literal",
        &key(T_BOOLEAN),
        json!({
            "term": "literal",
            "type": node_ref(&key(T_BOOLEAN)),
            "value_kind": "boolean",
            "value": true,
        }),
    );
    let int_bound = builder.bound(&INT);
    builder.code(
        V_BOUNDED_PARAM,
        "value",
        "literal",
        &int_bound,
        literal("integer", "3"),
    );
    builder.code(
        V_UNTYPED,
        "value",
        "literal",
        &key(V_BOOLEAN),
        literal("integer", "3"),
    );
    // Every catalogued law-role definition any corpus expression's
    // `operation.laws` names below (see `corpus_operation`), selected once
    // up front so `validate_operations`'s law-selection check finds each
    // one in `lock.definition_selections`.
    for profile in DivisionProfile::ALL {
        builder.select_definition(integer_division_definition(profile));
    }
    builder
        .select_definition(ieee_profile_definition())
        .select_definition(text_profile_definition());
    for expression in corpus() {
        add_corpus_expression(&mut builder, expression);
    }
    let integer_type = key(T_INTEGER);
    for (code, bounds) in [
        (DUPLICATED, vec![INT]),
        (WRONG_RESULT, vec![INT]),
        (WRONG_ARITY, vec![INT]),
        (UNBOUNDED, Vec::new()),
        (MATHEMATICAL, vec![INT]),
        (AMBIGUOUS, vec![INT, INT5]),
        (WRONG_BOUND_FORM, vec![wrong_form_bound()]),
        (DOMAIN_MISMATCH, vec![INT5]),
    ] {
        let bound_keys: Vec<String> = bounds.iter().map(|bound| builder.bound(bound)).collect();
        let operand_ref = if bound_keys.is_empty() {
            key(V_INTEGER)
        } else {
            builder.dedicated_operand("integer", &bound_keys)
        };
        builder.application_bounded(
            code,
            "expression",
            "binary",
            &integer_type,
            integer_pair_for_operand(code, &operand_ref),
            &bounds,
        );
    }
    // Distinct admitted target declarations keep provenance independent of operand bounds.
    let mut alternate = builder.value["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["node_id"] == node_ref(&unit_type()))
        .unwrap()
        .clone();
    alternate["nominal_identity_preimage"]["qualified_declaration"] =
        json!(["Example", "other_pace"]);
    alternate["declaration"]["qualified_name"] = json!(["Example", "other_pace"]);
    alternate["nominal_identity_preimage"]["target_unit_node_id"] = node_ref(&unit_type());
    let mut dependencies = [base_node_ids().dimension, unit_type()];
    dependencies.sort();
    alternate["dependencies"] = json!(dependencies
        .iter()
        .map(|key| node_ref(key))
        .collect::<Vec<_>>());
    let alternate_key = base_preimage_digest(alternate["nominal_identity_preimage"].clone());
    alternate["node_id"] = node_ref(&alternate_key);
    builder.value["semantic_graph"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(alternate);
    let mut correspondence = builder.value["source_map"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["node_id"] == node_ref(&unit_type()))
        .unwrap()
        .clone();
    correspondence["node_id"] = node_ref(&alternate_key);
    builder.value["source_map"]
        .as_array_mut()
        .unwrap()
        .push(correspondence);
    let int_key = builder.bound(&INT);
    for (code, operand, bounds) in [
        (
            QUANTITY_CONVERSIONS[0],
            builder.dedicated_unit_operand(&[]),
            Vec::new(),
        ),
        (
            QUANTITY_CONVERSIONS[1],
            builder.dedicated_unit_operand(&[]),
            Vec::new(),
        ),
        (
            QUANTITY_CONVERSIONS[2],
            builder.dedicated_unit_operand(&[int_key]),
            vec![INT],
        ),
    ] {
        let target = if code == QUANTITY_CONVERSIONS[1] {
            alternate_key.clone()
        } else {
            unit_type()
        };
        builder.application_bounded(
            code,
            "expression",
            "conversion",
            &target,
            application(
                "convert",
                op_full(
                    "quire.op.quantity.convert",
                    vec![],
                    Some(mode_kv("rounding", "exact")),
                    Some(type_argument_member(&target)),
                ),
                &target,
                vec![reference(&operand)],
            ),
            &bounds,
        );
    }
    builder.application_bounded(
        LITERAL_QUANTITY,
        "expression",
        "binary",
        &unit_type(),
        application(
            "binary",
            op("quire.op.quantity.add"),
            &unit_type(),
            // A deliberate negative fixture QSL never emits (it builds no quantity literal).
            // A quantity literal is typed by its unit: IR resolves a literal operand
            // through `literal.type`, and `quantity.add` takes a quantity on both sides.
            vec![
                reference(&key(V_QUANTITY)),
                json!({
                    "term": "literal",
                    "type": node_ref(&unit_type()),
                    "value_kind": "rational",
                    "value": "1",
                }),
            ],
        ),
        &[],
    );
    // IR-280's FR-322 join means each of these must anchor its own bound on
    // a node no other expression's differing bound also reaches (see
    // `dedicated_operand`'s doc): sharing `V_INTEGER`/`V_DECIMAL` directly
    // would union every one of these bounds onto whatever else references
    // the same shared node -- exactly the `AmbiguousBound` IR-280 catches.
    let expression_operand_anchor = {
        let int_key = builder.bound(&INT);
        builder.dedicated_operand("integer", &[int_key])
    };
    builder.application_bounded(
        EXPRESSION_OPERAND,
        "expression",
        "binary",
        &integer_type,
        application(
            "binary",
            op("quire.op.integer.add"),
            &integer_type,
            // An operand that is neither a literal nor a reference. The V2 reader's flat
            // wire refuses an application nested in `body.arguments[*]` as `malformed_wire`
            // (FR-038 "The flat wire"), so the operand is an `aggregate`, a term the reader
            // admits there and that no scalar operation takes as an operand. Its embedded
            // `reference` is still picked up by
            // `reference_targets`'s recursive walk (FR-322 says "anywhere"),
            // so it must name `expression_operand_anchor` too, not the
            // plain shared `V_INTEGER` -- otherwise this node's exact join
            // would still include `V_INTEGER`, right back to the
            // `AmbiguousBound` `dedicated_operand` exists to avoid.
            vec![
                json!({
                    "term": "aggregate",
                    "members": [member("operand", reference(&expression_operand_anchor))],
                }),
                reference(&expression_operand_anchor),
            ],
        ),
        &[INT],
    );
    let untyped_operand_anchor = {
        let int_key = builder.bound(&INT);
        builder.dedicated_operand("integer", &[int_key])
    };
    builder.application_bounded(
        UNTYPED_OPERAND,
        "expression",
        "binary",
        &integer_type,
        application(
            "binary",
            op("quire.op.integer.add"),
            &integer_type,
            vec![
                reference(&untyped_operand_anchor),
                reference(&key(V_UNTYPED)),
            ],
        ),
        &[INT],
    );
    let bounded_operand_anchor = {
        let int_key = builder.bound(&INT);
        builder.dedicated_operand("integer", &[int_key])
    };
    builder.application_bounded(
        BOUNDED_OPERAND,
        "expression",
        "binary",
        &integer_type,
        application(
            "binary",
            op("quire.op.integer.add"),
            &integer_type,
            vec![
                reference(&bounded_operand_anchor),
                reference(&key(V_BOUNDED_PARAM)),
            ],
        ),
        &[INT],
    );
    let boolean = key(T_BOOLEAN);
    let clause_profile = artifact_ref("quire.temporal.event-position.false-extension/v1");
    builder.select_profile("temporal_profile", clause_profile.clone());
    // The formula is registered by the chain below, ahead of the clause that names it.
    let clause_formula = {
        let body = application(
            "temporal_formula",
            op("quire.op.temporal.true"),
            &boolean,
            Vec::new(),
        );
        builder.application_code(TEMPORAL_FORMULA, "temporal", "formula", &boolean, body);
        code_id(TEMPORAL_FORMULA).digest.to_string()
    };
    builder.code(
        TEMPORAL_PARAMETER,
        "value",
        "parameter",
        &key(T_INTEGER),
        parameter_body("p", 0),
    );
    builder
        .code(COMPOSITE, "composite_type", "record", &boolean, aggregate())
        // `FUNCTION`/`TEMPORAL`/`PROTOCOL` are non-`expression`-tagged
        // nodes CG refuses on tag alone -- see `refused_items` below -- but
        // each still carries an `application` body, so IR-216 validates it
        // like any other application node. Their bodies are simplified to
        // a minimal catalogued `boolean.not` application (unrelated to
        // what each node tag actually denotes) rather than kept as
        // 0-argument placeholders: the catalog has no 0-operand identity
        // for any operator, so a 0-argument application could never be
        // made catalog-conformant at all.
        .application_code(
            FUNCTION,
            "function",
            "pure_function",
            &boolean,
            application(
                "unary",
                op("quire.op.boolean.not"),
                &boolean,
                vec![literal("boolean", "true")],
            ),
        )
        .code(MODEL, "model", "model_import", &boolean, aggregate())
        .code(
            RELATION,
            "relation",
            "relationship",
            &key(RELATION),
            aggregate(),
        )
        .code(STATE, "state", "snapshot", &boolean, aggregate())
        // IR admits a `temporal_clause` only as a `temporal`-operator application over a
        // declared `parameter` with one `temporal_profile` law and a `temporal` formula argument
        // (the QSpec temporal-clause rule), so this node is that minimal clause rather than a
        // `boolean.not` stand-in. CG refuses it as an unsupported `temporal` family; IR checks
        // the admitted closure in node-id order, which puts this clause before its formula.
        .application_code_with(
            TEMPORAL,
            "temporal",
            "temporal_clause",
            &boolean,
            application(
                "temporal",
                op_full(
                    "quire.op.temporal.clause",
                    vec![law("temporal_profile", clause_profile)],
                    None,
                    None,
                ),
                &boolean,
                vec![
                    reference(&key(TEMPORAL_PARAMETER)),
                    literal("text", "c"),
                    aggregate(),
                    aggregate(),
                    aggregate(),
                    reference(&clause_formula),
                ],
            ),
            &[key(TEMPORAL_PARAMETER), clause_formula.clone()],
        )
        .application_code(
            PROTOCOL,
            "protocol",
            "protocol_clause",
            &boolean,
            application(
                "unary",
                op("quire.op.boolean.not"),
                &boolean,
                vec![literal("boolean", "true")],
            ),
        );
    // Each anchor is dedicated to its own fixture's exact bound set (see
    // `dedicated_operand`'s doc): `key(V_INTEGER)` directly would union
    // these bounds onto every other expression that still references the
    // shared node too.
    let calls_function_anchor = {
        let int_key = builder.bound(&INT);
        builder.dedicated_operand("integer", &[int_key])
    };
    builder.application_bounded(
        CALLS_FUNCTION,
        "expression",
        "binary",
        &integer_type,
        application(
            // `quire.op.integer.add`'s two operands both require the
            // `integer` family (the catalog's own entry, not the broader
            // `exact_numeric` group); `FUNCTION` resolves to the catalog's
            // own `function` family (its `node_tag` is `function`, matched
            // directly by `resolve_family`), which no numeric identity's
            // operand family admits, so that pairing refuses
            // `IllTyped`/`OperatorIneligible` at admission before CG's own
            // upstream-blocked check (which runs at generation time, over
            // IR's already-lowered graph) is ever reached.
            // `quire.op.function.call` is the one catalog identity built for
            // exactly this shape: its first operand's family is `function`
            // outright and its `rest` accepts `any_term`, so `FUNCTION` and
            // a plain integer reference both admit unchanged.
            "call",
            op("quire.op.function.call"),
            &integer_type,
            // `FUNCTION` is itself an application-bodied node built above in
            // this same chain, so its real digest is already registered;
            // `key(FUNCTION)` would be the stale placeholder and leave this
            // reference dangling.
            vec![
                reference(&code_id(FUNCTION).digest),
                reference(&calls_function_anchor),
            ],
        ),
        &[INT],
    );
    let wrong_body_anchor = {
        let int_key = builder.bound(&INT);
        builder.dedicated_operand("integer", &[int_key])
    };
    builder.application_bounded(
        WRONG_BODY,
        "expression",
        "binary",
        &integer_type,
        application(
            "unary",
            op("quire.op.integer.negate"),
            &integer_type,
            vec![reference(&wrong_body_anchor)],
        ),
        &[INT],
    );
    let wrong_operand_anchor = {
        let keys = [builder.bound(&INT), builder.bound(&DEC)];
        builder.dedicated_operand("integer", &keys)
    };
    builder.application_bounded(
        WRONG_OPERAND,
        "expression",
        "binary",
        &integer_type,
        application(
            "binary",
            op("quire.op.integer.add"),
            &integer_type,
            // Contract IR resolves a literal operand's family through `literal.type`, so a
            // literal typed by `T_DECIMAL` would be `OperatorIneligible` at admission. It does
            // not cross-check `literal.type` against `value_kind`, so a literal typed by the
            // integer type but carrying the `decimal` value kind admits, while CG's
            // `check_operand` classifies it by its own `value_kind` and refuses the same
            // `OperandTypeMismatch { position: 1, expected: Integer, found: Some("decimal") }`.
            // A deliberate negative fixture: the type and `value_kind` disagree, and QSL builds no
            // decimal literal, so it never emits this shape.
            vec![
                reference(&wrong_operand_anchor),
                json!({
                    "term": "literal",
                    "type": node_ref(&key(T_INTEGER)),
                    "value_kind": "decimal",
                    "value": "1.5",
                }),
            ],
        ),
        &[INT, DEC],
    );
    let wrong_operand_reference_anchor = {
        let keys = [builder.bound(&INT), builder.bound(&RAT)];
        builder.dedicated_operand("integer", &keys)
    };
    builder.application_bounded(
        WRONG_OPERAND_REFERENCE,
        "expression",
        "binary",
        &key(T_RATIONAL),
        application(
            "binary",
            op("quire.op.rational.div"),
            &key(T_RATIONAL),
            // First operand is a `reference` to an integer-family node:
            // `quire.op.rational.div`'s catalogued operand family is
            // `rational_promotable` (`{integer, rational}`, see the
            // catalog's own `groups` table), so an integer-typed reference
            // admits at IR -- unlike `WRONG_OPERAND` above, this exercises
            // `check_operand`'s `"reference"` arm (`reference_form` -> a
            // graph lookup -> `type_form`), not its `"literal"` arm. CG's
            // own `Shape::of` for `RationalOperator::Divide` still requires
            // `[Rational, Rational]`, so generation refuses the first
            // operand with `OperandTypeMismatch { position: 0, expected:
            // Rational, found: Some("integer") }` before the second operand
            // is ever reached -- it is a literal only to keep this node's
            // body distinct from corpus code 1033's (`rational(1033, ...,
            // RationalOperator::IntegerDivide, ...)`), which already pairs
            // two integer-family reference operands with this same
            // catalogued identity under a descriptor `Shape::of` accepts.
            // This is the same shape a real catalog family being coarser
            // than CG's own `ScalarForm` produces in production: e.g. two
            // integer-typed reference operands requested against
            // `quire.op.rational.div` under a `Rational` descriptor.
            vec![
                reference(&wrong_operand_reference_anchor),
                literal("integer", "2032"),
            ],
        ),
        &[INT, RAT],
    );
    // FR-014-AC-5 / issue #100: no fixture node before this one carried the
    // `claim` tag, so `claim-map.json`'s `claims` field was structurally
    // always empty and TC-024 never exercised it (measured on origin/main:
    // zero `"node_tag": "claim"` occurrences anywhere in this file). These
    // two nodes are genuine `claim` nodes -- `CLAIM`'s `operation`/`body`
    // shape is adapted from Contract IR's own admitted
    // conformance fixture
    // (`tests/fixtures/checked-package/checked-package-v2/fixtures/
    // positive-all-families.json`, node index 11), which IR's own `admit()`
    // path accepts, not invented here. Two fields differ from that fixture
    // rather than matching it byte-for-byte, and IR admits both: `member`
    // there is `{"kind": "profile_operator", "operator": "always"}`,
    // while `member_kind` below emits only `{"kind": "profile_operator"}`
    // (IR checks presence and `kind` only, per `member_kind`'s own doc);
    // and `revision.value` there is `1-draft.3`, while `artifact_ref`'s
    // fixed `1-draft.1` is used here (that field is never checked against
    // anything, so this stays consistent with every other artifact ref
    // this module builds). Neither difference is invented to be wrong --
    // both are IR-admitted shapes, just not the exact fixture bytes.
    //
    // Both carry no dependencies of their own: `semantic_type` and
    // `result_type` all point at the corpus's existing `T_INTEGER` node,
    // already reachable from `LITERAL_OPERAND`, so neither adds any
    // *other* new node to that item's closure. Neither is ever itself
    // requested -- see `CLAIM`/`CLAIM_ALT`'s own doc comments -- and both
    // are wired below as extra dependencies of `LITERAL_OPERAND`.
    // Registered last, so their own synthetic source-map positions do not
    // shift any earlier node's.
    //
    // A single populated `claims` entry left the field's ordering and
    // truncation unfalsifiable against mutation (PR #132 review measured
    // this: a `.take(1)` truncation and a `.reverse()` both stayed green
    // over one element -- the same mutation on `bounds`, which has
    // multi-element arrays in this corpus, already goes red). `CLAIM_ALT`
    // is a second, distinct claim node -- a different `semantic_form`
    // (`analysis_claim`, not `verification_claim`) and a different
    // `temporal_profile` law -- so `LITERAL_OPERAND`'s `claims` array
    // gets two entries in a determinate (ascending node-id) order.
    // A profile-role law has no closed catalog to select from (see
    // `select_profile`'s own doc) -- any well-formed artifact ref the lock
    // selects under that role admits -- so both laws are locally chosen
    // well-formed values, distinct by construction.
    let temporal_profile = artifact_ref("quire.temporal.event-position.false-extension/v1");
    builder.select_profile("temporal_profile", temporal_profile.clone());
    builder.application_code(
        CLAIM,
        "claim",
        "verification_claim",
        &key(T_INTEGER),
        application(
            "claim",
            op_full(
                "quire.op.claim.clause",
                vec![law("temporal_profile", temporal_profile)],
                None,
                Some(member_kind("profile_operator")),
            ),
            &key(T_INTEGER),
            Vec::new(),
        ),
    );
    let temporal_profile_alt = artifact_ref("quire.temporal.event-position.true-extension/v1");
    builder.select_profile("temporal_profile", temporal_profile_alt.clone());
    builder.application_code(
        CLAIM_ALT,
        "claim",
        "analysis_claim",
        &key(T_INTEGER),
        application(
            "claim",
            op_full(
                "quire.op.claim.clause",
                vec![law("temporal_profile", temporal_profile_alt)],
                None,
                Some(member_kind("profile_operator")),
            ),
            &key(T_INTEGER),
            Vec::new(),
        ),
    );
    // IR-280's FR-322 application-node dependency join forbids `LITERAL_OPERAND`
    // (an application node) from carrying any dependency beyond its own body's
    // exact `reference` join, so `CLAIM`/`CLAIM_ALT` anchor instead on the one
    // node that join already names: `LITERAL_OPERAND`'s own first argument.
    // That is no longer the shared `V_INTEGER` (see `dedicated_operand`'s doc:
    // `LITERAL_OPERAND` declares the `INT` bound, so the main corpus loop
    // above already gave it a bound-dedicated operand instead, to keep other
    // expressions' bounds from unioning onto `V_INTEGER`'s closure) -- so
    // this recomputes that same dedicated node rather than naming `V_INTEGER`
    // directly. It must not, however, be the *same* `dedicated_operand`
    // digest the main corpus loop already gave `LITERAL_OPERAND`: that call
    // is keyed only by `(form, bound_keys)`, and every other corpus
    // expression that also carries the plain `INT` bound (integer add, mul,
    // lt, le, ge, the three division profiles, and `quantity.pow` -- nine
    // items) hashes to the identical digest, so `CLAIM`/`CLAIM_ALT` -- wired
    // below as dependencies of this one anchor -- were reachable from all of
    // them too, not just from `LITERAL_OPERAND`. `dedicated_operand_tagged`
    // salts this specific anchor so it cannot collide with the shared one.
    // Either way the anchor is a `value`/`literal` node the join constraint
    // does not cover, so both claims stay reachable from `LITERAL_OPERAND`'s
    // own closure: `LITERAL_OPERAND -> dedicated -> CLAIM`.
    let literal_operand_anchor = {
        let bound_key = builder.bound(&INT);
        builder.dedicated_operand_tagged("integer", &[bound_key], "literal-operand-claim-anchor")
    };
    builder.add_dependency(&literal_operand_anchor, code_id(CLAIM).digest.as_ref());
    builder.add_dependency(&literal_operand_anchor, code_id(CLAIM_ALT).digest.as_ref());
    builder
}

pub fn corpus_package() -> PackageBuilder {
    let mut builder = corpus_package_with_expression_operand();
    builder.omit_code(EXPRESSION_OPERAND);
    builder.omit_code(WRONG_OPERAND);
    builder.omit_code(LITERAL_QUANTITY);
    builder
}

/// A reader-negative integer application with an aggregate operand.
pub fn expression_operand_package() -> PackageBuilder {
    corpus_package_with_expression_operand()
}

/// A reader-negative scalar literal whose declared type disagrees with its value kind.
pub fn wrong_operand_package() -> PackageBuilder {
    let mut builder = corpus_package_with_expression_operand();
    builder.omit_code(EXPRESSION_OPERAND);
    builder
}

/// A reader-negative quantity literal paired with a quantity operator.
pub fn literal_quantity_package() -> PackageBuilder {
    let mut builder = corpus_package_with_expression_operand();
    builder.omit_code(EXPRESSION_OPERAND);
    builder.omit_code(WRONG_OPERAND);
    builder
}

/// A reader-negative operation whose rounding disagrees with its decimal type.
pub fn mode_mismatch_package() -> PackageBuilder {
    let mut builder = corpus_package();
    let decimal_bound = builder.bound(&DEC);
    let operand = builder.dedicated_operand("decimal", &[decimal_bound]);
    builder.application_bounded(
        MODE_MISMATCH,
        "expression",
        "binary",
        &key(T_DECIMAL),
        application(
            "binary",
            op_full(
                "quire.op.decimal.add",
                vec![],
                Some(mode_kv("rounding", "toward-zero")),
                None,
            ),
            &key(T_DECIMAL),
            vec![reference(&operand), reference(&operand)],
        ),
        &[DEC],
    );
    builder
}

/// The QSL parameter `x: Int[0, 9]` of [`bounded_increment_package`].
pub const V_PARAM_X: u32 = 111;
/// The second bounded operand for [`BOUNDED_BINARY_SUBTRACTION`].
pub const V_PARAM_SUBTRAHEND: u32 = 121;
/// `x + 1` over `Int[0, 9]`, with `x` typed by a `bounded_domain` and `[0, 9]` the one
/// `integer_range` over Integer.
pub const BOUNDED_INCREMENT: u32 = 2035;
/// `x - 1` over the `Int[0, 9]` domain, with the same single-parameter shape as the increment
/// fixture so native execution can distinguish operand order.
pub const BOUNDED_DECREMENT: u32 = 2036;
/// `x - y` over independently bounded `Int[0, 9]` operands.
pub const BOUNDED_BINARY_SUBTRACTION: u32 = 2037;

/// A package holding only `x + 1` over `Int[0, 9]` in QSL's shape: `x` is typed by an
/// `integer_range` `bounded_domain` over Integer whose members are `min`/`max` bindings.
pub fn bounded_increment_package() -> PackageBuilder {
    let mut builder = corpus_package();
    let bound = Bound::Integer(0, 9);
    let bound_key = builder.bound(&bound);
    builder.code(
        V_PARAM_X,
        "value",
        "literal",
        &bound_key,
        literal("integer", "3"),
    );
    builder.application_bounded(
        BOUNDED_INCREMENT,
        "expression",
        "binary",
        &key(T_INTEGER),
        application(
            "binary",
            op("quire.op.integer.add"),
            &key(T_INTEGER),
            vec![reference(&key(V_PARAM_X)), literal("integer", "1")],
        ),
        &[bound],
    );
    builder
}

/// A package holding `x - 1` over `Int[0, 9]`, parallel to [`bounded_increment_package`].
pub fn bounded_decrement_package() -> PackageBuilder {
    let mut builder = corpus_package();
    let bound = Bound::Integer(0, 9);
    let bound_key = builder.bound(&bound);
    builder.code(
        V_PARAM_X,
        "value",
        "literal",
        &bound_key,
        literal("integer", "3"),
    );
    builder.application_bounded(
        BOUNDED_DECREMENT,
        "expression",
        "binary",
        &key(T_INTEGER),
        application(
            "binary",
            op("quire.op.integer.sub"),
            &key(T_INTEGER),
            vec![reference(&key(V_PARAM_X)), literal("integer", "1")],
        ),
        &[bound],
    );
    builder
}

/// A package holding `x - y` where both symbolic operands independently admit `[0, 9]`.
pub fn bounded_binary_subtraction_package() -> PackageBuilder {
    let mut builder = corpus_package();
    let operand_bound = Bound::Integer(0, 9);
    let operand_bound_key = builder.bound(&operand_bound);
    let minuend_key = key(V_PARAM_X);
    let subtrahend_key = key(V_PARAM_SUBTRAHEND);
    builder.code(
        V_PARAM_X,
        "value",
        "literal",
        &operand_bound_key,
        literal("integer", "3"),
    );
    builder.code(
        V_PARAM_SUBTRAHEND,
        "value",
        "literal",
        &operand_bound_key,
        literal("integer", "1"),
    );
    builder.application_bounded_anchored(
        BOUNDED_BINARY_SUBTRACTION,
        "expression",
        "binary",
        &key(T_INTEGER),
        application(
            "binary",
            op("quire.op.integer.sub"),
            &key(T_INTEGER),
            vec![reference(&minuend_key), reference(&subtrahend_key)],
        ),
        &[Bound::Integer(-9, 9)],
        Some(&minuend_key),
    );
    builder
}

/// First application code in the FR-014-AC-40 source-backed byte-ceiling chain.
pub const BYTE_CHAIN_BASE: u32 = 31_000;
/// The chain's initial bounded literal code.
pub const BYTE_CHAIN_VALUE: u32 = BYTE_CHAIN_BASE - 1;

/// Byte-offset spans of each actual source token, grouped by its fixture node code.
pub type SourceTokenSites = BTreeMap<u32, Vec<(usize, usize)>>;

/// Build a source-backed chain whose additions use each predecessor twice and whose requests
/// each have an actual occurrence in the authored source.
pub fn source_backed_byte_chain(count: u32) -> (PackageBuilder, String, SourceTokenSites) {
    assert!(count > 0);
    let mut builder = PackageBuilder::default();
    let integer_type = key(T_INTEGER);
    builder.code(
        T_INTEGER,
        "scalar_type",
        "integer",
        &integer_type,
        aggregate(),
    );
    let bound = builder.bound(&Bound::Integer(0, 9));
    builder.code(
        BYTE_CHAIN_VALUE,
        "value",
        "literal",
        &bound,
        literal("integer", "3"),
    );

    let mut source = String::new();
    let mut sites = SourceTokenSites::new();
    let mut token = |source: &mut String, code: u32| {
        let start = source.len();
        source.push_str(&format!("n{code}"));
        sites.entry(code).or_default().push((start, source.len()));
    };
    source.push_str("let ");
    token(&mut source, BYTE_CHAIN_VALUE);
    source.push_str(" = 3\n");
    for offset in 0..count {
        let code = BYTE_CHAIN_BASE + offset;
        let predecessor = if offset == 0 {
            BYTE_CHAIN_VALUE
        } else {
            code - 1
        };
        source.push_str("let ");
        token(&mut source, code);
        source.push_str(" = add(");
        token(&mut source, predecessor);
        source.push_str(", ");
        token(&mut source, predecessor);
        source.push_str(")\nrequest ");
        token(&mut source, code);
        source.push('\n');
        let predecessor_key = code_id(predecessor).digest.to_string();
        builder.application_code_with(
            code,
            "expression",
            "binary",
            &integer_type,
            application(
                "binary",
                op("quire.op.integer.add"),
                &integer_type,
                vec![reference(&predecessor_key), reference(&predecessor_key)],
            ),
            &[predecessor_key],
        );
    }

    let source_ref = json!({
        "authority": "agent-ix",
        "identity": "ir764-ac40-source",
        "digest_domain": "quire.source.bytes/v1",
        "digest": format!("{:x}", Sha256::digest(source.as_bytes())),
    });
    builder.value["lock"]["sources"]
        .as_array_mut()
        .expect("sources")
        .push(source_ref.clone());
    let mut entries = Vec::new();
    let mut replaced = BTreeSet::new();
    for (code, spans) in &sites {
        let node_id = code_id(*code);
        replaced.insert(node_id.digest.to_string());
        let node = builder.value["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("nodes")
            .iter_mut()
            .find(|node| node["node_id"]["digest"] == node_id.digest.as_ref())
            .expect("source-backed node");
        if *code == BYTE_CHAIN_VALUE {
            node["owner"] = json!({
                "kind": "source", "authority": source_ref["authority"],
                "identity": source_ref["identity"],
            });
        }
        node["occurrences"] = Value::Array(
            spans
                .iter()
                .enumerate()
                .map(|(ordinal, _)| {
                    json!({
                        "role": if ordinal == 0 { "declaration" } else { "expression" },
                        "ordinal": ordinal,
                    })
                })
                .collect(),
        );
        for (ordinal, (start, end)) in spans.iter().enumerate() {
            entries.push(json!({
                "node_id": node_id,
                "role": if ordinal == 0 { "declaration" } else { "expression" },
                "ordinal": ordinal,
                "regions": [{"source": source_ref, "start": start, "end": end}],
            }));
        }
    }
    let map = builder.value["source_map"]
        .as_array_mut()
        .expect("source map");
    map.retain(|entry| !replaced.contains(entry["node_id"]["digest"].as_str().expect("digest")));
    map.extend(entries);
    (builder, source, sites)
}

/// Two parameters typed by distinct integer bounds, `a: Int[0, 9]` and `b: Int[10, 20]`.
pub const V_PARAM_A: u32 = 112;
pub const V_PARAM_B: u32 = 113;
/// `wide: Int[0, 50]`, outside the result bound `[0, 29]`.
pub const V_PARAM_WIDE: u32 = 114;
/// `c: Int[1, 8]` and `d: Int[11, 19]`, the parameters of [`TWO_PARAMETER_ATTACHED`].
pub const V_PARAM_C: u32 = 115;
pub const V_PARAM_D: u32 = 116;
/// `a + b` with the result typed by the `integer_range` `[0, 29]`.
pub const TWO_PARAMETER_SUM: u32 = 2050;
/// `a + wide` with the result typed `[0, 29]`: an operand bound outside the result domain.
pub const TWO_PARAMETER_WIDE: u32 = 2051;
/// `a + b` with a scalar-typed result and no bound but the two operands': the result role is
/// unresolvable.
pub const TWO_PARAMETER_NO_RESULT: u32 = 2052;
/// `c + d` with a scalar-typed result and `[0, 29]` attached as the only bound that is no
/// operand's own.
pub const TWO_PARAMETER_ATTACHED: u32 = 2053;
/// `a + w` where `w` is typed by a `text_bounds` bound over Integer.
pub const TWO_PARAMETER_WRONG_FORM_OPERAND: u32 = 2054;
/// `a + b` with the result typed by a `text_bounds` bound over Integer.
pub const TWO_PARAMETER_WRONG_FORM_RESULT: u32 = 2055;
/// `w`, the operand of [`TWO_PARAMETER_WRONG_FORM_OPERAND`].
pub const V_PARAM_WRONG_FORM: u32 = 117;
/// `a + 1` with the result typed `[0, 29]`: the literal operand has no bound of its own.
pub const TWO_PARAMETER_LITERAL: u32 = 2056;

/// `p`, the corpus's plain-Integer parameter: it owns no bound. Reuse its node because
/// a second parameter with the same body and type has the same structural key.
pub const V_PARAM_PLAIN: u32 = TEMPORAL_PARAMETER;
/// `e: Int[1, 9]` and `f: Int[100, 200]`.
pub const V_PARAM_E: u32 = 119;
pub const V_PARAM_F: u32 = 120;
/// `a + p` with the result typed `[0, 29]`: `p` has no bound of its own.
pub const TWO_PARAMETER_UNBOUNDED_OPERAND: u32 = 2057;
/// `p + p` with the result typed `[0, 29]`.
pub const TWO_PARAMETER_PLAIN_PAIR: u32 = 2058;
/// `a + p` with a scalar-typed result and nothing attached.
pub const TWO_PARAMETER_PLAIN_SCALAR_RESULT: u32 = 2059;
/// `-e` over `Int[1, 9]` with the result typed `[-9, -1]`.
pub const BOUNDED_NEGATE: u32 = 2060;
/// `e * f` over `Int[1, 9]` and `Int[100, 200]` with the result typed `[100, 1800]`.
pub const BOUNDED_PRODUCT: u32 = 2061;

/// [`corpus_package`] plus integer additions over parameters typed by distinct bounds: the
/// two-parameter shapes of IR-298.
pub fn two_parameter_package() -> PackageBuilder {
    let mut builder = corpus_package();
    let integer_type = key(T_INTEGER);
    let a = builder.bound(&Bound::Integer(0, 9));
    let b = builder.bound(&Bound::Integer(10, 20));
    let wide = builder.bound(&Bound::Integer(0, 50));
    let c = builder.bound(&Bound::Integer(1, 8));
    let d = builder.bound(&Bound::Integer(11, 19));
    let result = builder.bound(&Bound::Integer(0, 29));
    let wrong = builder.bound(&wrong_form_bound());
    let e = builder.bound(&Bound::Integer(1, 9));
    let f = builder.bound(&Bound::Integer(100, 200));
    let negated = builder.bound(&Bound::Integer(-9, -1));
    let product = builder.bound(&Bound::Integer(100, 1800));
    for (code, bound) in [
        (V_PARAM_E, &e),
        (V_PARAM_F, &f),
        (V_PARAM_A, &a),
        (V_PARAM_B, &b),
        (V_PARAM_WIDE, &wide),
        (V_PARAM_C, &c),
        (V_PARAM_D, &d),
    ] {
        builder.code(code, "value", "literal", bound, literal("integer", "3"));
    }
    builder.code(
        V_PARAM_WRONG_FORM,
        "value",
        "literal",
        &wrong,
        literal("integer", "3"),
    );
    let sum = |operands: Vec<Value>, result_type: &str| {
        application("binary", op("quire.op.integer.add"), result_type, operands)
    };
    let param = |code| reference(&key(code));
    for (code, operands, result_type) in [
        (
            TWO_PARAMETER_SUM,
            vec![param(V_PARAM_A), param(V_PARAM_B)],
            &result,
        ),
        (
            TWO_PARAMETER_WIDE,
            vec![param(V_PARAM_A), param(V_PARAM_WIDE)],
            &result,
        ),
        (
            TWO_PARAMETER_NO_RESULT,
            vec![param(V_PARAM_A), param(V_PARAM_B)],
            &integer_type,
        ),
        (
            TWO_PARAMETER_WRONG_FORM_OPERAND,
            vec![param(V_PARAM_A), param(V_PARAM_WRONG_FORM)],
            &result,
        ),
        (
            TWO_PARAMETER_WRONG_FORM_RESULT,
            vec![param(V_PARAM_A), param(V_PARAM_B)],
            &wrong,
        ),
        (
            TWO_PARAMETER_LITERAL,
            vec![param(V_PARAM_A), literal("integer", "1")],
            &result,
        ),
        (
            TWO_PARAMETER_UNBOUNDED_OPERAND,
            vec![param(V_PARAM_A), param(V_PARAM_PLAIN)],
            &result,
        ),
        (
            TWO_PARAMETER_PLAIN_PAIR,
            vec![param(V_PARAM_PLAIN), param(V_PARAM_PLAIN)],
            &result,
        ),
        (
            TWO_PARAMETER_PLAIN_SCALAR_RESULT,
            vec![param(V_PARAM_A), param(V_PARAM_PLAIN)],
            &integer_type,
        ),
        (
            BOUNDED_PRODUCT,
            vec![param(V_PARAM_E), param(V_PARAM_F)],
            &product,
        ),
    ] {
        // No bound is attached: each node reaches what its operands and its result type name.
        builder.application_bounded(
            code,
            "expression",
            "binary",
            result_type,
            match code {
                BOUNDED_PRODUCT => {
                    application("binary", op("quire.op.integer.mul"), result_type, operands)
                }
                _ => sum(operands, result_type),
            },
            &[],
        );
    }
    builder.application_bounded(
        BOUNDED_NEGATE,
        "expression",
        "unary",
        &negated,
        application(
            "unary",
            op("quire.op.integer.negate"),
            &negated,
            vec![param(V_PARAM_E)],
        ),
        &[],
    );
    // `[0, 29]` reaches this node through `c`, so it is the one reachable bound no operand types.
    builder.application_bounded_anchored(
        TWO_PARAMETER_ATTACHED,
        "expression",
        "binary",
        &integer_type,
        sum(vec![param(V_PARAM_C), param(V_PARAM_D)], &integer_type),
        &[Bound::Integer(0, 29)],
        Some(&key(V_PARAM_C)),
    );
    builder
}

/// The body QSL emits for a parameter's `value` node (form `parameter`): an `aggregate` of its
/// `name` (a `text` literal) and its `level` (an `integer` literal) as `binding` members. It is
/// not a literal, so a reference to it is an operand, never a constant.
pub fn parameter_body(name: &str, level: u32) -> Value {
    json!({
        "term": "aggregate",
        "members": [
            member("name", literal("text", name)),
            member("level", literal("integer", &level.to_string())),
        ],
    })
}

/// QSL's parameter `x: Int[0, 9]`, typed by an `integer_range` `bounded_domain`.
pub const QSL_PARAM_X: u32 = 130;
/// QSL's parameter `y: Int[10, 20]`.
pub const QSL_PARAM_Y: u32 = 131;
/// QSL's parameter `z: Int[0, 9]`: the same bound as `x` (bounds are content-addressed).
pub const QSL_PARAM_Z: u32 = 132;
/// A parameter `p` typed by the plain Integer scalar type.
pub const QSL_PARAM_PLAIN: u32 = 133;
/// `q`: a `value` node of form `literal` whose body is not a literal, typed by the plain Integer
/// scalar type and depending on the `Int[0, 29]` bound (the reader refuses an extra dependency on
/// a `parameter`-form node, so `q` stands in for a parameter whose closure reaches a bound).
pub const QSL_PARAM_REACHING: u32 = 139;
/// Parameters `u`, `v`, `w: Int[10, 20]`: an expression node's id is the digest of its content,
/// so each further `x + y`-shaped node sums `x` with its own parameter to stay a distinct node.
pub const QSL_PARAM_U: u32 = 136;
pub const QSL_PARAM_V: u32 = 137;
pub const QSL_PARAM_W: u32 = 138;
/// The `value` node QSL emits for the literal `1`: form `literal`, a `literal` body, typed by the
/// plain Integer scalar type.
pub const QSL_LITERAL_ONE: u32 = 134;
/// A `value` node of form `literal` typed by the plain Integer scalar type whose body is not a
/// literal: a reference to it is not a constant.
pub const QSL_NOT_LITERAL: u32 = 135;
/// `x + 1` (`function inc(x: Int[0, 9]): Int[0, 10] { x + 1 }`): the literal is a reference to
/// [`QSL_LITERAL_ONE`] and the node's own result type is the plain Integer type.
pub const QSL_INC: u32 = 2070;
/// The narrowing conversion of [`QSL_INC`] to `Int[0, 10]`.
pub const QSL_INC_NARROW: u32 = 2071;
/// `x + y` (`function add(x: Int[0, 9], y: Int[10, 20]): Int[10, 29] { x + y }`).
pub const QSL_ADD: u32 = 2072;
/// The narrowing conversion of [`QSL_ADD`] to `Int[10, 29]`.
pub const QSL_ADD_NARROW: u32 = 2073;
/// `-z` (`function negate(z: Int[0, 9]): Int[-9, 0] { -z }`).
pub const QSL_NEGATE: u32 = 2074;
/// The narrowing conversion of [`QSL_NEGATE`] to `Int[-9, 0]`.
pub const QSL_NEGATE_NARROW: u32 = 2075;
/// `x + p` narrowed to `Int[0, 29]`: `p` is a plain-Integer parameter.
pub const QSL_PLAIN_OPERAND: u32 = 2076;
/// The narrowing conversion of [`QSL_PLAIN_OPERAND`].
pub const QSL_PLAIN_OPERAND_NARROW: u32 = 2077;
/// `q + q` narrowed to `Int[0, 29]`: no operand owns a bound, and the narrowing bounds the result.
/// `q`'s node reaches `[0, 29]` so that IR's lowering finds a bound in the closure and it is CG's
/// rule, not the lowering, that refuses the plain operands (see [`QSL_PARAM_REACHING`]).
pub const QSL_PLAIN_PAIR: u32 = 2087;
/// The narrowing conversion of [`QSL_PLAIN_PAIR`].
pub const QSL_PLAIN_PAIR_NARROW: u32 = 2088;
/// `x + n` narrowed to `Int[0, 29]`, where `n` is [`QSL_NOT_LITERAL`].
pub const QSL_NOT_LITERAL_OPERAND: u32 = 2078;
/// The narrowing conversion of [`QSL_NOT_LITERAL_OPERAND`].
pub const QSL_NOT_LITERAL_OPERAND_NARROW: u32 = 2079;
/// `x + u` consumed by two narrowing conversions, to `Int[10, 29]` and to `Int[0, 40]`.
pub const QSL_TWICE_NARROWED: u32 = 2080;
/// The narrowing conversion of [`QSL_TWICE_NARROWED`] to `Int[10, 29]`.
pub const QSL_TWICE_NARROWED_FIRST: u32 = 2081;
/// The narrowing conversion of [`QSL_TWICE_NARROWED`] to `Int[0, 40]`.
pub const QSL_TWICE_NARROWED_SECOND: u32 = 2082;
/// `x + v` narrowed to a `text_bounds` bound over Integer.
pub const QSL_WRONG_FORM_NARROWED: u32 = 2083;
/// The narrowing conversion of [`QSL_WRONG_FORM_NARROWED`].
pub const QSL_WRONG_FORM_NARROW: u32 = 2084;
/// `x + w` narrowed to an `integer_range` over Rational rather than Integer.
pub const QSL_WRONG_BASE_NARROWED: u32 = 2085;
/// The narrowing conversion of [`QSL_WRONG_BASE_NARROWED`].
pub const QSL_WRONG_BASE_NARROW: u32 = 2086;
/// `value` nodes of form `literal` typed by the plain Integer type, in QSL's shape for a literal
/// operand: the integer literals `100`, `10^23` and `2`, and the text literal `"a"`.
pub const QSL_LITERAL_HUNDRED: u32 = 140;
pub const QSL_LITERAL_HUGE: u32 = 141;
pub const QSL_LITERAL_TEXT: u32 = 142;
pub const QSL_LITERAL_TWO: u32 = 143;
/// `x + 100` narrowed to `Int[0, 10]`: no `x` in `[0, 9]` gives a result inside the bound.
pub const QSL_PLUS_HUNDRED: u32 = 2089;
pub const QSL_PLUS_HUNDRED_NARROW: u32 = 2090;
/// `x + 10^23` narrowed to `Int[0, 10]`: the literal does not fit `i64`.
pub const QSL_PLUS_HUGE: u32 = 2091;
pub const QSL_PLUS_HUGE_NARROW: u32 = 2092;
/// `x + "a"` narrowed to `Int[0, 10]`, with the text literal in an Integer-typed `value` node.
pub const QSL_PLUS_TEXT: u32 = 2093;
pub const QSL_PLUS_TEXT_NARROW: u32 = 2094;
/// `x + 2`, narrowed to `Int[0, 11]` and also an operand of [`QSL_SHARED_OUTER`].
pub const QSL_SHARED: u32 = 2095;
pub const QSL_SHARED_NARROW: u32 = 2096;
/// `(x + 2) + x`, plain-Integer-typed and not narrowed: a consumer of [`QSL_SHARED`] that is not a
/// narrowing conversion.
pub const QSL_SHARED_OUTER: u32 = 2097;

/// An `integer_range` over Rational: the right form over another base type.
pub fn wrong_base_bound() -> Bound {
    Bound::Raw {
        form: "integer_range",
        bounded: "rational",
        body: Bound::Integer(10, 29).body(),
        foreign: vec![INT],
    }
}

/// [`corpus_package`] plus integer nodes in the shape QSL emits into a checked package for
/// `function inc using v(x: Int[0, 9]): Int[0, 10] pure { x + 1 }` and its siblings `add` and
/// `negate` (read from QSL's emitted package, not copied from it):
///
/// - a parameter is a `value` node of form `parameter` whose body is [`parameter_body`] and whose
///   type is its `bounded_domain`;
/// - a literal operand is a `reference` to its own `value` node of form `literal`, whose body is
///   the literal and whose type is the plain Integer scalar type;
/// - the arithmetic node's own result type is the plain Integer scalar type, and the declared
///   bounded result is the type of a `conversion` node that consumes it: a `convert`
///   application with the identity `quire.op.numeric.narrow`, its `type_argument` member naming
///   the bound, and one argument, a reference to the arithmetic node.
pub fn qsl_shaped_package() -> PackageBuilder {
    let mut builder = corpus_package();
    let integer_type = key(T_INTEGER);
    let x_bound = builder.bound(&Bound::Integer(0, 9));
    let y_bound = builder.bound(&Bound::Integer(10, 20));
    for (code, name, level, typing) in [
        (QSL_PARAM_X, "x", 0, &x_bound),
        (QSL_PARAM_Y, "y", 1, &y_bound),
        (QSL_PARAM_Z, "z", 0, &x_bound),
        (QSL_PARAM_PLAIN, "p", 1, &integer_type),
        (QSL_PARAM_U, "u", 1, &y_bound),
        (QSL_PARAM_V, "v", 1, &y_bound),
        (QSL_PARAM_W, "w", 1, &y_bound),
    ] {
        builder.code(
            code,
            "value",
            "parameter",
            typing,
            parameter_body(name, level),
        );
    }
    let reached = builder.bound(&Bound::Integer(0, 29));
    builder.code(
        QSL_PARAM_REACHING,
        "value",
        "literal",
        &integer_type,
        parameter_body("q", 0),
    );
    builder.add_dependency(&key(QSL_PARAM_REACHING), &reached);
    builder.code(
        QSL_LITERAL_ONE,
        "value",
        "literal",
        &integer_type,
        literal("integer", "1"),
    );
    builder.code(
        QSL_NOT_LITERAL,
        "value",
        "literal",
        &integer_type,
        parameter_body("n", 1),
    );
    for (code, value) in [
        (QSL_LITERAL_HUNDRED, literal("integer", "100")),
        (
            QSL_LITERAL_HUGE,
            literal("integer", "100000000000000000000000"),
        ),
        (QSL_LITERAL_TEXT, literal("text", "a")),
        (QSL_LITERAL_TWO, literal("integer", "2")),
    ] {
        builder.code(code, "value", "literal", &integer_type, value);
    }
    let param = |code| reference(&key(code));
    let sum = |left: u32, right: u32| {
        application(
            "binary",
            op("quire.op.integer.add"),
            &integer_type,
            vec![param(left), param(right)],
        )
    };
    for (code, body, narrowed) in [
        (
            QSL_INC,
            sum(QSL_PARAM_X, QSL_LITERAL_ONE),
            vec![(QSL_INC_NARROW, Bound::Integer(0, 10))],
        ),
        (
            QSL_ADD,
            sum(QSL_PARAM_X, QSL_PARAM_Y),
            vec![(QSL_ADD_NARROW, Bound::Integer(10, 29))],
        ),
        (
            QSL_NEGATE,
            application(
                "unary",
                op("quire.op.integer.negate"),
                &integer_type,
                vec![param(QSL_PARAM_Z)],
            ),
            vec![(QSL_NEGATE_NARROW, Bound::Integer(-9, 0))],
        ),
        (
            QSL_PLAIN_OPERAND,
            sum(QSL_PARAM_X, QSL_PARAM_PLAIN),
            vec![(QSL_PLAIN_OPERAND_NARROW, Bound::Integer(0, 29))],
        ),
        (
            QSL_PLAIN_PAIR,
            sum(QSL_PARAM_REACHING, QSL_PARAM_REACHING),
            vec![(QSL_PLAIN_PAIR_NARROW, Bound::Integer(0, 29))],
        ),
        (
            QSL_NOT_LITERAL_OPERAND,
            sum(QSL_PARAM_X, QSL_NOT_LITERAL),
            vec![(QSL_NOT_LITERAL_OPERAND_NARROW, Bound::Integer(0, 29))],
        ),
        (
            QSL_TWICE_NARROWED,
            sum(QSL_PARAM_X, QSL_PARAM_U),
            vec![
                (QSL_TWICE_NARROWED_FIRST, Bound::Integer(10, 29)),
                (QSL_TWICE_NARROWED_SECOND, Bound::Integer(0, 40)),
            ],
        ),
        (
            QSL_WRONG_FORM_NARROWED,
            sum(QSL_PARAM_X, QSL_PARAM_V),
            vec![(QSL_WRONG_FORM_NARROW, wrong_form_bound())],
        ),
        (
            QSL_PLUS_HUNDRED,
            sum(QSL_PARAM_X, QSL_LITERAL_HUNDRED),
            vec![(QSL_PLUS_HUNDRED_NARROW, Bound::Integer(0, 10))],
        ),
        (
            QSL_PLUS_HUGE,
            sum(QSL_PARAM_X, QSL_LITERAL_HUGE),
            vec![(QSL_PLUS_HUGE_NARROW, Bound::Integer(0, 10))],
        ),
        (
            QSL_PLUS_TEXT,
            sum(QSL_PARAM_X, QSL_LITERAL_TEXT),
            vec![(QSL_PLUS_TEXT_NARROW, Bound::Integer(0, 10))],
        ),
        (
            QSL_SHARED,
            sum(QSL_PARAM_X, QSL_LITERAL_TWO),
            vec![(QSL_SHARED_NARROW, Bound::Integer(0, 11))],
        ),
    ] {
        let form = body["operator"].as_str().expect("operator").to_owned();
        builder.application_bounded(code, "expression", &form, &integer_type, body, &[]);
        let operand = code_id(code).digest.to_string();
        for (narrow, bound) in narrowed {
            let target = builder.bound(&bound);
            builder.narrow(narrow, &operand, &target);
        }
    }
    let shared = code_id(QSL_SHARED).digest.to_string();
    builder.application_bounded(
        QSL_SHARED_OUTER,
        "expression",
        "binary",
        &integer_type,
        application(
            "binary",
            op("quire.op.integer.add"),
            &integer_type,
            vec![reference(&shared), reference(&key(QSL_PARAM_X))],
        ),
        &[],
    );
    builder
}

/// Keep the malformed Integer-range-over-Rational narrowing outside the admitted corpus.
/// The base wire is resolved first, so its deliberate invalid shape is the first reader refusal.
pub fn qsl_wrong_base_package() -> PackageBuilder {
    let rekey::ResolvedWire { wire, ids, outcome } =
        qsl_shaped_package().resolved_wire(CheckedPackageReadLimits::bounded());
    assert!(matches!(outcome, CheckedPackageV2ReadResult::Admitted(_)));
    let mut builder = PackageBuilder {
        value: wire,
        bounds: BTreeSet::new(),
        dedicated_operands: BTreeSet::new(),
    };
    let integer = ids.resolve(&code_id(T_INTEGER)).digest.to_string();
    let rational = ids.resolve(&code_id(T_RATIONAL)).digest.to_string();
    let operand_x = ids.resolve(&code_id(QSL_PARAM_X)).digest.to_string();
    let operand_w = ids.resolve(&code_id(QSL_PARAM_W)).digest.to_string();
    builder.application_bounded(
        QSL_WRONG_BASE_NARROWED,
        "expression",
        "binary",
        &integer,
        application(
            "binary",
            op("quire.op.integer.add"),
            &integer,
            vec![reference(&operand_x), reference(&operand_w)],
        ),
        &[],
    );
    let bound = wrong_base_bound();
    let bound_digest = bound.key();
    let mut body = bound.body();
    ids.resolve_value(&mut body);
    let foreign = vec![ids.resolve(&id(&INT.key())).digest.to_string()];
    builder.node_with(
        &bound_digest,
        "bounded_domain",
        bound.form(),
        &rational,
        body,
        &foreign,
    );
    builder.narrow(
        QSL_WRONG_BASE_NARROW,
        code_id(QSL_WRONG_BASE_NARROWED).digest.as_ref(),
        &bound_digest,
    );
    builder
}

impl PackageBuilder {
    /// A narrowing `conversion` of the node `operand` to the `bounded_domain` `target`, typed by
    /// `target`, in QSL's shape.
    pub fn narrow(&mut self, code: u32, operand: &str, target: &str) -> &mut Self {
        let body = application(
            "convert",
            op_full(
                "quire.op.numeric.narrow",
                Vec::new(),
                None,
                Some(type_argument_member(target)),
            ),
            target,
            vec![reference(operand)],
        );
        self.application_code_with(
            code,
            "expression",
            "conversion",
            target,
            body,
            &[operand.to_owned(), target.to_owned()],
        )
    }
}

/// `x rem 2` over `Int[-5, 5]`: `integer.rem` has no derivable descriptor.
pub const DERIVE_REM: u32 = 2040;
/// `x == 2` over integers.
pub const DERIVE_INTEGER_EQ: u32 = 2041;
/// Checked Boolean connective and equality applications.
pub const DERIVE_BOOLEAN_AND: u32 = 2300;
pub const DERIVE_BOOLEAN_OR: u32 = 2301;
pub const DERIVE_BOOLEAN_NOT: u32 = 2302;
pub const DERIVE_BOOLEAN_IMPLIES: u32 = 2303;
pub const DERIVE_BOOLEAN_EQ: u32 = 2304;
pub const DERIVE_BOOLEAN_NE: u32 = 2305;
pub const DERIVE_INTEGER_NE: u32 = 2306;
pub const DERIVE_BOOLEAN_SHORT_CIRCUIT: u32 = 2307;
pub const DERIVE_INTEGER_GT: u32 = 2308;
pub const DERIVE_BOOLEAN_LAZY_AND: u32 = 2309;
pub const DERIVE_BOOLEAN_LAZY_OR: u32 = 2310;
/// `rational.div` over one integer and one rational operand: neither overload applies.
pub const DERIVE_MIXED_DIV: u32 = 2042;
/// `numeric.convert` to an integer result: only a text result is derivable.
pub const DERIVE_CONVERT_TO_INTEGER: u32 = 2043;

/// A pure function whose body is a quantity conversion node.
pub const CONVERSION_FUNCTION: u32 = 2150;
/// A quantity equality whose left operand references a quantity conversion node.
pub const EQUALITY_OVER_CONVERSION: u32 = 2151;

/// A pure function named [`ADDER_NAME`] whose body is the integer addition node 1001.
pub const ADDER_FUNCTION: u32 = 2152;
/// A call of [`ADDER_FUNCTION`] whose first argument is a quantity conversion node.
pub const CALL_OVER_CONVERSION: u32 = 2153;
/// The declared name of [`ADDER_FUNCTION`].
/// A pure function whose body is a direct reference to the integer addition node 1001.
pub const REFERENCE_FUNCTION: u32 = 2156;
/// A call of [`REFERENCE_FUNCTION`] whose first argument is a quantity conversion node.
pub const CALL_REFERENCE_OVER_CONVERSION: u32 = 2157;
/// A pure function named [`CONVERTING_NAME`] whose body is the 1086 quantity conversion node.
pub const CONVERTING_FUNCTION: u32 = 2158;
/// A call of [`CONVERTING_FUNCTION`] with no arguments.
pub const CALL_CONVERTING_FUNCTION: u32 = 2159;
/// The declared name of [`CONVERTING_FUNCTION`].
pub const CONVERTING_NAME: &str = "nconverting";
/// The adder's parameters.
pub const ADDER_LEFT: u32 = 2154;
/// The adder's parameters.
pub const ADDER_RIGHT: u32 = 2155;
/// The declared name of [`ADDER_FUNCTION`].
pub const ADDER_NAME: &str = "nadder";

/// [`corpus_package`] plus a function declared over a quantity conversion node and a quantity
/// equality that reads a conversion node as an operand.
pub fn conversion_operand_package() -> PackageBuilder {
    let mut builder = corpus_package();
    let conversion = code_id(QUANTITY_CONVERSIONS[0]).digest.to_string();
    builder.application_bounded(
        EQUALITY_OVER_CONVERSION,
        "expression",
        "binary",
        &key(T_BOOLEAN),
        application(
            "binary",
            op("quire.op.quantity.eq"),
            &key(T_BOOLEAN),
            vec![reference(&conversion), reference(&key(V_QUANTITY))],
        ),
        &[],
    );
    register_code(CONVERSION_FUNCTION, key(CONVERSION_FUNCTION));
    builder.node_with(
        &key(CONVERSION_FUNCTION),
        "function",
        "pure_function",
        &unit_type(),
        json!({"term": "reference", "target": node_ref(&conversion)}),
        std::slice::from_ref(&conversion),
    );
    let addition = code_id(1001).digest.to_string();
    let left = key(ADDER_LEFT);
    let right = key(ADDER_RIGHT);
    builder
        .code(
            ADDER_LEFT,
            "value",
            "parameter",
            &unit_type(),
            parameter_body("a", 0),
        )
        .code(
            ADDER_RIGHT,
            "value",
            "parameter",
            &key(T_INTEGER),
            parameter_body("b", 0),
        );
    register_code(ADDER_FUNCTION, key(ADDER_FUNCTION));
    builder.node_with_label(
        &key(ADDER_FUNCTION),
        "adder",
        "function",
        "pure_function",
        &key(T_INTEGER),
        json!({"term": "aggregate", "members": [
            member(
                "parameters",
                json!({"term": "aggregate", "members": [reference(&left), reference(&right)]}),
            ),
            member("body", reference(&addition)),
        ]}),
        &[addition.clone(), left, right],
    );
    let function = key(ADDER_FUNCTION);
    let integer = key(V_INTEGER);
    builder.application_code_with(
        CALL_OVER_CONVERSION,
        "expression",
        "call",
        &key(T_INTEGER),
        application(
            "call",
            op("quire.op.function.call"),
            &key(T_INTEGER),
            vec![
                reference(&function),
                reference(&conversion),
                reference(&integer),
            ],
        ),
        &[function, conversion.clone(), integer.clone()],
    );
    register_code(CONVERTING_FUNCTION, key(CONVERTING_FUNCTION));
    builder.node_with_label(
        &key(CONVERTING_FUNCTION),
        "converting",
        "function",
        "pure_function",
        &unit_type(),
        json!({"term": "aggregate", "members": [
            member("parameters", json!({"term": "aggregate", "members": []})),
            member("body", reference(&conversion)),
        ]}),
        std::slice::from_ref(&conversion),
    );
    let converting = key(CONVERTING_FUNCTION);
    builder.application_code_with(
        CALL_CONVERTING_FUNCTION,
        "expression",
        "call",
        &unit_type(),
        application(
            "call",
            op("quire.op.function.call"),
            &unit_type(),
            vec![reference(&converting)],
        ),
        std::slice::from_ref(&converting),
    );
    register_code(REFERENCE_FUNCTION, key(REFERENCE_FUNCTION));
    builder.node_with(
        &key(REFERENCE_FUNCTION),
        "function",
        "pure_function",
        &key(T_INTEGER),
        json!({"term": "reference", "target": node_ref(&addition)}),
        std::slice::from_ref(&addition),
    );
    let reference_function = key(REFERENCE_FUNCTION);
    builder.application_code_with(
        CALL_REFERENCE_OVER_CONVERSION,
        "expression",
        "call",
        &key(T_INTEGER),
        application(
            "call",
            op("quire.op.function.call"),
            &key(T_INTEGER),
            vec![
                reference(&reference_function),
                reference(&conversion),
                reference(&integer),
            ],
        ),
        &[reference_function, conversion, integer],
    );
    builder
}

/// [`corpus_package`] plus the nodes IR-294's derivation refuses: `integer.rem`, `integer.eq`,
/// a mixed-operand `rational.div` and a `numeric.convert` to an integer.
pub fn derivation_package() -> PackageBuilder {
    let mut builder = corpus_package();
    let int5 = builder.bound(&INT5);
    let int = builder.bound(&INT);
    let rat = builder.bound(&RAT);
    let rem_operand =
        builder.dedicated_operand_tagged("integer", std::slice::from_ref(&int5), "derive-rem");
    builder.application_bounded(
        DERIVE_REM,
        "expression",
        "binary",
        &key(T_INTEGER),
        application(
            "binary",
            op_full(
                "quire.op.integer.rem",
                vec![law(
                    "integer_division",
                    integer_division_definition(DivisionProfile::Truncating),
                )],
                None,
                None,
            ),
            &key(T_INTEGER),
            vec![reference(&rem_operand), literal("integer", "2")],
        ),
        &[INT5],
    );
    let eq_operand =
        builder.dedicated_operand_tagged("integer", std::slice::from_ref(&int), "derive-eq");
    builder.application_bounded(
        DERIVE_INTEGER_EQ,
        "expression",
        "binary",
        &key(T_BOOLEAN),
        application(
            "binary",
            op("quire.op.integer.eq"),
            &key(T_BOOLEAN),
            vec![reference(&eq_operand), literal("integer", "2")],
        ),
        &[INT],
    );
    builder.application_bounded(
        DERIVE_INTEGER_NE,
        "expression",
        "binary",
        &key(T_BOOLEAN),
        application(
            "binary",
            op("quire.op.integer.ne"),
            &key(T_BOOLEAN),
            vec![reference(&eq_operand), literal("integer", "2")],
        ),
        &[INT],
    );
    builder.application_bounded(
        DERIVE_INTEGER_GT,
        "expression",
        "binary",
        &key(T_BOOLEAN),
        application(
            "binary",
            op("quire.op.integer.gt"),
            &key(T_BOOLEAN),
            vec![reference(&eq_operand), literal("integer", "2")],
        ),
        &[INT],
    );
    for (code, identity, operator, operands) in [
        (
            DERIVE_BOOLEAN_AND,
            "quire.op.boolean.and",
            "binary",
            vec![literal("boolean", "true"), literal("boolean", "false")],
        ),
        (
            DERIVE_BOOLEAN_OR,
            "quire.op.boolean.or",
            "binary",
            vec![literal("boolean", "true"), literal("boolean", "false")],
        ),
        (
            DERIVE_BOOLEAN_NOT,
            "quire.op.boolean.not",
            "unary",
            vec![literal("boolean", "true")],
        ),
        (
            DERIVE_BOOLEAN_IMPLIES,
            "quire.op.boolean.implies",
            "binary",
            vec![literal("boolean", "true"), literal("boolean", "false")],
        ),
        (
            DERIVE_BOOLEAN_EQ,
            "quire.op.boolean.eq",
            "binary",
            vec![literal("boolean", "true"), literal("boolean", "false")],
        ),
        (
            DERIVE_BOOLEAN_NE,
            "quire.op.boolean.ne",
            "binary",
            vec![literal("boolean", "true"), literal("boolean", "false")],
        ),
    ] {
        builder.application_bounded(
            code,
            "expression",
            operator,
            &key(T_BOOLEAN),
            application(operator, op(identity), &key(T_BOOLEAN), operands),
            &[],
        );
    }
    for (code, identity) in [
        (DERIVE_BOOLEAN_SHORT_CIRCUIT, "quire.op.boolean.implies"),
        (DERIVE_BOOLEAN_LAZY_AND, "quire.op.boolean.and"),
        (DERIVE_BOOLEAN_LAZY_OR, "quire.op.boolean.or"),
    ] {
        builder.application_bounded(
            code,
            "expression",
            "binary",
            &key(T_BOOLEAN),
            application(
                "binary",
                op(identity),
                &key(T_BOOLEAN),
                vec![
                    literal("boolean", "false"),
                    reference(code_id(DERIVE_BOOLEAN_NOT).digest.as_ref()),
                ],
            ),
            &[],
        );
    }
    let mixed_keys = [int.clone(), rat];
    let integer_operand = builder.dedicated_operand_tagged("integer", &mixed_keys, "derive-mixed");
    let rational_operand =
        builder.dedicated_operand_tagged("rational", &mixed_keys, "derive-mixed");
    builder.application_bounded(
        DERIVE_MIXED_DIV,
        "expression",
        "binary",
        &key(T_RATIONAL),
        application(
            "binary",
            op("quire.op.rational.div"),
            &key(T_RATIONAL),
            vec![reference(&integer_operand), reference(&rational_operand)],
        ),
        &[INT, RAT],
    );
    let convert_operand = builder.dedicated_operand_tagged("integer", &[int], "derive-convert");
    builder.application_bounded(
        DERIVE_CONVERT_TO_INTEGER,
        "expression",
        "conversion",
        &key(T_INTEGER),
        application(
            "convert",
            op_full(
                "quire.op.numeric.convert",
                vec![],
                None,
                Some(type_argument_member(&key(T_INTEGER))),
            ),
            &key(T_INTEGER),
            vec![reference(&convert_operand)],
        ),
        &[INT],
    );
    builder
}

/// Integer addition over the `Int[0, 9]` domain of [`bounded_increment_package`].
pub fn integer_add_0_9() -> ExactScalarOperation {
    ExactScalarOperation::IntegerArithmetic {
        operator: IntegerOperator::Add,
        domain: bounded(0, 9),
    }
}

pub fn integer_add() -> ExactScalarOperation {
    ExactScalarOperation::IntegerArithmetic {
        operator: IntegerOperator::Add,
        domain: bounded(-1000, 1000),
    }
}

/// Refused items of the golden request, each with an integer-add descriptor
/// unless the refusal is about the descriptor.
pub fn refused_items() -> Vec<ExactScalarItem> {
    let item = |code, operation| ExactScalarItem {
        node_id: code_id(code),
        operation,
    };
    let mut items: Vec<ExactScalarItem> = [
        COMPOSITE,
        FUNCTION,
        MODEL,
        RELATION,
        STATE,
        TEMPORAL,
        PROTOCOL,
        CALLS_FUNCTION,
        WRONG_BODY,
        MISSING,
        V_BOOLEAN,
        DUPLICATED,
        DUPLICATED,
        UNBOUNDED,
        AMBIGUOUS,
        WRONG_BOUND_FORM,
        UNTYPED_OPERAND,
    ]
    .into_iter()
    .map(|code| item(code, integer_add()))
    .collect();
    items.push(item(
        WRONG_RESULT,
        ExactScalarOperation::DecimalArithmetic {
            operator: DecimalOperator::Add,
            target: decimal_type(-1, 1, 0, 0, RoundingMode::NearestEven),
        },
    ));
    items.push(item(
        WRONG_ARITY,
        ExactScalarOperation::IntegerArithmetic {
            operator: IntegerOperator::Negate,
            domain: bounded(-1000, 1000),
        },
    ));
    items.push(item(
        MATHEMATICAL,
        ExactScalarOperation::IntegerArithmetic {
            operator: IntegerOperator::Add,
            domain: IntegerDomain::Mathematical,
        },
    ));
    items.push(item(
        DOMAIN_MISMATCH,
        ExactScalarOperation::IntegerDivision {
            profile: DivisionProfile::Floor,
            domain: bounded(-1000, 1000),
        },
    ));
    items.push(item(
        WRONG_OPERAND_REFERENCE,
        ExactScalarOperation::RationalArithmetic {
            operator: RationalOperator::Divide,
            domain: None,
        },
    ));
    items
}

/// The corpus request: every corpus expression plus every refused item.
pub fn golden_items() -> Vec<ExactScalarItem> {
    corpus()
        .into_iter()
        .map(|expression| ExactScalarItem {
            node_id: code_id(expression.code),
            operation: expression.operation,
        })
        .chain(refused_items())
        .collect()
}

#[cfg(test)]
mod bound_tests {
    use super::{aggregate, key, node_ref, Bound, PackageBuilder, DEC, INT, TEXT, T_INTEGER};
    use ix_trace_rs::trace;
    use quire_contract_model::CheckedPackageV2ReadResult;
    use serde_json::json;

    fn keys(bounds: &[Bound]) -> Vec<String> {
        bounds.iter().map(Bound::key).collect()
    }

    #[trace("FR-014-AC-10", "TC-024")]
    #[test]
    fn anonymous_integer_authority_admits_one_integer_range() {
        let mut builder = PackageBuilder::default();
        builder.code(
            T_INTEGER,
            "scalar_type",
            "integer",
            &key(T_INTEGER),
            aggregate(),
        );
        let anonymous = key(T_INTEGER);
        let range_source = builder.bound(&Bound::Integer(0, 9));

        let (outcome, _, ids) = builder.read_resolved();
        let CheckedPackageV2ReadResult::Admitted(package) = outcome else {
            panic!("one range over anonymous Integer must admit: {outcome:?}");
        };
        let anonymous = ids.resolve(&super::id(&anonymous));
        let integer_node = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == anonymous)
            .expect("admitted structural Integer");
        assert!(integer_node.declaration.is_none());
        let range = ids.resolve(&super::id(&range_source));
        let checked_range = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == range)
            .expect("admitted integer range");
        assert_eq!(checked_range.semantic_type, anonymous);
        for member in checked_range.body["members"]
            .as_array()
            .expect("admitted bounds")
        {
            assert_eq!(member["value"]["type"], node_ref(&anonymous.digest));
        }
    }

    /// Pins `Bound::foreign`'s derivation against the exact per-variant lists this crate
    /// hand-maintained before the PR #101 review finding that `foreign()` mirrored `body()`'s
    /// literal kinds by hand, in a second place that could silently drift from the first.
    /// Derivation must reproduce every one of the historically correct lists.
    #[test]
    fn foreign_matches_the_literal_kinds_body_actually_embeds() {
        assert_eq!(keys(&Bound::Integer(0, 1).foreign()), Vec::<String>::new());
        assert_eq!(keys(&Bound::Rational(0, 1, 1, 2).foreign()), keys(&[INT]));
        assert_eq!(
            keys(&Bound::Decimal(0, 1, 0, 2, "nearest-even").foreign()),
            keys(&[INT, TEXT])
        );
        assert_eq!(
            keys(&Bound::Rounding("float32", "nearest-even").foreign()),
            keys(&[TEXT])
        );
        assert_eq!(keys(&Bound::Text(0, 4, "nfc").foreign()), keys(&[INT]));
        // `Decimal`'s digest domain never appears in `Rational`'s foreign list, and vice
        // versa: derivation is exact per literal kind, not "any numeric foreign".
        assert_ne!(keys(&Bound::Rational(0, 1, 1, 2).foreign()), keys(&[DEC]));
    }

    /// Regression for the PR #101 review finding that `Bound::key()` omitted `foreign`, so two
    /// `Raw` bounds with identical form/bounded/body but different `foreign` collided in
    /// `PackageBuilder::bound`'s digest-keyed dedupe and one silently lost its own foreign
    /// dependency.
    #[test]
    fn key_differs_when_only_foreign_dependencies_differ() {
        let body = json!({"term": "aggregate", "members": []});
        let no_foreign = Bound::Raw {
            form: "text_bounds",
            bounded: "text",
            body: body.clone(),
            foreign: vec![],
        };
        let with_foreign = Bound::Raw {
            form: "text_bounds",
            bounded: "text",
            body,
            foreign: vec![INT],
        };
        assert_ne!(
            no_foreign.key(),
            with_foreign.key(),
            "two Raw bounds with identical form/bounded/body but different foreign \
             dependencies must not collide in the dedupe keyed by this digest"
        );
    }
}

#[cfg(test)]
mod fixture_identity_tests {
    use super::*;
    use ix_trace_rs::trace;

    #[trace("FR-037-AC-1", "FR-037-AC-3", "FR-037-AC-4", "TC-052")]
    #[test]
    fn exact_application_and_enum_member_candidates_match_admitted_nodes() {
        let builder = bounded_increment_package();
        let raw = builder.raw_wire();
        let provisional = code_id(BOUNDED_INCREMENT);
        let raw_node: CheckedSemanticNodeV2 = serde_json::from_value(
            raw["semantic_graph"]["nodes"]
                .as_array()
                .expect("raw nodes")
                .iter()
                .find(|node| node["node_id"] == json!(provisional))
                .expect("raw increment")
                .clone(),
        )
        .expect("typed raw increment");
        assert_eq!(
            derive_application_node_id(
                ApplicationNodeDraft::new(&raw_node),
                None,
                CheckedPackageReadLimits::bounded().bytes,
            )
            .expect("raw application candidate")
            .candidate_node_id(),
            &provisional,
        );

        let (admitted, ids) = builder.admit_resolved();
        let admitted_id = ids.resolve(&provisional);
        let checked = admitted
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == admitted_id)
            .expect("admitted increment");
        assert_eq!(
            derive_application_node_id(
                ApplicationNodeDraft::new(checked),
                None,
                CheckedPackageReadLimits::bounded().bytes,
            )
            .expect("admitted application candidate")
            .candidate_node_id(),
            &admitted_id,
        );

        let enum_id = id(&base_node_ids().enum_member);
        assert!(admitted
            .graph()
            .nodes
            .iter()
            .any(|node| node.node_id == enum_id));
        assert_eq!(
            base_enum_member_preimage(&base_node_ids().enum_type, "OPEN")
                .digest(CheckedPackageReadLimits::bounded().bytes)
                .expect("OPEN identity"),
            enum_id.digest.as_ref(),
        );
        assert_ne!(
            base_enum_member_preimage(&base_node_ids().enum_type, "SHUT")
                .digest(CheckedPackageReadLimits::bounded().bytes)
                .expect("SHUT identity"),
            enum_id.digest.as_ref(),
        );
    }
}

/// A conversion to an admitted compound type, outside the oracle's supported target forms.
pub fn compound_conversion_package() -> PackageBuilder {
    let mut builder = derivation_package();
    let compound = key(9901);
    builder.node(
        &compound,
        "scalar_type",
        "compound_unit",
        &compound,
        json!({"term":"aggregate", "members":[]}),
    );
    let node = builder.value["semantic_graph"]["nodes"]
        .as_array_mut()
        .unwrap()
        .last_mut()
        .unwrap();
    node.as_object_mut().unwrap().remove("declaration");
    node.as_object_mut().unwrap().remove("owner");
    builder.application_bounded(
        9902,
        "expression",
        "conversion",
        &compound,
        application(
            "convert",
            op_full(
                "quire.op.quantity.convert",
                vec![],
                Some(mode_kv("rounding", "exact")),
                Some(type_argument_member(&compound)),
            ),
            &compound,
            vec![reference(&key(V_QUANTITY))],
        ),
        &[],
    );
    builder
}

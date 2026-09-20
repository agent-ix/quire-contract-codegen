//! Admitted CheckedPackage V2 fixtures for exact scalar generation.
//!
//! The base is QSpec's `positive-nominal-identities.json` I04 vector, vendored
//! from Contract IR, which supplies an enum declaration, one of its
//! members, a dimension and a declared unit under their honest nominal keys.
//! It is not currently byte-identical to Contract IR's own copy: this
//! repository's vendored copy carries a `package_id.digest` of
//! `a4a3d1699e33ceed6084fab17ce174bd6391a21fcd504bbf5d153710c1d2df39`, while
//! upstream's is `b70a9f27c9ef49711fb603d56014aa5ce092379cd820c5e62a0154c89877e7b4`
//! (both at the pinned `ef11217` revision); every other byte matches. Scalar
//! types, values and expressions are appended under readable zero-padded
//! keys, and the package identity is re-derived exactly as the Contract IR
//! fixture support does.
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

use quire_contract_codegen::{
    DecimalOperator, ExactScalarItem, ExactScalarOperation, IeeeArithmeticOperator,
    IntegerOperator, OrderingOperandKind, QuantityOperator, RationalOperator,
};
use quire_contract_ir::{
    CheckedArtifactLocator, CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult,
};
use quire_contract_runtime::exact::{
    ComparisonOperator, DecimalType, DivisionProfile, IeeeComparison, IeeeWidth, Integer,
    IntegerDomain, IntegerInterval, OrderingOperator, QuantityTarget, RationalDomain, RoundingMode,
    TextProfile, TextType,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const NODE_DOMAIN: &str = "quire.checked-semantic-node/v1";
pub const ENUM_TYPE: &str = "7928f1e1b570335b404c8d21c66da8a3b8e37e434b0ebc622f80285488811562";
pub const ENUM_MEMBER: &str = "42ba51e7e622d99f292a5e6dcc196bd216efb98b33755910e54af4d65078d032";
pub const UNIT_TYPE: &str = "79637623a46d29e884b62c6fa292aeb29d41e4ecc4e800b4d7ee910a3eaf23a4";

/// `validate_application_keys`'s own preimage version tag (quire-contract-ir
/// dfd8bd78, crates/quire-contract-model/src/checked_package/v2/operations.rs).
const APPLICATION_NODE_VERSION: &str = "quire.application-node/v1";

/// A readable node key: the code, zero-padded to a 64-digit digest.
pub fn key(code: u32) -> String {
    format!("{code:0>64}")
}

pub fn id(digest: &str) -> CheckedNodeId {
    serde_json::from_value(node_ref(digest)).expect("node id")
}

/// Every code this module ever builds a node for, mapped to its real node
/// id: the computed application digest for an application-bodied node (see
/// [`PackageBuilder::application_code`]/[`PackageBuilder::application_bounded`]),
/// or the readable placeholder [`key`] for a plain node built by
/// [`PackageBuilder::code`]/[`PackageBuilder::bounded`] (`validate_application_keys`
/// never re-derives those, so `key(code)` really is their id). [`code_id`]
/// reads it so a caller building `golden_items()`/`refused_items()` without
/// a `&mut PackageBuilder` in hand still gets the same id IR would.
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

/// The node id IR actually assigns for `code`, read from [`application_registry`].
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

/// Contract IR (a606059, FR-038-AC-17) requires `application.operation`
/// and `application.result_type` as members; IR-216's `validate_operations`
/// (quire-contract-ir dfd8bd78) checks `operation` against the closed
/// 135-entry `quire.checked-operation-catalog/v1`
/// (`tests/fixtures/checked-package/checked-package-v2/operation-catalog.json`),
/// so `operation` must name a real catalogued identity with a conformant
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
    operation: Value,
    result_type: &str,
    arguments: Vec<Value>,
) -> Value {
    json!({
        "term": "application",
        "operator": operator,
        "operation": operation,
        "result_type": node_ref(result_type),
        "arguments": arguments,
    })
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

/// An `operation.mode` member: `{"kind", "value"}`. `value` is never
/// checked against the catalog's closed `modes` vocabulary by IR (only
/// `kind` is, and by a type-pin lookup this module's types never carry --
/// see `check_mode_type` in quire-contract-ir dfd8bd78's
/// `checked_package/v2/operations.rs`), so any readable string works.
pub fn mode_kv(kind: &str, value: &str) -> Value {
    json!({"kind": kind, "value": value})
}

/// An `operation.member` naming only a `kind`: sufficient for every member
/// kind this module uses (`type_argument`), which IR checks for presence
/// and kind only.
pub fn member_kind(kind: &str) -> Value {
    json!({"kind": kind})
}

/// One catalogued law-role definition artifact ref, copied verbatim from
/// quire-contract-ir dfd8bd78's `tests/fixtures/checked-package/checked-package-v2/
/// operation-catalog.json` `law_roles` table -- `validate_operations`
/// requires `operation.laws[].definition` to equal one of these exactly
/// (quire-contract-ir dfd8bd78 `checked_package/v2/operations.rs`).
pub fn artifact_ref(identity: &str, digest: &str) -> Value {
    json!({
        "authority": "agent-ix",
        "identity": identity,
        "revision": {"namespace": "quire-draft", "value": "1-draft.1"},
        "digest_domain": "quire.definition.bytes/v1",
        "digest": digest,
    })
}

/// The catalogued `integer_division` law definition selecting `profile`.
pub fn integer_division_definition(profile: DivisionProfile) -> Value {
    let digest = match profile {
        DivisionProfile::Truncating => {
            "9998507608e4885b314d5dcc59a88bb3d04ef3c263d2d8ae5810f92ae1893364"
        }
        DivisionProfile::Floor => {
            "ca8c7a20407eaff7f61074cc997e44ad1ab9a73f675d686c6250997c6ae6192f"
        }
        DivisionProfile::Euclidean => {
            "9f5e59b3bfe1dd3c1efc74065b2e3e7869e21813a0c90b9c5938d82107267a51"
        }
    };
    artifact_ref(profile.definition_identity(), digest)
}

/// The catalog's one `ieee_profile` law definition.
pub fn ieee_profile_definition() -> Value {
    artifact_ref(
        "quire.value.ieee754-2019-default/v1",
        "3e9736fb8e1637b554385192de34547bafc073e90b4b85256c824be31e0aa6e5",
    )
}

/// The catalog's one `text_profile` law definition.
pub fn text_profile_definition() -> Value {
    artifact_ref(
        "quire.value.text.unicode-17.0.0/v1",
        "cd4a985a0d7d2f2b3d3625caee3787832c00c5244e805fb49e1c2c7075b9de5e",
    )
}

fn aggregate() -> Value {
    json!({"term": "aggregate", "members": []})
}

/// The corpus's own scalar-type node for a literal `value_kind`. Contract IR
/// (a606059) requires `literal.type` as a member and validates only that it
/// resolves to a real node (FR-038-AC-17); it does not require the target to
/// equal the containing node's own `semantic_type`, and most hand-written
/// corpus fixtures that type themselves by kind rather than by a bound do
/// follow that convention (`literal.type` == the node's own `semantic_type`),
/// matching the vendored `positive-nominal-identities.json` pattern. `corpus_package`'s
/// own `V_QUANTITY` is the one exception: its `literal.type` is `rational`
/// (this value's own `value_kind`) even though the node's `semantic_type` is
/// `UNIT_TYPE`, because Contract IR's lowering reaches `literal.type`
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
        "enum" => ENUM_TYPE.to_owned(),
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

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// A V2 package under construction.
pub struct PackageBuilder {
    value: Value,
    bounds: BTreeSet<String>,
}

impl Default for PackageBuilder {
    fn default() -> Self {
        let text = include_str!("../fixtures/exact_scalar/positive-nominal-identities.json");
        Self {
            value: serde_json::from_str(text).expect("vendored fixture is JSON"),
            bounds: BTreeSet::new(),
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
            "occurrences": [{"role": "declaration", "ordinal": 0}],
            "body": body,
        });
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

    /// Registers one application-bodied node with the real `node_id`
    /// IR-216's `validate_application_keys` re-derives: the SHA-256 digest
    /// of `{version, node_tag, semantic_form, semantic_type, declaration,
    /// recursion, body}` over sorted-key JSON bytes (quire-contract-ir
    /// dfd8bd78, `crates/quire-contract-model/src/checked_package/v2/
    /// operations.rs`). `digest` is not known until `declaration` -- itself
    /// part of the preimage -- is built, so `declaration` is derived from
    /// `code` (via `declaration_for`'s `label` parameter) rather than from
    /// the digest this call computes. No node this module builds via this
    /// method ever sets `recursion_group`, so `recursion` is always `null`
    /// in the preimage. Also records `code -> digest` in the module's
    /// application registry so [`code_id`] can look the same digest up
    /// without rebuilding the node.
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
        self.node_with_label(
            &digest,
            &label,
            tag,
            form,
            semantic_type,
            body,
            dependencies,
        )
    }

    /// An application-bodied node that depends on `bounds`, adding each
    /// bound node once -- the application analogue of [`Self::bounded`].
    pub fn application_bounded(
        &mut self,
        code: u32,
        tag: &str,
        form: &str,
        semantic_type: &str,
        body: Value,
        bounds: &[Bound],
    ) -> &mut Self {
        let keys = bounds
            .iter()
            .map(|bound| self.bound(bound))
            .collect::<Vec<_>>();
        self.application_code_with(code, tag, form, semantic_type, body, &keys)
    }

    /// Registers `definition` in `lock.definition_selections` (deduplicated),
    /// so `validate_operations`'s law-selection check (quire-contract-ir
    /// dfd8bd78 `checked_package/v2/operations.rs`) finds it selected for
    /// any node whose `operation.laws` names it. Mirrors the same push into
    /// `identity_preimage.definition_selections`: `validate_lock`'s
    /// `same_non_graph_lock` (quire-contract-ir dfd8bd78
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
            self.node_with(
                &digest,
                "bounded_domain",
                bound.form(),
                &bound.bounded_type(),
                bound.body(),
                &foreign,
            );
        }
        digest
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
        self.admit_with(CheckedPackageReadLimits::bounded())
    }

    pub fn admit_with(&self, limits: CheckedPackageReadLimits) -> CheckedPackageV2 {
        let wire = self.wire();
        let bytes = serde_json::to_vec(&wire).expect("canonical bytes");
        match CheckedPackageV2::read(&bytes, limits, &evidence(&wire)) {
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
        result_type(self.bounded_form())
    }

    /// Contract IR (a606059, FR-038-AC-17) requires `literal.type`, so every
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
            Self::Integer(lower, upper) => vec![integer_literal(lower), integer_literal(upper)],
            Self::Rational(nl, nu, dl, du) => vec![
                integer_literal(nl),
                integer_literal(nu),
                integer_literal(dl),
                integer_literal(du),
            ],
            Self::Decimal(lower, upper, min, max, rounding) => vec![
                integer_literal(lower),
                integer_literal(upper),
                integer_literal(min),
                integer_literal(max),
                literal("text", rounding),
            ],
            Self::Rounding(_, rounding) => vec![literal("text", rounding)],
            Self::Text(min, max, profile) => vec![
                integer_literal(min),
                integer_literal(max),
                literal("text", profile),
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

/// An integer range whose upper bound is spelled non-canonically.
pub fn unreadable_bound() -> Bound {
    Bound::Raw {
        form: "integer_range",
        bounded: "integer",
        body: json!({
            "term": "aggregate",
            "members": [literal("integer", "-5"), literal("integer", "05")],
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
/// A value whose semantic type is a bound, not a scalar type.
pub const V_UNTYPED: u32 = 109;

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
/// A binary32 addition with no rounding bound.
pub const MISSING_ROUNDING: u32 = 2023;
/// An integer addition reaching two integer ranges.
pub const AMBIGUOUS: u32 = 2024;
/// An integer addition whose range spells a bound non-canonically.
pub const UNREADABLE: u32 = 2025;
/// An integer addition whose only integer bound has the text form.
pub const WRONG_BOUND_FORM: u32 = 2026;
/// A floor division bounded `[-5, 5]`, requested over `[-1000, 1000]`.
pub const DOMAIN_MISMATCH: u32 = 2027;
/// A quantity conversion to a bounded rational, requested as exact.
pub const QUANTITY_EXACT: u32 = 2028;
/// An integer addition whose left operand is itself an application.
pub const EXPRESSION_OPERAND: u32 = 2029;
/// A quantity addition whose right operand is a literal.
pub const LITERAL_QUANTITY: u32 = 2030;
/// An integer addition whose right operand has no scalar type.
pub const UNTYPED_OPERAND: u32 = 2031;
/// A rational division over two integer-typed *reference* operands: IR
/// admits it (`quire.op.rational.div`'s catalogued operand family,
/// `rational_promotable`, is `{integer, rational}`), but CG's own
/// `Shape::of` requires `[Rational, Rational]`, so it is refused through
/// the `reference` arm of `check_operand`, not the `literal` arm
/// `WRONG_OPERAND` exercises.
pub const WRONG_OPERAND_REFERENCE: u32 = 2032;
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
        expression(
            1086,
            CONVERSION,
            QTY1,
            "decimal",
            Op::QuantityConversion {
                target: QuantityTarget::Decimal(decimal_type(
                    -100_000,
                    100_000,
                    0,
                    2,
                    RoundingMode::NearestEven,
                )),
            },
            vec![Bound::Decimal(-100_000, 100_000, 0, 2, "nearest-even")],
        ),
        expression(
            1087,
            CONVERSION,
            QTY1,
            "integer",
            Op::QuantityConversion {
                target: QuantityTarget::Integer {
                    domain: interval(-1000, 1000),
                    rounding: RoundingMode::TowardZero,
                },
            },
            vec![INT],
        ),
    ];
    for (code, profile) in TEXT_ADMISSIONS.into_iter().zip(TextProfile::ALL) {
        corpus.push(expression(
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
        "enum" => ENUM_MEMBER.to_owned(),
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
        "unit" => UNIT_TYPE.to_owned(),
        other => panic!("no type node for {other}"),
    }
}

fn integer_pair() -> Value {
    integer_pair_for(0)
}

/// [`integer_pair`], with `code` folded into the second operand so callers
/// that register several distinct expression nodes sharing this body's
/// shape (operator/operation/result_type) still get distinct digests.
/// `bounds`/`dependencies` are not part of the node-id preimage (only
/// `body` is -- see `APPLICATION_NODE_VERSION`'s doc comment), so nodes
/// that differ only by their bound set collide on id unless `body` itself
/// also varies.
fn integer_pair_for(code: u32) -> Value {
    application(
        "binary",
        op("quire.op.integer.add"),
        &key(T_INTEGER),
        vec![
            reference(&key(V_INTEGER)),
            literal("integer", &code.to_string()),
        ],
    )
}

/// The catalogued `body.operator`/`operation` pair for one corpus
/// expression, matched on its own [`ExactScalarOperation`] descriptor
/// against quire-contract-ir dfd8bd78's 135-entry operation catalog.
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
        Op::QuantityConversion { target } => (
            expr.operator,
            op_full(
                "quire.op.quantity.convert",
                vec![],
                match target {
                    QuantityTarget::Exact => None,
                    QuantityTarget::Decimal(decimal) => {
                        Some(mode_kv("rounding", decimal.rounding().as_str()))
                    }
                    QuantityTarget::Integer { rounding, .. } => {
                        Some(mode_kv("rounding", rounding.as_str()))
                    }
                },
                Some(member_kind("type_argument")),
            ),
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

/// The complete package: types, values, the corpus, and refused nodes.
pub fn corpus_package() -> PackageBuilder {
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
        builder.code(code, "scalar_type", form, &key(code), aggregate());
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
    // the node's `semantic_type` is `UNIT_TYPE`; Contract IR's lowering
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
        UNIT_TYPE,
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
        V_UNTYPED,
        "value",
        "literal",
        &int_bound,
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
        use ExactScalarOperation as Op;
        let mut arguments = expression
            .operands
            .iter()
            .map(|form| reference(&operand(form)))
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
        // (see `corpus_operation`'s `TextAdmission` arm), so this operand
        // must be a literal -- IR's operand-family check never resolves a
        // family for a literal argument, so the mismatch is never reached.
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
            arguments = vec![literal("text", &format!("a{}", expression.code))];
        }
        // No node this corpus builds has a `resolve_family` path to
        // `ordered_enum` (the enum type's own `scalar_type` form resolves
        // only to `enum`; see quire-contract-ir dfd8bd78's `resolve_family`
        // in `checked_package/v2/operations.rs`), so `enum.lt/le/gt/ge`'s
        // `ordered_enum` operand family can never be satisfied by a
        // `reference` argument here. Substituting literals for exactly the
        // ordering comparisons (never `eq`/`ne`, which accept the
        // `enum_kind` group `ENUM_MEMBER` already resolves to) bypasses the
        // family check the same way `TextAdmission` does above.
        if let Op::EnumComparison { operator } = expression.operation {
            if !matches!(
                operator,
                ComparisonOperator::Equal | ComparisonOperator::NotEqual
            ) {
                // As with the `TextAdmission` literal above: a fixed pair
                // here would give every ordering-comparison code among
                // `ENUM_COMPARISONS` the same body (the differing
                // `operation.identity` this expression's own operator
                // picks still varies below, but folding `code` in too
                // keeps this resilient to that identity ever coinciding
                // across two ordering operators).
                arguments = vec![
                    literal("enum", "READY"),
                    literal("enum", &format!("READY{}", expression.code)),
                ];
            }
        }
        let (catalog_operator, operation) = corpus_operation(&expression);
        builder.application_bounded(
            expression.code,
            "expression",
            expression.form,
            &result_type(expression.result),
            application(
                catalog_operator,
                operation,
                &result_type(expression.result),
                arguments,
            ),
            &expression.bounds,
        );
    }
    let integer_type = key(T_INTEGER);
    for (code, bounds) in [
        (DUPLICATED, vec![INT]),
        (WRONG_RESULT, vec![INT]),
        (WRONG_ARITY, vec![INT]),
        (UNBOUNDED, Vec::new()),
        (MATHEMATICAL, vec![INT]),
        (AMBIGUOUS, vec![INT, INT5]),
        (UNREADABLE, vec![unreadable_bound()]),
        (
            WRONG_BOUND_FORM,
            vec![Bound::Raw {
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
                    body: json!({"term": "aggregate", "members": [literal("text", "nfc")]}),
                    foreign: vec![],
                }],
            }],
        ),
        (DOMAIN_MISMATCH, vec![INT5]),
    ] {
        builder.application_bounded(
            code,
            "expression",
            "binary",
            &integer_type,
            integer_pair_for(code),
            &bounds,
        );
    }
    builder
        .application_bounded(
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
                // A second operand distinct from corpus code 1061's own
                // `ieee(1061, F32_2, ..., Add, Binary32, NearestEven)` --
                // that corpus expression's `operation` (via
                // `corpus_operation`'s `IeeeArithmetic` arm) is byte-
                // identical to this fixture's, so a matching pair of
                // `reference(V_FLOAT32)` arguments here would give this
                // node the same digest as code 1061's and IR's
                // `validate_graph` would refuse the whole package as a
                // duplicate node id.
                vec![reference(&key(V_FLOAT32)), literal("float32_bits", "1")],
            ),
            &[],
        )
        .application_bounded(
            QUANTITY_EXACT,
            "expression",
            "conversion",
            &key(T_RATIONAL),
            application(
                "convert",
                op_full(
                    "quire.op.quantity.convert",
                    vec![],
                    Some(mode_kv("rounding", "nearest-even")),
                    Some(member_kind("type_argument")),
                ),
                &key(T_RATIONAL),
                vec![reference(&key(V_QUANTITY))],
            ),
            &[RAT],
        )
        .application_bounded(
            EXPRESSION_OPERAND,
            "expression",
            "binary",
            &integer_type,
            application(
                "binary",
                op("quire.op.integer.add"),
                &integer_type,
                // `integer_pair()` is embedded directly as operand data,
                // not as a separate registered node: IR-216's
                // `validate_application_keys`/`validate_operations` only
                // ever re-derive/check a node whose own top-level `body`
                // is an application term, never a nested application
                // inside `body.arguments[*]` (quire-contract-ir dfd8bd78's
                // module doc, `checked_package/v2/operations.rs`), so this
                // nested blob's own placeholder-shaped `operation` is never
                // itself validated.
                vec![integer_pair(), reference(&key(V_INTEGER))],
            ),
            &[INT],
        )
        .application_bounded(
            LITERAL_QUANTITY,
            "expression",
            "binary",
            UNIT_TYPE,
            application(
                "binary",
                op("quire.op.quantity.add"),
                UNIT_TYPE,
                vec![reference(&key(V_QUANTITY)), literal("rational", "1")],
            ),
            &[],
        )
        .application_bounded(
            UNTYPED_OPERAND,
            "expression",
            "binary",
            &integer_type,
            application(
                "binary",
                op("quire.op.integer.add"),
                &integer_type,
                vec![reference(&key(V_INTEGER)), reference(&key(V_UNTYPED))],
            ),
            &[INT],
        );
    let boolean = key(T_BOOLEAN);
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
        .code(RELATION, "relation", "relationship", &boolean, aggregate())
        .code(STATE, "state", "state_clause", &boolean, aggregate())
        .application_code(
            TEMPORAL,
            "temporal",
            "temporal_clause",
            &boolean,
            application(
                "unary",
                op("quire.op.boolean.not"),
                &boolean,
                vec![literal("boolean", "true")],
            ),
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
        )
        .application_bounded(
            CALLS_FUNCTION,
            "expression",
            "binary",
            &integer_type,
            application(
                // `quire.op.integer.add`'s two operands both require the
                // `integer` family (the catalog's own entry, not the
                // broader `exact_numeric` group); `FUNCTION` resolves to the
                // catalog's own `function` family (its `node_tag` is
                // `function`, matched directly by `resolve_family`), which
                // no numeric identity's operand family admits, so that
                // pairing refuses `IllTyped`/`OperatorIneligible` at
                // admission before CG's own upstream-blocked check (which
                // runs at generation time, over IR's already-lowered
                // graph) is ever reached. `quire.op.function.call` is the
                // one catalog identity built for exactly this shape: its
                // first operand's family is `function` outright and its
                // `rest` accepts `any_term`, so `FUNCTION` and a plain
                // integer reference both admit unchanged.
                "call",
                op("quire.op.function.call"),
                &integer_type,
                // `FUNCTION` is itself an application-bodied node built
                // above in this same chain, so its real digest is already
                // registered; `key(FUNCTION)` would be the stale
                // placeholder and leave this reference dangling.
                vec![
                    reference(&code_id(FUNCTION).digest),
                    reference(&key(V_INTEGER)),
                ],
            ),
            &[INT],
        )
        .application_bounded(
            WRONG_BODY,
            "expression",
            "binary",
            &integer_type,
            application(
                "unary",
                op("quire.op.integer.negate"),
                &integer_type,
                vec![reference(&key(V_INTEGER))],
            ),
            &[INT],
        )
        .application_bounded(
            WRONG_OPERAND,
            "expression",
            "binary",
            &integer_type,
            application(
                "binary",
                op("quire.op.integer.add"),
                &integer_type,
                // A `reference(V_DECIMAL)` operand would resolve to the
                // `decimal` family and `quire.op.integer.add`'s catalog
                // entry requires `integer` in both positions, so IR would
                // refuse the whole package `IllTyped`/`OperatorIneligible`
                // at admission -- this fixture exists to exercise CG's own
                // operand-type check at generation time, over an already
                // -admitted package, not IR's. `argument_family` only ever
                // resolves a family for `reference`/`binding` arguments (a
                // `literal` always resolves to `None`, per its own match
                // arms in quire-contract-ir dfd8bd78's
                // `checked_package/v2/operations.rs`), so a literal operand
                // bypasses that admission-time check entirely while CG's
                // `check_operand` still classifies it by its own
                // `value_kind` and refuses the same
                // `OperandTypeMismatch { position: 1, expected: Integer,
                // found: Some("decimal") }`.
                vec![reference(&key(V_INTEGER)), literal("decimal", "1.5")],
            ),
            &[INT, DEC],
        )
        .application_bounded(
            WRONG_OPERAND_REFERENCE,
            "expression",
            "binary",
            &key(T_RATIONAL),
            application(
                "binary",
                op("quire.op.rational.div"),
                &key(T_RATIONAL),
                // First operand is `reference(V_INTEGER)`: `quire.op.
                // rational.div`'s catalogued operand family is
                // `rational_promotable` (`{integer, rational}`, see the
                // catalog's own `groups` table), so an integer-typed
                // reference admits at IR -- unlike `WRONG_OPERAND` above,
                // this exercises `check_operand`'s `"reference"` arm
                // (`reference_form` -> a graph lookup -> `type_form`), not
                // its `"literal"` arm. CG's own `Shape::of` for
                // `RationalOperator::Divide` still requires
                // `[Rational, Rational]`, so generation refuses the first
                // operand with `OperandTypeMismatch { position: 0,
                // expected: Rational, found: Some("integer") }` before the
                // second operand is ever reached -- it is a literal only to
                // keep this node's body distinct from corpus code 1033's
                // (`rational(1033, ..., RationalOperator::IntegerDivide,
                // ...)`), which already pairs two `reference(V_INTEGER)`
                // operands with this same catalogued identity under a
                // descriptor `Shape::of` accepts. This is the same shape a
                // real catalog family being coarser than CG's own
                // `ScalarForm` produces in production: e.g. two
                // integer-typed reference operands requested against
                // `quire.op.rational.div` under a `Rational` descriptor.
                vec![reference(&key(V_INTEGER)), literal("integer", "2032")],
            ),
            &[INT, RAT],
        );
    builder
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
        WRONG_OPERAND,
        MISSING,
        V_BOOLEAN,
        DUPLICATED,
        DUPLICATED,
        UNBOUNDED,
        AMBIGUOUS,
        UNREADABLE,
        WRONG_BOUND_FORM,
        EXPRESSION_OPERAND,
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
        MISSING_ROUNDING,
        ExactScalarOperation::IeeeArithmetic {
            operator: IeeeArithmeticOperator::Add,
            width: IeeeWidth::Binary32,
            rounding: RoundingMode::NearestEven,
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
        QUANTITY_EXACT,
        ExactScalarOperation::QuantityConversion {
            target: QuantityTarget::Exact,
        },
    ));
    items.push(item(
        LITERAL_QUANTITY,
        ExactScalarOperation::QuantityArithmetic {
            operator: QuantityOperator::Add,
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

/// The golden request: every corpus expression plus every refused item.
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
    use super::{Bound, DEC, INT, TEXT};
    use serde_json::json;

    fn keys(bounds: &[Bound]) -> Vec<String> {
        bounds.iter().map(Bound::key).collect()
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

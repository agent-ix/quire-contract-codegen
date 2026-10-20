//! The harness and identity record types of a generated Kani obligation.
//!
//! The generators build these records and the runner, the witness join and the replay read
//! them, so they live below both: a harness is run from its identity and its source text, and
//! no reader imports a generator.

use std::{num::NonZeroU64, time::Duration};

use quire_canonical::FixedShape;
use quire_contract_model::{CheckedNodeId, ClauseRef, DependencyIdentity, SourceSpan};
use serde::{Deserialize, Serialize};

use crate::{
    core::artifact::Artifact,
    core::identity::{HarnessPath, HarnessSymbol, ModuleSymbol},
    kani::abi::{KaniBindingRole, KaniIntegerBounds, KaniPrimitiveType, KaniSolver},
};

/// Contract IR's admitted identity for Boolean implication.
pub(crate) const BOOLEAN_IMPLIES_IDENTITY: &str = "quire.op.boolean.implies";

/// Resource ceilings bound into a proof identity (FR-028).
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProofCeilings {
    /// Maximum aggregate resident memory of the backend tree, in bytes.
    pub memory_bytes: NonZeroU64,
    /// Maximum duration of one harness run.
    pub wall_clock: Duration,
}

/// The inclusive domain of one symbolic argument, recorded in execution evidence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "domain", rename_all = "snake_case")]
pub enum SymbolicBounds {
    /// Both Boolean values.
    Boolean,
    /// The inclusive integer range drawn by the harness.
    Integer {
        /// Inclusive lower bound.
        minimum: i64,
        /// Inclusive upper bound.
        maximum: i64,
    },
    /// Ordered symbolic collection with inclusive cardinality and element domains.
    Sequence {
        /// Inclusive minimum number of active elements.
        minimum_length: i64,
        /// Inclusive maximum number of active elements.
        maximum_length: i64,
        /// Domain of every active element.
        element: Box<Self>,
    },
}

/// One argument's identity and the symbolic bounds the run used.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolicArgumentBounds {
    /// Identifier of the symbolic argument.
    pub identifier: String,
    /// Its recorded domain.
    pub bounds: SymbolicBounds,
}

/// The contract role of one obligation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, FixedShape)]
#[serde(rename_all = "snake_case")]
pub enum ObligationKind {
    /// Lowered to a `kani::proof` that the precondition is total and satisfiable in bounds.
    Precondition,
    /// Lowered to a `kani::ensures` checked by `kani::proof_for_contract`.
    Postcondition,
    /// Lowered to a `kani::requires`/`kani::ensures` preservation contract.
    Invariant,
    /// A frame's effects; generated from its operation's `state_clause` by
    /// [`generate_state_frame_obligations`](crate::kani::generate::frame::generate_state_frame_obligations),
    /// never by the clause renderer.
    Frame,
}

/// One primitive argument or result position of a harness's subject ABI.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObligationBinding {
    /// Generated identifier.
    pub identifier: String,
    /// Argument or result.
    pub role: KaniBindingRole,
    /// Rust primitive.
    pub primitive_type: KaniPrimitiveType,
    /// IR integer bounds; the only source of a symbolic assumption.
    pub integer_bounds: Option<KaniIntegerBounds>,
    /// Every IR dependency bound to this position.
    pub dependencies: Vec<DependencyIdentity>,
}

/// One clause oracle embedded in a harness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedOracle {
    /// The clause.
    pub clause: ClauseRef,
    /// Its contract role.
    pub kind: ObligationKind,
    /// Oracle function symbol.
    pub symbol: String,
}

/// What a generated harness proves and how it is run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KaniObligationIdentity {
    /// Request resource ceilings the backend must run under.
    pub ceilings: ProofCeilings,
    /// Contract role.
    pub kind: ObligationKind,
    /// The obligation's clause.
    pub clause: ClauseRef,
    /// Its QSL source span.
    pub source_span: SourceSpan,
    /// The obligation's oracle followed by every assumed precondition's oracle.
    pub oracles: Vec<EmbeddedOracle>,
    /// Harness module symbol.
    pub module_symbol: ModuleSymbol,
    /// Proof function symbol.
    pub harness_symbol: HarnessSymbol,
    /// Contract function symbol, for postcondition and invariant harnesses.
    pub contract_symbol: Option<String>,
    /// Customer subject, for postcondition and invariant harnesses.
    pub subject_path: Option<String>,
    /// Symbolic arguments, ascending by identifier.
    pub arguments: Vec<ObligationBinding>,
    /// Subject results, ascending by identifier.
    pub results: Vec<ObligationBinding>,
    /// Solver.
    pub solver: KaniSolver,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Every flag passed after `cargo kani`.
    pub options: Vec<String>,
}

/// One generated harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniObligationHarness {
    /// Identity.
    pub identity: KaniObligationIdentity,
    /// Self-contained Rust source.
    pub rust: Artifact,
    /// JSON record of the identity and the Rust source path: the persisted obligation schema a
    /// witness is decoded against.
    pub record: Artifact,
}

/// One symbolic scalar argument of a rendered harness. Integer claims use checked `i64`
/// bounds; a Boolean claim records its two values as the inclusive `0..=1` domain.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalarObligationArgument {
    /// Generated identifier.
    pub identifier: String,
    /// Inclusive checked minimum.
    pub minimum: i64,
    /// Inclusive checked maximum.
    pub maximum: i64,
}

/// Exact IR source and range used for one recursively inferred scalar operand.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferredScalarOperand {
    /// Zero-based argument position.
    pub position: usize,
    /// Referenced expression whose interval was inferred.
    pub source_node_id: CheckedNodeId,
    /// Inclusive exact lower endpoint.
    pub lower: String,
    /// Inclusive exact upper endpoint.
    pub upper: String,
}

/// What a V2 scalar obligation harness proves and how it is run. Parallel to [`KaniObligationIdentity`] but for an IR-confirmed exact-scalar claim,
/// which has no QSL clause or declaration -- a node id and its confirmed operation identity
/// stand in their place. Boolean implication additionally calls the compiled runtime subject.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalarObligationIdentity {
    /// Request resource ceilings the backend must run under.
    pub ceilings: ProofCeilings,
    /// The claimed node.
    pub node_id: CheckedNodeId,
    /// The node's own catalogued operation identity, IR-confirmed at package admission.
    pub operation_identity: String,
    /// The embedded oracle's function symbol.
    pub oracle_symbol: String,
    /// Harness module symbol.
    pub module_symbol: ModuleSymbol,
    /// Proof function symbol.
    pub harness_symbol: HarnessSymbol,
    /// Symbolic arguments, in call order.
    pub arguments: Vec<ScalarObligationArgument>,
    /// Exact IR evidence for operands ranged by recursive inference.
    pub inferred_operands: Vec<InferredScalarOperand>,
    /// Solver.
    pub solver: KaniSolver,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Every flag passed after `cargo kani`.
    pub options: Vec<String>,
}

/// Exact predicate forms the bounded-collection Kani renderer preserves from an admitted QSL
/// quantifier body.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "predicate", rename_all = "snake_case")]
pub enum CollectionPredicateIdentity {
    /// `element >= threshold` over an admitted integer range.
    IntegerGreaterOrEqual {
        /// The inclusive comparison threshold.
        threshold: i64,
    },
    /// `element == value` over an admitted integer range.
    IntegerEqual {
        /// The compared integer value.
        value: i64,
    },
    /// `element != value` over admitted Boolean elements.
    BooleanNotEqual {
        /// The compared Boolean value.
        value: bool,
    },
}

/// The domain of a single element in a checked collection claim.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "domain", rename_all = "snake_case")]
pub enum CollectionElementIdentity {
    /// Both Boolean values.
    Boolean,
    /// Inclusive integer range from the selected checked field.
    Integer {
        /// Inclusive lower element bound.
        minimum: i64,
        /// Inclusive upper element bound.
        maximum: i64,
    },
}

/// Collection ordering and duplicate semantics carried by one collection identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionKindIdentity {
    /// Ordered values with repeats permitted.
    Sequence,
}

/// Exact selected graph edge and its admitted multiplicity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphEdgeIdentity {
    /// Producer operation (`reaches` or `reaches_field`).
    pub operation: String,
    /// Selected field owner or relationship declaration.
    pub declaration: CheckedNodeId,
    /// Exact field name or relationship role.
    pub name: String,
    /// Inclusive lower edge count.
    pub lower: u64,
    /// Inclusive upper edge count.
    pub upper: u64,
    /// Edge order is significant.
    pub ordered: bool,
    /// Repeated targets are forbidden.
    pub unique: bool,
}

/// Semantic and execution identity of one graph occurrence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphObligationIdentity {
    /// Resource ceilings.
    pub ceilings: ProofCeilings,
    /// Admitted source package.
    pub package_digest: String,
    /// Containing state clause.
    pub clause_id: CheckedNodeId,
    /// Argument path from its condition root.
    pub argument_path: Vec<usize>,
    /// Located reachability application.
    pub application_id: CheckedNodeId,
    /// Exact population binding.
    pub population_id: CheckedNodeId,
    /// Covering object type.
    pub object_type: CheckedNodeId,
    /// Inclusive maximum population count (minimum is zero).
    pub maximum_population: u64,
    /// Selected edge domain.
    pub edge: GraphEdgeIdentity,
    /// Customer subject selected by the request.
    pub subject_path: String,
    /// Solver.
    pub solver: KaniSolver,
    /// Loop unwind.
    pub unwind: u32,
    /// Complete ordered backend options.
    pub options: Vec<String>,
    /// Rust module symbol.
    pub module_symbol: ModuleSymbol,
    /// Rust proof symbol.
    pub harness_symbol: HarnessSymbol,
}

/// Sealed generated graph harness and its exact semantic record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniGraphObligationHarness {
    /// Persisted source and execution identity.
    pub identity: GraphObligationIdentity,
    /// Generated Rust source.
    pub rust: Artifact,
    /// Persisted identity and source path.
    pub record: Artifact,
    generated_identity: GraphObligationIdentity,
    generated_rust: Artifact,
    generated_record: Artifact,
}
impl KaniGraphObligationHarness {
    pub(crate) fn generated(
        identity: GraphObligationIdentity,
        rust: Artifact,
        record: Artifact,
    ) -> Self {
        Self {
            generated_identity: identity.clone(),
            generated_rust: rust.clone(),
            generated_record: record.clone(),
            identity,
            rust,
            record,
        }
    }
    pub(crate) fn has_valid_provenance(&self) -> bool {
        self.identity == self.generated_identity
            && self.rust == self.generated_rust
            && self.record == self.generated_record
    }
}

/// The exact selected V2 collection claim and execution configuration of one generated harness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionObligationIdentity {
    /// Backend ceilings.
    pub ceilings: ProofCeilings,
    /// Admitted package identity.
    pub package_digest: String,
    /// Caller-selected function subject whose body contains this claim.
    pub subject_path: String,
    /// Selected function identity.
    pub function_id: CheckedNodeId,
    /// Selected quantifier identity.
    pub quantifier_id: CheckedNodeId,
    /// The projection node resolving the exact model field.
    pub projection_id: CheckedNodeId,
    /// Model object declaration that owns the field.
    pub declaration_id: CheckedNodeId,
    /// Exact selected field name.
    pub field_name: String,
    /// Collection ordering and uniqueness semantics.
    pub collection_kind: CollectionKindIdentity,
    /// Inclusive cardinality bounds.
    pub minimum_length: i64,
    /// Inclusive cardinality bounds.
    pub maximum_length: i64,
    /// Element domain.
    pub element: CollectionElementIdentity,
    /// Exact admitted element type node.
    pub element_type_id: CheckedNodeId,
    /// Exact admitted quantifier operation identity.
    pub quantifier_operation: String,
    /// Exact predicate node identity.
    pub predicate_id: CheckedNodeId,
    /// Exact predicate form emitted by QSL.
    pub predicate: CollectionPredicateIdentity,
    /// Solver.
    pub solver: KaniSolver,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Every Kani flag passed after `cargo kani`.
    pub options: Vec<String>,
    /// Harness module symbol.
    pub module_symbol: ModuleSymbol,
    /// Harness function symbol.
    pub harness_symbol: HarnessSymbol,
}

/// A generated Kani harness for one admitted bounded collection claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniCollectionObligationHarness {
    /// Persisted exact claim and run identity.
    pub identity: CollectionObligationIdentity,
    /// Self-contained generated Rust.
    pub rust: Artifact,
    /// Persisted identity/source-path record.
    pub record: Artifact,
    generated_identity: CollectionObligationIdentity,
    generated_rust: Artifact,
    generated_record: Artifact,
}

impl KaniCollectionObligationHarness {
    /// Construct the immutable generator-origin seal for one collection harness.
    pub(crate) fn generated(
        identity: CollectionObligationIdentity,
        rust: Artifact,
        record: Artifact,
    ) -> Self {
        Self {
            generated_identity: identity.clone(),
            generated_rust: rust.clone(),
            generated_record: record.clone(),
            identity,
            rust,
            record,
        }
    }

    /// Whether public fields still match the immutable generator-origin seal.
    #[must_use]
    pub(crate) fn has_valid_provenance(&self) -> bool {
        self.identity == self.generated_identity
            && self.rust == self.generated_rust
            && self.record == self.generated_record
    }
}

/// One generated V2 exact-scalar obligation harness.
#[derive(Clone, Debug)]
pub struct KaniScalarObligationHarness {
    /// Identity.
    pub identity: ScalarObligationIdentity,
    /// Self-contained Rust source.
    pub rust: Artifact,
    /// JSON record of the identity and the Rust source path: the persisted obligation schema a
    /// witness is decoded against.
    pub record: Artifact,
    generated_identity: ScalarObligationIdentity,
    generated_rust: Artifact,
    generated_record: Artifact,
    generated_artifacts: Option<Vec<Artifact>>,
}

// This value compares the per-harness public output and its origin seal. The routed package seal
// is whole-call execution context; including it would make equality depend on unrelated siblings.
impl PartialEq for KaniScalarObligationHarness {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity
            && self.rust == other.rust
            && self.record == other.record
            && self.generated_identity == other.generated_identity
            && self.generated_rust == other.generated_rust
            && self.generated_record == other.generated_record
    }
}

impl Eq for KaniScalarObligationHarness {}

impl KaniScalarObligationHarness {
    /// Construct the immutable generator-origin seal for one scalar harness.
    pub(crate) fn generated(
        identity: ScalarObligationIdentity,
        rust: Artifact,
        record: Artifact,
    ) -> Self {
        Self {
            generated_identity: identity.clone(),
            generated_rust: rust.clone(),
            generated_record: record.clone(),
            identity,
            rust,
            record,
            generated_artifacts: None,
        }
    }

    /// Whether the public retained fields still match the generator-origin seal.
    #[must_use]
    pub(crate) fn has_valid_provenance(&self) -> bool {
        self.identity == self.generated_identity
            && self.rust == self.generated_rust
            && self.record == self.generated_record
    }

    /// Bind the package generated in the same routed call to this private retained harness.
    pub(crate) fn bind_artifact_package(&mut self, artifacts: &[Artifact]) {
        let mut artifacts = artifacts.to_vec();
        artifacts.sort_by(|left, right| left.path.cmp(&right.path));
        self.generated_artifacts = Some(artifacts);
    }

    /// Whether the caller-supplied artifacts still match the package captured during routing.
    #[must_use]
    pub(crate) fn matches_artifact_package(&self, artifacts: &[Artifact]) -> bool {
        self.has_valid_provenance()
            && self.generated_artifacts.as_ref().is_some_and(|expected| {
                let mut actual = artifacts.to_vec();
                actual.sort_by(|left, right| left.path.cmp(&right.path));
                &actual == expected
            })
    }
}

impl KaniObligationIdentity {
    /// The `module::harness` path of this harness.
    #[must_use]
    pub fn harness_path(&self) -> HarnessPath {
        HarnessPath {
            module: self.module_symbol.clone(),
            harness: self.harness_symbol.clone(),
        }
    }
}

impl ScalarObligationIdentity {
    /// The `module::harness` path of this harness.
    #[must_use]
    pub fn harness_path(&self) -> HarnessPath {
        HarnessPath {
            module: self.module_symbol.clone(),
            harness: self.harness_symbol.clone(),
        }
    }
}

/// One generated harness with the identity it proves and its persisted record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateFrameHarness {
    /// Everything the proof is about.
    pub identity: StateFrameIdentity,
    /// The harness source, `src/generated/<module>.rs`.
    pub rust: Artifact,
    /// The persisted identity record, `kani-obligations/<module>.json`.
    pub record: Artifact,
}

/// An integer comparison operator of the closed `quire.op.integer` ordering family.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StateComparison {
    /// `<`.
    Lt,
    /// `<=`.
    Le,
    /// `>`.
    Gt,
    /// `>=`.
    Ge,
    /// `==`.
    Eq,
    /// `!=`.
    Ne,
}

/// The property one harness proves.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
pub enum StateFrameProperty {
    /// `left <op> right` between the named observations of one field.
    Postcondition {
        /// The one integer field the condition reads.
        field: String,
        /// The comparison.
        comparison: StateComparison,
        /// Whether the left operand is the field's pre-state value (else its post-state value).
        left_is_pre: bool,
    },
    /// Every `checked` field is unchanged; the `granted` fields may change.
    Frame {
        /// The frame's `modifies` field entries: the allowed effects.
        granted: Vec<String>,
        /// Every other state field: the forbidden effects, each asserted unchanged.
        checked: Vec<String>,
    },
}

/// The inclusive integer range the IR carries for one state field. Every harness assumes it of
/// the symbolic pre-state, so a counterexample is a state the model admits.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StateFieldDomain {
    /// The state field.
    pub field: String,
    /// Inclusive lower bound.
    pub minimum: i64,
    /// Inclusive upper bound.
    pub maximum: i64,
}

/// The operation a harness is scoped to. Two harnesses with different scopes are never the same
/// proof.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StateFrameScope {
    /// The operation's name, from its anchor.
    pub operation: String,
    /// The framed object type node, the anchor's `context`.
    pub object: CheckedNodeId,
    /// The operation's `state`/`operation_anchor` node.
    pub anchor: CheckedNodeId,
    /// The operation's `state`/`frame` node.
    pub frame: CheckedNodeId,
}

/// Everything a generated harness proves, persisted with it.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StateFrameIdentity {
    /// Request resource ceilings the backend must run under.
    pub ceilings: ProofCeilings,
    /// The `state_clause` node both obligations of a request come from.
    pub clause: CheckedNodeId,
    /// The operation scope.
    pub scope: StateFrameScope,
    /// The property proved.
    pub property: StateFrameProperty,
    /// Every state field the harness draws, in draw order: the order of the harness's
    /// `kani::any()` calls, hence of the values a playback of it holds. It is what the decoder
    /// types a playback by; `domains` lists the ranged fields only and `property`'s lists are
    /// unordered. It decodes and orders and is no member of an obligation identity
    /// (FR-024-AC-23). A record that lacks it is not read as a frame identity.
    pub state_fields: Vec<String>,
    /// The IR range assumed of each state field that has one, in `state_fields` order.
    pub domains: Vec<StateFieldDomain>,
    /// Present state fields without an `i64` range, in `state_fields` order.
    pub unranged: Vec<StateUnrangedField>,
    /// Rust path of the state struct.
    pub state_path: String,
    /// Rust path of the operation subject.
    pub subject_path: String,
    /// The generated module holding the harness.
    pub module_symbol: ModuleSymbol,
    /// The `kani::proof` function.
    pub harness_symbol: HarnessSymbol,
    /// The solver, always `cadical`.
    pub solver: KaniSolver,
    /// The unwind bound.
    pub unwind: u32,
    /// The exact Kani option vector.
    pub options: Vec<String>,
}

/// A present state field whose model type does not provide an `i64` range.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StateUnrangedField {
    /// The field's declared name.
    pub field: String,
    /// Why no range can be assumed.
    pub reason: StateUnrangedReason,
}

/// Why a state field has no `i64` range.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StateUnrangedReason {
    /// The declared member type is not a representable `i64` range.
    TypeNotRange,
}

/// A persisted state-frame record is not a state-frame identity: it is no JSON object of the
/// record's shape, or it lacks a member the identity requires, `state_fields` among them.
#[derive(Debug)]
pub enum StateFrameRecordError {
    /// The record is not a JSON object of the record's shape, or lacks a member.
    Json(serde_json::Error),
    /// A state field is listed twice: the draw order would type two playback positions as one.
    RepeatedField {
        /// The repeated field.
        field: String,
    },
}

impl std::fmt::Display for StateFrameRecordError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(cause) => write!(
                formatter,
                "the record is not a state-frame identity: {cause}"
            ),
            Self::RepeatedField { field } => {
                write!(
                    formatter,
                    "the record lists the state field `{field}` twice"
                )
            }
        }
    }
}

impl std::error::Error for StateFrameRecordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(cause) => Some(cause),
            Self::RepeatedField { .. } => None,
        }
    }
}

/// The persisted `kani-obligations/<module>.json` record of a state-frame harness: its identity
/// and the path of the source it was generated beside.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct StateFrameRecord {
    identity: StateFrameIdentity,
    #[serde(rename = "rustPath")]
    _rust_path: String,
}

impl StateFrameIdentity {
    /// The `module::harness` path of this harness.
    #[must_use]
    pub fn harness_path(&self) -> HarnessPath {
        HarnessPath {
            module: self.module_symbol.clone(),
            harness: self.harness_symbol.clone(),
        }
    }

    /// The identity a persisted state-frame record holds.
    ///
    /// # Errors
    ///
    /// [`StateFrameRecordError`] when `record` is not a state-frame record: a record written
    /// before the identity carried `state_fields` has no draw order to decode a playback by and
    /// is refused, not read with a guessed one, and so is a record that lists a state field
    /// twice.
    pub fn from_record(record: &str) -> Result<Self, StateFrameRecordError> {
        let identity = serde_json::from_str::<StateFrameRecord>(record)
            .map(|record| record.identity)
            .map_err(StateFrameRecordError::Json)?;
        let mut seen = std::collections::BTreeSet::new();
        if let Some(field) = identity
            .state_fields
            .iter()
            .find(|field| !seen.insert(field.as_str()))
        {
            return Err(StateFrameRecordError::RepeatedField {
                field: field.clone(),
            });
        }
        seen.clear();
        if let Some(field) = identity
            .unranged
            .iter()
            .find(|field| !seen.insert(field.field.as_str()))
        {
            return Err(StateFrameRecordError::RepeatedField {
                field: field.field.clone(),
            });
        }
        Ok(identity)
    }
}

//! The harness and identity record types of a generated Kani obligation.
//!
//! The generators build these records and the runner, the witness join and the replay read
//! them, so they live below both: a harness is run from its identity and its source text, and
//! no reader imports a generator.

use std::{num::NonZeroU64, time::Duration};

use quire_canonical::FixedShape;
use quire_contract_model::{
    CheckedNodeId, CheckedSemanticId, ClauseRef, DependencyIdentity, SourceSpan,
};
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

/// Versioned physical-to-semantic binding policy for a V2 clause's `i64` draw.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V2SymbolicEncodingPolicy {
    /// The original `i64` draw feeds the semantic value directly.
    OriginalI64V1,
    /// A bounded `i64` draw is losslessly rebound through `u16` after its assumption.
    PostAssumptionU16RebindV1,
}

/// One admitted V2 clause and the execution facts of its generated harness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V2ClauseObligationIdentity {
    /// Required, versioned encoding of the generated symbolic value.
    pub symbolic_encoding_policy: V2SymbolicEncodingPolicy,
    /// Selected integer ABI; absent on the general route.
    #[serde(skip_serializing_if = "ScalarIntegerRepresentation::is_general")]
    pub integer_representation: ScalarIntegerRepresentation,
    /// Checked native capacity decisions, when selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_integer_selection: Option<crate::core::native_integer::NativeIntegerSelection>,
    /// Backend resource ceilings.
    pub ceilings: ProofCeilings,
    /// Admitted package identity.
    pub package: CheckedSemanticId,
    /// Selected state-clause node.
    pub clause: CheckedNodeId,
    /// Contract role read from the checked node.
    pub kind: ObligationKind,
    /// Embedded oracle symbol.
    pub oracle_symbol: String,
    /// Caller-selected subject path recorded for harness identity even when this role has no call.
    pub subject_path: String,
    /// Symbolic parameter and admitted bounds.
    pub arguments: Vec<ScalarObligationArgument>,
    /// Generated Rust module.
    pub module_symbol: ModuleSymbol,
    /// Generated proof function.
    pub harness_symbol: HarnessSymbol,
    /// Backend solver.
    pub solver: KaniSolver,
    /// Loop unwind.
    pub unwind: u32,
    /// Exact backend options.
    pub options: Vec<String>,
}

/// Generated V2 clause harness, sealed against caller edits before execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniV2ClauseObligationHarness {
    /// Execution identity.
    pub identity: V2ClauseObligationIdentity,
    /// Self-contained Rust source.
    pub rust: Artifact,
    /// Persisted identity record.
    pub record: Artifact,
    generated_identity: V2ClauseObligationIdentity,
    generated_rust: Artifact,
    generated_record: Artifact,
}

impl KaniV2ClauseObligationHarness {
    /// Construct generator-owned sealed output.
    pub(crate) fn generated(
        identity: V2ClauseObligationIdentity,
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

    /// Whether public fields retain their generator-owned values.
    pub(crate) fn has_valid_provenance(&self) -> bool {
        self.identity == self.generated_identity
            && self.rust == self.generated_rust
            && self.record == self.generated_record
    }
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

/// The integer representation selected for a scalar proof.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScalarIntegerRepresentation {
    /// Arbitrary-precision exact integers.
    #[default]
    General,
    /// Admitted native integers evaluated by the exact kernel.
    Native,
}

impl ScalarIntegerRepresentation {
    fn is_general(&self) -> bool {
        *self == Self::General
    }
}

/// What a V2 scalar obligation harness proves and how it is run. Parallel to [`KaniObligationIdentity`] but for an IR-confirmed exact-scalar claim,
/// which has no QSL clause or declaration -- a node id and its confirmed operation identity
/// stand in their place. Boolean implication additionally calls the compiled runtime subject.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalarObligationIdentity {
    /// Selected integer ABI; omitted from general-route records.
    #[serde(skip_serializing_if = "ScalarIntegerRepresentation::is_general")]
    pub integer_representation: ScalarIntegerRepresentation,
    /// Native capacity policy and checked intervals, when selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_integer_selection: Option<crate::core::native_integer::NativeIntegerSelection>,
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

/// Why the original scalar renderer limits cannot be read from a retained harness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarOriginalLimitsError {
    /// This harness was not produced with a retained scalar meter context.
    Unavailable,
    /// The retained identity, source or record differs from the generator-origin seal.
    GeneratedArtifactChanged,
}

impl std::fmt::Display for ScalarOriginalLimitsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Unavailable => "original scalar renderer limits were not retained",
            Self::GeneratedArtifactChanged => {
                "retained scalar artifact differs from its generation seal"
            }
        })
    }
}

impl std::error::Error for ScalarOriginalLimitsError {}

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
    original_scalar_limits: Option<quire_exact::ScalarLimits>,
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
            && self.original_scalar_limits == other.original_scalar_limits
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
            original_scalar_limits: None,
        }
    }

    /// Seal the actual meter limits supplied by the scalar renderer beside its generated source.
    pub(crate) fn generated_with_scalar_limits(
        identity: ScalarObligationIdentity,
        rust: Artifact,
        record: Artifact,
        limits: quire_exact::ScalarLimits,
    ) -> Self {
        let mut harness = Self::generated(identity, rust, record);
        harness.original_scalar_limits = Some(limits);
        harness
    }

    /// The original accounting limits supplied to the meter in this generated scalar source.
    ///
    /// These limits come from generation, independently of any later native or replay budget.
    /// The private generator seal retains them beside the exact source and identity that the
    /// existing proof-content authority stages. This accessor authenticates retained generation
    /// fields; it does not authenticate a proving run, native observation or original call site.
    ///
    /// # Errors
    /// Returns [`ScalarOriginalLimitsError::GeneratedArtifactChanged`] if any public retained
    /// field changed, or [`ScalarOriginalLimitsError::Unavailable`] for a harness without a
    /// retained scalar meter context.
    pub fn original_scalar_limits(
        &self,
    ) -> Result<quire_exact::ScalarLimits, ScalarOriginalLimitsError> {
        if !self.has_valid_provenance() {
            return Err(ScalarOriginalLimitsError::GeneratedArtifactChanged);
        }
        self.original_scalar_limits
            .ok_or(ScalarOriginalLimitsError::Unavailable)
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

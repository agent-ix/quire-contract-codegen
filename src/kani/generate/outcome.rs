//! The request and result vocabulary of Kani obligation negotiation (FR-015): the items a
//! request names, the records it returns and the reasons an item has no harness. A leaf inside
//! `generate/`: it imports no family.

use quire_contract_model::{
    BoundPackage, CheckedModelFieldsError, CheckedNodeId, CheckedNodeTag, CheckedPackageIncomplete,
    CheckedPackageLimit, CheckedPackageRefusal, CheckedPackageV2, CheckedSourceMapEntry,
    ClauseKind, ClauseRef, DependencyIdentity, SourceSpan,
};
use serde::{Serialize, Serializer};

use crate::{
    core::diagnostic::GenerationErrorCode,
    core::identity::HarnessSymbol,
    kani::identity::{
        KaniObligationHarness, KaniScalarObligationHarness, ObligationKind, StateFrameHarness,
    },
    oracle::claim::{ClaimMap, UpstreamBlocker},
    oracle::scalar::{ClaimDerivationRefusal, ExactScalarClaim, ExactScalarRefusal},
};

/// Largest number of items one request may negotiate.
pub const MAX_OBLIGATION_ITEMS: usize = 256;

/// Largest accepted loop unwind bound.
pub const MAX_OBLIGATION_UNWIND: u32 = 1024;

/// Which of a state clause's two harnesses a [`ObligationItem::StateFrame`] item asks for.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateFrameRole {
    /// The operation-contract harness: the clause's comparison between the pre-state and the
    /// post-state. Its record's kind is [`ObligationKind::Postcondition`].
    Contract,
    /// The frame-effect harness: every state field the frame does not grant is left unchanged.
    /// Its record's kind is [`ObligationKind::Frame`].
    Frame,
}

impl StateFrameRole {
    /// The obligation kind a record of this role carries.
    pub(crate) const fn obligation_kind(self) -> ObligationKind {
        match self {
            Self::Contract => ObligationKind::Postcondition,
            Self::Frame => ObligationKind::Frame,
        }
    }
}

/// One item to negotiate.
#[derive(Clone, Copy, Debug)]
pub enum ObligationItem<'a> {
    /// One role of one `postcondition` `state_clause` of an admitted package. The inputs are
    /// those of [`StateFrameRequest`](crate::kani::generate::frame::StateFrameRequest), and the
    /// request's unwind bound applies. The harness calls the item's own `subject_path`; the
    /// request's `subject_path` serves the other arms and is not read by this one.
    StateFrame {
        /// The admitted package.
        package: &'a CheckedPackageV2,
        /// The `state`/`state_clause` node of a `postcondition` clause.
        clause: &'a CheckedNodeId,
        /// The harness asked for.
        role: StateFrameRole,
        /// Rust path of the state struct.
        state_path: &'a str,
        /// Every field of the state struct, in declaration order.
        state_fields: &'a [&'a str],
        /// Rust path of the operation subject.
        subject_path: &'a str,
    },
    /// One clause of a frozen V1 bound package.
    BoundClause {
        /// The package the clause belongs to.
        package: &'a BoundPackage,
        /// The clause.
        clause: &'a ClauseRef,
    },
    /// One node of an FR-014 exact scalar claim map.
    ScalarClaim {
        /// The admitted package the claim map was generated from.
        package: &'a CheckedPackageV2,
        /// The claim map.
        claim_map: &'a ClaimMap<ExactScalarClaim>,
        /// The claimed node.
        node_id: &'a CheckedNodeId,
    },
}

/// A complete negotiation and generation request.
#[derive(Clone, Copy, Debug)]
pub struct KaniObligationRequest<'a> {
    /// Items, in the order records are reported.
    pub items: &'a [ObligationItem<'a>],
    /// Rust path of the customer subject called by postcondition and invariant harnesses.
    pub subject_path: &'a str,
    /// Loop unwind bound, `1..=MAX_OBLIGATION_UNWIND`.
    pub unwind: u32,
}

/// A request that cannot be negotiated at all; no item is accounted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KaniObligationError {
    /// The request names no items.
    EmptyRequest,
    /// The request names more than [`MAX_OBLIGATION_ITEMS`] items.
    TooManyItems {
        /// Items named.
        count: usize,
    },
    /// The subject path is not a Rust path.
    InvalidSubjectPath,
    /// The unwind bound is outside `1..=MAX_OBLIGATION_UNWIND`.
    InvalidUnwind {
        /// The requested bound.
        unwind: u32,
    },
}

/// The IR item a record is about.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "item", rename_all = "snake_case")]
pub enum ObligationSubject {
    /// A V1 clause.
    BoundClause {
        /// Full package, requirement, revision and clause identity.
        clause: ClauseRef,
        /// The clause's QSL source span, when the clause exists in the package.
        source_span: Option<SourceSpan>,
    },
    /// A V2 node.
    CheckedNode {
        /// The node.
        node_id: CheckedNodeId,
        /// Its QSL source correspondence.
        source_map: Vec<CheckedSourceMapEntry>,
    },
}

/// A symbolic domain the IR carries for a refused V2 item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "domain", rename_all = "snake_case")]
pub enum DerivedDomain {
    /// An inclusive integer range read from an `integer_range` bound.
    IntegerRange {
        /// The bound node.
        bound: CheckedNodeId,
        /// Inclusive lower bound, canonical decimal.
        lower: String,
        /// Inclusive upper bound, canonical decimal.
        upper: String,
    },
    /// A bound of a form with no `kani::any` range encoding.
    NotSymbolic {
        /// The bound node.
        bound: CheckedNodeId,
        /// Its form.
        form: String,
    },
}

/// Why a well-formed item has no harness.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum UnsupportedObligation {
    /// Assertion and case clauses are not contract obligations.
    ClauseKindNotObligation {
        /// The clause kind.
        kind: ClauseKind,
    },
    /// The clause carries definedness obligations the harness cannot preserve; an assumption
    /// would otherwise exclude undefined outcomes.
    DefinednessNotEncoded {
        /// Number of obligations.
        obligations: usize,
    },
    /// The clause oracle could not be lowered.
    ClauseLowering {
        /// First oracle-lowering code.
        generation_code: GenerationErrorCode,
    },
    /// A dependency's observation has no position at this obligation's execution point.
    ObservationNotBindable {
        /// The dependency.
        dependency: DependencyIdentity,
    },
    /// Two dependencies claim one subject slot with different types or bounds.
    AbiConflict {
        /// The slot identifier.
        identifier: String,
    },
    /// Contract obligations on one operation each bind, but their union gives one subject slot
    /// two types or bounds, so no single subject signature serves all of their harnesses.
    SubjectSignatureConflict {
        /// The shared operation.
        operation: String,
        /// The slot identifier.
        identifier: String,
    },
    /// A precondition sharing this obligation's anchor is not a supported item of the request.
    PreconditionNotNegotiated {
        /// The precondition.
        precondition: ClauseRef,
    },
    /// No harness could be produced for a reason that is not the bounded-resource or syntax
    /// ground below. Two distinct causes still collapse to this one code: (1) an
    /// internal-invariant fallback for an otherwise-successful render -- its oracle function
    /// symbol could not be located in generated source, or an ABI binding `abi` already
    /// resolved could not be found again when assembling the harness body; and (2) the harness
    /// identity or record struct failing to serialize as JSON. Distinct from
    /// [`Self::ResourceLimitExceeded`] and [`Self::InvalidGeneratedSyntax`] below, which this
    /// generator does split out.
    RenderFailed,
    /// A frame is not a clause oracle, so this renderer has no encoding for one. The frame
    /// obligations of an operation are generated from its `state_clause` by
    /// [`generate_state_frame_obligations`](crate::kani::generate::frame::generate_state_frame_obligations).
    FrameNotClauseRendered,
    /// The generated harness source exceeds
    /// [`MAX_GENERATED_SOURCE_BYTES`](crate::core::artifact::MAX_GENERATED_SOURCE_BYTES), the same
    /// bounded-resource ceiling every other generator in this crate enforces. Distinct from
    /// [`Self::InvalidGeneratedSyntax`]: a resource ceiling is not a generator defect.
    ResourceLimitExceeded {
        /// The generated source's length in bytes.
        bytes: usize,
    },
    /// The generated harness source is within the size ceiling but failed `syn::parse_file` --
    /// invalid Rust rather than a resource ceiling. Distinct from [`Self::ResourceLimitExceeded`]
    /// for the same reason.
    ///
    /// Known narrowing: the analogous
    /// [`KaniErrorCode::InvalidGeneratedSyntax`](crate::kani::generate::census_validation::KaniErrorCode::InvalidGeneratedSyntax)
    /// is classified `Inconclusive` by its `terminal_state` (a generator defect, not an honest
    /// refusal), distinct from `ResourceLimitExceeded`'s `Unsupported`. [`ObligationDisposition`]
    /// has no inconclusive-equivalent arm, so this variant is reported through the same
    /// `ObligationDisposition::Unsupported` as every genuine refusal -- a real generator defect
    /// reaching this ground is currently indistinguishable from an honest one. Restructuring
    /// `ObligationDisposition`'s terminal-state semantics to carry that distinction is out of
    /// scope for this change.
    InvalidGeneratedSyntax {
        /// The `syn::parse_file` error text.
        error: String,
    },
    /// An IR bound admits no value.
    UnsatisfiableBound {
        /// The bound node.
        bound: CheckedNodeId,
        /// Its form.
        form: String,
        /// Its lower bound.
        lower: String,
        /// Its upper bound.
        upper: String,
    },
    /// The operation identity is caller-declared, so its law is not checked by the IR.
    CallerDeclaredOperation {
        /// The declared operation.
        operation_identity: String,
        /// The missing upstream transport.
        blocked_on: UpstreamBlocker,
        /// The domains the IR does carry.
        derived_domains: Vec<DerivedDomain>,
    },
    /// The operation identity is IR-confirmed, but this generator has no Kani harness renderer
    /// for its family yet (today: every family but `IntegerArithmetic`), or the claim map is one
    /// this generator did not produce.
    OperationNotRendered {
        /// The confirmed operation.
        operation_identity: String,
        /// The domains the IR does carry.
        derived_domains: Vec<DerivedDomain>,
    },
    /// The operation identity is IR-confirmed and its checked bound is an integer range, but one
    /// of its inclusive endpoints, or a literal operand's value (then both `lower` and `upper`),
    /// does not fit `i64`, so no `kani::any::<i64>()` argument can be constrained to it.
    DomainNotRepresentableInI64 {
        /// The confirmed operation.
        operation_identity: String,
        /// Inclusive lower bound, canonical decimal.
        lower: String,
        /// Inclusive upper bound, canonical decimal.
        upper: String,
    },
    /// The operation identity is IR-confirmed, but no operand values the harness would assume
    /// give a result inside the result range: every exact result lies in
    /// `[reachable_lower, reachable_upper]`, disjoint from `[lower, upper]`. A harness would pass
    /// with its non-vacuity cover unmet by any input, so none is rendered.
    ResultBoundUnreachable {
        /// The confirmed operation.
        operation_identity: String,
        /// The result range's inclusive lower bound.
        lower: i64,
        /// The result range's inclusive upper bound.
        upper: i64,
        /// The least exact result over the operand ranges, canonical decimal.
        reachable_lower: String,
        /// The greatest exact result over the operand ranges, canonical decimal.
        reachable_upper: String,
    },
    /// A reachable node family has no finite encoding.
    NoFiniteEncoding {
        /// The node.
        node_id: CheckedNodeId,
        /// Its family.
        node_tag: &'static str,
    },
    /// A reachable node family awaits upstream semantics.
    BlockedOnUpstream {
        /// The node.
        node_id: CheckedNodeId,
        /// Its family.
        node_tag: &'static str,
        /// The upstream issue.
        issue: UpstreamBlocker,
    },
    /// The node carries no derivable FR-014 descriptor (FR-022), so no oracle was built for it.
    NoDerivableClaim {
        /// The node.
        node_id: CheckedNodeId,
        /// Why derivation refused.
        reason: ClaimDerivationRefusal,
    },
    /// The FR-014 oracle refused for another reason.
    OracleRefused {
        /// The FR-014 refusal.
        refusal: ExactScalarRefusal,
    },
    /// A V2 scalar-claim item's graph node carries a tag/form pair this generator does not model
    /// as a contract role. [`ObligationKind::Frame`] is the only pair `classify_node` recognizes
    /// (`state`/`frame`); every other node reaching this ground is present in the graph -- a node
    /// absent from the graph entirely is refused instead as [`InvalidObligationItem::UnknownNode`]
    /// (see `refuse_unknown_node_kind`), a distinct code from this one -- but would otherwise have
    /// been accounted with a null `kind` and no typed reason naming what was unrecognized.
    /// `generate_exact_scalar_oracles` already refuses every node whose own tag is
    /// not `expression` before it could reach a successful claim, so this ground is a second,
    /// independent gate against the same drift: an IR revision that adds a node kind this
    /// generator has not been taught, or a claim map assembled by another caller, is refused by
    /// name here too rather than depending on that other gate alone.
    UnknownNodeKind {
        /// The unrecognized node.
        node_id: CheckedNodeId,
        /// The node's own family.
        node_tag: String,
        /// The node's own form within that family.
        semantic_form: String,
    },
    /// The single-clause state and frame engine refused the clause for a ground that has no
    /// code of its own here: a condition shape, a frame effect, a clause field's bound, a
    /// malformed clause or a lowering refusal. It is not [`Self::NoFiniteEncoding`], which says
    /// a node family has no finite encoding: the shapes this reason carries have one that this
    /// arm does not render. Serialized as `code: state_frame_refused` with the refusal's own
    /// snake_case code beside it, never the IR record.
    StateFrameRefused {
        /// The engine's refusal.
        refusal: StateFrameRefusal,
    },
}

/// Why an item is not a valid request.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum InvalidObligationItem {
    /// The item repeats an earlier item.
    DuplicateItem {
        /// Index of the first occurrence.
        first_index: usize,
    },
    /// The clause is not an executable clause of the package.
    UnknownClause,
    /// The node has no entry in the claim map, or (see `refuse_unknown_node_kind`) the claim
    /// naming it has no entry in the admitted graph -- the same code
    /// [`ExactScalarRefusal::InvalidInput`] already reports for "the node is not in the admitted
    /// graph" through `classify_claim`'s `Refused` arm.
    UnknownNode,
    /// V1 items name more than one bound package.
    MixedBoundPackages,
    /// The claim map was not generated from the package.
    PackageMismatch,
    /// A `StateFrame` item's state path or subject path is not a Rust path.
    InvalidStatePath {
        /// The offending path.
        path: String,
    },
    /// A `StateFrame` item's state fields hold a name that is not a distinct Rust identifier, or
    /// lack the field its clause reads or its frame grants.
    InvalidStateField {
        /// The offending name.
        name: String,
    },
    /// The unwind bound is outside `1..=MAX_OBLIGATION_UNWIND` where the item's own engine
    /// checks it.
    InvalidStateUnwind {
        /// The offending bound.
        unwind: u32,
    },
    /// `StateFrame` items name more than one admitted package.
    MixedStatePackages,
}

/// Per-item negotiation outcome.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum ObligationDisposition {
    /// One harness is emitted.
    Supported {
        /// Its proof function symbol.
        harness_symbol: HarnessSymbol,
    },
    /// An unbounded type has no bounding domain.
    RequiresBound {
        /// The unbounded type.
        unbounded_type: CheckedNodeId,
    },
    /// No harness is emitted.
    Unsupported {
        /// Why.
        reason: UnsupportedObligation,
    },
    /// The item is invalid; the whole request is rejected.
    InvalidRequest {
        /// Why.
        reason: InvalidObligationItem,
    },
}

/// Accounting for one requested item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ObligationRecord {
    /// Position in the request.
    pub request_index: usize,
    /// Contract role, when the IR states one.
    pub kind: Option<ObligationKind>,
    /// The IR item.
    pub subject: ObligationSubject,
    /// Outcome.
    pub disposition: ObligationDisposition,
}

/// The result of one negotiated request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KaniObligationOutcome {
    /// No item was invalid; one harness per supported record, in request order.
    Emitted {
        /// Every item's record, in request order.
        records: Vec<ObligationRecord>,
        /// V1 frozen-clause harnesses.
        harnesses: Vec<KaniObligationHarness>,
        /// V2 IR-confirmed exact-scalar claim harnesses.
        scalar_harnesses: Vec<KaniScalarObligationHarness>,
        /// State-clause harnesses, one per `supported` `StateFrame` record, in request order.
        state_frame_harnesses: Vec<StateFrameHarness>,
    },
    /// At least one item was invalid; no harness bytes are returned.
    Rejected {
        /// Every item's record, in request order.
        records: Vec<ObligationRecord>,
    },
}

impl KaniObligationOutcome {
    /// Every item's record, in request order.
    #[must_use]
    pub fn records(&self) -> &[ObligationRecord] {
        match self {
            Self::Emitted { records, .. } | Self::Rejected { records } => records,
        }
    }
}

// ---------------------------------------------------------------------------
// The single-clause state and frame engine's refusals, and their one mapping to a record
// ---------------------------------------------------------------------------

/// A frame effect the state and frame engine has no finite encoding for.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedFrameEffect {
    /// The frame creates an object type.
    Creates,
    /// The frame deletes an object type.
    Deletes,
    /// The frame grants a relationship.
    Relationship,
    /// The frame grants a field of a type other than the clause's object.
    ForeignField,
}

/// The six refusal arms of Contract IR's lowering record, and not its `Lowered` arm: a value of
/// this type cannot hold a lowered node, so the mapping to a record has no row for one. Built
/// from IR's record by an exhaustive `match` with no wildcard arm. The fields are IR's own public
/// field types; IR's own refusal and incomplete records are carried for the caller and never
/// serialized, and the limit kind is serialized by name beside `limit` and `consumed`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "arm", rename_all = "snake_case")]
pub enum StateFrameLoweringRefusal {
    /// A reachable node's family is outside the lowering profile.
    Unsupported {
        /// The requested node.
        node_id: CheckedNodeId,
        /// The first unsupported reachable node.
        unsupported_node_id: CheckedNodeId,
        /// Its family.
        #[serde(serialize_with = "serialize_node_tag")]
        node_tag: CheckedNodeTag,
    },
    /// A reachable unbounded type has no reachable bounding domain.
    RequiresBound {
        /// The requested node.
        node_id: CheckedNodeId,
        /// The first unbounded reachable type.
        unbounded_type: CheckedNodeId,
    },
    /// The requested node is not in the admitted graph.
    InvalidInput {
        /// The requested node.
        node_id: CheckedNodeId,
    },
    /// A reachable node body refused re-validation.
    InvalidBody {
        /// The requested node.
        node_id: CheckedNodeId,
        /// The node whose body refused.
        body_node_id: CheckedNodeId,
        /// The term validator's refusal.
        #[serde(skip)]
        refusal: CheckedPackageRefusal,
    },
    /// A reachable node body stopped at a validation limit.
    BodyIncomplete {
        /// The requested node.
        node_id: CheckedNodeId,
        /// The node whose body stopped.
        body_node_id: CheckedNodeId,
        /// The term validator's limit stop.
        #[serde(skip)]
        incomplete: CheckedPackageIncomplete,
    },
    /// The request exceeded its work budget or the byte limit the package was read under.
    Failed {
        /// The requested node.
        node_id: CheckedNodeId,
        /// The limit that failed, serialized by its snake_case name so `limit` and `consumed` say
        /// what they count.
        #[serde(serialize_with = "serialize_limit_kind")]
        limit_kind: CheckedPackageLimit,
        /// The ceiling.
        limit: u64,
        /// The counter at the failed charge.
        consumed: u64,
    },
}

fn serialize_node_tag<S: Serializer>(
    tag: &CheckedNodeTag,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(tag.as_wire())
}

/// The snake_case name of a lowering limit. Exhaustive, so a kind IR adds fails to compile here.
fn serialize_limit_kind<S: Serializer>(
    limit: &CheckedPackageLimit,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(match limit {
        CheckedPackageLimit::Bytes => "bytes",
        CheckedPackageLimit::Nodes => "nodes",
        CheckedPackageLimit::Edges => "edges",
        CheckedPackageLimit::Occurrences => "occurrences",
        CheckedPackageLimit::Diagnostics => "diagnostics",
        CheckedPackageLimit::Work => "work",
    })
}

/// Why the single-clause state and frame engine produced no harness.
///
/// Serialized with the snake_case `code` of the variant and the node, field and effect it names.
/// No IR record is serialized.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum StateFrameRefusal {
    /// The unwind bound is outside `1..=MAX_OBLIGATION_UNWIND`.
    UnwindOutOfRange {
        /// The requested bound.
        unwind: u32,
    },
    /// A path is not a Rust path.
    InvalidPath {
        /// The offending path.
        path: String,
    },
    /// A field name the caller supplied is not a Rust identifier, or is named twice.
    InvalidField {
        /// The offending name.
        name: String,
    },
    /// Lowering did not produce the clause: an unsupported family, a missing bound or an
    /// unknown node.
    NotLowered {
        /// The refusal arm of IR's lowering record, boxed to keep the refusal small.
        refusal: Box<StateFrameLoweringRefusal>,
    },
    /// The node is not a `state`/`state_clause` node.
    NotAStateClause {
        /// The node.
        node: CheckedNodeId,
        /// The node's family.
        node_tag: String,
        /// The node's form.
        semantic_form: String,
    },
    /// The clause is an invariant or a precondition.
    NotAPostcondition {
        /// The clause kind the node names.
        clause: String,
    },
    /// A node does not have the shape QSpec FR-341 and FR-342 fix, a field name read from the
    /// graph is not a Rust identifier, or an operand names a node the graph does not hold.
    MalformedClause {
        /// The node that failed to decode.
        at: CheckedNodeId,
    },
    /// The condition is not one integer comparison of pre and post reads of one field.
    ConditionNotSupported {
        /// The node that is outside the supported shape.
        at: CheckedNodeId,
    },
    /// The two sides of the comparison read different fields.
    ObservationsDiffer {
        /// The left operand's field.
        left: String,
        /// The right operand's field.
        right: String,
    },
    /// Both sides of the comparison observe the same side of the operation.
    ObservationsSameSide,
    /// The frame has an effect with no finite encoding.
    FrameEffectUnsupported {
        /// The frame node.
        frame: CheckedNodeId,
        /// The effect.
        effect: UnsupportedFrameEffect,
    },
    /// The framed object's member for the clause's field declares no `i64` integer range.
    BoundNotResolved {
        /// The clause's field.
        field: String,
        /// Which ground, with the node it names when the member has one.
        cause: BoundNotResolvedCause,
    },
    /// A field the clause reads or the frame grants is not in the caller's state fields.
    UnknownStateField {
        /// The field.
        field: String,
    },
    /// The frame grants every state field, so it forbids no effect and there is nothing to
    /// assert.
    NothingForbidden {
        /// The frame node.
        frame: CheckedNodeId,
    },
    /// The generated source exceeds
    /// [`MAX_GENERATED_SOURCE_BYTES`](crate::core::artifact::MAX_GENERATED_SOURCE_BYTES).
    ResourceLimitExceeded {
        /// The generated size.
        bytes: usize,
    },
    /// The generated source does not parse as Rust.
    InvalidGeneratedSyntax {
        /// The parse error.
        error: String,
    },
    /// The identity record failed to serialize.
    RecordSerialization,
}

/// Why the framed object gives the clause's field no `i64` integer range: the one split of
/// [`StateFrameRefusal::BoundNotResolved`] that the mapping to a record keys on.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "ground", rename_all = "snake_case")]
pub enum BoundNotResolvedCause {
    /// The model field table could not be read for the declared object.
    ModelFieldsUnavailable {
        /// The model declaration object.
        object: CheckedNodeId,
        /// The accessor's exact error.
        #[serde(serialize_with = "serialize_model_fields_error")]
        error: CheckedModelFieldsError,
    },
    /// A present model field does not have a representable `i64` range.
    ModelMemberNotI64Range {
        /// The model declaration object.
        object: CheckedNodeId,
        /// The field's name.
        field: String,
        /// Why its member type is not a representable range.
        reason: ModelMemberRangeReason,
    },
    /// The member's `value.target` is an unbounded type, a node that is not a bound.
    UnboundedType {
        /// That target node.
        target: CheckedNodeId,
    },
    /// The member's `value.target` is a bound that is not a readable `integer_range`.
    NotIntegerRange {
        /// The bound node.
        bound: CheckedNodeId,
    },
    /// The member's `value.target` is an `integer_range` with an endpoint that does not fit `i64`.
    EndpointOutsideI64 {
        /// The bound node.
        bound: CheckedNodeId,
    },
    /// The object has no member of the field's name.
    MemberAbsent {
        /// The framed object whose field is missing.
        object: CheckedNodeId,
    },
    /// The member's value is not a reference, so it has no `value.target`.
    ValueNotReference,
}

fn serialize_model_fields_error<S: Serializer>(
    error: &CheckedModelFieldsError,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    #[derive(Serialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum Wire<'a> {
        UnknownNode,
        NotModelObjectType,
        AmbiguousField { name: &'a str },
    }
    match error {
        CheckedModelFieldsError::UnknownNode => Wire::UnknownNode,
        CheckedModelFieldsError::NotModelObjectType => Wire::NotModelObjectType,
        CheckedModelFieldsError::AmbiguousField(name) => Wire::AmbiguousField { name },
    }
    .serialize(serializer)
}

/// Why a present model field has no `i64` range.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum ModelMemberRangeReason {
    /// No checked member type was derived.
    NoMemberType,
    /// The checked member type is not an integer range.
    NonRangeType,
    /// The inclusive `i128` endpoints cannot both fit `i64`.
    EndpointOutsideI64 {
        /// Inclusive lower endpoint.
        lower: i128,
        /// Inclusive upper endpoint.
        upper: i128,
    },
}

impl std::fmt::Display for StateFrameRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnwindOutOfRange { unwind } => {
                write!(
                    formatter,
                    "unwind bound {unwind} is outside 1..={MAX_OBLIGATION_UNWIND}"
                )
            }
            Self::InvalidPath { path } => write!(formatter, "`{path}` is not a Rust path"),
            Self::InvalidField { name } => {
                write!(
                    formatter,
                    "`{name}` is not a distinct Rust field identifier"
                )
            }
            Self::NotLowered { refusal } => {
                write!(formatter, "the clause did not lower: {refusal:?}")
            }
            Self::NotAStateClause {
                node_tag,
                semantic_form,
                ..
            } => {
                write!(
                    formatter,
                    "node is `{node_tag}`/`{semantic_form}`, not state/state_clause"
                )
            }
            Self::NotAPostcondition { clause } => {
                write!(formatter, "clause kind `{clause}` is not a postcondition")
            }
            Self::MalformedClause { at } => write!(formatter, "malformed clause at {}", at.digest),
            Self::ConditionNotSupported { at } => {
                write!(
                    formatter,
                    "condition is outside the supported shape at {}",
                    at.digest
                )
            }
            Self::ObservationsDiffer { left, right } => {
                write!(
                    formatter,
                    "the condition reads `{left}` and `{right}`, not one field"
                )
            }
            Self::ObservationsSameSide => {
                formatter.write_str("the condition compares two reads of the same side")
            }
            Self::FrameEffectUnsupported { frame, effect } => {
                write!(
                    formatter,
                    "frame {} has unsupported effect {effect:?}",
                    frame.digest
                )
            }
            Self::BoundNotResolved { field, cause } => {
                write!(
                    formatter,
                    "the object declares no i64 integer range for `{field}`: {cause:?}"
                )
            }
            Self::UnknownStateField { field } => {
                write!(formatter, "`{field}` is not a declared state field")
            }
            Self::NothingForbidden { frame } => {
                write!(formatter, "frame {} grants every state field", frame.digest)
            }
            Self::ResourceLimitExceeded { bytes } => {
                write!(
                    formatter,
                    "generated source of {bytes} bytes exceeds the ceiling"
                )
            }
            Self::InvalidGeneratedSyntax { error } => {
                write!(formatter, "generated source does not parse: {error}")
            }
            Self::RecordSerialization => {
                formatter.write_str("the identity record did not serialize")
            }
        }
    }
}

impl std::error::Error for StateFrameRefusal {}

/// The disposition and reason a refusal of the single-clause engine takes as a record (FR-015,
/// the refusal-to-record table).
///
/// One total `match` over every [`StateFrameRefusal`] variant and every [`BoundNotResolvedCause`]
/// and [`StateFrameLoweringRefusal`] arm, with no wildcard arm, so a variant added to any of them
/// fails to compile until the table says where it goes. The `deny` below makes a wildcard arm a
/// lint error under `make lint`.
#[deny(
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants
)]
pub(crate) fn state_frame_disposition(refusal: StateFrameRefusal) -> ObligationDisposition {
    use BoundNotResolvedCause as Cause;
    use StateFrameLoweringRefusal as Lowering;
    let refused = |refusal| ObligationDisposition::Unsupported {
        reason: UnsupportedObligation::StateFrameRefused { refusal },
    };
    let unsupported = |reason| ObligationDisposition::Unsupported { reason };
    let invalid = |reason| ObligationDisposition::InvalidRequest { reason };
    match refusal {
        // The lowering refusal is boxed, so it is read through a reference; the two arms that
        // keep a node clone it, and the arms that carry the refusal keep the box whole.
        StateFrameRefusal::NotLowered {
            refusal: ref lowering,
        } => match &**lowering {
            Lowering::RequiresBound { unbounded_type, .. } => {
                ObligationDisposition::RequiresBound {
                    unbounded_type: unbounded_type.clone(),
                }
            }
            Lowering::Unsupported {
                unsupported_node_id,
                node_tag,
                ..
            } => unsupported(UnsupportedObligation::NoFiniteEncoding {
                node_id: unsupported_node_id.clone(),
                node_tag: node_tag.as_wire(),
            }),
            Lowering::InvalidInput { .. } => invalid(InvalidObligationItem::UnknownNode),
            Lowering::InvalidBody { .. }
            | Lowering::BodyIncomplete { .. }
            | Lowering::Failed { .. } => refused(refusal),
        },
        StateFrameRefusal::BoundNotResolved {
            cause: Cause::UnboundedType {
                target: unbounded_type,
            },
            ..
        } => ObligationDisposition::RequiresBound { unbounded_type },
        StateFrameRefusal::NotAStateClause {
            node,
            node_tag,
            semantic_form,
        } => unsupported(UnsupportedObligation::UnknownNodeKind {
            node_id: node,
            node_tag,
            semantic_form,
        }),
        StateFrameRefusal::BoundNotResolved {
            cause:
                Cause::NotIntegerRange { .. }
                | Cause::EndpointOutsideI64 { .. }
                | Cause::MemberAbsent { .. }
                | Cause::ValueNotReference
                | Cause::ModelFieldsUnavailable { .. }
                | Cause::ModelMemberNotI64Range { .. },
            ..
        }
        | StateFrameRefusal::NotAPostcondition { .. }
        | StateFrameRefusal::MalformedClause { .. }
        | StateFrameRefusal::ConditionNotSupported { .. }
        | StateFrameRefusal::ObservationsDiffer { .. }
        | StateFrameRefusal::ObservationsSameSide
        | StateFrameRefusal::FrameEffectUnsupported { .. }
        | StateFrameRefusal::NothingForbidden { .. } => refused(refusal),
        StateFrameRefusal::ResourceLimitExceeded { bytes } => {
            unsupported(UnsupportedObligation::ResourceLimitExceeded { bytes })
        }
        StateFrameRefusal::InvalidGeneratedSyntax { error } => {
            unsupported(UnsupportedObligation::InvalidGeneratedSyntax { error })
        }
        StateFrameRefusal::RecordSerialization => unsupported(UnsupportedObligation::RenderFailed),
        StateFrameRefusal::InvalidPath { path } => {
            invalid(InvalidObligationItem::InvalidStatePath { path })
        }
        StateFrameRefusal::InvalidField { name: field }
        | StateFrameRefusal::UnknownStateField { field } => {
            invalid(InvalidObligationItem::InvalidStateField { name: field })
        }
        StateFrameRefusal::UnwindOutOfRange { unwind } => {
            invalid(InvalidObligationItem::InvalidStateUnwind { unwind })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(seed: u8) -> CheckedNodeId {
        CheckedNodeId {
            domain: "quire.checked-semantic-node/v1".into(),
            digest: format!("{seed:064x}").into(),
        }
    }

    fn unsupported(reason: UnsupportedObligation) -> ObligationDisposition {
        ObligationDisposition::Unsupported { reason }
    }

    fn refused(refusal: StateFrameRefusal) -> ObligationDisposition {
        unsupported(UnsupportedObligation::StateFrameRefused { refusal })
    }

    fn invalid(reason: InvalidObligationItem) -> ObligationDisposition {
        ObligationDisposition::InvalidRequest { reason }
    }

    fn not_lowered(refusal: StateFrameLoweringRefusal) -> StateFrameRefusal {
        StateFrameRefusal::NotLowered {
            refusal: Box::new(refusal),
        }
    }

    fn unbound(cause: BoundNotResolvedCause) -> StateFrameRefusal {
        StateFrameRefusal::BoundNotResolved {
            field: "balance".to_owned(),
            cause,
        }
    }

    /// Every refusal the engine returns, with the disposition and reason the FR-015 table gives
    /// it, built directly: each `BoundNotResolved` ground the table separates, each of the six
    /// refusal arms of `NotLowered`, and the variants a request reaches only with a defective
    /// generator or a source over 1 MiB.
    fn table() -> Vec<(&'static str, StateFrameRefusal, ObligationDisposition)> {
        let refusal_detail = CheckedPackageRefusal {
            code: quire_contract_model::CheckedPackageRefusalCode::MalformedWire,
            path: None,
            cause: None,
            locus: None,
            contract_version: None,
            document_pointer: None,
        };
        let incomplete = CheckedPackageIncomplete {
            limit_kind: CheckedPackageLimit::Work,
            limit: 10,
            consumed: 11,
            path: None,
        };
        let same = |label, refusal: StateFrameRefusal| (label, refusal.clone(), refused(refusal));
        vec![
            (
                "unwind out of range",
                StateFrameRefusal::UnwindOutOfRange { unwind: 0 },
                invalid(InvalidObligationItem::InvalidStateUnwind { unwind: 0 }),
            ),
            (
                "invalid path",
                StateFrameRefusal::InvalidPath {
                    path: "not a path".to_owned(),
                },
                invalid(InvalidObligationItem::InvalidStatePath {
                    path: "not a path".to_owned(),
                }),
            ),
            (
                "invalid field",
                StateFrameRefusal::InvalidField {
                    name: "bad name".to_owned(),
                },
                invalid(InvalidObligationItem::InvalidStateField {
                    name: "bad name".to_owned(),
                }),
            ),
            (
                "unknown state field",
                StateFrameRefusal::UnknownStateField {
                    field: "audit".to_owned(),
                },
                invalid(InvalidObligationItem::InvalidStateField {
                    name: "audit".to_owned(),
                }),
            ),
            (
                "not lowered: requires bound",
                not_lowered(StateFrameLoweringRefusal::RequiresBound {
                    node_id: node(1),
                    unbounded_type: node(2),
                }),
                ObligationDisposition::RequiresBound {
                    unbounded_type: node(2),
                },
            ),
            (
                "not lowered: unsupported",
                not_lowered(StateFrameLoweringRefusal::Unsupported {
                    node_id: node(1),
                    unsupported_node_id: node(3),
                    node_tag: CheckedNodeTag::Temporal,
                }),
                unsupported(UnsupportedObligation::NoFiniteEncoding {
                    node_id: node(3),
                    node_tag: "temporal",
                }),
            ),
            (
                "not lowered: invalid input",
                not_lowered(StateFrameLoweringRefusal::InvalidInput { node_id: node(1) }),
                invalid(InvalidObligationItem::UnknownNode),
            ),
            same(
                "not lowered: invalid body",
                not_lowered(StateFrameLoweringRefusal::InvalidBody {
                    node_id: node(1),
                    body_node_id: node(4),
                    refusal: refusal_detail,
                }),
            ),
            same(
                "not lowered: body incomplete",
                not_lowered(StateFrameLoweringRefusal::BodyIncomplete {
                    node_id: node(1),
                    body_node_id: node(4),
                    incomplete,
                }),
            ),
            same(
                "not lowered: failed",
                not_lowered(StateFrameLoweringRefusal::Failed {
                    node_id: node(1),
                    limit_kind: CheckedPackageLimit::Work,
                    limit: 10,
                    consumed: 11,
                }),
            ),
            (
                "not a state clause",
                StateFrameRefusal::NotAStateClause {
                    node: node(5),
                    node_tag: "state".to_owned(),
                    semantic_form: "frame".to_owned(),
                },
                unsupported(UnsupportedObligation::UnknownNodeKind {
                    node_id: node(5),
                    node_tag: "state".to_owned(),
                    semantic_form: "frame".to_owned(),
                }),
            ),
            same(
                "not a postcondition",
                StateFrameRefusal::NotAPostcondition {
                    clause: "precondition".to_owned(),
                },
            ),
            same(
                "malformed clause",
                StateFrameRefusal::MalformedClause { at: node(6) },
            ),
            same(
                "condition not supported",
                StateFrameRefusal::ConditionNotSupported { at: node(7) },
            ),
            same(
                "observations differ",
                StateFrameRefusal::ObservationsDiffer {
                    left: "balance".to_owned(),
                    right: "audit".to_owned(),
                },
            ),
            same(
                "observations same side",
                StateFrameRefusal::ObservationsSameSide,
            ),
            same(
                "frame creates",
                StateFrameRefusal::FrameEffectUnsupported {
                    frame: node(8),
                    effect: UnsupportedFrameEffect::Creates,
                },
            ),
            same(
                "frame deletes",
                StateFrameRefusal::FrameEffectUnsupported {
                    frame: node(8),
                    effect: UnsupportedFrameEffect::Deletes,
                },
            ),
            same(
                "frame grants a relationship",
                StateFrameRefusal::FrameEffectUnsupported {
                    frame: node(8),
                    effect: UnsupportedFrameEffect::Relationship,
                },
            ),
            same(
                "frame grants a foreign field",
                StateFrameRefusal::FrameEffectUnsupported {
                    frame: node(8),
                    effect: UnsupportedFrameEffect::ForeignField,
                },
            ),
            (
                "bound not resolved: unbounded type",
                unbound(BoundNotResolvedCause::UnboundedType { target: node(9) }),
                ObligationDisposition::RequiresBound {
                    unbounded_type: node(9),
                },
            ),
            same(
                "bound not resolved: not an integer range",
                unbound(BoundNotResolvedCause::NotIntegerRange { bound: node(9) }),
            ),
            same(
                "bound not resolved: endpoint outside i64",
                unbound(BoundNotResolvedCause::EndpointOutsideI64 { bound: node(9) }),
            ),
            same(
                "bound not resolved: member absent",
                unbound(BoundNotResolvedCause::MemberAbsent { object: node(9) }),
            ),
            same(
                "bound not resolved: value not a reference",
                unbound(BoundNotResolvedCause::ValueNotReference),
            ),
            same(
                "nothing forbidden",
                StateFrameRefusal::NothingForbidden { frame: node(8) },
            ),
            (
                "resource limit exceeded",
                StateFrameRefusal::ResourceLimitExceeded { bytes: 1_048_577 },
                unsupported(UnsupportedObligation::ResourceLimitExceeded { bytes: 1_048_577 }),
            ),
            (
                "invalid generated syntax",
                StateFrameRefusal::InvalidGeneratedSyntax {
                    error: "expected item".to_owned(),
                },
                unsupported(UnsupportedObligation::InvalidGeneratedSyntax {
                    error: "expected item".to_owned(),
                }),
            ),
            (
                "record serialization",
                StateFrameRefusal::RecordSerialization,
                unsupported(UnsupportedObligation::RenderFailed),
            ),
        ]
    }

    /// Every `StateFrameRefusal` variant, each `BoundNotResolved` ground and each `NotLowered`
    /// arm maps to the disposition and reason of the table; a mutant that moves any one to
    /// another disposition or reason fails its row. The mapping is one `match` with no wildcard
    /// arm: `deny(clippy::wildcard_enum_match_arm)` rejects one under `make lint`, and the
    /// inspection that follows this test parses the function.
    ///
    /// Trace: FR-015-AC-66, TC-025.
    #[test]
    fn tc_025_every_state_frame_refusal_maps_to_the_disposition_the_table_gives() {
        let rows = table();
        // 17 variants, with `NotLowered` split into its 6 arms (+5), `BoundNotResolved` into its
        // 5 grounds (+4) and `FrameEffectUnsupported` into its 4 effects (+3).
        assert_eq!(rows.len(), 17 + 5 + 4 + 3);
        for (label, refusal, expected) in rows {
            assert_eq!(state_frame_disposition(refusal), expected, "{label}");
        }

        // The reason serializes as `state_frame_refused` with the refusal's snake_case code and
        // the effect it names, and no IR record.
        let serialized = serde_json::to_value(refused(StateFrameRefusal::FrameEffectUnsupported {
            frame: node(8),
            effect: UnsupportedFrameEffect::ForeignField,
        }))
        .expect("the reason serializes");
        assert_eq!(
            serialized,
            serde_json::json!({
                "disposition": "unsupported",
                "reason": {
                    "code": "state_frame_refused",
                    "refusal": {
                        "code": "frame_effect_unsupported",
                        "frame": {"domain": "quire.checked-semantic-node/v1", "digest": node(8).digest},
                        "effect": "foreign_field",
                    },
                },
            })
        );
        let lowering =
            serde_json::to_value(refused(not_lowered(StateFrameLoweringRefusal::Failed {
                node_id: node(1),
                limit_kind: CheckedPackageLimit::Work,
                limit: 10,
                consumed: 11,
            })))
            .expect("the reason serializes");
        assert_eq!(
            lowering["reason"]["refusal"],
            serde_json::json!({
                "code": "not_lowered",
                "refusal": {
                    "arm": "failed",
                    "node_id": {"domain": "quire.checked-semantic-node/v1", "digest": node(1).digest},
                    "limit_kind": "work",
                    "limit": 10,
                    "consumed": 11,
                },
            })
        );
    }

    /// Whether a pattern matches everything its scrutinee holds: `_`, a bare binding, or an
    /// alternative holding either.
    fn is_catch_all(pattern: &syn::Pat) -> bool {
        match pattern {
            syn::Pat::Wild(_) => true,
            syn::Pat::Ident(ident) => ident.subpat.is_none(),
            syn::Pat::Or(alternatives) => alternatives.cases.iter().any(is_catch_all),
            _ => false,
        }
    }

    /// The patterns of every match arm under a function, however nested.
    #[derive(Default)]
    struct ArmPatterns(Vec<syn::Pat>);

    impl<'ast> syn::visit::Visit<'ast> for ArmPatterns {
        fn visit_arm(&mut self, arm: &'ast syn::Arm) {
            self.0.push(arm.pat.clone());
            syn::visit::visit_arm(self, arm);
        }
    }

    /// The mapping is a `match` with no wildcard arm: the function is parsed, carries
    /// `deny(clippy::wildcard_enum_match_arm)`, and none of its arms, at any depth, is `_` or a
    /// bare binding. A mutant that drops the `deny`, or that folds explicit patterns into
    /// `other => ...`, fails here.
    ///
    /// Trace: FR-015-AC-66, TC-025.
    #[test]
    fn tc_025_the_state_frame_mapping_has_no_wildcard_or_binding_arm() {
        use syn::visit::Visit;
        let file = syn::parse_file(include_str!("outcome.rs")).expect("this file parses");
        let function = file
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Fn(function) if function.sig.ident == "state_frame_disposition" => {
                    Some(function)
                }
                _ => None,
            })
            .expect("the mapping is in this file");
        let denied = function
            .attrs
            .iter()
            .filter(|attribute| attribute.path().is_ident("deny"))
            .filter_map(|attribute| match &attribute.meta {
                syn::Meta::List(list) => Some(list.tokens.to_string()),
                syn::Meta::Path(_) | syn::Meta::NameValue(_) => None,
            })
            .collect::<String>();
        assert!(
            denied.contains("wildcard_enum_match_arm"),
            "the mapping must deny a wildcard arm: {denied:?}"
        );
        let mut arms = ArmPatterns::default();
        arms.visit_item_fn(function);
        assert!(arms.0.len() >= 14, "the visitor must see the arms");
        assert!(
            !arms.0.iter().any(is_catch_all),
            "the mapping has a wildcard or binding arm"
        );
    }
}

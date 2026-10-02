//! The request and result vocabulary of Kani obligation negotiation (FR-015): the items a
//! request names, the records it returns and the reasons an item has no harness. A leaf inside
//! `generate/`: it imports no family.

use quire_contract_model::{
    BoundPackage, CheckedNodeId, CheckedPackageV2, CheckedSourceMapEntry, ClauseKind, ClauseRef,
    DependencyIdentity, SourceSpan,
};
use serde::Serialize;

use crate::{
    core::diagnostic::GenerationErrorCode,
    core::identity::HarnessSymbol,
    kani::identity::{KaniObligationHarness, KaniScalarObligationHarness, ObligationKind},
    oracle::claim::{ClaimMap, UpstreamBlocker},
    oracle::scalar::{ClaimDerivationRefusal, ExactScalarClaim, ExactScalarRefusal},
};

/// Largest number of items one request may negotiate.
pub const MAX_OBLIGATION_ITEMS: usize = 256;

/// Largest accepted loop unwind bound.
pub const MAX_OBLIGATION_UNWIND: u32 = 1024;

/// One item to negotiate.
#[derive(Clone, Copy, Debug)]
pub enum ObligationItem<'a> {
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
    /// [`crate::generate_state_frame_obligations`].
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
    /// Known narrowing: the analogous [`crate::KaniErrorCode::InvalidGeneratedSyntax`] is
    /// classified `Inconclusive` by its `terminal_state` (a generator defect, not an honest
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
    /// for its family yet (today: every family but `IntegerArithmetic`).
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

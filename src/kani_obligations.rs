//! Separate bounded Kani obligations with per-item backend negotiation (FR-015).
//!
//! Every requested item is accounted for before any harness bytes are exposed: each receives
//! exactly one [`ObligationRecord`] whose disposition is `supported`, `requires_bound`,
//! `unsupported` or `invalid_request` (Contract IR FR-036). One `invalid_request` rejects the
//! whole request and no harness is returned. Otherwise one harness is returned per supported
//! item and never one for any other disposition.
//!
//! Each supported item is one IR clause lowered to one obligation — a precondition, postcondition
//! or invariant is never combined with another into a single proof. Symbolic inputs are
//! `kani::any()` values constrained only by `kani::assume` of the inclusive integer bounds the IR
//! declares for that dependency; no other assumption is emitted. Preconditions a postcondition or
//! invariant relies on are named by the IR anchor they share, must themselves be supported items
//! of the same request, and are recorded in the harness identity.
//!
//! Operation identity must be exact. Frozen V1 bound clauses carry their operators as IR enum
//! variants and are lowered. A CheckedPackage V2 scalar claim whose node this generator
//! successfully lowered and checked, and whose node's own catalogued `operation.identity`,
//! `operation.mode` and (for the one family with more than one catalogued law definition)
//! `operation.laws` all confirm the request item's own descriptor, is [`OperationProvenance::IrConfirmed`]
//! and -- for the `IntegerArithmetic` families this generator has a Kani renderer for
//! (`quire.op.integer.{add,sub,mul,negate}`) -- reaches a real Kani harness via
//! [`Outcome::LoweredScalar`]/`render_scalar`. Every other confirmed family is honestly refused as
//! [`UnsupportedObligation::OperationNotRendered`], never silently mis-rendered. A claim that
//! codegen generated but whose operation it did not confirm reports the request item's own
//! descriptor-derived identity as `CallerDeclared` and, unless a ground that does not depend on
//! the operation displaces that refusal -- an unsatisfiable bound it names or a mismatched
//! package, each of which precedes it, or a duplicate item, which overwrites it afterwards --
//! is refused here as
//! [`UnsupportedObligation::CallerDeclaredOperation`] with the domains the IR does carry. That is
//! the only case of [`crate::OperationProvenance::CallerDeclared`] this generator refuses under
//! that name: `CallerDeclaredOperation` is constructed once, inside the `Generated` arm of
//! `classify_claim`, so a claim codegen refused outright is classified through the `Refused` arm
//! instead and never reaches it. V1 has no
//! frame clause kind, and V2 frames have no finite encoding in the scalar profile, so this
//! negotiation emits no frame harness; `generate_state_frame_obligations` generates them from a
//! state clause. A V2 scalar claim's graph node that is present but is neither the one
//! recognized `state`/`frame` pair nor `expression`-tagged (the only family this generator ever
//! lowers to a `Generated` claim) is refused as [`UnsupportedObligation::UnknownNodeKind`] rather
//! than accounted with a null contract role and no typed reason; a claim naming a `node_id`
//! absent from the graph entirely is refused the same way as [`InvalidObligationItem::UnknownNode`]
//! (see `refuse_unknown_node_kind`).
//!
//! A render whose generated source would exceed [`crate::MAX_GENERATED_SOURCE_BYTES`] is refused
//! as [`UnsupportedObligation::ResourceLimitExceeded`], and one that fits that ceiling but fails
//! `syn::parse_file` is refused as [`UnsupportedObligation::InvalidGeneratedSyntax`]; the two
//! grounds are checked in that order and are never conflated with the internal-invariant
//! [`UnsupportedObligation::RenderFailed`] fallback.

use std::collections::{BTreeMap, BTreeSet};

use quire_contract_model::{
    BoundClause, BoundPackage, CheckedNodeId, CheckedNodeTag, CheckedPackageV2,
    CheckedSemanticNodeV2, CheckedSourceMapEntry, ClauseKind, ClauseRef, DependencyIdentity,
    DependencyKind, ExecutionPoint, SourceSpan, StateObservation,
};
use serde::Serialize;

use crate::{
    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},
    core::diagnostic::GenerationErrorCode,
    core::identity::{HarnessPath, HarnessSymbol, ModuleSymbol, SymbolError},
    core::naming::{reference_identifier, unique_names},
    generate_boolean_oracle,
    kani::{
        adapter_options, i64_literal, readable_component, KaniBindingRole, KaniIntegerBounds,
        KaniPrimitiveType, KaniSolver,
    },
    kani_identity::{
        EmbeddedOracle, KaniObligationHarness, KaniObligationIdentity, KaniScalarObligationHarness,
        ObligationBinding, ObligationKind, ScalarObligationArgument, ScalarObligationIdentity,
    },
    oracle::boolean_v1::{
        generate_named_boolean_oracle, typed_dependency_parameters, DependencyParameter,
        RustValueType,
    },
    oracle::scalar::{
        aggregate_members, bound_members, literal_count, literal_integer, operand_ranges,
        OperandRange, COLLECTION_BOUNDS_MEMBERS, INTEGER_RANGE_MEMBERS, TEXT_BOUNDS_MEMBERS,
    },
    ClaimDerivationRefusal, ClaimDisposition, ClaimMap, ExactScalarClaim, ExactScalarRefusal,
    GeneratedScalarClaim, OperationProvenance, OracleRequest, UpstreamBlocker,
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
    /// The generated harness source exceeds [`MAX_GENERATED_SOURCE_BYTES`], the same
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

/// Negotiates every item and, when no item is invalid, emits one harness per supported item.
///
/// Trace: TC-025
// Implements: FR-015
pub fn negotiate_kani_obligations(
    request: &KaniObligationRequest<'_>,
) -> Result<KaniObligationOutcome, KaniObligationError> {
    validate_request(request)?;
    let mut states = request.items.iter().map(classify).collect::<Vec<_>>();
    reject_duplicates_and_mixtures(request.items, &mut states);
    assign_names(&mut states);
    resolve_assumptions(&mut states);
    let rejected = states
        .iter()
        .any(|state| matches!(state.outcome, Outcome::Invalid(_)));
    let mut records = Vec::with_capacity(states.len());
    let mut harnesses = Vec::new();
    let mut scalar_harnesses = Vec::new();
    for (index, state) in states.into_iter().enumerate() {
        let disposition = if rejected {
            state.outcome.disposition_without_harness()
        } else {
            match state.outcome {
                Outcome::Lowered(lowered) => match render(request, &lowered) {
                    Ok(harness) => {
                        let symbol = harness.identity.harness_symbol.clone();
                        harnesses.push(harness);
                        ObligationDisposition::Supported {
                            harness_symbol: symbol,
                        }
                    }
                    Err(reason) => ObligationDisposition::Unsupported { reason },
                },
                Outcome::LoweredScalar(lowered) => match render_scalar(request, &lowered) {
                    Ok(harness) => {
                        let symbol = harness.identity.harness_symbol.clone();
                        scalar_harnesses.push(harness);
                        ObligationDisposition::Supported {
                            harness_symbol: symbol,
                        }
                    }
                    Err(reason) => ObligationDisposition::Unsupported { reason },
                },
                other => other.disposition_without_harness(),
            }
        };
        records.push(ObligationRecord {
            request_index: index,
            kind: state.kind,
            subject: state.subject,
            disposition,
        });
    }
    Ok(if rejected {
        KaniObligationOutcome::Rejected { records }
    } else {
        KaniObligationOutcome::Emitted {
            records,
            harnesses,
            scalar_harnesses,
        }
    })
}

fn validate_request(request: &KaniObligationRequest<'_>) -> Result<(), KaniObligationError> {
    if request.items.is_empty() {
        return Err(KaniObligationError::EmptyRequest);
    }
    if request.items.len() > MAX_OBLIGATION_ITEMS {
        return Err(KaniObligationError::TooManyItems {
            count: request.items.len(),
        });
    }
    if syn::parse_str::<syn::Path>(request.subject_path).is_err() {
        return Err(KaniObligationError::InvalidSubjectPath);
    }
    if request.unwind == 0 || request.unwind > MAX_OBLIGATION_UNWIND {
        return Err(KaniObligationError::InvalidUnwind {
            unwind: request.unwind,
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Negotiation
// ---------------------------------------------------------------------------

struct ItemState<'a> {
    kind: Option<ObligationKind>,
    subject: ObligationSubject,
    identity: Option<ItemIdentity>,
    outcome: Outcome<'a>,
}

#[derive(Clone, Eq, PartialEq)]
enum ItemIdentity {
    Clause {
        package: String,
        clause: ClauseRef,
    },
    Node {
        package: String,
        node: CheckedNodeId,
    },
}

enum Outcome<'a> {
    Lowered(Box<LoweredClause<'a>>),
    LoweredScalar(Box<LoweredScalarClaim>),
    RequiresBound(CheckedNodeId),
    Unsupported(UnsupportedObligation),
    Invalid(InvalidObligationItem),
}

impl Outcome<'_> {
    fn disposition_without_harness(self) -> ObligationDisposition {
        match self {
            // A lowered item in a rejected request is accounted but not emitted.
            Self::Lowered(lowered) => supported_without_harness(&lowered.symbols.harness),
            Self::LoweredScalar(lowered) => supported_without_harness(&lowered.harness_symbol),
            Self::RequiresBound(unbounded_type) => {
                ObligationDisposition::RequiresBound { unbounded_type }
            }
            Self::Unsupported(reason) => ObligationDisposition::Unsupported { reason },
            Self::Invalid(reason) => ObligationDisposition::InvalidRequest { reason },
        }
    }
}

/// The disposition of a lowered item whose request was rejected: accounted, not emitted.
fn supported_without_harness(harness: &str) -> ObligationDisposition {
    match HarnessSymbol::try_from(harness) {
        Ok(harness_symbol) => ObligationDisposition::Supported { harness_symbol },
        Err(error) => ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::InvalidGeneratedSyntax {
                error: error.to_string(),
            },
        },
    }
}

/// An IR-confirmed V2 exact-scalar claim this generator knows how to render a Kani harness for.
/// Scoped today to `IntegerArithmetic` (`quire.op.integer.{add,sub,mul,negate}`): its operands and
/// result are all `rt::Integer`, and `quire_contract_runtime::exact::Integer: From<i64>` gives a
/// direct `kani::any()` bridge with no construction of a compound runtime type. Every other
/// confirmed family (division's `QuotientRemainder`, rational, decimal, IEEE, text, enum, quantity)
/// has no renderer yet -- see [`lower_scalar_claim`].
struct LoweredScalarClaim {
    node_id: CheckedNodeId,
    operation_identity: String,
    /// The oracle's own self-contained source (`GeneratedScalarClaim::oracle_source`), embedded
    /// verbatim ahead of the harness module, exactly as a V1 harness embeds its `ClauseOracle`s.
    oracle_source: String,
    oracle_symbol: String,
    operation: ScalarOperation,
    /// Each operand's inclusive range, in call order.
    operands: Vec<(i64, i64)>,
    /// The result's inclusive range: the checked domain a completed result must lie in.
    lower: i64,
    upper: i64,
    module_symbol: String,
    harness_symbol: String,
}

/// One integer operation this generator renders a scalar harness for. Everything that differs
/// between the four -- the catalogued identity, the operand names, the exact result, the range of
/// reachable results -- is an exhaustive method here, so adding an operation is one new variant the
/// compiler checks at every use.
#[derive(Clone, Copy)]
enum ScalarOperation {
    Add,
    Subtract,
    Multiply,
    Negate,
}

impl ScalarOperation {
    /// The renderable operation for an IR-confirmed operation identity, or `None` when this
    /// generator has no scalar-harness renderer for it. Recognizing these four exact catalogued
    /// identities is not a vocabulary bridge: it only recognizes, among identities IR already
    /// confirmed, which ones this generator additionally knows how to turn into a harness.
    fn of(operation_identity: &str) -> Option<Self> {
        match operation_identity {
            "quire.op.integer.add" => Some(Self::Add),
            "quire.op.integer.sub" => Some(Self::Subtract),
            "quire.op.integer.mul" => Some(Self::Multiply),
            "quire.op.integer.negate" => Some(Self::Negate),
            _ => None,
        }
    }

    /// The oracle's parameter names, in call order, which are also the harness's variable names.
    fn operand_names(self) -> &'static [&'static str] {
        match self {
            Self::Negate => &["operand"],
            Self::Add | Self::Subtract | Self::Multiply => &["left", "right"],
        }
    }

    /// The exact result as a Rust expression over the harness's native `i64` operands
    /// (`<name>_native`), evaluated in `i128` where a sum, difference, product or negation of `i64`
    /// values cannot overflow. It shares no code with the embedded oracle, which evaluates through
    /// `quire_contract_runtime`'s `rt::Integer`; it is the independent statement of the clause that
    /// [`render_scalar`] compares the oracle's result with.
    fn native_expression(self) -> String {
        let native = |index: usize| format!("i128::from({}_native)", self.operand_names()[index]);
        match self {
            Self::Add => format!("{} + {}", native(0), native(1)),
            Self::Subtract => format!("{} - {}", native(0), native(1)),
            Self::Multiply => format!("{} * {}", native(0), native(1)),
            Self::Negate => format!("-{}", native(0)),
        }
    }

    /// The least and greatest exact results over inclusive `i64` operand ranges, one per operand.
    /// Each is exact in `i128`. The extremes of `+`, `-` and unary `-` lie at the range endpoints,
    /// and so do those of `*`, which is bilinear.
    fn reachable(self, operands: &[(i64, i64)]) -> (i128, i128) {
        let wide = |(low, high): (i64, i64)| (i128::from(low), i128::from(high));
        match (self, operands) {
            (Self::Negate, [operand]) => {
                let (low, high) = wide(*operand);
                (-high, -low)
            }
            (Self::Add, [left, right]) => {
                let ((a, b), (c, d)) = (wide(*left), wide(*right));
                (a + c, b + d)
            }
            (Self::Subtract, [left, right]) => {
                let ((a, b), (c, d)) = (wide(*left), wide(*right));
                (a - d, b - c)
            }
            (Self::Multiply, [left, right]) => {
                let ((a, b), (c, d)) = (wide(*left), wide(*right));
                let corners = [a * c, a * d, b * c, b * d];
                (
                    corners.into_iter().min().unwrap_or_default(),
                    corners.into_iter().max().unwrap_or_default(),
                )
            }
            _ => unreachable!("one range per operand name"),
        }
    }
}

struct LoweredClause<'a> {
    package: &'a BoundPackage,
    clause: &'a BoundClause,
    kind: ObligationKind,
    oracle: ClauseOracle,
    symbols: Symbols,
    /// Preconditions sharing the anchor, with their oracles, filled by `resolve_assumptions`.
    assumed: Vec<ClauseOracle>,
    /// For a contract obligation, the clause contexts its subject signature is built from: its
    /// own and those of every other supported contract obligation on the same operation, so all
    /// harnesses for one subject call it with one argument list. Filled by
    /// `unify_subject_signatures`.
    signature: Vec<(ClauseOracle, SlotContext)>,
}

#[derive(Clone)]
struct ClauseOracle {
    clause: ClauseRef,
    kind: ObligationKind,
    anchor: ExecutionPoint,
    symbol: String,
    source: String,
    parameters: Vec<Parameter>,
}

#[derive(Clone)]
struct Parameter {
    dependency: DependencyIdentity,
    value_type: RustValueType,
}

struct Symbols {
    module: String,
    harness: String,
    contract: String,
}

/// Classifies one request item. Its generated names are its readable stems until
/// [`assign_names`] settles them across the request.
fn classify<'a>(item: &ObligationItem<'a>) -> ItemState<'a> {
    match *item {
        ObligationItem::BoundClause { package, clause } => classify_clause(package, clause),
        ObligationItem::ScalarClaim {
            package,
            claim_map,
            node_id,
        } => classify_node(package, claim_map, node_id),
    }
}

/// The full identity a generated name is ordered by when several items share its readable stem.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum NameKey {
    Clause(String, u64, String),
    Node(CheckedNodeId),
}

fn clause_key(clause: &ClauseRef) -> NameKey {
    let requirement = clause.requirement();
    NameKey::Clause(
        requirement.requirement().as_str().to_owned(),
        requirement.revision().get(),
        clause.clause().as_str().to_owned(),
    )
}

/// Settles every lowered item's generated names across the request with [`unique_names`]: clause
/// oracle functions (any may be embedded beside another as an assumed precondition), then the
/// module and harness names of every clause and scalar obligation. A name depends on its item and
/// on the siblings sharing its readable stem, never on the item's request position.
fn assign_names(states: &mut [ItemState<'_>]) {
    let oracle_slots = states
        .iter()
        .enumerate()
        .filter_map(|(index, state)| match &state.outcome {
            Outcome::Lowered(lowered) => Some((
                index,
                (
                    lowered.oracle.symbol.clone(),
                    clause_key(&lowered.oracle.clause),
                ),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let oracle_names = unique_names(oracle_slots.iter().map(|(_, slot)| slot.clone()).collect());
    for ((index, (stem, _)), name) in oracle_slots.into_iter().zip(oracle_names) {
        if name == stem {
            continue;
        }
        let renamed = match &states[index].outcome {
            Outcome::Lowered(lowered) => named_oracle_source(lowered.clause, &name),
            _ => continue,
        };
        match renamed {
            Ok(source) => {
                if let Outcome::Lowered(lowered) = &mut states[index].outcome {
                    lowered.oracle.symbol = name;
                    lowered.oracle.source = source;
                }
            }
            Err(reason) => states[index].outcome = Outcome::Unsupported(reason),
        }
    }

    let module_slots = states
        .iter()
        .enumerate()
        .filter_map(|(index, state)| match &state.outcome {
            Outcome::Lowered(lowered) => Some((
                index,
                (
                    clause_stem(lowered.clause.identity()),
                    clause_key(lowered.clause.identity()),
                ),
            )),
            Outcome::LoweredScalar(lowered) => Some((
                index,
                (
                    scalar_stem(&lowered.operation_identity),
                    NameKey::Node(lowered.node_id.clone()),
                ),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let module_names = unique_names(module_slots.iter().map(|(_, slot)| slot.clone()).collect());
    for ((index, _), name) in module_slots.into_iter().zip(module_names) {
        match &mut states[index].outcome {
            Outcome::Lowered(lowered) => lowered.symbols = symbols(&name),
            Outcome::LoweredScalar(lowered) => {
                lowered.module_symbol = format!("{name}_module");
                lowered.harness_symbol = format!("{name}_proof");
            }
            _ => {}
        }
    }
}

/// The source of `clause`'s oracle with its function named `symbol`.
fn named_oracle_source(
    clause: &BoundClause,
    symbol: &str,
) -> Result<String, UnsupportedObligation> {
    let identity = clause.identity();
    generate_named_boolean_oracle(
        &OracleRequest {
            requirement: identity.requirement(),
            clause: identity.clause(),
            expression: clause.expression(),
        },
        symbol,
    )
    .map(|bundle| bundle.rust.contents)
    .map_err(|diagnostics| UnsupportedObligation::ClauseLowering {
        generation_code: diagnostics
            .first()
            .map_or(GenerationErrorCode::UnsupportedExpression, |diagnostic| {
                diagnostic.code
            }),
    })
}

fn classify_clause<'a>(package: &'a BoundPackage, clause_ref: &ClauseRef) -> ItemState<'a> {
    let package_digest = package.digest().to_string();
    let identity = Some(ItemIdentity::Clause {
        package: package_digest,
        clause: clause_ref.clone(),
    });
    let Some(clause) = package
        .clauses()
        .iter()
        .find(|candidate| candidate.identity() == clause_ref)
    else {
        return ItemState {
            kind: None,
            subject: ObligationSubject::BoundClause {
                clause: clause_ref.clone(),
                source_span: None,
            },
            identity,
            outcome: Outcome::Invalid(InvalidObligationItem::UnknownClause),
        };
    };
    let kind = obligation_kind(clause.kind());
    let subject = ObligationSubject::BoundClause {
        clause: clause_ref.clone(),
        source_span: Some(clause.source().clone()),
    };
    let outcome = match kind {
        None => Outcome::Unsupported(UnsupportedObligation::ClauseKindNotObligation {
            kind: clause.kind(),
        }),
        Some(kind) => match lower_clause(clause, kind) {
            Ok(oracle) => {
                let standalone = match kind {
                    ObligationKind::Precondition => {
                        abi(&[(&oracle, SlotContext::Precondition)]).map(|_| ())
                    }
                    ObligationKind::Postcondition => {
                        abi(&[(&oracle, SlotContext::Postcondition)]).map(|_| ())
                    }
                    ObligationKind::Invariant => abi(&[
                        (&oracle, SlotContext::InvariantBefore),
                        (&oracle, SlotContext::InvariantAfter),
                    ])
                    .map(|_| ()),
                    ObligationKind::Frame => Ok(()),
                };
                match standalone {
                    Ok(()) => Outcome::Lowered(Box::new(LoweredClause {
                        package,
                        clause,
                        kind,
                        symbols: symbols(&clause_stem(clause_ref)),
                        oracle,
                        assumed: Vec::new(),
                        signature: Vec::new(),
                    })),
                    Err(reason) => Outcome::Unsupported(reason),
                }
            }
            Err(reason) => Outcome::Unsupported(reason),
        },
    };
    ItemState {
        kind,
        subject,
        identity,
        outcome,
    }
}

const fn obligation_kind(kind: ClauseKind) -> Option<ObligationKind> {
    match kind {
        ClauseKind::Precondition => Some(ObligationKind::Precondition),
        ClauseKind::Postcondition => Some(ObligationKind::Postcondition),
        ClauseKind::Invariant => Some(ObligationKind::Invariant),
        ClauseKind::Assertion | ClauseKind::Case | ClauseKind::Information => None,
    }
}

fn lower_clause(
    clause: &BoundClause,
    kind: ObligationKind,
) -> Result<ClauseOracle, UnsupportedObligation> {
    let obligations = clause.expression().obligations().len();
    if obligations > 0 {
        return Err(UnsupportedObligation::DefinednessNotEncoded { obligations });
    }
    let identity = clause.identity();
    let oracle_request = OracleRequest {
        requirement: identity.requirement(),
        clause: identity.clause(),
        expression: clause.expression(),
    };
    let first_code =
        |diagnostics: Vec<crate::GenerationDiagnostic>| UnsupportedObligation::ClauseLowering {
            generation_code: diagnostics
                .first()
                .map_or(GenerationErrorCode::UnsupportedExpression, |diagnostic| {
                    diagnostic.code
                }),
        };
    let parameters = typed_dependency_parameters(&oracle_request).map_err(first_code)?;
    let bundle = generate_boolean_oracle(&oracle_request).map_err(first_code)?;
    let symbol =
        oracle_function_symbol(&bundle.rust.contents).ok_or(UnsupportedObligation::RenderFailed)?;
    Ok(ClauseOracle {
        clause: identity.clone(),
        kind,
        anchor: clause.anchor().clone(),
        symbol,
        source: bundle.rust.contents,
        parameters: parameters
            .into_iter()
            .map(
                |DependencyParameter {
                     dependency,
                     value_type,
                     ..
                 }| Parameter {
                    dependency,
                    value_type,
                },
            )
            .collect(),
    })
}

/// The one `pub fn oracle_…` the oracle generator emits.
fn oracle_function_symbol(source: &str) -> Option<String> {
    source.lines().find_map(|line| {
        line.strip_prefix("pub fn ")
            .and_then(|tail| tail.split('(').next())
            .map(str::to_owned)
    })
}

/// The readable stem of a clause obligation's names: its requirement, revision and clause. Kani
/// derives object-file names from these symbols, so the readable components are bounded.
fn clause_stem(clause: &ClauseRef) -> String {
    let requirement = clause.requirement();
    format!(
        "kob_{}_{}_{}",
        readable_component(requirement.requirement().as_str()),
        requirement.revision().get(),
        readable_component(clause.clause().as_str()),
    )
}

/// The readable stem of a scalar obligation's names: its confirmed operation.
fn scalar_stem(operation: &str) -> String {
    format!(
        "kob_scalar_{}",
        readable_component(operation.strip_prefix("quire.op.").unwrap_or(operation))
    )
}

/// The validated `module::harness` identity of a harness, built once where the harness is
/// generated. The names are `kob_`-prefixed readable components, so a refusal is an internal
/// invariant failing and is reported as invalid generated Rust.
fn harness_path(module: &str, harness: &str) -> Result<HarnessPath, UnsupportedObligation> {
    let invalid = |error: SymbolError| UnsupportedObligation::InvalidGeneratedSyntax {
        error: error.to_string(),
    };
    Ok(HarnessPath {
        module: ModuleSymbol::try_from(module).map_err(invalid)?,
        harness: HarnessSymbol::try_from(harness).map_err(invalid)?,
    })
}

/// The module, harness and contract names built on one settled name.
fn symbols(base: &str) -> Symbols {
    Symbols {
        module: format!("{base}_module"),
        harness: format!("{base}_proof"),
        contract: format!("{base}_contract"),
    }
}

const fn kind_name(kind: ObligationKind) -> &'static str {
    match kind {
        ObligationKind::Precondition => "precondition",
        ObligationKind::Postcondition => "postcondition",
        ObligationKind::Invariant => "invariant",
        ObligationKind::Frame => "frame",
    }
}

fn classify_node<'a>(
    package: &CheckedPackageV2,
    claim_map: &ClaimMap<ExactScalarClaim>,
    node_id: &CheckedNodeId,
) -> ItemState<'a> {
    let graph_node = package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == node_id);
    let kind = graph_node.and_then(|node| {
        (&*node.node_tag == "state" && &*node.semantic_form == "frame")
            .then_some(ObligationKind::Frame)
    });
    let subject = ObligationSubject::CheckedNode {
        node_id: node_id.clone(),
        source_map: package
            .source_map()
            .iter()
            .filter(|entry| &entry.node_id == node_id)
            .cloned()
            .collect(),
    };
    let identity = Some(ItemIdentity::Node {
        package: package.package_id().digest.to_string(),
        node: node_id.clone(),
    });
    let outcome = if &claim_map.package_id != package.package_id() {
        Outcome::Invalid(InvalidObligationItem::PackageMismatch)
    } else {
        match claim_map
            .items
            .iter()
            .find(|claim| &claim.node_id == node_id)
        {
            None => Outcome::Invalid(InvalidObligationItem::UnknownNode),
            Some(claim) => classify_claim(package, claim),
        }
    };
    let outcome = refuse_unknown_node_kind(graph_node, kind, outcome);
    ItemState {
        kind,
        subject,
        identity,
        outcome,
    }
}

/// Guards [`Outcome::LoweredScalar`] against a node whose graph entry does not match the one
/// recognized `state`/`frame` role pair, and against a node absent from the graph entirely.
/// [`ClaimMap`]/[`ExactScalarClaim`] are fully `pub`, so nothing enforces that a claim
/// map assembled by another caller names only node ids [`CheckedPackageV2::graph`] also carries --
/// that invariant holds only for a claim map this crate's own
/// [`crate::oracle::scalar::generate_exact_scalar_oracles`] produced. A hand-assembled claim map
/// whose [`GeneratedScalarClaim`] bounds still resolve can therefore reach
/// [`Outcome::LoweredScalar`] for a `node_id` this crate never checked is in the graph at all, so
/// that case is refused here too, as [`InvalidObligationItem::UnknownNode`] -- the same code
/// [`ExactScalarRefusal::InvalidInput`] ("the node is not in the admitted graph") already reports
/// through `classify_claim`'s `Refused` arm, so a node absent from the graph is refused under one
/// code regardless of which path notices it first. `expression` is excluded from the tag/form
/// check because it is the one family [`crate::oracle::scalar::generate_exact_scalar_oracles`] ever
/// lowers to a [`ClaimDisposition::Generated`] claim at all -- every real, golden-path
/// scalar claim this generator supports is an `expression` node, and none of those carry a
/// contract role, so a null `kind` there is correct, not unmodeled. Scoped to the success arm
/// rather than every arm: `classify_claim` already refuses every other `state`-tagged node with
/// its own typed reason (for every fixture reaching it today, always
/// [`UnsupportedObligation::NoFiniteEncoding`], since `generate_exact_scalar_oracles` accepts only
/// `expression`-tagged nodes), and that reason is more specific than this one -- this ground
/// exists for the gap those checks do not cover: a node this generator would otherwise have
/// accounted as `Supported` with a null `kind`.
fn refuse_unknown_node_kind<'a>(
    graph_node: Option<&CheckedSemanticNodeV2>,
    kind: Option<ObligationKind>,
    outcome: Outcome<'a>,
) -> Outcome<'a> {
    if kind.is_some() {
        return outcome;
    }
    let Outcome::LoweredScalar(_) = &outcome else {
        return outcome;
    };
    let Some(node) = graph_node else {
        return Outcome::Invalid(InvalidObligationItem::UnknownNode);
    };
    if &*node.node_tag == CheckedNodeTag::Expression.as_wire() {
        return outcome;
    }
    Outcome::Unsupported(UnsupportedObligation::UnknownNodeKind {
        node_id: node.node_id.clone(),
        node_tag: node.node_tag.to_string(),
        semantic_form: node.semantic_form.to_string(),
    })
}

fn classify_claim<'a>(package: &CheckedPackageV2, claim: &ExactScalarClaim) -> Outcome<'a> {
    let graph = |id: &CheckedNodeId| {
        package
            .graph()
            .nodes
            .iter()
            .find(|node| &node.node_id == id)
    };
    match &claim.result {
        ClaimDisposition::Generated(generated) => {
            let mut derived = Vec::new();
            let bound_ids = generated
                .checked_bounds
                .iter()
                .chain(&generated.bounds)
                .collect::<BTreeSet<_>>();
            for bound in bound_ids.into_iter().filter_map(graph) {
                let domain = derive_domain(bound);
                if let Some((lower, upper)) = unsatisfiable(bound) {
                    return Outcome::Unsupported(UnsupportedObligation::UnsatisfiableBound {
                        bound: bound.node_id.clone(),
                        form: bound.semantic_form.to_string(),
                        lower,
                        upper,
                    });
                }
                derived.push(domain);
            }
            match claim.operation.provenance {
                OperationProvenance::CallerDeclared { blocked_on } => {
                    Outcome::Unsupported(UnsupportedObligation::CallerDeclaredOperation {
                        operation_identity: claim.operation.identity.clone(),
                        blocked_on,
                        derived_domains: derived,
                    })
                }
                // `generate_exact_scalar_oracles` never generates an underived claim (only the
                // routed arm builds them, always refused); a hand-assembled claim map that
                // pairs the two is as unchecked as a caller-declared one.
                OperationProvenance::Underived => {
                    Outcome::Unsupported(UnsupportedObligation::CallerDeclaredOperation {
                        operation_identity: claim.operation.identity.clone(),
                        blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
                        derived_domains: derived,
                    })
                }
                OperationProvenance::IrConfirmed => {
                    let operand_ranges = operand_ranges(package, &claim.node_id);
                    match lower_scalar_claim(claim, generated, &derived, &operand_ranges) {
                        Ok(lowered) => Outcome::LoweredScalar(Box::new(lowered)),
                        Err(ScalarLoweringRefusal::NoRenderer) => {
                            Outcome::Unsupported(UnsupportedObligation::OperationNotRendered {
                                operation_identity: claim.operation.identity.clone(),
                                derived_domains: derived,
                            })
                        }
                        Err(ScalarLoweringRefusal::ResultUnreachable {
                            lower,
                            upper,
                            reachable_lower,
                            reachable_upper,
                        }) => Outcome::Unsupported(UnsupportedObligation::ResultBoundUnreachable {
                            operation_identity: claim.operation.identity.clone(),
                            lower,
                            upper,
                            reachable_lower,
                            reachable_upper,
                        }),
                        Err(ScalarLoweringRefusal::BoundNotI64 { lower, upper }) => {
                            Outcome::Unsupported(
                                UnsupportedObligation::DomainNotRepresentableInI64 {
                                    operation_identity: claim.operation.identity.clone(),
                                    lower,
                                    upper,
                                },
                            )
                        }
                    }
                }
            }
        }
        ClaimDisposition::Refused { refusal } => match refusal {
            ExactScalarRefusal::InvalidInput => {
                Outcome::Invalid(InvalidObligationItem::UnknownNode)
            }
            ExactScalarRefusal::RequiresBound { unbounded_type } => {
                Outcome::RequiresBound(unbounded_type.clone())
            }
            ExactScalarRefusal::MissingBound { bounded_type, .. } => {
                Outcome::RequiresBound(bounded_type.clone())
            }
            ExactScalarRefusal::Unsupported {
                unsupported_node_id,
                node_tag,
            } => Outcome::Unsupported(UnsupportedObligation::NoFiniteEncoding {
                node_id: unsupported_node_id.clone(),
                node_tag,
            }),
            ExactScalarRefusal::BlockedOnUpstream {
                unsupported_node_id,
                node_tag,
                issue,
            } => Outcome::Unsupported(UnsupportedObligation::BlockedOnUpstream {
                node_id: unsupported_node_id.clone(),
                node_tag,
                issue: *issue,
            }),
            ExactScalarRefusal::UnreadableBound { bound } => {
                match graph(bound).and_then(|node| unsatisfiable(node).map(|pair| (node, pair))) {
                    Some((node, (lower, upper))) => {
                        Outcome::Unsupported(UnsupportedObligation::UnsatisfiableBound {
                            bound: bound.clone(),
                            form: node.semantic_form.to_string(),
                            lower,
                            upper,
                        })
                    }
                    None => Outcome::Unsupported(UnsupportedObligation::OracleRefused {
                        refusal: refusal.clone(),
                    }),
                }
            }
            ExactScalarRefusal::NoDerivableClaim { reason } => {
                Outcome::Unsupported(UnsupportedObligation::NoDerivableClaim {
                    node_id: claim.node_id.clone(),
                    reason: reason.clone(),
                })
            }
            ExactScalarRefusal::DuplicateRequest
            | ExactScalarRefusal::AmbiguousBound { .. }
            | ExactScalarRefusal::BoundMismatch { .. }
            | ExactScalarRefusal::OperandUnsupported { .. }
            | ExactScalarRefusal::UnitlessLiteralOperand { .. }
            | ExactScalarRefusal::InvalidBody { .. }
            | ExactScalarRefusal::BodyIncomplete { .. }
            | ExactScalarRefusal::LoweringWorkExhausted { .. }
            | ExactScalarRefusal::NotExpression { .. }
            | ExactScalarRefusal::FormMismatch { .. }
            | ExactScalarRefusal::BodyMismatch { .. }
            | ExactScalarRefusal::ResultTypeMismatch { .. }
            | ExactScalarRefusal::OperandTypeMismatch { .. }
            | ExactScalarRefusal::MissingOperationIdentity { .. } => {
                Outcome::Unsupported(UnsupportedObligation::OracleRefused {
                    refusal: refusal.clone(),
                })
            }
        },
    }
}

/// Why [`lower_scalar_claim`] could not lower an IR-confirmed claim -- two distinct causes that
/// [`classify_claim`] reports as two distinct [`UnsupportedObligation`] reasons, rather than
/// folding them into one the way a single `Option` return would. A third and fourth candidate
/// cause -- `check_parameters` recording no `checked_bounds` entry, or recording one whose own
/// derived domain is not an `integer_range` -- are not represented here: [`ScalarOperation::of`] only
/// renders the `IntegerArithmetic` family, and for that family `check_parameters`'s own
/// `Bounds::equal` returns `Ok` only after successfully reading exactly one `integer_range` bound
/// (`oracle::scalar::check_parameters`, `oracle::scalar::Bounds::equal`) -- the same node, read by the
/// same `literal_integer`, that this function's own [`derive_domain`] call reads. Given that
/// guarantee, both failure shapes are structurally unreachable through this function today, so
/// they are asserted with `unreachable!` below rather than modelled as a caller-visible refusal a
/// test could never construct a fixture for.
enum ScalarLoweringRefusal {
    /// [`ScalarOperation::of`] has no renderer for this operation identity.
    NoRenderer,
    /// Every operand range combines to results outside the result range, so no input the harness
    /// assumes completes and its non-vacuity cover could never be met.
    ResultUnreachable {
        /// The result range's inclusive lower bound.
        lower: i64,
        /// The result range's inclusive upper bound.
        upper: i64,
        /// The least result the operand ranges produce, canonical decimal.
        reachable_lower: String,
        /// The greatest result the operand ranges produce, canonical decimal.
        reachable_upper: String,
    },
    /// The `integer_range`'s lower or upper endpoint, or a literal operand, does not fit `i64`.
    BoundNotI64 {
        /// Inclusive lower bound, canonical decimal.
        lower: String,
        /// Inclusive upper bound, canonical decimal.
        upper: String,
    },
}

/// Lowers one IR-confirmed V2 claim to a renderable scalar harness. The result range is the
/// claim's first checked bound; each operand ranges over its own bound where the node's argument
/// is typed by one, exactly at its value where it is a literal (`operand_ranges`, by position),
/// and over the result range otherwise. A claim whose operand ranges produce no result inside the
/// result range is refused rather than rendered with a cover no input can meet.
fn lower_scalar_claim(
    claim: &ExactScalarClaim,
    generated: &GeneratedScalarClaim,
    derived: &[DerivedDomain],
    operand_ranges: &[OperandRange],
) -> Result<LoweredScalarClaim, ScalarLoweringRefusal> {
    let operation =
        ScalarOperation::of(&claim.operation.identity).ok_or(ScalarLoweringRefusal::NoRenderer)?;
    let Some(bound_id) = generated.checked_bounds.first() else {
        unreachable!(
            "check_parameters's Bounds::equal records exactly one checked bound for every \
             IntegerArithmetic claim, the only family ScalarOperation::of renders a harness for"
        );
    };
    let range_of = |id: &CheckedNodeId| {
        derived.iter().find_map(|domain| match domain {
            DerivedDomain::IntegerRange {
                bound,
                lower,
                upper,
            } if bound == id => Some((lower, upper)),
            _ => None,
        })
    };
    let Some((lower, upper)) = range_of(bound_id) else {
        unreachable!(
            "the same node backs bound_id here and in check_parameters, which already parsed its \
             two members with the same literal_integer this function's derive_domain uses, so it \
             cannot fail to be read as an IntegerRange here"
        );
    };
    let to_i64 = |(lower, upper): (&String, &String)| match (lower.parse(), upper.parse()) {
        (Ok(lower), Ok(upper)) => Ok((lower, upper)),
        _ => Err(ScalarLoweringRefusal::BoundNotI64 {
            lower: lower.clone(),
            upper: upper.clone(),
        }),
    };
    let (lower, upper) = to_i64((lower, upper))?;
    let operands = (0..operation.operand_names().len())
        .map(|position| match operand_ranges.get(position) {
            None | Some(OperandRange::Result) => Ok((lower, upper)),
            Some(OperandRange::Literal(value)) => to_i64((value, value)),
            // An own bound `check_parameters` recorded is in `checked_bounds` and a derived
            // integer range; one that is not is a claim map this generator did not produce,
            // which has no renderer.
            Some(OperandRange::Bound(id)) => generated
                .checked_bounds
                .contains(id)
                .then(|| range_of(id))
                .flatten()
                .ok_or(ScalarLoweringRefusal::NoRenderer)
                .and_then(to_i64),
        })
        .collect::<Result<Vec<(i64, i64)>, _>>()?;
    let (reachable_lower, reachable_upper) = operation.reachable(&operands);
    if reachable_upper < i128::from(lower) || reachable_lower > i128::from(upper) {
        return Err(ScalarLoweringRefusal::ResultUnreachable {
            lower,
            upper,
            reachable_lower: reachable_lower.to_string(),
            reachable_upper: reachable_upper.to_string(),
        });
    }
    // The readable stem; `assign_names` settles the final name across the request.
    let base = scalar_stem(&claim.operation.identity);
    let module_symbol = format!("{base}_module");
    let harness_symbol = format!("{base}_proof");
    Ok(LoweredScalarClaim {
        node_id: claim.node_id.clone(),
        operation_identity: claim.operation.identity.clone(),
        oracle_source: generated.oracle_source.clone(),
        oracle_symbol: generated.symbol.clone(),
        operation,
        operands,
        lower,
        upper,
        module_symbol,
        harness_symbol,
    })
}

fn derive_domain(bound: &CheckedSemanticNodeV2) -> DerivedDomain {
    let members = aggregate_members(&bound.body);
    match (&*bound.semantic_form, members) {
        ("integer_range", Some(members)) => {
            match bound_members(members, INTEGER_RANGE_MEMBERS)
                .map(|[lower, upper]| (literal_integer(lower), literal_integer(upper)))
            {
                Some((Some(lower), Some(upper))) => DerivedDomain::IntegerRange {
                    bound: bound.node_id.clone(),
                    lower: lower.to_string(),
                    upper: upper.to_string(),
                },
                _ => not_symbolic(bound),
            }
        }
        _ => not_symbolic(bound),
    }
}

fn not_symbolic(bound: &CheckedSemanticNodeV2) -> DerivedDomain {
    DerivedDomain::NotSymbolic {
        bound: bound.node_id.clone(),
        form: bound.semantic_form.to_string(),
    }
}

/// The canonical inclusive limits of a range bound that admits no value.
fn unsatisfiable(bound: &CheckedSemanticNodeV2) -> Option<(String, String)> {
    let members = aggregate_members(&bound.body)?;
    match &*bound.semantic_form {
        "integer_range" => {
            let [lower, upper] = bound_members(members, INTEGER_RANGE_MEMBERS)?;
            let (lower, upper) = (literal_integer(lower)?, literal_integer(upper)?);
            (lower > upper).then(|| (lower.to_string(), upper.to_string()))
        }
        "text_bounds" => {
            let [minimum, maximum, _] = bound_members(members, TEXT_BOUNDS_MEMBERS)?;
            count_bounds(minimum, maximum)
        }
        "collection_bounds" => {
            let [minimum, maximum] = bound_members(members, COLLECTION_BOUNDS_MEMBERS)?;
            count_bounds(minimum, maximum)
        }
        _ => None,
    }
}

/// The canonical limits of a cardinality range whose minimum exceeds its maximum.
fn count_bounds(
    minimum: &serde_json::Value,
    maximum: &serde_json::Value,
) -> Option<(String, String)> {
    let (minimum, maximum) = (literal_count(minimum)?, literal_count(maximum)?);
    (minimum > maximum).then(|| (minimum.to_string(), maximum.to_string()))
}

fn reject_duplicates_and_mixtures(items: &[ObligationItem<'_>], states: &mut [ItemState<'_>]) {
    let mut first_bound_package: Option<String> = None;
    for index in 0..states.len() {
        let Some(identity) = states[index].identity.clone() else {
            continue;
        };
        if let Some(first_index) = states[..index]
            .iter()
            .position(|earlier| earlier.identity.as_ref() == Some(&identity))
        {
            states[index].outcome =
                Outcome::Invalid(InvalidObligationItem::DuplicateItem { first_index });
            continue;
        }
        if let ObligationItem::BoundClause { package, .. } = items[index] {
            let digest = package.digest().to_string();
            match &first_bound_package {
                None => first_bound_package = Some(digest),
                Some(first) if *first != digest => {
                    states[index].outcome =
                        Outcome::Invalid(InvalidObligationItem::MixedBoundPackages);
                }
                Some(_) => {}
            }
        }
    }
}

/// The operation a precondition is anchored to, or a postcondition/invariant relies on.
fn anchor_operation(anchor: &ExecutionPoint) -> Option<&str> {
    match anchor {
        ExecutionPoint::Pre { operation } | ExecutionPoint::Post { operation } => {
            Some(operation.as_str())
        }
        ExecutionPoint::Handler { name } => Some(name.as_str()),
        ExecutionPoint::Initialization { .. } => None,
    }
}

/// Attaches every package precondition sharing a postcondition's or invariant's anchor, refusing
/// the obligation when such a precondition is not a supported item of this request.
fn resolve_assumptions(states: &mut [ItemState<'_>]) {
    let supported_preconditions = states
        .iter()
        .filter_map(|state| match &state.outcome {
            Outcome::Lowered(lowered) if lowered.kind == ObligationKind::Precondition => {
                Some((lowered.oracle.clause.clone(), lowered.oracle.clone()))
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    for state in states.iter_mut() {
        let Outcome::Lowered(lowered) = &mut state.outcome else {
            continue;
        };
        if lowered.kind == ObligationKind::Precondition {
            continue;
        }
        let Some(operation) = anchor_operation(&lowered.oracle.anchor) else {
            continue;
        };
        let mut refusal = None;
        for candidate in lowered.package.clauses() {
            let shares_anchor = candidate.kind() == ClauseKind::Precondition
                && anchor_operation(candidate.anchor()) == Some(operation);
            if !shares_anchor {
                continue;
            }
            match supported_preconditions.get(candidate.identity()) {
                Some(oracle) => lowered.assumed.push(oracle.clone()),
                None => {
                    refusal = Some(UnsupportedObligation::PreconditionNotNegotiated {
                        precondition: candidate.identity().clone(),
                    });
                    break;
                }
            }
        }
        if refusal.is_none() {
            refusal = abi(&contract_contexts(lowered)).err();
        }
        if let Some(reason) = refusal {
            state.outcome = Outcome::Unsupported(reason);
        }
    }
    unify_subject_signatures(states);
}

/// The subject a group of contract obligations share.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum SubjectGroup {
    /// Every contract obligation anchored to this operation.
    Operation(String),
    /// A contract obligation with no operation anchor, alone.
    Item(usize),
}

/// Gives every supported contract obligation on one operation the same subject signature, the
/// union of their slots, or refuses all of them when that union conflicts.
fn unify_subject_signatures(states: &mut [ItemState<'_>]) {
    let mut groups: BTreeMap<SubjectGroup, Vec<usize>> = BTreeMap::new();
    for (index, state) in states.iter().enumerate() {
        let Outcome::Lowered(lowered) = &state.outcome else {
            continue;
        };
        if lowered.kind == ObligationKind::Precondition {
            continue;
        }
        let group = anchor_operation(&lowered.oracle.anchor)
            .map_or(SubjectGroup::Item(index), |operation| {
                SubjectGroup::Operation(operation.to_owned())
            });
        groups.entry(group).or_default().push(index);
    }
    for (group, members) in groups {
        let signature = members
            .iter()
            .filter_map(|&index| match &states[index].outcome {
                Outcome::Lowered(lowered) => Some(contract_contexts(lowered)),
                _ => None,
            })
            .flatten()
            .map(|(oracle, context)| (oracle.clone(), context))
            .collect::<Vec<_>>();
        let union = abi(&signature
            .iter()
            .map(|(oracle, context)| (oracle, *context))
            .collect::<Vec<_>>());
        let refusal = match (union, group) {
            (Ok(_), _) => None,
            (
                Err(UnsupportedObligation::AbiConflict { identifier }),
                SubjectGroup::Operation(operation),
            ) => Some(UnsupportedObligation::SubjectSignatureConflict {
                operation,
                identifier,
            }),
            (Err(reason), _) => Some(reason),
        };
        for index in members {
            match &refusal {
                Some(reason) => states[index].outcome = Outcome::Unsupported(reason.clone()),
                None => {
                    if let Outcome::Lowered(lowered) = &mut states[index].outcome {
                        lowered.signature.clone_from(&signature);
                    }
                }
            }
        }
    }
}

fn contract_contexts<'l>(lowered: &'l LoweredClause<'_>) -> Vec<(&'l ClauseOracle, SlotContext)> {
    let mut contexts = lowered
        .assumed
        .iter()
        .map(|oracle| (oracle, SlotContext::Precondition))
        .collect::<Vec<_>>();
    match lowered.kind {
        ObligationKind::Precondition => contexts.push((&lowered.oracle, SlotContext::Precondition)),
        ObligationKind::Postcondition => {
            contexts.push((&lowered.oracle, SlotContext::Postcondition));
        }
        ObligationKind::Invariant => {
            contexts.push((&lowered.oracle, SlotContext::InvariantBefore));
            contexts.push((&lowered.oracle, SlotContext::InvariantAfter));
        }
        ObligationKind::Frame => {}
    }
    contexts
}

// ---------------------------------------------------------------------------
// Subject ABI
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Eq, PartialEq)]
enum SlotContext {
    /// Evaluated on the arguments before the call.
    Precondition,
    /// Evaluated after the call, over pre-state arguments and post-state results.
    Postcondition,
    /// An invariant evaluated on the arguments before the call.
    InvariantBefore,
    /// An invariant evaluated on the results after the call.
    InvariantAfter,
}

struct Abi {
    arguments: Vec<ObligationBinding>,
    results: Vec<ObligationBinding>,
}

impl Abi {
    fn access(&self, identifier: &str) -> Option<String> {
        if self
            .arguments
            .iter()
            .any(|binding| binding.identifier == identifier)
        {
            return Some(identifier.to_owned());
        }
        let index = self
            .results
            .iter()
            .position(|binding| binding.identifier == identifier)?;
        Some(if self.results.len() == 1 {
            "*post_state".to_owned()
        } else {
            format!("post_state.{index}")
        })
    }
}

/// The slot a dependency occupies at one evaluation point.
fn slot(
    dependency: &DependencyIdentity,
    context: SlotContext,
) -> Option<(String, KaniBindingRole)> {
    let name = dependency.path().first()?.as_str();
    let observation = dependency.observation();
    let slot = |observation, role| Some((reference_identifier(name, Some(observation)), role));
    match (dependency.kind(), observation, context) {
        (DependencyKind::Input, None | Some(StateObservation::Current), _) => {
            slot(StateObservation::Current, KaniBindingRole::Argument)
        }
        (
            DependencyKind::State,
            Some(StateObservation::Current),
            SlotContext::Precondition | SlotContext::InvariantBefore,
        )
        | (
            DependencyKind::State,
            Some(StateObservation::Pre),
            SlotContext::Precondition | SlotContext::Postcondition,
        ) => slot(StateObservation::Pre, KaniBindingRole::Argument),
        (DependencyKind::State, Some(StateObservation::Current), SlotContext::InvariantAfter)
        | (DependencyKind::State, Some(StateObservation::Post), SlotContext::Postcondition) => {
            slot(StateObservation::Post, KaniBindingRole::Result)
        }
        _ => None,
    }
}

fn abi(contexts: &[(&ClauseOracle, SlotContext)]) -> Result<Abi, UnsupportedObligation> {
    let mut bindings: BTreeMap<String, ObligationBinding> = BTreeMap::new();
    for (oracle, context) in contexts {
        for parameter in &oracle.parameters {
            let (identifier, role) = slot(&parameter.dependency, *context).ok_or_else(|| {
                UnsupportedObligation::ObservationNotBindable {
                    dependency: parameter.dependency.clone(),
                }
            })?;
            let (primitive_type, integer_bounds) = match &parameter.value_type {
                RustValueType::Boolean => (KaniPrimitiveType::Boolean, None),
                RustValueType::Integer(value) => (
                    KaniPrimitiveType::I64,
                    Some(KaniIntegerBounds {
                        domain: value.domain(),
                        minimum: value.minimum(),
                        maximum: value.maximum(),
                        overflow: value.overflow(),
                    }),
                ),
            };
            match bindings.get_mut(&identifier) {
                Some(existing) => {
                    if existing.role != role
                        || existing.primitive_type != primitive_type
                        || existing.integer_bounds != integer_bounds
                        || existing.dependencies.first().map(DependencyIdentity::kind)
                            != Some(parameter.dependency.kind())
                    {
                        return Err(UnsupportedObligation::AbiConflict { identifier });
                    }
                    if !existing.dependencies.contains(&parameter.dependency) {
                        existing.dependencies.push(parameter.dependency.clone());
                        existing.dependencies.sort();
                    }
                }
                None => {
                    bindings.insert(
                        identifier.clone(),
                        ObligationBinding {
                            identifier,
                            role,
                            primitive_type,
                            integer_bounds,
                            dependencies: vec![parameter.dependency.clone()],
                        },
                    );
                }
            }
        }
    }
    let (arguments, results) = bindings
        .into_values()
        .partition(|binding| binding.role == KaniBindingRole::Argument);
    Ok(Abi { arguments, results })
}

fn call(oracle: &ClauseOracle, context: SlotContext, abi: &Abi) -> Option<String> {
    let arguments = oracle
        .parameters
        .iter()
        .map(|parameter| {
            let (identifier, _) = slot(&parameter.dependency, context)?;
            abi.access(&identifier)
        })
        .collect::<Option<Vec<_>>>()?;
    Some(format!("{}({})", oracle.symbol, arguments.join(", ")))
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

fn render(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredClause<'_>,
) -> Result<KaniObligationHarness, UnsupportedObligation> {
    let contexts = match lowered.kind {
        ObligationKind::Precondition => contract_contexts(lowered),
        ObligationKind::Postcondition | ObligationKind::Invariant | ObligationKind::Frame => {
            lowered
                .signature
                .iter()
                .map(|(oracle, context)| (oracle, *context))
                .collect()
        }
    };
    let abi = abi(&contexts)?;
    let path = harness_path(&lowered.symbols.module, &lowered.symbols.harness)?;
    let options = adapter_options(
        &path.to_string(),
        request.unwind,
        KaniSolver::Cadical,
        false,
    );
    // `assign_names` gave every clause oracle in the request a distinct name.
    let mut embedded = vec![&lowered.oracle];
    embedded.extend(&lowered.assumed);
    let oracle_sources = embedded
        .iter()
        .map(|oracle| oracle.source.as_str())
        .collect::<Vec<_>>();
    let is_contract = lowered.kind != ObligationKind::Precondition;
    let identity = KaniObligationIdentity {
        kind: lowered.kind,
        clause: lowered.clause.identity().clone(),
        source_span: lowered.clause.source().clone(),
        oracles: embedded
            .iter()
            .map(|oracle| EmbeddedOracle {
                clause: oracle.clause.clone(),
                kind: oracle.kind,
                symbol: oracle.symbol.clone(),
            })
            .collect(),
        module_symbol: path.module,
        harness_symbol: path.harness,
        contract_symbol: is_contract.then(|| lowered.symbols.contract.clone()),
        subject_path: is_contract.then(|| request.subject_path.to_owned()),
        arguments: abi.arguments.clone(),
        results: abi.results.clone(),
        solver: KaniSolver::Cadical,
        unwind: request.unwind,
        options,
    };
    let body = match lowered.kind {
        ObligationKind::Precondition => {
            render_precondition(lowered, &abi).ok_or(UnsupportedObligation::RenderFailed)?
        }
        ObligationKind::Postcondition | ObligationKind::Invariant => {
            render_contract(request.subject_path, lowered, &abi)
                .ok_or(UnsupportedObligation::RenderFailed)?
        }
        ObligationKind::Frame => return Err(UnsupportedObligation::FrameNotClauseRendered),
    };
    let clause = lowered.clause.identity();
    let mut source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Obligation: {} {}@{}/{}\n\n",
        kind_name(lowered.kind),
        clause.requirement().requirement().as_str(),
        clause.requirement().revision().get(),
        clause.clause().as_str(),
    );
    for oracle_source in &oracle_sources {
        source.push_str(oracle_source);
        source.push('\n');
    }
    source.push_str(&body);
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(UnsupportedObligation::ResourceLimitExceeded {
            bytes: source.len(),
        });
    }
    if let Err(error) = syn::parse_file(&source) {
        return Err(UnsupportedObligation::InvalidGeneratedSyntax {
            error: error.to_string(),
        });
    }
    let rust = artifact(
        format!("src/generated/{}.rs", lowered.symbols.module),
        source,
    );
    let record = record(&lowered.symbols.module, &identity, &rust)?;
    Ok(KaniObligationHarness {
        identity,
        rust,
        record,
    })
}

/// Renders one IR-confirmed V2 exact-scalar claim to a `kani::proof` that the embedded oracle
/// computes the clause's own arithmetic. The harness evaluates the operation a second time, in
/// native `i128` over the same symbolic `i64` operands ([`ScalarOperation::native_expression`]), and asserts the
/// oracle agrees with it in both directions: the outcome is `Ok(Completed(value))` with `value`
/// exactly the native result when that result lies within the checked domain (the result bound's
/// own literal bounds), and `Ok(Refused(_))` when it does not. Any other outcome fails. An oracle
/// that computed a different operation, dropped a carry, or mis-bounded the result therefore
/// falsifies the proof; asserting only that a completed value lies in the domain would restate the
/// check the oracle itself makes before returning `Completed`.
///
/// Each operand's `kani::assume` is its own bound (the `bounded_domain` typing it; a literal
/// operand, which is a constant, takes the result's). The operation is genuinely partial over that
/// domain (`quire.op.integer.add` over `[-1000,1000]` admits `600 + 600`), which is why the
/// refused direction is part of the property rather than an unconditional `Completed`. The meter
/// is unlimited, so `Incomplete` cannot occur. A `kani::cover!` on the `Completed` branch is the
/// FR-015-AC-7 non-vacuity guard: the property is checked on at least one completing input.
///
/// This generator renders the harness; it does not run the Kani/CBMC solver. The `kani` lane
/// (`tests/it/kani_obligations.rs`) proves it and runs the mutated-arithmetic control.
fn render_scalar(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredScalarClaim,
) -> Result<KaniScalarObligationHarness, UnsupportedObligation> {
    let path = harness_path(&lowered.module_symbol, &lowered.harness_symbol)?;
    let options = adapter_options(
        &path.to_string(),
        request.unwind,
        KaniSolver::Cadical,
        false,
    );
    let lower = i64_literal(lowered.lower);
    let upper = i64_literal(lowered.upper);
    let names = lowered.operation.operand_names();
    let declarations = names
        .iter()
        .zip(&lowered.operands)
        .map(|(name, (minimum, maximum))| {
            let (minimum, maximum) = (i64_literal(*minimum), i64_literal(*maximum));
            format!(
                "        let {name}: i64 = kani::any();\n\
        kani::assume({name} >= {minimum} && {name} <= {maximum});\n\
        let {name}_native = {name};\n\
        let {name} = rt::Integer::from({name});\n"
            )
        })
        .collect::<String>();
    let call_args = names
        .iter()
        .map(|name| format!("&{name}"))
        .collect::<Vec<_>>()
        .join(", ");
    let arguments = names
        .iter()
        .zip(&lowered.operands)
        .map(|(name, (minimum, maximum))| ScalarObligationArgument {
            identifier: (*name).to_owned(),
            minimum: *minimum,
            maximum: *maximum,
        })
        .collect();
    let body = format!(
        "#[cfg(kani)]\n\
mod {module} {{\n\
    use super::*;\n\
\n\
    #[kani::proof]\n\
    fn {harness}() {{\n\
{declarations}\
        let mut meter = rt::Meter::new(rt::ScalarLimits {{\n\
            integer_bits: u64::MAX,\n\
            decimal_digits: u64::MAX,\n\
            scale_expansion: u64::MAX,\n\
            text_input_bytes: u64::MAX,\n\
            text_scalars: u64::MAX,\n\
            normalized_scalars: u64::MAX,\n\
            unit_edges: u64::MAX,\n\
            value_occurrences: u64::MAX,\n\
            work_units: u64::MAX,\n\
            result_units: u64::MAX,\n\
        }});\n\
        let outcome = {symbol}({call_args}, &mut meter);\n\
        let exact: i128 = {exact};\n\
        let admitted = exact >= i128::from({lower}) && exact <= i128::from({upper});\n\
        let sound = match &outcome {{\n\
            Ok(rt::Outcome::Completed(value)) => admitted && *value == rt::Integer::from(exact),\n\
            Ok(rt::Outcome::Refused(_)) => !admitted,\n\
            _ => false,\n\
        }};\n\
        assert!(sound, \"the oracle must complete with exactly the native {operation} result when it lies in the checked domain, and refuse otherwise\");\n\
        let completed = matches!(outcome, Ok(rt::Outcome::Completed(_)));\n\
        kani::cover!(completed, \"the generated oracle's Ok(Outcome::Completed(_)) branch is reachable within its checked domain\");\n\
    }}\n\
}}\n",
        module = lowered.module_symbol,
        harness = lowered.harness_symbol,
        symbol = lowered.oracle_symbol,
        exact = lowered.operation.native_expression(),
        operation = lowered.operation_identity,
    );
    let identity = ScalarObligationIdentity {
        node_id: lowered.node_id.clone(),
        operation_identity: lowered.operation_identity.clone(),
        oracle_symbol: lowered.oracle_symbol.clone(),
        module_symbol: path.module,
        harness_symbol: path.harness,
        arguments,
        solver: KaniSolver::Cadical,
        unwind: request.unwind,
        options,
    };
    let mut source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Obligation: exact-scalar `{}`\n\n",
        lowered.operation_identity,
    );
    source.push_str(&lowered.oracle_source);
    source.push('\n');
    source.push_str(&body);
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(UnsupportedObligation::ResourceLimitExceeded {
            bytes: source.len(),
        });
    }
    if let Err(error) = syn::parse_file(&source) {
        return Err(UnsupportedObligation::InvalidGeneratedSyntax {
            error: error.to_string(),
        });
    }
    let rust = artifact(
        format!("src/generated/{}.rs", lowered.module_symbol),
        source,
    );
    let record = record(&lowered.module_symbol, &identity, &rust)?;
    Ok(KaniScalarObligationHarness {
        identity,
        rust,
        record,
    })
}

/// The persisted `kani-obligations/{module}.json` record of one harness.
fn record<T: Serialize>(
    module: &str,
    identity: &T,
    rust: &Artifact,
) -> Result<Artifact, UnsupportedObligation> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct HarnessRecord<'a, T> {
        identity: &'a T,
        rust_path: &'a str,
    }
    let mut json = serde_json::to_string(&HarnessRecord {
        identity,
        rust_path: &rust.path,
    })
    .map_err(|_| UnsupportedObligation::RenderFailed)?;
    json.push('\n');
    Ok(artifact(format!("kani-obligations/{module}.json"), json))
}

fn artifact(path: String, contents: String) -> Artifact {
    Artifact::new(path, contents)
}

/// `kani::any()` for every argument, constrained only by its IR integer bounds.
fn symbolic_arguments(arguments: &[ObligationBinding]) -> String {
    arguments
        .iter()
        .map(|binding| {
            let declaration = format!(
                "        let {}: {} = kani::any();\n",
                binding.identifier,
                binding.primitive_type.source_name()
            );
            match &binding.integer_bounds {
                Some(bounds) => format!(
                    "{declaration}        kani::assume({id} >= {} && {id} <= {});\n",
                    i64_literal(bounds.minimum),
                    i64_literal(bounds.maximum),
                    id = binding.identifier,
                ),
                None => declaration,
            }
        })
        .collect()
}

fn render_precondition(lowered: &LoweredClause<'_>, abi: &Abi) -> Option<String> {
    let holds = call(&lowered.oracle, SlotContext::Precondition, abi)?;
    Some(format!(
        "#[cfg(kani)]\n\
mod {module} {{\n\
    use super::*;\n\
\n\
    #[kani::proof]\n\
    fn {harness}() {{\n\
{arguments}        let precondition_holds = {holds};\n\
        kani::cover!(precondition_holds, \"precondition is satisfiable within the IR bounds\");\n\
    }}\n\
}}\n",
        module = lowered.symbols.module,
        harness = lowered.symbols.harness,
        arguments = symbolic_arguments(&abi.arguments),
    ))
}

/// The non-vacuity cover every contract harness ends with. It is reachable only on a path where
/// the IR argument bounds and every `requires` hold together, so an unsatisfied cover means the
/// `ensures` was never checked.
const CONTRACT_COVER: &str = "contract requires and IR bounds are jointly satisfiable";

fn render_contract(subject_path: &str, lowered: &LoweredClause<'_>, abi: &Abi) -> Option<String> {
    let mut requires = lowered
        .assumed
        .iter()
        .map(|oracle| call(oracle, SlotContext::Precondition, abi))
        .collect::<Option<Vec<_>>>()?;
    let obligation = match lowered.kind {
        ObligationKind::Postcondition => call(&lowered.oracle, SlotContext::Postcondition, abi)?,
        ObligationKind::Invariant => {
            requires.push(call(&lowered.oracle, SlotContext::InvariantBefore, abi)?);
            call(&lowered.oracle, SlotContext::InvariantAfter, abi)?
        }
        ObligationKind::Precondition | ObligationKind::Frame => return None,
    };
    let result_bounds = abi
        .results
        .iter()
        .filter_map(|binding| {
            let bounds = binding.integer_bounds.as_ref()?;
            let access = abi.access(&binding.identifier)?;
            Some(format!(
                "{access} >= {} && {access} <= {}",
                i64_literal(bounds.minimum),
                i64_literal(bounds.maximum)
            ))
        })
        .collect::<Vec<_>>();
    let ensures = if result_bounds.is_empty() {
        obligation
    } else {
        format!("({}) && {obligation}", result_bounds.join(" && "))
    };
    let result_type = match abi.results.as_slice() {
        [] => "()".to_owned(),
        [binding] => binding.primitive_type.source_name().to_owned(),
        bindings => format!(
            "({})",
            bindings
                .iter()
                .map(|binding| binding.primitive_type.source_name())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    let post_state = if abi.results.is_empty() {
        "_post_state"
    } else {
        "post_state"
    };
    let declarations = abi
        .arguments
        .iter()
        .map(|binding| {
            format!(
                "{}: {}",
                binding.identifier,
                binding.primitive_type.source_name()
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let names = abi
        .arguments
        .iter()
        .map(|binding| binding.identifier.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let requires = requires
        .iter()
        .map(|predicate| format!("    #[kani::requires({predicate})]\n"))
        .collect::<String>();
    Some(format!(
        "#[cfg(kani)]\n\
mod {module} {{\n\
    use super::*;\n\
\n\
{requires}    #[kani::ensures(|{post_state}: &{result_type}| {ensures})]\n\
    fn {contract}({declarations}) -> {result_type} {{\n\
        {subject_path}({names})\n\
    }}\n\
\n\
    #[kani::proof_for_contract({contract})]\n\
    fn {harness}() {{\n\
{arguments}        let _ = {contract}({names});\n\
        kani::cover!(true, \"{CONTRACT_COVER}\");\n\
    }}\n\
}}\n",
        module = lowered.symbols.module,
        contract = lowered.symbols.contract,
        harness = lowered.symbols.harness,
        arguments = symbolic_arguments(&abi.arguments),
    ))
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;
    use crate::OperationClaim;
    use quire_contract_model::{ClauseId, RequirementRef, EXECUTABLE_PROJECTION_FORMAT};

    /// Trace: FR-015-AC-1, TC-025.
    #[test]
    fn tc_025_every_clause_kind_maps_to_at_most_one_obligation_kind() {
        assert_eq!(
            obligation_kind(ClauseKind::Precondition),
            Some(ObligationKind::Precondition)
        );
        assert_eq!(
            obligation_kind(ClauseKind::Postcondition),
            Some(ObligationKind::Postcondition)
        );
        assert_eq!(
            obligation_kind(ClauseKind::Invariant),
            Some(ObligationKind::Invariant)
        );
        for kind in [
            ClauseKind::Assertion,
            ClauseKind::Case,
            ClauseKind::Information,
        ] {
            assert_eq!(obligation_kind(kind), None);
        }
    }

    /// The symbolic domain is read from binding-shaped `min`/`max` members (the shape QSL emits,
    /// FR-322); a bare-literal member is not symbolic.
    ///
    /// Trace: FR-015-AC-5, TC-025.
    #[test]
    fn tc_025_derive_domain_reads_binding_shaped_range_members() {
        let node = |members: Value| -> CheckedSemanticNodeV2 {
            serde_json::from_value(json!({
                "node_id": {"domain": "quire.checked-semantic-node/v1", "digest": "0".repeat(64)},
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "bounded_domain",
                "semantic_form": "integer_range",
                "semantic_type": {"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)},
                "dependencies": [],
                "occurrences": [],
                "body": {"term": "aggregate", "members": members},
            }))
            .expect("node")
        };
        let literal = |value: &str| {
            json!({"term": "literal",
                "type": {"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)},
                "value_kind": "integer", "value": value})
        };
        let bound = node(json!([
            {"term": "binding", "name": "min", "value": literal("0")},
            {"term": "binding", "name": "max", "value": literal("9")},
        ]));
        assert_eq!(
            derive_domain(&bound),
            DerivedDomain::IntegerRange {
                bound: bound.node_id.clone(),
                lower: "0".to_owned(),
                upper: "9".to_owned(),
            }
        );
        let bare = node(json!([literal("0"), literal("9")]));
        assert!(matches!(
            derive_domain(&bare),
            DerivedDomain::NotSymbolic { .. }
        ));
    }

    /// Bound members are looked up by name in any order: swapped `min`/`max` members still read
    /// the right range, while a duplicated or unknown name is not a range.
    ///
    /// Trace: FR-015-AC-5, TC-025.
    #[test]
    fn tc_025_range_members_are_read_by_name_not_position() {
        let node = |members: Value| -> CheckedSemanticNodeV2 {
            serde_json::from_value(json!({
                "node_id": {"domain": "quire.checked-semantic-node/v1", "digest": "0".repeat(64)},
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "bounded_domain",
                "semantic_form": "integer_range",
                "semantic_type": {"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)},
                "dependencies": [],
                "occurrences": [],
                "body": {"term": "aggregate", "members": members},
            }))
            .expect("node")
        };
        let member = |name: &str, value: &str| {
            json!({"term": "binding", "name": name, "value":
                {"term": "literal", "value_kind": "integer", "value": value}})
        };
        let swapped = node(json!([member("max", "9"), member("min", "0")]));
        assert_eq!(
            derive_domain(&swapped),
            DerivedDomain::IntegerRange {
                bound: swapped.node_id.clone(),
                lower: "0".to_owned(),
                upper: "9".to_owned(),
            }
        );
        // `[max=0, min=9]` written high-first is an inverted range, not the range `[0, 9]`.
        assert_eq!(
            unsatisfiable(&node(json!([member("max", "0"), member("min", "9")]))),
            Some(("9".to_owned(), "0".to_owned()))
        );
        for members in [
            json!([member("min", "0"), member("min", "9")]),
            json!([member("min", "0"), member("high", "9")]),
            json!([member("min", "0")]),
        ] {
            assert!(matches!(
                derive_domain(&node(members.clone())),
                DerivedDomain::NotSymbolic { .. }
            ));
            assert_eq!(unsatisfiable(&node(members)), None);
        }
    }

    /// Trace: FR-015-AC-5, TC-025.
    #[test]
    fn tc_025_inverted_ranges_are_unsatisfiable_and_ordered_ranges_are_not() {
        let node = |form: &str, members: Value| -> CheckedSemanticNodeV2 {
            serde_json::from_value(serde_json::json!({
                "node_id": {"domain": "quire.checked-semantic-node/v1", "digest": "0".repeat(64)},
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "bounded_domain",
                "semantic_form": form,
                "semantic_type": {"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)},
                "dependencies": [],
                "occurrences": [],
                "body": {"term": "aggregate", "members": members},
            }))
            .expect("node")
        };
        // The QSL-emitted shape: each bound member is a `binding` carrying its literal.
        let member = |name: &str, kind: &str, value: &str| {
            serde_json::json!({"term":"binding","name":name,"value":
                {"term":"literal","value_kind":kind,"value":value}})
        };
        let (lo, hi) = (
            |value: &str| member("min", "integer", value),
            |value: &str| member("max", "integer", value),
        );
        let text = |value: &str| member("text_profile", "text", value);
        assert_eq!(
            unsatisfiable(&node(
                "integer_range",
                serde_json::json!([lo("5"), hi("-5")])
            )),
            Some(("5".to_owned(), "-5".to_owned()))
        );
        assert_eq!(
            unsatisfiable(&node(
                "integer_range",
                serde_json::json!([lo("-5"), hi("5")])
            )),
            None
        );
        assert_eq!(
            unsatisfiable(&node(
                "integer_range",
                serde_json::json!([lo("5"), hi("5")])
            )),
            None
        );
        assert_eq!(
            unsatisfiable(&node(
                "text_bounds",
                serde_json::json!([lo("4"), hi("0"), text("nfc")])
            )),
            Some(("4".to_owned(), "0".to_owned()))
        );
    }

    // Deliberately untraced: no FR-015 AC states this. AC-3 ("unbounded or non-finite... refused")
    // is the closest in subject but does not describe this case -- the domain below is bounded
    // and finite, just outside `i64` -- so citing it would misdescribe what this test proves.
    //
    // `ScalarLoweringRefusal::NoCheckedBound` and `::NoIntegerDomain` were removed rather than
    // given a test here: `check_parameters`'s `Bounds::equal` (`exact_scalar.rs`) returns `Ok`
    // only after reading exactly one `integer_range` bound via the same `literal_integer` this
    // module's `derive_domain` uses on the identical node, so for `IntegerArithmetic` -- the only
    // family `ScalarOperation::of` renders -- neither an empty `checked_bounds` nor a checked bound with
    // no `IntegerRange` domain is reachable; both are now `unreachable!` invariants in
    // `lower_scalar_claim` instead of typed refusals no fixture could ever construct. This test
    // is the one cause of the original four that a fixture -- built directly against
    // `lower_scalar_claim`, not through package admission -- does reach.
    #[test]
    fn tc_026_a_domain_outside_i64_is_a_typed_refusal_not_a_panic() {
        let node_id = |digest: &str| -> CheckedNodeId {
            serde_json::from_value(serde_json::json!({
                "domain": "quire.checked-semantic-node/v1",
                "digest": digest.repeat(64),
            }))
            .expect("node id")
        };
        let bound = node_id("7");
        let claim = ExactScalarClaim {
            node_id: node_id("2"),
            operation: OperationClaim {
                identity: "quire.op.integer.add".to_owned(),
                provenance: OperationProvenance::IrConfirmed,
            },
            result: ClaimDisposition::Generated(Box::new(GeneratedScalarClaim {
                symbol: "oracle_test".to_owned(),
                ir_id: serde_json::from_value(serde_json::json!({
                    "domain": "quire.checked-semantic-node/v1",
                    "algorithm": "sha256",
                    "digest": "3".repeat(64),
                }))
                .expect("ir id"),
                semantic_form: "expression".to_owned(),
                semantic_type: node_id("4"),
                source_map: Vec::new(),
                claims: Vec::new(),
                bounds: vec![bound.clone()],
                checked_bounds: vec![bound.clone()],
                dependencies: Vec::new(),
                oracle_source: String::new(),
            })),
        };
        let ClaimDisposition::Generated(generated) = &claim.result else {
            unreachable!("built as Generated above");
        };
        let lower = "-99999999999999999999999999".to_owned();
        let upper = "99999999999999999999999999".to_owned();
        let derived = vec![DerivedDomain::IntegerRange {
            bound,
            lower: lower.clone(),
            upper: upper.clone(),
        }];
        match lower_scalar_claim(&claim, generated, &derived, &[]) {
            Err(ScalarLoweringRefusal::BoundNotI64 {
                lower: got_lower,
                upper: got_upper,
            }) => {
                assert_eq!(got_lower, lower);
                assert_eq!(got_upper, upper);
            }
            Err(ScalarLoweringRefusal::NoRenderer) => {
                panic!("quire.op.integer.add has a renderer")
            }
            Err(ScalarLoweringRefusal::ResultUnreachable { .. }) => {
                panic!("the domain does not fit i64, so no reachability is computed")
            }
            Ok(_) => panic!("an arbitrary-precision Integer domain outside i64 must not lower"),
        }
    }

    // ---- FND-003 regression: the byte-ceiling and syntax refusals split out of `RenderFailed`
    // ----
    //
    // `render`'s two checks are ordered size-then-syntax, so an oversized source never reaches
    // the syntax check; a legitimate IR fixture large enough to cross `MAX_GENERATED_SOURCE_BYTES`
    // (1 MiB) would need pathological nesting this crate has no fixture builder for. Instead these
    // tests run one real, minimal, legitimately-lowered precondition through `classify` -- the
    // same path `negotiate_kani_obligations` uses -- and then mutate `ClauseOracle.source`, the
    // exact field `render` concatenates into the generated text it measures and parses, before
    // calling `render` directly. This exercises the real check, not a reimplementation of it.

    fn render_probe_package() -> BoundPackage {
        let package_id = "test/kani-obligations-render-probe";
        let doc = json!({"document": "kani-obligations-render-probe", "revision": 1});
        let span = |line: u64| {
            json!({"start":{"source":doc,"line":line,"column":1,"byte_offset":line - 1},
                "end":{"source":doc,"line":line,"column":2,"byte_offset":line}})
        };
        let owner = json!({"package": package_id, "requirement": "FR-200", "revision": 1});
        let int_type = json!({"kind":"integer","domain":"signed","minimum":0,"maximum":1000,
            "overflow":"reject"});
        let read = |name: &str, line: u64| json!({"node":"value_reference","name":name,"observation":"current","source":span(line)});
        let identity = |kind: &str, name: &str| {
            json!({"node":"reference","identity":{"requirement":owner,"kind":kind,
                "observation":"current","path":[name]}})
        };
        let amount =
            json!({"name":"amount","kind":"input","value_type":int_type,"source":span(11)});
        let balance =
            json!({"name":"balance","kind":"state","value_type":int_type,"source":span(12)});
        let expression = json!({"node":"compare","operator":"less_equal",
            "left":read("amount", 13),"right":read("balance", 14),"source":span(11)});
        let package_clause = json!({"id":"amount-within-balance","kind":"precondition",
            "anchor":{"kind":"pre","operation":"withdraw"},"source":span(10),
            "body":{"node":"composite",
                "children":[identity("input","amount"),identity("state","balance")]}});
        let binding = json!({"clause":{"requirement":owner,"clause":"amount-within-balance"},
            "expression":{"owner":owner,"types":[],"values":[amount,balance],"functions":[],
                "expression":expression,"expected_type":{"kind":"boolean"},
                "execution_point":{"kind":"pre","operation":"withdraw"},"clause_root":true}});
        let projection = json!({
            "format": EXECUTABLE_PROJECTION_FORMAT,
            "package": {"id":package_id,"schema_version":{"major":1,"minor":1},
                "source":doc,"requirements":[{"id":"FR-200","revision":1,"source":span(1),
                    "clauses":[package_clause]}]},
            "bindings": [binding],
        });
        BoundPackage::from_json_bytes(&serde_json::to_vec(&projection).unwrap())
            .unwrap_or_else(|diagnostics| panic!("render-probe fixture must bind: {diagnostics:?}"))
    }

    fn render_probe_clause() -> ClauseRef {
        ClauseRef::new(
            RequirementRef::parse("test/kani-obligations-render-probe", "FR-200", 1).unwrap(),
            ClauseId::new("amount-within-balance").unwrap(),
        )
    }

    fn render_probe_request<'a>(items: &'a [ObligationItem<'a>]) -> KaniObligationRequest<'a> {
        KaniObligationRequest {
            items,
            subject_path: "render_probe::subject",
            unwind: 4,
        }
    }

    /// Real, legitimately-lowered `LoweredClause` for the probe package's one precondition, with
    /// its embedded oracle source intact for the caller to mutate.
    fn render_probe_lowered<'a>(item: &ObligationItem<'a>) -> Box<LoweredClause<'a>> {
        let Outcome::Lowered(lowered) = classify(item).outcome else {
            panic!("render-probe precondition must lower to a harness");
        };
        lowered
    }

    /// Trace: FR-015-AC-13, TC-025.
    #[test]
    fn render_refuses_a_generated_source_over_the_byte_ceiling() {
        let package = render_probe_package();
        let clause_ref = render_probe_clause();
        let items = [ObligationItem::BoundClause {
            package: &package,
            clause: &clause_ref,
        }];
        let request = render_probe_request(&items);
        let mut lowered = render_probe_lowered(&items[0]);
        // Oversized before the syntax check ever runs -- `render` checks byte length first, so
        // garbage content alone is enough to exercise this ground, and it stays garbage on
        // purpose to prove the length check short-circuits the syntax check.
        lowered.oracle.source = "x".repeat(MAX_GENERATED_SOURCE_BYTES + 1);
        match render(&request, &lowered) {
            Err(UnsupportedObligation::ResourceLimitExceeded { bytes }) => {
                assert!(bytes > MAX_GENERATED_SOURCE_BYTES);
            }
            Err(other) => panic!("expected ResourceLimitExceeded, got {other:?}"),
            Ok(_) => panic!("an oversized generated source must not render"),
        }
    }

    /// Trace: FR-015-AC-13, TC-025.
    #[test]
    fn render_refuses_a_generated_source_that_fails_to_parse() {
        let package = render_probe_package();
        let clause_ref = render_probe_clause();
        let items = [ObligationItem::BoundClause {
            package: &package,
            clause: &clause_ref,
        }];
        let request = render_probe_request(&items);
        let mut lowered = render_probe_lowered(&items[0]);
        // Well under the byte ceiling, but not valid Rust: an unbalanced brace `syn::parse_file`
        // rejects.
        lowered.oracle.source = "fn broken( {".to_owned();
        assert!(lowered.oracle.source.len() <= MAX_GENERATED_SOURCE_BYTES);
        match render(&request, &lowered) {
            Err(UnsupportedObligation::InvalidGeneratedSyntax { error }) => {
                assert!(!error.is_empty());
            }
            Err(other) => panic!("expected InvalidGeneratedSyntax, got {other:?}"),
            Ok(_) => panic!("a malformed generated source must not render"),
        }
    }

    /// A frame is not a clause oracle: the clause renderer refuses one by name rather than as
    /// an internal render failure.
    ///
    /// Trace: FR-015-AC-1, TC-025.
    #[test]
    fn render_refuses_a_frame_as_not_a_clause_oracle() {
        let package = render_probe_package();
        let clause_ref = render_probe_clause();
        let items = [ObligationItem::BoundClause {
            package: &package,
            clause: &clause_ref,
        }];
        let request = render_probe_request(&items);
        let mut lowered = render_probe_lowered(&items[0]);
        lowered.kind = ObligationKind::Frame;
        assert!(matches!(
            render(&request, &lowered),
            Err(UnsupportedObligation::FrameNotClauseRendered)
        ));
    }
}

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
//! [`UnsupportedObligation::OperationNotRendered`], never silently mis-rendered. A claim this
//! generator never reaches a node for (every refusal before lowering succeeds, every duplicate
//! copy, and a genuine descriptor/node disagreement) still reports the request item's own
//! descriptor-derived identity as `CallerDeclared` and is refused as
//! [`UnsupportedObligation::CallerDeclaredOperation`] with the domains the IR does carry. V1 has no
//! frame clause kind, and V2 frames have no finite encoding in the scalar profile, so no frame
//! harness is emitted.

use std::collections::{BTreeMap, BTreeSet};

use quire_contract_ir::{
    BoundClause, BoundPackage, CheckedNodeId, CheckedPackageV2, CheckedSemanticNodeV2,
    CheckedSourceMapEntry, ClauseKind, ClauseRef, DependencyIdentity, DependencyKind,
    ExecutionPoint, SourceSpan, StateObservation,
};
use serde::Serialize;

use crate::{
    exact_scalar::{aggregate_members, literal_count, literal_integer},
    kani::{
        adapter_options, i64_literal, readable_component, sha256, KaniBindingRole,
        KaniIntegerBounds, KaniPrimitiveType, KaniSolver, KANI_BACKEND_VERSION,
    },
    kani_execution::{KaniPinField, KaniToolPins},
    oracle::{
        attestation_context_is_valid, generate_oracle_with_derivation, length_delimited_identity,
        reference_identifier, typed_dependency_parameters, DependencyParameter, RustValueType,
    },
    Artifact, AttestationContext, ExactScalarClaim, ExactScalarClaimMap, ExactScalarDisposition,
    ExactScalarRefusal, GeneratedScalarClaim, GenerationErrorCode, OperationProvenance,
    OracleRequest, UpstreamBlocker, IR_CANDIDATE_REVISION, MAX_GENERATED_SOURCE_BYTES,
    RUNTIME_REVISION,
};

/// Schema identity of a generated obligation identity record.
pub const KANI_OBLIGATION_SCHEMA: &str = "quire.codegen.kani-obligation/v1";

/// Schema identity of a generated V2 exact-scalar obligation identity record.
pub const KANI_SCALAR_OBLIGATION_SCHEMA: &str = "quire.codegen.kani-scalar-obligation/v1";

/// Adapter profile for separate-obligation lowering against Kani 0.67.0.
pub const KANI_OBLIGATION_PROFILE: &str = "kani-0.67.0-separate-obligations-v1";

/// Largest number of items one request may negotiate.
pub const MAX_OBLIGATION_ITEMS: usize = 256;

/// Largest accepted loop unwind bound.
pub const MAX_OBLIGATION_UNWIND: u32 = 1024;

/// The contract role of one obligation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationKind {
    /// Lowered to a `kani::proof` that the precondition is total and satisfiable in bounds.
    Precondition,
    /// Lowered to a `kani::ensures` checked by `kani::proof_for_contract`.
    Postcondition,
    /// Lowered to a `kani::requires`/`kani::ensures` preservation contract.
    Invariant,
    /// Would lower to `kani::modifies`; no input currently reaches a supported frame.
    Frame,
}

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
        claim_map: &'a ExactScalarClaimMap,
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
    /// The backend identity harnesses are generated for and must later run under.
    pub pins: &'a KaniToolPins,
    /// Loop unwind bound, `1..=MAX_OBLIGATION_UNWIND`.
    pub unwind: u32,
    /// Binding for the embedded clause oracles' generation identity.
    pub attestation: AttestationContext<'a>,
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
    /// The requested backend is not the committed one ([`KaniToolPins::pinned`]).
    UnpinnedBackend {
        /// The first differing field.
        field: KaniPinField,
        /// The committed value.
        expected: String,
        /// The requested value.
        supplied: String,
    },
    /// The attestation binding is malformed.
    InvalidAttestationContext,
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
    /// The generated harness failed its own size or syntax check.
    RenderFailed,
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
    /// of its inclusive endpoints does not fit `i64`, so no `kani::any::<i64>()` argument can be
    /// constrained to it.
    DomainNotRepresentableInI64 {
        /// The confirmed operation.
        operation_identity: String,
        /// Inclusive lower bound, canonical decimal.
        lower: String,
        /// Inclusive upper bound, canonical decimal.
        upper: String,
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
    /// The FR-014 oracle refused for another reason.
    OracleRefused {
        /// The FR-014 refusal.
        refusal: ExactScalarRefusal,
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
    /// The node has no entry in the claim map.
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
        harness_symbol: String,
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
    /// Canonical IR expression digest.
    pub expression_digest: String,
    /// Oracle function symbol.
    pub symbol: String,
    /// Oracle source digest.
    pub sha256: String,
}

/// Everything a harness's meaning depends on; its digest is embedded in the harness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KaniObligationIdentity {
    /// [`KANI_OBLIGATION_SCHEMA`].
    pub schema: &'static str,
    /// [`KANI_OBLIGATION_PROFILE`].
    pub adapter_profile: &'static str,
    /// Contract role.
    pub kind: ObligationKind,
    /// The obligation's clause.
    pub clause: ClauseRef,
    /// Its QSL source span.
    pub source_span: SourceSpan,
    /// Canonical bound-package digest.
    pub bound_package_digest: String,
    /// Canonical declaration-environment digest.
    pub declaration_digest: String,
    /// Canonical expression digest.
    pub expression_digest: String,
    /// Contract IR revision.
    pub ir_revision: &'static str,
    /// The obligation's oracle followed by every assumed precondition's oracle.
    pub oracles: Vec<EmbeddedOracle>,
    /// SHA-256 over the length-delimited embedded oracle sources.
    pub oracle_digest: String,
    /// Harness module symbol.
    pub module_symbol: String,
    /// Proof function symbol.
    pub harness_symbol: String,
    /// Contract function symbol, for postcondition and invariant harnesses.
    pub contract_symbol: Option<String>,
    /// Customer subject, for postcondition and invariant harnesses.
    pub subject_path: Option<String>,
    /// Symbolic arguments, ascending by identifier.
    pub arguments: Vec<ObligationBinding>,
    /// Subject results, ascending by identifier.
    pub results: Vec<ObligationBinding>,
    /// Backend pins.
    pub pins: KaniToolPins,
    /// Solver.
    pub solver: String,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Every flag passed after `cargo kani`.
    pub options: Vec<String>,
    /// Contract Runtime revision the oracles call.
    pub runtime_revision: &'static str,
}

/// One generated harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniObligationHarness {
    /// Identity.
    pub identity: KaniObligationIdentity,
    /// SHA-256 of the identity's JSON, embedded in the Rust source.
    pub identity_sha256: String,
    /// Self-contained Rust source.
    pub rust: Artifact,
    /// JSON record of identity, identity digest and Rust digest.
    pub record: Artifact,
}

/// One symbolic `i64` argument of a rendered scalar harness, bounded by the IR domain the
/// lowered scalar claim carries.
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

/// Everything a V2 scalar obligation harness's meaning depends on; its digest is embedded in the
/// harness. Parallel to [`KaniObligationIdentity`] but for an IR-confirmed exact-scalar claim,
/// which has no QSL clause, declaration or subject signature -- a node id and its confirmed
/// operation identity stand in their place.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalarObligationIdentity {
    /// [`KANI_SCALAR_OBLIGATION_SCHEMA`].
    pub schema: &'static str,
    /// [`KANI_OBLIGATION_PROFILE`].
    pub adapter_profile: &'static str,
    /// The claimed node.
    pub node_id: CheckedNodeId,
    /// The node's own catalogued operation identity, IR-confirmed at package admission.
    pub operation_identity: String,
    /// Contract IR revision.
    pub ir_revision: &'static str,
    /// The embedded oracle's function symbol.
    pub oracle_symbol: String,
    /// SHA-256 of the embedded oracle's own source.
    pub oracle_sha256: String,
    /// Harness module symbol.
    pub module_symbol: String,
    /// Proof function symbol.
    pub harness_symbol: String,
    /// Symbolic arguments, in call order.
    pub arguments: Vec<ScalarObligationArgument>,
    /// Backend pins.
    pub pins: KaniToolPins,
    /// Solver.
    pub solver: String,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Every flag passed after `cargo kani`.
    pub options: Vec<String>,
    /// Contract Runtime revision the oracle calls.
    pub runtime_revision: &'static str,
}

/// One generated V2 exact-scalar obligation harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniScalarObligationHarness {
    /// Identity.
    pub identity: ScalarObligationIdentity,
    /// SHA-256 of the identity's JSON, embedded in the Rust source.
    pub identity_sha256: String,
    /// Self-contained Rust source.
    pub rust: Artifact,
    /// JSON record of identity, identity digest and Rust digest.
    pub record: Artifact,
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
    let mut states = request
        .items
        .iter()
        .map(|item| classify(request, item))
        .collect::<Vec<_>>();
    reject_duplicates_and_mixtures(request.items, &mut states);
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
                    Some(harness) => {
                        let symbol = harness.identity.harness_symbol.clone();
                        harnesses.push(harness);
                        ObligationDisposition::Supported {
                            harness_symbol: symbol,
                        }
                    }
                    None => ObligationDisposition::Unsupported {
                        reason: UnsupportedObligation::RenderFailed,
                    },
                },
                Outcome::LoweredScalar(lowered) => match render_scalar(request, &lowered) {
                    Some(harness) => {
                        let symbol = harness.identity.harness_symbol.clone();
                        scalar_harnesses.push(harness);
                        ObligationDisposition::Supported {
                            harness_symbol: symbol,
                        }
                    }
                    None => ObligationDisposition::Unsupported {
                        reason: UnsupportedObligation::RenderFailed,
                    },
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
    if let Some((field, expected, supplied)) =
        request.pins.first_difference(&KaniToolPins::pinned())
    {
        return Err(KaniObligationError::UnpinnedBackend {
            field,
            expected,
            supplied,
        });
    }
    if !attestation_context_is_valid(&request.attestation) {
        return Err(KaniObligationError::InvalidAttestationContext);
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
            Self::Lowered(lowered) => ObligationDisposition::Supported {
                harness_symbol: lowered.symbols.harness,
            },
            Self::LoweredScalar(lowered) => ObligationDisposition::Supported {
                harness_symbol: lowered.harness_symbol,
            },
            Self::RequiresBound(unbounded_type) => {
                ObligationDisposition::RequiresBound { unbounded_type }
            }
            Self::Unsupported(reason) => ObligationDisposition::Unsupported { reason },
            Self::Invalid(reason) => ObligationDisposition::InvalidRequest { reason },
        }
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
    arity: ScalarArity,
    lower: i64,
    upper: i64,
    module_symbol: String,
    harness_symbol: String,
}

#[derive(Clone, Copy)]
enum ScalarArity {
    Unary,
    Binary,
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
    expression_digest: String,
    symbol: String,
    source: String,
    sha256: String,
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

fn classify<'a>(request: &KaniObligationRequest<'_>, item: &ObligationItem<'a>) -> ItemState<'a> {
    match *item {
        ObligationItem::BoundClause { package, clause } => {
            classify_clause(request, package, clause)
        }
        ObligationItem::ScalarClaim {
            package,
            claim_map,
            node_id,
        } => classify_node(package, claim_map, node_id),
    }
}

fn classify_clause<'a>(
    request: &KaniObligationRequest<'_>,
    package: &'a BoundPackage,
    clause_ref: &ClauseRef,
) -> ItemState<'a> {
    let package_digest = package.digest().to_string();
    let identity = Some(ItemIdentity::Clause {
        package: package_digest.clone(),
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
        Some(kind) => match lower_clause(request, &package_digest, clause, kind) {
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
                        symbols: symbols(clause_ref, kind),
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
    request: &KaniObligationRequest<'_>,
    package_digest: &str,
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
        attestation: request.attestation,
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
    let bundle = generate_oracle_with_derivation(&oracle_request, Some(package_digest))
        .map_err(first_code)?;
    let symbol =
        oracle_function_symbol(&bundle.rust.contents).ok_or(UnsupportedObligation::RenderFailed)?;
    Ok(ClauseOracle {
        clause: identity.clone(),
        kind,
        anchor: clause.anchor().clone(),
        expression_digest: clause.expression_digest().to_string(),
        symbol,
        sha256: bundle.rust.sha256.clone(),
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

fn symbols(clause: &ClauseRef, kind: ObligationKind) -> Symbols {
    let requirement = clause.requirement();
    let revision = requirement.revision().get().to_string();
    let kind_text = kind_name(kind);
    let identity = length_delimited_identity(&[
        requirement.package().as_str(),
        requirement.requirement().as_str(),
        &revision,
        clause.clause().as_str(),
        kind_text,
    ]);
    // Kani derives object-file names from these symbols; keep them bounded.
    let digest = sha256(identity.as_bytes())
        .chars()
        .take(32)
        .collect::<String>();
    let base = format!(
        "kob_{}_{revision}_{}_{digest}",
        readable_component(requirement.requirement().as_str()),
        readable_component(clause.clause().as_str()),
    );
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
    claim_map: &ExactScalarClaimMap,
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
    ItemState {
        kind,
        subject,
        identity,
        outcome,
    }
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
        ExactScalarDisposition::Generated(generated) => {
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
                OperationProvenance::IrConfirmed => {
                    match lower_scalar_claim(claim, generated, &derived) {
                        Ok(lowered) => Outcome::LoweredScalar(Box::new(lowered)),
                        Err(ScalarLoweringRefusal::NoRenderer) => {
                            Outcome::Unsupported(UnsupportedObligation::OperationNotRendered {
                                operation_identity: claim.operation.identity.clone(),
                                derived_domains: derived,
                            })
                        }
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
        ExactScalarDisposition::Refused { refusal } => match refusal {
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

/// The renderable shape for a confirmed operation identity, or `None` when this generator has no
/// scalar-harness renderer for it. Recognizing these four exact catalogued identities is not a
/// vocabulary bridge -- it does not translate between codegen's own descriptor vocabulary and IR's
/// catalog to reconcile a disagreement; it only recognizes, among identities IR already confirmed,
/// which ones this generator additionally knows how to turn into a harness (the same kind of
/// closed match `Shape::of` already makes over `ExactScalarOperation`).
fn scalar_arity(operation_identity: &str) -> Option<ScalarArity> {
    match operation_identity {
        "quire.op.integer.add" | "quire.op.integer.sub" | "quire.op.integer.mul" => {
            Some(ScalarArity::Binary)
        }
        "quire.op.integer.negate" => Some(ScalarArity::Unary),
        _ => None,
    }
}

/// Why [`lower_scalar_claim`] could not lower an IR-confirmed claim -- two distinct causes that
/// [`classify_claim`] reports as two distinct [`UnsupportedObligation`] reasons, rather than
/// folding them into one the way a single `Option` return would. A third and fourth candidate
/// cause -- `check_parameters` recording no `checked_bounds` entry, or recording one whose own
/// derived domain is not an `integer_range` -- are not represented here: [`scalar_arity`] only
/// renders the `IntegerArithmetic` family, and for that family `check_parameters`'s own
/// `Bounds::equal` returns `Ok` only after successfully reading exactly one `integer_range` bound
/// (`exact_scalar::check_parameters`, `exact_scalar::Bounds::equal`) -- the same node, read by the
/// same `literal_integer`, that this function's own [`derive_domain`] call reads. Given that
/// guarantee, both failure shapes are structurally unreachable through this function today, so
/// they are asserted with `unreachable!` below rather than modelled as a caller-visible refusal a
/// test could never construct a fixture for.
enum ScalarLoweringRefusal {
    /// [`scalar_arity`] has no renderer for this operation identity.
    NoRenderer,
    /// The `integer_range`'s lower or upper endpoint does not fit `i64`.
    BoundNotI64 {
        /// Inclusive lower bound, canonical decimal.
        lower: String,
        /// Inclusive upper bound, canonical decimal.
        upper: String,
    },
}

/// Lowers one IR-confirmed V2 claim to a renderable scalar harness.
fn lower_scalar_claim(
    claim: &ExactScalarClaim,
    generated: &GeneratedScalarClaim,
    derived: &[DerivedDomain],
) -> Result<LoweredScalarClaim, ScalarLoweringRefusal> {
    let arity = scalar_arity(&claim.operation.identity).ok_or(ScalarLoweringRefusal::NoRenderer)?;
    let Some(bound_id) = generated.checked_bounds.first() else {
        unreachable!(
            "check_parameters's Bounds::equal records exactly one checked bound for every \
             IntegerArithmetic claim, the only family scalar_arity renders a harness for"
        );
    };
    let Some(DerivedDomain::IntegerRange { lower, upper, .. }) = derived.iter().find(
        |domain| matches!(domain, DerivedDomain::IntegerRange { bound, .. } if bound == bound_id),
    ) else {
        unreachable!(
            "the same node backs bound_id here and in check_parameters, which already parsed its \
             two members with the same literal_integer this function's derive_domain uses, so it \
             cannot fail to be read as an IntegerRange here"
        );
    };
    let (lower, upper) = match (lower.parse::<i64>(), upper.parse::<i64>()) {
        (Ok(lower), Ok(upper)) => (lower, upper),
        _ => {
            return Err(ScalarLoweringRefusal::BoundNotI64 {
                lower: lower.clone(),
                upper: upper.clone(),
            })
        }
    };
    let digest = sha256(claim.node_id.digest.as_bytes())
        .chars()
        .take(32)
        .collect::<String>();
    let module_symbol = format!("kob_scalar_{digest}_module");
    let harness_symbol = format!("kob_scalar_{digest}_proof");
    Ok(LoweredScalarClaim {
        node_id: claim.node_id.clone(),
        operation_identity: claim.operation.identity.clone(),
        oracle_source: generated.oracle_source.clone(),
        oracle_symbol: generated.symbol.clone(),
        arity,
        lower,
        upper,
        module_symbol,
        harness_symbol,
    })
}

fn derive_domain(bound: &CheckedSemanticNodeV2) -> DerivedDomain {
    let members = aggregate_members(&bound.body);
    match (&*bound.semantic_form, members) {
        ("integer_range", Some([lower, upper])) => {
            match (literal_integer(lower), literal_integer(upper)) {
                (Some(lower), Some(upper)) => DerivedDomain::IntegerRange {
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
    match (&*bound.semantic_form, members) {
        ("integer_range", [lower, upper]) => {
            let (lower, upper) = (literal_integer(lower)?, literal_integer(upper)?);
            (lower > upper).then(|| (lower.to_string(), upper.to_string()))
        }
        ("text_bounds" | "collection_bounds", [minimum, maximum, ..]) => {
            let (minimum, maximum) = (literal_count(minimum)?, literal_count(maximum)?);
            (minimum > maximum).then(|| (minimum.to_string(), maximum.to_string()))
        }
        _ => None,
    }
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
) -> Option<KaniObligationHarness> {
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
    let abi = abi(&contexts).ok()?;
    let exact_harness = format!("{}::{}", lowered.symbols.module, lowered.symbols.harness);
    let options = adapter_options(&exact_harness, request.unwind, KaniSolver::Cadical, false);
    let mut embedded = vec![&lowered.oracle];
    embedded.extend(&lowered.assumed);
    let oracle_sources = embedded
        .iter()
        .map(|oracle| oracle.source.as_str())
        .collect::<Vec<_>>();
    let is_contract = lowered.kind != ObligationKind::Precondition;
    let identity = KaniObligationIdentity {
        schema: KANI_OBLIGATION_SCHEMA,
        adapter_profile: KANI_OBLIGATION_PROFILE,
        kind: lowered.kind,
        clause: lowered.clause.identity().clone(),
        source_span: lowered.clause.source().clone(),
        bound_package_digest: lowered.package.digest().to_string(),
        declaration_digest: lowered.clause.declaration_digest().to_string(),
        expression_digest: lowered.oracle.expression_digest.clone(),
        ir_revision: IR_CANDIDATE_REVISION,
        oracles: embedded
            .iter()
            .map(|oracle| EmbeddedOracle {
                clause: oracle.clause.clone(),
                kind: oracle.kind,
                expression_digest: oracle.expression_digest.clone(),
                symbol: oracle.symbol.clone(),
                sha256: oracle.sha256.clone(),
            })
            .collect(),
        oracle_digest: sha256(length_delimited_identity(&oracle_sources).as_bytes()),
        module_symbol: lowered.symbols.module.clone(),
        harness_symbol: lowered.symbols.harness.clone(),
        contract_symbol: is_contract.then(|| lowered.symbols.contract.clone()),
        subject_path: is_contract.then(|| request.subject_path.to_owned()),
        arguments: abi.arguments.clone(),
        results: abi.results.clone(),
        pins: request.pins.clone(),
        solver: "cadical".to_owned(),
        unwind: request.unwind,
        options,
        runtime_revision: RUNTIME_REVISION,
    };
    let identity_json = serde_json::to_string(&identity).ok()?;
    let identity_sha256 = sha256(identity_json.as_bytes());
    let body = match lowered.kind {
        ObligationKind::Precondition => render_precondition(lowered, &abi)?,
        ObligationKind::Postcondition | ObligationKind::Invariant => {
            render_contract(request.subject_path, lowered, &abi)?
        }
        ObligationKind::Frame => return None,
    };
    let clause = lowered.clause.identity();
    let mut source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Generated by quire-contract-codegen {}; DO NOT EDIT.\n\
// Obligation: {} {}@{}/{}\n\
// Kani adapter: {KANI_OBLIGATION_PROFILE}; backend: {KANI_BACKEND_VERSION}\n\
// Obligation identity sha256: {identity_sha256}\n\n",
        env!("CARGO_PKG_VERSION"),
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
    if source.len() > MAX_GENERATED_SOURCE_BYTES || syn::parse_file(&source).is_err() {
        return None;
    }
    let rust = artifact(
        format!("src/generated/{}.rs", lowered.symbols.module),
        source,
    );
    let record = HarnessRecord {
        identity: &identity,
        identity_sha256: &identity_sha256,
        rust_path: &rust.path,
        rust_sha256: &rust.sha256,
    };
    let mut record_json = serde_json::to_string(&record).ok()?;
    record_json.push('\n');
    let record = artifact(
        format!("kani-obligations/{}.json", lowered.symbols.module),
        record_json,
    );
    Some(KaniObligationHarness {
        identity,
        identity_sha256,
        rust,
        record,
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HarnessRecord<'a> {
    identity: &'a KaniObligationIdentity,
    identity_sha256: &'a str,
    rust_path: &'a str,
    rust_sha256: &'a str,
}

/// Renders one IR-confirmed V2 exact-scalar claim to a `kani::proof` that the embedded oracle is
/// *sound*, not that it is *total*: `assert!` that whenever the outcome is
/// `Ok(rt::Outcome::Completed(value))`, `value` lies within the same checked domain -- reconstructed
/// here from the identical literal bounds the `kani::assume`s above use -- that `bound.contains(&result)`
/// (`quire_contract_runtime::exact::numeric.rs`) itself checks before returning `Completed`. This is not
/// `Result::is_ok()`: the oracle's `Result<rt::Outcome<T>, OracleStop>` nests the runtime's own four-way
/// `Outcome` (`Completed`/`Undefined`/`Refused`/`Incomplete`) inside `Ok`, so `Ok(Refused(_))` is `is_ok()`
/// without being sound-and-completed, and the `match` below only inspects the `Completed` arm; every other
/// outcome (in particular `Refused`, which is exactly what an operand pair whose exact result leaves the
/// domain produces) leaves `sound` at its vacuous default. The IR's own `require_bounds` closure typing
/// makes the *operands* here (not just the result) members of the same bounded `integer` type, so the
/// `kani::assume`s are IR-justified -- but the operation itself is genuinely partial over that domain
/// (`quire.op.integer.add` over `[-1000,1000]` admits `600 + 600`), so asserting `Completed` unconditionally
/// (this rendering's original defect) asserted a falsehood no operand assumption could fix. A `kani::cover!`
/// on the same `completed` flag is the FR-015-AC-7 non-vacuity guard (this harness's proof is reachable at
/// all), never a substitute for `sound` -- an assertion Kani cannot falsify because the harness body never
/// runs proves nothing. Unlike a V1 contract harness, there is no precondition/postcondition pair and no
/// subject signature to unify: the oracle's own parameters, in the order [`SourceBuilder::oracle`] declared
/// them (`operand` for a unary operation, `left`/`right` for a binary one), are the whole ABI.
///
/// This harness is rendered, and its proposition (above) is the one actually stated in the generated
/// source; it is NOT discharged by this generator. Nothing in this repository runs the Kani/CBMC solver
/// against it -- proving it (or finding the counterexample the partial-`add` case above predicts) requires
/// a real `cargo kani` run this generator does not perform.
fn render_scalar(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredScalarClaim,
) -> Option<KaniScalarObligationHarness> {
    let exact_harness = format!("{}::{}", lowered.module_symbol, lowered.harness_symbol);
    let options = adapter_options(&exact_harness, request.unwind, KaniSolver::Cadical, false);
    let lower = i64_literal(lowered.lower);
    let upper = i64_literal(lowered.upper);
    let names: &[&str] = match lowered.arity {
        ScalarArity::Unary => &["operand"],
        ScalarArity::Binary => &["left", "right"],
    };
    let declarations = names
        .iter()
        .map(|name| {
            format!(
                "        let {name}: i64 = kani::any();\n\
        kani::assume({name} >= {lower} && {name} <= {upper});\n\
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
        .map(|name| ScalarObligationArgument {
            identifier: (*name).to_owned(),
            minimum: lowered.lower,
            maximum: lowered.upper,
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
        let domain = rt::IntegerInterval::new(rt::Integer::from({lower}), rt::Integer::from({upper}))\n\
            .expect(\"the checked domain's own literal bounds form a valid interval\");\n\
        let sound = match &outcome {{\n\
            Ok(rt::Outcome::Completed(value)) => domain.contains(value),\n\
            _ => true,\n\
        }};\n\
        assert!(sound, \"a completed result must lie within the checked domain -- the generated oracle enforces `bound.contains(&result)` and never returns Completed for an operand pair whose exact result leaves it\");\n\
        let completed = matches!(outcome, Ok(rt::Outcome::Completed(_)));\n\
        kani::cover!(completed, \"the generated oracle's Ok(Outcome::Completed(_)) branch is reachable within its checked domain\");\n\
    }}\n\
}}\n",
        module = lowered.module_symbol,
        harness = lowered.harness_symbol,
        symbol = lowered.oracle_symbol,
    );
    let identity = ScalarObligationIdentity {
        schema: KANI_SCALAR_OBLIGATION_SCHEMA,
        adapter_profile: KANI_OBLIGATION_PROFILE,
        node_id: lowered.node_id.clone(),
        operation_identity: lowered.operation_identity.clone(),
        ir_revision: IR_CANDIDATE_REVISION,
        oracle_symbol: lowered.oracle_symbol.clone(),
        oracle_sha256: sha256(lowered.oracle_source.as_bytes()),
        module_symbol: lowered.module_symbol.clone(),
        harness_symbol: lowered.harness_symbol.clone(),
        arguments,
        pins: request.pins.clone(),
        solver: "cadical".to_owned(),
        unwind: request.unwind,
        options,
        runtime_revision: RUNTIME_REVISION,
    };
    let identity_json = serde_json::to_string(&identity).ok()?;
    let identity_sha256 = sha256(identity_json.as_bytes());
    let mut source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Generated by quire-contract-codegen {}; DO NOT EDIT.\n\
// Obligation: exact-scalar node {}/{}\n\
// Kani adapter: {KANI_OBLIGATION_PROFILE}; backend: {KANI_BACKEND_VERSION}\n\
// Obligation identity sha256: {identity_sha256}\n\n",
        env!("CARGO_PKG_VERSION"),
        lowered.node_id.domain,
        lowered.node_id.digest,
    );
    source.push_str(&lowered.oracle_source);
    source.push('\n');
    source.push_str(&body);
    if source.len() > MAX_GENERATED_SOURCE_BYTES || syn::parse_file(&source).is_err() {
        return None;
    }
    let rust = artifact(
        format!("src/generated/{}.rs", lowered.module_symbol),
        source,
    );
    let record = ScalarHarnessRecord {
        identity: &identity,
        identity_sha256: &identity_sha256,
        rust_path: &rust.path,
        rust_sha256: &rust.sha256,
    };
    let mut record_json = serde_json::to_string(&record).ok()?;
    record_json.push('\n');
    let record = artifact(
        format!("kani-obligations/{}.json", lowered.module_symbol),
        record_json,
    );
    Some(KaniScalarObligationHarness {
        identity,
        identity_sha256,
        rust,
        record,
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ScalarHarnessRecord<'a> {
    identity: &'a ScalarObligationIdentity,
    identity_sha256: &'a str,
    rust_path: &'a str,
    rust_sha256: &'a str,
}

fn artifact(path: String, contents: String) -> Artifact {
    Artifact {
        sha256: sha256(contents.as_bytes()),
        path,
        contents,
    }
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
    use serde_json::Value;

    use super::*;
    use crate::OperationClaim;

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
        let int = |value: &str| serde_json::json!({"term":"literal","value_kind":"integer","value":value});
        let text =
            |value: &str| serde_json::json!({"term":"literal","value_kind":"text","value":value});
        assert_eq!(
            unsatisfiable(&node(
                "integer_range",
                serde_json::json!([int("5"), int("-5")])
            )),
            Some(("5".to_owned(), "-5".to_owned()))
        );
        assert_eq!(
            unsatisfiable(&node(
                "integer_range",
                serde_json::json!([int("-5"), int("5")])
            )),
            None
        );
        assert_eq!(
            unsatisfiable(&node(
                "integer_range",
                serde_json::json!([int("5"), int("5")])
            )),
            None
        );
        assert_eq!(
            unsatisfiable(&node(
                "text_bounds",
                serde_json::json!([int("4"), int("0"), text("nfc")])
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
    // family `scalar_arity` renders -- neither an empty `checked_bounds` nor a checked bound with
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
            result: ExactScalarDisposition::Generated(Box::new(GeneratedScalarClaim {
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
        let ExactScalarDisposition::Generated(generated) = &claim.result else {
            unreachable!("built as Generated above");
        };
        let lower = "-99999999999999999999999999".to_owned();
        let upper = "99999999999999999999999999".to_owned();
        let derived = vec![DerivedDomain::IntegerRange {
            bound,
            lower: lower.clone(),
            upper: upper.clone(),
        }];
        match lower_scalar_claim(&claim, generated, &derived) {
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
            Ok(_) => panic!("an arbitrary-precision Integer domain outside i64 must not lower"),
        }
    }
}

//! Exact complete-V1 scalar oracle generation from CheckedPackage V2 (FR-014).
//!
//! The V2 transport names only an operator class (`binary`, `unary`,
//! `convert`) for a scalar expression, not the operation law. Each request
//! item therefore pairs a checked node with a typed [`ExactScalarOperation`]
//! built from quire-exact values and runtime-owned residue, and this module cross-checks
//! that descriptor against the lowered node before emitting anything.
//!
//! Every parameter the node can carry is checked against the IR: the
//! descriptor's integer, rational and decimal domains, IEEE rounding and text
//! bounds must equal the reachable `bounded_domain` node on the node's result
//! type, and lowering requires every reachable integer, rational, decimal and
//! text type to be bounded. CheckedPackage V2 does carry the operation
//! identity and its laws (`operation.identity`/`operation.laws`, checked at
//! admission by `validate_operations`). A claim whose node this generator
//! successfully lowers and checks against its descriptor now reads that
//! confirmed identity from `body.operation.identity` and records it as
//! [`OperationProvenance::IrConfirmed`], rather than re-deriving a string in
//! this generator's own vocabulary. A claim this generator does not confirm
//! still reports the request item's own descriptor-derived identity as
//! [`OperationProvenance::CallerDeclared`], with
//! [`UpstreamBlocker::OperationIdentityNotConsumed`]; the cases that reach it
//! are enumerated on that variant. This
//! generator's shape classifiers (`Shape::of`, `application_arguments`)
//! match the owner's retained `CheckedTerm` and closed `ApplicationOperator`,
//! borrowing typed arguments through the shared checked-graph view. They
//! cross-check the request item's descriptor; the preserved catalog identity
//! determines the claim's provenance rather than a second term decoder.
//!
//! V2 does not define a normative body for `bounded_domain` nodes. This
//! generator reads exactly one encoding and refuses anything else as
//! [`ExactScalarRefusal::UnreadableBound`]: an `aggregate` whose members are
//! literals, integers as canonical decimal strings and spellings as text.
//!
//! | Form | Members |
//! |------|---------|
//! | `integer_range` | lower, upper |
//! | `rational_range` | numerator lower, numerator upper, denominator lower, denominator upper |
//! | `decimal_range` | lower, upper, minimum scale, maximum scale, rounding |
//! | `float_rounding` | rounding |
//! | `text_bounds` | minimum length, maximum length, profile |
//!
//! Generated functions call `quire-exact` scalar operators, `quire-semantic-value`
//! quantity and enumeration operators, or Runtime Boolean operators with a caller-supplied `Meter`. Each
//! operator owns its charges; no amount is copied into the generated source.
//! An item that fails any check receives a typed
//! [`ExactScalarRefusal`] and contributes no code; its siblings are unaffected.

use super::operator_catalog::IntegerOperator;

pub(crate) mod derive;
pub(crate) mod lower;
pub(crate) mod native;
pub(crate) mod render;

use crate::core::artifact::Artifact;
use crate::core::contract_package::{
    ContractDriverResultSelection, ContractRequestPosition, PackageItems, PackageRequestError,
};
use crate::oracle::claim::{ClaimDisposition, ClaimMap, OracleGenerationError, UpstreamBlocker};

use lower::selected_scalar_key;
use quire_contract_model::{
    BoundedDomainForm, CheckedNodeId, CheckedNodeTag, CheckedPackageV2, CheckedSemanticId,
    CheckedSourceMapEntry, ContractFeatureRecordV1, ContractPackageV1, LiteralKind,
};
use quire_exact::{
    ComparisonOperator, DecimalType, DivisionProfile, IeeeComparison, IeeeWidth, IntegerDomain,
    OrderingOperator, RationalDomain, RoundingMode, TextType,
};
use quire_semantic_value::declaration::EqualityOperator;
use serde::Serialize;

/// Why a node has no derivable [`ExactScalarItem`], beyond the refusals
/// [`ExactScalarRefusal`] already names for its lowering and bounds.
///
/// Trace: TC-024
// Implements: FR-014
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ClaimDerivationRefusal {
    /// The node is not an `application` term, so it has no operation to read.
    NotApplication {
        /// The node's family.
        node_tag: String,
    },
    /// The application carries no `operation.identity`, which IR admission requires.
    MissingOperationIdentity,
    /// The identity is outside the set this generator derives.
    OperationNotDerivable {
        /// The node's `operation.identity`.
        operation_identity: String,
    },
    /// The operand or result scalar forms select no operator for the identity.
    OperandFormsNotDerivable {
        /// The node's `operation.identity`.
        operation_identity: String,
    },
    /// `operation.laws` selects no single division profile.
    LawNotDerivable {
        /// The node's `operation.identity`.
        operation_identity: String,
    },
    /// `operation.mode` is absent, or its value selects no rounding that agrees with the bound.
    ModeNotDerivable {
        /// The node's `operation.identity`.
        operation_identity: String,
    },
}

impl ClaimDerivationRefusal {
    fn refusal(self) -> ExactScalarRefusal {
        ExactScalarRefusal::NoDerivableClaim { reason: self }
    }
}

/// The operation identity of a quantity conversion.
pub(crate) const QUANTITY_CONVERT_IDENTITY: &str = "quire.op.quantity.convert";

/// Work budget for lowering one requested scalar node and its closure.
pub const SCALAR_LOWERING_WORK_LIMIT: u64 = 65_536;

/// Name of the generated crate.
pub const EXACT_SCALAR_CRATE_NAME: &str = "quire-exact-scalar-oracles";

/// The `CheckedNodeTag` families `scalar_profile`'s `CompleteLoweringProfileV2` admits.
///
/// `CheckedNodeTag::Correspondence` (cg#133) is deliberately absent: its four closed forms
/// (`source_locus`, `model_correspondence`, `binding_role`, `profile_correspondence`) are
/// provenance/binding metadata tying the checked graph back to an external source, model or
/// profile -- never a scalar expression's operand, bound, or result type -- so no node this
/// generator's shape checks reach can plausibly carry it. A real census over
/// `corpus_package().wire()`'s nodes (not a source-text grep, which the same ticket found
/// inflates unrelated tags) found zero `correspondence` nodes anywhere in the corpus, confirming
/// nothing exercises the gap either way. Correspondence oracle generation, if ever wanted, is a
/// different generator's concern -- a scalar-computation module has no natural way to emit a
/// provenance/binding claim.
///
/// Exported (cg#134) so `tests/exact_scalar_generation.rs`'s
/// `tc_024_claim_map_carries_identity_source_bounds_and_operation_per_item` can assert its own,
/// independently-constructed lowering profile's tag list against this one without calling
/// `scalar_profile` itself -- which would make that cross-check circular.
pub const SCALAR_LOWERING_SUPPORTED_TAGS: [CheckedNodeTag; 5] = [
    CheckedNodeTag::ScalarType,
    CheckedNodeTag::BoundedDomain,
    CheckedNodeTag::Value,
    CheckedNodeTag::Expression,
    CheckedNodeTag::Claim,
];

/// One requested oracle: a checked node and the operation it denotes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactScalarItem {
    /// Checked expression node to generate.
    pub node_id: CheckedNodeId,
    /// Operation law the node denotes.
    pub operation: ExactScalarOperation,
}

/// Rational `+`, `-`, `*`, `/`, unary `-`, and integer division to a rational.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RationalOperator {
    /// `a + b`.
    Add,
    /// `a - b`.
    Subtract,
    /// `a * b`.
    Multiply,
    /// `a / b`.
    Divide,
    /// `-a`.
    Negate,
    /// Integer `n / m` with a rational result.
    IntegerDivide,
}

/// Decimal `+`, `-`, `*`, `/`, unary `-`, and explicit rounding conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecimalOperator {
    /// `a + b`.
    Add,
    /// `a - b`.
    Subtract,
    /// `a * b`.
    Multiply,
    /// `-a`.
    Negate,
    /// `a / b`.
    Divide,
    /// Explicit conversion of `a` into the target type.
    Round,
}

/// IEEE binary arithmetic generated by this slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IeeeArithmeticOperator {
    /// `a + b`.
    Add,
    /// `a - b`.
    Subtract,
    /// `a * b`.
    Multiply,
    /// `a / b`.
    Divide,
}

/// Quantity arithmetic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuantityOperator {
    /// `a + b` in one unit.
    Add,
    /// `a - b` in one unit.
    Subtract,
    /// `a * b` under the compound unit.
    Multiply,
    /// `a / b` under the compound unit.
    Divide,
    /// `a ^ n` for an integer `n`.
    Power,
}

/// Representation of both ordering operands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrderingOperandKind {
    /// `Integer` or `Int[..]`.
    Integer,
    /// `Rational[..]`.
    Rational,
    /// `Decimal[..]`.
    Decimal,
}

/// The closed set of exact scalar operations this generator emits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactScalarOperation {
    /// Runtime Boolean connective over decided operands or a stopping right oracle.
    Boolean {
        /// Checked connective.
        operator: BooleanOperator,
        /// The right operand is an oracle and must be evaluated lazily.
        right_may_stop: bool,
    },
    /// The runtime's typed equality relation over scalar operands.
    Equality {
        /// Equality or inequality.
        operator: EqualityOperator,
        /// Static type of both operands.
        operands: EqualityOperandKind,
    },
    /// `evaluate_integer` in `domain`.
    IntegerArithmetic {
        /// Operator.
        operator: IntegerOperator,
        /// Consumer domain.
        domain: IntegerDomain,
    },
    /// The quotient of `divide` under a selected law.
    IntegerDivision {
        /// Division law.
        profile: DivisionProfile,
        /// Consumer domain.
        domain: IntegerDomain,
    },
    /// Euclidean `modulo`.
    IntegerModulo {
        /// Consumer domain.
        domain: IntegerDomain,
    },
    /// `evaluate_rational`, optionally bounded.
    RationalArithmetic {
        /// Operator.
        operator: RationalOperator,
        /// Consumer domain, or unbounded.
        domain: Option<RationalDomain>,
    },
    /// `evaluate_ordering`.
    Ordering {
        /// Operator.
        operator: OrderingOperator,
        /// Operand representation.
        operands: OrderingOperandKind,
    },
    /// `evaluate_decimal` into `target`.
    DecimalArithmetic {
        /// Operator.
        operator: DecimalOperator,
        /// Result type.
        target: DecimalType,
    },
    /// `evaluate_ieee` at `width` under `rounding`.
    IeeeArithmetic {
        /// Operator.
        operator: IeeeArithmeticOperator,
        /// Operand and result width.
        width: IeeeWidth,
        /// Rounding attribute.
        rounding: RoundingMode,
    },
    /// `compare_ieee` at `width`.
    IeeeComparison {
        /// Comparison intrinsic.
        comparison: IeeeComparison,
        /// Operand width.
        width: IeeeWidth,
    },
    /// `convert_ieee_width` from `source` to `target`.
    IeeeWidthConversion {
        /// Operand width.
        source: IeeeWidth,
        /// Result width.
        target: IeeeWidth,
        /// Rounding attribute.
        rounding: RoundingMode,
    },
    /// `admit_text` into `text_type`.
    TextAdmission {
        /// Admitted type.
        text_type: TextType,
    },
    /// `compare_text`.
    TextComparison {
        /// Operator.
        operator: ComparisonOperator,
    },
    /// `compare_enum`.
    EnumComparison {
        /// Operator.
        operator: ComparisonOperator,
    },
    /// Exact quantity conversion into the unit supplied at call time.
    QuantityConversion,
    /// `evaluate_quantity`.
    QuantityArithmetic {
        /// Operator.
        operator: QuantityOperator,
    },
    /// `compare_quantity`.
    QuantityComparison {
        /// Operator.
        operator: ComparisonOperator,
    },
}

/// Boolean connective named by a checked operation identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BooleanOperator {
    /// Conjunction.
    And,
    /// Disjunction.
    Or,
    /// Negation.
    Not,
    /// Implication.
    Implies,
}

/// Operand type of a checked scalar equality.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EqualityOperandKind {
    /// Boolean operands.
    Boolean,
    /// Integer operands.
    Integer,
}

/// A complete-V1 scalar type form as spelled on the V2 wire.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScalarForm {
    /// `boolean`.
    Boolean,
    /// `integer`.
    Integer,
    /// `rational`.
    Rational,
    /// `decimal`.
    Decimal,
    /// `float32`.
    Float32,
    /// `float64`.
    Float64,
    /// `text`.
    Text,
    /// `enum`.
    Enum,
    /// `unit`.
    Unit,
}

impl ScalarForm {
    fn from_literal_kind(kind: LiteralKind) -> Option<Self> {
        Some(match kind {
            LiteralKind::Boolean => Self::Boolean,
            LiteralKind::Integer => Self::Integer,
            LiteralKind::Rational => Self::Rational,
            LiteralKind::Decimal => Self::Decimal,
            LiteralKind::Float32Bits => Self::Float32,
            LiteralKind::Float64Bits => Self::Float64,
            LiteralKind::Text => Self::Text,
            LiteralKind::Enum => Self::Enum,
            LiteralKind::Quantity
            | LiteralKind::String
            | LiteralKind::Uuid
            | LiteralKind::Timestamp
            | LiteralKind::None => return None,
        })
    }

    fn of_width(width: IeeeWidth) -> Result<Self, OracleGenerationError> {
        match width {
            IeeeWidth::Binary32 => Ok(Self::Float32),
            IeeeWidth::Binary64 => Ok(Self::Float64),
        }
    }
}

/// A `bounded_domain` form this generator reads.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundForm {
    /// `integer_range`.
    IntegerRange,
    /// `rational_range`.
    RationalRange,
    /// `decimal_range`.
    DecimalRange,
    /// `float_rounding`.
    FloatRounding,
    /// `text_bounds`.
    TextBounds,
}

impl BoundForm {
    fn wire(self) -> &'static str {
        match self {
            Self::IntegerRange => "integer_range",
            Self::RationalRange => "rational_range",
            Self::DecimalRange => "decimal_range",
            Self::FloatRounding => "float_rounding",
            Self::TextBounds => "text_bounds",
        }
    }

    fn ir(self) -> BoundedDomainForm {
        match self {
            Self::IntegerRange => BoundedDomainForm::IntegerRange,
            Self::RationalRange => BoundedDomainForm::RationalRange,
            Self::DecimalRange => BoundedDomainForm::DecimalRange,
            Self::FloatRounding => BoundedDomainForm::FloatRounding,
            Self::TextBounds => BoundedDomainForm::TextBounds,
        }
    }
}

/// Where a claim's operation identity comes from.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OperationProvenance {
    /// This generator never confirmed the node's own catalogued operation
    /// against the descriptor. It did one of three things: never inspected the
    /// node; inspected it and refused the item with a typed reason; or lowered
    /// it and found the operation disagreed on the catalogued identity, the
    /// law definition or the mode value. Only the third still generates the
    /// oracle the descriptor names; what is withheld there is the
    /// confirmation, not the code.
    ///
    /// In this crate's Rust source, enumerate these cases here and nowhere
    /// else. Every copy that existed elsewhere in the source had drifted
    /// against this one and against the code by the time it was found; a
    /// second copy is how that starts again. FR-014 states the same three
    /// cases normatively, because a requirement has to; this doc describes the
    /// code as it is, so a divergence between them is a defect on one side or
    /// the other and must be resolved, not documented around.
    ///
    /// In every case the reported identity is the request item's own
    /// descriptor-derived one, and a consumer must not treat the operation law
    /// as checked.
    CallerDeclared {
        /// The missing upstream transport.
        blocked_on: UpstreamBlocker,
    },
    /// This generator successfully lowered the claim's node and checked it
    /// against the descriptor's shape, then read the node's own catalogued
    /// operation identity from `body.operation.identity` -- confirmed by
    /// Contract IR's `validate_operations` against the closed
    /// `quire.checked-operation-catalog/v1` at package admission, before
    /// this generator ever saw the package. A consumer may treat the
    /// operation identity as checked.
    IrConfirmed,
    /// Derivation refused the node (`derive_exact_scalar_items`), so no descriptor was built:
    /// nobody declared an operation and nothing was confirmed. The identity, when the node
    /// carries one, is the node's own.
    Underived,
}

/// The operation a claim-map entry is about.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OperationClaim {
    /// Stable identity of the declared operation.
    pub identity: String,
    /// Where the identity comes from.
    pub provenance: OperationProvenance,
}

/// Why one requested item generated no code.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ExactScalarRefusal {
    /// The original admitted package accounting and its exact available sources.
    PackageAccounting {
        /// The complete typed refusal, including its payload.
        record: Box<ContractFeatureRecordV1>,
        /// Source occurrences for this requested position.
        occurrences: Vec<CheckedSourceMapEntry>,
    },
    /// Selected external targets are admitted but have no scalar encoder in this backend.
    SelectedDependencyUnsupported {
        /// Every reader-validated `(package_id, target_node_id)` pair, in reached order.
        references: Vec<(CheckedSemanticId, CheckedNodeId)>,
    },
    /// The node was requested more than once; every copy is refused.
    DuplicateRequest,
    /// The node is not in the admitted graph.
    InvalidInput,
    /// A reachable node's family has no finite exact encoding.
    Unsupported {
        /// First unsupported reachable node.
        unsupported_node_id: CheckedNodeId,
        /// Its family.
        node_tag: &'static str,
    },
    /// A reachable node's family awaits upstream semantics.
    BlockedOnUpstream {
        /// First unsupported reachable node.
        unsupported_node_id: CheckedNodeId,
        /// Its family.
        node_tag: &'static str,
        /// The upstream issue.
        issue: UpstreamBlocker,
    },
    /// A reachable unbounded type has no bounding domain.
    RequiresBound {
        /// The unbounded type.
        unbounded_type: CheckedNodeId,
    },
    /// No reachable bound of the form the descriptor needs bounds the result type.
    MissingBound {
        /// The result type.
        bounded_type: CheckedNodeId,
        /// The form the descriptor needs.
        expected_form: BoundForm,
    },
    /// The caller supplied no applicable or no represented result occurrence.
    IncompleteContext,
    /// More than one reachable bound of that form bounds the result type.
    AmbiguousBound {
        /// The result type.
        bounded_type: CheckedNodeId,
        /// The repeated form.
        expected_form: BoundForm,
    },
    /// The bound's body is not the encoding this generator reads.
    UnreadableBound {
        /// The bound node.
        bound: CheckedNodeId,
    },
    /// The descriptor's parameter differs from the IR bound.
    BoundMismatch {
        /// The bound node.
        bound: CheckedNodeId,
        /// Its form.
        form: BoundForm,
    },
    /// An operand is a term other than a literal or a reference.
    OperandUnsupported {
        /// Zero-based argument position.
        position: usize,
        /// The term kind.
        term: String,
    },
    /// Recursive operand inference exceeded one of its explicit resource ceilings.
    OperandRangeLimitExceeded {
        /// The expression whose visit exceeded the ceiling.
        node_id: CheckedNodeId,
        /// The finite limit that was reached.
        resource_kind: OperandRangeLimitKind,
        /// The configured limit.
        limit: u64,
        /// The first denied count or endpoint width.
        consumed: u64,
    },
    /// A literal stands where a quantity is required; a literal carries no unit.
    UnitlessLiteralOperand {
        /// Zero-based argument position.
        position: usize,
    },
    /// A reachable body refused re-validation.
    InvalidBody {
        /// The refusing node.
        body_node_id: CheckedNodeId,
    },
    /// A reachable body stopped at a validation limit.
    BodyIncomplete {
        /// The stopped node.
        body_node_id: CheckedNodeId,
    },
    /// Lowering exceeded [`SCALAR_LOWERING_WORK_LIMIT`]; the refusal of the work ceiling and of
    /// nothing else.
    LoweringWorkExhausted {
        /// The ceiling.
        limit: u64,
        /// Counter at the failed charge.
        consumed: u64,
    },
    /// The package's byte ceiling failed the lowering (Contract IR FR-038-AC-95).
    LoweringByteLimitExceeded {
        /// The ceiling the package was read under.
        limit: u64,
        /// The canonical byte count the encoder needed.
        consumed: u64,
    },
    /// A `failed` lowering record names a limit kind that is neither `work` nor `bytes`.
    LoweringLimitUnrecognised {
        /// The snake_case name of the `CheckedPackageLimit` variant.
        limit_kind: &'static str,
        /// The record's ceiling.
        limit: u64,
        /// The record's counter.
        consumed: u64,
    },
    /// The node is not an expression.
    NotExpression {
        /// Its family.
        node_tag: &'static str,
    },
    /// The expression form does not match the descriptor.
    FormMismatch {
        /// Form the descriptor requires.
        expected: &'static str,
        /// Form on the node.
        found: String,
    },
    /// The body is not an application of the expected operator and arity.
    BodyMismatch {
        /// Application operator the descriptor requires.
        expected_operator: &'static str,
        /// Argument count the descriptor requires.
        expected_arguments: usize,
    },
    /// The node's semantic type does not match the descriptor's result.
    ResultTypeMismatch {
        /// Result form the descriptor requires.
        expected: ScalarForm,
        /// Scalar form of the node's semantic type, when it has one.
        found: Option<String>,
    },
    /// An operand's type does not match the descriptor.
    OperandTypeMismatch {
        /// Zero-based argument position.
        position: usize,
        /// Operand form the descriptor requires.
        expected: ScalarForm,
        /// Scalar form found, when the operand has one.
        found: Option<String>,
    },
    /// The right operand's application status disagrees with the descriptor's evaluation mode.
    BooleanRightOperandMismatch {
        /// Whether the request described a stopping right operand.
        declared_may_stop: bool,
        /// Whether the checked node's right operand can stop.
        actual_may_stop: bool,
    },
    /// The node's `operation.identity` member is absent, violating the IR
    /// admission invariant `check_item`'s callers otherwise rely on.
    MissingOperationIdentity {
        /// The affected node.
        node_id: CheckedNodeId,
    },
    /// The node carries no derivable claim, so no descriptor was built for it
    /// ([`derive_exact_scalar_items`]). It is recorded by the routed generation arm,
    /// never by [`generate_exact_scalar_oracles`] itself.
    NoDerivableClaim {
        /// Why derivation refused.
        reason: ClaimDerivationRefusal,
    },
}

/// A finite ceiling applied while inferring a recursive integer operand interval.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperandRangeLimitKind {
    /// Maximum nested application/value depth.
    Depth,
    /// Maximum distinct application and value nodes visited.
    Nodes,
    /// Maximum magnitude width of either exact endpoint.
    EndpointBits,
}

/// Traceability for one generated oracle.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GeneratedScalarClaim {
    /// Generated function name.
    pub symbol: String,
    /// Identity of the one Contract IR package that admitted this claim.
    pub contract_package_identity: CheckedSemanticId,
    /// Exact requested node key.
    pub node_id: CheckedNodeId,
    /// Identity of the originating checked package.
    pub package_id: CheckedSemanticId,
    /// Expression form.
    pub semantic_form: String,
    /// Semantic type key.
    pub semantic_type: CheckedNodeId,
    /// Admitted conversion type-argument identity, when this is a quantity conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_conversion_target: Option<CheckedNodeId>,
    /// Exact source correspondence.
    pub source_map: Vec<CheckedSourceMapEntry>,
    /// Reachable claim keys.
    pub claims: Vec<CheckedNodeId>,
    /// Reachable bounding-domain keys.
    pub bounds: Vec<CheckedNodeId>,
    /// The bounds the descriptor's parameters were checked equal to, ascending.
    pub checked_bounds: Vec<CheckedNodeId>,
    /// Every reachable key.
    pub dependencies: Vec<CheckedNodeId>,
    /// This oracle's function, self-contained (its own preamble and any
    /// helper it needs) for direct embedding in a generated Kani harness
    /// file. Process-internal: not part of the `quire.codegen.exact-scalar-
    /// claim-map/v1` wire schema (this struct has no `Deserialize`, so
    /// nothing round-trips it through `claim-map.json`), consumed only by
    /// `kani::generate::scalar::render_scalar` from the live claim map this
    /// generation produced.
    #[serde(skip)]
    pub oracle_source: String,
    /// Whether a generated Boolean oracle's right parameter is a lazy outcome closure.
    #[serde(skip)]
    pub boolean_right_may_stop: bool,
}

/// One claim-map entry per selected requested occurrence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExactScalarClaim {
    /// Requested node.
    pub node_id: CheckedNodeId,
    /// The declared operation and its provenance.
    pub operation: OperationClaim,
    /// Outcome.
    pub result: ClaimDisposition<GeneratedScalarClaim, ExactScalarRefusal>,
}

/// Generation output: the crate files and the claim map they are described by.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactScalarOracles {
    /// `Cargo.toml`, `src/lib.rs`, `source-maps/lib.json` and `claim-map.json`, in that order.
    pub artifacts: Vec<Artifact>,
    /// Typed claim map, identical to `claim-map.json`. Items ascend by node
    /// id: digest domain, then digest; selected occurrence breaks equal-node ties.
    /// `blocked` lists the upstream gaps every
    /// entry is subject to. A `Generated` disposition means one oracle
    /// function was emitted. The blockers this generator records are
    /// `QuireSpecLanguage120`, `QuireContractRuntime34` and
    /// `OperationIdentityNotConsumed`.
    pub claim_map: ClaimMap<ExactScalarClaim>,
}

/// A checked, typed scalar lowering plan before artifact rendering.
///
/// This is the shared authority for Kani settlement and oracle generation: its claims are
/// derived by lowering the admitted package and checking each operation descriptor against the
/// checked node. Rendering is a separate step and cannot change the plan's disposition.
#[derive(Clone, Debug)]
pub struct ExactScalarPlan {
    package_id: CheckedSemanticId,
    claims: Vec<ExactScalarClaim>,
    pending: Vec<PendingOraclePlan>,
}

#[derive(Clone, Debug)]
struct PendingOraclePlan {
    claim: usize,
    identity: String,
    confirmed: bool,
    operation: ExactScalarOperation,
    expected_consequents: u32,
}

impl ExactScalarPlan {
    /// The typed per-node claims that settlement classifies before rendering.
    pub fn claims(&self) -> &[ExactScalarClaim] {
        &self.claims
    }
}

/// Derives one [`ExactScalarItem`] per node id, in input order, from the package alone.
///
/// Each descriptor is read from the node's `operation.identity`, `operation.laws`,
/// `operation.mode`, its operand scalar forms and the one bound of the needed form on its result
/// type (the selector [`generate_exact_scalar_oracles`] checks the descriptor against). A node
/// that yields no descriptor is the [`ExactScalarRefusal`] `generate_exact_scalar_oracles` would
/// give its lowering or bound (so FR-015 classifies it as before), or `NoDerivableClaim` carrying a
/// [`ClaimDerivationRefusal`].
///
/// Trace: TC-024
// Implements: FR-014
pub fn derive_exact_scalar_items(
    package: &CheckedPackageV2,
    node_ids: &[CheckedNodeId],
) -> Vec<Result<ExactScalarItem, ExactScalarRefusal>> {
    derive::derive_exact_scalar_items(package, node_ids)
}

/// Build the complete checked scalar plan without publishing generated artifacts.
pub fn plan_exact_scalar_oracles(
    package: &CheckedPackageV2,
    items: &[ExactScalarItem],
) -> Result<ExactScalarPlan, OracleGenerationError> {
    lower::plan_exact_scalar_oracles(package, items)
}

/// Render an already checked scalar plan into oracle artifacts.
pub fn render_exact_scalar_plan(
    plan: ExactScalarPlan,
) -> Result<ExactScalarOracles, OracleGenerationError> {
    render::render_exact_scalar_plan(plan)
}

/// Generate exact scalar oracles for `items` from an admitted package.
///
/// Fails as a whole only with `SourceTooLarge`, `ClaimMapSerialization`, or
/// `UnknownRuntimeVariant` (any of the roughly two dozen scalar sites that
/// match an RT `#[non_exhaustive]` enum, in `check_parameters` and in shape,
/// identity and rendering, against a variant the generator's own match does
/// not know); every per-item problem is a refusal in the claim map.
pub fn generate_exact_scalar_oracles(
    package: &CheckedPackageV2,
    items: &[ExactScalarItem],
) -> Result<ExactScalarOracles, OracleGenerationError> {
    render_exact_scalar_plan(plan_exact_scalar_oracles(package, items)?)
}

pub(crate) use render::publish_native_scalar_artifacts;
pub(crate) use render::render_clause_native_operation_helpers;

/// A whole-call failure before admitted scalar claims can be published.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractScalarError {
    /// The caller's request sequence disagrees with the admitted accounting.
    Request(PackageRequestError),
    /// Rendering or publication failed.
    Oracle(OracleGenerationError),
}

/// Generate scalar oracles from the exact reached FR-330 graph.
///
/// This arm handles scalar operations with no result-bound parameter. Bound-
/// parameterized families retain a typed refusal until their derivation reads
/// the admitted closure's typed bounds.
/// The returned claim indexes align with `positions` and identify each selected
/// occurrence after the claim map's stable sort.
pub(crate) fn generate_contract_scalar_oracles(
    package: &ContractPackageV1,
    requested: &[ContractRequestPosition],
    positions: &[usize],
) -> Result<(ExactScalarOracles, Vec<usize>), ContractScalarError> {
    render::build_contract_scalar_oracles(
        package,
        requested,
        positions,
        render::ScalarBuildMode::Render,
    )
}

/// Derive a selected scalar claim for capability preflight without rendering artifacts.
pub(crate) fn preflight_contract_scalar_claim(
    package: &ContractPackageV1,
    requested: &[ContractRequestPosition],
    request_index: usize,
) -> Result<ExactScalarClaim, ContractScalarError> {
    let (oracles, indexes) = render::build_contract_scalar_oracles(
        package,
        requested,
        &[request_index],
        render::ScalarBuildMode::Preflight,
    )?;
    indexes
        .first()
        .and_then(|index| oracles.claim_map.items.get(*index))
        .cloned()
        .ok_or(ContractScalarError::Request(
            PackageRequestError::Position { request_index },
        ))
}

pub(crate) fn valid_selected_scalar_result(
    package: &ContractPackageV1,
    items: &PackageItems<'_>,
    request_index: usize,
    node_id: &CheckedNodeId,
    selection: &ContractDriverResultSelection,
) -> bool {
    matches!(selection, ContractDriverResultSelection::OwnResult { .. })
        && selected_scalar_key(package, items, request_index, node_id, selection).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(digit: char) -> CheckedNodeId {
        serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": digit.to_string().repeat(64),
        }))
        .expect("node id")
    }

    #[test]
    fn generated_scalar_claim_serializes_package_identity_and_requested_key() {
        let identity: CheckedSemanticId = serde_json::from_value(serde_json::json!({
            "domain": "quire.contract-ir.semantic/v1",
            "algorithm": "sha256",
            "digest": "a".repeat(64),
        }))
        .expect("package identity");
        let generated = GeneratedScalarClaim {
            symbol: "oracle_integer_add".to_owned(),
            contract_package_identity: identity.clone(),
            node_id: node_id('b'),
            package_id: identity,
            semantic_form: "binary".to_owned(),
            semantic_type: node_id('c'),
            quantity_conversion_target: None,
            source_map: Vec::new(),
            claims: Vec::new(),
            bounds: Vec::new(),
            checked_bounds: Vec::new(),
            dependencies: Vec::new(),
            oracle_source: String::new(),
            boolean_right_may_stop: false,
        };
        let value = serde_json::to_value(generated).expect("claim serializes");
        assert_eq!(value["node_id"], serde_json::json!(node_id('b')));
        assert!(value.get("contract_package_identity").is_some());
        assert!(value.get("package_id").is_some());
        assert!(value.get("ir_id").is_none());
        for key in ["claims", "bounds", "dependencies", "checked_bounds"] {
            assert_eq!(value[key], serde_json::json!([]), "{key}");
        }
    }
}

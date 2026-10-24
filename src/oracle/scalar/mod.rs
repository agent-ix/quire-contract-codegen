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

use super::operator_catalog::{IntegerOperator, Operator as CatalogOperator};

pub(crate) mod derive;
pub(crate) mod lower;
pub(crate) mod render;

use crate::core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES};
use crate::core::contract_package::{
    ContractDriverResultSelection, ContractRequestPosition, PackageItem, PackageItems,
    PackageRequestError,
};
use crate::core::naming::{bounded_readable_component, unique_names};
use crate::core::profile::oracle_crate_manifest;
use crate::core::source_map::SourceProbe;
use crate::oracle::claim::{ClaimDisposition, ClaimMap, OracleGenerationError, UpstreamBlocker};
use crate::oracle::{classify_lowering_failure, direct_unsupported_form, LoweringFailure};
use derive::conversion_target;

use lower::{
    catalogued_operation_identity, check_item, contract_checked_operation, operation_confirmed,
    operation_identity, operation_name, refused_claim, selected_scalar_key, ItemCheckError,
};
use num_bigint::BigInt;
use quire_contract_model::{
    ApplicationOperator, BoundedDomainForm, CheckedNodeId, CheckedNodeTag, CheckedOccurrenceRole,
    CheckedPackageV2, CheckedSemanticId, CheckedSemanticNodeV2, CheckedSourceMapEntry, CheckedTerm,
    CompleteContractNodeV2, CompleteLoweringProfileV2, CompleteLoweringRecordV2,
    ContractFeatureRecordV1, ContractOccurrenceKeyV1, ContractPackageV1, ContractResultContextV1,
    ContractResultViewV1, ContractRootViewError, LiteralKind,
};
use quire_exact::{
    ComparisonOperator, DecimalType, DivisionProfile, IeeeComparison, IeeeWidth, IntegerDomain,
    IntegerInterval, OrderingOperator, RationalDomain, RoundingMode, TextProfile, TextType,
};
use quire_semantic_value::declaration::EqualityOperator;
use render::{scalar_coverage_regions, standalone_oracle_source, SourceBuilder};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

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

fn node_implication_count<Body: GraphBody, NodeBody>(
    graph: &Graph<'_, Body>,
    node: &CheckedSemanticNodeV2<NodeBody>,
) -> u32 {
    u32::from(
        graph
            .operation(&node.node_id)
            .and_then(|op| op.get("identity"))
            .and_then(Value::as_str)
            == Some("quire.op.boolean.implies"),
    )
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

/// Build the complete checked scalar plan without publishing generated artifacts.
pub fn plan_exact_scalar_oracles(
    package: &CheckedPackageV2,
    items: &[ExactScalarItem],
) -> Result<ExactScalarPlan, OracleGenerationError> {
    let mut requests: BTreeMap<&CheckedNodeId, Vec<&ExactScalarOperation>> = BTreeMap::new();
    for item in items {
        requests
            .entry(&item.node_id)
            .or_default()
            .push(&item.operation);
    }
    let requested = requests.keys().map(|id| (*id).clone()).collect::<Vec<_>>();
    let lowering = package.lower(&requested, &scalar_profile());
    let graph = Graph::from_checked(package);

    let mut claims = Vec::with_capacity(requests.len());
    let mut pending: Vec<PendingOraclePlan> = Vec::new();
    for ((node_id, operations), record) in requests.into_iter().zip(&lowering.records) {
        let operation_claim = match operations.as_slice() {
            [operation] => match check_item(&graph, node_id, record, operation) {
                Ok((node, checked_bounds)) => {
                    // This node was lowered from an admitted package, so IR-216's
                    // `validate_operations` already confirmed `body.operation.identity`
                    // against the closed catalog before this generator ever saw it. That
                    // confirms the node's own operation, not that it is the one the
                    // descriptor named: `check_item`'s shape checks alone cannot
                    // distinguish, say, `add` from `mul` (both are a binary
                    // `&rt::Integer` shape) or truncating from floor division, so
                    // `operation_confirmed` additionally compares the node's own
                    // `operation.identity`/`operation.mode` against the descriptor's
                    // implied catalog entry, plus the one law family with more than
                    // one catalogued definition. Confirmed, this generator reads the
                    // node's own identity rather than re-deriving codegen's own
                    // descriptor string; not confirmed, it reports the descriptor's
                    // own string exactly as before this generator ever read
                    // `operation.identity` -- never a silent preference between the
                    // two when they disagree.
                    // The node's own identity is read before confirmation, not
                    // behind it. `operation_confirmed` returns false for a node
                    // missing `operation.identity` just as it does for one whose
                    // identity simply disagrees, so asking it first collapsed a
                    // violated upstream invariant into the ordinary
                    // caller-declared answer -- reporting that codegen had not
                    // consumed an identity the node does not carry. Reading it
                    // first keeps the two apart: absent is refused, present and
                    // disagreeing is caller-declared.
                    let confirmed_identity = match catalogued_operation_identity(&graph, node) {
                        Ok(catalogued) => {
                            Ok(operation_confirmed(&graph, node, operation)?.then_some(catalogued))
                        }
                        Err(refusal) => Err(refusal),
                    };
                    match confirmed_identity {
                        Err(refusal) => refused_claim(operation_identity(operation)?, refusal),
                        Ok(maybe_identity) => {
                            let confirmed = maybe_identity.is_some();
                            let (identity, provenance) = match maybe_identity {
                                Some(identity) => (identity, OperationProvenance::IrConfirmed),
                                None => (
                                    operation_identity(operation)?,
                                    OperationProvenance::CallerDeclared {
                                        blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
                                    },
                                ),
                            };
                            pending.push(PendingOraclePlan {
                                claim: claims.len(),
                                identity: identity.clone(),
                                confirmed,
                                operation: (*operation).clone(),
                                expected_consequents: node_implication_count(&graph, &node.node),
                            });
                            (
                                OperationClaim {
                                    identity,
                                    provenance,
                                },
                                ClaimDisposition::Generated(Box::new(GeneratedScalarClaim {
                                    symbol: String::new(),
                                    contract_package_identity: lowering
                                        .package
                                        .wire()
                                        .ok_or(OracleGenerationError::ContractPackageUnavailable)?
                                        .identity()
                                        .clone(),
                                    node_id: node_id.clone(),
                                    package_id: lowering.package.source_package_id().clone(),
                                    semantic_form: node.node.semantic_form.to_string(),
                                    semantic_type: node.semantic_type.clone(),
                                    quantity_conversion_target: conversion_target(
                                        &graph, &node.node,
                                    ),
                                    source_map: node.source_map.clone(),
                                    claims: node.claims.clone(),
                                    bounds: node.bounds.clone(),
                                    checked_bounds,
                                    dependencies: node.dependencies.clone(),
                                    oracle_source: String::new(),
                                    boolean_right_may_stop: matches!(
                                        operation,
                                        ExactScalarOperation::Boolean {
                                            right_may_stop: true,
                                            ..
                                        }
                                    ),
                                })),
                            )
                        }
                    }
                }
                Err(ItemCheckError::Refusal(refusal)) => {
                    refused_claim(operation_identity(operation)?, refusal)
                }
                // An RT `#[non_exhaustive]` enum yielded a variant
                // `check_parameters`'s own exhaustive match does not know:
                // this is not this one item's business refusal, so it aborts
                // the whole generation instead (see
                // `OracleGenerationError::UnknownRuntimeVariant`'s own doc).
                Err(ItemCheckError::Generation(error)) => return Err(error),
            },
            // Every copy is refused. The entry is named by the least identity
            // so that it does not depend on which copy arrived first.
            copies => {
                let identities = copies
                    .iter()
                    .map(|op| operation_identity(op))
                    .collect::<Result<Vec<_>, _>>()?;
                let Some(identity) = identities.into_iter().min() else {
                    continue;
                };
                refused_claim(identity, ExactScalarRefusal::DuplicateRequest)
            }
        };
        let (claim_operation, result) = operation_claim;
        claims.push(ExactScalarClaim {
            node_id: node_id.clone(),
            operation: claim_operation,
            result,
        });
    }
    Ok(ExactScalarPlan {
        package_id: lowering.package.source_package_id().clone(),
        claims,
        pending,
    })
}

/// Render an already checked scalar plan into oracle artifacts.
pub fn render_exact_scalar_plan(
    mut plan: ExactScalarPlan,
) -> Result<ExactScalarOracles, OracleGenerationError> {
    let mut source = SourceBuilder::default();
    // Each generated oracle is named by its operation; oracles sharing an operation are numbered
    // in node-id order, so a refused or differently requested sibling never renames one.
    let names = unique_names(
        plan.pending
            .iter()
            .map(|oracle| {
                Ok((
                    format!("oracle_{}", operation_name(&oracle.operation)?),
                    plan.claims[oracle.claim].node_id.clone(),
                ))
            })
            .collect::<Result<Vec<_>, OracleGenerationError>>()?,
    );
    for (oracle, symbol) in plan.pending.iter().zip(names) {
        source.oracle(&symbol, &oracle.identity, &oracle.operation)?;
        if let ClaimDisposition::Generated(generated) = &mut plan.claims[oracle.claim].result {
            if oracle.confirmed {
                generated.oracle_source =
                    standalone_oracle_source(&symbol, &oracle.identity, &oracle.operation)?;
            }
            generated.symbol = symbol;
        }
    }

    let claim_map = ClaimMap {
        package_id: plan.package_id,
        // No blocker applies to every entry any more: a confirmed claim's
        // provenance is `IrConfirmed`, and an unconfirmed claim's
        // `CallerDeclared` provenance names
        // `UpstreamBlocker::OperationIdentityNotConsumed` per-item, whichever
        // of the cases on `OperationProvenance::CallerDeclared` applies.
        blocked: Vec::new(),
        items: plan.claims,
    };
    let subjects = claim_map
        .items
        .iter()
        .filter(|claim| matches!(&claim.result, ClaimDisposition::Generated(_)))
        .map(|claim| crate::core::provenance::node_subject(&claim.node_id))
        .collect::<Vec<_>>();
    let lib = crate::core::provenance::with_rust_header(
        source.finish(),
        subjects.iter().map(String::as_str),
    );
    if lib.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(OracleGenerationError::SourceTooLarge {
            bytes: lib.len(),
            limit: MAX_GENERATED_SOURCE_BYTES,
            setting: "cg.scalar.source_bytes",
        });
    }
    let coverage_regions = scalar_coverage_regions(
        &lib,
        &claim_map,
        plan.pending
            .iter()
            .map(|oracle| (oracle.claim, &oracle.operation, oracle.expected_consequents)),
    )?;
    let mut coverage_bytes = serde_json::to_vec_pretty(&coverage_regions)
        .map_err(|_| OracleGenerationError::LocationMapSerialization)?;
    coverage_bytes.push(b'\n');
    let coverage_text = String::from_utf8(coverage_bytes)
        .map_err(|_| OracleGenerationError::LocationMapSerialization)?;
    let mut map_bytes = serde_json::to_vec_pretty(&claim_map)
        .map_err(|_| OracleGenerationError::ClaimMapSerialization)?;
    map_bytes.push(b'\n');
    let map_text =
        String::from_utf8(map_bytes).map_err(|_| OracleGenerationError::ClaimMapSerialization)?;
    Ok(ExactScalarOracles {
        artifacts: vec![
            Artifact::new("Cargo.toml", oracle_crate_manifest(EXACT_SCALAR_CRATE_NAME)),
            Artifact::new("src/lib.rs", lib),
            Artifact::new("source-maps/lib.json", coverage_text),
            Artifact::new("claim-map.json", map_text),
        ],
        claim_map,
    })
}

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
    build_contract_scalar_oracles(package, requested, positions, ScalarBuildMode::Render)
}

/// Derive a selected scalar claim for capability preflight without rendering artifacts.
pub(crate) fn preflight_contract_scalar_claim(
    package: &ContractPackageV1,
    requested: &[ContractRequestPosition],
    request_index: usize,
) -> Result<ExactScalarClaim, ContractScalarError> {
    let (oracles, indexes) = build_contract_scalar_oracles(
        package,
        requested,
        &[request_index],
        ScalarBuildMode::Preflight,
    )?;
    indexes
        .first()
        .and_then(|index| oracles.claim_map.items.get(*index))
        .cloned()
        .ok_or(ContractScalarError::Request(
            PackageRequestError::Position { request_index },
        ))
}

#[derive(Clone, Copy)]
enum ScalarBuildMode {
    Preflight,
    Render,
}

fn build_contract_scalar_oracles(
    package: &ContractPackageV1,
    requested: &[ContractRequestPosition],
    positions: &[usize],
    mode: ScalarBuildMode,
) -> Result<(ExactScalarOracles, Vec<usize>), ContractScalarError> {
    let items = PackageItems::new(package, requested).map_err(ContractScalarError::Request)?;
    let graph = Graph::new(package);
    let mut source = SourceBuilder::default();
    let mut claims = Vec::with_capacity(positions.len());
    let mut pending = Vec::new();
    let mut counts = BTreeMap::new();
    let mut selected_keys = Vec::with_capacity(positions.len());
    for &request_index in positions {
        let node_id = items
            .bound_driver_node(request_index)
            .map_err(ContractScalarError::Request)?;
        let selection =
            items
                .result_selection(request_index)
                .ok_or(ContractScalarError::Request(
                    PackageRequestError::Position { request_index },
                ))?;
        let key = selected_scalar_key(package, &items, request_index, node_id, selection);
        if let Some(key) = &key {
            *counts.entry(key.clone()).or_insert(0usize) += 1;
        }
        selected_keys.push(key);
    }
    for (&request_index, selected_key) in positions.iter().zip(&selected_keys) {
        let node_id = items
            .bound_driver_node(request_index)
            .map_err(ContractScalarError::Request)?;
        let item = items
            .at(request_index, node_id)
            .map_err(ContractScalarError::Request)?;
        let (identity, result) = match item {
            PackageItem::Refused {
                record,
                occurrences,
            } => (
                String::new(),
                ClaimDisposition::Refused {
                    refusal: ExactScalarRefusal::PackageAccounting {
                        record: Box::new(record.clone()),
                        occurrences: occurrences.to_vec(),
                    },
                },
            ),
            PackageItem::Lowered { closure, .. } => {
                let root_sources = closure
                    .nodes
                    .iter()
                    .find(|entry| entry.node.node_id == *node_id)
                    .map(|entry| entry.sources)
                    .ok_or_else(|| {
                        ContractScalarError::Request(PackageRequestError::ReachedClosure {
                            request_index,
                            node_id: node_id.clone(),
                            cause: Box::new(ContractRootViewError::Inconsistent),
                        })
                    })?;
                let identity = graph
                    .operation(&closure.root.node_id)
                    .and_then(|operation| operation.get("identity"))
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned();
                let tag = CheckedNodeTag::from_wire(&closure.root.node_tag);
                if selected_key.is_none() {
                    (
                        identity,
                        ClaimDisposition::Refused {
                            refusal: ExactScalarRefusal::IncompleteContext,
                        },
                    )
                } else if selected_key
                    .as_ref()
                    .is_some_and(|key| counts.get(key).copied().unwrap_or(0) > 1)
                {
                    (
                        identity,
                        ClaimDisposition::Refused {
                            refusal: ExactScalarRefusal::DuplicateRequest,
                        },
                    )
                } else if !closure.selected_references.is_empty() {
                    (
                        identity,
                        ClaimDisposition::Refused {
                            refusal: ExactScalarRefusal::SelectedDependencyUnsupported {
                                references: closure
                                    .selected_references
                                    .into_iter()
                                    .map(|reference| (reference.package, reference.node))
                                    .collect(),
                            },
                        },
                    )
                } else {
                    let selection = items.result_selection(request_index).ok_or(
                        ContractScalarError::Request(PackageRequestError::Position {
                            request_index,
                        }),
                    )?;
                    match contract_checked_operation(
                        &graph, &closure, tag, node_id, package, selection,
                    )
                    .map_err(ContractScalarError::Oracle)?
                    {
                        Ok((operation, checked_bounds)) => {
                            let dependencies = closure
                                .nodes
                                .iter()
                                .filter(|entry| entry.node.node_id != *node_id)
                                .map(|entry| entry.node.node_id.clone())
                                .collect();
                            pending.push((
                                claims.len(),
                                operation.clone(),
                                node_implication_count(&graph, closure.root),
                            ));
                            (
                                identity,
                                ClaimDisposition::Generated(Box::new(GeneratedScalarClaim {
                                    symbol: String::new(),
                                    contract_package_identity: package.identity().clone(),
                                    node_id: node_id.clone(),
                                    package_id: package.wire().package.source.package_id.clone(),
                                    semantic_form: closure.root.semantic_form.to_string(),
                                    semantic_type: closure.root.semantic_type.clone(),
                                    quantity_conversion_target: conversion_target(
                                        &graph,
                                        closure.root,
                                    ),
                                    source_map: root_sources.to_vec(),
                                    claims: closure.claims,
                                    bounds: closure.bounds,
                                    checked_bounds,
                                    dependencies,
                                    oracle_source: String::new(),
                                    boolean_right_may_stop: matches!(
                                        operation,
                                        ExactScalarOperation::Boolean {
                                            right_may_stop: true,
                                            ..
                                        }
                                    ),
                                })),
                            )
                        }
                        Err(refusal) => (identity, ClaimDisposition::Refused { refusal }),
                    }
                }
            }
        };
        let provenance = if matches!(&result, ClaimDisposition::Generated(_)) {
            OperationProvenance::IrConfirmed
        } else {
            OperationProvenance::Underived
        };
        claims.push(ExactScalarClaim {
            node_id: node_id.clone(),
            operation: OperationClaim {
                identity,
                provenance,
            },
            result,
        });
    }
    // Emit in selected-occurrence order, including two requests over one
    // expression node. Request permutation cannot rename or reorder source.
    pending.sort_by(|left, right| selected_keys[left.0].cmp(&selected_keys[right.0]));
    let names = unique_names(
        pending
            .iter()
            .map(|(claim, operation, _)| {
                Ok((
                    format!("oracle_{}", operation_name(operation)?),
                    selected_keys[*claim].clone(),
                ))
            })
            .collect::<Result<Vec<_>, OracleGenerationError>>()
            .map_err(ContractScalarError::Oracle)?,
    );
    for ((claim, operation, _), symbol) in pending.iter().zip(names) {
        if matches!(mode, ScalarBuildMode::Preflight) {
            continue;
        }
        let claim_entry = &mut claims[*claim];
        let identity = &claim_entry.operation.identity;
        source
            .oracle(&symbol, identity, operation)
            .map_err(ContractScalarError::Oracle)?;
        if let ClaimDisposition::Generated(generated) = &mut claim_entry.result {
            generated.oracle_source = standalone_oracle_source(&symbol, identity, operation)
                .map_err(ContractScalarError::Oracle)?;
            generated.symbol = symbol;
        }
    }
    // FR-014's claim-map order is node order. The selected pair breaks ties
    // over one expression node without consulting driver request indexes.
    let mut keyed_claims = claims
        .into_iter()
        .zip(selected_keys)
        .enumerate()
        .map(|(position, (claim, key))| (position, claim, key))
        .collect::<Vec<_>>();
    keyed_claims.sort_by(|(_, left, left_key), (_, right, right_key)| {
        left.node_id
            .cmp(&right.node_id)
            .then(left_key.cmp(right_key))
    });
    let mut claim_indexes = vec![0; positions.len()];
    let claims = keyed_claims
        .into_iter()
        .enumerate()
        .map(|(claim_index, (position, claim, _))| {
            claim_indexes[position] = claim_index;
            claim
        })
        .collect();
    let claim_map = ClaimMap {
        package_id: package.wire().package.source.package_id.clone(),
        blocked: Vec::new(),
        items: claims,
    };
    if matches!(mode, ScalarBuildMode::Preflight) {
        return Ok((
            ExactScalarOracles {
                artifacts: Vec::new(),
                claim_map,
            },
            claim_indexes,
        ));
    }
    let subjects = claim_map
        .items
        .iter()
        .filter(|claim| matches!(&claim.result, ClaimDisposition::Generated(_)))
        .map(|claim| crate::core::provenance::node_subject(&claim.node_id))
        .collect::<Vec<_>>();
    let lib = crate::core::provenance::with_rust_header(
        source.finish(),
        subjects.iter().map(String::as_str),
    );
    if lib.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(ContractScalarError::Oracle(
            OracleGenerationError::SourceTooLarge {
                bytes: lib.len(),
                limit: MAX_GENERATED_SOURCE_BYTES,
                setting: "cg.scalar.source_bytes",
            },
        ));
    }
    let coverage_regions = scalar_coverage_regions(
        &lib,
        &claim_map,
        pending
            .iter()
            .map(|(claim, operation, count)| (claim_indexes[*claim], operation, *count)),
    )
    .map_err(ContractScalarError::Oracle)?;
    let mut coverage_bytes = serde_json::to_vec_pretty(&coverage_regions).map_err(|_| {
        ContractScalarError::Oracle(OracleGenerationError::LocationMapSerialization)
    })?;
    coverage_bytes.push(b'\n');
    let coverage_text = String::from_utf8(coverage_bytes).map_err(|_| {
        ContractScalarError::Oracle(OracleGenerationError::LocationMapSerialization)
    })?;
    let mut map_bytes = serde_json::to_vec_pretty(&claim_map)
        .map_err(|_| ContractScalarError::Oracle(OracleGenerationError::ClaimMapSerialization))?;
    map_bytes.push(b'\n');
    let map_text = String::from_utf8(map_bytes)
        .map_err(|_| ContractScalarError::Oracle(OracleGenerationError::ClaimMapSerialization))?;
    Ok((
        ExactScalarOracles {
            artifacts: vec![
                Artifact::new("Cargo.toml", oracle_crate_manifest(EXACT_SCALAR_CRATE_NAME)),
                Artifact::new("src/lib.rs", lib),
                Artifact::new("source-maps/lib.json", coverage_text),
                Artifact::new("claim-map.json", map_text),
            ],
            claim_map,
        },
        claim_indexes,
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
mod source_selection_tests;

#[cfg(test)]
mod source_selection_work {
    use std::cell::Cell;

    #[derive(Clone, Copy, Debug, Default)]
    pub(super) struct Work {
        pub(super) whole_map_visits: usize,
        pub(super) indexed_lookups: usize,
        pub(super) application_occurrences: usize,
        pub(super) conversion_occurrences: usize,
    }

    thread_local! {
        static WORK: Cell<Work> = const { Cell::new(Work {
            whole_map_visits: 0,
            indexed_lookups: 0,
            application_occurrences: 0,
            conversion_occurrences: 0,
        }) };
    }

    // The historical qualification attaches false to each executed whole-map
    // predicate; the retained path attaches true to each indexed API call.
    pub(super) fn source(indexed: bool) {
        WORK.with(|work| {
            let mut current = work.get();
            if indexed {
                current.indexed_lookups += 1;
            } else {
                current.whole_map_visits += 1;
            }
            work.set(current);
        });
    }

    pub(super) fn occurrence(conversion: bool) {
        WORK.with(|work| {
            let mut current = work.get();
            if conversion {
                current.conversion_occurrences += 1;
            } else {
                current.application_occurrences += 1;
            }
            work.set(current);
        });
    }

    pub(super) fn take() -> Work {
        WORK.with(|work| work.replace(Work::default()))
    }
}

fn scalar_profile() -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
        reference: quire_contract_model::LoweringProfileRef {
            authority: "agent-ix".into(),
            identity: "quire.codegen.exact-scalar/v1".into(),
        },
        supported_tags: BTreeSet::from(SCALAR_LOWERING_SUPPORTED_TAGS),
        require_bounds: true,
        work_limit: SCALAR_LOWERING_WORK_LIMIT,
    }
}

use crate::oracle::checked_graph::{
    aggregate_members, bound_members, literal, literal_count, literal_integer, term_name,
    CheckedGraph as Graph, GraphBody,
};

/// The lowered node and the bounds its descriptor was checked against.
#[cfg(test)]
mod tests {
    use super::derive::{derive_operation, family_of_identity, Derivation, Family};
    use super::lower::{
        catalogued_operation, check_operand, infer_integer_range, lowered, narrows,
        read_decimal_range, read_integer_range, read_rational_range, read_text_bounds,
        unsupported_family, BoundView, Bounds, NARROW_IDENTITY,
    };
    use super::*;
    type Graph<'a> = crate::oracle::checked_graph::CheckedGraph<'a, CheckedTerm>;
    use crate::oracle::checked_graph::fixture::Node as FixtureNode;
    use ix_trace_rs::trace;
    use quire_contract_model::{CheckedPackageIncomplete, CheckedPackageLimit};

    #[trace("FR-014-AC-3")]
    #[test]
    fn hybrid_family_is_unsupported_and_preserves_node_identity() {
        let node = node_id('b');
        assert_eq!(
            unsupported_family(&node, CheckedNodeTag::Hybrid),
            ExactScalarRefusal::Unsupported {
                unsupported_node_id: node,
                node_tag: "hybrid",
            }
        );
    }

    fn node_id(digit: char) -> CheckedNodeId {
        serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": digit.to_string().repeat(64),
        }))
        .expect("node id")
    }

    fn numbered_node_id(value: u64) -> CheckedNodeId {
        serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": format!("{value:064x}"),
        }))
        .expect("numbered node id")
    }

    fn inference_node(
        id: CheckedNodeId,
        tag: &str,
        form: &str,
        semantic_type: CheckedNodeId,
        body: impl Into<Option<CheckedTerm>>,
    ) -> FixtureNode {
        FixtureNode::new(
            CheckedSemanticNodeV2 {
                node_id: id,
                owner: None,
                schema_version: "quire.checked-semantic-graph/v2".into(),
                node_tag: tag.into(),
                semantic_form: form.into(),
                semantic_type,
                dependencies: Vec::new(),
                occurrences: Vec::new(),
                recursion_group: None,
                nominal_identity_preimage: None,
                declaration: None,
                body: CheckedTerm::aggregate(Vec::new()),
            },
            body.into(),
        )
    }

    fn inference_application_node(
        id: CheckedNodeId,
        integer_type: &CheckedNodeId,
        operation: &str,
        arguments: Vec<CheckedTerm>,
    ) -> FixtureNode {
        inference_node(
            id,
            "expression",
            "binary",
            integer_type.clone(),
            CheckedTerm::Application {
                operator: ApplicationOperator::from_wire(if operation.ends_with("negate") {
                    "unary"
                } else {
                    "binary"
                })
                .expect("closed fixture kind"),
                operation: serde_json::json!({"identity": operation}),
                result_type: node_id('0'),
                arguments,
            },
        )
    }

    fn reference(id: &CheckedNodeId) -> CheckedTerm {
        CheckedTerm::Reference { target: id.clone() }
    }

    fn inference_graph(nodes: &[FixtureNode]) -> Graph<'_> {
        nodes.iter().map(|node| (&node.node_id, node)).collect()
    }

    /// Admission normally prevents malformed cycles and references; the inference boundary still
    /// reports each cause distinctly if a damaged graph reaches it.
    #[trace("FR-014-AC-46", "TC-024")]
    #[test]
    fn tc_024_nested_inference_reports_missing_cycle_unsupported_and_bad_literals() {
        let integer = node_id('0');
        let scalar = inference_node(
            integer.clone(),
            "scalar_type",
            "integer",
            integer.clone(),
            None,
        );
        let missing = node_id('b');
        let root_id = node_id('a');
        let root = inference_application_node(
            root_id.clone(),
            &integer,
            "quire.op.integer.negate",
            vec![reference(&missing)],
        );
        let nodes = [scalar.clone(), root];
        let graph = inference_graph(&nodes);
        assert_eq!(
            infer_integer_range(&graph, &root_id, 2),
            Err(ExactScalarRefusal::InvalidBody {
                body_node_id: missing.clone()
            })
        );

        // A target-less Reference cannot be constructed through CheckedTerm. An absent
        // retained body still preserves the private InvalidBody/root-key defense.
        let missing_body = inference_node(
            root_id.clone(),
            "expression",
            "binary",
            integer.clone(),
            None,
        );
        let nodes = [scalar.clone(), missing_body];
        let graph = inference_graph(&nodes);
        assert_eq!(
            infer_integer_range(&graph, &root_id, 2),
            Err(ExactScalarRefusal::InvalidBody {
                body_node_id: root_id.clone()
            })
        );

        let cycle = inference_application_node(
            root_id.clone(),
            &integer,
            "quire.op.integer.negate",
            vec![reference(&root_id)],
        );
        let nodes = [scalar.clone(), cycle];
        let graph = inference_graph(&nodes);
        assert_eq!(
            infer_integer_range(&graph, &root_id, 0),
            Err(ExactScalarRefusal::InvalidBody {
                body_node_id: root_id.clone()
            })
        );

        let unsupported = inference_application_node(
            root_id.clone(),
            &integer,
            "quire.op.integer.divide",
            Vec::new(),
        );
        let nodes = [scalar.clone(), unsupported];
        let graph = inference_graph(&nodes);
        assert_eq!(
            infer_integer_range(&graph, &root_id, 1),
            Err(ExactScalarRefusal::OperandUnsupported {
                position: 1,
                term: "quire.op.integer.divide".to_owned()
            })
        );

        for (literal_value, found) in [("text", Some("text")), ("01", Some("integer"))] {
            let literal_id = node_id('c');
            let value_kind = if literal_value == "text" {
                "text"
            } else {
                "integer"
            };
            let literal = inference_node(
                literal_id.clone(),
                "value",
                "literal",
                integer.clone(),
                CheckedTerm::Literal {
                    type_id: node_id('0'),
                    value_kind: LiteralKind::from_wire(value_kind).expect("closed fixture kind"),
                    value: serde_json::json!(literal_value),
                },
            );
            let unary = inference_application_node(
                root_id.clone(),
                &integer,
                "quire.op.integer.negate",
                vec![reference(&literal_id)],
            );
            let nodes = [scalar.clone(), literal, unary];
            let graph = inference_graph(&nodes);
            assert_eq!(
                infer_integer_range(&graph, &root_id, 3),
                Err(ExactScalarRefusal::OperandTypeMismatch {
                    position: 3,
                    expected: ScalarForm::Integer,
                    found: found.map(str::to_owned),
                })
            );
        }
    }

    #[trace("FR-014-AC-45", "FR-014-AC-46", "TC-024")]
    #[test]
    fn tc_024_nested_inference_accepts_inline_canonical_integer_literals() {
        let integer = node_id('0');
        let bounded = node_id('1');
        let parameter = node_id('2');
        let root = node_id('3');
        let scalar = inference_node(
            integer.clone(),
            "scalar_type",
            "integer",
            integer.clone(),
            None,
        );
        let bounded_type = inference_node(
            bounded.clone(),
            "bounded_domain",
            "integer_range",
            integer.clone(),
            CheckedTerm::Aggregate {
                members: vec![
                    CheckedTerm::Binding {
                        name: ("min").into(),
                        value: Box::new(CheckedTerm::Literal {
                            type_id: node_id('0'),
                            value_kind: LiteralKind::Integer,
                            value: serde_json::json!("0"),
                        }),
                    },
                    CheckedTerm::Binding {
                        name: ("max").into(),
                        value: Box::new(CheckedTerm::Literal {
                            type_id: node_id('0'),
                            value_kind: LiteralKind::Integer,
                            value: serde_json::json!("9"),
                        }),
                    },
                ],
            },
        );
        let value = inference_node(parameter.clone(), "value", "parameter", bounded, None);
        let application = inference_application_node(
            root.clone(),
            &integer,
            "quire.op.integer.add",
            vec![
                reference(&parameter),
                CheckedTerm::Literal {
                    type_id: node_id('0'),
                    value_kind: LiteralKind::Integer,
                    value: serde_json::json!("1"),
                },
            ],
        );
        let nodes = [scalar, bounded_type, value, application];
        let graph = inference_graph(&nodes);
        assert_eq!(
            infer_integer_range(&graph, &root, 0),
            Ok(("1".to_owned(), "10".to_owned()))
        );
    }

    #[trace("FR-014-AC-46", "FR-015-AC-104", "TC-024", "TC-033")]
    #[test]
    fn tc_024_nested_inference_enforces_depth_before_reusing_memoized_nodes() {
        let integer = node_id('0');
        let bounded = node_id('1');
        let parameter = node_id('2');
        let shared = node_id('3');
        let root = numbered_node_id(200);
        let chain = (101..=163).map(numbered_node_id).collect::<Vec<_>>();
        let scalar = inference_node(
            integer.clone(),
            "scalar_type",
            "integer",
            integer.clone(),
            None,
        );
        let bounded_type = inference_node(
            bounded.clone(),
            "bounded_domain",
            "integer_range",
            integer.clone(),
            CheckedTerm::Aggregate {
                members: vec![
                    CheckedTerm::Binding {
                        name: ("min").into(),
                        value: Box::new(CheckedTerm::Literal {
                            type_id: node_id('0'),
                            value_kind: LiteralKind::Integer,
                            value: serde_json::json!("0"),
                        }),
                    },
                    CheckedTerm::Binding {
                        name: ("max").into(),
                        value: Box::new(CheckedTerm::Literal {
                            type_id: node_id('0'),
                            value_kind: LiteralKind::Integer,
                            value: serde_json::json!("9"),
                        }),
                    },
                ],
            },
        );
        let value = inference_node(parameter.clone(), "value", "parameter", bounded, None);
        let shared_expression = inference_application_node(
            shared.clone(),
            &integer,
            "quire.op.integer.add",
            vec![
                reference(&parameter),
                CheckedTerm::Literal {
                    type_id: node_id('0'),
                    value_kind: LiteralKind::Integer,
                    value: serde_json::json!("1"),
                },
            ],
        );
        let mut nodes = vec![scalar, bounded_type, value, shared_expression];
        for (index, node_id) in chain.iter().enumerate() {
            let next = chain.get(index + 1).unwrap_or(&shared);
            nodes.push(inference_application_node(
                node_id.clone(),
                &integer,
                "quire.op.integer.add",
                vec![reference(&parameter), reference(next)],
            ));
        }
        nodes.push(inference_application_node(
            root.clone(),
            &integer,
            "quire.op.integer.add",
            vec![reference(&shared), reference(&chain[0])],
        ));
        let graph = inference_graph(&nodes);
        assert_eq!(
            infer_integer_range(&graph, &root, 0),
            Err(ExactScalarRefusal::OperandRangeLimitExceeded {
                node_id: shared,
                resource_kind: OperandRangeLimitKind::Depth,
                limit: 64,
                consumed: 65,
            })
        );
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

    /// Admission re-validates every body before lowering, so an admitted package
    /// cannot produce these records; they are built directly.
    #[trace("FR-014-AC-3", "TC-024")]
    #[test]
    fn tc_024_invalid_and_incomplete_bodies_are_typed_refusals() {
        let refusal = match quire_contract_model::CheckedPackageV2::read(
            b"{}",
            quire_contract_model::CheckedPackageReadLimits::bounded(),
            &quire_contract_model::CheckedPackageEvidence::new(),
        ) {
            quire_contract_model::CheckedPackageV2ReadResult::Refused(refusal) => refusal,
            other => panic!("an empty package is refused by the reader: {other:?}"),
        };
        let invalid = CompleteLoweringRecordV2::InvalidBody {
            node_id: node_id('a'),
            body_node_id: node_id('b'),
            refusal,
        };
        assert_eq!(
            lowered(&invalid).err(),
            Some(ExactScalarRefusal::InvalidBody {
                body_node_id: node_id('b')
            })
        );
        let incomplete = CompleteLoweringRecordV2::BodyIncomplete {
            node_id: node_id('a'),
            body_node_id: node_id('c'),
            incomplete: CheckedPackageIncomplete {
                limit_kind: CheckedPackageLimit::Nodes,
                limit: 128,
                consumed: 129,
                path: None,
            },
        };
        assert_eq!(
            lowered(&incomplete).err(),
            Some(ExactScalarRefusal::BodyIncomplete {
                body_node_id: node_id('c')
            })
        );
    }

    /// A `failed` record is refused by its limit kind through the one shared classifier: `work`
    /// as `LoweringWorkExhausted`, `bytes` as `LoweringByteLimitExceeded` with the record's own
    /// `limit` and `consumed` (the per-node case Contract IR's FR-038-AC-95 defines and CG's
    /// public API cannot reach), and every other kind as `LoweringLimitUnrecognised` under its
    /// snake_case name, never as work exhaustion and without a panic.
    #[trace("FR-014-AC-40", "FR-014-AC-41", "TC-024")]
    #[test]
    fn tc_024_a_failed_record_is_refused_by_its_limit_kind() {
        use crate::oracle::failed_records::{failed_record, unrecognised_kinds};

        assert_eq!(
            lowered(&failed_record(CheckedPackageLimit::Work, 65_536, 65_537)).err(),
            Some(ExactScalarRefusal::LoweringWorkExhausted {
                limit: 65_536,
                consumed: 65_537
            })
        );
        assert_eq!(
            lowered(&failed_record(CheckedPackageLimit::Bytes, 1_000, 1_001)).err(),
            Some(ExactScalarRefusal::LoweringByteLimitExceeded {
                limit: 1_000,
                consumed: 1_001
            })
        );
        for (kind, name) in unrecognised_kinds() {
            assert_eq!(
                lowered(&failed_record(kind, 7, 9)).err(),
                Some(ExactScalarRefusal::LoweringLimitUnrecognised {
                    limit_kind: name,
                    limit: 7,
                    consumed: 9
                }),
                "{name}"
            );
        }
    }

    /// quire-contract-ir's `validate_operations` requires `operation.identity`
    /// on every application node before admission, so no admitted package can
    /// carry a node without it and this node is built directly -- the same
    /// reason the record cases above are.
    ///
    /// The guard is still worth compiling. That admission check is recent
    /// (IR-216) and is itself the kind of thing that can regress; if it did,
    /// the node would arrive here. Before IR-224 this refusal was unreachable,
    /// because `operation_confirmed` was consulted first and answers `false`
    /// for a missing member exactly as it does for a disagreeing one -- so a
    /// violated upstream invariant was reported as
    /// `CallerDeclared { OperationIdentityNotConsumed }`, blaming this
    /// generator for not consuming an identity the node never carried.
    ///
    /// What this test covers and what it does not: it calls
    /// [`catalogued_operation_identity`] directly, so it pins that function's
    /// refusal and would pass under either call order. The reordering in
    /// `generate_exact_scalar_oracles` is **not** covered by any test, and
    /// cannot be: reaching it needs an admitted package containing an
    /// application node with no `operation.identity`, which is the very thing
    /// admission refuses to produce. Do not read this test as evidence that the
    /// generator refuses such a node end to end.
    #[trace("FR-014-AC-3", "TC-024")]
    #[test]
    fn tc_024_catalogued_operation_identity_refuses_a_node_whose_operation_has_no_identity() {
        let inner: CheckedSemanticNodeV2 = serde_json::from_value(serde_json::json!({
            "node_id": {
                "domain": "quire.checked-semantic-node/v1",
                "digest": "d".repeat(64),
            },
            "schema_version": "quire.checked-semantic-graph/v2",
            "node_tag": "expression",
            "semantic_form": "application",
            "semantic_type": {
                "domain": "quire.checked-semantic-node/v1",
                "digest": "e".repeat(64),
            },
            "dependencies": [],
            "occurrences": [],
            // `operation` present, `operation.identity` absent: the exact shape
            // admission is supposed to make impossible.
            "body": {
                "term": "application",
                "operator": "binary",
                "operation": { "mode": { "kind": "rounding", "value": "truncate" } },
                "result_type": {
                    "domain": "quire.checked-semantic-node/v1",
                    "digest": "e".repeat(64),
                },
                "arguments": [],
            },
        }))
        .expect("a node whose operation carries no identity");
        let node = CompleteContractNodeV2 {
            node: inner,
            node_tag: CheckedNodeTag::Expression,
            source_map: vec![],
            semantic_type: node_id('e'),
            dependencies: vec![],
            bounds: vec![],
            claims: vec![],
            ir_id: serde_json::from_value(serde_json::json!({
                "domain": "quire.contract-ir.semantic/v1",
                "algorithm": "sha-256",
                "digest": "f".repeat(64),
            }))
            .expect("an ir id"),
        };

        let fixture = FixtureNode::new(
            node.node.clone(),
            Some(app_term(serde_json::json!({
                "mode": { "kind": "rounding", "value": "truncate" }
            }))),
        );
        let key = fixture.node_id.clone();
        let graph: Graph<'_> = [(&key, &fixture)].into_iter().collect();
        assert_eq!(
            catalogued_operation_identity(&graph, &node),
            Err(ExactScalarRefusal::MissingOperationIdentity {
                node_id: node_id('d'),
            }),
            "a node missing the member IR admission guarantees must be refused by name, \
             not reported as an identity this generator declined to consume"
        );
    }
    // -- QSL shape (FR-322): bounded_domain-typed parameters and binding-shaped bounds ---------

    fn v2_node(
        digit: char,
        tag: &str,
        form: &str,
        semantic_type: char,
        body: impl Into<Option<CheckedTerm>>,
    ) -> FixtureNode {
        inference_node(node_id(digit), tag, form, node_id(semantic_type), body)
    }

    /// A `{term: binding, name, value: <literal>}` member, the shape QSL emits and IR reads;
    /// the literal carries `type` as FR-322 requires.
    fn binding_member(name: &str, value: &str) -> CheckedTerm {
        CheckedTerm::Binding { name: (name).into(), value: Box::new(CheckedTerm::Literal { type_id: serde_json::from_value(serde_json::json!({"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)})).expect("fixture type key"), value_kind: LiteralKind::Integer, value: serde_json::json!(value) }) }
    }

    /// A `text`-kind binding member.
    fn text_member(name: &str, value: &str) -> CheckedTerm {
        CheckedTerm::Binding { name: (name).into(), value: Box::new(CheckedTerm::Literal { type_id: serde_json::from_value(serde_json::json!({"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)})).expect("fixture type key"), value_kind: LiteralKind::Text, value: serde_json::json!(value) }) }
    }

    fn interval_of(lower: &str, upper: &str) -> IntegerInterval {
        IntegerInterval::new(
            lower.parse().expect("integer"),
            upper.parse().expect("integer"),
        )
        .expect("interval")
    }

    fn range_body(members: Vec<CheckedTerm>) -> CheckedTerm {
        CheckedTerm::Aggregate { members }
    }

    /// Nodes `1` (Integer scalar type), `2` (`Int[0,9]`, an `integer_range` bounded_domain over
    /// `1`), `3` (a parameter typed by `2`), `4` (Text scalar type) and `5` (a parameter typed by
    /// a `text_bounds` bounded_domain `6` over `4`).
    fn bounded_parameter_nodes() -> Vec<FixtureNode> {
        vec![
            v2_node('1', "scalar_type", "integer", '1', range_body(vec![])),
            v2_node(
                '2',
                "bounded_domain",
                "integer_range",
                '1',
                range_body(vec![binding_member("min", "0"), binding_member("max", "9")]),
            ),
            v2_node('3', "value", "parameter", '2', range_body(vec![])),
            v2_node('4', "scalar_type", "text", '4', range_body(vec![])),
            v2_node(
                '6',
                "bounded_domain",
                "text_bounds",
                '4',
                range_body(vec![]),
            ),
            v2_node('5', "value", "parameter", '6', range_body(vec![])),
        ]
    }

    fn reference_to(digit: char) -> CheckedTerm {
        CheckedTerm::Reference {
            target: serde_json::from_value(serde_json::json!({
                "domain": "quire.checked-semantic-node/v1", "digest": digit.to_string().repeat(64),
            }))
            .expect("fixture type key"),
        }
    }

    /// A conversion narrows a node only with the catalogued narrow identity and exactly one
    /// argument, a reference to that node. IR refuses a two-argument narrow at admission, so this
    /// reaches the check through a raw graph node.
    #[trace("FR-014-AC-27", "TC-024")]
    #[test]
    fn tc_024_a_conversion_narrows_only_its_one_referenced_argument() {
        let target = node_id('2');
        let conversion = |digit, identity: &str, arguments: Vec<CheckedTerm>| {
            v2_node(
                digit,
                "expression",
                "conversion",
                '9',
                CheckedTerm::Application {
                    operator: ApplicationOperator::Convert,
                    operation: serde_json::json!({"identity": identity}),
                    result_type: node_id('0'),
                    arguments,
                },
            )
        };
        let narrows_fixture = |node: &FixtureNode, target: &CheckedNodeId| {
            let graph: Graph<'_> = [(&node.node_id, node)].into_iter().collect();
            narrows(&graph, node, target)
        };
        assert!(narrows_fixture(
            &conversion('a', NARROW_IDENTITY, vec![reference_to('2')]),
            &target
        ));
        assert!(!narrows_fixture(
            &conversion(
                'b',
                NARROW_IDENTITY,
                vec![reference_to('2'), reference_to('3')]
            ),
            &target
        ));
        assert!(!narrows_fixture(
            &conversion('c', "quire.op.numeric.convert", vec![reference_to('2')]),
            &target
        ));
        assert!(!narrows_fixture(
            &conversion('d', NARROW_IDENTITY, vec![reference_to('3')]),
            &target
        ));
    }

    /// A reference to a parameter typed by a `bounded_domain` is classified by the domain's base
    /// scalar type, not refused as `found: None`.
    #[trace("FR-014-AC-17", "TC-024")]
    #[test]
    fn tc_024_reference_typed_by_a_bounded_domain_is_classified_by_its_base_scalar_type() {
        let nodes = bounded_parameter_nodes();
        let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
        assert_eq!(
            check_operand(&graph, 0, &reference_to('3'), ScalarForm::Integer),
            Ok(())
        );
        assert_eq!(
            check_operand(&graph, 1, &reference_to('5'), ScalarForm::Integer),
            Err(ExactScalarRefusal::OperandTypeMismatch {
                position: 1,
                expected: ScalarForm::Integer,
                found: Some("text".to_owned()),
            }),
            "a bounded text parameter is a text operand, not an integer one"
        );
    }

    /// The bound is read from binding-shaped members (`min`, `max`); the bare-literal shape this
    /// generator once read is refused as unreadable.
    #[trace("FR-014-AC-16", "TC-024")]
    #[test]
    fn tc_024_integer_range_is_read_from_binding_members_and_refuses_bare_literals() {
        let bound_id = node_id('2');
        let mut nodes = bounded_parameter_nodes();
        let bare = |value: &str| CheckedTerm::Literal {
            type_id: node_id('0'),
            value_kind: LiteralKind::Integer,
            value: serde_json::json!(value),
        };
        nodes.push(v2_node(
            '7',
            "bounded_domain",
            "integer_range",
            '1',
            range_body(vec![bare("0"), bare("9")]),
        ));
        let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
        let members = |digit: char| {
            aggregate_members(graph.term(&node_id(digit)))
                .expect("aggregate body")
                .to_vec()
        };
        assert_eq!(
            read_integer_range(&members('2')),
            Some(interval_of("0", "9"))
        );
        assert_eq!(read_integer_range(&members('7')), None);

        // Through `Bounds::equal`, as `check_parameters` reads it: the descriptor parameter equals
        // the binding-shaped bound, and the bare one is `UnreadableBound`.
        let node = |bound: char| CompleteContractNodeV2 {
            node: v2_node('8', "expression", "application", '1', range_body(vec![])).metadata,
            node_tag: CheckedNodeTag::Expression,
            source_map: vec![],
            semantic_type: node_id('1'),
            dependencies: vec![],
            bounds: vec![node_id(bound)],
            claims: vec![],
            ir_id: serde_json::from_value(serde_json::json!({
                "domain": "quire.contract-ir.semantic/v1",
                "algorithm": "sha-256",
                "digest": "f".repeat(64),
            }))
            .expect("an ir id"),
        };
        let declared = interval_of("0", "9");
        let (good, bad) = (node('2'), node('7'));
        assert_eq!(
            Bounds {
                graph: &graph,
                node: BoundView::complete(&good)
            }
            .equal(BoundForm::IntegerRange, read_integer_range, Some(&declared)),
            Ok(bound_id)
        );
        assert_eq!(
            Bounds {
                graph: &graph,
                node: BoundView::complete(&bad)
            }
            .equal(BoundForm::IntegerRange, read_integer_range, Some(&declared)),
            Err(ExactScalarRefusal::UnreadableBound {
                bound: node_id('7')
            })
        );
    }

    /// Bound members are looked up by name, in any order (QSpec's reader does the same); a
    /// missing, duplicate or unknown name is refused.
    #[trace("FR-014-AC-16", "TC-024")]
    #[test]
    fn tc_024_bound_members_are_looked_up_by_name_in_any_order() {
        // Swapped integer range.
        assert_eq!(
            read_integer_range(&[binding_member("max", "9"), binding_member("min", "0")]),
            Some(interval_of("0", "9"))
        );
        // A shuffled rational range still puts numerator and denominator in their own intervals.
        let rational = read_rational_range(&[
            binding_member("denominator_max", "7"),
            binding_member("numerator_min", "-3"),
            binding_member("denominator_min", "2"),
            binding_member("numerator_max", "5"),
        ])
        .expect("shuffled rational range");
        assert_eq!(
            rational,
            RationalDomain::new(interval_of("-3", "5"), interval_of("2", "7")).expect("domain")
        );
        // A shuffled decimal range and text bounds.
        let decimal = read_decimal_range(&[
            text_member("rounding", "nearest-even"),
            binding_member("scale_max", "2"),
            binding_member("coefficient_max", "1000"),
            binding_member("scale_min", "0"),
            binding_member("coefficient_min", "-1000"),
        ])
        .expect("shuffled decimal range");
        assert_eq!(
            decimal,
            DecimalType::new(
                "-1000".parse().expect("integer"),
                "1000".parse().expect("integer"),
                0,
                2,
                RoundingMode::NearestEven,
            )
            .expect("decimal type")
        );
        assert!(read_text_bounds(&[
            text_member("text_profile", "nfc"),
            binding_member("max", "64"),
            binding_member("min", "0"),
        ])
        .is_some());
        // Refusals: duplicate, wrong, missing and extra names.
        assert_eq!(
            read_integer_range(&[binding_member("min", "0"), binding_member("min", "9")]),
            None
        );
        assert_eq!(
            read_integer_range(&[binding_member("min", "0"), binding_member("high", "9")]),
            None
        );
        assert_eq!(read_integer_range(&[binding_member("min", "0")]), None);
        assert_eq!(
            read_integer_range(&[
                binding_member("min", "0"),
                binding_member("max", "9"),
                binding_member("max", "9"),
            ]),
            None
        );
        // A rational range whose members carry the integer-range names is not a rational range.
        assert_eq!(
            read_rational_range(&[
                binding_member("min", "0"),
                binding_member("max", "9"),
                binding_member("min", "1"),
                binding_member("max", "2"),
            ]),
            None
        );
    }
    // -- Derivation (IR-294) ---------------------------------------------------------------

    /// One descriptor per catalogued identity the forward map can name.
    fn every_operation() -> Vec<ExactScalarOperation> {
        use ExactScalarOperation as Op;
        let domain = IntegerDomain::Mathematical;
        let decimal = DecimalType::new(0_i64.into(), 9_i64.into(), 0, 0, RoundingMode::NearestEven)
            .expect("type");
        let text = TextType::new(0, 4, TextProfile::Nfc).expect("text");
        let mut all = Vec::new();
        for operator in [
            BooleanOperator::And,
            BooleanOperator::Or,
            BooleanOperator::Not,
            BooleanOperator::Implies,
        ] {
            all.push(Op::Boolean {
                operator,
                right_may_stop: false,
            });
        }
        for operands in [EqualityOperandKind::Boolean, EqualityOperandKind::Integer] {
            for operator in [EqualityOperator::Equal, EqualityOperator::NotEqual] {
                all.push(Op::Equality { operator, operands });
            }
        }
        for operator in [
            IntegerOperator::Add,
            IntegerOperator::Subtract,
            IntegerOperator::Multiply,
            IntegerOperator::Negate,
        ] {
            all.push(Op::IntegerArithmetic {
                operator,
                domain: domain.clone(),
            });
        }
        for profile in DivisionProfile::ALL {
            all.push(Op::IntegerDivision {
                profile,
                domain: domain.clone(),
            });
        }
        all.push(Op::IntegerModulo { domain });
        for operator in [
            RationalOperator::Add,
            RationalOperator::Subtract,
            RationalOperator::Multiply,
            RationalOperator::Negate,
        ] {
            all.push(Op::RationalArithmetic {
                operator,
                domain: None,
            });
        }
        for operator in [RationalOperator::Divide, RationalOperator::IntegerDivide] {
            all.push(Op::RationalArithmetic {
                operator,
                domain: None,
            });
        }
        for operator in [
            OrderingOperator::Less,
            OrderingOperator::LessOrEqual,
            OrderingOperator::Greater,
            OrderingOperator::GreaterOrEqual,
        ] {
            for operands in [
                OrderingOperandKind::Integer,
                OrderingOperandKind::Rational,
                OrderingOperandKind::Decimal,
            ] {
                all.push(Op::Ordering { operator, operands });
            }
        }
        for operator in [
            DecimalOperator::Add,
            DecimalOperator::Subtract,
            DecimalOperator::Multiply,
            DecimalOperator::Negate,
            DecimalOperator::Divide,
            DecimalOperator::Round,
        ] {
            all.push(Op::DecimalArithmetic {
                operator,
                target: decimal.clone(),
            });
        }
        for width in [IeeeWidth::Binary32, IeeeWidth::Binary64] {
            for operator in [
                IeeeArithmeticOperator::Add,
                IeeeArithmeticOperator::Subtract,
                IeeeArithmeticOperator::Multiply,
                IeeeArithmeticOperator::Divide,
            ] {
                all.push(Op::IeeeArithmetic {
                    operator,
                    width,
                    rounding: RoundingMode::NearestEven,
                });
            }
            for comparison in [
                IeeeComparison::NumericEqual,
                IeeeComparison::TotalOrder,
                IeeeComparison::BitIdentical,
            ] {
                all.push(Op::IeeeComparison { comparison, width });
            }
        }
        all.push(Op::IeeeWidthConversion {
            source: IeeeWidth::Binary64,
            target: IeeeWidth::Binary32,
            rounding: RoundingMode::NearestEven,
        });
        all.push(Op::IeeeWidthConversion {
            source: IeeeWidth::Binary32,
            target: IeeeWidth::Binary64,
            rounding: RoundingMode::NearestEven,
        });
        all.push(Op::TextAdmission { text_type: text });
        for operator in ComparisonOperator::ALL {
            all.push(Op::TextComparison { operator });
            all.push(Op::EnumComparison { operator });
            all.push(Op::QuantityComparison { operator });
        }
        all.push(Op::QuantityConversion);
        for operator in [
            QuantityOperator::Add,
            QuantityOperator::Subtract,
            QuantityOperator::Multiply,
            QuantityOperator::Divide,
            QuantityOperator::Power,
        ] {
            all.push(Op::QuantityArithmetic { operator });
        }
        all
    }

    /// The number of `ExactScalarOperation` variants: [`expected_family`]'s match names each, and
    /// adding one fails to compile there until it has an arm. `every_operation` must then carry a
    /// descriptor of it, which the coverage assertion below checks.
    const OPERATION_VARIANTS: usize = 17;

    /// The family each descriptor belongs to, by an exhaustive match with no wildcard arm over
    /// `ExactScalarOperation` and, where the operator selects the identity, over the operator.
    /// `None` for the one descriptor with no catalogued identity (same-width IEEE conversion).
    fn expected_family(operation: &ExactScalarOperation) -> Option<Family> {
        use ExactScalarOperation as Op;
        Some(match operation {
            Op::Boolean { operator, .. } => Family::Boolean(*operator),
            Op::Equality { operator, operands } => Family::Equality(*operator, *operands),
            Op::IntegerArithmetic { operator, .. } => Family::IntegerArithmetic(*operator),
            Op::IntegerDivision { .. } => Family::IntegerDivision,
            Op::IntegerModulo { .. } => Family::IntegerModulo,
            Op::RationalArithmetic { operator, .. } => match operator {
                RationalOperator::Divide | RationalOperator::IntegerDivide => {
                    Family::RationalDivision
                }
                RationalOperator::Add
                | RationalOperator::Subtract
                | RationalOperator::Multiply
                | RationalOperator::Negate => Family::RationalArithmetic(*operator),
            },
            Op::Ordering { operator, operands } => Family::Ordering(*operator, *operands),
            Op::DecimalArithmetic { operator, .. } => Family::DecimalArithmetic(*operator),
            Op::IeeeArithmetic {
                operator, width, ..
            } => Family::IeeeArithmetic(*operator, *width),
            Op::IeeeComparison { comparison, .. } => Family::IeeeComparison(*comparison),
            Op::IeeeWidthConversion { source, target, .. } => match (source, target) {
                (IeeeWidth::Binary64, IeeeWidth::Binary32) => Family::IeeeToFloat32,
                (IeeeWidth::Binary32, IeeeWidth::Binary64) => Family::IeeeToFloat64,
                _ => return None,
            },
            Op::TextAdmission { .. } => Family::NumericConvert,
            Op::TextComparison { operator } => Family::TextComparison(*operator),
            Op::EnumComparison { operator } => Family::EnumComparison(*operator),
            Op::QuantityConversion => Family::QuantityConversion,
            Op::QuantityArithmetic { operator } => Family::QuantityArithmetic(*operator),
            Op::QuantityComparison { operator } => Family::QuantityComparison(*operator),
        })
    }

    /// The inverse identity match and the forward `catalogued_operation` are two exhaustive
    /// matches over one vocabulary, kept aligned by this round trip: every descriptor's forward
    /// identity inverts to the family that descriptor belongs to, no two families share an
    /// identity, and `every_operation` covers every `ExactScalarOperation` variant. A catalogued
    /// identity added to one map and not the other fails here.
    #[trace("FR-014-AC-18", "FR-014-AC-19", "TC-024")]
    #[test]
    fn tc_024_the_identity_inverse_matches_the_forward_catalogue_for_every_descriptor() {
        let operations = every_operation();
        let covered = operations
            .iter()
            .map(std::mem::discriminant)
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(
            covered.len(),
            OPERATION_VARIANTS,
            "every variant has a descriptor"
        );
        let mut seen = BTreeMap::new();
        for operation in &operations {
            let family = expected_family(operation).expect("every descriptor here is catalogued");
            let identity = catalogued_operation(operation)
                .expect("every descriptor here is a known RT variant")
                .expect("every descriptor here has a catalogued identity")
                .identity;
            assert_eq!(
                family_of_identity(identity),
                Some(family),
                "{identity} inverts to the family of {operation:?}"
            );
            if let Some(previous) = seen.insert(identity, family) {
                assert_eq!(previous, family, "{identity} names two families");
            }
        }
    }

    /// Catalogued identities derivation refuses (checked against the operation catalog): every
    /// one is outside `family_of_identity`, so it is `OperationNotDerivable`.
    #[trace("FR-014-AC-18", "TC-024")]
    #[test]
    fn tc_024_the_refused_catalogue_identities_have_no_family() {
        for identity in [
            "quire.op.rational.eq",
            "quire.op.rational.ne",
            "quire.op.decimal.eq",
            "quire.op.decimal.ne",
            "quire.op.integer.rem",
            "quire.op.ieee.float32.sqrt",
            "quire.op.ieee.float64.sqrt",
            "quire.op.ieee.float32.fma",
            "quire.op.ieee.float64.fma",
            "quire.op.ieee.from_exact",
            "quire.op.ieee.to_rational",
            "quire.op.numeric.narrow",
            "quire.op.rational.narrow",
            "quire.op.text.size",
        ] {
            assert_eq!(family_of_identity(identity), None, "{identity}");
        }
    }

    fn app_term(operation: Value) -> CheckedTerm {
        CheckedTerm::Application {
            operator: ApplicationOperator::Binary,
            operation,
            result_type: node_id('1'),
            arguments: Vec::new(),
        }
    }

    fn zero_literal() -> CheckedTerm {
        CheckedTerm::Literal {
            type_id: node_id('1'),
            value_kind: LiteralKind::Integer,
            value: serde_json::json!("0"),
        }
    }

    fn application_node(body: impl Into<Option<CheckedTerm>>) -> (CheckedNodeId, FixtureNode) {
        let node = v2_node('a', "expression", "binary", '1', body);
        (node.node_id.clone(), node)
    }

    /// The lowered form of a hand-built node, as IR lowering would return it.
    fn lowered_node(node: &FixtureNode) -> CompleteContractNodeV2 {
        CompleteContractNodeV2 {
            node: node.metadata.clone(),
            node_tag: CheckedNodeTag::Expression,
            source_map: vec![],
            semantic_type: node.semantic_type.clone(),
            dependencies: vec![],
            bounds: vec![],
            claims: vec![],
            ir_id: serde_json::from_value(serde_json::json!({
                "domain": "quire.contract-ir.semantic/v1",
                "algorithm": "sha-256",
                "digest": "f".repeat(64),
            }))
            .expect("an ir id"),
        }
    }

    fn derive_hand_built(
        body: impl Into<Option<CheckedTerm>>,
    ) -> Result<ExactScalarOperation, ExactScalarRefusal> {
        let (id, node) = application_node(body);
        let graph: Graph<'_> = [(&id, &node)].into_iter().collect();
        let record = CompleteLoweringRecordV2::Lowered {
            node: Box::new(lowered_node(&node)),
        };
        derive_operation(&graph, &id, &record)
    }

    fn no_claim(reason: ClaimDerivationRefusal) -> ExactScalarRefusal {
        ExactScalarRefusal::NoDerivableClaim { reason }
    }

    /// A node absent from the graph, and one whose record did not lower, keep the refusal
    /// `generate_exact_scalar_oracles` gives them, before any identity is read.
    #[trace("FR-014-AC-18", "FR-022-AC-8", "TC-024")]
    #[test]
    fn tc_024_derivation_checks_the_graph_and_the_lowering_before_the_identity() {
        let (id, node) = application_node(zero_literal());
        let graph: Graph<'_> = [(&id, &node)].into_iter().collect();
        let record = CompleteLoweringRecordV2::InvalidInput {
            node_id: id.clone(),
        };
        assert_eq!(
            derive_operation(&graph, &node_id('9'), &record),
            Err(ExactScalarRefusal::InvalidInput)
        );
        assert_eq!(
            derive_operation(&graph, &id, &record),
            Err(ExactScalarRefusal::InvalidInput),
            "an unlowered record is refused before the body is read"
        );
    }

    #[trace("FR-014-AC-48", "TC-024")]
    #[test]
    fn quantity_conversion_confirmation_requires_identity_and_exact_mode() {
        for identity in [QUANTITY_CONVERT_IDENTITY, "quire.op.quantity.add"] {
            for mode in ["exact", "nearest-even"] {
                let (_, node) = application_node(app_term(serde_json::json!({
                    "identity": identity, "mode": {"kind":"rounding", "value": mode}
                })));
                let graph: Graph<'_> = [(&node.node_id, &node)].into_iter().collect();
                assert_eq!(
                    operation_confirmed(
                        &graph,
                        &lowered_node(&node),
                        &ExactScalarOperation::QuantityConversion
                    )
                    .unwrap(),
                    identity == QUANTITY_CONVERT_IDENTITY && mode == "exact"
                );
            }
        }
    }

    /// Scalar targets cannot enter an admitted package; mode validation runs first at this seam.
    #[trace("FR-014-AC-50", "TC-024")]
    #[test]
    fn quantity_conversion_rejects_scalar_targets_after_mode() {
        for form in ["decimal", "integer", "compound_unit"] {
            let target = v2_node('b', "scalar_type", form, 'b', None);
            for mode in ["exact", "nearest-even", "toward-zero"] {
                let (id, node) = application_node(app_term(serde_json::json!({
                    "identity": QUANTITY_CONVERT_IDENTITY,
                    "mode": {"kind": "rounding", "value": mode},
                    "member": {"kind": "type_argument", "declaration": target.node_id},
                })));
                let graph: Graph<'_> = [(&id, &node), (&target.node_id, &target)]
                    .into_iter()
                    .collect();
                let record = CompleteLoweringRecordV2::Lowered {
                    node: Box::new(lowered_node(&node)),
                };
                let reason = if mode == "exact" {
                    ClaimDerivationRefusal::OperandFormsNotDerivable {
                        operation_identity: QUANTITY_CONVERT_IDENTITY.to_owned(),
                    }
                } else {
                    ClaimDerivationRefusal::ModeNotDerivable {
                        operation_identity: QUANTITY_CONVERT_IDENTITY.to_owned(),
                    }
                };
                assert_eq!(
                    derive_operation(&graph, &id, &record),
                    Err(no_claim(reason)),
                    "{form}: {mode}"
                );
            }
        }
    }

    /// A node with no `operation.identity` (which admission forbids), a node that is not an
    /// application, and one naming an identity outside the derivable set each get their own
    /// typed refusal.
    #[trace("FR-014-AC-18", "TC-024")]
    #[test]
    fn tc_024_derivation_refuses_a_missing_identity_a_non_application_and_a_foreign_identity() {
        assert_eq!(
            derive_hand_built(app_term(serde_json::json!({}))),
            Err(no_claim(ClaimDerivationRefusal::MissingOperationIdentity))
        );
        assert_eq!(
            derive_hand_built(zero_literal()),
            Err(no_claim(ClaimDerivationRefusal::NotApplication {
                node_tag: "expression".to_owned()
            }))
        );
        assert_eq!(
            derive_hand_built(app_term(
                serde_json::json!({"identity": "quire.op.rational.eq"})
            )),
            Err(no_claim(ClaimDerivationRefusal::OperationNotDerivable {
                operation_identity: "quire.op.rational.eq".to_owned()
            }))
        );
    }

    /// `integer.div` names its profile only through `operation.laws`: none, or two different
    /// profiles, select no profile; a foreign definition selects none either.
    #[trace("FR-014-AC-18", "TC-024")]
    #[test]
    fn tc_024_derivation_reads_the_division_profile_from_the_laws_and_nowhere_else() {
        let laws = |identities: &[&str]| {
            app_term(serde_json::json!({
                "identity": "quire.op.integer.div",
                "laws": identities
                    .iter()
                    .map(|identity| serde_json::json!({
                        "role": "integer_division", "definition": {"identity": identity}
                    }))
                    .collect::<Vec<_>>(),
            }))
        };
        let profile_of = |body: CheckedTerm| {
            let (id, node) = application_node(body);
            let graph: Graph<'_> = [(&id, &node)].into_iter().collect();
            let lowered = lowered_node(&node);
            Derivation {
                graph: &graph,
                node: &node,
                bounds: BoundView::complete(&lowered),
                identity: "quire.op.integer.div",
            }
            .division_profile()
        };
        let law_refusal = Err(no_claim(ClaimDerivationRefusal::LawNotDerivable {
            operation_identity: "quire.op.integer.div".to_owned(),
        }));
        assert_eq!(profile_of(laws(&[])), law_refusal);
        assert_eq!(profile_of(laws(&["quire.value.other/v1"])), law_refusal);
        assert_eq!(
            profile_of(laws(&[
                DivisionProfile::Floor.definition_identity(),
                DivisionProfile::Euclidean.definition_identity(),
            ])),
            law_refusal
        );
        for profile in DivisionProfile::ALL {
            assert_eq!(
                profile_of(laws(&[profile.definition_identity()])),
                Ok(profile)
            );
        }
    }

    /// `operation.mode` must carry the rounding of the bound it is read against: absent, of
    /// another kind or another value is `ModeNotDerivable`.
    #[trace("FR-014-AC-18", "TC-024")]
    #[test]
    fn tc_024_derivation_requires_the_mode_to_agree_with_the_bound_rounding() {
        let derivation_of = |mode: Value| {
            let (id, node) = application_node(app_term(
                serde_json::json!({"identity": "quire.op.decimal.add", "mode": mode}),
            ));
            let graph: Graph<'_> = [(&id, &node)].into_iter().collect();
            let lowered = lowered_node(&node);
            Derivation {
                graph: &graph,
                node: &node,
                bounds: BoundView::complete(&lowered),
                identity: "quire.op.decimal.add",
            }
            .require_mode(RoundingMode::NearestEven)
        };
        let refusal = Err(no_claim(ClaimDerivationRefusal::ModeNotDerivable {
            operation_identity: "quire.op.decimal.add".to_owned(),
        }));
        assert_eq!(derivation_of(Value::Null), refusal);
        assert_eq!(
            derivation_of(serde_json::json!({"kind": "text_profile", "value": "nearest-even"})),
            refusal
        );
        assert_eq!(
            derivation_of(serde_json::json!({"kind": "rounding", "value": "toward-zero"})),
            refusal
        );
        assert_eq!(
            derivation_of(serde_json::json!({"kind": "rounding", "value": "nearest-even"})),
            Ok(())
        );
    }
}

#[cfg(test)]
mod opaque_metadata_tests;

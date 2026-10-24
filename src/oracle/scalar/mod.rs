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
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

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

/// Executable source region for one V2 scalar claim. The node id identifies the checked
/// expression directly: a V2 expression carries no V1 requirement-clause key.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ScalarCoverageRegion {
    artifact_path: &'static str,
    node_id: CheckedNodeId,
    role: &'static str,
    start_line: u32,
    end_line: u32,
    probe: SourceProbe,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_consequents: Option<u32>,
}

fn scalar_coverage_regions<'a>(
    source: &str,
    claims: &ClaimMap<ExactScalarClaim>,
    pending: impl IntoIterator<Item = (usize, &'a ExactScalarOperation, u32)>,
) -> Result<Vec<ScalarCoverageRegion>, OracleGenerationError> {
    let lines = source.lines().collect::<Vec<_>>();
    let declarations = lines
        .iter()
        .enumerate()
        .filter_map(|(line, text)| {
            text.strip_prefix("pub fn ")
                .and_then(|rest| rest.split_once('('))
                .map(|(symbol, _)| (symbol, line))
        })
        .collect::<BTreeMap<_, _>>();
    let mut regions = Vec::new();
    for (claim_index, operation, expected_consequents) in pending {
        let claim = &claims.items[claim_index];
        let ClaimDisposition::Generated(generated) = &claim.result else {
            return Err(OracleGenerationError::InvalidPlanState);
        };
        let at = *declarations
            .get(generated.symbol.as_str())
            .ok_or(OracleGenerationError::InvalidPlanState)?;
        let entry_line =
            u32::try_from(at + 1).map_err(|_| OracleGenerationError::InvalidPlanState)?;
        let entry_column = lines[at]
            .find("pub fn")
            .ok_or(OracleGenerationError::InvalidPlanState)?;
        let entry_column =
            u32::try_from(entry_column + 1).map_err(|_| OracleGenerationError::InvalidPlanState)?;
        let entry = ScalarCoverageRegion {
            artifact_path: "src/lib.rs",
            node_id: claim.node_id.clone(),
            role: "oracle_evaluation",
            start_line: entry_line,
            end_line: entry_line,
            probe: SourceProbe {
                line: entry_line,
                start_column: entry_column,
                end_column: entry_column + 3,
            },
            expected_consequents: Some(expected_consequents),
        };
        regions.push(entry);
        if let ExactScalarOperation::Boolean {
            operator: BooleanOperator::Implies,
            right_may_stop,
        } = operation
        {
            let call_at = lines
                .iter()
                .enumerate()
                .skip(at + 1)
                .take_while(|(_, line)| !line.starts_with('}'))
                .find(|(_, line)| {
                    if *right_may_stop {
                        line.trim() == "right()"
                    } else {
                        line.contains("rt::evaluate_boolean")
                            && line.contains("Implies")
                            && line.contains("right")
                    }
                })
                .map(|(line, _)| line)
                .ok_or(OracleGenerationError::InvalidPlanState)?;
            let line =
                u32::try_from(call_at + 1).map_err(|_| OracleGenerationError::InvalidPlanState)?;
            let column = lines[call_at]
                .rfind("right")
                .ok_or(OracleGenerationError::InvalidPlanState)?;
            let column =
                u32::try_from(column + 1).map_err(|_| OracleGenerationError::InvalidPlanState)?;
            regions.push(ScalarCoverageRegion {
                artifact_path: "src/lib.rs",
                node_id: claim.node_id.clone(),
                role: "implication_consequent",
                start_line: line,
                end_line: line,
                probe: SourceProbe {
                    line,
                    start_column: column,
                    end_column: column + 5,
                },
                expected_consequents: None,
            });
        }
    }
    Ok(regions)
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

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum SelectedScalarKey {
    NotApplicable(CheckedNodeId),
    OwnResult {
        node_id: CheckedNodeId,
        application_ordinal: u64,
        conversion: Option<(CheckedNodeId, u64)>,
    },
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

fn selected_scalar_key(
    package: &ContractPackageV1,
    items: &PackageItems<'_>,
    request_index: usize,
    node_id: &CheckedNodeId,
    selection: &ContractDriverResultSelection,
) -> Option<SelectedScalarKey> {
    let ContractDriverResultSelection::OwnResult {
        application,
        conversion,
    } = selection
    else {
        return Some(SelectedScalarKey::NotApplicable(node_id.clone()));
    };
    #[cfg(test)]
    source_selection_work::source(true);
    package.source_occurrences(node_id)?.iter().find(|source| {
        #[cfg(test)]
        source_selection_work::occurrence(false);
        application.node_id == *node_id
            && application.role == CheckedOccurrenceRole::Expression
            && source.role == application.role
            && source.ordinal == application.ordinal
    })?;
    if let Some(conversion) = conversion {
        if conversion.role != CheckedOccurrenceRole::Expression
            || !items.has_context_root(request_index, &conversion.node_id)
        {
            return None;
        }
        #[cfg(test)]
        source_selection_work::source(true);
        package
            .source_occurrences(&conversion.node_id)?
            .iter()
            .find(|entry| {
                #[cfg(test)]
                source_selection_work::occurrence(true);
                entry.role == conversion.role && entry.ordinal == conversion.ordinal
            })?;
    }
    let ContractResultViewV1::Lowered(view) = package
        .result_context(
            node_id,
            application,
            conversion.as_ref(),
            BoundedDomainForm::IntegerRange,
        )
        .ok()?
    else {
        return None;
    };
    if matches!(view.context, ContractResultContextV1::IncompleteContext) {
        return None;
    }
    Some(SelectedScalarKey::OwnResult {
        node_id: node_id.clone(),
        application_ordinal: application.ordinal,
        conversion: conversion
            .as_ref()
            .map(|key| (key.node_id.clone(), key.ordinal)),
    })
}

fn contract_checked_operation<'a>(
    graph: &Graph<'_>,
    closure: &'a quire_contract_model::ContractReachedClosureV1<'_>,
    tag: Option<CheckedNodeTag>,
    node_id: &CheckedNodeId,
    package: &'a ContractPackageV1,
    selection: &'a ContractDriverResultSelection,
) -> Result<
    Result<(ExactScalarOperation, Vec<CheckedNodeId>), ExactScalarRefusal>,
    OracleGenerationError,
> {
    if tag != Some(CheckedNodeTag::Expression) {
        return Ok(Err(tag.map_or_else(
            || ExactScalarRefusal::Unsupported {
                unsupported_node_id: node_id.clone(),
                node_tag: "unknown",
            },
            |tag| unsupported_family(node_id, tag),
        )));
    }
    let ContractDriverResultSelection::OwnResult {
        application,
        conversion,
    } = selection
    else {
        return Ok(Err(ExactScalarRefusal::IncompleteContext));
    };
    if &application.node_id != node_id {
        return Ok(Err(ExactScalarRefusal::IncompleteContext));
    }
    let bounds = BoundView::admitted(
        closure,
        SelectedResult {
            package,
            application,
            conversion: conversion.as_ref(),
        },
    );
    let operation = match derive_operation_from_view(graph, closure.root, bounds.clone()) {
        Ok(operation) => operation,
        Err(refusal) => return Ok(Err(refusal)),
    };
    if let Err(refusal) = contract_scalar_shape(graph, closure.root, &operation)? {
        return Ok(Err(refusal));
    }
    match check_parameters_from_view(graph, bounds, &operation) {
        Ok(checked) => Ok(Ok((operation, checked))),
        Err(ItemCheckError::Refusal(refusal)) => Ok(Err(refusal)),
        Err(ItemCheckError::Generation(error)) => Err(error),
    }
}

fn contract_scalar_shape(
    graph: &Graph<'_>,
    node: &CheckedSemanticNodeV2<Value>,
    operation: &ExactScalarOperation,
) -> Result<Result<(), ExactScalarRefusal>, OracleGenerationError> {
    let shape = Shape::of(operation)?;
    Ok((|| {
        if &*node.semantic_form != shape.form {
            return Err(ExactScalarRefusal::FormMismatch {
                expected: shape.form,
                found: node.semantic_form.to_string(),
            });
        }
        let arguments = application_arguments(graph.term(&node.node_id), shape.body_operator)
            .filter(|arguments| arguments.len() == shape.operands.len())
            .ok_or(ExactScalarRefusal::BodyMismatch {
                expected_operator: shape.body_operator,
                expected_arguments: shape.operands.len(),
            })?;
        let result = scalar_type_form(graph, &node.semantic_type);
        if result.as_deref() != Some(shape.result.wire()) {
            return Err(ExactScalarRefusal::ResultTypeMismatch {
                expected: shape.result,
                found: result,
            });
        }
        for (position, (argument, expected)) in arguments.iter().zip(shape.operands).enumerate() {
            check_operand(graph, position, argument, *expected)?;
        }
        Ok(())
    })())
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
type CheckedItem<'r> = (&'r CompleteContractNodeV2, Vec<CheckedNodeId>);

/// Either [`ExactScalarRefusal`] (a per-item business refusal, folded into
/// the claim map) or [`OracleGenerationError`] (an RT `#[non_exhaustive]`
/// enum yielding a variant this generator's own match (in `check_parameters`
/// or one of the other scalar sites) does not know -- see [`OracleGenerationError::UnknownRuntimeVariant`]'s own doc for
/// why that aborts the whole generation instead of refusing one item).
enum ItemCheckError {
    Refusal(ExactScalarRefusal),
    Generation(OracleGenerationError),
}

impl From<ExactScalarRefusal> for ItemCheckError {
    fn from(refusal: ExactScalarRefusal) -> Self {
        Self::Refusal(refusal)
    }
}

fn check_item<'r, Body: GraphBody>(
    graph: &Graph<'_, Body>,
    requested_node_id: &CheckedNodeId,
    record: &'r CompleteLoweringRecordV2,
    operation: &ExactScalarOperation,
) -> Result<CheckedItem<'r>, ItemCheckError> {
    if let Some(node) = graph.node(requested_node_id) {
        if let Some(node_tag) = direct_unsupported_form(node, graph.term(requested_node_id)) {
            return Err(ExactScalarRefusal::Unsupported {
                unsupported_node_id: requested_node_id.clone(),
                node_tag,
            }
            .into());
        }
    }
    if let CompleteLoweringRecordV2::Unsupported {
        unsupported_node_id,
        ..
    } = record
    {
        if let Some(node) = graph.node(unsupported_node_id) {
            if let Some(node_tag) = direct_unsupported_form(node, graph.term(unsupported_node_id)) {
                return Err(ExactScalarRefusal::Unsupported {
                    unsupported_node_id: unsupported_node_id.clone(),
                    node_tag,
                }
                .into());
            }
        }
    }
    let node = lowered(record)?;
    if node.node_tag != CheckedNodeTag::Expression {
        return Err(ExactScalarRefusal::NotExpression {
            node_tag: node.node_tag.as_wire(),
        }
        .into());
    }
    if matches!(operation, ExactScalarOperation::QuantityConversion) {
        let derivation = Derivation {
            graph,
            node: &node.node,
            bounds: BoundView::complete(node),
            identity: QUANTITY_CONVERT_IDENTITY,
        };
        derivation.require_mode(RoundingMode::Exact)?;
        if !quantity_conversion_target_is_unit(graph, &node.node) {
            return Err(derivation.operand_forms_not_derivable().into());
        }
    }
    if let ExactScalarOperation::Boolean { right_may_stop, .. } = operation {
        let derivation = Derivation {
            graph,
            node: &node.node,
            bounds: BoundView::complete(node),
            identity: "",
        };
        let actual_may_stop = derivation.right_may_stop();
        if *right_may_stop != actual_may_stop {
            return Err(ExactScalarRefusal::BooleanRightOperandMismatch {
                declared_may_stop: *right_may_stop,
                actual_may_stop,
            }
            .into());
        }
    }
    let shape = Shape::of(operation).map_err(ItemCheckError::Generation)?;
    if &*node.node.semantic_form != shape.form {
        return Err(ExactScalarRefusal::FormMismatch {
            expected: shape.form,
            found: node.node.semantic_form.to_string(),
        }
        .into());
    }
    let arguments = application_arguments(graph.term(&node.node.node_id), shape.body_operator)
        .filter(|arguments| arguments.len() == shape.operands.len())
        .ok_or(ExactScalarRefusal::BodyMismatch {
            expected_operator: shape.body_operator,
            expected_arguments: shape.operands.len(),
        })?;
    let result = scalar_type_form(graph, &node.semantic_type);
    if result.as_deref() != Some(shape.result.wire()) {
        return Err(ExactScalarRefusal::ResultTypeMismatch {
            expected: shape.result,
            found: result,
        }
        .into());
    }
    for (position, (argument, expected)) in arguments.iter().zip(shape.operands).enumerate() {
        check_operand(graph, position, argument, *expected)?;
    }
    let checked_bounds = check_parameters(graph, node, operation)?;
    Ok((node, checked_bounds))
}

fn lowered(
    record: &CompleteLoweringRecordV2,
) -> Result<&CompleteContractNodeV2, ExactScalarRefusal> {
    match record {
        CompleteLoweringRecordV2::Lowered { node } => Ok(node),
        CompleteLoweringRecordV2::Unsupported {
            unsupported_node_id,
            node_tag,
            ..
        } => Err(unsupported_family(unsupported_node_id, *node_tag)),
        CompleteLoweringRecordV2::RequiresBound { unbounded_type, .. } => {
            Err(ExactScalarRefusal::RequiresBound {
                unbounded_type: unbounded_type.clone(),
            })
        }
        CompleteLoweringRecordV2::InvalidInput { .. } => Err(ExactScalarRefusal::InvalidInput),
        CompleteLoweringRecordV2::InvalidBody { body_node_id, .. } => {
            Err(ExactScalarRefusal::InvalidBody {
                body_node_id: body_node_id.clone(),
            })
        }
        CompleteLoweringRecordV2::BodyIncomplete { body_node_id, .. } => {
            Err(ExactScalarRefusal::BodyIncomplete {
                body_node_id: body_node_id.clone(),
            })
        }
        // FR-014-AC-42 keeps this arm from binding the record's fields, so the typed outcome is
        // read back through the classifier, which returns `None` for a record that is not
        // `Failed`. This arm only runs for a `Failed` record, so the `InvalidInput` fallback is
        // unreachable; it exists because the arm cannot hand the classifier a narrower type.
        CompleteLoweringRecordV2::Failed { .. } => Err(classify_lowering_failure(record)
            .map_or(ExactScalarRefusal::InvalidInput, ExactScalarRefusal::from)),
    }
}

impl From<LoweringFailure> for ExactScalarRefusal {
    fn from(failure: LoweringFailure) -> Self {
        match failure {
            LoweringFailure::WorkExhausted { limit, consumed } => {
                Self::LoweringWorkExhausted { limit, consumed }
            }
            LoweringFailure::ByteLimitExceeded { limit, consumed } => {
                Self::LoweringByteLimitExceeded { limit, consumed }
            }
            LoweringFailure::LimitUnrecognised {
                limit_kind,
                limit,
                consumed,
            } => Self::LoweringLimitUnrecognised {
                limit_kind,
                limit,
                consumed,
            },
        }
    }
}

fn unsupported_family(node_id: &CheckedNodeId, tag: CheckedNodeTag) -> ExactScalarRefusal {
    let blocked = |issue| ExactScalarRefusal::BlockedOnUpstream {
        unsupported_node_id: node_id.clone(),
        node_tag: tag.as_wire(),
        issue,
    };
    match tag {
        CheckedNodeTag::Function => blocked(UpstreamBlocker::QuireContractRuntime34),
        CheckedNodeTag::Model | CheckedNodeTag::Relation => {
            blocked(UpstreamBlocker::QuireSpecLanguage120)
        }
        CheckedNodeTag::State
        | CheckedNodeTag::Temporal
        | CheckedNodeTag::Protocol
        | CheckedNodeTag::TimedModel
        | CheckedNodeTag::ScalarType
        | CheckedNodeTag::CompositeType
        | CheckedNodeTag::BoundedDomain
        | CheckedNodeTag::Value
        | CheckedNodeTag::Expression
        | CheckedNodeTag::Claim
        | CheckedNodeTag::Correspondence
        | CheckedNodeTag::Extension
        | CheckedNodeTag::Hybrid => ExactScalarRefusal::Unsupported {
            unsupported_node_id: node_id.clone(),
            node_tag: tag.as_wire(),
        },
    }
}

fn application_arguments<'a>(
    body: impl Into<Option<&'a CheckedTerm>>,
    expected: &str,
) -> Option<&'a [CheckedTerm]> {
    match body.into()? {
        CheckedTerm::Application {
            operator,
            arguments,
            ..
        } if operator.as_wire() == expected => Some(arguments),
        _ => None,
    }
}

fn type_form<Body: GraphBody>(graph: &Graph<'_, Body>, type_id: &CheckedNodeId) -> Option<String> {
    let node = graph.node(type_id)?;
    (&*node.node_tag == CheckedNodeTag::ScalarType.as_wire())
        .then(|| node.semantic_form.to_string())
}

/// An operand is a literal classified by its value kind, or a reference
/// classified by its target's scalar type. A literal carries no unit, so it is
/// never a quantity; any other term is refused as unsupported.
fn check_operand<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    position: usize,
    term: &CheckedTerm,
    expected: ScalarForm,
) -> Result<(), ExactScalarRefusal> {
    let found = match term {
        CheckedTerm::Literal { .. } if expected == ScalarForm::Unit => {
            return Err(ExactScalarRefusal::UnitlessLiteralOperand { position });
        }
        CheckedTerm::Literal { .. } => literal_form(term),
        CheckedTerm::Reference { .. } => reference_form(graph, term),
        other => {
            return Err(ExactScalarRefusal::OperandUnsupported {
                position,
                term: term_name(other).to_owned(),
            });
        }
    };
    if found.as_deref() == Some(expected.wire()) {
        Ok(())
    } else {
        Err(ExactScalarRefusal::OperandTypeMismatch {
            position,
            expected,
            found,
        })
    }
}

fn literal_form(term: &CheckedTerm) -> Option<String> {
    let CheckedTerm::Literal { value_kind, .. } = term else {
        return None;
    };
    ScalarForm::from_literal_kind(*value_kind).map(|form| form.wire().to_owned())
}

/// Classify only the owner's literal and reference variants.
fn operand_form<Body: GraphBody>(graph: &Graph<'_, Body>, term: &CheckedTerm) -> Option<String> {
    match term {
        CheckedTerm::Literal { .. } => literal_form(term),
        CheckedTerm::Reference { .. } => reference_form(graph, term),
        _ => None,
    }
}

fn reference_form<Body: GraphBody>(graph: &Graph<'_, Body>, term: &CheckedTerm) -> Option<String> {
    let target = reference_target(term)?;
    scalar_type_form(graph, &graph.node(&target)?.semantic_type)
}

/// The scalar form of `type_id`, following one `bounded_domain` step. A parameter typed
/// `Int[0, 9]` is typed by a `bounded_domain` whose own `semantic_type` is its base scalar type:
/// QSL emits a bounded domain directly over its scalar base, so one step reaches the type the form
/// is read from. A node's result typed by a bounded domain reads the same way.
fn scalar_type_form<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    type_id: &CheckedNodeId,
) -> Option<String> {
    let node = graph.node(type_id)?;
    if is_bounded_domain(node) {
        type_form(graph, &node.semantic_type)
    } else {
        type_form(graph, type_id)
    }
}

fn is_bounded_domain<Body>(node: &CheckedSemanticNodeV2<Body>) -> bool {
    &*node.node_tag == CheckedNodeTag::BoundedDomain.as_wire()
}

/// The `bounded_domain` node that types a `reference` operand's target: the operand's own bound.
/// A literal, a reference typed by a scalar type, or any other term has none.
fn operand_own_bound<'g, Body: GraphBody>(
    graph: &Graph<'g, Body>,
    term: &CheckedTerm,
) -> Option<&'g CheckedSemanticNodeV2<Body>> {
    let target = reference_target(term)?;
    let typing = graph.node(&graph.node(&target)?.semantic_type)?;
    is_bounded_domain(typing).then_some(typing)
}

/// The type of a `reference` operand that no `bounded_domain` types; `None` for a literal, a
/// reference to a literal value node, or an operand with an own bound.
///
/// QSL emits a literal operand as a `reference` to its own `value` node, whose body is the
/// literal and whose type is the plain scalar type. That is a constant, like an inline literal,
/// and is decided by the referenced node's body, never by its type: a `value` node with any other
/// body (a parameter) and a plain type is an unbounded operand.
fn plain_reference_type<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    term: &CheckedTerm,
) -> Option<CheckedNodeId> {
    if operand_own_bound(graph, term).is_some() {
        return None;
    }
    let node = graph.node(&reference_target(term)?)?;
    (!is_literal_value(graph, node)).then(|| node.semantic_type.clone())
}

/// A `value` node whose body is a literal: the node QSL emits for a literal operand.
fn is_literal_value<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node: &CheckedSemanticNodeV2<Body>,
) -> bool {
    &*node.node_tag == CheckedNodeTag::Value.as_wire()
        && matches!(graph.term(&node.node_id), Some(CheckedTerm::Literal { .. }))
}

/// The catalogued identity of the narrowing conversion QSL wraps a bounded-context expression in.
const NARROW_IDENTITY: &str = "quire.op.numeric.narrow";

/// How the nodes that consume a scalar-typed node bound its result.
enum Narrowing<'g, Body> {
    /// No narrowing conversion consumes the node.
    Unnarrowed,
    /// Only narrowing conversions consume the node: the `bounded_domain` nodes they are typed by,
    /// ascending and deduplicated (empty where none is typed by a `bounded_domain`).
    To(Vec<&'g CheckedSemanticNodeV2<Body>>),
    /// A narrowing conversion consumes the node and so does a node that is not one: the narrow's
    /// bound does not bound every use of the node's result.
    Mixed,
}

impl<Body> Narrowing<'_, Body> {
    /// Whether a consuming conversion narrows the node's result, so the package bounds it.
    fn narrowed(&self) -> bool {
        match self {
            Self::Unnarrowed => false,
            Self::To(bounds) => !bounds.is_empty(),
            Self::Mixed => true,
        }
    }
}

/// Whether `consumer` is a narrowing conversion of the node `id`: an `expression` of form
/// `conversion` whose body is a `convert` application with the catalogued identity
/// `quire.op.numeric.narrow` and exactly one argument, a `reference` to `id`.
fn narrows<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    consumer: &CheckedSemanticNodeV2<Body>,
    id: &CheckedNodeId,
) -> bool {
    &*consumer.node_tag == CheckedNodeTag::Expression.as_wire()
        && &*consumer.semantic_form == "conversion"
        && graph
            .operation(&consumer.node_id)
            .and_then(|op| op.get("identity"))
            .and_then(Value::as_str)
            == Some(NARROW_IDENTITY)
        && application_arguments(graph.term(&consumer.node_id), "convert").is_some_and(
            |arguments| matches!(arguments, [only] if reference_target(only).as_ref() == Some(id)),
        )
}

/// Walk only the reader-admitted term structure; operation payloads are not term bodies.
fn references(term: Option<&CheckedTerm>, id: &CheckedNodeId) -> bool {
    match term {
        Some(CheckedTerm::Reference { target }) => target == id,
        Some(CheckedTerm::Application { arguments, .. }) => {
            arguments.iter().any(|term| references(Some(term), id))
        }
        Some(CheckedTerm::Aggregate { members }) => {
            members.iter().any(|term| references(Some(term), id))
        }
        Some(CheckedTerm::Binding { value, .. }) => references(Some(value), id),
        Some(CheckedTerm::ExtensionForm { clauses, .. }) => {
            clauses.iter().any(|term| references(Some(term), id))
        }
        Some(
            CheckedTerm::Literal { .. }
            | CheckedTerm::DependencyReference { .. }
            | CheckedTerm::Frame { .. }
            | CheckedTerm::AbstractionRelation { .. },
        )
        | None => false,
    }
}

/// How the consumers of the node `id` narrow it. A consumer is any other node whose body holds a
/// `reference` to `id`; a narrowing conversion is one [`narrows`] accepts.
fn narrowing<'g, Body: GraphBody>(
    graph: &Graph<'g, Body>,
    id: &CheckedNodeId,
) -> Narrowing<'g, Body> {
    let mut bounds = BTreeMap::new();
    let (mut narrowed, mut other) = (false, false);
    for consumer in graph.nodes() {
        if &consumer.node_id == id || !references(graph.term(&consumer.node_id), id) {
            continue;
        }
        if !narrows(graph, consumer, id) {
            other = true;
            continue;
        }
        narrowed = true;
        if let Some(bound) = graph
            .node(&consumer.semantic_type)
            .filter(|typing| is_bounded_domain(typing))
        {
            bounds.insert(&bound.node_id, bound);
        }
    }
    match (narrowed, other) {
        (false, _) => Narrowing::Unnarrowed,
        (true, true) => Narrowing::Mixed,
        (true, false) => Narrowing::To(bounds.into_values().collect()),
    }
}

/// The node a `reference` term names.
fn reference_target(term: &CheckedTerm) -> Option<CheckedNodeId> {
    match term {
        CheckedTerm::Reference { target } => Some(target.clone()),
        _ => None,
    }
}

fn own_bounds<'a, Body: GraphBody>(
    graph: &Graph<'a, Body>,
    body: Option<&CheckedTerm>,
) -> Vec<Option<&'a CheckedSemanticNodeV2<Body>>> {
    let arguments = match body {
        Some(CheckedTerm::Application { arguments, .. }) => arguments.as_slice(),
        _ => &[],
    };
    arguments
        .iter()
        .map(|argument| operand_own_bound(graph, argument))
        .collect()
}

fn operand_literal<'a, Body: GraphBody>(
    graph: &Graph<'a, Body>,
    term: &'a CheckedTerm,
) -> Option<&'a CheckedTerm> {
    if matches!(term, CheckedTerm::Literal { .. }) {
        return Some(term);
    }
    let node = graph.node(&reference_target(term)?)?;
    is_literal_value(graph, node)
        .then(|| graph.term(&node.node_id))
        .flatten()
}

/// How a Kani harness ranges one operand of a scalar node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum OperandRange {
    /// Over the operand's own `bounded_domain`, the node named.
    Bound(CheckedNodeId),
    /// Exactly the operand's own literal value, its spelling as the package carries it.
    Literal(String),
    /// Over the result bound: an operand that is neither (the shape IR-299 tracks).
    Result,
    /// Exact range recursively inferred from this referenced application tree.
    Inferred {
        source: CheckedNodeId,
        lower: String,
        upper: String,
    },
}

/// How a Kani harness ranges each of the node's arguments, by position: over the
/// `bounded_domain` typing the argument, exactly at the value of a literal argument (inline or a
/// reference to a literal `value` node), or over the result bound otherwise. Reads the same graph
/// [`generate_exact_scalar_oracles`] checked, so a Kani harness ranges each operand as the
/// oracle's parameter check did. A claim records the own-bound ids in `checked_bounds` but not
/// their positions, so the harness takes the positions from here and requires every id to appear
/// in the claim's `checked_bounds` (`lower_scalar_claim`): the two sources cannot silently
/// disagree.
pub(crate) fn operand_ranges(graph: &Graph<'_>, node_id: &CheckedNodeId) -> Vec<OperandRange> {
    let Some(node) = graph.node(node_id) else {
        return Vec::new();
    };
    graph
        .arguments(&node.node_id)
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(position, argument)| {
            if let Some(bound) = operand_own_bound(graph, argument) {
                OperandRange::Bound(bound.node_id.clone())
            } else if let Some(literal) = operand_literal(graph, argument) {
                OperandRange::Literal(
                    match literal {
                        CheckedTerm::Literal { value, .. } => value.as_str().unwrap_or_default(),
                        _ => "",
                    }
                    .to_owned(),
                )
            } else if let Some(source) = reference_target(argument) {
                infer_integer_range(graph, &source, position).ok().map_or(
                    OperandRange::Result,
                    |(lower, upper)| OperandRange::Inferred {
                        source,
                        lower,
                        upper,
                    },
                )
            } else {
                OperandRange::Result
            }
        })
        .collect()
}

/// Infer an exact integer interval from canonical literals, bounded integer parameters, and the
/// four admitted integer operations. The recursion is deliberately limited to plain-Integer
/// application results: an operand's own bounded type is handled by `operand_own_bound`.
const OPERAND_RANGE_MAX_DEPTH: u64 = 64;
const OPERAND_RANGE_MAX_WORK: u64 = 4096;
const OPERAND_RANGE_MAX_ENDPOINT_BITS: u64 = 4096;

fn infer_integer_range<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node_id: &CheckedNodeId,
    position: usize,
) -> Result<(String, String), ExactScalarRefusal> {
    fn visit<Body: GraphBody>(
        graph: &Graph<'_, Body>,
        node_id: &CheckedNodeId,
        position: usize,
        active: &mut BTreeSet<CheckedNodeId>,
        memo: &mut BTreeMap<CheckedNodeId, (BigInt, BigInt)>,
        application_depth: u64,
        work: &mut u64,
    ) -> Result<(BigInt, BigInt), ExactScalarRefusal> {
        let node = graph
            .node(node_id)
            .ok_or_else(|| ExactScalarRefusal::InvalidBody {
                body_node_id: node_id.clone(),
            })?;
        let is_application = &*node.node_tag == CheckedNodeTag::Expression.as_wire();
        let application_depth = if is_application {
            application_depth + 1
        } else {
            application_depth
        };
        if is_application && application_depth > OPERAND_RANGE_MAX_DEPTH {
            return Err(ExactScalarRefusal::OperandRangeLimitExceeded {
                node_id: node_id.clone(),
                resource_kind: OperandRangeLimitKind::Depth,
                limit: OPERAND_RANGE_MAX_DEPTH,
                consumed: application_depth,
            });
        }
        if let Some(range) = memo.get(node_id) {
            return Ok(range.clone());
        }
        *work += 1;
        if *work > OPERAND_RANGE_MAX_WORK {
            return Err(ExactScalarRefusal::OperandRangeLimitExceeded {
                node_id: node_id.clone(),
                resource_kind: OperandRangeLimitKind::Nodes,
                limit: OPERAND_RANGE_MAX_WORK,
                consumed: *work,
            });
        }
        if !active.insert(node_id.clone()) {
            return Err(ExactScalarRefusal::InvalidBody {
                body_node_id: node_id.clone(),
            });
        }
        let result = if is_literal_value(graph, node) {
            let literal = graph
                .term(&node.node_id)
                .and_then(literal_integer)
                .ok_or_else(|| ExactScalarRefusal::OperandTypeMismatch {
                    position,
                    expected: ScalarForm::Integer,
                    found: graph.term(&node.node_id).and_then(literal_form),
                })?;
            let value = literal.to_string().parse::<BigInt>().map_err(|_| {
                ExactScalarRefusal::InvalidBody {
                    body_node_id: node_id.clone(),
                }
            })?;
            (value.clone(), value)
        } else if let Some(typing) = graph.node(&node.semantic_type) {
            if is_bounded_domain(typing) {
                let range =
                    read_integer_range(aggregate_members(graph.term(&typing.node_id)).ok_or_else(
                        || ExactScalarRefusal::InvalidBody {
                            body_node_id: typing.node_id.clone(),
                        },
                    )?)
                    .ok_or_else(|| ExactScalarRefusal::InvalidBody {
                        body_node_id: typing.node_id.clone(),
                    })?;
                (
                    range.lower().to_string().parse().map_err(|_| {
                        ExactScalarRefusal::InvalidBody {
                            body_node_id: typing.node_id.clone(),
                        }
                    })?,
                    range.upper().to_string().parse().map_err(|_| {
                        ExactScalarRefusal::InvalidBody {
                            body_node_id: typing.node_id.clone(),
                        }
                    })?,
                )
            } else if &*node.node_tag == CheckedNodeTag::Value.as_wire()
                && scalar_type_form(graph, &node.semantic_type).as_deref() == Some("integer")
            {
                return Err(ExactScalarRefusal::RequiresBound {
                    unbounded_type: node.semantic_type.clone(),
                });
            } else if &*node.node_tag == CheckedNodeTag::Expression.as_wire()
                && scalar_type_form(graph, &node.semantic_type).as_deref() == Some("integer")
            {
                let operation = graph
                    .operation(&node.node_id)
                    .and_then(|v| v.get("identity"))
                    .and_then(Value::as_str)
                    .ok_or_else(|| ExactScalarRefusal::InvalidBody {
                        body_node_id: node_id.clone(),
                    })?;
                let integer_operator = CatalogOperator::from_identity(operation)
                    .and_then(|catalogued| catalogued.entry().scalar_operator)
                    .ok_or_else(|| ExactScalarRefusal::OperandUnsupported {
                        position,
                        term: operation.to_owned(),
                    })?;
                let args = graph.arguments(&node.node_id).ok_or_else(|| {
                    ExactScalarRefusal::InvalidBody {
                        body_node_id: node_id.clone(),
                    }
                })?;
                let (expected_operator, expected_arity) = match integer_operator {
                    IntegerOperator::Add
                    | IntegerOperator::Subtract
                    | IntegerOperator::Multiply => ("binary", 2),
                    IntegerOperator::Negate => ("unary", 1),
                };
                if graph
                    .operator(&node.node_id)
                    .map(ApplicationOperator::as_wire)
                    != Some(expected_operator)
                    || args.len() != expected_arity
                {
                    return Err(ExactScalarRefusal::InvalidBody {
                        body_node_id: node_id.clone(),
                    });
                }
                let mut ranges = Vec::with_capacity(args.len());
                for arg in args {
                    if matches!(arg, CheckedTerm::Literal { .. }) {
                        let literal = literal_integer(arg).ok_or_else(|| {
                            ExactScalarRefusal::OperandTypeMismatch {
                                position,
                                expected: ScalarForm::Integer,
                                found: literal_form(arg),
                            }
                        })?;
                        let value = literal.to_string().parse::<BigInt>().map_err(|_| {
                            ExactScalarRefusal::InvalidBody {
                                body_node_id: node_id.clone(),
                            }
                        })?;
                        check_operand_endpoint_bits(node_id, &value, &value)?;
                        ranges.push((value.clone(), value));
                        continue;
                    }
                    let Some(target) = reference_target(arg) else {
                        return Err(ExactScalarRefusal::OperandUnsupported {
                            position,
                            term: term_name(arg).to_owned(),
                        });
                    };
                    ranges.push(visit(
                        graph,
                        &target,
                        position,
                        active,
                        memo,
                        application_depth,
                        work,
                    )?);
                }
                match (integer_operator, ranges.as_slice()) {
                    (IntegerOperator::Add, [(al, au), (bl, bu)]) => (al + bl, au + bu),
                    (IntegerOperator::Subtract, [(al, au), (bl, bu)]) => (al - bu, au - bl),
                    (IntegerOperator::Multiply, [(al, au), (bl, bu)]) => {
                        let products = [al * bl, al * bu, au * bl, au * bu];
                        let mut products = products.into_iter();
                        let first =
                            products
                                .next()
                                .ok_or_else(|| ExactScalarRefusal::InvalidBody {
                                    body_node_id: node_id.clone(),
                                })?;
                        products.fold((first.clone(), first), |(lower, upper), product| {
                            (lower.min(product.clone()), upper.max(product))
                        })
                    }
                    (IntegerOperator::Negate, [(lower, upper)]) => (-upper, -lower),
                    _ => {
                        return Err(ExactScalarRefusal::InvalidBody {
                            body_node_id: node_id.clone(),
                        });
                    }
                }
            } else {
                return Err(ExactScalarRefusal::OperandTypeMismatch {
                    position,
                    expected: ScalarForm::Integer,
                    found: scalar_type_form(graph, &node.semantic_type),
                });
            }
        } else if &*node.node_tag == CheckedNodeTag::Expression.as_wire() {
            return Err(ExactScalarRefusal::InvalidBody {
                body_node_id: node_id.clone(),
            });
        } else {
            return Err(ExactScalarRefusal::OperandUnsupported {
                position,
                term: node.node_tag.to_string(),
            });
        };
        check_operand_endpoint_bits(node_id, &result.0, &result.1)?;
        active.remove(node_id);
        memo.insert(node_id.clone(), result.clone());
        Ok(result)
    }
    let (lower, upper) = visit(
        graph,
        node_id,
        position,
        &mut BTreeSet::new(),
        &mut BTreeMap::new(),
        0,
        &mut 0,
    )?;
    Ok((lower.to_string(), upper.to_string()))
}

fn check_operand_endpoint_bits(
    node_id: &CheckedNodeId,
    lower: &BigInt,
    upper: &BigInt,
) -> Result<(), ExactScalarRefusal> {
    for endpoint in [lower, upper] {
        let consumed = endpoint.bits();
        if consumed > OPERAND_RANGE_MAX_ENDPOINT_BITS {
            return Err(ExactScalarRefusal::OperandRangeLimitExceeded {
                node_id: node_id.clone(),
                resource_kind: OperandRangeLimitKind::EndpointBits,
                limit: OPERAND_RANGE_MAX_ENDPOINT_BITS,
                consumed,
            });
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Parameters against IR bounds
// ---------------------------------------------------------------------------

/// Check every descriptor parameter the IR carries against the node's result bound of its form,
/// returning the bounds checked: the result bound first, then, for integer arithmetic, division
/// and modulo, each operand's own bound. Only the result bound is compared with the descriptor.
fn check_parameters<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node: &CompleteContractNodeV2,
    operation: &ExactScalarOperation,
) -> Result<Vec<CheckedNodeId>, ItemCheckError> {
    check_parameters_from_view(graph, BoundView::complete(node), operation)
}

fn check_parameters_from_view<Body: GraphBody, NodeBody>(
    graph: &Graph<'_, Body>,
    node: BoundView<'_, NodeBody>,
    operation: &ExactScalarOperation,
) -> Result<Vec<CheckedNodeId>, ItemCheckError> {
    let bounds = Bounds { graph, node };
    let checked = match operation {
        ExactScalarOperation::IntegerArithmetic { domain, .. }
        | ExactScalarOperation::IntegerDivision { domain, .. }
        | ExactScalarOperation::IntegerModulo { domain } => {
            let declared = match domain {
                IntegerDomain::Bounded(interval) => Some(interval),
                IntegerDomain::Mathematical => None,
            };
            let result = bounds.equal(BoundForm::IntegerRange, read_integer_range, declared)?;
            let mut checked = vec![result];
            for bound in bounds.integer_operand_bounds()? {
                if !checked.contains(&bound) {
                    checked.push(bound);
                }
            }
            return Ok(checked);
        }
        ExactScalarOperation::RationalArithmetic { domain, .. } => bounds.equal(
            BoundForm::RationalRange,
            read_rational_range,
            domain.as_ref(),
        )?,
        ExactScalarOperation::DecimalArithmetic { target, .. } => {
            bounds.equal(BoundForm::DecimalRange, read_decimal_range, Some(target))?
        }
        ExactScalarOperation::IeeeArithmetic { rounding, .. }
        | ExactScalarOperation::IeeeWidthConversion { rounding, .. } => bounds.equal(
            BoundForm::FloatRounding,
            read_float_rounding,
            Some(rounding),
        )?,
        ExactScalarOperation::TextAdmission { text_type } => {
            bounds.equal(BoundForm::TextBounds, read_text_bounds, Some(text_type))?
        }
        ExactScalarOperation::Ordering { .. }
        | ExactScalarOperation::Boolean { .. }
        | ExactScalarOperation::Equality { .. }
        | ExactScalarOperation::IeeeComparison { .. }
        | ExactScalarOperation::TextComparison { .. }
        | ExactScalarOperation::EnumComparison { .. }
        | ExactScalarOperation::QuantityArithmetic { .. }
        | ExactScalarOperation::QuantityComparison { .. }
        | ExactScalarOperation::QuantityConversion => return Ok(Vec::new()),
    };
    Ok(vec![checked])
}

/// The bound inputs shared by old lowering records and admitted FR-330 closures.
/// No per-node IR identity is reconstructed for the package path.
struct BoundView<'n, Body = CheckedTerm> {
    root: &'n CheckedSemanticNodeV2<Body>,
    semantic_type: CheckedNodeId,
    bounds: Vec<CheckedNodeId>,
    selected: Option<SelectedResult<'n>>,
}

impl<Body> Clone for BoundView<'_, Body> {
    fn clone(&self) -> Self {
        Self {
            root: self.root,
            semantic_type: self.semantic_type.clone(),
            bounds: self.bounds.clone(),
            selected: self.selected.clone(),
        }
    }
}

#[derive(Clone)]
struct SelectedResult<'a> {
    package: &'a ContractPackageV1,
    application: &'a ContractOccurrenceKeyV1,
    conversion: Option<&'a ContractOccurrenceKeyV1>,
}

impl<'n> BoundView<'n, CheckedTerm> {
    fn complete(node: &'n CompleteContractNodeV2) -> Self {
        Self {
            root: &node.node,
            semantic_type: node.semantic_type.clone(),
            bounds: node.bounds.clone(),
            selected: None,
        }
    }
}

impl<'n> BoundView<'n, Value> {
    fn admitted(
        closure: &'n quire_contract_model::ContractReachedClosureV1<'_>,
        selected: SelectedResult<'n>,
    ) -> Self {
        Self {
            root: closure.root,
            semantic_type: closure.root.semantic_type.clone(),
            bounds: closure.bounds.clone(),
            selected: Some(selected),
        }
    }
}

struct Bounds<'g, 'n, Body: GraphBody = Value, NodeBody = CheckedTerm> {
    graph: &'g Graph<'g, Body>,
    node: BoundView<'n, NodeBody>,
}

impl<Body: GraphBody, NodeBody> Bounds<'_, '_, Body, NodeBody> {
    /// Read the result bound of `form`: the one selector both [`Self::equal`] and
    /// [`derive_exact_scalar_items`] use. Returns the bound's node and the value it reads as.
    ///
    /// The result's role is named by the node itself. A result typed by a `bounded_domain` has
    /// that domain as its bound. A result typed by a scalar type has the bound of the narrowing
    /// conversion that consumes it (QSL's shape for a bounded context); failing one, the one
    /// reachable bound of `form` over that type and, where several are reachable, the one that is
    /// not an operand's own bound; a node whose bounds do not resolve that way is ambiguous.
    fn read<T>(
        &self,
        form: BoundForm,
        read: fn(&[CheckedTerm]) -> Option<T>,
    ) -> Result<(CheckedNodeId, T), ExactScalarRefusal> {
        if let Some(selected) = &self.node.selected {
            let result = selected
                .package
                .result_context(
                    &self.node.root.node_id,
                    selected.application,
                    selected.conversion,
                    form.ir(),
                )
                .map_err(|_| ExactScalarRefusal::IncompleteContext)?;
            let bound = match result {
                ContractResultViewV1::Lowered(result) => match result.context {
                    ContractResultContextV1::DirectBounded { bound }
                    | ContractResultContextV1::Narrowed { bound, .. } => bound.node,
                    ContractResultContextV1::MissingBound { type_key } => {
                        return Err(ExactScalarRefusal::MissingBound {
                            bounded_type: type_key,
                            expected_form: form,
                        });
                    }
                    ContractResultContextV1::Unnarrowed => {
                        return Err(ExactScalarRefusal::MissingBound {
                            bounded_type: self.node.semantic_type.clone(),
                            expected_form: form,
                        });
                    }
                    ContractResultContextV1::IncompleteContext => {
                        return Err(ExactScalarRefusal::IncompleteContext);
                    }
                },
                ContractResultViewV1::NonLowered(_) | ContractResultViewV1::Absent => {
                    return Err(ExactScalarRefusal::IncompleteContext);
                }
            };
            let value = aggregate_members(self.graph.term(&bound.node_id))
                .and_then(read)
                .ok_or_else(|| ExactScalarRefusal::UnreadableBound {
                    bound: bound.node_id.clone(),
                })?;
            return Ok((bound.node_id.clone(), value));
        }
        let bounded_type = &self.node.semantic_type;
        let typing = self.graph.node(bounded_type);
        let bound = match typing.filter(|typing| is_bounded_domain(typing)) {
            Some(bound) if &*bound.semantic_form == form.wire() => bound,
            Some(bound) => {
                return Err(ExactScalarRefusal::MissingBound {
                    bounded_type: bound.semantic_type.clone(),
                    expected_form: form,
                });
            }
            None => self.scalar_result_bound(form)?,
        };
        let value = aggregate_members(self.graph.term(&bound.node_id))
            .and_then(read)
            .ok_or_else(|| ExactScalarRefusal::UnreadableBound {
                bound: bound.node_id.clone(),
            })?;
        Ok((bound.node_id.clone(), value))
    }

    /// How the nodes consuming this node narrow it, for a node whose result is typed by a scalar
    /// type. QSL wraps an expression in a bounded context in a narrowing `conversion` typed by the
    /// bound, and IR's forward-only `bounds` closure never reaches it from the expression, so the
    /// enclosing conversion is where the result bound is named.
    fn narrowing(&self) -> Narrowing<'_, Body> {
        narrowing(self.graph, &self.node.root.node_id)
    }

    /// The result bound of a node whose result is typed by a scalar type.
    ///
    /// Where narrowing conversions consume the node, the result bound is the one bound they
    /// narrow it to: of the descriptor's form and over the node's own scalar type, or it is
    /// `MissingBound`; two or more distinct bounds are `AmbiguousBound`, and so is a node that a
    /// narrowing conversion and any other node both consume, since the narrow's bound does not
    /// bound the other use. Otherwise it is the one
    /// reachable bound of the form over that type, or, where several are reachable, the one that
    /// is no operand's own.
    fn scalar_result_bound(
        &self,
        form: BoundForm,
    ) -> Result<&CheckedSemanticNodeV2<Body>, ExactScalarRefusal> {
        let bounded_type = &self.node.semantic_type;
        let narrowing = self.narrowing();
        let narrowed = match &narrowing {
            Narrowing::Unnarrowed => &[][..],
            Narrowing::To(bounds) => bounds.as_slice(),
            Narrowing::Mixed => {
                return Err(ExactScalarRefusal::AmbiguousBound {
                    bounded_type: bounded_type.clone(),
                    expected_form: form,
                });
            }
        };
        match narrowed {
            [] => {}
            [bound] if &*bound.semantic_form != form.wire() => {
                return Err(ExactScalarRefusal::MissingBound {
                    bounded_type: bound.semantic_type.clone(),
                    expected_form: form,
                });
            }
            [bound] if &bound.semantic_type != bounded_type => {
                return Err(ExactScalarRefusal::MissingBound {
                    bounded_type: bounded_type.clone(),
                    expected_form: form,
                });
            }
            [bound] => return Ok(bound),
            [_, _, ..] => {
                return Err(ExactScalarRefusal::AmbiguousBound {
                    bounded_type: bounded_type.clone(),
                    expected_form: form,
                });
            }
        }
        let candidates = self
            .node
            .bounds
            .iter()
            .filter_map(|id| self.graph.node(id))
            .filter(|bound| {
                &bound.semantic_type == bounded_type && &*bound.semantic_form == form.wire()
            })
            .collect::<Vec<_>>();
        match candidates.as_slice() {
            [bound] => Ok(bound),
            [] => Err(ExactScalarRefusal::MissingBound {
                bounded_type: bounded_type.clone(),
                expected_form: form,
            }),
            [_, _, ..] => {
                let own = own_bounds(self.graph, self.graph.term(&self.node.root.node_id));
                let mut free = candidates.iter().filter(|candidate| {
                    !own.iter()
                        .flatten()
                        .any(|operand| operand.node_id == candidate.node_id)
                });
                match (free.next(), free.next()) {
                    (Some(bound), None) => Ok(bound),
                    _ => Err(ExactScalarRefusal::AmbiguousBound {
                        bounded_type: bounded_type.clone(),
                        expected_form: form,
                    }),
                }
            }
        }
    }

    /// The own `integer_range` bound of each operand, in argument order and deduplicated. An
    /// operand typed by a bound of another form has no integer bound and is `MissingBound`.
    ///
    /// A `reference` operand typed by a plain scalar type owns no bound, so nothing bounds it but
    /// the result's: where any operand owns a bound, or the result is typed by one or narrowed to
    /// one by a consuming conversion, the package bounds each operand by its own typing and such an
    /// operand is `RequiresBound`. A literal, inline or a reference to a `value` node whose body is
    /// a literal, is a constant: it owns no bound and a Kani harness ranges it at its own value
    /// ([`operand_ranges`]); one that is not a canonical `integer` literal is `OperandTypeMismatch`
    /// naming its kind, whatever the referenced node's type. Where no
    /// operand owns a bound and the result is scalar-typed and not narrowed, the operands are
    /// bounded by the one bound the node reaches (the shape IR-299 tracks), as before.
    fn integer_operand_bounds(&self) -> Result<Vec<CheckedNodeId>, ExactScalarRefusal> {
        let form = BoundForm::IntegerRange;
        let arguments = self
            .graph
            .arguments(&self.node.root.node_id)
            .unwrap_or_default();
        for (position, argument) in arguments.iter().enumerate() {
            if let Some(literal) = operand_literal(self.graph, argument) {
                if literal_integer(literal).is_none() {
                    return Err(ExactScalarRefusal::OperandTypeMismatch {
                        position,
                        expected: ScalarForm::Integer,
                        found: literal_form(literal),
                    });
                }
            }
        }
        let owned = own_bounds(self.graph, self.graph.term(&self.node.root.node_id));
        let result_typed = if let Some(selected) = &self.node.selected {
            match selected
                .package
                .result_context(
                    &self.node.root.node_id,
                    selected.application,
                    selected.conversion,
                    BoundedDomainForm::IntegerRange,
                )
                .map_err(|_| ExactScalarRefusal::IncompleteContext)?
            {
                ContractResultViewV1::Lowered(result) => match result.context {
                    ContractResultContextV1::DirectBounded { .. }
                    | ContractResultContextV1::Narrowed { .. } => true,
                    ContractResultContextV1::Unnarrowed => false,
                    ContractResultContextV1::MissingBound { .. } => selected.conversion.is_some(),
                    ContractResultContextV1::IncompleteContext => {
                        return Err(ExactScalarRefusal::IncompleteContext);
                    }
                },
                ContractResultViewV1::NonLowered(_) | ContractResultViewV1::Absent => {
                    return Err(ExactScalarRefusal::IncompleteContext);
                }
            }
        } else {
            self.graph
                .node(&self.node.semantic_type)
                .is_some_and(is_bounded_domain)
                || self.narrowing().narrowed()
        };
        if result_typed || owned.iter().any(Option::is_some) {
            for (position, (argument, own)) in arguments.iter().zip(&owned).enumerate() {
                let Some(target) = reference_target(argument) else {
                    continue;
                };
                let eligible = own.is_none()
                    && self.graph.node(&target).is_some_and(|node| {
                        &*node.node_tag == CheckedNodeTag::Expression.as_wire()
                            && scalar_type_form(self.graph, &node.semantic_type).as_deref()
                                == Some("integer")
                    });
                if eligible {
                    infer_integer_range(self.graph, &target, position)?;
                }
            }
            let plain = arguments
                .iter()
                .zip(&owned)
                .find_map(|(argument, own)| match own {
                    None if reference_target(argument).is_some_and(|target| {
                        self.graph.node(&target).is_some_and(|node| {
                            &*node.node_tag == CheckedNodeTag::Expression.as_wire()
                                && scalar_type_form(self.graph, &node.semantic_type).as_deref()
                                    == Some("integer")
                        })
                    }) =>
                    {
                        None
                    }
                    None => plain_reference_type(self.graph, argument),
                    Some(_) => None,
                });
            if let Some(unbounded_type) = plain {
                return Err(ExactScalarRefusal::RequiresBound { unbounded_type });
            }
        }
        let mut checked = Vec::new();
        for bound in owned.into_iter().flatten() {
            if &*bound.semantic_form != form.wire() {
                return Err(ExactScalarRefusal::MissingBound {
                    bounded_type: bound.semantic_type.clone(),
                    expected_form: form,
                });
            }
            if aggregate_members(self.graph.term(&bound.node_id))
                .and_then(read_integer_range)
                .is_none()
            {
                return Err(ExactScalarRefusal::UnreadableBound {
                    bound: bound.node_id.clone(),
                });
            }
            if !checked.contains(&bound.node_id) {
                checked.push(bound.node_id.clone());
            }
        }
        Ok(checked)
    }

    /// Read the single bound of `form` on the result type and require the
    /// declared parameter to equal it. An absent declaration (an unbounded
    /// descriptor) never equals an IR bound.
    fn equal<T: PartialEq>(
        &self,
        form: BoundForm,
        read: fn(&[CheckedTerm]) -> Option<T>,
        declared: Option<&T>,
    ) -> Result<CheckedNodeId, ExactScalarRefusal> {
        let (bound, value) = self.read(form, read)?;
        if declared == Some(&value) {
            Ok(bound)
        } else {
            Err(ExactScalarRefusal::BoundMismatch { bound, form })
        }
    }
}

fn literal_interval(lower: &CheckedTerm, upper: &CheckedTerm) -> Option<IntegerInterval> {
    IntegerInterval::new(literal_integer(lower)?, literal_integer(upper)?).ok()
}

/// The `integer_range` member names (QSpec `positive-operation-identities.json`).
pub(crate) const INTEGER_RANGE_MEMBERS: [&str; 2] = ["min", "max"];
/// The `rational_range` member names.
pub(crate) const RATIONAL_RANGE_MEMBERS: [&str; 4] = [
    "numerator_min",
    "numerator_max",
    "denominator_min",
    "denominator_max",
];
/// The `decimal_range` member names.
pub(crate) const DECIMAL_RANGE_MEMBERS: [&str; 5] = [
    "coefficient_min",
    "coefficient_max",
    "scale_min",
    "scale_max",
    "rounding",
];
/// The `float_rounding` member names.
pub(crate) const FLOAT_ROUNDING_MEMBERS: [&str; 1] = ["rounding"];
/// The `text_bounds` member names.
pub(crate) const TEXT_BOUNDS_MEMBERS: [&str; 3] = ["min", "max", "text_profile"];
/// The `collection_bounds` member names.
pub(crate) const COLLECTION_BOUNDS_MEMBERS: [&str; 2] = ["min", "max"];

pub(crate) fn read_integer_range(members: &[CheckedTerm]) -> Option<IntegerInterval> {
    let [lower, upper] = bound_members(members, INTEGER_RANGE_MEMBERS)?;
    literal_interval(lower, upper)
}

pub(crate) fn read_rational_range(members: &[CheckedTerm]) -> Option<RationalDomain> {
    let [numerator_lower, numerator_upper, denominator_lower, denominator_upper] =
        bound_members(members, RATIONAL_RANGE_MEMBERS)?;
    RationalDomain::new(
        literal_interval(numerator_lower, numerator_upper)?,
        literal_interval(denominator_lower, denominator_upper)?,
    )
    .ok()
}

pub(crate) fn read_decimal_range(members: &[CheckedTerm]) -> Option<DecimalType> {
    let [lower, upper, min_scale, max_scale, rounding] =
        bound_members(members, DECIMAL_RANGE_MEMBERS)?;
    DecimalType::new(
        literal_integer(lower)?,
        literal_integer(upper)?,
        literal_count(min_scale)?,
        literal_count(max_scale)?,
        RoundingMode::from_code(literal(rounding, LiteralKind::Text)?)?,
    )
    .ok()
}

fn read_float_rounding(members: &[CheckedTerm]) -> Option<RoundingMode> {
    let [rounding] = bound_members(members, FLOAT_ROUNDING_MEMBERS)?;
    RoundingMode::from_code(literal(rounding, LiteralKind::Text)?)
}

pub(crate) fn read_text_bounds(members: &[CheckedTerm]) -> Option<TextType> {
    let [min, max, profile] = bound_members(members, TEXT_BOUNDS_MEMBERS)?;
    TextType::new(
        literal_count(min)?,
        literal_count(max)?,
        TextProfile::from_code(literal(profile, LiteralKind::Text)?)?,
    )
    .ok()
}

impl ScalarForm {
    fn wire(self) -> &'static str {
        match self {
            Self::Boolean => "boolean",
            Self::Integer => "integer",
            Self::Rational => "rational",
            Self::Decimal => "decimal",
            Self::Float32 => "float32",
            Self::Float64 => "float64",
            Self::Text => "text",
            Self::Enum => "enum",
            Self::Unit => "unit",
        }
    }
}

/// The node shape a descriptor requires.
struct Shape {
    form: &'static str,
    body_operator: &'static str,
    operands: &'static [ScalarForm],
    result: ScalarForm,
}

impl Shape {
    fn binary(operands: &'static [ScalarForm; 2], result: ScalarForm) -> Self {
        Self {
            form: "binary",
            body_operator: "binary",
            operands,
            result,
        }
    }

    fn unary(operand: &'static [ScalarForm; 1], result: ScalarForm) -> Self {
        Self {
            form: "unary",
            body_operator: "unary",
            operands: operand,
            result,
        }
    }

    fn conversion(operand: &'static [ScalarForm; 1], result: ScalarForm) -> Self {
        Self {
            form: "conversion",
            body_operator: "convert",
            operands: operand,
            result,
        }
    }

    /// The `body_operator: "call"` half is grounded in the catalog: the
    /// operation-catalog's three IEEE-comparison identities
    /// (`quire.op.ieee.numeric_equal`, `.total_order`, `.bit_identical`) are
    /// all `call`-operator entries, and Contract IR's
    /// `validate_operations` refuses any package whose `body.operator`
    /// disagrees with its catalogued `operation.identity`'s own `operator`,
    /// so a real, admitted IEEE comparison node's `body.operator` is always
    /// `"call"`, never `"binary"`.
    ///
    /// The `form: "binary"` half is grounded only in this repo's own
    /// fixtures, not in IR or the catalog: IR does not couple
    /// `semantic_form` to `body.operator` -- `Expression::forms()` admits
    /// `"call"` and `"binary"` as independent, unrelated values -- so a real
    /// producer is free to emit `semantic_form: "call"` for an IEEE
    /// comparison node, and this shape would then refuse it `FormMismatch`.
    fn binary_call(operands: &'static [ScalarForm; 2], result: ScalarForm) -> Self {
        Self {
            form: "binary",
            body_operator: "call",
            operands,
            result,
        }
    }

    fn of(operation: &ExactScalarOperation) -> Result<Self, OracleGenerationError> {
        use ScalarForm as F;
        Ok(match operation {
            ExactScalarOperation::Boolean { operator, .. } => match operator {
                BooleanOperator::Not => Self::unary(&[F::Boolean], F::Boolean),
                BooleanOperator::And | BooleanOperator::Or | BooleanOperator::Implies => {
                    Self::binary(&[F::Boolean, F::Boolean], F::Boolean)
                }
            },
            ExactScalarOperation::Equality { operands, .. } => match operands {
                EqualityOperandKind::Boolean => Self::binary(&[F::Boolean, F::Boolean], F::Boolean),
                EqualityOperandKind::Integer => Self::binary(&[F::Integer, F::Integer], F::Boolean),
            },
            ExactScalarOperation::IntegerArithmetic { operator, .. } => match operator {
                IntegerOperator::Negate => Self::unary(&[F::Integer], F::Integer),
                IntegerOperator::Add | IntegerOperator::Subtract | IntegerOperator::Multiply => {
                    Self::binary(&[F::Integer, F::Integer], F::Integer)
                }
            },
            ExactScalarOperation::IntegerDivision { .. }
            | ExactScalarOperation::IntegerModulo { .. } => {
                Self::binary(&[F::Integer, F::Integer], F::Integer)
            }
            ExactScalarOperation::RationalArithmetic { operator, .. } => match operator {
                RationalOperator::Negate => Self::unary(&[F::Rational], F::Rational),
                RationalOperator::IntegerDivide => {
                    Self::binary(&[F::Integer, F::Integer], F::Rational)
                }
                RationalOperator::Add
                | RationalOperator::Subtract
                | RationalOperator::Multiply
                | RationalOperator::Divide => {
                    Self::binary(&[F::Rational, F::Rational], F::Rational)
                }
            },
            ExactScalarOperation::Ordering { operands, .. } => match operands {
                OrderingOperandKind::Integer => Self::binary(&[F::Integer, F::Integer], F::Boolean),
                OrderingOperandKind::Rational => {
                    Self::binary(&[F::Rational, F::Rational], F::Boolean)
                }
                OrderingOperandKind::Decimal => Self::binary(&[F::Decimal, F::Decimal], F::Boolean),
            },
            ExactScalarOperation::DecimalArithmetic { operator, .. } => match operator {
                DecimalOperator::Negate => Self::unary(&[F::Decimal], F::Decimal),
                DecimalOperator::Round => Self::conversion(&[F::Decimal], F::Decimal),
                DecimalOperator::Add
                | DecimalOperator::Subtract
                | DecimalOperator::Multiply
                | DecimalOperator::Divide => Self::binary(&[F::Decimal, F::Decimal], F::Decimal),
            },
            ExactScalarOperation::IeeeArithmetic { width, .. } => match width {
                IeeeWidth::Binary32 => Self::binary(&[F::Float32, F::Float32], F::Float32),
                IeeeWidth::Binary64 => Self::binary(&[F::Float64, F::Float64], F::Float64),
            },
            ExactScalarOperation::IeeeComparison { width, .. } => match width {
                IeeeWidth::Binary32 => Self::binary_call(&[F::Float32, F::Float32], F::Boolean),
                IeeeWidth::Binary64 => Self::binary_call(&[F::Float64, F::Float64], F::Boolean),
            },
            ExactScalarOperation::IeeeWidthConversion { source, target, .. } => {
                let operand: &'static [ScalarForm; 1] = match source {
                    IeeeWidth::Binary32 => &[F::Float32],
                    IeeeWidth::Binary64 => &[F::Float64],
                };
                Self::conversion(operand, F::of_width(*target)?)
            }
            ExactScalarOperation::TextAdmission { .. } => Self::conversion(&[F::Text], F::Text),
            ExactScalarOperation::TextComparison { .. } => {
                Self::binary(&[F::Text, F::Text], F::Boolean)
            }
            ExactScalarOperation::EnumComparison { .. } => {
                Self::binary(&[F::Enum, F::Enum], F::Boolean)
            }
            ExactScalarOperation::QuantityArithmetic { operator } => match operator {
                QuantityOperator::Power => Self::binary(&[F::Unit, F::Integer], F::Unit),
                QuantityOperator::Add
                | QuantityOperator::Subtract
                | QuantityOperator::Multiply
                | QuantityOperator::Divide => Self::binary(&[F::Unit, F::Unit], F::Unit),
            },
            ExactScalarOperation::QuantityConversion => Self::conversion(&[F::Unit], F::Unit),
            ExactScalarOperation::QuantityComparison { .. } => {
                Self::binary(&[F::Unit, F::Unit], F::Boolean)
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Operation identity
// ---------------------------------------------------------------------------

/// A refused item's claim and disposition together: the request item's own
/// descriptor-derived identity, marked
/// [`OperationProvenance::CallerDeclared`], paired with the
/// [`ClaimDisposition::Refused`] that names why. Returning the pair
/// keeps them from drifting apart -- a call site that built only the claim
/// half, or built a different disposition beside it, would not compile.
///
/// One of the three call sites, the `Err` arm guarding
/// [`catalogued_operation_identity`], used to be unreachable:
/// [`operation_confirmed`] was asked first and had already required that same
/// member, so `MissingOperationIdentity` was unconstructible through it and a
/// node missing the member was reported here instead, as though codegen had
/// declined to consume an identity that was not there. IR-224 reordered the
/// two, so that arm is now reachable: an identity-absent node reaches it, and
/// this function names that claim caller-declared while the disposition
/// beside it is `Refused`. Before the reorder such a node took the inline
/// `CallerDeclared` construction instead and generated an oracle. Nothing is
/// claimed here about the other sites: they refuse items `check_item`
/// rejected and duplicate requests, neither of which turns on the operation
/// member at all.
///
/// A lowered node whose catalogued operation disagrees with the descriptor is
/// `CallerDeclared` too, but does not come through here: it generates, and
/// builds its provenance inline at the disagreement site.
fn refused_claim(
    identity: String,
    refusal: ExactScalarRefusal,
) -> (
    OperationClaim,
    ClaimDisposition<GeneratedScalarClaim, ExactScalarRefusal>,
) {
    (
        OperationClaim {
            identity,
            provenance: OperationProvenance::CallerDeclared {
                blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
            },
        },
        ClaimDisposition::Refused { refusal },
    )
}

/// The node's own catalogued operation identity, or a typed refusal when the
/// member IR admission is supposed to guarantee is absent. `check_item`
/// having returned this node `Ok` already establishes that its `body` is an
/// `application` term (`application_arguments` requires `term ==
/// "application"`), and Contract IR's `validate_operations`
/// requires `identity` on every such node's `operation` member before the
/// package that contains it is ever admitted -- so this generator should
/// never receive a node for which this member is absent. It is another
/// crate's invariant, not this one's, so a violation is reported rather than
/// panicked on.
fn catalogued_operation_identity<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node: &CompleteContractNodeV2,
) -> Result<String, ExactScalarRefusal> {
    graph
        .operation(&node.node.node_id)
        .and_then(|operation| operation.get("identity"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| ExactScalarRefusal::MissingOperationIdentity {
            node_id: node.node.node_id.clone(),
        })
}

/// The catalogued identity a descriptor implies, and -- where the catalog's
/// own entry for it carries a `mode` member -- the `(kind, value)` that
/// member must equal for the descriptor's own rounding parameter to be
/// trusted. `None` when the descriptor names a combination the catalog has
/// no entry for at all (same-width [`ExactScalarOperation::IeeeWidthConversion`]).
///
/// This mirrors `tests/exact_scalar_support/package.rs`'s own
/// `corpus_operation`, which builds every fixture node from the identical
/// mapping (see its own doc for the catalog cross-reference); moving it here
/// lets [`operation_confirmed`] recognize an identity and mode IR already
/// confirmed -- not translate between codegen's and IR's vocabularies to
/// reconcile a disagreement between them, the same kind of closed match
/// `Shape::of` already makes over `ExactScalarOperation`.
struct CataloguedOperation {
    identity: &'static str,
    mode: Option<(&'static str, &'static str)>,
}

fn plain(identity: &'static str) -> Option<CataloguedOperation> {
    Some(CataloguedOperation {
        identity,
        mode: None,
    })
}

fn with_rounding(identity: &'static str, rounding: RoundingMode) -> Option<CataloguedOperation> {
    Some(CataloguedOperation {
        identity,
        mode: Some(("rounding", rounding.as_str())),
    })
}

fn catalogued_operation(
    operation: &ExactScalarOperation,
) -> Result<Option<CataloguedOperation>, OracleGenerationError> {
    use ExactScalarOperation as Op;
    Ok(match operation {
        Op::Boolean { operator, .. } => plain(match operator {
            BooleanOperator::And => "quire.op.boolean.and",
            BooleanOperator::Or => "quire.op.boolean.or",
            BooleanOperator::Not => "quire.op.boolean.not",
            BooleanOperator::Implies => "quire.op.boolean.implies",
        }),
        Op::Equality { operator, operands } => plain(match (operands, operator) {
            (EqualityOperandKind::Boolean, EqualityOperator::Equal) => "quire.op.boolean.eq",
            (EqualityOperandKind::Boolean, EqualityOperator::NotEqual) => "quire.op.boolean.ne",
            (EqualityOperandKind::Integer, EqualityOperator::Equal) => "quire.op.integer.eq",
            (EqualityOperandKind::Integer, EqualityOperator::NotEqual) => "quire.op.integer.ne",
        }),
        Op::IntegerArithmetic { operator, .. } => plain(match operator {
            IntegerOperator::Add => CatalogOperator::Add.entry().identity,
            IntegerOperator::Subtract => CatalogOperator::Subtract.entry().identity,
            IntegerOperator::Multiply => CatalogOperator::Multiply.entry().identity,
            IntegerOperator::Negate => CatalogOperator::Negate.entry().identity,
        }),
        Op::IntegerDivision { .. } => plain("quire.op.integer.div"),
        Op::IntegerModulo { .. } => plain("quire.op.integer.mod"),
        Op::RationalArithmetic { operator, .. } => plain(match operator {
            RationalOperator::Add => "quire.op.rational.add",
            RationalOperator::Subtract => "quire.op.rational.sub",
            RationalOperator::Multiply => "quire.op.rational.mul",
            // `rational.div`'s operands are `rational_promotable` (integer or
            // rational): there is no separate catalogued "integer division to
            // a rational result" identity.
            RationalOperator::Divide | RationalOperator::IntegerDivide => "quire.op.rational.div",
            RationalOperator::Negate => "quire.op.rational.negate",
        }),
        Op::Ordering { operator, operands } => plain(ordering_identity(operands, operator)?),
        Op::DecimalArithmetic { operator, target } => match operator {
            DecimalOperator::Add => with_rounding("quire.op.decimal.add", target.rounding()),
            DecimalOperator::Subtract => with_rounding("quire.op.decimal.sub", target.rounding()),
            DecimalOperator::Multiply => with_rounding("quire.op.decimal.mul", target.rounding()),
            DecimalOperator::Divide => with_rounding("quire.op.decimal.div", target.rounding()),
            DecimalOperator::Negate => plain("quire.op.decimal.negate"),
            DecimalOperator::Round => {
                with_rounding("quire.op.numeric.convert_rounding", target.rounding())
            }
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
            with_rounding(identity, *rounding)
        }
        Op::IeeeComparison { comparison, .. } => plain(match comparison {
            IeeeComparison::NumericEqual => "quire.op.ieee.numeric_equal",
            IeeeComparison::TotalOrder => "quire.op.ieee.total_order",
            IeeeComparison::BitIdentical => "quire.op.ieee.bit_identical",
        }),
        Op::IeeeWidthConversion {
            source,
            target,
            rounding,
        } => match (source, target) {
            (IeeeWidth::Binary64, IeeeWidth::Binary32) => {
                with_rounding("quire.op.ieee.to_float32", *rounding)
            }
            (IeeeWidth::Binary32, IeeeWidth::Binary64) => plain("quire.op.ieee.to_float64"),
            // Same-width "conversion" has no catalogued identity at all; a
            // descriptor naming one can never be confirmed.
            (IeeeWidth::Binary32, IeeeWidth::Binary32)
            | (IeeeWidth::Binary64, IeeeWidth::Binary64) => None,
        },
        Op::TextAdmission { .. } => plain("quire.op.numeric.convert"),
        // `text.*`/`enum.*` comparisons carry a catalogued `text_profile` mode
        // (`"nfc"`), but the descriptor itself has no profile-bearing field to
        // compare it against, so only the identity is checked.
        Op::TextComparison { operator } => {
            plain(comparison_identity(ComparisonFamily::Text, operator)?)
        }
        Op::EnumComparison { operator } => {
            plain(comparison_identity(ComparisonFamily::Enum, operator)?)
        }
        Op::QuantityConversion => with_rounding(QUANTITY_CONVERT_IDENTITY, RoundingMode::Exact),
        Op::QuantityArithmetic { operator } => plain(match operator {
            QuantityOperator::Add => "quire.op.quantity.add",
            QuantityOperator::Subtract => "quire.op.quantity.sub",
            QuantityOperator::Multiply => "quire.op.quantity.mul",
            QuantityOperator::Divide => "quire.op.quantity.div",
            QuantityOperator::Power => "quire.op.quantity.pow",
        }),
        Op::QuantityComparison { operator } => {
            plain(comparison_identity(ComparisonFamily::Quantity, operator)?)
        }
    })
}

/// `quire.op.<family>.<lt|le|gt|ge>` for an integer, rational or decimal
/// `Ordering` descriptor, one catalogued identity per (operands, operator).
fn ordering_identity(
    operands: &OrderingOperandKind,
    operator: &OrderingOperator,
) -> Result<&'static str, OracleGenerationError> {
    use OrderingOperandKind as Kind;
    use OrderingOperator as Op;
    if matches!(operands, Kind::Integer) {
        return CatalogOperator::from_ordering_operator(*operator)
            .map(|operation| operation.entry().identity)
            .ok_or_else(|| OracleGenerationError::unknown_variant("OrderingOperator"));
    }
    Ok(match (operands, operator) {
        (Kind::Rational, Op::Less) => "quire.op.rational.lt",
        (Kind::Rational, Op::LessOrEqual) => "quire.op.rational.le",
        (Kind::Rational, Op::Greater) => "quire.op.rational.gt",
        (Kind::Rational, Op::GreaterOrEqual) => "quire.op.rational.ge",
        (Kind::Decimal, Op::Less) => "quire.op.decimal.lt",
        (Kind::Decimal, Op::LessOrEqual) => "quire.op.decimal.le",
        (Kind::Decimal, Op::Greater) => "quire.op.decimal.gt",
        (Kind::Decimal, Op::GreaterOrEqual) => "quire.op.decimal.ge",
        (Kind::Integer, _) => {
            return Err(OracleGenerationError::unknown_variant(
                "OrderingOperandKind",
            ));
        }
    })
}

/// The catalog's comparison families that carry all six of
/// `eq`/`ne`/`lt`/`le`/`gt`/`ge`.
#[derive(Clone, Copy)]
enum ComparisonFamily {
    Text,
    Enum,
    Quantity,
}

/// `quire.op.<family>.<suffix>` for one of the catalog's six comparison
/// identities per family.
fn comparison_identity(
    family: ComparisonFamily,
    operator: &ComparisonOperator,
) -> Result<&'static str, OracleGenerationError> {
    use ComparisonFamily as F;
    use ComparisonOperator as Op;
    Ok(match (family, operator) {
        (F::Text, Op::Equal) => "quire.op.text.eq",
        (F::Text, Op::NotEqual) => "quire.op.text.ne",
        (F::Text, Op::Less) => "quire.op.text.lt",
        (F::Text, Op::LessOrEqual) => "quire.op.text.le",
        (F::Text, Op::Greater) => "quire.op.text.gt",
        (F::Text, Op::GreaterOrEqual) => "quire.op.text.ge",
        (F::Enum, Op::Equal) => "quire.op.enum.eq",
        (F::Enum, Op::NotEqual) => "quire.op.enum.ne",
        (F::Enum, Op::Less) => "quire.op.enum.lt",
        (F::Enum, Op::LessOrEqual) => "quire.op.enum.le",
        (F::Enum, Op::Greater) => "quire.op.enum.gt",
        (F::Enum, Op::GreaterOrEqual) => "quire.op.enum.ge",
        (F::Quantity, Op::Equal) => "quire.op.quantity.eq",
        (F::Quantity, Op::NotEqual) => "quire.op.quantity.ne",
        (F::Quantity, Op::Less) => "quire.op.quantity.lt",
        (F::Quantity, Op::LessOrEqual) => "quire.op.quantity.le",
        (F::Quantity, Op::Greater) => "quire.op.quantity.gt",
        (F::Quantity, Op::GreaterOrEqual) => "quire.op.quantity.ge",
    })
}

/// Whether the checked node's own catalogued operation genuinely confirms the
/// descriptor beyond `check_item`'s shape checks: its `operation.identity`
/// names the same catalogued operation the descriptor implies
/// ([`catalogued_operation`]), its `operation.mode` (where that catalog entry
/// has one) carries the same value, and -- for `IntegerDivision`, the one
/// family with more than one catalogued law definition -- its
/// `operation.laws` names the descriptor's own division profile. IR's
/// `integer_division` law role and this crate's own
/// `DivisionProfile::definition_identity()` name the same three law
/// definitions (`quire.value.integer-division.{truncating,floor,euclidean}/v1`
/// -- see `operation_identity`'s own use of `profile.definition_identity()`),
/// so comparing them is not a bridge between codegen's and IR's vocabularies:
/// it is one vocabulary, read from two places. Every other family either has
/// no more than one catalogued law definition (`ieee_profile`,
/// `text_profile`) or none at all, so its shape and identity/mode checks are
/// already the whole story for it.
fn operation_confirmed<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node: &CompleteContractNodeV2,
    operation: &ExactScalarOperation,
) -> Result<bool, OracleGenerationError> {
    let Some(catalogued) = catalogued_operation(operation)? else {
        return Ok(false);
    };
    let Some(payload) = graph.operation(&node.node.node_id) else {
        return Ok(false);
    };
    if payload["identity"].as_str() != Some(catalogued.identity) {
        return Ok(false);
    }
    if let Some((kind, value)) = catalogued.mode {
        let mode = &payload["mode"];
        if mode["kind"].as_str() != Some(kind) || mode["value"].as_str() != Some(value) {
            return Ok(false);
        }
    }
    if let ExactScalarOperation::IntegerDivision { profile, .. } = operation {
        let law_confirmed = payload["laws"].as_array().into_iter().flatten().any(|law| {
            law["definition"]["identity"].as_str() == Some(profile.definition_identity())
        });
        if !law_confirmed {
            return Ok(false);
        }
    }
    Ok(true)
}

/// One oracle's function, self-contained for direct embedding in a generated
/// Kani harness file (`kani::generate::scalar::render_scalar`): unlike
/// [`SourceBuilder::finish`]'s crate-wide accumulation (one set of shared
/// helpers for every oracle in the crate), this always includes whatever
/// preamble helpers its own body needs, even when that duplicates them
/// across separate harness files that each embed a different oracle.
fn standalone_oracle_source(
    symbol: &str,
    identity: &str,
    operation: &ExactScalarOperation,
) -> Result<String, OracleGenerationError> {
    let mut builder = SourceBuilder::default();
    builder.oracle(symbol, identity, operation)?;
    Ok(builder.finish())
}

/// A generated claim whose oracle is named and rendered after every claim is checked.
/// The readable operation family of `operation`, such as `integer_add`, for generated names.
fn operation_name(operation: &ExactScalarOperation) -> Result<String, OracleGenerationError> {
    let identity = operation_identity(operation)?;
    Ok(bounded_readable_component(
        identity.split(' ').next().unwrap_or_default(),
    ))
}

fn operation_identity(operation: &ExactScalarOperation) -> Result<String, OracleGenerationError> {
    Ok(match operation {
        ExactScalarOperation::Boolean { operator, .. } => {
            let name = match operator {
                BooleanOperator::And => "and",
                BooleanOperator::Or => "or",
                BooleanOperator::Not => "not",
                BooleanOperator::Implies => "implies",
            };
            format!("boolean.{name}")
        }
        ExactScalarOperation::Equality { operator, operands } => {
            let kind = match operands {
                EqualityOperandKind::Boolean => "boolean",
                EqualityOperandKind::Integer => "integer",
            };
            let name = match operator {
                EqualityOperator::Equal => "eq",
                EqualityOperator::NotEqual => "ne",
            };
            format!("{kind}.{name}")
        }
        ExactScalarOperation::IntegerArithmetic { operator, domain } => {
            let name = match operator {
                IntegerOperator::Add => "add",
                IntegerOperator::Subtract => "subtract",
                IntegerOperator::Multiply => "multiply",
                IntegerOperator::Negate => "negate",
            };
            format!("integer.{name} domain={}", integer_domain_identity(domain)?)
        }
        ExactScalarOperation::IntegerDivision { profile, domain } => format!(
            "{} domain={}",
            profile.definition_identity(),
            integer_domain_identity(domain)?
        ),
        ExactScalarOperation::IntegerModulo { domain } => {
            format!("integer.modulo domain={}", integer_domain_identity(domain)?)
        }
        ExactScalarOperation::RationalArithmetic { operator, domain } => {
            let name = match operator {
                RationalOperator::Add => "add",
                RationalOperator::Subtract => "subtract",
                RationalOperator::Multiply => "multiply",
                RationalOperator::Divide => "divide",
                RationalOperator::Negate => "negate",
                RationalOperator::IntegerDivide => "integer_divide",
            };
            let domain = domain.as_ref().map_or_else(
                || "unbounded".to_owned(),
                |domain| {
                    format!(
                        "{}/{}",
                        interval_identity(domain.numerator()),
                        interval_identity(domain.denominator())
                    )
                },
            );
            format!("rational.{name} domain={domain}")
        }
        ExactScalarOperation::Ordering { operator, operands } => {
            let name = match operator {
                OrderingOperator::Less => "less",
                OrderingOperator::LessOrEqual => "less_or_equal",
                OrderingOperator::Greater => "greater",
                OrderingOperator::GreaterOrEqual => "greater_or_equal",
            };
            let kind = match operands {
                OrderingOperandKind::Integer => "integer",
                OrderingOperandKind::Rational => "rational",
                OrderingOperandKind::Decimal => "decimal",
            };
            format!("ordering.{name} operands={kind}")
        }
        ExactScalarOperation::DecimalArithmetic { operator, target } => {
            let name = match operator {
                DecimalOperator::Add => "add",
                DecimalOperator::Subtract => "subtract",
                DecimalOperator::Multiply => "multiply",
                DecimalOperator::Negate => "negate",
                DecimalOperator::Divide => "divide",
                DecimalOperator::Round => "round",
            };
            format!(
                "decimal.{name} target=[{},{}] scale=[{},{}] rounding={}",
                target.lower(),
                target.upper(),
                target.min_scale(),
                target.max_scale(),
                target.rounding().as_str()
            )
        }
        ExactScalarOperation::IeeeArithmetic {
            operator,
            width,
            rounding,
        } => {
            let name = match operator {
                IeeeArithmeticOperator::Add => "add",
                IeeeArithmeticOperator::Subtract => "subtract",
                IeeeArithmeticOperator::Multiply => "multiply",
                IeeeArithmeticOperator::Divide => "divide",
            };
            format!(
                "ieee.{name} width={} rounding={}",
                width.as_str(),
                rounding.as_str()
            )
        }
        ExactScalarOperation::IeeeComparison { comparison, width } => {
            let name = match comparison {
                IeeeComparison::NumericEqual => "numeric_equal",
                IeeeComparison::TotalOrder => "total_order",
                IeeeComparison::BitIdentical => "bit_identical",
            };
            format!("ieee.{name} width={}", width.as_str())
        }
        ExactScalarOperation::IeeeWidthConversion {
            source,
            target,
            rounding,
        } => format!(
            "ieee.convert_width source={} target={} rounding={}",
            source.as_str(),
            target.as_str(),
            rounding.as_str()
        ),
        ExactScalarOperation::TextAdmission { text_type } => format!(
            "text.admit length=[{},{}] profile={}",
            text_type.min(),
            text_type.max(),
            text_type.profile().as_str()
        ),
        ExactScalarOperation::TextComparison { operator } => {
            format!("text.compare.{}", comparison_name(*operator)?)
        }
        ExactScalarOperation::EnumComparison { operator } => {
            format!("enum.compare.{}", comparison_name(*operator)?)
        }
        ExactScalarOperation::QuantityConversion => "quantity.convert.exact".to_owned(),
        ExactScalarOperation::QuantityArithmetic { operator } => {
            let name = match operator {
                QuantityOperator::Add => "add",
                QuantityOperator::Subtract => "subtract",
                QuantityOperator::Multiply => "multiply",
                QuantityOperator::Divide => "divide",
                QuantityOperator::Power => "power",
            };
            format!("quantity.{name}")
        }
        ExactScalarOperation::QuantityComparison { operator } => {
            format!("quantity.compare.{}", comparison_name(*operator)?)
        }
    })
}

fn comparison_name(operator: ComparisonOperator) -> Result<&'static str, OracleGenerationError> {
    match operator {
        ComparisonOperator::Equal => Ok("equal"),
        ComparisonOperator::NotEqual => Ok("not_equal"),
        ComparisonOperator::Less => Ok("less"),
        ComparisonOperator::LessOrEqual => Ok("less_or_equal"),
        ComparisonOperator::Greater => Ok("greater"),
        ComparisonOperator::GreaterOrEqual => Ok("greater_or_equal"),
    }
}

fn integer_domain_identity(domain: &IntegerDomain) -> Result<String, OracleGenerationError> {
    match domain {
        IntegerDomain::Mathematical => Ok("mathematical".to_owned()),
        IntegerDomain::Bounded(interval) => Ok(interval_identity(interval)),
    }
}

fn interval_identity(interval: &IntegerInterval) -> String {
    format!("[{},{}]", interval.lower(), interval.upper())
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

const SOURCE_HEADER: &str = "\
// @generated by quire-contract-codegen exact scalar oracles. Do not edit.
//
// Each function calls its owning `quire-exact`, `quire-semantic-value` or Runtime
// operator with the caller's `Meter`; no charge amount appears in this source.
// The source has no inner attributes so that it can be `include!`d; the manifest forbids
// unsafe code instead.

use quire_exact as rt;

/// Why a generated oracle stopped before an operator outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OracleStop {
    /// The operator refused the operands or a generated type as ill-typed.
    IllTyped(rt::IllTyped),
    /// A generated constant was rejected by an exact-kernel constructor.
    InvalidConstant,
    /// An IEEE operand does not have the declared width.
    OperandWidth,
    /* quantity refusal */
}
";

const INTEGER_HELPER: &str = "
fn integer(spelling: &str) -> Result<rt::Integer, OracleStop> {
    spelling.parse().map_err(|_| OracleStop::InvalidConstant)
}
";

const WIDTH_HELPER: &str = "
fn expect_width(value: rt::IeeeValue, width: rt::IeeeWidth) -> Result<rt::IeeeValue, OracleStop> {
    if value.width() == width {
        Ok(value)
    } else {
        Err(OracleStop::OperandWidth)
    }
}
";

#[derive(Default)]
struct SourceBuilder {
    functions: String,
    needs_integer: bool,
    needs_width: bool,
}

/// Render scalar operation helpers for a checked clause condition using the same
/// runtime calls and source template as standalone scalar oracles.
pub(crate) fn render_clause_operation_helpers(
    operations: &[(String, String, ExactScalarOperation)],
) -> Result<String, OracleGenerationError> {
    let mut source = SourceBuilder::default();
    for (symbol, identity, operation) in operations {
        source.oracle(symbol, identity, operation)?;
    }
    Ok(source.finish())
}

/// Select an admitted clause subexpression's supported scalar renderer without
/// duplicating the operation catalog in the clause generator.
pub(crate) fn clause_scalar_operation(
    identity: &str,
    result_bound: Option<(i128, i128)>,
) -> Option<ExactScalarOperation> {
    match family_of_identity(identity)? {
        Family::Boolean(operator) => Some(ExactScalarOperation::Boolean {
            right_may_stop: !matches!(operator, BooleanOperator::Not),
            operator,
        }),
        Family::Equality(operator, operands) => {
            Some(ExactScalarOperation::Equality { operator, operands })
        }
        Family::Ordering(operator, operands) if operands == OrderingOperandKind::Integer => {
            Some(ExactScalarOperation::Ordering { operator, operands })
        }
        Family::IntegerArithmetic(operator) => {
            let domain = match result_bound {
                Some((lower, upper)) => {
                    IntegerDomain::Bounded(IntegerInterval::new(lower.into(), upper.into()).ok()?)
                }
                None => IntegerDomain::Mathematical,
            };
            Some(ExactScalarOperation::IntegerArithmetic { operator, domain })
        }
        _ => None,
    }
}

impl SourceBuilder {
    fn oracle(
        &mut self,
        symbol: &str,
        identity: &str,
        operation: &ExactScalarOperation,
    ) -> Result<(), OracleGenerationError> {
        let body = self.body(operation)?;
        let parameters = body
            .parameters
            .iter()
            .map(|(name, ty)| format!("{name}: {ty}, "))
            .collect::<String>();
        self.functions.push_str(&format!(
            "\n/// `{}`.\npub fn {symbol}({parameters}meter: &mut rt::Meter) -> Result<{}, OracleStop> {{\n",
            identity, body.output.render()
        ));
        for line in body.prelude {
            self.functions.push_str("    ");
            self.functions.push_str(&line);
            self.functions.push('\n');
        }
        self.functions.push_str("    ");
        self.functions.push_str(&body.call);
        self.functions.push_str("\n}\n");
        Ok(())
    }

    fn finish(self) -> String {
        let quantity_refusal = if self.functions.contains("quantity_owner::") {
            "/// QSV quantity type checking or result-unit identity refused before evaluation.\n    Quantity(quire_semantic_value::quantity::QuantityRefusal),"
        } else {
            ""
        };
        let mut source = SOURCE_HEADER.replace("/* quantity refusal */", quantity_refusal);
        if self.functions.contains("quantity_owner::") {
            source.push_str("use quire_semantic_value::quantity as quantity_owner;\n");
        }
        if self.functions.contains("sv::") {
            source.push_str("use quire_semantic_value::declaration as sv;\n");
        }
        if self.functions.contains("enum_owner::") {
            source.push_str("use quire_semantic_value::enumeration as enum_owner;\n");
        }
        if self.functions.contains("residue::") {
            source.push_str("use quire_contract_runtime::exact as residue;\n");
        }
        if self.needs_integer {
            source.push_str(INTEGER_HELPER);
        }
        if self.needs_width {
            source.push_str(WIDTH_HELPER);
        }
        source.push_str(&self.functions);
        source
    }

    fn integer_domain(&mut self, domain: &IntegerDomain) -> Result<String, OracleGenerationError> {
        match domain {
            IntegerDomain::Mathematical => Ok("rt::IntegerDomain::Mathematical".to_owned()),
            IntegerDomain::Bounded(interval) => Ok(format!(
                "rt::IntegerDomain::Bounded({})",
                self.interval(interval)
            )),
        }
    }

    /// `rt::evaluate_integer_arithmetic`'s bound argument, `Option<&rt::IntegerInterval>`: the
    /// same two states codegen's own `IntegerDomain` carries, since the kernel dropped the
    /// redundant `Mathematical`/`Bounded` wrapper in favor of `None`/`Some` directly over the
    /// interval it already had (`quire-exact` `src/numeric.rs`). Unlike
    /// [`Self::integer_domain`], which still emits the wrapper for `rt::divide`/`rt::modulo`,
    /// this is only for `evaluate_integer_arithmetic`'s call site.
    fn integer_arithmetic_bound(
        &mut self,
        domain: &IntegerDomain,
    ) -> Result<(Vec<String>, String), OracleGenerationError> {
        match domain {
            IntegerDomain::Mathematical => Ok((Vec::new(), "None".to_owned())),
            IntegerDomain::Bounded(interval) => Ok((
                vec![format!("let domain = {};", self.interval(interval))],
                "Some(&domain)".to_owned(),
            )),
        }
    }

    fn interval(&mut self, interval: &IntegerInterval) -> String {
        self.needs_integer = true;
        format!(
            "rt::IntegerInterval::new(integer(\"{}\")?, integer(\"{}\")?).map_err(|_| OracleStop::InvalidConstant)?",
            interval.lower(),
            interval.upper()
        )
    }

    fn decimal_type(&mut self, decimal: &DecimalType) -> Result<String, OracleGenerationError> {
        self.needs_integer = true;
        Ok(format!(
            "rt::DecimalType::new(integer(\"{}\")?, integer(\"{}\")?, {}, {}, {}).map_err(|_| OracleStop::InvalidConstant)?",
            decimal.lower(),
            decimal.upper(),
            decimal.min_scale(),
            decimal.max_scale(),
            rounding_path(decimal.rounding())?
        ))
    }

    fn body(&mut self, operation: &ExactScalarOperation) -> Result<Body, OracleGenerationError> {
        Ok(match operation {
            ExactScalarOperation::Boolean { operator, right_may_stop } => {
                let (parameters, connective) = match operator {
                    BooleanOperator::And => (binary("bool"), "And(left, right)"),
                    BooleanOperator::Or => (binary("bool"), "Or(left, right)"),
                    BooleanOperator::Not => (unary("bool"), "Not(operand)"),
                    BooleanOperator::Implies => (binary("bool"), "Implies(left, right)"),
                };
                let (parameters, call) = if *right_may_stop {
                    let kind = match operator {
                        BooleanOperator::And => "And",
                        BooleanOperator::Or => "Or",
                        BooleanOperator::Implies => "Implies",
                        BooleanOperator::Not => return Err(OracleGenerationError::unknown_variant("BooleanOperator")),
                    };
                    (
                        vec![("left", "bool"), ("right", "impl FnOnce() -> rt::Outcome<bool>")],
                        format!("Ok(residue::evaluate_boolean_short_circuit(residue::ShortCircuitConnective::{kind}, left, || {{\n        right()\n    }}, meter))"),
                    )
                } else {
                    (parameters, format!("Ok(rt::evaluate_boolean(rt::BooleanConnective::{connective}, meter))"))
                };
                Body {
                    parameters,
                    output: OracleOutput::Value("bool"),
                    prelude: Vec::new(),
                    call,
                }
            }
            ExactScalarOperation::Equality { operator, operands } => {
                let operator = match operator {
                    EqualityOperator::Equal => "Equal",
                    EqualityOperator::NotEqual => "NotEqual",
                };
                let (ty, value_type, value) = match operands {
                    EqualityOperandKind::Boolean => ("bool", "Boolean", "Boolean"),
                    EqualityOperandKind::Integer => ("&rt::Integer", "Integer", "Integer"),
                };
                let (left, right) = match operands {
                    EqualityOperandKind::Boolean => ("left".to_owned(), "right".to_owned()),
                    EqualityOperandKind::Integer => ("left.clone()".to_owned(), "right.clone()".to_owned()),
                };
                Body {
                    parameters: binary(ty),
                    output: OracleOutput::Value("bool"),
                    prelude: vec![format!(
                        "let environment = sv::TypeEnvironment::default();\n    let checked = environment.check_equality(sv::EqualityOperator::{operator}, sv::EqualityOperand::typed(rt::ValueType::{value_type}), sv::EqualityOperand::typed(rt::ValueType::{value_type}), &quire_semantic_value::enumeration::EnumMemberIndex::default()).map_err(OracleStop::IllTyped)?;"
                    )],
                    call: format!("Ok(checked.evaluate(&environment, &rt::Value::{value}({left}), &rt::Value::{value}({right}), meter))"),
                }
            }
            ExactScalarOperation::IntegerArithmetic { operator, domain } => {
                let (prelude, domain_argument) = self.integer_arithmetic_bound(domain)?;
                let (parameters, operation) = match operator {
                    IntegerOperator::Negate => (unary("&rt::Integer"), "Negate(operand)"),
                    IntegerOperator::Add => (binary("&rt::Integer"), "Add(left, right)"),
                    IntegerOperator::Subtract => (binary("&rt::Integer"), "Subtract(left, right)"),
                    IntegerOperator::Multiply => (binary("&rt::Integer"), "Multiply(left, right)"),
                };
                Body {
                    parameters,
                    output: OracleOutput::Value("rt::Integer"),
                    prelude,
                    call: format!(
                        "Ok(rt::evaluate_integer_arithmetic(rt::IntegerArithmetic::{operation}, {domain_argument}, meter))"
                    ),
                }
            }
            ExactScalarOperation::IntegerDivision { profile, domain } => {
                let domain = self.integer_domain(domain)?;
                let profile = match profile {
                    DivisionProfile::Truncating => "Truncating",
                    DivisionProfile::Floor => "Floor",
                    DivisionProfile::Euclidean => "Euclidean",
                };
                Body {
                    parameters: binary("&rt::Integer"),
                    output: OracleOutput::Value("rt::Integer"),
                    prelude: vec![format!("let domain = {domain};")],
                    call: format!(
                        "Ok(rt::divide(rt::DivisionProfile::{profile}, rt::DivisionMember::Quotient, left, right, &domain, meter))"
                    ),
                }
            }
            ExactScalarOperation::IntegerModulo { domain } => {
                let domain = self.integer_domain(domain)?;
                Body {
                    parameters: binary("&rt::Integer"),
                    output: OracleOutput::Value("rt::Integer"),
                    prelude: vec![format!("let domain = {domain};")],
                    call: "Ok(rt::modulo(left, right, &domain, meter))".to_owned(),
                }
            }
            ExactScalarOperation::RationalArithmetic { operator, domain } => {
                let (domain_prelude, domain_argument) = match domain {
                    Some(domain) => {
                        let numerator = self.interval(domain.numerator());
                        let denominator = self.interval(domain.denominator());
                        (
                            vec![format!(
                                "let domain = rt::RationalDomain::new({numerator}, {denominator}).map_err(|_| OracleStop::InvalidConstant)?;"
                            )],
                            "Some(&domain)",
                        )
                    }
                    None => (Vec::new(), "None"),
                };
                // `RationalArithmetic` has no `IntegerDivide` variant: its own doc comment
                // specifies the lowering, `n` and `m` each as `n/1`/`m/1` via
                // `Rational::from_integer`, then `Divide` in quire-exact. The deleted
                // `IntegerDivide` variant charged the
                // same four rational-arithmetic points via the same `fn rational()`, so
                // bit-for-bit identical operands charge identically here. What actually
                // moved is upstream's own redesign of `fn rational()` itself (`.size` to
                // `.exact_size`, normalize-on-unreduced instead of normalize-on-reduced),
                // which affects every `RationalArithmetic` node uniformly, not something
                // specific to this lowering; `from_integer` itself is an uncharged
                // construction, so the lift below is free.
                let (parameters, lift_prelude, operation) = match operator {
                    RationalOperator::Negate => {
                        (unary("&rt::Rational"), Vec::new(), "Negate(operand)")
                    }
                    RationalOperator::Add => {
                        (binary("&rt::Rational"), Vec::new(), "Add(left, right)")
                    }
                    RationalOperator::Subtract => {
                        (binary("&rt::Rational"), Vec::new(), "Subtract(left, right)")
                    }
                    RationalOperator::Multiply => {
                        (binary("&rt::Rational"), Vec::new(), "Multiply(left, right)")
                    }
                    RationalOperator::Divide => {
                        (binary("&rt::Rational"), Vec::new(), "Divide(left, right)")
                    }
                    RationalOperator::IntegerDivide => (
                        binary("&rt::Integer"),
                        vec![
                            "let left = rt::Rational::from_integer(left.clone());".to_owned(),
                            "let right = rt::Rational::from_integer(right.clone());".to_owned(),
                        ],
                        "Divide(&left, &right)",
                    ),
                };
                let mut prelude = domain_prelude;
                prelude.extend(lift_prelude);
                Body {
                    parameters,
                    output: OracleOutput::Value("rt::Rational"),
                    prelude,
                    call: format!(
                        "Ok(rt::evaluate_rational_arithmetic(rt::RationalArithmetic::{operation}, {domain_argument}, meter))"
                    ),
                }
            }
            ExactScalarOperation::Ordering { operator, operands } => {
                let operator = match operator {
                    OrderingOperator::Less => "Less",
                    OrderingOperator::LessOrEqual => "LessOrEqual",
                    OrderingOperator::Greater => "Greater",
                    OrderingOperator::GreaterOrEqual => "GreaterOrEqual",
                };
                let (ty, kind) = match operands {
                    OrderingOperandKind::Integer => ("&rt::Integer", "Integers"),
                    OrderingOperandKind::Rational => ("&rt::Rational", "Rationals"),
                    OrderingOperandKind::Decimal => ("&rt::Decimal", "Decimals"),
                };
                Body {
                    parameters: binary(ty),
                    output: OracleOutput::Value("bool"),
                    prelude: Vec::new(),
                    call: format!(
                        "Ok(rt::order_numbers(rt::OrderingOperator::{operator}, rt::OrderedOperands::{kind}(left, right), meter))"
                    ),
                }
            }
            ExactScalarOperation::DecimalArithmetic { operator, target } => {
                let target = self.decimal_type(target)?;
                let (parameters, operation) = match operator {
                    DecimalOperator::Negate => (unary("&rt::Decimal"), "Negate(operand)"),
                    DecimalOperator::Round => (unary("&rt::Decimal"), "Round(operand)"),
                    DecimalOperator::Add => (binary("&rt::Decimal"), "Add(left, right)"),
                    DecimalOperator::Subtract => (binary("&rt::Decimal"), "Subtract(left, right)"),
                    DecimalOperator::Multiply => (binary("&rt::Decimal"), "Multiply(left, right)"),
                    DecimalOperator::Divide => (binary("&rt::Decimal"), "Divide(left, right)"),
                };
                Body {
                    parameters,
                    output: OracleOutput::Value("rt::DecimalResult"),
                    prelude: vec![format!("let target = {target};")],
                    call: format!(
                        "Ok(rt::evaluate_decimal(rt::DecimalOperation::{operation}, &target, meter))"
                    ),
                }
            }
            ExactScalarOperation::IeeeArithmetic {
                operator,
                width,
                rounding,
            } => {
                self.needs_width = true;
                let operator = match operator {
                    IeeeArithmeticOperator::Add => "Add",
                    IeeeArithmeticOperator::Subtract => "Subtract",
                    IeeeArithmeticOperator::Multiply => "Multiply",
                    IeeeArithmeticOperator::Divide => "Divide",
                };
                let width = width_path(*width)?;
                Body {
                    parameters: binary("rt::IeeeValue"),
                    output: OracleOutput::Value("rt::IeeeResult"),
                    prelude: vec![
                        format!("let left = expect_width(left, {width})?;"),
                        format!("let right = expect_width(right, {width})?;"),
                    ],
                    call: format!(
                        "rt::evaluate_ieee(rt::IeeeOperation::{operator}(left, right), {}, meter).map_err(OracleStop::IllTyped)",
                        rounding_path(*rounding)?
                    ),
                }
            }
            ExactScalarOperation::IeeeComparison { comparison, width } => {
                self.needs_width = true;
                let comparison = match comparison {
                    IeeeComparison::NumericEqual => "NumericEqual",
                    IeeeComparison::TotalOrder => "TotalOrder",
                    IeeeComparison::BitIdentical => "BitIdentical",
                };
                let width = width_path(*width)?;
                Body {
                    parameters: binary("rt::IeeeValue"),
                    output: OracleOutput::Value("bool"),
                    prelude: vec![
                        format!("let left = expect_width(left, {width})?;"),
                        format!("let right = expect_width(right, {width})?;"),
                    ],
                    call: format!(
                        "rt::compare_ieee(rt::IeeeComparison::{comparison}, left, right, meter).map_err(OracleStop::IllTyped)"
                    ),
                }
            }
            ExactScalarOperation::IeeeWidthConversion {
                source,
                target,
                rounding,
            } => {
                self.needs_width = true;
                Body {
                    parameters: unary("rt::IeeeValue"),
                    output: OracleOutput::Value("rt::IeeeResult"),
                    prelude: vec![format!(
                        "let operand = expect_width(operand, {})?;",
                        width_path(*source)?
                    )],
                    call: format!(
                        "Ok(rt::convert_ieee_width(operand, {}, {}, meter))",
                        width_path(*target)?,
                        rounding_path(*rounding)?
                    ),
                }
            }
            ExactScalarOperation::TextAdmission { text_type } => Body {
                parameters: unary("&rt::TextPayload"),
                output: OracleOutput::Value("rt::Text"),
                prelude: vec![format!(
                    "let text_type = rt::TextType::new({}, {}, {}).map_err(|_| OracleStop::InvalidConstant)?;",
                    text_type.min(),
                    text_type.max(),
                    profile_path(text_type.profile())?
                )],
                call: "Ok(rt::admit_text(operand, &text_type, meter))".to_owned(),
            },
            ExactScalarOperation::TextComparison { operator } => {
                comparison_body("&rt::Text", "rt::compare_text", *operator)?
            }
            ExactScalarOperation::EnumComparison { operator } => {
                comparison_body("&enum_owner::EnumValue", "enum_owner::compare_enum", *operator)?
            }
            ExactScalarOperation::QuantityConversion => Body {
                parameters: vec![("operand", "quantity_owner::UnitQuantity<'_>"), ("unit", "&quantity_owner::IdentifiedUnit")],
                output: OracleOutput::Value("rt::Quantity"),
                prelude: Vec::new(),
                call: "match quantity_owner::convert_quantity(operand, unit.unit(), &quantity_owner::QuantityTarget::Exact, meter).map_err(OracleStop::IllTyped)? {
        rt::Outcome::Completed(conversion) => match conversion.value() {
            quantity_owner::ConvertedValue::Exact(value) => Ok(rt::Outcome::Completed(rt::Quantity::new(value.clone(), unit.id()))),
            _ => Err(OracleStop::InvalidConstant),
        },
        rt::Outcome::Undefined(reason) => Ok(rt::Outcome::Undefined(reason)),
        rt::Outcome::Refused(reason) => Ok(rt::Outcome::Refused(reason)),
        rt::Outcome::Incomplete(reason) => Ok(rt::Outcome::Incomplete(reason)),
    }".to_owned(),
            },
            ExactScalarOperation::QuantityComparison { operator } => {
                comparison_body("quantity_owner::UnitQuantity<'_>", "quantity_owner::compare_quantity", *operator)?
            }
            ExactScalarOperation::QuantityArithmetic { operator } => {
                let (parameters, operation) = match operator {
                    QuantityOperator::Add => (binary("quantity_owner::UnitQuantity<'_>"), "Add(left, right)"),
                    QuantityOperator::Subtract => {
                        (binary("quantity_owner::UnitQuantity<'_>"), "Subtract(left, right)")
                    }
                    QuantityOperator::Multiply => {
                        (binary("quantity_owner::UnitQuantity<'_>"), "Multiply(left, right)")
                    }
                    QuantityOperator::Divide => (binary("quantity_owner::UnitQuantity<'_>"), "Divide(left, right)"),
                    QuantityOperator::Power => (
                        vec![("left", "quantity_owner::UnitQuantity<'_>"), ("right", "&rt::Integer")],
                        "Power(left, right)",
                    ),
                };
                Body {
                    parameters,
                    output: OracleOutput::QuantityWithUnit,
                    prelude: Vec::new(),
                    call: format!(
                        "quantity_owner::evaluate_quantity_unit(quantity_owner::QuantityOperation::{operation}, meter).map_err(OracleStop::Quantity)"
                    ),
                }
            }
        })
    }
}

enum OracleOutput {
    Value(&'static str),
    QuantityWithUnit,
}

impl OracleOutput {
    fn render(&self) -> String {
        match self {
            Self::Value(value) => format!("rt::Outcome<{value}>"),
            Self::QuantityWithUnit => {
                "(rt::Outcome<rt::Quantity>, quantity_owner::IdentifiedUnit)".to_owned()
            }
        }
    }
}

struct Body {
    parameters: Vec<(&'static str, &'static str)>,
    output: OracleOutput,
    prelude: Vec<String>,
    call: String,
}

fn unary(ty: &'static str) -> Vec<(&'static str, &'static str)> {
    vec![("operand", ty)]
}

fn binary(ty: &'static str) -> Vec<(&'static str, &'static str)> {
    vec![("left", ty), ("right", ty)]
}

fn comparison_body(
    ty: &'static str,
    function: &str,
    operator: ComparisonOperator,
) -> Result<Body, OracleGenerationError> {
    let operator = match operator {
        ComparisonOperator::Equal => "Equal",
        ComparisonOperator::NotEqual => "NotEqual",
        ComparisonOperator::Less => "Less",
        ComparisonOperator::LessOrEqual => "LessOrEqual",
        ComparisonOperator::Greater => "Greater",
        ComparisonOperator::GreaterOrEqual => "GreaterOrEqual",
    };
    Ok(Body {
        parameters: binary(ty),
        output: OracleOutput::Value("bool"),
        prelude: Vec::new(),
        call: format!(
            "{function}(rt::ComparisonOperator::{operator}, left, right, meter).map_err(OracleStop::IllTyped)"
        ),
    })
}

fn rounding_path(rounding: RoundingMode) -> Result<&'static str, OracleGenerationError> {
    match rounding {
        RoundingMode::Exact => Ok("rt::RoundingMode::Exact"),
        RoundingMode::TowardZero => Ok("rt::RoundingMode::TowardZero"),
        RoundingMode::TowardPositive => Ok("rt::RoundingMode::TowardPositive"),
        RoundingMode::TowardNegative => Ok("rt::RoundingMode::TowardNegative"),
        RoundingMode::NearestEven => Ok("rt::RoundingMode::NearestEven"),
        RoundingMode::NearestAway => Ok("rt::RoundingMode::NearestAway"),
    }
}

fn width_path(width: IeeeWidth) -> Result<&'static str, OracleGenerationError> {
    match width {
        IeeeWidth::Binary32 => Ok("rt::IeeeWidth::Binary32"),
        IeeeWidth::Binary64 => Ok("rt::IeeeWidth::Binary64"),
    }
}

fn profile_path(profile: TextProfile) -> Result<&'static str, OracleGenerationError> {
    match profile {
        TextProfile::UnicodeScalars => Ok("rt::TextProfile::UnicodeScalars"),
        TextProfile::Nfc => Ok("rt::TextProfile::Nfc"),
        TextProfile::Nfd => Ok("rt::TextProfile::Nfd"),
        TextProfile::Nfkc => Ok("rt::TextProfile::Nfkc"),
        TextProfile::Nfkd => Ok("rt::TextProfile::Nfkd"),
        TextProfile::BinaryUtf8 => Ok("rt::TextProfile::BinaryUtf8"),
    }
}

// ---------------------------------------------------------------------------
// Deriving the descriptor from the node
// ---------------------------------------------------------------------------

/// The operation identity of a quantity conversion.
pub(crate) const QUANTITY_CONVERT_IDENTITY: &str = "quire.op.quantity.convert";

fn conversion_target<Body: GraphBody, NodeBody>(
    graph: &Graph<'_, Body>,
    node: &CheckedSemanticNodeV2<NodeBody>,
) -> Option<CheckedNodeId> {
    let payload = graph.operation(&node.node_id)?;
    (payload["identity"].as_str() == Some(QUANTITY_CONVERT_IDENTITY))
        .then(|| serde_json::from_value(payload["member"]["declaration"].clone()).ok())
        .flatten()
}

fn quantity_conversion_target_is_unit<Body: GraphBody, NodeBody>(
    graph: &Graph<'_, Body>,
    node: &CheckedSemanticNodeV2<NodeBody>,
) -> bool {
    conversion_target(graph, node)
        .and_then(|id| graph.node(&id))
        .is_some_and(|target| {
            target.node_tag.as_ref() == "scalar_type" && target.semantic_form.as_ref() == "unit"
        })
}

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

/// An operation identity with the parameters that are not read from the node's bounds, laws,
/// mode or operand forms already resolved. It is the inverse of [`catalogued_operation`]:
/// `family_of_identity` is the one match over identity strings, and a round-trip test walks
/// every descriptor through both.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Family {
    Boolean(BooleanOperator),
    Equality(EqualityOperator, EqualityOperandKind),
    IntegerArithmetic(IntegerOperator),
    IntegerDivision,
    IntegerModulo,
    RationalArithmetic(RationalOperator),
    /// `rational.div`: `IntegerDivide` or `Divide` by the operand forms.
    RationalDivision,
    Ordering(OrderingOperator, OrderingOperandKind),
    DecimalArithmetic(DecimalOperator),
    IeeeArithmetic(IeeeArithmeticOperator, IeeeWidth),
    /// `ieee.numeric_equal`, `total_order` and `bit_identical`: the width comes from the operands.
    IeeeComparison(IeeeComparison),
    IeeeToFloat32,
    IeeeToFloat64,
    /// `numeric.convert`: text admission when the result is text.
    NumericConvert,
    TextComparison(ComparisonOperator),
    EnumComparison(ComparisonOperator),
    QuantityConversion,
    QuantityArithmetic(QuantityOperator),
    QuantityComparison(ComparisonOperator),
}

/// The inverse of [`catalogued_operation`] over identities. Identities outside the derivable set,
/// which is every identity this match does not name, are `None`.
fn family_of_identity(identity: &str) -> Option<Family> {
    use ComparisonOperator as C;
    use OrderingOperandKind as K;
    use OrderingOperator as O;
    if let Some(catalog_operator) = CatalogOperator::from_identity(identity) {
        return Some(match catalog_operator {
            CatalogOperator::Add
            | CatalogOperator::Subtract
            | CatalogOperator::Multiply
            | CatalogOperator::Negate => {
                Family::IntegerArithmetic(catalog_operator.entry().scalar_operator?)
            }
            CatalogOperator::Less
            | CatalogOperator::LessOrEqual
            | CatalogOperator::Greater
            | CatalogOperator::GreaterOrEqual => Family::Ordering(
                catalog_operator.entry().ordering_operator?,
                OrderingOperandKind::Integer,
            ),
            CatalogOperator::Equal => {
                Family::Equality(EqualityOperator::Equal, EqualityOperandKind::Integer)
            }
            CatalogOperator::NotEqual => {
                Family::Equality(EqualityOperator::NotEqual, EqualityOperandKind::Integer)
            }
        });
    }
    Some(match identity {
        "quire.op.boolean.and" => Family::Boolean(BooleanOperator::And),
        "quire.op.boolean.or" => Family::Boolean(BooleanOperator::Or),
        "quire.op.boolean.not" => Family::Boolean(BooleanOperator::Not),
        "quire.op.boolean.implies" => Family::Boolean(BooleanOperator::Implies),
        "quire.op.boolean.eq" => {
            Family::Equality(EqualityOperator::Equal, EqualityOperandKind::Boolean)
        }
        "quire.op.boolean.ne" => {
            Family::Equality(EqualityOperator::NotEqual, EqualityOperandKind::Boolean)
        }
        "quire.op.integer.div" => Family::IntegerDivision,
        "quire.op.integer.mod" => Family::IntegerModulo,
        "quire.op.rational.add" => Family::RationalArithmetic(RationalOperator::Add),
        "quire.op.rational.sub" => Family::RationalArithmetic(RationalOperator::Subtract),
        "quire.op.rational.mul" => Family::RationalArithmetic(RationalOperator::Multiply),
        "quire.op.rational.negate" => Family::RationalArithmetic(RationalOperator::Negate),
        "quire.op.rational.div" => Family::RationalDivision,
        "quire.op.rational.lt" => Family::Ordering(O::Less, K::Rational),
        "quire.op.rational.le" => Family::Ordering(O::LessOrEqual, K::Rational),
        "quire.op.rational.gt" => Family::Ordering(O::Greater, K::Rational),
        "quire.op.rational.ge" => Family::Ordering(O::GreaterOrEqual, K::Rational),
        "quire.op.decimal.lt" => Family::Ordering(O::Less, K::Decimal),
        "quire.op.decimal.le" => Family::Ordering(O::LessOrEqual, K::Decimal),
        "quire.op.decimal.gt" => Family::Ordering(O::Greater, K::Decimal),
        "quire.op.decimal.ge" => Family::Ordering(O::GreaterOrEqual, K::Decimal),
        "quire.op.decimal.add" => Family::DecimalArithmetic(DecimalOperator::Add),
        "quire.op.decimal.sub" => Family::DecimalArithmetic(DecimalOperator::Subtract),
        "quire.op.decimal.mul" => Family::DecimalArithmetic(DecimalOperator::Multiply),
        "quire.op.decimal.div" => Family::DecimalArithmetic(DecimalOperator::Divide),
        "quire.op.decimal.negate" => Family::DecimalArithmetic(DecimalOperator::Negate),
        "quire.op.numeric.convert_rounding" => Family::DecimalArithmetic(DecimalOperator::Round),
        "quire.op.ieee.float32.add" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Add, IeeeWidth::Binary32)
        }
        "quire.op.ieee.float32.sub" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Subtract, IeeeWidth::Binary32)
        }
        "quire.op.ieee.float32.mul" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Multiply, IeeeWidth::Binary32)
        }
        "quire.op.ieee.float32.div" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Divide, IeeeWidth::Binary32)
        }
        "quire.op.ieee.float64.add" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Add, IeeeWidth::Binary64)
        }
        "quire.op.ieee.float64.sub" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Subtract, IeeeWidth::Binary64)
        }
        "quire.op.ieee.float64.mul" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Multiply, IeeeWidth::Binary64)
        }
        "quire.op.ieee.float64.div" => {
            Family::IeeeArithmetic(IeeeArithmeticOperator::Divide, IeeeWidth::Binary64)
        }
        "quire.op.ieee.numeric_equal" => Family::IeeeComparison(IeeeComparison::NumericEqual),
        "quire.op.ieee.total_order" => Family::IeeeComparison(IeeeComparison::TotalOrder),
        "quire.op.ieee.bit_identical" => Family::IeeeComparison(IeeeComparison::BitIdentical),
        "quire.op.ieee.to_float32" => Family::IeeeToFloat32,
        "quire.op.ieee.to_float64" => Family::IeeeToFloat64,
        "quire.op.numeric.convert" => Family::NumericConvert,
        "quire.op.text.eq" => Family::TextComparison(C::Equal),
        "quire.op.text.ne" => Family::TextComparison(C::NotEqual),
        "quire.op.text.lt" => Family::TextComparison(C::Less),
        "quire.op.text.le" => Family::TextComparison(C::LessOrEqual),
        "quire.op.text.gt" => Family::TextComparison(C::Greater),
        "quire.op.text.ge" => Family::TextComparison(C::GreaterOrEqual),
        "quire.op.enum.eq" => Family::EnumComparison(C::Equal),
        "quire.op.enum.ne" => Family::EnumComparison(C::NotEqual),
        "quire.op.enum.lt" => Family::EnumComparison(C::Less),
        "quire.op.enum.le" => Family::EnumComparison(C::LessOrEqual),
        "quire.op.enum.gt" => Family::EnumComparison(C::Greater),
        "quire.op.enum.ge" => Family::EnumComparison(C::GreaterOrEqual),
        QUANTITY_CONVERT_IDENTITY => Family::QuantityConversion,
        "quire.op.quantity.add" => Family::QuantityArithmetic(QuantityOperator::Add),
        "quire.op.quantity.sub" => Family::QuantityArithmetic(QuantityOperator::Subtract),
        "quire.op.quantity.mul" => Family::QuantityArithmetic(QuantityOperator::Multiply),
        "quire.op.quantity.div" => Family::QuantityArithmetic(QuantityOperator::Divide),
        "quire.op.quantity.pow" => Family::QuantityArithmetic(QuantityOperator::Power),
        "quire.op.quantity.eq" => Family::QuantityComparison(C::Equal),
        "quire.op.quantity.ne" => Family::QuantityComparison(C::NotEqual),
        "quire.op.quantity.lt" => Family::QuantityComparison(C::Less),
        "quire.op.quantity.le" => Family::QuantityComparison(C::LessOrEqual),
        "quire.op.quantity.gt" => Family::QuantityComparison(C::Greater),
        "quire.op.quantity.ge" => Family::QuantityComparison(C::GreaterOrEqual),
        _ => return None,
    })
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
    let lowering = package.lower(node_ids, &scalar_profile());
    derive_exact_scalar_items_from_records(package, node_ids, &lowering.records)
}

/// Classify each item under its original one-key lowering semantics while
/// sharing Contract IR's graph and closure context across the request.
pub(crate) fn derive_exact_scalar_items_independent(
    package: &CheckedPackageV2,
    node_ids: &[CheckedNodeId],
) -> Vec<Result<ExactScalarItem, ExactScalarRefusal>> {
    let records = package.lower_independent_records(node_ids, &scalar_profile());
    derive_exact_scalar_items_from_records(package, node_ids, &records)
}

fn derive_exact_scalar_items_from_records(
    package: &CheckedPackageV2,
    node_ids: &[CheckedNodeId],
    records: &[CompleteLoweringRecordV2],
) -> Vec<Result<ExactScalarItem, ExactScalarRefusal>> {
    let graph = Graph::from_checked_with_node_visit(package, || {
        #[cfg(test)]
        SCALAR_DERIVE_GRAPH_VISITS.with(|count| count.set(count.get() + 1));
    });
    node_ids
        .iter()
        .zip(records)
        .map(|(node_id, record)| {
            derive_operation(&graph, node_id, record).map(|operation| ExactScalarItem {
                node_id: node_id.clone(),
                operation,
            })
        })
        .collect()
}

#[cfg(test)]
thread_local! {
    static SCALAR_DERIVE_GRAPH_VISITS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn take_scalar_derive_graph_visits() -> usize {
    SCALAR_DERIVE_GRAPH_VISITS.with(|count| count.replace(0))
}

/// What the derivation reads about one node.
struct Derivation<
    'a,
    'g,
    'n,
    Body: GraphBody = Value,
    NodeBody = CheckedTerm,
    BoundBody = CheckedTerm,
> {
    graph: &'a Graph<'g, Body>,
    node: &'g CheckedSemanticNodeV2<NodeBody>,
    bounds: BoundView<'n, BoundBody>,
    identity: &'a str,
}

impl<Body: GraphBody, NodeBody, BoundBody> Derivation<'_, '_, '_, Body, NodeBody, BoundBody> {
    fn right_may_stop(&self) -> bool {
        let Some(right) = self
            .graph
            .arguments(&self.node.node_id)
            .and_then(|args| args.get(1))
        else {
            return false;
        };
        match right {
            CheckedTerm::Application { .. } => true,
            CheckedTerm::Reference { target } => self.graph.node(target).is_some_and(|node| {
                &*node.node_tag == CheckedNodeTag::Expression.as_wire()
                    && matches!(
                        self.graph.term(&node.node_id),
                        Some(CheckedTerm::Application { .. })
                    )
            }),
            _ => false,
        }
    }

    fn operand_forms_not_derivable(&self) -> ExactScalarRefusal {
        ClaimDerivationRefusal::OperandFormsNotDerivable {
            operation_identity: self.identity.to_owned(),
        }
        .refusal()
    }

    fn mode_not_derivable(&self) -> ExactScalarRefusal {
        ClaimDerivationRefusal::ModeNotDerivable {
            operation_identity: self.identity.to_owned(),
        }
        .refusal()
    }

    /// The node's result bound of `form`.
    fn read<T>(
        &self,
        form: BoundForm,
        read: fn(&[CheckedTerm]) -> Option<T>,
    ) -> Result<T, ExactScalarRefusal> {
        let bounds = Bounds {
            graph: self.graph,
            node: self.bounds.clone(),
        };
        bounds.read(form, read).map(|(_, value)| value)
    }

    /// The scalar forms of the application's arguments.
    fn operand_forms(&self) -> Vec<Option<String>> {
        self.graph
            .arguments(&self.node.node_id)
            .into_iter()
            .flatten()
            .map(|argument| operand_form(self.graph, argument))
            .collect()
    }

    /// The scalar form of the node's result type.
    fn result_form(&self) -> Option<String> {
        scalar_type_form(self.graph, &self.node.semantic_type)
    }

    /// The `operation.mode` value when its kind is `rounding`.
    fn mode_value(&self) -> Option<&str> {
        let mode = &self.graph.operation(&self.node.node_id)?["mode"];
        (mode["kind"].as_str() == Some("rounding"))
            .then(|| mode["value"].as_str())
            .flatten()
    }

    /// Requires `operation.mode` to carry `rounding`.
    fn require_mode(&self, rounding: RoundingMode) -> Result<(), ExactScalarRefusal> {
        if self.mode_value() == Some(rounding.as_str()) {
            Ok(())
        } else {
            Err(self.mode_not_derivable())
        }
    }

    /// The one division profile `operation.laws` names.
    fn division_profile(&self) -> Result<DivisionProfile, ExactScalarRefusal> {
        let named = DivisionProfile::ALL
            .into_iter()
            .filter(|profile| {
                self.graph
                    .operation(&self.node.node_id)
                    .and_then(|operation| operation.get("laws"))
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .any(|law| {
                        law["definition"]["identity"].as_str()
                            == Some(profile.definition_identity())
                    })
            })
            .collect::<Vec<_>>();
        match named.as_slice() {
            [profile] => Ok(*profile),
            _ => Err(ClaimDerivationRefusal::LawNotDerivable {
                operation_identity: self.identity.to_owned(),
            }
            .refusal()),
        }
    }

    fn integer_domain(&self) -> Result<IntegerDomain, ExactScalarRefusal> {
        let bounds = Bounds {
            graph: self.graph,
            node: self.bounds.clone(),
        };
        // A reached bounded operand beside a plain reference is refused for
        // operand ownership before the selected own result can lack a bound.
        if self.bounds.selected.is_some() {
            bounds.integer_operand_bounds()?;
        }
        let domain = self.read(BoundForm::IntegerRange, read_integer_range)?;
        // Derivation refuses what generation refuses, so the two never disagree on a node.
        bounds.integer_operand_bounds()?;
        Ok(IntegerDomain::Bounded(domain))
    }

    fn decimal_target(&self) -> Result<DecimalType, ExactScalarRefusal> {
        self.read(BoundForm::DecimalRange, read_decimal_range)
    }

    fn float_rounding(&self) -> Result<RoundingMode, ExactScalarRefusal> {
        self.read(BoundForm::FloatRounding, read_float_rounding)
    }

    /// Both operands share one scalar form, of the `candidates`.
    fn shared_operand_form<T: Copy>(
        &self,
        candidates: &[(&str, T)],
    ) -> Result<T, ExactScalarRefusal> {
        let forms = self.operand_forms();
        let [Some(first), Some(second)] = forms.as_slice() else {
            return Err(self.operand_forms_not_derivable());
        };
        candidates
            .iter()
            .find(|(form, _)| first == form && second == form)
            .map(|(_, value)| *value)
            .ok_or_else(|| self.operand_forms_not_derivable())
    }
}

fn derive_operation<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    node_id: &CheckedNodeId,
    record: &CompleteLoweringRecordV2,
) -> Result<ExactScalarOperation, ExactScalarRefusal> {
    // The graph and the lowering come first, so a node absent from the graph, or one that does not
    // lower, keeps the refusal `generate_exact_scalar_oracles` gives it and FR-015 classifies.
    let node = graph
        .node(node_id)
        .ok_or(ExactScalarRefusal::InvalidInput)?;
    let lowered = lowered(record)?;
    derive_operation_from_view(graph, node, BoundView::complete(lowered))
}

fn derive_operation_from_view<'g, Body: GraphBody, NodeBody, BoundBody>(
    graph: &Graph<'g, Body>,
    node: &'g CheckedSemanticNodeV2<NodeBody>,
    bounds: BoundView<'_, BoundBody>,
) -> Result<ExactScalarOperation, ExactScalarRefusal> {
    use ExactScalarOperation as Op;
    if &*node.node_tag != CheckedNodeTag::Expression.as_wire()
        || !matches!(
            graph.term(&node.node_id),
            Some(CheckedTerm::Application { .. })
        )
    {
        return Err(ClaimDerivationRefusal::NotApplication {
            node_tag: node.node_tag.to_string(),
        }
        .refusal());
    }
    let identity = graph
        .operation(&node.node_id)
        .and_then(|operation| operation.get("identity"))
        .and_then(Value::as_str)
        .ok_or_else(|| ClaimDerivationRefusal::MissingOperationIdentity.refusal())?;
    let derivation = Derivation {
        graph,
        node,
        bounds,
        identity,
    };
    let family = family_of_identity(identity).ok_or_else(|| {
        ClaimDerivationRefusal::OperationNotDerivable {
            operation_identity: identity.to_owned(),
        }
        .refusal()
    })?;
    Ok(match family {
        Family::Boolean(operator) => Op::Boolean {
            operator,
            right_may_stop: matches!(
                operator,
                BooleanOperator::And | BooleanOperator::Or | BooleanOperator::Implies
            ) && derivation.right_may_stop(),
        },
        Family::Equality(operator, operands) => Op::Equality { operator, operands },
        Family::IntegerArithmetic(operator) => Op::IntegerArithmetic {
            operator,
            domain: derivation.integer_domain()?,
        },
        Family::IntegerDivision => Op::IntegerDivision {
            profile: derivation.division_profile()?,
            domain: derivation.integer_domain()?,
        },
        Family::IntegerModulo => Op::IntegerModulo {
            domain: derivation.integer_domain()?,
        },
        Family::RationalArithmetic(operator) => Op::RationalArithmetic {
            operator,
            domain: Some(derivation.read(BoundForm::RationalRange, read_rational_range)?),
        },
        Family::RationalDivision => Op::RationalArithmetic {
            operator: derivation.shared_operand_form(&[
                ("integer", RationalOperator::IntegerDivide),
                ("rational", RationalOperator::Divide),
            ])?,
            domain: Some(derivation.read(BoundForm::RationalRange, read_rational_range)?),
        },
        Family::Ordering(operator, operands) => Op::Ordering { operator, operands },
        Family::DecimalArithmetic(operator) => {
            if operator == DecimalOperator::Round
                && derivation.result_form().as_deref() != Some(ScalarForm::Decimal.wire())
            {
                return Err(derivation.operand_forms_not_derivable());
            }
            let target = derivation.decimal_target()?;
            if operator != DecimalOperator::Negate {
                derivation.require_mode(target.rounding())?;
            }
            Op::DecimalArithmetic { operator, target }
        }
        Family::IeeeArithmetic(operator, width) => {
            let rounding = derivation.float_rounding()?;
            derivation.require_mode(rounding)?;
            Op::IeeeArithmetic {
                operator,
                width,
                rounding,
            }
        }
        Family::IeeeComparison(comparison) => Op::IeeeComparison {
            comparison,
            width: derivation.shared_operand_form(&[
                ("float32", IeeeWidth::Binary32),
                ("float64", IeeeWidth::Binary64),
            ])?,
        },
        Family::IeeeToFloat32 => {
            let rounding = derivation.float_rounding()?;
            derivation.require_mode(rounding)?;
            Op::IeeeWidthConversion {
                source: IeeeWidth::Binary64,
                target: IeeeWidth::Binary32,
                rounding,
            }
        }
        Family::IeeeToFloat64 => Op::IeeeWidthConversion {
            source: IeeeWidth::Binary32,
            target: IeeeWidth::Binary64,
            rounding: derivation.float_rounding()?,
        },
        Family::NumericConvert => {
            if derivation.result_form().as_deref() != Some(ScalarForm::Text.wire()) {
                return Err(derivation.operand_forms_not_derivable());
            }
            Op::TextAdmission {
                text_type: derivation.read(BoundForm::TextBounds, read_text_bounds)?,
            }
        }
        Family::TextComparison(operator) => Op::TextComparison { operator },
        Family::EnumComparison(operator) => Op::EnumComparison { operator },
        Family::QuantityConversion => {
            derivation.require_mode(RoundingMode::Exact)?;
            if !quantity_conversion_target_is_unit(graph, node) {
                return Err(derivation.operand_forms_not_derivable());
            }
            Op::QuantityConversion
        }
        Family::QuantityArithmetic(operator) => Op::QuantityArithmetic { operator },
        Family::QuantityComparison(operator) => Op::QuantityComparison { operator },
    })
}

#[cfg(test)]
mod tests {
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
                "operation": { "mode": { "kind": "rounding", "value": "truncate" } },
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

//! Exact complete-V1 scalar oracle generation from CheckedPackage V2 (FR-014).
//!
//! The V2 transport names only an operator class (`binary`, `unary`,
//! `convert`) for a scalar expression, not the operation law. Each request
//! item therefore pairs a checked node with a typed [`ExactScalarOperation`]
//! built from Contract Runtime `exact` values, and this module cross-checks
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
//! still classify a body from its `term`/`operator`/`arguments` and the
//! request item's own descriptor, not from `operation.identity` -- reading
//! the confirmed identity changes what a claim reports, not how its code is
//! generated.
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
//! Generated functions call the pinned runtime operator with a caller-supplied
//! `Meter`: every charge comes from the runtime, and no amount is copied into
//! the generated source. An item that fails any check receives a typed
//! [`ExactScalarRefusal`] and contributes no code; its siblings are unaffected.

use crate::generation::{ClaimDisposition, ClaimMap, OracleGenerationError, UpstreamBlocker};
use crate::oracle::{Artifact, MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION};
use quire_contract_ir::{
    CheckedNodeId, CheckedNodeTag, CheckedPackageV2, CheckedSemanticId, CheckedSemanticNodeV2,
    CheckedSourceMapEntry, CompleteContractNodeV2, CompleteLoweringProfileV2,
    CompleteLoweringRecordV2,
};
use quire_contract_runtime::exact::{
    ComparisonOperator, DecimalType, DivisionProfile, IeeeComparison, IeeeWidth, Integer,
    IntegerDomain, IntegerInterval, OrderingOperator, QuantityTarget, RationalDomain, RoundingMode,
    TextProfile, TextType,
};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

/// Version of the emitted claim map.
pub const EXACT_SCALAR_CLAIM_MAP_VERSION: &str = "quire.codegen.exact-scalar-claim-map/v1";

/// Work budget for lowering one requested scalar node and its closure.
pub const SCALAR_LOWERING_WORK_LIMIT: u64 = 65_536;

/// Name of the generated crate.
pub const EXACT_SCALAR_CRATE_NAME: &str = "quire-exact-scalar-oracles";

/// The `CheckedNodeTag` families [`scalar_profile`]'s `CompleteLoweringProfileV2` admits.
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
/// [`scalar_profile`] itself -- which would make that cross-check circular.
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

/// Integer `+`, `-`, `*` and unary `-`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegerOperator {
    /// `a + b`.
    Add,
    /// `a - b`.
    Subtract,
    /// `a * b`.
    Multiply,
    /// `-a`.
    Negate,
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
    /// `evaluate_integer` in `domain`.
    IntegerArithmetic {
        /// Operator.
        operator: IntegerOperator,
        /// Consumer domain.
        domain: IntegerDomain,
    },
    /// `divide` under a selected law.
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
    /// `convert_quantity` into an admitted unit supplied at call time.
    QuantityConversion {
        /// Value representation of the result.
        target: QuantityTarget,
    },
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
    fn from_literal_kind(kind: &str) -> Option<Self> {
        Some(match kind {
            "boolean" => Self::Boolean,
            "integer" => Self::Integer,
            "rational" => Self::Rational,
            "decimal" => Self::Decimal,
            "float32_bits" => Self::Float32,
            "float64_bits" => Self::Float64,
            "text" => Self::Text,
            "enum" => Self::Enum,
            _ => return None,
        })
    }

    fn of_width(width: IeeeWidth) -> Self {
        match width {
            IeeeWidth::Binary32 => Self::Float32,
            IeeeWidth::Binary64 => Self::Float64,
            _ => unreachable!("IeeeWidth gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
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
    /// quire-contract-ir dfd8bd78's `validate_operations` against the closed
    /// `quire.checked-operation-catalog/v1` at package admission, before
    /// this generator ever saw the package. A consumer may treat the
    /// operation identity as checked.
    IrConfirmed,
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
    /// Lowering exceeded [`SCALAR_LOWERING_WORK_LIMIT`].
    LoweringWorkExhausted {
        /// The ceiling.
        limit: u64,
        /// Counter at the failed charge.
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
    /// The node's `operation.identity` member is absent, violating the IR
    /// admission invariant `check_item`'s callers otherwise rely on.
    MissingOperationIdentity {
        /// The affected node.
        node_id: CheckedNodeId,
    },
}

/// Traceability for one generated oracle.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GeneratedScalarClaim {
    /// Generated function name.
    pub symbol: String,
    /// Contract IR identity of the lowered node.
    pub ir_id: CheckedSemanticId,
    /// Expression form.
    pub semantic_form: String,
    /// Semantic type key.
    pub semantic_type: CheckedNodeId,
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
    /// `kani_obligations::render_scalar` from the live claim map this
    /// generation produced.
    #[serde(skip)]
    pub oracle_source: String,
}

/// One claim-map entry per distinct requested node.
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
    /// `Cargo.toml`, `src/lib.rs` and `claim-map.json`, in that order.
    pub artifacts: Vec<Artifact>,
    /// Typed claim map, identical to `claim-map.json`. Items ascend by node
    /// id: digest domain, then digest. `blocked` lists the upstream gaps every
    /// entry is subject to. A `Generated` disposition means one oracle
    /// function was emitted. The blockers this generator records are
    /// `QuireSpecLanguage120`, `QuireContractRuntime34` and
    /// `OperationIdentityNotConsumed`.
    pub claim_map: ClaimMap<ExactScalarClaim>,
}

/// Generate exact scalar oracles for `items` from an admitted package.
///
/// Fails as a whole only with `SourceTooLarge`, `ClaimMapSerialization`, or
/// `UnknownRuntimeVariant` (`check_parameters` matching an RT
/// `#[non_exhaustive]` enum against a variant its own exhaustive match does
/// not know); every per-item problem is a refusal in the claim map.
pub fn generate_exact_scalar_oracles(
    package: &CheckedPackageV2,
    items: &[ExactScalarItem],
) -> Result<ExactScalarOracles, OracleGenerationError> {
    let mut requests: BTreeMap<&CheckedNodeId, Vec<&ExactScalarOperation>> = BTreeMap::new();
    for item in items {
        requests
            .entry(&item.node_id)
            .or_default()
            .push(&item.operation);
    }
    let requested = requests.keys().map(|id| (*id).clone()).collect::<Vec<_>>();
    let lowering = package.lower(&requested, &scalar_profile());
    let graph = package
        .graph()
        .nodes
        .iter()
        .map(|node| (&node.node_id, node))
        .collect::<BTreeMap<_, _>>();

    let mut source = SourceBuilder::default();
    let mut claims = Vec::with_capacity(requests.len());
    for ((node_id, operations), record) in requests.into_iter().zip(&lowering.records) {
        let operation_claim = match operations.as_slice() {
            [operation] => match check_item(&graph, record, operation) {
                Ok((node, checked_bounds)) => {
                    let symbol = format!("oracle_{}", node.node.node_id.digest);
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
                    let confirmed_identity =
                        catalogued_operation_identity(node).map(|catalogued| {
                            operation_confirmed(node, operation).then_some(catalogued)
                        });
                    match confirmed_identity {
                        Err(refusal) => refused_claim(operation_identity(operation), refusal),
                        Ok(maybe_identity) => {
                            let (identity, provenance, oracle_source) = match maybe_identity {
                                Some(identity) => {
                                    source.oracle(&symbol, node_id, &identity, operation);
                                    let oracle_source = standalone_oracle_source(
                                        lowering.package.source_package_id(),
                                        &symbol,
                                        node_id,
                                        &identity,
                                        operation,
                                    );
                                    (identity, OperationProvenance::IrConfirmed, oracle_source)
                                }
                                None => {
                                    let identity = operation_identity(operation);
                                    source.oracle(&symbol, node_id, &identity, operation);
                                    (
                                        identity,
                                        OperationProvenance::CallerDeclared {
                                            blocked_on:
                                                UpstreamBlocker::OperationIdentityNotConsumed,
                                        },
                                        String::new(),
                                    )
                                }
                            };
                            (
                                OperationClaim {
                                    identity,
                                    provenance,
                                },
                                ClaimDisposition::Generated(Box::new(GeneratedScalarClaim {
                                    symbol,
                                    ir_id: node.ir_id.clone(),
                                    semantic_form: node.node.semantic_form.to_string(),
                                    semantic_type: node.semantic_type.clone(),
                                    source_map: node.source_map.clone(),
                                    claims: node.claims.clone(),
                                    bounds: node.bounds.clone(),
                                    checked_bounds,
                                    dependencies: node.dependencies.clone(),
                                    oracle_source,
                                })),
                            )
                        }
                    }
                }
                Err(ItemCheckError::Refusal(refusal)) => {
                    refused_claim(operation_identity(operation), refusal)
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
                let Some(identity) = copies.iter().map(|op| operation_identity(op)).min() else {
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

    let claim_map = ClaimMap {
        version: EXACT_SCALAR_CLAIM_MAP_VERSION,
        package_id: lowering.package.source_package_id().clone(),
        runtime_revision: RUNTIME_REVISION,
        // No blocker applies to every entry any more: a confirmed claim's
        // provenance is `IrConfirmed`, and an unconfirmed claim's
        // `CallerDeclared` provenance names
        // `UpstreamBlocker::OperationIdentityNotConsumed` per-item, whichever
        // of the cases on `OperationProvenance::CallerDeclared` applies.
        blocked: Vec::new(),
        items: claims,
    };
    let lib = source.finish(&claim_map.package_id);
    if lib.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(OracleGenerationError::SourceTooLarge { bytes: lib.len() });
    }
    let mut map_bytes = serde_json::to_vec_pretty(&claim_map)
        .map_err(|_| OracleGenerationError::ClaimMapSerialization)?;
    map_bytes.push(b'\n');
    let map_text =
        String::from_utf8(map_bytes).map_err(|_| OracleGenerationError::ClaimMapSerialization)?;
    Ok(ExactScalarOracles {
        artifacts: vec![
            artifact("Cargo.toml", manifest()),
            artifact("src/lib.rs", lib),
            artifact("claim-map.json", map_text),
        ],
        claim_map,
    })
}

fn scalar_profile() -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
        supported_tags: BTreeSet::from(SCALAR_LOWERING_SUPPORTED_TAGS),
        require_bounds: true,
        work_limit: SCALAR_LOWERING_WORK_LIMIT,
    }
}

type Graph<'a> = BTreeMap<&'a CheckedNodeId, &'a CheckedSemanticNodeV2>;

/// The lowered node and the bounds its descriptor was checked against.
type CheckedItem<'r> = (&'r CompleteContractNodeV2, Vec<CheckedNodeId>);

/// Either [`ExactScalarRefusal`] (a per-item business refusal, folded into
/// the claim map) or [`OracleGenerationError`] (an RT `#[non_exhaustive]`
/// enum yielding a variant `check_parameters`'s own exhaustive match does not
/// know -- see [`OracleGenerationError::UnknownRuntimeVariant`]'s own doc for
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

fn check_item<'r>(
    graph: &Graph<'_>,
    record: &'r CompleteLoweringRecordV2,
    operation: &ExactScalarOperation,
) -> Result<CheckedItem<'r>, ItemCheckError> {
    let node = lowered(record)?;
    if node.node_tag != CheckedNodeTag::Expression {
        return Err(ExactScalarRefusal::NotExpression {
            node_tag: node.node_tag.as_wire(),
        }
        .into());
    }
    let shape = Shape::of(operation);
    if &*node.node.semantic_form != shape.form {
        return Err(ExactScalarRefusal::FormMismatch {
            expected: shape.form,
            found: node.node.semantic_form.to_string(),
        }
        .into());
    }
    let arguments = application_arguments(&node.node.body, shape.body_operator)
        .filter(|arguments| arguments.len() == shape.operands.len())
        .ok_or(ExactScalarRefusal::BodyMismatch {
            expected_operator: shape.body_operator,
            expected_arguments: shape.operands.len(),
        })?;
    let result = type_form(graph, &node.semantic_type);
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
        CompleteLoweringRecordV2::Failed {
            limit, consumed, ..
        } => Err(ExactScalarRefusal::LoweringWorkExhausted {
            limit: *limit,
            consumed: *consumed,
        }),
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
        | CheckedNodeTag::ScalarType
        | CheckedNodeTag::CompositeType
        | CheckedNodeTag::BoundedDomain
        | CheckedNodeTag::Value
        | CheckedNodeTag::Expression
        | CheckedNodeTag::Claim
        | CheckedNodeTag::Correspondence => ExactScalarRefusal::Unsupported {
            unsupported_node_id: node_id.clone(),
            node_tag: tag.as_wire(),
        },
    }
}

fn application_arguments<'v>(body: &'v Value, operator: &str) -> Option<&'v Vec<Value>> {
    if body.get("term")?.as_str()? != "application" || body.get("operator")?.as_str()? != operator {
        return None;
    }
    body.get("arguments")?.as_array()
}

fn type_form(graph: &Graph<'_>, type_id: &CheckedNodeId) -> Option<String> {
    let node = graph.get(type_id)?;
    (&*node.node_tag == CheckedNodeTag::ScalarType.as_wire())
        .then(|| node.semantic_form.to_string())
}

/// An operand is a literal classified by its value kind, or a reference
/// classified by its target's scalar type. A literal carries no unit, so it is
/// never a quantity; any other term is refused as unsupported.
fn check_operand(
    graph: &Graph<'_>,
    position: usize,
    term: &Value,
    expected: ScalarForm,
) -> Result<(), ExactScalarRefusal> {
    let kind = term.get("term").and_then(Value::as_str).unwrap_or_default();
    let found = match kind {
        "literal" if expected == ScalarForm::Unit => {
            return Err(ExactScalarRefusal::UnitlessLiteralOperand { position });
        }
        "literal" => term
            .get("value_kind")
            .and_then(Value::as_str)
            .and_then(ScalarForm::from_literal_kind)
            .map(|form| form.wire().to_owned()),
        "reference" => reference_form(graph, term),
        other => {
            return Err(ExactScalarRefusal::OperandUnsupported {
                position,
                term: other.to_owned(),
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

fn reference_form(graph: &Graph<'_>, term: &Value) -> Option<String> {
    let target: CheckedNodeId = serde_json::from_value(term.get("target")?.clone()).ok()?;
    type_form(graph, &graph.get(&target)?.semantic_type)
}

// ---------------------------------------------------------------------------
// Parameters against IR bounds
// ---------------------------------------------------------------------------

/// Check every descriptor parameter the IR carries against the one reachable
/// bound of its form on the node's result type, returning the bounds checked.
fn check_parameters(
    graph: &Graph<'_>,
    node: &CompleteContractNodeV2,
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
                &_ => {
                    return Err(ItemCheckError::Generation(
                        OracleGenerationError::UnknownRuntimeVariant {
                            enum_name: "IntegerDomain",
                        },
                    ))
                }
            };
            bounds.equal(BoundForm::IntegerRange, read_integer_range, declared)?
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
        ExactScalarOperation::QuantityConversion { target } => match target {
            QuantityTarget::Exact => bounds.equal::<RationalDomain>(
                BoundForm::RationalRange,
                read_rational_range,
                None,
            )?,
            QuantityTarget::Decimal(decimal) => {
                bounds.equal(BoundForm::DecimalRange, read_decimal_range, Some(decimal))?
            }
            // The rounding of an integer conversion has no IR bound; it stays
            // caller-declared with the operation.
            QuantityTarget::Integer { domain, .. } => {
                bounds.equal(BoundForm::IntegerRange, read_integer_range, Some(domain))?
            }
            &_ => {
                return Err(ItemCheckError::Generation(
                    OracleGenerationError::UnknownRuntimeVariant {
                        enum_name: "QuantityTarget",
                    },
                ))
            }
        },
        ExactScalarOperation::Ordering { .. }
        | ExactScalarOperation::IeeeComparison { .. }
        | ExactScalarOperation::TextComparison { .. }
        | ExactScalarOperation::EnumComparison { .. }
        | ExactScalarOperation::QuantityArithmetic { .. }
        | ExactScalarOperation::QuantityComparison { .. } => return Ok(Vec::new()),
    };
    Ok(vec![checked])
}

struct Bounds<'g, 'n> {
    graph: &'g Graph<'g>,
    node: &'n CompleteContractNodeV2,
}

impl Bounds<'_, '_> {
    /// Read the single bound of `form` on the result type and require the
    /// declared parameter to equal it. An absent declaration (an unbounded
    /// descriptor) never equals an IR bound.
    fn equal<T: PartialEq>(
        &self,
        form: BoundForm,
        read: fn(&[Value]) -> Option<T>,
        declared: Option<&T>,
    ) -> Result<CheckedNodeId, ExactScalarRefusal> {
        let bounded_type = &self.node.semantic_type;
        let candidates = self
            .node
            .bounds
            .iter()
            .filter_map(|id| self.graph.get(id))
            .filter(|bound| {
                &bound.semantic_type == bounded_type && &*bound.semantic_form == form.wire()
            })
            .collect::<Vec<_>>();
        let bound = match candidates.as_slice() {
            [bound] => bound,
            [] => {
                return Err(ExactScalarRefusal::MissingBound {
                    bounded_type: bounded_type.clone(),
                    expected_form: form,
                })
            }
            [_, _, ..] => {
                return Err(ExactScalarRefusal::AmbiguousBound {
                    bounded_type: bounded_type.clone(),
                    expected_form: form,
                })
            }
        };
        let value = aggregate_members(&bound.body)
            .and_then(read)
            .ok_or_else(|| ExactScalarRefusal::UnreadableBound {
                bound: bound.node_id.clone(),
            })?;
        if declared == Some(&value) {
            Ok(bound.node_id.clone())
        } else {
            Err(ExactScalarRefusal::BoundMismatch {
                bound: bound.node_id.clone(),
                form,
            })
        }
    }
}

pub(crate) fn aggregate_members(body: &Value) -> Option<&[Value]> {
    if body.get("term")?.as_str()? != "aggregate" {
        return None;
    }
    body.get("members")?.as_array().map(Vec::as_slice)
}

fn literal<'v>(term: &'v Value, kind: &str) -> Option<&'v str> {
    if term.get("term")?.as_str()? != "literal" || term.get("value_kind")?.as_str()? != kind {
        return None;
    }
    term.get("value")?.as_str()
}

/// A canonical decimal integer literal.
pub(crate) fn literal_integer(term: &Value) -> Option<Integer> {
    let spelling = literal(term, "integer")?;
    let value: Integer = spelling.parse().ok()?;
    (value.to_string() == spelling).then_some(value)
}

/// A canonical decimal integer literal within `u64`.
pub(crate) fn literal_count(term: &Value) -> Option<u64> {
    let spelling = literal(term, "integer")?;
    let value: u64 = spelling.parse().ok()?;
    (value.to_string() == spelling).then_some(value)
}

fn literal_interval(lower: &Value, upper: &Value) -> Option<IntegerInterval> {
    IntegerInterval::new(literal_integer(lower)?, literal_integer(upper)?).ok()
}

fn read_integer_range(members: &[Value]) -> Option<IntegerInterval> {
    let [lower, upper] = members else {
        return None;
    };
    literal_interval(lower, upper)
}

fn read_rational_range(members: &[Value]) -> Option<RationalDomain> {
    let [numerator_lower, numerator_upper, denominator_lower, denominator_upper] = members else {
        return None;
    };
    RationalDomain::new(
        literal_interval(numerator_lower, numerator_upper)?,
        literal_interval(denominator_lower, denominator_upper)?,
    )
    .ok()
}

fn read_decimal_range(members: &[Value]) -> Option<DecimalType> {
    let [lower, upper, min_scale, max_scale, rounding] = members else {
        return None;
    };
    DecimalType::new(
        literal_integer(lower)?,
        literal_integer(upper)?,
        literal_count(min_scale)?,
        literal_count(max_scale)?,
        RoundingMode::from_code(literal(rounding, "text")?)?,
    )
    .ok()
}

fn read_float_rounding(members: &[Value]) -> Option<RoundingMode> {
    let [rounding] = members else {
        return None;
    };
    RoundingMode::from_code(literal(rounding, "text")?)
}

fn read_text_bounds(members: &[Value]) -> Option<TextType> {
    let [min, max, profile] = members else {
        return None;
    };
    TextType::new(
        literal_count(min)?,
        literal_count(max)?,
        TextProfile::from_code(literal(profile, "text")?)?,
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
    /// all `call`-operator entries, and quire-contract-ir dfd8bd78's
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

    fn of(operation: &ExactScalarOperation) -> Self {
        use ScalarForm as F;
        match operation {
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
                &_ => unreachable!("IeeeWidth gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
            },
            ExactScalarOperation::IeeeComparison { width, .. } => match width {
                IeeeWidth::Binary32 => Self::binary_call(&[F::Float32, F::Float32], F::Boolean),
                IeeeWidth::Binary64 => Self::binary_call(&[F::Float64, F::Float64], F::Boolean),
                &_ => unreachable!("IeeeWidth gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
            },
            ExactScalarOperation::IeeeWidthConversion { source, target, .. } => {
                let operand: &'static [ScalarForm; 1] = match source {
                    IeeeWidth::Binary32 => &[F::Float32],
                    IeeeWidth::Binary64 => &[F::Float64],
                    &_ => unreachable!("IeeeWidth gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
                };
                Self::conversion(operand, F::of_width(*target))
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
            ExactScalarOperation::QuantityComparison { .. } => {
                Self::binary(&[F::Unit, F::Unit], F::Boolean)
            }
            ExactScalarOperation::QuantityConversion { target } => {
                let result = match target {
                    QuantityTarget::Exact => F::Rational,
                    QuantityTarget::Decimal(_) => F::Decimal,
                    QuantityTarget::Integer { .. } => F::Integer,
                    &_ => unreachable!("QuantityTarget gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
                };
                Self::conversion(&[F::Unit], result)
            }
        }
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
/// "application"`), and quire-contract-ir dfd8bd78's `validate_operations`
/// requires `identity` on every such node's `operation` member before the
/// package that contains it is ever admitted -- so this generator should
/// never receive a node for which this member is absent. It is another
/// crate's invariant, not this one's, so a violation is reported rather than
/// panicked on.
fn catalogued_operation_identity(
    node: &CompleteContractNodeV2,
) -> Result<String, ExactScalarRefusal> {
    node.node.body["operation"]["identity"]
        .as_str()
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

fn catalogued_operation(operation: &ExactScalarOperation) -> Option<CataloguedOperation> {
    use ExactScalarOperation as Op;
    match operation {
        Op::IntegerArithmetic { operator, .. } => plain(match operator {
            IntegerOperator::Add => "quire.op.integer.add",
            IntegerOperator::Subtract => "quire.op.integer.sub",
            IntegerOperator::Multiply => "quire.op.integer.mul",
            IntegerOperator::Negate => "quire.op.integer.negate",
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
        Op::Ordering { operator, operands } => {
            let family = match operands {
                OrderingOperandKind::Integer => "integer",
                OrderingOperandKind::Rational => "rational",
                OrderingOperandKind::Decimal => "decimal",
            };
            plain(match (family, ordering_suffix(*operator)) {
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
            })
        }
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
                },
                (&_, _) => unreachable!("IeeeWidth gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
            };
            with_rounding(identity, *rounding)
        }
        Op::IeeeComparison { comparison, .. } => plain(match comparison {
            IeeeComparison::NumericEqual => "quire.op.ieee.numeric_equal",
            IeeeComparison::TotalOrder => "quire.op.ieee.total_order",
            IeeeComparison::BitIdentical => "quire.op.ieee.bit_identical",
            &_ => unreachable!("IeeeComparison gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
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
            (&_, _) => unreachable!("IeeeWidth gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
        },
        Op::TextAdmission { .. } => plain("quire.op.numeric.convert"),
        // `text.*`/`enum.*` comparisons carry a catalogued `text_profile` mode
        // (`"nfc"`), but the descriptor itself has no profile-bearing field to
        // compare it against, so only the identity is checked.
        Op::TextComparison { operator } => plain(text_family_identity("text", *operator)),
        Op::EnumComparison { operator } => plain(text_family_identity("enum", *operator)),
        Op::QuantityArithmetic { operator } => plain(match operator {
            QuantityOperator::Add => "quire.op.quantity.add",
            QuantityOperator::Subtract => "quire.op.quantity.sub",
            QuantityOperator::Multiply => "quire.op.quantity.mul",
            QuantityOperator::Divide => "quire.op.quantity.div",
            QuantityOperator::Power => "quire.op.quantity.pow",
        }),
        Op::QuantityComparison { operator } => plain(text_family_identity("quantity", *operator)),
        Op::QuantityConversion { target } => {
            let identity = "quire.op.quantity.convert";
            match target {
                // A conversion to `Rational[..]` is exact; the catalog's
                // `rounding` mode is not meaningful for it and this
                // descriptor carries no value for it either, so it is not
                // checked.
                QuantityTarget::Exact => plain(identity),
                QuantityTarget::Decimal(decimal) => with_rounding(identity, decimal.rounding()),
                QuantityTarget::Integer { rounding, .. } => with_rounding(identity, *rounding),
                &_ => unreachable!("QuantityTarget gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
            }
        }
    }
}

/// `lt`/`le`/`gt`/`ge` per [`OrderingOperator`] variant, shared by
/// [`catalogued_operation`]'s integer/rational/decimal `Ordering` arm.
fn ordering_suffix(operator: OrderingOperator) -> &'static str {
    match operator {
        OrderingOperator::Less => "lt",
        OrderingOperator::LessOrEqual => "le",
        OrderingOperator::Greater => "gt",
        OrderingOperator::GreaterOrEqual => "ge",
        _ => unreachable!("OrderingOperator gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
    }
}

/// `quire.op.<family>.<suffix>` for one of the catalog's six comparison
/// identities per family (`eq`/`ne`/`lt`/`le`/`gt`/`ge`).
fn text_family_identity(family: &'static str, operator: ComparisonOperator) -> &'static str {
    let suffix = match operator {
        ComparisonOperator::Equal => "eq",
        ComparisonOperator::NotEqual => "ne",
        ComparisonOperator::Less => "lt",
        ComparisonOperator::LessOrEqual => "le",
        ComparisonOperator::Greater => "gt",
        ComparisonOperator::GreaterOrEqual => "ge",
        _ => unreachable!("ComparisonOperator gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
    };
    match (family, suffix) {
        ("text", "eq") => "quire.op.text.eq",
        ("text", "ne") => "quire.op.text.ne",
        ("text", "lt") => "quire.op.text.lt",
        ("text", "le") => "quire.op.text.le",
        ("text", "gt") => "quire.op.text.gt",
        ("text", "ge") => "quire.op.text.ge",
        ("enum", "eq") => "quire.op.enum.eq",
        ("enum", "ne") => "quire.op.enum.ne",
        ("enum", "lt") => "quire.op.enum.lt",
        ("enum", "le") => "quire.op.enum.le",
        ("enum", "gt") => "quire.op.enum.gt",
        ("enum", "ge") => "quire.op.enum.ge",
        ("quantity", "eq") => "quire.op.quantity.eq",
        ("quantity", "ne") => "quire.op.quantity.ne",
        ("quantity", "lt") => "quire.op.quantity.lt",
        ("quantity", "le") => "quire.op.quantity.le",
        ("quantity", "gt") => "quire.op.quantity.gt",
        ("quantity", "ge") => "quire.op.quantity.ge",
        _ => unreachable!("every (family, suffix) pair is covered above"),
    }
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
fn operation_confirmed(node: &CompleteContractNodeV2, operation: &ExactScalarOperation) -> bool {
    let Some(catalogued) = catalogued_operation(operation) else {
        return false;
    };
    if node.node.body["operation"]["identity"].as_str() != Some(catalogued.identity) {
        return false;
    }
    if let Some((kind, value)) = catalogued.mode {
        let mode = &node.node.body["operation"]["mode"];
        if mode["kind"].as_str() != Some(kind) || mode["value"].as_str() != Some(value) {
            return false;
        }
    }
    if let ExactScalarOperation::IntegerDivision { profile, .. } = operation {
        let law_confirmed = node.node.body["operation"]["laws"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|law| {
                law["definition"]["identity"].as_str() == Some(profile.definition_identity())
            });
        if !law_confirmed {
            return false;
        }
    }
    true
}

/// One oracle's function, self-contained for direct embedding in a generated
/// Kani harness file (`kani_obligations::render_scalar`): unlike
/// [`SourceBuilder::finish`]'s crate-wide accumulation (one set of shared
/// helpers for every oracle in the crate), this always includes whatever
/// preamble helpers its own body needs, even when that duplicates them
/// across separate harness files that each embed a different oracle.
fn standalone_oracle_source(
    package_id: &CheckedSemanticId,
    symbol: &str,
    node_id: &CheckedNodeId,
    identity: &str,
    operation: &ExactScalarOperation,
) -> String {
    let mut builder = SourceBuilder::default();
    builder.oracle(symbol, node_id, identity, operation);
    builder.finish(package_id)
}

fn operation_identity(operation: &ExactScalarOperation) -> String {
    match operation {
        ExactScalarOperation::IntegerArithmetic { operator, domain } => {
            let name = match operator {
                IntegerOperator::Add => "add",
                IntegerOperator::Subtract => "subtract",
                IntegerOperator::Multiply => "multiply",
                IntegerOperator::Negate => "negate",
            };
            format!("integer.{name} domain={}", integer_domain_identity(domain))
        }
        ExactScalarOperation::IntegerDivision { profile, domain } => format!(
            "{} domain={}",
            profile.definition_identity(),
            integer_domain_identity(domain)
        ),
        ExactScalarOperation::IntegerModulo { domain } => {
            format!("integer.modulo domain={}", integer_domain_identity(domain))
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
                &_ => unreachable!("OrderingOperator gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
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
                &_ => unreachable!("IeeeComparison gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
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
            format!("text.compare.{}", comparison_name(*operator))
        }
        ExactScalarOperation::EnumComparison { operator } => {
            format!("enum.compare.{}", comparison_name(*operator))
        }
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
            format!("quantity.compare.{}", comparison_name(*operator))
        }
        ExactScalarOperation::QuantityConversion { target } => match target {
            QuantityTarget::Exact => "quantity.convert target=exact".to_owned(),
            QuantityTarget::Decimal(decimal) => format!(
                "quantity.convert target=decimal[{},{}] scale=[{},{}] rounding={}",
                decimal.lower(),
                decimal.upper(),
                decimal.min_scale(),
                decimal.max_scale(),
                decimal.rounding().as_str()
            ),
            QuantityTarget::Integer { domain, rounding } => format!(
                "quantity.convert target=integer{} rounding={}",
                interval_identity(domain),
                rounding.as_str()
            ),
            &_ => unreachable!("QuantityTarget gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
        },
    }
}

fn comparison_name(operator: ComparisonOperator) -> &'static str {
    match operator {
        ComparisonOperator::Equal => "equal",
        ComparisonOperator::NotEqual => "not_equal",
        ComparisonOperator::Less => "less",
        ComparisonOperator::LessOrEqual => "less_or_equal",
        ComparisonOperator::Greater => "greater",
        ComparisonOperator::GreaterOrEqual => "greater_or_equal",
        _ => unreachable!("ComparisonOperator gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
    }
}

fn integer_domain_identity(domain: &IntegerDomain) -> String {
    match domain {
        IntegerDomain::Mathematical => "mathematical".to_owned(),
        IntegerDomain::Bounded(interval) => interval_identity(interval),
        &_ => unreachable!("IntegerDomain gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
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
// Every function calls the pinned `quire_contract_runtime::exact` operator with
// the caller's `Meter`; no charge amount appears in this source. The source has
// no inner attributes so that it can be `include!`d; the manifest forbids
// unsafe code instead.

use quire_contract_runtime::exact as rt;

/// Why a generated oracle stopped before a runtime outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OracleStop {
    /// The runtime refused the operands or a generated type as ill-typed.
    IllTyped(rt::IllTyped),
    /// A generated constant was rejected by its runtime constructor.
    InvalidConstant,
    /// An IEEE operand does not have the declared width.
    OperandWidth,
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

impl SourceBuilder {
    fn oracle(
        &mut self,
        symbol: &str,
        node_id: &CheckedNodeId,
        identity: &str,
        operation: &ExactScalarOperation,
    ) {
        let body = self.body(operation);
        let parameters = body
            .parameters
            .iter()
            .map(|(name, ty)| format!("{name}: {ty}, "))
            .collect::<String>();
        self.functions.push_str(&format!(
            "\n/// Node `{}`: `{}`.\npub fn {symbol}({parameters}meter: &mut rt::Meter) -> Result<rt::Outcome<{}>, OracleStop> {{\n",
            node_id.digest, identity, body.output
        ));
        for line in body.prelude {
            self.functions.push_str("    ");
            self.functions.push_str(&line);
            self.functions.push('\n');
        }
        self.functions.push_str("    ");
        self.functions.push_str(&body.call);
        self.functions.push_str("\n}\n");
    }

    fn finish(self, package_id: &CheckedSemanticId) -> String {
        let mut source = format!("// Source package: {}\n", package_id.digest);
        source.push_str(SOURCE_HEADER);
        if self.needs_integer {
            source.push_str(INTEGER_HELPER);
        }
        if self.needs_width {
            source.push_str(WIDTH_HELPER);
        }
        source.push_str(&self.functions);
        source
    }

    fn integer_domain(&mut self, domain: &IntegerDomain) -> String {
        match domain {
            IntegerDomain::Mathematical => "rt::IntegerDomain::Mathematical".to_owned(),
            IntegerDomain::Bounded(interval) => {
                format!("rt::IntegerDomain::Bounded({})", self.interval(interval))
            },
            &_ => unreachable!("IntegerDomain gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
        }
    }

    /// `rt::evaluate_integer_arithmetic`'s bound argument, `Option<&rt::IntegerInterval>`: the
    /// same two states codegen's own `IntegerDomain` carries, since the runtime dropped the
    /// redundant `Mathematical`/`Bounded` wrapper in favor of `None`/`Some` directly over the
    /// interval it already had (`quire-contract-runtime` `src/exact/numeric.rs`). Unlike
    /// [`Self::integer_domain`], which still emits the wrapper for `rt::divide`/`rt::modulo`,
    /// this is only for `evaluate_integer_arithmetic`'s call site.
    fn integer_arithmetic_bound(&mut self, domain: &IntegerDomain) -> (Vec<String>, String) {
        match domain {
            IntegerDomain::Mathematical => (Vec::new(), "None".to_owned()),
            IntegerDomain::Bounded(interval) => (
                vec![format!("let domain = {};", self.interval(interval))],
                "Some(&domain)".to_owned(),
            ),
            &_ => unreachable!("IntegerDomain gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
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

    fn decimal_type(&mut self, decimal: &DecimalType) -> String {
        self.needs_integer = true;
        format!(
            "rt::DecimalType::new(integer(\"{}\")?, integer(\"{}\")?, {}, {}, {}).map_err(|_| OracleStop::InvalidConstant)?",
            decimal.lower(),
            decimal.upper(),
            decimal.min_scale(),
            decimal.max_scale(),
            rounding_path(decimal.rounding())
        )
    }

    fn body(&mut self, operation: &ExactScalarOperation) -> Body {
        match operation {
            ExactScalarOperation::IntegerArithmetic { operator, domain } => {
                let (prelude, domain_argument) = self.integer_arithmetic_bound(domain);
                let (parameters, operation) = match operator {
                    IntegerOperator::Negate => (unary("&rt::Integer"), "Negate(operand)"),
                    IntegerOperator::Add => (binary("&rt::Integer"), "Add(left, right)"),
                    IntegerOperator::Subtract => (binary("&rt::Integer"), "Subtract(left, right)"),
                    IntegerOperator::Multiply => (binary("&rt::Integer"), "Multiply(left, right)"),
                };
                Body {
                    parameters,
                    output: "rt::Integer",
                    prelude,
                    call: format!(
                        "Ok(rt::evaluate_integer_arithmetic(rt::IntegerArithmetic::{operation}, {domain_argument}, meter))"
                    ),
                }
            }
            ExactScalarOperation::IntegerDivision { profile, domain } => {
                let domain = self.integer_domain(domain);
                let profile = match profile {
                    DivisionProfile::Truncating => "Truncating",
                    DivisionProfile::Floor => "Floor",
                    DivisionProfile::Euclidean => "Euclidean",
                    &_ => unreachable!("DivisionProfile gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
                };
                Body {
                    parameters: binary("&rt::Integer"),
                    output: "rt::QuotientRemainder",
                    prelude: vec![format!("let domain = {domain};")],
                    call: format!(
                        "Ok(rt::divide(rt::DivisionProfile::{profile}, left, right, &domain, meter))"
                    ),
                }
            }
            ExactScalarOperation::IntegerModulo { domain } => {
                let domain = self.integer_domain(domain);
                Body {
                    parameters: binary("&rt::Integer"),
                    output: "rt::Integer",
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
                // `Rational::from_integer`, then `Divide` (quire-contract-runtime
                // src/exact/numeric.rs). The deleted `IntegerDivide` variant charged the
                // same four rational-arithmetic points via the same `fn rational()`, so
                // bit-for-bit identical operands charge identically here. What actually
                // moved is upstream's own redesign of `fn rational()` itself (`.size` to
                // `.exact_size`, normalize-on-unreduced instead of normalize-on-reduced),
                // which affects every `RationalArithmetic` node uniformly, not something
                // specific to this lowering; `from_integer` itself is an uncharged
                // construction, so the lift below is free.
                let (parameters, lift_prelude, operation) = match operator {
                    RationalOperator::Negate => (unary("&rt::Rational"), Vec::new(), "Negate(operand)"),
                    RationalOperator::Add => (binary("&rt::Rational"), Vec::new(), "Add(left, right)"),
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
                    output: "rt::Rational",
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
                    &_ => unreachable!("OrderingOperator gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
                };
                let (ty, kind) = match operands {
                    OrderingOperandKind::Integer => ("&rt::Integer", "Integers"),
                    OrderingOperandKind::Rational => ("&rt::Rational", "Rationals"),
                    OrderingOperandKind::Decimal => ("&rt::Decimal", "Decimals"),
                };
                Body {
                    parameters: binary(ty),
                    output: "bool",
                    prelude: Vec::new(),
                    call: format!(
                        "Ok(rt::order_numbers(rt::OrderingOperator::{operator}, rt::OrderedOperands::{kind}(left, right), meter))"
                    ),
                }
            }
            ExactScalarOperation::DecimalArithmetic { operator, target } => {
                let target = self.decimal_type(target);
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
                    output: "rt::DecimalResult",
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
                let width = width_path(*width);
                Body {
                    parameters: binary("rt::IeeeValue"),
                    output: "rt::IeeeResult",
                    prelude: vec![
                        format!("let left = expect_width(left, {width})?;"),
                        format!("let right = expect_width(right, {width})?;"),
                    ],
                    call: format!(
                        "rt::evaluate_ieee(rt::IeeeOperation::{operator}(left, right), {}, meter).map_err(OracleStop::IllTyped)",
                        rounding_path(*rounding)
                    ),
                }
            }
            ExactScalarOperation::IeeeComparison { comparison, width } => {
                self.needs_width = true;
                let comparison = match comparison {
                    IeeeComparison::NumericEqual => "NumericEqual",
                    IeeeComparison::TotalOrder => "TotalOrder",
                    IeeeComparison::BitIdentical => "BitIdentical",
                    &_ => unreachable!("IeeeComparison gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
                };
                let width = width_path(*width);
                Body {
                    parameters: binary("rt::IeeeValue"),
                    output: "bool",
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
                    output: "rt::IeeeResult",
                    prelude: vec![format!(
                        "let operand = expect_width(operand, {})?;",
                        width_path(*source)
                    )],
                    call: format!(
                        "Ok(rt::convert_ieee_width(operand, {}, {}, meter))",
                        width_path(*target),
                        rounding_path(*rounding)
                    ),
                }
            }
            ExactScalarOperation::TextAdmission { text_type } => Body {
                parameters: unary("&rt::TextPayload"),
                output: "rt::Text",
                prelude: vec![format!(
                    "let text_type = rt::TextType::new({}, {}, {}).map_err(|_| OracleStop::InvalidConstant)?;",
                    text_type.min(),
                    text_type.max(),
                    profile_path(text_type.profile())
                )],
                call: "Ok(rt::admit_text(operand, &text_type, meter))".to_owned(),
            },
            ExactScalarOperation::TextComparison { operator } => comparison_body(
                "&rt::Text",
                "compare_text",
                *operator,
            ),
            ExactScalarOperation::EnumComparison { operator } => comparison_body(
                "&rt::EnumValue",
                "compare_enum",
                *operator,
            ),
            ExactScalarOperation::QuantityComparison { operator } => comparison_body(
                "&rt::Quantity",
                "compare_quantity",
                *operator,
            ),
            ExactScalarOperation::QuantityArithmetic { operator } => {
                let (parameters, operation) = match operator {
                    QuantityOperator::Add => (binary("&rt::Quantity"), "Add(left, right)"),
                    QuantityOperator::Subtract => {
                        (binary("&rt::Quantity"), "Subtract(left, right)")
                    }
                    QuantityOperator::Multiply => {
                        (binary("&rt::Quantity"), "Multiply(left, right)")
                    }
                    QuantityOperator::Divide => (binary("&rt::Quantity"), "Divide(left, right)"),
                    QuantityOperator::Power => (
                        vec![("left", "&rt::Quantity"), ("right", "&rt::Integer")],
                        "Power(left, right)",
                    ),
                };
                Body {
                    parameters,
                    output: "rt::Quantity",
                    prelude: Vec::new(),
                    call: format!(
                        "rt::evaluate_quantity(rt::QuantityOperation::{operation}, meter).map_err(OracleStop::IllTyped)"
                    ),
                }
            }
            ExactScalarOperation::QuantityConversion { target } => {
                let target = match target {
                    QuantityTarget::Exact => "rt::QuantityTarget::Exact".to_owned(),
                    QuantityTarget::Decimal(decimal) => {
                        format!("rt::QuantityTarget::Decimal({})", self.decimal_type(decimal))
                    }
                    QuantityTarget::Integer { domain, rounding } => format!(
                        "rt::QuantityTarget::Integer {{ domain: {}, rounding: {} }}",
                        self.interval(domain),
                        rounding_path(*rounding)
                    ),
                    &_ => unreachable!("QuantityTarget gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
                };
                Body {
                    parameters: vec![("operand", "&rt::Quantity"), ("unit", "&rt::QuantityUnit")],
                    output: "rt::Conversion",
                    prelude: vec![format!("let target = {target};")],
                    call: "rt::convert_quantity(operand, unit, &target, meter).map_err(OracleStop::IllTyped)"
                        .to_owned(),
                }
            }
        }
    }
}

struct Body {
    parameters: Vec<(&'static str, &'static str)>,
    output: &'static str,
    prelude: Vec<String>,
    call: String,
}

fn unary(ty: &'static str) -> Vec<(&'static str, &'static str)> {
    vec![("operand", ty)]
}

fn binary(ty: &'static str) -> Vec<(&'static str, &'static str)> {
    vec![("left", ty), ("right", ty)]
}

fn comparison_body(ty: &'static str, function: &str, operator: ComparisonOperator) -> Body {
    let operator = match operator {
        ComparisonOperator::Equal => "Equal",
        ComparisonOperator::NotEqual => "NotEqual",
        ComparisonOperator::Less => "Less",
        ComparisonOperator::LessOrEqual => "LessOrEqual",
        ComparisonOperator::Greater => "Greater",
        ComparisonOperator::GreaterOrEqual => "GreaterOrEqual",
        _ => unreachable!("ComparisonOperator gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
    };
    Body {
        parameters: binary(ty),
        output: "bool",
        prelude: Vec::new(),
        call: format!(
            "rt::{function}(rt::ComparisonOperator::{operator}, left, right, meter).map_err(OracleStop::IllTyped)"
        ),
    }
}

fn rounding_path(rounding: RoundingMode) -> &'static str {
    match rounding {
        RoundingMode::Exact => "rt::RoundingMode::Exact",
        RoundingMode::TowardZero => "rt::RoundingMode::TowardZero",
        RoundingMode::TowardPositive => "rt::RoundingMode::TowardPositive",
        RoundingMode::TowardNegative => "rt::RoundingMode::TowardNegative",
        RoundingMode::NearestEven => "rt::RoundingMode::NearestEven",
        RoundingMode::NearestAway => "rt::RoundingMode::NearestAway",
        _ => unreachable!("RoundingMode gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
    }
}

fn width_path(width: IeeeWidth) -> &'static str {
    match width {
        IeeeWidth::Binary32 => "rt::IeeeWidth::Binary32",
        IeeeWidth::Binary64 => "rt::IeeeWidth::Binary64",
        _ => unreachable!("IeeeWidth gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
    }
}

fn profile_path(profile: TextProfile) -> &'static str {
    match profile {
        TextProfile::UnicodeScalars => "rt::TextProfile::UnicodeScalars",
        TextProfile::Nfc => "rt::TextProfile::Nfc",
        TextProfile::Nfd => "rt::TextProfile::Nfd",
        TextProfile::Nfkc => "rt::TextProfile::Nfkc",
        TextProfile::Nfkd => "rt::TextProfile::Nfkd",
        TextProfile::BinaryUtf8 => "rt::TextProfile::BinaryUtf8",
        _ => unreachable!("TextProfile gained a variant after RT #70 (IR-77) added #[non_exhaustive]; every variant that existed then is matched above"),
    }
}

fn manifest() -> String {
    format!(
        "[package]\nname = \"{EXACT_SCALAR_CRATE_NAME}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[lib]\npath = \"src/lib.rs\"\n\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\", features = [\"exact\"] }}\n\n[lints.rust]\nunsafe_code = \"forbid\"\n\n[workspace]\n"
    )
}

fn artifact(path: &str, contents: String) -> Artifact {
    let digest = Sha256::digest(contents.as_bytes());
    let sha256 = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    Artifact {
        path: path.to_owned(),
        contents,
        sha256,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quire_contract_ir::{
        CheckedPackageIncomplete, CheckedPackageLimit, CheckedPackageRefusal,
        CheckedPackageRefusalCode,
    };

    fn node_id(digit: char) -> CheckedNodeId {
        serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": digit.to_string().repeat(64),
        }))
        .expect("node id")
    }

    /// Admission re-validates every body before lowering, so an admitted package
    /// cannot produce these records; they are built directly.
    ///
    /// Trace: FR-014-AC-3, TC-024.
    #[test]
    fn tc_024_invalid_and_incomplete_bodies_are_typed_refusals() {
        let invalid = CompleteLoweringRecordV2::InvalidBody {
            node_id: node_id('a'),
            body_node_id: node_id('b'),
            refusal: CheckedPackageRefusal {
                code: CheckedPackageRefusalCode::InvalidSemanticGraph,
                path: None,
                cause: None,
                locus: None,
                contract_version: None,
            },
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
                limit_kind: CheckedPackageLimit::Depth,
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
    ///
    /// Trace: FR-014-AC-3, TC-024.
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

        assert_eq!(
            catalogued_operation_identity(&node),
            Err(ExactScalarRefusal::MissingOperationIdentity {
                node_id: node_id('d'),
            }),
            "a node missing the member IR admission guarantees must be refused by name, \
             not reported as an identity this generator declined to consume"
        );
    }
}

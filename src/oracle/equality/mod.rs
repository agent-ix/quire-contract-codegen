//! Exact complete-V1 composite/structural equality oracle generation from
//! CheckedPackage V2 (FR-018).
//!
//! This is the composite/structural sibling of [`crate::oracle::scalar`]
//! (FR-014): a requested item names one checked `binary` expression node and a
//! typed equality descriptor — an operator and two operands, each either
//! `EqualityOperand::typed(source)` or `EqualityOperand::converted(source,
//! target)` — where `source`/`target` are named as V2 type node ids, not as
//! runtime `ValueType` values, because a runtime `ValueType` carries no V2
//! node id once built and this generator's ordering needs one
//! (see the module's `DescriptorKey`).
//!
//! The runtime carries no structural equality on `Value`; the FR-149 relation
//! reached through `TypeEnvironment::check_equality` and
//! `CheckedEquality::evaluate` is the only equality. This generator therefore
//! never compares two values itself: it reconstructs the record/tuple
//! declaration closure reachable from each operand's type, admits it through
//! `TypeEnvironment::new` (refusing per [`CompositeEqualityRefusal::Declaration`] on failure),
//! checks the descriptor through `TypeEnvironment::check_equality` (refusing
//! per [`IllTypedCauseKind`] on failure), and emits two functions per
//! surviving item: an environment constructor
//! (`Result<TypeEnvironment, EnvironmentError>`) and an oracle function
//! (`(&TypeEnvironment, &Value, &Value, &mut Meter) -> Outcome<bool>`). The
//! oracle calls `TypeEnvironment::check_type` on each operand's comparison
//! type before `check_equality`, so a caller-supplied environment that does
//! not admit the descriptor's types is refused with
//! `Outcome::Refused(Refusal::CheckedInvariant)` rather than completing.
//!
//! V2 does not define a normative body for `composite_type` or
//! `collection_bounds` nodes. This generator reads exactly one encoding,
//! defined by this generator rather than by V2, and refuses anything else:
//!
//! | Form | Members |
//! |------|---------|
//! | `record` | one `binding` per field in declaration order: its `name` is the field name; a direct reference is required, while an `aggregate` with one `binding(optional, reference Option)` is optional |
//! | `tuple` | one `reference` per position, in declaration order |
//! | `option` | one `reference` to the payload type node |
//! | `sequence` | one element `reference` when named by a `collection_bounds` node; an inline bounds form has a second bounds `reference` |
//! | `set`, `bag`, `ordered_set` | an element `reference`, then a bounds `reference` |
//! | `collection_bounds` | named `min` and `max` bindings of canonical decimal `integer` literals |
//!
//! Neither this generator nor the source it emits panics (FR-018-AC-17,
//! FR-018-AC-19). Every runtime constructor the emitted source calls to
//! rebuild a bound (`IntegerInterval::new`, `RationalDomain::new`,
//! `DecimalType::new`, `TextType::new`, `CardinalityBound::new`, and the integer
//! parse) is called only inside a `rebuild_*` helper returning
//! `Result<_, ReconstructionError>`. The per-item `ValueType` functions
//! propagate that `Result`; the environment constructor turns a failure into
//! `EnvironmentError::Reconstruction`, and the oracle function into
//! `Outcome::Refused(Refusal::CheckedInvariant)`. A runtime that tightens a
//! constructor therefore refuses; it never panics a generated oracle and never
//! widens a type. `TypeEnvironment::new`'s own refusal is the constructor's
//! `EnvironmentError::Declaration`.
//!
//! Model graph, identity and reachability oracles
//! (agent-ix/quire-spec-language#120), function application oracles
//! (agent-ix/quire-contract-runtime#34), and temporal/protocol oracles
//! (agent-ix/quire-spec-language#121) are out of scope; items reaching them
//! are refused with a distinct typed blocker. `Reference<T>` operands are
//! refused for the same reason: CheckedPackage V2 carries no form that yields
//! one. Quantity (`unit`/`dimension` scalar leaves) is also refused as
//! unsupported by this generator: FR-018's Inputs section closes the
//! declaration-closure vocabulary at `composite_type`, `scalar_type` and
//! `bounded_domain` nodes, and reconstructing a concrete `QuantityUnit`
//! requires walking a unit graph this generator does not read.

use crate::core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES};
use crate::core::naming::unique_names;
use crate::core::profile::oracle_crate_manifest;
use crate::oracle::claim::{ClaimDisposition, ClaimMap, OracleGenerationError, UpstreamBlocker};
use crate::oracle::scalar::{
    aggregate_members, bound_members, literal_count, read_decimal_range, read_integer_range,
    read_rational_range, read_text_bounds, COLLECTION_BOUNDS_MEMBERS,
};
use crate::oracle::{classify_lowering_failure, LoweringFailure};
use quire_contract_model::{
    CheckedNodeId, CheckedNodeTag, CheckedPackageV2, CheckedSemanticId, CheckedSemanticNodeV2,
    CheckedSourceMapEntry, CompleteLoweringProfileV2, CompleteLoweringRecordV2,
};
use quire_contract_runtime::exact::{
    self as rt, CardinalityBound, CollectionKind, CollectionType, CompositeDeclaration,
    CompositeShape, DecimalType, EqualityOperand, EqualitySchedule, FieldDeclaration,
    IllTypedCause, IntegerInterval, NodeKey, Presence, RoundingMode, TextProfile, TypeEnvironment,
    ValueType,
};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

mod resolution;

/// Work budget for lowering one requested equality expression node.
pub const COMPOSITE_EQUALITY_LOWERING_WORK_LIMIT: u64 = 65_536;

/// Type-node entries permitted while resolving one equality item.
pub const COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT: u64 = 65_536;

/// Name of the generated crate.
pub const COMPOSITE_EQUALITY_CRATE_NAME: &str = "quire-composite-equality-oracles";

/// The grammar's equality operators, mirroring
/// `quire_contract_runtime::exact::EqualityOperator`. Declaration order is
/// its rank: `Equal` before `NotEqual`, per the descriptor key.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EqualityOperatorKind {
    /// `=`.
    Equal,
    /// `!=`.
    NotEqual,
}

impl EqualityOperatorKind {
    fn to_runtime(self) -> rt::EqualityOperator {
        match self {
            Self::Equal => rt::EqualityOperator::Equal,
            Self::NotEqual => rt::EqualityOperator::NotEqual,
        }
    }

    fn identity(self) -> &'static str {
        match self {
            Self::Equal => "equality.equal",
            Self::NotEqual => "equality.not_equal",
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::Equal => "rt::EqualityOperator::Equal",
            Self::NotEqual => "rt::EqualityOperator::NotEqual",
        }
    }
}

/// One operand's static type, named as V2 node ids: a source type and,
/// for `convert<T>(e)`, a conversion target.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EqualityOperandDescriptor {
    /// V2 node id of the operand's static (source) type.
    pub source_type: CheckedNodeId,
    /// V2 node id of the `convert<T>` target type, when the operand is
    /// converted.
    pub conversion_target: Option<CheckedNodeId>,
}

impl EqualityOperandDescriptor {
    /// An operand of static type `source_type`.
    pub fn typed(source_type: CheckedNodeId) -> Self {
        Self {
            source_type,
            conversion_target: None,
        }
    }

    /// An operand `convert<target_type>(e)` for `e` of static type
    /// `source_type`.
    pub fn converted(source_type: CheckedNodeId, target_type: CheckedNodeId) -> Self {
        Self {
            source_type,
            conversion_target: Some(target_type),
        }
    }
}

/// One requested oracle: a checked `binary` expression node and its typed
/// equality descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositeEqualityItem {
    /// Checked expression node to generate.
    pub node_id: CheckedNodeId,
    /// `=` or `!=`.
    pub operator: EqualityOperatorKind,
    /// Left operand descriptor.
    pub left: EqualityOperandDescriptor,
    /// Right operand descriptor.
    pub right: EqualityOperandDescriptor,
}

/// Where a claim's operation identity comes from.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CompositeOperationProvenance {
    /// The request descriptor declared it and the IR cannot confirm it.
    CallerDeclared {
        /// The missing upstream transport.
        blocked_on: UpstreamBlocker,
    },
}

/// The operation a claim-map entry is about.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompositeOperationClaim {
    /// Stable identity of the declared operation.
    pub identity: String,
    /// Where the identity comes from.
    pub provenance: CompositeOperationProvenance,
}

/// A serializable mirror of `quire_contract_runtime::exact::IllTypedCause`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IllTypedCauseKind {
    /// `IllTypedCause::DistinctTextProfiles`.
    DistinctTextProfiles,
    /// `IllTypedCause::DistinctEnumDeclarations`.
    DistinctEnumDeclarations,
    /// `IllTypedCause::UnorderedEnumOrdering`.
    UnorderedEnumOrdering,
    /// `IllTypedCause::IncompatibleDimensions`.
    IncompatibleDimensions,
    /// `IllTypedCause::AffineUnitArithmetic`.
    AffineUnitArithmetic,
    /// `IllTypedCause::DistinctUnits`.
    DistinctUnits,
    /// `IllTypedCause::MalformedDecimalType`.
    MalformedDecimalType,
    /// `IllTypedCause::DistinctIeeeWidths`.
    DistinctIeeeWidths,
    /// `IllTypedCause::IeeeWithExactOperand`.
    IeeeWithExactOperand,
    /// `IllTypedCause::IeeeToNonRationalExact`.
    IeeeToNonRationalExact,
    /// `IllTypedCause::TypeMismatch`.
    TypeMismatch,
    /// `IllTypedCause::OperatorIneligible`.
    OperatorIneligible,
    /// `IllTypedCause::AmbiguousLiteral`.
    AmbiguousLiteral,
}

impl TryFrom<IllTypedCause> for IllTypedCauseKind {
    type Error = OracleGenerationError;

    fn try_from(cause: IllTypedCause) -> Result<Self, Self::Error> {
        Ok(match cause {
            IllTypedCause::DistinctTextProfiles => Self::DistinctTextProfiles,
            IllTypedCause::DistinctEnumDeclarations => Self::DistinctEnumDeclarations,
            IllTypedCause::UnorderedEnumOrdering => Self::UnorderedEnumOrdering,
            IllTypedCause::IncompatibleDimensions => Self::IncompatibleDimensions,
            IllTypedCause::AffineUnitArithmetic => Self::AffineUnitArithmetic,
            IllTypedCause::DistinctUnits => Self::DistinctUnits,
            IllTypedCause::MalformedDecimalType => Self::MalformedDecimalType,
            IllTypedCause::DistinctIeeeWidths => Self::DistinctIeeeWidths,
            IllTypedCause::IeeeWithExactOperand => Self::IeeeWithExactOperand,
            IllTypedCause::IeeeToNonRationalExact => Self::IeeeToNonRationalExact,
            IllTypedCause::TypeMismatch => Self::TypeMismatch,
            IllTypedCause::OperatorIneligible => Self::OperatorIneligible,
            IllTypedCause::AmbiguousLiteral => Self::AmbiguousLiteral,
            _ => return Err(OracleGenerationError::unknown_variant("IllTypedCause")),
        })
    }
}

/// A serializable mirror of `quire_contract_runtime::exact::RecursionEdges`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecursionEdgesKind {
    /// `RecursionEdges::Unnamed`.
    Unnamed,
    /// `RecursionEdges::NonEscaping`.
    NonEscaping,
}

impl TryFrom<rt::RecursionEdges> for RecursionEdgesKind {
    type Error = OracleGenerationError;

    fn try_from(edges: rt::RecursionEdges) -> Result<Self, Self::Error> {
        match edges {
            rt::RecursionEdges::Unnamed => Ok(Self::Unnamed),
            rt::RecursionEdges::NonEscaping => Ok(Self::NonEscaping),
            _ => Err(OracleGenerationError::unknown_variant("RecursionEdges")),
        }
    }
}

/// A serializable mirror of `quire_contract_runtime::exact::DeclarationCause`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DeclarationRefusalCause {
    /// `DeclarationCause::DuplicateKey`.
    DuplicateKey,
    /// `DeclarationCause::DuplicateMember`.
    DuplicateMember {
        /// The repeated field or attribute name.
        name: String,
    },
    /// `DeclarationCause::UnknownDeclaration`.
    UnknownDeclaration {
        /// The unresolved declaration key, as hexadecimal digits.
        key: String,
    },
    /// `DeclarationCause::Type`.
    Type {
        /// The ill-typed member's cause.
        cause: IllTypedCauseKind,
    },
    /// `DeclarationCause::Recursion`.
    Recursion {
        /// The offending subgraph.
        edges: RecursionEdgesKind,
        /// The declaration names along the cycle.
        cycle: Vec<String>,
    },
}

impl TryFrom<rt::DeclarationCause> for DeclarationRefusalCause {
    type Error = OracleGenerationError;

    fn try_from(cause: rt::DeclarationCause) -> Result<Self, Self::Error> {
        Ok(match cause {
            rt::DeclarationCause::DuplicateKey => Self::DuplicateKey,
            rt::DeclarationCause::DuplicateMember(name) => Self::DuplicateMember { name },
            rt::DeclarationCause::UnknownDeclaration(key) => Self::UnknownDeclaration {
                key: hex_digest(&key),
            },
            rt::DeclarationCause::Type(cause) => Self::Type {
                cause: cause.try_into()?,
            },
            rt::DeclarationCause::Recursion { edges, cycle } => Self::Recursion {
                edges: edges.try_into()?,
                cycle,
            },
            _ => return Err(OracleGenerationError::unknown_variant("DeclarationCause")),
        })
    }
}

/// Why one requested item generated no code.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum CompositeEqualityRefusal {
    /// The item's node id and descriptor were requested more than once;
    /// every copy is refused.
    DuplicateRequest,
    /// The expression node is not in the admitted graph.
    InvalidInput,
    /// The node is not an expression.
    NotExpression {
        /// Its family.
        node_tag: &'static str,
    },
    /// The expression form is not `binary`.
    FormMismatch {
        /// Form found.
        found: String,
    },
    /// The body is not a two-argument `binary` application.
    BodyMismatch,
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
    /// Lowering exceeded [`COMPOSITE_EQUALITY_LOWERING_WORK_LIMIT`]; the refusal of the work
    /// ceiling and of nothing else.
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
    /// A named type node id is not in the admitted graph.
    UnknownTypeNode {
        /// The unresolved node id.
        type_node_id: CheckedNodeId,
    },
    /// An unclosed type node repeats on the active resolution path.
    TypeResolutionCycle {
        /// The repeated type node.
        repeated_type_node_id: CheckedNodeId,
    },
    /// Resolving one item's type nodes exceeded its work ceiling.
    TypeResolutionWorkExhausted {
        /// The ceiling.
        limit: u64,
        /// Counter at the failed charge.
        consumed: u64,
    },
    /// A reachable node's family has no finite exact encoding this
    /// generator reads.
    Unsupported {
        /// First unsupported reachable node.
        unsupported_node_id: CheckedNodeId,
        /// Its family or form.
        node_tag: &'static str,
    },
    /// A reachable node's family awaits upstream semantics.
    BlockedOnUpstream {
        /// First blocked reachable node.
        unsupported_node_id: CheckedNodeId,
        /// Its family or form.
        node_tag: &'static str,
        /// The upstream issue.
        issue: UpstreamBlocker,
    },
    /// A reachable type has no reachable bound of the form it needs.
    MissingBound {
        /// The unbounded type.
        bounded_type: CheckedNodeId,
        /// The form needed.
        expected_form: &'static str,
    },
    /// More than one reachable bound of that form bounds the type.
    AmbiguousBound {
        /// The type.
        bounded_type: CheckedNodeId,
        /// The repeated form.
        expected_form: &'static str,
    },
    /// The bound's body is not the encoding this generator reads.
    UnreadableBound {
        /// The bound node.
        bound: CheckedNodeId,
    },
    /// The composite's body is not the encoding this generator reads.
    MalformedComposite {
        /// The composite node.
        composite: CheckedNodeId,
    },
    /// `TypeEnvironment::new` refused the reconstructed declaration closure.
    Declaration {
        /// The declaration name where the refusal originates.
        declaration: String,
        /// The typed cause.
        cause: DeclarationRefusalCause,
    },
    /// `TypeEnvironment::check_equality` refused the descriptor.
    IllTyped {
        /// The typed cause.
        cause: IllTypedCauseKind,
    },
    /// A body operand's declared type disagrees with the descriptor's for
    /// that position.
    OperandTypeMismatch {
        /// Zero-based operand position (0 = left, 1 = right).
        position: usize,
        /// Type node id the descriptor declares for that operand.
        expected: CheckedNodeId,
        /// Type node id the body operand actually names, when it names one
        /// at all.
        found: Option<CheckedNodeId>,
    },
}

/// Traceability for one generated oracle.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GeneratedCompositeEqualityClaim {
    /// Generated environment constructor name.
    pub environment_symbol: String,
    /// Generated oracle function name.
    pub oracle_symbol: String,
    /// Contract IR identity of the lowered expression node.
    pub ir_id: CheckedSemanticId,
    /// Package id.
    pub package_id: CheckedSemanticId,
    /// Semantic type key of the expression node.
    pub semantic_type: CheckedNodeId,
    /// Exact source correspondence.
    pub source_map: Vec<CheckedSourceMapEntry>,
    /// Reachable claim keys.
    pub claims: Vec<CheckedNodeId>,
    /// The descriptor the request supplied: operator and both operands'
    /// source and conversion-target node ids, read from the request, never
    /// from the generated source.
    pub descriptor: RecordedDescriptor,
    /// Every V2 node id whose declaration entered the reconstructed closure,
    /// ascending by digest domain then digest.
    pub declaration_keys: Vec<CheckedNodeId>,
    /// The runtime key of each declaration entry, ascending, equal to
    /// `NodeKey::from_hex` of `declaration_keys`.
    pub declaration_runtime_keys: Vec<String>,
    /// The schedule `CheckedEquality::schedule()` selected.
    pub schedule: RecordedSchedule,
}

/// A serializable mirror of `EqualitySchedule`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedSchedule {
    /// `EqualitySchedule::Text`.
    Text,
    /// `EqualitySchedule::Enum`.
    Enum,
    /// `EqualitySchedule::Quantity`.
    Quantity,
    /// `EqualitySchedule::Plan`.
    Plan,
}

impl TryFrom<EqualitySchedule> for RecordedSchedule {
    type Error = OracleGenerationError;

    fn try_from(schedule: EqualitySchedule) -> Result<Self, Self::Error> {
        match schedule {
            EqualitySchedule::Text => Ok(Self::Text),
            EqualitySchedule::Enum => Ok(Self::Enum),
            EqualitySchedule::Quantity => Ok(Self::Quantity),
            EqualitySchedule::Plan => Ok(Self::Plan),
            _ => Err(OracleGenerationError::unknown_variant("EqualitySchedule")),
        }
    }
}

/// The claim-map's own copy of the request descriptor.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RecordedDescriptor {
    /// `=` or `!=`.
    pub operator: EqualityOperatorKind,
    /// Left operand's source type node id.
    pub left_source_type: CheckedNodeId,
    /// Left operand's conversion target node id, when converted.
    pub left_conversion_target: Option<CheckedNodeId>,
    /// Right operand's source type node id.
    pub right_source_type: CheckedNodeId,
    /// Right operand's conversion target node id, when converted.
    pub right_conversion_target: Option<CheckedNodeId>,
}

/// One claim-map entry per distinct requested `(node id, descriptor)`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompositeEqualityClaim {
    /// Requested expression node.
    pub node_id: CheckedNodeId,
    /// The declared operation and its provenance.
    pub operation: CompositeOperationClaim,
    /// Outcome.
    pub result: ClaimDisposition<GeneratedCompositeEqualityClaim, CompositeEqualityRefusal>,
}

/// Generation output: the crate files and the claim map they are described
/// by.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositeEqualityOracles {
    /// `Cargo.toml`, `src/lib.rs` and `claim-map.json`, in that order.
    pub artifacts: Vec<Artifact>,
    /// Typed claim map, identical to `claim-map.json`. Items are ordered by
    /// the descriptor key. `blocked` lists the upstream gaps every entry is
    /// subject to. A `Generated` disposition means one environment
    /// constructor and one oracle function were emitted. The blockers this
    /// generator records are `QuireSpecLanguage120`, `QuireSpecLanguage121`,
    /// `QuireContractRuntime34` and `OperationIdentityNotConsumed`.
    pub claim_map: ClaimMap<CompositeEqualityClaim>,
}

/// Generate composite equality oracles for `items` from an admitted package.
///
/// Fails as a whole only with `SourceTooLarge`, `ClaimMapSerialization` or
/// `UnknownRuntimeVariant` (a Contract Runtime `#[non_exhaustive]` enum
/// yielded a variant this generator does not know); every per-item problem is
/// a refusal in the claim map.
pub fn generate_composite_equality_oracles(
    package: &CheckedPackageV2,
    items: &[CompositeEqualityItem],
) -> Result<CompositeEqualityOracles, OracleGenerationError> {
    let graph: Graph<'_> = package
        .graph()
        .nodes
        .iter()
        .map(|node| (&node.node_id, node))
        .collect();
    let mut bounds_by_type: BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2>> = BTreeMap::new();
    for node in package.graph().nodes.iter() {
        if CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::BoundedDomain) {
            bounds_by_type
                .entry(&node.semantic_type)
                .or_default()
                .push(node);
        }
    }

    // Requests are deduplicated and ordered by the descriptor key so that
    // equal requests in any order produce identical bytes (FR-018-AC-10).
    // One `(node id, descriptor)` pair requested more than once refuses
    // every copy (FR-018-AC-11); one node id under two descriptors is two
    // distinct items.
    let mut counts: BTreeMap<DescriptorKey, u32> = BTreeMap::new();
    let mut by_key: BTreeMap<DescriptorKey, &CompositeEqualityItem> = BTreeMap::new();
    for item in items {
        let key = DescriptorKey::of(item);
        *counts.entry(key.clone()).or_insert(0) += 1;
        by_key.entry(key).or_insert(item);
    }

    let requested: Vec<CheckedNodeId> = by_key.values().map(|item| item.node_id.clone()).collect();
    let lowering = package.lower(&requested, &lowering_profile());

    let mut source = SourceBuilder::default();
    let mut claims = Vec::with_capacity(by_key.len());
    // Generated claims as (claim position, key, item, checked item), rendered once every
    // generated oracle is named.
    let mut pending = Vec::new();
    for ((key, item), record) in by_key.into_iter().zip(&lowering.records) {
        let duplicate = counts.get(&key).copied().unwrap_or(0) > 1;
        let result = if duplicate {
            ClaimDisposition::Refused {
                refusal: CompositeEqualityRefusal::DuplicateRequest,
            }
        } else {
            match check_item(&graph, &bounds_by_type, record, item) {
                Ok(generated) => {
                    let claim =
                        ClaimDisposition::Generated(Box::new(GeneratedCompositeEqualityClaim {
                            environment_symbol: String::new(),
                            oracle_symbol: String::new(),
                            ir_id: generated.node.ir_id.clone(),
                            package_id: lowering.package.source_package_id().clone(),
                            semantic_type: generated.node.semantic_type.clone(),
                            source_map: generated.node.source_map.clone(),
                            claims: generated.node.claims.clone(),
                            descriptor: recorded_descriptor(item),
                            declaration_keys: generated.declaration_keys.clone(),
                            declaration_runtime_keys: generated
                                .declaration_keys
                                .iter()
                                .map(|id| id.digest.to_lowercase())
                                .collect(),
                            schedule: generated.checked.schedule().try_into()?,
                        }));
                    pending.push((claims.len(), key, item, generated));
                    claim
                }
                Err(ItemCheckError::Refusal(refusal)) => ClaimDisposition::Refused { refusal },
                // An RT `#[non_exhaustive]` enum yielded a variant this
                // generator does not know: not this one item's refusal, so it
                // aborts the whole generation (see
                // `OracleGenerationError::UnknownRuntimeVariant`'s own doc).
                Err(ItemCheckError::Generation(error)) => return Err(error),
            }
        };
        claims.push(CompositeEqualityClaim {
            node_id: item.node_id.clone(),
            operation: CompositeOperationClaim {
                identity: item.operator.identity().to_owned(),
                provenance: CompositeOperationProvenance::CallerDeclared {
                    blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
                },
            },
            result,
        });
    }
    render_and_emit(&mut claims, &mut source, pending, render_item)?;

    let claim_map = ClaimMap {
        package_id: lowering.package.source_package_id().clone(),
        blocked: vec![UpstreamBlocker::OperationIdentityNotConsumed],
        items: claims,
    };
    let lib = source.finish();
    if lib.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(OracleGenerationError::SourceTooLarge { bytes: lib.len() });
    }
    let mut map_bytes = serde_json::to_vec_pretty(&claim_map)
        .map_err(|_| OracleGenerationError::ClaimMapSerialization)?;
    map_bytes.push(b'\n');
    let map_text =
        String::from_utf8(map_bytes).map_err(|_| OracleGenerationError::ClaimMapSerialization)?;
    Ok(CompositeEqualityOracles {
        artifacts: vec![
            artifact(
                "Cargo.toml",
                oracle_crate_manifest(COMPOSITE_EQUALITY_CRATE_NAME),
            ),
            artifact("src/lib.rs", lib),
            artifact("claim-map.json", map_text),
        ],
        claim_map,
    })
}

fn recorded_descriptor(item: &CompositeEqualityItem) -> RecordedDescriptor {
    RecordedDescriptor {
        operator: item.operator,
        left_source_type: item.left.source_type.clone(),
        left_conversion_target: item.left.conversion_target.clone(),
        right_source_type: item.right.source_type.clone(),
        right_conversion_target: item.right.conversion_target.clone(),
    }
}

fn lowering_profile() -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
        supported_tags: BTreeSet::from([
            CheckedNodeTag::ScalarType,
            CheckedNodeTag::CompositeType,
            CheckedNodeTag::BoundedDomain,
            CheckedNodeTag::Value,
            CheckedNodeTag::Expression,
            CheckedNodeTag::Claim,
            CheckedNodeTag::Correspondence,
        ]),
        require_bounds: false,
        work_limit: COMPOSITE_EQUALITY_LOWERING_WORK_LIMIT,
    }
}

type Graph<'a> = BTreeMap<&'a CheckedNodeId, &'a CheckedSemanticNodeV2>;

// ---------------------------------------------------------------------------
// Descriptor key and ordering (FR-018-AC-10, FR-018-AC-11)
// ---------------------------------------------------------------------------

/// The total order and deduplication key: the expression node's id,
/// the operator's rank, then the V2 node ids of the left operand's source
/// type and conversion target, then the right's, an absent conversion target
/// ranking before every present one. Never derived from a declaration,
/// field or rendered name.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DescriptorKey {
    node_id: CheckedNodeId,
    operator: EqualityOperatorKind,
    left_source_type: CheckedNodeId,
    left_conversion_target: Option<CheckedNodeId>,
    right_source_type: CheckedNodeId,
    right_conversion_target: Option<CheckedNodeId>,
}

impl DescriptorKey {
    fn of(item: &CompositeEqualityItem) -> Self {
        Self {
            node_id: item.node_id.clone(),
            operator: item.operator,
            left_source_type: item.left.source_type.clone(),
            left_conversion_target: item.left.conversion_target.clone(),
            right_source_type: item.right.source_type.clone(),
            right_conversion_target: item.right.conversion_target.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Per-item checking
// ---------------------------------------------------------------------------

/// A checked, admitted item ready to emit.
struct CheckedItem<'r> {
    node: &'r quire_contract_model::CompleteContractNodeV2,
    checked: rt::CheckedEquality,
    left_source: ValueType,
    left_target: Option<ValueType>,
    right_source: ValueType,
    right_target: Option<ValueType>,
    /// Every reachable record/tuple declaration, ascending by `NodeKey`, the
    /// same order `TypeEnvironment::new` admitted.
    composites: Vec<CompositeDeclaration>,
    declaration_keys: Vec<CheckedNodeId>,
}

/// Why [`check_item`] produced no [`CheckedItem`]: a per-item refusal, or an
/// RT `#[non_exhaustive]` enum yielding a variant this generator does not
/// know, which aborts the whole generation.
enum ItemCheckError {
    Refusal(CompositeEqualityRefusal),
    Generation(OracleGenerationError),
}

impl From<CompositeEqualityRefusal> for ItemCheckError {
    fn from(refusal: CompositeEqualityRefusal) -> Self {
        Self::Refusal(refusal)
    }
}

impl From<OracleGenerationError> for ItemCheckError {
    fn from(error: OracleGenerationError) -> Self {
        Self::Generation(error)
    }
}

fn check_item<'r>(
    graph: &Graph<'_>,
    bounds_by_type: &BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2>>,
    record: &'r CompleteLoweringRecordV2,
    item: &CompositeEqualityItem,
) -> Result<CheckedItem<'r>, ItemCheckError> {
    let node = lowered(record)?;
    if node.node_tag != CheckedNodeTag::Expression {
        return Err(CompositeEqualityRefusal::NotExpression {
            node_tag: node.node_tag.as_wire(),
        }
        .into());
    }
    if &*node.node.semantic_form == "call" {
        return Err(CompositeEqualityRefusal::BlockedOnUpstream {
            unsupported_node_id: node.node.node_id.clone(),
            node_tag: "expression.call",
            issue: UpstreamBlocker::QuireContractRuntime34,
        }
        .into());
    }
    if &*node.node.semantic_form != "binary" {
        return Err(CompositeEqualityRefusal::FormMismatch {
            found: node.node.semantic_form.to_string(),
        }
        .into());
    }
    let arguments = application_arguments(&node.node.body)
        .filter(|arguments| arguments.len() == 2)
        .ok_or(CompositeEqualityRefusal::BodyMismatch)?;
    check_operand_types(graph, arguments, item)?;

    let mut closure = TypeClosure::default();
    let left_source = resolve_type(graph, bounds_by_type, &mut closure, &item.left.source_type)?;
    let left_target = item
        .left
        .conversion_target
        .as_ref()
        .map(|target| resolve_type(graph, bounds_by_type, &mut closure, target))
        .transpose()?;
    let right_source = resolve_type(graph, bounds_by_type, &mut closure, &item.right.source_type)?;
    let right_target = item
        .right
        .conversion_target
        .as_ref()
        .map(|target| resolve_type(graph, bounds_by_type, &mut closure, target))
        .transpose()?;

    let declaration_keys = closure.node_ids.clone();
    let composites: Vec<CompositeDeclaration> = closure.composites.values().cloned().collect();
    let environment = match TypeEnvironment::new(composites.clone(), std::iter::empty()) {
        Ok(environment) => environment,
        Err(invalid) => {
            return Err(CompositeEqualityRefusal::Declaration {
                declaration: invalid.declaration,
                cause: invalid.cause.try_into()?,
            }
            .into())
        }
    };

    let left_operand = match left_target.clone() {
        Some(target) => EqualityOperand::converted(left_source.clone(), target),
        None => EqualityOperand::typed(left_source.clone()),
    };
    let right_operand = match right_target.clone() {
        Some(target) => EqualityOperand::converted(right_source.clone(), target),
        None => EqualityOperand::typed(right_source.clone()),
    };
    let checked =
        match environment.check_equality(item.operator.to_runtime(), left_operand, right_operand) {
            Ok(checked) => checked,
            Err(ill_typed) => {
                return Err(CompositeEqualityRefusal::IllTyped {
                    cause: ill_typed.cause.try_into()?,
                }
                .into())
            }
        };

    Ok(CheckedItem {
        node,
        checked,
        left_source,
        left_target,
        right_source,
        right_target,
        composites,
        declaration_keys,
    })
}

fn lowered(
    record: &CompleteLoweringRecordV2,
) -> Result<&quire_contract_model::CompleteContractNodeV2, CompositeEqualityRefusal> {
    match record {
        CompleteLoweringRecordV2::Lowered { node } => Ok(node),
        CompleteLoweringRecordV2::Unsupported {
            unsupported_node_id,
            node_tag,
            ..
        } => Err(unsupported_family(unsupported_node_id, *node_tag)),
        CompleteLoweringRecordV2::RequiresBound { unbounded_type, .. } => {
            Err(CompositeEqualityRefusal::MissingBound {
                bounded_type: unbounded_type.clone(),
                expected_form: "bound",
            })
        }
        CompleteLoweringRecordV2::InvalidInput { .. } => {
            Err(CompositeEqualityRefusal::InvalidInput)
        }
        CompleteLoweringRecordV2::InvalidBody { body_node_id, .. } => {
            Err(CompositeEqualityRefusal::InvalidBody {
                body_node_id: body_node_id.clone(),
            })
        }
        CompleteLoweringRecordV2::BodyIncomplete { body_node_id, .. } => {
            Err(CompositeEqualityRefusal::BodyIncomplete {
                body_node_id: body_node_id.clone(),
            })
        }
        // FR-014-AC-42 keeps this arm from binding the record's fields, so the typed outcome is
        // read back through the classifier, which returns `None` for a record that is not
        // `Failed`. This arm only runs for a `Failed` record, so the `InvalidInput` fallback is
        // unreachable; it exists because the arm cannot hand the classifier a narrower type.
        CompleteLoweringRecordV2::Failed { .. } => Err(classify_lowering_failure(record).map_or(
            CompositeEqualityRefusal::InvalidInput,
            CompositeEqualityRefusal::from,
        )),
    }
}

impl From<LoweringFailure> for CompositeEqualityRefusal {
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

fn unsupported_family(node_id: &CheckedNodeId, tag: CheckedNodeTag) -> CompositeEqualityRefusal {
    let blocked = |issue| CompositeEqualityRefusal::BlockedOnUpstream {
        unsupported_node_id: node_id.clone(),
        node_tag: tag.as_wire(),
        issue,
    };
    match tag {
        CheckedNodeTag::Model | CheckedNodeTag::Relation => {
            blocked(UpstreamBlocker::QuireSpecLanguage120)
        }
        CheckedNodeTag::Function => blocked(UpstreamBlocker::QuireContractRuntime34),
        CheckedNodeTag::State | CheckedNodeTag::Temporal | CheckedNodeTag::Protocol => {
            blocked(UpstreamBlocker::QuireSpecLanguage121)
        }
        CheckedNodeTag::ScalarType
        | CheckedNodeTag::CompositeType
        | CheckedNodeTag::BoundedDomain
        | CheckedNodeTag::Value
        | CheckedNodeTag::Expression
        | CheckedNodeTag::Claim
        | CheckedNodeTag::Correspondence => CompositeEqualityRefusal::Unsupported {
            unsupported_node_id: node_id.clone(),
            node_tag: tag.as_wire(),
        },
    }
}

fn application_arguments(body: &Value) -> Option<&Vec<Value>> {
    if body.get("term")?.as_str()? != "application" || body.get("operator")?.as_str()? != "binary" {
        return None;
    }
    body.get("arguments")?.as_array()
}

/// The type a `binary` operand stands for, read as QSL emits operands: each operand is its own
/// node, and the equality's `arguments` hold a `reference` to it (FR-018). The operand's type is
/// that node's `semantic_type`, except that a node that is an `expression` whose body is an
/// `application` of operator `convert` is a conversion, and the operand is read through it: the
/// type is that of the conversion's first argument, followed through nested conversions to the
/// first node that is not one, which is the descriptor's `source_type`. The conversion's own
/// `result_type` is not read: whether the descriptor's conversion is admitted is the runtime's
/// `check_equality` verdict on the descriptor, never the body's. An inline term (a `literal`, an
/// inline `application`) is not an operand QSL emits and reads as no type.
///
/// This is the generator's only read of body content (FR-018's Behavior clause "disagrees with
/// its descriptor's arity or operand types"): the type used to build the runtime call always
/// comes from the descriptor, never from this value, so disagreement here is refused before
/// either operand's type is resolved.
fn operand_type_id(graph: &Graph<'_>, term: &Value) -> Option<CheckedNodeId> {
    let mut term = term;
    // Each step follows one reference to a distinct-or-repeated node; the graph's size bounds a
    // chain that does not cycle, and a cycle of conversions reads as no type.
    for _ in 0..=graph.len() {
        let target = read_reference(term)?;
        let node = graph.get(&target)?;
        let conversion = (&*node.node_tag == CheckedNodeTag::Expression.as_wire())
            .then(|| application_of(&node.body, "convert"))
            .flatten();
        match conversion {
            Some(arguments) => term = arguments.first()?,
            None => return Some(node.semantic_type.clone()),
        }
    }
    None
}

/// The `arguments` of a node body that is an `application` of `operator`.
fn application_of<'b>(body: &'b Value, operator: &str) -> Option<&'b Vec<Value>> {
    if body.get("term")?.as_str()? != "application" || body.get("operator")?.as_str()? != operator {
        return None;
    }
    body.get("arguments")?.as_array()
}

/// Compare each body operand's declared type against the descriptor's for
/// that position, left then right, refusing at the first disagreement.
fn check_operand_types(
    graph: &Graph<'_>,
    arguments: &[Value],
    item: &CompositeEqualityItem,
) -> Result<(), CompositeEqualityRefusal> {
    let expected = [&item.left.source_type, &item.right.source_type];
    for (position, (argument, expected_type)) in arguments.iter().zip(expected).enumerate() {
        let found = operand_type_id(graph, argument);
        if found.as_ref() != Some(expected_type) {
            return Err(CompositeEqualityRefusal::OperandTypeMismatch {
                position,
                expected: expected_type.clone(),
                found,
            });
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Type and declaration-closure reconstruction
// ---------------------------------------------------------------------------

#[derive(Default)]
struct TypeClosure {
    composites: BTreeMap<NodeKey, CompositeDeclaration>,
    in_progress: BTreeSet<NodeKey>,
    work_consumed: u64,
    /// Every V2 node id whose declaration entered the closure, ascending.
    node_ids: Vec<CheckedNodeId>,
}

fn node_key(node_id: &CheckedNodeId) -> Result<NodeKey, CompositeEqualityRefusal> {
    NodeKey::from_hex(&node_id.digest).ok_or_else(|| CompositeEqualityRefusal::UnknownTypeNode {
        type_node_id: node_id.clone(),
    })
}

fn lookup<'g>(
    graph: &Graph<'g>,
    node_id: &CheckedNodeId,
) -> Result<&'g CheckedSemanticNodeV2, CompositeEqualityRefusal> {
    graph
        .get(node_id)
        .copied()
        .ok_or_else(|| CompositeEqualityRefusal::UnknownTypeNode {
            type_node_id: node_id.clone(),
        })
}

fn read_reference(term: &Value) -> Option<CheckedNodeId> {
    if term.get("term")?.as_str()? != "reference" {
        return None;
    }
    serde_json::from_value(term.get("target")?.clone()).ok()
}

fn read_binding(term: &Value) -> Option<(String, CheckedNodeId)> {
    if term.get("term")?.as_str()? != "binding" {
        return None;
    }
    let name = term.get("name")?.as_str()?.to_owned();
    let target = read_reference(term.get("value")?)?;
    Some((name, target))
}

/// A record field has either a direct type reference or the checked optional wrapper.
fn read_record_field(term: &Value) -> Option<(String, CheckedNodeId, bool)> {
    if term.get("term")?.as_str()? != "binding" {
        return None;
    }
    let name = term.get("name")?.as_str()?.to_owned();
    let value = term.get("value")?;
    if let Some(target) = read_reference(value) {
        return Some((name, target, false));
    }
    let [optional] = aggregate_members(value)? else {
        return None;
    };
    let (wrapper_name, target) = read_binding(optional)?;
    (wrapper_name == "optional").then_some((name, target, true))
}

fn is_option_node(node: &CheckedSemanticNodeV2) -> bool {
    CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::CompositeType)
        && &*node.semantic_form == "option"
}

/// Resolve `type_id` to a `ValueType`, registering every reachable
/// record/tuple declaration into `closure`.
fn resolve_type(
    graph: &Graph<'_>,
    bounds_by_type: &BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2>>,
    closure: &mut TypeClosure,
    type_id: &CheckedNodeId,
) -> Result<ValueType, CompositeEqualityRefusal> {
    resolution::resolve_type(graph, bounds_by_type, closure, type_id)
}

/// Resolve a scalar type: `node` is the base `scalar_type` and `bounds` the `bounded_domain`
/// nodes that bound it for this reference (every one over the scalar, or the one the member
/// names); `type_id` is the member's type node, for refusals.
fn resolve_scalar(
    bounds: &[&CheckedSemanticNodeV2],
    type_id: &CheckedNodeId,
    node: &CheckedSemanticNodeV2,
) -> Result<ValueType, CompositeEqualityRefusal> {
    let bound = |form: &'static str| -> Result<&[Value], CompositeEqualityRefusal> {
        let candidates: Vec<_> = bounds
            .iter()
            .filter(|bound| &*bound.semantic_form == form)
            .collect();
        match candidates.as_slice() {
            [bound] => aggregate_members(&bound.body).ok_or_else(|| {
                CompositeEqualityRefusal::UnreadableBound {
                    bound: bound.node_id.clone(),
                }
            }),
            [] => Err(CompositeEqualityRefusal::MissingBound {
                bounded_type: type_id.clone(),
                expected_form: form,
            }),
            _ => Err(CompositeEqualityRefusal::AmbiguousBound {
                bounded_type: type_id.clone(),
                expected_form: form,
            }),
        }
    };
    match &*node.semantic_form {
        "boolean" => Ok(ValueType::Boolean),
        "integer" => {
            let candidates: Vec<_> = bounds
                .iter()
                .filter(|bound| &*bound.semantic_form == "integer_range")
                .collect();
            match candidates.as_slice() {
                [] => Ok(ValueType::Integer),
                [bound] => {
                    let members = aggregate_members(&bound.body).ok_or_else(|| {
                        CompositeEqualityRefusal::UnreadableBound {
                            bound: bound.node_id.clone(),
                        }
                    })?;
                    let interval = read_integer_range(members).ok_or_else(|| {
                        CompositeEqualityRefusal::UnreadableBound {
                            bound: bound.node_id.clone(),
                        }
                    })?;
                    Ok(ValueType::Int(interval))
                }
                _ => Err(CompositeEqualityRefusal::AmbiguousBound {
                    bounded_type: type_id.clone(),
                    expected_form: "integer_range",
                }),
            }
        }
        "rational" => {
            let members = bound("rational_range")?;
            let domain = read_rational_range(members).ok_or_else(|| {
                CompositeEqualityRefusal::UnreadableBound {
                    bound: type_id.clone(),
                }
            })?;
            Ok(ValueType::Rational(domain))
        }
        "decimal" => {
            let members = bound("decimal_range")?;
            let decimal = read_decimal_range(members).ok_or_else(|| {
                CompositeEqualityRefusal::UnreadableBound {
                    bound: type_id.clone(),
                }
            })?;
            Ok(ValueType::Decimal(decimal))
        }
        "float32" => Ok(ValueType::Float(rt::IeeeWidth::Binary32)),
        "float64" => Ok(ValueType::Float(rt::IeeeWidth::Binary64)),
        "text" => {
            let members = bound("text_bounds")?;
            let text_type = read_text_bounds(members).ok_or_else(|| {
                CompositeEqualityRefusal::UnreadableBound {
                    bound: type_id.clone(),
                }
            })?;
            Ok(ValueType::Text(text_type))
        }
        "enum" => Ok(ValueType::Enum(node_key(type_id)?)),
        other => Err(CompositeEqualityRefusal::Unsupported {
            unsupported_node_id: type_id.clone(),
            node_tag: match other {
                "unit" | "dimension" => "quantity",
                _ => "scalar_type",
            },
        }),
    }
}

fn hex_digest(key: &NodeKey) -> String {
    key.as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

const SOURCE_HEADER: &str = "\
// @generated by quire-contract-codegen composite equality oracles. Do not edit.
//
// Every oracle function calls `TypeEnvironment::check_type` on each operand's
// comparison type, then `check_equality`, then `CheckedEquality::evaluate`
// with the caller's `Meter`; an `IllTyped` from either check becomes
// `Outcome::Refused(Refusal::CheckedInvariant)` before any charge. No charge
// amount and no planned pair count appear in this source: every charge and
// every pair comes from runtime metering. The source has no inner attributes
// so that it can be `include!`d; the manifest forbids unsafe code instead.

use quire_contract_runtime::exact as rt;

/// Why an emitted bound could not be rebuilt from its generation-time spelling. Each variant
/// names the `rebuild_*` helper that failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReconstructionError {
    /// `rebuild_integer` could not parse a canonical integer literal.
    Integer,
    /// `rebuild_interval` was given an empty interval.
    Interval,
    /// `rebuild_rational` was given a domain the runtime refuses.
    Rational,
    /// `rebuild_decimal` was given a decimal type the runtime refuses.
    Decimal,
    /// `rebuild_text` was given a text type the runtime refuses.
    Text,
    /// `rebuild_cardinality` was given an empty cardinality bound.
    Cardinality,
}

/// Why an environment constructor returned no environment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnvironmentError {
    /// The runtime refused the declaration closure.
    Declaration(rt::InvalidDeclaration),
    /// A bound of a declaration did not reconstruct.
    Reconstruction(ReconstructionError),
}
";

/// One emitted reconstruction helper: its name, its source, and the names of the helpers its
/// source calls. A helper is emitted only when the rendered functions call it, or when an
/// emitted helper does: an unused one would be dead code under the generated crate's own
/// `-D warnings` build. A runtime constructor, or an integer parse, appears only in one of these.
struct ReconstructionHelper {
    name: &'static str,
    source: &'static str,
    calls: &'static [&'static str],
}

/// The reconstruction helpers, callees before callers.
const RECONSTRUCTION_HELPERS: &[ReconstructionHelper] = &[
    ReconstructionHelper {
        name: "rebuild_integer",
        source: "\n\
/// A canonical decimal integer literal, re-parsed from generation-time output.
fn rebuild_integer(spelling: &str) -> Result<rt::Integer, ReconstructionError> {
    spelling.parse().map_err(|_| ReconstructionError::Integer)
}
",
        calls: &[],
    },
    ReconstructionHelper {
        name: "rebuild_interval",
        source: "\n\
/// An integer interval over two canonical integer literals.
fn rebuild_interval(lower: &str, upper: &str) -> Result<rt::IntegerInterval, ReconstructionError> {
    rt::IntegerInterval::new(rebuild_integer(lower)?, rebuild_integer(upper)?)
        .map_err(|_| ReconstructionError::Interval)
}
",
        calls: &["rebuild_integer"],
    },
    ReconstructionHelper {
        name: "rebuild_rational",
        source: "\n\
/// A rational domain over a numerator and a denominator interval.
fn rebuild_rational(
    numerator: rt::IntegerInterval,
    denominator: rt::IntegerInterval,
) -> Result<rt::RationalDomain, ReconstructionError> {
    rt::RationalDomain::new(numerator, denominator).map_err(|_| ReconstructionError::Rational)
}
",
        calls: &[],
    },
    ReconstructionHelper {
        name: "rebuild_decimal",
        source: "\n\
/// A decimal type over two canonical integer literals, a scale range and a rounding mode.
fn rebuild_decimal(
    lower: &str,
    upper: &str,
    min_scale: u64,
    max_scale: u64,
    rounding: rt::RoundingMode,
) -> Result<rt::DecimalType, ReconstructionError> {
    rt::DecimalType::new(
        rebuild_integer(lower)?,
        rebuild_integer(upper)?,
        min_scale,
        max_scale,
        rounding,
    )
    .map_err(|_| ReconstructionError::Decimal)
}
",
        calls: &["rebuild_integer"],
    },
    ReconstructionHelper {
        name: "rebuild_text",
        source: "\n\
/// A text type over a length range and a profile.
fn rebuild_text(
    min: u64,
    max: u64,
    profile: rt::TextProfile,
) -> Result<rt::TextType, ReconstructionError> {
    rt::TextType::new(min, max, profile).map_err(|_| ReconstructionError::Text)
}
",
        calls: &[],
    },
    ReconstructionHelper {
        name: "rebuild_cardinality",
        source: "\n\
/// A collection cardinality bound.
fn rebuild_cardinality(
    minimum: u64,
    maximum: u64,
) -> Result<rt::CardinalityBound, ReconstructionError> {
    rt::CardinalityBound::new(minimum, maximum).map_err(|_| ReconstructionError::Cardinality)
}
",
        calls: &[],
    },
];

/// Why one item could not be rendered. Private: the public failure type is
/// [`OracleGenerationError`], which the item boundary maps this onto.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RenderError {
    /// A `ValueType` family this generator never reconstructs: the item is refused as
    /// [`CompositeEqualityRefusal::Unsupported`] naming the family.
    UnsupportedValueType {
        /// The family, as the refusal's `node_tag`.
        family: &'static str,
    },
    /// A whole-call failure, an unknown runtime variant, that renders no item.
    Generation(OracleGenerationError),
}

impl From<OracleGenerationError> for RenderError {
    fn from(error: OracleGenerationError) -> Self {
        Self::Generation(error)
    }
}

/// The rendered Rust expressions of one item, before it is named.
struct RenderedItem {
    composites: String,
    left_source: String,
    left_target: Option<String>,
    right_source: String,
    right_target: Option<String>,
}

fn render_item(generated: &CheckedItem<'_>) -> Result<RenderedItem, RenderError> {
    // Every reachable record/tuple declaration, in the same `NodeKey`
    // order `TypeEnvironment::new` admitted at generation time.
    let composites: String = generated
        .composites
        .iter()
        .map(|declaration| {
            render_composite_declaration(declaration)
                .map(|rendered| format!("{rendered},\n        "))
        })
        .collect::<Result<_, _>>()?;
    let target = |target: &Option<ValueType>| -> Result<Option<String>, RenderError> {
        target.as_ref().map(render_value_type).transpose()
    };
    Ok(RenderedItem {
        composites,
        left_source: render_value_type(&generated.left_source)?,
        left_target: target(&generated.left_target)?,
        right_source: render_value_type(&generated.right_source)?,
        right_target: target(&generated.right_target)?,
    })
}

/// Render every pending item, then name and emit the ones that rendered. `pending` holds, per
/// item, the position of its claim, its descriptor key, the request and what `render` reads.
///
/// Every item is rendered before any is named, so an item the render refuses never takes a number
/// from a sibling: oracles sharing an operator stem are numbered in key order among the items that
/// rendered. An unrenderable item becomes a per-item refusal on its own claim
/// ([`settle_render`]); a whole-call failure aborts.
fn render_and_emit<T>(
    claims: &mut [CompositeEqualityClaim],
    source: &mut SourceBuilder,
    pending: Vec<(usize, DescriptorKey, &CompositeEqualityItem, T)>,
    render: impl Fn(&T) -> Result<RenderedItem, RenderError>,
) -> Result<(), OracleGenerationError> {
    let mut rendered = Vec::with_capacity(pending.len());
    for (claim, key, item, checked) in pending {
        let result = render(&checked);
        if let Some(rendered_item) = settle_render(&mut claims[claim], result)? {
            rendered.push((claim, key, item, rendered_item));
        }
    }
    let names = unique_names(
        rendered
            .iter()
            .map(|(_, key, item, _)| (item.operator.identity().replace('.', "_"), key.clone()))
            .collect(),
    );
    for ((claim, _, item, rendered_item), symbol) in rendered.into_iter().zip(names) {
        source.item(&symbol, item, &rendered_item);
        if let ClaimDisposition::Generated(claim) = &mut claims[claim].result {
            claim.environment_symbol = format!("environment_{symbol}");
            claim.oracle_symbol = format!("oracle_{symbol}");
        }
    }
    Ok(())
}

/// Settle one rendered item against its claim, at the item boundary. A render that failed with
/// [`RenderError::UnsupportedValueType`] turns the item's claim into the per-item refusal
/// [`CompositeEqualityRefusal::Unsupported`], whose `node_tag` is the family and whose
/// `unsupported_node_id` is the item's expression node (a `ValueType` carries no node id, and the
/// expression node is the one every call site knows), and renders nothing; any sibling is
/// untouched. A [`RenderError::Generation`] still fails the whole call.
fn settle_render(
    claim: &mut CompositeEqualityClaim,
    rendered: Result<RenderedItem, RenderError>,
) -> Result<Option<RenderedItem>, OracleGenerationError> {
    match rendered {
        Ok(rendered) => Ok(Some(rendered)),
        Err(RenderError::UnsupportedValueType { family }) => {
            claim.result = ClaimDisposition::Refused {
                refusal: CompositeEqualityRefusal::Unsupported {
                    unsupported_node_id: claim.node_id.clone(),
                    node_tag: family,
                },
            };
            Ok(None)
        }
        Err(RenderError::Generation(error)) => Err(error),
    }
}

#[derive(Default)]
struct SourceBuilder {
    functions: String,
}

impl SourceBuilder {
    fn item(&mut self, symbol: &str, item: &CompositeEqualityItem, rendered: &RenderedItem) {
        let RenderedItem {
            composites,
            left_source,
            left_target,
            right_source,
            right_target,
        } = rendered;
        let optional = |target: &Option<String>| match target {
            Some(target) => format!("Some({target})"),
            None => "None".to_owned(),
        };
        let left_target = optional(left_target);
        let right_target = optional(right_target);

        self.functions.push_str(&format!(
            "\nfn composites_{symbol}() -> Result<Vec<rt::CompositeDeclaration>, ReconstructionError> {{\n    Ok(vec![{composites}])\n}}\n"
        ));

        self.functions.push_str(&format!(
            "\nfn left_source_{symbol}() -> Result<rt::ValueType, ReconstructionError> {{\n    Ok({left_source})\n}}\n\
             fn left_target_{symbol}() -> Result<Option<rt::ValueType>, ReconstructionError> {{\n    Ok({left_target})\n}}\n\
             fn right_source_{symbol}() -> Result<rt::ValueType, ReconstructionError> {{\n    Ok({right_source})\n}}\n\
             fn right_target_{symbol}() -> Result<Option<rt::ValueType>, ReconstructionError> {{\n    Ok({right_target})\n}}\n"
        ));

        self.functions.push_str(&format!(
            "\n/// `{}`. Its operation identity is caller-declared: CheckedPackage V2 carries\n\
             /// an operator class, not this operator law.\n\
             pub fn environment_{symbol}() -> Result<rt::TypeEnvironment, EnvironmentError> {{\n\
             \x20   let composites = match composites_{symbol}() {{\n\
             \x20       Ok(composites) => composites,\n\
             \x20       Err(error) => return Err(EnvironmentError::Reconstruction(error)),\n\x20   }};\n\
             \x20   rt::TypeEnvironment::new(composites, core::iter::empty::<rt::ObjectTypeDeclaration>())\n\
             \x20       .map_err(EnvironmentError::Declaration)\n}}\n",
            item.operator.identity()
        ));

        self.functions.push_str(&format!(
            "\n/// Environment-checked oracle for `{}`.\npub fn oracle_{symbol}(\n    environment: &rt::TypeEnvironment,\n    left: &rt::Value,\n    right: &rt::Value,\n    meter: &mut rt::Meter,\n) -> rt::Outcome<bool> {{\n\
             \x20   let left_source = match left_source_{symbol}() {{\n\
             \x20       Ok(value_type) => value_type,\n\
             \x20       Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),\n\x20   }};\n\
             \x20   let left_target = match left_target_{symbol}() {{\n\
             \x20       Ok(value_type) => value_type,\n\
             \x20       Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),\n\x20   }};\n\
             \x20   let left_comparison = match &left_target {{\n\
             \x20       Some(target) => target.clone(),\n\
             \x20       None => left_source.clone(),\n\x20   }};\n\
             \x20   if environment.check_type(&left_comparison).is_err() {{\n\
             \x20       return rt::Outcome::Refused(rt::Refusal::CheckedInvariant);\n\x20   }}\n\
             \x20   let right_source = match right_source_{symbol}() {{\n\
             \x20       Ok(value_type) => value_type,\n\
             \x20       Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),\n\x20   }};\n\
             \x20   let right_target = match right_target_{symbol}() {{\n\
             \x20       Ok(value_type) => value_type,\n\
             \x20       Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),\n\x20   }};\n\
             \x20   let right_comparison = match &right_target {{\n\
             \x20       Some(target) => target.clone(),\n\
             \x20       None => right_source.clone(),\n\x20   }};\n\
             \x20   if environment.check_type(&right_comparison).is_err() {{\n\
             \x20       return rt::Outcome::Refused(rt::Refusal::CheckedInvariant);\n\x20   }}\n\
             \x20   let left_operand = match left_target {{\n\
             \x20       Some(target) => rt::EqualityOperand::converted(left_source, target),\n\
             \x20       None => rt::EqualityOperand::typed(left_source),\n\x20   }};\n\
             \x20   let right_operand = match right_target {{\n\
             \x20       Some(target) => rt::EqualityOperand::converted(right_source, target),\n\
             \x20       None => rt::EqualityOperand::typed(right_source),\n\x20   }};\n\
             \x20   let checked = match environment.check_equality({}, left_operand, right_operand) {{\n\
             \x20       Ok(checked) => checked,\n\
             \x20       Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),\n\x20   }};\n\
             \x20   checked.evaluate(left, right, meter)\n}}\n",
            item.operator.identity(),
            item.operator.path()
        ));
    }

    fn finish(self) -> String {
        let mut source = SOURCE_HEADER.to_owned();
        // A helper is needed when the item functions call it or a needed helper does. The table
        // lists callees first, so one pass from the last helper to the first settles the
        // transitive closure.
        let mut needed: BTreeSet<&str> = BTreeSet::new();
        for helper in RECONSTRUCTION_HELPERS.iter().rev() {
            if self.functions.contains(&format!("{}(", helper.name)) || needed.contains(helper.name)
            {
                needed.insert(helper.name);
                needed.extend(helper.calls);
            }
        }
        for helper in RECONSTRUCTION_HELPERS {
            if needed.contains(helper.name) {
                source.push_str(helper.source);
            }
        }
        source.push_str(&self.functions);
        source
    }
}

/// Render one admitted `CompositeDeclaration` as a Rust expression of type
/// `rt::CompositeDeclaration`.
fn render_composite_declaration(declaration: &CompositeDeclaration) -> Result<String, RenderError> {
    let key = render_key(declaration.key());
    let shape = match declaration.shape() {
        CompositeShape::Record(fields) => {
            let rendered: String = fields
                .iter()
                .map(|field| {
                    Ok(format!(
                        "rt::FieldDeclaration::new({:?}, {}, {}), ",
                        field.name(),
                        render_value_type(field.value_type())?,
                        presence_path(field.presence())
                    ))
                })
                .collect::<Result<_, RenderError>>()?;
            format!("rt::CompositeShape::Record(vec![{rendered}])")
        }
        CompositeShape::Tuple(positions) => {
            let rendered: String = positions
                .iter()
                .map(|value_type| Ok(format!("{}, ", render_value_type(value_type)?)))
                .collect::<Result<_, RenderError>>()?;
            format!("rt::CompositeShape::Tuple(vec![{rendered}])")
        }
        &_ => return Err(OracleGenerationError::unknown_variant("CompositeShape").into()),
    };
    Ok(format!(
        "rt::CompositeDeclaration::new({key}, {:?}, {shape})",
        declaration.name()
    ))
}

fn presence_path(presence: Presence) -> &'static str {
    match presence {
        Presence::Required => "rt::Presence::Required",
        Presence::Optional => "rt::Presence::Optional",
    }
}

/// Render one `ValueType` as a Rust expression of type `rt::ValueType`.
///
/// Every bound is rebuilt through an emitted `rebuild_*` helper whose `Result` the expression
/// propagates with `?`, so the expression belongs in a function returning
/// `Result<_, ReconstructionError>`.
fn render_value_type(value_type: &ValueType) -> Result<String, RenderError> {
    enum Task<'a> {
        Value(&'a ValueType),
        CloseOption,
        CloseCollection { minimum: u64, maximum: u64 },
    }

    let mut rendered = String::new();
    let mut tasks = vec![Task::Value(value_type)];
    while let Some(task) = tasks.pop() {
        match task {
            Task::CloseOption => rendered.push(')'),
            Task::CloseCollection { minimum, maximum } => {
                rendered.push_str(&format!(", rebuild_cardinality({minimum}, {maximum})?))"))
            }
            Task::Value(value_type) => match value_type {
                ValueType::Boolean => rendered.push_str("rt::ValueType::Boolean"),
                ValueType::Integer => rendered.push_str("rt::ValueType::Integer"),
                ValueType::Int(interval) => rendered.push_str(&format!(
                    "rt::ValueType::Int({})",
                    render_interval(interval)
                )),
                ValueType::Rational(domain) => rendered.push_str(&format!(
                    "rt::ValueType::Rational(rebuild_rational({}, {})?)",
                    render_interval(domain.numerator()),
                    render_interval(domain.denominator())
                )),
                ValueType::Decimal(decimal) => rendered.push_str(&format!(
                    "rt::ValueType::Decimal({})",
                    render_decimal_type(decimal)?
                )),
                ValueType::Float(rt::IeeeWidth::Binary32) => {
                    rendered.push_str("rt::ValueType::Float(rt::IeeeWidth::Binary32)");
                }
                ValueType::Float(rt::IeeeWidth::Binary64) => {
                    rendered.push_str("rt::ValueType::Float(rt::IeeeWidth::Binary64)");
                }
                ValueType::Quantity(_) => {
                    return Err(RenderError::UnsupportedValueType { family: "quantity" });
                }
                ValueType::Text(text_type) => rendered.push_str(&format!(
                    "rt::ValueType::Text(rebuild_text({}, {}, {})?)",
                    text_type.min(),
                    text_type.max(),
                    profile_path(text_type.profile())?
                )),
                ValueType::Enum(key) => {
                    rendered.push_str(&format!("rt::ValueType::Enum({})", render_key(*key)));
                }
                ValueType::Option(payload) => {
                    rendered.push_str("rt::ValueType::option(");
                    tasks.push(Task::CloseOption);
                    tasks.push(Task::Value(payload));
                }
                ValueType::Composite(key) => {
                    rendered.push_str(&format!("rt::ValueType::Composite({})", render_key(*key)));
                }
                ValueType::Collection(collection) => {
                    rendered.push_str(&format!(
                        "rt::ValueType::collection(rt::CollectionType::new({}, ",
                        collection_kind_path(collection.kind())?
                    ));
                    tasks.push(Task::CloseCollection {
                        minimum: collection.bound().minimum(),
                        maximum: collection.bound().maximum(),
                    });
                    tasks.push(Task::Value(collection.element()));
                }
                ValueType::Reference(_) => {
                    return Err(RenderError::UnsupportedValueType {
                        family: "reference",
                    });
                }
                &_ => return Err(OracleGenerationError::unknown_variant("ValueType").into()),
            },
        }
    }
    Ok(rendered)
}

fn render_interval(interval: &IntegerInterval) -> String {
    format!(
        "rebuild_interval(\"{}\", \"{}\")?",
        interval.lower(),
        interval.upper()
    )
}

fn render_decimal_type(decimal: &DecimalType) -> Result<String, OracleGenerationError> {
    Ok(format!(
        "rebuild_decimal(\"{}\", \"{}\", {}, {}, {})?",
        decimal.lower(),
        decimal.upper(),
        decimal.min_scale(),
        decimal.max_scale(),
        rounding_path(decimal.rounding())?
    ))
}

fn render_key(key: NodeKey) -> String {
    let bytes = key
        .as_bytes()
        .iter()
        .map(|byte| byte.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    format!("rt::NodeKey::from_bytes([{bytes}])")
}

fn collection_kind_path(kind: CollectionKind) -> Result<&'static str, OracleGenerationError> {
    match kind {
        CollectionKind::Sequence => Ok("rt::CollectionKind::Sequence"),
        CollectionKind::Set => Ok("rt::CollectionKind::Set"),
        CollectionKind::Bag => Ok("rt::CollectionKind::Bag"),
        CollectionKind::OrderedSet => Ok("rt::CollectionKind::OrderedSet"),
        _ => Err(OracleGenerationError::unknown_variant("CollectionKind")),
    }
}

fn rounding_path(rounding: RoundingMode) -> Result<&'static str, OracleGenerationError> {
    match rounding {
        RoundingMode::Exact => Ok("rt::RoundingMode::Exact"),
        RoundingMode::TowardZero => Ok("rt::RoundingMode::TowardZero"),
        RoundingMode::TowardPositive => Ok("rt::RoundingMode::TowardPositive"),
        RoundingMode::TowardNegative => Ok("rt::RoundingMode::TowardNegative"),
        RoundingMode::NearestEven => Ok("rt::RoundingMode::NearestEven"),
        RoundingMode::NearestAway => Ok("rt::RoundingMode::NearestAway"),
        _ => Err(OracleGenerationError::unknown_variant("RoundingMode")),
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
        _ => Err(OracleGenerationError::unknown_variant("TextProfile")),
    }
}

fn artifact(path: &str, contents: String) -> Artifact {
    Artifact::new(path, contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn recursive_shape_nodes() -> Vec<CheckedSemanticNodeV2> {
        let reference = |digit| json!({"term": "reference", "target": node_id(digit)});
        let node = |digit: char, tag: &str, form: &str, semantic_type: char, body: Value| {
            serde_json::from_value(json!({
                "node_id": node_id(digit),
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": tag,
                "semantic_form": form,
                "semantic_type": node_id(semantic_type),
                "dependencies": [],
                "occurrences": [],
                "body": body,
            }))
            .expect("synthetic shape")
        };
        vec![
            node(
                'a',
                "composite_type",
                "record",
                'a',
                json!({"term": "aggregate", "members": [{
                    "term": "binding", "name": "next", "value": {"term": "aggregate", "members": [{
                        "term": "binding", "name": "optional", "value": reference('b')
                    }]}
                }]}),
            ),
            node(
                'b',
                "composite_type",
                "option",
                'b',
                json!({"term": "aggregate", "members": [reference('a')]}),
            ),
            node(
                'c',
                "composite_type",
                "record",
                'c',
                json!({"term": "aggregate", "members": [{
                    "term": "binding", "name": "kids", "value": reference('e')
                }]}),
            ),
            node(
                'd',
                "composite_type",
                "sequence",
                'd',
                json!({"term": "aggregate", "members": [reference('c')]}),
            ),
            node(
                'e',
                "bounded_domain",
                "collection_bounds",
                'd',
                json!({"term": "aggregate", "members": [
                    {"term": "binding", "name": "min", "value": {"term": "literal", "type": node_id('f'), "value_kind": "integer", "value": "0"}},
                    {"term": "binding", "name": "max", "value": {"term": "literal", "type": node_id('f'), "value_kind": "integer", "value": "3"}}
                ]}),
            ),
            node(
                'f',
                "scalar_type",
                "integer",
                'f',
                json!({"term": "aggregate", "members": []}),
            ),
        ]
    }

    /// Compile the recursive QSpec forms through QSL's public facade and read its package.
    fn qsl_recursive_package() -> CheckedPackageV2 {
        use qsl_replay::{
            compile_package, DependencyInput, ScalarLimits, SourceIdentity, StageLimits,
        };
        use quire_contract_model::{
            CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2ReadResult,
        };

        let source = include_bytes!("../../../tests/composite_equality_support/recursive.native");
        let unbounded = ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        };
        let compiled = compile_package(
            SourceIdentity::new("a", "u", "git", "1"),
            "recursive.native",
            source,
            [],
            &DependencyInput::default(),
            StageLimits {
                s1: ScalarLimits {
                    text_input_bytes: 1 << 20,
                    ..unbounded
                },
                s2: unbounded,
                s3: unbounded,
                s4: unbounded,
            },
        )
        .expect("QSL compiles recursive records");
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        let read = CheckedPackageV2::read(
            compiled.bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence,
        );
        let CheckedPackageV2ReadResult::Admitted(package) = read else {
            panic!("QSL recursive package must pass Contract IR's strict reader: {read:?}")
        };
        *package
    }

    /// Trace: FR-018-AC-26, FR-018-AC-27, TC-029.
    #[test]
    fn tc_029_qsl_recursive_package_uses_the_reader_shapes() {
        let package = qsl_recursive_package();
        let nodes = &package.graph().nodes;
        let named = |name: &str| {
            nodes
                .iter()
                .find(|node| {
                    node.declaration.as_ref().is_some_and(|declaration| {
                        declaration.qualified_name.len() == 1
                            && declaration.qualified_name[0].as_ref() == name
                    })
                })
                .expect("named QSpec record")
        };
        let list = named("List");
        let tree = named("Tree");
        assert!(resolve_shape(nodes, &list.node_id).is_ok());
        assert!(resolve_shape(nodes, &tree.node_id).is_ok());
        let list_closure = resolve_shape(nodes, &list.node_id).unwrap();
        let list_key = node_key(&list.node_id).unwrap();
        let CompositeShape::Record(fields) = list_closure.composites[&list_key].shape() else {
            panic!("QSpec List is a record")
        };
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name(), "next");
        assert_eq!(fields[0].presence(), Presence::Optional);
        assert_eq!(fields[0].value_type(), &ValueType::Composite(list_key));
        let tree_closure = resolve_shape(nodes, &tree.node_id).unwrap();
        let tree_key = node_key(&tree.node_id).unwrap();
        let CompositeShape::Record(fields) = tree_closure.composites[&tree_key].shape() else {
            panic!("QSpec Tree is a record")
        };
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name(), "kids");
        assert_eq!(
            fields[0].value_type(),
            &ValueType::collection(CollectionType::new(
                CollectionKind::Sequence,
                ValueType::Composite(tree_key),
                CardinalityBound::new(0, 3).unwrap(),
            ))
        );
    }

    fn resolve_shape(
        nodes: &[CheckedSemanticNodeV2],
        type_id: &CheckedNodeId,
    ) -> Result<TypeClosure, CompositeEqualityRefusal> {
        let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
        let mut bounds = BTreeMap::new();
        for node in nodes {
            if CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::BoundedDomain) {
                bounds
                    .entry(&node.semantic_type)
                    .or_insert_with(Vec::new)
                    .push(node);
            }
        }
        let mut closure = TypeClosure::default();
        resolve_type(&graph, &bounds, &mut closure, type_id)?;
        Ok(closure)
    }

    fn check_synthetic_items(
        mut nodes: Vec<CheckedSemanticNodeV2>,
        operands: &[(EqualityOperandDescriptor, EqualityOperandDescriptor)],
    ) -> (Vec<Result<(), CompositeEqualityRefusal>>, String) {
        assert!(operands.len() <= 2, "fixture has two distinct item ids");
        let mut records = Vec::new();
        let mut items = Vec::new();
        for ((left, right), (left_digit, right_digit, expression_digit)) in
            operands.iter().zip([('8', 'a', '9'), ('6', 'b', '7')])
        {
            let left_id = node_id(left_digit);
            let right_id = node_id(right_digit);
            let expression_id = node_id(expression_digit);
            let parameter = |id: &CheckedNodeId, source_type: &CheckedNodeId| {
                serde_json::from_value::<CheckedSemanticNodeV2>(json!({
                    "node_id": id,
                    "schema_version": "quire.checked-semantic-graph/v2",
                    "node_tag": "value",
                    "semantic_form": "parameter",
                    "semantic_type": source_type,
                    "dependencies": [],
                    "occurrences": [],
                    "body": {"term": "aggregate", "members": []},
                }))
                .expect("synthetic parameter")
            };
            let expression: CheckedSemanticNodeV2 = serde_json::from_value(json!({
                "node_id": expression_id,
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "expression",
                "semantic_form": "binary",
                "semantic_type": left.source_type,
                "dependencies": [],
                "occurrences": [],
                "body": {"term": "application", "operator": "binary", "arguments": [
                    {"term": "reference", "target": left_id},
                    {"term": "reference", "target": right_id}
                ]},
            }))
            .unwrap();
            records.push(CompleteLoweringRecordV2::Lowered {
                node: Box::new(quire_contract_model::CompleteContractNodeV2 {
                    node: expression.clone(),
                    node_tag: CheckedNodeTag::Expression,
                    source_map: Vec::new(),
                    semantic_type: left.source_type.clone(),
                    dependencies: Vec::new(),
                    bounds: Vec::new(),
                    claims: Vec::new(),
                    ir_id: CheckedSemanticId {
                        domain: "quire.contract-ir.semantic/v1".into(),
                        algorithm: "sha256".into(),
                        digest: "0".repeat(64).into(),
                    },
                }),
            });
            items.push(CompositeEqualityItem {
                node_id: expression_id,
                operator: EqualityOperatorKind::Equal,
                left: left.clone(),
                right: right.clone(),
            });
            nodes.push(parameter(&left_id, &left.source_type));
            nodes.push(parameter(&right_id, &right.source_type));
            nodes.push(expression);
        }
        let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
        let mut source = SourceBuilder::default();
        let outcomes = records
            .iter()
            .zip(&items)
            .enumerate()
            .map(|(index, (record, item))| {
                match check_item(&graph, &BTreeMap::new(), record, item) {
                    Ok(checked) => {
                        let rendered = render_item(&checked).expect("healthy item renders");
                        source.item(&format!("test{index}"), item, &rendered);
                        Ok(())
                    }
                    Err(ItemCheckError::Refusal(refusal)) => Err(refusal),
                    Err(ItemCheckError::Generation(error)) => {
                        panic!("unexpected generation failure: {error:?}")
                    }
                }
            })
            .collect();
        (outcomes, source.finish())
    }

    fn check_synthetic_item(
        nodes: Vec<CheckedSemanticNodeV2>,
        type_id: CheckedNodeId,
    ) -> Result<(), CompositeEqualityRefusal> {
        let typed = EqualityOperandDescriptor::typed(type_id);
        let (mut outcomes, _) = check_synthetic_items(nodes, &[(typed.clone(), typed)]);
        outcomes.remove(0)
    }

    /// Trace: FR-018-AC-24, TC-029.
    #[test]
    fn tc_029_unclosed_option_and_sequence_cycles_refuse_at_the_repeated_node() {
        const TEST_NAME: &str = "oracle::equality::tests::tc_029_unclosed_option_and_sequence_cycles_refuse_at_the_repeated_node";
        if let Some(form) = std::env::var_os("QUIRE_CG_TC029_CYCLE_CHILD") {
            let form = form.to_str().expect("UTF-8 fixture form");
            let mut nodes = recursive_shape_nodes();
            let (root, repeated) = if form == "option" {
                nodes[1].body["members"][0]["target"] = json!(node_id('b'));
                (node_id('a'), node_id('b'))
            } else {
                nodes[3].body["members"][0]["target"] = json!(node_id('d'));
                (node_id('c'), node_id('d'))
            };
            assert_eq!(
                resolve_shape(&nodes, &root).err(),
                Some(CompositeEqualityRefusal::TypeResolutionCycle {
                    repeated_type_node_id: repeated.clone(),
                }),
                "{form} cycle"
            );
            assert_eq!(
                check_synthetic_item(nodes.clone(), root).err(),
                Some(CompositeEqualityRefusal::TypeResolutionCycle {
                    repeated_type_node_id: repeated,
                }),
                "{form} item refusal"
            );
            let sibling = if form == "option" {
                node_id('c')
            } else {
                node_id('a')
            };
            assert!(resolve_shape(&nodes, &sibling).is_ok(), "{form} sibling");
            assert!(
                check_synthetic_item(nodes, node_id('f')).is_ok(),
                "{form} healthy item"
            );
            return;
        }
        for form in ["option", "sequence"] {
            let outcome = std::process::Command::new(std::env::current_exe().unwrap())
                .arg("--exact")
                .arg(TEST_NAME)
                .env("QUIRE_CG_TC029_CYCLE_CHILD", form)
                .output()
                .expect("subprocess starts");
            assert!(
                outcome.status.success(),
                "{form} subprocess refused or crashed: {}",
                String::from_utf8_lossy(&outcome.stderr)
            );
        }
    }

    /// Trace: FR-018-AC-25, TC-029.
    #[test]
    fn tc_029_type_resolution_charges_each_entry_and_the_first_over_limit_entry() {
        const TEST_NAME: &str = "oracle::equality::tests::tc_029_type_resolution_charges_each_entry_and_the_first_over_limit_entry";
        if std::env::var_os("QUIRE_CG_TC029_WORK_CHILD").is_none() {
            let outcome = std::process::Command::new(std::env::current_exe().unwrap())
                .arg("--exact")
                .arg(TEST_NAME)
                .env("QUIRE_CG_TC029_WORK_CHILD", "1")
                .output()
                .expect("work-bound subprocess starts");
            assert!(
                outcome.status.success(),
                "work-bound subprocess failed or aborted: {} {}",
                String::from_utf8_lossy(&outcome.stdout),
                String::from_utf8_lossy(&outcome.stderr)
            );
            return;
        }
        let nodes = recursive_shape_nodes();
        let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
        let bounds = BTreeMap::from([(&nodes[4].semantic_type, vec![&nodes[4]])]);
        let mut closure = TypeClosure::default();
        resolve_type(&graph, &bounds, &mut closure, &node_id('a')).expect("List closes");
        assert_eq!(closure.work_consumed, 3); // record, option, repeated record
        resolve_type(&graph, &bounds, &mut closure, &node_id('c')).expect("Tree closes");
        assert_eq!(closure.work_consumed, 7); // record, bound, sequence, repeated record
        resolve_type(&graph, &bounds, &mut closure, &node_id('a')).expect("cached List");
        assert_eq!(closure.work_consumed, 8); // the cached root still charges

        let count = usize::try_from(COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT).unwrap();
        let ids: Vec<CheckedNodeId> = (0..=count)
            .map(|index| CheckedNodeId {
                domain: "quire.checked-semantic-node/v1".into(),
                digest: format!("{:064x}", index + 1).into(),
            })
            .collect();
        let chain: Vec<CheckedSemanticNodeV2> = (0..=count)
            .map(|index| {
                let (form, tag, body) = if index == count {
                    (
                        "boolean",
                        "scalar_type",
                        json!({"term": "aggregate", "members": []}),
                    )
                } else {
                    (
                        "record",
                        "composite_type",
                        json!({"term": "aggregate", "members": [{
                            "term": "binding", "name": "next", "value": {
                                "term": "reference", "target": ids[index + 1]
                            }
                        }]}),
                    )
                };
                serde_json::from_value(json!({
                    "node_id": ids[index],
                    "schema_version": "quire.checked-semantic-graph/v2",
                    "node_tag": tag,
                    "semantic_form": form,
                    "semantic_type": ids[index],
                    "dependencies": [],
                    "occurrences": [],
                    "body": body,
                }))
                .expect("synthetic chain node")
            })
            .collect();
        let graph: Graph<'_> = chain.iter().map(|node| (&node.node_id, node)).collect();
        let empty = BTreeMap::new();
        let mut over = TypeClosure::default();
        assert_eq!(
            resolve_type(&graph, &empty, &mut over, &ids[0]).err(),
            Some(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                limit: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
                consumed: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT + 1,
            })
        );
        let mut at_limit = TypeClosure::default();
        assert!(resolve_type(&graph, &empty, &mut at_limit, &ids[2]).is_ok());
        assert_eq!(
            at_limit.work_consumed,
            COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT - 1
        );
        assert!(resolve_type(&graph, &empty, &mut at_limit, &ids[2]).is_ok());
        assert_eq!(
            at_limit.work_consumed,
            COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT
        );
        let mut sibling = TypeClosure::default();
        assert_eq!(
            resolve_type(&graph, &empty, &mut sibling, &ids[count]),
            Ok(ValueType::Boolean)
        );

        // Four roots in one item (left source/target, then right source/target)
        // consume exactly the limit: the first 65,533-node walk plus three
        // cached-root entries. The item must still be generated.
        let at_limit_operand = EqualityOperandDescriptor::converted(ids[4].clone(), ids[4].clone());
        let (at_limit_outcomes, at_limit_source) = check_synthetic_items(
            chain.clone(),
            &[(at_limit_operand.clone(), at_limit_operand)],
        );
        assert_eq!(at_limit_outcomes, vec![Ok(())]);
        assert!(at_limit_source.contains("pub fn oracle_test0"));

        // Both items share one synthetic lowered graph and emission pass.
        let exhausted = EqualityOperandDescriptor::typed(ids[0].clone());
        let healthy = EqualityOperandDescriptor::typed(ids[count].clone());
        let (outcomes, emitted) = check_synthetic_items(
            chain,
            &[(exhausted.clone(), exhausted), (healthy.clone(), healthy)],
        );
        assert_eq!(
            outcomes[0],
            Err(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                limit: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
                consumed: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT + 1,
            })
        );
        assert!(outcomes[1].is_ok());
        assert_eq!(emitted.matches("pub fn oracle_").count(), 1);
        assert!(!emitted.contains("pub fn oracle_test0"));
        assert!(emitted.contains("pub fn oracle_test1"));
    }

    /// Trace: FR-018-AC-25, TC-029.
    #[test]
    fn tc_029_deep_option_resolution_and_runtime_type_operations_fit_a_small_stack() {
        const DEPTH: usize = 2_048;
        let ids: Vec<CheckedNodeId> = (0..=DEPTH)
            .map(|index| CheckedNodeId {
                domain: "quire.checked-semantic-node/v1".into(),
                digest: format!("{:064x}", index + 1).into(),
            })
            .collect();
        let nodes: Vec<CheckedSemanticNodeV2> = (0..=DEPTH)
            .map(|index| {
                let (tag, form, body) = if index == DEPTH {
                    ("scalar_type", "boolean", json!({"term": "aggregate", "members": []}))
                } else {
                    (
                        "composite_type",
                        "option",
                        json!({"term": "aggregate", "members": [{"term": "reference", "target": ids[index + 1]}]}),
                    )
                };
                serde_json::from_value(json!({
                    "node_id": ids[index],
                    "schema_version": "quire.checked-semantic-graph/v2",
                    "node_tag": tag,
                    "semantic_form": form,
                    "semantic_type": ids[index],
                    "dependencies": [],
                    "occurrences": [],
                    "body": body,
                }))
                .expect("synthetic deep Option node")
            })
            .collect();
        std::thread::Builder::new()
            .stack_size(64 * 1024)
            .spawn(move || {
                let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
                let mut closure = TypeClosure::default();
                let value_type = resolve_type(&graph, &BTreeMap::new(), &mut closure, &ids[0])
                    .expect("deep Option resolves within the work budget");
                assert_eq!(closure.work_consumed, DEPTH as u64 + 1);
                assert!(matches!(value_type, ValueType::Option(_)));
                let copy = value_type.clone();
                assert_eq!(copy, value_type);
                let rendered = render_value_type(&value_type).expect("deep Option renders");
                assert_eq!(rendered.matches("rt::ValueType::option(").count(), DEPTH);
                assert!(rendered.contains("rt::ValueType::Boolean"));
                assert!(rendered.len() < MAX_GENERATED_SOURCE_BYTES);
                drop(copy);
                drop(value_type);
            })
            .expect("small-stack worker starts")
            .join()
            .expect("deep Option operations fit the worker stack");
    }

    /// Trace: FR-018-AC-25, TC-029.
    #[test]
    fn tc_029_inline_collection_bound_is_charged_before_lookup() {
        let mut nodes = recursive_shape_nodes();
        nodes[3].body = json!({"term": "aggregate", "members": [
            {"term": "reference", "target": node_id('f')},
            {"term": "reference", "target": node_id('7')}
        ]});
        let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
        let mut at_limit = TypeClosure {
            work_consumed: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT - 2,
            ..TypeClosure::default()
        };
        assert_eq!(
            resolve_type(&graph, &BTreeMap::new(), &mut at_limit, &node_id('d')),
            Err(CompositeEqualityRefusal::UnknownTypeNode {
                type_node_id: node_id('7'),
            })
        );
        assert_eq!(
            at_limit.work_consumed,
            COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT
        );
        let mut over_limit = TypeClosure {
            work_consumed: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT - 1,
            ..TypeClosure::default()
        };
        assert_eq!(
            resolve_type(&graph, &BTreeMap::new(), &mut over_limit, &node_id('d')),
            Err(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                limit: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
                consumed: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT + 1,
            })
        );
    }

    /// Trace: FR-018-AC-26, FR-018-AC-27, TC-029.
    #[test]
    fn tc_029_recursive_list_and_tree_shapes_close_at_record_keys() {
        let nodes = recursive_shape_nodes();
        let list = resolve_shape(&nodes, &node_id('a')).expect("List closes");
        let list_key = node_key(&node_id('a')).unwrap();
        let CompositeShape::Record(list_fields) = list.composites[&list_key].shape() else {
            panic!("List is a record")
        };
        assert_eq!(list_fields.len(), 1);
        assert_eq!(list_fields[0].name(), "next");
        assert_eq!(list_fields[0].presence(), Presence::Optional);
        assert_eq!(list_fields[0].value_type(), &ValueType::Composite(list_key));

        let tree = resolve_shape(&nodes, &node_id('c')).expect("Tree closes");
        let tree_key = node_key(&node_id('c')).unwrap();
        let CompositeShape::Record(tree_fields) = tree.composites[&tree_key].shape() else {
            panic!("Tree is a record")
        };
        assert_eq!(tree_fields.len(), 1);
        assert_eq!(tree_fields[0].name(), "kids");
        assert_eq!(tree_fields[0].presence(), Presence::Required);
        assert_eq!(
            tree_fields[0].value_type(),
            &ValueType::collection(CollectionType::new(
                CollectionKind::Sequence,
                ValueType::Composite(tree_key),
                CardinalityBound::new(0, 3).unwrap(),
            ))
        );
    }

    /// Trace: FR-018-AC-26, TC-029.
    #[test]
    fn tc_029_optional_wrapper_malformed_shapes_refuse_without_affecting_tree() {
        for mutation in ["remove", "rename", "second", "non_option"] {
            let mut nodes = recursive_shape_nodes();
            let optional = &mut nodes[0].body["members"][0]["value"]["members"];
            match mutation {
                "remove" => *optional = json!([]),
                "rename" => optional[0]["name"] = json!("maybe"),
                "second" => {
                    let duplicate = optional[0].clone();
                    optional.as_array_mut().unwrap().push(duplicate);
                }
                "non_option" => optional[0]["value"]["target"] = json!(node_id('f')),
                _ => unreachable!(),
            }
            assert_eq!(
                resolve_shape(&nodes, &node_id('a')).err(),
                Some(CompositeEqualityRefusal::MalformedComposite {
                    composite: node_id('a')
                }),
                "{mutation}"
            );
            assert!(
                resolve_shape(&nodes, &node_id('c')).is_ok(),
                "{mutation}: healthy Tree"
            );
        }
    }

    /// Trace: FR-018-AC-26, TC-029.
    #[test]
    fn tc_029_direct_option_field_is_required_not_silently_optional() {
        let mut nodes = recursive_shape_nodes();
        nodes[0].body["members"][0]["value"] = json!({"term": "reference", "target": node_id('b')});
        let closure = resolve_shape(&nodes, &node_id('a')).expect("direct option is valid");
        let key = node_key(&node_id('a')).unwrap();
        let CompositeShape::Record(fields) = closure.composites[&key].shape() else {
            panic!("List is a record")
        };
        assert_eq!(fields[0].presence(), Presence::Required);
        assert_eq!(
            fields[0].value_type(),
            &ValueType::option(ValueType::Composite(key))
        );
    }

    /// Trace: FR-018-AC-27, TC-029.
    #[test]
    fn tc_029_sequence_bound_shape_mutants_refuse_without_affecting_list() {
        for mutation in [
            "non_sequence",
            "missing_min",
            "duplicate_max",
            "non_integer",
        ] {
            let mut nodes = recursive_shape_nodes();
            match mutation {
                "non_sequence" => nodes[4].semantic_type = node_id('f'),
                "missing_min" => {
                    nodes[4].body["members"].as_array_mut().unwrap().remove(0);
                }
                "duplicate_max" => {
                    let duplicate = nodes[4].body["members"][1].clone();
                    nodes[4].body["members"]
                        .as_array_mut()
                        .unwrap()
                        .push(duplicate);
                }
                "non_integer" => nodes[4].body["members"][0]["value"]["value_kind"] = json!("text"),
                _ => unreachable!(),
            }
            let expected = if mutation == "non_sequence" {
                CompositeEqualityRefusal::Unsupported {
                    unsupported_node_id: node_id('e'),
                    node_tag: "bounded_domain",
                }
            } else {
                CompositeEqualityRefusal::UnreadableBound {
                    bound: node_id('e'),
                }
            };
            assert_eq!(
                resolve_shape(&nodes, &node_id('c')).err(),
                Some(expected),
                "{mutation}"
            );
            assert!(
                resolve_shape(&nodes, &node_id('a')).is_ok(),
                "{mutation}: healthy List"
            );
        }
    }

    fn interval_of(lower: &str, upper: &str) -> IntegerInterval {
        IntegerInterval::new(
            lower.parse().expect("integer"),
            upper.parse().expect("integer"),
        )
        .expect("interval")
    }

    fn member(name: &str, value: &str) -> Value {
        json!({"term": "binding", "name": name, "value": {
            "term": "literal",
            "type": {"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)},
            "value_kind": "integer",
            "value": value,
        }})
    }

    fn text_member(name: &str, value: &str) -> Value {
        json!({"term": "binding", "name": name, "value": {
            "term": "literal", "value_kind": "text", "value": value,
        }})
    }

    fn bare(kind: &str, value: &str) -> Value {
        json!({"term": "literal", "value_kind": kind, "value": value})
    }

    /// Integer ranges, decimal ranges, text bounds and collection cardinalities are read from
    /// binding-shaped members named as in QSpec's `positive-operation-identities.json`.
    ///
    /// Trace: FR-018-AC-14, TC-029.
    #[test]
    fn tc_029_bound_members_are_read_from_named_bindings() {
        assert_eq!(
            read_integer_range(&[member("max", "5"), member("min", "-5")]),
            Some(interval_of("-5", "5"))
        );
        assert!(read_decimal_range(&[
            member("coefficient_min", "-100"),
            member("coefficient_max", "100"),
            member("scale_min", "0"),
            member("scale_max", "2"),
            text_member("rounding", "nearest-even"),
        ])
        .is_some());
        assert!(read_text_bounds(&[
            member("min", "0"),
            member("max", "16"),
            text_member("text_profile", "nfc"),
        ])
        .is_some());
        assert_eq!(
            bound_members(
                &[member("min", "0"), member("max", "8")],
                COLLECTION_BOUNDS_MEMBERS
            )
            .map(|[minimum, maximum]| (literal_count(minimum), literal_count(maximum))),
            Some((Some(0), Some(8)))
        );
    }

    /// A bare literal in place of a `binding` member is refused: the integer endpoints, the
    /// decimal `rounding`, the text `text_profile` and a collection cardinality.
    ///
    /// Trace: FR-018-AC-14, TC-029.
    #[test]
    fn tc_029_bare_literal_bound_members_are_refused() {
        assert_eq!(
            read_integer_range(&[bare("integer", "-5"), bare("integer", "5")]),
            None
        );
        assert_eq!(
            read_decimal_range(&[
                member("coefficient_min", "-100"),
                member("coefficient_max", "100"),
                member("scale_min", "0"),
                member("scale_max", "2"),
                bare("text", "nearest-even"),
            ]),
            None,
            "a bare `rounding` member"
        );
        assert_eq!(
            read_text_bounds(&[member("min", "0"), member("max", "16"), bare("text", "nfc"),]),
            None,
            "a bare `text_profile` member"
        );
        assert_eq!(
            read_text_bounds(&[
                bare("integer", "0"),
                bare("integer", "16"),
                text_member("text_profile", "nfc"),
            ]),
            None,
            "bare text-bounds `min`/`max` members"
        );
        assert_eq!(
            bound_members(
                &[bare("integer", "0"), bare("integer", "8")],
                COLLECTION_BOUNDS_MEMBERS
            )
            .map(|_| ()),
            None
        );
        // A wrong name is refused as well as a wrong shape.
        assert_eq!(
            read_decimal_range(&[
                member("coefficient_min", "-100"),
                member("coefficient_max", "100"),
                member("scale_min", "0"),
                member("scale_max", "2"),
                text_member("mode", "nearest-even"),
            ]),
            None
        );
    }

    fn quantity() -> ValueType {
        ValueType::Quantity(rt::QuantityUnit::Compound(rt::CompoundUnit::dimensionless()))
    }

    fn reference() -> ValueType {
        ValueType::Reference(NodeKey::from_bytes([7; 32]))
    }

    fn node_id(digit: char) -> CheckedNodeId {
        CheckedNodeId {
            domain: "quire.checked-semantic-node/v1".into(),
            digest: digit.to_string().repeat(64).into(),
        }
    }

    /// A `failed` record is refused by its limit kind through the one shared classifier: `work`
    /// as `LoweringWorkExhausted`, `bytes` as `LoweringByteLimitExceeded` with the record's own
    /// `limit` and `consumed` (the per-node case Contract IR's FR-038-AC-95 defines and CG's
    /// public API cannot reach), and every other kind as `LoweringLimitUnrecognised` under its
    /// snake_case name, never as work exhaustion and without a panic.
    ///
    /// Trace: FR-018-AC-20, TC-029.
    #[test]
    fn tc_029_a_failed_record_is_refused_by_its_limit_kind() {
        use crate::oracle::failed_records::{failed_record, unrecognised_kinds};
        use quire_contract_model::CheckedPackageLimit;

        assert_eq!(
            lowered(&failed_record(CheckedPackageLimit::Work, 65_536, 65_537)).err(),
            Some(CompositeEqualityRefusal::LoweringWorkExhausted {
                limit: 65_536,
                consumed: 65_537
            })
        );
        assert_eq!(
            lowered(&failed_record(CheckedPackageLimit::Bytes, 1_000, 1_001)).err(),
            Some(CompositeEqualityRefusal::LoweringByteLimitExceeded {
                limit: 1_000,
                consumed: 1_001
            })
        );
        for (kind, name) in unrecognised_kinds() {
            assert_eq!(
                lowered(&failed_record(kind, 7, 9)).err(),
                Some(CompositeEqualityRefusal::LoweringLimitUnrecognised {
                    limit_kind: name,
                    limit: 7,
                    consumed: 9
                }),
                "{name}"
            );
        }
    }

    fn semantic_id(digit: char) -> CheckedSemanticId {
        CheckedSemanticId {
            domain: "quire.checked-semantic/v1".into(),
            algorithm: "sha256".into(),
            digest: digit.to_string().repeat(64).into(),
        }
    }

    /// A generated claim for the expression node `node`, as the item loop records it before
    /// rendering.
    fn generated_claim(node: char) -> CompositeEqualityClaim {
        CompositeEqualityClaim {
            node_id: node_id(node),
            operation: CompositeOperationClaim {
                identity: "equality.equal".to_owned(),
                provenance: CompositeOperationProvenance::CallerDeclared {
                    blocked_on: UpstreamBlocker::OperationIdentityNotConsumed,
                },
            },
            result: ClaimDisposition::Generated(Box::new(GeneratedCompositeEqualityClaim {
                environment_symbol: String::new(),
                oracle_symbol: String::new(),
                ir_id: semantic_id('a'),
                package_id: semantic_id('b'),
                semantic_type: node_id('c'),
                source_map: Vec::new(),
                claims: Vec::new(),
                descriptor: RecordedDescriptor {
                    operator: EqualityOperatorKind::Equal,
                    left_source_type: node_id('d'),
                    left_conversion_target: None,
                    right_source_type: node_id('d'),
                    right_conversion_target: None,
                },
                declaration_keys: Vec::new(),
                declaration_runtime_keys: Vec::new(),
                schedule: RecordedSchedule::Plan,
            })),
        }
    }

    fn rendered_item() -> RenderedItem {
        RenderedItem {
            composites: String::new(),
            left_source: "rt::ValueType::Integer".to_owned(),
            left_target: None,
            right_source: "rt::ValueType::Integer".to_owned(),
            right_target: None,
        }
    }

    /// `render_value_type` returns the typed error for `ValueType::Quantity` and for
    /// `ValueType::Reference`, directly, nested in an option, and inside a composite declaration
    /// (the second call site, beside an operand's own types). It does not panic.
    ///
    /// Trace: FR-018-AC-18, TC-029.
    #[test]
    fn tc_029_ac18_render_value_type_refuses_quantity_and_reference_with_a_typed_error() {
        for (value_type, family) in [(quantity(), "quantity"), (reference(), "reference")] {
            let expected = Err(RenderError::UnsupportedValueType { family });
            assert_eq!(render_value_type(&value_type), expected);
            assert_eq!(
                render_value_type(&ValueType::option(value_type.clone())),
                expected,
                "{family} nested in an option"
            );
            let declaration = CompositeDeclaration::new(
                NodeKey::from_bytes([1; 32]),
                "D",
                CompositeShape::Tuple(vec![ValueType::Integer, value_type]),
            );
            assert_eq!(
                render_composite_declaration(&declaration),
                expected,
                "{family} in a composite declaration"
            );
        }
    }

    /// The item boundary turns `RenderError::UnsupportedValueType` into the per-item refusal,
    /// naming the family and the item's expression node, renders no code for it, and leaves a
    /// sibling's claim unchanged; `RenderError::Generation` still fails the whole call, with the
    /// carried error, and touches no claim.
    ///
    /// Trace: FR-018-AC-18, TC-029.
    #[test]
    fn tc_029_ac18_the_item_boundary_refuses_the_item_and_fails_the_call_on_a_generation_error() {
        for family in ["quantity", "reference"] {
            let mut item = generated_claim('1');
            let settled =
                settle_render(&mut item, Err(RenderError::UnsupportedValueType { family }));
            assert!(
                matches!(settled, Ok(None)),
                "no code is rendered for the item"
            );
            assert_eq!(
                item.result,
                ClaimDisposition::Refused {
                    refusal: CompositeEqualityRefusal::Unsupported {
                        unsupported_node_id: node_id('1'),
                        node_tag: family,
                    }
                }
            );
        }

        let mut item = generated_claim('1');
        let error = OracleGenerationError::UnknownRuntimeVariant {
            enum_name: "CollectionKind",
        };
        assert_eq!(
            settle_render(&mut item, Err(RenderError::Generation(error))).err(),
            Some(error)
        );
        assert_eq!(
            item,
            generated_claim('1'),
            "a whole-call failure refuses no item"
        );

        let mut item = generated_claim('1');
        assert!(matches!(
            settle_render(&mut item, Ok(rendered_item())),
            Ok(Some(_))
        ));
        assert_eq!(
            item,
            generated_claim('1'),
            "a rendered item stays generated"
        );
    }

    fn request(node: char) -> CompositeEqualityItem {
        CompositeEqualityItem {
            node_id: node_id(node),
            operator: EqualityOperatorKind::Equal,
            left: EqualityOperandDescriptor::typed(node_id('d')),
            right: EqualityOperandDescriptor::typed(node_id('d')),
        }
    }

    /// Run [`render_and_emit`] over two items, `1` then `2`, sharing one operator stem; an item
    /// whose flag is true fails to render with `error`.
    fn emit_two(
        fails: [bool; 2],
        error: RenderError,
    ) -> (
        Result<(), OracleGenerationError>,
        Vec<CompositeEqualityClaim>,
        String,
    ) {
        let requests = [request('1'), request('2')];
        let mut claims = vec![generated_claim('1'), generated_claim('2')];
        let pending = requests
            .iter()
            .zip(fails)
            .enumerate()
            .map(|(position, (item, fails))| (position, DescriptorKey::of(item), item, fails))
            .collect();
        let mut source = SourceBuilder::default();
        let result = render_and_emit(&mut claims, &mut source, pending, |fails| {
            if *fails {
                Err(error)
            } else {
                Ok(rendered_item())
            }
        });
        (result, claims, source.finish())
    }

    fn symbols(claim: &CompositeEqualityClaim) -> Option<(&str, &str)> {
        match &claim.result {
            ClaimDisposition::Generated(generated) => Some((
                generated.environment_symbol.as_str(),
                generated.oracle_symbol.as_str(),
            )),
            ClaimDisposition::Refused { .. } => None,
        }
    }

    fn unsupported(
        node: char,
        family: &'static str,
    ) -> ClaimDisposition<GeneratedCompositeEqualityClaim, CompositeEqualityRefusal> {
        ClaimDisposition::Refused {
            refusal: CompositeEqualityRefusal::Unsupported {
                unsupported_node_id: node_id(node),
                node_tag: family,
            },
        }
    }

    /// An item that fails to render is refused on its own claim, emits no code, and leaves its
    /// sibling's claim and symbols unchanged, whichever of the two fails; the sibling keeps the
    /// bare stem the refused item would otherwise have shared. A whole-call render error aborts.
    ///
    /// Trace: FR-018-AC-18, TC-029.
    #[test]
    fn tc_029_ac18_a_render_failure_refuses_only_its_item_and_never_renames_a_sibling() {
        let quantity = RenderError::UnsupportedValueType { family: "quantity" };

        // Both render: the stem is shared, so the two are numbered in key order.
        let (result, claims, lib) = emit_two([false, false], quantity);
        assert_eq!(result, Ok(()));
        assert_eq!(
            symbols(&claims[0]),
            Some(("environment_equality_equal_1", "oracle_equality_equal_1"))
        );
        assert_eq!(
            symbols(&claims[1]),
            Some(("environment_equality_equal_2", "oracle_equality_equal_2"))
        );
        assert_eq!(lib.matches("pub fn environment_").count(), 2);

        // The second fails: the first is untouched in claim, and holds the bare stem.
        let (result, claims, lib) = emit_two([false, true], quantity);
        assert_eq!(result, Ok(()));
        assert_eq!(
            symbols(&claims[0]),
            Some(("environment_equality_equal", "oracle_equality_equal"))
        );
        assert_eq!(claims[1].result, unsupported('2', "quantity"));
        assert_eq!(claims[1].node_id, node_id('2'));
        assert_eq!(lib.matches("pub fn environment_").count(), 1);

        // The first fails: the second is the one that survives, under the bare stem.
        let (result, claims, lib) = emit_two([true, false], quantity);
        assert_eq!(result, Ok(()));
        assert_eq!(claims[0].result, unsupported('1', "quantity"));
        assert_eq!(
            symbols(&claims[1]),
            Some(("environment_equality_equal", "oracle_equality_equal"))
        );
        assert_eq!(lib.matches("pub fn environment_").count(), 1);

        // Neither renders: both are refused, with no code.
        let (result, claims, lib) = emit_two([true, true], quantity);
        assert_eq!(result, Ok(()));
        assert_eq!(claims[0].result, unsupported('1', "quantity"));
        assert_eq!(claims[1].result, unsupported('2', "quantity"));
        assert_eq!(lib.matches("pub fn environment_").count(), 0);

        // A whole-call error aborts the call, however the other item fares.
        let error = OracleGenerationError::UnknownRuntimeVariant {
            enum_name: "TextProfile",
        };
        let (result, _, _) = emit_two([false, true], RenderError::Generation(error));
        assert_eq!(result, Err(error));
    }
}

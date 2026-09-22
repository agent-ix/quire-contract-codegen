//! Exact complete-V1 composite/structural equality oracle generation from
//! CheckedPackage V2 (FR-018).
//!
//! This is the composite/structural sibling of [`crate::exact_scalar`]
//! (FR-014): a requested item names one checked `binary` expression node and a
//! typed equality descriptor — an operator and two operands, each either
//! `EqualityOperand::typed(source)` or `EqualityOperand::converted(source,
//! target)` — where `source`/`target` are named as V2 type node ids, not as
//! runtime `ValueType` values, because a runtime `ValueType` carries no V2
//! node id once built and this generator's ordering and symbol both need one
//! (see the module's `DescriptorKey`).
//!
//! The runtime carries no structural equality on `Value`; the FR-149 relation
//! reached through `TypeEnvironment::check_equality` and
//! `CheckedEquality::evaluate` is the only equality. This generator therefore
//! never compares two values itself: it reconstructs the record/tuple
//! declaration closure reachable from each operand's type, admits it through
//! `TypeEnvironment::new` (refusing per [`DeclarationRefusal`] on failure),
//! checks the descriptor through `TypeEnvironment::check_equality` (refusing
//! per [`IllTypedCauseKind`] on failure), and emits two functions per
//! surviving item: an environment constructor
//! (`Result<TypeEnvironment, InvalidDeclaration>`) and an oracle function
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
//! | `record` | one `binding` per field in declaration order: its `name` is the field name, its value a `reference` to the field's type node; a field whose bound node is itself a `composite_type` of form `option` is `Presence::Optional` with the unwrapped payload type, every other field `Presence::Required` |
//! | `tuple` | one `reference` per position, in declaration order |
//! | `option` | one `reference` to the payload type node |
//! | `sequence`, `set`, `bag`, `ordered_set` | a `reference` to the element type node, then a `reference` to the `collection_bounds` domain node |
//! | `collection_bounds` | two canonical decimal `integer` literals: minimum, maximum |
//!
//! Every runtime constructor this generator or its emitted source calls to
//! rebuild a bound (`IntegerInterval::new`, `RationalDomain::new`,
//! `DecimalType::new`, `TextType::new`, `CardinalityBound::new`) is fallible
//! only on values this generator has already validated once, at generation
//! time, through the identical pure constructor over the identical canonical
//! literal. The emitted **oracle function** never calls any of them and never
//! panics, exactly as FR-018-AC-8 and FR-018-AC-9 require: it receives its
//! `ValueType` trees from a private per-item helper. That helper, and the
//! public environment constructor's `composites_*` helper, both call these
//! constructors through a generated `.expect("generation-time validation
//! guarantees this bound reconstructs")` — a narrow, disclosed exception to
//! "no panic" that is textually scoped to construction the emitted crate
//! shares with its already-validated generation-time twin, not to the oracle
//! function's own control flow, which the FR's no-panic sentence names
//! explicitly. `TypeEnvironment::new` itself is not wrapped this way: its
//! `Result<TypeEnvironment, InvalidDeclaration>` is the environment
//! constructor's own pinned return type and is propagated unchanged, because
//! a caller may reasonably want to observe a declaration refusal rather than
//! have it hidden behind a panic that can never fire in this generator's own
//! use.
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

use crate::exact_scalar::{aggregate_members, literal_count};
use crate::oracle::{Artifact, MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION};
use quire_contract_ir::{
    CheckedNodeId, CheckedNodeTag, CheckedPackageV2, CheckedSemanticId, CheckedSemanticNodeV2,
    CheckedSourceMapEntry, CompleteLoweringProfileV2, CompleteLoweringRecordV2,
};
use quire_contract_runtime::exact::{
    self as rt, CardinalityBound, CollectionKind, CollectionType, CompositeDeclaration,
    CompositeShape, DecimalType, EqualityOperand, EqualitySchedule, FieldDeclaration,
    IllTypedCause, IntegerInterval, NodeKey, Presence, RationalDomain, RoundingMode, TextProfile,
    TextType, TypeEnvironment, ValueType,
};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

/// Version of the emitted claim map.
pub const COMPOSITE_EQUALITY_CLAIM_MAP_VERSION: &str =
    "quire.codegen.composite-equality-claim-map/v1";

/// Work budget for lowering one requested equality expression node.
pub const COMPOSITE_EQUALITY_LOWERING_WORK_LIMIT: u64 = 65_536;

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

/// Upstream work an item is blocked on.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum UpstreamBlocker {
    /// Model, relation and reference-reaching semantics.
    #[serde(rename = "agent-ix/quire-spec-language#120")]
    QuireSpecLanguage120,
    /// State, temporal and protocol semantics.
    #[serde(rename = "agent-ix/quire-spec-language#121")]
    QuireSpecLanguage121,
    /// Function application.
    #[serde(rename = "agent-ix/quire-contract-runtime#34")]
    QuireContractRuntime34,
    /// CheckedPackage V2 carries the operation identity and its laws, but
    /// this generator's classifiers never read `operation` -- they classify
    /// a body from its `term`/`operator`/`arguments` and the request item's
    /// own descriptor instead.
    #[serde(rename = "operation identity not consumed by codegen's generators")]
    OperationIdentityNotConsumed,
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

impl From<IllTypedCause> for IllTypedCauseKind {
    fn from(cause: IllTypedCause) -> Self {
        match cause {
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
        }
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

impl From<rt::RecursionEdges> for RecursionEdgesKind {
    fn from(edges: rt::RecursionEdges) -> Self {
        match edges {
            rt::RecursionEdges::Unnamed => Self::Unnamed,
            rt::RecursionEdges::NonEscaping => Self::NonEscaping,
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

impl From<rt::DeclarationCause> for DeclarationRefusalCause {
    fn from(cause: rt::DeclarationCause) -> Self {
        match cause {
            rt::DeclarationCause::DuplicateKey => Self::DuplicateKey,
            rt::DeclarationCause::DuplicateMember(name) => Self::DuplicateMember { name },
            rt::DeclarationCause::UnknownDeclaration(key) => Self::UnknownDeclaration {
                key: hex_digest(&key),
            },
            rt::DeclarationCause::Type(cause) => Self::Type {
                cause: cause.into(),
            },
            rt::DeclarationCause::Recursion { edges, cycle } => Self::Recursion {
                edges: edges.into(),
                cycle,
            },
        }
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
    /// Lowering exceeded [`COMPOSITE_EQUALITY_LOWERING_WORK_LIMIT`].
    LoweringWorkExhausted {
        /// The ceiling.
        limit: u64,
        /// Counter at the failed charge.
        consumed: u64,
    },
    /// A named type node id is not in the admitted graph.
    UnknownTypeNode {
        /// The unresolved node id.
        type_node_id: CheckedNodeId,
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

impl From<EqualitySchedule> for RecordedSchedule {
    fn from(schedule: EqualitySchedule) -> Self {
        match schedule {
            EqualitySchedule::Text => Self::Text,
            EqualitySchedule::Enum => Self::Enum,
            EqualitySchedule::Quantity => Self::Quantity,
            EqualitySchedule::Plan => Self::Plan,
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

/// Generated or refused.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum CompositeEqualityDisposition {
    /// One environment constructor and one oracle function were emitted.
    Generated(Box<GeneratedCompositeEqualityClaim>),
    /// Nothing was emitted.
    Refused {
        /// The typed reason.
        refusal: CompositeEqualityRefusal,
    },
}

/// One claim-map entry per distinct requested `(node id, descriptor)`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompositeEqualityClaim {
    /// Requested expression node.
    pub node_id: CheckedNodeId,
    /// The declared operation and its provenance.
    pub operation: CompositeOperationClaim,
    /// Outcome.
    pub result: CompositeEqualityDisposition,
}

/// The per-item source and claim map of one generation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompositeEqualityClaimMap {
    /// [`COMPOSITE_EQUALITY_CLAIM_MAP_VERSION`].
    pub version: &'static str,
    /// Source package identity.
    pub package_id: CheckedSemanticId,
    /// Pinned runtime revision the oracles call.
    pub runtime_revision: &'static str,
    /// Upstream gaps every entry is subject to.
    pub blocked: Vec<UpstreamBlocker>,
    /// Entries, ordered by the descriptor key.
    pub items: Vec<CompositeEqualityClaim>,
}

/// Generation output: the crate files and the claim map they are described
/// by.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositeEqualityOracles {
    /// `Cargo.toml`, `src/lib.rs` and `claim-map.json`, in that order.
    pub artifacts: Vec<Artifact>,
    /// Typed claim map, identical to `claim-map.json`.
    pub claim_map: CompositeEqualityClaimMap,
}

/// Whole-generation failure; per-item problems are refusals, not errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompositeEqualityGenerationError {
    /// The generated source exceeds [`MAX_GENERATED_SOURCE_BYTES`].
    SourceTooLarge {
        /// Generated size.
        bytes: usize,
    },
    /// The claim map could not be serialized.
    ClaimMapSerialization,
}

/// Generate composite equality oracles for `items` from an admitted package.
pub fn generate_composite_equality_oracles(
    package: &CheckedPackageV2,
    items: &[CompositeEqualityItem],
) -> Result<CompositeEqualityOracles, CompositeEqualityGenerationError> {
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
    for ((key, item), record) in by_key.into_iter().zip(&lowering.records) {
        let duplicate = counts.get(&key).copied().unwrap_or(0) > 1;
        let result = if duplicate {
            CompositeEqualityDisposition::Refused {
                refusal: CompositeEqualityRefusal::DuplicateRequest,
            }
        } else {
            match check_item(&graph, &bounds_by_type, record, item) {
                Ok(generated) => {
                    let symbol = key.digest();
                    source.item(&symbol, item, &generated);
                    CompositeEqualityDisposition::Generated(Box::new(
                        GeneratedCompositeEqualityClaim {
                            environment_symbol: format!("environment_{symbol}"),
                            oracle_symbol: format!("oracle_{symbol}"),
                            ir_id: generated.node.ir_id.clone(),
                            package_id: lowering.package_id.clone(),
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
                            schedule: generated.checked.schedule().into(),
                        },
                    ))
                }
                Err(refusal) => CompositeEqualityDisposition::Refused { refusal },
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

    let claim_map = CompositeEqualityClaimMap {
        version: COMPOSITE_EQUALITY_CLAIM_MAP_VERSION,
        package_id: lowering.package_id,
        runtime_revision: RUNTIME_REVISION,
        blocked: vec![UpstreamBlocker::OperationIdentityNotConsumed],
        items: claims,
    };
    let lib = source.finish(&claim_map.package_id);
    if lib.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(CompositeEqualityGenerationError::SourceTooLarge { bytes: lib.len() });
    }
    let mut map_bytes = serde_json::to_vec_pretty(&claim_map)
        .map_err(|_| CompositeEqualityGenerationError::ClaimMapSerialization)?;
    map_bytes.push(b'\n');
    let map_text = String::from_utf8(map_bytes)
        .map_err(|_| CompositeEqualityGenerationError::ClaimMapSerialization)?;
    Ok(CompositeEqualityOracles {
        artifacts: vec![
            artifact("Cargo.toml", manifest()),
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

/// The total order and symbol-disambiguation key: the expression node's id,
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

    /// A digest over exactly this key's node id digests and operator rank,
    /// in key order, and over no rendered name.
    fn digest(&self) -> String {
        let mut hasher = Sha256::new();
        hash_node_id(&mut hasher, &self.node_id);
        hasher.update([self.operator as u8]);
        hash_node_id(&mut hasher, &self.left_source_type);
        hash_optional_node_id(&mut hasher, self.left_conversion_target.as_ref());
        hash_node_id(&mut hasher, &self.right_source_type);
        hash_optional_node_id(&mut hasher, self.right_conversion_target.as_ref());
        let digest = hasher.finalize();
        digest.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

fn hash_node_id(hasher: &mut Sha256, node_id: &CheckedNodeId) {
    hasher.update((node_id.domain.len() as u64).to_le_bytes());
    hasher.update(node_id.domain.as_bytes());
    hasher.update((node_id.digest.len() as u64).to_le_bytes());
    hasher.update(node_id.digest.as_bytes());
}

fn hash_optional_node_id(hasher: &mut Sha256, node_id: Option<&CheckedNodeId>) {
    match node_id {
        None => hasher.update([0_u8]),
        Some(node_id) => {
            hasher.update([1_u8]);
            hash_node_id(hasher, node_id);
        }
    }
}

// ---------------------------------------------------------------------------
// Per-item checking
// ---------------------------------------------------------------------------

/// A checked, admitted item ready to emit.
struct CheckedItem<'r> {
    node: &'r quire_contract_ir::CompleteContractNodeV2,
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

fn check_item<'r>(
    graph: &Graph<'_>,
    bounds_by_type: &BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2>>,
    record: &'r CompleteLoweringRecordV2,
    item: &CompositeEqualityItem,
) -> Result<CheckedItem<'r>, CompositeEqualityRefusal> {
    let node = lowered(record)?;
    if node.node_tag != CheckedNodeTag::Expression {
        return Err(CompositeEqualityRefusal::NotExpression {
            node_tag: node.node_tag.as_wire(),
        });
    }
    if &*node.node.semantic_form == "call" {
        return Err(CompositeEqualityRefusal::BlockedOnUpstream {
            unsupported_node_id: node.node.node_id.clone(),
            node_tag: "expression.call",
            issue: UpstreamBlocker::QuireContractRuntime34,
        });
    }
    if &*node.node.semantic_form != "binary" {
        return Err(CompositeEqualityRefusal::FormMismatch {
            found: node.node.semantic_form.to_string(),
        });
    }
    let arguments = application_arguments(&node.node.body)
        .filter(|arguments| arguments.len() == 2)
        .ok_or(CompositeEqualityRefusal::BodyMismatch)?;
    check_operand_types(arguments, item)?;

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
    let environment =
        TypeEnvironment::new(composites.clone(), std::iter::empty()).map_err(|invalid| {
            CompositeEqualityRefusal::Declaration {
                declaration: invalid.declaration,
                cause: invalid.cause.into(),
            }
        })?;

    let left_operand = match left_target.clone() {
        Some(target) => EqualityOperand::converted(left_source.clone(), target),
        None => EqualityOperand::typed(left_source.clone()),
    };
    let right_operand = match right_target.clone() {
        Some(target) => EqualityOperand::converted(right_source.clone(), target),
        None => EqualityOperand::typed(right_source.clone()),
    };
    let checked = environment
        .check_equality(item.operator.to_runtime(), left_operand, right_operand)
        .map_err(|ill_typed| CompositeEqualityRefusal::IllTyped {
            cause: ill_typed.cause.into(),
        })?;

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
) -> Result<&quire_contract_ir::CompleteContractNodeV2, CompositeEqualityRefusal> {
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
        CompleteLoweringRecordV2::Failed {
            limit, consumed, ..
        } => Err(CompositeEqualityRefusal::LoweringWorkExhausted {
            limit: *limit,
            consumed: *consumed,
        }),
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

/// Each `binary` operand names its own static type through `literal.type`
/// -- IR-216 requires that member on every literal and validates only that
/// it resolves to a real node, never cross-checking it against `value_kind`,
/// so it is free for this generator to read on its own terms. A `reference`
/// operand is deliberately not read here: IR's own admission-time
/// `argument_family`/`check_operands` check resolves and enforces a
/// `reference` operand's family against the operation's declared operand
/// family, so a type node named by `reference` never admits at all, before
/// this generator would run.
///
/// This is the generator's only read of body content (FR-018's Behavior
/// clause "disagrees with its descriptor's arity or operand types"): the
/// type used to build the runtime call always comes from the descriptor,
/// never from this value, so disagreement here is refused before either
/// operand's type is resolved.
fn operand_type_id(term: &Value) -> Option<CheckedNodeId> {
    if term.get("term")?.as_str()? != "literal" {
        return None;
    }
    serde_json::from_value(term.get("type")?.clone()).ok()
}

/// Compare each body operand's declared type against the descriptor's for
/// that position, left then right, refusing at the first disagreement.
fn check_operand_types(
    arguments: &[Value],
    item: &CompositeEqualityItem,
) -> Result<(), CompositeEqualityRefusal> {
    let expected = [&item.left.source_type, &item.right.source_type];
    for (position, (argument, expected_type)) in arguments.iter().zip(expected).enumerate() {
        let found = operand_type_id(argument);
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
    let node = lookup(graph, type_id)?;
    match CheckedNodeTag::from_wire(&node.node_tag) {
        Some(CheckedNodeTag::ScalarType) => resolve_scalar(bounds_by_type, type_id, node),
        Some(CheckedNodeTag::CompositeType) => {
            resolve_composite(graph, bounds_by_type, closure, type_id, node)
        }
        _ => Err(CompositeEqualityRefusal::Unsupported {
            unsupported_node_id: type_id.clone(),
            node_tag: "unknown",
        }),
    }
}

fn resolve_scalar(
    bounds_by_type: &BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2>>,
    type_id: &CheckedNodeId,
    node: &CheckedSemanticNodeV2,
) -> Result<ValueType, CompositeEqualityRefusal> {
    let bound = |form: &'static str| -> Result<&[Value], CompositeEqualityRefusal> {
        let candidates: Vec<_> = bounds_by_type
            .get(type_id)
            .into_iter()
            .flatten()
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
            let candidates: Vec<_> = bounds_by_type
                .get(type_id)
                .into_iter()
                .flatten()
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

fn resolve_composite(
    graph: &Graph<'_>,
    bounds_by_type: &BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2>>,
    closure: &mut TypeClosure,
    type_id: &CheckedNodeId,
    node: &CheckedSemanticNodeV2,
) -> Result<ValueType, CompositeEqualityRefusal> {
    match &*node.semantic_form {
        "record" | "tuple" => {
            let key = node_key(type_id)?;
            if !closure.composites.contains_key(&key) && !closure.in_progress.contains(&key) {
                closure.in_progress.insert(key);
                let members = aggregate_members(&node.body).ok_or_else(|| {
                    CompositeEqualityRefusal::MalformedComposite {
                        composite: type_id.clone(),
                    }
                })?;
                let shape = if node.semantic_form.as_ref() == "record" {
                    let mut fields = Vec::with_capacity(members.len());
                    for member in members {
                        let (name, target) = read_binding(member).ok_or_else(|| {
                            CompositeEqualityRefusal::MalformedComposite {
                                composite: type_id.clone(),
                            }
                        })?;
                        let target_node = lookup(graph, &target)?;
                        let (value_type, presence) = if is_option_node(target_node) {
                            let payload_members =
                                aggregate_members(&target_node.body).ok_or_else(|| {
                                    CompositeEqualityRefusal::MalformedComposite {
                                        composite: target.clone(),
                                    }
                                })?;
                            let [payload_term] = payload_members else {
                                return Err(CompositeEqualityRefusal::MalformedComposite {
                                    composite: target.clone(),
                                });
                            };
                            let payload_target = read_reference(payload_term).ok_or_else(|| {
                                CompositeEqualityRefusal::MalformedComposite {
                                    composite: target.clone(),
                                }
                            })?;
                            let value_type =
                                resolve_type(graph, bounds_by_type, closure, &payload_target)?;
                            (value_type, Presence::Optional)
                        } else {
                            let value_type = resolve_type(graph, bounds_by_type, closure, &target)?;
                            (value_type, Presence::Required)
                        };
                        fields.push(FieldDeclaration::new(name, value_type, presence));
                    }
                    CompositeShape::Record(fields)
                } else {
                    let mut positions = Vec::with_capacity(members.len());
                    for member in members {
                        let target = read_reference(member).ok_or_else(|| {
                            CompositeEqualityRefusal::MalformedComposite {
                                composite: type_id.clone(),
                            }
                        })?;
                        positions.push(resolve_type(graph, bounds_by_type, closure, &target)?);
                    }
                    CompositeShape::Tuple(positions)
                };
                closure.in_progress.remove(&key);
                closure.composites.insert(
                    key,
                    CompositeDeclaration::new(key, node.node_id.digest.to_string(), shape),
                );
                closure.node_ids.push(type_id.clone());
            }
            Ok(ValueType::Composite(key))
        }
        "option" => {
            let members = aggregate_members(&node.body).ok_or_else(|| {
                CompositeEqualityRefusal::MalformedComposite {
                    composite: type_id.clone(),
                }
            })?;
            let [payload_term] = members else {
                return Err(CompositeEqualityRefusal::MalformedComposite {
                    composite: type_id.clone(),
                });
            };
            let payload_target = read_reference(payload_term).ok_or_else(|| {
                CompositeEqualityRefusal::MalformedComposite {
                    composite: type_id.clone(),
                }
            })?;
            let payload = resolve_type(graph, bounds_by_type, closure, &payload_target)?;
            Ok(ValueType::option(payload))
        }
        form @ ("sequence" | "set" | "bag" | "ordered_set") => {
            let members = aggregate_members(&node.body).ok_or_else(|| {
                CompositeEqualityRefusal::MalformedComposite {
                    composite: type_id.clone(),
                }
            })?;
            let [element_term, bounds_term] = members else {
                return Err(CompositeEqualityRefusal::MalformedComposite {
                    composite: type_id.clone(),
                });
            };
            let element_target = read_reference(element_term).ok_or_else(|| {
                CompositeEqualityRefusal::MalformedComposite {
                    composite: type_id.clone(),
                }
            })?;
            let element = resolve_type(graph, bounds_by_type, closure, &element_target)?;
            let bounds_target = read_reference(bounds_term).ok_or_else(|| {
                CompositeEqualityRefusal::MalformedComposite {
                    composite: type_id.clone(),
                }
            })?;
            let bounds_node = lookup(graph, &bounds_target)?;
            if &*bounds_node.semantic_form != "collection_bounds" {
                return Err(CompositeEqualityRefusal::UnreadableBound {
                    bound: bounds_target,
                });
            }
            let bound_members = aggregate_members(&bounds_node.body).ok_or_else(|| {
                CompositeEqualityRefusal::UnreadableBound {
                    bound: bounds_target.clone(),
                }
            })?;
            let [minimum, maximum] = bound_members else {
                return Err(CompositeEqualityRefusal::UnreadableBound {
                    bound: bounds_target,
                });
            };
            let (minimum, maximum) = (
                literal_count(minimum).ok_or_else(|| {
                    CompositeEqualityRefusal::UnreadableBound {
                        bound: bounds_target.clone(),
                    }
                })?,
                literal_count(maximum).ok_or_else(|| {
                    CompositeEqualityRefusal::UnreadableBound {
                        bound: bounds_target.clone(),
                    }
                })?,
            );
            let cardinality = CardinalityBound::new(minimum, maximum).map_err(|_| {
                CompositeEqualityRefusal::UnreadableBound {
                    bound: bounds_target,
                }
            })?;
            let kind = match form {
                "sequence" => CollectionKind::Sequence,
                "set" => CollectionKind::Set,
                "bag" => CollectionKind::Bag,
                _ => CollectionKind::OrderedSet,
            };
            Ok(ValueType::collection(CollectionType::new(
                kind,
                element,
                cardinality,
            )))
        }
        "reference" => Err(CompositeEqualityRefusal::BlockedOnUpstream {
            unsupported_node_id: type_id.clone(),
            node_tag: "composite_type.reference",
            issue: UpstreamBlocker::QuireSpecLanguage120,
        }),
        _ => Err(CompositeEqualityRefusal::Unsupported {
            unsupported_node_id: type_id.clone(),
            node_tag: "composite_type",
        }),
    }
}

fn read_integer_range(members: &[Value]) -> Option<IntegerInterval> {
    let [lower, upper] = members else {
        return None;
    };
    IntegerInterval::new(
        crate::exact_scalar::literal_integer(lower)?,
        crate::exact_scalar::literal_integer(upper)?,
    )
    .ok()
}

fn read_rational_range(members: &[Value]) -> Option<RationalDomain> {
    let [numerator_lower, numerator_upper, denominator_lower, denominator_upper] = members else {
        return None;
    };
    RationalDomain::new(
        IntegerInterval::new(
            crate::exact_scalar::literal_integer(numerator_lower)?,
            crate::exact_scalar::literal_integer(numerator_upper)?,
        )
        .ok()?,
        IntegerInterval::new(
            crate::exact_scalar::literal_integer(denominator_lower)?,
            crate::exact_scalar::literal_integer(denominator_upper)?,
        )
        .ok()?,
    )
    .ok()
}

fn read_decimal_range(members: &[Value]) -> Option<DecimalType> {
    let [lower, upper, min_scale, max_scale, rounding] = members else {
        return None;
    };
    DecimalType::new(
        crate::exact_scalar::literal_integer(lower)?,
        crate::exact_scalar::literal_integer(upper)?,
        literal_count(min_scale)?,
        literal_count(max_scale)?,
        RoundingMode::from_code(literal_text(rounding)?)?,
    )
    .ok()
}

fn read_text_bounds(members: &[Value]) -> Option<TextType> {
    let [min, max, profile] = members else {
        return None;
    };
    TextType::new(
        literal_count(min)?,
        literal_count(max)?,
        TextProfile::from_code(literal_text(profile)?)?,
    )
    .ok()
}

fn literal_text(term: &Value) -> Option<&str> {
    if term.get("term")?.as_str()? != "literal" || term.get("value_kind")?.as_str()? != "text" {
        return None;
    }
    term.get("value")?.as_str()
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
";

/// Reconstructs a canonical decimal integer literal at generation-time
/// output. Emitted only when a rendered bound actually calls `integer(...)`
/// (a bounded `Int` or `Decimal` value type reached by some generated item);
/// unconditional emission would be unused, and hence dead code under the
/// generated crate's own `-D warnings` build, for a request that reaches
/// neither.
const INTEGER_HELPER: &str = "\n\
/// A canonical decimal integer literal, re-parsed from generation-time
/// output. Reconstructing it can fail only if the emitted spelling does not
/// round-trip through `rt::Integer`'s own parser, which generation already
/// validated once through the identical parser over the identical spelling.
fn integer(spelling: &str) -> rt::Integer {
    spelling
        .parse()
        .expect(\"generation-time validation guarantees this literal reconstructs\")
}
";

#[derive(Default)]
struct SourceBuilder {
    functions: String,
}

impl SourceBuilder {
    fn item(&mut self, symbol: &str, item: &CompositeEqualityItem, generated: &CheckedItem<'_>) {
        // Every reachable record/tuple declaration, in the same `NodeKey`
        // order `TypeEnvironment::new` admitted at generation time.
        let composites: String = generated
            .composites
            .iter()
            .map(|declaration| format!("{},\n        ", render_composite_declaration(declaration)))
            .collect();

        self.functions.push_str(&format!(
            "\nfn composites_{symbol}() -> Vec<rt::CompositeDeclaration> {{\n    vec![{composites}]\n}}\n"
        ));

        let left_source = render_value_type(&generated.left_source);
        let left_target = match &generated.left_target {
            Some(target) => format!("Some({})", render_value_type(target)),
            None => "None".to_owned(),
        };
        let right_source = render_value_type(&generated.right_source);
        let right_target = match &generated.right_target {
            Some(target) => format!("Some({})", render_value_type(target)),
            None => "None".to_owned(),
        };
        self.functions.push_str(&format!(
            "\nfn left_source_{symbol}() -> rt::ValueType {{\n    {left_source}\n}}\n\
             fn left_target_{symbol}() -> Option<rt::ValueType> {{\n    {left_target}\n}}\n\
             fn right_source_{symbol}() -> rt::ValueType {{\n    {right_source}\n}}\n\
             fn right_target_{symbol}() -> Option<rt::ValueType> {{\n    {right_target}\n}}\n"
        ));

        self.functions.push_str(&format!(
            "\n/// Node `{}`: `{}`. Its operation identity is caller-declared: CheckedPackage V2 carries\n\
             /// an operator class, not this operator law.\n\
             pub fn environment_{symbol}() -> Result<rt::TypeEnvironment, rt::InvalidDeclaration> {{\n\
             \x20   rt::TypeEnvironment::new(composites_{symbol}(), core::iter::empty::<rt::ObjectTypeDeclaration>())\n}}\n",
            item.node_id.digest, item.operator.identity()
        ));

        self.functions.push_str(&format!(
            "\n/// Node `{}`, environment-checked oracle for `{}`.\npub fn oracle_{symbol}(\n    environment: &rt::TypeEnvironment,\n    left: &rt::Value,\n    right: &rt::Value,\n    meter: &mut rt::Meter,\n) -> rt::Outcome<bool> {{\n\
             \x20   let left_target = left_target_{symbol}();\n\
             \x20   let left_comparison = left_target.clone().unwrap_or_else(left_source_{symbol});\n\
             \x20   if environment.check_type(&left_comparison).is_err() {{\n\
             \x20       return rt::Outcome::Refused(rt::Refusal::CheckedInvariant);\n\x20   }}\n\
             \x20   let right_target = right_target_{symbol}();\n\
             \x20   let right_comparison = right_target.clone().unwrap_or_else(right_source_{symbol});\n\
             \x20   if environment.check_type(&right_comparison).is_err() {{\n\
             \x20       return rt::Outcome::Refused(rt::Refusal::CheckedInvariant);\n\x20   }}\n\
             \x20   let left_operand = match left_target {{\n\
             \x20       Some(target) => rt::EqualityOperand::converted(left_source_{symbol}(), target),\n\
             \x20       None => rt::EqualityOperand::typed(left_source_{symbol}()),\n\x20   }};\n\
             \x20   let right_operand = match right_target {{\n\
             \x20       Some(target) => rt::EqualityOperand::converted(right_source_{symbol}(), target),\n\
             \x20       None => rt::EqualityOperand::typed(right_source_{symbol}()),\n\x20   }};\n\
             \x20   let checked = match environment.check_equality({}, left_operand, right_operand) {{\n\
             \x20       Ok(checked) => checked,\n\
             \x20       Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant),\n\x20   }};\n\
             \x20   checked.evaluate(left, right, meter)\n}}\n",
            item.node_id.digest,
            item.operator.identity(),
            item.operator.path()
        ));
    }

    fn finish(self, package_id: &CheckedSemanticId) -> String {
        let mut source = format!("// Source package: {}\n", package_id.digest);
        source.push_str(SOURCE_HEADER);
        if self.functions.contains("integer(\"") {
            source.push_str(INTEGER_HELPER);
        }
        source.push_str(&self.functions);
        source
    }
}

/// Render one admitted `CompositeDeclaration` as a Rust expression of type
/// `rt::CompositeDeclaration`.
fn render_composite_declaration(declaration: &CompositeDeclaration) -> String {
    let key = render_key(declaration.key());
    let shape = match declaration.shape() {
        CompositeShape::Record(fields) => {
            let rendered: String = fields
                .iter()
                .map(|field| {
                    format!(
                        "rt::FieldDeclaration::new({:?}, {}, {}), ",
                        field.name(),
                        render_value_type(field.value_type()),
                        presence_path(field.presence())
                    )
                })
                .collect();
            format!("rt::CompositeShape::Record(vec![{rendered}])")
        }
        CompositeShape::Tuple(positions) => {
            let rendered: String = positions
                .iter()
                .map(|value_type| format!("{}, ", render_value_type(value_type)))
                .collect();
            format!("rt::CompositeShape::Tuple(vec![{rendered}])")
        }
    };
    format!(
        "rt::CompositeDeclaration::new({key}, {:?}, {shape})",
        declaration.name()
    )
}

fn presence_path(presence: Presence) -> &'static str {
    match presence {
        Presence::Required => "rt::Presence::Required",
        Presence::Optional => "rt::Presence::Optional",
    }
}

/// Render one `ValueType` as a Rust expression of type `rt::ValueType`.
fn render_value_type(value_type: &ValueType) -> String {
    match value_type {
        ValueType::Boolean => "rt::ValueType::Boolean".to_owned(),
        ValueType::Integer => "rt::ValueType::Integer".to_owned(),
        ValueType::Int(interval) => format!(
            "rt::ValueType::Int({})",
            render_interval(interval)
        ),
        ValueType::Rational(domain) => format!(
            "rt::ValueType::Rational(rt::RationalDomain::new({}, {}).expect(\"generation-time validation guarantees this bound reconstructs\"))",
            render_interval(domain.numerator()),
            render_interval(domain.denominator())
        ),
        ValueType::Decimal(decimal) => format!(
            "rt::ValueType::Decimal({})",
            render_decimal_type(decimal)
        ),
        ValueType::Float(rt::IeeeWidth::Binary32) => {
            "rt::ValueType::Float(rt::IeeeWidth::Binary32)".to_owned()
        }
        ValueType::Float(rt::IeeeWidth::Binary64) => {
            "rt::ValueType::Float(rt::IeeeWidth::Binary64)".to_owned()
        }
        ValueType::Quantity(_) => {
            unreachable!("quantity leaves are refused at generation time")
        }
        ValueType::Text(text_type) => format!(
            "rt::ValueType::Text(rt::TextType::new({}, {}, {}).expect(\"generation-time validation guarantees this bound reconstructs\"))",
            text_type.min(),
            text_type.max(),
            profile_path(text_type.profile())
        ),
        ValueType::Enum(key) => format!("rt::ValueType::Enum({})", render_key(*key)),
        ValueType::Option(payload) => {
            format!("rt::ValueType::option({})", render_value_type(payload))
        }
        ValueType::Composite(key) => format!("rt::ValueType::Composite({})", render_key(*key)),
        ValueType::Collection(collection) => format!(
            "rt::ValueType::collection(rt::CollectionType::new({}, {}, rt::CardinalityBound::new({}, {}).expect(\"generation-time validation guarantees this bound reconstructs\")))",
            collection_kind_path(collection.kind()),
            render_value_type(collection.element()),
            collection.bound().minimum(),
            collection.bound().maximum()
        ),
        ValueType::Reference(_) => {
            unreachable!("reference operands are refused at generation time")
        }
    }
}

fn render_interval(interval: &IntegerInterval) -> String {
    format!(
        "rt::IntegerInterval::new(integer(\"{}\"), integer(\"{}\")).expect(\"generation-time validation guarantees this bound reconstructs\")",
        interval.lower(),
        interval.upper()
    )
}

fn render_decimal_type(decimal: &DecimalType) -> String {
    format!(
        "rt::DecimalType::new(integer(\"{}\"), integer(\"{}\"), {}, {}, {}).expect(\"generation-time validation guarantees this bound reconstructs\")",
        decimal.lower(),
        decimal.upper(),
        decimal.min_scale(),
        decimal.max_scale(),
        rounding_path(decimal.rounding())
    )
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

fn collection_kind_path(kind: CollectionKind) -> &'static str {
    match kind {
        CollectionKind::Sequence => "rt::CollectionKind::Sequence",
        CollectionKind::Set => "rt::CollectionKind::Set",
        CollectionKind::Bag => "rt::CollectionKind::Bag",
        CollectionKind::OrderedSet => "rt::CollectionKind::OrderedSet",
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
    }
}

fn manifest() -> String {
    format!(
        "[package]\nname = \"{COMPOSITE_EQUALITY_CRATE_NAME}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[lib]\npath = \"src/lib.rs\"\n\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\", features = [\"exact\"] }}\n\n[lints.rust]\nunsafe_code = \"forbid\"\n\n[workspace]\n"
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

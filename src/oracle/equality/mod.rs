//! Exact composite/structural equality oracle generation from one admitted
//! ContractPackage V1 (FR-018, FR-039).
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
//! never compares two values itself: it reconstructs the record/tuple/union
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
//! | `union` | owner-authenticated member `binding`s, each grouping its payload type references in declaration order |
//! | `option` | one `reference` to the payload type node |
//! | `sequence` | one element `reference`, with cardinality from a separate `collection_bounds` node |
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
//! `EnvironmentError::Declaration`, which preserves the complete typed QSV
//! `EnvironmentFailure`, including bounded admission stops without inventing a declaration cause.
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

use crate::core::artifact::{
    Artifact, MAX_ARTIFACTS, MAX_ARTIFACT_BYTES, MAX_BUNDLE_BYTES, MAX_GENERATED_SOURCE_BYTES,
};
use crate::core::contract_package::{
    ContractRequestPosition, PackageItem, PackageItems, PackageRequestError,
};
use crate::core::naming::unique_names;
use crate::core::profile::equality_oracle_crate_manifest;
use crate::oracle::checked_graph::{
    aggregate_members, bound_members, literal_count, CheckedGraph as Graph, GraphBody,
};
use crate::oracle::claim::{ClaimDisposition, ClaimMap, OracleGenerationError, UpstreamBlocker};
use crate::oracle::direct_unsupported_form;
use crate::oracle::scalar::{
    read_decimal_range, read_integer_range, read_rational_range, read_text_bounds,
    COLLECTION_BOUNDS_MEMBERS,
};
#[cfg(test)]
use quire_contract_model::CheckedPackageV2;
use quire_contract_model::{
    ApplicationOperator, CheckedNodeId, CheckedNodeTag, CheckedSemanticId, CheckedSemanticNodeV2,
    CheckedSourceMapEntry, CheckedTerm, ContractFeatureRecordV1, ContractPackageV1,
};
use quire_exact::{
    self as exact, CardinalityBound, CollectionKind, CollectionType, DecimalType, IllTypedCause,
    IntegerInterval, NodeKey, Presence, RoundingMode, TextProfile, ValueType,
};
use quire_semantic_value::declaration::{
    self as sv, CompositeDeclaration, CompositeShape, EqualityOperand, EqualitySchedule,
    FieldDeclaration, TypeEnvironment,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

mod admitted;
mod resolution;
#[cfg(test)]
mod union_tests;
mod units;

/// Work budget for lowering one requested equality expression node.
pub const COMPOSITE_EQUALITY_LOWERING_WORK_LIMIT: u64 = 65_536;

/// Type-node entries permitted while resolving one equality item.
pub const COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT: u64 = 65_536;

/// Caller limits for one composite equality generation request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompositeEqualityLimits {
    /// Maximum charged type-node entries for each requested item.
    pub type_resolution_work_units: u64,
    /// Maximum bytes of generated Rust source across the request.
    pub source_bytes: usize,
}

impl Default for CompositeEqualityLimits {
    fn default() -> Self {
        Self {
            type_resolution_work_units: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
            source_bytes: MAX_GENERATED_SOURCE_BYTES,
        }
    }
}

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
    fn to_runtime(self) -> sv::EqualityOperator {
        match self {
            Self::Equal => sv::EqualityOperator::Equal,
            Self::NotEqual => sv::EqualityOperator::NotEqual,
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
            Self::Equal => "sv::EqualityOperator::Equal",
            Self::NotEqual => "sv::EqualityOperator::NotEqual",
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

/// A serializable mirror of `quire_exact::IllTypedCause`.
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

impl TryFrom<sv::RecursionEdges> for RecursionEdgesKind {
    type Error = OracleGenerationError;

    fn try_from(edges: sv::RecursionEdges) -> Result<Self, Self::Error> {
        match edges {
            sv::RecursionEdges::Unnamed => Ok(Self::Unnamed),
            sv::RecursionEdges::NonEscaping => Ok(Self::NonEscaping),
        }
    }
}

/// A serializable mirror of `quire_semantic_value::declaration::DeclarationCause`.
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

impl TryFrom<sv::DeclarationCause> for DeclarationRefusalCause {
    type Error = OracleGenerationError;

    fn try_from(cause: sv::DeclarationCause) -> Result<Self, Self::Error> {
        Ok(match cause {
            sv::DeclarationCause::DuplicateKey => Self::DuplicateKey,
            sv::DeclarationCause::DuplicateMember(name) => Self::DuplicateMember { name },
            sv::DeclarationCause::UnknownDeclaration(key) => Self::UnknownDeclaration {
                key: hex_digest(&key),
            },
            sv::DeclarationCause::Type(cause) => Self::Type {
                cause: cause.try_into()?,
            },
            sv::DeclarationCause::Recursion { edges, cycle } => Self::Recursion {
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
    /// Reached nominal identity requires a genuine admitted owner capability.
    MissingAdmittedContext {
        /// First nominal declaration reached by the charged traversal.
        type_node_id: CheckedNodeId,
    },
    /// The owner cannot resolve a declaration or case in the selected context.
    UnknownEnum {
        /// The exact wire declaration identity requested.
        declaration: String,
        /// The requested case, if any.
        case: Option<String>,
    },
    /// The requested declaration belongs to an imported package.
    ForeignEnum {
        /// The exact wire declaration identity requested.
        declaration: String,
        /// The owning package domain.
        package_domain: String,
        /// The owning package digest.
        package_digest: String,
    },
    /// Admitted enum metadata disagrees with the retained IR declaration.
    EnumContextMismatch {
        /// The enum declaration whose name, ordering or cases disagree.
        declaration: CheckedNodeId,
    },
    /// The owner cannot resolve the requested nominal composite declaration.
    UnknownComposite {
        /// Exact wire declaration identity requested.
        declaration: String,
    },
    /// The requested nominal composite belongs to an imported package.
    ForeignComposite {
        /// Exact wire declaration identity requested.
        declaration: String,
        /// Owning package domain.
        package_domain: String,
        /// Owning package digest.
        package_digest: String,
    },
    /// The owner admitted this node as a different declaration kind.
    WrongAdmittedKind {
        /// Exact wire declaration identity requested.
        declaration: String,
    },
    /// Admitted composite form or qualified name disagrees with the retained IR.
    CompositeContextMismatch {
        /// The declaration whose owner metadata disagrees.
        declaration: CheckedNodeId,
    },
    /// Exact refusal and available source occurrences from admitted FR-330 accounting.
    PackageAccounting {
        /// Reader-provided accounting disposition.
        record: Box<ContractFeatureRecordV1>,
        /// Reader-provided source occurrences for this request position.
        occurrences: Vec<CheckedSourceMapEntry>,
    },
    /// Selected external targets require an external resolver; their identities are preserved.
    SelectedDependencyUnsupported {
        /// Ordered `(package_id, target_node_id)` pairs from the admitted closure.
        references: Vec<(CheckedSemanticId, CheckedNodeId)>,
    },
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
    /// Identity of the one Contract IR package that admitted this claim.
    pub contract_package_identity: CheckedSemanticId,
    /// Exact requested node key.
    pub node_id: CheckedNodeId,
    /// Package id.
    pub package_id: CheckedSemanticId,
    /// Semantic type key of the expression node.
    pub semantic_type: CheckedNodeId,
    /// Exact source correspondence.
    pub source_map: Vec<CheckedSourceMapEntry>,
    /// Reachable claim keys.
    pub claims: Vec<CheckedNodeId>,
    /// Reachable bounded-domain keys.
    pub bounds: Vec<CheckedNodeId>,
    /// Every reached key except the root.
    pub dependencies: Vec<CheckedNodeId>,
    /// The descriptor the request supplied: operator and both operands'
    /// source and conversion-target node ids, read from the request, never
    /// from the generated source.
    pub descriptor: RecordedDescriptor,
    /// Every V2 node id whose declaration entered the reconstructed closure,
    /// ascending by digest domain then digest.
    pub declaration_keys: Vec<CheckedNodeId>,
    /// The original owner-issued kernel declaration keys, in the corresponding
    /// `declaration_keys` order; these are distinct from the V2 composite node ids.
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

/// Generate composite equality oracles for `items` from one admitted FR-330 package.
///
/// Each `positions` entry names the matching item in `requested`; accounting
/// and source occurrences are read at that exact index. A mismatch refuses
/// the whole call before publication.
///
/// Fails as a whole for an invalid work-limit setting, generated-source or
/// publication bounds, claim-map serialization, or an unknown Contract Runtime
/// `#[non_exhaustive]` variant. Item-specific problems are refusals in the
/// claim map.
pub fn generate_composite_equality_oracles(
    package: &ContractPackageV1,
    admitted: Option<&qsl_replay::AdmittedContext>,
    requested: &[ContractRequestPosition],
    positions: &[usize],
    items: &[CompositeEqualityItem],
    limits: CompositeEqualityLimits,
) -> Result<CompositeEqualityOracles, OracleGenerationError> {
    if limits.type_resolution_work_units == u64::MAX {
        return Err(OracleGenerationError::InvalidTypeResolutionWorkLimit);
    }
    let accounting =
        PackageItems::new(package, requested).map_err(OracleGenerationError::PackageRequest)?;
    if positions.len() != items.len() {
        return Err(OracleGenerationError::PackageRequest(
            PackageRequestError::ItemPositions {
                items: items.len(),
                positions: positions.len(),
            },
        ));
    }
    let graph = Graph::new(package);
    let mut bounds_by_type: BTreeMap<
        &CheckedNodeId,
        Vec<&CheckedSemanticNodeV2<serde_json::Value>>,
    > = BTreeMap::new();
    for node in package.wire().semantic_graph.nodes.iter() {
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
    let mut by_key: BTreeMap<DescriptorKey, (usize, &CompositeEqualityItem)> = BTreeMap::new();
    for (item, &request_index) in items.iter().zip(positions) {
        let accounted = accounting
            .bound_driver_node(request_index)
            .map_err(OracleGenerationError::PackageRequest)?;
        if accounted != &item.node_id {
            return Err(OracleGenerationError::PackageRequest(
                PackageRequestError::Identity {
                    request_index,
                    requested: item.node_id.clone(),
                    accounted: accounted.clone(),
                },
            ));
        }
        let key = DescriptorKey::of(item);
        *counts.entry(key.clone()).or_insert(0) += 1;
        by_key.entry(key).or_insert((request_index, item));
    }

    let expected_package = &package.wire().package.source.package_id;
    if let Some(admitted) = admitted {
        let actual_package = admitted.package_id().record();
        if expected_package.domain.as_ref() != actual_package.domain().as_str()
            || expected_package.digest.as_ref() != actual_package.hex()
        {
            return Err(OracleGenerationError::ContextPackageMismatch {
                expected: expected_package.clone(),
                actual_domain: actual_package.domain().as_str().to_owned(),
                actual_digest: actual_package.hex(),
            });
        }
    }
    let mut source = SourceBuilder::new(limits.source_bytes)?;
    source.expected_package = Some(expected_package.clone());
    let mut claims = Vec::with_capacity(by_key.len());
    // Generated claims as (claim position, key, item, checked item), rendered once every
    // generated oracle is named.
    let mut pending = Vec::new();
    for (key, (request_index, item)) in by_key {
        let duplicate = counts.get(&key).copied().unwrap_or(0) > 1;
        let result = if duplicate {
            ClaimDisposition::Refused {
                refusal: CompositeEqualityRefusal::DuplicateRequest,
            }
        } else if let Some(node_tag) = graph
            .node(&item.node_id)
            .and_then(|node| direct_unsupported_form(node, graph.term(&item.node_id)))
        {
            ClaimDisposition::Refused {
                refusal: CompositeEqualityRefusal::Unsupported {
                    unsupported_node_id: item.node_id.clone(),
                    node_tag,
                },
            }
        } else {
            let accounted_item = accounting
                .at(request_index, &item.node_id)
                .map_err(OracleGenerationError::PackageRequest)?;
            match accounted_item {
                PackageItem::Refused {
                    record,
                    occurrences,
                } => ClaimDisposition::Refused {
                    refusal: CompositeEqualityRefusal::PackageAccounting {
                        record: Box::new(record.clone()),
                        occurrences: occurrences.to_vec(),
                    },
                },
                PackageItem::Lowered { closure, .. } if !closure.selected_references.is_empty() => {
                    ClaimDisposition::Refused {
                        refusal: CompositeEqualityRefusal::SelectedDependencyUnsupported {
                            references: closure
                                .selected_references
                                .into_iter()
                                .map(|reference| (reference.package, reference.node))
                                .collect(),
                        },
                    }
                }
                PackageItem::Lowered {
                    closure,
                    occurrences,
                    ..
                } => match check_item(
                    admitted,
                    &graph,
                    &bounds_by_type,
                    closure.root,
                    item,
                    limits.type_resolution_work_units,
                ) {
                    Ok(generated) => {
                        let runtime_by_node = generated
                            .composites
                            .iter()
                            .map(|declaration| {
                                let binding = generated
                                    .composite_bindings
                                    .get(&declaration.key())
                                    .ok_or(OracleGenerationError::InvalidPlanState)?;
                                Ok((&binding.node, declaration.key()))
                            })
                            .collect::<Result<BTreeMap<_, _>, OracleGenerationError>>()?;
                        let runtime_keys = generated
                            .declaration_keys
                            .iter()
                            .map(|id| {
                                runtime_by_node
                                    .get(id)
                                    .map(ToString::to_string)
                                    .ok_or(OracleGenerationError::InvalidPlanState)
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        let claim = ClaimDisposition::Generated(Box::new(
                            GeneratedCompositeEqualityClaim {
                                environment_symbol: String::new(),
                                oracle_symbol: String::new(),
                                contract_package_identity: package.identity().clone(),
                                node_id: item.node_id.clone(),
                                package_id: package.wire().package.source.package_id.clone(),
                                semantic_type: closure.root.semantic_type.clone(),
                                source_map: occurrences.to_vec(),
                                claims: closure.claims,
                                bounds: closure.bounds,
                                dependencies: closure
                                    .nodes
                                    .iter()
                                    .filter(|entry| entry.node.node_id != item.node_id)
                                    .map(|entry| entry.node.node_id.clone())
                                    .collect(),
                                descriptor: recorded_descriptor(item),
                                declaration_keys: generated.declaration_keys.clone(),
                                declaration_runtime_keys: runtime_keys,
                                schedule: generated.checked.schedule().try_into()?,
                            },
                        ));
                        pending.push((claims.len(), key, item, generated));
                        claim
                    }
                    Err(ItemCheckError::Refusal(refusal)) => ClaimDisposition::Refused { refusal },
                    // An RT `#[non_exhaustive]` enum yielded a variant this
                    // generator does not know: not this one item's refusal, so it
                    // aborts the whole generation (see
                    // `OracleGenerationError::UnknownRuntimeVariant`'s own doc).
                    Err(ItemCheckError::Generation(error)) => return Err(error),
                },
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
        package_id: package.wire().package.source.package_id.clone(),
        blocked: vec![UpstreamBlocker::OperationIdentityNotConsumed],
        items: claims,
    };
    let mut subjects = claim_map
        .items
        .iter()
        .filter(|claim| matches!(&claim.result, ClaimDisposition::Generated(_)))
        .map(|claim| crate::core::provenance::node_subject(&claim.node_id))
        .collect::<Vec<_>>();
    subjects.extend(
        source
            .inline_composite_subjects
            .iter()
            .map(crate::core::provenance::node_subject),
    );
    let source_artifacts = source
        .finish(limits.source_bytes)?
        .into_iter()
        .map(|mut artifact| {
            if artifact.path == "src/lib.rs" {
                artifact.contents = crate::core::provenance::with_rust_header(
                    artifact.contents,
                    subjects.iter().map(String::as_str),
                );
            }
            artifact
        })
        .collect::<Vec<_>>();
    if source_artifacts.iter().fold(0usize, |total, artifact| {
        total.saturating_add(artifact.contents.len())
    }) > limits.source_bytes
    {
        return Err(source_too_large(limits.source_bytes));
    }
    let mut map_bytes = serde_json::to_vec_pretty(&claim_map)
        .map_err(|_| OracleGenerationError::ClaimMapSerialization)?;
    map_bytes.push(b'\n');
    let map_text =
        String::from_utf8(map_bytes).map_err(|_| OracleGenerationError::ClaimMapSerialization)?;
    let mut artifacts = Vec::with_capacity(source_artifacts.len() + 2);
    artifacts.push(Artifact::new(
        "Cargo.toml",
        equality_oracle_crate_manifest(COMPOSITE_EQUALITY_CRATE_NAME),
    ));
    artifacts.extend(source_artifacts);
    artifacts.push(Artifact::new("claim-map.json", map_text));
    if artifacts.len() > MAX_ARTIFACTS {
        return Err(OracleGenerationError::ArtifactCountExceeded {
            count: artifacts.len(),
            limit: MAX_ARTIFACTS,
        });
    }
    let mut bundle_bytes = 0usize;
    for artifact in &artifacts {
        if artifact.contents.len() > MAX_ARTIFACT_BYTES {
            return Err(OracleGenerationError::ArtifactTooLarge {
                bytes: artifact.contents.len(),
                limit: MAX_ARTIFACT_BYTES,
            });
        }
        bundle_bytes = bundle_bytes.saturating_add(artifact.contents.len());
    }
    if bundle_bytes > MAX_BUNDLE_BYTES {
        return Err(OracleGenerationError::BundleTooLarge {
            bytes: bundle_bytes,
            limit: MAX_BUNDLE_BYTES,
        });
    }
    Ok(CompositeEqualityOracles {
        artifacts,
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
struct CheckedItem {
    checked: sv::CheckedEquality,
    left_source: ValueType,
    left_target: Option<ValueType>,
    right_source: ValueType,
    right_target: Option<ValueType>,
    /// Every reachable nominal composite declaration, ascending by `NodeKey`, the
    /// same order `TypeEnvironment::new` admitted.
    composites: Vec<CompositeDeclaration>,
    declaration_keys: Vec<CheckedNodeId>,
    enums: Vec<admitted::EnumBinding>,
    composite_bindings: BTreeMap<NodeKey, admitted::CompositeBinding>,
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

fn check_item<Body: GraphBody>(
    admitted: Option<&qsl_replay::AdmittedContext>,
    graph: &Graph<'_, Body>,
    bounds_by_type: &BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2<Body>>>,
    node: &CheckedSemanticNodeV2<Body>,
    item: &CompositeEqualityItem,
    type_resolution_work_units: u64,
) -> Result<CheckedItem, ItemCheckError> {
    if let Some(node_tag) = direct_unsupported_form(node, graph.term(&node.node_id)) {
        return Err(CompositeEqualityRefusal::Unsupported {
            unsupported_node_id: node.node_id.clone(),
            node_tag,
        }
        .into());
    }
    let tag = CheckedNodeTag::from_wire(&node.node_tag);
    if tag != Some(CheckedNodeTag::Expression) {
        if let Some(CheckedNodeTag::Protocol) = tag {
            return Err(unsupported_family(&node.node_id, CheckedNodeTag::Protocol).into());
        }
        return Err(CompositeEqualityRefusal::NotExpression {
            node_tag: tag.map_or("unknown", |tag| tag.as_wire()),
        }
        .into());
    }
    if &*node.semantic_form == "call" {
        return Err(CompositeEqualityRefusal::BlockedOnUpstream {
            unsupported_node_id: node.node_id.clone(),
            node_tag: "expression.call",
            issue: UpstreamBlocker::QuireContractRuntime34,
        }
        .into());
    }
    if &*node.semantic_form != "binary" {
        return Err(CompositeEqualityRefusal::FormMismatch {
            found: node.semantic_form.to_string(),
        }
        .into());
    }
    let arguments = application_arguments(graph.term(&node.node_id))
        .filter(|arguments| arguments.len() == 2)
        .ok_or(CompositeEqualityRefusal::BodyMismatch)?;
    check_operand_types(graph, arguments, item)?;

    let mut closure = TypeClosure {
        work_limit: type_resolution_work_units,
        ..TypeClosure::default()
    };
    let left_source = resolve_type(
        admitted,
        graph,
        bounds_by_type,
        &mut closure,
        &item.left.source_type,
    )?;
    let left_target = item
        .left
        .conversion_target
        .as_ref()
        .map(|target| resolve_type(admitted, graph, bounds_by_type, &mut closure, target))
        .transpose()?;
    let right_source = resolve_type(
        admitted,
        graph,
        bounds_by_type,
        &mut closure,
        &item.right.source_type,
    )?;
    let right_target = item
        .right
        .conversion_target
        .as_ref()
        .map(|target| resolve_type(admitted, graph, bounds_by_type, &mut closure, target))
        .transpose()?;

    let mut declaration_keys = closure.node_ids.clone();
    declaration_keys.sort();
    let composites: Vec<CompositeDeclaration> = closure.composites.values().cloned().collect();
    let environment = admit_environment(admitted, composites.clone(), &closure.composite_bindings)?;

    let left_operand = match left_target.clone() {
        Some(target) => EqualityOperand::converted(left_source.clone(), target),
        None => EqualityOperand::typed(left_source.clone()),
    };
    let right_operand = match right_target.clone() {
        Some(target) => EqualityOperand::converted(right_source.clone(), target),
        None => EqualityOperand::typed(right_source.clone()),
    };
    let checked = match environment.check_equality(
        item.operator.to_runtime(),
        left_operand,
        right_operand,
        &closure.enum_members,
    ) {
        Ok(checked) => checked,
        Err(ill_typed) => {
            return Err(CompositeEqualityRefusal::IllTyped {
                cause: ill_typed.cause.try_into()?,
            }
            .into());
        }
    };

    Ok(CheckedItem {
        checked,
        left_source,
        left_target,
        right_source,
        right_target,
        composites,
        declaration_keys,
        enums: closure.enums,
        composite_bindings: closure.composite_bindings,
    })
}

fn admit_environment(
    admitted: Option<&qsl_replay::AdmittedContext>,
    composites: Vec<CompositeDeclaration>,
    bindings: &BTreeMap<NodeKey, admitted::CompositeBinding>,
) -> Result<TypeEnvironment, ItemCheckError> {
    let mut environment = units::attach_owner_units(
        TypeEnvironment::new(composites, std::iter::empty()).map_err(environment_failure)?,
        admitted,
    );
    for (key, binding) in bindings {
        if let Some(members) = binding.declaration.union_members() {
            environment
                .seal_union_verified(
                    *key,
                    binding.declaration.node(),
                    members
                        .iter()
                        .map(|member| member.member().clone())
                        .collect(),
                )
                .map_err(environment_failure)?;
        }
    }
    Ok(environment)
}

fn environment_failure(failure: sv::EnvironmentFailure) -> ItemCheckError {
    match failure {
        sv::EnvironmentFailure::Refused(invalid) => match invalid.cause.try_into() {
            Ok(cause) => CompositeEqualityRefusal::Declaration {
                declaration: invalid.declaration,
                cause,
            }
            .into(),
            Err(error) => ItemCheckError::Generation(error),
        },
        failure => OracleGenerationError::EnvironmentAdmission(failure).into(),
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
        CheckedNodeTag::State
        | CheckedNodeTag::Temporal
        | CheckedNodeTag::Protocol
        | CheckedNodeTag::TimedModel => blocked(UpstreamBlocker::QuireSpecLanguage121),
        CheckedNodeTag::ScalarType
        | CheckedNodeTag::CompositeType
        | CheckedNodeTag::BoundedDomain
        | CheckedNodeTag::Value
        | CheckedNodeTag::Expression
        | CheckedNodeTag::Claim
        | CheckedNodeTag::Correspondence
        | CheckedNodeTag::Extension
        | CheckedNodeTag::Hybrid => CompositeEqualityRefusal::Unsupported {
            unsupported_node_id: node_id.clone(),
            node_tag: tag.as_wire(),
        },
    }
}

fn application_arguments(body: Option<&CheckedTerm>) -> Option<&[CheckedTerm]> {
    application_of(body, ApplicationOperator::Binary)
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
fn operand_type_id<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    term: &CheckedTerm,
) -> Option<CheckedNodeId> {
    let mut term = term;
    // Each step follows one reference to a distinct-or-repeated node; the graph's size bounds a
    // chain that does not cycle, and a cycle of conversions reads as no type.
    for _ in 0..=graph.len() {
        let target = read_reference(term)?;
        let node = graph.node(&target)?;
        let conversion = (&*node.node_tag == CheckedNodeTag::Expression.as_wire())
            .then(|| application_of(graph.term(&node.node_id), ApplicationOperator::Convert))
            .flatten();
        match conversion {
            Some(arguments) => term = arguments.first()?,
            None => return Some(node.semantic_type.clone()),
        }
    }
    None
}

/// The `arguments` of a node body that is an `application` of `operator`.
fn application_of(
    body: Option<&CheckedTerm>,
    expected: ApplicationOperator,
) -> Option<&[CheckedTerm]> {
    match body? {
        CheckedTerm::Application {
            operator,
            arguments,
            ..
        } if *operator == expected => Some(arguments),
        _ => None,
    }
}

/// Compare each body operand's declared type against the descriptor's for
/// that position, left then right, refusing at the first disagreement.
fn check_operand_types<Body: GraphBody>(
    graph: &Graph<'_, Body>,
    arguments: &[CheckedTerm],
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

struct TypeClosure {
    composites: BTreeMap<NodeKey, CompositeDeclaration>,
    in_progress: BTreeSet<NodeKey>,
    work_consumed: u64,
    work_limit: u64,
    enum_members: quire_semantic_value::enumeration::EnumMemberIndex,
    enums: Vec<admitted::EnumBinding>,
    composite_bindings: BTreeMap<NodeKey, admitted::CompositeBinding>,
    /// Every V2 node id whose declaration entered the closure, ascending.
    node_ids: Vec<CheckedNodeId>,
}

impl Default for TypeClosure {
    fn default() -> Self {
        Self {
            composites: BTreeMap::new(),
            in_progress: BTreeSet::new(),
            work_consumed: 0,
            work_limit: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
            node_ids: Vec::new(),
            enum_members: Default::default(),
            enums: Vec::new(),
            composite_bindings: BTreeMap::new(),
        }
    }
}

fn lookup<'g, Body: GraphBody>(
    graph: &Graph<'g, Body>,
    node_id: &CheckedNodeId,
) -> Result<&'g CheckedSemanticNodeV2<Body>, CompositeEqualityRefusal> {
    graph
        .node(node_id)
        .ok_or_else(|| CompositeEqualityRefusal::UnknownTypeNode {
            type_node_id: node_id.clone(),
        })
}

fn read_reference(term: &CheckedTerm) -> Option<CheckedNodeId> {
    match term {
        CheckedTerm::Reference { target } => Some(target.clone()),
        _ => None,
    }
}

fn read_binding(term: &CheckedTerm) -> Option<(String, CheckedNodeId)> {
    let CheckedTerm::Binding { name, value } = term else {
        return None;
    };
    Some((name.to_string(), read_reference(value)?))
}

/// A record field has either a direct type reference or the checked optional wrapper.
fn read_record_field(term: &CheckedTerm) -> Option<(String, CheckedNodeId, bool)> {
    let CheckedTerm::Binding { name, value } = term else {
        return None;
    };
    if let Some(target) = read_reference(value) {
        return Some((name.to_string(), target, false));
    }
    let [optional] = aggregate_members(value.as_ref())? else {
        return None;
    };
    let (wrapper_name, target) = read_binding(optional)?;
    (wrapper_name == "optional").then_some((name.to_string(), target, true))
}

fn is_option_node<Body>(node: &CheckedSemanticNodeV2<Body>) -> bool {
    CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::CompositeType)
        && &*node.semantic_form == "option"
}

/// Resolve `type_id` to a `ValueType`, registering every reachable
/// nominal composite declaration into `closure`.
fn resolve_type<Body: GraphBody>(
    admitted: Option<&qsl_replay::AdmittedContext>,
    graph: &Graph<'_, Body>,
    bounds_by_type: &BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2<Body>>>,
    closure: &mut TypeClosure,
    type_id: &CheckedNodeId,
) -> Result<ValueType, ItemCheckError> {
    resolution::resolve_type(admitted, graph, bounds_by_type, closure, type_id)
}

/// Resolve a scalar type: `node` is the base `scalar_type` and `bounds` the `bounded_domain`
/// nodes that bound it for this reference (every one over the scalar, or the one the member
/// names); `type_id` is the member's type node, for refusals.
fn resolve_scalar<Body: GraphBody>(
    admitted: Option<&qsl_replay::AdmittedContext>,
    graph: &Graph<'_, Body>,
    closure: &mut TypeClosure,
    bounds: &[&CheckedSemanticNodeV2<Body>],
    type_id: &CheckedNodeId,
    node: &CheckedSemanticNodeV2<Body>,
) -> Result<ValueType, ItemCheckError> {
    let bound = |form: &'static str| -> Result<&[CheckedTerm], CompositeEqualityRefusal> {
        let candidates: Vec<_> = bounds
            .iter()
            .filter(|bound| &*bound.semantic_form == form)
            .collect();
        match candidates.as_slice() {
            [bound] => aggregate_members(graph.term(&bound.node_id)).ok_or_else(|| {
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
                    let members =
                        aggregate_members(graph.term(&bound.node_id)).ok_or_else(|| {
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
                }
                .into()),
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
        "float32" => Ok(ValueType::Float(exact::FloatType::exact(
            exact::IeeeWidth::Binary32,
        ))),
        "float64" => Ok(ValueType::Float(exact::FloatType::exact(
            exact::IeeeWidth::Binary64,
        ))),
        "text" => {
            let members = bound("text_bounds")?;
            let text_type = read_text_bounds(members).ok_or_else(|| {
                CompositeEqualityRefusal::UnreadableBound {
                    bound: type_id.clone(),
                }
            })?;
            Ok(ValueType::Text(text_type))
        }
        "enum" => admitted::resolve_enum(
            admitted::require_context(admitted, &node.node_id)?,
            closure,
            node,
        ),
        other => Err(CompositeEqualityRefusal::Unsupported {
            unsupported_node_id: type_id.clone(),
            node_tag: match other {
                "unit" | "dimension" => "quantity",
                _ => "scalar_type",
            },
        }
        .into()),
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

const SOURCE_HEADER: &str = "// @generated by quire-contract-codegen composite equality oracles. Do not edit.\n// The prepared capability retains genuine owner context and an admitted private enum index.\n// The oracle remains the implementation under test: it calls QSV checking/evaluation directly.\nuse quire_semantic_value::declaration as sv;\nuse quire_exact as exact;\n\n/// A generated numeric, text, cardinality or enum descriptor failed reconstruction.\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\npub enum ReconstructionError {\n    /// Canonical integer spelling failed to parse.\n    Integer,\n    /// Interval endpoints are invalid.\n    Interval,\n    /// Rational bounds are invalid.\n    Rational,\n    /// Decimal bounds are invalid.\n    Decimal,\n    /// Text bounds are invalid.\n    Text,\n    /// Cardinality endpoints are invalid.\n    Cardinality,\n    /// Genuine enum keys could not reconstruct the declared enum.\n    Enum,\n    /// Genuine composite metadata disagrees with its retained lookup recipe.\n    Composite,\n}\n\n/// A constructor failed without publishing a partial prepared capability.\n#[derive(Clone, Debug, Eq, PartialEq)]\npub enum EnvironmentError {\n    /// Reached nominal declaration requires its genuine owner capability.\n    MissingAdmittedContext {\n        /// Actual wire declaration identity awaiting lookup.\n        declaration: qsl_replay::WireNodeId,\n    },\n    /// The immutable context does not name the original checked package.\n    ContextPackageMismatch {\n        /// Expected original package domain.\n        expected_domain: &'static str,\n        /// Expected original package digest.\n        expected_digest: &'static str,\n        /// Actual context package domain.\n        actual_domain: String,\n        /// Actual context package digest.\n        actual_digest: String,\n    },\n    /// Genuine lookup retained the owner's Unknown/Foreign/WrongKind distinction.\n    Lookup(qsl_replay::AdmittedLookupRefusal),\n    /// Semantic refusal and resource failure remain owner-typed and distinct.\n    Declaration(sv::EnvironmentFailure),\n    /// A non-lookup descriptor reconstruction failed.\n    Reconstruction(ReconstructionError),\n}\n\nimpl From<ReconstructionError> for EnvironmentError {\n    fn from(cause: ReconstructionError) -> Self { Self::Reconstruction(cause) }\n}\n\n/// An opaque item-bound environment prepared from a genuine admitted context.\npub struct PreparedEqualityEnvironment<'ctx> {\n    context: Option<&'ctx qsl_replay::AdmittedContext>,\n    expected_domain: &'static str,\n    expected_digest: &'static str,\n    binding: &'static str,\n    requires_context: bool,\n    environment: sv::TypeEnvironment,\n    enum_members: quire_semantic_value::enumeration::EnumMemberIndex,\n    left_source: exact::ValueType,\n    left_target: Option<exact::ValueType>,\n    right_source: exact::ValueType,\n    right_target: Option<exact::ValueType>,\n}\n";

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
    ReconstructionHelper { name: "lookup_composite_key", source: "\nfn lookup_composite_key(\n    context: Option<&qsl_replay::AdmittedContext>,\n    digest: &str, qualified: &[&str], form: qsl_replay::AdmittedCompositeForm,\n) -> Result<exact::NodeKey, EnvironmentError> {\n    let wire = qsl_replay::WireNodeId::from_hex(digest).ok_or(ReconstructionError::Composite)?;\n    let context = context.ok_or(EnvironmentError::MissingAdmittedContext { declaration: wire })?;\n    let admitted = context.composite_declaration(wire).map_err(EnvironmentError::Lookup)?;\n    if admitted.form() != form || !admitted.qualified_name().iter()\n        .map(|part| part.as_str()).eq(qualified.iter().copied())\n    { return Err(ReconstructionError::Composite.into()); }\n    Ok(admitted.key())\n}\n", calls: &[] },
    ReconstructionHelper {
        name: "rebuild_union",
        source: "\nfn rebuild_union(\n    context: Option<&qsl_replay::AdmittedContext>, digest: &str, qualified: &[&str],\n    payloads: Vec<Vec<exact::ValueType>>,\n) -> Result<sv::CompositeShape, EnvironmentError> {\n    let key = lookup_composite_key(context, digest, qualified, qsl_replay::AdmittedCompositeForm::Union)?;\n    let wire = qsl_replay::WireNodeId::from_hex(digest).ok_or(ReconstructionError::Composite)?;\n    let context = context.ok_or(EnvironmentError::MissingAdmittedContext { declaration: wire })?;\n    let declaration = context.composite_declaration(wire).map_err(EnvironmentError::Lookup)?;\n    if declaration.key() != key { return Err(ReconstructionError::Composite.into()); }\n    let members = declaration.union_members().ok_or(ReconstructionError::Composite)?;\n    if members.len() != payloads.len() { return Err(ReconstructionError::Composite.into()); }\n    let mut rebuilt = Vec::with_capacity(members.len());\n    for (member, payload) in members.iter().zip(payloads) {\n        if member.payload() != payload { return Err(ReconstructionError::Composite.into()); }\n        rebuilt.push(sv::UnionMemberDeclaration::resolved(member.identifier().clone(), payload));\n    }\n    Ok(sv::CompositeShape::Union(rebuilt))\n}\n",
        calls: &["lookup_composite_key"],
    },
    ReconstructionHelper {
        name: "seal_union",
        source: "\nfn seal_union(\n    context: Option<&qsl_replay::AdmittedContext>, environment: &mut sv::TypeEnvironment,\n    key: exact::NodeKey, digest: &str,\n) -> Result<(), EnvironmentError> {\n    let wire = qsl_replay::WireNodeId::from_hex(digest).ok_or(ReconstructionError::Composite)?;\n    let context = context.ok_or(EnvironmentError::MissingAdmittedContext { declaration: wire })?;\n    let declaration = context.composite_declaration(wire).map_err(EnvironmentError::Lookup)?;\n    if declaration.key() != key { return Err(ReconstructionError::Composite.into()); }\n    let members = declaration.union_members().ok_or(ReconstructionError::Composite)?;\n    environment.seal_union_verified(key, declaration.node(), members.iter().map(|member| member.member().clone()).collect())\n        .map_err(EnvironmentError::Declaration)\n}\n",
        calls: &[],
    },
    ReconstructionHelper { name: "lookup_enum_case", source: "\nfn lookup_enum_case<'ctx>(\n    context: Option<&'ctx qsl_replay::AdmittedContext>,\n    declaration: qsl_replay::WireNodeId, case: &exact::Identifier,\n) -> Result<&'ctx qsl_replay::AdmittedEnumCase, EnvironmentError> {\n    let context = context.ok_or(EnvironmentError::MissingAdmittedContext { declaration })?;\n    context.enum_case(declaration, case).map_err(EnvironmentError::Lookup)\n}\n", calls: &[] },
    ReconstructionHelper { name: "rebuild_enum", source: "\nfn rebuild_enum(\n    context: Option<&qsl_replay::AdmittedContext>,\n    enum_members: &mut quire_semantic_value::enumeration::EnumMemberIndex,\n    digest: &str, qualified: &[&str], ordered: bool,\n) -> Result<exact::ValueType, EnvironmentError> {\n    let wire = qsl_replay::WireNodeId::from_hex(digest).ok_or(ReconstructionError::Enum)?;\n    let context = context.ok_or(EnvironmentError::MissingAdmittedContext { declaration: wire })?;\n    let admitted = context.enum_declaration(wire).map_err(EnvironmentError::Lookup)?;\n    if admitted.is_ordered() != ordered\n        || !admitted.qualified_name().iter().map(|part| part.as_str()).eq(qualified.iter().copied())\n    { return Err(ReconstructionError::Enum.into()); }\n    let declaration = quire_semantic_value::enumeration::EnumDeclaration::new(\n        admitted.key(), admitted.is_ordered(),\n        admitted.cases().iter().map(|case| case.case().as_str().to_owned()).collect(),\n    ).map_err(|_| ReconstructionError::Enum)?;\n    let mut variants = Vec::with_capacity(admitted.cases().len());\n    for case in admitted.cases() {\n        let selected = lookup_enum_case(Some(context), wire, case.case())?;\n        let value = declaration.member(selected.case().as_str(), selected.key())\n            .map_err(|_| ReconstructionError::Enum)?;\n        variants.push(value.variant());\n        enum_members.record(value);\n    }\n    Ok(exact::ValueType::Enum(exact::EnumShape::new(ordered, variants)))\n}\n", calls: &["lookup_enum_case"] },
    ReconstructionHelper {
        name: "rebuild_integer",
        source: "\n\
/// A canonical decimal integer literal, re-parsed from generation-time output.
fn rebuild_integer(spelling: &str) -> Result<exact::Integer, ReconstructionError> {
    spelling.parse().map_err(|_| ReconstructionError::Integer)
}
",
        calls: &[],
    },
    ReconstructionHelper {
        name: "rebuild_interval",
        source: "\n\
/// An integer interval over two canonical integer literals.
fn rebuild_interval(lower: &str, upper: &str) -> Result<exact::IntegerInterval, ReconstructionError> {
    exact::IntegerInterval::new(rebuild_integer(lower)?, rebuild_integer(upper)?)
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
    numerator: exact::IntegerInterval,
    denominator: exact::IntegerInterval,
) -> Result<exact::RationalDomain, ReconstructionError> {
    exact::RationalDomain::new(numerator, denominator).map_err(|_| ReconstructionError::Rational)
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
    rounding: exact::RoundingMode,
) -> Result<exact::DecimalType, ReconstructionError> {
    exact::DecimalType::new(
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
    profile: exact::TextProfile,
) -> Result<exact::TextType, ReconstructionError> {
    exact::TextType::new(min, max, profile).map_err(|_| ReconstructionError::Text)
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
) -> Result<exact::CardinalityBound, ReconstructionError> {
    exact::CardinalityBound::new(minimum, maximum).map_err(|_| ReconstructionError::Cardinality)
}
",
        calls: &[],
    },
];

/// Why one item could not be rendered. Private: the public failure type is
/// [`OracleGenerationError`], which the item boundary maps this onto.
#[derive(Clone, Debug, Eq, PartialEq)]
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
    union_seals: String,
    requires_context: bool,
    composites: Vec<(CheckedNodeId, String)>,
    left_source: String,
    left_target: Option<String>,
    right_source: String,
    right_target: Option<String>,
}

impl RenderedItem {
    fn source_bytes(&self) -> usize {
        self.composites
            .iter()
            .map(|(_, source)| source.len())
            .chain([
                self.union_seals.len(),
                self.left_source.len(),
                self.left_target.as_ref().map_or(0, String::len),
                self.right_source.len(),
                self.right_target.as_ref().map_or(0, String::len),
            ])
            .fold(0usize, usize::saturating_add)
    }
}

fn render_item(generated: &CheckedItem) -> Result<RenderedItem, RenderError> {
    // Every reachable nominal composite declaration, in the same `NodeKey`
    // order `TypeEnvironment::new` admitted at generation time.
    let declared = generated.declaration_keys.iter().collect::<BTreeSet<_>>();
    if generated.composite_bindings.len() != generated.declaration_keys.len()
        || generated.composites.len() != generated.composite_bindings.len()
        || generated.composite_bindings.values().any(|binding| {
            binding.node.domain.as_ref() != exact::NODE_KEY_DOMAIN
                || !declared.contains(&binding.node)
        })
    {
        return Err(OracleGenerationError::InvalidPlanState.into());
    }
    let composites = generated
        .composites
        .iter()
        .map(|declaration| {
            let binding = generated
                .composite_bindings
                .get(&declaration.key())
                .ok_or(OracleGenerationError::InvalidPlanState)?;
            Ok((
                binding.node.clone(),
                render_composite_declaration(
                    declaration,
                    &generated.enums,
                    &generated.composite_bindings,
                )?,
            ))
        })
        .collect::<Result<Vec<_>, RenderError>>()?;
    let target = |target: &Option<ValueType>| -> Result<Option<String>, RenderError> {
        target
            .as_ref()
            .map(|value| render_value_type(value, &generated.enums, &generated.composite_bindings))
            .transpose()
    };
    let mut union_seals = String::new();
    for (key, binding) in &generated.composite_bindings {
        if binding.form == qsl_replay::AdmittedCompositeForm::Union {
            let handle = render_composite_key(*key, &generated.composite_bindings)?;
            union_seals.push_str(&format!(
                "    seal_union(context, &mut environment, {handle}, {:?})?;\n",
                binding.node.digest,
            ));
        }
    }
    if !union_seals.is_empty() {
        union_seals.insert_str(
            0,
            "    let _context = context;\n    let mut environment = environment;\n",
        );
    }
    Ok(RenderedItem {
        union_seals,
        requires_context: !generated.enums.is_empty() || !generated.composites.is_empty(),
        composites,
        left_source: render_value_type(
            &generated.left_source,
            &generated.enums,
            &generated.composite_bindings,
        )?,
        left_target: target(&generated.left_target)?,
        right_source: render_value_type(
            &generated.right_source,
            &generated.enums,
            &generated.composite_bindings,
        )?,
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
    let mut rendered_bytes = 0usize;
    for (claim, key, item, checked) in pending {
        let result = render(&checked);
        if let Some(rendered_item) = settle_render(&mut claims[claim], result)? {
            rendered_bytes = rendered_bytes.saturating_add(rendered_item.source_bytes());
            if rendered_bytes > source.limit {
                return Err(source_too_large(source.limit));
            }
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
        source.item(&symbol, item, &rendered_item)?;
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

struct SourceBuilder {
    expected_package: Option<CheckedSemanticId>,
    functions: String,
    modules: Vec<(String, String)>,
    module_declarations: String,
    inline_composite_subjects: BTreeSet<CheckedNodeId>,
    charged_bytes: usize,
    limit: usize,
}

#[cfg(test)]
impl Default for SourceBuilder {
    fn default() -> Self {
        Self::new(MAX_GENERATED_SOURCE_BYTES).expect("default source limit admits header")
    }
}

// Debug Rust codegen can retain temporaries from sequential statements in a
// function frame. Keep each generated constructor small enough for a 512 KiB
// caller stack independently of source-byte artifact partitioning.
const MAX_DECLARATIONS_PER_FUNCTION: usize = 128;

impl SourceBuilder {
    fn new(limit: usize) -> Result<Self, OracleGenerationError> {
        if SOURCE_HEADER.len() > limit {
            return Err(source_too_large(limit));
        }
        Ok(Self {
            expected_package: None,
            functions: String::new(),
            modules: Vec::new(),
            module_declarations: String::new(),
            inline_composite_subjects: BTreeSet::new(),
            charged_bytes: SOURCE_HEADER.len(),
            limit,
        })
    }

    fn charge(&mut self, bytes: usize) -> Result<(), OracleGenerationError> {
        let next = self.charged_bytes.saturating_add(bytes);
        if next > self.limit {
            return Err(source_too_large(self.limit));
        }
        self.charged_bytes = next;
        Ok(())
    }

    fn append_function(&mut self, text: String) -> Result<(), OracleGenerationError> {
        self.charge(text.len())?;
        self.functions.push_str(&text);
        Ok(())
    }

    fn item(
        &mut self,
        symbol: &str,
        item: &CompositeEqualityItem,
        rendered: &RenderedItem,
    ) -> Result<(), OracleGenerationError> {
        let RenderedItem {
            union_seals,
            requires_context,
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

        let composite_bytes = composites.iter().fold(0usize, |size, (_, declaration)| {
            size.saturating_add(declaration.len() + 2)
        });
        if composites.len() <= MAX_DECLARATIONS_PER_FUNCTION
            && composite_bytes <= MAX_ARTIFACT_BYTES / 2
        {
            if composites.is_empty() {
                self.append_function(format!(
                    "\nfn composites_{symbol}(_context: Option<&qsl_replay::AdmittedContext>, _enum_members: &mut quire_semantic_value::enumeration::EnumMemberIndex) -> Result<Vec<sv::CompositeDeclaration>, EnvironmentError> {{\n    Ok(Vec::new())\n}}\n"
                ))?;
            } else {
                self.inline_composite_subjects
                    .extend(composites.iter().map(|(id, _)| id.clone()));
                let declarations = composites
                    .iter()
                    .map(|(_, value)| format!("    declarations.push({value});\n"))
                    .collect::<String>();
                self.append_function(format!(
                    "\nfn composites_{symbol}(_context: Option<&qsl_replay::AdmittedContext>, _enum_members: &mut quire_semantic_value::enumeration::EnumMemberIndex) -> Result<Vec<sv::CompositeDeclaration>, EnvironmentError> {{\n    let mut declarations = Vec::with_capacity({});\n{declarations}    Ok(declarations)\n}}\n",
                    composites.len()
                ))?;
            }
        } else {
            let mut chunk_calls = String::new();
            let mut chunk = String::new();
            let mut chunk_index = 0usize;
            let mut chunk_count = 0usize;
            let mut chunk_subjects = Vec::new();
            for (key, declaration) in composites {
                if !chunk.is_empty()
                    && (chunk_count == MAX_DECLARATIONS_PER_FUNCTION
                        || chunk.len().saturating_add(declaration.len() + 26)
                            > MAX_ARTIFACT_BYTES / 2)
                {
                    self.add_composite_module(
                        symbol,
                        &crate::core::provenance::node_subject(&item.node_id),
                        chunk_index,
                        &chunk,
                        &mut chunk_calls,
                        &chunk_subjects,
                    )?;
                    chunk_index += 1;
                    chunk.clear();
                    chunk_count = 0;
                    chunk_subjects.clear();
                }
                chunk.push_str("    declarations.push(");
                chunk.push_str(declaration);
                chunk.push_str(");\n");
                chunk_count += 1;
                chunk_subjects.push(crate::core::provenance::node_subject(key));
            }
            if !chunk.is_empty() {
                self.add_composite_module(
                    symbol,
                    &crate::core::provenance::node_subject(&item.node_id),
                    chunk_index,
                    &chunk,
                    &mut chunk_calls,
                    &chunk_subjects,
                )?;
            }
            self.append_function(format!(
                "\nfn composites_{symbol}(_context: Option<&qsl_replay::AdmittedContext>, _enum_members: &mut quire_semantic_value::enumeration::EnumMemberIndex) -> Result<Vec<sv::CompositeDeclaration>, EnvironmentError> {{\n    let mut declarations = Vec::new();\n{chunk_calls}    Ok(declarations)\n}}\n"
            ))?;
        }

        self.append_function(format!(
            "\nfn left_source_{symbol}(_context: Option<&qsl_replay::AdmittedContext>, _enum_members: &mut quire_semantic_value::enumeration::EnumMemberIndex) -> Result<exact::ValueType, EnvironmentError> {{\n    Ok({left_source})\n}}\n\
             fn left_target_{symbol}(_context: Option<&qsl_replay::AdmittedContext>, _enum_members: &mut quire_semantic_value::enumeration::EnumMemberIndex) -> Result<Option<exact::ValueType>, EnvironmentError> {{\n    Ok({left_target})\n}}\n\
             fn right_source_{symbol}(_context: Option<&qsl_replay::AdmittedContext>, _enum_members: &mut quire_semantic_value::enumeration::EnumMemberIndex) -> Result<exact::ValueType, EnvironmentError> {{\n    Ok({right_source})\n}}\n\
             fn right_target_{symbol}(_context: Option<&qsl_replay::AdmittedContext>, _enum_members: &mut quire_semantic_value::enumeration::EnumMemberIndex) -> Result<Option<exact::ValueType>, EnvironmentError> {{\n    Ok({right_target})\n}}\n"
        ))?;

        let package = self
            .expected_package
            .clone()
            .ok_or(OracleGenerationError::InvalidPlanState)?;
        let package_domain = package.domain.as_ref();
        let package_digest = package.digest.as_ref();
        self.append_function(format!(
"\n/// Prepare this item's descriptors and environment from the original package context.\npub fn environment_{symbol}<'ctx>(context: Option<&'ctx qsl_replay::AdmittedContext>) -> Result<PreparedEqualityEnvironment<'ctx>, EnvironmentError> {{\n    if let Some(context) = context {{\n    let actual = context.package_id().record();\n    if actual.domain().as_str() != {package_domain:?} || actual.hex() != {package_digest:?} {{\n        return Err(EnvironmentError::ContextPackageMismatch {{\n            expected_domain: {package_domain:?}, expected_digest: {package_digest:?},\n            actual_domain: actual.domain().as_str().to_owned(), actual_digest: actual.hex(),\n        }});\n    }}\n    }}\n    let mut enum_members = quire_semantic_value::enumeration::EnumMemberIndex::default();\n    let composites = composites_{symbol}(context, &mut enum_members)?;\n    let environment = sv::TypeEnvironment::new(composites, core::iter::empty::<sv::ObjectTypeDeclaration>())\n        .map_err(EnvironmentError::Declaration)?;\n    let environment = match context {{\n        Some(context) => environment.with_units(context.unit_table().clone()),\n        None => environment,\n    }};\n{union_seals}    let left_source = left_source_{symbol}(context, &mut enum_members)?;\n    let left_target = left_target_{symbol}(context, &mut enum_members)?;\n    let right_source = right_source_{symbol}(context, &mut enum_members)?;\n    let right_target = right_target_{symbol}(context, &mut enum_members)?;\n    Ok(PreparedEqualityEnvironment {{\n        context, expected_domain: {package_domain:?}, expected_digest: {package_digest:?},\n        binding: {symbol:?}, requires_context: {requires_context}, environment, enum_members,\n        left_source, left_target, right_source, right_target,\n    }})\n}}\n", package_domain = package_domain, package_digest = package_digest))?;
        self.append_function(format!(
"\n/// Environment-checked oracle for `{operation}`.\npub fn oracle_{symbol}(\n    prepared: &PreparedEqualityEnvironment<'_>, left: &exact::Value,\n    right: &exact::Value, meter: &mut exact::Meter,\n) -> exact::Outcome<bool> {{\n    let context_mismatch = match prepared.context {{\n        Some(context) => {{ let actual = context.package_id().record();\n            actual.domain().as_str() != {package_domain:?} || actual.hex() != {package_digest:?} }},\n        None => prepared.requires_context,\n    }};\n    if prepared.binding != {symbol:?} || prepared.expected_domain != {package_domain:?}\n        || prepared.expected_digest != {package_digest:?}\n        || prepared.requires_context != {requires_context} || context_mismatch\n    {{\n        return exact::Outcome::Refused(exact::Refusal::CheckedInvariant {{ cause: exact::CheckedInvariantCause::GeneratedDescriptorReconstructionFailed }});\n    }}\n    let environment = &prepared.environment;\n    let left_comparison = match &prepared.left_target {{ Some(target) => target, None => &prepared.left_source }};\n    let right_comparison = match &prepared.right_target {{ Some(target) => target, None => &prepared.right_source }};\n    if let Err(error) = environment.check_type(left_comparison) {{\n        return exact::Outcome::Refused(exact::Refusal::CheckedInvariant {{ cause: exact::CheckedInvariantCause::GeneratedTypeCheckRejected {{ cause: error.cause }} }});\n    }}\n    if let Err(error) = environment.check_type(right_comparison) {{\n        return exact::Outcome::Refused(exact::Refusal::CheckedInvariant {{ cause: exact::CheckedInvariantCause::GeneratedTypeCheckRejected {{ cause: error.cause }} }});\n    }}\n    let left_operand = match prepared.left_target.clone() {{\n        Some(target) => sv::EqualityOperand::converted(prepared.left_source.clone(), target),\n        None => sv::EqualityOperand::typed(prepared.left_source.clone()),\n    }};\n    let right_operand = match prepared.right_target.clone() {{\n        Some(target) => sv::EqualityOperand::converted(prepared.right_source.clone(), target),\n        None => sv::EqualityOperand::typed(prepared.right_source.clone()),\n    }};\n    let checked = match environment.check_equality({operator}, left_operand, right_operand, &prepared.enum_members) {{\n        Ok(checked) => checked,\n        Err(error) => return exact::Outcome::Refused(exact::Refusal::CheckedInvariant {{ cause: exact::CheckedInvariantCause::GeneratedEqualityCheckRejected {{ cause: error.cause }} }}),\n    }};\n    checked.evaluate(environment, left, right, meter)\n}}\n", operation = item.operator.identity(), operator = item.operator.path(), package_domain = package_domain, package_digest = package_digest))?;
        Ok(())
    }

    fn add_composite_module(
        &mut self,
        symbol: &str,
        subject: &str,
        index: usize,
        declarations: &str,
        calls: &mut String,
        composite_subjects: &[String],
    ) -> Result<(), OracleGenerationError> {
        let name = format!("composites_{symbol}_{index}");
        let module_declaration = format!("mod {name};\n");
        let module_source = crate::core::provenance::with_rust_header(
            format!(
                "use super::*;\npub(super) fn declarations(_context: Option<&qsl_replay::AdmittedContext>, _enum_members: &mut quire_semantic_value::enumeration::EnumMemberIndex) -> Result<Vec<sv::CompositeDeclaration>, EnvironmentError> {{\n    let mut declarations = Vec::new();\n{declarations}    Ok(declarations)\n}}\n"
            ),
            std::iter::once(subject).chain(composite_subjects.iter().map(String::as_str)),
        );
        self.charge(module_declaration.len().saturating_add(module_source.len()))?;
        self.module_declarations.push_str(&module_declaration);
        self.modules.push((format!("src/{name}.rs"), module_source));
        calls.push_str(&format!(
            "    declarations.extend({name}::declarations(_context, _enum_members)?);\n"
        ));
        Ok(())
    }

    fn finish(&self, limit: usize) -> Result<Vec<Artifact>, OracleGenerationError> {
        let mut source = String::new();
        append_source(&mut source, SOURCE_HEADER, limit)?;
        append_source(&mut source, &self.module_declarations, limit)?;
        // A helper is needed when the item functions call it or a needed helper does. The table
        // lists callees first, so one pass from the last helper to the first settles the
        // transitive closure.
        let mut needed: BTreeSet<&str> = BTreeSet::new();
        for helper in RECONSTRUCTION_HELPERS.iter().rev() {
            if self.functions.contains(&format!("{}(", helper.name))
                || self
                    .modules
                    .iter()
                    .any(|(_, text)| text.contains(&format!("{}(", helper.name)))
                || needed.contains(helper.name)
            {
                needed.insert(helper.name);
                needed.extend(helper.calls);
            }
        }
        for helper in RECONSTRUCTION_HELPERS {
            if needed.contains(helper.name) {
                append_source(&mut source, helper.source, limit)?;
            }
        }
        append_source(&mut source, &self.functions, limit)?;
        let mut total = source.len();
        let mut artifacts = vec![Artifact::new("src/lib.rs", source)];
        for (path, contents) in &self.modules {
            total = total.saturating_add(contents.len());
            if total > limit {
                return Err(source_too_large(limit));
            }
            artifacts.push(Artifact::new(path, contents.clone()));
        }
        Ok(artifacts)
    }
}

fn append_source(
    source: &mut String,
    text: &str,
    limit: usize,
) -> Result<(), OracleGenerationError> {
    let needed = source.len().saturating_add(text.len());
    if needed > limit {
        return Err(source_too_large(limit));
    }
    source.push_str(text);
    Ok(())
}

fn source_too_large(limit: usize) -> OracleGenerationError {
    OracleGenerationError::SourceTooLarge {
        bytes: limit + 1,
        limit,
        setting: "cg.equality.source_bytes",
    }
}

/// Render one admitted `CompositeDeclaration` as a Rust expression of type
/// `sv::CompositeDeclaration`.
fn render_composite_declaration(
    declaration: &CompositeDeclaration,
    enums: &[admitted::EnumBinding],
    composites: &BTreeMap<NodeKey, admitted::CompositeBinding>,
) -> Result<String, RenderError> {
    let shape = match declaration.shape() {
        CompositeShape::Record(fields) => {
            let rendered: String = fields
                .iter()
                .map(|field| {
                    Ok(format!(
                        "sv::FieldDeclaration::new({:?}, {}, {}), ",
                        field.name(),
                        render_value_type(field.value_type(), enums, composites)?,
                        presence_path(field.presence())
                    ))
                })
                .collect::<Result<_, RenderError>>()?;
            format!("sv::CompositeShape::Record(vec![{rendered}])")
        }
        CompositeShape::Tuple(positions) => {
            let rendered: String = positions
                .iter()
                .map(|value_type| {
                    Ok(format!(
                        "{}, ",
                        render_value_type(value_type, enums, composites)?
                    ))
                })
                .collect::<Result<_, RenderError>>()?;
            format!("sv::CompositeShape::Tuple(vec![{rendered}])")
        }
        CompositeShape::Union(members) => {
            let binding = composites
                .get(&declaration.key())
                .ok_or(OracleGenerationError::InvalidPlanState)?;
            let qualified = binding
                .qualified
                .iter()
                .map(|part| format!("{part:?}"))
                .collect::<Vec<_>>()
                .join(", ");
            let payloads = members
                .iter()
                .map(|member| {
                    let positions = member
                        .positions()
                        .iter()
                        .map(|position| render_value_type(position, enums, composites))
                        .collect::<Result<Vec<_>, _>>()?
                        .join(", ");
                    Ok(format!("vec![{positions}]"))
                })
                .collect::<Result<Vec<_>, RenderError>>()?
                .join(", ");
            format!(
                "rebuild_union(_context, {:?}, &[{qualified}], vec![{payloads}])?",
                binding.node.digest
            )
        }
    };
    let key = render_composite_key(declaration.key(), composites)?;
    Ok(format!(
        "sv::CompositeDeclaration::new({key}, {:?}, {shape})",
        declaration.name()
    ))
}

fn presence_path(presence: Presence) -> &'static str {
    match presence {
        Presence::Required => "exact::Presence::Required",
        Presence::Optional => "exact::Presence::Optional",
    }
}

/// Render one `ValueType` as a Rust expression of type `exact::ValueType`.
///
/// Every bound is rebuilt through an emitted `rebuild_*` helper whose `Result` the expression
/// propagates with `?`, so the expression belongs in a function returning
/// `Result<_, ReconstructionError>`.
fn render_value_type(
    value_type: &ValueType,
    enums: &[admitted::EnumBinding],
    composites: &BTreeMap<NodeKey, admitted::CompositeBinding>,
) -> Result<String, RenderError> {
    enum Task<'a> {
        Value(&'a ValueType),
        FinishOption,
        FinishCollection {
            kind: CollectionKind,
            bound: Option<CardinalityBound>,
        },
    }

    let mut rendered = String::from("{\n");
    let mut tasks = vec![Task::Value(value_type)];
    let mut values = Vec::new();
    let mut next_slot = 0usize;
    while let Some(task) = tasks.pop() {
        let expression = match task {
            Task::FinishOption => {
                let child = values
                    .pop()
                    .ok_or_else(|| OracleGenerationError::unknown_variant("ValueType"))?;
                format!("exact::ValueType::option({child})")
            }
            Task::FinishCollection { kind, bound } => {
                let child = values
                    .pop()
                    .ok_or_else(|| OracleGenerationError::unknown_variant("ValueType"))?;
                let bound = match bound {
                    Some(bound) => format!(
                        "Some(rebuild_cardinality({}, {})?)",
                        bound.minimum(),
                        bound.maximum()
                    ),
                    None => "None".to_owned(),
                };
                format!(
                    "exact::ValueType::collection(exact::CollectionType::new({}, {child}, {bound}))",
                    collection_kind_path(kind)?
                )
            }
            Task::Value(value_type) => match value_type {
                ValueType::Boolean => "exact::ValueType::Boolean".to_owned(),
                ValueType::Integer => "exact::ValueType::Integer".to_owned(),
                ValueType::Int(interval) => {
                    format!("exact::ValueType::Int({})", render_interval(interval))
                }
                ValueType::Rational(domain) => format!(
                    "exact::ValueType::Rational(rebuild_rational({}, {})?)",
                    render_interval(domain.numerator()),
                    render_interval(domain.denominator())
                ),
                ValueType::Decimal(decimal) => {
                    format!(
                        "exact::ValueType::Decimal({})",
                        render_decimal_type(decimal)?
                    )
                }
                ValueType::Float(float) => {
                    let width = match float.width() {
                        exact::IeeeWidth::Binary32 => "exact::IeeeWidth::Binary32",
                        exact::IeeeWidth::Binary64 => "exact::IeeeWidth::Binary64",
                    };
                    format!(
                        "exact::ValueType::Float(exact::FloatType::new({width}, {}))",
                        rounding_path(float.rounding())?
                    )
                }
                ValueType::Quantity(_) => {
                    return Err(RenderError::UnsupportedValueType { family: "quantity" });
                }
                ValueType::Text(text_type) => format!(
                    "exact::ValueType::Text(rebuild_text({}, {}, {})?)",
                    text_type.min(),
                    text_type.max(),
                    profile_path(text_type.profile())?
                ),
                ValueType::Enum(shape) => {
                    let binding = enums
                        .iter()
                        .find(|binding| &binding.shape == shape)
                        .ok_or(OracleGenerationError::InvalidPlanState)?;
                    let qualified = binding
                        .qualified
                        .iter()
                        .map(|part| format!("{part:?}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!(
                        "rebuild_enum(_context, _enum_members, {:?}, &[{qualified}], {})?",
                        binding.node.digest, binding.ordered
                    )
                }
                ValueType::Option(payload) => {
                    tasks.push(Task::FinishOption);
                    tasks.push(Task::Value(payload));
                    continue;
                }
                ValueType::Composite(key) => {
                    format!(
                        "exact::ValueType::Composite({})",
                        render_composite_key(*key, composites)?
                    )
                }
                ValueType::Collection(collection) => {
                    tasks.push(Task::FinishCollection {
                        kind: collection.kind(),
                        bound: collection.bound(),
                    });
                    tasks.push(Task::Value(collection.element()));
                    continue;
                }
                ValueType::Reference(_) => {
                    return Err(RenderError::UnsupportedValueType {
                        family: "reference",
                    });
                }
                &_ => return Err(OracleGenerationError::unknown_variant("ValueType").into()),
            },
        };
        let slot = format!("t{next_slot}");
        rendered.push_str(&format!("let {slot} = {expression};\n"));
        values.push(slot);
        next_slot += 1;
    }
    let root = values
        .pop()
        .ok_or_else(|| OracleGenerationError::unknown_variant("ValueType"))?;
    rendered.push_str(&root);
    rendered.push_str("\n}");
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

fn render_composite_key(
    key: NodeKey,
    composites: &BTreeMap<NodeKey, admitted::CompositeBinding>,
) -> Result<String, RenderError> {
    let binding = composites
        .get(&key)
        .ok_or(OracleGenerationError::InvalidPlanState)?;
    let qualified = binding
        .qualified
        .iter()
        .map(|part| format!("{part:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    let form = match binding.form {
        qsl_replay::AdmittedCompositeForm::Record => "qsl_replay::AdmittedCompositeForm::Record",
        qsl_replay::AdmittedCompositeForm::Tuple => "qsl_replay::AdmittedCompositeForm::Tuple",
        qsl_replay::AdmittedCompositeForm::Union => "qsl_replay::AdmittedCompositeForm::Union",
    };
    Ok(format!(
        "lookup_composite_key(_context, {:?}, &[{qualified}], {form})?",
        binding.node.digest
    ))
}

fn collection_kind_path(kind: CollectionKind) -> Result<&'static str, OracleGenerationError> {
    match kind {
        CollectionKind::Sequence => Ok("exact::CollectionKind::Sequence"),
        CollectionKind::Set => Ok("exact::CollectionKind::Set"),
        CollectionKind::Bag => Ok("exact::CollectionKind::Bag"),
        CollectionKind::OrderedSet => Ok("exact::CollectionKind::OrderedSet"),
    }
}

fn rounding_path(rounding: RoundingMode) -> Result<&'static str, OracleGenerationError> {
    match rounding {
        RoundingMode::Exact => Ok("exact::RoundingMode::Exact"),
        RoundingMode::TowardZero => Ok("exact::RoundingMode::TowardZero"),
        RoundingMode::TowardPositive => Ok("exact::RoundingMode::TowardPositive"),
        RoundingMode::TowardNegative => Ok("exact::RoundingMode::TowardNegative"),
        RoundingMode::NearestEven => Ok("exact::RoundingMode::NearestEven"),
        RoundingMode::NearestAway => Ok("exact::RoundingMode::NearestAway"),
    }
}

fn profile_path(profile: TextProfile) -> Result<&'static str, OracleGenerationError> {
    match profile {
        TextProfile::UnicodeScalars => Ok("exact::TextProfile::UnicodeScalars"),
        TextProfile::Nfc => Ok("exact::TextProfile::Nfc"),
        TextProfile::Nfd => Ok("exact::TextProfile::Nfd"),
        TextProfile::Nfkc => Ok("exact::TextProfile::Nfkc"),
        TextProfile::Nfkd => Ok("exact::TextProfile::Nfkd"),
        TextProfile::BinaryUtf8 => Ok("exact::TextProfile::BinaryUtf8"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Graph<'a> = crate::oracle::checked_graph::CheckedGraph<'a, CheckedTerm>;
    use crate::oracle::checked_graph::fixture::Node as FixtureNode;
    use ix_trace_rs::trace;
    use quire_contract_model::LiteralKind;
    use serde_json::json;

    #[trace("FR-018-AC-1")]
    #[test]
    fn hybrid_family_is_unsupported_and_preserves_node_identity() {
        let node = node_id('a');
        assert_eq!(
            unsupported_family(&node, CheckedNodeTag::Hybrid),
            CompositeEqualityRefusal::Unsupported {
                unsupported_node_id: node,
                node_tag: "hybrid",
            }
        );
    }

    fn test_source(source: SourceBuilder) -> String {
        source
            .finish(MAX_GENERATED_SOURCE_BYTES)
            .expect("test source fits")
            .into_iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .expect("library source exists")
            .contents
    }

    /// Genuine baseline nodes; private mutation tests alter bodies only after this admission.
    fn recursive_shape_nodes() -> Vec<FixtureNode> {
        let (package, _) = recursive_fixture();
        let nodes = &package.graph().nodes;
        let named = |name: &str| {
            nodes
                .iter()
                .find(|node| {
                    node.declaration.as_ref().is_some_and(|declaration| {
                        declaration
                            .qualified_name
                            .iter()
                            .map(|part| part.as_ref())
                            .eq([name])
                    })
                })
                .expect("actual recursive declaration")
        };
        let list = named("List");
        let tree = named("Tree");
        let references = |node: &CheckedSemanticNodeV2, target: &CheckedNodeId| {
            aggregate_members(package.term(&node.node_id)).is_some_and(|members| {
                members
                    .iter()
                    .any(|term| read_reference(term).as_ref() == Some(target))
            })
        };
        let option = nodes
            .iter()
            .find(|node| node.semantic_form.as_ref() == "option" && references(node, &list.node_id))
            .expect("actual optional List type");
        let sequence = nodes
            .iter()
            .find(|node| {
                node.semantic_form.as_ref() == "sequence" && references(node, &tree.node_id)
            })
            .expect("actual Tree sequence type");
        let bound = nodes
            .iter()
            .find(|node| {
                node.semantic_form.as_ref() == "collection_bounds"
                    && node.semantic_type == sequence.node_id
            })
            .expect("actual Tree cardinality");
        let integer = nodes
            .iter()
            .find(|node| node.semantic_form.as_ref() == "integer")
            .expect("actual source Integer type");
        [list, option, tree, sequence, bound, integer]
            .into_iter()
            .map(|node| FixtureNode::from_admitted(node, package.term(&node.node_id)))
            .collect()
    }

    fn fixture_nodes(package: &CheckedPackageV2) -> Vec<FixtureNode> {
        package
            .graph()
            .nodes
            .iter()
            .map(|node| FixtureNode::from_admitted(node, package.term(&node.node_id)))
            .collect()
    }

    fn fixture_members_mut(node: &mut FixtureNode) -> &mut Vec<CheckedTerm> {
        let Some(CheckedTerm::Aggregate { members }) = node.body.as_mut() else {
            panic!("fixture must be an aggregate")
        };
        members
    }

    fn fixture_binding_value_mut(term: &mut CheckedTerm) -> &mut CheckedTerm {
        let CheckedTerm::Binding { value, .. } = term else {
            panic!("fixture binding")
        };
        value
    }

    fn fixture_binding_name_mut(term: &mut CheckedTerm) -> &mut Box<str> {
        let CheckedTerm::Binding { name, .. } = term else {
            panic!("fixture binding")
        };
        name
    }

    fn fixture_reference_mut(term: &mut CheckedTerm) -> &mut CheckedNodeId {
        let CheckedTerm::Reference { target } = term else {
            panic!("fixture reference")
        };
        target
    }

    fn recursive_node_id(code: char) -> CheckedNodeId {
        let position = match code {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            // Deliberately absent transport identities in private cycle/unknown-node controls.
            other => return node_id(other),
        };
        recursive_shape_nodes()[position].node_id.clone()
    }

    fn recursive_fixture() -> &'static (CheckedPackageV2, qsl_replay::AdmittedContext) {
        static FIXTURE: std::sync::OnceLock<(CheckedPackageV2, qsl_replay::AdmittedContext)> =
            std::sync::OnceLock::new();
        FIXTURE.get_or_init(qsl_recursive_admission)
    }

    /// Compile the recursive QSpec forms through QSL's public facade and read its package.
    fn qsl_recursive_admission() -> (CheckedPackageV2, qsl_replay::AdmittedContext) {
        use qsl_replay::{
            compile_admitted, DependencyInput, ReplayLimits, SourceIdentity, StageLimits,
        };
        use quire_contract_model::{
            CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2ReadResult,
        };

        let source = concat!(
            include_str!("../../../tests/composite_equality_support/recursive.native"),
            "\nfunction integer_same using v(a: Integer,b: Integer): Boolean pure { a = b }\n"
        );
        let compiled = compile_admitted(
            SourceIdentity::new("a", "u", "git", "1"),
            "recursive.native",
            source.as_bytes(),
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("QSL compiles recursive records");
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        let read = CheckedPackageV2::read(
            compiled.package().bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence,
        );
        let CheckedPackageV2ReadResult::Admitted(package) = read else {
            panic!("QSL recursive package must pass Contract IR's strict reader: {read:?}")
        };
        (*package, compiled.into_parts().1)
    }

    fn recursive_owner_key(name: &str) -> NodeKey {
        qsl_recursive_admission()
            .1
            .composites()
            .find(|actual| {
                actual
                    .qualified_name()
                    .iter()
                    .map(|part| part.as_str())
                    .eq([name])
            })
            .expect("the authentic recursive source declares this record")
            .key()
    }

    /// Start from genuine owner-admitted numeric members. Changes below exercise only
    /// the private parser/resolver, not package/source/context authentication.
    fn numeric_tuple_component(
        base_members: bool,
        mutation: Option<&str>,
    ) -> (String, CheckedNodeId) {
        use quire_contract_model::{
            CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2ReadResult,
        };
        let source = concat!(
            "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n",
            "enum Marker { UNUSED, ALSO_UNUSED }\nrecord Point { x: Boolean; }\n",
            "tuple Pair(Int[-100,100],Text[0,16;nfc],Decimal[-100,100;0,0;nearest-even],Rational[-10,10;1,1]);\n",
            "function f using v(a: Pair,b: Pair): Boolean pure { a = b }\n",
        );
        let compiled = qsl_replay::compile_admitted(
            qsl_replay::SourceIdentity::new("test", "numeric-member.native", "git", "fixture"),
            "numeric-member.native",
            source.as_bytes(),
            [],
            &qsl_replay::DependencyInput::default(),
            qsl_replay::StageLimits::default(),
            qsl_replay::ReplayLimits::default(),
        )
        .expect("the unchanged original numeric member source compiles");
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
            compiled.package().bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence,
        ) else {
            panic!("the unmodified public tuple package must admit");
        };
        let mut nodes = fixture_nodes(&package);
        let bound = |form: &str| {
            nodes
                .iter()
                .find(|node| node.semantic_form.as_ref() == form)
                .expect("original numeric/text bound")
                .clone()
        };
        let integer = bound("integer_range");
        let text = bound("text_bounds");
        let decimal = bound("decimal_range");
        let rational = bound("rational_range");
        let members = [
            if base_members {
                integer.semantic_type.clone()
            } else {
                integer.node_id.clone()
            },
            text.node_id.clone(),
            if base_members {
                decimal.semantic_type.clone()
            } else {
                decimal.node_id.clone()
            },
            if base_members {
                rational.semantic_type.clone()
            } else {
                rational.node_id.clone()
            },
        ];
        let tuple = nodes
            .iter_mut()
            .find(|node| node.semantic_form.as_ref() == "tuple")
            .expect("actual Pair");
        let root = tuple.node_id.clone();
        tuple.body = Some(CheckedTerm::Aggregate {
            members: members
                .iter()
                .map(|id| CheckedTerm::Reference { target: id.clone() })
                .collect(),
        });
        if let Some(mutation) = mutation {
            let base = match mutation {
                "wrong_form" => None,
                "boolean" => Some(
                    nodes
                        .iter()
                        .find(|node| node.semantic_form.as_ref() == "boolean")
                        .unwrap()
                        .node_id
                        .clone(),
                ),
                "enum" | "record" => Some(
                    nodes
                        .iter()
                        .find(|node| {
                            node.declaration.as_ref().is_some_and(|declaration| {
                                declaration
                                    .qualified_name
                                    .iter()
                                    .map(|part| part.as_ref())
                                    .eq([if mutation == "enum" {
                                        "Marker"
                                    } else {
                                        "Point"
                                    }])
                            })
                        })
                        .unwrap()
                        .node_id
                        .clone(),
                ),
                _ => panic!("unknown private numeric member mutation"),
            };
            let bound = nodes
                .iter_mut()
                .find(|node| node.node_id == integer.node_id)
                .unwrap();
            if let Some(base) = base {
                bound.semantic_type = base;
            } else {
                bound.semantic_form = "text_bounds".into();
            }
        }
        let typed = EqualityOperandDescriptor::typed(root);
        let healthy = EqualityOperandDescriptor::typed(
            nodes
                .iter()
                .find(|node| node.semantic_form.as_ref() == "boolean")
                .unwrap()
                .node_id
                .clone(),
        );
        let operands = vec![(typed.clone(), typed), (healthy.clone(), healthy)];
        let (outcomes, source) = check_component_items_with_context(
            Some(compiled.context()),
            nodes,
            &operands,
            CompositeEqualityLimits {
                source_bytes: usize::MAX,
                ..Default::default()
            },
        );
        match mutation {
            Some("wrong_form") => assert_eq!(
                outcomes[0],
                Err(CompositeEqualityRefusal::MissingBound {
                    bounded_type: integer.node_id.clone(),
                    expected_form: "integer_range"
                })
            ),
            Some(_) => assert_eq!(
                outcomes[0],
                Err(CompositeEqualityRefusal::Unsupported {
                    unsupported_node_id: integer.node_id.clone(),
                    node_tag: "bounded_domain"
                })
            ),
            None => assert_eq!(outcomes, vec![Ok(()), Ok(())]),
        }
        assert_eq!(outcomes[1], Ok(()));
        let artifacts = source.finish(usize::MAX).expect("private component source");
        let lib = artifacts
            .into_iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .unwrap()
            .contents;
        (lib, integer.node_id.clone())
    }

    #[trace("FR-018-AC-16", "TC-029")]
    #[test]
    fn legacy_numeric_member_alias_equivalence_is_a_private_resolution_control() {
        let (through_scalars, _) = numeric_tuple_component(true, None);
        let (through_domains, _) = numeric_tuple_component(false, None);
        assert_eq!(
            through_scalars, through_domains,
            "both private numeric member forms reconstruct identical source"
        );
        assert!(
            through_scalars.contains("exact::ValueType::Decimal(")
                && through_scalars.contains("exact::ValueType::Rational(")
        );
        for reconstruction in [
            "exact::ValueType::Int(rebuild_interval(\"-100\", \"100\")?)",
            "exact::ValueType::Decimal(rebuild_decimal(\"-100\", \"100\", 0, 0, exact::RoundingMode::NearestEven)?)",
            "exact::ValueType::Rational(rebuild_rational(rebuild_interval(\"-10\", \"10\")?, rebuild_interval(\"1\", \"1\")?)?)",
        ] {
            assert!(
                through_scalars.contains(reconstruction),
                "scalar member: {reconstruction}"
            );
            assert!(
                through_domains.contains(reconstruction),
                "domain member: {reconstruction}"
            );
        }
    }

    fn assert_numeric_private_sibling_unchanged(actual: &str) {
        let (baseline, _) = numeric_tuple_component(false, None);
        for name in ["oracle_test1", "environment_test1"] {
            let marker = format!("pub fn {name}");
            let body = |source: &str| {
                source
                    .split_once(&marker)
                    .expect("healthy sibling function")
                    .1
                    .split_once("\n}\n")
                    .expect("healthy sibling closes")
                    .0
                    .to_owned()
            };
            assert_eq!(
                body(actual),
                body(&baseline),
                "{name}: independent sibling source remains identical"
            );
        }
    }

    #[trace("FR-018-AC-16", "TC-029")]
    #[test]
    fn wrong_bound_form_private_defense_keeps_exact_missing_integer_range() {
        let (lib, _) = numeric_tuple_component(false, Some("wrong_form"));
        assert_numeric_private_sibling_unchanged(&lib);
        assert!(!lib.contains("pub fn oracle_test0"));
        assert!(!lib.contains("pub fn environment_test0"));
        assert!(lib.contains("pub fn oracle_test1"));
        assert!(lib.contains("pub fn environment_test1"));
    }

    #[trace("FR-018-AC-16", "TC-029")]
    #[test]
    fn unsupported_private_bound_bases_name_the_original_bound_and_preserve_sibling() {
        for base in ["boolean", "enum", "record"] {
            let (lib, _) = numeric_tuple_component(false, Some(base));
            assert_numeric_private_sibling_unchanged(&lib);
            assert!(!lib.contains("pub fn oracle_test0"), "{base}");
            assert!(!lib.contains("pub fn environment_test0"), "{base}");
            assert!(
                lib.contains("pub fn oracle_test1"),
                "{base}: healthy sibling"
            );
            assert!(
                lib.contains("pub fn environment_test1"),
                "{base}: healthy sibling"
            );
        }
    }

    /// A deliberately inconsistent private graph exercises declaration admission only;
    /// it is not a successful source/context admission or public generator request.
    #[trace("FR-018-AC-1", "FR-018-AC-8", "TC-029")]
    #[test]
    fn duplicate_member_private_defense_retains_exact_cause_and_healthy_sibling() {
        let mut nodes = recursive_shape_nodes();
        let members = fixture_members_mut(&mut nodes[0]);
        *fixture_binding_name_mut(&mut members[0]) = "x".into();
        let duplicate = members[0].clone();
        members.push(duplicate);
        let list = EqualityOperandDescriptor::typed(recursive_node_id('a'));
        let tree = EqualityOperandDescriptor::typed(recursive_node_id('c'));
        let (outcomes, source) = check_nominal_items_with_limits(
            nodes,
            &[(list.clone(), list), (tree.clone(), tree)],
            CompositeEqualityLimits {
                source_bytes: usize::MAX,
                ..Default::default()
            },
        );
        assert!(
            matches!(&outcomes[0], Err(CompositeEqualityRefusal::Declaration {
            cause: DeclarationRefusalCause::DuplicateMember { name }, ..
        }) if name == "x")
        );
        assert_eq!(outcomes[1], Ok(()));
        let artifacts = source.finish(usize::MAX).expect("healthy sibling source");
        let lib = &artifacts
            .iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .unwrap()
            .contents;
        assert_eq!(
            lib.matches("pub fn oracle_").count(),
            1,
            "refused item has no oracle"
        );
        assert_eq!(
            lib.matches("pub fn environment_").count(),
            1,
            "refused item has no constructor"
        );
        assert!(!lib.contains("oracle_test0"));
        assert!(lib.contains("oracle_test1"));
    }

    #[trace("FR-018-AC-26", "FR-018-AC-27", "TC-029")]
    #[test]
    fn tc_029_qsl_recursive_package_uses_the_reader_shapes() {
        let (package, context) = qsl_recursive_admission();
        let retained = fixture_nodes(&package);
        let nodes = &retained;
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
        assert!(resolve_shape_with_context(Some(&context), nodes, &list.node_id).is_ok());
        assert!(resolve_shape_with_context(Some(&context), nodes, &tree.node_id).is_ok());
        let list_closure =
            resolve_shape_with_context(Some(&context), nodes, &list.node_id).unwrap();
        let list_key = context
            .composite_declaration(qsl_replay::WireNodeId::from_hex(&list.node_id.digest).unwrap())
            .unwrap()
            .key();
        let CompositeShape::Record(fields) = list_closure.composites[&list_key].shape() else {
            panic!("QSpec List is a record")
        };
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name(), "next");
        assert_eq!(fields[0].presence(), Presence::Optional);
        assert_eq!(fields[0].value_type(), &ValueType::Composite(list_key));
        let tree_closure =
            resolve_shape_with_context(Some(&context), nodes, &tree.node_id).unwrap();
        let tree_key = context
            .composite_declaration(qsl_replay::WireNodeId::from_hex(&tree.node_id.digest).unwrap())
            .unwrap()
            .key();
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
                Some(CardinalityBound::new(0, 3).unwrap()),
            ))
        );
    }

    fn admitted_record_chain(
        records: usize,
        optional: bool,
    ) -> (
        String,
        CheckedPackageV2,
        qsl_replay::AdmittedContext,
        Vec<CheckedNodeId>,
        CheckedNodeId,
    ) {
        use qsl_replay::{
            compile_admitted, DependencyInput, ReplayLimits, SourceIdentity, StageLimits,
        };
        use quire_contract_model::{
            CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2ReadResult,
        };
        let mut source = String::from(
            "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n",
        );
        for index in 0..records {
            let payload = if index + 1 == records {
                "Boolean".to_owned()
            } else {
                format!("Chain{}", index + 1)
            };
            let suffix = if optional { "?" } else { "" };
            source.push_str(&format!(
                "record Chain{index} {{ next: {payload}{suffix}; }}\n"
            ));
        }
        source.push_str("record Shallow { next: Boolean?; }\n");
        // Only fixture intake grows with these unchanged source bytes. CG's
        // resolution, emitted-source and traversal-stack limits stay independent.
        let source_bytes = u64::try_from(source.len()).expect("finite fixture source byte count");
        // Every token in this fixed ASCII grammar occupies at least one source
        // byte; one additional token accommodates EOF. This bounds fixture intake
        // without depending on the compiler's tokenization or changing CG work.
        let token_bound = source_bytes
            .checked_add(1)
            .expect("finite fixture token bound");
        // A one-field record has at most Declaration, RecordDeclaration, Field,
        // TypeReference and QualifiedName nodes. Eight slots per original record
        // plus 32 for the header/root/Shallow declaration bound this fixed grammar.
        let count = u64::try_from(records).expect("fixture node count fits u64");
        let syntax_nodes = count
            .checked_mul(8)
            .and_then(|n| n.checked_add(32))
            .expect("finite fixture syntax node bound");
        let stages = StageLimits::default()
            .with_named("s1.input_bytes", source_bytes)
            .and_then(|limits| limits.with_named("s1.tokens", token_bound))
            .and_then(|limits| limits.with_named("s1.nodes", syntax_nodes))
            .expect("the public compiler recognizes its fixture intake settings");
        eprintln!(
            "genuine large fixture records={records}; optional={optional}; source_bytes={source_bytes}; token_bound={token_bound}; syntax_nodes={syntax_nodes}"
        );
        let admitted = compile_admitted(
            SourceIdentity::new("test", "deep.native", "git", "fixture"),
            "deep.native",
            source.as_bytes(),
            [],
            &DependencyInput::default(),
            stages,
            ReplayLimits::default().with_input_bytes(source_bytes),
        )
        .expect("the unchanged large source must compile before its identities are used");
        let bytes = admitted.package().bytes();
        // Finite fixture-read budgets for the existing large record count and its fixed grammar.
        let read_limits = CheckedPackageReadLimits {
            bytes: u64::try_from(bytes.len()).expect("retained fixture bytes fit u64"),
            nodes: count
                .checked_mul(8)
                .and_then(|value| value.checked_add(32))
                .expect("finite node bound"),
            edges: count
                .checked_mul(32)
                .and_then(|value| value.checked_add(128))
                .expect("finite edge bound"),
            occurrences: count
                .checked_mul(32)
                .and_then(|value| value.checked_add(128))
                .expect("finite occurrence bound"),
            diagnostics: count.checked_add(32).expect("finite diagnostic bound"),
            work: count
                .checked_mul(1024)
                .and_then(|value| value.checked_add(1024))
                .expect("finite read-work bound"),
        };
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        let package = match CheckedPackageV2::read(bytes, read_limits, &evidence) {
            CheckedPackageV2ReadResult::Admitted(package) => *package,
            other => panic!("large original source must pass the real IR reader: {other:?}"),
        };
        let by_name = package
            .graph()
            .nodes
            .iter()
            .filter_map(|node| {
                node.declaration.as_ref().map(|declaration| {
                    (
                        declaration
                            .qualified_name
                            .iter()
                            .map(|part| part.as_ref())
                            .collect::<Vec<_>>()
                            .join("."),
                        node.node_id.clone(),
                    )
                })
            })
            .collect::<BTreeMap<_, _>>();
        let ids = (0..records)
            .map(|index| {
                by_name
                    .get(&format!("Chain{index}"))
                    .expect("original chain record")
                    .clone()
            })
            .collect();
        let leaf = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.semantic_form.as_ref() == "boolean")
            .expect("original Boolean leaf")
            .node_id
            .clone();
        (source, package, admitted.into_parts().1, ids, leaf)
    }

    fn resolve_component(
        graph: &Graph<'_>,
        bounds: &BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2>>,
        closure: &mut TypeClosure,
        root: &CheckedNodeId,
    ) -> Result<ValueType, CompositeEqualityRefusal> {
        resolve_type(None, graph, bounds, closure, root).map_err(|error| match error {
            ItemCheckError::Refusal(refusal) => refusal,
            ItemCheckError::Generation(error) => {
                panic!("unexpected component generation failure: {error:?}")
            }
        })
    }

    fn resolve_nominal_component(
        graph: &Graph<'_>,
        bounds: &BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2>>,
        closure: &mut TypeClosure,
        root: &CheckedNodeId,
    ) -> Result<ValueType, CompositeEqualityRefusal> {
        resolve_type(Some(&recursive_fixture().1), graph, bounds, closure, root).map_err(|error| {
            match error {
                ItemCheckError::Refusal(refusal) => refusal,
                ItemCheckError::Generation(error) => {
                    panic!("unexpected private component error: {error:?}")
                }
            }
        })
    }

    fn resolve_shape(
        nodes: &[FixtureNode],
        type_id: &CheckedNodeId,
    ) -> Result<TypeClosure, CompositeEqualityRefusal> {
        resolve_shape_with_context(Some(&recursive_fixture().1), nodes, type_id)
    }

    fn resolve_shape_with_context(
        context: Option<&qsl_replay::AdmittedContext>,
        nodes: &[FixtureNode],
        type_id: &CheckedNodeId,
    ) -> Result<TypeClosure, CompositeEqualityRefusal> {
        let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
        let mut bounds = BTreeMap::new();
        for node in nodes {
            if CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::BoundedDomain) {
                bounds
                    .entry(&node.semantic_type)
                    .or_insert_with(Vec::new)
                    .push(&node.metadata);
            }
        }
        let mut closure = TypeClosure::default();
        resolve_type(context, &graph, &bounds, &mut closure, type_id).map_err(
            |error| match error {
                ItemCheckError::Refusal(refusal) => refusal,
                ItemCheckError::Generation(error) => {
                    panic!("unexpected private shape generation error: {error:?}")
                }
            },
        )?;
        Ok(closure)
    }

    fn check_nominal_items_with_limits(
        nodes: Vec<FixtureNode>,
        operands: &[(EqualityOperandDescriptor, EqualityOperandDescriptor)],
        limits: CompositeEqualityLimits,
    ) -> (Vec<Result<(), CompositeEqualityRefusal>>, SourceBuilder) {
        check_component_items_with_context(Some(&recursive_fixture().1), nodes, operands, limits)
    }

    fn check_component_items_with_context(
        context: Option<&qsl_replay::AdmittedContext>,
        mut nodes: Vec<FixtureNode>,
        operands: &[(EqualityOperandDescriptor, EqualityOperandDescriptor)],
        limits: CompositeEqualityLimits,
    ) -> (Vec<Result<(), CompositeEqualityRefusal>>, SourceBuilder) {
        assert!(operands.len() <= 2, "fixture has two distinct item ids");
        let mut items = Vec::new();
        for ((left, right), (left_digit, right_digit, expression_digit)) in
            operands.iter().zip([('8', '0', '9'), ('6', '1', '7')])
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
                .map(|node| {
                    FixtureNode::new(
                        node,
                        Some(CheckedTerm::Aggregate {
                            members: Vec::new(),
                        }),
                    )
                })
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
                "body": {"term": "application", "operator": "binary",
                         "operation": {}, "result_type": left.source_type,
                         "arguments": [
                    {"term": "reference", "target": left_id},
                    {"term": "reference", "target": right_id}
                ]},
            }))
            .unwrap();
            let expression = FixtureNode::new(
                expression,
                Some(CheckedTerm::Application {
                    operator: ApplicationOperator::Binary,
                    operation: json!({}),
                    result_type: left.source_type.clone(),
                    arguments: vec![
                        CheckedTerm::Reference {
                            target: left_id.clone(),
                        },
                        CheckedTerm::Reference {
                            target: right_id.clone(),
                        },
                    ],
                }),
            );
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
        let mut bounds = BTreeMap::new();
        for node in &nodes {
            if CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::BoundedDomain) {
                bounds
                    .entry(&node.semantic_type)
                    .or_insert_with(Vec::new)
                    .push(&node.metadata);
            }
        }
        let mut source = SourceBuilder::new(limits.source_bytes).expect("synthetic source starts");
        // Private source-budget/component fixture, not an authentic source capability.
        source.expected_package = Some(match context {
            Some(context) => {
                let record = context.package_id().record();
                CheckedSemanticId {
                    domain: record.domain().as_str().into(),
                    algorithm: "sha256".into(),
                    digest: record.hex().into(),
                }
            }
            None => semantic_id('b'),
        });
        let outcomes = items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                match check_item(
                    context,
                    &graph,
                    &bounds,
                    graph.node(&item.node_id).expect("synthetic node"),
                    item,
                    limits.type_resolution_work_units,
                ) {
                    Ok(checked) => {
                        let rendered = render_item(&checked).expect("healthy item renders");
                        source
                            .item(&format!("test{index}"), item, &rendered)
                            .expect("synthetic item source fits");
                        Ok(())
                    }
                    Err(ItemCheckError::Refusal(refusal)) => Err(refusal),
                    Err(ItemCheckError::Generation(error)) => {
                        panic!("unexpected generation failure: {error:?}")
                    }
                }
            })
            .collect();
        (outcomes, source)
    }

    fn check_nominal_component_item(
        nodes: Vec<FixtureNode>,
        type_id: CheckedNodeId,
    ) -> Result<(), CompositeEqualityRefusal> {
        let typed = EqualityOperandDescriptor::typed(type_id);
        let (mut outcomes, _) = check_nominal_items_with_limits(
            nodes,
            &[(typed.clone(), typed)],
            CompositeEqualityLimits {
                source_bytes: usize::MAX,
                ..Default::default()
            },
        );
        outcomes.remove(0)
    }

    #[trace("FR-018-AC-24", "TC-029")]
    #[test]
    fn tc_029_unclosed_option_and_sequence_cycles_refuse_at_the_repeated_node() {
        const TEST_NAME: &str = "oracle::equality::tests::tc_029_unclosed_option_and_sequence_cycles_refuse_at_the_repeated_node";
        if let Some(form) = std::env::var_os("QUIRE_CG_TC029_CYCLE_CHILD") {
            let form = form.to_str().expect("UTF-8 fixture form");
            let mut nodes = recursive_shape_nodes();
            let (root, repeated) = if form == "option" {
                *fixture_reference_mut(&mut fixture_members_mut(&mut nodes[1])[0]) =
                    recursive_node_id('b');
                (recursive_node_id('a'), recursive_node_id('b'))
            } else {
                *fixture_reference_mut(&mut fixture_members_mut(&mut nodes[3])[0]) =
                    recursive_node_id('d');
                (recursive_node_id('c'), recursive_node_id('d'))
            };
            assert_eq!(
                resolve_shape(&nodes, &root).err(),
                Some(CompositeEqualityRefusal::TypeResolutionCycle {
                    repeated_type_node_id: repeated.clone(),
                }),
                "{form} cycle"
            );
            assert_eq!(
                check_nominal_component_item(nodes.clone(), root).err(),
                Some(CompositeEqualityRefusal::TypeResolutionCycle {
                    repeated_type_node_id: repeated,
                }),
                "{form} item refusal"
            );
            let sibling = if form == "option" {
                recursive_node_id('c')
            } else {
                recursive_node_id('a')
            };
            assert!(resolve_shape(&nodes, &sibling).is_ok(), "{form} sibling");
            assert!(
                check_nominal_component_item(nodes, recursive_node_id('f')).is_ok(),
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
                "{form} subprocess refused or crashed ({}): {} {}",
                outcome.status,
                String::from_utf8_lossy(&outcome.stdout),
                String::from_utf8_lossy(&outcome.stderr)
            );
        }
    }

    #[trace("FR-018-AC-25", "TC-029")]
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
        let bounds = BTreeMap::from([(&nodes[4].semantic_type, vec![&nodes[4].metadata])]);
        let mut closure = TypeClosure::default();
        resolve_nominal_component(&graph, &bounds, &mut closure, &recursive_node_id('a'))
            .expect("List closes");
        assert_eq!(closure.work_consumed, 3); // record, option, repeated record
        resolve_nominal_component(&graph, &bounds, &mut closure, &recursive_node_id('c'))
            .expect("Tree closes");
        assert_eq!(closure.work_consumed, 7); // record, bound, sequence, repeated record
        resolve_nominal_component(&graph, &bounds, &mut closure, &recursive_node_id('a'))
            .expect("cached List");
        assert_eq!(closure.work_consumed, 8); // the cached root still charges

        let count = usize::try_from(COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT).unwrap();
        let (_, original, context, mut ids, leaf) = admitted_record_chain(count, false);
        ids.push(leaf);
        let chain = fixture_nodes(&original);
        let graph: Graph<'_> = chain.iter().map(|node| (&node.node_id, node)).collect();
        let empty = BTreeMap::new();
        let mut over = TypeClosure::default();
        assert_eq!(
            resolve_type(Some(&context), &graph, &empty, &mut over, &ids[0])
                .map_err(|error| match error {
                    ItemCheckError::Refusal(cause) => cause,
                    ItemCheckError::Generation(error) => panic!("{error:?}"),
                })
                .err(),
            Some(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                limit: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
                consumed: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT + 1,
            })
        );
        let mut at_limit = TypeClosure::default();
        assert!(resolve_type(Some(&context), &graph, &empty, &mut at_limit, &ids[2]).is_ok());
        assert_eq!(
            at_limit.work_consumed,
            COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT - 1
        );
        assert!(resolve_type(Some(&context), &graph, &empty, &mut at_limit, &ids[2]).is_ok());
        assert_eq!(
            at_limit.work_consumed,
            COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT
        );
        let mut sibling = TypeClosure::default();
        assert_eq!(
            resolve_component(&graph, &empty, &mut sibling, &ids[count]),
            Ok(ValueType::Boolean)
        );

        // Four roots in one item (left source/target, then right source/target)
        // consume exactly the limit: the first 65,533-node walk plus three
        // cached-root entries. The item must still be generated.
        let at_limit_operand = EqualityOperandDescriptor::converted(ids[4].clone(), ids[4].clone());
        let (at_limit_outcomes, at_limit_source) = check_component_items_with_context(
            Some(&context),
            chain.clone(),
            &[(at_limit_operand.clone(), at_limit_operand)],
            CompositeEqualityLimits {
                source_bytes: usize::MAX,
                ..Default::default()
            },
        );
        // This work-accounting fixture already selected an unlimited source
        // allowance above; do not reapply the unrelated default while inspecting it.
        let at_limit_source = at_limit_source
            .finish(usize::MAX)
            .expect("the selected work fixture source allowance admits its output")
            .into_iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .expect("the at-limit item emits a library")
            .contents;
        assert_eq!(at_limit_outcomes, vec![Ok(())]);
        assert!(at_limit_source.contains("pub fn oracle_test0"));

        // Both items share one synthetic lowered graph and emission pass.
        let exhausted = EqualityOperandDescriptor::typed(ids[0].clone());
        let healthy = EqualityOperandDescriptor::typed(ids[count].clone());
        let (outcomes, emitted) = check_component_items_with_context(
            Some(&context),
            chain,
            &[(exhausted.clone(), exhausted), (healthy.clone(), healthy)],
            CompositeEqualityLimits {
                source_bytes: usize::MAX,
                ..Default::default()
            },
        );
        let emitted = test_source(emitted);
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

    #[trace("FR-018-AC-25", "TC-029")]
    #[test]
    fn tc_029_deep_option_resolution_and_runtime_type_operations_fit_a_small_stack() {
        const DEPTH: usize = 2_048;
        let ids: Vec<CheckedNodeId> = (0..=DEPTH)
            .map(|index| CheckedNodeId {
                domain: "quire.checked-semantic-node/v1".into(),
                digest: format!("{:064x}", index + 1).into(),
            })
            .collect();
        let nodes: Vec<FixtureNode> = (0..=DEPTH)
            .map(|index| {
                let (tag, form, body) = if index == DEPTH {
                    (
                        "scalar_type",
                        "boolean",
                        CheckedTerm::Aggregate { members: vec![] },
                    )
                } else {
                    (
                        "composite_type",
                        "option",
                        CheckedTerm::Aggregate {
                            members: vec![CheckedTerm::Reference {
                                target: ids[index + 1].clone(),
                            }],
                        },
                    )
                };
                let metadata = serde_json::from_value::<CheckedSemanticNodeV2>(json!({
                    "node_id": ids[index],
                    "schema_version": "quire.checked-semantic-graph/v2",
                    "node_tag": tag,
                    "semantic_form": form,
                    "semantic_type": ids[index],
                    "dependencies": [],
                    "occurrences": [],
                    "body": body,
                }))
                .expect("synthetic deep Option node");
                FixtureNode::new(metadata, Some(body))
            })
            .collect();
        std::thread::Builder::new()
            .stack_size(64 * 1024)
            .spawn(move || {
                let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
                let mut closure = TypeClosure::default();
                let value_type = resolve_component(&graph, &BTreeMap::new(), &mut closure, &ids[0])
                    .expect("deep Option resolves within the work budget");
                assert_eq!(closure.work_consumed, DEPTH as u64 + 1);
                assert!(matches!(value_type, ValueType::Option(_)));
                let copy = value_type.clone();
                assert_eq!(copy, value_type);
                let rendered = render_value_type(&value_type, &[], &BTreeMap::new())
                    .expect("deep Option renders");
                assert_eq!(rendered.matches("exact::ValueType::option(").count(), DEPTH);
                assert!(rendered.contains("exact::ValueType::Boolean"));
                assert!(rendered.len() < MAX_GENERATED_SOURCE_BYTES);
                drop(copy);
                drop(value_type);
            })
            .expect("small-stack worker starts")
            .join()
            .expect("deep Option operations fit the worker stack");
    }

    #[trace("FR-018-AC-29", "TC-029")]
    #[test]
    fn cycle_refusal_follows_its_charged_entry_and_work_refusal_wins_at_the_boundary() {
        let mut nodes = recursive_shape_nodes();
        *fixture_reference_mut(&mut fixture_members_mut(&mut nodes[1])[0]) = recursive_node_id('b');
        let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
        let mut cycle = TypeClosure {
            work_limit: 3,
            ..TypeClosure::default()
        };
        assert_eq!(
            resolve_nominal_component(
                &graph,
                &BTreeMap::new(),
                &mut cycle,
                &recursive_node_id('b')
            ),
            Err(CompositeEqualityRefusal::TypeResolutionCycle {
                repeated_type_node_id: recursive_node_id('b')
            })
        );
        assert_eq!(cycle.work_consumed, 2);
        let mut denied = TypeClosure {
            work_limit: 1,
            ..TypeClosure::default()
        };
        assert_eq!(
            resolve_nominal_component(
                &graph,
                &BTreeMap::new(),
                &mut denied,
                &recursive_node_id('b')
            ),
            Err(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                limit: 1,
                consumed: 2
            })
        );
        let mut sibling = TypeClosure::default();
        assert_eq!(
            resolve_nominal_component(
                &graph,
                &BTreeMap::new(),
                &mut sibling,
                &recursive_node_id('f')
            ),
            Ok(ValueType::Integer)
        );

        let cyclic = EqualityOperandDescriptor::typed(recursive_node_id('b'));
        let healthy = EqualityOperandDescriptor::typed(recursive_node_id('f'));
        let requests = [(cyclic.clone(), cyclic), (healthy.clone(), healthy)];
        let (cycle_items, cycle_source) = check_nominal_items_with_limits(
            nodes.clone(),
            &requests,
            CompositeEqualityLimits {
                type_resolution_work_units: 3,
                ..CompositeEqualityLimits::default()
            },
        );
        assert_eq!(
            cycle_items,
            vec![
                Err(CompositeEqualityRefusal::TypeResolutionCycle {
                    repeated_type_node_id: recursive_node_id('b'),
                }),
                Ok(()),
            ]
        );
        let cycle_lib = test_source(cycle_source);
        assert!(!cycle_lib.contains("oracle_test0"));
        assert!(cycle_lib.contains("oracle_test1"));

        let mut denied_nodes = nodes;
        let mut extra = denied_nodes[1].clone();
        extra.node_id = recursive_node_id('5');
        extra.semantic_type = recursive_node_id('5');
        *fixture_reference_mut(&mut fixture_members_mut(&mut extra)[0]) = recursive_node_id('b');
        *fixture_reference_mut(&mut fixture_members_mut(&mut denied_nodes[1])[0]) =
            recursive_node_id('5');
        denied_nodes.push(extra);
        let (denied_items, denied_source) = check_nominal_items_with_limits(
            denied_nodes,
            &requests,
            CompositeEqualityLimits {
                type_resolution_work_units: 2,
                ..CompositeEqualityLimits::default()
            },
        );
        assert_eq!(
            denied_items,
            vec![
                Err(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                    limit: 2,
                    consumed: 3,
                }),
                Ok(()),
            ]
        );
        let denied_lib = test_source(denied_source);
        assert!(!denied_lib.contains("oracle_test0"));
        assert!(denied_lib.contains("oracle_test1"));
    }

    #[trace("FR-018-AC-31", "TC-029")]
    #[test]
    fn maximum_work_setting_refuses_and_last_denied_count_is_representable() {
        let mut closure = TypeClosure {
            work_limit: u64::MAX - 1,
            work_consumed: u64::MAX - 1,
            ..TypeClosure::default()
        };
        assert_eq!(
            resolve_component(
                &std::iter::empty::<(&CheckedNodeId, &FixtureNode)>().collect::<Graph<'_>>(),
                &BTreeMap::new(),
                &mut closure,
                &node_id('a')
            ),
            Err(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                limit: u64::MAX - 1,
                consumed: u64::MAX,
            })
        );
    }

    #[trace("FR-018-AC-30", "TC-029")]
    #[test]
    fn source_limit_refuses_before_a_function_or_module_fragment_is_appended() {
        let limit = SOURCE_HEADER.len();
        let mut source = SourceBuilder::new(limit).expect("header fits exactly");
        source.expected_package = Some(semantic_id('b'));
        assert_eq!(
            source.append_function("x".to_owned()),
            Err(OracleGenerationError::SourceTooLarge {
                bytes: limit + 1,
                limit,
                setting: "cg.equality.source_bytes",
            })
        );
        assert!(source.functions.is_empty());
        assert_eq!(source.charged_bytes, limit);
        assert_eq!(
            source.add_composite_module("test", "test", 0, "x", &mut String::new(), &[]),
            Err(OracleGenerationError::SourceTooLarge {
                bytes: limit + 1,
                limit,
                setting: "cg.equality.source_bytes",
            })
        );
        assert!(source.modules.is_empty());
        assert!(source.module_declarations.is_empty());
    }

    #[trace("TC-001", "NFR-002-AC-1", "NFR-002-AC-2")]
    #[test]
    fn extra_equality_module_has_its_own_header() {
        let mut source = SourceBuilder::default();
        source
            .add_composite_module("test", "test", 0, "", &mut String::new(), &[])
            .expect("small module fits");
        let artifacts = source
            .finish(MAX_GENERATED_SOURCE_BYTES)
            .expect("source fits");
        let module = artifacts
            .iter()
            .find(|artifact| artifact.path.starts_with("src/composites_"))
            .expect("extra module emitted");
        let header = module
            .contents
            .lines()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(header.contains("// @generated by quire-contract-codegen"));
        assert!(header.contains("// SPDX-License-Identifier: MIT OR Apache-2.0"));
        assert!(header.contains("test"));
        assert!(!header
            .replacen("// @generated by quire-contract-codegen", "", 1)
            .contains("// @generated by quire-contract-codegen"));
        assert!(!header
            .replacen("// SPDX-License-Identifier: MIT OR Apache-2.0", "", 1)
            .contains("// SPDX-License-Identifier: MIT OR Apache-2.0"));
        assert!(!header.replace("test", "").contains("test"));
    }

    #[trace("FR-018-AC-28", "FR-018-AC-30", "TC-029")]
    #[test]
    fn deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack() {
        const RECORDS: usize = 50_000;
        let (original_source, original, context, ids, leaf) = admitted_record_chain(RECORDS, true);
        let root = ids[0].clone();
        let (artifacts, _root) = std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(move || {
                let nodes = fixture_nodes(&original);
                let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
                let mut default = TypeClosure::default();
                assert_eq!(
                    resolve_type(
                        Some(&context),
                        &graph,
                        &BTreeMap::new(),
                        &mut default,
                        &root
                    )
                    .map_err(|error| match error {
                        ItemCheckError::Refusal(cause) => cause,
                        ItemCheckError::Generation(error) => panic!("{error:?}"),
                    }),
                    Err(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                        limit: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
                        consumed: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT + 1,
                    })
                );
                let entry_count = 2 * RECORDS as u64 + 1;
                let mut one_below = TypeClosure {
                    work_limit: entry_count - 1,
                    ..TypeClosure::default()
                };
                assert_eq!(
                    resolve_type(
                        Some(&context),
                        &graph,
                        &BTreeMap::new(),
                        &mut one_below,
                        &root
                    )
                    .map_err(|error| match error {
                        ItemCheckError::Refusal(cause) => cause,
                        ItemCheckError::Generation(error) => panic!("{error:?}"),
                    }),
                    Err(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                        limit: entry_count - 1,
                        consumed: entry_count,
                    })
                );
                let deep = EqualityOperandDescriptor::typed(root.clone());
                let healthy = EqualityOperandDescriptor::typed(leaf.clone());
                let requests = [(deep.clone(), deep), (healthy.clone(), healthy)];
                for limit in [
                    COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
                    entry_count - 1,
                ] {
                    let (outcomes, source) = check_component_items_with_context(
                        Some(&context),
                        nodes.clone(),
                        &requests,
                        CompositeEqualityLimits {
                            type_resolution_work_units: limit,
                            source_bytes: usize::MAX,
                        },
                    );
                    assert_eq!(
                        outcomes,
                        vec![
                            Err(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                                limit,
                                consumed: limit + 1,
                            }),
                            Ok(()),
                        ]
                    );
                    let source = source.finish(usize::MAX).expect("sibling source");
                    let lib = source
                        .iter()
                        .find(|artifact| artifact.path == "src/lib.rs")
                        .expect("sibling library");
                    assert!(!lib.contents.contains("oracle_test0"));
                    assert!(lib.contents.contains("oracle_test1"));
                }
                let operand = EqualityOperandDescriptor::typed(root.clone());
                let (outcomes, source) = check_component_items_with_context(
                    Some(&context),
                    nodes,
                    &[(operand.clone(), operand)],
                    CompositeEqualityLimits {
                        type_resolution_work_units: entry_count + 1,
                        source_bytes: usize::MAX,
                    },
                );
                assert_eq!(outcomes, vec![Ok(())]);
                let all = source.finish(usize::MAX).expect("complete source");
                let bytes: usize = all.iter().map(|artifact| artifact.contents.len()).sum();
                assert!(bytes > MAX_GENERATED_SOURCE_BYTES);
                assert!(matches!(
                    source.finish(MAX_GENERATED_SOURCE_BYTES),
                    Err(OracleGenerationError::SourceTooLarge {
                        limit: MAX_GENERATED_SOURCE_BYTES,
                        setting: "cg.equality.source_bytes",
                        ..
                    })
                ));
                assert_eq!(
                    source.finish(bytes - 1),
                    Err(OracleGenerationError::SourceTooLarge {
                        bytes,
                        limit: bytes - 1,
                        setting: "cg.equality.source_bytes",
                    })
                );
                let exact = source.finish(bytes).expect("exact source limit admits");
                assert!(exact
                    .iter()
                    .all(|artifact| artifact.contents.len() <= MAX_ARTIFACT_BYTES));
                assert!(exact.iter().any(|artifact| artifact.path != "src/lib.rs"));
                let claim = format!(
                    "// Subject: {}",
                    crate::core::provenance::node_subject(&node_id('9'))
                );
                for module in exact
                    .iter()
                    .filter(|artifact| artifact.path.starts_with("src/composites_"))
                {
                    let header = module
                        .contents
                        .lines()
                        .take_while(|line| !line.is_empty())
                        .collect::<BTreeSet<_>>();
                    assert!(
                        header.contains(claim.as_str()),
                        "{} lacks its claim subject",
                        module.path
                    );
                }
                // Genuine owner keys determine declaration order. Locate the module
                // that actually reconstructs Chain0, rather than assuming its key sorts first.
                let root_lookup = format!(
                    "sv::CompositeDeclaration::new(lookup_composite_key(_context, {:?}, &[\"Chain0\"], qsl_replay::AdmittedCompositeForm::Record)?",
                    root.digest,
                );
                let root_module = exact
                    .iter()
                    .find(|artifact| artifact.path.starts_with("src/composites_")
                        && artifact.contents.contains(&root_lookup))
                    .expect("an extra module reconstructs the genuine root declaration");
                let root_subject = format!(
                    "// Subject: {}",
                    crate::core::provenance::node_subject(&root)
                );
                assert!(
                    root_module
                        .contents
                        .lines()
                        .take_while(|line| !line.is_empty())
                        .any(|line| line == root_subject),
                    "the root's reconstruction module lacks its composite subject"
                );
                (exact, root)
            })
            .expect("small-stack worker starts")
            .join()
            .expect("deep generation fits 512 KiB stack");

        let scratch = crate::core::test_support::discover_scratch("oracle-equality-deep");
        std::fs::create_dir_all(scratch.join("src")).expect("scratch source directory");
        std::fs::create_dir_all(scratch.join("tests")).expect("scratch test directory");
        std::fs::write(
            scratch.join("Cargo.toml"),
            equality_oracle_crate_manifest(COMPOSITE_EQUALITY_CRATE_NAME),
        )
        .expect("scratch manifest");
        std::fs::write(scratch.join("deep.native"), &original_source)
            .expect("retain exact original nominal source");
        for artifact in &artifacts {
            std::fs::write(scratch.join(&artifact.path), &artifact.contents)
                .expect("generated source");
        }
        let test = r#"
mod generated {
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
#[test]
fn deep_matches_shallow() {
    let source = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/deep.native"));
    let admitted = qsl_replay::compile_admitted(
        qsl_replay::SourceIdentity::new("test", "deep.native", "git", "fixture"),
        "deep.native", source, [], &qsl_replay::DependencyInput::default(),
        {
            let bytes = u64::try_from(source.len()).expect("finite retained source bytes");
            let tokens = bytes.checked_add(1).expect("finite byte-derived token allowance plus EOF");
            let nodes = (__RECORDS___u64).checked_mul(8).and_then(|n| n.checked_add(32))
                .expect("finite fixed-record grammar node allowance");
            qsl_replay::StageLimits::default()
                .with_named("s1.input_bytes", bytes)
                .and_then(|limits| limits.with_named("s1.tokens", tokens))
                .and_then(|limits| limits.with_named("s1.nodes", nodes))
                .expect("public fixed-fixture intake settings")
        },
        qsl_replay::ReplayLimits::default().with_input_bytes(u64::try_from(source.len()).unwrap()),
    ).expect("same retained source must admit independently");
    std::thread::Builder::new().stack_size(512 * 1024).spawn(move || {
        let context = admitted.context();
        let key = |name: &str| context.composites().find(|declaration|
            declaration.qualified_name().iter().map(|part| part.as_str()).eq([name])
        ).expect("original source declaration").key();
        let deep = environment_test0(Some(context)).expect("deep genuine prepared environment");
        let deep_value = deep.environment.record(key("Chain0"), vec![]).expect("absent optional field");
        let shallow_key = key("Shallow");
        let shallow = sv::TypeEnvironment::new(vec![sv::CompositeDeclaration::new(
            shallow_key, "Shallow", sv::CompositeShape::Record(vec![sv::FieldDeclaration::new(
                "next", exact::ValueType::Boolean, exact::Presence::Optional,
            )]),
        )], core::iter::empty::<sv::ObjectTypeDeclaration>()).expect("shallow environment");
        let shallow_value = shallow.record(shallow_key, vec![]).expect("absent optional field");
        let limits = exact::ScalarLimits {
            integer_bits: u64::MAX, decimal_digits: u64::MAX,
            scale_expansion: u64::MAX, text_input_bytes: u64::MAX,
            text_scalars: u64::MAX, normalized_scalars: u64::MAX,
            unit_edges: u64::MAX, value_occurrences: u64::MAX,
            work_units: u64::MAX, result_units: u64::MAX,
        };
        let actual = oracle_test0(&deep, &deep_value, &deep_value, &mut exact::Meter::new(limits));
        let expected = shallow.check_equality(
            sv::EqualityOperator::Equal,
            sv::EqualityOperand::typed(exact::ValueType::Composite(shallow_key)),
            sv::EqualityOperand::typed(exact::ValueType::Composite(shallow_key)),
            &quire_semantic_value::enumeration::EnumMemberIndex::default(),
        ).expect("shallow equality").evaluate(&shallow, &shallow_value, &shallow_value, &mut exact::Meter::new(limits));
        assert_eq!(actual, expected);
        assert_eq!(actual, exact::Outcome::Completed(true));
    }).expect("runtime worker starts").join().expect("runtime fits 512 KiB");
}
}
"#;
        let test = test.replace("__RECORDS__", &RECORDS.to_string());
        std::fs::write(scratch.join("tests/deep.rs"), test).expect("deep agreement test");
        // Recompilation must use the same public owner cohort as generation. Cargo
        // adapts the scratch package entry while retaining those resolved dependencies.
        std::fs::copy(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
            scratch.join("Cargo.lock"),
        )
        .expect("seed the generated fixture with the executing owner's cohort");
        let output = std::process::Command::new(env!("CARGO"))
            .args([
                "test",
                "--offline",
                "--quiet",
                "--test",
                "deep",
                "--",
                "--nocapture",
            ])
            .env(
                "CARGO_TARGET_DIR",
                concat!(env!("CARGO_MANIFEST_DIR"), "/target/generated-equality"),
            )
            .current_dir(&scratch)
            .output()
            .expect("generated crate test starts");
        std::fs::remove_dir_all(&scratch).expect("remove scratch crate");
        assert!(
            output.status.success(),
            "generated crate failed: {} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[trace("FR-018-AC-25", "TC-029")]
    #[test]
    fn tc_029_inline_collection_bound_is_charged_before_lookup() {
        let mut nodes = recursive_shape_nodes();
        // Inline bounds remain meaningful for set/bag forms. A sequence's bound is a separate
        // typed node, so giving it a second body reference would now be malformed first.
        nodes[3].semantic_form = "set".into();
        nodes[3].body = Some(CheckedTerm::Aggregate {
            members: vec![
                CheckedTerm::Reference {
                    target: recursive_node_id('f'),
                },
                CheckedTerm::Reference {
                    target: recursive_node_id('7'),
                },
            ],
        });
        let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
        let mut at_limit = TypeClosure {
            work_consumed: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT - 2,
            ..TypeClosure::default()
        };
        assert_eq!(
            resolve_nominal_component(
                &graph,
                &BTreeMap::new(),
                &mut at_limit,
                &recursive_node_id('d')
            ),
            Err(CompositeEqualityRefusal::UnknownTypeNode {
                type_node_id: recursive_node_id('7'),
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
            resolve_nominal_component(
                &graph,
                &BTreeMap::new(),
                &mut over_limit,
                &recursive_node_id('d')
            ),
            Err(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                limit: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
                consumed: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT + 1,
            })
        );
    }

    #[trace("FR-018-AC-26", "FR-018-AC-27", "TC-029")]
    #[test]
    fn tc_029_recursive_list_and_tree_shapes_close_at_record_keys() {
        let nodes = recursive_shape_nodes();
        let list = resolve_shape(&nodes, &recursive_node_id('a')).expect("List closes");
        let list_key = recursive_owner_key("List");
        let CompositeShape::Record(list_fields) = list.composites[&list_key].shape() else {
            panic!("List is a record")
        };
        assert_eq!(list_fields.len(), 1);
        assert_eq!(list_fields[0].name(), "next");
        assert_eq!(list_fields[0].presence(), Presence::Optional);
        assert_eq!(list_fields[0].value_type(), &ValueType::Composite(list_key));

        let tree = resolve_shape(&nodes, &recursive_node_id('c')).expect("Tree closes");
        let tree_key = recursive_owner_key("Tree");
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
                Some(CardinalityBound::new(0, 3).unwrap()),
            ))
        );
    }

    #[trace("FR-018-AC-26", "TC-029")]
    #[test]
    fn tc_029_optional_wrapper_malformed_shapes_refuse_without_affecting_tree() {
        for mutation in ["remove", "rename", "second", "non_option"] {
            let mut nodes = recursive_shape_nodes();
            let value = fixture_binding_value_mut(&mut fixture_members_mut(&mut nodes[0])[0]);
            let CheckedTerm::Aggregate { members: optional } = value else {
                panic!("optional wrapper")
            };
            match mutation {
                "remove" => optional.clear(),
                "rename" => *fixture_binding_name_mut(&mut optional[0]) = "maybe".into(),
                "second" => {
                    let duplicate = optional[0].clone();
                    optional.push(duplicate);
                }
                "non_option" => {
                    *fixture_reference_mut(fixture_binding_value_mut(&mut optional[0])) =
                        recursive_node_id('f')
                }
                _ => unreachable!(),
            }
            assert_eq!(
                resolve_shape(&nodes, &recursive_node_id('a')).err(),
                Some(CompositeEqualityRefusal::MalformedComposite {
                    composite: recursive_node_id('a')
                }),
                "{mutation}"
            );
            assert!(
                resolve_shape(&nodes, &recursive_node_id('c')).is_ok(),
                "{mutation}: healthy Tree"
            );
        }
    }

    #[trace("FR-018-AC-26", "TC-029")]
    #[test]
    fn tc_029_direct_option_field_is_required_not_silently_optional() {
        let mut nodes = recursive_shape_nodes();
        *fixture_binding_value_mut(&mut fixture_members_mut(&mut nodes[0])[0]) =
            CheckedTerm::Reference {
                target: recursive_node_id('b'),
            };
        let closure =
            resolve_shape(&nodes, &recursive_node_id('a')).expect("direct option is valid");
        let key = recursive_owner_key("List");
        let CompositeShape::Record(fields) = closure.composites[&key].shape() else {
            panic!("List is a record")
        };
        assert_eq!(fields[0].presence(), Presence::Required);
        assert_eq!(
            fields[0].value_type(),
            &ValueType::option(ValueType::Composite(key))
        );
    }

    #[trace("FR-018-AC-27", "TC-029")]
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
                "non_sequence" => nodes[4].semantic_type = recursive_node_id('f'),
                "missing_min" => {
                    fixture_members_mut(&mut nodes[4]).remove(0);
                }
                "duplicate_max" => {
                    let members = fixture_members_mut(&mut nodes[4]);
                    let duplicate = members[1].clone();
                    members.push(duplicate);
                }
                "non_integer" => {
                    let CheckedTerm::Literal { value_kind, .. } =
                        fixture_binding_value_mut(&mut fixture_members_mut(&mut nodes[4])[0])
                    else {
                        panic!("literal bound")
                    };
                    *value_kind = LiteralKind::Text;
                }
                _ => unreachable!(),
            }
            let expected = if mutation == "non_sequence" {
                CompositeEqualityRefusal::Unsupported {
                    unsupported_node_id: recursive_node_id('e'),
                    node_tag: "bounded_domain",
                }
            } else {
                CompositeEqualityRefusal::UnreadableBound {
                    bound: recursive_node_id('e'),
                }
            };
            assert_eq!(
                resolve_shape(&nodes, &recursive_node_id('c')).err(),
                Some(expected),
                "{mutation}"
            );
            assert!(
                resolve_shape(&nodes, &recursive_node_id('a')).is_ok(),
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

    fn member(name: &str, value: &str) -> CheckedTerm {
        CheckedTerm::Binding { name: (name).into(), value: Box::new(CheckedTerm::Literal { type_id: serde_json::from_value(serde_json::json!({"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)})).expect("fixture type key"), value_kind: LiteralKind::Integer, value: serde_json::json!(value) }) }
    }

    fn text_member(name: &str, value: &str) -> CheckedTerm {
        CheckedTerm::Binding {
            name: (name).into(),
            value: Box::new(CheckedTerm::Literal {
                type_id: node_id('0'),
                value_kind: LiteralKind::Text,
                value: serde_json::json!(value),
            }),
        }
    }

    fn bare(kind: &str, value: &str) -> CheckedTerm {
        CheckedTerm::Literal {
            type_id: node_id('0'),
            value_kind: LiteralKind::from_wire(kind).expect("closed fixture kind"),
            value: serde_json::json!(value),
        }
    }

    /// Private parser defense over binding-shaped members. Reordered integer/collection
    /// inputs here are not public reader-admitted generator inputs; intake controls are separate.
    #[trace("FR-018-AC-14", "TC-029")]
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

    /// Private parser defense refuses bare members without weakening the reader boundary.
    /// Integer/collection malformed shapes are already excluded before public generation.
    #[trace("FR-018-AC-14", "TC-029")]
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
        ValueType::Quantity(
            quire_semantic_value::unit::compound_unit_id(
                core::iter::empty::<([u8; 32], &exact::Integer)>(),
                quire_semantic_value::semantic_node::IDENTITY_LIMITS,
            )
            .expect("the actual owner's empty compound unit is dimensionless"),
        )
    }

    fn reference() -> ValueType {
        ValueType::Reference(exact::EffectiveId::from_digest([7; 32]))
    }

    fn node_id(digit: char) -> CheckedNodeId {
        CheckedNodeId {
            domain: "quire.checked-semantic-node/v1".into(),
            digest: digit.to_string().repeat(64).into(),
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
                contract_package_identity: semantic_id('a'),
                node_id: node_id(node),
                package_id: semantic_id('b'),
                semantic_type: node_id('c'),
                source_map: Vec::new(),
                claims: Vec::new(),
                bounds: Vec::new(),
                dependencies: Vec::new(),
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

    #[test]
    fn generated_claim_serializes_package_and_requested_id_without_node_ir_id() {
        let claim = generated_claim('8');
        let ClaimDisposition::Generated(generated) = &claim.result else {
            panic!("fixture must generate");
        };
        let value = serde_json::to_value(generated).expect("generated claim serializes");
        assert_eq!(
            value["contract_package_identity"],
            serde_json::json!(semantic_id('a'))
        );
        assert_eq!(value["node_id"], serde_json::json!(claim.node_id));
        assert_eq!(value["package_id"], serde_json::json!(semantic_id('b')));
        assert_eq!(value["bounds"], serde_json::json!([]));
        assert_eq!(value["dependencies"], serde_json::json!([]));
        assert!(value.get("ir_id").is_none());
    }

    fn rendered_item() -> RenderedItem {
        RenderedItem {
            union_seals: String::new(),
            requires_context: false,
            composites: Vec::new(),
            left_source: "exact::ValueType::Integer".to_owned(),
            left_target: None,
            right_source: "exact::ValueType::Integer".to_owned(),
            right_target: None,
        }
    }

    /// `render_value_type` returns the typed error for `ValueType::Quantity` and for
    /// `ValueType::Reference`, directly, nested in an option, and inside a composite declaration
    /// (the second call site, beside an operand's own types). It does not panic.
    #[trace("FR-018-AC-18", "TC-029")]
    #[test]
    fn tc_029_ac18_render_value_type_refuses_quantity_and_reference_with_a_typed_error() {
        for (value_type, family) in [(quantity(), "quantity"), (reference(), "reference")] {
            let expected = Err(RenderError::UnsupportedValueType { family });
            assert_eq!(
                render_value_type(&value_type, &[], &BTreeMap::new()),
                expected
            );
            assert_eq!(
                render_value_type(
                    &ValueType::option(value_type.clone()),
                    &[],
                    &BTreeMap::new()
                ),
                expected,
                "{family} nested in an option"
            );
            let declaration = CompositeDeclaration::new(
                recursive_owner_key("List"),
                "D",
                CompositeShape::Tuple(vec![ValueType::Integer, value_type]),
            );
            assert_eq!(
                render_composite_declaration(&declaration, &[], &BTreeMap::new()),
                expected,
                "{family} in a composite declaration"
            );
        }
    }

    /// The item boundary turns `RenderError::UnsupportedValueType` into the per-item refusal,
    /// naming the family and the item's expression node, renders no code for it, and leaves a
    /// sibling's claim unchanged; `RenderError::Generation` still fails the whole call, with the
    /// carried error, and touches no claim.
    #[trace("FR-018-AC-18", "TC-029")]
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
            settle_render(&mut item, Err(RenderError::Generation(error.clone()))).err(),
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
        let admitted = qsl_replay::compile_admitted(
            qsl_replay::SourceIdentity::new("test", "render-boundary.native", "git", "fixture"),
            "render-boundary.native",
            b"language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\ndimension Length;\nunit m : Length = rational(1, 1);\nfunction same using v(a: Quantity<m>, b: Quantity<m>): Boolean pure { a = b }\n",
            [], &qsl_replay::DependencyInput::default(), qsl_replay::StageLimits::default(),
            qsl_replay::ReplayLimits::default(),
        ).expect("the render boundary has a genuine owner-admitted quantity unit");
        let record = admitted.context().package_id().record();
        let package = CheckedSemanticId {
            domain: record.domain().as_str().into(),
            algorithm: "sha256".into(),
            digest: record.hex().into(),
        };
        for claim in &mut claims {
            let ClaimDisposition::Generated(claim) = &mut claim.result else {
                panic!("the fixture starts generated");
            };
            claim.package_id = package.clone();
        }
        let unit = admitted
            .context()
            .units()
            .next()
            .expect("original metre unit")
            .id();
        let environment = TypeEnvironment::new([], [])
            .expect("scalar environment")
            .with_units(admitted.context().unit_table().clone());
        let pending = requests
            .iter()
            .zip(fails)
            .enumerate()
            .map(|(position, (item, fails))| {
                let unsupported_quantity = fails
                    && matches!(
                        error,
                        RenderError::UnsupportedValueType { family: "quantity" }
                    );
                let value_type = if unsupported_quantity {
                    ValueType::Quantity(unit)
                } else {
                    ValueType::Integer
                };
                let checked = environment
                    .check_equality(
                        item.operator.to_runtime(),
                        EqualityOperand::typed(value_type.clone()),
                        EqualityOperand::typed(value_type.clone()),
                        &quire_semantic_value::enumeration::EnumMemberIndex::default(),
                    )
                    .expect("the owner admits this equality plan before rendering");
                let prepared = CheckedItem {
                    checked,
                    left_source: value_type.clone(),
                    right_source: value_type,
                    left_target: None,
                    right_target: None,
                    composites: Vec::new(),
                    declaration_keys: Vec::new(),
                    enums: Vec::new(),
                    composite_bindings: BTreeMap::new(),
                };
                (
                    position,
                    DescriptorKey::of(item),
                    item,
                    (fails, unsupported_quantity, prepared),
                )
            })
            .collect();
        let mut source = SourceBuilder {
            expected_package: Some(package),
            ..SourceBuilder::default()
        };
        let result = render_and_emit(
            &mut claims,
            &mut source,
            pending,
            |(fails, unsupported_quantity, checked)| {
                // Quantity's actual render error follows a valid owner-checked plan.
                // The whole-call generation error remains a deliberate boundary injection.
                let rendered = render_item(checked)?;
                if *fails {
                    assert!(
                        !unsupported_quantity,
                        "the actual quantity renderer must already refuse"
                    );
                    Err(error.clone())
                } else {
                    Ok(rendered)
                }
            },
        );
        (result, claims, test_source(source))
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
    #[trace("FR-018-AC-18", "TC-029")]
    #[test]
    fn tc_029_ac18_a_render_failure_refuses_only_its_item_and_never_renames_a_sibling() {
        let quantity = RenderError::UnsupportedValueType { family: "quantity" };

        // Both render: the stem is shared, so the two are numbered in key order.
        let (result, claims, lib) = emit_two([false, false], quantity.clone());
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
        let (result, claims, lib) = emit_two([false, true], quantity.clone());
        assert_eq!(result, Ok(()));
        assert_eq!(
            symbols(&claims[0]),
            Some(("environment_equality_equal", "oracle_equality_equal"))
        );
        assert_eq!(claims[1].result, unsupported('2', "quantity"));
        assert_eq!(claims[1].node_id, node_id('2'));
        assert_eq!(lib.matches("pub fn environment_").count(), 1);

        // The first fails: the second is the one that survives, under the bare stem.
        let (result, claims, lib) = emit_two([true, false], quantity.clone());
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
        let (result, _, _) = emit_two([false, true], RenderError::Generation(error.clone()));
        assert_eq!(result, Err(error));
    }
}

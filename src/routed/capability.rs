//! Capability settlement: the one point where a requested claim is settled.
//!
//! AD-016 places capability negotiation here so that language admission stays
//! language-only and the IR stays target-neutral. The boundary holds only when
//! adding a backend kind is a compile error everywhere it must be handled, so
//! the settlement point dispatches one arm per variant of [`BackendKind`] with
//! no catch-all: a kind with no arm does not build, rather than reaching a run
//! with no settlement and nothing saying so.
//!
//! `candidates` is read, never computed. The `quire-spec-language` registry
//! computes it under FR-290's candidate-set rule; this module is the consumer
//! on the far side of the FR-331 envelope.

use std::collections::{BTreeMap, BTreeSet};

use qsl_replay::{DomainKey, DomainKind, ProofBound};
use quire_contract_model::ClauseKind;
use quire_contract_model::{CheckedNodeId, ContractPackageV1};
use serde::Serialize;

use crate::core::contract_package::{
    AdmittedItemAccounting, ContractRequestPosition, PackageItem, PackageItems, PackageRequestError,
};

fn serialize_domain_kind<S: serde::Serializer>(
    kind: &DomainKind,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(kind.to_wire())
}

/// The FR-331 envelope's contract version, the only one this module reads.
pub const BACKEND_PROVIDER_CONTRACT: &str = "quire.backend-provider/v1";

/// The FR-290 vocabulary identity, the only one this module reads.
pub const CAPABILITY_VOCABULARY: &str = "quire.capability-kind/v1";

/// The ten FR-290 capability kinds.
///
/// The vocabulary is closed. A label outside it is refused with its exact
/// received bytes and never mapped onto a member.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CapabilityKind {
    /// Whether a Boolean clause holds for every assignment of its domain.
    ValueValidity,
    /// Whether an operation's contract clauses hold for every admitted pre-state.
    OperationContract,
    /// Whether one finite execution conforms step by step.
    FiniteReplay,
    /// Whether a temporal requirement holds over its profile's traces.
    TemporalSatisfaction,
    /// Finite global protocol conformance under the selected closure premises.
    GlobalConformance,
    /// Whether the selected fragment can be monitored under the declared observations.
    Monitorability,
    /// Whether one participant projection is defined under its premises.
    LocalProjection,
    /// Whether one protocol refines another under a selected relation.
    Refinement,
    /// Whether the protocol admits a causal implementation strategy.
    Realizability,
    /// Whether selected components compose under independently discharged assumptions.
    Composition,
}

impl CapabilityKind {
    /// Every member, in the order FR-290 declares them.
    ///
    /// Declaration order carries no strength and no dispatch precedence; this
    /// is a census, not a ranking.
    pub const ALL: [Self; 10] = [
        Self::ValueValidity,
        Self::OperationContract,
        Self::FiniteReplay,
        Self::TemporalSatisfaction,
        Self::GlobalConformance,
        Self::Monitorability,
        Self::LocalProjection,
        Self::Refinement,
        Self::Realizability,
        Self::Composition,
    ];

    /// The exact label this kind serializes as.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ValueValidity => "value-validity",
            Self::OperationContract => "operation-contract",
            Self::FiniteReplay => "finite-replay",
            Self::TemporalSatisfaction => "temporal-satisfaction",
            Self::GlobalConformance => "global-conformance",
            Self::Monitorability => "monitorability",
            Self::LocalProjection => "local-projection",
            Self::Refinement => "refinement",
            Self::Realizability => "realizability",
            Self::Composition => "composition",
        }
    }

    /// The member whose label is byte-equal to `label`.
    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.label() == label)
    }

    /// This member's position in [`Self::ALL`].
    ///
    /// The match is exhaustive, so a new member cannot be added without giving
    /// it an index; TC-030 then reads every index back out of `ALL` and fails if
    /// the array was not extended to match. Without it `ALL` is a hand-kept list
    /// whose length happens to be right, and a member missing from it is a label
    /// `from_label` refuses while the vocabulary contains it.
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::ValueValidity => 0,
            Self::OperationContract => 1,
            Self::FiniteReplay => 2,
            Self::TemporalSatisfaction => 3,
            Self::GlobalConformance => 4,
            Self::Monitorability => 5,
            Self::LocalProjection => 6,
            Self::Refinement => 7,
            Self::Realizability => 8,
            Self::Composition => 9,
        }
    }
}

impl serde::Serialize for CapabilityKind {
    /// Serialises through [`Self::label`], so the emitted label and the label a
    /// request is read against cannot drift apart.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.label())
    }
}

/// The kind a request item carries, including the two ways it can fail to name one.
///
/// An absent kind and an unknown label settle differently and are reported with
/// different causes, so they are separate variants rather than one `Option`
/// that loses which happened.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestedKind {
    /// The item names a member of the vocabulary.
    Known(CapabilityKind),
    /// The item's kind is absent or `null`.
    Absent,
    /// The item names a label that is not byte-equal to any member.
    Unknown(String),
}

/// An advertised mode. A backend advertises a set of (kind, mode) pairs.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    /// The backend discharges the kind over a finite bound.
    Bounded,
    /// The backend discharges the kind without one.
    Unbounded,
}

/// A claim's extent, classified by the boundedness architecture.
///
/// `quire-spec-language#222` owns the representation, the classification and
/// the finite-bound predicate; this module reads all three and computes none.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtentClassification {
    /// The classification itself.
    pub extent: Mode,
    /// Whether a finite bound is available for an unbounded extent.
    ///
    /// It separates `requires-bound` from `unsupported` and is read from #222,
    /// never inferred from the extent.
    pub finite_bound_available: bool,
    /// Kinds of the item's unbounded domains, supplied by QSL.
    pub domains: Vec<ExtentDomain>,
    /// Substituted proof bounds, including each explicit kind and numeric bound.
    pub bounds: Vec<ProofBound>,
}

/// One unbounded domain and its explicit QSL kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtentDomain {
    /// Domain identity from the request.
    pub domain: DomainKey,
    /// Its declared domain kind.
    pub kind: DomainKind,
}

/// The registry-owned provider origin projected into CG's in-process descriptor.
///
/// QSL layer R owns this closed vocabulary (FR-288, ADR-029 PV-1). The driver
/// maps its registry value exhaustively; CG does not infer it from identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderOrigin {
    /// A provider linked into the host at compile time.
    Linked,
    /// A process provider admitted through the registry.
    Process,
}

/// A registered backend: its identity, advertised capabilities, and origin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendDescriptor {
    /// Backend identity, unique within a registry.
    pub identity: String,
    /// Advertised (kind, mode) pairs.
    pub advertised: Vec<(CapabilityKind, Mode)>,
    /// Origin supplied by the QSL registry, never derived from identity.
    pub origin: ProviderOrigin,
    /// Domain kinds admitted for finite proof bounds by the manifest.
    pub domains: Option<BTreeSet<DomainKind>>,
    /// Optional manifest run-limit defaults; absence differs from a present empty map.
    /// Negotiation retains but never compares these values.
    pub bounds: Option<BTreeMap<String, u64>>,
}

impl BackendDescriptor {
    /// The modes this backend advertises for `kind`, ascending.
    fn modes_for(&self, kind: CapabilityKind) -> BTreeSet<Mode> {
        self.advertised
            .iter()
            .filter(|(advertised, _)| *advertised == kind)
            .map(|(_, mode)| *mode)
            .collect()
    }

    fn advertises(&self, kind: CapabilityKind) -> bool {
        !self.modes_for(kind).is_empty()
    }
}

/// A candidate: the identity of a registered backend.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Candidate {
    /// Backend identity.
    pub identity: String,
}

/// An item's `candidates` value.
///
/// FR-331 admits exactly two shapes, and the unknown-backend mark carries the
/// identity the request named so the refusal can name it back.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Candidates {
    /// The candidate set the registry computed, ordered by identity.
    Set(Vec<Candidate>),
    /// The request named a backend that is not registered.
    UnknownBackend(String),
}

/// A typed identity for a Kani lowering form.
///
/// This is CG's in-process projection of the admitted IR form. It is not a
/// wire member and its identity is never reconstructed from `Debug` or
/// display output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KaniForm {
    /// A clause copied from a bound V1 package. Its private fields and validated
    /// expression come from Contract IR admission.
    V1BoundClause {
        /// The admitted typed clause.
        clause: Box<quire_contract_model::BoundClause>,
    },
    /// A checked claim and its exact admitted result occurrence.
    ///
    /// The operation identity, family, and provenance are read from the typed
    /// claim entry and checked graph; callers cannot assert them independently.
    V2ScalarClaim {
        /// The checked source package containing the source node.
        package: Box<quire_contract_model::CheckedPackageV2>,
        /// The admitted lowering of the full driver request.
        contract: Box<ContractPackageV1>,
        /// The full typed lowering positions, including context-only roots.
        positions: Vec<ContractRequestPosition>,
        /// This claim's index among driver positions.
        request_index: usize,
        /// The requested checked node.
        node_id: quire_contract_model::CheckedNodeId,
    },
    /// A V2 state-clause/frame role handled by the state-frame path.
    V2StateFrame {
        /// The admitted package containing the state clause.
        package: Box<quire_contract_model::CheckedPackageV2>,
        /// The checked state-clause node.
        clause: quire_contract_model::CheckedNodeId,
        /// The harness role requested from the checked state-clause node.
        role: KaniObligationRole,
        /// Rust path of the state ABI.
        state_path: String,
        /// State ABI fields in declaration order.
        state_fields: Vec<String>,
        /// Rust path of the operation subject.
        subject_path: String,
    },
    /// A checked V2 function declaration and its local checked call item.
    /// The argument list is a request descriptor and is accepted only when
    /// it is byte-for-byte the ordered reference list in the call node.
    V2FunctionApplication {
        /// The admitted package containing both nodes.
        package: Box<quire_contract_model::CheckedPackageV2>,
        /// The checked `function/pure_function` declaration being applied.
        function_node_id: quire_contract_model::CheckedNodeId,
        /// The checked local `expression/call` node.
        call_node_id: quire_contract_model::CheckedNodeId,
        /// Source argument nodes, in the caller's claimed order.
        argument_node_ids: Vec<quire_contract_model::CheckedNodeId>,
    },
    /// A typed form outside the normative V1/V2 Kani set.
    OutsideSet {
        /// The admitted package containing the typed node.
        package: Box<quire_contract_model::CheckedPackageV2>,
        /// The checked node outside Kani's normative form set.
        node_id: quire_contract_model::CheckedNodeId,
    },
}

/// Stable identity fields used when a Kani form is refused.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KaniFormIdentity {
    /// The finite family vocabulary from FR-019 AC-25.
    pub family: KaniFormFamily,
    /// The typed form within that family.
    pub form: KaniFormName,
    /// The exact operation identity, where the form has one.
    pub operation_identity: Option<String>,
    /// The obligation role, where the form has one.
    pub role: Option<KaniObligationRole>,
    /// The selected contract or frame harness role for a V2 state clause.
    pub state_frame_role: Option<KaniStateFrameRole>,
}

/// The selected harness role for a V2 state clause.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniStateFrameRole {
    /// The operation-contract harness.
    Contract,
    /// The frame-effect harness.
    Frame,
}

/// The complete V1 family vocabulary named by FR-019 AC-25.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniFormFamily {
    /// Boolean scalar claims.
    Boolean,
    /// Integer scalar claims.
    Integer,
    /// Rational scalar claims.
    Rational,
    /// Decimal scalar claims.
    Decimal,
    /// IEEE floating-point scalar claims.
    IeeeFloat,
    /// Text scalar claims.
    Text,
    /// Enumeration scalar claims.
    Enum,
    /// Quantity scalar claims.
    Quantity,
    /// Record or tuple composite equality.
    CompositeEquality,
    /// Function application.
    FunctionApplication,
    /// Precondition, postcondition, invariant or frame clauses.
    Clause,
    /// State before/after and frame obligations.
    StateFrame,
    /// Bounded collection forms.
    BoundedCollection,
    /// Bounded graph forms.
    BoundedGraph,
    /// A typed IR form outside the normative V1/V2 set.
    OutsideSet,
}

/// Form names used in stable Kani form identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniFormName {
    /// Admitted V1 bound clause.
    V1BoundClause,
    /// V2 exact scalar claim.
    V2ScalarClaim,
    /// V2 state-clause contract/frame role.
    V2StateFrame,
    /// V1 composite equality.
    V1CompositeEquality,
    /// V1 function application.
    V1FunctionApplication,
    /// V2 function application.
    V2FunctionApplication,
    /// Bounded collection operation.
    BoundedCollection,
    /// Bounded graph operation.
    BoundedGraph,
    /// An IR form outside the normative set.
    OutsideSet,
}

/// Obligation role carried by a typed Kani form.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniObligationRole {
    /// A precondition obligation.
    Precondition,
    /// A postcondition obligation.
    Postcondition,
    /// An invariant obligation.
    Invariant,
    /// A frame-effect obligation.
    Frame,
}

impl KaniFormName {
    const fn label(self) -> &'static str {
        match self {
            Self::V1BoundClause => "v1_bound_clause",
            Self::V2ScalarClaim => "v2_scalar_claim",
            Self::V2StateFrame => "v2_state_frame",
            Self::V1CompositeEquality => "v1_composite_equality",
            Self::V1FunctionApplication => "v1_function_application",
            Self::V2FunctionApplication => "v2_function_application",
            Self::BoundedCollection => "bounded_collection",
            Self::BoundedGraph => "bounded_graph",
            Self::OutsideSet => "outside_set",
        }
    }
}

fn obligation_role(role: &str) -> Option<KaniObligationRole> {
    match role {
        "precondition" => Some(KaniObligationRole::Precondition),
        "postcondition" => Some(KaniObligationRole::Postcondition),
        "invariant" => Some(KaniObligationRole::Invariant),
        _ => None,
    }
}

fn scalar_operation_family(
    package: &quire_contract_model::CheckedPackageV2,
    node_id: &quire_contract_model::CheckedNodeId,
) -> Option<KaniFormFamily> {
    use crate::oracle::scalar::{ExactScalarOperation as Op, OrderingOperandKind};
    let item =
        crate::oracle::scalar::derive_exact_scalar_items(package, std::slice::from_ref(node_id))
            .into_iter()
            .next()?
            .ok()?;
    Some(match item.operation {
        Op::IntegerArithmetic { .. } | Op::IntegerDivision { .. } | Op::IntegerModulo { .. } => {
            KaniFormFamily::Integer
        }
        Op::RationalArithmetic { .. } => KaniFormFamily::Rational,
        Op::Ordering { operands, .. } => match operands {
            OrderingOperandKind::Integer => KaniFormFamily::Integer,
            OrderingOperandKind::Rational => KaniFormFamily::Rational,
            OrderingOperandKind::Decimal => KaniFormFamily::Decimal,
        },
        Op::DecimalArithmetic { .. } => KaniFormFamily::Decimal,
        Op::IeeeArithmetic { .. } | Op::IeeeComparison { .. } | Op::IeeeWidthConversion { .. } => {
            KaniFormFamily::IeeeFloat
        }
        Op::TextAdmission { .. } | Op::TextComparison { .. } => KaniFormFamily::Text,
        Op::EnumComparison { .. } => KaniFormFamily::Enum,
        Op::QuantityConversion | Op::QuantityArithmetic { .. } | Op::QuantityComparison { .. } => {
            KaniFormFamily::Quantity
        }
    })
}

fn checked_node_family(
    package: &quire_contract_model::CheckedPackageV2,
    node_id: &quire_contract_model::CheckedNodeId,
) -> KaniFormFamily {
    if let Some(family) = scalar_operation_family(package, node_id) {
        return family;
    }
    let identity = package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == node_id)
        .and_then(|node| node.body.get("operation")?.get("identity")?.as_str());
    match identity {
        Some(
            "quire.op.boolean.and"
            | "quire.op.boolean.or"
            | "quire.op.boolean.not"
            | "quire.op.boolean.implies"
            | "quire.op.boolean.eq"
            | "quire.op.boolean.ne",
        ) => KaniFormFamily::Boolean,
        Some("quire.op.equality.equal" | "quire.op.equality.not_equal") => {
            KaniFormFamily::CompositeEquality
        }
        Some("quire.op.function.apply" | "quire.op.function.call") => {
            KaniFormFamily::FunctionApplication
        }
        Some("quire.op.collection.for_all_non_negative" | "quire.op.collection.exists_equal") => {
            KaniFormFamily::BoundedCollection
        }
        Some("quire.op.graph.reachability") => KaniFormFamily::BoundedGraph,
        _ => KaniFormFamily::OutsideSet,
    }
}

fn checked_function_call_matches(
    package: &quire_contract_model::CheckedPackageV2,
    function_node_id: &quire_contract_model::CheckedNodeId,
    call_node_id: &quire_contract_model::CheckedNodeId,
    argument_node_ids: &[quire_contract_model::CheckedNodeId],
) -> bool {
    let graph = &package.graph().nodes;
    let Some(function) = graph.iter().find(|node| &node.node_id == function_node_id) else {
        return false;
    };
    if &*function.node_tag != "function" || &*function.semantic_form != "pure_function" {
        return false;
    }
    let Some(function_members) = function
        .body
        .get("members")
        .and_then(serde_json::Value::as_array)
    else {
        return false;
    };
    let binding = |name: &str| {
        function_members
            .iter()
            .find(|member| {
                member.get("term").and_then(serde_json::Value::as_str) == Some("binding")
                    && member.get("name").and_then(serde_json::Value::as_str) == Some(name)
            })
            .and_then(|member| member.get("value"))
    };
    let Some(parameters) = binding("parameters")
        .and_then(|term| term.get("members"))
        .and_then(serde_json::Value::as_array)
    else {
        return false;
    };
    let valid_parameters = parameters.iter().all(|term| {
        resolve_checked_reference(graph, term)
            .is_some_and(|node| &*node.node_tag == "value" && &*node.semantic_form == "parameter")
    });
    let valid_body = binding("body")
        .and_then(|term| resolve_checked_reference(graph, term))
        .is_some_and(|node| &*node.node_tag == "expression");
    if !valid_parameters || !valid_body {
        return false;
    }
    let Some(call) = graph.iter().find(|node| &node.node_id == call_node_id) else {
        return false;
    };
    if &*call.node_tag != "expression"
        || &*call.semantic_form != "call"
        || call.body.get("term").and_then(serde_json::Value::as_str) != Some("application")
        || call
            .body
            .get("operator")
            .and_then(serde_json::Value::as_str)
            != Some("call")
        || call
            .body
            .pointer("/operation/identity")
            .and_then(serde_json::Value::as_str)
            != Some("quire.op.function.call")
    {
        return false;
    }
    let Some(arguments) = call
        .body
        .get("arguments")
        .and_then(serde_json::Value::as_array)
    else {
        return false;
    };
    if arguments.len() != argument_node_ids.len() + 1 || parameters.len() != argument_node_ids.len()
    {
        return false;
    }
    let expected = std::iter::once(function_node_id).chain(argument_node_ids.iter());
    arguments.iter().zip(expected).all(|(term, expected)| {
        let Some(target) = term
            .get("term")
            .filter(|term| term.as_str() == Some("reference"))
            .and_then(|_| term.get("target"))
        else {
            return false;
        };
        let Some(digest) = target.get("digest").and_then(serde_json::Value::as_str) else {
            return false;
        };
        let Some(domain) = target.get("domain").and_then(serde_json::Value::as_str) else {
            return false;
        };
        let actual = graph.iter().find(|node| {
            node.node_id.digest.as_ref() == digest && node.node_id.domain.as_ref() == domain
        });
        actual.is_some_and(|node| node.node_id == *expected)
    })
}

fn resolve_checked_reference<'a>(
    graph: &'a [quire_contract_model::CheckedSemanticNodeV2],
    term: &serde_json::Value,
) -> Option<&'a quire_contract_model::CheckedSemanticNodeV2> {
    if term.get("term").and_then(serde_json::Value::as_str) != Some("reference") {
        return None;
    }
    let target = term.get("target")?;
    let digest = target.get("digest")?.as_str()?;
    let domain = target.get("domain")?.as_str()?;
    graph.iter().find(|node| {
        node.node_id.digest.as_ref() == digest && node.node_id.domain.as_ref() == domain
    })
}

impl KaniForm {
    fn identity(&self) -> KaniFormIdentity {
        match self {
            Self::V1BoundClause { clause } => KaniFormIdentity {
                family: KaniFormFamily::Clause,
                form: KaniFormName::V1BoundClause,
                operation_identity: Some("v1.bound_clause".to_owned()),
                role: match clause.kind() {
                    ClauseKind::Precondition => Some(KaniObligationRole::Precondition),
                    ClauseKind::Postcondition => Some(KaniObligationRole::Postcondition),
                    ClauseKind::Invariant => Some(KaniObligationRole::Invariant),
                    ClauseKind::Assertion | ClauseKind::Case | ClauseKind::Information => None,
                },
                state_frame_role: None,
            },
            Self::V2ScalarClaim {
                package, node_id, ..
            } => {
                let operation_identity = package
                    .graph()
                    .nodes
                    .iter()
                    .find(|node| &node.node_id == node_id)
                    .and_then(|node| node.body.get("operation")?.get("identity")?.as_str())
                    .map(str::to_owned);
                let state_clause_role = package
                    .graph()
                    .nodes
                    .iter()
                    .find(|node| &node.node_id == node_id)
                    .filter(|node| {
                        &*node.node_tag == "state" && &*node.semantic_form == "state_clause"
                    })
                    .and_then(|node| {
                        node.body
                            .get("operation")?
                            .get("member")?
                            .get("clause")?
                            .as_str()
                    });
                let identity_family = checked_node_family(package, node_id);
                KaniFormIdentity {
                    family: if state_clause_role.is_some() {
                        KaniFormFamily::StateFrame
                    } else {
                        identity_family
                    },
                    form: KaniFormName::V2ScalarClaim,
                    operation_identity,
                    role: state_clause_role.and_then(obligation_role),
                    state_frame_role: None,
                }
            }
            Self::V2StateFrame {
                package,
                clause,
                role,
                ..
            } => KaniFormIdentity {
                family: KaniFormFamily::StateFrame,
                form: KaniFormName::V2StateFrame,
                operation_identity: package
                    .graph()
                    .nodes
                    .iter()
                    .find(|node| &node.node_id == clause)
                    .and_then(|node| node.body.get("operation")?.get("identity")?.as_str())
                    .map(str::to_owned),
                role: Some(*role),
                state_frame_role: Some(match role {
                    KaniObligationRole::Frame => KaniStateFrameRole::Frame,
                    KaniObligationRole::Precondition
                    | KaniObligationRole::Postcondition
                    | KaniObligationRole::Invariant => KaniStateFrameRole::Contract,
                }),
            },
            Self::V2FunctionApplication {
                package,
                function_node_id: _,
                call_node_id,
                ..
            } => KaniFormIdentity {
                family: KaniFormFamily::FunctionApplication,
                form: KaniFormName::V2FunctionApplication,
                operation_identity: package
                    .graph()
                    .nodes
                    .iter()
                    .find(|node| &node.node_id == call_node_id)
                    .and_then(|node| node.body.get("operation")?.get("identity")?.as_str())
                    .map(str::to_owned),
                role: None,
                state_frame_role: None,
            },
            Self::OutsideSet { package, node_id } => KaniFormIdentity {
                family: KaniFormFamily::OutsideSet,
                form: KaniFormName::OutsideSet,
                operation_identity: package
                    .graph()
                    .nodes
                    .iter()
                    .find(|node| &node.node_id == node_id)
                    .map(|node| node.semantic_form.to_string()),
                role: None,
                state_frame_role: None,
            },
        }
    }

    fn preflight(&self, backend: &BackendDescriptor) -> Result<KaniFormReadiness, Box<Cause>> {
        let identity = self.identity();
        match self {
            Self::V1BoundClause { clause } => {
                let refusal = match clause.kind() {
                    ClauseKind::Precondition | ClauseKind::Postcondition | ClauseKind::Invariant => {
                        crate::kani::generate::clause::classify_bound_clause(clause).err()
                    }
                    kind => Some(
                        crate::kani::generate::outcome::UnsupportedObligation::ClauseKindNotObligation {
                            kind,
                        },
                    ),
                };
                match refusal {
                    Some(refusal) => Err(Box::new(Cause::KaniFormRefused {
                        form: identity,
                        refusal,
                        backend: backend.identity.clone(),
                    })),
                    None => Ok(KaniFormReadiness::Ready),
                }
            }
            Self::V2ScalarClaim {
                package,
                contract,
                positions,
                request_index,
                node_id,
            } => {
                use crate::kani::generate::negotiate::ScalarFormAssessment;
                match crate::kani::generate::negotiate::classify_scalar_form(
                    package,
                    contract,
                    positions,
                    *request_index,
                    node_id,
                ) {
                    ScalarFormAssessment::Supported => Ok(KaniFormReadiness::Ready),
                    ScalarFormAssessment::RequiresBound => Ok(KaniFormReadiness::RequiresBound),
                    ScalarFormAssessment::Unsupported(refusal) => {
                        Err(Box::new(Cause::KaniFormRefused {
                            form: identity,
                            refusal,
                            backend: backend.identity.clone(),
                        }))
                    }
                    ScalarFormAssessment::Invalid(refusal) => {
                        Err(Box::new(Cause::KaniFormInvalid {
                            form: identity,
                            refusal,
                            backend: backend.identity.clone(),
                        }))
                    }
                }
            }
            Self::V2StateFrame {
                package,
                clause,
                role,
                state_path,
                state_fields,
                subject_path,
            } => {
                use crate::kani::generate::outcome::StateFrameRole;
                let actual_clause = package
                    .graph()
                    .nodes
                    .iter()
                    .find(|node| &node.node_id == clause)
                    .and_then(|node| {
                        node.body
                            .get("operation")?
                            .get("member")?
                            .get("clause")?
                            .as_str()
                    });
                let role = match role {
                    KaniObligationRole::Postcondition => StateFrameRole::Contract,
                    KaniObligationRole::Frame => StateFrameRole::Frame,
                    KaniObligationRole::Precondition | KaniObligationRole::Invariant => {
                        let expected = if *role == KaniObligationRole::Precondition {
                            "precondition"
                        } else {
                            "invariant"
                        };
                        if actual_clause != Some(expected) {
                            return Err(Box::new(Cause::KaniFormInvalid {
                                form: identity,
                                refusal: crate::kani::generate::outcome::InvalidObligationItem::UnknownNode,
                                backend: backend.identity.clone(),
                            }));
                        }
                        StateFrameRole::Contract
                    }
                };
                let state_fields = state_fields.iter().map(String::as_str).collect::<Vec<_>>();
                let request = crate::kani::generate::frame::StateFrameRequest {
                    ceilings: crate::kani::generate::frame::default_settlement_ceilings(),
                    package,
                    clause,
                    state_path,
                    state_fields: &state_fields,
                    subject_path,
                    unwind: 4,
                };
                match crate::kani::generate::frame::classify_state_frame_role(&request, role) {
                    Ok(()) => Ok(KaniFormReadiness::Ready),
                    Err(refusal) => Err(Box::new(Cause::KaniStateFrameRefused {
                        form: identity,
                        refusal,
                        backend: backend.identity.clone(),
                    })),
                }
            }
            Self::V2FunctionApplication {
                package,
                function_node_id,
                call_node_id,
                argument_node_ids,
            } => {
                let source_matches = checked_function_call_matches(
                    package,
                    function_node_id,
                    call_node_id,
                    argument_node_ids,
                );
                if !source_matches {
                    return Err(Box::new(Cause::KaniFormInvalid {
                        form: identity,
                        refusal: crate::kani::generate::outcome::InvalidObligationItem::UnknownNode,
                        backend: backend.identity.clone(),
                    }));
                }
                if crate::oracle::function::checked_integer_function_application_is_supported(
                    package,
                    function_node_id,
                    call_node_id,
                    argument_node_ids,
                ) {
                    Ok(KaniFormReadiness::Ready)
                } else {
                    Err(Box::new(Cause::KaniFormRefused {
                        form: identity,
                        refusal: crate::kani::generate::outcome::UnsupportedObligation::OperationNotRendered {
                            operation_identity: "quire.op.function.call".to_owned(),
                            derived_domains: Vec::new(),
                        },
                        backend: backend.identity.clone(),
                    }))
                }
            }
            Self::OutsideSet { package, node_id } => {
                let node = package
                    .graph()
                    .nodes
                    .iter()
                    .find(|node| &node.node_id == node_id);
                let Some(node) = node else {
                    return Err(Box::new(Cause::KaniFormInvalid {
                        form: identity,
                        refusal: crate::kani::generate::outcome::InvalidObligationItem::UnknownNode,
                        backend: backend.identity.clone(),
                    }));
                };
                if scalar_operation_family(package, node_id).is_some()
                    || &*node.semantic_form == "state_clause"
                {
                    Err(Box::new(Cause::KaniFormInvalid {
                        form: identity,
                        refusal: crate::kani::generate::outcome::InvalidObligationItem::UnknownNode,
                        backend: backend.identity.clone(),
                    }))
                } else {
                    Err(Box::new(Cause::UnsupportedKaniForm {
                        form: identity,
                        backend: backend.identity.clone(),
                    }))
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum KaniFormReadiness {
    Ready,
    RequiresBound,
}

/// One request item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestItem {
    /// The one FR-290 kind this item requires.
    pub kind: RequestedKind,
    /// The item's extent classification, absent when the request carries none.
    pub extent: Option<ExtentClassification>,
    /// The backend the caller names, when it names one.
    pub named_backend: Option<String>,
    /// The candidates the registry computed for this item.
    pub candidates: Candidates,
    /// The required typed IR form supplied to Kani settlement.
    pub kani_form: KaniForm,
}

/// The FR-331 backend provider envelope, as far as settlement reads it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendProviderEnvelope {
    /// Exactly [`BACKEND_PROVIDER_CONTRACT`].
    pub contract_version: String,
    /// Exactly [`CAPABILITY_VOCABULARY`].
    pub capability_vocabulary: String,
    /// One descriptor per backend identity in a valid FR-331 registry snapshot.
    /// A caller-supplied repeat violates the CG input precondition; FR-019-AC-45
    /// requires whole-envelope refusal even when both descriptors are identical.
    pub manifest: Vec<BackendDescriptor>,
    /// The request's items, in request order.
    pub items: Vec<RequestItem>,
}

/// A refusal of the whole carrier, before any kind is read.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "cause")]
pub enum EnvelopeRefusal {
    /// The contract version or the capability vocabulary is absent or another identity.
    ///
    /// FR-290 refuses the carrier rather than the item, and none of its labels
    /// are read: a vocabulary that is not the one selected does not merely
    /// rename members, it makes every label in the carrier unreadable.
    InvalidCapability {
        /// Always `unsupported-version`; the field keeps the cause printable
        /// beside the item-level causes rather than implied by the variant.
        cause: &'static str,
        /// Which member of the carrier was wrong.
        ///
        /// Both members refuse with one cause, so without this a consumer that
        /// got both wrong is told only about the first and cannot tell which
        /// field to fix from the refusal it was handed.
        member: &'static str,
        /// The identity the carrier supplied, or the empty string when absent.
        received: String,
    },
}

/// Why an item settled as something other than `supported`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
// FR-290 spells every item-level cause in kebab-case (`absent-kind`,
// `unknown-backend`, `inconsistent-candidates`) while the codes that type them
// are snake (`invalid_capability`, `unsupported_projection`). A consumer
// matching the spec's spelling never matched `absent_kind`.
#[serde(rename_all = "kebab-case", tag = "cause")]
pub enum Cause {
    /// The item's kind is absent or `null`.
    AbsentKind,
    /// The item's kind is a label outside the vocabulary, reported verbatim.
    UnknownKind {
        /// The exact received label.
        received: String,
    },
    /// The item carries no extent classification.
    AbsentExtent,
    /// The request named an unregistered backend, or one with no negotiation arm.
    UnknownBackend {
        /// The backend identity the request named.
        backend: String,
    },
    /// A candidate is absent from the manifest, does not advertise the kind, or
    /// disagrees with the named backend.
    InconsistentCandidates {
        /// Every offending candidate, in candidate order.
        candidates: Vec<Candidate>,
    },
    /// More than one candidate, and the request names no backend.
    AmbiguousBackend {
        /// Every candidate, in candidate order.
        candidates: Vec<Candidate>,
    },
    /// No registered backend advertises the item's kind.
    UnsupportedRequestedCapability {
        /// The kind no candidate advertises.
        kind: CapabilityKind,
        /// The backend the request named, when it named one.
        backend: Option<String>,
    },
    /// A process descriptor cannot discharge one explicit domain kind.
    #[serde(rename = "unsupported-requested-capability")]
    UnsupportedDomain {
        /// The capability kind requested.
        kind: CapabilityKind,
        /// The uncovered domain kind.
        #[serde(serialize_with = "serialize_domain_kind")]
        domain: DomainKind,
        /// The candidate that cannot discharge it.
        backend: String,
    },
    /// A Kani form is outside the renderer's current typed capability set.
    #[serde(rename = "unsupported-requested-capability")]
    UnsupportedKaniForm {
        /// The form's stable typed identity.
        form: KaniFormIdentity,
        /// The sole Kani candidate.
        backend: String,
    },
    /// An in-set Kani form was refused by its FR-015 typed validation/lowering path.
    #[serde(rename = "unsupported-requested-capability")]
    KaniFormRefused {
        /// The in-set form's stable typed identity.
        form: KaniFormIdentity,
        /// The exact typed FR-015 refusal.
        refusal: crate::kani::generate::outcome::UnsupportedObligation,
        /// The sole Kani candidate.
        backend: String,
    },
    /// The typed IR form is malformed or does not belong to its admitted package.
    #[serde(rename = "invalid-capability")]
    KaniFormInvalid {
        /// The form's stable typed identity.
        form: KaniFormIdentity,
        /// The FR-015 invalid-item reason.
        refusal: crate::kani::generate::outcome::InvalidObligationItem,
        /// The sole Kani candidate.
        backend: String,
    },
    /// A checked state-frame form was refused by its source-backed FR-015 classifier.
    #[serde(rename = "unsupported-requested-capability")]
    KaniStateFrameRefused {
        /// The form's stable typed identity.
        form: KaniFormIdentity,
        /// The exact typed state-frame refusal.
        refusal: crate::kani::generate::outcome::StateFrameRefusal,
        /// The sole Kani candidate.
        backend: String,
    },
    /// An unbounded extent against a bounded-only advertisement, with no finite
    /// bound available.
    UnboundedExtent {
        /// The kind whose extent could not be bounded.
        kind: CapabilityKind,
        /// The single candidate that advertises `bounded` only.
        backend: String,
    },
}

impl Cause {
    /// The FR-272 code this cause is typed under.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::AbsentKind
            | Self::UnknownKind { .. }
            | Self::AbsentExtent
            | Self::UnknownBackend { .. }
            | Self::InconsistentCandidates { .. }
            | Self::AmbiguousBackend { .. } => "invalid_capability",
            Self::UnsupportedRequestedCapability { .. }
            | Self::UnsupportedDomain { .. }
            | Self::UnsupportedKaniForm { .. }
            | Self::KaniFormRefused { .. }
            | Self::KaniStateFrameRefused { .. }
            | Self::UnboundedExtent { .. } => "unsupported_projection",
            Self::KaniFormInvalid { .. } => "invalid_capability",
        }
    }
}

/// How one item settled.
///
/// The four dispositions are FR-331's. `RequiresBound` is a negotiation
/// disposition and never a verification result; nothing in this module converts
/// one into the other.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "disposition")]
pub enum Disposition {
    /// The single candidate's arm discharges the claim as requested.
    Supported {
        /// The backend that settled it.
        backend: String,
    },
    /// The claim needs a finite bound before the single candidate can discharge it.
    RequiresBound {
        /// The backend that settled it.
        backend: String,
    },
    /// No candidate discharges the claim; the warning names what was requested.
    Unsupported {
        /// Why, typed under `unsupported_projection`.
        cause: Cause,
    },
    /// The request itself is wrong, and the cause names what is wrong with it.
    InvalidRequest {
        /// Why, typed under `invalid_capability`.
        cause: Cause,
    },
}

/// One item's settlement, in request order.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ItemSettlement {
    /// The item's index in the request.
    pub request_index: usize,
    /// The kind that settled, absent when the item named none this module reads.
    pub kind: Option<CapabilityKind>,
    /// The disposition.
    pub disposition: Disposition,
}

impl ItemSettlement {
    /// The warning an `unsupported` settlement carries, if any.
    ///
    /// `None` means this settlement has no warning to name: it is not
    /// `unsupported`, or its cause is not an `unsupported_projection` one. CG
    /// never builds an `unsupported` settlement with an `invalid_capability`
    /// cause, so FR-019's rule that an `unsupported` settlement carries a
    /// warning holds for every settlement CG builds; a caller that assembles
    /// one by hand gets `None`.
    ///
    /// FR-290 requires the warning to name the item's kind, and the named
    /// backend when the request names one. It is derived from the cause rather
    /// than stored beside it, so the two cannot disagree.
    #[must_use]
    pub fn warning(&self) -> Option<String> {
        match &self.disposition {
            Disposition::Unsupported { cause } => match cause {
                Cause::UnsupportedRequestedCapability { kind, backend } => Some(match backend {
                    Some(backend) => format!(
                        "no registered backend advertises {} for named backend {backend}",
                        kind.label()
                    ),
                    None => format!("no registered backend advertises {}", kind.label()),
                }),
                Cause::UnboundedExtent { kind, backend } => Some(format!(
                    "{backend} advertises {} bounded only and no finite bound is available",
                    kind.label()
                )),
                Cause::UnsupportedDomain {
                    kind,
                    domain,
                    backend,
                } => Some(format!(
                    "{backend} does not advertise domain {} for {}",
                    domain.to_wire(),
                    kind.label()
                )),
                Cause::UnsupportedKaniForm { form, backend } => Some(format!(
                    "{backend} does not support Kani form {}",
                    form.form.label()
                )),
                Cause::KaniFormRefused {
                    form,
                    refusal,
                    backend,
                } => Some(format!(
                    "{backend} refused Kani form {}: {refusal:?}",
                    form.form.label()
                )),
                Cause::KaniFormInvalid { .. } => None,
                Cause::KaniStateFrameRefused {
                    form,
                    refusal,
                    backend,
                } => Some(format!(
                    "{backend} refused Kani state-frame form {}: {refusal}",
                    form.form.label()
                )),
                // Exhaustive on purpose. A catch-all here would give the next
                // cause a warning that names neither the kind nor the backend
                // FR-290 requires it to name, with nothing failing to say so.
                // An `invalid_capability` cause settles invalid-request, so this
                // generator never builds it under `Unsupported`; the fields are
                // public, so a caller can, and this settlement has no warning.
                Cause::AbsentKind
                | Cause::UnknownKind { .. }
                | Cause::AbsentExtent
                | Cause::UnknownBackend { .. }
                | Cause::InconsistentCandidates { .. }
                | Cause::AmbiguousBackend { .. } => None,
            },
            Disposition::Supported { .. }
            | Disposition::RequiresBound { .. }
            | Disposition::InvalidRequest { .. } => None,
        }
    }
}

/// The closed set of backend kinds this generator settles for.
///
/// One arm per variant, no catch-all. Adding a variant without an arm is a
/// compile error at the dispatch, which is the whole point of the kind
/// being closed: an open set of hand-written negotiate functions gives the same
/// behaviour at run time and none of the enforcement.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackendKind {
    /// The Kani backend descriptor, implemented here and in `quire-contract-ir`.
    Kani,
    /// One process provider, identified by its exact registry identity.
    Process(BackendId),
}

/// A CG-owned backend identity, independent of QSL's Rust identity type.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct BackendId(pub String);

impl BackendKind {
    /// The finite built-in kinds; process identities come from descriptors.
    pub const ALL: [Self; 1] = [Self::Kani];

    /// Category sort key: built-in Kani precedes process identities.
    #[must_use]
    pub const fn index(&self) -> usize {
        match self {
            Self::Kani => 0,
            Self::Process(_) => 1,
        }
    }

    /// The backend identity this kind registers under.
    #[must_use]
    pub fn identity(&self) -> &str {
        match self {
            Self::Kani => "kani",
            Self::Process(id) => &id.0,
        }
    }

    /// The built-in kind a linked identity names, if any.
    ///
    /// A registered backend with no arm answers `None` and settles
    /// `invalid-request`/`unknown-backend`, which is FR-290's first candidate
    /// row: a backend nothing here can settle for is not silently treated as
    /// one that can.
    #[must_use]
    pub fn from_identity(identity: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.identity() == identity)
    }

    /// Classify a descriptor by its registry origin before reading its identity.
    #[must_use]
    pub fn from_descriptor(descriptor: &BackendDescriptor) -> Option<Self> {
        match descriptor.origin {
            ProviderOrigin::Linked => Self::from_identity(&descriptor.identity),
            ProviderOrigin::Process => Some(Self::Process(BackendId(descriptor.identity.clone()))),
        }
    }
}

/// Settle every item of an envelope.
///
/// Trace: TC-030
// Implements: FR-019
pub fn negotiate_backend_provider(
    envelope: &BackendProviderEnvelope,
) -> Result<Vec<ItemSettlement>, EnvelopeRefusal> {
    if envelope.contract_version != BACKEND_PROVIDER_CONTRACT {
        return Err(EnvelopeRefusal::InvalidCapability {
            cause: "unsupported-version",
            member: "contract_version",
            received: envelope.contract_version.clone(),
        });
    }
    if envelope.capability_vocabulary != CAPABILITY_VOCABULARY {
        return Err(EnvelopeRefusal::InvalidCapability {
            cause: "unsupported-version",
            member: "capability_vocabulary",
            received: envelope.capability_vocabulary.clone(),
        });
    }
    Ok(envelope
        .items
        .iter()
        .enumerate()
        .map(|(request_index, item)| ItemSettlement {
            request_index,
            kind: match item.kind {
                RequestedKind::Known(kind) => Some(kind),
                RequestedKind::Absent | RequestedKind::Unknown(_) => None,
            },
            disposition: negotiate_item(&envelope.manifest, item),
        })
        .collect())
}

/// The decision for one exact FR-330 request position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractItemSettlement {
    /// Position among the original driver items; context roots do not count.
    pub request_index: usize,
    /// Exact requested key, including repeated positions.
    pub node_id: CheckedNodeId,
    /// The exact admitted accounting record and source occurrences.
    pub admitted: AdmittedItemAccounting,
    /// The backend settlement after `Lowered`; absent for an IR-refused item.
    pub backend: Option<ItemSettlement>,
}

/// A whole-request mismatch before any item can be settled.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractNegotiationError {
    /// The FR-331 envelope is invalid.
    Envelope(EnvelopeRefusal),
    /// Request positions and exact keys disagree with the admitted package.
    Package(PackageRequestError),
    /// The FR-331 envelope and the original request have different position counts.
    EnvelopeItemCount {
        /// Number of original request positions.
        requested: usize,
        /// Number of FR-331 item records supplied.
        supplied: usize,
    },
}

/// Settle every request position from one admitted FR-330 package.
///
/// FR-019 still owns backend negotiation for a lowered item. A non-lowered
/// item retains its exact FR-330 record and cannot become backend-supported.
pub fn negotiate_contract_package(
    package: &ContractPackageV1,
    positions: &[ContractRequestPosition],
    envelope: &BackendProviderEnvelope,
) -> Result<Vec<ContractItemSettlement>, ContractNegotiationError> {
    let items = PackageItems::new(package, positions).map_err(ContractNegotiationError::Package)?;
    let backend =
        negotiate_backend_provider(envelope).map_err(ContractNegotiationError::Envelope)?;
    if backend.len() != items.driver_count() {
        return Err(ContractNegotiationError::EnvelopeItemCount {
            requested: items.driver_count(),
            supplied: backend.len(),
        });
    }
    backend
        .into_iter()
        .enumerate()
        .map(|(request_index, settlement)| {
            let node_id =
                items
                    .driver_node(request_index)
                    .ok_or(ContractNegotiationError::Package(
                        PackageRequestError::Position { request_index },
                    ))?;
            let item = items
                .at(request_index, node_id)
                .map_err(ContractNegotiationError::Package)?;
            let admitted = item.accounting();
            let backend = match item {
                PackageItem::Lowered { .. } => Some(settlement),
                PackageItem::Refused { .. } => None,
            };
            Ok(ContractItemSettlement {
                request_index,
                node_id: node_id.clone(),
                admitted,
                backend,
            })
        })
        .collect()
}

/// Settle one item, evaluating FR-290's rules in order; the first match settles.
fn negotiate_item(manifest: &[BackendDescriptor], item: &RequestItem) -> Disposition {
    let kind = match &item.kind {
        RequestedKind::Absent => {
            return Disposition::InvalidRequest {
                cause: Cause::AbsentKind,
            }
        }
        RequestedKind::Unknown(received) => {
            return Disposition::InvalidRequest {
                cause: Cause::UnknownKind {
                    received: received.clone(),
                },
            }
        }
        RequestedKind::Known(kind) => *kind,
    };
    let Some(extent) = item.extent.as_ref() else {
        return Disposition::InvalidRequest {
            cause: Cause::AbsentExtent,
        };
    };
    let candidates = match &item.candidates {
        Candidates::UnknownBackend(backend) => {
            return Disposition::InvalidRequest {
                cause: Cause::UnknownBackend {
                    backend: backend.clone(),
                },
            }
        }
        Candidates::Set(candidates) => candidates.as_slice(),
    };
    if let Some(backend) = unroutable_named_backend(manifest, item.named_backend.as_deref()) {
        return Disposition::InvalidRequest {
            cause: Cause::UnknownBackend { backend },
        };
    }
    let offending = inconsistent_candidates(manifest, item, kind, candidates);
    if !offending.is_empty() {
        return Disposition::InvalidRequest {
            cause: Cause::InconsistentCandidates {
                candidates: offending,
            },
        };
    }
    match candidates {
        [] => Disposition::Unsupported {
            cause: Cause::UnsupportedRequestedCapability {
                kind,
                backend: item.named_backend.clone(),
            },
        },
        [single] => negotiate_single_candidate(manifest, single, kind, extent, &item.kani_form),
        many if item.named_backend.is_none() => Disposition::InvalidRequest {
            cause: Cause::AmbiguousBackend {
                candidates: many.to_vec(),
            },
        },
        // FR-290's candidate table has no row for "more than one candidate and
        // the request names one": reaching it needs a manifest that repeats a
        // backend identity, which the registry refuses as `duplicate-backend`
        // before a request is built. A set that disagrees with the named backend
        // is the nearest stated row, and is what such a set is.
        many => Disposition::InvalidRequest {
            cause: Cause::InconsistentCandidates {
                candidates: many.to_vec(),
            },
        },
    }
}

/// The named backend, when it names no negotiation arm.
///
/// FR-290's first candidate row pairs "a registered backend with no negotiation
/// arm" with the unknown-backend mark, and that mark carries the identity the
/// request *named*. Both halves of the row are therefore read as being about the
/// named backend, not about any member of the set: the alternative reading makes
/// the row swallow the ambiguous-backend row below it, because an unarmed
/// backend anywhere in a multi-candidate set would settle before the ambiguity
/// the caller actually has to resolve. A candidate that reaches routing with no
/// arm is still refused with this cause, at the single-candidate arm.
fn unroutable_named_backend(manifest: &[BackendDescriptor], named: Option<&str>) -> Option<String> {
    named
        .filter(|named| {
            manifest
                .iter()
                .find(|backend| backend.identity == *named)
                .and_then(BackendKind::from_descriptor)
                .is_none()
        })
        .map(str::to_owned)
}

/// Every candidate absent from the manifest, not advertising the kind, or
/// disagreeing with the named backend, in candidate order.
fn inconsistent_candidates(
    manifest: &[BackendDescriptor],
    item: &RequestItem,
    kind: CapabilityKind,
    candidates: &[Candidate],
) -> Vec<Candidate> {
    candidates
        .iter()
        .filter(|candidate| {
            let registered = descriptor(manifest, candidate);
            let disagrees = item
                .named_backend
                .as_ref()
                .is_some_and(|named| *named != candidate.identity);
            disagrees || registered.is_none_or(|backend| !backend.advertises(kind))
        })
        .cloned()
        .collect()
}

fn descriptor<'a>(
    manifest: &'a [BackendDescriptor],
    candidate: &Candidate,
) -> Option<&'a BackendDescriptor> {
    manifest
        .iter()
        .find(|backend| backend.identity == candidate.identity)
}

/// Route the single candidate to its arm.
///
/// Every function that constructs a [`Disposition`] is named `negotiate_*`.
fn negotiate_single_candidate(
    manifest: &[BackendDescriptor],
    candidate: &Candidate,
    kind: CapabilityKind,
    extent: &ExtentClassification,
    kani_form: &KaniForm,
) -> Disposition {
    let Some(backend) = descriptor(manifest, candidate) else {
        return Disposition::InvalidRequest {
            cause: Cause::InconsistentCandidates {
                candidates: vec![candidate.clone()],
            },
        };
    };
    let Some(routed) = BackendKind::from_descriptor(backend) else {
        return Disposition::InvalidRequest {
            cause: Cause::UnknownBackend {
                backend: backend.identity.clone(),
            },
        };
    };
    negotiate_arm(routed, backend, kind, extent, kani_form)
}

/// Dispatch one arm per backend kind.
///
/// Trace: TC-030
// Implements: FR-019
fn negotiate_arm(
    routed: BackendKind,
    backend: &BackendDescriptor,
    kind: CapabilityKind,
    extent: &ExtentClassification,
    kani_form: &KaniForm,
) -> Disposition {
    match routed {
        BackendKind::Kani => negotiate_kani(backend, kind, extent, kani_form),
        BackendKind::Process(_) => negotiate_process(backend, kind, extent),
    }
}

/// The Kani arm.
///
/// Kani discharges what its manifest advertises. It compares the claim's
/// extent against the advertised modes and never narrows the extent to reach a
/// disposition.
fn negotiate_kani(
    backend: &BackendDescriptor,
    kind: CapabilityKind,
    extent: &ExtentClassification,
    kani_form: &KaniForm,
) -> Disposition {
    let readiness = match kani_form.preflight(backend) {
        Ok(readiness) => readiness,
        Err(cause) => match *cause {
            cause @ Cause::KaniFormInvalid { .. } => return Disposition::InvalidRequest { cause },
            cause => return Disposition::Unsupported { cause },
        },
    };
    if matches!(readiness, KaniFormReadiness::RequiresBound) {
        return Disposition::RequiresBound {
            backend: backend.identity.clone(),
        };
    }
    let modes = backend.modes_for(kind);
    match extent.extent {
        Mode::Bounded if modes.contains(&Mode::Bounded) => Disposition::Supported {
            backend: backend.identity.clone(),
        },
        Mode::Bounded => Disposition::Unsupported {
            cause: Cause::UnsupportedRequestedCapability {
                kind,
                backend: Some(backend.identity.clone()),
            },
        },
        Mode::Unbounded if modes.contains(&Mode::Unbounded) => Disposition::Supported {
            backend: backend.identity.clone(),
        },
        Mode::Unbounded if extent.finite_bound_available => Disposition::RequiresBound {
            backend: backend.identity.clone(),
        },
        Mode::Unbounded => Disposition::Unsupported {
            cause: Cause::UnboundedExtent {
                kind,
                backend: backend.identity.clone(),
            },
        },
    }
}

/// Settle a process provider solely from its manifest and the item's extent.
fn negotiate_process(
    backend: &BackendDescriptor,
    kind: CapabilityKind,
    extent: &ExtentClassification,
) -> Disposition {
    let modes = backend.modes_for(kind);
    let advertised_domains = backend.domains.as_ref();
    let unsupported_domain = match extent.extent {
        Mode::Bounded => extent
            .bounds
            .iter()
            .filter_map(ProofBound::kind)
            .find(|domain| {
                !advertised_domains.is_some_and(|advertised| advertised.contains(domain))
            }),
        Mode::Unbounded if modes.contains(&Mode::Unbounded) => None,
        Mode::Unbounded => extent
            .domains
            .iter()
            .map(|domain| domain.kind)
            .find(|domain| {
                domain.finite_kind().is_some()
                    && !advertised_domains.is_some_and(|advertised| advertised.contains(domain))
            }),
    };
    if let Some(domain) = unsupported_domain {
        return Disposition::Unsupported {
            cause: Cause::UnsupportedDomain {
                kind,
                domain,
                backend: backend.identity.clone(),
            },
        };
    }
    match extent.extent {
        Mode::Bounded if modes.contains(&Mode::Bounded) => Disposition::Supported {
            backend: backend.identity.clone(),
        },
        Mode::Bounded => Disposition::Unsupported {
            cause: Cause::UnsupportedRequestedCapability {
                kind,
                backend: Some(backend.identity.clone()),
            },
        },
        Mode::Unbounded if modes.contains(&Mode::Unbounded) => Disposition::Supported {
            backend: backend.identity.clone(),
        },
        Mode::Unbounded if extent.finite_bound_available => Disposition::RequiresBound {
            backend: backend.identity.clone(),
        },
        Mode::Unbounded => Disposition::Unsupported {
            cause: Cause::UnboundedExtent {
                kind,
                backend: backend.identity.clone(),
            },
        },
    }
}

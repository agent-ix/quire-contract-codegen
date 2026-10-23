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
//! the only case of [`OperationProvenance::CallerDeclared`] this generator refuses under
//! that name: `CallerDeclaredOperation` is constructed once, inside the `Generated` arm of
//! `classify_claim`, so a claim codegen refused outright is classified through the `Refused` arm
//! instead and never reaches it. V1 has no
//! frame clause kind, and V2 frames have no finite encoding in the scalar profile, so the clause
//! and scalar arms emit no frame harness. A [`ObligationItem::StateFrame`] item is the arm that
//! does: one role of one `postcondition` `state_clause`, whose harness comes from
//! `generate_state_frame_role` and whose refusal becomes its record by
//! `state_frame_disposition`, each item settling on its own. A V2 scalar claim's graph node that is present but is neither the one
//! recognized `state`/`frame` pair nor `expression`-tagged (the only family this generator ever
//! lowers to a `Generated` claim) is refused as [`UnsupportedObligation::UnknownNodeKind`] rather
//! than accounted with a null contract role and no typed reason; a claim naming a `node_id`
//! absent from the graph entirely is refused the same way as [`InvalidObligationItem::UnknownNode`]
//! (see `refuse_unknown_node_kind`).
//!
//! A render whose generated source would exceed [`MAX_GENERATED_SOURCE_BYTES`] is refused
//! as [`UnsupportedObligation::ResourceLimitExceeded`], and one that fits that ceiling but fails
//! `syn::parse_file` is refused as [`UnsupportedObligation::InvalidGeneratedSyntax`]; the two
//! grounds are checked in that order and are never conflated with the internal-invariant
//! [`UnsupportedObligation::RenderFailed`] fallback.

use std::collections::{BTreeMap, BTreeSet};

use quire_contract_model::{
    BoundClause, BoundPackage, CheckedNodeId, CheckedNodeTag, CheckedPackageV2, CheckedSemanticId,
    CheckedSemanticNodeV2, CheckedSourceMapEntry, ClauseKind, ClauseRef, ContractPackageV1,
    ExecutionPoint,
};

use crate::{
    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},
    core::contract_package::ContractDriverResultSelection,
    core::diagnostic::GenerationErrorCode,
    core::identity::HarnessSymbol,
    core::naming::unique_names,
    kani::abi::{adapter_options, KaniSolver},
    kani::generate::clause::v2::{
        lower as lower_v2_clause, render_clause as render_v2_clause, LoweredV2Clause,
    },
    kani::generate::clause::{
        abi, clause_stem, contract_contexts, lower_clause, obligation_kind, symbols, LoweredClause,
        SlotContext,
    },
    kani::generate::collection::{
        collection_symbols_conflict, lower_collection_claim, render_collection,
        CollectionDomainRefusal, CollectionGraph, LoweredCollectionClaim,
    },
    kani::generate::contract::render_contract,
    kani::generate::frame::{generate_state_frame_role, StateFrameRequest},
    kani::generate::outcome::{
        state_frame_disposition, CollectionUnsupportedReason, InvalidObligationItem,
        KaniObligationError, KaniObligationOutcome, KaniObligationRequest, ObligationDisposition,
        ObligationItem, ObligationRecord, ObligationSubject, StateFrameRefusal, StateFrameRole,
        UnsupportedObligation, MAX_OBLIGATION_ITEMS, MAX_OBLIGATION_UNWIND,
    },
    kani::generate::precondition::render_precondition,
    kani::generate::record::{harness_path, record},
    kani::generate::scalar::{
        derive_domain, lower_scalar_claim, render_scalar, scalar_stem, unsatisfiable,
        LoweredScalarClaim, ScalarLoweringRefusal,
    },
    kani::identity::{
        EmbeddedOracle, KaniObligationHarness, KaniObligationIdentity, ObligationKind,
        StateFrameHarness,
    },
    oracle::boolean_v1::{generate_named_boolean_oracle, OracleRequest, OracleShape},
    oracle::claim::{ClaimDisposition, ClaimMap, UpstreamBlocker},
    oracle::scalar::{operand_ranges, ExactScalarClaim, ExactScalarRefusal, OperationProvenance},
};

/// One admitted scalar package's node and source lookups, shared by its request items.
/// A node's first graph entry and all source entries retain their original wire order.
struct ContractLookup<'a> {
    graph: crate::oracle::scalar::Graph<'a>,
    sources: BTreeMap<&'a CheckedNodeId, Vec<&'a CheckedSourceMapEntry>>,
}

impl<'a> ContractLookup<'a> {
    fn new(package: &'a ContractPackageV1) -> Self {
        let mut graph = BTreeMap::new();
        for node in &package.wire().semantic_graph.nodes {
            graph.entry(&node.node_id).or_insert(node);
        }
        let mut sources = BTreeMap::<_, Vec<_>>::new();
        for entry in &package.wire().source_map.nodes {
            sources
                .entry(&entry.node_id)
                .or_default()
                .extend(entry.occurrences.iter());
        }
        Self { graph, sources }
    }

    fn node(&self, id: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2> {
        self.graph.get(id).copied()
    }

    fn occurrences(&self, id: &CheckedNodeId) -> Vec<CheckedSourceMapEntry> {
        self.sources.get(id).map_or_else(Vec::new, |entries| {
            entries.iter().map(|entry| (*entry).clone()).collect()
        })
    }
}

/// Reuse admitted scalar graph lookups across capability items, whose boxed packages may be
/// distinct allocations of the same admitted identity.
pub(crate) struct ScalarFormLookups<'a> {
    checked: BTreeMap<CheckedSemanticId, crate::oracle::scalar::Graph<'a>>,
    contract: BTreeMap<CheckedSemanticId, ContractLookup<'a>>,
}

impl<'a> ScalarFormLookups<'a> {
    pub(crate) fn new() -> Self {
        Self {
            checked: BTreeMap::new(),
            contract: BTreeMap::new(),
        }
    }

    fn checked_node(
        &mut self,
        package: &'a CheckedPackageV2,
        node_id: &CheckedNodeId,
    ) -> Option<&CheckedSemanticNodeV2> {
        self.checked
            .entry(package.package_id().clone())
            .or_insert_with(|| {
                let mut graph = BTreeMap::new();
                for node in &package.graph().nodes {
                    graph.entry(&node.node_id).or_insert(node);
                }
                graph
            })
            .get(node_id)
            .copied()
    }

    fn contract(&mut self, package: &'a ContractPackageV1) -> &ContractLookup<'a> {
        self.contract
            .entry(package.identity().clone())
            .or_insert_with(|| ContractLookup::new(package))
    }
}

/// Negotiates every item and, when no item is invalid, emits one harness per supported item.
// Implements: FR-015
pub fn negotiate_kani_obligations(
    request: &KaniObligationRequest<'_>,
) -> Result<KaniObligationOutcome, KaniObligationError> {
    validate_request(request)?;
    let mut scalar_lookups = BTreeMap::new();
    let mut states = request
        .items
        .iter()
        .map(|item| classify(item, request.unwind, request.ceilings, &mut scalar_lookups))
        .collect::<Vec<_>>();
    reject_duplicates_and_mixtures(request.items, &mut states);
    assign_names(request.items, &mut states);
    resolve_assumptions(&mut states);
    let rejected = states.iter().any(|state| state.outcome.is_invalid());
    let mut records = Vec::with_capacity(states.len());
    let mut harnesses = Vec::new();
    let mut scalar_harnesses = Vec::new();
    let mut state_frame_harnesses = Vec::new();
    let mut collection_harnesses: Vec<crate::kani::identity::KaniCollectionObligationHarness> =
        Vec::new();
    let mut v2_clause_harnesses = Vec::new();
    for (index, state) in states.into_iter().enumerate() {
        let disposition = if rejected {
            state.outcome.disposition_without_harness(request)
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
                Outcome::LoweredV2Clause(lowered) => match render_v2_clause(request, &lowered) {
                    Ok(harness) => {
                        let symbol = harness.identity.harness_symbol.clone();
                        v2_clause_harnesses.push(harness);
                        ObligationDisposition::Supported {
                            harness_symbol: symbol,
                        }
                    }
                    Err(reason) => ObligationDisposition::Unsupported { reason },
                },
                Outcome::LoweredCollection(lowered) => match render_collection(request, &lowered) {
                    Ok(harness) => {
                        let collides = collection_harnesses.iter().any(|existing| {
                            collection_symbols_conflict(
                                existing.identity.module_symbol.as_str(),
                                existing.identity.harness_symbol.as_str(),
                                &existing.identity,
                                harness.identity.module_symbol.as_str(),
                                harness.identity.harness_symbol.as_str(),
                                &harness.identity,
                            )
                        });
                        if collides {
                            ObligationDisposition::Unsupported {
                                reason: UnsupportedObligation::CollectionClaimNotRendered {
                                    node_id: harness.identity.quantifier_id.clone(),
                                    reason: CollectionUnsupportedReason::HarnessSymbolCollision,
                                },
                            }
                        } else {
                            let symbol = harness.identity.harness_symbol.clone();
                            collection_harnesses.push(harness);
                            ObligationDisposition::Supported {
                                harness_symbol: symbol,
                            }
                        }
                    }
                    Err(reason) => ObligationDisposition::Unsupported { reason },
                },
                Outcome::StateFrame(harness) => {
                    let symbol = harness.identity.harness_symbol.clone();
                    state_frame_harnesses.push(*harness);
                    ObligationDisposition::Supported {
                        harness_symbol: symbol,
                    }
                }
                other => other.disposition_without_harness(request),
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
            state_frame_harnesses,
            collection_harnesses,
            v2_clause_harnesses,
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
        result_selection: ContractDriverResultSelection,
    },
    /// One role of one state clause. `DuplicateItem` spans this arm and one role only.
    StateFrame {
        package: String,
        clause: CheckedNodeId,
        role: StateFrameRole,
    },
    Collection {
        package: String,
        function: CheckedNodeId,
        quantifier: CheckedNodeId,
    },
    V2Clause {
        package: String,
        clause: CheckedNodeId,
    },
}

enum Outcome<'a> {
    Lowered(Box<LoweredClause<'a>>),
    LoweredScalar(Box<LoweredScalarClaim>),
    LoweredCollection(Box<LoweredCollectionClaim>),
    LoweredV2Clause(Box<LoweredV2Clause>),
    /// A `StateFrame` item that yielded its harness.
    StateFrame(Box<StateFrameHarness>),
    /// A `StateFrame` item the engine refused. The refusal is held, not its record, so the item
    /// cannot be `supported`; `state_frame_disposition` maps it where the record is built.
    StateFrameRefused(StateFrameRefusal),
    RequiresBound(CheckedNodeId),
    Unsupported(UnsupportedObligation),
    Invalid(InvalidObligationItem),
}

impl Outcome<'_> {
    /// Whether this item rejects the whole request.
    fn is_invalid(&self) -> bool {
        match self {
            Self::Invalid(_) => true,
            Self::StateFrameRefused(refusal) => matches!(
                state_frame_disposition(refusal.clone()),
                ObligationDisposition::InvalidRequest { .. }
            ),
            Self::Lowered(_)
            | Self::LoweredScalar(_)
            | Self::LoweredCollection(_)
            | Self::LoweredV2Clause(_)
            | Self::StateFrame(_)
            | Self::RequiresBound(_)
            | Self::Unsupported(_) => false,
        }
    }

    fn disposition_without_harness(
        self,
        request: &KaniObligationRequest<'_>,
    ) -> ObligationDisposition {
        match self {
            // A lowered item in a rejected request is accounted but not emitted.
            Self::Lowered(lowered) => supported_without_harness(&lowered.symbols.harness),
            Self::LoweredScalar(lowered) => supported_without_harness(&lowered.harness_symbol),
            Self::LoweredCollection(lowered) => match render_collection(request, &lowered) {
                Ok(harness) => supported_without_harness(harness.identity.harness_symbol.as_str()),
                Err(reason) => ObligationDisposition::Unsupported { reason },
            },
            Self::LoweredV2Clause(lowered) => match render_v2_clause(request, &lowered) {
                Ok(harness) => supported_without_harness(harness.identity.harness_symbol.as_str()),
                Err(reason) => ObligationDisposition::Unsupported { reason },
            },
            Self::StateFrame(harness) => ObligationDisposition::Supported {
                harness_symbol: harness.identity.harness_symbol,
            },
            Self::StateFrameRefused(refusal) => state_frame_disposition(refusal),
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

/// Classifies one request item. Its generated names are its readable stems until
/// [`assign_names`] settles them across the request.
fn classify<'a>(
    item: &ObligationItem<'a>,
    unwind: u32,
    ceilings: crate::kani::identity::ProofCeilings,
    scalar_lookups: &mut BTreeMap<*const ContractPackageV1, ContractLookup<'a>>,
) -> ItemState<'a> {
    match *item {
        ObligationItem::V2Clause { package, clause } => classify_v2_clause(package, clause),
        ObligationItem::BoundClause { package, clause } => classify_clause(package, clause),
        ObligationItem::ScalarClaim {
            package,
            claim_map,
            node_id,
            claim_index,
            result_selection,
        } => {
            let lookup = scalar_lookups
                .entry(std::ptr::from_ref(package))
                .or_insert_with(|| ContractLookup::new(package));
            classify_node(
                package,
                lookup,
                claim_map,
                node_id,
                claim_index,
                result_selection,
            )
        }
        ObligationItem::CollectionClaim {
            package,
            function_id,
            node_id,
        } => classify_collection(package, function_id, node_id),
        ObligationItem::StateFrame {
            package,
            clause,
            role,
            state_path,
            state_fields,
            subject_path,
        } => classify_state_frame(
            &StateFrameRequest {
                ceilings,
                package,
                clause,
                state_path,
                state_fields,
                subject_path,
                unwind,
            },
            role,
        ),
    }
}

fn classify_v2_clause<'a>(
    package: &'a CheckedPackageV2,
    clause: &'a CheckedNodeId,
) -> ItemState<'a> {
    let graph_node = package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == clause);
    let kind = graph_node
        .and_then(|node| {
            node.body
                .get("operation")?
                .get("member")?
                .get("clause")?
                .as_str()
        })
        .and_then(|name| match name {
            "precondition" => Some(ObligationKind::Precondition),
            "postcondition" => Some(ObligationKind::Postcondition),
            "invariant" => Some(ObligationKind::Invariant),
            _ => None,
        });
    let outcome = match graph_node {
        None => Outcome::Invalid(InvalidObligationItem::UnknownNode),
        Some(node) if &*node.node_tag != "state" || &*node.semantic_form != "state_clause" => {
            Outcome::Unsupported(UnsupportedObligation::UnknownNodeKind {
                node_id: clause.clone(),
                node_tag: node.node_tag.to_string(),
                semantic_form: node.semantic_form.to_string(),
            })
        }
        Some(_) if kind != Some(ObligationKind::Precondition) => {
            Outcome::Unsupported(UnsupportedObligation::V2ClauseConditionNotRendered {
                node_id: clause.clone(),
            })
        }
        Some(_) => match lower_v2_clause(package, clause) {
            Ok(lowered) => Outcome::LoweredV2Clause(Box::new(lowered)),
            Err(reason) => Outcome::Unsupported(reason),
        },
    };
    ItemState {
        kind,
        subject: ObligationSubject::CheckedNode {
            node_id: clause.clone(),
            source_map: package
                .source_map()
                .iter()
                .filter(|entry| &entry.node_id == clause)
                .cloned()
                .collect(),
        },
        identity: Some(ItemIdentity::V2Clause {
            package: package.package_id().digest.to_string(),
            clause: clause.clone(),
        }),
        outcome,
    }
}

fn classify_collection<'a>(
    package: &'a CheckedPackageV2,
    function_id: &'a CheckedNodeId,
    node_id: &'a CheckedNodeId,
) -> ItemState<'a> {
    let graph = CollectionGraph::new(package);
    let outcome = match lower_collection_claim(&graph, function_id, node_id) {
        Ok(lowered) => Outcome::LoweredCollection(Box::new(lowered)),
        Err(CollectionDomainRefusal::RequiresBound { unbounded_type }) => {
            Outcome::RequiresBound(unbounded_type)
        }
        Err(CollectionDomainRefusal::UnsupportedShape) => {
            collection_refusal(node_id, CollectionUnsupportedReason::UnsupportedClaimShape)
        }
        Err(CollectionDomainRefusal::FieldNotAdmitted) => {
            collection_refusal(node_id, CollectionUnsupportedReason::FieldNotAdmitted)
        }
        Err(CollectionDomainRefusal::UnsupportedFieldType) => {
            collection_refusal(node_id, CollectionUnsupportedReason::UnsupportedFieldType)
        }
        Err(CollectionDomainRefusal::UnsatisfiableElementDomain) => collection_refusal(
            node_id,
            CollectionUnsupportedReason::UnsatisfiableElementDomain,
        ),
    };
    ItemState {
        kind: None,
        subject: ObligationSubject::CheckedNode {
            node_id: node_id.clone(),
            source_map: package
                .source_map()
                .iter()
                .filter(|entry| &entry.node_id == node_id)
                .cloned()
                .collect(),
        },
        identity: Some(ItemIdentity::Collection {
            package: package.package_id().digest.to_string(),
            function: function_id.clone(),
            quantifier: node_id.clone(),
        }),
        outcome,
    }
}

fn collection_refusal(node_id: &CheckedNodeId, reason: CollectionUnsupportedReason) -> Outcome<'_> {
    Outcome::Unsupported(UnsupportedObligation::CollectionClaimNotRendered {
        node_id: node_id.clone(),
        reason,
    })
}

/// One `StateFrame` item: its harness, or the record its engine refusal maps to. The item's own
/// subject path is what the harness calls.
fn classify_state_frame<'a>(
    request: &StateFrameRequest<'a>,
    role: StateFrameRole,
) -> ItemState<'a> {
    let outcome = match generate_state_frame_role(request, role) {
        Ok(harness) => Outcome::StateFrame(Box::new(harness)),
        Err(refusal) => Outcome::StateFrameRefused(refusal),
    };
    ItemState {
        kind: Some(role.obligation_kind()),
        subject: ObligationSubject::CheckedNode {
            node_id: request.clause.clone(),
            source_map: request
                .package
                .source_map()
                .iter()
                .filter(|entry| &entry.node_id == request.clause)
                .cloned()
                .collect(),
        },
        identity: Some(ItemIdentity::StateFrame {
            package: request.package.package_id().digest.to_string(),
            clause: request.clause.clone(),
            role,
        }),
        outcome,
    }
}

/// The full identity a generated name is ordered by when several items share its readable stem.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum NameKey {
    Clause(String, u64, String),
    Node(CheckedNodeId, usize),
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
fn assign_names(items: &[ObligationItem<'_>], states: &mut [ItemState<'_>]) {
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
            Outcome::LoweredScalar(lowered) => {
                let Some(ObligationItem::ScalarClaim { claim_index, .. }) = items.get(index) else {
                    return None;
                };
                Some((
                    index,
                    (
                        scalar_stem(&lowered.operation_identity),
                        NameKey::Node(lowered.node_id.clone(), *claim_index),
                    ),
                ))
            }
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
        // Embedded where a plain `bool` is required: arithmetic is refused (FR-031).
        OracleShape::PlainBool,
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

fn classify_node<'a>(
    package: &ContractPackageV1,
    lookup: &ContractLookup<'_>,
    claim_map: &ClaimMap<ExactScalarClaim>,
    node_id: &CheckedNodeId,
    claim_index: usize,
    result_selection: &ContractDriverResultSelection,
) -> ItemState<'a> {
    let graph_node = lookup.node(node_id);
    let kind = graph_node.and_then(|node| {
        (&*node.node_tag == "state" && &*node.semantic_form == "frame")
            .then_some(ObligationKind::Frame)
    });
    let subject = ObligationSubject::CheckedNode {
        node_id: node_id.clone(),
        source_map: lookup.occurrences(node_id),
    };
    let identity = Some(ItemIdentity::Node {
        package: package.identity().digest.to_string(),
        node: node_id.clone(),
        result_selection: result_selection.clone(),
    });
    let outcome = if claim_map.package_id != package.wire().package.source.package_id {
        Outcome::Invalid(InvalidObligationItem::PackageMismatch)
    } else {
        match claim_map
            .items
            .get(claim_index)
            .filter(|claim| &claim.node_id == node_id)
        {
            None => Outcome::Invalid(InvalidObligationItem::UnknownNode),
            Some(claim) => classify_claim(lookup, claim),
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

/// Artifact-free scalar-form classification shared with capability settlement.
///
/// This deliberately runs the same typed lowering classifier that the Kani
/// obligation engine uses, so settlement cannot accept an operation or domain
/// that its generator refuses.
pub(crate) fn classify_scalar_form<'a>(
    package: &'a CheckedPackageV2,
    contract: &'a ContractPackageV1,
    positions: &[crate::core::contract_package::ContractRequestPosition],
    request_index: usize,
    node_id: &CheckedNodeId,
    lookups: &mut ScalarFormLookups<'a>,
) -> ScalarFormAssessment {
    if contract.wire().package.source.package_id != *package.package_id() {
        return ScalarFormAssessment::Invalid(InvalidObligationItem::ScalarSourceMismatch);
    }
    let items = match crate::core::contract_package::PackageItems::new(contract, positions) {
        Ok(items) => items,
        Err(_) => {
            return ScalarFormAssessment::Invalid(InvalidObligationItem::ScalarPositionMismatch)
        }
    };
    if items.driver_node(request_index) != Some(node_id) {
        return ScalarFormAssessment::Invalid(InvalidObligationItem::ScalarPositionMismatch);
    }
    let Some(selection) = items.result_selection(request_index) else {
        return ScalarFormAssessment::Invalid(InvalidObligationItem::ScalarPositionMismatch);
    };
    if !matches!(selection, ContractDriverResultSelection::OwnResult { .. }) {
        return ScalarFormAssessment::Invalid(InvalidObligationItem::ScalarResultMissing);
    }
    if !crate::oracle::scalar::valid_selected_scalar_result(
        contract,
        &items,
        request_index,
        node_id,
        selection,
    ) {
        return ScalarFormAssessment::Invalid(InvalidObligationItem::ScalarContextMismatch);
    }
    let outcome = match crate::oracle::scalar::preflight_contract_scalar_claim(
        contract,
        positions,
        request_index,
    ) {
        Ok(claim) => classify_claim(lookups.contract(contract), &claim),
        Err(_) => Outcome::Invalid(InvalidObligationItem::ScalarDerivationFailed),
    };
    let graph_node = lookups.checked_node(package, node_id);
    let kind = graph_node.and_then(|node| {
        (&*node.node_tag == "state" && &*node.semantic_form == "frame")
            .then_some(ObligationKind::Frame)
    });
    match refuse_unknown_node_kind(graph_node, kind, outcome) {
        Outcome::LoweredScalar(_) => ScalarFormAssessment::Supported,
        Outcome::Unsupported(refusal) => ScalarFormAssessment::Unsupported(refusal),
        Outcome::RequiresBound(_) => ScalarFormAssessment::RequiresBound,
        Outcome::Invalid(refusal) => ScalarFormAssessment::Invalid(refusal),
        Outcome::Lowered(_)
        | Outcome::LoweredCollection(_)
        | Outcome::LoweredV2Clause(_)
        | Outcome::StateFrame(_)
        | Outcome::StateFrameRefused(_) => {
            ScalarFormAssessment::Invalid(InvalidObligationItem::UnknownNode)
        }
    }
}

pub(crate) enum ScalarFormAssessment {
    Supported,
    RequiresBound,
    Unsupported(UnsupportedObligation),
    Invalid(InvalidObligationItem),
}

/// Guards [`Outcome::LoweredScalar`] against a node whose graph entry does not match the one
/// recognized `state`/`frame` role pair, and against a node absent from the graph entirely.
/// [`ClaimMap`]/[`ExactScalarClaim`] are fully `pub`, so nothing enforces that a claim
/// map assembled by another caller names only node ids the admitted FR-330 graph also carries --
/// that invariant holds only for a claim map this crate's own
/// [`crate::oracle::scalar::generate_exact_scalar_oracles`] produced. A hand-assembled claim map
/// whose [`GeneratedScalarClaim`](crate::oracle::scalar::GeneratedScalarClaim) bounds still resolve can therefore reach
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

fn classify_claim<'a>(lookup: &ContractLookup<'_>, claim: &ExactScalarClaim) -> Outcome<'a> {
    let graph = |id: &CheckedNodeId| lookup.node(id);
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
                    let operand_ranges = operand_ranges(&lookup.graph, &claim.node_id);
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
            | ExactScalarRefusal::IncompleteContext
            | ExactScalarRefusal::PackageAccounting { .. }
            | ExactScalarRefusal::SelectedDependencyUnsupported { .. }
            | ExactScalarRefusal::AmbiguousBound { .. }
            | ExactScalarRefusal::BoundMismatch { .. }
            | ExactScalarRefusal::OperandUnsupported { .. }
            | ExactScalarRefusal::OperandRangeLimitExceeded { .. }
            | ExactScalarRefusal::UnitlessLiteralOperand { .. }
            | ExactScalarRefusal::InvalidBody { .. }
            | ExactScalarRefusal::BodyIncomplete { .. }
            | ExactScalarRefusal::LoweringWorkExhausted { .. }
            | ExactScalarRefusal::LoweringByteLimitExceeded { .. }
            | ExactScalarRefusal::LoweringLimitUnrecognised { .. }
            | ExactScalarRefusal::NotExpression { .. }
            | ExactScalarRefusal::FormMismatch { .. }
            | ExactScalarRefusal::BodyMismatch { .. }
            | ExactScalarRefusal::ResultTypeMismatch { .. }
            | ExactScalarRefusal::OperandTypeMismatch { .. }
            | ExactScalarRefusal::BooleanRightOperandMismatch { .. }
            | ExactScalarRefusal::MissingOperationIdentity { .. } => {
                Outcome::Unsupported(UnsupportedObligation::OracleRefused {
                    refusal: refusal.clone(),
                })
            }
        },
    }
}

fn reject_duplicates_and_mixtures(items: &[ObligationItem<'_>], states: &mut [ItemState<'_>]) {
    let mut first_bound_package: Option<String> = None;
    let mut first_state_package: Option<String> = None;
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
        if let ItemIdentity::StateFrame { package, .. } = &identity {
            match &first_state_package {
                None => first_state_package = Some(package.clone()),
                Some(first) if first != package => {
                    states[index].outcome =
                        Outcome::Invalid(InvalidObligationItem::MixedStatePackages);
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

const fn kind_name(kind: ObligationKind) -> &'static str {
    match kind {
        ObligationKind::Precondition => "precondition",
        ObligationKind::Postcondition => "postcondition",
        ObligationKind::Invariant => "invariant",
        ObligationKind::Frame => "frame",
    }
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
        ceilings: request.ceilings,
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
    let subject = format!(
        "{}@{}/{}",
        clause.requirement().requirement().as_str(),
        clause.requirement().revision().get(),
        clause.clause().as_str()
    );
    let subjects = std::iter::once(subject)
        .chain(embedded.iter().map(|oracle| {
            let clause = &oracle.clause;
            format!(
                "{}@{}/{}",
                clause.requirement().requirement().as_str(),
                clause.requirement().revision().get(),
                clause.clause().as_str()
            )
        }))
        .collect::<Vec<_>>();
    let source =
        crate::core::provenance::with_rust_header(source, subjects.iter().map(String::as_str));
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
    let rust = Artifact::new(
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

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;
    use serde_json::json;

    use super::*;
    use quire_contract_model::{ClauseId, RequirementRef, EXECUTABLE_PROJECTION_FORMAT};

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
        let int_type = json!({"kind":"integer","domain":"signed","minimum":"0","maximum":"1000",
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
            ceilings: crate::kani::test_support::proof_ceilings::proof_ceilings(),
            items,
            subject_path: "render_probe::subject",
            unwind: 4,
        }
    }

    /// Real, legitimately-lowered `LoweredClause` for the probe package's one precondition, with
    /// its embedded oracle source intact for the caller to mutate.
    fn render_probe_lowered<'a>(item: &ObligationItem<'a>) -> Box<LoweredClause<'a>> {
        let Outcome::Lowered(lowered) = classify(
            item,
            4,
            render_probe_request(&[]).ceilings,
            &mut BTreeMap::new(),
        )
        .outcome
        else {
            panic!("render-probe precondition must lower to a harness");
        };
        lowered
    }

    #[trace("FR-015-AC-13", "TC-025")]
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

    #[trace("FR-015-AC-13", "TC-025")]
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
    #[trace("FR-015-AC-1", "TC-025")]
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

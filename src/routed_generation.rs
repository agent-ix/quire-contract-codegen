//! FR-022: generation for items the driver has already settled and routed.
//!
//! FR-019 settles a disposition and names a backend kind; QSL `route` picks the backend. This
//! module is the generation arm of that seam. It takes the routed backend and its kind as given
//! and runs the kind's generation arm, dispatched through one exhaustive `match` over
//! [`BackendKind`] with no catch-all. It takes no manifest, candidate set, extent or capability
//! kind, so it cannot settle, select or route again, and it builds no FR-019 disposition.
//!
//! The Kani arm derives each item's FR-014 descriptor from the package with
//! [`derive_exact_scalar_items`], generates the derived oracles, and calls
//! [`negotiate_kani_obligations`] once over the routed Kani items in ascending
//! request index. That generator numbers its records by position; this module maps every position
//! back to the driver's request index and pairs each harness with its record by `harness_symbol`.

use std::collections::BTreeMap;

use quire_contract_ir::{CheckedNodeId, CheckedPackageV2};

use crate::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, negotiate_kani_obligations,
    AttestationContext, BackendKind, Candidate, ClaimMap, ExactScalarClaim, InvalidObligationItem,
    KaniObligationError, KaniObligationOutcome, KaniObligationRequest, KaniScalarObligationHarness,
    KaniToolPins, ObligationDisposition, ObligationItem, ObligationRecord, OracleGenerationError,
};

/// One item the driver routed to a backend.
///
/// Trace: TC-033
// Implements: FR-022
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutedGenerationItem {
    /// The item's index in the driver's request.
    pub request_index: usize,
    /// The IR node the item checks.
    pub node_id: CheckedNodeId,
    /// The routed backend, as the FR-331 wire pair.
    pub backend: Candidate,
    /// The routed backend's kind.
    pub kind: BackendKind,
}

/// The Kani generation context. Its fields have FR-015's meanings.
#[derive(Clone, Copy, Debug)]
pub struct KaniGenerationContext<'a> {
    /// Rust path of the customer subject.
    pub subject_path: &'a str,
    /// The backend identity harnesses are generated for.
    pub pins: &'a KaniToolPins,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Binding for the generation identity.
    pub attestation: AttestationContext<'a>,
}

/// One optional generation context per [`BackendKind`] variant.
///
/// A kind added without an arm in the generation dispatch and in `has` does not compile; its
/// context field is added beside those arms.
#[derive(Clone, Copy, Debug, Default)]
pub struct GenerationContexts<'a> {
    /// The Kani context.
    pub kani: Option<KaniGenerationContext<'a>>,
}

impl GenerationContexts<'_> {
    /// Whether the context for `kind` is supplied.
    const fn has(&self, kind: BackendKind) -> bool {
        match kind {
            BackendKind::Kani => self.kani.is_some(),
        }
    }
}

/// A backend kind's output for one routed item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KindOutput {
    /// The Kani arm's record and, when the item lowered, its harness.
    Kani {
        /// FR-015's record, with every index the driver's request index.
        record: ObligationRecord,
        /// The item's harness, when one was emitted.
        harness: Option<KaniScalarObligationHarness>,
    },
}

/// The output for one routed item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutedItemOutput {
    /// The driver's request index.
    pub request_index: usize,
    /// The routed backend.
    pub backend: Candidate,
    /// The routed kind's output.
    pub output: KindOutput,
}

/// The output of one call.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RoutedGeneration {
    /// One entry per routed item, in ascending request index.
    pub items: Vec<RoutedItemOutput>,
    /// The kinds whose arm rejected its whole group, in [`BackendKind::ALL`] order.
    pub rejected: Vec<BackendKind>,
    /// The FR-014 claim map the Kani arm derived for its group, ascending by node id: the
    /// generated claims of the derivable nodes and a `NoDerivableClaim` claim for each other.
    /// `None` when no Kani group ran.
    pub claim_map: Option<ClaimMap<ExactScalarClaim>>,
}

/// A whole-call refusal; nothing is generated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RoutedGenerationError {
    /// The routed kind is not the kind the backend's identity converts to.
    BackendKindDisagrees {
        /// The item's request index.
        request_index: usize,
        /// The routed backend.
        backend: Candidate,
        /// The routed kind.
        routed: BackendKind,
        /// The kind [`BackendKind::from_identity`] returns, or `None` when it has none.
        converted: Option<BackendKind>,
    },
    /// Two routed items share one request index.
    DuplicateRequestIndex {
        /// The shared index.
        request_index: usize,
    },
    /// A routed item's kind has no supplied context.
    MissingKindContext {
        /// The kind.
        kind: BackendKind,
    },
    /// The Kani arm refused its group as a whole.
    Kani(KaniObligationError),
    /// FR-014 generation over the derived items failed as a whole.
    Oracle(OracleGenerationError),
}

/// What one kind's arm produced.
struct ArmOutput {
    outputs: Vec<RoutedItemOutput>,
    rejected: bool,
}

/// Runs the generation arm of every routed item's backend kind.
///
/// Refusals are checked in the order kind disagreement, duplicate request index, missing context,
/// each naming the lowest offending request index, before anything is generated.
///
/// Trace: TC-033
// Implements: FR-022
pub fn generate_routed(
    package: &CheckedPackageV2,
    routed: &[RoutedGenerationItem],
    contexts: &GenerationContexts<'_>,
) -> Result<RoutedGeneration, RoutedGenerationError> {
    let mut ordered = routed.iter().collect::<Vec<_>>();
    ordered.sort_by(|a, b| {
        (a.request_index, &a.backend, a.kind.index()).cmp(&(
            b.request_index,
            &b.backend,
            b.kind.index(),
        ))
    });
    refuse_inconsistent_routing(&ordered, contexts)?;

    let mut items = Vec::with_capacity(ordered.len());
    let mut rejected = Vec::new();
    let mut claim_map = None;
    for kind in BackendKind::ALL {
        let group = ordered
            .iter()
            .copied()
            .filter(|item| item.kind == kind)
            .collect::<Vec<_>>();
        if group.is_empty() {
            continue;
        }
        let arm = match kind {
            BackendKind::Kani => {
                let (arm, kani_claim_map) = generate_kani(package, &group, contexts)?;
                claim_map = Some(kani_claim_map);
                arm
            }
        };
        if arm.rejected {
            rejected.push(kind);
        }
        items.extend(arm.outputs);
    }
    items.sort_by_key(|item| item.request_index);
    Ok(RoutedGeneration {
        items,
        rejected,
        claim_map,
    })
}

/// The three whole-call refusals, in their fixed order.
fn refuse_inconsistent_routing(
    ordered: &[&RoutedGenerationItem],
    contexts: &GenerationContexts<'_>,
) -> Result<(), RoutedGenerationError> {
    for item in ordered {
        let converted = BackendKind::from_identity(&item.backend.identity);
        if converted != Some(item.kind) {
            return Err(RoutedGenerationError::BackendKindDisagrees {
                request_index: item.request_index,
                backend: item.backend.clone(),
                routed: item.kind,
                converted,
            });
        }
    }
    if let Some(pair) = ordered
        .windows(2)
        .find(|pair| pair[0].request_index == pair[1].request_index)
    {
        return Err(RoutedGenerationError::DuplicateRequestIndex {
            request_index: pair[0].request_index,
        });
    }
    if let Some(item) = ordered.iter().find(|item| !contexts.has(item.kind)) {
        return Err(RoutedGenerationError::MissingKindContext { kind: item.kind });
    }
    Ok(())
}

/// The Kani arm, and the claim map it derived: one `negotiate_kani_obligations` call over `group`, in ascending request index.
fn generate_kani(
    package: &CheckedPackageV2,
    group: &[&RoutedGenerationItem],
    contexts: &GenerationContexts<'_>,
) -> Result<(ArmOutput, ClaimMap<ExactScalarClaim>), RoutedGenerationError> {
    let Some(context) = contexts.kani else {
        return Err(RoutedGenerationError::MissingKindContext {
            kind: BackendKind::Kani,
        });
    };
    let claim_map = derive_claim_map(package, group)?;
    let items = group
        .iter()
        .map(|item| ObligationItem::ScalarClaim {
            package,
            claim_map: &claim_map,
            node_id: &item.node_id,
        })
        .collect::<Vec<_>>();
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        items: &items,
        subject_path: context.subject_path,
        pins: context.pins,
        unwind: context.unwind,
        attestation: context.attestation,
    })
    .map_err(RoutedGenerationError::Kani)?;

    // FR-015 reports exactly one record per item, numbered by position in `group` (the loop index
    // in `negotiate_kani_obligations`), and a `DuplicateItem`'s `first_index` is an earlier
    // position. So `records` and `group` pair one to one, and every position is in `driver`.
    let driver = group
        .iter()
        .map(|item| item.request_index)
        .collect::<Vec<_>>();
    let (records, mut harnesses, rejected) = match outcome {
        KaniObligationOutcome::Emitted {
            records,
            scalar_harnesses,
            ..
        } => (
            records,
            scalar_harnesses
                .into_iter()
                .map(|harness| (harness.identity.harness_symbol.clone(), harness))
                .collect::<BTreeMap<_, _>>(),
            false,
        ),
        KaniObligationOutcome::Rejected { records } => (records, BTreeMap::new(), true),
    };
    debug_assert_eq!(
        records.len(),
        group.len(),
        "FR-015 reports one record per item"
    );
    let outputs = records
        .into_iter()
        .zip(group)
        .map(|(mut record, item)| {
            record.request_index = item.request_index;
            if let ObligationDisposition::InvalidRequest {
                reason: InvalidObligationItem::DuplicateItem { first_index },
            } = &mut record.disposition
            {
                let first = driver.get(*first_index).copied();
                debug_assert!(first.is_some(), "FR-015 names an earlier position");
                if let Some(first) = first {
                    *first_index = first;
                }
            }
            let harness = match &record.disposition {
                ObligationDisposition::Supported { harness_symbol } => {
                    harnesses.remove(harness_symbol)
                }
                ObligationDisposition::RequiresBound { .. }
                | ObligationDisposition::Unsupported { .. }
                | ObligationDisposition::InvalidRequest { .. } => None,
            };
            RoutedItemOutput {
                request_index: item.request_index,
                backend: item.backend.clone(),
                output: KindOutput::Kani { record, harness },
            }
        })
        .collect();
    Ok((ArmOutput { outputs, rejected }, claim_map))
}

/// The FR-014 claim map of `group`'s distinct nodes: each derivable node generated from its
/// derived descriptor, each other node a `NoDerivableClaim` claim, ascending by node id.
fn derive_claim_map(
    package: &CheckedPackageV2,
    group: &[&RoutedGenerationItem],
) -> Result<ClaimMap<ExactScalarClaim>, RoutedGenerationError> {
    // A node routed twice is one claim: FR-014 refuses every copy of a repeated request, and FR-015
    // reports the repeat as its own `DuplicateItem`.
    let mut node_ids = group
        .iter()
        .map(|item| item.node_id.clone())
        .collect::<Vec<_>>();
    node_ids.sort();
    node_ids.dedup();
    let mut derivable = Vec::new();
    let mut underivable = Vec::new();
    for (node_id, derived) in node_ids
        .iter()
        .zip(derive_exact_scalar_items(package, &node_ids))
    {
        match derived {
            Ok(item) => derivable.push(item),
            Err(refusal) => {
                underivable.push(ExactScalarClaim::derivation_refused(
                    package,
                    node_id.clone(),
                    refusal,
                ));
            }
        }
    }
    let mut claim_map = generate_exact_scalar_oracles(package, &derivable)
        .map_err(RoutedGenerationError::Oracle)?
        .claim_map;
    claim_map.items.extend(underivable);
    claim_map
        .items
        .sort_by(|left, right| left.node_id.cmp(&right.node_id));
    Ok(claim_map)
}

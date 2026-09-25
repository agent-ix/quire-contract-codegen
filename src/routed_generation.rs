//! FR-022: generation for items the driver has already settled and routed.
//!
//! FR-019 settles a disposition and names a backend kind; QSL `route` picks the backend. This
//! module is the generation arm of that seam. It takes the routed backend and its kind as given
//! and runs the kind's generation arm, dispatched through one exhaustive `match` over
//! [`BackendKind`] with no catch-all. It takes no manifest, candidate set, extent or capability
//! kind, so it cannot settle, select or route again, and it builds no FR-019 disposition.
//!
//! The Kani arm calls [`negotiate_kani_obligations`] once over the routed Kani items in ascending
//! request index. That generator numbers its records by position; this module maps every position
//! back to the driver's request index and pairs each harness with its record by `harness_symbol`.

use std::collections::BTreeMap;

use quire_contract_ir::{CheckedNodeId, CheckedPackageV2};

use crate::{
    negotiate_kani_obligations, AttestationContext, BackendKind, Candidate, ClaimMap,
    ExactScalarClaim, InvalidObligationItem, KaniObligationError, KaniObligationOutcome,
    KaniObligationRequest, KaniScalarObligationHarness, KaniToolPins, ObligationDisposition,
    ObligationItem, ObligationRecord,
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
    /// The FR-014 claim map generated from the same package.
    pub claim_map: &'a ClaimMap<ExactScalarClaim>,
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
/// A struct member rather than a map entry, so a kind added without its context does not compile.
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
}

/// What one kind's arm produced.
struct ArmOutput {
    outputs: Vec<(usize, KindOutput)>,
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
            BackendKind::Kani => generate_kani(package, &group, contexts)?,
        };
        if arm.rejected {
            rejected.push(kind);
        }
        for (position, (request_index, output)) in arm.outputs.into_iter().enumerate() {
            // `arm.outputs` is in `group` order, so `position` indexes `group`.
            if let Some(item) = group.get(position) {
                items.push(RoutedItemOutput {
                    request_index,
                    backend: item.backend.clone(),
                    output,
                });
            }
        }
    }
    items.sort_by_key(|item| item.request_index);
    Ok(RoutedGeneration { items, rejected })
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

/// The Kani arm: one `negotiate_kani_obligations` call over `group`, in ascending request index.
fn generate_kani(
    package: &CheckedPackageV2,
    group: &[&RoutedGenerationItem],
    contexts: &GenerationContexts<'_>,
) -> Result<ArmOutput, RoutedGenerationError> {
    let Some(context) = contexts.kani else {
        return Err(RoutedGenerationError::MissingKindContext {
            kind: BackendKind::Kani,
        });
    };
    let items = group
        .iter()
        .map(|item| ObligationItem::ScalarClaim {
            package,
            claim_map: context.claim_map,
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

    // FR-015 numbers records by position in `group`; every position maps back to a driver index.
    // A position outside `group` cannot occur, since FR-015 reports only positions of its input;
    // it is left as reported rather than invented.
    let driver_index = |position: usize| group.get(position).map_or(position, |i| i.request_index);
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
    let outputs = records
        .into_iter()
        .map(|mut record| {
            let request_index = driver_index(record.request_index);
            record.request_index = request_index;
            if let ObligationDisposition::InvalidRequest {
                reason: InvalidObligationItem::DuplicateItem { first_index },
            } = &mut record.disposition
            {
                *first_index = driver_index(*first_index);
            }
            let harness = match &record.disposition {
                ObligationDisposition::Supported { harness_symbol } => {
                    harnesses.remove(harness_symbol)
                }
                ObligationDisposition::RequiresBound { .. }
                | ObligationDisposition::Unsupported { .. }
                | ObligationDisposition::InvalidRequest { .. } => None,
            };
            (request_index, KindOutput::Kani { record, harness })
        })
        .collect();
    Ok(ArmOutput { outputs, rejected })
}

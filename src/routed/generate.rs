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
//! request index. The oracle crate FR-014 generated is returned beside the claim map. Each harness
//! embeds its own oracle's source, which uses `quire_contract_runtime`, so the driver writes the
//! returned `Cargo.toml` and the harness's own source as `src/lib.rs`, and not the returned
//! `src/lib.rs`. That generator numbers its records by position; this module maps every position
//! back to the driver's request index and pairs each harness with its record by `harness_symbol`.

use std::collections::{btree_map::Entry, BTreeMap};

use quire_contract_model::{CheckedNodeId, CheckedPackageV2};

use crate::{
    core::artifact::Artifact,
    core::identity::{HarnessPath, HarnessSymbol},
    kani::generate::negotiate::negotiate_kani_obligations,
    kani::generate::outcome::{
        InvalidObligationItem, KaniObligationError, KaniObligationOutcome, KaniObligationRequest,
        ObligationDisposition, ObligationItem, ObligationRecord,
    },
    kani::identity::KaniScalarObligationHarness,
    oracle::claim::{ClaimMap, OracleGenerationError},
    oracle::scalar::{
        derive_exact_scalar_items, generate_exact_scalar_oracles, ExactScalarClaim,
        ExactScalarOracles,
    },
    routed::capability::{BackendKind, Candidate},
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
    /// Resource ceilings recorded by every generated harness.
    pub ceilings: crate::kani::identity::ProofCeilings,
    /// Rust path of the customer subject.
    pub subject_path: &'a str,
    /// Loop unwind bound.
    pub unwind: u32,
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
    /// The FR-014 oracle crate the Kani arm generated for its group: `Cargo.toml`, `src/lib.rs`
    /// and `claim-map.json`, byte-identical to `generate_exact_scalar_oracles`'s artifacts over the
    /// group's derived items. Every `Generated` claim's oracle symbol is defined in its
    /// `src/lib.rs`. `None` when no Kani group ran.
    pub oracle_artifacts: Option<Vec<Artifact>>,
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
    /// Two harnesses the Kani arm emitted share one proof symbol, so a record's
    /// `Supported { harness_symbol }` cannot name one of them.
    DuplicateHarness {
        /// The `module::harness` path of the second harness with the shared symbol.
        harness: HarnessPath,
    },
    /// FR-015 reported a number of records other than the Kani group's item count, so a record
    /// cannot be paired with its item.
    KaniRecordCountMismatch {
        /// The number of records reported.
        records: usize,
        /// The number of items in the Kani group.
        items: usize,
    },
    /// FR-015 reported a `DuplicateItem` whose `first_index` is not a position in the Kani group,
    /// so it cannot be rewritten to a request index.
    KaniDuplicatePositionOutOfRange {
        /// The position the record named.
        first_index: usize,
        /// The number of items in the Kani group.
        items: usize,
    },
    /// The Kani arm refused its group as a whole.
    Kani(KaniObligationError),
    /// FR-014 generation over the derived items failed as a whole.
    Oracle(OracleGenerationError),
}

/// The harnesses by proof symbol, which is what a record's `Supported` disposition names. Two
/// harnesses with one symbol are a typed refusal: collecting them into the map would keep the last
/// and silently drop the first (AD-004 L-10).
fn index_harnesses(
    harnesses: Vec<KaniScalarObligationHarness>,
) -> Result<BTreeMap<HarnessSymbol, KaniScalarObligationHarness>, RoutedGenerationError> {
    let mut indexed = BTreeMap::new();
    for harness in harnesses {
        match indexed.entry(harness.identity.harness_symbol.clone()) {
            Entry::Occupied(_) => {
                return Err(RoutedGenerationError::DuplicateHarness {
                    harness: harness.identity.harness_path(),
                });
            }
            Entry::Vacant(slot) => {
                slot.insert(harness);
            }
        }
    }
    Ok(indexed)
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
    let mut oracle_artifacts = None;
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
                let (arm, kani_claim_map, kani_artifacts) =
                    generate_kani(package, &group, contexts)?;
                claim_map = Some(kani_claim_map);
                oracle_artifacts = Some(kani_artifacts);
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
        oracle_artifacts,
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

/// The Kani arm, and the claim map and oracle crate it derived: one `negotiate_kani_obligations` call over `group`, in ascending request index.
fn generate_kani(
    package: &CheckedPackageV2,
    group: &[&RoutedGenerationItem],
    contexts: &GenerationContexts<'_>,
) -> Result<(ArmOutput, ClaimMap<ExactScalarClaim>, Vec<Artifact>), RoutedGenerationError> {
    let Some(context) = contexts.kani else {
        return Err(RoutedGenerationError::MissingKindContext {
            kind: BackendKind::Kani,
        });
    };
    let (claim_map, artifacts) = derive_claim_map(package, group)?;
    let items = group
        .iter()
        .map(|item| ObligationItem::ScalarClaim {
            package,
            claim_map: &claim_map,
            node_id: &item.node_id,
        })
        .collect::<Vec<_>>();
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: context.ceilings,
        items: &items,
        subject_path: context.subject_path,
        unwind: context.unwind,
    })
    .map_err(RoutedGenerationError::Kani)?;

    // FR-015 reports exactly one record per item, numbered by position in `group` (the loop index
    // in `negotiate_kani_obligations`), and a `DuplicateItem`'s `first_index` is an earlier
    // position. So `records` and `group` pair one to one; a position outside `group` is refused.
    let (records, mut harnesses, rejected) = match outcome {
        KaniObligationOutcome::Emitted {
            records,
            scalar_harnesses,
            ..
        } => (records, index_harnesses(scalar_harnesses)?, false),
        KaniObligationOutcome::Rejected { records } => (records, BTreeMap::new(), true),
    };
    let outputs = route_records(records, group, &mut harnesses)?;
    Ok((ArmOutput { outputs, rejected }, claim_map, artifacts))
}

/// Pairs FR-015's records with `group`, rewrites every record's positions into the driver's
/// request index and joins each `Supported` record with its harness. A count mismatch or a
/// `DuplicateItem` position outside `group` refuses the whole call.
fn route_records(
    records: Vec<ObligationRecord>,
    group: &[&RoutedGenerationItem],
    harnesses: &mut BTreeMap<HarnessSymbol, KaniScalarObligationHarness>,
) -> Result<Vec<RoutedItemOutput>, RoutedGenerationError> {
    pair_records(records, group)?
        .into_iter()
        .map(|(mut record, item)| {
            record.request_index = item.request_index;
            rewrite_duplicate_position(&mut record, group)?;
            let harness = match &record.disposition {
                ObligationDisposition::Supported { harness_symbol } => {
                    harnesses.remove(harness_symbol)
                }
                ObligationDisposition::RequiresBound { .. }
                | ObligationDisposition::Unsupported { .. }
                | ObligationDisposition::InvalidRequest { .. } => None,
            };
            Ok(RoutedItemOutput {
                request_index: item.request_index,
                backend: item.backend.clone(),
                output: KindOutput::Kani { record, harness },
            })
        })
        .collect()
}

/// Rewrites a `DuplicateItem` record's `first_index`, a position in `group`, into the request
/// index of the item at that position. Any other record is unchanged. A position outside `group`
/// is a typed refusal, so the record is never left naming a position the driver did not send.
fn rewrite_duplicate_position(
    record: &mut ObligationRecord,
    group: &[&RoutedGenerationItem],
) -> Result<(), RoutedGenerationError> {
    if let ObligationDisposition::InvalidRequest {
        reason: InvalidObligationItem::DuplicateItem { first_index },
    } = &mut record.disposition
    {
        let Some(earlier) = group.get(*first_index) else {
            return Err(RoutedGenerationError::KaniDuplicatePositionOutOfRange {
                first_index: *first_index,
                items: group.len(),
            });
        };
        *first_index = earlier.request_index;
    }
    Ok(())
}

/// Pairs each of FR-015's records with the item of `group` at its position. A different count is
/// a typed refusal and nothing is generated, so the pairing never truncates to the shorter side
/// and drops items or records.
fn pair_records<'g>(
    records: Vec<ObligationRecord>,
    group: &'g [&'g RoutedGenerationItem],
) -> Result<Vec<(ObligationRecord, &'g RoutedGenerationItem)>, RoutedGenerationError> {
    if records.len() != group.len() {
        return Err(RoutedGenerationError::KaniRecordCountMismatch {
            records: records.len(),
            items: group.len(),
        });
    }
    Ok(records.into_iter().zip(group.iter().copied()).collect())
}

/// The FR-014 claim map of `group`'s distinct nodes: each derivable node generated from its
/// derived descriptor, each other node a `NoDerivableClaim` claim, ascending by node id; and the
/// oracle crate FR-014 generated for the derivable nodes.
fn derive_claim_map(
    package: &CheckedPackageV2,
    group: &[&RoutedGenerationItem],
) -> Result<(ClaimMap<ExactScalarClaim>, Vec<Artifact>), RoutedGenerationError> {
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
    let ExactScalarOracles {
        artifacts,
        mut claim_map,
    } = generate_exact_scalar_oracles(package, &derivable)
        .map_err(RoutedGenerationError::Oracle)?;
    claim_map.items.extend(underivable);
    claim_map
        .items
        .sort_by(|left, right| left.node_id.cmp(&right.node_id));
    Ok((claim_map, artifacts))
}

#[cfg(test)]
mod tests {
    use super::{
        index_harnesses, pair_records, rewrite_duplicate_position, route_records,
        RoutedGenerationError, RoutedGenerationItem,
    };
    use crate::{
        core::artifact::Artifact,
        core::identity::{HarnessSymbol, ModuleSymbol},
        kani::abi::KaniSolver,
        kani::generate::outcome::{
            InvalidObligationItem, ObligationDisposition, ObligationRecord, ObligationSubject,
        },
        kani::identity::{KaniScalarObligationHarness, ScalarObligationIdentity},
        routed::capability::{BackendKind, Candidate},
    };

    fn node_id() -> quire_contract_model::CheckedNodeId {
        serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": "1".repeat(64),
        }))
        .expect("a checked node id")
    }

    fn record(request_index: usize) -> ObligationRecord {
        ObligationRecord {
            request_index,
            kind: None,
            subject: ObligationSubject::CheckedNode {
                node_id: node_id(),
                source_map: Vec::new(),
            },
            disposition: ObligationDisposition::InvalidRequest {
                reason: InvalidObligationItem::UnknownNode,
            },
        }
    }

    fn item(request_index: usize) -> RoutedGenerationItem {
        RoutedGenerationItem {
            request_index,
            node_id: node_id(),
            backend: Candidate {
                identity: "kani".to_owned(),
            },
            kind: BackendKind::Kani,
        }
    }

    /// Trace: NFR-005-AC-5, TC-042.
    #[test]
    fn tc_042_ac5_a_record_count_other_than_the_item_count_is_a_typed_refusal() {
        let items = [item(10), item(11), item(12)];
        let group = items.iter().collect::<Vec<_>>();

        let fewer = pair_records(vec![record(0), record(1)], &group)
            .expect_err("one record fewer than the items is refused");
        assert_eq!(
            fewer,
            RoutedGenerationError::KaniRecordCountMismatch {
                records: 2,
                items: 3
            }
        );
        let more = pair_records((0..4).map(record).collect(), &group)
            .expect_err("one record more than the items is refused");
        assert_eq!(
            more,
            RoutedGenerationError::KaniRecordCountMismatch {
                records: 4,
                items: 3
            }
        );

        let paired = pair_records((0..3).map(record).collect(), &group).expect("equal counts pair");
        assert_eq!(
            paired
                .iter()
                .map(|(record, item)| (record.request_index, item.request_index))
                .collect::<Vec<_>>(),
            vec![(0, 10), (1, 11), (2, 12)]
        );
    }

    fn duplicate_record(first_index: usize) -> ObligationRecord {
        ObligationRecord {
            disposition: ObligationDisposition::InvalidRequest {
                reason: InvalidObligationItem::DuplicateItem { first_index },
            },
            ..record(0)
        }
    }

    /// Trace: NFR-005-AC-6, TC-042.
    #[test]
    fn tc_042_ac6_a_duplicate_position_outside_the_group_is_a_typed_refusal() {
        let items = [item(10), item(11), item(12)];
        let group = items.iter().collect::<Vec<_>>();

        for first_index in [3, 4, usize::MAX] {
            let mut record = duplicate_record(first_index);
            assert_eq!(
                rewrite_duplicate_position(&mut record, &group),
                Err(RoutedGenerationError::KaniDuplicatePositionOutOfRange {
                    first_index,
                    items: 3
                })
            );
        }

        let mut last = duplicate_record(2);
        rewrite_duplicate_position(&mut last, &group).expect("the last valid position rewrites");
        assert_eq!(
            last.disposition,
            ObligationDisposition::InvalidRequest {
                reason: InvalidObligationItem::DuplicateItem { first_index: 12 }
            }
        );

        let mut other = record(0);
        rewrite_duplicate_position(&mut other, &group).expect("another record is unchanged");
        assert_eq!(other, record(0));
    }

    /// Trace: NFR-005-AC-6, TC-042. The refusal reaches the caller of the Kani arm's
    /// record routing: one record naming a position outside the group refuses the whole call and
    /// yields no output, while in-range duplicates are rewritten to request indices.
    #[test]
    fn tc_042_ac6_an_out_of_range_duplicate_position_refuses_the_whole_routing() {
        let items = [item(10), item(11), item(12)];
        let group = items.iter().collect::<Vec<_>>();
        let mut harnesses = std::collections::BTreeMap::new();

        let refused = route_records(
            vec![record(0), duplicate_record(7), duplicate_record(0)],
            &group,
            &mut harnesses,
        );
        assert_eq!(
            refused,
            Err(RoutedGenerationError::KaniDuplicatePositionOutOfRange {
                first_index: 7,
                items: 3
            })
        );

        let routed = route_records(
            vec![record(0), duplicate_record(0), duplicate_record(1)],
            &group,
            &mut harnesses,
        )
        .expect("in-range positions route");
        let positions = routed
            .iter()
            .map(|output| match &output.output {
                super::KindOutput::Kani { record, .. } => match &record.disposition {
                    ObligationDisposition::InvalidRequest {
                        reason: InvalidObligationItem::DuplicateItem { first_index },
                    } => Some(*first_index),
                    _ => None,
                },
            })
            .collect::<Vec<_>>();
        assert_eq!(positions, vec![None, Some(10), Some(11)]);
    }

    fn harness(module: &str, symbol: &str) -> KaniScalarObligationHarness {
        let node_id = serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": "1".repeat(64),
        }))
        .expect("a checked node id");
        KaniScalarObligationHarness {
            identity: ScalarObligationIdentity {
                ceilings: crate::kani::test_support::proof_ceilings(),
                node_id,
                operation_identity: "quire.op.integer.add".to_owned(),
                oracle_symbol: "oracle".to_owned(),
                module_symbol: ModuleSymbol::try_from(module).expect("a module symbol"),
                harness_symbol: HarnessSymbol::try_from(symbol).expect("a harness symbol"),
                arguments: Vec::new(),
                solver: KaniSolver::Cadical,
                unwind: 1,
                options: Vec::new(),
            },
            rust: Artifact::new(format!("src/generated/{module}.rs"), String::new()),
            record: Artifact::new(format!("kani-obligations/{module}.json"), String::new()),
        }
    }

    /// Trace: TC-033, FR-022-AC-16.
    #[test]
    fn tc_033_two_harnesses_sharing_a_symbol_are_a_typed_error_not_a_silent_overwrite() {
        let refused = index_harnesses(vec![
            harness("a_module", "x_proof"),
            harness("b_module", "x_proof"),
        ])
        .expect_err("a shared symbol is refused");
        let RoutedGenerationError::DuplicateHarness { harness } = refused else {
            panic!("expected DuplicateHarness, got {refused:?}");
        };
        assert_eq!(harness.to_string(), "b_module::x_proof");

        let indexed = index_harnesses(vec![harness_pair().0, harness_pair().1])
            .expect("distinct symbols are indexed");
        assert_eq!(indexed.len(), 2);
    }

    fn harness_pair() -> (KaniScalarObligationHarness, KaniScalarObligationHarness) {
        (
            harness("a_module", "a_proof"),
            harness("b_module", "b_proof"),
        )
    }
}

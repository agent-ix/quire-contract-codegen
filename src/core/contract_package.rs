// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-039: position-preserving access to Contract IR's admitted FR-330 package.

use std::collections::BTreeSet;

use quire_contract_model::{
    CheckedNodeId, CheckedNodeTag, CheckedSourceMapEntry, CompleteLoweringProfileV2,
    ContractFeatureRecordV1, ContractOccurrenceKeyV1, ContractPackageV1, ContractReachedClosureV1,
    ContractRootViewError, ContractRootViewV1, LoweringProfileRef,
};

/// The CG-owned lowering profile for one mixed routed request.
///
/// The driver passes this profile to Contract IR before submitting the single
/// admitted package to negotiation and generation. Its tags cover the built-in
/// scalar, equality and exact-function arms; process providers do not enlarge
/// that set. A lowered scalar with an unbounded operand still receives CG's
/// typed `RequiresBound` refusal from its admitted reached closure.
///
/// Trace: FR-039-AC-2, TC-053.
#[must_use]
pub fn routed_contract_lowering_profile() -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
        reference: LoweringProfileRef {
            authority: "agent-ix".into(),
            identity: "quire.codegen.routed-contract/v1".into(),
        },
        supported_tags: BTreeSet::from([
            CheckedNodeTag::ScalarType,
            CheckedNodeTag::CompositeType,
            CheckedNodeTag::BoundedDomain,
            CheckedNodeTag::Value,
            CheckedNodeTag::Expression,
            CheckedNodeTag::Function,
            CheckedNodeTag::Claim,
            CheckedNodeTag::Correspondence,
        ]),
        require_bounds: false,
        work_limit: 65_536,
    }
}

/// The role of one root in the exact sequence passed to Contract IR lowering.
///
/// Bound and excluded driver positions retain their original indexes even when
/// context-only roots are interleaved. Excluded and context positions have
/// FR-330 accounting but no backend settlement or generated claim of their own.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractRequestPosition {
    /// One original driver request, including repeated requested node IDs.
    DriverItem {
        /// Exact root key of this original driver request.
        node_id: CheckedNodeId,
        /// Caller-authored result occurrence for scalar expressions.
        result_selection: ContractDriverResultSelection,
    },
    /// Accounting-only driver position with no envelope item or generated claim.
    Excluded {
        /// Exact root key retained in admitted accounting.
        node_id: CheckedNodeId,
    },
    /// A root needed to represent the meaning of a driver item.
    ContextRoot {
        /// Exact root key submitted only to represent request context.
        node_id: CheckedNodeId,
    },
}

/// The result occurrence selected by the driver for one request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractDriverResultSelection {
    /// This request has no scalar result-context query.
    NotApplicable,
    /// The caller selected this application's own result and optional direct narrow.
    OwnResult {
        /// Exact checked application occurrence.
        application: ContractOccurrenceKeyV1,
        /// Exact checked conversion occurrence, when the result is narrowed.
        conversion: Option<ContractOccurrenceKeyV1>,
    },
}

impl ContractRequestPosition {
    /// The exact root key submitted at this lowering position.
    #[must_use]
    pub fn node_id(&self) -> &CheckedNodeId {
        match self {
            Self::DriverItem { node_id, .. }
            | Self::Excluded { node_id }
            | Self::ContextRoot { node_id } => node_id,
        }
    }
}

/// A mismatch between the driver's request and one admitted package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PackageRequestError {
    /// The caller supplied a different number of descriptors and request positions.
    ItemPositions {
        /// Number of descriptors.
        items: usize,
        /// Number of request positions.
        positions: usize,
    },
    /// The package has a different number of accounting records.
    Count {
        /// Number of keys the caller requested.
        requested: usize,
        /// Number of admitted accounting records.
        accounted: usize,
    },
    /// An accounting record names a different exact requested key.
    Identity {
        /// Position of the disagreement.
        request_index: usize,
        /// Exact key requested by the caller.
        requested: CheckedNodeId,
        /// Exact key in the admitted accounting record.
        accounted: CheckedNodeId,
    },
    /// A lowering root, driver or context, disagrees with IR accounting.
    AccountingIdentity {
        /// Position in the full lowering request.
        accounting_index: usize,
        /// Exact root key passed to lowering.
        requested: CheckedNodeId,
        /// Exact key in admitted FR-330 accounting.
        accounted: CheckedNodeId,
    },
    /// A requested position is outside the admitted accounting.
    Position {
        /// Position outside the admitted accounting.
        request_index: usize,
    },
    /// A generator selected an accounting-only driver position.
    ExcludedDriverIndex {
        /// Original driver request index.
        request_index: usize,
        /// Exact key of the excluded position.
        node_id: CheckedNodeId,
    },
    /// A context root follows an excluded position, so no bound item owns it.
    OrphanContextRoot {
        /// Position in the complete lowering sequence.
        accounting_index: usize,
        /// Exact unowned root key.
        node_id: CheckedNodeId,
    },
    /// The admitted traversal could not reproduce a lowered item's closure.
    ReachedClosure {
        /// Position of the lowered record.
        request_index: usize,
        /// Exact key whose closure failed.
        node_id: CheckedNodeId,
        /// Reader traversal's typed failure.
        cause: Box<ContractRootViewError>,
    },
}

/// The exact admitted FR-330 decision and source occurrences at one request position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedItemAccounting {
    /// Contract IR's original typed record, including a `Lowered` disposition.
    pub record: ContractFeatureRecordV1,
    /// Exact source occurrences in the admitted request-position source map.
    pub occurrences: Vec<CheckedSourceMapEntry>,
}

/// The admitted meaning and exact available source occurrences for one position.
pub(crate) enum PackageItem<'a> {
    /// The root is represented in the package's semantic graph.
    Lowered {
        record: &'a ContractFeatureRecordV1,
        closure: ContractReachedClosureV1<'a>,
        occurrences: &'a [CheckedSourceMapEntry],
    },
    /// The original typed accounting record remains the cause of refusal.
    Refused {
        record: &'a ContractFeatureRecordV1,
        occurrences: &'a [CheckedSourceMapEntry],
    },
}

impl PackageItem<'_> {
    /// Own the record and occurrences once for a public production decision.
    pub(crate) fn accounting(&self) -> AdmittedItemAccounting {
        match self {
            Self::Lowered {
                record,
                occurrences,
                ..
            }
            | Self::Refused {
                record,
                occurrences,
            } => AdmittedItemAccounting {
                record: (**record).clone(),
                occurrences: occurrences.to_vec(),
            },
        }
    }
}

/// A checked correspondence between request positions and one admitted package.
pub(crate) struct PackageItems<'a> {
    package: &'a ContractPackageV1,
    positions: &'a [ContractRequestPosition],
    driver_to_accounting: Vec<usize>,
    bound_driver_indices: Vec<usize>,
}

impl<'a> PackageItems<'a> {
    /// Match every position and exact key before any item can be consumed.
    pub(crate) fn new(
        package: &'a ContractPackageV1,
        positions: &'a [ContractRequestPosition],
    ) -> Result<Self, PackageRequestError> {
        let accounted = &package.wire().feature_accounting;
        if positions.len() != accounted.len() {
            return Err(PackageRequestError::Count {
                requested: positions.len(),
                accounted: accounted.len(),
            });
        }
        let mut driver_to_accounting = Vec::new();
        let mut bound_driver_indices = Vec::new();
        let mut after_excluded = false;
        for (accounting_index, (position, record)) in positions.iter().zip(accounted).enumerate() {
            if position.node_id() != record.node_id() {
                return Err(PackageRequestError::AccountingIdentity {
                    accounting_index,
                    requested: position.node_id().clone(),
                    accounted: record.node_id().clone(),
                });
            }
            match position {
                ContractRequestPosition::DriverItem { .. } => {
                    bound_driver_indices.push(driver_to_accounting.len());
                    driver_to_accounting.push(accounting_index);
                    after_excluded = false;
                }
                ContractRequestPosition::Excluded { .. } => {
                    driver_to_accounting.push(accounting_index);
                    after_excluded = true;
                }
                ContractRequestPosition::ContextRoot { node_id } if after_excluded => {
                    return Err(PackageRequestError::OrphanContextRoot {
                        accounting_index,
                        node_id: node_id.clone(),
                    });
                }
                ContractRequestPosition::ContextRoot { .. } => {}
            }
        }
        Ok(Self {
            package,
            positions,
            driver_to_accounting,
            bound_driver_indices,
        })
    }

    /// Number of positions with their own envelope item.
    pub(crate) fn bound_count(&self) -> usize {
        self.bound_driver_indices.len()
    }

    /// Driver index associated with one FR-331 envelope item.
    pub(crate) fn bound_driver_index(&self, envelope_index: usize) -> Option<usize> {
        self.bound_driver_indices.get(envelope_index).copied()
    }

    /// Original driver indexes for FR-331 items in envelope order.
    pub(crate) fn bound_driver_indices(&self) -> &[usize] {
        &self.bound_driver_indices
    }

    /// Exact key when this original driver position is accounting-only.
    pub(crate) fn excluded_node(&self, request_index: usize) -> Option<&'a CheckedNodeId> {
        let accounting_index = *self.driver_to_accounting.get(request_index)?;
        match self.positions.get(accounting_index)? {
            ContractRequestPosition::Excluded { node_id } => Some(node_id),
            ContractRequestPosition::DriverItem { .. }
            | ContractRequestPosition::ContextRoot { .. } => None,
        }
    }

    /// Exact key of one driver request, independently of context-root order.
    pub(crate) fn driver_node(&self, request_index: usize) -> Option<&'a CheckedNodeId> {
        let accounting_index = *self.driver_to_accounting.get(request_index)?;
        self.package
            .wire()
            .feature_accounting
            .get(accounting_index)
            .map(ContractFeatureRecordV1::node_id)
    }

    /// Accounting slot for one bound driver; context roots may occupy slots
    /// between drivers, so the driver index is not the accounting index.
    pub(crate) fn accounting_index(
        &self,
        request_index: usize,
    ) -> Result<usize, PackageRequestError> {
        self.bound_driver_node(request_index)?;
        self.driver_to_accounting
            .get(request_index)
            .copied()
            .ok_or(PackageRequestError::Position { request_index })
    }

    /// Exact key only for a bound driver position; refuse excluded indexes before generation.
    pub(crate) fn bound_driver_node(
        &self,
        request_index: usize,
    ) -> Result<&'a CheckedNodeId, PackageRequestError> {
        if let Some(node_id) = self.excluded_node(request_index) {
            return Err(PackageRequestError::ExcludedDriverIndex {
                request_index,
                node_id: node_id.clone(),
            });
        }
        self.driver_node(request_index)
            .ok_or(PackageRequestError::Position { request_index })
    }

    /// Caller-authored result selection for one driver position.
    pub(crate) fn result_selection(
        &self,
        request_index: usize,
    ) -> Option<&'a ContractDriverResultSelection> {
        let accounting_index = *self.driver_to_accounting.get(request_index)?;
        match self.positions.get(accounting_index)? {
            ContractRequestPosition::DriverItem {
                result_selection, ..
            } => Some(result_selection),
            ContractRequestPosition::Excluded { .. }
            | ContractRequestPosition::ContextRoot { .. } => None,
        }
    }

    /// Whether the selected conversion root belongs to this driver's
    /// contiguous context span, before the next driver position.
    pub(crate) fn has_context_root(&self, request_index: usize, node_id: &CheckedNodeId) -> bool {
        let Some(accounting_index) = self.driver_to_accounting.get(request_index).copied() else {
            return false;
        };
        self.positions[accounting_index + 1..]
            .iter()
            .take_while(|position| matches!(position, ContractRequestPosition::ContextRoot { .. }))
            .any(|position| position.node_id() == node_id)
    }

    /// Return the exact admitted outcome at one request position.
    pub(crate) fn at(
        &self,
        request_index: usize,
        node_id: &CheckedNodeId,
    ) -> Result<PackageItem<'a>, PackageRequestError> {
        self.bound_driver_node(request_index)?;
        let accounting_index = *self
            .driver_to_accounting
            .get(request_index)
            .ok_or(PackageRequestError::Position { request_index })?;
        let wire = self.package.wire();
        let record = wire
            .feature_accounting
            .get(accounting_index)
            .ok_or(PackageRequestError::Position { request_index })?;
        if record.node_id() != node_id {
            return Err(PackageRequestError::Identity {
                request_index,
                requested: node_id.clone(),
                accounted: record.node_id().clone(),
            });
        }
        // The sole IR reader guarantees source-map alignment with accounting.
        let occurrences = wire.source_map.requested_items[accounting_index]
            .occurrences
            .as_slice();
        match record {
            ContractFeatureRecordV1::Lowered { node_id } => {
                let view = self.package.reached_closure(node_id).map_err(|cause| {
                    PackageRequestError::ReachedClosure {
                        request_index,
                        node_id: node_id.clone(),
                        cause: Box::new(cause),
                    }
                })?;
                let ContractRootViewV1::Lowered(closure) = view else {
                    return Err(PackageRequestError::ReachedClosure {
                        request_index,
                        node_id: node_id.clone(),
                        cause: Box::new(ContractRootViewError::Inconsistent),
                    });
                };
                Ok(PackageItem::Lowered {
                    record,
                    closure,
                    occurrences,
                })
            }
            ContractFeatureRecordV1::Unsupported { .. }
            | ContractFeatureRecordV1::RequiresBound { .. }
            | ContractFeatureRecordV1::InvalidInput { .. }
            | ContractFeatureRecordV1::Failed { .. }
            | ContractFeatureRecordV1::InvalidBody { .. }
            | ContractFeatureRecordV1::BodyIncomplete { .. } => Ok(PackageItem::Refused {
                record,
                occurrences,
            }),
        }
    }
}

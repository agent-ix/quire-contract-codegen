//! The driver's typed projection of QSL's per-occurrence requirement records (FR-022, IR-304).
//!
//! One application node can carry several QSL occurrences, each with its own result bound and
//! path condition, so a node id alone cannot say which claim an item makes. These are CG-owned
//! types: the driver projects QSL's checked values into them at the boundary, and CG takes no
//! QSL library dependency for them. Node ids are in the authenticated V2 source frame.

use std::collections::BTreeMap;

use quire_contract_model::{CheckedNodeId, CheckedOccurrenceRole, ContractOccurrenceKeyV1};
use serde::Serialize;

/// CG's key for one QSL `OccurrenceKey`, field for field the IR `ContractOccurrenceKeyV1`.
// Implements: FR-022
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RequirementOccurrence {
    /// The node, in the authenticated V2 source frame.
    pub node_id: CheckedNodeId,
    /// The source-map occurrence role.
    pub role: CheckedOccurrenceRole,
    /// The source-map occurrence ordinal.
    pub ordinal: u64,
}

impl From<&ContractOccurrenceKeyV1> for RequirementOccurrence {
    fn from(key: &ContractOccurrenceKeyV1) -> Self {
        Self {
            node_id: key.node_id.clone(),
            role: key.role.clone(),
            ordinal: key.ordinal,
        }
    }
}

/// One QSL `PathGuard`: the guard's `expression` occurrence and its required outcome.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RequirementGuard {
    /// The guard expression's occurrence.
    pub occurrence: RequirementOccurrence,
    /// The outcome the path requires of the guard.
    pub holds: bool,
}

/// The projection of one QSL `RequirementRecord`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequirementRecordInput {
    /// QSL's `ResultBound` type node: the semantic type of the selected narrow, or of the
    /// application when there is none.
    pub result_bound: CheckedNodeId,
    /// The direct conversion occurrence, when the result is narrowed.
    pub conversion_context: Option<RequirementOccurrence>,
    /// The path condition, outermost guard first.
    pub path_condition: Vec<RequirementGuard>,
}

/// The requirement records of one checked package, keyed by occurrence. One key has at most one
/// record by construction.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RequirementRecords(BTreeMap<RequirementOccurrence, RequirementRecordInput>);

impl RequirementRecords {
    /// The records of a whole checked package.
    #[must_use]
    pub const fn new(records: BTreeMap<RequirementOccurrence, RequirementRecordInput>) -> Self {
        Self(records)
    }

    /// The record at exactly `occurrence`.
    pub(crate) fn get(
        &self,
        occurrence: &RequirementOccurrence,
    ) -> Option<&RequirementRecordInput> {
        self.0.get(occurrence)
    }

    /// The occurrences that hold a record.
    pub(crate) fn occurrences(&self) -> impl Iterator<Item = &RequirementOccurrence> {
        self.0.keys()
    }
}

/// The projection of the QSL `RequirementItem` at one request index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutedRequirement {
    /// The item's occurrence key.
    pub occurrence: RequirementOccurrence,
    /// The result-bound type node.
    pub result_bound: CheckedNodeId,
    /// The direct conversion occurrence, when the result is narrowed.
    pub conversion_context: Option<RequirementOccurrence>,
    /// The path condition, outermost guard first.
    pub path_condition: Vec<RequirementGuard>,
}

/// How a routed item's requirement or its keyed record disagrees with the authenticated
/// selection (FR-022 "Per-occurrence requirement records").
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequirementRecordMismatch {
    /// An `OwnResult` item carries no `RoutedRequirement`.
    RequirementMissing,
    /// A `NotApplicable` item has records for its node or carries a `RoutedRequirement`.
    OccurrenceNotSelected,
    /// `requirement_records` has no entry at the item's occurrence.
    RecordMissing,
    /// The item's occurrence is not the authenticated `OwnResult.application` occurrence.
    ApplicationMismatch {
        /// The authenticated application occurrence.
        selected: RequirementOccurrence,
    },
    /// The result-bound nodes of the item, the record and the admitted selection disagree.
    ResultBoundMismatch {
        /// The item's result bound.
        item: CheckedNodeId,
        /// The record's result bound.
        record: CheckedNodeId,
        /// The admitted selection's result bound, translated to the V2 frame, when it has one.
        selected: Option<CheckedNodeId>,
    },
    /// The item's, record's and authenticated conversion occurrences are not all equal.
    ConversionContextMismatch {
        /// The item's conversion context.
        item: Option<RequirementOccurrence>,
        /// The record's conversion context.
        record: Option<RequirementOccurrence>,
        /// The authenticated conversion occurrence.
        selected: Option<RequirementOccurrence>,
    },
    /// The two guard sequences differ in length, order, any guard occurrence or any `holds`.
    PathConditionMismatch {
        /// The item's path condition.
        item: Vec<RequirementGuard>,
        /// The record's path condition.
        record: Vec<RequirementGuard>,
    },
}

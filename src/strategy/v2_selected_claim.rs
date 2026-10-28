// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! V2 strategy claim selection through the admitted package (FR-011-AC-13).

use quire_contract_model::{
    CheckedExecutionPointError, CheckedNodeId, CheckedOccurrence, CheckedPackageV2,
    ExecutionPoint,
};

/// Complete typed identity of one claim in an admitted checked package.
#[derive(Clone, Copy, Debug)]
pub struct V2SelectedClaimKey<'a> {
    /// The checked state-clause node to select.
    pub clause: &'a CheckedNodeId,
    /// The node that owns the claim occurrence.
    pub claim_owner: &'a CheckedNodeId,
    /// The authentic claim occurrence on that node.
    pub occurrence: &'a CheckedOccurrence,
}

/// A V2 strategy selection refusal, retaining IR's exact typed cause and key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct V2SelectedClaimDiagnostic {
    /// IR's refusal, including the supplied clause, owner, and occurrence.
    pub refusal: CheckedExecutionPointError,
}

/// Select one claim's exact admitted execution point without rendering an artifact.
///
/// A failed selection has no artifact to publish. The point comes only from the
/// admitted package's clause-context accessor.
// Implements: FR-011-AC-13
pub fn select_v2_claim_execution_point(
    package: &CheckedPackageV2,
    key: V2SelectedClaimKey<'_>,
) -> Result<ExecutionPoint, V2SelectedClaimDiagnostic> {
    package
        .clause_execution_point(key.clause, (key.claim_owner, key.occurrence))
        .map_err(|refusal| V2SelectedClaimDiagnostic { refusal })
}

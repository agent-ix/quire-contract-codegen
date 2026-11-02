// SPDX-License-Identifier: AGPL-3.0-or-later
//! Identity and generated artifacts of a Boolean field-transition proposition.

use super::ProofCeilings;
use crate::{
    core::{artifact::Artifact, identity::HarnessPath},
    kani::abi::KaniSolver,
};
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrence, CheckedSourceMapEntry, ContractSourceV1,
};
use serde::Serialize;

/// The admitted Boolean field-transition contract and its exact implementation binding.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StateTransitionIdentity {
    /// Resource ceilings used by the bounded executor.
    pub ceilings: ProofCeilings,
    /// The admitted package name and complete source/dependency lock.
    pub package: ContractSourceV1,
    /// Selected canonical `state/transition` node.
    pub transition: CheckedNodeId,
    /// Complete selected transition occurrence keys, without synthesized ordinals.
    pub occurrences: Vec<CheckedOccurrence>,
    /// Every source correspondence for the selected transition.
    pub source_map: Vec<CheckedSourceMapEntry>,
    /// Digest of the admitted semantic graph and source material.
    pub semantic_digest: String,
    /// The operation anchor named by the canonical transition body.
    pub anchor: CheckedNodeId,
    /// Canonical selected field projection.
    pub subject: CheckedNodeId,
    /// The single Boolean operation parameter in declaration order.
    pub input: CheckedNodeId,
    /// The admitted precondition clause.
    pub precondition: CheckedNodeId,
    /// The admitted postcondition clause.
    pub postcondition: CheckedNodeId,
    /// Model declaration owning the selected field.
    pub declaring_node: CheckedNodeId,
    /// Canonical declared field name.
    pub field: String,
    /// Canonical declared Boolean parameter name.
    pub parameter: String,
    /// Actual Rust `fn(bool, &mut bool)` implementation.
    pub subject_path: String,
    /// Generated module holding this proposition.
    pub module_symbol: crate::core::identity::ModuleSymbol,
    /// Generated proof function.
    pub harness_symbol: crate::core::identity::HarnessSymbol,
    /// Selected backend solver.
    pub solver: KaniSolver,
    /// Loop unwind ceiling.
    pub unwind: u32,
    /// Exact backend option vector.
    pub options: Vec<String>,
}

impl StateTransitionIdentity {
    /// The qualified proof function selected in backend execution.
    #[must_use]
    pub fn harness_path(&self) -> HarnessPath {
        HarnessPath {
            module: self.module_symbol.clone(),
            harness: self.harness_symbol.clone(),
        }
    }
}

/// A generated field-transition proof and its persisted proposition identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniStateTransitionHarness {
    /// The admitted proposition and actual subject binding.
    pub identity: StateTransitionIdentity,
    /// Generated Rust consumed by the bounded executor.
    pub rust: Artifact,
    /// Persisted identity and generated source path.
    pub record: Artifact,
}

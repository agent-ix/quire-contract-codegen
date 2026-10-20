//! V2 state-clause condition oracles over admitted checked packages.

use quire_contract_model::{CheckedNodeId, CheckedPackageV2};

use super::scalar::OperationProvenance;

/// A state field value supplied to one generated clause oracle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClauseStateInput {
    /// The generated parameter name.
    pub name: String,
    /// Inclusive lower bound from the admitted model member.
    pub lower: i128,
    /// Inclusive upper bound from the admitted model member.
    pub upper: i128,
}

/// One generated oracle of a V2 clause's Boolean condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct V2ClauseOracle {
    /// Exact checked clause node.
    pub node_id: CheckedNodeId,
    /// The operation's checked provenance.
    pub provenance: OperationProvenance,
    /// Self-contained generated Rust source.
    pub source: String,
    /// Ordered field values expected by the generated function.
    pub inputs: Vec<ClauseStateInput>,
}

/// Why a selected clause condition could not be generated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum V2ClauseRefusal {
    /// The checked clause's body is outside this generator's supported shape.
    UnsupportedCondition,
}

/// Generate a checked V2 clause's Boolean condition oracle.
pub fn generate_v2_clause_oracle(
    _package: &CheckedPackageV2,
    _node_id: &CheckedNodeId,
) -> Result<V2ClauseOracle, V2ClauseRefusal> {
    Err(V2ClauseRefusal::UnsupportedCondition)
}

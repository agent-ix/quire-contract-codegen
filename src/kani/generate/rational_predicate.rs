//! Physical I64 positions for one prepared Rational predicate's logical arguments.

use quire_contract_model::CheckedNodeId;
use serde::Serialize;

use crate::{kani::abi::KaniPrimitiveType, oracle::rational_predicate::RationalPredicateOracle};

/// The semantic role of one physical Rational component position.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RationalComponent {
    /// Canonical reduced numerator.
    Numerator,
    /// Canonical positive denominator.
    Denominator,
}

/// One physical draw, grouped to its single real source parameter.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RationalComponentBinding {
    /// Distinct generated draw identifier, not a new parameter node id.
    pub identifier: String,
    /// Original logical parameter node id.
    pub parameter: CheckedNodeId,
    /// Numerator or denominator role.
    pub component: RationalComponent,
    /// Physical call position across all arguments.
    pub position: usize,
    /// Existing primitive representation.
    pub primitive: KaniPrimitiveType,
    /// Original inclusive component minimum.
    pub minimum: i64,
    /// Original inclusive component maximum.
    pub maximum: i64,
}

/// Expand each original parameter as numerator then denominator without a proof artifact.
pub fn rational_predicate_bindings(
    oracle: &RationalPredicateOracle,
) -> Vec<RationalComponentBinding> {
    oracle
        .parameters()
        .iter()
        .enumerate()
        .flat_map(|(index, parameter)| {
            [
                (
                    RationalComponent::Numerator,
                    parameter.domain.numerator_minimum,
                    parameter.domain.numerator_maximum,
                ),
                (
                    RationalComponent::Denominator,
                    parameter.domain.denominator_minimum,
                    parameter.domain.denominator_maximum,
                ),
            ]
            .into_iter()
            .enumerate()
            .map(
                move |(component_index, (component, minimum, maximum))| RationalComponentBinding {
                    identifier: format!(
                        "argument_{index}_{}",
                        if component_index == 0 {
                            "numerator"
                        } else {
                            "denominator"
                        }
                    ),
                    parameter: parameter.node.clone(),
                    component,
                    position: index * 2 + component_index,
                    primitive: KaniPrimitiveType::I64,
                    minimum,
                    maximum,
                },
            )
        })
        .collect()
}

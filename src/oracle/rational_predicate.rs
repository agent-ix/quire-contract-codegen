//! Whole-function Rational predicate native source preparation (FR-046).

use quire_contract_model::{CheckedNodeId, CheckedSemanticId, CheckedSourceMapEntry};
use serde::Serialize;

use crate::kani::generate::outcome::FunctionPredicateItem;

/// Original finite inclusive component domains for one Rational parameter.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RationalPredicateDomain {
    /// Reduced numerator lower bound.
    pub numerator_minimum: i64,
    /// Reduced numerator upper bound.
    pub numerator_maximum: i64,
    /// Positive denominator lower bound.
    pub denominator_minimum: i64,
    /// Positive denominator upper bound.
    pub denominator_maximum: i64,
}

/// One real function parameter, in ascending original node-id order.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RationalPredicateParameter {
    /// Owner-issued parameter identity.
    pub node: CheckedNodeId,
    /// Original authored name.
    pub name: String,
    /// Original inclusive domain, without clipping or normalization duplicates.
    pub domain: RationalPredicateDomain,
}

/// A native oracle prepared from a complete admitted function closure.
///
/// This is not a Kani harness or a claim that public replay is implemented.
#[derive(Clone, Debug)]
pub struct RationalPredicateOracle {
    /// Owner-admitted package identity.
    pub package: CheckedSemanticId,
    /// Original selected function identity.
    pub function: CheckedNodeId,
    /// Original source correspondence for the reached closure.
    pub sources: Vec<CheckedSourceMapEntry>,
    /// Native Rust source over the public exact-value kernel.
    pub source: String,
    parameters: Vec<RationalPredicateParameter>,
}

impl RationalPredicateOracle {
    /// Logical parameters in ascending original node-id order.
    pub fn parameters(&self) -> &[RationalPredicateParameter] {
        &self.parameters
    }
}

/// A preparation refusal; no proof artifact is emitted for a refused item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RationalPredicateRefusal {
    /// The selected root has no admitted lowered function closure.
    SelectionUnavailable {
        /// Requested original node.
        node: CheckedNodeId,
    },
    /// The body uses an operation outside this exact predicate slice.
    UnsupportedOperation {
        /// Original reached node.
        node: CheckedNodeId,
        /// Original operation or semantic form.
        operation: String,
    },
    /// A Rational parameter has no finite domain.
    RequiresBound {
        /// Original parameter needing the bound.
        parameter: CheckedNodeId,
    },
    /// A finite endpoint cannot be represented by this component ABI.
    UnsupportedEndpoint {
        /// Original parameter.
        parameter: CheckedNodeId,
        /// Named original component endpoint.
        component: String,
        /// Exact original endpoint, not a narrowed substitute.
        value: String,
    },
    /// A reached typed shape cannot be represented by this renderer.
    UnsupportedShape {
        /// Original reached node.
        node: CheckedNodeId,
    },
    /// Preparation exceeded its existing source/work bound.
    PreparationLimit,
}

/// Prepare native source for the complete selected Boolean function predicate.
///
/// # Errors
///
/// Returns a source-associated refusal when the original closure or exact domain
/// is not representable. This entry does not negotiate or run a Kani proof.
pub fn generate_rational_predicate_oracle(
    item: FunctionPredicateItem<'_>,
) -> Result<RationalPredicateOracle, RationalPredicateRefusal> {
    Err(RationalPredicateRefusal::UnsupportedOperation {
        node: item.function.clone(),
        operation: "whole-function predicate preparation".into(),
    })
}

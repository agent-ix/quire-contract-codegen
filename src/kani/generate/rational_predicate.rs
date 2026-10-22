//! Physical I64 positions for one prepared Rational predicate's logical arguments.

use quire_contract_model::CheckedNodeId;
use serde::{Deserialize, Serialize};

use crate::{kani::abi::KaniPrimitiveType, oracle::rational_predicate::RationalPredicateOracle};

/// The semantic role of one physical Rational component position.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RationalComponent {
    /// Canonical reduced numerator.
    Numerator,
    /// Canonical positive denominator.
    Denominator,
}

/// One physical draw, grouped to its single real source parameter.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

/// A physical component candidate that does not encode one admitted logical argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RationalComponentRefusal {
    /// The number of component pairs differs from the original function's parameters.
    ArgumentCount,
    /// A component is outside its original declared domain.
    OutsideDomain {
        /// Original logical parameter.
        parameter: CheckedNodeId,
        /// Rejected component role.
        component: RationalComponent,
    },
    /// Owner construction would normalize the pair into a different representation.
    NonCanonical {
        /// Original logical parameter.
        parameter: CheckedNodeId,
    },
}

/// Admit exactly one physical pair per original parameter using the owner's Rational constructor.
///
/// # Errors
///
/// Refuses missing arguments, out-of-domain components and noncanonical candidates.
/// This is representation admission for prepared arguments, not backend witness decoding.
pub fn canonical_rational_arguments(
    oracle: &RationalPredicateOracle,
    components: &[[i64; 2]],
) -> Result<Vec<quire_exact::Rational>, RationalComponentRefusal> {
    use quire_exact::{Integer, Rational};
    if components.len() != oracle.parameters().len() {
        return Err(RationalComponentRefusal::ArgumentCount);
    }
    oracle
        .parameters()
        .iter()
        .zip(components)
        .map(|(parameter, pair)| {
            let domain = &parameter.domain;
            for (component, value, minimum, maximum) in [
                (
                    RationalComponent::Numerator,
                    pair[0],
                    domain.numerator_minimum,
                    domain.numerator_maximum,
                ),
                (
                    RationalComponent::Denominator,
                    pair[1],
                    domain.denominator_minimum,
                    domain.denominator_maximum,
                ),
            ] {
                if !(minimum..=maximum).contains(&value) {
                    return Err(RationalComponentRefusal::OutsideDomain {
                        parameter: parameter.node.clone(),
                        component,
                    });
                }
            }
            let numerator = Integer::from(pair[0]);
            let denominator = Integer::from(pair[1]);
            let value = Rational::new(numerator.clone(), denominator.clone()).map_err(|_| {
                RationalComponentRefusal::NonCanonical {
                    parameter: parameter.node.clone(),
                }
            })?;
            if value.numerator() != &numerator || value.denominator() != &denominator {
                return Err(RationalComponentRefusal::NonCanonical {
                    parameter: parameter.node.clone(),
                });
            }
            Ok(value)
        })
        .collect()
}

/// Real owner declaration occurrence retained as metadata, without manufacturing an occurrence.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RationalDeclarationBinding {
    /// Actual owner declaration node spelling.
    pub node: String,
    /// Actual owner occurrence role.
    pub role: String,
    /// Actual owner occurrence ordinal.
    pub ordinal: u64,
}

/// One original logical argument and its mapping to the body oracle's node-ordered input slice.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RationalLogicalBinding {
    /// Original parameter identifier, defining canonical logical order.
    pub identifier: String,
    /// Original owner parameter node.
    pub parameter: CheckedNodeId,
    /// Position in the existing body oracle's node-ordered argument slice.
    pub oracle_position: usize,
    /// Original inclusive numerator lower endpoint, canonical exact decimal.
    pub numerator_minimum: String,
    /// Original inclusive numerator upper endpoint, canonical exact decimal.
    pub numerator_maximum: String,
    /// Original inclusive denominator lower endpoint, canonical exact decimal.
    pub denominator_minimum: String,
    /// Original inclusive denominator upper endpoint, canonical exact decimal.
    pub denominator_maximum: String,
}

/// Retainable native input schema. A supplied copy is untrusted until compared to its codec.
/// This schema supplies no generated-harness, backend-playback or solver provenance.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RationalPredicateSchema {
    /// Actual admitted ContractPackage identity.
    pub package: quire_contract_model::CheckedSemanticId,
    /// Actual checked source identity and byte lock.
    pub source: quire_contract_model::ContractSourceV1,
    /// Actual selected function.
    pub function: CheckedNodeId,
    /// Actual owner declaration occurrence.
    pub declaration: RationalDeclarationBinding,
    /// Existing O09 obligation identity, not a new schema digest.
    pub obligation: String,
    /// Complete logical arguments in identifier order.
    pub logical: Vec<RationalLogicalBinding>,
    /// Complete numerator-then-denominator primitive call order.
    pub physical: Vec<RationalComponentBinding>,
}

/// Derive the only expected schema from the immutable validated source/domain context.
pub(crate) fn native_predicate_schema(
    context: &crate::replay::function::ValidatedRationalPredicateContext,
) -> RationalPredicateSchema {
    let oracle = context.oracle();
    let mut ordered = oracle.parameters().iter().enumerate().collect::<Vec<_>>();
    ordered.sort_by(|(_, left), (_, right)| left.name.cmp(&right.name));
    let logical = ordered
        .iter()
        .map(|(position, parameter)| {
            let domain = &parameter.domain;
            RationalLogicalBinding {
                identifier: parameter.name.clone(),
                parameter: parameter.node.clone(),
                oracle_position: *position,
                numerator_minimum: domain.numerator_minimum.to_string(),
                numerator_maximum: domain.numerator_maximum.to_string(),
                denominator_minimum: domain.denominator_minimum.to_string(),
                denominator_maximum: domain.denominator_maximum.to_string(),
            }
        })
        .collect::<Vec<_>>();
    let order = logical
        .iter()
        .enumerate()
        .map(|(i, p)| (&p.parameter, i))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut physical = rational_predicate_bindings(oracle);
    physical.sort_by_key(|binding| {
        (
            order[&binding.parameter],
            match binding.component {
                RationalComponent::Numerator => 0,
                RationalComponent::Denominator => 1,
            },
        )
    });
    for (position, binding) in physical.iter_mut().enumerate() {
        binding.position = position;
    }
    let occurrence = context.declaration();
    RationalPredicateSchema {
        package: oracle.package.clone(),
        source: context.source().clone(),
        function: oracle.function.clone(),
        declaration: RationalDeclarationBinding {
            node: occurrence.node().to_string(),
            role: occurrence.origin().role().to_string(),
            ordinal: occurrence.origin().ordinal(),
        },
        obligation: context.obligation().to_string(),
        logical,
        physical,
    }
}

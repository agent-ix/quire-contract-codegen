//! Decode native physical inputs against immutable admitted Rational metadata.
//! Actual harness-artifact provenance and solver-origin qualification are NOT supplied here.

use qsl_replay::WitnessValue;
use quire_contract_model::CheckedNodeId;
use quire_exact::Rational;

use crate::{
    kani::generate::rational_predicate::{
        native_predicate_schema, RationalComponent, RationalPredicateSchema,
    },
    replay::{function::ValidatedRationalPredicateContext, witness::WitnessDecodeLimits},
    DecodeFailure,
};

/// Native source/domain metadata authority. Public retained schema copies cannot change it.
#[derive(Clone, Debug)]
pub struct RationalPredicateCodec {
    context: ValidatedRationalPredicateContext,
    schema: RationalPredicateSchema,
}

/// Why native input decoding produced no values. No branch is a semantic contract failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RationalPredicateDecodeError {
    /// Retained metadata differs from its actual admitted source/domain authority.
    SchemaMismatch,
    /// The existing primitive playback decoder refused the selected block.
    Decode(DecodeFailure),
    /// A decoded component is outside its original inclusive domain.
    OutsideDomain {
        /// Original real parameter.
        parameter: CheckedNodeId,
        /// Original component role.
        component: RationalComponent,
    },
    /// The owner's constructor rejects or normalizes the original component pair.
    NonCanonical {
        /// Original real parameter.
        parameter: CheckedNodeId,
    },
}
impl std::fmt::Display for RationalPredicateDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for RationalPredicateDecodeError {}

/// Decoded native inputs and selected assertion text, not an authenticated solver counterexample.
#[derive(Clone, Eq, PartialEq)]
pub struct RationalPredicateDecodedInputs<'a> {
    /// One canonical owner Rational per original logical parameter, in identifier order.
    pub values: Vec<(String, WitnessValue)>,
    /// The same values mapped to the existing body oracle's node-ordered slice.
    pub oracle_arguments: Vec<Rational>,
    /// Selected raw block's decoded assertion text.
    pub check_text: String,
    /// Selected raw block's verbatim assertion clause (including its original quotes).
    pub check_clause: &'a str,
}

// Payloads and the retained raw clause require explicit typed-field access.
impl std::fmt::Debug for RationalPredicateDecodedInputs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RationalPredicateDecodedInputs")
            .field("logical_values", &self.values.len())
            .field("oracle_arguments", &self.oracle_arguments.len())
            .field("check_text_bytes", &self.check_text.len())
            .field("check_clause_bytes", &self.check_clause.len())
            .finish()
    }
}

impl RationalPredicateCodec {
    pub(crate) fn from_context(context: ValidatedRationalPredicateContext) -> Self {
        let schema = native_predicate_schema(&context);
        Self { context, schema }
    }

    /// Read-only native schema; a caller's cloned/loaded copy is never authoritative.
    #[must_use]
    pub fn schema(&self) -> &RationalPredicateSchema {
        &self.schema
    }

    /// Decode inputs from a caller-selected assertion block using the required physical budgets.
    ///
    /// The selection names parser input only: no actual generated artifact, Kani launch,
    /// solver playback or proof verdict is authenticated by this native prerequisite.
    ///
    /// # Errors
    /// Refuses schema mismatch, malformed/over-budget primitive entries, out-of-domain
    /// components and noncanonical constructor/accessor roundtrips without replaying anything.
    pub fn decode_inputs<'a>(
        &self,
        schema: &RationalPredicateSchema,
        harness_symbol: &str,
        module_symbol: &str,
        transcript: &'a str,
        limits: WitnessDecodeLimits,
    ) -> Result<RationalPredicateDecodedInputs<'a>, RationalPredicateDecodeError> {
        let failure = |code| {
            RationalPredicateDecodeError::Decode(DecodeFailure::new(
                code,
                harness_symbol,
                module_symbol,
            ))
        };
        if transcript.len() > limits.bytes {
            return Err(RationalPredicateDecodeError::Decode(DecodeFailure::new(
                "cg_witness_decode_bytes_limit",
                harness_symbol,
                &format!("{module_symbol};bytes={}", limits.bytes),
            )));
        }
        if schema != &self.schema {
            return Err(RationalPredicateDecodeError::SchemaMismatch);
        }
        // Refuse before allocating the owner-sized primitive view when the required physical
        // budget cannot represent this selection. Grouping never hides two values as one.
        if self.schema.physical.len() > limits.values {
            return Err(RationalPredicateDecodeError::Decode(DecodeFailure::new(
                "cg_witness_decode_values_limit",
                harness_symbol,
                &format!("{module_symbol};values={}", limits.values),
            )));
        }
        let primitive = self
            .schema
            .physical
            .iter()
            .map(|binding| crate::kani::identity::ObligationBinding {
                identifier: binding.identifier.clone(),
                role: crate::kani::abi::KaniBindingRole::Argument,
                primitive_type: binding.primitive,
                // Only a temporary primitive view: no invented integer IR domain/dependency.
                // The original Rational components are checked from the immutable native schema.
                integer_bounds: None,
                dependencies: Vec::new(),
            })
            .collect::<Vec<_>>();
        let decoded = crate::replay::witness::decode_playback(
            harness_symbol,
            module_symbol,
            &primitive,
            transcript,
            limits,
        )
        .map_err(RationalPredicateDecodeError::Decode)?;
        let mut values = Vec::with_capacity(self.schema.logical.len());
        let mut oracle_arguments = std::iter::repeat_with(|| None)
            .take(self.context.oracle().parameters().len())
            .collect::<Vec<Option<Rational>>>();
        for (logical_index, logical) in self.schema.logical.iter().enumerate() {
            let position = logical_index * 2;
            let mut components = [0_i64; 2];
            for (offset, component) in components.iter_mut().enumerate() {
                let (_, WitnessValue::Integer(value)) = decoded
                    .values
                    .get(position + offset)
                    .ok_or_else(|| failure("kani_witness_arity_mismatch"))?
                else {
                    return Err(failure("kani_witness_width_mismatch"));
                };
                *component =
                    i64::try_from(*value).map_err(|_| failure("kani_witness_width_mismatch"))?;
            }
            let numerator = quire_exact::Integer::from(components[0]);
            let denominator = quire_exact::Integer::from(components[1]);
            // Malformed representation is never normalized into an alternative valid witness.
            let rational = Rational::new(numerator.clone(), denominator.clone()).map_err(|_| {
                RationalPredicateDecodeError::NonCanonical {
                    parameter: logical.parameter.clone(),
                }
            })?;
            if rational.numerator() != &numerator || rational.denominator() != &denominator {
                return Err(RationalPredicateDecodeError::NonCanonical {
                    parameter: logical.parameter.clone(),
                });
            }
            for (offset, value) in components.iter().enumerate() {
                let binding = self
                    .schema
                    .physical
                    .get(position + offset)
                    .ok_or(RationalPredicateDecodeError::SchemaMismatch)?;
                if !(binding.minimum..=binding.maximum).contains(value) {
                    return Err(RationalPredicateDecodeError::OutsideDomain {
                        parameter: logical.parameter.clone(),
                        component: binding.component,
                    });
                }
            }
            let slot = oracle_arguments
                .get_mut(logical.oracle_position)
                .ok_or(RationalPredicateDecodeError::SchemaMismatch)?;
            if slot.replace(rational).is_some() {
                return Err(RationalPredicateDecodeError::SchemaMismatch);
            }
            values.push((
                logical.identifier.clone(),
                WitnessValue::Rational {
                    numerator,
                    denominator,
                },
            ));
        }
        Ok(RationalPredicateDecodedInputs {
            values,
            oracle_arguments: oracle_arguments
                .into_iter()
                .map(|value| value.ok_or(RationalPredicateDecodeError::SchemaMismatch))
                .collect::<Result<Vec<_>, _>>()?,
            check_text: decoded.check_text,
            check_clause: decoded.check_clause,
        })
    }
}

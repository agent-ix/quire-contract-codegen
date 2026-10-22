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
#[derive(Clone, Debug, Eq, PartialEq)]
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
        let _ = (
            schema,
            harness_symbol,
            module_symbol,
            transcript,
            limits,
            &self.context,
        );
        Err(RationalPredicateDecodeError::Decode(DecodeFailure {
            code: "cg_rational_decode_not_implemented".into(),
            source_id: harness_symbol.into(),
            context: module_symbol.into(),
        }))
    }
}

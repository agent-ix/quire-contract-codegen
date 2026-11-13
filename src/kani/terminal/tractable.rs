//! Pre-launch tractability refusal for the general `rt::Integer` scalar route (FR-056, IR-1530).
//!
//! The measured table of FR-056 records the general integer scalar harness as not tractable
//! (`docs/qualification/ir1530-general-route-scalar-not-tractable.md`). Generation keeps emitting
//! that harness, because host tests exercise it; the refusal is made here, before any Kani process
//! starts, through the one [`TractabilityDecider`] seam and the one
//! [`UnsupportedObligation::V2ClauseNotTractable`] arm. The wall-clock ceiling stays the backstop
//! for everything the decider lets through.

use crate::{
    kani::{
        generate::outcome::UnsupportedObligation,
        identity::ScalarIntegerRepresentation,
        run::{
            execute::{
                execute_kani_obligation, execute_kani_obligations, KaniExecutionEvidence,
                KaniExecutionRefusal, KaniExecutionRequest, KaniGroupRun,
            },
            harness::KaniExecutableHarness,
        },
        tractability::{
            ClauseFeatures, Decision, InputBound, OperationWidth, TractabilityDecider,
            TractabilityRoute,
        },
    },
    oracle::clause::interval_bits,
};

/// Why a decided run did not produce evidence.
#[derive(Debug)]
pub enum DecidedRunRefusal {
    /// The decider refused the harness before any Kani process started (FR-056-AC-14). The
    /// refusal is the one `UnsupportedObligation::V2ClauseNotTractable` arm, naming the node,
    /// operation id, width and the measured row with its versions.
    NotTractable {
        /// The generated artifact path.
        harness_path: String,
        /// The typed refusal.
        refusal: Box<UnsupportedObligation>,
    },
    /// The decider ran the harness and the runner refused it.
    Execution(KaniExecutionRefusal),
}

impl std::fmt::Display for DecidedRunRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotTractable {
                harness_path,
                refusal,
            } => match refusal.as_ref() {
                UnsupportedObligation::V2ClauseNotTractable {
                    operation_identity,
                    width_bits,
                    ..
                } => write!(
                    formatter,
                    "{harness_path} is a measured not-tractable Kani form: {operation_identity} at {width_bits} bits"
                ),
                other => write!(formatter, "{harness_path} is not tractable: {other:?}"),
            },
            Self::Execution(refusal) => write!(formatter, "{refusal}"),
        }
    }
}

impl std::error::Error for DecidedRunRefusal {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Execution(refusal) => Some(refusal),
            Self::NotTractable { .. } => None,
        }
    }
}

/// Decide one harness. Only a routed scalar harness on the general integer route is asked; every
/// other harness, and a native-route scalar, is `Ok` without consulting the decider.
///
/// # Errors
///
/// [`DecidedRunRefusal::NotTractable`] when the decider refuses.
pub fn refuse_if_not_tractable(
    harness: &KaniExecutableHarness<'_>,
    decider: &dyn TractabilityDecider,
) -> Result<(), DecidedRunRefusal> {
    let KaniExecutableHarness::Scalar(scalar) = harness else {
        return Ok(());
    };
    let identity = &scalar.identity;
    if identity.integer_representation != ScalarIntegerRepresentation::General {
        return Ok(());
    }
    let bits = identity
        .arguments
        .iter()
        .map(|argument| interval_bits(i128::from(argument.minimum), i128::from(argument.maximum)))
        .max();
    let features = ClauseFeatures::new(
        vec![OperationWidth {
            operation_id: identity.operation_identity.clone(),
            bits,
        }],
        identity
            .arguments
            .iter()
            .map(|argument| InputBound {
                lower: i128::from(argument.minimum),
                upper: i128::from(argument.maximum),
                boolean: false,
            })
            .collect(),
        None,
    )
    .on_route(TractabilityRoute::GeneralScalar);
    match decider.decide(&features) {
        Decision::Run => Ok(()),
        Decision::Refuse {
            reason,
            evidence_row,
        } => Err(DecidedRunRefusal::NotTractable {
            harness_path: scalar.rust.path.clone(),
            refusal: Box::new(UnsupportedObligation::V2ClauseNotTractable {
                node_id: identity.node_id.clone(),
                operation_identity: reason.operation_id,
                width_bits: reason.width_bits,
                row: evidence_row,
            }),
        }),
    }
}

/// [`execute_kani_obligation`] after the tractability decision: a harness the decider refuses is
/// never launched.
///
/// # Errors
///
/// [`DecidedRunRefusal`]: `NotTractable`, or the runner's own refusal from [`execute_kani_obligation`] refuses.
pub fn execute_kani_obligation_decided(
    request: &KaniExecutionRequest<'_>,
    decider: &dyn TractabilityDecider,
) -> Result<KaniExecutionEvidence, DecidedRunRefusal> {
    refuse_if_not_tractable(&request.harness, decider)?;
    execute_kani_obligation(request).map_err(DecidedRunRefusal::Execution)
}

/// [`execute_kani_obligations`] after the tractability decision: one refused member refuses the
/// call before any process starts, as an absent harness does.
///
/// # Errors
///
/// [`DecidedRunRefusal`]: `NotTractable`, or the runner's own refusal from [`execute_kani_obligations`] refuses.
pub fn execute_kani_obligations_decided(
    requests: &[KaniExecutionRequest<'_>],
    decider: &dyn TractabilityDecider,
) -> Result<Vec<KaniGroupRun>, DecidedRunRefusal> {
    for request in requests {
        refuse_if_not_tractable(&request.harness, decider)?;
    }
    execute_kani_obligations(requests).map_err(DecidedRunRefusal::Execution)
}

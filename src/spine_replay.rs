//! The replay adapter: a decoded Kani falsification goes to QSL's layer-6 `replay` facade, and
//! QSL's own verdict comes back.
//!
//! This module builds the backend-witness transcript in the grammar `qsl-replay` admits from the
//! typed values [`crate::decode_falsification`] returns, binds each value to its parameter by
//! the parameter's node id, and calls [`qsl_replay::replay`]. The executor recompiles the
//! request's digest-addressed source and evaluates the selected function itself, so the verdict
//! is QSL's, not a value this crate supplies.

use std::fmt;

use qsl_replay::{
    replay, MalformedTranscript, ReplayRefusal, ReplayRequestWire, ReplayResult, ReplaySource,
    Witness, WitnessArmResult,
};
use quire_contract_ir::kani::WitnessValue;

/// One parameter of the function the replay selects: the harness argument name a decoded value
/// carries, and the parameter's node id as 64 lowercase hex digits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayParameter<'a> {
    /// The harness argument identifier the decoded value is named by.
    pub argument: &'a str,
    /// The parameter's node id in the proved package.
    pub node_id: &'a str,
}

/// Why a falsification produced no replay verdict.
#[derive(Debug)]
pub enum SpineReplayError {
    /// A decoded value names an argument no [`ReplayParameter`] binds.
    UnboundArgument {
        /// The decoded value's name.
        argument: String,
    },
    /// The harness or check text would change the transcript's field boundaries.
    FieldDelimiter,
    /// The transcript this adapter built is not one `qsl-replay` admits.
    Transcript(MalformedTranscript),
    /// The QSL executor refused the request before settling a verdict.
    Refused(Box<ReplayRefusal>),
    /// A witness-sourced request settled on the input arm.
    WrongArm,
}

impl fmt::Display for SpineReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnboundArgument { argument } => {
                write!(
                    f,
                    "no replay parameter is bound to the harness argument `{argument}`"
                )
            }
            Self::FieldDelimiter => {
                f.write_str("the harness or check text contains a transcript delimiter")
            }
            Self::Transcript(cause) => write!(f, "the witness transcript is not admitted: {cause}"),
            Self::Refused(refusal) => write!(f, "the replay was refused: {refusal}"),
            Self::WrongArm => f.write_str("a witness-sourced request settled on the input arm"),
        }
    }
}

impl std::error::Error for SpineReplayError {}

impl From<MalformedTranscript> for SpineReplayError {
    fn from(cause: MalformedTranscript) -> Self {
        Self::Transcript(cause)
    }
}

/// Replays one decoded falsification through [`qsl_replay::replay`].
///
/// `harness` and `check_text` fill the transcript's harness and check fields; `values` are the
/// decoded arguments, and `parameters` bind each argument name to its node id. A Boolean value
/// is replayed as the integer `1` or `0`, the encoding QSL admits for a Boolean parameter.
/// `request` receives the witness source and returns the complete request, whose package
/// reference, selection and limits are the proving run's.
///
/// # Errors
///
/// [`SpineReplayError`] when a value has no parameter, the transcript is not admitted, or the
/// executor refuses the request.
pub fn replay_falsification(
    harness: &str,
    check_text: &str,
    values: &[(String, WitnessValue)],
    parameters: &[ReplayParameter<'_>],
    request: impl FnOnce(ReplaySource) -> ReplayRequestWire,
) -> Result<WitnessArmResult, SpineReplayError> {
    if [harness, check_text]
        .iter()
        .any(|field| field.contains(['|', '<', '>']))
    {
        return Err(SpineReplayError::FieldDelimiter);
    }
    let bindings = values
        .iter()
        .map(|(argument, value)| {
            let parameter = parameters
                .iter()
                .find(|parameter| parameter.argument == argument)
                .ok_or_else(|| SpineReplayError::UnboundArgument {
                    argument: argument.clone(),
                })?;
            let integer = match value {
                WitnessValue::Integer(integer) => *integer,
                WitnessValue::Boolean(boolean) => i64::from(*boolean),
            };
            Ok(format!("{}={integer}", parameter.node_id))
        })
        .collect::<Result<Vec<_>, SpineReplayError>>()?;
    let witness = Witness::parse(format!(
        "<<<assertion|{harness}|{check_text}|{}>>>",
        bindings.join(";")
    ))?;
    match replay(request(ReplaySource::Witness(witness))) {
        Ok(ReplayResult::Witness(result)) => Ok(result),
        Ok(ReplayResult::Input(_)) => Err(SpineReplayError::WrongArm),
        Err(refusal) => Err(SpineReplayError::Refused(Box::new(refusal))),
    }
}

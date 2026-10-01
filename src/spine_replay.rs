//! The replay adapter: a decoded Kani falsification goes to QSL's layer-6 `replay` facade, and
//! QSL's own verdict comes back.
//!
//! This module builds the backend-witness transcript in the grammar `qsl-replay` admits from the
//! typed values [`crate::decode_falsification`] returns, binds each value to its parameter by
//! the parameter's node id, and calls [`qsl_replay::replay`]. The executor recompiles the
//! request's digest-addressed source and evaluates the selected function itself, so the verdict
//! is QSL's, not a value this crate supplies.

use std::{collections::BTreeMap, fmt};

use qsl_replay::{
    call_site, replay, ByteDigest, CallSite, CallSiteRefusal, DependencyEntryWire, DigestDomain,
    DigestRecord, Identifier, MalformedTranscript, ProofCategory, QualifiedName, ReplayRefusal,
    ReplayRequestWire, ReplayResult, ReplaySource, ScalarLimits, SourceIdentity, StageLimits,
    StateEnvironment, Witness, WitnessArmResult, WitnessSettlement, WitnessValue,
};

use crate::{
    kani_obligations::KaniObligationIdentity,
    kani_witness_join::{decode_falsification, first_out_of_domain, DecodeFailure},
};

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

/// One source file of a proved package: the source reference its lock records and the bytes the
/// replay recompiles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LockedSource {
    /// The source reference's authority.
    pub authority: String,
    /// The source reference's identity.
    pub identity: String,
    /// The revision namespace.
    pub namespace: String,
    /// The revision.
    pub revision: String,
    /// The file's bytes; the request addresses them by digest.
    pub bytes: Vec<u8>,
}

impl LockedSource {
    fn digest(&self) -> DigestRecord {
        DigestRecord::mint(
            DigestDomain::SourceBytesV1,
            ByteDigest::of(&self.bytes).as_bytes(),
        )
    }

    fn wire(&self) -> (String, String, String, String, Option<String>, String) {
        (
            self.authority.clone(),
            self.identity.clone(),
            self.namespace.clone(),
            self.revision.clone(),
            Some(DigestDomain::SourceBytesV1.as_str().to_owned()),
            self.digest().hex(),
        )
    }
}

/// One entry of the proved package lock's `dependency_selections`, with the dependency's own lock
/// `sources`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyLock {
    /// The library identity.
    pub identity: String,
    /// The library version.
    pub version: String,
    /// The dependency's `package_id`, as the proving run recorded it.
    pub package_id: DigestRecord,
    /// The dependency's lock `sources`.
    pub sources: Vec<LockedSource>,
}

/// Everything about the proving run a replay request repeats.
#[derive(Clone, Debug)]
pub struct ReplayInputs {
    /// The proved unit's source.
    pub source: LockedSource,
    /// The proved package lock's dependency selections, each with its own sources.
    pub dependencies: Vec<DependencyLock>,
    /// The selected function's name.
    pub function: String,
    /// The digest of the tool manifest of the backend that found the counterexample.
    pub backend_manifest: DigestRecord,
    /// The limits the replay run itself is charged against.
    pub accounting_limits: ScalarLimits,
    /// The S1 to S4 stage limits of the proving run.
    pub stage_limits: StageLimits,
}

/// Why a [`ReplayPackage`] could not be built.
#[derive(Debug)]
pub enum ReplayPackageError {
    /// The selected function's name is not a valid identifier.
    InvalidFunction {
        /// The rejected name.
        function: String,
    },
    /// Two dependency selections name the same library; QSL admits each identity once.
    DuplicateDependency {
        /// The repeated library identity.
        identity: String,
    },
    /// QSL could not locate the function in the compiled unit.
    CallSite(Box<CallSiteRefusal>),
}

impl fmt::Display for ReplayPackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFunction { function } => {
                write!(f, "`{function}` is not a valid function identifier")
            }
            Self::DuplicateDependency { identity } => {
                write!(
                    f,
                    "the lock selects the dependency `{identity}` more than once"
                )
            }
            Self::CallSite(refusal) => write!(f, "the call site was not located: {refusal}"),
        }
    }
}

impl std::error::Error for ReplayPackageError {}

/// A compiled proving-run package: the facts QSL supplies about it (package id and parameter
/// node ids, through [`qsl_replay::call_site`]) joined to the lock the request repeats.
#[derive(Clone, Debug)]
pub struct ReplayPackage {
    inputs: ReplayInputs,
    selection: QualifiedName,
    site: CallSite,
    parameters: Vec<(String, String)>,
}

impl ReplayPackage {
    /// Compiles `inputs.source` and locates the selected function in it.
    ///
    /// # Errors
    ///
    /// [`ReplayPackageError`] when the function name is not an identifier, a dependency identity
    /// repeats, or QSL does not compile the unit or find the function. `call_site` compiles a
    /// standalone unit, so a unit that imports a selected dependency is refused as
    /// [`ReplayPackageError::CallSite`].
    pub fn new(mut inputs: ReplayInputs) -> Result<Self, ReplayPackageError> {
        // QSL admits dependency entries in strictly ascending identity order, each identity once.
        inputs
            .dependencies
            .sort_by(|left, right| left.identity.cmp(&right.identity));
        if let Some(pair) = inputs
            .dependencies
            .windows(2)
            .find(|pair| pair[0].identity == pair[1].identity)
        {
            return Err(ReplayPackageError::DuplicateDependency {
                identity: pair[0].identity.clone(),
            });
        }
        let invalid = || ReplayPackageError::InvalidFunction {
            function: inputs.function.clone(),
        };
        let identifier = Identifier::new(&inputs.function).map_err(|_| invalid())?;
        let selection = QualifiedName::new(vec![identifier]).map_err(|_| invalid())?;
        let source = &inputs.source;
        let site = call_site(
            SourceIdentity::new(
                &source.authority,
                &source.identity,
                &source.namespace,
                &source.revision,
            ),
            &source.identity,
            &source.bytes,
            &selection,
        )
        .map_err(ReplayPackageError::CallSite)?;
        let parameters = site
            .parameters
            .iter()
            .map(|(name, node)| (name.as_str().to_owned(), node.to_string()))
            .collect();
        Ok(Self {
            inputs,
            selection,
            site,
            parameters,
        })
    }

    /// The parameters of the selected function, each with its node id.
    pub fn parameters(&self) -> Vec<ReplayParameter<'_>> {
        self.parameters
            .iter()
            .map(|(argument, node_id)| ReplayParameter { argument, node_id })
            .collect()
    }

    /// The complete replay request for `counterexample`, replaying `source`.
    ///
    /// The package reference's `dependencies` are the lock's dependency selections, one entry
    /// each, and the byte provision holds the proved unit's source and every dependency source.
    pub fn request(&self, counterexample: &str, source: ReplaySource) -> ReplayRequestWire {
        let inputs = &self.inputs;
        let dependencies = inputs
            .dependencies
            .iter()
            .map(|dependency| DependencyEntryWire {
                identity: dependency.identity.clone(),
                version: dependency.version.clone(),
                package_id: (
                    Some(dependency.package_id.domain().as_str().to_owned()),
                    dependency.package_id.hex(),
                ),
                sources: dependency.sources.iter().map(LockedSource::wire).collect(),
            })
            .collect();
        // One entry per distinct digest, in digest order: a source shared between the unit and
        // a dependency, or between dependencies, is provided once.
        let byte_provision = std::iter::once(&inputs.source)
            .chain(
                inputs
                    .dependencies
                    .iter()
                    .flat_map(|dependency| dependency.sources.iter()),
            )
            .map(|file| {
                let digest = file.digest();
                (
                    digest.hex(),
                    (
                        Some(digest.domain().as_str().to_owned()),
                        file.bytes.clone(),
                    ),
                )
            })
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .map(|(hex, (domain, bytes))| (domain, hex, bytes))
            .collect();
        ReplayRequestWire {
            contract_version: "quire.native-runtime/v1".to_owned(),
            capability_vocabulary: Some("quire.capability-kind/v1".to_owned()),
            profile_selections: Vec::new(),
            package_id: (
                Some(self.site.package_id.domain().as_str().to_owned()),
                self.site.package_id.hex(),
            ),
            package_contract_version: "quire.checked-package/v2".to_owned(),
            source_digests: vec![inputs.source.wire()],
            dependencies,
            selected_function: self.selection.clone(),
            source,
            originating_counterexample_identity: ByteDigest::of(counterexample.as_bytes())
                .as_bytes(),
            backend: (
                "kani".to_owned(),
                Some(inputs.backend_manifest.domain().as_str().to_owned()),
                inputs.backend_manifest.hex(),
            ),
            state_environment: StateEnvironment::new(Vec::new()),
            accounting_limits: inputs.accounting_limits,
            stage_limits: inputs.stage_limits,
            byte_provision,
        }
    }
}

/// Why a counterexample is evidence failure: the backend's evidence and the native replay do not
/// establish the same typed violation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceFailureCause {
    /// The transcript did not decode against the obligation's schema or names another harness.
    Decode(DecodeFailure),
    /// A decoded value lies outside its argument's declared domain.
    Domain {
        /// The argument the value is bound to.
        argument: String,
    },
    /// Native replay ran and did not settle a reproduced violation.
    Verdict {
        /// The settlement QSL reached.
        settlement: WitnessSettlement,
        /// The category QSL evaluated.
        category: ProofCategory,
    },
}

/// The verdict on one backend counterexample. A counterexample that is not reproduced is never
/// a clause success or failure: it is evidence failure, whatever its cause.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayVerdict {
    /// Native replay reproduced the violation the backend reported.
    Reproduced,
    /// The counterexample is not valid evidence.
    EvidenceFailure(EvidenceFailureCause),
}

/// Decodes `transcript` against `identity`, checks every value against its declared domain,
/// replays it natively through `package` and reports the verdict.
///
/// # Errors
///
/// [`SpineReplayError`] when the adapter cannot build an admitted request or QSL refuses it;
/// those are not verdicts on the evidence.
pub fn replay_counterexample(
    identity: &KaniObligationIdentity,
    transcript: &str,
    package: &ReplayPackage,
) -> Result<ReplayVerdict, SpineReplayError> {
    let failure = |cause| Ok(ReplayVerdict::EvidenceFailure(cause));
    let values = match decode_falsification(
        &identity.harness_symbol,
        &identity.module_symbol,
        &identity.arguments,
        transcript,
    ) {
        Ok(values) => values,
        Err(cause) => return failure(EvidenceFailureCause::Decode(cause)),
    };
    if let Some(argument) = first_out_of_domain(&identity.arguments, &values) {
        return failure(EvidenceFailureCause::Domain {
            argument: argument.to_owned(),
        });
    }
    let harness = format!("{}::{}", identity.module_symbol, identity.harness_symbol);
    let result = replay_falsification(
        &harness,
        identity.clause.clause().as_str(),
        &values,
        &package.parameters(),
        |source| package.request(transcript, source),
    )?;
    Ok(verdict_of(result.settlement(), result.category()))
}

/// The verdict one witness-arm settlement decides: only an agreement with backend evidence in
/// the `violation` category reproduces the backend's falsification; every other settlement is
/// evidence failure.
fn verdict_of(settlement: WitnessSettlement, category: ProofCategory) -> ReplayVerdict {
    match (settlement, category) {
        (WitnessSettlement::ReproducedWithEvaluatedWitness, ProofCategory::Violation) => {
            ReplayVerdict::Reproduced
        }
        (
            WitnessSettlement::ReproducedWithEvaluatedWitness | WitnessSettlement::Inconclusive,
            _,
        ) => ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Verdict {
            settlement,
            category,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A reproduced settlement in any category other than `violation` is not a reproduced
    /// failure, and an inconclusive settlement never is, whatever its category.
    ///
    /// Trace: FR-016-AC-13, TC-026
    #[test]
    fn only_a_reproduced_violation_reproduces() {
        use WitnessSettlement::{Inconclusive, ReproducedWithEvaluatedWitness as Reproduced};
        assert_eq!(
            verdict_of(Reproduced, ProofCategory::Violation),
            ReplayVerdict::Reproduced
        );
        for (settlement, category) in [
            (Reproduced, ProofCategory::Success),
            (Inconclusive, ProofCategory::Violation),
            (Inconclusive, ProofCategory::Success),
        ] {
            assert_eq!(
                verdict_of(settlement, category),
                ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Verdict {
                    settlement,
                    category
                }),
                "{settlement:?} {category:?}"
            );
        }
    }
}

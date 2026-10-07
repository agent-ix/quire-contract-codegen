//! The replay adapter: a decoded Kani falsification goes to QSL's layer-6 `replay` facade, and
//! QSL's own verdict comes back.
//!
//! This module builds the backend-witness transcript in the grammar `qsl-replay` admits from the
//! typed values [`decode_falsification`]
//! returns, binds each value to its parameter by
//! the parameter's node id, and calls [`qsl_replay::replay`]. The executor recompiles the
//! request's digest-addressed source and evaluates the selected function itself, so the verdict
//! is QSL's, not a value this crate supplies.

use std::{collections::BTreeMap, fmt};

use qsl_replay::{
    call_site, replay, ByteDigest, CallSite, CallSiteRefusal, Category, DeclaredDomain,
    DependencyEntryWire, DependencyInput, DependencyInputRefusal, DigestDomain, DigestRecord,
    DisagreementCause, FunctionSite, Identifier, MalformedTranscript, ObligationIdentity,
    QualifiedName, ReplayLimits, ReplayRefusal, ReplayRequestWire, ReplayResult, ReplaySource,
    ScalarLimits, SourceIdentity, StateEnvironment, SuppliedLibrary, Witness, WitnessArmResult,
    WitnessSettlement, WitnessValue,
};

use crate::{
    kani::identity::KaniObligationIdentity,
    kani::output::playback::DecodeFailure,
    kani::terminal::ReplaySettlement,
    replay::{
        obligation::{contract_arguments, function_contract_identity, ObligationIdentityError},
        witness::{decode_falsification, first_out_of_domain},
    },
};

/// The request's `backend` member: the identity of the backend that found the counterexample.
/// QSL records it and does not interpret it (AD-002); the only backend this crate runs is Kani.
const BACKEND_IDENTITY: &str = "kani";

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
    /// The selected scalar Kani harness cannot transcribe this QSL value into its witness text.
    UnsupportedWitnessValue {
        /// The decoded argument's name.
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
    /// The harness has no function-contract obligation identity.
    Identity(ObligationIdentityError),
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
            Self::UnsupportedWitnessValue { argument } => {
                write!(
                    f,
                    "the harness argument `{argument}` is not an integer or Boolean"
                )
            }
            Self::FieldDelimiter => {
                f.write_str("the harness or check text contains a transcript delimiter")
            }
            Self::Transcript(cause) => write!(f, "the witness transcript is not admitted: {cause}"),
            Self::Refused(refusal) => write!(f, "the replay was refused: {refusal}"),
            Self::WrongArm => f.write_str("a witness-sourced request settled on the input arm"),
            Self::Identity(cause) => write!(f, "the obligation identity was not built: {cause}"),
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
    replay_limits: ReplayLimits,
) -> Result<WitnessArmResult, SpineReplayError> {
    replay_falsification_through(
        harness,
        check_text,
        values,
        parameters,
        request,
        &mut |wire| replay(wire, replay_limits),
    )
}

/// The one function that renders a backend-witness transcript (FR-024-AC-2): the assertion block
/// `qsl-replay` admits, built from decoded values only and never from backend-native text. Each
/// binding is a name (a parameter node id, or a state field) and the integer decoded for it.
/// Every transcript this crate passes to [`Witness::parse`] comes from here.
///
/// # Errors
///
/// [`MalformedTranscript`] when `qsl-replay` does not admit the rendered block, as when a field
/// holds a delimiter.
pub(crate) fn render_witness(
    harness: &str,
    check_text: &str,
    bindings: &[(&str, i128)],
) -> Result<Witness, MalformedTranscript> {
    let bindings = bindings
        .iter()
        .map(|(name, integer)| format!("{name}={integer}"))
        .collect::<Vec<_>>();
    Witness::parse(format!(
        "<<<assertion|{harness}|{check_text}|{}>>>",
        bindings.join(";")
    ))
}

/// What executes a built request: [`qsl_replay::replay`], or a caller's wrapper around it.
type Execute<'a> = &'a mut dyn FnMut(ReplayRequestWire) -> Result<ReplayResult, ReplayRefusal>;

fn replay_falsification_through(
    harness: &str,
    check_text: &str,
    values: &[(String, WitnessValue)],
    parameters: &[ReplayParameter<'_>],
    request: impl FnOnce(ReplaySource) -> ReplayRequestWire,
    execute: Execute<'_>,
) -> Result<WitnessArmResult, SpineReplayError> {
    if [harness, check_text]
        .iter()
        .any(|field| field.contains(['|', '<', '>']))
    {
        return Err(SpineReplayError::FieldDelimiter);
    }
    let mut bindings = values
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
                WitnessValue::Boolean(boolean) => i128::from(*boolean),
                WitnessValue::ExactInteger(_)
                | WitnessValue::Enum { .. }
                | WitnessValue::Text(_)
                | WitnessValue::Rational { .. }
                | WitnessValue::Decimal { .. }
                | WitnessValue::Float32(_)
                | WitnessValue::Float64(_)
                | WitnessValue::Quantity { .. }
                | WitnessValue::Reference { .. }
                | WitnessValue::Option(_)
                | WitnessValue::Record { .. }
                | WitnessValue::Tuple { .. }
                | WitnessValue::Union { .. }
                | WitnessValue::Sequence(_)
                | WitnessValue::OrderedSet(_)
                | WitnessValue::Set(_)
                | WitnessValue::Bag(_) => {
                    return Err(SpineReplayError::UnsupportedWitnessValue {
                        argument: argument.clone(),
                    })
                }
            };
            Ok((parameter.node_id, integer))
        })
        .collect::<Result<Vec<_>, SpineReplayError>>()?;
    // QSL joins function witness entries by parameter node id and requires ascending id order.
    bindings.sort_unstable_by(|left, right| left.0.cmp(right.0));
    let witness = render_witness(harness, check_text, &bindings)?;
    match execute(request(ReplaySource::Witness(witness))) {
        Ok(ReplayResult::Witness(result)) => Ok(result),
        Ok(ReplayResult::Input(_)) => Err(SpineReplayError::WrongArm),
        Err(refusal) => Err(SpineReplayError::Refused(Box::new(refusal))),
    }
}

/// One document the replay reads from the request's byte provision, with the digest the request
/// addresses it by. The frame and state-clause replays both take their documents this way.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedDocument {
    /// The document's digest record: `sha256-jcs` for a domain package or a state document.
    pub digest: DigestRecord,
    /// The document's bytes.
    pub bytes: Vec<u8>,
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
    pub(crate) fn digest(&self) -> DigestRecord {
        DigestRecord::mint(
            DigestDomain::SourceBytesV1,
            ByteDigest::of(&self.bytes).as_bytes(),
        )
    }

    pub(crate) fn source_identity(&self) -> SourceIdentity {
        SourceIdentity::new(
            &self.authority,
            &self.identity,
            &self.namespace,
            &self.revision,
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
/// source. QSL compiles a library from exactly one source unit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyLock {
    /// The library identity.
    pub identity: String,
    /// The dependency's `package_id`, as the proving run recorded it.
    pub package_id: DigestRecord,
    /// The dependency's lock source.
    pub source: LockedSource,
}

/// Everything about the proving run a replay request repeats.
#[derive(Clone, Debug)]
pub struct ReplayInputs {
    /// The proved unit's source.
    pub source: LockedSource,
    /// The proved package lock's dependency selections, each with its own source.
    pub dependencies: Vec<DependencyLock>,
    /// The limits the replay run itself is charged against.
    pub accounting_limits: ScalarLimits,
    /// The proving run's named stage bounds. Omitted names use QSL's published defaults.
    pub stage_limits: BTreeMap<String, u64>,
    /// The proving run's declared finite domains, keyed by their original proof positions.
    pub declared_domains: Vec<DeclaredDomain>,
    /// The configured reader bound for the replay request and envelopes.
    pub replay_limits: ReplayLimits,
}

/// Why a lock's dependency selections are not a dependency input QSL admits.
#[derive(Debug)]
pub enum DependencyLockError {
    /// The selections are no dependency input QSL's `DependencyInput::new` admits: a library
    /// under an empty identity, a repeated identity or two libraries sharing a source owner.
    /// A library sharing the unit's owner is refused later, by the call site, as
    /// [`ReplayPackageError::CallSite`].
    Input(DependencyInputRefusal),
}

impl fmt::Display for DependencyLockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(refusal) => {
                write!(
                    f,
                    "the lock's dependencies are no dependency input: {refusal}"
                )
            }
        }
    }
}

impl std::error::Error for DependencyLockError {}

/// Why a [`ReplayPackage`] could not be built.
#[derive(Debug)]
pub enum ReplayPackageError {
    /// The selected function's name is not a valid identifier.
    InvalidFunction {
        /// The rejected name.
        function: String,
    },
    /// The lock's dependency selections are not admitted.
    Dependencies(DependencyLockError),
    /// QSL could not compile the unit against its dependencies or locate the function in it.
    CallSite(Box<CallSiteRefusal>),
}

impl fmt::Display for ReplayPackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFunction { function } => {
                write!(f, "`{function}` is not a valid function identifier")
            }
            Self::Dependencies(cause) => cause.fmt(f),
            Self::CallSite(refusal) => write!(f, "the call site was not located: {refusal}"),
        }
    }
}

impl std::error::Error for ReplayPackageError {}

impl ReplayInputs {
    /// The lock with its dependencies in the ascending identity order the request carries, and the
    /// dependency input the same selections make. QSL's `DependencyInput::new` refuses an empty,
    /// repeated or shared-owner selection; this adapter checks none of those itself.
    pub(crate) fn admit(mut self) -> Result<(Self, DependencyInput), DependencyLockError> {
        self.dependencies
            .sort_by(|left, right| left.identity.cmp(&right.identity));
        let libraries = self
            .dependencies
            .iter()
            .map(|dependency| SuppliedLibrary {
                identity: dependency.identity.clone(),
                source: dependency.source.source_identity(),
                path: dependency.source.identity.clone(),
                bytes: dependency.source.bytes.clone(),
            })
            .collect::<Vec<_>>();
        let input = DependencyInput::new(libraries).map_err(DependencyLockError::Input)?;
        Ok((self, input))
    }

    /// The request QSL replays: this lock's package reference over `package_id`, selecting
    /// `selected`, replaying `source`, with `documents` provided beside the unit's and the
    /// dependencies' sources.
    pub(crate) fn wire(
        &self,
        package_id: DigestRecord,
        selected: QualifiedName,
        source: ReplaySource,
        obligation_identity: [u8; 32],
        documents: &[(DigestRecord, &[u8])],
    ) -> ReplayRequestWire {
        let dependencies = self
            .dependencies
            .iter()
            .map(|dependency| DependencyEntryWire {
                identity: dependency.identity.clone(),
                package_id: (
                    Some(dependency.package_id.domain().as_str().to_owned()),
                    dependency.package_id.hex(),
                ),
                sources: vec![dependency.source.wire()],
            })
            .collect();
        // One entry per distinct digest, in digest order: a source shared between the unit and
        // a dependency, or between dependencies, is provided once.
        let sources = std::iter::once(&self.source)
            .chain(
                self.dependencies
                    .iter()
                    .map(|dependency| &dependency.source),
            )
            .map(|file| (file.digest(), file.bytes.as_slice()));
        let byte_provision = sources
            .chain(documents.iter().copied())
            .map(|(digest, bytes)| {
                (
                    digest.hex(),
                    (Some(digest.domain().as_str().to_owned()), bytes),
                )
            })
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .map(|(hex, (domain, bytes))| (domain, hex, bytes.to_vec()))
            .collect();
        ReplayRequestWire {
            profile_selections: Vec::new(),
            package_id: (
                Some(package_id.domain().as_str().to_owned()),
                package_id.hex(),
            ),
            source_digests: vec![self.source.wire()],
            dependencies,
            selected_function: selected,
            source,
            obligation_identity,
            backend: BACKEND_IDENTITY.to_owned(),
            state_environment: StateEnvironment::new(Vec::new()),
            accounting_limits: self.accounting_limits,
            stage_limits: self.stage_limits.clone(),
            declared_domains: self.declared_domains.clone(),
            byte_provision,
        }
    }
}

/// A compiled proving-run package: the facts QSL supplies about it (package id and parameter
/// node ids, through [`qsl_replay::call_site`]) joined to the lock the request repeats.
#[derive(Clone, Debug)]
pub struct ReplayPackage {
    inputs: ReplayInputs,
    selection: QualifiedName,
    site: CallSite<FunctionSite>,
    parameters: Vec<(String, String)>,
}

impl ReplayPackage {
    /// Compiles `inputs.source` against the lock's dependencies and locates `function` in it.
    /// A dependency the unit imports is compiled from its lock source; one the unit does not
    /// import changes nothing.
    ///
    /// # Errors
    ///
    /// [`ReplayPackageError`] when the function name is not an identifier, a dependency identity
    /// repeats, the dependencies are no dependency input, or QSL does not compile the unit or
    /// find the function.
    pub fn new(inputs: ReplayInputs, function: &str) -> Result<Self, ReplayPackageError> {
        let (inputs, dependency_input) =
            inputs.admit().map_err(ReplayPackageError::Dependencies)?;
        let invalid = || ReplayPackageError::InvalidFunction {
            function: function.to_owned(),
        };
        let identifier = Identifier::new(function).map_err(|_| invalid())?;
        let selection = QualifiedName::new(vec![identifier]).map_err(|_| invalid())?;
        let source = &inputs.source;
        let site = call_site(
            source.source_identity(),
            &source.identity,
            &source.bytes,
            [],
            &dependency_input,
            &selection,
        )
        .map_err(ReplayPackageError::CallSite)?;
        let parameters = site
            .site
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
    ///
    /// The request's `obligation_identity` slot holds the identity
    /// [`ReplayPackage::obligation_identity`] returns for `identity`, the harness replayed, so a
    /// slot that is not that harness's identity is not representable here.
    ///
    /// # Errors
    ///
    /// [`ObligationIdentityError`] when `identity` has no function-contract identity.
    pub fn request(
        &self,
        identity: &KaniObligationIdentity,
        source: ReplaySource,
    ) -> Result<ReplayRequestWire, ObligationIdentityError> {
        Ok(self.request_for(self.obligation_identity(identity)?, source))
    }

    fn request_for(
        &self,
        obligation: ObligationIdentity,
        source: ReplaySource,
    ) -> ReplayRequestWire {
        self.inputs.wire(
            self.site.package_id,
            self.selection.clone(),
            source,
            *obligation.as_bytes(),
            &[],
        )
    }

    /// The ADR-013 O-09 function-contract obligation identity of the selected function for the
    /// harness `identity`: its kind and its arguments, each joined by identifier to the
    /// parameter node id of this package's `FunctionSite`. The function node id and its
    /// `declaration` occurrence key are the site's own, never derived here.
    ///
    /// # Errors
    ///
    /// [`ObligationIdentityError`] when a harness argument names no parameter of the function.
    pub fn obligation_identity(
        &self,
        identity: &KaniObligationIdentity,
    ) -> Result<ObligationIdentity, ObligationIdentityError> {
        let arguments = contract_arguments(&self.site.site.parameters, &identity.arguments)?;
        function_contract_identity(
            self.site.site.function,
            &self.site.site.declaration,
            identity.kind,
            &arguments,
        )
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
        category: Category,
        /// QSL's typed disagreement cause: present exactly when the settlement is
        /// `Inconclusive`.
        disagreement: Option<DisagreementCause>,
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

/// A replay setup refusal, read as the settlement of a falsified run (FR-029): a fault is a
/// fault, and every other refusal is a refusal on data carrying the code QSL's value supplies.
impl<'a> From<&'a CallSiteRefusal> for ReplaySettlement<'a> {
    fn from(refusal: &'a CallSiteRefusal) -> Self {
        match refusal {
            CallSiteRefusal::Fault(_) => Self::Fault,
            CallSiteRefusal::Compile { .. }
            | CallSiteRefusal::ModelIntake { .. }
            | CallSiteRefusal::DependencyInput(_)
            | CallSiteRefusal::Import { .. }
            | CallSiteRefusal::Dependency { .. }
            | CallSiteRefusal::UnknownFunction { .. }
            | CallSiteRefusal::UnknownOperation { .. }
            | CallSiteRefusal::UnknownClause { .. }
            | CallSiteRefusal::UnknownField { .. }
            | CallSiteRefusal::UnknownPopulation { .. } => Self::SetupRefused(refusal.code()),
        }
    }
}

impl<'a> From<&'a DependencyLockError> for ReplaySettlement<'a> {
    fn from(error: &'a DependencyLockError) -> Self {
        match error {
            DependencyLockError::Input(refusal) => Self::SetupRefused(refusal.code()),
        }
    }
}

impl<'a> From<&'a ReplayPackageError> for ReplaySettlement<'a> {
    fn from(error: &'a ReplayPackageError) -> Self {
        match error {
            ReplayPackageError::InvalidFunction { .. } => Self::CgDefect,
            ReplayPackageError::Dependencies(cause) => cause.into(),
            ReplayPackageError::CallSite(refusal) => refusal.as_ref().into(),
        }
    }
}

impl<'a> From<&'a SpineReplayError> for ReplaySettlement<'a> {
    fn from(error: &'a SpineReplayError) -> Self {
        match error {
            SpineReplayError::UnboundArgument { .. }
            | SpineReplayError::UnsupportedWitnessValue { .. }
            | SpineReplayError::FieldDelimiter
            | SpineReplayError::Transcript(_)
            | SpineReplayError::WrongArm
            | SpineReplayError::Identity(_) => Self::CgDefect,
            SpineReplayError::Refused(refusal) => Self::Refused(refusal),
        }
    }
}

impl<'a> From<&'a EvidenceFailureCause> for ReplaySettlement<'a> {
    fn from(cause: &'a EvidenceFailureCause) -> Self {
        match cause {
            // A playback that does not type against the persisted bindings, and a value outside
            // the harness proof bound, are this repository's defects.
            EvidenceFailureCause::Decode(_) | EvidenceFailureCause::Domain { .. } => Self::CgDefect,
            EvidenceFailureCause::Verdict {
                disagreement: Some(cause),
                ..
            } => Self::Disagreement(cause),
            // QSL settles a disagreement with its cause. A settlement that carries none is a
            // reproduction in a category other than `violation` (FR-016-AC-13): the replay
            // agreed with the run yet evaluated no violation. It is neither a refutation nor a
            // disagreement QSL typed, and no cause is invented for it, so it reads as a defect.
            EvidenceFailureCause::Verdict {
                disagreement: None, ..
            } => Self::CgDefect,
        }
    }
}

impl<'a> From<&'a ReplayVerdict> for ReplaySettlement<'a> {
    fn from(verdict: &'a ReplayVerdict) -> Self {
        match verdict {
            ReplayVerdict::Reproduced => Self::Reproduced,
            ReplayVerdict::EvidenceFailure(cause) => cause.into(),
        }
    }
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
    replay_counterexample_through(identity, transcript, package, |wire| {
        replay(wire, package.inputs.replay_limits)
    })
}

/// [`replay_counterexample`] with the request handed to `execute` instead of straight to
/// [`qsl_replay::replay`]: a caller that records or wraps the request QSL receives passes a
/// closure that ends in `qsl_replay::replay`.
///
/// # Errors
///
/// As [`replay_counterexample`].
pub fn replay_counterexample_through(
    identity: &KaniObligationIdentity,
    transcript: &str,
    package: &ReplayPackage,
    mut execute: impl FnMut(ReplayRequestWire) -> Result<ReplayResult, ReplayRefusal>,
) -> Result<ReplayVerdict, SpineReplayError> {
    let failure = |cause| Ok(ReplayVerdict::EvidenceFailure(cause));
    let values = match decode_falsification(
        identity.harness_symbol.as_str(),
        identity.module_symbol.as_str(),
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
    let obligation = package
        .obligation_identity(identity)
        .map_err(SpineReplayError::Identity)?;
    let harness = identity.harness_path().to_string();
    let result = replay_falsification_through(
        &harness,
        identity.clause.clause().as_str(),
        &values,
        &package.parameters(),
        |source| package.request_for(obligation, source),
        &mut execute,
    )?;
    Ok(verdict_of(
        result.settlement(),
        result.category(),
        result.disagreement(),
    ))
}

/// The verdict one witness-arm settlement decides: only an agreement with backend evidence in
/// the `violation` category reproduces the backend's falsification; every other settlement is
/// evidence failure, carrying QSL's disagreement cause when it has one.
fn verdict_of(
    settlement: WitnessSettlement,
    category: Category,
    disagreement: Option<&DisagreementCause>,
) -> ReplayVerdict {
    match (settlement, category) {
        (WitnessSettlement::ReproducedWithEvaluatedWitness, Category::Violation) => {
            ReplayVerdict::Reproduced
        }
        (
            WitnessSettlement::ReproducedWithEvaluatedWitness | WitnessSettlement::Inconclusive,
            _,
        ) => ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Verdict {
            settlement,
            category,
            disagreement: disagreement.cloned(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani::{
        abi::{KaniBindingRole, KaniIntegerBounds, KaniPrimitiveType},
        identity::ObligationBinding,
    };
    use qsl_replay::{OccurrenceKey, WireNodeId};
    use quire_contract_model::{IntegerDomain, OverflowPolicy};

    const UNIT: &str = "language \"ix:native\" edition \"1-draft\";\n\
        profile v = \"quire.value.complete/v1\";\n\
        function f using v(a: Int[0, 9], b: Int[0, 9]): Boolean pure { a <= b }\n";

    fn unlimited() -> ScalarLimits {
        let max = u64::MAX;
        ScalarLimits {
            integer_bits: max,
            decimal_digits: max,
            scale_expansion: max,
            text_input_bytes: max,
            text_scalars: max,
            normalized_scalars: max,
            unit_edges: max,
            value_occurrences: max,
            work_units: max,
            result_units: max,
        }
    }

    fn package() -> ReplayPackage {
        let limits = unlimited();
        let inputs = ReplayInputs {
            source: LockedSource {
                authority: "agent-ix".to_owned(),
                identity: "unit".to_owned(),
                namespace: "git".to_owned(),
                revision: "r1".to_owned(),
                bytes: UNIT.as_bytes().to_vec(),
            },
            dependencies: Vec::new(),
            accounting_limits: limits,
            stage_limits: BTreeMap::from([("s1.input_bytes".to_owned(), 1 << 20)]),
            declared_domains: Vec::new(),
            replay_limits: ReplayLimits::default(),
        };
        ReplayPackage::new(inputs, "f").expect("the unit compiles and declares `f`")
    }

    fn binding(identifier: &str) -> ObligationBinding {
        ObligationBinding {
            identifier: identifier.to_owned(),
            role: KaniBindingRole::Argument,
            primitive_type: KaniPrimitiveType::I64,
            integer_bounds: Some(KaniIntegerBounds {
                domain: IntegerDomain::Signed,
                minimum: 0,
                maximum: 9,
                overflow: OverflowPolicy::Reject,
            }),
            dependencies: Vec::new(),
        }
    }

    /// With the function, declaration, kind and arguments fixed, reordering `FunctionSite.parameters`
    /// leaves the identity unchanged, and a perturbed `function` or `declaration` changes it.
    ///
    /// Trace: FR-016-AC-23, TC-026
    #[test]
    fn tc_026_the_identity_ignores_the_declared_order_of_the_site_parameters() {
        use crate::kani::identity::ObligationKind;

        let arguments = [binding("a"), binding("b")];
        let arguments_of = |package: &ReplayPackage| {
            contract_arguments(&package.site.site.parameters, &arguments).unwrap()
        };
        let identity = |package: &ReplayPackage, arguments: &[_]| {
            function_contract_identity(
                package.site.site.function,
                &package.site.site.declaration,
                ObligationKind::Postcondition,
                arguments,
            )
            .unwrap()
        };
        let original = package();
        let baseline = identity(&original, &arguments_of(&original));

        let mut reordered = original.clone();
        reordered.site.site.parameters.reverse();
        assert_ne!(
            reordered.site.site.parameters, original.site.site.parameters,
            "the site's declared order changed"
        );
        assert_eq!(baseline, identity(&reordered, &arguments_of(&reordered)));
        let mut harness_order = arguments_of(&original);
        harness_order.reverse();
        assert_eq!(baseline, identity(&original, &harness_order));

        let mut other_function = original.clone();
        other_function.site.site.function = WireNodeId::from_digest([9; 32]);
        assert_ne!(
            baseline,
            identity(&other_function, &arguments_of(&other_function))
        );
        let mut other_declaration = original.clone();
        other_declaration.site.site.declaration = OccurrenceKey::new(
            original.site.site.function,
            qsl_replay::Origin::new(qsl_replay::Role::new("declaration"), 1),
        );
        assert_ne!(
            baseline,
            identity(&other_declaration, &arguments_of(&other_declaration))
        );
    }

    /// A reproduced settlement in any category other than `violation` is not a reproduced
    /// failure, and an inconclusive settlement never is, whatever its category.
    ///
    /// Trace: FR-016-AC-13, TC-026
    #[test]
    fn only_a_reproduced_violation_reproduces() {
        use WitnessSettlement::{Inconclusive, ReproducedWithEvaluatedWitness as Reproduced};
        assert_eq!(
            verdict_of(Reproduced, Category::Violation, None),
            ReplayVerdict::Reproduced
        );
        for (settlement, category) in [
            (Reproduced, Category::Success),
            (Inconclusive, Category::Violation),
            (Inconclusive, Category::Success),
        ] {
            assert_eq!(
                verdict_of(settlement, category, None),
                ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Verdict {
                    settlement,
                    category,
                    disagreement: None,
                }),
                "{settlement:?} {category:?}"
            );
        }
    }
}

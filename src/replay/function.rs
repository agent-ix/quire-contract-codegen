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
    call_site, replay, ByteDigest, CallSite, CallSiteRefusal, DeclaredDomain, DependencyEntryWire,
    DependencyInput, DependencyInputRefusal, DigestDomain, DigestRecord, DisagreementCause,
    EnvelopeSource, FunctionSite, Identifier, MalformedTranscript, NoPayload, ObligationIdentity,
    OccurrenceKey, QualifiedName, ReplayLimits, ReplayRefusal, ReplayRequestWire, ReplayResult,
    ReplaySource, ReplayTarget, RequestTargetWire, ScalarLimits, SourceIdentity, StateEnvironment,
    SuppliedLibrary, WireNodeId, Witness, WitnessArmResult, WitnessEnvelope, WitnessPacket,
    WitnessRefusal, WitnessSettlement, WitnessValue,
};
use quire_semantic_value::catalog::Category;

use crate::{
    kani::generate::corpus::bounded_kani_corpus::BoundedCorpusCase,
    kani::identity::KaniObligationIdentity,
    kani::output::playback::{Playback, WitnessDecodeLimits},
    kani::terminal::ReplaySettlement,
    replay::{
        obligation::{contract_arguments, function_contract_identity, ObligationIdentityError},
        witness::{decode_falsification, decode_playback, first_out_of_domain, DecodeFailure},
    },
};

#[cfg(test)]
use crate::replay::witness::unbounded_decode_limits;

/// The Kani identity of a harness whose claim is a selected QSL function's Boolean result.
///
/// Unlike [`KaniObligationIdentity`], this carries no clause or source-span fields: QSL's
/// [`call_site`] supplies the function node and declaration occurrence used for the typed replay
/// identity. `module_symbol` is absent for a top-level harness, as the bounded corpus emits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReplayHarnessIdentity {
    /// Optional Rust module containing the harness.
    pub module_symbol: Option<crate::core::identity::ModuleSymbol>,
    /// Kani harness function symbol.
    pub harness_symbol: crate::core::identity::HarnessSymbol,
    /// Argument bindings in the order Kani's harness draws them.
    pub arguments: Vec<crate::kani::identity::ObligationBinding>,
    package_id: DigestRecord,
    source_identity: SourceIdentity,
    function: WireNodeId,
    declaration: OccurrenceKey,
}

impl ReplayHarnessIdentity {
    fn qualified_harness(&self) -> String {
        self.module_symbol.as_ref().map_or_else(
            || self.harness_symbol.to_string(),
            |module| format!("{module}::{}", self.harness_symbol),
        )
    }

    pub(crate) fn matches_package(&self, package: &ReplayPackage) -> bool {
        self.package_id == package.site.package_id
            && self.source_identity == package.inputs.source.source_identity()
            && self.function == package.site.site.function
            && self.declaration == package.site.site.declaration
    }
}

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
    /// An input-sourced function evaluation returned no Boolean value.
    #[cfg(test)]
    NoInputValue,
    /// The typed harness identity was created from a different QSL package/site.
    #[cfg(test)]
    PackageIdentity,
    /// The harness has no function-contract obligation identity.
    Identity(ObligationIdentityError),
    /// The producer packet did not admit or bind to the selected function claim.
    Packet(FunctionPacketError),
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
            #[cfg(test)]
            Self::NoInputValue => f.write_str("the QSL input evaluation returned no Boolean value"),
            #[cfg(test)]
            Self::PackageIdentity => {
                f.write_str("the harness identity belongs to another QSL package")
            }
            Self::Identity(cause) => write!(f, "the obligation identity was not built: {cause}"),
            Self::Packet(cause) => write!(f, "the producer packet was refused: {cause:?}"),
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
        |source| Ok(request(source.into())),
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
    request: impl FnOnce(EnvelopeSource) -> Result<ReplayRequestWire, SpineReplayError>,
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
                | WitnessValue::String(_)
                | WitnessValue::Uuid(_)
                | WitnessValue::Timestamp(_)
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
    let request = request(EnvelopeSource::Witness(witness))?;
    match execute(request) {
        Ok(ReplayResult::Witness(result)) => Ok(result),
        Ok(
            ReplayResult::Input(_)
            | ReplayResult::ObservedTemporal(_)
            | ReplayResult::ModelGraph(_),
        ) => Err(SpineReplayError::WrongArm),
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

    pub(crate) fn wire(&self) -> (String, String, String, String, Option<String>, String) {
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
///
/// Replay inputs cannot silently acquire decode bounds (FR-016-AC-6, TC-026):
///
/// ```compile_fail
/// let _ = quire_contract_codegen::ReplayInputs::default();
/// ```
#[derive(Clone, Debug)]
pub struct ReplayInputs {
    /// Required limits for decoding the retained Kani witness.
    pub decode_limits: WitnessDecodeLimits,
    /// The originating checked package's manifest library name.
    pub package_name: String,
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
        replay_target: RequestTargetWire,
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
            replay_target,
            source,
            obligation_identity,
            backend: BACKEND_IDENTITY.to_owned(),
            state_environment: StateEnvironment::new(Vec::new()),
            accounting_limits: self.accounting_limits,
            stage_limits: self.stage_limits.clone(),
            // This path has no caller-supplied helper ceilings; QSL resolves `None` to its published defaults.
            supplied_limits: None,
            declared_domains: self.declared_domains.clone(),
            byte_provision,
            // No Reference parameter is replayed on this path, so no object environment is supplied.
            object_environment: None,
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
    kani_binding: Option<KaniProofBinding>,
}

/// The retained proving-run identity and the QSL site selected for its hand-mirrored replay.
#[derive(Clone, Debug)]
struct KaniProofBinding {
    identity: KaniObligationIdentity,
    package_id: DigestRecord,
    source_identity: SourceIdentity,
    function: WireNodeId,
    declaration: OccurrenceKey,
}

/// Which independently selected function-claim member a producer packet failed to match.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PacketClaimMember {
    /// The packet selected another function.
    Target,
    /// The packet selected another occurrence of the function.
    Occurrence,
    /// The packet named another subject node.
    Subject,
    /// The packet named another checked package.
    Package,
    /// The packet's source references differ from the package lock.
    SourceDigests,
    /// The packet's profile selections differ from the request context.
    Profiles,
    /// The packet's accounting limits differ from the proving run.
    Limits,
    /// The packet's declared domains differ from the proving run.
    Domains,
    /// The packet names a different backend producer.
    Backend,
    /// A function falsification requires a backend witness.
    SourceArm,
    /// The packet named another obligation.
    Obligation,
}

/// A producer packet did not admit or bind to the function claim selected for replay.
#[derive(Debug)]
pub enum FunctionPacketError {
    /// QSL refused the packet before a request existed.
    Admission(WitnessRefusal),
    /// An admitted packet belongs to another claim.
    ClaimMismatch(PacketClaimMember),
    /// The supplied Kani identity differs from the identity retained with this QSL twin.
    ProvingRunMismatch,
    /// The selected Kani identity did not bind to this function package.
    Identity(ObligationIdentityError),
}

impl fmt::Display for FunctionPacketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admission(cause) => write!(f, "QSL refused the producer packet: {cause}"),
            Self::ClaimMismatch(member) => {
                write!(
                    f,
                    "the producer packet does not match the selected claim: {member:?}"
                )
            }
            Self::ProvingRunMismatch => {
                f.write_str("the Kani proving-run identity differs from this QSL twin")
            }
            Self::Identity(cause) => write!(f, "the selected claim identity failed: {cause}"),
        }
    }
}

impl std::error::Error for FunctionPacketError {}

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
            inputs.package_name.clone(),
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
            kani_binding: None,
        })
    }

    /// Retains the independent Kani proving-run identity beside this selected QSL twin before
    /// any playback is decoded. The caller is responsible for pairing the hand-mirrored QSL
    /// source with the proof it represents; subsequent replays refuse another identity or site.
    ///
    /// # Errors
    ///
    /// [`ObligationIdentityError`] when the proof arguments do not bind this QSL function.
    pub fn with_kani_identity(
        mut self,
        identity: &KaniObligationIdentity,
    ) -> Result<Self, ObligationIdentityError> {
        self.obligation_identity(identity)?;
        self.kani_binding = Some(KaniProofBinding {
            identity: identity.clone(),
            package_id: self.site.package_id,
            source_identity: self.inputs.source.source_identity(),
            function: self.site.site.function,
            declaration: self.site.site.declaration.clone(),
        });
        Ok(self)
    }

    fn matches_kani_identity(&self, identity: &KaniObligationIdentity) -> bool {
        self.kani_binding.as_ref().is_some_and(|binding| {
            &binding.identity == identity
                && binding.package_id == self.site.package_id
                && binding.source_identity == self.inputs.source.source_identity()
                && binding.function == self.site.site.function
                && binding.declaration == self.site.site.declaration
        })
    }

    /// The parameters of the selected function, each with its node id.
    pub fn parameters(&self) -> Vec<ReplayParameter<'_>> {
        self.parameters
            .iter()
            .map(|(argument, node_id)| ReplayParameter { argument, node_id })
            .collect()
    }

    /// Binds one real Kani harness identity to this package's QSL-selected function site.
    ///
    /// The returned identity retains QSL's package, function and declaration identities. Passing
    /// it to a different package is rejected by [`replay_harness_counterexample`].
    pub(crate) fn harness_identity(
        &self,
        module_symbol: Option<crate::core::identity::ModuleSymbol>,
        harness_symbol: crate::core::identity::HarnessSymbol,
        arguments: Vec<crate::kani::identity::ObligationBinding>,
    ) -> Result<ReplayHarnessIdentity, ObligationIdentityError> {
        self.obligation_identity_for(
            crate::kani::identity::ObligationKind::Postcondition,
            &arguments,
        )?;
        Ok(ReplayHarnessIdentity {
            module_symbol,
            harness_symbol,
            arguments,
            package_id: self.site.package_id,
            source_identity: self.inputs.source.source_identity(),
            function: self.site.site.function,
            declaration: self.site.site.declaration.clone(),
        })
    }

    pub(crate) fn source_matches(&self, source: &[u8]) -> bool {
        self.inputs.source.bytes.as_slice() == source
    }

    /// Evaluates this selected Boolean function as a QSL semantic oracle.
    ///
    /// This uses the `Input` arm only to check the renderer's function result. It is not backend
    /// or counterexample evidence; corpus counterexamples must use
    /// [`replay_harness_counterexample`] and its `Witness` arm.
    #[cfg(test)]
    pub(crate) fn evaluate_boolean_function(
        &self,
        identity: &ReplayHarnessIdentity,
    ) -> Result<bool, SpineReplayError> {
        if !identity.matches_package(self) {
            return Err(SpineReplayError::PackageIdentity);
        }
        let obligation = self
            .obligation_identity_for(
                crate::kani::identity::ObligationKind::Postcondition,
                &identity.arguments,
            )
            .map_err(SpineReplayError::Identity)?;
        match replay(
            self.request_for_input_oracle(obligation),
            self.inputs.replay_limits,
        )
        .map_err(|refusal| SpineReplayError::Refused(Box::new(refusal)))?
        {
            ReplayResult::Input(result) => match result.value() {
                Some(qsl_replay::EvaluatedValue::Boolean(value)) => Ok(value),
                None => Err(SpineReplayError::NoInputValue),
            },
            ReplayResult::Witness(_)
            | ReplayResult::ObservedTemporal(_)
            | ReplayResult::ModelGraph(_) => Err(SpineReplayError::WrongArm),
        }
    }

    /// The complete function-contract replay request from an independently supplied producer
    /// packet. QSL admits the packet, then CG binds it to this selected function and identity.
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
    /// [`FunctionPacketError`] when `identity` has no function-contract identity, QSL refuses
    /// the packet, the retained proving-run binding differs, or an admitted packet belongs to
    /// another selected claim.
    pub fn request(
        &self,
        identity: &KaniObligationIdentity,
        packet: WitnessPacket<NoPayload>,
    ) -> Result<ReplayRequestWire, FunctionPacketError> {
        if !self.matches_kani_identity(identity) {
            return Err(FunctionPacketError::ProvingRunMismatch);
        }
        let obligation = self
            .obligation_identity(identity)
            .map_err(FunctionPacketError::Identity)?;
        self.request_from_packet(packet, obligation)
    }

    /// Completes a backend witness packet from this checked function selection and the retained
    /// proving-run identity before [`Self::request`] admits it. The transcript is QSL's already
    /// parsed witness; it is never interpreted to choose the function or declaration occurrence.
    ///
    /// # Errors
    ///
    /// [`ObligationIdentityError`] when the harness arguments do not bind to this function.
    pub fn backend_witness_packet(
        &self,
        identity: &KaniObligationIdentity,
        witness: Witness,
    ) -> Result<WitnessPacket<NoPayload>, ObligationIdentityError> {
        let obligation = self.obligation_identity(identity)?;
        Ok(backend_function_packet(
            self,
            obligation,
            self.site.site.function,
            self.site.site.declaration.clone(),
            EnvelopeSource::Witness(witness),
        ))
    }

    fn request_from_packet(
        &self,
        packet: WitnessPacket<NoPayload>,
        obligation: ObligationIdentity,
    ) -> Result<ReplayRequestWire, FunctionPacketError> {
        let envelope = WitnessEnvelope::reconstruct(packet, self.inputs.replay_limits)
            .map_err(FunctionPacketError::Admission)?;
        if envelope.replay_target() != &ReplayTarget::function(self.selection.clone(), None) {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Target,
            ));
        }
        if envelope.occurrence_key() != &self.site.site.declaration {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Occurrence,
            ));
        }
        if envelope.clause_node() != self.site.site.function {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Subject,
            ));
        }
        if envelope.package_id() != self.site.package_id {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Package,
            ));
        }
        let [source_digest] = envelope.source_digests() else {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::SourceDigests,
            ));
        };
        if source_digest.to_wire() != self.inputs.source.wire() {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::SourceDigests,
            ));
        }
        if !envelope.profile_selections().is_empty() {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Profiles,
            ));
        }
        if envelope.run_limits() != self.inputs.accounting_limits {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Limits,
            ));
        }
        if envelope.declared_domains() != self.inputs.declared_domains.as_slice() {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Domains,
            ));
        }
        if envelope.backend().as_str() != BACKEND_IDENTITY {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Backend,
            ));
        }
        if !matches!(envelope.source(), EnvelopeSource::Witness(_)) {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::SourceArm,
            ));
        }
        if envelope.obligation_identity() != obligation {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Obligation,
            ));
        }
        self.request_from_envelope(&envelope)
    }

    fn request_from_envelope(
        &self,
        envelope: &WitnessEnvelope<NoPayload>,
    ) -> Result<ReplayRequestWire, FunctionPacketError> {
        let ReplayTarget::Declaration { name, .. } = envelope.replay_target() else {
            return Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Target,
            ));
        };
        Ok(self.inputs.wire(
            envelope.package_id(),
            RequestTargetWire::function(name.clone(), envelope.occurrence_key().clone()),
            envelope.source().clone().into(),
            *envelope.obligation_identity().as_bytes(),
            &[],
        ))
    }

    #[cfg(test)]
    fn request_for_input_oracle(&self, obligation: ObligationIdentity) -> ReplayRequestWire {
        self.inputs.wire(
            self.site.package_id,
            RequestTargetWire::function(self.selection.clone(), self.site.site.declaration.clone()),
            ReplaySource::Input(Vec::new()),
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
        self.obligation_identity_for(identity.kind, &identity.arguments)
    }

    /// The typed identity of this selected function result for one non-clause Kani harness.
    ///
    /// The selected function and declaration occurrence come from QSL's [`call_site`]; each
    /// argument is joined by identifier to that site's parameter node. Corpus harnesses prove a
    /// generated Boolean case predicate, so their QSL result is replayed as a postcondition.
    ///
    /// # Errors
    ///
    /// [`ObligationIdentityError`] when a harness argument names no parameter, a parameter has no
    /// harness binding, or an argument has no finite domain.
    pub(crate) fn obligation_identity_for(
        &self,
        kind: crate::kani::identity::ObligationKind,
        bindings: &[crate::kani::identity::ObligationBinding],
    ) -> Result<ObligationIdentity, ObligationIdentityError> {
        let arguments = contract_arguments(&self.site.site.parameters, bindings)?;
        function_contract_identity(
            self.site.site.function,
            &self.site.site.declaration,
            kind,
            &arguments,
        )
    }
}

/// The function backend adapter completes its packet from the retained proving-run selection
/// and the decoded witness source before the packet enters the replay request boundary.
fn backend_function_packet(
    package: &ReplayPackage,
    obligation: ObligationIdentity,
    subject: WireNodeId,
    occurrence: OccurrenceKey,
    source: EnvelopeSource,
) -> WitnessPacket<NoPayload> {
    let package_id = package.site.package_id;
    WitnessPacket {
        obligation_identity: Some(*obligation.as_bytes()),
        occurrence_key: Some(occurrence),
        clause_node: Some(subject),
        replay_target: qsl_replay::ReplayTargetMembers::function(package.selection.clone()),
        package_id: Some((
            Some(package_id.domain().as_str().to_owned()),
            package_id.hex(),
        )),
        source_digests: Some(vec![package.inputs.source.wire()]),
        profile_selections: Some(Vec::new()),
        run_limits: Some(package.inputs.accounting_limits),
        declared_domains: Some(package.inputs.declared_domains.clone()),
        backend: Some(BACKEND_IDENTITY.to_owned()),
        trace_position: Some(None),
        source: Some(source),
        family_payload: Some(NoPayload),
    }
}

/// Why a retained corpus case could not be replayed against its QSL source.
#[derive(Debug)]
pub enum BoundedCorpusReplayError {
    /// The retained unit/dependency package did not compile or select the corpus function.
    Package(ReplayPackageError),
    /// The package source did not match the case's retained source bytes.
    SourceMismatch,
    /// Kani playback could not be decoded or QSL refused the witness request.
    Replay(SpineReplayError),
}

impl BoundedCorpusCase {
    /// Replays bounded, parsed Kani counterexample playback against this case's retained QSL package.
    ///
    /// The caller parses the complete captured text through [`crate::parse_assertion_playback`].
    /// The package is compiled from the generated unit and imported dependency source each time;
    /// the harness identity is then tied to QSL's selected function site. The replay checks the
    /// parsed source length and value count against `limits` before native execution.
    pub fn replay_counterexample(
        &self,
        playback: &Playback<'_>,
        limits: WitnessDecodeLimits,
    ) -> Result<ReplayVerdict, BoundedCorpusReplayError> {
        let package =
            compile_corpus_qsl_package(self, limits).map_err(BoundedCorpusReplayError::Package)?;
        if !package.source_matches(self.artifacts.qsl_source.contents.as_bytes()) {
            return Err(BoundedCorpusReplayError::SourceMismatch);
        }
        let identity = package
            .harness_identity(None, self.harness_symbol.clone(), Vec::new())
            .map_err(|error| BoundedCorpusReplayError::Replay(SpineReplayError::Identity(error)))?;
        replay_harness_counterexample(&identity, playback, &package)
            .map_err(BoundedCorpusReplayError::Replay)
    }

    #[cfg(test)]
    pub(crate) fn qsl_source_matches_case_claim(&self) -> Result<bool, BoundedCorpusReplayError> {
        let package = compile_corpus_qsl_package(self, unbounded_decode_limits())
            .map_err(BoundedCorpusReplayError::Package)?;
        if !package.source_matches(self.artifacts.qsl_source.contents.as_bytes()) {
            return Err(BoundedCorpusReplayError::SourceMismatch);
        }
        let identity = package
            .harness_identity(None, self.harness_symbol.clone(), Vec::new())
            .map_err(|error| BoundedCorpusReplayError::Replay(SpineReplayError::Identity(error)))?;
        let expected = self.expectation.expected_assertion_holds();
        Ok(package.parameters().is_empty()
            && identity.matches_package(&package)
            && package.evaluate_boolean_function(&identity).ok() == Some(expected))
    }

    #[cfg(test)]
    pub(crate) fn qsl_identity_rejects_case(
        &self,
        other: &Self,
    ) -> Result<bool, BoundedCorpusReplayError> {
        let package = compile_corpus_qsl_package(self, unbounded_decode_limits())
            .map_err(BoundedCorpusReplayError::Package)?;
        let identity = package
            .harness_identity(None, self.harness_symbol.clone(), Vec::new())
            .map_err(|error| BoundedCorpusReplayError::Replay(SpineReplayError::Identity(error)))?;
        let other = compile_corpus_qsl_package(other, unbounded_decode_limits())
            .map_err(BoundedCorpusReplayError::Package)?;
        Ok(!identity.matches_package(&other))
    }

    #[cfg(test)]
    pub(crate) fn replay_rejects_other_case_package(
        &self,
        playback: &Playback<'_>,
        other: &Self,
    ) -> Result<bool, BoundedCorpusReplayError> {
        let package = compile_corpus_qsl_package(self, unbounded_decode_limits())
            .map_err(BoundedCorpusReplayError::Package)?;
        let identity = package
            .harness_identity(None, self.harness_symbol.clone(), Vec::new())
            .map_err(|error| BoundedCorpusReplayError::Replay(SpineReplayError::Identity(error)))?;
        let other = compile_corpus_qsl_package(other, unbounded_decode_limits())
            .map_err(BoundedCorpusReplayError::Package)?;
        replay_harness_counterexample(&identity, playback, &other)
            .map(|verdict| {
                verdict == ReplayVerdict::EvidenceFailure(EvidenceFailureCause::PackageIdentity)
            })
            .map_err(BoundedCorpusReplayError::Replay)
    }

    #[cfg(test)]
    pub(crate) fn replay_rejects_wrong_harness(
        &self,
        playback: &Playback<'_>,
    ) -> Result<bool, BoundedCorpusReplayError> {
        let package = compile_corpus_qsl_package(self, unbounded_decode_limits())
            .map_err(BoundedCorpusReplayError::Package)?;
        let Ok(harness) = crate::core::identity::HarnessSymbol::try_from("another_harness") else {
            return Ok(false);
        };
        let identity = package
            .harness_identity(None, harness, Vec::new())
            .map_err(|error| BoundedCorpusReplayError::Replay(SpineReplayError::Identity(error)))?;
        replay_harness_counterexample(&identity, playback, &package)
            .map(|verdict| {
                matches!(
                    verdict,
                    ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Decode(_))
                )
            })
            .map_err(BoundedCorpusReplayError::Replay)
    }
}

fn compile_corpus_qsl_package(
    case: &BoundedCorpusCase,
    decode_limits: WitnessDecodeLimits,
) -> Result<ReplayPackage, ReplayPackageError> {
    use qsl_replay::{
        call_site, DependencyInput, Identifier, QualifiedName, ReplayLimits, ScalarLimits,
        SourceIdentity,
    };
    use std::collections::BTreeMap;

    // The producer imports the dependency under this library name in its QSL source.
    const DEPENDENCY_NAME: &str = "ir-453/corpus-dependency";
    // The corpus producer assigns one name to its shared generated source unit.
    const PACKAGE_NAME: &str = "ir-453-corpus";
    let maximum = u64::MAX;
    let dependency_source = LockedSource {
        authority: "agent-ix".to_owned(),
        identity: DEPENDENCY_NAME.to_owned(),
        namespace: "generated".to_owned(),
        revision: "ir-453".to_owned(),
        bytes: case.artifacts.qsl_dependency.contents.as_bytes().to_vec(),
    };
    let function = "corpus_case";
    let invalid_function = || ReplayPackageError::InvalidFunction {
        function: function.to_owned(),
    };
    let dependency_function = QualifiedName::new(vec![
        Identifier::new(function).map_err(|_| invalid_function())?
    ])
    .map_err(|_| invalid_function())?;
    let dependency_package_id = call_site(
        SourceIdentity::new(
            &dependency_source.authority,
            &dependency_source.identity,
            &dependency_source.namespace,
            &dependency_source.revision,
        ),
        DEPENDENCY_NAME.to_owned(),
        &dependency_source.identity,
        &dependency_source.bytes,
        [],
        &DependencyInput::default(),
        &dependency_function,
    )
    .map_err(ReplayPackageError::CallSite)?
    .package_id;
    ReplayPackage::new(
        ReplayInputs {
            decode_limits,
            package_name: PACKAGE_NAME.to_owned(),
            source: LockedSource {
                authority: "agent-ix".to_owned(),
                identity: case.artifacts.qsl_source.path.clone(),
                namespace: "generated".to_owned(),
                revision: "ir-453".to_owned(),
                bytes: case.artifacts.qsl_source.contents.as_bytes().to_vec(),
            },
            dependencies: vec![DependencyLock {
                identity: DEPENDENCY_NAME.to_owned(),
                package_id: dependency_package_id,
                source: dependency_source,
            }],
            accounting_limits: ScalarLimits {
                integer_bits: maximum,
                decimal_digits: maximum,
                scale_expansion: maximum,
                text_input_bytes: maximum,
                text_scalars: maximum,
                normalized_scalars: maximum,
                unit_edges: maximum,
                value_occurrences: maximum,
                work_units: maximum,
                result_units: maximum,
            },
            stage_limits: BTreeMap::new(),
            declared_domains: Vec::new(),
            replay_limits: ReplayLimits::default(),
        },
        function,
    )
}

/// Why a counterexample is evidence failure: the backend's evidence and the native replay do not
/// establish the same typed violation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceFailureCause {
    /// The parsed playback did not decode against the obligation's schema or names another harness.
    Decode(DecodeFailure),
    /// A decoded value lies outside its argument's declared domain.
    Domain {
        /// The argument the value is bound to.
        argument: String,
    },
    /// The selected QSL package/site differs from the one that created this Kani identity.
    PackageIdentity,
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
            | CallSiteRefusal::NotAnOperationOwner { .. }
            | CallSiteRefusal::AmbiguousOperation { .. }
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
            | SpineReplayError::Identity(_)
            | SpineReplayError::Packet(_) => Self::CgDefect,
            #[cfg(test)]
            SpineReplayError::NoInputValue | SpineReplayError::PackageIdentity => Self::CgDefect,
            SpineReplayError::Refused(refusal) => Self::Refused(refusal),
        }
    }
}

impl<'a> From<&'a EvidenceFailureCause> for ReplaySettlement<'a> {
    fn from(cause: &'a EvidenceFailureCause) -> Self {
        match cause {
            // A playback that does not type against the persisted bindings, and a value outside
            // the harness proof bound, are this repository's defects.
            EvidenceFailureCause::Decode(_)
            | EvidenceFailureCause::Domain { .. }
            | EvidenceFailureCause::PackageIdentity => Self::CgDefect,
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

/// Decodes `playback` against `identity`, checks every value against its declared domain,
/// replays it natively through `package` and reports the verdict. The package must first retain
/// this exact proving-run identity with [`ReplayPackage::with_kani_identity`]. The caller parses
/// the complete captured text through [`crate::parse_assertion_playback`]; replay rechecks the
/// parsed source length and value count against the package's proving-run limits.
///
/// # Errors
///
/// [`SpineReplayError`] when the adapter cannot build an admitted request or QSL refuses it;
/// those are not verdicts on the evidence.
pub fn replay_counterexample(
    identity: &KaniObligationIdentity,
    playback: &Playback<'_>,
    package: &ReplayPackage,
) -> Result<ReplayVerdict, SpineReplayError> {
    replay_counterexample_through(identity, playback, package, |wire| {
        replay(wire, package.inputs.replay_limits)
    })
}

/// Replays a corpus harness's real Kani playback against the exact QSL function source retained
/// by that case. The QSL obligation identity is derived from the package's selected `FunctionSite`,
/// not from a fabricated clause identity.
///
/// # Errors
///
/// [`SpineReplayError`] when the playback cannot be decoded or QSL refuses the replay request.
pub(crate) fn replay_harness_counterexample(
    identity: &ReplayHarnessIdentity,
    playback: &Playback<'_>,
    package: &ReplayPackage,
) -> Result<ReplayVerdict, SpineReplayError> {
    replay_harness_counterexample_through(identity, playback, package, |wire| {
        replay(wire, package.inputs.replay_limits)
    })
}

/// [`replay_harness_counterexample`] with an injected executor, for recording the exact typed
/// request QSL receives in focused tests.
pub(crate) fn replay_harness_counterexample_through(
    identity: &ReplayHarnessIdentity,
    playback: &Playback<'_>,
    package: &ReplayPackage,
    mut execute: impl FnMut(ReplayRequestWire) -> Result<ReplayResult, ReplayRefusal>,
) -> Result<ReplayVerdict, SpineReplayError> {
    if !identity.matches_package(package) {
        return Ok(ReplayVerdict::EvidenceFailure(
            EvidenceFailureCause::PackageIdentity,
        ));
    }
    let module = identity
        .module_symbol
        .as_ref()
        .map_or("", crate::core::identity::ModuleSymbol::as_str);
    let decoded = match decode_playback(
        identity.harness_symbol.as_str(),
        module,
        &identity.arguments,
        playback,
        package.inputs.decode_limits,
    ) {
        Ok(decoded) => decoded,
        Err(cause) => {
            return Ok(ReplayVerdict::EvidenceFailure(
                EvidenceFailureCause::Decode(cause),
            ))
        }
    };
    if let Some(argument) = first_out_of_domain(&identity.arguments, &decoded.values) {
        return Ok(ReplayVerdict::EvidenceFailure(
            EvidenceFailureCause::Domain {
                argument: argument.to_owned(),
            },
        ));
    }
    let obligation = package
        .obligation_identity_for(
            crate::kani::identity::ObligationKind::Postcondition,
            &identity.arguments,
        )
        .map_err(SpineReplayError::Identity)?;
    let result = replay_falsification_through(
        &identity.qualified_harness(),
        &decoded.check_text,
        &decoded.values,
        &package.parameters(),
        |source| {
            let packet = backend_function_packet(
                package,
                obligation,
                identity.function,
                identity.declaration.clone(),
                source,
            );
            package
                .request_from_packet(packet, obligation)
                .map_err(SpineReplayError::Packet)
        },
        &mut execute,
    )?;
    Ok(verdict_of(
        result.settlement(),
        result.category(),
        result.disagreement(),
    ))
}

/// [`replay_counterexample`] with the request handed to `execute` instead of straight to
/// [`qsl_replay::replay`]: a caller that records or wraps the request QSL receives passes a
/// closure that ends in `qsl_replay::replay`. A different or absent retained proving-run binding
/// yields [`EvidenceFailureCause::PackageIdentity`] before decoding or invoking `execute`.
///
/// # Errors
///
/// As [`replay_counterexample`].
pub fn replay_counterexample_through(
    identity: &KaniObligationIdentity,
    playback: &Playback<'_>,
    package: &ReplayPackage,
    mut execute: impl FnMut(ReplayRequestWire) -> Result<ReplayResult, ReplayRefusal>,
) -> Result<ReplayVerdict, SpineReplayError> {
    let failure = |cause| Ok(ReplayVerdict::EvidenceFailure(cause));
    if !package.matches_kani_identity(identity) {
        return failure(EvidenceFailureCause::PackageIdentity);
    }
    let values = match decode_falsification(
        identity.harness_symbol.as_str(),
        identity.module_symbol.as_str(),
        &identity.arguments,
        playback,
        package.inputs.decode_limits,
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
        |source| {
            let packet = backend_function_packet(
                package,
                obligation,
                package.site.site.function,
                package.site.site.declaration.clone(),
                source,
            );
            package
                .request_from_packet(packet, obligation)
                .map_err(SpineReplayError::Packet)
        },
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
    use crate::kani::generate::corpus::bounded_kani_corpus::tests::false_cases;
    use crate::kani::output::playback::parse_assertion_playback;
    use crate::kani::{
        abi::{KaniBindingRole, KaniIntegerBounds, KaniPrimitiveType},
        identity::ObligationBinding,
    };
    use ix_trace_rs::trace;
    use qsl_replay::{
        NoPayload, OccurrenceKey, ReplayRequest, ReplayRequestRefusal, ReplayTargetMembers,
        WireNodeId, WitnessPacket,
    };
    use quire_contract_model::{IntegerDomain, OverflowPolicy};

    const UNIT: &str = "language \"ix:native\" edition \"1-draft\";\n\
        profile v = \"quire.value.complete/v1\";\n\
        function f using v(a: Int[0, 9], b: Int[0, 9]): Boolean pure { a <= b }\n";

    /// A source-bound typed identity rejects a different case package and a transcript naming
    /// another harness before any replay verdict is produced.
    #[trace("TC-023")]
    #[test]
    fn tc_023_source_replay_rejects_wrong_package_and_harness() {
        let cases = false_cases();
        let case = &cases[0];
        let transcript = format!(
            "/// Test generated for harness `{}`\n\
             /// Check for `assertion`: \"synthetic assertion\"\n\
             #[test]\n\
             fn kani_concrete_playback_synthetic() {{\n\
                 let concrete_vals: Vec<Vec<u8>> = vec![];\n\
                 kani::concrete_playback_run(concrete_vals, synthetic);\n\
             }}\n",
            case.harness_symbol.as_str()
        );
        let parsed = parse_assertion_playback(
            &transcript,
            case.harness_symbol.as_str(),
            "corpus",
            transcript.len(),
            0,
        )
        .unwrap();
        assert!(case
            .replay_rejects_other_case_package(&parsed, &cases[1])
            .unwrap());
        assert!(case.replay_rejects_wrong_harness(&parsed).unwrap());
    }

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
            decode_limits: WitnessDecodeLimits {
                bytes: 1_048_576,
                values: 1024,
            },
            package_name: "fixture".to_owned(),
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

    fn producer_packet(
        package: &ReplayPackage,
        obligation: ObligationIdentity,
    ) -> WitnessPacket<NoPayload> {
        let package_id = package.site.package_id;
        WitnessPacket {
            obligation_identity: Some(*obligation.as_bytes()),
            occurrence_key: Some(package.site.site.declaration.clone()),
            clause_node: Some(package.site.site.function),
            replay_target: ReplayTargetMembers::function(package.selection.clone()),
            package_id: Some((
                Some(package_id.domain().as_str().to_owned()),
                package_id.hex(),
            )),
            source_digests: Some(vec![package.inputs.source.wire()]),
            profile_selections: Some(Vec::new()),
            run_limits: Some(package.inputs.accounting_limits),
            declared_domains: Some(package.inputs.declared_domains.clone()),
            backend: Some(BACKEND_IDENTITY.to_owned()),
            trace_position: Some(None),
            source: Some(EnvelopeSource::Witness(
                render_witness("function_fixture", "falsified", &[])
                    .expect("the fixture witness renders"),
            )),
            family_payload: Some(NoPayload),
        }
    }

    /// A producer packet is admitted and bound to the independent function claim before
    /// request construction. Shape-valid packets for another source, package or subject refuse.
    #[trace("FR-024-AC-43", "FR-024-AC-46", "TC-035")]
    #[test]
    fn tc_035_function_request_requires_its_producer_packet() {
        let package = package();
        let obligation = package
            .obligation_identity_for(
                crate::kani::identity::ObligationKind::Postcondition,
                &[binding("a"), binding("b")],
            )
            .expect("both arguments bind");
        let packet = producer_packet(&package, obligation);
        let request = package
            .request_from_packet(packet.clone(), obligation)
            .expect("the matching witness packet is admitted");
        assert_eq!(
            request.replay_target.occurrence_key.as_ref(),
            packet.occurrence_key.as_ref()
        );
        let readback = ReplayRequest::decode(request, package.inputs.replay_limits)
            .expect("QSL reads the packet-backed request");
        assert_eq!(
            readback
                .replay_target()
                .and_then(ReplayTarget::occurrence_key),
            packet.occurrence_key.as_ref()
        );

        let mut wrong_arm = packet.clone();
        wrong_arm.source = Some(EnvelopeSource::Input(Vec::new()));
        assert!(matches!(
            package.request_from_packet(wrong_arm, obligation),
            Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::SourceArm
            ))
        ));

        let mut wrong_package = packet.clone();
        wrong_package.package_id = Some((
            Some(package.site.package_id.domain().as_str().to_owned()),
            "ab".repeat(32),
        ));
        assert!(matches!(
            package.request_from_packet(wrong_package, obligation),
            Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Package
            ))
        ));

        let mut wrong_subject = packet;
        wrong_subject.clause_node =
            Some(WireNodeId::from_hex(&"44".repeat(32)).expect("the test node id is valid"));
        assert!(matches!(
            package.request_from_packet(wrong_subject, obligation),
            Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Subject
            ))
        ));

        let mut wrong_occurrence = producer_packet(&package, obligation);
        wrong_occurrence.occurrence_key = Some(OccurrenceKey::new(
            package.site.site.function,
            qsl_replay::Origin::new(qsl_replay::Role::new("declaration"), 1),
        ));
        assert!(matches!(
            package.request_from_packet(wrong_occurrence, obligation),
            Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Occurrence
            ))
        ));

        let mut wrong_target = producer_packet(&package, obligation);
        wrong_target.replay_target = ReplayTargetMembers::function(
            QualifiedName::new(vec![Identifier::new("other").expect("valid name")])
                .expect("valid selection"),
        );
        assert!(matches!(
            package.request_from_packet(wrong_target, obligation),
            Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Target
            ))
        ));

        let mut wrong_obligation = producer_packet(&package, obligation);
        wrong_obligation.obligation_identity = Some([0x55; 32]);
        assert!(matches!(
            package.request_from_packet(wrong_obligation, obligation),
            Err(FunctionPacketError::ClaimMismatch(
                PacketClaimMember::Obligation
            ))
        ));

        let mut missing_occurrence = producer_packet(&package, obligation);
        missing_occurrence.occurrence_key = None;
        assert!(matches!(
            package.request_from_packet(missing_occurrence, obligation),
            Err(FunctionPacketError::Admission(
                WitnessRefusal::MissingMember("occurrence_key")
            ))
        ));
    }

    /// The function request carries QSL's selected declaration occurrence.
    /// Packet-sourced provenance is a separate FR-024-AC-43 obligation.
    #[trace("TC-035")]
    #[test]
    fn tc_035_function_request_carries_selected_site_and_occurrence() {
        let package = package();
        let obligation = package
            .obligation_identity_for(
                crate::kani::identity::ObligationKind::Postcondition,
                &[binding("a"), binding("b")],
            )
            .expect("both arguments bind");
        let request = package.request_for_input_oracle(obligation);
        assert_eq!(
            request.replay_target.members.function.as_ref(),
            Some(&package.selection)
        );
        assert!(request.replay_target.members.frame_operation.is_none());
        assert!(request.replay_target.members.state_clause.is_none());
        assert_eq!(
            request.replay_target.occurrence_key.as_ref(),
            Some(&package.site.site.declaration)
        );
    }

    /// QSL rejects a missing function occurrence before checking provided bytes.
    #[trace("FR-024-AC-45", "TC-035")]
    #[test]
    fn tc_035_missing_function_request_occurrence_refuses_before_byte_provision() {
        let package = package();
        let obligation = package
            .obligation_identity_for(
                crate::kani::identity::ObligationKind::Postcondition,
                &[binding("a"), binding("b")],
            )
            .expect("both arguments bind");
        let request = package.request_for_input_oracle(obligation);
        assert!(!request.byte_provision.is_empty());
        for drop_bytes in [false, true] {
            let mut missing = request.clone();
            missing.replay_target.occurrence_key = None;
            if drop_bytes {
                missing.byte_provision.clear();
            }
            let refusal = ReplayRequest::decode(missing, package.inputs.replay_limits)
                .expect_err("missing occurrence must refuse before replay");
            assert!(matches!(
                &refusal,
                ReplayRequestRefusal::MissingOccurrenceKey
            ));
            assert_eq!(refusal.code(), qsl_replay::Code::InvalidRuntimeInput);
            assert_eq!(refusal.cause(), Some("missing-member"));
            assert_eq!(refusal.pointer(), Some("/replay/occurrence_key"));
        }
    }

    /// Both integer-domain endpoints enter the function executor. Values immediately outside
    /// either endpoint retain the named argument and never call it.
    #[trace("TC-035", "FR-024-AC-3")]
    #[test]
    fn tc_035_function_domain_boundaries_stop_before_replay() {
        let package = package();
        let harness = crate::core::identity::HarnessSymbol::try_from("domain_check")
            .expect("valid harness symbol");
        let identity = package
            .harness_identity(None, harness, vec![binding("a"), binding("b")])
            .expect("the function binds both arguments");
        let playback = |a: i64, b: i64| {
            let entries = [a, b]
                .iter()
                .map(|value| {
                    let bytes = value
                        .to_le_bytes()
                        .iter()
                        .map(u8::to_string)
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("        // {value}\n        vec![{bytes}],\n")
                })
                .collect::<String>();
            format!(
                "/// Test generated for harness `domain_check`\n\
                 /// Check for `assertion`: \"domain check\"\n\
                 #[test]\n\
                 fn kani_concrete_playback_check_1() {{\n\
                 \x20   let concrete_vals: Vec<Vec<u8>> = vec![\n{entries}    ];\n\
                 \x20   kani::concrete_playback_run(concrete_vals, check);\n\
                 }}\n"
            )
        };
        for (argument, a, b, expected_domain_failure) in [
            ("a", 0, 9, false),
            ("a", 9, 9, false),
            ("a", -1, 9, true),
            ("a", 10, 9, true),
            ("b", 0, 0, false),
            ("b", 0, 9, false),
            ("b", 0, -1, true),
            ("b", 0, 10, true),
        ] {
            let mut calls = 0;
            let transcript = playback(a, b);
            let parsed = parse_assertion_playback(
                &transcript,
                "function-replay-test",
                "domain",
                transcript.len(),
                2,
            )
            .expect("complete playback parses");
            let result =
                replay_harness_counterexample_through(&identity, &parsed, &package, |wire| {
                    calls += 1;
                    replay(wire, package.inputs.replay_limits)
                });
            if expected_domain_failure {
                assert_eq!(calls, 0, "{argument}={a},{b} must stop before replay");
                assert_eq!(
                    result.expect("domain failure is an evidence verdict"),
                    ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Domain {
                        argument: argument.to_owned(),
                    })
                );
            } else {
                assert_eq!(calls, 1, "{argument}={a},{b} must reach replay: {result:?}");
                assert!(result.is_ok(), "{argument}={a},{b}: {result:?}");
            }
        }
    }

    /// With the function, declaration, kind and arguments fixed, reordering `FunctionSite.parameters`
    /// leaves the identity unchanged, and a perturbed `function` or `declaration` changes it.
    #[trace("FR-016-AC-23", "TC-026")]
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
    #[trace("FR-016-AC-13", "TC-026")]
    #[test]
    fn only_a_reproduced_violation_reproduces() {
        use WitnessSettlement::{Inconclusive, ReproducedWithEvaluatedWitness as Reproduced};
        let category: quire_semantic_value::catalog::Category = Category::Violation;
        assert_eq!(
            verdict_of(Reproduced, category, None),
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

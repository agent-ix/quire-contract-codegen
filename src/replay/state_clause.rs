//! Native replay of a postcondition state-clause counterexample: a Kani falsification of an
//! operation-contract harness (FR-015-AC-26) is put before QSL's `replay_state_clause` as a
//! request and an envelope built here (FR-024-AC-11 to AC-19).
//!
//! The clause's identities are not computed by this crate: [`qsl_replay::call_site`] compiles the
//! proved unit, with its domain packages and dependencies, and names the clause, so the envelope's
//! `clause_node` and `occurrence_key` are QSL's own answer for the clause name. The payload carries
//! neither identity (QSL FR-122).
//!
//! The invocation document and the two snapshot documents the replay reads are built here from the
//! decoded playback and the caller's post-state values. Each is a typed record whose bytes and
//! `sha256-jcs` digest come from [`crate::core::canonical`] and nowhere else (AD-004): this module
//! orders no member and rewrites no string.
//!
//! The package is read by the harness generator's own reader (`kani::generate::frame`): the
//! clause, its anchor, the framed object and each field's admitted model range, so replay and
//! generation cannot disagree about one package. This module adds the clause's parameter names
//! to tell the supported operation shape from an unsupported one.
//!
//! A state field of a model declaration comes from the caller's ordered field list, checked
//! against the admitted model table. An `IntRange` fitting `i64` is checked and becomes a
//! `DeclaredDomain`; a present field without one remains an integer without an assumption.

use std::{collections::BTreeMap, fmt};

use qsl_replay::{
    call_site, replay_state_clause, CallSiteRefusal, Category, ClauseName, ClauseSelectionInput,
    DeclaredDomain, DigestDomain, DigestRecord, DocumentRef, DomainKey, EmptyQualifiedName,
    FiniteBound, Integer, MalformedTranscript, OperationName, ProofBound, QualifiedName,
    ReplayRefusal, ReplayRequestWire, ReplayResult, ReplaySource, StateClauseCounterexample,
    StateClauseReplayResult, WireNodeId, WitnessEnvelope, WitnessPacket, WitnessRefusal,
    WitnessSettlement,
};
use quire_canonical::{Encode, FixedShape};
use quire_contract_model::{CheckedModelFieldsError, CheckedNodeId, CheckedPackageV2};
use serde::{Deserialize, Serialize};

use crate::{
    core::canonical::{content_bytes, content_digest, DigestError},
    kani::{
        generate::{
            frame::{field_range, ClauseShape, Graph},
            outcome::StateFrameRefusal,
        },
        terminal::ReplaySettlement,
    },
    replay::function::{render_witness, DependencyLockError, ProvidedDocument, ReplayInputs},
};

/// The state object's address in the snapshots: the population that holds it, its key in that
/// population, and its object type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateObjectAddress {
    /// The population's identity.
    pub population: String,
    /// The object's key in the population.
    pub key: String,
    /// The object type's identity.
    pub object_type: String,
}

/// The identity label of one document: its authority, identity and revision. Its digest is
/// computed from the document's bytes, not supplied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentLabel {
    /// The document's authority.
    pub authority: String,
    /// The document's identity.
    pub identity: String,
    /// The namespace its revision is named in.
    pub revision_namespace: String,
    /// The revision.
    pub revision: String,
}

/// Everything one state-clause replay needs beyond the proved unit.
#[derive(Clone, Debug)]
pub struct StateClauseReplayInputs<'a> {
    /// The proving run's lock. A state-clause replay selects by the clause, so no function is
    /// named.
    pub run: ReplayInputs,
    /// The domain package documents the unit's `model` declarations select: the whole byte
    /// provision this replay is given beside the documents it builds.
    pub packages: Vec<ProvidedDocument>,
    /// The admitted package the harness was generated from.
    pub package: &'a CheckedPackageV2,
    /// The package's `state`/`state_clause` node of the clause.
    pub clause_node: &'a CheckedNodeId,
    /// The state struct's fields in the harness draw order.
    pub state_fields: Vec<String>,
    /// The operation the clause anchors.
    pub operation: OperationName,
    /// The clause's declared name.
    pub clause: ClauseName,
    /// The state object the single-`self` harness draws.
    pub object: StateObjectAddress,
    /// The invocation document's label.
    pub invocation_label: DocumentLabel,
    /// The pre snapshot's label.
    pub pre_label: DocumentLabel,
    /// The post snapshot's label.
    pub post_label: DocumentLabel,
    /// The decoded playback: the `i64` Kani bound to each state field, the symbolic pre state.
    pub playback: Vec<(String, i64)>,
    /// The post state: the `i64` of each state field after the caller ran the customer's subject
    /// natively over the pre state.
    pub post_state: Vec<(String, i64)>,
    /// The identity of the obligation the counterexample falsified.
    pub obligation_identity: [u8; 32],
}

/// Why the framed model declaration does not supply the requested field list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateClauseModelFieldsCause {
    /// The Contract IR model-fields accessor refused the object.
    Accessor(CheckedModelFieldsError),
    /// A requested field is absent from the accessor's table.
    Absent {
        /// The first absent field in caller order.
        field: String,
    },
}

/// What an operation declares beyond `self`: the parameters and the result the supported shape
/// does not have.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OperationDeclaration {
    /// The declared parameters' names, in declaration order.
    pub parameters: Vec<String>,
    /// Whether the operation declares a result.
    pub result: bool,
}

/// Why the model header of a document could not be taken from the supplied domain packages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModelError {
    /// The state object's type is not an `ix://<package identity>/<name>` address.
    NotAnAddress {
        /// The object type.
        object_type: String,
    },
    /// A supplied package is not a package document: it has no `package` identity and version.
    Unreadable,
    /// A supplied package is addressed by a digest that is not `sha256-jcs`, the domain a model
    /// header names its package by.
    WrongDigestDomain,
    /// No supplied package has an identity that owns the object type.
    NoOwner {
        /// The object type.
        object_type: String,
    },
    /// More than one supplied package has an identity that owns the object type.
    Ambiguous {
        /// The object type.
        object_type: String,
    },
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAnAddress { object_type } => {
                write!(
                    f,
                    "`{object_type}` is not an `ix://<package>/<name>` address"
                )
            }
            Self::Unreadable => f.write_str("a supplied package is not a package document"),
            Self::WrongDigestDomain => {
                f.write_str("a supplied package is not addressed by a `sha256-jcs` digest")
            }
            Self::NoOwner { object_type } => {
                write!(f, "no supplied package owns `{object_type}`")
            }
            Self::Ambiguous { object_type } => {
                write!(f, "more than one supplied package owns `{object_type}`")
            }
        }
    }
}

/// Why a document could not be built or a fact it is built from could not be read.
#[derive(Debug)]
pub enum DocumentError {
    /// The document has no RFC 8785 encoding.
    Encode(DigestError),
    /// The model header could not be taken from the supplied packages.
    Model(ModelError),
    /// The clause, its anchor, its frame or its parameters are not readable from the admitted
    /// package: the harness generator's own refusal of the same node.
    Clause {
        /// The generator's refusal.
        refusal: StateFrameRefusal,
    },
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encode(cause) => cause.fmt(f),
            Self::Model(cause) => cause.fmt(f),
            Self::Clause { refusal } => write!(f, "the clause is not readable: {refusal}"),
        }
    }
}

impl std::error::Error for DocumentError {}

/// Why a state-clause replay produced no result.
#[derive(Debug)]
pub enum StateClauseReplayError {
    /// The lock's dependency selections are not admitted.
    Dependencies(DependencyLockError),
    /// QSL could not compile the unit or find the clause in it.
    CallSite(Box<CallSiteRefusal>),
    /// A qualified name built from the operation's identifiers is not admitted.
    Name(EmptyQualifiedName),
    /// The harness text would change the witness transcript's field boundaries.
    Transcript(MalformedTranscript),
    /// The envelope this adapter built is not one QSL admits.
    Envelope(WitnessRefusal),
    /// A document could not be built.
    Document(DocumentError),
    /// The framed model declaration does not provide the requested field list.
    ModelFields {
        /// The model declaration node.
        object: CheckedNodeId,
        /// The exact accessor error or first missing field.
        cause: StateClauseModelFieldsCause,
    },
    /// The playback or the post state binds no value for a declared state field.
    MissingField {
        /// The field.
        field: String,
    },
    /// The playback or the post state binds a name the framed object does not declare as a field.
    UndeclaredField {
        /// The name.
        field: String,
    },
    /// The playback or the post state binds one state field more than once.
    DuplicateField {
        /// The field.
        field: String,
    },
    /// A playback value lies outside its field's declared integer range.
    OutOfDomain {
        /// The field.
        field: String,
    },
    /// The operation declares a parameter or a result, which the invocation document this path
    /// builds does not carry.
    UnsupportedOperationShape {
        /// The operation.
        operation: OperationName,
        /// What it declares.
        declaration: OperationDeclaration,
    },
    /// QSL refused the request or the envelope before settling a result.
    Refused(Box<ReplayRefusal>),
}

impl fmt::Display for StateClauseReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dependencies(cause) => cause.fmt(f),
            Self::CallSite(refusal) => write!(f, "the clause was not located: {refusal}"),
            Self::Name(cause) => write!(f, "the operation's name is not admitted: {cause}"),
            Self::Transcript(cause) => write!(f, "the witness transcript is not admitted: {cause}"),
            Self::Envelope(cause) => write!(f, "the envelope is not admitted: {cause}"),
            Self::Document(cause) => write!(f, "a document was not built: {cause}"),
            Self::ModelFields { object, cause } => {
                write!(
                    f,
                    "model fields of {} are unavailable: {cause:?}",
                    object.digest
                )
            }
            Self::MissingField { field } => {
                write!(f, "no value is bound for the state field `{field}`")
            }
            Self::UndeclaredField { field } => {
                write!(f, "`{field}` is not a state field of the framed object")
            }
            Self::DuplicateField { field } => {
                write!(f, "the state field `{field}` is bound more than once")
            }
            Self::OutOfDomain { field } => write!(
                f,
                "the value of the state field `{field}` is outside its declared range"
            ),
            Self::UnsupportedOperationShape {
                operation,
                declaration,
            } => write!(
                f,
                "the operation {operation} declares parameters {:?} and a result: {}; the \
                 state-clause replay supports neither",
                declaration.parameters, declaration.result
            ),
            Self::Refused(refusal) => write!(f, "the state-clause replay was refused: {refusal}"),
        }
    }
}

impl std::error::Error for StateClauseReplayError {}

/// A state-clause replay failure, read as the settlement of a falsified run (FR-029-AC-16): the
/// failures this repository raised carry no QSL code and are defects or limits, and the QSL
/// refusals read by their walked content, as the frame path's do.
impl<'a> From<&'a StateClauseReplayError> for ReplaySettlement<'a> {
    fn from(error: &'a StateClauseReplayError) -> Self {
        match error {
            StateClauseReplayError::Dependencies(cause) => cause.into(),
            StateClauseReplayError::CallSite(refusal) => refusal.as_ref().into(),
            StateClauseReplayError::Name(_)
            | StateClauseReplayError::Transcript(_)
            | StateClauseReplayError::Envelope(_)
            | StateClauseReplayError::Document(_)
            | StateClauseReplayError::ModelFields { .. }
            | StateClauseReplayError::MissingField { .. }
            | StateClauseReplayError::UndeclaredField { .. }
            | StateClauseReplayError::DuplicateField { .. }
            | StateClauseReplayError::OutOfDomain { .. }
            | StateClauseReplayError::UnsupportedOperationShape { .. } => Self::CgDefect,
            StateClauseReplayError::Refused(refusal) => Self::Refused(refusal),
        }
    }
}

/// A state-clause replay result, read as the settlement of a falsified run (FR-029-AC-16): a
/// witness-arm agreement in the `violation` category is a reproduction, an inconclusive one is a
/// disagreement with its cause, and anything else cannot be read as either.
impl<'a> From<&'a StateClauseReplayResult> for ReplaySettlement<'a> {
    fn from(result: &'a StateClauseReplayResult) -> Self {
        match result.result() {
            ReplayResult::Witness(arm) => {
                match (arm.settlement(), arm.category(), arm.disagreement()) {
                    (WitnessSettlement::ReproducedWithEvaluatedWitness, Category::Violation, _) => {
                        Self::Reproduced
                    }
                    (WitnessSettlement::Inconclusive, _, Some(cause)) => Self::Disagreement(cause),
                    (WitnessSettlement::Inconclusive, _, None)
                    | (WitnessSettlement::ReproducedWithEvaluatedWitness, _, _) => Self::CgDefect,
                }
            }
            // The envelope is on the witness arm, so the result is: an input-arm result is not
            // one this adapter asked for.
            ReplayResult::Input(_) => Self::CgDefect,
        }
    }
}

/// A state-clause counterexample ready to replay: QSL's request and the packet the envelope is
/// reconstructed from.
pub struct StateClauseReplay {
    /// The request supplying the package reference, byte provision and limits.
    pub wire: ReplayRequestWire,
    /// The envelope's members. `clause_node` and `occurrence_key` are the `ClauseSite`'s.
    pub packet: WitnessPacket<StateClauseCounterexample>,
}

impl StateClauseReplay {
    /// Compiles the proved unit against its domain packages and dependencies, locates the clause
    /// in it, reads the operation's shape and the state object's fields from the admitted package,
    /// builds the invocation and snapshot documents, and builds the request and envelope members.
    ///
    /// # Errors
    ///
    /// [`StateClauseReplayError`] when the lock's dependencies are not admitted, QSL does not
    /// compile the unit or find the clause, the operation declares a parameter or a result, a
    /// state field has no playback or post-state value or its playback value is outside its range,
    /// or a document or the transcript is not admitted.
    pub fn new(inputs: StateClauseReplayInputs<'_>) -> Result<Self, StateClauseReplayError> {
        let StateClauseReplayInputs {
            run,
            packages,
            package,
            clause_node,
            state_fields: requested_fields,
            operation,
            clause,
            object,
            invocation_label,
            pre_label,
            post_label,
            playback,
            post_state,
            obligation_identity,
        } = inputs;
        let (run, dependencies) = run.admit().map_err(StateClauseReplayError::Dependencies)?;
        let located = call_site(
            run.source.source_identity(),
            &run.source.identity,
            &run.source.bytes,
            packages.iter().map(|package| package.bytes.as_slice()),
            &dependencies,
            &clause,
        )
        .map_err(StateClauseReplayError::CallSite)?;
        let package_id = located.package_id;
        let site = located.site;

        let graph = Graph::of(package);
        let shape = ClauseShape::read(&graph, clause_node).map_err(|refusal| {
            StateClauseReplayError::Document(DocumentError::Clause { refusal })
        })?;
        let declaration = operation_declaration(&graph, &shape)?;
        if declaration.result || !declaration.parameters.is_empty() {
            return Err(StateClauseReplayError::UnsupportedOperationShape {
                operation,
                declaration,
            });
        }
        let fields = state_fields(&graph, &shape, &requested_fields)?;
        let pre = bind(&fields, &playback, Side::Playback)?;
        let post = bind(&fields, &post_state, Side::PostState)?;
        let domains = declared_domains(&shape, &fields)?;

        let model = model_header(&packages, &object.object_type)
            .map_err(|cause| StateClauseReplayError::Document(DocumentError::Model(cause)))?;
        let pre_snapshot = snapshot(&model, &object, &pre_label, "pre", &pre)
            .map_err(StateClauseReplayError::Document)?;
        let post_snapshot = snapshot(&model, &object, &post_label, "post", &post)
            .map_err(StateClauseReplayError::Document)?;
        let invocation_document = InvocationDocument {
            format: "quire.state.invocation/v1",
            identity: (&invocation_label).into(),
            model: model.clone(),
            context: object.object_type.clone(),
            operation: operation.operation.as_str().to_owned(),
            self_object: ObjectRef {
                population: object.population.clone(),
                key: object.key.clone(),
            },
            pre: pre_snapshot.link(&pre_label),
            post: post_snapshot.link(&post_label),
            parameters: BTreeMap::new(),
            result: None,
            created: Vec::new(),
            deleted: Vec::new(),
        };
        let invocation_encoded = encode_document(&invocation_document)
            .map_err(|cause| StateClauseReplayError::Document(DocumentError::Encode(cause)))?;
        let invocation = invocation_label.reference(invocation_encoded.digest);

        let selected = QualifiedName::new(vec![operation.operation.clone()])
            .map_err(StateClauseReplayError::Name)?;
        let bindings = pre
            .iter()
            .map(|(field, value)| (field.as_str(), *value))
            .collect::<Vec<_>>();
        let witness = render_witness(&operation.to_string(), clause.0.as_str(), &bindings)
            .map_err(StateClauseReplayError::Transcript)?;
        let payload = StateClauseCounterexample {
            clause: clause.0.clone(),
            observation: ClauseSelectionInput::Invocation { invocation },
            witness: None,
        };
        let state_documents = [
            (&invocation_encoded.digest, &invocation_encoded.bytes),
            (&pre_snapshot.digest, &pre_snapshot.bytes),
            (&post_snapshot.digest, &post_snapshot.bytes),
        ]
        .map(|(digest, bytes)| (DigestRecord::mint(DigestDomain::Sha256Jcs, *digest), bytes));
        let documents = packages
            .iter()
            .map(|document| (document.digest, document.bytes.as_slice()))
            .chain(
                state_documents
                    .iter()
                    .map(|(digest, bytes)| (*digest, bytes.as_slice())),
            )
            .collect::<Vec<_>>();
        let wire = run.wire(
            package_id,
            selected.clone(),
            ReplaySource::Witness(witness.clone()),
            obligation_identity,
            &documents,
        );
        let packet = WitnessPacket {
            obligation_identity: Some(obligation_identity),
            occurrence_key: Some(site.occurrence),
            clause_node: Some(site.node),
            selected_function: Some(selected),
            package_id: Some(wire.package_id.clone()),
            source_digests: Some(wire.source_digests.clone()),
            profile_selections: Some(Vec::new()),
            run_limits: Some(run.accounting_limits),
            declared_domains: Some(domains),
            backend: Some(wire.backend.clone()),
            trace_position: Some(None),
            source: Some(ReplaySource::Witness(witness)),
            family_payload: Some(payload),
        };
        Ok(Self { wire, packet })
    }

    /// Replays the counterexample through [`qsl_replay::replay_state_clause`].
    ///
    /// # Errors
    ///
    /// As [`Self::replay_through`].
    pub fn replay(self) -> Result<StateClauseReplayResult, StateClauseReplayError> {
        self.replay_through(replay_state_clause)
    }

    /// Admits the envelope and hands the request and the envelope to `executor`, returning the
    /// executor's result as it is. [`Self::replay`] passes `qsl_replay::replay_state_clause`; a
    /// caller that observes or wraps the call passes a function that ends in it.
    ///
    /// # Errors
    ///
    /// [`StateClauseReplayError::Envelope`] when the packet is no admitted envelope (the executor
    /// is not called), and [`StateClauseReplayError::Refused`] when the executor refuses.
    pub fn replay_through(
        self,
        executor: impl FnOnce(
            ReplayRequestWire,
            &WitnessEnvelope<StateClauseCounterexample>,
        ) -> Result<StateClauseReplayResult, ReplayRefusal>,
    ) -> Result<StateClauseReplayResult, StateClauseReplayError> {
        let envelope =
            WitnessEnvelope::reconstruct(self.packet).map_err(StateClauseReplayError::Envelope)?;
        executor(self.wire, &envelope)
            .map_err(|refusal| StateClauseReplayError::Refused(Box::new(refusal)))
    }
}

// ---------------------------------------------------------------------------
// Documents
// ---------------------------------------------------------------------------

/// A document's canonical bytes and the digest of those bytes, both from [`crate::core::canonical`].
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct EncodedDocument {
    /// The RFC 8785 bytes.
    pub(crate) bytes: Vec<u8>,
    /// The digest `sha256-jcs` addresses the document by.
    pub(crate) digest: [u8; 32],
}

/// Encodes `document` and computes its digest, each only through [`crate::core::canonical`].
///
/// # Errors
///
/// [`DigestError`] when `document` has no RFC 8785 encoding.
pub(crate) fn encode_document<T: Encode + ?Sized>(
    document: &T,
) -> Result<EncodedDocument, DigestError> {
    Ok(EncodedDocument {
        bytes: content_bytes(document)?,
        digest: content_digest(document)?,
    })
}

impl DocumentLabel {
    fn reference(&self, digest: [u8; 32]) -> DocumentRef {
        DocumentRef {
            authority: self.authority.clone(),
            identity: self.identity.clone(),
            revision_namespace: self.revision_namespace.clone(),
            revision: self.revision.clone(),
            digest,
        }
    }
}

/// A document's identity member.
#[derive(Clone, Deserialize, Serialize, FixedShape)]
pub(crate) struct IdentityMember {
    authority: String,
    identity: String,
    revision_namespace: String,
    revision: String,
}

impl From<&DocumentLabel> for IdentityMember {
    fn from(label: &DocumentLabel) -> Self {
        Self {
            authority: label.authority.clone(),
            identity: label.identity.clone(),
            revision_namespace: label.revision_namespace.clone(),
            revision: label.revision.clone(),
        }
    }
}

/// The model header of a document: the domain package it is over.
#[derive(Clone, Serialize, FixedShape)]
struct ModelHeader {
    identity: String,
    version: String,
    digest: String,
}

/// An integer value as the snapshot grammar spells it: a decimal string.
///
/// The leaf shapes below (`IntegerValue`, `SnapshotObject`, `SnapshotPopulation`, `ObjectRef`,
/// `SnapshotLink`) are written and read through one definition: the frame replay reads the
/// invocation and pre snapshot it ties to a playback with them (`replay::frame`).
#[derive(Deserialize, Serialize, FixedShape)]
pub(crate) struct IntegerValue {
    pub(crate) integer: String,
}

#[derive(Deserialize, Serialize, FixedShape)]
pub(crate) struct SnapshotObject {
    pub(crate) key: String,
    #[serde(rename = "type")]
    object_type: String,
    pub(crate) fields: BTreeMap<String, IntegerValue>,
}

#[derive(Deserialize, Serialize, FixedShape)]
pub(crate) struct SnapshotPopulation {
    pub(crate) population: String,
    complete: bool,
    pub(crate) objects: Vec<SnapshotObject>,
}

#[derive(Serialize, FixedShape)]
struct SnapshotDocument {
    format: &'static str,
    identity: IdentityMember,
    observation: &'static str,
    model: ModelHeader,
    populations: Vec<SnapshotPopulation>,
}

/// An object named by its population and key.
#[derive(Deserialize, Serialize, FixedShape)]
pub(crate) struct ObjectRef {
    pub(crate) population: String,
    pub(crate) key: String,
}

/// A snapshot as the invocation names it: its identity and `sha256-jcs` digest.
#[derive(Deserialize, Serialize, FixedShape)]
pub(crate) struct SnapshotLink {
    identity: IdentityMember,
    pub(crate) digest: String,
}

#[derive(Serialize, FixedShape)]
struct InvocationDocument {
    format: &'static str,
    identity: IdentityMember,
    model: ModelHeader,
    context: String,
    operation: String,
    #[serde(rename = "self")]
    self_object: ObjectRef,
    pre: SnapshotLink,
    post: SnapshotLink,
    parameters: BTreeMap<String, IntegerValue>,
    result: Option<IntegerValue>,
    created: Vec<ObjectRef>,
    deleted: Vec<ObjectRef>,
}

/// An encoded snapshot.
struct Snapshot {
    bytes: Vec<u8>,
    digest: [u8; 32],
}

impl Snapshot {
    fn link(&self, label: &DocumentLabel) -> SnapshotLink {
        SnapshotLink {
            identity: label.into(),
            digest: digest_text(&DigestRecord::mint(DigestDomain::Sha256Jcs, self.digest)),
        }
    }
}

/// A digest as a document member spells it: `<domain>:<64 hex digits>`.
fn digest_text(record: &DigestRecord) -> String {
    format!("{}:{}", record.domain().as_str(), record.hex())
}

/// The snapshot of `object` with `values` in `observation`'s role: one `complete` population
/// holding the one object, every declared field an integer.
fn snapshot(
    model: &ModelHeader,
    object: &StateObjectAddress,
    label: &DocumentLabel,
    observation: &'static str,
    values: &[(String, i64)],
) -> Result<Snapshot, DocumentError> {
    let document = SnapshotDocument {
        format: "quire.state.snapshot/v1",
        identity: label.into(),
        observation,
        model: model.clone(),
        populations: vec![SnapshotPopulation {
            population: object.population.clone(),
            complete: true,
            objects: vec![SnapshotObject {
                key: object.key.clone(),
                object_type: object.object_type.clone(),
                fields: values
                    .iter()
                    .map(|(field, value)| {
                        (
                            field.clone(),
                            IntegerValue {
                                integer: value.to_string(),
                            },
                        )
                    })
                    .collect(),
            }],
        }],
    };
    let EncodedDocument { bytes, digest } =
        encode_document(&document).map_err(DocumentError::Encode)?;
    Ok(Snapshot { bytes, digest })
}

/// The members of a domain package document this module reads: the `package` identity and version
/// a model header names. The document itself is QSL's to validate.
#[derive(Deserialize)]
struct DomainPackageHeader {
    package: DomainPackageIdentity,
}

#[derive(Deserialize)]
struct DomainPackageIdentity {
    identity: String,
    version: String,
}

/// The model header of the one supplied package whose identity owns `object_type`
/// (`ix://<identity>/<name>`). Every supplied package must be a package document addressed by a
/// `sha256-jcs` digest, the domain a model header names its package by.
fn model_header(
    packages: &[ProvidedDocument],
    object_type: &str,
) -> Result<ModelHeader, ModelError> {
    let owner = object_type
        .strip_prefix("ix://")
        .ok_or_else(|| ModelError::NotAnAddress {
            object_type: object_type.to_owned(),
        })?;
    let mut owners = Vec::new();
    for package in packages {
        if package.digest.domain() != DigestDomain::Sha256Jcs {
            return Err(ModelError::WrongDigestDomain);
        }
        let header = serde_json::from_slice::<DomainPackageHeader>(&package.bytes)
            .map_err(|_| ModelError::Unreadable)?;
        let identity = header.package.identity;
        if owner
            .strip_prefix(identity.as_str())
            .is_some_and(|rest| rest.starts_with('/'))
        {
            owners.push(ModelHeader {
                identity,
                version: header.package.version,
                digest: digest_text(&package.digest),
            });
        }
    }
    let mut owners = owners.into_iter();
    match (owners.next(), owners.next()) {
        (Some(header), None) => Ok(header),
        (None, _) => Err(ModelError::NoOwner {
            object_type: object_type.to_owned(),
        }),
        (Some(_), Some(_)) => Err(ModelError::Ambiguous {
            object_type: object_type.to_owned(),
        }),
    }
}

// ---------------------------------------------------------------------------
// State fields
// ---------------------------------------------------------------------------

/// One state field of the framed object and the inclusive range its member declares, if any.
struct StateField {
    name: String,
    range: Option<(i64, i64)>,
}

/// Which supplied values are being bound.
#[derive(Clone, Copy)]
enum Side {
    /// The decoded playback: range-checked.
    Playback,
    /// The post state: whatever the subject ran to, not range-checked.
    PostState,
}

/// The framed object's fields and ranges. A model declaration uses the caller's draw order after
/// checking every name against the admitted table; a non-declaration uses body-member order.
/// A present field without a representable `i64` range remains unranged.
fn state_fields(
    graph: &Graph<'_>,
    shape: &ClauseShape,
    requested: &[String],
) -> Result<Vec<StateField>, StateClauseReplayError> {
    let object = &shape.scope.object;
    let body_members = graph
        .nodes
        .get(object)
        .and_then(|node| node.body.get("members")?.as_array())
        .map(Vec::as_slice)
        .unwrap_or_default();
    match graph.package.model_object_fields(object) {
        Ok(fields) => {
            let mut resolved = Vec::with_capacity(requested.len());
            for name in requested {
                if fields.field(name).is_none() {
                    return Err(StateClauseReplayError::ModelFields {
                        object: object.clone(),
                        cause: StateClauseModelFieldsCause::Absent {
                            field: name.clone(),
                        },
                    });
                }
                resolved.push(StateField {
                    name: name.clone(),
                    range: field_range(graph, object, name).ok(),
                });
            }
            return Ok(resolved);
        }
        Err(CheckedModelFieldsError::NotModelObjectType) if !body_members.is_empty() => {}
        Err(cause) => {
            return Err(StateClauseReplayError::ModelFields {
                object: object.clone(),
                cause: StateClauseModelFieldsCause::Accessor(cause),
            });
        }
    }
    let members = graph
        .nodes
        .get(object)
        .and_then(|node| node.body.get("members")?.as_array())
        .map(Vec::as_slice)
        .unwrap_or_default();
    Ok(members
        .iter()
        .filter_map(|member| member.get("name")?.as_str())
        .map(|name| StateField {
            name: name.to_owned(),
            range: field_range(graph, object, name).ok(),
        })
        .collect())
}

/// The value bound to each declared field in `values`, in declaration order. A name the object
/// does not declare and a field bound twice are refused before any field is read; on the
/// playback, a value must also lie in its field's declared range.
fn bind(
    fields: &[StateField],
    values: &[(String, i64)],
    side: Side,
) -> Result<Vec<(String, i64)>, StateClauseReplayError> {
    for (position, (name, _)) in values.iter().enumerate() {
        if !fields.iter().any(|field| field.name == *name) {
            return Err(StateClauseReplayError::UndeclaredField {
                field: name.clone(),
            });
        }
        if values.iter().take(position).any(|(seen, _)| seen == name) {
            return Err(StateClauseReplayError::DuplicateField {
                field: name.clone(),
            });
        }
    }
    fields
        .iter()
        .map(|field| {
            let (_, value) = values
                .iter()
                .find(|(name, _)| *name == field.name)
                .ok_or_else(|| StateClauseReplayError::MissingField {
                    field: field.name.clone(),
                })?;
            if let (Side::Playback, Some((minimum, maximum))) = (side, field.range) {
                if !(minimum..=maximum).contains(value) {
                    return Err(StateClauseReplayError::OutOfDomain {
                        field: field.name.clone(),
                    });
                }
            }
            Ok((field.name.clone(), *value))
        })
        .collect()
}

/// One `DeclaredDomain` per state field that declares a range: the range, on `self`'s parameter
/// node, at the field's child index among the framed object's members.
fn declared_domains(
    shape: &ClauseShape,
    fields: &[StateField],
) -> Result<Vec<DeclaredDomain>, StateClauseReplayError> {
    let unreadable = |at: &CheckedNodeId| {
        StateClauseReplayError::Document(DocumentError::Clause {
            refusal: StateFrameRefusal::MalformedClause { at: at.clone() },
        })
    };
    let Some(self_parameter) = shape.parameters.first() else {
        return Err(unreadable(&shape.scope.anchor));
    };
    let parameter =
        WireNodeId::from_hex(&self_parameter.digest).ok_or_else(|| unreadable(self_parameter))?;
    let mut domains = Vec::new();
    for (position, field) in fields.iter().enumerate() {
        let Some((minimum, maximum)) = field.range else {
            continue;
        };
        let path = u32::try_from(position).map_err(|_| unreadable(self_parameter))?;
        let bound = FiniteBound::integer_range(Integer::from(minimum), Integer::from(maximum))
            .map_err(|_| unreadable(&shape.scope.object))?;
        domains.push(DeclaredDomain::new(ProofBound {
            domain: DomainKey::Node {
                node: parameter,
                path: vec![path],
            },
            bound,
        }));
    }
    Ok(domains)
}

// ---------------------------------------------------------------------------
// The admitted package
// ---------------------------------------------------------------------------

/// What the clause's operation declares beyond `self` (FR-341: the clause binds `self` at level
/// 0, then `result` when the operation has one, then its parameters in declared order). The
/// clause's parameter nodes come from the generator's `ClauseShape`; each node's `name` and
/// `level` are read here. A parameter named `result` at level 1 is the result.
fn operation_declaration(
    graph: &Graph<'_>,
    shape: &ClauseShape,
) -> Result<OperationDeclaration, StateClauseReplayError> {
    let mut declaration = OperationDeclaration::default();
    let declared = shape.declared_parameters(graph).map_err(|at| {
        StateClauseReplayError::Document(DocumentError::Clause {
            refusal: StateFrameRefusal::MalformedClause { at },
        })
    })?;
    for (name, level) in declared {
        if name == "result" && level == "1" {
            declaration.result = true;
        } else {
            declaration.parameters.push(name);
        }
    }
    Ok(declaration)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A document whose members are in non-sorted source order, whose text holds characters JSON
    /// escapes and whose numbers include a large integer, a fraction and an exponent form.
    #[derive(Serialize, FixedShape)]
    struct Vector {
        zebra: &'static str,
        alpha: &'static str,
        large: i64,
        fraction: f64,
        exponent: f64,
        middle: Option<()>,
    }

    fn vector() -> Vector {
        Vector {
            zebra: "quote \" backslash \\ control \u{1} letter \u{e9}",
            alpha: "first",
            large: 9_007_199_254_740_991,
            fraction: 0.25,
            exponent: 1.0e21,
            middle: None,
        }
    }

    /// FR-024-AC-14: the builder returns the bytes and digest `core::canonical` returns, and the
    /// bytes are RFC 8785's: members by UTF-16 code unit, JSON escapes, ES6 number spellings.
    ///
    /// Trace: FR-024-AC-14, TC-035
    #[test]
    fn tc_035_the_builder_returns_what_core_canonical_returns_for_the_vector() {
        let document = vector();
        let built = encode_document(&document).expect("the vector encodes");
        assert_eq!(built.bytes, content_bytes(&document).expect("bytes"));
        assert_eq!(built.digest, content_digest(&document).expect("digest"));
        let expected = "{\"alpha\":\"first\",\"exponent\":1e+21,\"fraction\":0.25,\
            \"large\":9007199254740991,\"middle\":null,\
            \"zebra\":\"quote \\\" backslash \\\\ control \\u0001 letter \u{e9}\"}";
        assert_eq!(String::from_utf8(built.bytes).expect("UTF-8"), expected);
    }

    fn package(bytes: &[u8], domain: DigestDomain) -> ProvidedDocument {
        ProvidedDocument {
            digest: DigestRecord::mint(domain, [7; 32]),
            bytes: bytes.to_vec(),
        }
    }

    const BANK: &[u8] = br#"{"package":{"identity":"test/bank","version":"1.0.0"}}"#;

    /// The model header names the one package that owns the object type, by its `sha256-jcs`
    /// digest; an unreadable package, a package under another digest domain, a type nobody owns,
    /// a type that is no address, a package whose identity only shares a prefix, and two owners
    /// are each their own error.
    ///
    /// Trace: FR-024-AC-13, TC-035
    #[test]
    fn tc_035_the_model_header_is_the_one_owner_or_a_distinct_error() {
        let bank = package(BANK, DigestDomain::Sha256Jcs);
        let header = model_header(std::slice::from_ref(&bank), "ix://test/bank/Account")
            .expect("one package owns the type");
        assert_eq!(header.identity, "test/bank");
        assert_eq!(header.version, "1.0.0");
        assert_eq!(header.digest, format!("sha256-jcs:{}", "07".repeat(32)));

        let error = |packages: &[ProvidedDocument], object_type: &str| {
            model_header(packages, object_type)
                .err()
                .expect("the header is refused")
        };
        let owned = "ix://test/bank/Account";
        assert_eq!(
            error(&[package(b"not json", DigestDomain::Sha256Jcs)], owned),
            ModelError::Unreadable
        );
        assert_eq!(
            error(&[package(BANK, DigestDomain::IrCanonical)], owned),
            ModelError::WrongDigestDomain
        );
        assert_eq!(
            error(std::slice::from_ref(&bank), "ix://other/Thing"),
            ModelError::NoOwner {
                object_type: "ix://other/Thing".to_owned()
            }
        );
        assert_eq!(
            error(std::slice::from_ref(&bank), "test/bank/Account"),
            ModelError::NotAnAddress {
                object_type: "test/bank/Account".to_owned()
            }
        );
        let prefix = package(
            br#"{"package":{"identity":"test/ban","version":"1.0.0"}}"#,
            DigestDomain::Sha256Jcs,
        );
        assert_eq!(
            error(&[prefix], owned),
            ModelError::NoOwner {
                object_type: owned.to_owned()
            }
        );
        assert_eq!(
            error(&[bank.clone(), bank], owned),
            ModelError::Ambiguous {
                object_type: owned.to_owned()
            }
        );
    }

    /// FR-024-AC-14: the production source of this module holds no encoder, no hashing, no member
    /// sort and no escaping routine of its own, and `sha2` is not a dependency.
    ///
    /// Trace: FR-024-AC-14, TC-035
    #[test]
    fn tc_035_the_module_encodes_and_hashes_only_through_core_canonical() {
        let source = include_str!("state_clause.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("the source has a production part")
            .replace("use quire_canonical::{Encode, FixedShape};", "");
        for banned in [
            "quire_canonical",
            "sha2::",
            "extern crate sha2",
            "Sha256::",
            "ByteDigest::of",
            ".sort",
            "sort_by",
            "escape",
            "to_vec(",
        ] {
            assert!(
                !production.contains(banned),
                "the production source names `{banned}`"
            );
        }
        let manifest = include_str!("../../Cargo.toml");
        let dependencies = manifest
            .split("[dependencies]")
            .nth(1)
            .and_then(|rest| rest.split("\n[").next())
            .expect("the manifest has a dependencies table");
        assert!(!dependencies.contains("sha2 ="));
        assert!(!dependencies.contains("sha2="));
    }
}

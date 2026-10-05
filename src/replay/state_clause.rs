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
//! The package reading in this module is its own and small: the clause's parameter aggregate (to
//! tell a supported operation shape from an unsupported one), its anchor's `context` object and
//! that object's integer fields with the range each declares. The generator of the harness reads
//! the same facts for its own purpose.

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
use quire_contract_model::{CheckedNodeId, CheckedPackageV2, CheckedSemanticNodeV2};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    core::canonical::{content_bytes, content_digest, DigestError},
    kani::terminal::ReplaySettlement,
    replay::{
        frame::ProvidedDocument,
        function::{render_witness, DependencyLockError, ReplayInputs},
    },
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

/// What an operation declares beyond `self`: the parameters and the result the supported shape
/// does not have.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OperationDeclaration {
    /// The declared parameters' names, in declaration order.
    pub parameters: Vec<String>,
    /// Whether the operation declares a result.
    pub result: bool,
}

/// Why a document could not be built or a fact it is built from could not be read.
#[derive(Debug)]
pub enum DocumentError {
    /// The document has no RFC 8785 encoding.
    Encode(DigestError),
    /// No supplied domain package declares the state object's model, or the one that does is not
    /// a package document.
    Model {
        /// The object type the model was sought for.
        object_type: String,
    },
    /// The clause, its anchor, its parameters or its framed object are not readable from the
    /// admitted package as the state-clause path reads them.
    Clause {
        /// The node that could not be read.
        at: CheckedNodeId,
    },
    /// A field of the framed object declares no usable integer range.
    NoRange {
        /// The field.
        field: String,
    },
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encode(cause) => cause.fmt(f),
            Self::Model { object_type } => write!(
                f,
                "no supplied domain package declares the model of `{object_type}`"
            ),
            Self::Clause { at } => write!(f, "the package node {} is not readable", at.digest),
            Self::NoRange { field } => {
                write!(f, "the field `{field}` declares no usable integer range")
            }
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
    /// The playback or the post state binds no value for a declared state field.
    MissingField {
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
            Self::MissingField { field } => {
                write!(f, "no value is bound for the state field `{field}`")
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
            | StateClauseReplayError::MissingField { .. }
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
        let read = graph.read_clause(clause_node).ok_or_else(|| {
            StateClauseReplayError::Document(DocumentError::Clause {
                at: clause_node.clone(),
            })
        })?;
        if read.declaration.result || !read.declaration.parameters.is_empty() {
            return Err(StateClauseReplayError::UnsupportedOperationShape {
                operation,
                declaration: read.declaration,
            });
        }
        let fields = graph
            .read_fields(&read.object)
            .map_err(StateClauseReplayError::Document)?;
        let pre = bind(&fields, &playback, true)?;
        let post = bind(&fields, &post_state, false)?;
        let domains = declared_domains(&read, &fields)?;

        let model = model_header(&packages, &object.object_type)
            .map_err(StateClauseReplayError::Document)?;
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
#[derive(Clone, Serialize, FixedShape)]
struct IdentityMember {
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
#[derive(Serialize, FixedShape)]
struct IntegerValue {
    integer: String,
}

#[derive(Serialize, FixedShape)]
struct SnapshotObject {
    key: String,
    #[serde(rename = "type")]
    object_type: String,
    fields: BTreeMap<String, IntegerValue>,
}

#[derive(Serialize, FixedShape)]
struct SnapshotPopulation {
    population: String,
    complete: bool,
    objects: Vec<SnapshotObject>,
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
#[derive(Serialize, FixedShape)]
struct ObjectRef {
    population: String,
    key: String,
}

/// A snapshot as the invocation names it: its identity and `sha256-jcs` digest.
#[derive(Serialize, FixedShape)]
struct SnapshotLink {
    identity: IdentityMember,
    digest: String,
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
            digest: format!("{}:{}", DigestDomain::Sha256Jcs.as_str(), hex(&self.digest)),
        }
    }
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
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
/// (`ix://<identity>/<name>`).
fn model_header(
    packages: &[ProvidedDocument],
    object_type: &str,
) -> Result<ModelHeader, DocumentError> {
    let missing = || DocumentError::Model {
        object_type: object_type.to_owned(),
    };
    let owner = object_type.strip_prefix("ix://").ok_or_else(missing)?;
    let mut owners = packages.iter().filter_map(|package| {
        let header = serde_json::from_slice::<DomainPackageHeader>(&package.bytes).ok()?;
        owner
            .strip_prefix(header.package.identity.as_str())
            .is_some_and(|rest| rest.starts_with('/'))
            .then(|| ModelHeader {
                identity: header.package.identity,
                version: header.package.version,
                digest: format!(
                    "{}:{}",
                    package.digest.domain().as_str(),
                    package.digest.hex()
                ),
            })
    });
    match (owners.next(), owners.next()) {
        (Some(header), None) => Ok(header),
        (None, _) | (Some(_), Some(_)) => Err(missing()),
    }
}

// ---------------------------------------------------------------------------
// State fields
// ---------------------------------------------------------------------------

/// One state field of the framed object and the inclusive range it declares.
struct StateField {
    name: String,
    minimum: i64,
    maximum: i64,
}

/// The value bound to each declared field in `values`, in declaration order. A playback value
/// (`check_range`) must also lie in the field's declared range; the post state is whatever the
/// subject ran to and is not range-checked.
fn bind(
    fields: &[StateField],
    values: &[(String, i64)],
    check_range: bool,
) -> Result<Vec<(String, i64)>, StateClauseReplayError> {
    fields
        .iter()
        .map(|field| {
            let (_, value) = values
                .iter()
                .find(|(name, _)| *name == field.name)
                .ok_or_else(|| StateClauseReplayError::MissingField {
                    field: field.name.clone(),
                })?;
            if check_range && !(field.minimum..=field.maximum).contains(value) {
                return Err(StateClauseReplayError::OutOfDomain {
                    field: field.name.clone(),
                });
            }
            Ok((field.name.clone(), *value))
        })
        .collect()
}

/// One `DeclaredDomain` per state field: the field's declared range, on `self`'s parameter node,
/// at the field's child index in the framed object.
fn declared_domains(
    read: &ClauseRead,
    fields: &[StateField],
) -> Result<Vec<DeclaredDomain>, StateClauseReplayError> {
    let clause_error = || {
        StateClauseReplayError::Document(DocumentError::Clause {
            at: read.self_parameter.clone(),
        })
    };
    let parameter = WireNodeId::from_hex(&read.self_parameter.digest).ok_or_else(clause_error)?;
    fields
        .iter()
        .enumerate()
        .map(|(position, field)| {
            let path = u32::try_from(position).map_err(|_| clause_error())?;
            let bound = FiniteBound::integer_range(
                Integer::from(field.minimum),
                Integer::from(field.maximum),
            )
            .map_err(|_| {
                StateClauseReplayError::Document(DocumentError::NoRange {
                    field: field.name.clone(),
                })
            })?;
            Ok(DeclaredDomain::new(ProofBound {
                domain: DomainKey::new(parameter, vec![path]),
                bound,
            }))
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The admitted package
// ---------------------------------------------------------------------------

/// What the clause's node says of its operation and its framed object.
struct ClauseRead {
    declaration: OperationDeclaration,
    self_parameter: CheckedNodeId,
    object: CheckedNodeId,
}

/// The admitted graph by node id.
struct Graph<'g> {
    nodes: BTreeMap<&'g CheckedNodeId, &'g CheckedSemanticNodeV2>,
}

fn node_id(value: &Value) -> Option<CheckedNodeId> {
    serde_json::from_value(value.clone()).ok()
}

/// The node a `reference` term names.
fn target(term: &Value) -> Option<CheckedNodeId> {
    node_id(term.get("target")?)
}

/// The value of the `binding` named `name` among `members`.
fn binding<'v>(members: &'v [Value], name: &str) -> Option<&'v Value> {
    members
        .iter()
        .find(|member| {
            member.get("term").and_then(Value::as_str) == Some("binding")
                && member.get("name").and_then(Value::as_str) == Some(name)
        })
        .and_then(|member| member.get("value"))
}

/// The text of a `literal` term of `kind`.
fn literal<'v>(term: &'v Value, kind: &str) -> Option<&'v str> {
    if term.get("term")?.as_str()? != "literal" || term.get("value_kind")?.as_str()? != kind {
        return None;
    }
    term.get("value")?.as_str()
}

impl<'g> Graph<'g> {
    fn of(package: &'g CheckedPackageV2) -> Self {
        Self {
            nodes: package
                .graph()
                .nodes
                .iter()
                .map(|node| (&node.node_id, node))
                .collect(),
        }
    }

    fn members(&self, id: &CheckedNodeId) -> Option<&'g [Value]> {
        let node: &'g CheckedSemanticNodeV2 = self.nodes.get(id).copied()?;
        node.body.get("members")?.as_array().map(Vec::as_slice)
    }

    /// The clause's parameters (FR-341: `self` at level 0, then `result` when present, then the
    /// operation's parameters), and its anchor's `context` object (FR-342).
    fn read_clause(&self, clause: &CheckedNodeId) -> Option<ClauseRead> {
        let node = self.nodes.get(clause)?;
        let arguments = node.body.get("arguments")?.as_array()?;
        let [parameters, anchor, _condition] = arguments.as_slice() else {
            return None;
        };
        let parameters = parameters
            .get("members")?
            .as_array()?
            .iter()
            .map(target)
            .collect::<Option<Vec<_>>>()?;
        let (self_parameter, declared) = parameters.split_first()?;
        let mut declaration = OperationDeclaration::default();
        for parameter in declared {
            let members = self.members(parameter)?;
            let name = literal(binding(members, "name")?, "text")?;
            let level = literal(binding(members, "level")?, "integer")?;
            if name == "result" && level == "1" {
                declaration.result = true;
            } else {
                declaration.parameters.push(name.to_owned());
            }
        }
        let context = binding(self.members(&target(anchor)?)?, "context")?;
        Some(ClauseRead {
            declaration,
            self_parameter: self_parameter.clone(),
            object: target(context)?,
        })
    }

    /// Every field of the framed object, with the `integer_range` its member references.
    fn read_fields(&self, object: &CheckedNodeId) -> Result<Vec<StateField>, DocumentError> {
        let unreadable = || DocumentError::Clause { at: object.clone() };
        self.members(object)
            .ok_or_else(unreadable)?
            .iter()
            .map(|member| {
                let name = member
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(unreadable)?;
                let no_range = || DocumentError::NoRange {
                    field: name.to_owned(),
                };
                let bound = member
                    .get("value")
                    .and_then(target)
                    .and_then(|id| self.nodes.get(&id).copied())
                    .filter(|node| &*node.semantic_form == "integer_range")
                    .ok_or_else(no_range)?;
                let members = bound.body.get("members").and_then(Value::as_array);
                let bound_value = |key: &str| {
                    let term = binding(members?, key)?;
                    literal(term, "integer")?.parse::<i64>().ok()
                };
                Ok(StateField {
                    name: name.to_owned(),
                    minimum: bound_value("min").ok_or_else(no_range)?,
                    maximum: bound_value("max").ok_or_else(no_range)?,
                })
            })
            .collect()
    }
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

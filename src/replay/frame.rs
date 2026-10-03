//! Native replay of a frame counterexample: a Kani falsification of an operation's frame is put
//! before QSL's `replay_frame` as a request and an envelope built here.
//!
//! The operation's identities (its anchor and frame nodes and the frame's occurrence) are not
//! computed by this crate: [`qsl_replay::call_site`] compiles the proved unit, with its domain
//! packages and dependencies, and names them, so the payload, the envelope's `clause_node` and
//! its `occurrence_key` are all QSL's own answer for the operation. The envelope names the
//! payload's frame node and frame occurrence, the pair `replay_frame` requires it to agree with.

use std::fmt;

use qsl_replay::{
    call_site, replay_frame, CallSiteRefusal, ClaimedChange, DigestRecord, DocumentRef,
    EmptyQualifiedName, FrameCounterexample, FrameOperation, FrameReplayResult,
    MalformedTranscript, OperationName, OperationSite, QualifiedName, ReplayRefusal,
    ReplayRequestWire, ReplaySource, Witness, WitnessEnvelope, WitnessPacket, WitnessRefusal,
};

use crate::replay::function::{DependencyLockError, ReplayInputs};

/// One document the replay reads from the request's byte provision, with the digest the request
/// addresses it by.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedDocument {
    /// The document's digest record: `sha256-jcs` for a domain package or a state document.
    pub digest: DigestRecord,
    /// The document's bytes.
    pub bytes: Vec<u8>,
}

/// Everything one frame replay needs beyond the proved unit.
#[derive(Clone, Debug)]
pub struct FrameReplayInputs {
    /// The proving run's lock. A frame replay selects by the operation, so no function is named.
    pub run: ReplayInputs,
    /// The domain package documents the unit's `model` declarations select.
    pub packages: Vec<ProvidedDocument>,
    /// The invocation document and its pre and post snapshots.
    pub state_documents: Vec<ProvidedDocument>,
    /// The operation whose frame the counterexample refutes.
    pub operation: OperationName,
    /// The `quire.state.invocation/v1` document.
    pub invocation: DocumentRef,
    /// The change the counterexample claims.
    pub change: ClaimedChange,
    /// The identity of the obligation the counterexample falsified.
    pub obligation_identity: [u8; 32],
}

/// Why a frame replay produced no result.
#[derive(Debug)]
pub enum FrameReplayError {
    /// The lock's dependency selections are not admitted.
    Dependencies(DependencyLockError),
    /// QSL could not compile the unit or find the operation's frame in it.
    CallSite(Box<CallSiteRefusal>),
    /// A qualified name built from the operation's identifiers is not admitted.
    Name(EmptyQualifiedName),
    /// The harness text would change the witness transcript's field boundaries.
    Transcript(MalformedTranscript),
    /// The envelope this adapter built is not one QSL admits.
    Envelope(WitnessRefusal),
    /// QSL refused the request or the envelope before settling a result.
    Refused(Box<ReplayRefusal>),
}

impl fmt::Display for FrameReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dependencies(cause) => cause.fmt(f),
            Self::CallSite(refusal) => write!(f, "the operation was not located: {refusal}"),
            Self::Name(cause) => write!(f, "the operation's name is not admitted: {cause}"),
            Self::Transcript(cause) => write!(f, "the witness transcript is not admitted: {cause}"),
            Self::Envelope(cause) => write!(f, "the envelope is not admitted: {cause}"),
            Self::Refused(refusal) => write!(f, "the frame replay was refused: {refusal}"),
        }
    }
}

impl std::error::Error for FrameReplayError {}

/// A frame counterexample ready to replay: QSL's request and the packet the envelope is
/// reconstructed from.
pub struct FrameReplay {
    /// The request supplying the package reference, byte provision and limits.
    pub wire: ReplayRequestWire,
    /// The envelope's members. `clause_node` is the payload's frame node and `occurrence_key`
    /// its frame occurrence.
    pub packet: WitnessPacket<FrameCounterexample>,
}

impl FrameReplay {
    /// Compiles the proved unit against its domain packages and dependencies, locates
    /// `inputs.operation` in it, and builds the request and envelope members for the claimed
    /// change.
    ///
    /// # Errors
    ///
    /// [`FrameReplayError`] when the lock's dependencies are not admitted, QSL does not compile
    /// the unit or find the operation's frame, or the witness transcript is not admitted.
    pub fn new(inputs: FrameReplayInputs) -> Result<Self, FrameReplayError> {
        let FrameReplayInputs {
            run,
            packages,
            state_documents,
            operation,
            invocation,
            change,
            obligation_identity,
        } = inputs;
        let (run, dependencies) = run.admit().map_err(FrameReplayError::Dependencies)?;
        let site = call_site(
            run.source.source_identity(),
            &run.source.identity,
            &run.source.bytes,
            packages.iter().map(|package| package.bytes.as_slice()),
            &dependencies,
            &operation,
        )
        .map_err(FrameReplayError::CallSite)?;
        let OperationSite {
            anchor,
            frame,
            frame_occurrence,
            ..
        } = site.site;
        let object = QualifiedName::new(vec![operation.model.clone(), operation.object.clone()])
            .map_err(FrameReplayError::Name)?;
        let selected = QualifiedName::new(vec![operation.operation.clone()])
            .map_err(FrameReplayError::Name)?;
        let witness = Witness::parse(format!("<<<assertion|{operation}|frame|>>>"))
            .map_err(FrameReplayError::Transcript)?;
        let payload = FrameCounterexample {
            operation: FrameOperation {
                object,
                operation: operation.operation,
            },
            anchor,
            frame,
            occurrence: frame_occurrence,
            invocation,
            change,
        };
        let documents = packages
            .iter()
            .chain(&state_documents)
            .map(|document| (document.digest, document.bytes.as_slice()))
            .collect::<Vec<_>>();
        let wire = run.wire(
            site.package_id,
            selected.clone(),
            ReplaySource::Witness(witness.clone()),
            obligation_identity,
            &documents,
        );
        let packet = WitnessPacket {
            obligation_identity: Some(obligation_identity),
            occurrence_key: Some(payload.occurrence.clone()),
            clause_node: Some(payload.frame),
            selected_function: Some(selected),
            package_id: Some(wire.package_id.clone()),
            source_digests: Some(wire.source_digests.clone()),
            profile_selections: Some(Vec::new()),
            run_limits: Some(run.accounting_limits),
            declared_domains: Some(Vec::new()),
            backend: Some(wire.backend.clone()),
            trace_position: Some(None),
            source: Some(ReplaySource::Witness(witness)),
            family_payload: Some(payload),
        };
        Ok(Self { wire, packet })
    }

    /// Replays the counterexample through [`qsl_replay::replay_frame`].
    ///
    /// # Errors
    ///
    /// [`FrameReplayError::Envelope`] when the packet is no admitted envelope, and
    /// [`FrameReplayError::Refused`] when QSL refuses the request or the envelope.
    pub fn replay(self) -> Result<FrameReplayResult, FrameReplayError> {
        let envelope =
            WitnessEnvelope::reconstruct(self.packet).map_err(FrameReplayError::Envelope)?;
        replay_frame(self.wire, &envelope)
            .map_err(|refusal| FrameReplayError::Refused(Box::new(refusal)))
    }
}

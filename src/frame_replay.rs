//! Native replay of a frame counterexample: a Kani falsification of an operation's frame is put
//! before QSL's `replay_frame` as a request and an envelope built here.
//!
//! The operation's identities (its anchor and frame nodes and the frame's occurrence) are not
//! computed by this crate: [`qsl_replay::call_site`] compiles the proved unit, with its domain
//! packages and dependencies, and names them, so the payload, the envelope's `clause_node` and
//! its `occurrence_key` are all QSL's own answer for the operation. The envelope names the
//! payload's frame node and frame occurrence, the pair `replay_frame` requires it to agree with.
//!
//! The counterexample is the real Kani playback of the frame obligation: [`decode_frame_witness`]
//! types its bytes with the obligation's own bindings, refuses a value outside the domain the
//! harness assumed, and ties the values to the obligation's identity digest, which the envelope
//! then carries. QSL's `replay_frame` reads none of the witness, the identity or the declared
//! domains, so those checks are this crate's.

use std::fmt;

use qsl_replay::{
    call_site, replay_frame, CallSiteRefusal, ClaimedChange, DigestRecord, DocumentRef,
    EmptyQualifiedName, FrameCounterexample, FrameOperation, FrameReplayResult,
    MalformedTranscript, OperationName, OperationSite, QualifiedName, ReplayRefusal,
    ReplayRequestWire, ReplaySource, Witness, WitnessEnvelope, WitnessPacket, WitnessRefusal,
    WitnessValue,
};

use crate::{
    kani_obligations::ObligationKind,
    kani_witness_join::{decode_falsification, first_out_of_domain, DecodeFailure},
    obligation_identity::{obligation_digest, IdentityRefusal, ObligationDigest},
    spine_replay::{DependencyLockError, ReplayInputs},
    state_frame::StateFrameIdentity,
};

/// Why a Kani playback is no witness of a frame obligation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FrameWitnessRefusal {
    /// The obligation is not a frame obligation.
    NotAFrame {
        /// The obligation's kind.
        kind: ObligationKind,
    },
    /// The obligation's identity digest is not the digest of its own clause, kind and bindings.
    IdentityStale(Option<IdentityRefusal>),
    /// The transcript does not decode against the obligation's bindings, or is another
    /// harness's.
    Decode(DecodeFailure),
    /// A decoded value is outside the domain the harness assumed of its binding.
    OutOfDomain {
        /// The binding.
        argument: String,
    },
}

impl fmt::Display for FrameWitnessRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAFrame { kind } => {
                write!(f, "a {kind:?} obligation is not a frame obligation")
            }
            Self::IdentityStale(_) => {
                f.write_str("the obligation's identity is not its clause, kind and bindings'")
            }
            Self::Decode(cause) => {
                write!(f, "the playback does not decode: {}", cause.code)
            }
            Self::OutOfDomain { argument } => {
                write!(f, "`{argument}` is outside the domain the harness assumed")
            }
        }
    }
}

impl std::error::Error for FrameWitnessRefusal {}

/// The values of a Kani playback of a frame obligation, decoded and within the harness's domains,
/// and the identity of the obligation they were decoded under.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameWitness {
    obligation: ObligationDigest,
    harness: String,
    values: Vec<(String, WitnessValue)>,
}

impl FrameWitness {
    /// The identity digest of the obligation the playback was decoded under.
    #[must_use]
    pub fn obligation(&self) -> ObligationDigest {
        self.obligation
    }

    /// The decoded value of binding `argument`.
    #[must_use]
    pub fn integer(&self, argument: &str) -> Option<i64> {
        self.values.iter().find_map(|(name, value)| match value {
            WitnessValue::Integer(integer) if name == argument => Some(*integer),
            WitnessValue::Integer(_) | WitnessValue::Boolean(_) => None,
        })
    }
}

/// Decodes `transcript`, the counterexample of a falsified run of the frame obligation
/// `obligation`, into the values of its bindings.
///
/// # Errors
///
/// [`FrameWitnessRefusal`] when `obligation` is not a frame obligation or its identity is not
/// derived from its own contents, when the transcript is not a playback of this harness over
/// these bindings, or when a value lies outside the range the harness assumed of its binding.
pub fn decode_frame_witness(
    obligation: &StateFrameIdentity,
    transcript: &str,
) -> Result<FrameWitness, FrameWitnessRefusal> {
    if obligation.kind != ObligationKind::Frame {
        return Err(FrameWitnessRefusal::NotAFrame {
            kind: obligation.kind,
        });
    }
    let minted = obligation_digest(&obligation.clause, obligation.kind, &obligation.arguments)
        .map_err(|refusal| FrameWitnessRefusal::IdentityStale(Some(refusal)))?;
    if minted != obligation.obligation_identity {
        return Err(FrameWitnessRefusal::IdentityStale(None));
    }
    let values = decode_falsification(
        &obligation.harness_symbol,
        &obligation.module_symbol,
        &obligation.arguments,
        transcript,
    )
    .map_err(FrameWitnessRefusal::Decode)?;
    if let Some(argument) = first_out_of_domain(&obligation.arguments, &values) {
        return Err(FrameWitnessRefusal::OutOfDomain {
            argument: argument.to_owned(),
        });
    }
    Ok(FrameWitness {
        obligation: minted,
        harness: format!(
            "{}::{}",
            obligation.module_symbol, obligation.harness_symbol
        ),
        values,
    })
}

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
    /// The frame obligation the counterexample falsified.
    pub obligation: StateFrameIdentity,
    /// The decoded playback of that obligation's harness.
    pub witness: FrameWitness,
    /// The identity of the counterexample itself.
    pub counterexample_identity: [u8; 32],
}

/// Why a frame replay produced no result.
#[derive(Debug)]
pub enum FrameReplayError {
    /// The lock's dependency selections are not admitted.
    Dependencies(DependencyLockError),
    /// QSL could not compile the unit or find the operation's frame in it.
    CallSite(Box<CallSiteRefusal>),
    /// The witness was decoded under another obligation than the one the request names.
    ObligationMismatch {
        /// The identity the witness was decoded under.
        witness: ObligationDigest,
        /// The identity of the request's obligation.
        obligation: ObligationDigest,
    },
    /// The obligation is scoped to another operation than the one the request selects.
    OperationMismatch {
        /// The operation the obligation is scoped to.
        obligation: String,
        /// The operation the request selects.
        selected: String,
    },
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
            Self::ObligationMismatch {
                witness,
                obligation,
            } => write!(
                f,
                "the witness was decoded under obligation {witness}, not {obligation}"
            ),
            Self::OperationMismatch {
                obligation,
                selected,
            } => write!(
                f,
                "the obligation is scoped to operation `{obligation}`, not `{selected}`"
            ),
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
            obligation,
            witness,
            counterexample_identity,
        } = inputs;
        if witness.obligation != obligation.obligation_identity {
            return Err(FrameReplayError::ObligationMismatch {
                witness: witness.obligation,
                obligation: obligation.obligation_identity,
            });
        }
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
        if obligation.scope.operation != operation.operation.as_str() {
            return Err(FrameReplayError::OperationMismatch {
                obligation: obligation.scope.operation,
                selected: operation.operation.as_str().to_owned(),
            });
        }
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
        let witness = transcript(&witness)?;
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
            counterexample_identity,
            &documents,
        );
        let packet = WitnessPacket {
            obligation_identity: Some(*obligation.obligation_identity.as_bytes()),
            occurrence_key: Some(payload.occurrence.clone()),
            clause_node: Some(payload.frame),
            selected_function: Some(selected),
            package_id: Some(wire.package_id.clone()),
            package_contract_version: Some(wire.package_contract_version.clone()),
            source_digests: Some(wire.source_digests.clone()),
            profile_selections: Some(Vec::new()),
            run_limits: Some(run.accounting_limits),
            // Blocked on QSL-345: QSL's facade does not yet export `ProofBound`, `DomainKey` and
            // `FiniteBound`, so no `DeclaredDomain` can be built. The harness's real domains are
            // the obligation's `arguments`; `decode_frame_witness` enforces them in this crate.
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

/// The witness transcript of `witness`: one entry per binding, named by the binding's identifier
/// and valued by the decoded value. The entry name becomes the binding's domain key once QSL's
/// key shape lands (QSL-345).
fn transcript(witness: &FrameWitness) -> Result<Witness, FrameReplayError> {
    let entries = witness
        .values
        .iter()
        .map(|(argument, value)| {
            let value = match value {
                WitnessValue::Integer(integer) => *integer,
                WitnessValue::Boolean(boolean) => i64::from(*boolean),
            };
            format!("{argument}={value}")
        })
        .collect::<Vec<_>>();
    Witness::parse(format!(
        "<<<assertion|{}|frame|{}>>>",
        witness.harness,
        entries.join(";")
    ))
    .map_err(FrameReplayError::Transcript)
}

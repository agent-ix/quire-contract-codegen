//! Native replay of a frame counterexample: a Kani falsification of an operation's frame is put
//! before QSL's `replay_frame` as a request and an envelope built here.
//!
//! The operation's identities (its anchor and frame nodes and the frame's occurrence) are not
//! computed by this crate: [`qsl_replay::call_site`] compiles the proved unit, with its domain
//! packages and dependencies, and names them, so the payload, the envelope's `clause_node` and
//! its `occurrence_key` are all QSL's own answer for the operation. The envelope names the
//! payload's frame node and frame occurrence, the pair `replay_frame` requires it to agree with.
//!
//! The obligation identity is minted here from that same site and the falsified harness's
//! `StateFrameIdentity` (`replay::obligation`), never supplied. The witness is the real playback:
//! decoded against the harness's `state_fields` by the one decoder and rendered by the one adapter
//! rendering function. The harness is tied to the replay by checks, not by the identity: the
//! property is a frame over exactly the state fields, the scope names the operation, anchor and
//! frame the call site names, every decoded value is in its field's range, and the pre snapshot
//! the invocation names holds the decoded values.
//!
//! The envelope's `declared_domains` is empty until QSL-345 settles the declared-domain key and
//! refuses an empty declaration; no key shape is adopted here before then (FR-024-AC-29).

use std::{collections::BTreeMap, fmt};

use qsl_replay::{
    call_site, replay_frame, CallSiteRefusal, ClaimedChange, DigestDomain, DigestRecord,
    DocumentRef, EmptyQualifiedName, FrameCounterexample, FrameOperation, FrameReplayResult,
    MalformedTranscript, OperationName, OperationSite, QualifiedName, ReplayLimits, ReplayRefusal,
    ReplayRequest, ReplayRequestWire, ReplaySource, WireNodeId, WitnessEnvelope, WitnessPacket,
    WitnessRefusal, WitnessValue,
};
use serde::Deserialize;

use crate::{
    kani::{
        abi::{KaniBindingRole, KaniPrimitiveType},
        identity::{ObligationBinding, StateFrameIdentity, StateFrameProperty, StateUnrangedField},
        output::playback::DecodeFailure,
        terminal::ReplaySettlement,
    },
    replay::{
        function::{render_witness, DependencyLockError, ProvidedDocument, ReplayInputs},
        obligation::{frame_identity, frame_kind, ObligationIdentityError},
        state_clause::{IntegerValue, ObjectRef, SnapshotLink, SnapshotPopulation},
        witness::decode_playback,
    },
};

/// Everything one frame replay needs beyond the proved unit. It names no obligation identity and
/// no witness transcript: both are built from the harness and its playback.
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
    /// The identity of the falsified frame harness, as the generator persisted it.
    pub harness: StateFrameIdentity,
    /// The falsified run's concrete-playback text, as the backend printed it.
    pub playback: String,
}

/// The member of the harness's scope that is not the one the replay names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeMember {
    /// The operation's name.
    Operation,
    /// The operation's `operation_anchor` node.
    Anchor,
    /// The operation's `frame` node.
    Frame,
}

impl fmt::Display for ScopeMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Operation => "operation",
            Self::Anchor => "anchor",
            Self::Frame => "frame",
        })
    }
}

/// Why the decoded pre state is not the pre snapshot the invocation names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreStateFault {
    /// The invocation document is not among the provided documents.
    InvocationNotProvided,
    /// The invocation document is not JSON of the shape `state_clause` writes, or does not
    /// address its state object by population and key or name its pre snapshot.
    InvocationUnreadable,
    /// The pre snapshot the invocation names, by a `sha256-jcs` digest, is not among the provided
    /// documents.
    PreNotProvided {
        /// The digest the invocation names, as lowercase hexadecimal.
        digest: String,
    },
    /// The pre snapshot is not JSON.
    PreUnreadable,
    /// The pre snapshot holds no such object in a population of that name.
    ObjectMissing {
        /// The population the invocation's `self` names.
        population: String,
        /// The object key the invocation's `self` names.
        key: String,
    },
    /// The object holds no value for the state field.
    FieldMissing {
        /// The state field.
        field: String,
    },
    /// The object's value for the state field is no `i64` integer.
    FieldNotInteger {
        /// The state field.
        field: String,
    },
    /// The snapshot's value is not the one the playback decoded.
    Differs {
        /// The state field.
        field: String,
        /// The value the playback decoded.
        decoded: i64,
        /// The value the pre snapshot holds.
        snapshot: i64,
    },
}

impl fmt::Display for PreStateFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvocationNotProvided => f.write_str("the invocation is not a provided document"),
            Self::InvocationUnreadable => {
                f.write_str("the invocation is not a readable invocation document")
            }
            Self::PreNotProvided { digest } => {
                write!(f, "the pre snapshot `{digest}` is not a provided document")
            }
            Self::PreUnreadable => f.write_str("the pre snapshot is not readable JSON"),
            Self::ObjectMissing { population, key } => {
                write!(
                    f,
                    "the pre snapshot holds no object `{key}` of `{population}`"
                )
            }
            Self::FieldMissing { field } => {
                write!(f, "the pre snapshot's object holds no value for `{field}`")
            }
            Self::FieldNotInteger { field } => {
                write!(f, "the pre snapshot's `{field}` is not an `i64` integer")
            }
            Self::Differs {
                field,
                decoded,
                snapshot,
            } => write!(
                f,
                "the playback decoded `{field}` as {decoded} and the pre snapshot holds {snapshot}"
            ),
        }
    }
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
    /// QSL refused the request or the envelope before settling a result. The persisted unranged
    /// fields are context, not a claim that any one of them caused QSL's refusal.
    Refused {
        /// QSL's original refusal and catalog code.
        refusal: Box<ReplayRefusal>,
        /// Fields the harness drew without an `i64` domain, with their persisted reasons.
        unranged: Vec<StateUnrangedField>,
    },
    /// The harness's property is not a frame.
    NotAFrame,
    /// The harness's granted and checked fields are not exactly its state fields.
    FieldSetMismatch {
        /// Every field the harness draws, in draw order.
        state_fields: Vec<String>,
        /// The fields the frame grants.
        granted: Vec<String>,
        /// The fields the frame checks unchanged.
        checked: Vec<String>,
    },
    /// The playback does not decode against the harness.
    Decode(DecodeFailure),
    /// A decoded value lies outside its state field's declared range.
    OutOfDomain {
        /// The state field.
        field: String,
        /// The decoded value.
        value: i64,
    },
    /// The decoded pre state is not the pre snapshot the invocation names.
    PreState(PreStateFault),
    /// The harness's scope is not the operation, anchor and frame the replay names.
    ScopeMismatch {
        /// The member that differs.
        member: ScopeMember,
        /// What the harness's scope holds.
        harness: String,
        /// What the replay names.
        named: String,
    },
    /// The frame obligation has no identity.
    Identity(ObligationIdentityError),
}

impl fmt::Display for FrameReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dependencies(cause) => cause.fmt(f),
            Self::CallSite(refusal) => write!(f, "the operation was not located: {refusal}"),
            Self::Name(cause) => write!(f, "the operation's name is not admitted: {cause}"),
            Self::Transcript(cause) => write!(f, "the witness transcript is not admitted: {cause}"),
            Self::Envelope(cause) => write!(f, "the envelope is not admitted: {cause}"),
            Self::Refused { refusal, unranged } => write!(
                f,
                "the frame replay was refused: {refusal}; harness unranged fields: {unranged:?}"
            ),
            Self::NotAFrame => f.write_str("the harness's property is not a frame"),
            Self::FieldSetMismatch {
                state_fields,
                granted,
                checked,
            } => write!(
                f,
                "the frame grants {granted:?} and checks {checked:?}, not the state fields \
                 {state_fields:?}"
            ),
            Self::Decode(cause) => write!(
                f,
                "the playback does not decode against the harness: {} ({}: {})",
                cause.code, cause.source_id, cause.context
            ),
            Self::OutOfDomain { field, value } => {
                write!(
                    f,
                    "the decoded `{field}` = {value} is outside its declared range"
                )
            }
            Self::PreState(fault) => {
                write!(f, "the pre state is not tied to the invocation: {fault}")
            }
            Self::ScopeMismatch {
                member,
                harness,
                named,
            } => write!(
                f,
                "the harness's {member} is `{harness}` and the replay names `{named}`"
            ),
            Self::Identity(cause) => write!(f, "the frame obligation has no identity: {cause}"),
        }
    }
}

impl std::error::Error for FrameReplayError {}

/// A frame replay failure, read as the settlement of a falsified run (FR-029): the failures this
/// repository raised carry no QSL code and are defects, and the QSL refusals read by their walked
/// content.
impl<'a> From<&'a FrameReplayError> for ReplaySettlement<'a> {
    fn from(error: &'a FrameReplayError) -> Self {
        match error {
            FrameReplayError::Dependencies(cause) => cause.into(),
            FrameReplayError::CallSite(refusal) => refusal.as_ref().into(),
            FrameReplayError::Name(_)
            | FrameReplayError::Transcript(_)
            | FrameReplayError::Envelope(_)
            | FrameReplayError::NotAFrame
            | FrameReplayError::FieldSetMismatch { .. }
            | FrameReplayError::Decode(_)
            | FrameReplayError::OutOfDomain { .. }
            | FrameReplayError::PreState(_)
            | FrameReplayError::ScopeMismatch { .. }
            | FrameReplayError::Identity(_) => Self::CgDefect,
            FrameReplayError::Refused { refusal, .. } => Self::Refused(refusal),
        }
    }
}

/// A frame counterexample ready to replay: QSL's request and the packet the envelope is
/// reconstructed from.
pub struct FrameReplay {
    /// The request supplying the package reference, byte provision and limits.
    pub wire: ReplayRequestWire,
    /// The envelope's members. `clause_node` is the payload's frame node and `occurrence_key`
    /// its frame occurrence.
    pub packet: WitnessPacket<FrameCounterexample>,
    /// The caller's configured replay reader bound.
    replay_limits: ReplayLimits,
    /// Persisted context for QSL refusals; an unranged field is not necessarily their cause.
    unranged: Vec<StateUnrangedField>,
}

/// A frame harness is a frame over its state only when its granted and checked fields, taken
/// together, are exactly its state fields: a field neither granted nor checked, one both granted
/// and checked and one named though the harness does not draw it each refuse. A state field
/// listed twice is refused by the decoder (`cg_witness_schema_duplicate_binding`) and by
/// `StateFrameIdentity::from_record`.
fn field_set_matches(harness: &StateFrameIdentity) -> Result<(), FrameReplayError> {
    let StateFrameProperty::Frame { granted, checked } = &harness.property else {
        return Err(FrameReplayError::NotAFrame);
    };
    let mut named = granted.iter().chain(checked).collect::<Vec<_>>();
    named.sort();
    let mut drawn = harness.state_fields.iter().collect::<Vec<_>>();
    drawn.sort();
    if named == drawn {
        Ok(())
    } else {
        Err(FrameReplayError::FieldSetMismatch {
            state_fields: harness.state_fields.clone(),
            granted: granted.clone(),
            checked: checked.clone(),
        })
    }
}

/// The values the playback decodes to, one `i64` per state field in draw order, each inside its
/// declared range, with the check text the decode names. A field with no declared range is
/// decoded and not range-checked.
fn decoded_state(
    harness: &StateFrameIdentity,
    playback: &str,
) -> Result<(String, Vec<(String, i64)>), FrameReplayError> {
    let bindings = harness
        .state_fields
        .iter()
        .map(|field| ObligationBinding {
            identifier: field.clone(),
            role: KaniBindingRole::Argument,
            primitive_type: KaniPrimitiveType::I64,
            integer_bounds: None,
            dependencies: Vec::new(),
        })
        .collect::<Vec<_>>();
    let decoded = decode_playback(
        harness.harness_symbol.as_str(),
        harness.module_symbol.as_str(),
        &bindings,
        playback,
    )
    .map_err(FrameReplayError::Decode)?;
    let values = decoded
        .values
        .into_iter()
        .map(|(field, value)| {
            // Every binding above is an `i64`, so the decoder yields only integers; the Boolean
            // arm is the encoding QSL admits for a Boolean (as the function path replays one)
            // and keeps this match free of a wildcard.
            let integer = match value {
                WitnessValue::Integer(integer) => i64::try_from(integer).map_err(|_| {
                    FrameReplayError::Decode(DecodeFailure::new(
                        "kani_witness_i64_overflow",
                        harness.harness_symbol.as_str(),
                        &field,
                    ))
                })?,
                WitnessValue::Boolean(boolean) => i64::from(boolean),
                _ => {
                    return Err(FrameReplayError::Decode(DecodeFailure::new(
                        "kani_witness_unsupported_value",
                        harness.harness_symbol.as_str(),
                        &field,
                    )))
                }
            };
            Ok((field, integer))
        })
        .collect::<Result<Vec<_>, FrameReplayError>>()?;
    let out_of_range = values.iter().find(|(field, value)| {
        harness
            .domains
            .iter()
            .find(|domain| domain.field == *field)
            .is_some_and(|domain| !(domain.minimum..=domain.maximum).contains(value))
    });
    match out_of_range {
        Some((field, value)) => Err(FrameReplayError::OutOfDomain {
            field: field.clone(),
            value: *value,
        }),
        None => Ok((decoded.check_text, values)),
    }
}

/// The provided document addressed by the `sha256-jcs` digest `hex`.
fn provided<'a>(documents: &'a [ProvidedDocument], hex: &str) -> Option<&'a [u8]> {
    documents
        .iter()
        .find(|document| {
            document.digest.domain() == DigestDomain::Sha256Jcs && document.digest.hex() == hex
        })
        .map(|document| document.bytes.as_slice())
}

/// The members of the invocation document this check reads, as `state_clause` writes them.
#[derive(Deserialize)]
struct InvocationRead {
    #[serde(rename = "self")]
    self_object: ObjectRef,
    pre: SnapshotLink,
}

/// The members of a snapshot document this check reads, as `state_clause` writes them.
#[derive(Deserialize)]
struct SnapshotRead {
    populations: Vec<SnapshotPopulation>,
}

/// The fields of the state object the invocation's `self` addresses, in the pre snapshot it
/// names. A document is read only after QSL's own request decode has checked every provided
/// document's bytes against its digest (`FrameReplay::new`), so a document whose bytes do not
/// match its digest is QSL's refusal and never reaches this reader. A document that is absent or
/// not of the written shape is this check's refusal: it cannot tie a pre state it cannot read.
fn pre_fields(
    invocation: &DocumentRef,
    documents: &[ProvidedDocument],
) -> Result<BTreeMap<String, IntegerValue>, PreStateFault> {
    let own = DigestRecord::mint(DigestDomain::Sha256Jcs, invocation.digest).hex();
    let bytes = provided(documents, &own).ok_or(PreStateFault::InvocationNotProvided)?;
    let InvocationRead {
        self_object: ObjectRef { population, key },
        pre,
    } = serde_json::from_slice(bytes).map_err(|_| PreStateFault::InvocationUnreadable)?;
    let digest =
        pre.digest
            .strip_prefix("sha256-jcs:")
            .ok_or_else(|| PreStateFault::PreNotProvided {
                digest: pre.digest.clone(),
            })?;
    let snapshot = provided(documents, digest).ok_or_else(|| PreStateFault::PreNotProvided {
        digest: digest.to_owned(),
    })?;
    let SnapshotRead { populations } =
        serde_json::from_slice(snapshot).map_err(|_| PreStateFault::PreUnreadable)?;
    populations
        .into_iter()
        .filter(|candidate| candidate.population == population)
        .flat_map(|candidate| candidate.objects)
        .find(|object| object.key == key)
        .map(|object| object.fields)
        .ok_or(PreStateFault::ObjectMissing { population, key })
}

/// Checks that the pre snapshot the invocation names holds, for the object its `self` addresses,
/// the value the playback decoded for every state field. This is a check of the documents the
/// caller supplied against the run, not a document built here.
fn tie_pre_state(
    invocation: &DocumentRef,
    documents: &[ProvidedDocument],
    values: &[(String, i64)],
) -> Result<(), PreStateFault> {
    let fields = pre_fields(invocation, documents)?;
    values.iter().try_for_each(|(field, decoded)| {
        let held = fields
            .get(field)
            .ok_or_else(|| PreStateFault::FieldMissing {
                field: field.clone(),
            })?;
        let snapshot = held
            .integer
            .parse::<i64>()
            .map_err(|_| PreStateFault::FieldNotInteger {
                field: field.clone(),
            })?;
        if snapshot == *decoded {
            Ok(())
        } else {
            Err(PreStateFault::Differs {
                field: field.clone(),
                decoded: *decoded,
                snapshot,
            })
        }
    })
}

/// `member` of the harness's scope against the node the call site names.
fn same_node(
    member: ScopeMember,
    harness: &quire_contract_model::CheckedNodeId,
    named: WireNodeId,
) -> Result<(), FrameReplayError> {
    if WireNodeId::from_hex(&harness.digest) == Some(named) {
        Ok(())
    } else {
        Err(FrameReplayError::ScopeMismatch {
            member,
            harness: harness.digest.to_string(),
            named: named.to_string(),
        })
    }
}

impl FrameReplay {
    /// Checks the harness against the operation and its playback, compiles the proved unit
    /// against its domain packages and dependencies, locates `inputs.operation` in it, and builds
    /// the request and envelope members for the claimed change.
    ///
    /// The harness's property must be a frame over exactly its state fields, its scope must name
    /// the operation requested, its playback must decode against its state fields, and every
    /// decoded value must lie in its field's declared range, all before the unit is compiled. The
    /// call site's anchor and frame must then be the harness's, and the pre snapshot the
    /// invocation names must hold the decoded values. The obligation identity and the witness
    /// transcript are built here from the call site, the harness and the decoded playback.
    ///
    /// # Errors
    ///
    /// [`FrameReplayError`] when a check above fails, the lock's dependencies are not admitted,
    /// QSL does not compile the unit or find the operation's frame, or the witness transcript is
    /// not admitted.
    pub fn new(inputs: FrameReplayInputs) -> Result<Self, FrameReplayError> {
        let FrameReplayInputs {
            run,
            packages,
            state_documents,
            operation,
            invocation,
            change,
            harness,
            playback,
        } = inputs;
        let kind = frame_kind(&harness.property).ok_or(FrameReplayError::NotAFrame)?;
        field_set_matches(&harness)?;
        if harness.scope.operation != operation.operation.as_str() {
            return Err(FrameReplayError::ScopeMismatch {
                member: ScopeMember::Operation,
                harness: harness.scope.operation,
                named: operation.operation.as_str().to_owned(),
            });
        }
        let (check_text, values) = decoded_state(&harness, &playback)?;
        let (run, dependencies) = run.admit().map_err(FrameReplayError::Dependencies)?;
        let replay_limits = run.replay_limits;
        let located = call_site(
            run.source.source_identity(),
            &run.source.identity,
            &run.source.bytes,
            packages.iter().map(|package| package.bytes.as_slice()),
            &dependencies,
            &operation,
        )
        .map_err(FrameReplayError::CallSite)?;
        let site = located.site;
        same_node(ScopeMember::Anchor, &harness.scope.anchor, site.anchor)?;
        same_node(ScopeMember::Frame, &harness.scope.frame, site.frame)?;
        let obligation = *frame_identity(&site, kind)
            .map_err(FrameReplayError::Identity)?
            .as_bytes();
        let OperationSite {
            anchor,
            frame,
            frame_occurrence,
            ..
        } = site;
        let object = QualifiedName::new(vec![operation.model.clone(), operation.object.clone()])
            .map_err(FrameReplayError::Name)?;
        let selected = QualifiedName::new(vec![operation.operation.clone()])
            .map_err(FrameReplayError::Name)?;
        let bindings = values
            .iter()
            .map(|(field, value)| (field.as_str(), i128::from(*value)))
            .collect::<Vec<_>>();
        let witness = render_witness(&harness.harness_path().to_string(), &check_text, &bindings)
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
        let request = |source: ReplaySource| {
            run.wire(
                located.package_id,
                selected.clone(),
                source,
                obligation,
                &documents,
            )
        };
        // QSL's own decode of the request checks every provided document against its digest
        // and refuses with its own code; the pre-state tie reads those documents only after it
        // has passed, so a document whose bytes do not match its digest is QSL's refusal.
        let unranged = harness.unranged.clone();
        ReplayRequest::decode(
            request(ReplaySource::Witness(witness.clone())),
            replay_limits,
        )
        .map_err(|refusal| FrameReplayError::Refused {
            refusal: Box::new(ReplayRefusal::Request(refusal)),
            unranged: unranged.clone(),
        })?;
        tie_pre_state(&payload.invocation, &state_documents, &values)
            .map_err(FrameReplayError::PreState)?;
        let wire = request(ReplaySource::Witness(witness.clone()));
        let packet = WitnessPacket {
            obligation_identity: Some(obligation),
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
        Ok(Self {
            wire,
            packet,
            replay_limits,
            unranged,
        })
    }

    /// Replays the counterexample through [`qsl_replay::replay_frame`].
    ///
    /// # Errors
    ///
    /// [`FrameReplayError::Envelope`] when the packet is no admitted envelope, and
    /// [`FrameReplayError::Refused`] when QSL refuses the request or the envelope.
    pub fn replay(self) -> Result<FrameReplayResult, FrameReplayError> {
        let Self {
            wire,
            packet,
            replay_limits,
            unranged,
        } = self;
        let envelope = WitnessEnvelope::reconstruct(packet, replay_limits)
            .map_err(FrameReplayError::Envelope)?;
        replay_frame(wire, &envelope, replay_limits).map_err(|refusal| FrameReplayError::Refused {
            refusal: Box::new(refusal),
            unranged,
        })
    }
}

#[cfg(test)]
mod tests {
    /// The frame envelope's `declared_domains` is the empty list and this module builds no
    /// declared-domain value and no key for one; its header says once, and only once, that the
    /// declaration is empty until QSL-345 settles the key. The fixed per-operation transcript the
    /// module once passed to `Witness::parse` is gone.
    ///
    /// Trace: FR-024-AC-25, FR-024-AC-29, TC-035
    #[test]
    fn tc_035_the_declared_domains_are_empty_and_the_header_names_qsl_345_once() {
        let source = include_str!("frame.rs");
        let (header, production) = source
            .split("#[cfg(test)]")
            .next()
            .expect("the source has a production part")
            .split_once("\n\nuse ")
            .expect("the header precedes the imports");
        for banned in ["DeclaredDomain", "DomainKey", "<<<assertion"] {
            assert!(
                !production.contains(banned) && !header.contains(banned),
                "the production source names `{banned}`"
            );
        }
        assert!(production.contains("declared_domains: Some(Vec::new())"));
        assert_eq!(header.matches("QSL-345").count(), 1);
        assert_eq!(production.matches("QSL-345").count(), 0);
    }
}

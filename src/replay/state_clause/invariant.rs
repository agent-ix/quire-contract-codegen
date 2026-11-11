//! Native replay of a Falsified V2 invariant harness (FR-024-AC-57 to AC-64, IR-1505): the
//! harness's concrete playback is put before QSL's `replay_state_clause` as a `Current`
//! observation, one snapshot document and the caller's anchor.
//!
//! CG never infers the anchor: a drawn-state invariant harness carries none, so the caller supplies
//! the [`SelectedAnchor`] and the self object's address. The snapshot binds only the fields the
//! harness draws. A declared field the invariant does not read stays unbound and QSL refuses the
//! snapshot as `missing-member` (the limit IR-1512 owns); this module invents no value for it.

use qsl_replay::{
    call_site, AnchorKind, ClauseName, ClauseSelectionInput, EnvelopeSource, ObligationIdentity,
    QualifiedName, ReplayPackage, ReplaySource, ReplayTargetMembers, RequestTargetWire,
    SelectedAnchor, SelectedObject, StateClauseCounterexample, WitnessPacket, WitnessValue,
};
use quire_canonical::FixedShape;
use quire_contract_model::{CheckedNodeId, CheckedNodeOwner, CheckedPackageV2};
use serde::Serialize;

use super::{
    bind, encode_document, ensure_unique_documents, model_header, populations, state_fields,
    DocumentError, DocumentLabel, IdentityMember, ModelHeader, Side, SnapshotPopulation,
    StateClauseHarnessMember, StateClauseReplay, StateClauseReplayError, StateObjectAddress,
};
use crate::{
    core::ir::{
        clause::{read_clause, ClauseGraphView},
        node_key::checked_node_key,
    },
    kani::{
        abi::{KaniBindingRole, KaniPrimitiveType},
        identity::{
            KaniV2ClauseObligationHarness, ObligationBinding, ObligationKind, StateFrameIdentity,
            V2_INVARIANT_ASSERTION_MESSAGE,
        },
        output::playback::{Playback, WitnessDecodeLimits},
    },
    replay::{
        function::{render_witness, ProvidedDocument, ReplayInputs},
        witness::{decode_playback, DecodeFailure},
    },
};

/// A generated harness handed to the invariant path. Only a scalar V2 invariant harness is
/// accepted; any other generated harness is [`StateClauseReplayError::NotInvariantHarness`].
#[derive(Clone, Copy, Debug)]
pub enum StateClauseHarness<'a> {
    /// A V2 clause harness (an invariant, a postcondition or a precondition).
    V2Clause(&'a KaniV2ClauseObligationHarness),
    /// An operation-contract harness (a postcondition or a frame).
    StateFrame(&'a StateFrameIdentity),
}

/// Everything one invariant replay needs beyond the proved unit.
#[derive(Clone, Debug)]
pub struct InvariantReplayInputs<'a> {
    /// The proving run's lock.
    pub run: ReplayInputs,
    /// The domain package documents the unit's `model` declarations select.
    pub packages: Vec<ProvidedDocument>,
    /// The admitted package the harness was generated from.
    pub package: &'a CheckedPackageV2,
    /// The package's `state`/`state_clause` node of the invariant.
    pub clause_node: &'a CheckedNodeId,
    /// The model object declaration whose fields the harness draws.
    pub object_node: &'a CheckedNodeId,
    /// The fields the harness draws, in draw order: the fields the invariant reads.
    pub state_fields: Vec<String>,
    /// The invariant's declared name.
    pub clause: ClauseName,
    /// The self object's address in the snapshot.
    pub object: StateObjectAddress,
    /// The anchor the snapshot and the selection both name. Supplied by the caller, never
    /// inferred.
    pub anchor: SelectedAnchor,
    /// The current snapshot's identity labels.
    pub snapshot_label: DocumentLabel,
    /// The decoded playback: the `i64` Kani bound to each drawn field.
    pub playback: Vec<(String, i64)>,
    /// The identity of the obligation the counterexample falsified.
    pub obligation_identity: ObligationIdentity,
}

/// A snapshot's `anchor` member.
#[derive(Serialize, FixedShape)]
struct AnchorMember {
    kind: &'static str,
    name: String,
}

/// The current snapshot of an invariant: the role `current` and the anchor.
#[derive(Serialize, FixedShape)]
struct CurrentSnapshotDocument {
    format: &'static str,
    identity: IdentityMember,
    observation: &'static str,
    anchor: AnchorMember,
    model: ModelHeader,
    populations: Vec<SnapshotPopulation>,
}

impl InvariantReplayInputs<'_> {
    /// Decodes a parsed falsified V2 invariant harness playback into this input's drawn-field
    /// values. `self.playback` must be empty. The effective decode bound is the smaller of
    /// `limits` and `run.decode_limits` for each dimension.
    ///
    /// # Errors
    ///
    /// [`StateClauseReplayError`] when the harness is no V2 invariant harness, differs from these
    /// inputs, or the playback is not the decodable playback of its generated assertion.
    pub fn decode_falsification(
        &mut self,
        harness: StateClauseHarness<'_>,
        playback: &Playback<'_>,
        limits: WitnessDecodeLimits,
    ) -> Result<(), StateClauseReplayError> {
        if !self.playback.is_empty() {
            return Err(StateClauseReplayError::PlaybackAlreadySupplied);
        }
        let StateClauseHarness::V2Clause(harness) = harness else {
            return Err(StateClauseReplayError::NotInvariantHarness);
        };
        let identity = &harness.identity;
        if identity.kind != ObligationKind::Invariant
            || identity.collection_precondition.is_some()
            || identity.collection_operation.is_some()
        {
            return Err(StateClauseReplayError::NotInvariantHarness);
        }
        if !harness.has_valid_provenance() {
            return Err(StateClauseReplayError::Decode(DecodeFailure::new(
                "cg_witness_harness_provenance_mismatch",
                identity.harness_symbol.as_str(),
                identity.module_symbol.as_str(),
            )));
        }
        let mismatch = |member| Err(StateClauseReplayError::HarnessMismatch { member });
        if identity.clause != *self.clause_node {
            return mismatch(StateClauseHarnessMember::Clause);
        }
        if identity
            .state_fields
            .iter()
            .any(|field| field.declaring_node_id != *self.object_node)
        {
            return mismatch(StateClauseHarnessMember::Object);
        }
        let drawn = identity
            .state_fields
            .iter()
            .map(|field| field.field_name.as_str());
        if identity.state_fields.is_empty()
            || !drawn.eq(self.state_fields.iter().map(String::as_str))
        {
            return mismatch(StateClauseHarnessMember::StateFields);
        }
        let limits = WitnessDecodeLimits {
            bytes: limits.bytes.min(self.run.decode_limits.bytes),
            values: limits.values.min(self.run.decode_limits.values),
        };
        let bindings = identity
            .state_fields
            .iter()
            .map(|field| ObligationBinding {
                identifier: field.identifier.clone(),
                role: KaniBindingRole::Argument,
                primitive_type: KaniPrimitiveType::I64,
                integer_bounds: None,
                dependencies: Vec::new(),
            })
            .collect::<Vec<_>>();
        let decoded = decode_playback(
            identity.harness_symbol.as_str(),
            identity.module_symbol.as_str(),
            &bindings,
            playback,
            limits,
        )
        .map_err(StateClauseReplayError::Decode)?;
        if !playback.matches_generated_assertion_message(V2_INVARIANT_ASSERTION_MESSAGE) {
            return Err(StateClauseReplayError::AssertionMismatch);
        }
        self.playback = decoded
            .values
            .into_iter()
            .zip(&identity.state_fields)
            .map(|((name, value), field)| {
                // The `i64` binding the decoder types yields only an integer within `i64`, so
                // this refusal is the decoder contract's guard, not a reachable input.
                let integer = match value {
                    WitnessValue::Integer(integer) => i64::try_from(integer).ok(),
                    _ => None,
                }
                .ok_or_else(|| {
                    StateClauseReplayError::Decode(DecodeFailure::new(
                        "kani_witness_unsupported_value",
                        identity.harness_symbol.as_str(),
                        &name,
                    ))
                })?;
                Ok((field.field_name.clone(), integer))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
}

/// The caller's object is the one the invariant is declared on: every field the clause reads
/// is declared by `object_node`, and the node is the model object whose address the snapshot
/// names. A different model object that happens to declare a field of the same name, or an
/// address that is not `object_node`'s, would otherwise lend its field set and range to the
/// replay.
fn ensure_declaring_object(
    graph: &ClauseGraphView<'_>,
    clause_node: &CheckedNodeId,
    object_node: &CheckedNodeId,
    object_type: &str,
) -> Result<(), StateClauseReplayError> {
    let mismatch = || StateClauseReplayError::HarnessMismatch {
        member: StateClauseHarnessMember::Object,
    };
    let condition = read_clause(clause_node, graph).ok_or_else(mismatch)?;
    if condition.inputs.iter().any(|input| {
        input
            .declaring_node_id
            .as_ref()
            .is_some_and(|node| node != object_node)
    }) {
        return Err(mismatch());
    }
    let address = graph
        .package()
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| node.node_id == *object_node)
        .and_then(|node| match &node.owner {
            Some(CheckedNodeOwner::Model { node, .. }) => Some(node.as_ref()),
            _ => None,
        });
    if address != Some(object_type) {
        return Err(mismatch());
    }
    Ok(())
}

/// The documents the request provides: the domain packages, then the one current snapshot. The
/// shared duplicate-identity guard (FR-024-AC-52) runs here, before the request is built.
fn request_documents<'a>(
    packages: &'a [ProvidedDocument],
    snapshot: &'a [u8],
) -> Result<Vec<&'a [u8]>, StateClauseReplayError> {
    let documents = packages
        .iter()
        .map(|document| document.bytes.as_slice())
        .chain(std::iter::once(snapshot))
        .collect::<Vec<_>>();
    ensure_unique_documents(&documents)?;
    Ok(documents)
}

impl StateClauseReplay {
    /// Compiles the proved unit against its domain packages and dependencies, locates the invariant
    /// in it by its declared name, binds the drawn fields from the playback, builds the one current
    /// snapshot, and builds the request and the `Current` envelope members.
    ///
    /// # Errors
    ///
    /// [`StateClauseReplayError`] when the lock's dependencies are not admitted, QSL does not
    /// compile the unit or find the clause, the clause node is not the one QSL names, a drawn
    /// field has no playback value, a playback name is undeclared or repeated, a value is outside
    /// its field's range, or the document or the transcript is not admitted.
    pub fn new_invariant(
        inputs: InvariantReplayInputs<'_>,
    ) -> Result<Self, StateClauseReplayError> {
        let InvariantReplayInputs {
            run,
            packages,
            package,
            clause_node,
            object_node,
            state_fields: requested,
            clause,
            object,
            anchor,
            snapshot_label,
            playback,
            obligation_identity,
        } = inputs;
        let (run, dependencies) = run.admit().map_err(StateClauseReplayError::Dependencies)?;
        let located = call_site(
            run.source.source_identity(),
            package.name().to_owned(),
            &run.source.identity,
            &run.source.bytes,
            packages.iter().map(|package| package.bytes.as_slice()),
            &dependencies,
            &clause,
        )
        .map_err(StateClauseReplayError::CallSite)?;
        let raw_package = ReplayPackage::read(&located.package, u64::MAX)
            .map_err(StateClauseReplayError::RawPackage)?;
        let site = located.site;
        if package
            .structural_key(clause_node)
            .and_then(checked_node_key)
            .as_ref()
            != Some(&site.node)
        {
            return Err(StateClauseReplayError::ClauseSelectionMismatch);
        }

        let graph = ClauseGraphView::new(package);
        ensure_declaring_object(&graph, clause_node, object_node, &object.object_type)?;
        let fields = state_fields(&graph, object_node, &requested)?;
        let values = bind(&fields, &playback, Side::Playback)?;

        let model = model_header(&packages, &object.object_type)
            .map_err(|cause| StateClauseReplayError::Document(DocumentError::Model(cause)))?;
        let document = CurrentSnapshotDocument {
            format: "quire.state.snapshot/v1",
            identity: (&snapshot_label).into(),
            observation: "current",
            anchor: AnchorMember {
                kind: match anchor.kind {
                    AnchorKind::Initialization => "initialization",
                    AnchorKind::Handler => "handler",
                },
                name: anchor.name.clone(),
            },
            model,
            populations: populations(&object, &values),
        };
        let snapshot = encode_document(&document)
            .map_err(|cause| StateClauseReplayError::Document(DocumentError::Encode(cause)))?;

        let selected =
            QualifiedName::new(vec![site.name.clone()]).map_err(StateClauseReplayError::Name)?;
        let bindings = values
            .iter()
            .map(|(field, value)| (field.as_str(), i128::from(*value)))
            .collect::<Vec<_>>();
        // An object-type invariant has no operation: the transcript names the anchor.
        let witness = render_witness(&anchor.name, clause.0.as_str(), &bindings)
            .map_err(StateClauseReplayError::Transcript)?;
        let payload = StateClauseCounterexample {
            observation: ClauseSelectionInput::Current {
                snapshot: snapshot_label.reference(),
                anchor,
                self_object: Some(SelectedObject {
                    population: object.population.clone(),
                    key: object.key.clone(),
                }),
            },
            witness: None,
        };
        let documents = request_documents(&packages, &snapshot.bytes)?;
        let wire = run.wire(
            raw_package,
            RequestTargetWire::state_clause(selected.clone()),
            ReplaySource::Witness(witness.clone()),
            obligation_identity.clone(),
            &documents,
        );
        let packet = WitnessPacket {
            obligation_identity: Some(obligation_identity),
            occurrence_key: Some(site.occurrence),
            clause_node: Some(site.node),
            replay_target: ReplayTargetMembers::state_clause(selected),
            profile_selections: Some(Vec::new()),
            run_limits: Some(run.accounting_limits),
            declared_domains: Some(Vec::new()),
            backend: Some(wire.backend.clone()),
            trace_position: Some(None),
            source: Some(EnvelopeSource::Witness(witness)),
            family_payload: Some(payload),
        };
        Ok(Self {
            wire,
            packet,
            replay_limits: run.replay_limits,
        })
    }
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;

    use super::*;

    /// FR-024-AC-62: documents sharing the snapshot's identity labels return
    /// `DuplicateDocumentIdentity` at the seam `new_invariant` builds its documents through. The domain package QSL admits has no top-level identity
    /// member, so the public path has no second document that can collide; the guard is the one
    /// the postcondition path shares.
    #[trace("FR-024-AC-62", "TC-035")]
    #[test]
    fn tc_035_the_snapshot_identity_is_checked_for_duplicates_before_the_request() {
        let label = DocumentLabel {
            authority: "test".to_owned(),
            identity: "twin-current".to_owned(),
        };
        let document = CurrentSnapshotDocument {
            format: "quire.state.snapshot/v1",
            identity: (&label).into(),
            observation: "current",
            anchor: AnchorMember {
                kind: "handler",
                name: "deposit".to_owned(),
            },
            model: ModelHeader {
                identity: "test/bank".to_owned(),
            },
            populations: Vec::new(),
        };
        let snapshot = encode_document(&document).expect("the snapshot encodes");
        let twin = br#"{"identity":{"authority":"test","identity":"twin-current"},"x":1}"#;
        // A supplied package declaring the snapshot's labels collides with it.
        let colliding = [ProvidedDocument {
            bytes: twin.to_vec(),
        }];
        let error = request_documents(&colliding, &snapshot.bytes)
            .expect_err("the seam refuses the repeated labels");
        assert!(
            matches!(error, StateClauseReplayError::DuplicateDocumentIdentity { identity }
                if identity.authority == "test" && identity.identity == "twin-current")
        );
        let distinct = [ProvidedDocument {
            bytes: br#"{"package":{"identity":"p","version":"1"}}"#.to_vec(),
        }];
        assert_eq!(
            request_documents(&distinct, &snapshot.bytes)
                .expect("distinct labels pass")
                .len(),
            2
        );
    }
}

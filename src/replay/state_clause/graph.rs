//! Native replay of a Falsified relationship-end graph harness (FR-015-AC-106, FR-024): the
//! harness's concrete playback is decoded into the graph it drew and put before QSL's
//! `replay_state_clause` as a `Current` observation, so the authored clause itself, not a
//! separately written reachability, judges the graph.
//!
//! The snapshot holds one population of `object_count` objects keyed `n0`, `n1`, ... and one
//! complete relationship entry holding the drawn edges. The objects declare no field: a model
//! object with a declared field needs a caller-supplied value for it, which this path does not
//! invent. Only a `reaches(self, self, role)` claim is replayed, because the harness draws one
//! `start` that is also the `target`, and `self` is the object the harness started from. CG never
//! infers the anchor or the addresses of the population and the relationship: the caller supplies
//! them, and QSL refuses a snapshot that names an object type, population or relationship the
//! recompiled package does not declare.

use qsl_replay::{
    call_site, AnchorKind, ClauseName, ClauseSelectionInput, EnvelopeSource, ObligationIdentity,
    QualifiedName, ReplayPackage, ReplaySource, ReplayTargetMembers, RequestTargetWire,
    SelectedAnchor, SelectedObject, StateClauseCounterexample, WitnessPacket, WitnessValue,
};
use quire_canonical::FixedShape;
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrence, CheckedOccurrenceRole, CheckedPackageV2,
};
use quire_exact::Origin;
use serde::Serialize;
use std::collections::BTreeMap;

use super::{
    encode_document, ensure_unique_documents, model_header, DocumentError, DocumentLabel,
    IdentityMember, ModelHeader, ObjectRef, SnapshotObject, SnapshotPopulation,
    StateClauseHarnessMember, StateClauseReplay, StateClauseReplayError,
};
use crate::{
    core::ir::node_key::checked_node_key,
    kani::{
        abi::{KaniBindingRole, KaniPrimitiveType},
        identity::{KaniGraphObligationHarness, ObligationBinding, GRAPH_ASSERTION_MESSAGE},
        output::playback::{Playback, WitnessDecodeLimits},
    },
    replay::{
        function::{render_witness, ProvidedDocument, ReplayInputs},
        witness::{decode_playback, DecodeFailure},
    },
};

/// The concrete graph a falsified harness drew.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphWitness {
    /// How many objects the population holds.
    pub objects: usize,
    /// The object the harness started from, which is also its `self`.
    pub start: usize,
    /// The target objects of each object's selected edges, in draw order; one row per object.
    pub edges: Vec<Vec<usize>>,
    /// Whether an edge runs from the row object to its target (the harness's forward direction)
    /// or from the target to the row object.
    pub forward: bool,
}

/// Where the graph lives in the domain package, as the snapshot spells it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphReplayAddress {
    /// The population's identity.
    pub population: String,
    /// The objects' type identity.
    pub object_type: String,
    /// The relationship declaration's identity.
    pub relationship: String,
}

/// Everything one graph replay needs beyond the proved unit.
#[derive(Clone, Debug)]
pub struct GraphReplayInputs<'a> {
    /// The proving run's lock.
    pub run: ReplayInputs,
    /// The domain package documents the unit's `model` declarations select.
    pub packages: Vec<ProvidedDocument>,
    /// The admitted package the harness was generated from.
    pub package: &'a CheckedPackageV2,
    /// The package's `state`/`state_clause` node of the invariant.
    pub clause_node: &'a CheckedNodeId,
    /// The invariant's declared name.
    pub clause: ClauseName,
    /// The population, object type and relationship the snapshot names.
    pub address: GraphReplayAddress,
    /// The anchor the snapshot and the selection both name. Supplied by the caller, never
    /// inferred.
    pub anchor: SelectedAnchor,
    /// The current snapshot's identity labels.
    pub snapshot_label: DocumentLabel,
    /// The graph decoded from the playback; `None` until [`Self::decode_falsification`]. Private:
    /// only a decoded playback of the harness sets it, so a caller cannot replay a graph Kani
    /// never drew.
    witness: Option<GraphWitness>,
    /// The occurrence of the clause node the decoded harness was keyed by; set with the witness.
    /// A structure declared under two clause names is one node, so the selected clause must also
    /// be this occurrence.
    occurrence: Option<CheckedOccurrence>,
}

#[derive(Serialize, FixedShape)]
struct AnchorMember {
    kind: &'static str,
    name: String,
}

#[derive(Serialize, FixedShape)]
struct Instance {
    source: ObjectRef,
    target: ObjectRef,
}

#[derive(Serialize, FixedShape)]
struct RelationshipEntry {
    relationship: String,
    complete: bool,
    instances: Vec<Instance>,
}

#[derive(Serialize, FixedShape)]
struct GraphSnapshotDocument {
    format: &'static str,
    identity: IdentityMember,
    observation: &'static str,
    anchor: AnchorMember,
    model: ModelHeader,
    populations: Vec<SnapshotPopulation>,
    relationships: Vec<RelationshipEntry>,
}

impl<'a> GraphReplayInputs<'a> {
    /// The inputs of one graph replay, with no witness yet.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        run: ReplayInputs,
        packages: Vec<ProvidedDocument>,
        package: &'a CheckedPackageV2,
        clause_node: &'a CheckedNodeId,
        clause: ClauseName,
        address: GraphReplayAddress,
        anchor: SelectedAnchor,
        snapshot_label: DocumentLabel,
    ) -> Self {
        Self {
            run,
            packages,
            package,
            clause_node,
            clause,
            address,
            anchor,
            snapshot_label,
            witness: None,
            occurrence: None,
        }
    }

    /// The graph decoded from the playback, once [`Self::decode_falsification`] succeeded.
    #[must_use]
    pub fn witness(&self) -> Option<&GraphWitness> {
        self.witness.as_ref()
    }

    /// Test seam: a witness no playback produced, to reach the snapshot paths a Kani run does not
    /// (the reverse direction and a non-unique end).
    #[cfg(test)]
    pub(crate) fn with_witness(&mut self, witness: GraphWitness) {
        self.witness = Some(witness);
    }

    /// Decodes a parsed falsified graph harness playback into the graph it drew. The slots are
    /// read in the generated draw order (`object_count`, `start`, `target`, then each row's degree
    /// and `upper` edge slots) and checked against the harness's own domain. That domain (population
    /// maximum, edge `lower`, `upper` and `unique`) is read from the harness identity, which the
    /// generator filled from the authored package's `select_population` and `relationship_end`
    /// projections; this check does not re-derive it from the package, so it is a consistency
    /// check of the playback against the harness, not an independent domain oracle.
    ///
    /// # Errors
    ///
    /// [`StateClauseReplayError`] when a witness was already decoded, the harness differs from
    /// these inputs, the playback is not the decodable playback of the harness's assertion, or a
    /// slot lies outside the domain the harness assumed.
    pub fn decode_falsification(
        &mut self,
        harness: &KaniGraphObligationHarness,
        playback: &Playback<'_>,
        limits: WitnessDecodeLimits,
    ) -> Result<(), StateClauseReplayError> {
        if self.witness.is_some() {
            return Err(StateClauseReplayError::PlaybackAlreadySupplied);
        }
        if !harness.has_valid_provenance() {
            return Err(StateClauseReplayError::Decode(DecodeFailure::new(
                "cg_witness_harness_provenance_mismatch",
                harness.identity.harness_symbol.as_str(),
                harness.identity.module_symbol.as_str(),
            )));
        }
        let identity = &harness.identity;
        if identity.clause_id != *self.clause_node {
            return Err(StateClauseReplayError::HarnessMismatch {
                member: StateClauseHarnessMember::Clause,
            });
        }
        let objects = usize::try_from(identity.maximum_population).map_err(|_| not_in_domain())?;
        let upper = usize::try_from(identity.edge.upper).map_err(|_| not_in_domain())?;
        let mut names = vec![
            "object_count".to_owned(),
            "start".to_owned(),
            "target".to_owned(),
        ];
        for row in 0..objects {
            names.push(format!("degree_{row}"));
            names.extend((0..upper).map(|column| format!("edge_{row}_{column}")));
        }
        let bindings: Vec<ObligationBinding> = names
            .iter()
            .map(|identifier| ObligationBinding {
                identifier: identifier.clone(),
                role: KaniBindingRole::Argument,
                primitive_type: KaniPrimitiveType::I64,
                integer_bounds: None,
                dependencies: Vec::new(),
            })
            .collect();
        let limits = WitnessDecodeLimits {
            bytes: limits.bytes.min(self.run.decode_limits.bytes),
            values: limits.values.min(self.run.decode_limits.values),
        };
        let decoded = decode_playback(
            identity.harness_symbol.as_str(),
            identity.module_symbol.as_str(),
            &bindings,
            playback,
            limits,
        )
        .map_err(StateClauseReplayError::Decode)?;
        if !playback.matches_generated_assertion_message(GRAPH_ASSERTION_MESSAGE) {
            return Err(StateClauseReplayError::AssertionMismatch);
        }
        let slots = decoded
            .values
            .into_iter()
            .map(|(_, value)| match value {
                WitnessValue::Integer(integer) => i64::try_from(integer).ok(),
                _ => None,
            })
            .collect::<Option<Vec<i64>>>()
            .ok_or_else(|| {
                StateClauseReplayError::Decode(DecodeFailure::new(
                    "kani_witness_unsupported_value",
                    identity.harness_symbol.as_str(),
                    identity.module_symbol.as_str(),
                ))
            })?;
        let slot = |index: usize| usize::try_from(slots[index]).map_err(|_| not_in_domain());
        let count = slot(0)?;
        let (start, target) = (slot(1)?, slot(2)?);
        // The harness asserts `object_count == 0 || claim`, so a falsifying draw is non-empty.
        if count == 0 || count > objects || start >= count {
            return Err(not_in_domain());
        }
        // Only a self-operand claim is replayed: `self` is the one object the harness started
        // from.
        if start != target {
            return Err(StateClauseReplayError::GraphStartIsNotTarget);
        }
        let mut edges = Vec::with_capacity(count);
        for row in 0..count {
            let base = 3 + row * (1 + upper);
            let degree = slot(base)?;
            let lower = usize::try_from(identity.edge.lower).map_err(|_| not_in_domain())?;
            if degree < lower || degree > upper {
                return Err(not_in_domain());
            }
            let mut targets = Vec::with_capacity(degree);
            for column in 0..degree {
                let to = slot(base + 1 + column)?;
                if to >= count || (identity.edge.unique && targets.contains(&to)) {
                    return Err(not_in_domain());
                }
                targets.push(to);
            }
            edges.push(targets);
        }
        self.witness = Some(GraphWitness {
            objects: count,
            start,
            edges,
            forward: identity.edge.forward,
        });
        self.occurrence = Some(identity.occurrence.clone());
        Ok(())
    }
}

fn not_in_domain() -> StateClauseReplayError {
    StateClauseReplayError::GraphWitnessNotInDomain
}

/// Whether QSL's call-site origin is the clause node occurrence a harness is keyed by: the same
/// role and the same ordinal within that role.
fn names_occurrence(origin: &Origin, occurrence: &CheckedOccurrence) -> bool {
    let role = match occurrence.role {
        CheckedOccurrenceRole::Declaration => "declaration",
        CheckedOccurrenceRole::Type => "type",
        CheckedOccurrenceRole::Expression => "expression",
        CheckedOccurrenceRole::Anchor => "anchor",
        CheckedOccurrenceRole::Claim => "claim",
        CheckedOccurrenceRole::Generated => "generated",
    };
    origin.role().as_str() == role && origin.ordinal() == occurrence.ordinal
}

fn key(index: usize) -> String {
    format!("n{index}")
}

impl StateClauseReplay {
    /// Compiles the proved unit against its domain packages, locates the invariant by its declared
    /// name, builds the one current snapshot of the decoded graph and builds the request and the
    /// `Current` envelope members. `self` is the object the harness started from.
    ///
    /// # Errors
    ///
    /// [`StateClauseReplayError`] when no witness was decoded, the lock's dependencies are not
    /// admitted, QSL does not compile the unit or find the clause, the clause node is not the one
    /// QSL names, or a document or the transcript is not admitted.
    pub fn new_graph_invariant(
        inputs: GraphReplayInputs<'_>,
    ) -> Result<Self, StateClauseReplayError> {
        let GraphReplayInputs {
            run,
            packages,
            package,
            clause_node,
            clause,
            address,
            anchor,
            snapshot_label,
            witness,
            occurrence,
        } = inputs;
        let witness = witness.ok_or(StateClauseReplayError::MissingField {
            field: "playback".to_owned(),
        })?;
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
        // The harness is keyed by one occurrence of the clause node; the named clause must be it.
        if occurrence
            .as_ref()
            .is_some_and(|occurrence| !names_occurrence(site.occurrence.origin(), occurrence))
        {
            return Err(StateClauseReplayError::ClauseSelectionMismatch);
        }
        let obligation_identity = ObligationIdentity::new(
            site.node.clone(),
            None,
            u32::try_from(site.occurrence.origin().ordinal())
                .map_err(|_| StateClauseReplayError::ClauseSelectionMismatch)?,
        );

        let model = model_header(&packages, &address.object_type)
            .map_err(|cause| StateClauseReplayError::Document(DocumentError::Model(cause)))?;
        let end = |index: usize| ObjectRef {
            population: address.population.clone(),
            key: key(index),
        };
        let instances = witness
            .edges
            .iter()
            .enumerate()
            .flat_map(|(row, targets)| targets.iter().map(move |to| (row, *to)))
            .map(|(row, to)| {
                if witness.forward {
                    Instance {
                        source: end(row),
                        target: end(to),
                    }
                } else {
                    Instance {
                        source: end(to),
                        target: end(row),
                    }
                }
            })
            .collect();
        let document = GraphSnapshotDocument {
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
            populations: vec![SnapshotPopulation {
                population: address.population.clone(),
                complete: true,
                objects: (0..witness.objects)
                    .map(|index| SnapshotObject {
                        key: key(index),
                        object_type: address.object_type.clone(),
                        fields: BTreeMap::new(),
                    })
                    .collect(),
            }],
            relationships: vec![RelationshipEntry {
                relationship: address.relationship.clone(),
                complete: true,
                instances,
            }],
        };
        let snapshot = encode_document(&document)
            .map_err(|cause| StateClauseReplayError::Document(DocumentError::Encode(cause)))?;

        let selected =
            QualifiedName::new(vec![site.name.clone()]).map_err(StateClauseReplayError::Name)?;
        let count = i128::try_from(witness.objects)
            .map_err(|_| StateClauseReplayError::GraphWitnessNotInDomain)?;
        let start = i128::try_from(witness.start)
            .map_err(|_| StateClauseReplayError::GraphWitnessNotInDomain)?;
        let transcript = render_witness(
            &anchor.name,
            clause.0.as_str(),
            &[("object_count", count), ("start", start)],
        )
        .map_err(StateClauseReplayError::Transcript)?;
        let payload = StateClauseCounterexample {
            observation: ClauseSelectionInput::Current {
                snapshot: snapshot_label.reference(),
                anchor,
                self_object: Some(SelectedObject {
                    population: address.population.clone(),
                    key: key(witness.start),
                }),
            },
            witness: None,
        };
        let documents = packages
            .iter()
            .map(|document| document.bytes.as_slice())
            .chain(std::iter::once(snapshot.bytes.as_slice()))
            .collect::<Vec<_>>();
        ensure_unique_documents(&documents)?;
        let wire = run.wire(
            raw_package,
            RequestTargetWire::state_clause(selected.clone()),
            ReplaySource::Witness(transcript.clone()),
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
            source: Some(EnvelopeSource::Witness(transcript)),
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
    use super::*;
    use ix_trace_rs::trace;
    use quire_exact::Role;

    fn claim(ordinal: u64) -> CheckedOccurrence {
        CheckedOccurrence {
            role: CheckedOccurrenceRole::Claim,
            ordinal,
        }
    }

    #[trace("TC-025", "FR-015-AC-107")]
    #[test]
    fn an_origin_names_an_occurrence_only_by_role_and_ordinal() {
        let origin = Origin::new(Role::new("claim"), 1);
        assert!(names_occurrence(&origin, &claim(1)));
        assert!(!names_occurrence(&origin, &claim(0)));
        let declaration = CheckedOccurrence {
            role: CheckedOccurrenceRole::Declaration,
            ordinal: 1,
        };
        assert!(!names_occurrence(&origin, &declaration));
    }
}

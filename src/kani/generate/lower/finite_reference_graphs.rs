//! CG-owned finite graph reachability over IR's validated object and reference ABI.

use std::collections::{BTreeMap, BTreeSet};

use quire_contract_ir::kani::{
    DispatchIndex, KaniOutcomeKind, KaniProfile, SemanticFamily, ValidatedFiniteInput,
};

use quire_contract_model::std001_code;

use super::{admit_family, refusal, FamilyLoweringError};

/// A positive-length reachability request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphRequest {
    /// Source clause identity.
    pub source_id: String,
    /// Start object identity.
    pub start_id: String,
    /// Target object identity.
    pub target_id: String,
    /// Reference field followed.
    pub field_id: String,
    /// Maximum number of expanded objects.
    pub max_expansions: usize,
}

/// Exact reachability result; this is not a Kani verdict.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphLowering {
    /// Original request.
    pub request: GraphRequest,
    /// Whether a matching positive-length path exists.
    pub reachable: bool,
    /// Expanded identities in deterministic discovery order.
    pub expanded: Vec<String>,
}

/// One entered object and the neighbors whose subgraphs have not been visited yet.
struct TraversalFrame<'a> {
    remaining: std::vec::IntoIter<&'a str>,
}

/// Prepares a bounded positive-length reachability result.
pub fn prepare_finite_graph_reaches(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    input: &ValidatedFiniteInput,
    request: GraphRequest,
) -> Result<GraphLowering, FamilyLoweringError> {
    admit_family(
        profile,
        dispatch,
        "finite-reference-graph",
        SemanticFamily::ObjectsReferencesGraphs,
        &request.source_id,
    )?;
    if request.max_expansions == 0 {
        return Err(refusal(
            KaniOutcomeKind::InvalidInput,
            std001_code!("kani_graph_bound_invalid"),
            &request.source_id,
            profile,
        ));
    }
    let identities: BTreeSet<_> = input
        .input()
        .objects
        .iter()
        .map(|object| object.identity.as_str())
        .collect();
    if !identities.contains(request.start_id.as_str())
        || !identities.contains(request.target_id.as_str())
    {
        return Err(refusal(
            KaniOutcomeKind::InvalidInput,
            std001_code!("kani_graph_identity_invalid"),
            &request.source_id,
            profile,
        ));
    }
    // Index the selected reference field once. The ordered sets give each source a stable
    // neighbor order even if the validated input lists references in another order.
    let adjacency = input
        .input()
        .references
        .iter()
        .filter(|edge| edge.field_id == request.field_id)
        .fold(
            BTreeMap::<&str, BTreeSet<&str>>::new(),
            |mut index, edge| {
                index
                    .entry(edge.source_id.as_str())
                    .or_default()
                    .insert(edge.target_id.as_str());
                index
            },
        );
    let mut seen = BTreeSet::new();
    let mut expanded = Vec::new();
    let mut next_object = Some(request.start_id.as_str());
    let mut frames = Vec::<TraversalFrame<'_>>::new();
    loop {
        let Some(current) = next_object.take() else {
            let Some(frame) = frames.last_mut() else {
                break;
            };
            if let Some(neighbor) = frame.remaining.next() {
                if !seen.contains(neighbor) {
                    next_object = Some(neighbor);
                }
            } else {
                frames.pop();
            }
            continue;
        };
        seen.insert(current);
        if expanded.len() == request.max_expansions {
            return Err(refusal(
                KaniOutcomeKind::ResourceExhausted,
                std001_code!("kani_graph_expansion_exhausted"),
                &request.source_id,
                profile,
            ));
        }
        expanded.push(current.to_owned());
        let neighbors = adjacency.get(current);
        if neighbors.is_some_and(|neighbors| neighbors.contains(request.target_id.as_str())) {
            return Ok(GraphLowering {
                request,
                reachable: true,
                expanded,
            });
        }
        frames.push(TraversalFrame {
            remaining: neighbors
                .into_iter()
                .flat_map(|neighbors| neighbors.iter().copied())
                .collect::<Vec<_>>()
                .into_iter(),
        });
    }
    Ok(GraphLowering {
        request,
        reachable: false,
        expanded,
    })
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, FiniteObject,
        FiniteReference, KaniOutcomeKind, KaniProfile, ModuleDescriptor, PopulationCompleteness,
        ProfileSelection, ResourceBounds, SemanticFamily,
    };

    use super::{prepare_finite_graph_reaches, GraphRequest};

    /// Trace: TC-023.
    #[test]
    fn branching_graph_uses_a_stable_bounded_discovery_order() {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            abi_revision: "abi".to_owned(),
        };
        let profile = KaniProfile::new(
            selection.clone(),
            vec![CapabilityEntry {
                construct: "finite-reference-graph".to_owned(),
                disposition: CapabilityDisposition::Supported {
                    module: "graphs".to_owned(),
                },
            }],
        )
        .expect("profile fixture");
        let dispatch = DispatchIndex::new(vec![ModuleDescriptor {
            module_id: "graphs".to_owned(),
            family: SemanticFamily::ObjectsReferencesGraphs,
            abi_revision: "abi".to_owned(),
            constructs: vec!["finite-reference-graph".to_owned()],
        }])
        .expect("dispatch fixture");
        let input = FiniteInput {
            model_id: "model".to_owned(),
            source_id: "source".to_owned(),
            profile: selection,
            completeness: PopulationCompleteness::Complete,
            bounds: ResourceBounds {
                max_objects: 5,
                max_references: 4,
                max_input_bytes: 1,
            },
            input_bytes: 0,
            objects: ["a", "b", "c", "d", "target"]
                .into_iter()
                .map(|identity| FiniteObject {
                    identity: identity.to_owned(),
                    type_id: "node".to_owned(),
                    snapshot_id: "snapshot".to_owned(),
                })
                .collect(),
            references: [("a", "c"), ("b", "d"), ("c", "target"), ("a", "b")]
                .into_iter()
                .map(|(source_id, target_id)| FiniteReference {
                    source_id: source_id.to_owned(),
                    field_id: "next".to_owned(),
                    target_id: target_id.to_owned(),
                })
                .collect(),
        }
        .validate()
        .expect("finite graph fixture");
        let request = GraphRequest {
            source_id: "source".to_owned(),
            start_id: "a".to_owned(),
            target_id: "target".to_owned(),
            field_id: "next".to_owned(),
            max_expansions: 4,
        };
        let reached = prepare_finite_graph_reaches(&profile, &dispatch, &input, request.clone())
            .expect("target is reached within four depth-first expansions");
        assert!(reached.reachable);
        assert_eq!(reached.expanded, ["a", "b", "d", "c"]);

        let exhausted = prepare_finite_graph_reaches(
            &profile,
            &dispatch,
            &input,
            GraphRequest {
                max_expansions: 3,
                ..request
            },
        )
        .expect_err("the other branch consumes the third expansion");
        let outcome = exhausted.outcome().expect("IR built the typed outcome");
        assert_eq!(outcome.kind, KaniOutcomeKind::ResourceExhausted);
        assert_eq!(outcome.code.as_str(), "kani_graph_expansion_exhausted");

        // The second graph has both a back edge and a cross edge into an already pending
        // identity. Neither revisits an expanded identity or consumes another expansion.
        let mut cyclic = input.input().clone();
        cyclic.bounds.max_references = 6;
        cyclic.references = [
            ("a", "c"),
            ("b", "c"),
            ("d", "b"),
            ("b", "a"),
            ("c", "d"),
            ("a", "b"),
        ]
        .into_iter()
        .map(|(source_id, target_id)| FiniteReference {
            source_id: source_id.to_owned(),
            field_id: "next".to_owned(),
            target_id: target_id.to_owned(),
        })
        .collect();
        let cyclic = cyclic.validate().expect("valid cycle and cross edge");
        let absent = prepare_finite_graph_reaches(
            &profile,
            &dispatch,
            &cyclic,
            GraphRequest {
                max_expansions: 4,
                ..reached.request
            },
        )
        .expect("all reachable identities fit the budget");
        assert!(!absent.reachable);
        assert_eq!(absent.expanded, ["a", "b", "c", "d"]);
    }
}

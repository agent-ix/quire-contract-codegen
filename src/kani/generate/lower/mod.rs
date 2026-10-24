// Bounded collection query admission.
pub(crate) mod bounded_collections;
// The bounded-Kani profile and its classification.
pub(crate) mod bounded_kani_profile;
// Finite reference graph reachability admission.
pub(crate) mod finite_reference_graphs;

use quire_contract_ir::kani::{
    CapabilityDisposition, DispatchIndex, KaniOutcome, KaniProfile, SemanticFamily,
};
use quire_contract_model::{std001_code, Std001Code};

/// Admit one construct through the shared profile and dispatch authorities.
fn admit_family(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    construct: &str,
    family: SemanticFamily,
    source_id: &str,
) -> Result<(), KaniOutcome> {
    let entries = profile.classify(&[construct.to_owned()], source_id)?;
    // A successful single-construct classification contains exactly that entry.
    let Some(entry) = entries.first() else {
        return Err(KaniOutcome::refused(
            Std001Code::KANI_CAPABILITY_MISSING,
            source_id,
            profile.selection().revision.clone(),
        ));
    };
    match &entry.disposition {
        CapabilityDisposition::Supported { module } => {
            let descriptor = dispatch.resolve(construct).map_err(|_| {
                KaniOutcome::refused(
                    std001_code!("kani_dispatch_unowned"),
                    source_id,
                    profile.selection().revision.clone(),
                )
            })?;
            if descriptor.family != family || descriptor.module_id != *module {
                return Err(KaniOutcome::refused(
                    std001_code!("kani_dispatch_profile_mismatch"),
                    source_id,
                    profile.selection().revision.clone(),
                ));
            }
            Ok(())
        }
        CapabilityDisposition::Refused { code } => Err(KaniOutcome::refused(
            *code,
            source_id,
            profile.selection().revision.clone(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, FiniteObject,
        FiniteReference, KaniOutcomeKind, KaniProfile, ModuleDescriptor, PopulationCompleteness,
        ProfileSelection, ResourceBounds, SemanticFamily, ValidatedFiniteInput,
    };
    use quire_contract_model::{std001_code, Std001Code};

    use super::bounded_collections::{
        prepare_bounded_collection_query, CollectionQuery, QueryKind,
    };
    use super::finite_reference_graphs::{prepare_finite_graph_reaches, GraphRequest};
    use quire_contract_ir::kani::KaniOutcome;

    fn fixture(
        disposition: impl Fn(&str) -> CapabilityDisposition,
    ) -> (KaniProfile, DispatchIndex, ValidatedFiniteInput) {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".into(),
            revision: "profile-r1".into(),
            abi_revision: "abi".into(),
        };
        let modules = [
            (
                "bounded-collection-query",
                "collections",
                SemanticFamily::CollectionsQueries,
            ),
            (
                "finite-reference-graph",
                "graphs",
                SemanticFamily::ObjectsReferencesGraphs,
            ),
        ];
        let profile = KaniProfile::new(
            selection.clone(),
            modules
                .iter()
                .map(|(construct, module, _)| CapabilityEntry {
                    construct: (*construct).into(),
                    disposition: disposition(module),
                })
                .collect(),
        )
        .expect("valid profile");
        let dispatch = DispatchIndex::new(
            modules
                .into_iter()
                .map(|(construct, module, family)| ModuleDescriptor {
                    module_id: module.into(),
                    family,
                    abi_revision: "abi".into(),
                    constructs: vec![construct.into()],
                })
                .collect(),
        )
        .expect("valid dispatch");
        let input = FiniteInput {
            model_id: "model".into(),
            source_id: "source:family".into(),
            profile: selection,
            completeness: PopulationCompleteness::Complete,
            bounds: ResourceBounds {
                max_objects: 2,
                max_references: 1,
                max_input_bytes: 1,
            },
            input_bytes: 0,
            objects: ["a", "b"]
                .into_iter()
                .map(|identity| FiniteObject {
                    identity: identity.into(),
                    type_id: "node".into(),
                    snapshot_id: "snapshot".into(),
                })
                .collect(),
            references: vec![FiniteReference {
                source_id: "a".into(),
                field_id: "next".into(),
                target_id: "b".into(),
            }],
        }
        .validate()
        .expect("valid finite population");
        (profile, dispatch, input)
    }

    fn prepare_both(
        profile: &KaniProfile,
        dispatch: &DispatchIndex,
        input: &ValidatedFiniteInput,
    ) -> [Result<(), KaniOutcome>; 2] {
        [
            prepare_bounded_collection_query(
                profile,
                dispatch,
                input,
                CollectionQuery {
                    source_id: "source:family".into(),
                    values: vec![2, 2, 3],
                    max_items: 3,
                    kind: QueryKind::ExistsEqual(3),
                },
            )
            .map(|lowered| {
                assert!(lowered.value);
                assert_eq!(lowered.examined, 3);
                assert_eq!(lowered.query.values, vec![2, 2, 3]);
            }),
            prepare_finite_graph_reaches(
                profile,
                dispatch,
                input,
                GraphRequest {
                    source_id: "source:family".into(),
                    start_id: "a".into(),
                    target_id: "b".into(),
                    field_id: "next".into(),
                    max_expansions: 1,
                },
            )
            .map(|lowered| {
                assert!(lowered.reachable);
                assert_eq!(lowered.expanded, vec!["a"]);
            }),
        ]
    }

    fn assert_refusal(outcome: KaniOutcome, code: Std001Code) {
        assert_eq!(outcome.kind(), KaniOutcomeKind::Refused);
        assert_eq!(outcome.code(), code);
        assert_eq!(outcome.source_id(), "source:family");
        assert_eq!(outcome.context(), "profile-r1");
        assert_eq!(outcome.boolean_claim(), None);
    }

    #[trace("TC-023", "FR-015-AC-86", "FR-015-AC-88")]
    #[test]
    fn supported_profile_still_prepares_exact_collection_and_graph_results() {
        let (profile, dispatch, input) = fixture(|module| CapabilityDisposition::Supported {
            module: module.into(),
        });
        for result in prepare_both(&profile, &dispatch, &input) {
            result.expect("supported family prepares its exact result");
        }
    }

    #[trace("TC-023", "FR-015-AC-82")]
    #[test]
    fn profile_refusals_preserve_original_causes_and_context_in_both_families() {
        let (profile, _, input) = fixture(|module| CapabilityDisposition::Refused {
            code: match module {
                "collections" => std001_code!("collection_not_qualified"),
                "graphs" => std001_code!("graph_not_qualified"),
                _ => unreachable!("fixture modules are closed"),
            },
        });
        let empty_dispatch = DispatchIndex::new(vec![]).expect("empty dispatch is valid");
        let codes = [
            std001_code!("collection_not_qualified"),
            std001_code!("graph_not_qualified"),
        ];
        for (result, code) in prepare_both(&profile, &empty_dispatch, &input)
            .into_iter()
            .zip(codes)
        {
            assert_refusal(
                result.expect_err("profile refusal precedes dispatch and lowering"),
                code,
            );
        }
    }

    #[trace("TC-023", "FR-015-AC-82")]
    #[test]
    fn supported_profile_keeps_unowned_and_mismatched_dispatch_refusals() {
        let (profile, _, input) = fixture(|module| CapabilityDisposition::Supported {
            module: module.into(),
        });
        let empty_dispatch = DispatchIndex::new(vec![]).expect("empty dispatch is valid");
        for result in prepare_both(&profile, &empty_dispatch, &input) {
            assert_refusal(
                result.expect_err("unowned family refuses"),
                std001_code!("kani_dispatch_unowned"),
            );
        }
        let (mismatched, dispatch, input) = fixture(|_| CapabilityDisposition::Supported {
            module: "foreign-module".into(),
        });
        for result in prepare_both(&mismatched, &dispatch, &input) {
            assert_refusal(
                result.expect_err("profile/dispatch mismatch refuses"),
                std001_code!("kani_dispatch_profile_mismatch"),
            );
        }
    }
}

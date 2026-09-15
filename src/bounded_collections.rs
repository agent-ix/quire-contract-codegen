//! Ordered duplicate-preserving bounded collection admission for generated backends.

use quire_contract_ir::kani::{
    lower_query, CollectionLowering, CollectionQuery, DispatchIndex, KaniOutcome, KaniProfile,
    ValidatedFiniteInput,
};

/// Produces the native reviewed collection-query plan before generated artifacts are rendered.
///
/// Bound exhaustion and profile/dispatch refusal remain typed non-Boolean outcomes with no partial
/// generated artifact.
pub fn prepare_bounded_collection_query(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    input: &ValidatedFiniteInput,
    query: CollectionQuery,
) -> Result<CollectionLowering, KaniOutcome> {
    lower_query(profile, dispatch, input, query)
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, CollectionQuery, DispatchIndex, FiniteInput,
        KaniOutcomeKind, KaniProfile, ModuleDescriptor, PopulationCompleteness, ProfileSelection,
        QueryKind, ResourceBounds, SemanticFamily,
    };

    use super::prepare_bounded_collection_query;

    fn fixture() -> (
        KaniProfile,
        DispatchIndex,
        quire_contract_ir::kani::ValidatedFiniteInput,
    ) {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
            executable_digest: "exe".to_owned(),
            options_digest: "opts".to_owned(),
            abi_revision: "abi".to_owned(),
        };
        let profile = KaniProfile::new(
            selection.clone(),
            vec![CapabilityEntry {
                construct: "bounded-collection-query".to_owned(),
                disposition: CapabilityDisposition::Supported {
                    module: "collections".to_owned(),
                },
            }],
        )
        .expect("valid profile");
        let dispatch = DispatchIndex::new(vec![ModuleDescriptor {
            module_id: "collections".to_owned(),
            family: SemanticFamily::CollectionsQueries,
            abi_revision: "abi".to_owned(),
            constructs: vec!["bounded-collection-query".to_owned()],
        }])
        .expect("valid dispatch");
        let input = FiniteInput {
            model_id: "model".to_owned(),
            source_id: "source".to_owned(),
            profile: selection,
            completeness: PopulationCompleteness::Complete,
            bounds: ResourceBounds {
                max_objects: 1,
                max_references: 0,
                max_input_bytes: 1,
            },
            input_bytes: 0,
            objects: vec![],
            references: vec![],
        }
        .validate()
        .expect("valid input");
        (profile, dispatch, input)
    }

    /// Trace: FR-007-AC-1, FR-007-AC-3, TC-023.
    #[test]
    fn tc_023_collection_order_duplicates_and_bounds_remain_exact() {
        let (profile, dispatch, input) = fixture();
        let lowered = prepare_bounded_collection_query(
            &profile,
            &dispatch,
            &input,
            CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![2, 2, 7],
                max_items: 3,
                kind: QueryKind::ExistsEqual(7),
            },
        )
        .expect("bounded duplicate sequence is admitted");
        assert!(lowered.value);
        assert_eq!(lowered.examined, 3);
        let exhausted = prepare_bounded_collection_query(
            &profile,
            &dispatch,
            &input,
            CollectionQuery {
                source_id: "source".to_owned(),
                values: vec![1, 2],
                max_items: 1,
                kind: QueryKind::ForAllNonNegative,
            },
        )
        .expect_err("one-past bound must not lower");
        assert_eq!(exhausted.kind, KaniOutcomeKind::ResourceExhausted);
        assert_eq!(exhausted.boolean_claim(), None);
    }
}

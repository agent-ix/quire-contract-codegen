//! CG-owned ordered collection queries over IR's validated finite input and typed outcome.

use quire_contract_ir::kani::{
    DispatchIndex, KaniOutcomeKind, KaniProfile, SemanticFamily, ValidatedFiniteInput,
};

use quire_contract_model::std001_code;

use super::{admit_family, refusal, FamilyLoweringError};

/// Query over one ordered finite sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryKind {
    /// Every value is nonnegative.
    ForAllNonNegative,
    /// At least one value equals the operand.
    ExistsEqual(i128),
}

/// A bounded collection request retaining order and duplicates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionQuery {
    /// Source clause identity.
    pub source_id: String,
    /// Ordered values, including duplicates.
    pub values: Vec<i128>,
    /// Maximum count admitted.
    pub max_items: usize,
    /// Query to evaluate.
    pub kind: QueryKind,
}

/// Exact query result; this is not a Kani verdict.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionLowering {
    /// Original query.
    pub query: CollectionQuery,
    /// Query truth value.
    pub value: bool,
    /// Number of values examined, including the decisive one.
    pub examined: usize,
}

/// Prepares a bounded query while retaining order and duplicates.
pub fn prepare_bounded_collection_query(
    profile: &KaniProfile,
    dispatch: &DispatchIndex,
    _input: &ValidatedFiniteInput,
    query: CollectionQuery,
) -> Result<CollectionLowering, FamilyLoweringError> {
    admit_family(
        profile,
        dispatch,
        "bounded-collection-query",
        SemanticFamily::CollectionsQueries,
        &query.source_id,
    )?;
    if query.values.len() > query.max_items {
        return Err(refusal(
            KaniOutcomeKind::ResourceExhausted,
            std001_code!("kani_collection_bound_exhausted"),
            &query.source_id,
            profile,
        ));
    }
    let position = match query.kind {
        QueryKind::ForAllNonNegative => query.values.iter().position(|value| *value < 0),
        QueryKind::ExistsEqual(expected) => {
            query.values.iter().position(|value| *value == expected)
        }
    };
    let value = match query.kind {
        QueryKind::ForAllNonNegative => position.is_none(),
        QueryKind::ExistsEqual(_) => position.is_some(),
    };
    let examined = position.map_or(query.values.len(), |index| index + 1);
    Ok(CollectionLowering {
        query,
        value,
        examined,
    })
}

#[cfg(test)]
mod tests {
    use quire_contract_ir::kani::{
        CapabilityDisposition, CapabilityEntry, DispatchIndex, FiniteInput, KaniOutcomeKind,
        KaniProfile, ModuleDescriptor, PopulationCompleteness, ProfileSelection, ResourceBounds,
        SemanticFamily,
    };

    use super::{prepare_bounded_collection_query, CollectionQuery, QueryKind};

    fn fixture() -> (
        KaniProfile,
        DispatchIndex,
        quire_contract_ir::kani::ValidatedFiniteInput,
    ) {
        let selection = ProfileSelection {
            profile: "kani-bounded/1".to_owned(),
            revision: "r1".to_owned(),
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

    /// Trace: TC-023.
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
        let exhausted = exhausted.outcome().expect("IR outcome was built");
        assert_eq!(exhausted.kind, KaniOutcomeKind::ResourceExhausted);
        assert_eq!(exhausted.boolean_claim(), None);
    }
}

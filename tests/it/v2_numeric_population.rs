//! Public V2 population and census construction from IR-admitted selected claims.

use ix_trace_rs::trace;
use quire_contract_codegen::{
    admit_v2_integer_comparison, prepare_v2_numeric_population, Population, PopulationSide,
    StrategyErrorCode, V2SelectedClaimKey,
};

use super::v2_integer_comparison_admission::admitted;

/// The V2 path carries the source-authored equality's constructive sides and full census.
#[trace("TC-018", "TC-019")]
#[test]
fn selected_v2_equality_prepares_constructive_population_and_census() {
    let (package, clause, claim) = admitted("self.balance = pre(self.balance)");
    let selected = admit_v2_integer_comparison(
        &package,
        V2SelectedClaimKey {
            clause: &clause,
            claim_owner: &clause,
            occurrence: &claim,
        },
    )
    .expect("selected V2 comparison");
    let broad = prepare_v2_numeric_population(&selected, Population::Broad)
        .expect("both comparison sides exist");
    assert!(broad.population.source.contains("V2CaseBroad"));
    assert!(broad.population.source.contains("v2_population_broad"));
    assert!(broad.census_source.contains("IN_DOMAIN_CENSUS"));
    assert!(broad.census.in_domain().iter().all(|case| {
        case.tag
            == if case.primary == case.partner.expect("two reads") {
                quire_contract_codegen::CensusTag::Holds
            } else {
                quire_contract_codegen::CensusTag::Violated
            }
    }));
    assert_eq!(broad.census.in_domain().len(), 10);
    let satisfying = quire_contract_codegen::side_values(
        &selected.relation,
        selected.domain,
        PopulationSide::Satisfying,
    )
    .expect("equality side");
    let violating = quire_contract_codegen::side_values(
        &selected.relation,
        selected.domain,
        PopulationSide::Violating,
    )
    .expect("inequality side");
    for (post, pre) in [(0, 0), (0, 1), (1, 0), (999, 999), (1000, 1000)] {
        assert_eq!(satisfying.contains(post, Some(pre)), post == pre);
        assert_eq!(violating.contains(post, Some(pre)), post != pre);
    }
}

/// An absent requested side refuses before source is returned, with the selected key intact.
#[trace("TC-018", "FR-009-AC-4")]
#[test]
fn selected_v2_empty_side_returns_typed_no_population_refusal() {
    let (package, clause, claim) = admitted("self.balance < 0");
    let selected = admit_v2_integer_comparison(
        &package,
        V2SelectedClaimKey {
            clause: &clause,
            claim_owner: &clause,
            occurrence: &claim,
        },
    )
    .expect("the comparison itself is admitted");
    let refusal = prepare_v2_numeric_population(&selected, Population::Broad)
        .expect_err("the satisfying side is empty");
    assert_eq!(refusal.code, StrategyErrorCode::EmptyPopulation);
    assert_eq!(refusal.clause, clause);
    assert_eq!(refusal.claim_owner, clause);
    assert_eq!(refusal.occurrence, claim);
    assert_eq!(refusal.path, "population.satisfying");
    let violating = prepare_v2_numeric_population(&selected, Population::Violating)
        .expect("every admitted value violates x < 0");
    assert!(!violating.census.in_domain().is_empty());
}

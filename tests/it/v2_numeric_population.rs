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

/// Authored literal-first predicates and their mirrors generate the independently enumerated sets.
#[trace("TC-018", "FR-009-AC-7")]
#[test]
fn selected_v2_literal_mirrors_generate_exact_population_sets() {
    use std::{fmt::Write as _, fs, process::Command};

    use super::{
        bound_populations::GENERATED_SUPPORT, kani_obligations_state_frame::native_twin::Twin,
        scratch_crate, v2_integer_comparison_admission::admit_twin,
    };

    type Predicate = fn(i64) -> bool;
    let comparisons: [(&str, &str, Predicate); 4] = [
        ("<", ">", |value| value < 1),
        ("<=", ">=", |value| value <= 1),
        (">", "<", |value| value > 1),
        (">=", "<=", |value| value >= 1),
    ];
    let scratch = scratch_crate::TemporaryDirectory::new("v2-literal-mirror-populations");
    let mut source = String::from("mod support;\n");
    for (index, (read_operator, literal_operator, predicate)) in comparisons.into_iter().enumerate()
    {
        let conditions = [
            format!("self.balance {read_operator} 1"),
            format!("1 {literal_operator} self.balance"),
        ];
        let mut mirrored_sets = Vec::new();
        for (position, condition) in conditions.into_iter().enumerate() {
            let (package, clause, claim) =
                admit_twin(Twin::with_balance_condition_in_range(&condition, (0, 3)));
            let selected = admit_v2_integer_comparison(
                &package,
                V2SelectedClaimKey {
                    clause: &clause,
                    claim_owner: &clause,
                    occurrence: &claim,
                },
            )
            .expect("source-authored literal comparison");
            assert_eq!(selected.domain.minimum, 0);
            assert_eq!(selected.domain.maximum, 3);
            let mut sets = Vec::new();
            for (side, holds) in [
                (PopulationSide::Satisfying, true),
                (PopulationSide::Violating, false),
            ] {
                let actual =
                    quire_contract_codegen::side_values(&selected.relation, selected.domain, side)
                        .expect("both literal sides exist");
                let expected: Vec<_> = (0..=3).filter(|value| predicate(*value) == holds).collect();
                let actual: Vec<_> = (0..=3)
                    .filter(|value| actual.contains(*value, None))
                    .collect();
                assert_eq!(actual, expected, "{condition}, {side:?}");
                sets.push(actual);
            }
            mirrored_sets.push(sets);
            for population in Population::ALL {
                let generated = prepare_v2_numeric_population(&selected, population)
                    .expect("both comparison sides are nonempty")
                    .population;
                let name = format!("comparison_{index}_{position}_{population:?}").to_lowercase();
                let expected = (0..=3)
                    .filter_map(|value| {
                        let holds = predicate(value);
                        let included = match population {
                            Population::Satisfying => holds,
                            Population::Violating => !holds,
                            Population::Broad => true,
                        };
                        included.then(|| format!("({value}i64, None, {holds})"))
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(source, "mod {name} {{\n{}\n}}", generated.source).unwrap();
                writeln!(
                    source,
                    "#[test] fn {name}_enumerates() {{\n\
                     support::check_small({condition:?}, {name}::{}(),\n\
                     |case: &{name}::{}| (case.primary, None, case.expected == {name}::{}::Holds),\n\
                     &[{expected}], 0, 3);\n}}",
                    generated.strategy_function, generated.case_type, generated.expectation_type,
                )
                .unwrap();
            }
        }
        assert_eq!(
            mirrored_sets[0], mirrored_sets[1],
            "operator {read_operator}"
        );
    }
    scratch_crate::write_manifest(
        &scratch.path,
        "[package]\nname = \"v2-literal-mirror-populations\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[dependencies]\nproptest = \"=1.5.0\"\n[workspace]\n",
    );
    fs::write(scratch.path.join("src/lib.rs"), source).expect("generated population controls");
    fs::write(scratch.path.join("src/support.rs"), GENERATED_SUPPORT)
        .expect("shared generated value-tree walker");
    scratch_crate::resolve_seeded_lock(&scratch.path);
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args(["test", "--locked", "--offline", "-j2", "--lib"])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .current_dir(&scratch.path)
        .output()
        .expect("run generated mirrored populations");
    assert!(
        output.status.success(),
        "generated mirrored populations failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

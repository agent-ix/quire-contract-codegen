// SPDX-License-Identifier: AGPL-3.0-or-later
//! TC-007 differential observations for the surviving bound integer pair.

use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path},
    process::Command,
};

use ix_trace_rs::trace;
use quire_contract_codegen::{
    generate_bound_oracles, BoundOracleGeneration, BoundStrategyPopulation,
};
use quire_contract_model::{ClauseId, ClauseRef, RequirementRef};
use serde::Deserialize;

use crate::{
    bound_strategy_generation::{
        decode, generate_projection_population, generated_census_runner, generated_item,
        scalar_projection,
    },
    scratch_crate::{resolve_seeded_lock, runtime_dependency, write_manifest, TemporaryDirectory},
};

const PAIR: &str = "bound_strategy_census / executable_bound_oracle";
const REGISTER: &str = include_str!("../../spec/core/matrix/TC-007-cross-backend-parity.md");
const MUTANT_PATH: &str = "tests/it/fixtures/tc007-mutant-bound-census.json";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Outcome {
    Holds,
    Violated,
}

impl Outcome {
    fn flipped(self) -> Self {
        match self {
            Self::Holds => Self::Violated,
            Self::Violated => Self::Holds,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Comparable {
    case_id: String,
    census: Outcome,
    oracle: Outcome,
    runner: Outcome,
    dependency: String,
    diagnostic: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OutsideDomain {
    case_id: String,
    oracle: Outcome,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Key {
    case_id: String,
    pair: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Difference {
    key: Key,
    census: Outcome,
    oracle: Outcome,
    dependency: String,
    diagnostic: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisterRow {
    key: Key,
    disposition: String,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum JoinError {
    Missing(Key),
    Duplicate(Key),
    Stale(Key),
    BadDisposition(Key),
    BadFixture(Key),
    DuplicateMeasurement(Key),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MutantFixture {
    case_id: String,
    backend_pair: String,
    mutation: String,
}

#[ix_trace_rs::string_edge]
fn normalized_tag(text: &str) -> Outcome {
    // Branch-gating generated census spelling: an unknown tag fails the comparison.
    match text {
        "Holds" => Outcome::Holds,
        "Violated" => Outcome::Violated,
        other => panic!("unknown generated census tag {other:?}"),
    }
}

#[ix_trace_rs::string_edge]
fn normalized_boolean(text: &str) -> Outcome {
    // Branch-gating generated Boolean spelling: an unknown result fails the comparison.
    match text {
        "true" => Outcome::Holds,
        "false" => Outcome::Violated,
        other => panic!("unknown executable-oracle Boolean {other:?}"),
    }
}

#[ix_trace_rs::string_edge]
fn normalized_verdict(text: &str) -> Outcome {
    // Branch-gating runtime verdict spelling: the supported invariant has two outcomes.
    match text {
        "Passed" => Outcome::Holds,
        "FailedPostcondition" => Outcome::Violated,
        other => panic!("unknown generated runner verdict {other:?}"),
    }
}

#[ix_trace_rs::string_edge]
fn register_rows(markdown: &str) -> Vec<RegisterRow> {
    let section = markdown
        .split_once("## Differential Disposition Register")
        .expect("TC-007 register heading")
        .1
        .split_once("## Expected Results")
        .expect("TC-007 expected-results heading")
        .0;
    let mut lines = section.lines().skip_while(|line| {
        !line.starts_with("| Case | Backends | Disposition | Fixture or distinction |")
    });
    assert_eq!(
        lines.next(),
        Some("| Case | Backends | Disposition | Fixture or distinction |")
    );
    assert_eq!(
        lines.next(),
        Some("|------|----------|-------------|------------------------|")
    );
    let table = lines.collect::<Vec<_>>().join("\n");
    table
        .trim_end()
        .lines()
        .map(|line| {
            assert!(
                line.starts_with('|') && line.ends_with('|'),
                "malformed TC-007 register row: {line:?}"
            );
            let cells: Vec<_> = line.trim_matches('|').split('|').map(str::trim).collect();
            let [case_id, pair, disposition, detail] = cells.as_slice() else {
                panic!("TC-007 register row has {} cells: {line}", cells.len());
            };
            RegisterRow {
                key: Key {
                    case_id: (*case_id).to_owned(),
                    pair: (*pair).to_owned(),
                },
                disposition: (*disposition).to_owned(),
                detail: (*detail).to_owned(),
            }
        })
        .collect()
}

#[ix_trace_rs::string_edge]
fn validate_row(row: &RegisterRow) -> Result<(), JoinError> {
    // Branch-gating TC-007 disposition vocabulary: unknown kinds fail closed.
    match row.disposition.as_str() {
        "fixture" => {
            let path = Path::new(&row.detail);
            if !path.starts_with("tests")
                || path
                    .components()
                    .any(|part| !matches!(part, Component::Normal(_)))
                || !Path::new(env!("CARGO_MANIFEST_DIR")).join(path).is_file()
            {
                return Err(JoinError::BadFixture(row.key.clone()));
            }
        }
        "profile/backend distinction" if !row.detail.is_empty() => {}
        _ => return Err(JoinError::BadDisposition(row.key.clone())),
    }
    Ok(())
}

fn join(differences: &[Difference], rows: &[RegisterRow]) -> Result<(), JoinError> {
    let mut measured = BTreeSet::new();
    for difference in differences {
        if !measured.insert(difference.key.clone()) {
            return Err(JoinError::DuplicateMeasurement(difference.key.clone()));
        }
        let matches: Vec<_> = rows
            .iter()
            .filter(|row| row.key == difference.key)
            .collect();
        let [row] = matches.as_slice() else {
            return Err(if matches.is_empty() {
                JoinError::Missing(difference.key.clone())
            } else {
                JoinError::Duplicate(difference.key.clone())
            });
        };
        validate_row(row)?;
    }
    for row in rows {
        if !measured.contains(&row.key) {
            return Err(JoinError::Stale(row.key.clone()));
        }
    }
    Ok(())
}

fn differences(observations: &[Comparable]) -> Vec<Difference> {
    observations
        .iter()
        .filter(|case| case.census != case.oracle)
        .map(|case| Difference {
            key: Key {
                case_id: case.case_id.clone(),
                pair: PAIR.to_owned(),
            },
            census: case.census,
            oracle: case.oracle,
            dependency: case.dependency.clone(),
            diagnostic: case.diagnostic.clone(),
        })
        .collect()
}

fn generated_observations() -> (Vec<Comparable>, Vec<OutsideDomain>, (u64, u64, u64)) {
    let projection = scalar_projection("test/tc007-bound-parity", "invariant", "less", 500);
    let package = decode(&projection);
    let clause = ClauseRef::new(
        RequirementRef::parse("test/tc007-bound-parity", "FR-100", 3).expect("requirement"),
        ClauseId::new("amount-check").expect("clause"),
    );
    let strategy = generate_projection_population(
        &projection,
        &clause,
        BoundStrategyPopulation::Boundary,
        1,
        0,
        0,
    );
    let BoundOracleGeneration::Generated(oracles) =
        generate_bound_oracles(&package).expect("bound oracle generation")
    else {
        panic!("the admitted bound package has no executable clause");
    };
    let [oracle_clause] = oracles.clauses() else {
        panic!("TC-007 requires exactly one generated bound oracle");
    };
    assert_eq!(oracle_clause.identity(), &clause);
    let oracle_source = &oracle_clause.bundle().rust.contents;
    let strategy_source = &strategy.rust.contents;
    let oracle = generated_item(oracle_source, "pub fn oracle_");
    let census = generated_item(strategy_source, "pub const IN_DOMAIN_CENSUS_");
    let outside = generated_item(strategy_source, "pub const OUT_OF_DOMAIN_CASES_");
    let case_type = generated_item(strategy_source, "pub struct CensusCase");
    let census_tag_type = generated_item(strategy_source, "pub enum CensusTag");
    let bound_case_type = generated_item(strategy_source, "pub struct BoundCase");
    let expectation_type = generated_item(strategy_source, "pub enum BoundExpectation");
    let census_runner = generated_census_runner(strategy_source);
    let evaluator = format!(
        "{}_evaluate",
        census_runner
            .strip_suffix("_run_census")
            .expect("runner suffix")
    );
    let mut strategy_source = strategy_source.clone();
    strategy_source.push_str(&format!(
        r#"
/// Test-only observation of one generated census runner verdict.
pub fn tc007_verdict(case: &{case_type}) -> quire_contract_runtime::VerdictKind {{
    let bound_case = {bound_case_type} {{
        amount_current: case.amount_current,
        expected: match case.tag {{
            {census_tag_type}::Holds => {expectation_type}::Holds,
            {census_tag_type}::Violated => {expectation_type}::Violated,
        }},
    }};
    let mut report = quire_contract_runtime::CampaignReport::new(
        quire_contract_runtime::ContractIdentity::new(
            quire_contract_runtime::RequirementId::new("FR-100"),
            quire_contract_runtime::RevisionId::new("3"),
        ),
    );
    {evaluator}(&bound_case, &mut report).expect("one generated census case evaluates");
    let counts = report.counts();
    match (counts.accepted(), counts.failed(), counts.rejected(), counts.discarded()) {{
        (1, 0, 0, 0) => quire_contract_runtime::VerdictKind::Passed,
        (1, 1, 0, 0) => quire_contract_runtime::VerdictKind::FailedPostcondition,
        other => panic!("unexpected one-case campaign counts: {{other:?}}"),
    }}
}}
"#
    ));
    let main_source = format!(
        r#"
#![allow(dead_code)] // The narrow consumer exercises only the TC-007 generated entry points.
mod executable_oracle;
mod bound_strategy;

fn main() {{
    let mut report = quire_contract_runtime::CampaignReport::new(
        quire_contract_runtime::ContractIdentity::new(
            quire_contract_runtime::RequirementId::new("FR-100"),
            quire_contract_runtime::RevisionId::new("3"),
        ),
    );
    let summary = bound_strategy::{census_runner}(&mut report).expect("complete census runner");
    println!("SUMMARY|{{}}|{{}}|{{}}", summary.attempted, summary.accepted, summary.failed);
    for case in bound_strategy::{census} {{
        let oracle = executable_oracle::{oracle}(case.amount_current);
        let verdict = bound_strategy::tc007_verdict(&case);
        println!("IN|amount:{{}}|{{:?}}|{{oracle}}|{{verdict:?}}", case.amount_current, case.tag);
    }}
    for case in bound_strategy::{outside} {{
        let oracle = executable_oracle::{oracle}(case.amount_current);
        println!("OUT|amount:{{}}|{{oracle}}", case.amount_current);
    }}
}}
"#
    );
    let scratch = TemporaryDirectory::new("ir1295-tc007-bound-parity");
    fs::write(scratch.path.join("src/executable_oracle.rs"), oracle_source).expect("oracle source");
    fs::write(scratch.path.join("src/bound_strategy.rs"), strategy_source)
        .expect("strategy source");
    fs::write(scratch.path.join("src/main.rs"), main_source).expect("consumer source");
    write_manifest(
        &scratch.path,
        &format!(
            "[package]\nname = \"ir1295-tc007-bound-parity\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nproptest = {{ version = \"=1.5.0\", default-features = false, features = [\"std\"] }}\n{}\n\n[workspace]\n",
            runtime_dependency(&[])
        ),
    );
    resolve_seeded_lock(&scratch.path);
    let output = Command::new(env!("CARGO"))
        .args(["run", "--offline", "--locked", "--quiet"])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&scratch.path)
        .output()
        .expect("run generated TC-007 consumer");
    assert!(
        output.status.success(),
        "generated consumer failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("generated stdout UTF-8");
    let dependencies = package.clauses()[0].expression().dependencies();
    assert_eq!(dependencies.len(), 1, "one checked input dependency");
    let dependency = format!("{:?}", dependencies.iter().next().expect("dependency"));
    let mut comparable = Vec::new();
    let mut outside = Vec::new();
    let mut summary = None;
    for line in stdout.lines() {
        let cells: Vec<_> = line.split('|').collect();
        match cells.as_slice() {
            ["SUMMARY", attempted, accepted, failed] => {
                assert!(summary.is_none(), "one generated summary");
                summary = Some((
                    attempted.parse().expect("attempted"),
                    accepted.parse().expect("accepted"),
                    failed.parse().expect("failed"),
                ));
            }
            ["IN", case_id, tag, oracle, verdict] => comparable.push(Comparable {
                case_id: (*case_id).to_owned(),
                census: normalized_tag(tag),
                oracle: normalized_boolean(oracle),
                runner: normalized_verdict(verdict),
                dependency: dependency.clone(),
                diagnostic: None,
            }),
            ["OUT", case_id, oracle] => outside.push(OutsideDomain {
                case_id: (*case_id).to_owned(),
                oracle: normalized_boolean(oracle),
            }),
            other => panic!("unexpected generated TC-007 output: {other:?}"),
        }
    }
    (
        comparable,
        outside,
        summary.expect("generated census summary"),
    )
}

/// Trace: TC-007, FR-005-AC-3.
#[trace("TC-007", "FR-005-AC-3")]
#[test]
fn tc007_bound_census_and_executable_oracle_join_the_authoritative_register() {
    let (cases, outside, summary) = generated_observations();
    let identities: Vec<_> = cases.iter().map(|case| case.case_id.as_str()).collect();
    assert_eq!(
        identities,
        [
            "amount:0",
            "amount:1",
            "amount:499",
            "amount:500",
            "amount:501",
            "amount:999",
            "amount:1000"
        ]
    );
    assert_eq!(summary, (7, 7, 4), "complete typed runner census");
    assert_eq!(
        outside,
        [
            OutsideDomain {
                case_id: "amount:-1".to_owned(),
                oracle: Outcome::Holds
            },
            OutsideDomain {
                case_id: "amount:1001".to_owned(),
                oracle: Outcome::Violated
            },
        ],
        "outside-domain observations remain NotComparable"
    );
    assert!(
        cases.iter().all(|case| case.runner == case.oracle),
        "typed runner verdict must match executable oracle"
    );
    assert!(cases.iter().any(|case| case.census == Outcome::Holds));
    assert!(cases.iter().any(|case| case.census == Outcome::Violated));
    assert!(cases
        .iter()
        .all(|case| case.diagnostic.is_none() && !case.dependency.is_empty()));
    let measured = differences(&cases);
    let authoritative = register_rows(REGISTER);
    join(&measured, &authoritative)
        .expect("every measured discrepancy has exactly one authoritative row");

    let fixture: MutantFixture =
        serde_json::from_str(include_str!("fixtures/tc007-mutant-bound-census.json"))
            .expect("mutant fixture");
    assert_eq!(fixture.mutation, "flip-census-tag");
    assert_eq!(fixture.backend_pair, PAIR);
    let mut mutant = cases
        .iter()
        .find(|case| case.case_id == fixture.case_id)
        .expect("fixture names an admitted case")
        .clone();
    mutant.census = mutant.census.flipped();
    let mutant_differences = differences(&[mutant]);
    let [mutant_difference] = mutant_differences.as_slice() else {
        panic!("mutant produces one named discrepancy");
    };
    assert_eq!(mutant_difference.key.case_id, fixture.case_id);
    let row = RegisterRow {
        key: mutant_difference.key.clone(),
        disposition: "fixture".to_owned(),
        detail: MUTANT_PATH.to_owned(),
    };
    let row_line = format!(
        "| {} | {} | {} | {} |",
        row.key.case_id, row.key.pair, row.disposition, row.detail
    );
    let seeded_markdown = REGISTER.replacen(
        "|------|----------|-------------|------------------------|",
        &format!("|------|----------|-------------|------------------------|\n{row_line}"),
        1,
    );
    let seeded_rows = register_rows(&seeded_markdown);
    assert_eq!(seeded_rows.as_slice(), std::slice::from_ref(&row));
    join(&mutant_differences, &seeded_rows).expect("temporary mutant register joins exactly once");
    let malformed_markdown = seeded_markdown.replacen(
        &row_line,
        &format!("{row_line}\nmalformed row before duplicate\n{row_line}"),
        1,
    );
    assert!(
        std::panic::catch_unwind(|| register_rows(&malformed_markdown)).is_err(),
        "a malformed row cannot hide a later duplicate"
    );
    assert_eq!(
        join(&mutant_differences, &[]),
        Err(JoinError::Missing(row.key.clone()))
    );
    assert_eq!(
        join(&mutant_differences, &[row.clone(), row.clone()]),
        Err(JoinError::Duplicate(row.key.clone()))
    );
    let mut bad_fixture = row.clone();
    bad_fixture.detail = "tests/it/fixtures/absent-tc007.json".to_owned();
    assert_eq!(
        join(&mutant_differences, &[bad_fixture]),
        Err(JoinError::BadFixture(row.key.clone()))
    );
    let mut empty_fixture = row.clone();
    empty_fixture.detail.clear();
    assert_eq!(
        join(&mutant_differences, &[empty_fixture]),
        Err(JoinError::BadFixture(row.key.clone()))
    );
    let mut empty_disposition = row.clone();
    empty_disposition.disposition.clear();
    assert_eq!(
        join(&mutant_differences, &[empty_disposition]),
        Err(JoinError::BadDisposition(row.key.clone()))
    );
    let mut both_dispositions = row.clone();
    both_dispositions.disposition = "fixture; profile/backend distinction".to_owned();
    assert_eq!(
        join(&mutant_differences, &[both_dispositions]),
        Err(JoinError::BadDisposition(row.key.clone()))
    );
    assert_eq!(
        join(&[], &[row]),
        Err(JoinError::Stale(mutant_difference.key.clone()))
    );
}

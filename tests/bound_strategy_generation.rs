//! Synthetic public executable projections for the complete numeric strategy slice.

mod common;

use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    generate_bound_strategy, AttestationContext, BoundStrategyPopulation, BoundStrategyRequest,
    GenerationTerminalState, ProofAttestationBody, StrategyErrorCode, IR_CANDIDATE_REVISION,
};
use quire_contract_ir::{
    BoundPackage, ClauseId, ClauseRef, RequirementRef, BOUND_IDENTITY_PROFILE,
    EXECUTABLE_PROJECTION_FORMAT,
};
use serde_json::{json, Value};

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new(prefix: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(path.join("src")).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn context() -> AttestationContext<'static> {
    AttestationContext {
        record_digest: "0000000000000000000000000000000000000000000000000000000000000000",
        candidate_revision: IR_CANDIDATE_REVISION,
    }
}

fn span(line: u64) -> Value {
    let source = json!({"document":"synthetic-numeric", "revision":1});
    json!({"start":{"source":source,"line":line,"column":1,"byte_offset":line - 1},
        "end":{"source":source,"line":line,"column":2,"byte_offset":line}})
}

fn version_projection() -> Value {
    let package = "test/version-strategy";
    let owner = json!({"package":package,"requirement":"FR-034","revision":9});
    let anchor = json!({"kind":"post","operation":"attemptUpdate"});
    json!({
        "format": EXECUTABLE_PROJECTION_FORMAT,
        "package": {
            "id": package,
            "schema_version":{"major":1,"minor":1},
            "source":{"document":"synthetic-numeric","revision":1},
            "requirements":[{
                "id":"FR-034", "revision":9, "source":span(1),
                "clauses":[{
                    "id":"VersionUnchanged", "kind":"postcondition", "anchor":anchor,
                    "source":span(2),
                    "body":{"node":"composite","children":[
                        {"node":"reference","identity":{"requirement":owner,"kind":"state","observation":"post","path":["versionNumber"]}},
                        {"node":"reference","identity":{"requirement":owner,"kind":"state","observation":"pre","path":["versionNumber"]}}
                    ]}
                }]
            }]
        },
        "bindings":[{
            "clause":{"requirement":owner,"clause":"VersionUnchanged"},
            "expression":{
                "owner":owner,
                "types":[],
                "values":[{"name":"versionNumber","kind":"state","value_type":{"kind":"integer","domain":"signed","minimum":0,"maximum":1000,"overflow":"reject"},"source":span(3)}],
                "functions":[],
                "expression":{"node":"compare","operator":"equal",
                    "left":{"node":"value_reference","name":"versionNumber","observation":"post","source":span(4)},
                    "right":{"node":"value_reference","name":"versionNumber","observation":"pre","source":span(5)},
                    "source":span(3)},
                "expected_type":{"kind":"boolean"},
                "execution_point":anchor,
                "clause_root":true
            }
        }]
    })
}

fn boolean_comparison_projection() -> Value {
    let mut value = version_projection();
    value["package"]["requirements"][0]["clauses"][0]["body"] = json!({
        "node":"reference",
        "identity":{
            "requirement":{"package":"test/version-strategy","requirement":"FR-034","revision":9},
            "kind":"state", "observation":"post", "path":["flag"]
        }
    });
    value["bindings"][0]["expression"]["values"] = json!([{
        "name":"flag", "kind":"state", "value_type":{"kind":"boolean"}, "source":span(3)
    }]);
    value["bindings"][0]["expression"]["expression"] = json!({
        "node":"compare", "operator":"equal",
        "left":{"node":"value_reference","name":"flag","observation":"post","source":span(4)},
        "right":{"node":"boolean_literal","value":true,"source":span(5)},
        "source":span(3)
    });
    value
}

fn scalar_projection(package: &str, kind: &str, operator: &str, literal: i64) -> Value {
    let owner = json!({"package":package,"requirement":"FR-100","revision":3});
    let (declaration_kind, anchor) = if kind == "precondition" {
        ("input", json!({"kind":"pre","operation":"checkAmount"}))
    } else {
        ("state", json!({"kind":"handler","name":"checkAmount"}))
    };
    let body_identity = json!({
        "requirement":owner, "kind":declaration_kind, "observation":"current", "path":["amount"]
    });
    json!({
        "format":EXECUTABLE_PROJECTION_FORMAT,
        "package":{
            "id":package, "schema_version":{"major":1,"minor":1},
            "source":{"document":"synthetic-numeric","revision":1},
            "requirements":[{"id":"FR-100","revision":3,"source":span(1),"clauses":[{
                "id":"amount-check", "kind":kind, "anchor":anchor, "source":span(2),
                "body":{"node":"reference","identity":body_identity}
            }]}]
        },
        "bindings":[{
            "clause":{"requirement":owner,"clause":"amount-check"},
            "expression":{
                "owner":owner, "types":[],
                "values":[{"name":"amount","kind":declaration_kind,"value_type":{"kind":"integer","domain":"signed","minimum":0,"maximum":1000,"overflow":"reject"},"source":span(3)}],
                "functions":[],
                "expression":{"node":"compare","operator":operator,
                    "left":{"node":"value_reference","name":"amount","observation":"current","source":span(4)},
                    "right":{"node":"integer_literal","value":literal,"value_type":{"kind":"integer","domain":"signed","minimum":0,"maximum":1000,"overflow":"reject"},"source":span(5)},
                    "source":span(3)},
                "expected_type":{"kind":"boolean"}, "execution_point":anchor, "clause_root":true
            }
        }]
    })
}

fn decode(value: &Value) -> BoundPackage {
    BoundPackage::from_json_bytes(&serde_json::to_vec(value).unwrap()).unwrap()
}

fn clause_ref() -> ClauseRef {
    ClauseRef::new(
        RequirementRef::parse("test/version-strategy", "FR-034", 9).unwrap(),
        ClauseId::new("VersionUnchanged").unwrap(),
    )
}

fn generate(
    population: BoundStrategyPopulation,
) -> quire_contract_codegen::GeneratedArtifactBundle {
    let package = decode(&version_projection());
    generate_bound_strategy(&BoundStrategyRequest {
        package: &package,
        clause: &clause_ref(),
        population,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap()
}

fn generate_projection(
    value: &Value,
    clause: &ClauseRef,
    minimum_rejected_cases: u64,
) -> quire_contract_codegen::GeneratedArtifactBundle {
    let package = decode(value);
    generate_bound_strategy(&BoundStrategyRequest {
        package: &package,
        clause,
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap()
}

fn generated_item(source: &str, prefix: &str) -> String {
    source
        .lines()
        .find_map(|line| {
            line.starts_with(prefix).then(|| {
                line.split_whitespace()
                    .nth(2)
                    .expect("public item name")
                    .split(['(', '<', ' ', '{'])
                    .next()
                    .unwrap()
                    .to_owned()
            })
        })
        .unwrap_or_else(|| panic!("generated source has no item beginning {prefix:?}"))
}

/// Trace: TC-017, FR-008-AC-1, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-008-AC-5, FR-008-CON-2
#[test]
fn tc_017_bound_admission_uses_the_public_clause_and_domain() {
    let package = decode(&version_projection());
    let clause = clause_ref();
    let first = generate_bound_strategy(&BoundStrategyRequest {
        package: &package,
        clause: &clause,
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap();
    let second = generate(BoundStrategyPopulation::Broad);
    assert_eq!(first, second);
    assert!(first.rust.contents.contains("DOMAIN_MINIMUM: i64 = 0"));
    assert!(first.rust.contents.contains("DOMAIN_MAXIMUM: i64 = 1000"));
    assert!(first
        .rust
        .contents
        .contains("VERSION_4EUMBER_POST_DECLARATION"));
    assert!(first
        .rust
        .contents
        .contains("VERSION_4EUMBER_PRE_OBSERVATION"));
    assert!(first.rust.contents.contains("\"versionNumber\""));
    assert!(first.rust.contents.contains("\"post\""));
    assert!(first.rust.contents.contains("\"pre\""));

    let unknown = ClauseRef::new(
        clause.requirement().clone(),
        ClauseId::new("missing").unwrap(),
    );
    let error = generate_bound_strategy(&BoundStrategyRequest {
        package: &package,
        clause: &unknown,
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap_err();
    assert_eq!(error.code, StrategyErrorCode::UnknownClause);
    assert_eq!(error.terminal_state, GenerationTerminalState::InvalidInput);
    assert_eq!(error.clause.as_deref(), Some(&unknown));
    assert!(error.source_span.is_none());

    let mut assertion_value = version_projection();
    assertion_value["package"]["requirements"][0]["clauses"][0]["kind"] = json!("assertion");
    let assertion = decode(&assertion_value);
    let error = generate_bound_strategy(&BoundStrategyRequest {
        package: &assertion,
        clause: &clause,
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap_err();
    assert_eq!(error.code, StrategyErrorCode::UnsupportedClauseKind);
    assert_eq!(error.terminal_state, GenerationTerminalState::Unsupported);
    assert!(error.source_span.is_none());

    let boolean = decode(&boolean_comparison_projection());
    let error = generate_bound_strategy(&BoundStrategyRequest {
        package: &boolean,
        clause: &clause,
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap_err();
    assert_eq!(error.code, StrategyErrorCode::UnsupportedRelation);
    assert_eq!(error.terminal_state, GenerationTerminalState::Unsupported);
    assert_eq!(error.clause.as_deref(), Some(&clause));
    assert!(error.source_span.is_some());

    let mut same_read_value = version_projection();
    same_read_value["bindings"][0]["expression"]["expression"]["right"]["observation"] =
        json!("post");
    same_read_value["package"]["requirements"][0]["clauses"][0]["body"]["children"][1]
        ["identity"]["observation"] = json!("post");
    let same_read = decode(&same_read_value);
    let error = generate_bound_strategy(&BoundStrategyRequest {
        package: &same_read,
        clause: &clause,
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap_err();
    assert_eq!(error.code, StrategyErrorCode::UnsupportedRelation);
    assert!(error.source_span.is_some());

    let arithmetic_value = {
        let mut value = scalar_projection("test/arithmetic", "precondition", "less", 7);
        let integer = json!({"kind":"integer","domain":"signed","minimum":0,"maximum":1000,"overflow":"reject"});
        value["bindings"][0]["expression"]["expression"]["left"] = json!({
            "node":"numeric", "operator":"add",
            "left":{"node":"value_reference","name":"amount","observation":"current","source":span(4)},
            "right":{"node":"integer_literal","value":0,"value_type":integer,"source":span(6)},
            "source":span(4)
        });
        value
    };
    let arithmetic = decode(&arithmetic_value);
    let arithmetic_clause = ClauseRef::new(
        RequirementRef::parse("test/arithmetic", "FR-100", 3).unwrap(),
        ClauseId::new("amount-check").unwrap(),
    );
    let error = generate_bound_strategy(&BoundStrategyRequest {
        package: &arithmetic,
        clause: &arithmetic_clause,
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap_err();
    assert_eq!(error.code, StrategyErrorCode::UnsupportedClause);
    assert!(matches!(
        error.generation_code,
        Some(
            quire_contract_codegen::GenerationErrorCode::UnsupportedExpression
                | quire_contract_codegen::GenerationErrorCode::UnsupportedObligations
        )
    ));
    assert!(error.source_span.is_some());

    for population in [
        BoundStrategyPopulation::Satisfying,
        BoundStrategyPopulation::Violating,
        BoundStrategyPopulation::Broad,
        BoundStrategyPopulation::Boundary,
    ] {
        assert!(generate_bound_strategy(&BoundStrategyRequest {
            package: &package,
            clause: &clause,
            population,
            minimum_accepted_cases: 1,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
            attestation: context(),
        })
        .is_ok());
    }

    let single_tag_value =
        scalar_projection("test/single-tagged", "precondition", "less_equal", 1000);
    let single_tag_package = decode(&single_tag_value);
    let single_tag_clause = ClauseRef::new(
        RequirementRef::parse("test/single-tagged", "FR-100", 3).unwrap(),
        ClauseId::new("amount-check").unwrap(),
    );
    let satisfying = generate_bound_strategy(&BoundStrategyRequest {
        package: &single_tag_package,
        clause: &single_tag_clause,
        population: BoundStrategyPopulation::Satisfying,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap();
    assert!(satisfying.rust.contents.contains("OUT_OF_DOMAIN_CASES_"));
    assert!(!satisfying.rust.contents.contains("IN_DOMAIN_CENSUS_"));
    let boundary_error = generate_bound_strategy(&BoundStrategyRequest {
        package: &single_tag_package,
        clause: &single_tag_clause,
        population: BoundStrategyPopulation::Boundary,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap_err();
    assert_eq!(
        boundary_error.code,
        StrategyErrorCode::UnsupportedCampaignConstraint
    );
    assert_eq!(boundary_error.clause.as_deref(), Some(&single_tag_clause));
    assert!(boundary_error.source_span.is_none());
}

/// Trace: TC-020, TC-021, FR-011-AC-3, FR-011-AC-4, FR-012-AC-4
#[test]
fn tc_020_tc_021_runner_rates_census_and_replay_surface_are_generated() {
    let source = generate(BoundStrategyPopulation::Boundary).rust.contents;
    assert!(source.contains("pub const IN_DOMAIN_CENSUS_"));
    assert!(source.contains("run_census"));
    assert!(source.contains("report.record_verdict(&verdict)"));
    assert!(source.contains("discard_rate"));
    assert!(source.contains("rejection_rate"));
    assert!(source.contains("ConformanceMismatch"));
    assert!(!source.contains("adapt_to_proptest"));
    assert!(!source.contains("TestCaseError::reject"));
}

/// Trace: TC-020, TC-021, TC-022, FR-011-AC-1, FR-011-AC-3, FR-011-AC-4,
/// FR-011-AC-5, FR-012-AC-4, FR-013-AC-1
#[test]
fn tc_020_tc_021_tc_022_generated_consumer_runs_sampled_and_census_campaigns() {
    let generated = generate(BoundStrategyPopulation::Broad);
    let strategy = generated_item(&generated.rust.contents, "pub fn bound_strategy_");
    let runner = generated
        .rust
        .contents
        .lines()
        .find_map(|line| {
            line.strip_prefix("pub fn bound_campaign_")
                .filter(|tail| tail.contains("_run<Strategy>"))
                .map(|tail| {
                    format!(
                        "bound_campaign_{}",
                        tail.split('<').next().expect("runner generic marker")
                    )
                })
        })
        .expect("generated sampled runner");
    let census_runner = generated
        .rust
        .contents
        .lines()
        .find_map(|line| {
            line.strip_prefix("pub fn bound_campaign_")
                .filter(|tail| tail.contains("_run_census("))
                .map(|tail| {
                    format!(
                        "bound_campaign_{}",
                        tail.split('(').next().expect("census runner marker")
                    )
                })
        })
        .expect("generated census runner");
    let case_type = generated_item(&generated.rust.contents, "pub struct BoundCase");
    let mut source = generated.rust.contents;
    source.push_str(&format!(
        r#"

#[cfg(test)]
mod generated_checks {{
    use super::*;

    fn report() -> quire_contract_runtime::CampaignReport<'static> {{
        quire_contract_runtime::CampaignReport::new(
            quire_contract_runtime::ContractIdentity::new(
                quire_contract_runtime::RequirementId::new("FR-034"),
                quire_contract_runtime::RevisionId::new("9"),
            ),
        )
    }}

    #[test]
    fn sampled_campaign_has_exact_rates_and_no_discards() {{
        let mut config = proptest::test_runner::Config::with_cases(256);
        config.failure_persistence = None;
        let mut runner = proptest::test_runner::TestRunner::new(config);
        let strategy = {strategy}();
        let mut report = report();
        let summary = {runner}(&mut runner, &strategy, &mut report).unwrap();
        assert_eq!(summary.attempted, 256);
        assert_eq!(summary.discard_rate(), Some((0, 256)));
        assert_eq!(summary.rejection_rate(), Some((0, 256)));
        assert!(summary.failed > 0);
        assert_eq!({case_type}::VERSION_4EUMBER_POST_DECLARATION, "versionNumber");
        assert_eq!({case_type}::VERSION_4EUMBER_POST_OBSERVATION, "post");
        assert_eq!({case_type}::VERSION_4EUMBER_PRE_OBSERVATION, "pre");
    }}

    #[test]
    fn census_is_exact_once_ordered_and_excludes_out_of_domain_cases() {{
        let mut report = report();
        let summary = {census_runner}(&mut report).unwrap();
        assert_eq!(summary.attempted, 10);
        assert_eq!(summary.accepted, 10);
        assert_eq!(summary.failed, 6);
        assert_eq!(summary.discarded, 0);
    }}
}}
"#
    ));
    let temporary = TemporaryDirectory::new("quire-bound-strategy-consumer");
    fs::write(
        temporary.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"bound-strategy-consumer\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nproptest = {{ version = \"=1.5.0\", default-features = false, features = [\"std\"] }}\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{}\" }}\n\n[workspace]\n",
            quire_contract_codegen::RUNTIME_REVISION
        ),
    )
    .unwrap();
    fs::write(temporary.0.join("src/lib.rs"), source).unwrap();
    let output = Command::new(env!("CARGO"))
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", temporary.0.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&temporary.0)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated consumer failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Trace: TC-020, TC-021, FR-011-AC-2, FR-012-AC-4
#[test]
fn tc_020_tc_021_negated_oracle_returns_a_structured_minimal_mismatch() {
    let generated = generate(BoundStrategyPopulation::Broad);
    let strategy = generated_item(&generated.rust.contents, "pub fn bound_strategy_");
    let runner = generated
        .rust
        .contents
        .lines()
        .find_map(|line| {
            line.strip_prefix("pub fn bound_campaign_")
                .filter(|tail| tail.contains("_run<Strategy>"))
                .map(|tail| format!("bound_campaign_{}", tail.split('<').next().unwrap()))
        })
        .expect("generated sampled runner");
    let error_type = generated_item(&generated.rust.contents, "pub enum BoundCampaignError");
    let mut source = generated.rust.contents.replacen("\n==\n", "\n!=\n", 1);
    assert_ne!(
        source, generated.rust.contents,
        "oracle operator was not replaced"
    );
    source.push_str(&format!(
        r#"

#[cfg(test)]
mod mismatch_check {{
    use super::*;

    #[test]
    fn shrink_replays_stay_accounted_in_the_structured_failure() {{
        let mut config = proptest::test_runner::Config::with_cases(1);
        config.failure_persistence = None;
        let mut runner = proptest::test_runner::TestRunner::new(config);
        let strategy = {strategy}();
        let mut report = quire_contract_runtime::CampaignReport::new(
            quire_contract_runtime::ContractIdentity::new(
                quire_contract_runtime::RequirementId::new("FR-034"),
                quire_contract_runtime::RevisionId::new("9"),
            ),
        );
        match {runner}(&mut runner, &strategy, &mut report).unwrap_err() {{
            {error_type}::ConformanceMismatch {{ summary, primary, partner, expected: _, observed: _ }} => {{
                assert!(summary.attempted > 1, "shrink replays were not recorded");
                assert!((0..=1000).contains(&primary));
                assert!(partner.is_some_and(|value| (0..=1000).contains(&value)));
            }}
            other => panic!("expected conformance mismatch, got {{other:?}}"),
        }}
    }}
}}
"#
    ));
    let temporary = TemporaryDirectory::new("quire-bound-strategy-mismatch");
    fs::write(
        temporary.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"bound-strategy-mismatch\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nproptest = {{ version = \"=1.5.0\", default-features = false, features = [\"std\"] }}\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{}\" }}\n\n[workspace]\n",
            quire_contract_codegen::RUNTIME_REVISION
        ),
    )
    .unwrap();
    fs::write(temporary.0.join("src/lib.rs"), source).unwrap();
    let output = Command::new(env!("CARGO"))
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", temporary.0.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&temporary.0)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated mismatch consumer failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Trace: TC-020, FR-011-AC-1, FR-011-AC-5, NFR-004-AC-1
#[test]
fn tc_020_precondition_and_invariant_campaigns_run_ten_thousand_without_discards() {
    let pre_value = scalar_projection(
        "test/amount-precondition",
        "precondition",
        "less_equal",
        500,
    );
    let pre_clause = ClauseRef::new(
        RequirementRef::parse("test/amount-precondition", "FR-100", 3).unwrap(),
        ClauseId::new("amount-check").unwrap(),
    );
    let pre = generate_projection(&pre_value, &pre_clause, 1);
    let invariant_value = scalar_projection("test/amount-invariant", "invariant", "less", 7);
    let invariant_clause = ClauseRef::new(
        RequirementRef::parse("test/amount-invariant", "FR-100", 3).unwrap(),
        ClauseId::new("amount-check").unwrap(),
    );
    let invariant = generate_projection(&invariant_value, &invariant_clause, 0);
    assert!(pre.rust.contents.contains("Verdict::RejectedPrecondition"));
    assert!(invariant.rust.contents.contains("ClauseKind::Invariant"));
    assert!(invariant.rust.contents.contains("FailureKind::Contract"));

    let pre_strategy = generated_item(&pre.rust.contents, "pub fn bound_strategy_");
    let pre_runner = pre
        .rust
        .contents
        .lines()
        .find_map(|line| {
            line.strip_prefix("pub fn bound_campaign_")
                .filter(|tail| tail.contains("_run<Strategy>"))
                .map(|tail| format!("bound_campaign_{}", tail.split('<').next().unwrap()))
        })
        .unwrap();
    let invariant_strategy = generated_item(&invariant.rust.contents, "pub fn bound_strategy_");
    let invariant_runner = invariant
        .rust
        .contents
        .lines()
        .find_map(|line| {
            line.strip_prefix("pub fn bound_campaign_")
                .filter(|tail| tail.contains("_run<Strategy>"))
                .map(|tail| format!("bound_campaign_{}", tail.split('<').next().unwrap()))
        })
        .unwrap();

    let temporary = TemporaryDirectory::new("quire-bound-strategy-clause-kinds");
    fs::write(temporary.0.join("src/precondition.rs"), pre.rust.contents).unwrap();
    fs::write(
        temporary.0.join("src/invariant.rs"),
        invariant.rust.contents,
    )
    .unwrap();
    fs::write(
        temporary.0.join("src/lib.rs"),
        format!(
            r#"#![deny(missing_docs)]
//! Clause-kind campaign checks.

/// Generated precondition campaign.
pub mod precondition;
/// Generated invariant campaign.
pub mod invariant;

#[cfg(test)]
mod checks {{
    #[test]
    fn both_clause_kinds_run_without_discards() {{
        let mut config = proptest::test_runner::Config::with_cases(10_000);
        config.failure_persistence = None;
        let mut runner = proptest::test_runner::TestRunner::new(config.clone());
        let mut pre_report = quire_contract_runtime::CampaignReport::new(
            quire_contract_runtime::ContractIdentity::new(
                quire_contract_runtime::RequirementId::new("FR-100"),
                quire_contract_runtime::RevisionId::new("3"),
            ),
        );
        let pre_strategy = super::precondition::{pre_strategy}();
        let pre = super::precondition::{pre_runner}(&mut runner, &pre_strategy, &mut pre_report).unwrap();
        assert_eq!(pre.attempted, 10_000);
        assert!(pre.accepted > 0 && pre.rejected > 0);
        assert_eq!(pre.failed, 0);
        assert_eq!(pre.discard_rate(), Some((0, 10_000)));

        let mut runner = proptest::test_runner::TestRunner::new(config);
        let mut invariant_report = quire_contract_runtime::CampaignReport::new(
            quire_contract_runtime::ContractIdentity::new(
                quire_contract_runtime::RequirementId::new("FR-100"),
                quire_contract_runtime::RevisionId::new("3"),
            ),
        );
        let invariant_strategy = super::invariant::{invariant_strategy}();
        let invariant = super::invariant::{invariant_runner}(&mut runner, &invariant_strategy, &mut invariant_report).unwrap();
        assert_eq!(invariant.attempted, 10_000);
        assert_eq!(invariant.accepted, 10_000);
        assert_eq!(invariant.rejected, 0);
        assert!(invariant.failed > 0);
        assert_eq!(invariant.discard_rate(), Some((0, 10_000)));
    }}
}}
"#
        ),
    )
    .unwrap();
    fs::write(
        temporary.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"bound-strategy-clause-kinds\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nproptest = {{ version = \"=1.5.0\", default-features = false, features = [\"std\"] }}\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{}\" }}\n\n[workspace]\n",
            quire_contract_codegen::RUNTIME_REVISION
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO"))
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", temporary.0.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&temporary.0)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated clause-kind consumer failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Trace: TC-022, FR-013-AC-1, FR-013-AC-2, FR-013-AC-3, FR-013-AC-4
#[test]
fn tc_022_bundle_is_typed_rust_with_bound_identity_and_packaged_attestation() {
    let package = decode(&version_projection());
    let generated = generate(BoundStrategyPopulation::Broad);
    assert!(generated.rust.path.ends_with(".rs"));
    assert!(generated.attestation.path.ends_with(".json"));
    assert!(!generated.rust.path.contains("schema"));
    let digest = package.digest().to_string();
    assert!(generated.rust.contents.contains(&digest));
    assert!(generated
        .rust
        .contents
        .contains("test/version-strategy/FR-034@9/VersionUnchanged"));
    let attestation: ProofAttestationBody =
        serde_json::from_str(&generated.attestation.contents).unwrap();
    assert_eq!(
        attestation.proof_id,
        "PROOF-codegen-generated-rust-strategy"
    );
    let argv = &attestation.command.argv;
    let flag = |name| &argv[argv.iter().position(|item| item == name).unwrap() + 1];
    assert_eq!(argv[1], "generate_bound_strategy");
    assert_eq!(flag("--canonical-profile"), BOUND_IDENTITY_PROFILE);
    assert_eq!(flag("--input-digest"), &digest);
    assert_eq!(flag("--requirement"), "FR-034@9");
    assert_eq!(flag("--clause"), "VersionUnchanged");

    let schema = common::packaged_attestation_schema();
    let validator = common::packaged_attestation_validator(&schema);
    let directory = TemporaryDirectory::new("quire-bound-strategy-attestation");
    let sealed = common::seal_and_validate(
        &generated.attestation.contents,
        &generated.rust,
        &directory.0,
        &validator,
    );
    assert_eq!(sealed["proof_id"], "PROOF-codegen-generated-rust-strategy");
}

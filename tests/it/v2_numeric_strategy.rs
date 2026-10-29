//! Public selected-claim V2 numeric source, map, runner, and no-artifact refusals.

use std::{fs, process::Command};

use ix_trace_rs::trace;
use quire_contract_codegen::{
    generate_v2_numeric_strategy, Population, V2NumericErrorCode, V2NumericPolicy,
    V2NumericRequest, V2SelectedClaimKey,
};
use quire_contract_model::{CheckedOccurrence, CheckedOccurrenceRole};

use super::{scratch_crate, v2_integer_comparison_admission::admitted};

fn request<'a>(
    package: &'a quire_contract_model::CheckedPackageV2,
    clause: &'a quire_contract_model::CheckedNodeId,
    claim: &'a CheckedOccurrence,
) -> V2NumericRequest<'a> {
    V2NumericRequest {
        package,
        key: V2SelectedClaimKey {
            clause,
            claim_owner: clause,
            occurrence: claim,
        },
        population: Population::Broad,
        policy: V2NumericPolicy {
            minimum_accepted_cases: 0,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
        },
        process_package_id: None,
    }
}

/// One QSL-authored selected claim yields one complete source/map bundle and executable runners.
#[trace("TC-020")]
#[test]
fn v2_numeric_bundle_keeps_selected_identity_and_runs_both_modes() {
    let (package, clause, claim) = admitted("self.balance >= pre(self.balance)");
    let generated = generate_v2_numeric_strategy(&request(&package, &clause, &claim))
        .expect("selected V2 numeric runner");
    let repeated = generate_v2_numeric_strategy(&request(&package, &clause, &claim))
        .expect("same selected claim");
    assert_eq!(
        generated.identity.requirement,
        format!("qocc2_{}_5_636c61696d_{}", clause.digest, claim.ordinal)
    );
    let package_id = package.package_id();
    let hex = |text: &str| {
        text.as_bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    };
    assert_eq!(
        generated.identity.revision,
        format!(
            "qpkg2_{}_{}_{}_{}_{}_{}",
            package_id.domain.len(),
            hex(&package_id.domain),
            package_id.algorithm.len(),
            hex(&package_id.algorithm),
            package_id.digest.len(),
            hex(&package_id.digest),
        )
    );
    assert_eq!(
        generated.artifacts.artifacts(),
        repeated.artifacts.artifacts()
    );
    assert_eq!(generated.artifacts.artifacts().len(), 2);
    assert_eq!(generated.comparison.clause, clause);
    assert_eq!(generated.comparison.claim, claim);
    assert_eq!(
        generated.comparison,
        package
            .direct_integer_comparison(&clause, &claim)
            .expect("exact selected IR comparison")
    );
    let source = generated
        .artifacts
        .artifacts()
        .iter()
        .find(|artifact| artifact.path.ends_with(".rs"))
        .expect("generated source");
    let map = generated
        .artifacts
        .artifacts()
        .iter()
        .find(|artifact| artifact.path.ends_with(".json"))
        .expect("generated map");
    let parsed: serde_json::Value = serde_json::from_str(&map.contents).expect("source map JSON");
    assert_eq!(parsed["identity"]["clause"]["digest"], &*clause.digest);
    assert_eq!(parsed["identity"]["occurrence"]["role"], "claim");
    assert_eq!(
        parsed["identity"]["packageId"]["digest"],
        &*package.package_id().digest
    );
    assert_eq!(
        parsed["identity"]["requirement"],
        generated.identity.requirement
    );
    assert_eq!(parsed["identity"]["revision"], generated.identity.revision);
    assert_eq!(
        parsed["comparison"]["digest"],
        &*generated.comparison.comparison.digest
    );
    let lines: Vec<_> = source.contents.lines().collect();
    for (field, marker) in [
        (
            "oracleLine",
            "/// Evaluates the selected checked integer comparison.",
        ),
        (
            "populationLine",
            "// Generated V2 numeric population artifact.",
        ),
        (
            "censusLine",
            "/// Domain edge an unrepresentable edge value is adjacent to.",
        ),
        (
            "runnerLine",
            "/// Exact selected V2 requirement and admitted package revision.",
        ),
    ] {
        let line = parsed[field].as_u64().expect("map line number") as usize;
        assert_eq!(lines.get(line - 1), Some(&marker), "{field}");
    }
    assert!(source.contents.contains(&generated.identity.requirement));
    assert!(source.contents.contains(&generated.identity.revision));
    assert!(source
        .contents
        .contains(&generated.identity.execution_point));
    assert!(source.contents.contains(&generated.identity.symbol_suffix));
    assert!(source
        .contents
        .contains("pub fn run_v2_numeric_population("));
    assert!(source.contents.contains("pub fn run_v2_numeric_census("));
    assert!(!source.contents.contains("generate_bound_strategy"));
    assert!(!source.contents.contains("BoundPackage"));
    assert!(!source.contents.contains("ClauseRef"));

    let scratch = scratch_crate::TemporaryDirectory::new("v2-numeric-campaign-generated");
    let manifest = format!(
        "[package]\nname = \"v2-numeric-campaign-generated\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[dependencies]\nproptest = \"=1.5.0\"\n{}\n[workspace]\n",
        scratch_crate::runtime_dependency(&[]),
    );
    scratch_crate::write_manifest(&scratch.path, &manifest);
    let tests = r#"
#[test]
fn population_and_census() {
    use v2_numeric_campaign_generated as generated;
    let config = proptest::test_runner::Config {
        cases: 32, max_global_rejects: 0, failure_persistence: None,
        ..proptest::test_runner::Config::default()
    };
    let mut runner = proptest::test_runner::TestRunner::new_with_rng(
        config,
        proptest::test_runner::TestRng::deterministic_rng(
            proptest::test_runner::RngAlgorithm::ChaCha,
        ),
    );
    let mut report = quire_contract_runtime::CampaignReport::new(generated::V2_NUMERIC_IDENTITY);
    let summary = generated::run_v2_numeric_population(&mut runner, &mut report).unwrap();
    assert_eq!(summary.attempted, 32);
    assert_eq!(summary.discard_rate(), Some((0, 32)));
    assert_eq!(runner.config().max_global_rejects, 0);
    let mut census_report = quire_contract_runtime::CampaignReport::new(generated::V2_NUMERIC_IDENTITY);
    let census = generated::run_v2_numeric_census(&mut census_report).unwrap();
    assert_eq!(census.attempted, 10);
    assert_eq!(census.discard_rate(), Some((0, 10)));
}

/// Every closed direct operator is emitted through the same selected V2 entry.
#[trace("TC-020")]
#[test]
fn v2_numeric_bundle_admits_all_six_authored_comparisons() {
    for (operator, emitted) in [
        ("=", "=="),
        ("!=", "!="),
        ("<", "<"),
        ("<=", "<="),
        (">", ">"),
        (">=", ">="),
    ] {
        let condition = format!("self.balance {operator} pre(self.balance)");
        let (package, clause, claim) = admitted(&condition);
        let generated = generate_v2_numeric_strategy(&request(&package, &clause, &claim))
            .expect("the selected V2 claim produces a complete bundle");
        assert_eq!(generated.artifacts.artifacts().len(), 2);
        let source = &generated
            .artifacts
            .artifacts()
            .iter()
            .find(|artifact| artifact.path.ends_with(".rs"))
            .expect("generated runner source")
            .contents;
        assert!(source.contains(&format!("primary {emitted} other")));
        assert!(!generated.census.in_domain().is_empty());
    }
}
"#;
    fs::write(scratch.path.join("src/lib.rs"), &source.contents).expect("generated source");
    fs::create_dir_all(scratch.path.join("tests")).expect("scratch tests");
    fs::write(scratch.path.join("tests/run.rs"), tests).expect("scratch runner test");
    scratch_crate::resolve_seeded_lock(&scratch.path);
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = Command::new(cargo)
        .args(["test", "--locked", "--offline", "-j2", "--test", "run"])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .current_dir(&scratch.path)
        .output()
        .expect("run generated campaign");
    assert!(
        output.status.success(),
        "generated V2 campaign failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let negated_oracle = source
        .contents
        .replace("Some(primary >= other)", "Some(primary < other)");
    assert_ne!(
        negated_oracle, source.contents,
        "the selected oracle changed"
    );
    fs::write(scratch.path.join("src/lib.rs"), negated_oracle).expect("write oracle mutant");
    let mutant_test = r#"
#[test]
fn oracle_negation_is_a_typed_conformance_failure() {
    use v2_numeric_campaign_generated as generated;
    let config = proptest::test_runner::Config {
        cases: 8, failure_persistence: None,
        ..proptest::test_runner::Config::default()
    };
    let mut runner = proptest::test_runner::TestRunner::new_with_rng(
        config,
        proptest::test_runner::TestRng::deterministic_rng(
            proptest::test_runner::RngAlgorithm::ChaCha,
        ),
    );
    let mut report = quire_contract_runtime::CampaignReport::new(generated::V2_NUMERIC_IDENTITY);
    let result = generated::run_v2_numeric_population(&mut runner, &mut report);
    assert!(matches!(result,
        Err(generated::V2NumericCampaignError::ConformanceMismatch {
            summary, partner: Some(_), ..
        }) if summary.attempted > 0 && summary.discarded == 0
    ));
}
"#;
    fs::write(scratch.path.join("tests/mutant.rs"), mutant_test).expect("write mutant control");
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args(["test", "--locked", "--offline", "-j2", "--test", "mutant"])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .current_dir(&scratch.path)
        .output()
        .expect("run mutated V2 campaign");
    assert!(
        output.status.success(),
        "mutated V2 campaign control failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

/// Refused selections never yield a source, map, or campaign result.
#[trace("TC-020")]
#[test]
fn v2_numeric_refusals_keep_first_typed_cause() {
    let (package, clause, claim) = admitted("self.balance >= pre(self.balance)");
    let wrong_role = CheckedOccurrence {
        role: CheckedOccurrenceRole::Expression,
        ordinal: claim.ordinal,
    };
    let mut wrong = request(&package, &clause, &wrong_role);
    wrong.process_package_id = None;
    let diagnostic = generate_v2_numeric_strategy(&wrong).expect_err("wrong claim role");
    assert_eq!(diagnostic.code, V2NumericErrorCode::MissingClaimOccurrence);
    assert_eq!(diagnostic.clause, clause);
    assert_eq!(diagnostic.occurrence, wrong_role);
    assert!(diagnostic.comparison.is_some());

    let wrong_ordinal = CheckedOccurrence {
        role: CheckedOccurrenceRole::Claim,
        ordinal: claim.ordinal + 1,
    };
    let diagnostic = generate_v2_numeric_strategy(&request(&package, &clause, &wrong_ordinal))
        .expect_err("unowned claim ordinal");
    assert_eq!(diagnostic.code, V2NumericErrorCode::MissingClaimOccurrence);
    assert_eq!(diagnostic.occurrence, wrong_ordinal);

    let (foreign_package, _, _) = admitted("self.balance > pre(self.balance)");
    let mut mismatched_owner = request(&package, &clause, &claim);
    mismatched_owner.process_package_id = Some(foreign_package.package_id());
    let diagnostic = generate_v2_numeric_strategy(&mismatched_owner)
        .expect_err("process owner identity mismatch");
    assert_eq!(diagnostic.code, V2NumericErrorCode::OwnerIdentityMismatch);
    assert!(diagnostic.comparison.is_none());
    assert!(diagnostic.point.is_none());
    assert_eq!(diagnostic.path, "request.package_id");

    let overlapping = V2NumericRequest {
        key: V2SelectedClaimKey {
            clause: &clause,
            claim_owner: &clause,
            occurrence: &wrong_role,
        },
        process_package_id: Some(foreign_package.package_id()),
        ..request(&package, &clause, &claim)
    };
    let diagnostic = generate_v2_numeric_strategy(&overlapping)
        .expect_err("comparison refusal precedes request owner mismatch");
    assert_eq!(diagnostic.code, V2NumericErrorCode::MissingClaimOccurrence);
    assert!(generate_v2_numeric_strategy(&request(&package, &clause, &claim)).is_ok());

    let (other_package, absent_clause, _) = admitted("self.balance < pre(self.balance)");
    assert_ne!(package.package_id(), other_package.package_id());
    let absent = V2NumericRequest {
        key: V2SelectedClaimKey {
            clause: &absent_clause,
            claim_owner: &absent_clause,
            occurrence: &wrong_role,
        },
        ..request(&package, &clause, &claim)
    };
    let diagnostic = generate_v2_numeric_strategy(&absent).expect_err("unknown clause first");
    assert_eq!(diagnostic.code, V2NumericErrorCode::UnknownClause);

    let non_clause = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_tag.as_ref() != "state")
        .expect("non-state node");
    let invalid_kind = V2NumericRequest {
        key: V2SelectedClaimKey {
            clause: &non_clause.node_id,
            claim_owner: &non_clause.node_id,
            occurrence: &wrong_role,
        },
        ..request(&package, &clause, &claim)
    };
    let diagnostic = generate_v2_numeric_strategy(&invalid_kind)
        .expect_err("non-state clause refuses before occurrence");
    assert_eq!(diagnostic.code, V2NumericErrorCode::UnsupportedClauseKind);

    let (empty_package, empty_clause, empty_claim) = admitted("self.balance < 0");
    let diagnostic =
        generate_v2_numeric_strategy(&request(&empty_package, &empty_clause, &empty_claim))
            .expect_err("Broad needs both sides");
    assert_eq!(diagnostic.code, V2NumericErrorCode::EmptyPopulation);
    assert_eq!(diagnostic.clause, empty_clause);
}

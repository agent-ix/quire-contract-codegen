//! Synthetic public executable projections for the complete numeric strategy slice.

mod common;

use std::{
    fmt::Write as _,
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    generate_bound_oracles, generate_bound_strategy, AttestationContext, BoundGenerationError,
    BoundStrategyPopulation, BoundStrategyRequest, GenerationTerminalState, ProofAttestationBody,
    StrategyErrorCode, IR_CANDIDATE_REVISION,
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

fn generate_projection_population(
    value: &Value,
    clause: &ClauseRef,
    population: BoundStrategyPopulation,
    minimum_accepted_cases: u64,
    minimum_rejected_cases: u64,
    maximum_discarded_cases: u64,
) -> quire_contract_codegen::GeneratedArtifactBundle {
    let package = decode(value);
    generate_bound_strategy(&BoundStrategyRequest {
        package: &package,
        clause,
        population,
        minimum_accepted_cases,
        minimum_rejected_cases,
        maximum_discarded_cases,
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
                    .split(['(', '<', ' ', '{', ':'])
                    .next()
                    .unwrap()
                    .to_owned()
            })
        })
        .unwrap_or_else(|| panic!("generated source has no item beginning {prefix:?}"))
}

fn generated_runner(source: &str) -> String {
    source
        .lines()
        .find_map(|line| {
            line.strip_prefix("pub fn bound_campaign_")
                .filter(|tail| tail.contains("_run<Strategy>"))
                .map(|tail| format!("bound_campaign_{}", tail.split('<').next().unwrap()))
        })
        .expect("generated sampled runner")
}

fn generated_census_runner(source: &str) -> String {
    source
        .lines()
        .find_map(|line| {
            line.strip_prefix("pub fn bound_campaign_")
                .filter(|tail| tail.contains("_run_census("))
                .map(|tail| format!("bound_campaign_{}", tail.split('(').next().unwrap()))
        })
        .expect("generated census runner")
}

/// Trace: TC-017, FR-008-AC-1, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-008-AC-5, FR-008-CON-1, FR-008-CON-2
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
    assert!(first
        .rust
        .contents
        .contains("VERSION_4EUMBER_POST_DECLARATION_KIND: &'static str = \"state\""));
    assert!(first
        .rust
        .contents
        .contains("VERSION_4EUMBER_PRE_DECLARATION_KIND: &'static str = \"state\""));

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
    assert_eq!(error.source_span.as_ref().unwrap().start().line(), 4);

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
    assert_eq!(error.source_span.as_ref().unwrap().start().line(), 3);

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
    let oracle_error = match generate_bound_oracles(&arithmetic, context()).unwrap_err() {
        BoundGenerationError::Clause {
            identity,
            mut diagnostics,
        } => {
            assert_eq!(identity, arithmetic_clause);
            assert_eq!(diagnostics.len(), 1);
            diagnostics.remove(0)
        }
        other => panic!("expected clause refusal, got {other:?}"),
    };
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
    assert_eq!(error.generation_code, Some(oracle_error.code));
    assert_eq!(error.terminal_state, oracle_error.terminal_state);
    assert_eq!(
        error.source_span.as_deref(),
        oracle_error.source_span.as_ref()
    );

    let amount_value = scalar_projection("test/integer-healthy", "invariant", "less", 7);
    let amount_clause = ClauseRef::new(
        RequirementRef::parse("test/integer-healthy", "FR-100", 3).unwrap(),
        ClauseId::new("amount-check").unwrap(),
    );
    let amount = generate_projection_population(
        &amount_value,
        &amount_clause,
        BoundStrategyPopulation::Broad,
        1,
        0,
        0,
    );
    assert!(amount.rust.contents.contains("DOMAIN_MINIMUM: i64 = 0"));
    assert!(amount.rust.contents.contains("DOMAIN_MAXIMUM: i64 = 1000"));
    assert!(amount.rust.contents.contains("< 7"));
    assert!(amount
        .rust
        .contents
        .contains("AMOUNT_CURRENT_DECLARATION_KIND: &'static str = \"state\""));
    assert!(amount
        .rust
        .contents
        .contains("AMOUNT_CURRENT_OBSERVATION: &'static str = \"current\""));

    let integer =
        json!({"kind":"integer","domain":"signed","minimum":0,"maximum":1000,"overflow":"reject"});
    let amount_read = || json!({"node":"value_reference","name":"amount","observation":"current","source":span(4)});
    let amount_literal = |value| json!({"node":"integer_literal","value":value,"value_type":integer,"source":span(5)});

    let mut negated_value = amount_value.clone();
    negated_value["bindings"][0]["expression"]["values"][0]["value_type"]["minimum"] = json!(-1000);
    negated_value["bindings"][0]["expression"]["expression"]["right"]["value_type"]["minimum"] =
        json!(-1000);
    negated_value["bindings"][0]["expression"]["expression"]["left"] = json!({
        "node":"numeric_negate", "operand":amount_read(), "source":span(4)
    });
    let mut obligation_value = amount_value.clone();
    let nonzero = json!({
        "node":"compare", "operator":"not_equal", "left":amount_read(),
        "right":amount_literal(0), "source":span(4)
    });
    let division_bound = json!({
        "node":"compare", "operator":"less_equal",
        "left":{"node":"numeric","operator":"divide","left":amount_literal(10),"right":amount_read(),"source":span(4)},
        "right":amount_literal(10), "source":span(4)
    });
    obligation_value["bindings"][0]["expression"]["expression"] = json!({
        "node":"boolean", "operator":"short_circuit_and", "left":nonzero,
        "right":division_bound, "source":span(4)
    });
    for (value, expected) in [
        (
            negated_value,
            quire_contract_codegen::GenerationErrorCode::UnsupportedObligations,
        ),
        (
            obligation_value,
            quire_contract_codegen::GenerationErrorCode::UnsupportedObligations,
        ),
    ] {
        let package = decode(&value);
        let oracle_error = match generate_bound_oracles(&package, context()).unwrap_err() {
            BoundGenerationError::Clause {
                identity,
                mut diagnostics,
            } => {
                assert_eq!(identity, amount_clause);
                assert_eq!(diagnostics.len(), 1);
                diagnostics.remove(0)
            }
            other => panic!("expected clause refusal, got {other:?}"),
        };
        let error = generate_bound_strategy(&BoundStrategyRequest {
            package: &package,
            clause: &amount_clause,
            population: BoundStrategyPopulation::Broad,
            minimum_accepted_cases: 1,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
            attestation: context(),
        })
        .unwrap_err();
        assert_eq!(error.code, StrategyErrorCode::UnsupportedClause);
        assert_eq!(oracle_error.code, expected);
        assert_eq!(error.generation_code, Some(oracle_error.code));
        assert_eq!(error.terminal_state, oracle_error.terminal_state);
        assert_eq!(error.clause.as_deref(), Some(&amount_clause));
        assert_eq!(
            error.source_span.as_deref(),
            oracle_error.source_span.as_ref()
        );
    }

    let comparison = amount_value["bindings"][0]["expression"]["expression"].clone();
    let mut connective_value = amount_value.clone();
    connective_value["bindings"][0]["expression"]["expression"] = json!({
        "node":"boolean", "operator":"total_and", "left":comparison.clone(),
        "right":comparison, "source":span(4)
    });
    let mut literal_only_value = amount_value.clone();
    literal_only_value["bindings"][0]["expression"]["expression"]["left"] = amount_literal(6);
    literal_only_value["package"]["requirements"][0]["clauses"][0]["body"] =
        json!({"node":"composite","children":[]});
    let mut current_pre_value = amount_value.clone();
    let owner = json!({"package":"test/integer-healthy","requirement":"FR-100","revision":3});
    current_pre_value["package"]["requirements"][0]["clauses"][0]["body"] = json!({
        "node":"composite", "children":[
            {"node":"reference","identity":{"requirement":owner,"kind":"state","observation":"current","path":["amount"]}},
            {"node":"reference","identity":{"requirement":owner,"kind":"state","observation":"pre","path":["amount"]}}
        ]
    });
    current_pre_value["bindings"][0]["expression"]["expression"]["right"] = json!({
        "node":"value_reference","name":"amount","observation":"pre","source":span(5)
    });
    let mut boolean_literal_value = amount_value.clone();
    boolean_literal_value["package"]["requirements"][0]["clauses"][0]["body"] =
        json!({"node":"composite","children":[]});
    boolean_literal_value["bindings"][0]["expression"]["expression"] =
        json!({"node":"boolean_literal","value":true,"source":span(4)});
    let mut boolean_reference_value = amount_value.clone();
    boolean_reference_value["bindings"][0]["expression"]["values"][0]["value_type"] =
        json!({"kind":"boolean"});
    let boolean_reference = json!({
        "node":"value_reference", "name":"amount", "observation":"current", "source":span(4)
    });
    boolean_reference_value["bindings"][0]["expression"]["expression"] = boolean_reference.clone();
    let mut boolean_negation_value = boolean_reference_value.clone();
    boolean_negation_value["bindings"][0]["expression"]["expression"] = json!({
        "node":"boolean_not", "operand":boolean_reference, "source":span(4)
    });
    let mut text_comparison_value = amount_value.clone();
    text_comparison_value["bindings"][0]["expression"]["values"][0]["value_type"] =
        json!({"kind":"text"});
    text_comparison_value["bindings"][0]["expression"]["expression"]["right"] =
        json!({"node":"text_literal","value":"stable","source":span(5)});
    for (value, expected_line) in [
        (connective_value.clone(), 4),
        (literal_only_value, 3),
        (current_pre_value, 3),
        (boolean_literal_value, 4),
        (boolean_reference_value, 4),
        (boolean_negation_value, 4),
        (text_comparison_value, 4),
    ] {
        let package = decode(&value);
        let error = generate_bound_strategy(&BoundStrategyRequest {
            package: &package,
            clause: &amount_clause,
            population: BoundStrategyPopulation::Broad,
            minimum_accepted_cases: 1,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
            attestation: context(),
        })
        .unwrap_err();
        assert_eq!(error.code, StrategyErrorCode::UnsupportedRelation);
        assert_eq!(error.clause.as_deref(), Some(&amount_clause));
        assert_eq!(
            error.source_span.as_ref().unwrap().start().line(),
            expected_line
        );
    }

    for kind in ["assertion", "case"] {
        let mut value = amount_value.clone();
        value["package"]["requirements"][0]["clauses"][0]["kind"] = json!(kind);
        let package = decode(&value);
        let error = generate_bound_strategy(&BoundStrategyRequest {
            package: &package,
            clause: &amount_clause,
            population: BoundStrategyPopulation::Broad,
            minimum_accepted_cases: 1,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
            attestation: context(),
        })
        .unwrap_err();
        assert_eq!(error.code, StrategyErrorCode::UnsupportedClauseKind);
        assert_eq!(error.terminal_state, GenerationTerminalState::Unsupported);
        assert_eq!(error.clause.as_deref(), Some(&amount_clause));
        assert!(error.source_span.is_none());
    }
    connective_value["package"]["requirements"][0]["clauses"][0]["kind"] = json!("assertion");
    let combined = decode(&connective_value);
    assert_eq!(
        generate_bound_strategy(&BoundStrategyRequest {
            package: &combined,
            clause: &amount_clause,
            population: BoundStrategyPopulation::Broad,
            minimum_accepted_cases: 1,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
            attestation: context(),
        })
        .unwrap_err()
        .code,
        StrategyErrorCode::UnsupportedClauseKind
    );

    let mut assertion_arithmetic_value = arithmetic_value.clone();
    assertion_arithmetic_value["package"]["requirements"][0]["clauses"][0]["kind"] =
        json!("assertion");
    let assertion_arithmetic = decode(&assertion_arithmetic_value);
    assert_eq!(
        generate_bound_strategy(&BoundStrategyRequest {
            package: &assertion_arithmetic,
            clause: &arithmetic_clause,
            population: BoundStrategyPopulation::Broad,
            minimum_accepted_cases: 1,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
            attestation: context(),
        })
        .unwrap_err()
        .code,
        StrategyErrorCode::UnsupportedClause
    );
    let absent_arithmetic = ClauseRef::new(
        arithmetic_clause.requirement().clone(),
        ClauseId::new("missing-arithmetic").unwrap(),
    );
    assert_eq!(
        generate_bound_strategy(&BoundStrategyRequest {
            package: &arithmetic,
            clause: &absent_arithmetic,
            population: BoundStrategyPopulation::Broad,
            minimum_accepted_cases: 1,
            minimum_rejected_cases: 0,
            maximum_discarded_cases: 0,
            attestation: context(),
        })
        .unwrap_err()
        .code,
        StrategyErrorCode::UnknownClause
    );

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

/// Trace: TC-018, FR-009-AC-4
#[test]
fn tc_018_public_generation_refuses_every_population_with_an_empty_required_side() {
    let fixtures = [
        (
            scalar_projection("test/empty-satisfying", "precondition", "less", 0),
            "test/empty-satisfying",
            [
                BoundStrategyPopulation::Satisfying,
                BoundStrategyPopulation::Broad,
            ],
        ),
        (
            scalar_projection("test/empty-violating", "precondition", "greater_equal", 0),
            "test/empty-violating",
            [
                BoundStrategyPopulation::Violating,
                BoundStrategyPopulation::Broad,
            ],
        ),
    ];
    for (value, package_name, populations) in fixtures {
        let package = decode(&value);
        let clause = ClauseRef::new(
            RequirementRef::parse(package_name, "FR-100", 3).unwrap(),
            ClauseId::new("amount-check").unwrap(),
        );
        for population in populations {
            let error = generate_bound_strategy(&BoundStrategyRequest {
                package: &package,
                clause: &clause,
                population,
                minimum_accepted_cases: 0,
                minimum_rejected_cases: 0,
                maximum_discarded_cases: 0,
                attestation: context(),
            })
            .unwrap_err();
            assert_eq!(error.code, StrategyErrorCode::EmptyPopulation);
            assert_eq!(error.clause.as_deref(), Some(&clause));
            assert!(error.source_span.is_none());
        }
    }
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
    let summary_from_snapshot = format!(
        "{}_summary_from_snapshot",
        runner.strip_suffix("_run").expect("runner suffix")
    );
    let case_type = generated_item(&generated.rust.contents, "pub struct BoundCase");
    let out_of_domain = generated_item(&generated.rust.contents, "pub const OUT_OF_DOMAIN_CASES_");
    let oracle = generated_item(&generated.rust.contents, "pub fn oracle_");
    let mut source = generated.rust.contents;
    source = source.replacen(
        &format!("pub fn {oracle}("),
        &format!("fn {oracle}_uninstrumented("),
        1,
    );
    source.push_str(&format!(
        r#"

thread_local! {{
    static CENSUS_OBSERVATIONS: core::cell::RefCell<Vec<(i64, i64)>> = const {{ core::cell::RefCell::new(Vec::new()) }};
}}

/// Instrumented oracle used by this generated-consumer fixture.
pub fn {oracle}(version_4_eumber_pre: i64, version_4_eumber_post: i64) -> bool {{
    CENSUS_OBSERVATIONS.with(|values| values.borrow_mut().push((version_4_eumber_post, version_4_eumber_pre)));
    {oracle}_uninstrumented(version_4_eumber_pre, version_4_eumber_post)
}}
"#
    ));
    source.push_str(&format!(
        r##"

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
        assert_eq!({case_type}::VERSION_4EUMBER_POST_DECLARATION_KIND, "state");
        assert_eq!({case_type}::VERSION_4EUMBER_POST_OBSERVATION, "post");
        assert_eq!({case_type}::VERSION_4EUMBER_PRE_DECLARATION, "versionNumber");
        assert_eq!({case_type}::VERSION_4EUMBER_PRE_DECLARATION_KIND, "state");
        assert_eq!({case_type}::VERSION_4EUMBER_PRE_OBSERVATION, "pre");
        let outside = {out_of_domain}[0];
        assert_eq!(outside.version_4eumber_post, -1);
        assert_eq!(outside.version_4eumber_pre, 0);
    }}

    #[test]
    fn census_is_exact_once_ordered_and_excludes_out_of_domain_cases() {{
        CENSUS_OBSERVATIONS.with(|values| values.borrow_mut().clear());
        let mut report = report();
        let summary = {census_runner}(&mut report).unwrap();
        assert_eq!(summary.attempted, 10);
        assert_eq!(summary.accepted, 10);
        assert_eq!(summary.failed, 6);
        assert_eq!(summary.discarded, 0);
        CENSUS_OBSERVATIONS.with(|values| assert_eq!(
            values.borrow().as_slice(),
            &[(0, 0), (0, 1), (1, 0), (1, 1), (1, 2), (999, 998), (999, 999), (999, 1000), (1000, 999), (1000, 1000)],
        ));
    }}

    #[test]
    fn zero_and_saturated_snapshots_have_no_rate() {{
        let report = report();
        let zero = {summary_from_snapshot}(&report.snapshot());
        assert_eq!(zero.attempted, 0);
        assert_eq!(zero.discard_rate(), None);
        assert_eq!(zero.rejection_rate(), None);

        let encoded = br#"{{"schemaVersion":"runtime.campaign-snapshot/v1","requirement":"FR-034","revision":"9","counterSemantics":"saturating-u64-v1","counts":{{"accepted":18446744073709551615,"rejected":0,"failed":0,"discarded":0}}}}"#;
        let decoded = quire_contract_runtime::decode_campaign_snapshot(encoded).unwrap();
        let saturated = {summary_from_snapshot}(&decoded.snapshot());
        assert_eq!(saturated.attempted, u64::MAX);
        assert_eq!(saturated.discard_rate(), None);
        assert_eq!(saturated.rejection_rate(), None);
    }}
}}
"##
    ));
    let temporary = TemporaryDirectory::new("quire-bound-strategy-consumer");
    fs::write(
        temporary.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"bound-strategy-consumer\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nproptest = {{ version = \"=1.5.0\", default-features = false, features = [\"std\"] }}\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{}\", features = [\"snapshot-json\"] }}\n\n[workspace]\n",
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
    let invariant_value = scalar_projection("test/amount-invariant", "invariant", "less", 7);
    let invariant_clause = ClauseRef::new(
        RequirementRef::parse("test/amount-invariant", "FR-100", 3).unwrap(),
        ClauseId::new("amount-check").unwrap(),
    );
    let version_value = version_projection();
    let version_clause = clause_ref();
    let fixtures = [
        (
            "version",
            &version_value,
            &version_clause,
            "FR-034",
            "9",
            "\n==\n",
            "\n!=\n",
            "FailedPostcondition",
            "version_4_eumber_pre: i64, version_4_eumber_post: i64",
            "version_4_eumber_pre, version_4_eumber_post",
        ),
        (
            "precondition",
            &pre_value,
            &pre_clause,
            "FR-100",
            "3",
            "\n<=\n",
            "\n>\n",
            "RejectedPrecondition",
            "amount_current: i64",
            "amount_current",
        ),
        (
            "invariant",
            &invariant_value,
            &invariant_clause,
            "FR-100",
            "3",
            "\n<\n",
            "\n>=\n",
            "FailedPostcondition",
            "amount_current: i64",
            "amount_current",
        ),
    ];
    let temporary = TemporaryDirectory::new("quire-bound-strategy-mismatch");
    let mut root =
        String::from("#![deny(missing_docs)]\n//! Negated-oracle conformance mismatch checks.\n\n");
    let mut checks = String::from("#[cfg(test)]\nmod mismatch_checks {\n");
    for (
        module,
        value,
        clause,
        requirement,
        revision,
        needle,
        replacement,
        false_kind,
        oracle_parameters,
        oracle_arguments,
    ) in fixtures
    {
        let generated = generate_projection_population(
            value,
            clause,
            BoundStrategyPopulation::Broad,
            1,
            u64::from(module == "precondition"),
            0,
        );
        let strategy = generated_item(&generated.rust.contents, "pub fn bound_strategy_");
        let runner = generated_runner(&generated.rust.contents);
        let error_type = generated_item(&generated.rust.contents, "pub enum BoundCampaignError");
        let expectation_type =
            generated_item(&generated.rust.contents, "pub enum BoundExpectation");
        let oracle = generated_item(&generated.rust.contents, "pub fn oracle_");
        let mut source = generated.rust.contents.replacen(needle, replacement, 1);
        assert_ne!(
            source, generated.rust.contents,
            "{module} oracle operator was not replaced"
        );
        source = source.replacen(
            &format!("pub fn {oracle}("),
            &format!("fn {oracle}_uninstrumented("),
            1,
        );
        source.push_str(&format!(
            r#"

/// Number of calls made to the deliberately negated oracle.
pub static ORACLE_CALLS: core::sync::atomic::AtomicU64 =
    core::sync::atomic::AtomicU64::new(0);

/// Instrumented, deliberately negated oracle used by this fixture.
pub fn {oracle}({oracle_parameters}) -> bool {{
    ORACLE_CALLS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    {oracle}_uninstrumented({oracle_arguments})
}}
"#,
        ));
        fs::write(temporary.0.join(format!("src/{module}.rs")), source).unwrap();
        writeln!(
            root,
            "/// Generated {module} campaign with a deliberately negated oracle.\npub mod {module};"
        )
        .unwrap();
        writeln!(
            checks,
            r#"
    #[test]
    fn {module}_negation_is_a_structured_shrunk_mismatch() {{
        let config = proptest::test_runner::Config {{
            cases: 256,
            max_global_rejects: 0,
            failure_persistence: None,
            ..proptest::test_runner::Config::default()
        }};
        let mut runner = proptest::test_runner::TestRunner::new_with_rng(
            config,
            proptest::test_runner::TestRng::deterministic_rng(
                proptest::test_runner::RngAlgorithm::ChaCha,
            ),
        );
        let strategy = super::{module}::{strategy}();
        let mut report = quire_contract_runtime::CampaignReport::new(
            quire_contract_runtime::ContractIdentity::new(
                quire_contract_runtime::RequirementId::new({requirement:?}),
                quire_contract_runtime::RevisionId::new({revision:?}),
            ),
        );
        match super::{module}::{runner}(&mut runner, &strategy, &mut report).unwrap_err() {{
            super::{module}::{error_type}::ConformanceMismatch {{ summary, primary, partner, expected, observed }} => {{
                assert!(summary.attempted > 1, "shrink replays were not recorded");
                assert_eq!(
                    summary.attempted,
                    super::{module}::ORACLE_CALLS.load(core::sync::atomic::Ordering::Relaxed),
                    "every oracle evaluation, including shrink replays, must be counted exactly once",
                );
                assert!((0..=1000).contains(&primary));
                assert!(partner.map_or(true, |value| (0..=1000).contains(&value)));
                assert!(matches!(
                    (expected, observed),
                    (super::{module}::{expectation_type}::Holds, quire_contract_runtime::VerdictKind::{false_kind})
                        | (super::{module}::{expectation_type}::Violated, quire_contract_runtime::VerdictKind::Passed)
                ));
            }}
            other => panic!("expected conformance mismatch, got {{other:?}}"),
        }}
    }}
"#,
        )
        .unwrap();
    }
    checks.push_str("}\n");
    root.push_str(&checks);
    fs::write(temporary.0.join("src/lib.rs"), root).unwrap();
    fs::write(
        temporary.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"bound-strategy-mismatch\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nproptest = {{ version = \"=1.5.0\", default-features = false, features = [\"std\"] }}\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{}\" }}\n\n[workspace]\n",
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
        "generated mismatch consumer failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Trace: TC-020, FR-011-AC-1, FR-011-AC-4, FR-011-AC-5, NFR-004-AC-1
#[test]
fn tc_020_all_clause_kinds_and_populations_run_without_discards() {
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
    let invariant_value = scalar_projection("test/amount-invariant", "invariant", "less", 7);
    let invariant_clause = ClauseRef::new(
        RequirementRef::parse("test/amount-invariant", "FR-100", 3).unwrap(),
        ClauseId::new("amount-check").unwrap(),
    );
    let version_value = version_projection();
    let version_clause = clause_ref();
    let fixtures = [
        (
            "version",
            &version_value,
            &version_clause,
            "FR-034",
            "9",
            false,
        ),
        ("precondition", &pre_value, &pre_clause, "FR-100", "3", true),
        (
            "invariant",
            &invariant_value,
            &invariant_clause,
            "FR-100",
            "3",
            false,
        ),
    ];
    let populations = [
        ("satisfying", BoundStrategyPopulation::Satisfying),
        ("violating", BoundStrategyPopulation::Violating),
        ("broad", BoundStrategyPopulation::Broad),
    ];
    let temporary = TemporaryDirectory::new("quire-bound-strategy-all-populations");
    let mut root =
        String::from("#![deny(missing_docs)]\n//! Every supported clause kind and population.\n\n");
    let mut checks = String::from("#[cfg(test)]\nmod checks {\n");

    for (fixture, value, clause, requirement, revision, is_precondition) in fixtures {
        for (population_name, population) in populations {
            let (minimum_accepted, minimum_rejected) = if is_precondition {
                match population {
                    BoundStrategyPopulation::Satisfying => (1, 0),
                    BoundStrategyPopulation::Violating => (0, 1),
                    BoundStrategyPopulation::Broad => (1, 1),
                    BoundStrategyPopulation::Boundary => unreachable!(),
                }
            } else {
                (1, 0)
            };
            let maximum_discarded = 0;
            let generated = generate_projection_population(
                value,
                clause,
                population,
                minimum_accepted,
                minimum_rejected,
                maximum_discarded,
            );
            if is_precondition {
                assert!(generated
                    .rust
                    .contents
                    .contains("Verdict::RejectedPrecondition"));
                assert!(generated
                    .rust
                    .contents
                    .contains("AMOUNT_CURRENT_DECLARATION_KIND: &'static str = \"input\""));
                assert!(generated
                    .rust
                    .contents
                    .contains("AMOUNT_CURRENT_OBSERVATION: &'static str = \"current\""));
            }
            if fixture == "invariant" {
                assert!(generated.rust.contents.contains("ClauseKind::Invariant"));
                assert!(generated.rust.contents.contains("FailureKind::Contract"));
            }
            let module = format!("{fixture}_{population_name}");
            fs::write(
                temporary.0.join(format!("src/{module}.rs")),
                &generated.rust.contents,
            )
            .unwrap();
            writeln!(
                root,
                "/// Generated {fixture} {population_name} campaign.\npub mod {module};"
            )
            .unwrap();
            let strategy = generated_item(&generated.rust.contents, "pub fn bound_strategy_");
            let runner = generated_runner(&generated.rust.contents);
            writeln!(
                checks,
                r#"
    #[test]
    fn {module}_runs_256_and_10000_without_rejects_or_discards() {{
        for cases in [256, 10_000] {{
            let config = proptest::test_runner::Config {{
                cases,
                max_global_rejects: 0,
                failure_persistence: None,
                ..proptest::test_runner::Config::default()
            }};
            let mut runner = proptest::test_runner::TestRunner::new_with_rng(
                config,
                proptest::test_runner::TestRng::deterministic_rng(
                    proptest::test_runner::RngAlgorithm::ChaCha,
                ),
            );
            let strategy = super::{module}::{strategy}();
            let mut report = quire_contract_runtime::CampaignReport::new(
                quire_contract_runtime::ContractIdentity::new(
                    quire_contract_runtime::RequirementId::new({requirement:?}),
                    quire_contract_runtime::RevisionId::new({revision:?}),
                ),
            );
            let summary = super::{module}::{runner}(&mut runner, &strategy, &mut report).unwrap();
            assert_eq!(summary.attempted, u64::from(cases));
            assert_eq!(summary.discarded, 0);
            assert_eq!(summary.discard_rate(), Some((0, u64::from(cases))));
            assert_eq!(summary.rejection_rate(), Some((summary.rejected, summary.attempted)));
            {population_assertions}
        }}
    }}
"#,
                population_assertions = if is_precondition {
                    match population {
                        BoundStrategyPopulation::Satisfying => {
                            "assert_eq!(summary.accepted, u64::from(cases)); assert_eq!(summary.rejected, 0); assert_eq!(summary.failed, 0);"
                        }
                        BoundStrategyPopulation::Violating => {
                            "assert_eq!(summary.accepted, 0); assert_eq!(summary.rejected, u64::from(cases)); assert_eq!(summary.failed, 0);"
                        }
                        BoundStrategyPopulation::Broad => {
                            "assert!(summary.accepted > 0); assert!(summary.rejected > 0); assert_eq!(summary.failed, 0);"
                        }
                        BoundStrategyPopulation::Boundary => unreachable!(),
                    }
                } else {
                    match population {
                        BoundStrategyPopulation::Satisfying => {
                            "assert_eq!(summary.accepted, u64::from(cases)); assert_eq!(summary.rejected, 0); assert_eq!(summary.failed, 0);"
                        }
                        BoundStrategyPopulation::Violating => {
                            "assert_eq!(summary.accepted, u64::from(cases)); assert_eq!(summary.rejected, 0); assert_eq!(summary.failed, u64::from(cases));"
                        }
                        BoundStrategyPopulation::Broad => {
                            "assert_eq!(summary.accepted, u64::from(cases)); assert_eq!(summary.rejected, 0); assert!(summary.failed > 0); assert!(summary.failed < u64::from(cases));"
                        }
                        BoundStrategyPopulation::Boundary => unreachable!(),
                    }
                },
            )
            .unwrap();

            if module == "precondition_broad" {
                let census_runner = generated_census_runner(&generated.rust.contents);
                let error_type =
                    generated_item(&generated.rust.contents, "pub enum BoundCampaignError");
                writeln!(
                    checks,
                    r#"
    #[test]
    fn seeded_report_rates_include_prior_accepted_rejected_and_discarded_counts() {{
        let mut report = quire_contract_runtime::CampaignReport::new(
            quire_contract_runtime::ContractIdentity::new(
                quire_contract_runtime::RequirementId::new("FR-100"),
                quire_contract_runtime::RevisionId::new("3"),
            ),
        );
        let prior = super::{module}::{census_runner}(&mut report).unwrap();
        assert_eq!(prior.attempted, 7);
        assert_eq!(prior.accepted, 4);
        assert_eq!(prior.rejected, 3);
        assert_eq!(prior.failed, 0);
        assert_eq!(prior.discarded, 0);
        report.record_discard();
        let config = proptest::test_runner::Config {{
            cases: 256,
            max_global_rejects: 0,
            failure_persistence: None,
            ..proptest::test_runner::Config::default()
        }};
        let mut runner = proptest::test_runner::TestRunner::new_with_rng(
            config,
            proptest::test_runner::TestRng::deterministic_rng(
                proptest::test_runner::RngAlgorithm::ChaCha,
            ),
        );
        let strategy = super::{module}::{strategy}();
        let summary = match super::{module}::{runner}(&mut runner, &strategy, &mut report).unwrap_err() {{
            super::{module}::{error_type}::AboveDiscardCeiling {{ summary }} => summary,
            other => panic!("expected discard-ceiling refusal, got {{other:?}}"),
        }};
        assert_eq!(summary.attempted, 264);
        assert_eq!(summary.accepted, 128);
        assert_eq!(summary.rejected, 135);
        assert_eq!(summary.failed, 0);
        assert_eq!(summary.discard_rate(), Some((1, 264)));
        assert_eq!(summary.rejection_rate(), Some((135, 264)));
    }}

    #[test]
    fn identity_mismatch_is_not_masked_by_a_prior_discard() {{
        let mut report = quire_contract_runtime::CampaignReport::new(
            quire_contract_runtime::ContractIdentity::new(
                quire_contract_runtime::RequirementId::new("FR-WRONG"),
                quire_contract_runtime::RevisionId::new("0"),
            ),
        );
        report.record_discard();
        let config = proptest::test_runner::Config {{
            cases: 1,
            max_global_rejects: 0,
            failure_persistence: None,
            ..proptest::test_runner::Config::default()
        }};
        let mut runner = proptest::test_runner::TestRunner::new_with_rng(
            config,
            proptest::test_runner::TestRng::deterministic_rng(
                proptest::test_runner::RngAlgorithm::ChaCha,
            ),
        );
        let strategy = super::{module}::{strategy}();
        match super::{module}::{runner}(&mut runner, &strategy, &mut report).unwrap_err() {{
            super::{module}::{error_type}::IdentityMismatch {{ summary, .. }} => {{
                assert_eq!(summary.attempted, 1);
                assert_eq!(summary.discarded, 1);
            }}
            other => panic!("expected identity mismatch, got {{other:?}}"),
        }}
    }}
"#,
                )
                .unwrap();
            }
        }
    }
    checks.push_str("}\n");
    root.push_str(&checks);
    fs::write(temporary.0.join("src/lib.rs"), root).unwrap();
    fs::write(
        temporary.0.join("Cargo.toml"),
        format!(
            "[package]\nname = \"bound-strategy-all-populations\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nproptest = {{ version = \"=1.5.0\", default-features = false, features = [\"std\"] }}\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{}\" }}\n\n[workspace]\n",
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

/// Trace: TC-022, FR-013-AC-1, FR-013-AC-2, FR-013-AC-3, FR-013-AC-4, FR-013-AC-5
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

    let mut changed_package_value = version_projection();
    changed_package_value["bindings"][0]["expression"]["values"][0]["value_type"]["maximum"] =
        json!(999);
    let changed_package = decode(&changed_package_value);
    let changed_package_bundle = generate_bound_strategy(&BoundStrategyRequest {
        package: &changed_package,
        clause: &clause_ref(),
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap();
    assert_ne!(changed_package.digest(), package.digest());
    assert_ne!(changed_package_bundle.rust.path, generated.rust.path);
    assert_ne!(
        changed_package_bundle.attestation.path,
        generated.attestation.path
    );
    assert!(changed_package_bundle
        .rust
        .contents
        .contains(&changed_package.digest().to_string()));
    let changed_package_attestation: ProofAttestationBody =
        serde_json::from_str(&changed_package_bundle.attestation.contents).unwrap();
    let changed_package_argv = &changed_package_attestation.command.argv;
    assert_eq!(
        changed_package_argv[changed_package_argv
            .iter()
            .position(|item| item == "--input-digest")
            .unwrap()
            + 1],
        changed_package.digest().to_string()
    );

    let mut two_clause_value = version_projection();
    let mut second_clause = two_clause_value["package"]["requirements"][0]["clauses"][0].clone();
    second_clause["id"] = json!("VersionStillUnchanged");
    two_clause_value["package"]["requirements"][0]["clauses"]
        .as_array_mut()
        .unwrap()
        .push(second_clause);
    let mut second_binding = two_clause_value["bindings"][0].clone();
    second_binding["clause"]["clause"] = json!("VersionStillUnchanged");
    two_clause_value["bindings"]
        .as_array_mut()
        .unwrap()
        .push(second_binding);
    let two_clause_package = decode(&two_clause_value);
    let changed_clause = ClauseRef::new(
        clause_ref().requirement().clone(),
        ClauseId::new("VersionStillUnchanged").unwrap(),
    );
    let first_clause_bundle = generate_bound_strategy(&BoundStrategyRequest {
        package: &two_clause_package,
        clause: &clause_ref(),
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap();
    let changed_clause_bundle = generate_bound_strategy(&BoundStrategyRequest {
        package: &two_clause_package,
        clause: &changed_clause,
        population: BoundStrategyPopulation::Broad,
        minimum_accepted_cases: 1,
        minimum_rejected_cases: 0,
        maximum_discarded_cases: 0,
        attestation: context(),
    })
    .unwrap();
    assert_ne!(
        changed_clause_bundle.rust.path,
        first_clause_bundle.rust.path
    );
    assert_ne!(
        changed_clause_bundle.attestation.path,
        first_clause_bundle.attestation.path
    );
    assert!(changed_clause_bundle
        .rust
        .contents
        .contains("test/version-strategy/FR-034@9/VersionStillUnchanged"));
    let changed_attestation: ProofAttestationBody =
        serde_json::from_str(&changed_clause_bundle.attestation.contents).unwrap();
    let changed_argv = &changed_attestation.command.argv;
    assert_eq!(
        changed_argv[changed_argv
            .iter()
            .position(|item| item == "--input-digest")
            .unwrap()
            + 1],
        two_clause_package.digest().to_string()
    );
    assert_eq!(
        changed_argv[changed_argv
            .iter()
            .position(|item| item == "--clause")
            .unwrap()
            + 1],
        "VersionStillUnchanged"
    );

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

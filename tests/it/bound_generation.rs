//! Explicitly synthetic public executable projections, not a source-language frontend.

use quire_contract_codegen::{
    generate_bound_oracles, BoundGenerationError, BoundOracleGeneration, GenerationErrorCode,
    SourceRegion,
};
use quire_contract_model::{BoundPackage, EXECUTABLE_PROJECTION_FORMAT};
use serde_json::{json, Value};

fn span() -> Value {
    let source = json!({"document":"synthetic-contract", "revision":1});
    json!({"start":{"source":source,"line":1,"column":1,"byte_offset":0},
        "end":{"source":source,"line":1,"column":2,"byte_offset":1}})
}

fn projection(package: &str, executable: usize, info: bool) -> Value {
    let owner = json!({"package":package,"requirement":"FR-001","revision":7});
    let mut clauses = Vec::new();
    let mut bindings = Vec::new();
    for index in 0..executable {
        let id = format!("clause-{index:04}");
        let (kind, anchor) = match index % 3 {
            0 => ("precondition", json!({"kind":"pre","operation":"check"})),
            1 => ("postcondition", json!({"kind":"post","operation":"check"})),
            _ => ("assertion", json!({"kind":"pre","operation":"check"})),
        };
        clauses.push(json!({"id":id,"kind":kind,"anchor":anchor,"source":span(),
            "body":{"node":"literal"}}));
        bindings.push(json!({"clause":{"requirement":owner,"clause":id},
            "expression":{"owner":owner,"types":[],"values":[],"functions":[],
                "expression":{"node":"boolean_literal","value":true,"source":span()},
                "expected_type":{"kind":"boolean"},"execution_point":anchor,
                "clause_root":true}}));
    }
    if info {
        clauses.push(
            json!({"id":"information","kind":"information","source":span(),
            "body":{"node":"literal"}}),
        );
    }
    json!({"format":EXECUTABLE_PROJECTION_FORMAT,
        "package":{"id":package,"schema_version":{"major":1,"minor":1},
            "source":{"document":"synthetic-contract","revision":1},
            "requirements":[{"id":"FR-001","revision":7,"source":span(),"clauses":clauses}]},
        "bindings":bindings})
}

fn decode(value: &Value) -> BoundPackage {
    BoundPackage::from_json_bytes(&serde_json::to_vec(value).unwrap()).unwrap()
}

fn generated(value: &Value) -> quire_contract_codegen::GeneratedBoundOracles {
    match generate_bound_oracles(&decode(value)).unwrap() {
        BoundOracleGeneration::Generated(result) => result,
        other => panic!("expected executable result: {other:?}"),
    }
}

fn state_integer_projection() -> Value {
    let mut value = projection("test/state-scalar", 1, false);
    let point = json!({"kind":"post","operation":"attemptUpdate"});
    let owner = value["bindings"][0]["clause"]["requirement"].clone();
    value["package"]["requirements"][0]["clauses"][0]["kind"] = json!("postcondition");
    value["package"]["requirements"][0]["clauses"][0]["anchor"] = point.clone();
    value["bindings"][0]["expression"]["execution_point"] = point;
    value["bindings"][0]["expression"]["values"] = json!([{
        "name":"versionNumber",
        "kind":"state",
        "value_type":{
            "kind":"integer",
            "domain":"signed",
            "minimum":0,
            "maximum":1000,
            "overflow":"reject"
        },
        "source":span()
    }]);
    value["bindings"][0]["expression"]["expression"] = json!({
        "node":"compare",
        "operator":"equal",
        "left":{
            "node":"value_reference",
            "name":"versionNumber",
            "observation":"post",
            "source":span()
        },
        "right":{
            "node":"value_reference",
            "name":"versionNumber",
            "observation":"pre",
            "source":span()
        },
        "source":span()
    });
    value["package"]["requirements"][0]["clauses"][0]["body"] = json!({
        "node":"composite",
        "children":[
            {"node":"reference","identity":{
                "requirement":owner,
                "kind":"state",
                "observation":"post",
                "path":["versionNumber"]
            }},
            {"node":"reference","identity":{
                "requirement":owner,
                "kind":"state",
                "observation":"pre",
                "path":["versionNumber"]
            }}
        ]
    });
    value
}

/// TC-001
#[test]
fn complete_public_binding_preserves_identity_population_and_derivation() {
    let value = projection("test/bound", 3, true);
    let bound = decode(&value);
    let result = generated(&value);
    assert_eq!(result.clauses().len(), 3);
    assert_eq!(result.informational().len(), 1);
    assert_eq!(result.bundle().artifacts().len(), 6);
    assert_eq!(result.informational(), bound.informational());
    for (output, clause) in result.clauses().iter().zip(bound.clauses()) {
        assert_eq!(output.identity(), clause.identity());
        let bundle = output.bundle();
        let map: Vec<SourceRegion> = serde_json::from_str(&bundle.source_map.contents).unwrap();
        for region in &map {
            assert_eq!(region.package_id, "test/bound");
            assert_eq!(region.requirement_id, "FR-001");
            assert_eq!(region.requirement_revision, 7);
            assert_eq!(region.clause_id, output.identity().clause().as_str());
        }
        assert_eq!(map[0].expected_consequents, Some(0));
        assert!(map
            .iter()
            .any(|region| region.role == "oracle_evaluation" && region.probe.is_some()));
    }
    let mut reordered = value;
    reordered["bindings"].as_array_mut().unwrap().reverse();
    reordered["package"]["requirements"][0]["clauses"]
        .as_array_mut()
        .unwrap()
        .reverse();
    assert_eq!(result, generated(&reordered));
}

/// TC-002
#[test]
fn public_bound_state_scalar_projection_generates_typed_observation_parameters() {
    let value = state_integer_projection();
    let first = generated(&value);
    let second = generated(&value);
    assert_eq!(first, second);
    let source = &first.clauses()[0].bundle().rust.contents;
    assert!(source.contains("version_4eumber_pre: i64"), "{source}");
    assert!(source.contains("version_4eumber_post: i64"), "{source}");
    assert!(source.contains("\n==\n"), "{source}");
    assert!(!source.contains(": bool"), "{source}");
}

/// TC-001
#[test]
fn empty_and_informational_only_are_explicit_non_artifact_results() {
    for info in [false, true] {
        let mut value = projection("test/no-work", 0, info);
        if !info {
            value["package"]["requirements"] = json!([]);
        }
        let bound = decode(&value);
        let BoundOracleGeneration::NoExecutable(result) = generate_bound_oracles(&bound).unwrap()
        else {
            panic!("must not publish an empty package")
        };
        assert_eq!(result.informational().len(), usize::from(info));
        assert_eq!(result.informational(), bound.informational());
    }
}

/// TC-002
#[test]
fn unsupported_later_clause_fails_whole_batch_with_full_identity() {
    let mut value = projection("test/unsupported", 3, true);
    value["bindings"][2]["expression"]["expression"] = json!({
        "node":"compare","operator":"equal", "source":span(),
        "left":{"node":"boolean_literal","value":true,"source":span()},
        "right":{"node":"boolean_literal","value":false,"source":span()}});
    let bound = decode(&value);
    match generate_bound_oracles(&bound).unwrap_err() {
        BoundGenerationError::Clause {
            identity,
            diagnostics,
        } => {
            assert_eq!(&identity, bound.clauses()[2].identity());
            assert_eq!(
                diagnostics[0].code,
                GenerationErrorCode::UnsupportedExpression
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

/// TC-002
#[test]
fn batch_artifact_count_is_preflighted_before_lowering() {
    let bound = decode(&projection("test/too-many", 2049, false));
    assert_eq!(
        generate_bound_oracles(&bound).unwrap_err(),
        BoundGenerationError::ResourceLimitExceeded
    );
}

/// TC-001
#[test]
fn actual_bound_outputs_publish_then_compile_and_execute_against_the_runtime() {
    use std::{
        fs,
        process::Command,
        time::{SystemTime, UNIX_EPOCH},
    };
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "codegen-bound-native-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir(&root).unwrap();
    let destination = root.join("published");
    let result = generated(&projection("test/native", 3, true));
    assert!(!destination.exists(), "generation must not publish");
    quire_contract_codegen::write_bundle_atomic(result.bundle(), &destination).unwrap();
    for artifact in result.bundle().artifacts() {
        assert_eq!(
            fs::read(destination.join(&artifact.path)).unwrap(),
            artifact.contents.as_bytes()
        );
    }
    let mut main = String::new();
    let mut calls = String::from("fn main() {\n");
    for (index, clause) in result.clauses().iter().enumerate() {
        let rust = &clause.bundle().rust;
        let symbol = rust
            .contents
            .lines()
            .find_map(|line| line.strip_prefix("pub fn "))
            .unwrap()
            .split('(')
            .next()
            .unwrap();
        main.push_str(&format!(
            "#[allow(dead_code)] mod clause_{index} {{ include!({:?}); }}\n",
            destination.join(&rust.path)
        ));
        calls.push_str(&format!("assert!(clause_{index}::{symbol}());\n"));
    }
    calls.push_str("}\n");
    main.push_str(&calls);
    fs::create_dir(root.join("src")).unwrap();
    fs::write(root.join("src/main.rs"), main).unwrap();
    fs::write(root.join("Cargo.toml"),
        "[package]\nname=\"bound-native-control\"\nversion=\"0.0.0\"\nedition=\"2021\"\n\n[dependencies]\nquire-contract-runtime={git=\"https://github.com/agent-ix/quire-contract-runtime\",branch=\"main\"}\n\n[workspace]\n").unwrap();
    let output = Command::new("cargo")
        .args(["run", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", root.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "native bound fixture failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    fs::remove_dir_all(&root).unwrap();
}

//! Explicitly synthetic public executable projections, not a source-language frontend.

use quire_contract_codegen::{
    generate_bound_oracles, AttestationContext, BoundGenerationError, BoundOracleGeneration,
    GenerationErrorCode, ProofAttestationBody, SourceRegion, IR_CANDIDATE_REVISION,
};
use quire_contract_ir::{BoundPackage, BOUND_IDENTITY_PROFILE, EXECUTABLE_PROJECTION_FORMAT};
use serde_json::{json, Value};

fn context() -> AttestationContext<'static> {
    AttestationContext {
        record_digest: "0000000000000000000000000000000000000000000000000000000000000000",
        candidate_revision: IR_CANDIDATE_REVISION,
    }
}

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
    match generate_bound_oracles(&decode(value), context()).unwrap() {
        BoundOracleGeneration::Generated(result) => result,
        other => panic!("expected executable result: {other:?}"),
    }
}

/// TC-001
/// FR-001-AC-6
/// FR-001-AC-7
#[test]
fn complete_public_binding_preserves_identity_population_and_derivation() {
    let value = projection("test/bound", 3, true);
    let bound = decode(&value);
    let result = generated(&value);
    assert_eq!(result.clauses().len(), 3);
    assert_eq!(result.informational().len(), 1);
    assert_eq!(result.bundle().artifacts().len(), 12);
    assert_eq!(result.bound_digest(), bound.digest());
    assert_eq!(result.informational(), bound.informational());
    for (output, clause) in result.clauses().iter().zip(bound.clauses()) {
        assert_eq!(output.identity(), clause.identity());
        assert_eq!(output.expression_digest(), clause.expression_digest());
        assert_eq!(output.declaration_digest(), clause.declaration_digest());
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
        for artifact in [&bundle.rust_attestation, &bundle.source_map_attestation] {
            let body: ProofAttestationBody = serde_json::from_str(&artifact.contents).unwrap();
            let argv = &body.command.argv;
            let flag = |key| &argv[argv.iter().position(|item| item == key).unwrap() + 1];
            assert_eq!(argv[1], "generate_bound_oracles");
            assert_eq!(flag("--input-digest"), &bound.digest().to_string());
            assert_eq!(flag("--canonical-profile"), BOUND_IDENTITY_PROFILE);
            assert_eq!(
                flag("--expression-canonical-digest"),
                &clause.expression_digest().to_string()
            );
        }
    }
    let mut reordered = value;
    reordered["bindings"].as_array_mut().unwrap().reverse();
    reordered["package"]["requirements"][0]["clauses"]
        .as_array_mut()
        .unwrap()
        .reverse();
    assert_eq!(result, generated(&reordered));
}

/// TC-001
/// FR-001-AC-7
#[test]
fn distinct_packages_never_alias_oracle_paths_or_symbols() {
    let first = generated(&projection("test/first", 1, false));
    let second = generated(&projection("test/second", 1, false));
    assert_ne!(first.bound_digest(), second.bound_digest());
    assert_ne!(
        first.clauses()[0].bundle().rust.path,
        second.clauses()[0].bundle().rust.path
    );
    assert_ne!(
        first.clauses()[0].bundle().rust.contents,
        second.clauses()[0].bundle().rust.contents
    );
}

/// TC-001
/// FR-001-AC-6
#[test]
fn empty_and_informational_only_are_explicit_non_artifact_results() {
    for info in [false, true] {
        let mut value = projection("test/no-work", 0, info);
        if !info {
            value["package"]["requirements"] = json!([]);
        }
        let bound = decode(&value);
        let BoundOracleGeneration::NoExecutable(result) =
            generate_bound_oracles(&bound, context()).unwrap()
        else {
            panic!("must not publish an empty package")
        };
        assert_eq!(result.bound_digest(), bound.digest());
        assert_eq!(result.informational().len(), usize::from(info));
        assert_eq!(result.informational(), bound.informational());
    }
}

/// TC-002
/// FR-001-AC-6
#[test]
fn unsupported_later_clause_fails_whole_batch_with_full_identity() {
    let mut value = projection("test/unsupported", 3, true);
    value["bindings"][2]["expression"]["expression"] = json!({
        "node":"compare","operator":"equal", "source":span(),
        "left":{"node":"boolean_literal","value":true,"source":span()},
        "right":{"node":"boolean_literal","value":false,"source":span()}});
    let bound = decode(&value);
    match generate_bound_oracles(&bound, context()).unwrap_err() {
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
/// FR-001-AC-6
#[test]
fn batch_artifact_count_is_preflighted_before_lowering() {
    let bound = decode(&projection("test/too-many", 1025, false));
    assert_eq!(
        generate_bound_oracles(&bound, context()).unwrap_err(),
        BoundGenerationError::ResourceLimitExceeded
    );
}

/// TC-001
/// FR-001-AC-7
#[test]
fn declaration_identity_changes_bind_attestations_even_when_source_is_unchanged() {
    let original = projection("test/declarations", 1, false);
    let mut changed = original.clone();
    changed["bindings"][0]["expression"]["values"] = json!([{
        "name":"unused_input","kind":"input","value_type":{"kind":"boolean"},
        "source":span()}]);
    let first = generated(&original);
    let second = generated(&changed);
    let left = &first.clauses()[0];
    let right = &second.clauses()[0];
    assert_eq!(left.bundle().rust, right.bundle().rust);
    assert_eq!(left.expression_digest(), right.expression_digest());
    assert_ne!(left.declaration_digest(), right.declaration_digest());
    assert_ne!(first.bound_digest(), second.bound_digest());
    assert_ne!(
        left.bundle().rust_attestation,
        right.bundle().rust_attestation
    );
}

/// TC-001
/// FR-001-AC-6
#[test]
fn actual_bound_outputs_publish_then_compile_and_execute_against_pinned_runtime() {
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
    fs::write(root.join("Cargo.toml"), format!(
        "[package]\nname=\"bound-native-control\"\nversion=\"0.0.0\"\nedition=\"2021\"\n\n[dependencies]\nquire-contract-runtime={{git=\"https://github.com/agent-ix/quire-contract-runtime\",rev=\"{}\"}}\n\n[workspace]\n",
        quire_contract_codegen::RUNTIME_REVISION)).unwrap();
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

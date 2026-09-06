//! Synthetic projections and exports are explicit; native control below uses real LLVM.
use quire_contract_codegen::{
    analyze_bound_coverage, generate_bound_oracles, ArtifactBytes, AttestationContext,
    BoundCoverageInputs, BoundOracleGeneration, IR_CANDIDATE_REVISION,
};
use quire_contract_ir::{BoundPackage, EXECUTABLE_PROJECTION_FORMAT};
use serde_json::{json, Value};

fn span() -> Value {
    let source = json!({"document":"synthetic-coverage", "revision":1});
    json!({"start":{"source":source,"line":1,"column":1,"byte_offset":0},
        "end":{"source":source,"line":1,"column":2,"byte_offset":1}})
}

fn input(name: &str) -> Value {
    json!({"node":"value_reference","name":name,"observation":"current","source":span()})
}

fn op(operator: &str, left: Value, right: Value) -> Value {
    json!({"node":"boolean","operator":operator,"left":left,"right":right,"source":span()})
}

fn expressions() -> Vec<Value> {
    let implication = || op("implication", input("a"), input("b"));
    vec![
        implication(),
        input("a"),
        op(
            "total_and",
            implication(),
            op("implication", input("b"), input("a")),
        ),
        implication(),
        op(
            "implication",
            input("a"),
            op("implication", input("b"), input("a")),
        ),
        implication(),
        // An implication in the antecedent distinguishes left-own-right from pre-order.
        op("implication", implication(), input("a")),
    ]
}

fn projection(package: &str, expressions: &[Value], info: bool) -> Value {
    let owner = json!({"package":package,"requirement":"FR-001","revision":7});
    let anchor = json!({"kind":"pre","operation":"check"});
    let mut clauses = Vec::new();
    let mut bindings = Vec::new();
    for (index, expression) in expressions.iter().enumerate() {
        let id = format!("c{index}");
        let text = expression.to_string();
        let names: Vec<_> = ["a", "b"]
            .into_iter()
            .filter(|name| text.contains(&format!("\"name\":\"{name}\"")))
            .collect();
        let children: Vec<_> = names
            .iter()
            .map(|name| {
                json!({"node":"reference", "identity":{
            "requirement":owner,"kind":"input","path":[name],"observation":"current"}})
            })
            .collect();
        clauses.push(
            json!({"id":id,"kind":"precondition","anchor":anchor,"source":span(),
            "body":{"node":"composite","children":children}}),
        );
        let values: Vec<_> = names
            .iter()
            .map(|name| {
                json!({"name":name,"kind":"input",
            "value_type":{"kind":"boolean"},"source":span()})
            })
            .collect();
        bindings.push(
            json!({"clause":{"requirement":owner,"clause":id},"expression":{
            "owner":owner,"types":[],"values":values,"functions":[],"expression":expression,
            "expected_type":{"kind":"boolean"},"execution_point":anchor,"clause_root":true}}),
        );
    }
    if info {
        clauses.push(
            json!({"id":"info","kind":"information","source":span(),"body":{"node":"literal"}}),
        );
    }
    json!({"format":EXECUTABLE_PROJECTION_FORMAT,"package":{"id":package,
        "schema_version":{"major":1,"minor":1},"source":{"document":"synthetic-coverage","revision":1},
        "requirements":[{"id":"FR-001","revision":7,"source":span(),"clauses":clauses}]},"bindings":bindings})
}

fn generate(value: &Value) -> (BoundPackage, BoundOracleGeneration) {
    let package = BoundPackage::from_json_bytes(&serde_json::to_vec(value).unwrap()).unwrap();
    let generated = generate_bound_oracles(
        &package,
        AttestationContext {
            record_digest: "0000000000000000000000000000000000000000000000000000000000000000",
            candidate_revision: IR_CANDIDATE_REVISION,
        },
    )
    .unwrap();
    (package, generated)
}

fn inventory(generated: &BoundOracleGeneration) -> Vec<(String, Vec<u8>)> {
    match generated {
        BoundOracleGeneration::Generated(g) => g
            .bundle()
            .artifacts()
            .iter()
            .map(|a| (a.path.clone(), a.contents.as_bytes().to_vec()))
            .collect(),
        BoundOracleGeneration::NoExecutable(_) => Vec::new(),
    }
}

fn export(generated: &BoundOracleGeneration) -> Value {
    let mut files = Vec::new();
    if let BoundOracleGeneration::Generated(g) = generated {
        for clause in g.clauses() {
            let map: Vec<quire_contract_codegen::SourceRegion> =
                serde_json::from_str(&clause.bundle().source_map.contents).unwrap();
            let mut segments = Vec::new();
            for region in map {
                if let Some(p) = region.probe {
                    segments.push(json!([p.line, p.start_column, 1, true, true, false]));
                    segments.push(json!([p.line, p.end_column, 0, false, false, false]));
                }
            }
            segments.sort_by_key(|s| (s[0].as_u64().unwrap(), s[1].as_u64().unwrap()));
            files.push(json!({"filename":clause.bundle().rust.path,"segments":segments}));
        }
    }
    json!({"type":"llvm.coverage.json.export","version":"3.0.1",
        "cargo_llvm_cov":{"version":"0.9.0","manifest_path":"/fixture/Cargo.toml"},"data":[{"files":files}]})
}

fn analyze(
    package: &BoundPackage,
    generated: &BoundOracleGeneration,
    artifacts: &[(String, Vec<u8>)],
    coverage: Option<&[u8]>,
) -> Value {
    let borrowed: Vec<_> = artifacts
        .iter()
        .map(|(path, bytes)| ArtifactBytes { path, bytes })
        .collect();
    let report = analyze_bound_coverage(
        package,
        generated,
        BoundCoverageInputs {
            source_root: "/fixture",
            artifacts: &borrowed,
            llvm_export: coverage,
        },
    );
    let value: Value = serde_json::from_slice(&report.to_json_bytes().unwrap()).unwrap();
    validate_schema(&value);
    value
}

fn schema() -> jsonschema::JSONSchema {
    let schema: Value =
        serde_json::from_str(quire_contract_codegen::BOUND_COVERAGE_SCHEMA).unwrap();
    jsonschema::JSONSchema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .compile(&schema)
        .unwrap()
}

fn validate_schema(value: &Value) {
    let validator = schema();
    let errors: Vec<_> = match validator.validate(value) {
        Ok(()) => Vec::new(),
        Err(errors) => errors.map(|e| e.to_string()).collect(),
    };
    assert!(errors.is_empty(), "schema errors {errors:?}: {value}");
}

/// Trace: TC-006, FR-004-AC-4, FR-004-AC-5, FR-004-AC-9
#[test]
fn strict_domain_schema_refuses_qualification_and_erased_identity() {
    let (package, generated) = generate(&projection("coverage/schema", &expressions(), true));
    let bytes = serde_json::to_vec(&export(&generated)).unwrap();
    let report = analyze(&package, &generated, &inventory(&generated), Some(&bytes));
    let validator = schema();
    assert!(validator.is_valid(&report));
    for (pointer, replacement) in [
        ("/provenance", json!("run_qualified")),
        ("/state", json!("passed")),
        ("/population", json!("not_emitted")),
        ("/export_sha256", Value::Null),
        ("/schema_sha256", json!("missing")),
        ("/clauses/0/classification", Value::Null),
        ("/clauses/0/identity/requirement/package", json!("")),
        ("/clauses/0/consequents/0/count", Value::Null),
    ] {
        let mut bad = report.clone();
        *bad.pointer_mut(pointer).unwrap() = replacement;
        assert!(!validator.is_valid(&bad), "schema accepts {pointer}");
    }
    let mut bad = report;
    bad["verified"] = json!(true);
    assert!(!validator.is_valid(&bad));
}

/// Trace: TC-006, FR-004-AC-5, FR-004-AC-7
#[test]
fn resource_profile_and_path_refusals_preserve_no_classifications() {
    let (package, generated) = generate(&projection("coverage/refusals", &expressions(), true));
    let artifacts = inventory(&generated);
    let mut unsupported = export(&generated);
    unsupported["version"] = json!("2.0.1");
    let oversized = vec![b' '; quire_contract_codegen::MAX_COVERAGE_BYTES + 1];
    for bytes in [serde_json::to_vec(&unsupported).unwrap(), oversized] {
        let result = analyze(&package, &generated, &artifacts, Some(&bytes));
        assert_eq!(result["state"], "unsupported");
        assert_eq!(result["population"], "not_emitted");
        assert!(result["clauses"].as_array().unwrap().is_empty());
    }
    let mut aliased = export(&generated);
    let original = aliased["data"][0]["files"][0]["filename"]
        .as_str()
        .unwrap()
        .to_string();
    aliased["data"][0]["files"][0]["filename"] = json!(format!("/fixture//./{original}"));
    assert_eq!(
        analyze(
            &package,
            &generated,
            &artifacts,
            Some(&serde_json::to_vec(&aliased).unwrap())
        )["state"],
        "complete"
    );
    let duplicate = aliased["data"][0]["files"][0].clone();
    aliased["data"][0]["files"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    let report = analyze(
        &package,
        &generated,
        &artifacts,
        Some(&serde_json::to_vec(&aliased).unwrap()),
    );
    assert_eq!(report["state"], "invalid_input");
    assert_eq!(report["diagnostics"][0]["code"], "duplicate_file");
    assert!(report["clauses"].as_array().unwrap().is_empty());
    let mut paths = artifacts;
    paths[0].0 = "../foreign".to_owned();
    assert_eq!(
        analyze(&package, &generated, &paths, None)["state"],
        "invalid_input"
    );
}

/// Trace: TC-006, FR-004-AC-1, FR-004-AC-2, FR-004-AC-3, FR-004-AC-7, FR-004-AC-9
#[test]
fn complete_bound_package_is_observed_against_actual_native_llvm() {
    use std::{
        fs,
        path::PathBuf,
        process::Command,
        time::{SystemTime, UNIX_EPOCH},
    };
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "bound-coverage-native-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir(&root).unwrap();
    let (package, generated) = generate(&projection("coverage/native", &expressions(), true));
    let BoundOracleGeneration::Generated(g) = &generated else {
        panic!("native expected executable")
    };
    let source_root = root.join("package");
    quire_contract_codegen::write_bundle_atomic(g.bundle(), &source_root).unwrap();
    let mut modules = String::from("#![allow(dead_code)]\n");
    let mut calls = String::from("#[test] fn native_run() {\n");
    for (index, clause) in g.clauses().iter().enumerate() {
        let rust = &clause.bundle().rust;
        let symbol = rust
            .contents
            .lines()
            .find_map(|l| l.strip_prefix("pub fn "))
            .unwrap()
            .split('(')
            .next()
            .unwrap();
        modules.push_str(&format!(
            "#[path = {:?}] mod c{index};\n",
            rust.path.strip_prefix("src/").unwrap()
        ));
        let arguments = match index {
            0 => "std::hint::black_box(false), std::hint::black_box(true)",
            1 => "std::hint::black_box(true)",
            2 | 4 | 6 => "std::hint::black_box(true), std::hint::black_box(false)",
            5 => "std::hint::black_box(true), std::hint::black_box(true)",
            _ => continue,
        };
        let negation = if index == 2 { "!" } else { "" };
        calls.push_str(&format!(
            "assert!({negation}c{index}::{symbol}({arguments}));\n"
        ));
    }
    calls.push_str("}\n");
    modules.push_str(&calls);
    fs::write(source_root.join("src/lib.rs"), modules).unwrap();
    fs::write(source_root.join("Cargo.toml"),format!("[package]\nname=\"bound-coverage-native\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[dependencies]\nquire-contract-runtime={{git=\"https://github.com/agent-ix/quire-contract-runtime\",rev=\"{}\"}}\n[workspace]\n",quire_contract_codegen::RUNTIME_REVISION)).unwrap();
    let compiler = Command::new("rustc")
        .args(["+stable", "--version"])
        .output()
        .unwrap();
    assert!(compiler.status.success());
    assert_eq!(
        String::from_utf8(compiler.stdout).unwrap().trim(),
        "rustc 1.94.1 (e408947bf 2026-03-25)"
    );
    let sysroot = Command::new("rustc")
        .args(["+stable", "--print", "sysroot"])
        .output()
        .unwrap();
    assert!(sysroot.status.success());
    let tools = PathBuf::from(String::from_utf8(sysroot.stdout).unwrap().trim())
        .join("lib/rustlib/x86_64-unknown-linux-gnu/bin");
    for tool in ["llvm-cov", "llvm-profdata"] {
        assert!(tools.join(tool).is_file(), "qualified tool missing");
    }
    let out = Command::new("cargo")
        .args([
            "+stable",
            "llvm-cov",
            "--offline",
            "--json",
            "--output-path",
        ])
        .arg(source_root.join("coverage.json"))
        .current_dir(&source_root)
        .env("CARGO_TARGET_DIR", root.join("target"))
        .env("LLVM_COV", tools.join("llvm-cov"))
        .env("LLVM_PROFDATA", tools.join("llvm-profdata"))
        .env("CARGO_PROFILE_TEST_OPT_LEVEL", "0")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "native LLVM: {}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    // Re-read actual published bytes after the native producer. Digest claims from generation alone do not suffice.
    let artifacts: Vec<_> = g
        .bundle()
        .artifacts()
        .iter()
        .map(|a| (a.path.clone(), fs::read(source_root.join(&a.path)).unwrap()))
        .collect();
    let borrowed: Vec<_> = artifacts
        .iter()
        .map(|(path, bytes)| ArtifactBytes { path, bytes })
        .collect();
    let coverage = fs::read(source_root.join("coverage.json")).unwrap();
    let report = analyze_bound_coverage(
        &package,
        &generated,
        BoundCoverageInputs {
            source_root: source_root.to_str().unwrap(),
            artifacts: &borrowed,
            llvm_export: Some(&coverage),
        },
    );
    let report: Value = serde_json::from_slice(&report.to_json_bytes().unwrap()).unwrap();
    validate_schema(&report);
    assert_eq!(report["state"], "complete", "{report}");
    assert_eq!(report["provenance"], "unqualified");
    assert_eq!(report["informational"].as_array().unwrap().len(), 1);
    assert_eq!(report["clauses"].as_array().unwrap().len(), 7);
    for (clause, classification) in report["clauses"].as_array().unwrap().iter().zip([
        "vacuous",
        "exercised",
        "partially_exercised",
        "unexecuted",
        "partially_exercised",
        "exercised",
        "partially_exercised",
    ]) {
        assert_eq!(clause["classification"], classification, "{clause}");
    }
    assert_eq!(
        report["clauses"][6]["consequents"],
        json!([{"ordinal":0,"count":1},{"ordinal":1,"count":0}])
    );
    fs::remove_dir_all(root).unwrap();
}

/// Trace: TC-006, FR-004-AC-3, FR-004-AC-7, FR-004-AC-9
#[test]
fn complete_population_is_measured_but_never_run_qualified() {
    let (package, generated) = generate(&projection("coverage/first", &expressions(), true));
    let artifacts = inventory(&generated);
    let bytes = serde_json::to_vec(&export(&generated)).unwrap();
    let report = analyze(&package, &generated, &artifacts, Some(&bytes));
    assert_eq!(report["state"], "complete");
    assert_eq!(report["source_root"], "/fixture");
    assert_eq!(report["provenance"], "unqualified");
    assert_eq!(report["informational"].as_array().unwrap().len(), 1);
    assert_eq!(report["clauses"].as_array().unwrap().len(), 7);
    for (clause, expected) in report["clauses"]
        .as_array()
        .unwrap()
        .iter()
        .zip([1, 0, 2, 1, 2, 1, 2])
    {
        assert_eq!(clause["classification"], "exercised");
        assert_eq!(clause["expected_consequents"], expected);
        assert_eq!(
            clause["consequents"].as_array().unwrap().len(),
            expected as usize
        );
    }
    let repeated = analyze(&package, &generated, &artifacts, Some(&bytes));
    assert_eq!(report, repeated);
    let mut reversed = artifacts;
    reversed.reverse();
    assert_eq!(
        report,
        analyze(&package, &generated, &reversed, Some(&bytes))
    );
}

/// Trace: TC-006, FR-004-AC-4, FR-004-AC-5, FR-004-AC-7
#[test]
fn exact_whole_inventory_and_foreign_binding_fail_without_classifications() {
    let value = projection("coverage/first", &expressions(), true);
    let (package, generated) = generate(&value);
    let original = inventory(&generated);
    let bytes = serde_json::to_vec(&export(&generated)).unwrap();
    assert_eq!(
        analyze(&package, &generated, &original, Some(&bytes))["state"],
        "complete"
    );
    let mut cases = vec![Vec::new(), original[1..].to_vec()];
    let mut duplicate = original.clone();
    duplicate.push(original[0].clone());
    cases.push(duplicate);
    let mut source = original.clone();
    source
        .iter_mut()
        .find(|(path, _)| path.starts_with("src/generated/"))
        .unwrap()
        .1
        .push(b' ');
    cases.push(source);
    for role in ["clause", "oracle_evaluation", "implication_consequent"] {
        let mut changed = original.clone();
        let (_, bytes) = changed
            .iter_mut()
            .find(|(path, _)| path.starts_with("source-maps/"))
            .unwrap();
        let mut map: Vec<Value> = serde_json::from_slice(bytes).unwrap();
        map.retain(|r| r["role"] != role);
        *bytes = serde_json::to_vec(&map).unwrap();
        cases.push(changed);
    }
    for artifacts in cases {
        let report = analyze(&package, &generated, &artifacts, Some(&bytes));
        assert_eq!(report["state"], "invalid_input");
        assert!(report["clauses"].as_array().unwrap().is_empty());
    }
    let mut without_info = value;
    without_info["package"]["requirements"][0]["clauses"]
        .as_array_mut()
        .unwrap()
        .pop();
    let (other, _) = generate(&without_info);
    let (foreign, foreign_generation) =
        generate(&projection("coverage/other", &expressions(), true));
    for owner in [&other, &foreign] {
        let report = analyze(owner, &generated, &original, Some(&bytes));
        assert_eq!(report["state"], "invalid_input");
        assert!(report["clauses"].as_array().unwrap().is_empty());
    }
    let report = analyze(
        &package,
        &foreign_generation,
        &inventory(&foreign_generation),
        Some(&serde_json::to_vec(&export(&foreign_generation)).unwrap()),
    );
    assert_eq!(report["state"], "invalid_input");
    assert!(report["clauses"].as_array().unwrap().is_empty());
}

/// Trace: TC-006, FR-004-AC-5, FR-004-AC-9
#[test]
fn missing_observations_are_not_zero_and_foreign_generated_files_refuse() {
    let (package, generated) = generate(&projection("coverage/missing", &expressions(), true));
    let artifacts = inventory(&generated);
    let report = analyze(&package, &generated, &artifacts, None);
    assert_eq!(report["state"], "incomplete");
    assert_eq!(report["clauses"].as_array().unwrap().len(), 7);
    assert!(report["clauses"]
        .as_array()
        .unwrap()
        .iter()
        .all(|c| c["classification"].is_null()));
    let mut coverage = export(&generated);
    coverage["data"][0]["files"][0]["segments"] = json!([]);
    let report = analyze(
        &package,
        &generated,
        &artifacts,
        Some(&serde_json::to_vec(&coverage).unwrap()),
    );
    assert_eq!(report["state"], "incomplete");
    assert!(report["clauses"][0]["classification"].is_null());
    assert_eq!(report["clauses"][1]["classification"], "exercised");
    let mut coverage = export(&generated);
    coverage["data"][0]["files"]
        .as_array_mut()
        .unwrap()
        .push(json!({"filename":"/fixture/src/generated/foreign.rs","segments":[]}));
    let report = analyze(
        &package,
        &generated,
        &artifacts,
        Some(&serde_json::to_vec(&coverage).unwrap()),
    );
    assert_eq!(report["state"], "invalid_input");
    assert!(report["clauses"].as_array().unwrap().is_empty());
}

/// Trace: TC-006, FR-004-AC-9
#[test]
fn informational_only_is_no_executable_not_invalid_or_exercised() {
    for info in [false, true] {
        let mut value = projection("coverage/info", &[], info);
        if !info {
            value["package"]["requirements"] = json!([]);
        }
        let (package, generated) = generate(&value);
        let report = analyze(&package, &generated, &[], None);
        assert_eq!(report["state"], "no_executable");
        assert_eq!(report["provenance"], "unqualified");
        assert_eq!(
            report["informational"].as_array().unwrap().len(),
            usize::from(info)
        );
        assert!(report["clauses"].as_array().unwrap().is_empty());
        let (_, foreign_generation) = generate(&projection("coverage/info", &expressions(), info));
        let report = analyze(
            &package,
            &foreign_generation,
            &inventory(&foreign_generation),
            None,
        );
        assert_eq!(report["state"], "invalid_input");
        assert!(report["clauses"].as_array().unwrap().is_empty());
    }
}

/// Trace: TC-006, FR-004-AC-4, FR-004-AC-5
#[test]
fn normalized_mapping_root_is_retained_and_bounded() {
    let (package, generated) = generate(&projection("coverage/root", &expressions(), false));
    let artifacts = inventory(&generated);
    let borrowed: Vec<_> = artifacts
        .iter()
        .map(|(path, bytes)| ArtifactBytes { path, bytes })
        .collect();
    let coverage = serde_json::to_vec(&export(&generated)).unwrap();
    for (root, state, normalized) in [
        ("/fixture//./".to_owned(), "complete", json!("/fixture")),
        ("relative".to_owned(), "invalid_input", Value::Null),
        ("".to_owned(), "invalid_input", Value::Null),
        ("/".to_owned(), "invalid_input", Value::Null),
        (
            "/fixture/../foreign".to_owned(),
            "invalid_input",
            Value::Null,
        ),
        (format!("/{}", "x".repeat(4096)), "unsupported", Value::Null),
    ] {
        let result = analyze_bound_coverage(
            &package,
            &generated,
            BoundCoverageInputs {
                source_root: &root,
                artifacts: &borrowed,
                llvm_export: Some(&coverage),
            },
        );
        let result: Value = serde_json::from_slice(&result.to_json_bytes().unwrap()).unwrap();
        validate_schema(&result);
        assert_eq!(result["state"], state);
        assert_eq!(result["source_root"], normalized);
        if state != "complete" {
            assert!(result["clauses"].as_array().unwrap().is_empty());
        }
    }
}

/// Trace: TC-006, FR-004-AC-3, FR-004-AC-5
#[test]
fn aggregate_consequent_cannot_outcount_its_loop_free_oracle() {
    let (package, generated) = generate(&projection("coverage/counts", &expressions(), false));
    let artifacts = inventory(&generated);
    let mut coverage = export(&generated);
    assert_eq!(
        analyze(
            &package,
            &generated,
            &artifacts,
            Some(&serde_json::to_vec(&coverage).unwrap())
        )["state"],
        "complete"
    );
    let BoundOracleGeneration::Generated(g) = &generated else {
        unreachable!()
    };
    let map: Vec<quire_contract_codegen::SourceRegion> =
        serde_json::from_str(&g.clauses()[0].bundle().source_map.contents).unwrap();
    let probe = map[2].probe.unwrap();
    let segment = coverage["data"][0]["files"][0]["segments"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|s| s[0] == probe.line && s[1] == probe.start_column)
        .unwrap();
    assert_eq!(segment[2], 1);
    segment[2] = json!(2);
    let report = analyze(
        &package,
        &generated,
        &artifacts,
        Some(&serde_json::to_vec(&coverage).unwrap()),
    );
    assert_eq!(report["state"], "incomplete");
    assert!(report["clauses"][0]["classification"].is_null());
    assert_eq!(report["clauses"][0]["evaluation_count"], 1);
    assert_eq!(report["clauses"][0]["consequents"][0]["count"], 2);
    assert_eq!(
        report["clauses"][0]["diagnostics"][0]["code"],
        "inconsistent_observation"
    );
    assert_eq!(report["clauses"][1]["classification"], "exercised");
}

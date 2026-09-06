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
    vec![implication(), input("a"), op("total_and", implication(),
        op("implication", input("b"), input("a"))), implication(),
        op("implication", input("a"), op("implication", input("b"), input("a"))),
        implication(),
        // An implication in the antecedent distinguishes left-own-right from pre-order.
        op("implication", implication(), input("a"))]
}

fn projection(package: &str, expressions: &[Value], info: bool) -> Value {
    let owner = json!({"package":package,"requirement":"FR-001","revision":7});
    let anchor = json!({"kind":"pre","operation":"check"});
    let mut clauses = Vec::new();
    let mut bindings = Vec::new();
    for (index, expression) in expressions.iter().enumerate() {
        let id = format!("c{index}");
        let text = expression.to_string();
        let names: Vec<_> = ["a", "b"].into_iter()
            .filter(|name| text.contains(&format!("\"name\":\"{name}\""))).collect();
        let children: Vec<_> = names.iter().map(|name| json!({"node":"reference", "identity":{
            "requirement":owner,"kind":"input","path":[name],"observation":"current"}})).collect();
        clauses.push(json!({"id":id,"kind":"precondition","anchor":anchor,"source":span(),
            "body":{"node":"composite","children":children}}));
        let values: Vec<_> = names.iter().map(|name| json!({"name":name,"kind":"input",
            "value_type":{"kind":"boolean"},"source":span()})).collect();
        bindings.push(json!({"clause":{"requirement":owner,"clause":id},"expression":{
            "owner":owner,"types":[],"values":values,"functions":[],"expression":expression,
            "expected_type":{"kind":"boolean"},"execution_point":anchor,"clause_root":true}}));
    }
    if info { clauses.push(json!({"id":"info","kind":"information","source":span(),"body":{"node":"literal"}})); }
    json!({"format":EXECUTABLE_PROJECTION_FORMAT,"package":{"id":package,
        "schema_version":{"major":1,"minor":1},"source":{"document":"synthetic-coverage","revision":1},
        "requirements":[{"id":"FR-001","revision":7,"source":span(),"clauses":clauses}]},"bindings":bindings})
}

fn generate(value: &Value) -> (BoundPackage, BoundOracleGeneration) {
    let package = BoundPackage::from_json_bytes(&serde_json::to_vec(value).unwrap()).unwrap();
    let generated = generate_bound_oracles(&package, AttestationContext {
        record_digest: "0000000000000000000000000000000000000000000000000000000000000000",
        candidate_revision: IR_CANDIDATE_REVISION,
    }).unwrap();
    (package, generated)
}

fn inventory(generated: &BoundOracleGeneration) -> Vec<(String, Vec<u8>)> {
    match generated {
        BoundOracleGeneration::Generated(g) => g.bundle().artifacts().iter()
            .map(|a| (a.path.clone(), a.contents.as_bytes().to_vec())).collect(),
        BoundOracleGeneration::NoExecutable(_) => Vec::new(),
    }
}

fn export(generated: &BoundOracleGeneration) -> Value {
    let mut files = Vec::new();
    if let BoundOracleGeneration::Generated(g) = generated {
        for clause in g.clauses() {
            let map: Vec<quire_contract_codegen::SourceRegion> = serde_json::from_str(&clause.bundle().source_map.contents).unwrap();
            let mut segments = Vec::new();
            for region in map {
                if let Some(p) = region.probe {
                    segments.push(json!([p.line,p.start_column,1,true,true,false]));
                    segments.push(json!([p.line,p.end_column,0,false,false,false]));
                }
            }
            segments.sort_by_key(|s| (s[0].as_u64().unwrap(),s[1].as_u64().unwrap()));
            files.push(json!({"filename":clause.bundle().rust.path,"segments":segments}));
        }
    }
    json!({"type":"llvm.coverage.json.export","version":"3.0.1",
        "cargo_llvm_cov":{"version":"0.9.0","manifest_path":"/fixture/Cargo.toml"},"data":[{"files":files}]})
}

fn analyze(package: &BoundPackage, generated: &BoundOracleGeneration, artifacts: &[(String, Vec<u8>)], coverage: Option<&[u8]>) -> Value {
    let borrowed: Vec<_> = artifacts.iter().map(|(path, bytes)| ArtifactBytes {path, bytes}).collect();
    let report = analyze_bound_coverage(package, generated, BoundCoverageInputs {
        source_root: "/fixture", artifacts: &borrowed, llvm_export: coverage,
    });
    serde_json::from_slice(&report.to_json_bytes().unwrap()).unwrap()
}

/// Trace: TC-006, FR-004-AC-3, FR-004-AC-7, FR-004-AC-9
#[test]
fn complete_population_is_measured_but_never_run_qualified() {
    let (package, generated) = generate(&projection("coverage/first", &expressions(), true));
    let artifacts = inventory(&generated);
    let bytes = serde_json::to_vec(&export(&generated)).unwrap();
    let report = analyze(&package, &generated, &artifacts, Some(&bytes));
    assert_eq!(report["state"], "complete");
    assert_eq!(report["provenance"], "unqualified");
    assert_eq!(report["informational"].as_array().unwrap().len(), 1);
    assert_eq!(report["clauses"].as_array().unwrap().len(), 7);
    for (clause, expected) in report["clauses"].as_array().unwrap().iter().zip([1,0,2,1,2,1,2]) {
        assert_eq!(clause["classification"], "exercised");
        assert_eq!(clause["expected_consequents"], expected);
        assert_eq!(clause["consequents"].as_array().unwrap().len(), expected as usize);
    }
    let repeated = analyze(&package, &generated, &artifacts, Some(&bytes));
    assert_eq!(report, repeated);
    let mut reversed = artifacts; reversed.reverse();
    assert_eq!(report, analyze(&package, &generated, &reversed, Some(&bytes)));
}

/// Trace: TC-006, FR-004-AC-4, FR-004-AC-5, FR-004-AC-7
#[test]
fn exact_whole_inventory_and_foreign_binding_fail_without_classifications() {
    let value = projection("coverage/first", &expressions(), true);
    let (package, generated) = generate(&value);
    let original = inventory(&generated);
    let bytes = serde_json::to_vec(&export(&generated)).unwrap();
    assert_eq!(analyze(&package,&generated,&original,Some(&bytes))["state"],"complete");
    let mut cases = vec![Vec::new(), original[1..].to_vec()];
    let mut duplicate = original.clone(); duplicate.push(original[0].clone()); cases.push(duplicate);
    let mut source = original.clone(); source[0].1.push(b' '); cases.push(source);
    for role in ["clause", "oracle_evaluation", "implication_consequent"] {
        let mut changed = original.clone();
        let (_, bytes) = changed.iter_mut().find(|(path,_)| path.starts_with("source-maps/")).unwrap();
        let mut map: Vec<Value> = serde_json::from_slice(bytes).unwrap();
        map.retain(|r| r["role"] != role); *bytes = serde_json::to_vec(&map).unwrap(); cases.push(changed);
    }
    for artifacts in cases {
        let report = analyze(&package,&generated,&artifacts,Some(&bytes));
        assert_eq!(report["state"],"invalid_input");
        assert!(report["clauses"].as_array().unwrap().is_empty());
    }
    let mut without_info = value;
    without_info["package"]["requirements"][0]["clauses"].as_array_mut().unwrap().pop();
    let (other, _) = generate(&without_info);
    let (foreign, foreign_generation) = generate(&projection("coverage/other", &expressions(), true));
    for owner in [&other, &foreign] {
        let report = analyze(owner,&generated,&original,Some(&bytes));
        assert_eq!(report["state"],"invalid_input");
        assert!(report["clauses"].as_array().unwrap().is_empty());
    }
    let report = analyze(&package,&foreign_generation,&inventory(&foreign_generation),Some(&serde_json::to_vec(&export(&foreign_generation)).unwrap()));
    assert_eq!(report["state"],"invalid_input");
    assert!(report["clauses"].as_array().unwrap().is_empty());
}

/// Trace: TC-006, FR-004-AC-5, FR-004-AC-9
#[test]
fn missing_observations_are_not_zero_and_foreign_generated_files_refuse() {
    let (package, generated) = generate(&projection("coverage/missing", &expressions(), true));
    let artifacts = inventory(&generated);
    let report = analyze(&package,&generated,&artifacts,None);
    assert_eq!(report["state"],"incomplete");
    assert_eq!(report["clauses"].as_array().unwrap().len(),7);
    assert!(report["clauses"].as_array().unwrap().iter().all(|c| c["classification"].is_null()));
    let mut coverage = export(&generated);
    coverage["data"][0]["files"][0]["segments"] = json!([]);
    let report = analyze(&package,&generated,&artifacts,Some(&serde_json::to_vec(&coverage).unwrap()));
    assert_eq!(report["state"],"incomplete");
    assert!(report["clauses"][0]["classification"].is_null());
    assert_eq!(report["clauses"][1]["classification"],"exercised");
    let mut coverage = export(&generated);
    coverage["data"][0]["files"].as_array_mut().unwrap().push(json!({"filename":"/fixture/src/generated/foreign.rs","segments":[]}));
    let report = analyze(&package,&generated,&artifacts,Some(&serde_json::to_vec(&coverage).unwrap()));
    assert_eq!(report["state"],"invalid_input");
    assert!(report["clauses"].as_array().unwrap().is_empty());
}

/// Trace: TC-006, FR-004-AC-9
#[test]
fn informational_only_is_no_executable_not_invalid_or_exercised() {
    let (package, generated) = generate(&projection("coverage/info", &[], true));
    let report = analyze(&package,&generated,&[],None);
    assert_eq!(report["state"],"no_executable");
    assert_eq!(report["provenance"],"unqualified");
    assert_eq!(report["informational"].as_array().unwrap().len(),1);
    assert!(report["clauses"].as_array().unwrap().is_empty());
}

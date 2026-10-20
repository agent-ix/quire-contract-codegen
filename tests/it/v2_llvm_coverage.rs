//! V2 clause population and LLVM probe observations from admitted checked packages.

use ix_trace_rs::trace;
use quire_contract_codegen::{analyze_v2_llvm_coverage, V2ClauseCoverageInput};
use serde_json::{json, Value};

use super::kani_obligations_state_frame::native_twin::Twin;

fn fixture(condition: &str) -> (quire_contract_model::CheckedPackageV2, Vec<V2ClauseCoverageInput<'static>>) {
    let (package, node_id) =
        Twin::with_balance_condition(condition).emitted_package("BalanceNeverDrops");
    let oracle = quire_contract_codegen::generate_v2_clause_oracle(&package, &node_id).unwrap();
    let source = Box::leak(oracle.source.into_bytes().into_boxed_slice());
    let source_map = Box::leak(serde_json::to_vec(&oracle.source_map).unwrap().into_boxed_slice());
    let node_id = Box::leak(Box::new(node_id));
    let segments: Vec<_> = oracle
        .source_map
        .iter()
        .flat_map(|region| {
            let p = region.probe;
            [
                json!([p.line, p.start_column, 0, true, true, false]),
                json!([p.line, p.end_column, 0, false, false, false]),
            ]
        })
        .collect();
    let llvm = json!({"type":"llvm.coverage.json.export",
        "cargo_llvm_cov":{"manifest_path":"/fixture/Cargo.toml"},
        "data":[{"files":[{"filename":"src/lib.rs","segments":segments}]}]});
    let llvm_export = Box::leak(serde_json::to_vec(&llvm).unwrap().into_boxed_slice());
    (
        package,
        vec![V2ClauseCoverageInput {
            node_id,
            source_root: "/fixture",
            source,
            source_map,
            llvm_export: Some(llvm_export),
        }],
    )
}

fn report(
    package: &quire_contract_model::CheckedPackageV2,
    inputs: &[V2ClauseCoverageInput<'_>],
) -> Value {
    serde_json::from_slice(&analyze_v2_llvm_coverage(package, inputs).to_json_bytes().unwrap())
        .unwrap()
}

#[trace("FR-004-AC-16", "FR-004-AC-19", "FR-004-AC-20", "FR-004-AC-21", "FR-004-AC-22")]
#[test]
fn admitted_clause_body_sets_the_population_and_implication_count() {
    let (package, mut inputs) = fixture("self.balance >= 0 implies (self.balance <= 1000 implies self.balance >= pre(self.balance))");
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "complete");
    assert_eq!(row["clauses"][0]["expected_consequents"], 2);
    assert_eq!(row["clauses"][0]["classification"], "unexecuted");

    let mut llvm: Value = serde_json::from_slice(inputs[0].llvm_export.unwrap()).unwrap();
    for entry in llvm["data"][0]["files"][0]["segments"].as_array_mut().unwrap() {
        entry[2] = json!(1);
    }
    inputs[0].llvm_export = Some(Box::leak(serde_json::to_vec(&llvm).unwrap().into_boxed_slice()));
    let row = report(&package, &inputs);
    assert_eq!(row["clauses"][0]["classification"], "exercised");
}

#[trace("FR-004-AC-18", "FR-004-AC-23")]
#[test]
fn changed_map_census_and_missing_probe_refuse_all_classifications() {
    let (package, mut inputs) = fixture("self.balance >= 0 implies self.balance <= 1000");
    let mut map: Value = serde_json::from_slice(inputs[0].source_map).unwrap();
    map[0]["expectedConsequents"] = json!(0);
    inputs[0].source_map = Box::leak(serde_json::to_vec(&map).unwrap().into_boxed_slice());
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "invalid_input");
    assert_eq!(row["clauses"], json!([]));
    assert_eq!(row["diagnostics"][0]["code"], "map_mismatch");
    assert!(row["diagnostics"][0]["message"].as_str().unwrap().contains("BalanceNeverDrops"));

    let (package, mut inputs) = fixture("self.balance >= 0 implies self.balance <= 1000");
    let mut map: Value = serde_json::from_slice(inputs[0].source_map).unwrap();
    map.as_array_mut().unwrap().pop();
    inputs[0].source_map = Box::leak(serde_json::to_vec(&map).unwrap().into_boxed_slice());
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "invalid_input");
    assert_eq!(row["clauses"], json!([]));
}

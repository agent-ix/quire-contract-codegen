//! V2 clause population and LLVM probe observations from admitted checked packages.

use ix_trace_rs::trace;
use quire_contract_codegen::{analyze_v2_llvm_coverage, V2ClauseCoverageInput};
use serde_json::{json, Value};

use super::kani_obligations_state_frame::native_twin::Twin;

fn fixture(
    condition: &str,
) -> (
    quire_contract_model::CheckedPackageV2,
    Vec<V2ClauseCoverageInput<'static>>,
) {
    let (package, node_id) =
        Twin::with_balance_condition(condition).emitted_package("BalanceNeverDrops");
    let oracle = quire_contract_codegen::generate_v2_clause_oracle(&package, &node_id).unwrap();
    let source = Box::leak(oracle.source.into_bytes().into_boxed_slice());
    let source_map = Box::leak(
        serde_json::to_vec(&oracle.source_map)
            .unwrap()
            .into_boxed_slice(),
    );
    let node_id = Box::leak(Box::new(node_id));
    let mut segments: Vec<_> = oracle
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
    segments.sort_by_key(|segment| (segment[0].as_u64().unwrap(), segment[1].as_u64().unwrap()));
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
    serde_json::from_slice(
        &analyze_v2_llvm_coverage(package, inputs)
            .to_json_bytes()
            .unwrap(),
    )
    .unwrap()
}

fn set_counts(input: &mut V2ClauseCoverageInput<'_>, counts: &[u64]) {
    let map: Value = serde_json::from_slice(input.source_map).unwrap();
    assert_eq!(map.as_array().unwrap().len(), counts.len());
    let mut llvm: Value = serde_json::from_slice(input.llvm_export.unwrap()).unwrap();
    let segments = llvm["data"][0]["files"][0]["segments"]
        .as_array_mut()
        .unwrap();
    for (region, count) in map.as_array().unwrap().iter().zip(counts) {
        let probe = &region["probe"];
        let segment = segments
            .iter_mut()
            .find(|segment| segment[0] == probe["line"] && segment[1] == probe["startColumn"])
            .unwrap();
        segment[2] = json!(count);
    }
    input.llvm_export = Some(Box::leak(
        serde_json::to_vec(&llvm).unwrap().into_boxed_slice(),
    ));
}

#[trace(
    "FR-004-AC-16",
    "FR-004-AC-19",
    "FR-004-AC-20",
    "FR-004-AC-21",
    "FR-004-AC-22"
)]
#[test]
fn admitted_clause_body_sets_the_population_and_implication_count() {
    let (package, mut inputs) = fixture("self.balance >= 0 implies (self.balance <= 1000 implies self.balance >= pre(self.balance))");
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "complete");
    assert_eq!(row["clauses"][0]["expected_consequents"], 2);
    assert_eq!(row["clauses"][0]["classification"], "unexecuted");

    set_counts(&mut inputs[0], &[1, 1, 1]);
    let row = report(&package, &inputs);
    assert_eq!(row["clauses"][0]["classification"], "exercised");
}

#[trace("FR-004-AC-19", "FR-004-AC-21", "FR-004-AC-22")]
#[test]
fn entered_clause_classifies_zero_some_and_all_consequent_probes() {
    let (package, mut inputs) = fixture("self.balance >= 0 implies (self.balance <= 1000 implies self.balance >= pre(self.balance))");
    set_counts(&mut inputs[0], &[1, 0, 0]);
    assert_eq!(
        report(&package, &inputs)["clauses"][0]["classification"],
        "vacuous"
    );
    set_counts(&mut inputs[0], &[1, 1, 0]);
    assert_eq!(
        report(&package, &inputs)["clauses"][0]["classification"],
        "partially_exercised"
    );
    set_counts(&mut inputs[0], &[1, 1, 1]);
    assert_eq!(
        report(&package, &inputs)["clauses"][0]["classification"],
        "exercised"
    );

    let (package, mut inputs) = fixture("self.balance >= 0");
    set_counts(&mut inputs[0], &[1]);
    assert_eq!(
        report(&package, &inputs)["clauses"][0]["classification"],
        "exercised"
    );
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
    assert!(row["diagnostics"][0]["message"]
        .as_str()
        .unwrap()
        .contains(&format!("{:?}", inputs[0].node_id)));

    let (package, mut inputs) = fixture("self.balance >= 0 implies self.balance <= 1000");
    let mut map: Value = serde_json::from_slice(inputs[0].source_map).unwrap();
    map.as_array_mut().unwrap().pop();
    inputs[0].source_map = Box::leak(serde_json::to_vec(&map).unwrap().into_boxed_slice());
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "invalid_input");
    assert_eq!(row["clauses"], json!([]));
}

#[trace("FR-004-AC-23")]
#[test]
fn removing_source_map_or_complete_clause_inventory_prevents_success() {
    let (package, _inputs) = fixture("self.balance >= 0 implies self.balance <= 1000");
    let row = report(&package, &[]);
    assert_eq!(row["state"], "invalid_input");
    assert_eq!(row["clauses"], json!([]));
    assert_eq!(row["diagnostics"][0]["code"], "population_mismatch");

    let (package, mut inputs) = fixture("self.balance >= 0 implies self.balance <= 1000");
    inputs[0].source = b"";
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "invalid_input");
    assert_eq!(row["diagnostics"][0]["code"], "artifact_mismatch");

    let (package, mut inputs) = fixture("self.balance >= 0 implies self.balance <= 1000");
    inputs[0].source_map = b"[]";
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "invalid_input");
    assert_eq!(row["diagnostics"][0]["code"], "map_mismatch");
}

#[trace("FR-004-AC-24")]
#[test]
fn malformed_export_and_wrong_source_root_never_invent_a_classification() {
    let (package, mut inputs) = fixture("self.balance >= 0 implies self.balance <= 1000");
    inputs[0].llvm_export = Some(br#"{"type":"llvm.coverage.json.export"}"#);
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "invalid_input");
    assert_eq!(row["clauses"], json!([]));
    assert_eq!(row["diagnostics"][0]["code"], "malformed_export");

    let (package, mut inputs) = fixture("self.balance >= 0 implies self.balance <= 1000");
    inputs[0].source_root = "/elsewhere";
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "invalid_input");
    assert_eq!(row["clauses"], json!([]));
    assert_eq!(row["diagnostics"][0]["code"], "invalid_path");

    let (package, mut inputs) = fixture("self.balance >= 0 implies self.balance <= 1000");
    inputs[0].llvm_export = None;
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "incomplete");
    assert_eq!(row["clauses"][0]["classification"], Value::Null);
    assert_eq!(
        row["clauses"][0]["diagnostics"][0]["code"],
        "unavailable_observation"
    );
}

#[trace("FR-004-AC-16", "FR-004-AC-19", "FR-004-AC-24")]
#[test]
fn admitted_v2_oracle_is_vacuous_under_real_llvm_when_consequent_is_skipped() {
    use std::{fs, path::PathBuf, process::Command};

    let (package, mut inputs) = fixture("self.balance >= 500 implies self.balance <= 1000");
    let scratch = super::scratch_crate::TemporaryDirectory::new("v2-llvm-coverage");
    let root = scratch.path.join("package");
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("tests")).unwrap();
    fs::write(root.join("src/lib.rs"), inputs[0].source).unwrap();
    fs::write(
        root.join("tests/native.rs"),
        "use quire_contract_runtime::exact as rt;\n#[test] fn skips_consequent() {\n    let limits = rt::ScalarLimits { integer_bits: 1000, decimal_digits: 1000, scale_expansion: 1000, text_input_bytes: 1000, text_scalars: 1000, normalized_scalars: 1000, unit_edges: 1000, value_occurrences: 1000, work_units: 1000, result_units: 1000 };\n    let mut meter = rt::Meter::new(limits);\n    let value = rt::Integer::from(1i64);\n    assert!(matches!(v2_coverage_native::oracle_clause(&value, &mut meter), Ok(rt::Outcome::Completed(true))));\n}\n",
    )
    .unwrap();
    super::scratch_crate::write_manifest(
        &root,
        &format!(
            "[package]\nname=\"v2-coverage-native\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[dependencies]\n{}\n[workspace]\n",
            super::scratch_crate::runtime_dependency(&["exact"])
        ),
    );
    let toolchain = format!("+{}", env!("CARGO_PKG_RUST_VERSION"));
    let sysroot = Command::new("rustc")
        .args([&toolchain, "--print", "sysroot"])
        .output()
        .unwrap();
    assert!(sysroot.status.success());
    let tools = PathBuf::from(String::from_utf8(sysroot.stdout).unwrap().trim())
        .join("lib/rustlib/x86_64-unknown-linux-gnu/bin");
    for tool in ["llvm-cov", "llvm-profdata"] {
        assert!(tools.join(tool).is_file());
    }
    let run = Command::new("cargo")
        .args([
            &toolchain,
            "llvm-cov",
            "--offline",
            "--json",
            "--output-path",
        ])
        .arg(root.join("coverage.json"))
        .current_dir(&root)
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .env("LLVM_COV", tools.join("llvm-cov"))
        .env("LLVM_PROFDATA", tools.join("llvm-profdata"))
        .env("CARGO_PROFILE_TEST_OPT_LEVEL", "0")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "native LLVM: {}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let coverage = fs::read(root.join("coverage.json")).unwrap();
    inputs[0].source_root = Box::leak(root.to_str().unwrap().to_owned().into_boxed_str());
    inputs[0].llvm_export = Some(Box::leak(coverage.into_boxed_slice()));
    let row = report(&package, &inputs);
    assert_eq!(row["state"], "complete", "{row}");
    assert_eq!(row["clauses"][0]["evaluation_count"], 1);
    assert_eq!(row["clauses"][0]["consequents"][0]["count"], 0);
    assert_eq!(row["clauses"][0]["classification"], "vacuous");
}

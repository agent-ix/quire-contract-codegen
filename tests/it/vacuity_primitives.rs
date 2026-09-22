use quire_contract_codegen::{
    classify_clause, parse_llvm_coverage, ClauseCoverage, CoverageErrorCode, SourceProbe,
    MAX_COVERAGE_BYTES,
};
use serde_json::{json, Value};

fn export(segments: Value) -> Value {
    json!({"type":"llvm.coverage.json.export", "version":"3.0.1",
        "cargo_llvm_cov":{"version":"0.9.0","manifest_path":"/fixture/Cargo.toml"},
        "data":[{"files":[{"filename":"/fixture/src/generated.rs","segments":segments}]}]})
}

fn probe(line: u32, start_column: u32, end_column: u32) -> SourceProbe {
    SourceProbe {
        line,
        start_column,
        end_column,
    }
}

fn code(value: &Value) -> CoverageErrorCode {
    parse_llvm_coverage(&serde_json::to_vec(value).unwrap(), "/fixture")
        .unwrap_err()
        .code
}

/// Trace: TC-006
#[test]
fn tc_006_complete_span_observation_and_total_measured_partition() {
    let bytes = serde_json::to_vec(&export(json!([
        [1, 1, 3, true, true, false],
        [2, 1, 0, true, true, false],
        [3, 1, 1, true, true, false],
        [4, 1, 0, false, false, false]
    ])))
    .unwrap();
    let coverage = parse_llvm_coverage(&bytes, "/fixture").unwrap();
    assert_eq!(coverage.export_sha256().len(), 64);
    let positive = coverage
        .observe("src/generated.rs", probe(1, 1, 4))
        .unwrap();
    let zero = coverage
        .observe("src/generated.rs", probe(2, 1, 4))
        .unwrap();
    assert_eq!(positive.count(), 3);
    assert_eq!(zero.count(), 0);
    for (evaluation, expected, consequents, classification) in [
        (zero, 0, vec![], ClauseCoverage::Unexecuted),
        (zero, 1, vec![zero], ClauseCoverage::Unexecuted),
        (positive, 0, vec![], ClauseCoverage::Exercised),
        (positive, 1, vec![zero], ClauseCoverage::Vacuous),
        (
            positive,
            2,
            vec![zero, positive],
            ClauseCoverage::PartiallyExercised,
        ),
        (
            positive,
            2,
            vec![positive, positive],
            ClauseCoverage::Exercised,
        ),
    ] {
        assert_eq!(
            classify_clause(evaluation, expected, &consequents).unwrap(),
            classification
        );
    }
    assert_eq!(
        classify_clause(positive, 1, &[]).unwrap_err().code,
        CoverageErrorCode::PopulationMismatch
    );
    assert_eq!(
        classify_clause(zero, 1, &[positive]).unwrap_err().code,
        CoverageErrorCode::InconsistentObservation
    );
    for (path, token) in [
        ("missing.rs", probe(1, 1, 2)),
        ("src/generated.rs", probe(4, 1, 2)),
        ("src/generated.rs", probe(9, 1, 2)),
    ] {
        assert_eq!(
            coverage.observe(path, token).unwrap_err().code,
            CoverageErrorCode::UnavailableObservation
        );
    }
}

/// Trace: TC-006
#[test]
fn tc_006_partial_gap_unterminated_and_noncount_spans_cannot_prove_entry() {
    for segments in [
        json!([[1, 2, 1, true, true, false], [1, 5, 0, false, false, false]]),
        json!([[1, 1, 1, true, true, false], [1, 3, 0, false, false, false]]),
        json!([[1, 1, 1, true, true, true], [1, 5, 0, false, false, false]]),
        json!([
            [1, 1, 1, false, false, false],
            [1, 5, 0, false, false, false]
        ]),
        json!([[1, 1, 1, true, true, false]]),
    ] {
        let bytes = serde_json::to_vec(&export(segments)).unwrap();
        let coverage = parse_llvm_coverage(&bytes, "/fixture").unwrap();
        assert_eq!(
            coverage
                .observe("src/generated.rs", probe(1, 1, 4))
                .unwrap_err()
                .code,
            CoverageErrorCode::UnavailableObservation
        );
    }
}

/// Trace: TC-006
#[test]
fn tc_006_export_shape_version_tuple_order_and_size_refuse_independently() {
    let valid = export(json!([
        [1, 1, 0, true, true, false],
        [2, 1, 0, false, false, false]
    ]));
    for (key, value) in [("type", json!("other")), ("version", json!("2.0.2"))] {
        let mut changed = valid.clone();
        changed[key] = value;
        assert_eq!(code(&changed), CoverageErrorCode::UnsupportedProfile);
    }
    let mut changed = valid.clone();
    changed["cargo_llvm_cov"]["version"] = json!("0.8.0");
    assert_eq!(code(&changed), CoverageErrorCode::UnsupportedProfile);
    for segments in [
        json!([[1, 1, 0, true, true]]),
        json!([[1, 1, 0, true, true, false, false]]),
        json!([[1, 1, -1, true, true, false]]),
        json!([[1, 1, 0, 1, true, false]]),
        json!([[0, 1, 0, true, true, false]]),
        json!([[1, 0, 0, true, true, false]]),
        json!([[2, 1, 0, true, true, false], [1, 1, 0, true, true, false]]),
        json!([[1, 1, 0, true, true, false], [1, 1, 0, true, true, false]]),
    ] {
        assert_eq!(code(&export(segments)), CoverageErrorCode::MalformedExport);
    }
    changed = valid.clone();
    changed["data"][0]["files"][0]
        .as_object_mut()
        .unwrap()
        .remove("segments");
    assert_eq!(code(&changed), CoverageErrorCode::MalformedExport);
    changed = valid;
    changed["data"] = json!([]);
    assert_eq!(code(&changed), CoverageErrorCode::MalformedExport);
    assert_eq!(
        parse_llvm_coverage(&vec![b' '; MAX_COVERAGE_BYTES + 1], "/fixture")
            .unwrap_err()
            .code,
        CoverageErrorCode::ResourceLimitExceeded
    );
}

/// Trace: TC-006
#[test]
fn tc_006_paths_use_root_boundaries_and_normalized_duplicate_detection() {
    let valid = export(json!([
        [1, 1, 1, true, true, false],
        [2, 1, 0, false, false, false]
    ]));
    for filename in [
        "../src/generated.rs",
        "/fixture/a/../src/generated.rs",
        "src\\generated.rs",
    ] {
        let mut changed = valid.clone();
        changed["data"][0]["files"][0]["filename"] = json!(filename);
        assert_eq!(code(&changed), CoverageErrorCode::InvalidPath);
    }
    let mut changed = valid.clone();
    let mut duplicate = changed["data"][0]["files"][0].clone();
    duplicate["filename"] = json!("src/./generated.rs");
    changed["data"][0]["files"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    assert_eq!(code(&changed), CoverageErrorCode::DuplicateFile);
    changed = valid.clone();
    changed["data"][0]["files"][0]["filename"] = json!("/fixture-other/src/generated.rs");
    let coverage = parse_llvm_coverage(&serde_json::to_vec(&changed).unwrap(), "/fixture").unwrap();
    assert_eq!(
        coverage
            .observe("src/generated.rs", probe(1, 1, 2))
            .unwrap_err()
            .code,
        CoverageErrorCode::UnavailableObservation
    );
    changed = valid;
    changed["cargo_llvm_cov"]["manifest_path"] = json!("/other/Cargo.toml");
    assert_eq!(code(&changed), CoverageErrorCode::InvalidPath);
}

/// Trace: TC-006
#[test]
fn tc_006_file_and_segment_census_bounds_apply_across_export() {
    let mut too_many_files = export(json!([]));
    too_many_files["data"][0]["files"] = Value::Array(
        (0..4097)
            .map(|index| json!({"filename":format!("src/{index}.rs"),"segments":[]}))
            .collect(),
    );
    assert_eq!(
        code(&too_many_files),
        CoverageErrorCode::ResourceLimitExceeded
    );
    let too_many_segments = export(Value::Array(
        (1..=250_001)
            .map(|line| json!([line, 1, 0, true, true, false]))
            .collect(),
    ));
    assert_eq!(
        code(&too_many_segments),
        CoverageErrorCode::ResourceLimitExceeded
    );
}

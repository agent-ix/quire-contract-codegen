//! FR-056 (IR-1526): refuse a V2 clause whose operation at its width is a measured
//! not-Kani-tractable row. Host tests only (no Kani launch): the decider seam, the data-loaded
//! table, the refusal through public CG generation, and the corpus scoring.

use std::{
    fs,
    path::{Path, PathBuf},
};

use ix_trace_rs::trace;
use quire_contract_codegen::{
    derive_v2_clause_operation_widths, generate_v2_clause_native_oracle, interval_bits,
    negotiate_kani_obligations, negotiate_kani_obligations_with_options, ClauseFeatures, Decision,
    GenerationOptions, InputBound, KaniObligationError, KaniObligationOutcome,
    KaniObligationRequest, NotTractable, ObligationDisposition, ObligationItem, OperationWidth,
    TractabilityDecider, TractabilityLoadError, TractabilityRow, TractabilityTable,
    UnsupportedObligation, GENERATOR_VERSION, KERNEL_VERSION,
};
use quire_contract_model::{CheckedNodeId, CheckedPackageV2};
use serde::Deserialize;
use serde_json::{json, Value};

use super::kani_obligations_state_frame::native_twin::Twin;
use crate::scratch_crate::TemporaryDirectory;

const MUL: &str = "quire.op.integer.mul";
const ADD: &str = "quire.op.integer.add";

fn op_width(operation_id: &str, bits: u32) -> OperationWidth {
    OperationWidth {
        operation_id: operation_id.to_owned(),
        bits,
    }
}

fn features(operations: Vec<OperationWidth>) -> ClauseFeatures {
    ClauseFeatures::new(operations, Vec::new(), None)
}

/// A row as table-file JSON, current at CG's own generator and kernel versions.
fn row_json(ids: &[&str], max_operand_bits: u32, verdict: &str) -> Value {
    json!({
        "operation_ids": ids,
        "max_operand_bits": max_operand_bits,
        "collection_bound": null,
        "verdict": verdict,
        "wall_seconds": 600,
        "peak_bytes": 17_179_869_184_u64,
        "kani_version": "0.68.0",
        "cbmc_version": "6.11.0",
        "solver_version": "cadical-2.0",
        "generator_version": GENERATOR_VERSION,
        "kernel_version": KERNEL_VERSION,
    })
}

fn write_table(directory: &TemporaryDirectory, name: &str, rows: &[Value]) -> PathBuf {
    let path = directory.path.join(name);
    fs::write(&path, serde_json::to_vec(rows).unwrap()).unwrap();
    path
}

fn table_of(rows: &[Value]) -> TractabilityTable {
    TractabilityTable::from_rows(
        rows.iter()
            .map(|row| serde_json::from_value(row.clone()).unwrap())
            .collect(),
    )
}

fn options(path: &Path) -> GenerationOptions {
    GenerationOptions {
        tractability_table: Some(path.to_owned()),
    }
}

/// A precondition over `other` (`Int[0, other_upper]`) and `offset` (`Int[0, offset_upper]`).
fn package(
    other_upper: i64,
    offset_upper: i64,
    condition: &str,
) -> (CheckedPackageV2, CheckedNodeId) {
    Twin::with_two_parameter_precondition((0, other_upper), (0, offset_upper), condition)
        .emitted_package("TwoParameters")
}

fn negotiate_with(
    items: &[ObligationItem<'_>],
    options: &GenerationOptions,
) -> Result<KaniObligationOutcome, KaniObligationError> {
    negotiate_kani_obligations_with_options(&request(items), options, false)
}

fn request<'a>(items: &'a [ObligationItem<'a>]) -> KaniObligationRequest<'a> {
    KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items,
        subject_path: "crate::request::unrelated",
        unwind: 3,
    }
}

fn item<'a>(
    package: &'a CheckedPackageV2,
    clause: &'a CheckedNodeId,
    claim: &'a quire_contract_codegen::V2ClauseObligationClaim,
) -> ObligationItem<'a> {
    ObligationItem::V2Clause {
        package,
        clause,
        claim,
        dependencies: None,
    }
}

fn disposition(outcome: &KaniObligationOutcome, index: usize) -> &ObligationDisposition {
    &outcome.records()[index].disposition
}

fn harness_count(outcome: &KaniObligationOutcome) -> usize {
    match outcome {
        KaniObligationOutcome::Emitted {
            v2_clause_harnesses,
            ..
        } => v2_clause_harnesses.len(),
        KaniObligationOutcome::Rejected { .. } => panic!("the request is valid"),
    }
}

/// An always-`Run` decider: a second implementation of the one interface.
struct AlwaysRun;

impl TractabilityDecider for AlwaysRun {
    fn decide(&self, _: &ClauseFeatures) -> Decision {
        Decision::Run
    }
}

/// An always-`Refuse` decider, for the scoring mutants.
struct AlwaysRefuse;

impl TractabilityDecider for AlwaysRefuse {
    fn decide(&self, _: &ClauseFeatures) -> Decision {
        Decision::Refuse {
            reason: NotTractable {
                operation_id: MUL.to_owned(),
                width_bits: 0,
            },
            evidence_row: Box::new(
                table_of(&[row_json(&[MUL], 64, "not_tractable")]).rows()[0].clone(),
            ),
        }
    }
}

/// FR-056-AC-1: both the table lookup and a test decider implement the one interface, whose
/// parameter is the feature value alone. Binding the method to a function pointer type fixes the
/// whole signature at compile time.
#[trace("FR-056-AC-1", "TC-070")]
#[test]
fn tc_070_the_decider_interface_takes_the_feature_value_alone() {
    let table = TractabilityTable::empty();
    let through_table: &dyn TractabilityDecider = &table;
    let through_trivial: &dyn TractabilityDecider = &AlwaysRun;
    let features = features(vec![op_width(MUL, 64)]);
    assert_eq!(through_table.decide(&features), Decision::Run);
    assert_eq!(through_trivial.decide(&features), Decision::Run);
    let signature: fn(&TractabilityTable, &ClauseFeatures) -> Decision =
        <TractabilityTable as TractabilityDecider>::decide;
    let _ = signature;
}

/// FR-056-AC-2: the exposed width of every operation of `other * offset + 3 <= 100` at three
/// widths equals the width of the intervals the native oracle rendering computes, and the add
/// after the multiply carries the propagated (product) width, not the declared operand width.
#[trace("FR-056-AC-2", "TC-070")]
#[test]
fn tc_070_the_exposed_width_is_the_propagated_width() {
    for k in [8_u32, 16, 24] {
        let upper = 1_i64 << k;
        let (package, clause) = package(upper, upper, "other * offset + 3 <= 100");
        let widths = derive_v2_clause_operation_widths(&package, &clause).expect("widths");
        let ids: Vec<&str> = widths.iter().map(|(id, _)| id.as_str()).collect();
        assert_eq!(ids, [MUL, ADD, "quire.op.integer.le"], "k = {k}");
        // The independent route: the intervals the rendering selected for each operation.
        let oracle = generate_v2_clause_native_oracle(&package, &clause).expect("native oracle");
        let selection = oracle.native_integer_selection.expect("a selection");
        let rendered: Vec<u32> = selection
            .operations
            .iter()
            .map(|decision| {
                decision
                    .operands
                    .iter()
                    .map(|operand| {
                        interval_bits(
                            operand.lower.parse().unwrap(),
                            operand.upper.parse().unwrap(),
                        )
                    })
                    .max()
                    .unwrap()
            })
            .collect();
        let exposed: Vec<u32> = widths.iter().map(|(_, bits)| *bits).collect();
        assert_eq!(exposed, rendered, "k = {k}");
        // `Int[0, 2^k]` counts as k + 1 bits; the product reaches 2k + 1.
        assert_eq!(exposed[0], k + 1, "multiply, k = {k}");
        assert!(
            exposed[1] > k + 1,
            "the add after the multiply carries the propagated width, not the declared one, k = {k}"
        );
    }
}

/// FR-056-AC-3: the file format, the typed load errors naming the row, the empty table, and a
/// decision changed by data alone.
#[trace("FR-056-AC-3", "TC-070")]
#[test]
fn tc_070_the_table_loads_from_data_and_a_bad_row_fails_the_load() {
    let directory = TemporaryDirectory::new("ir1526-table-load");
    let valid = write_table(
        &directory,
        "valid.json",
        &[
            row_json(&[MUL], 64, "not_tractable"),
            row_json(&[ADD], 8, "tractable"),
        ],
    );
    let table = TractabilityTable::load(&valid).expect("a valid table loads");
    assert_eq!(table.rows().len(), 2);
    let empty = write_table(&directory, "empty.json", &[]);
    assert!(TractabilityTable::load(&empty).unwrap().rows().is_empty());

    let mut unknown_verdict = row_json(&[MUL], 64, "not_tractable");
    unknown_verdict["verdict"] = json!("slow");
    let mut missing_field = row_json(&[MUL], 64, "not_tractable");
    missing_field.as_object_mut().unwrap().remove("peak_bytes");
    let mut missing_nullable = row_json(&[MUL], 64, "not_tractable");
    missing_nullable
        .as_object_mut()
        .unwrap()
        .remove("collection_bound");
    let mut fractional_width = row_json(&[MUL], 64, "not_tractable");
    fractional_width["max_operand_bits"] = json!(64.5);
    for (name, bad) in [
        ("unknown-verdict", unknown_verdict),
        ("missing-field", missing_field),
        ("missing-nullable", missing_nullable),
        ("fractional-width", fractional_width),
    ] {
        // The malformed row is the second one: the error names index 1, never 0.
        let path = write_table(
            &directory,
            &format!("{name}.json"),
            &[row_json(&[ADD], 8, "tractable"), bad],
        );
        match TractabilityTable::load(&path) {
            Err(TractabilityLoadError::MalformedRow {
                path: named, row, ..
            }) => {
                assert_eq!(named, path, "{name}");
                assert_eq!(row, 1, "{name}");
            }
            other => panic!("{name}: expected a malformed-row error, got {other:?}"),
        }
    }
    let missing = directory.path.join("absent.json");
    assert!(matches!(
        TractabilityTable::load(&missing),
        Err(TractabilityLoadError::Unreadable { path, .. }) if path == missing
    ));
    let not_an_array = directory.path.join("object.json");
    fs::write(&not_an_array, "{}").unwrap();
    assert!(matches!(
        TractabilityTable::load(&not_an_array),
        Err(TractabilityLoadError::NotATable { .. })
    ));

    // A row for a fresh operation id changes the decision with no source change.
    let fresh = "quire.op.fresh.operation";
    let clause = features(vec![op_width(fresh, 10)]);
    assert_eq!(table.decide(&clause), Decision::Run);
    let with_fresh = table_of(&[row_json(&[fresh], 10, "not_tractable")]);
    assert!(matches!(
        with_fresh.decide(&clause),
        Decision::Refuse { .. }
    ));
}

/// FR-056-AC-3 (request level): a path that does not load fails the request, naming the path,
/// and generates nothing; an absent option runs every clause.
#[trace("FR-056-AC-3", "TC-070")]
#[test]
fn tc_070_a_named_table_that_does_not_load_fails_the_request() {
    let directory = TemporaryDirectory::new("ir1526-request-load");
    let (package, clause) = package(1 << 8, 1 << 8, "other * offset + 3 <= 100");
    let claim = crate::common::v2_claim::claim_for(&package, &clause);
    let items = [item(&package, &clause, &claim)];

    let missing = directory.path.join("absent.json");
    match negotiate_with(&items, &options(&missing)) {
        Err(KaniObligationError::TractabilityTable(TractabilityLoadError::Unreadable {
            path,
            ..
        })) => assert_eq!(path, missing),
        other => panic!("expected a typed load error, got {other:?}"),
    }
    let mut bad = row_json(&[MUL], 64, "not_tractable");
    bad["verdict"] = json!("slow");
    let malformed = write_table(&directory, "malformed.json", &[bad]);
    match negotiate_with(&items, &options(&malformed)) {
        Err(KaniObligationError::TractabilityTable(TractabilityLoadError::MalformedRow {
            path,
            row,
            ..
        })) => {
            assert_eq!(path, malformed);
            assert_eq!(row, 0);
        }
        other => panic!("expected a typed load error, got {other:?}"),
    }
    let absent = negotiate_with(&items, &GenerationOptions::default()).expect("no table");
    assert_eq!(harness_count(&absent), 1);
}

/// FR-056-AC-4: a clause with a current `not_tractable` row for its operation at or above its
/// width is refused with the typed reason and no harness; a sibling keeps its harness; an
/// operator the V2 path refuses keeps its own reason.
#[trace("FR-056-AC-4", "TC-070")]
#[test]
fn tc_070_a_measured_bad_row_refuses_the_clause_before_generation() {
    let directory = TemporaryDirectory::new("ir1526-refusal");
    let bad = row_json(&[MUL], 64, "not_tractable");
    let path = write_table(&directory, "table.json", std::slice::from_ref(&bad));

    let (wide, wide_clause) = package(1 << 20, 1 << 20, "other * offset + 3 <= 100");
    let wide_claim = crate::common::v2_claim::claim_for(&wide, &wide_clause);
    let (sibling, sibling_clause) = package(1000, 10, "other < 1000 and offset > 0");
    let sibling_claim = crate::common::v2_claim::claim_for(&sibling, &sibling_clause);
    let (division, division_clause) = package(1000, 10, "other mod offset > 0");
    let division_claim = crate::common::v2_claim::claim_for(&division, &division_clause);
    let items = [
        item(&wide, &wide_clause, &wide_claim),
        item(&sibling, &sibling_clause, &sibling_claim),
        item(&division, &division_clause, &division_claim),
    ];
    let outcome = negotiate_with(&items, &options(&path)).expect("negotiates");
    let expected_row: TractabilityRow = serde_json::from_value(bad).unwrap();
    assert_eq!(
        disposition(&outcome, 0),
        &ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::V2ClauseNotTractable {
                node_id: wide_clause.clone(),
                operation_identity: MUL.to_owned(),
                width_bits: 21,
                row: Box::new(expected_row.clone()),
            }
        }
    );
    // The row carries the Kani, CBMC, solver, generator and kernel versions.
    assert_eq!(expected_row.kani_version, "0.68.0");
    assert_eq!(expected_row.cbmc_version, "6.11.0");
    assert_eq!(expected_row.generator_version, GENERATOR_VERSION);
    assert_eq!(expected_row.kernel_version, KERNEL_VERSION);
    assert!(matches!(
        disposition(&outcome, 1),
        ObligationDisposition::Supported { .. }
    ));
    assert!(matches!(
        disposition(&outcome, 2),
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::V2ClauseOperatorNotSupported { .. }
        }
    ));
    assert_eq!(harness_count(&outcome), 1, "only the sibling has a harness");
}

/// FR-056-AC-5 at the decider: the boundary is inclusive, in bits, and per operation.
#[trace("FR-056-AC-5", "TC-070")]
#[test]
fn tc_070_the_boundary_is_inclusive_in_bits_and_per_operation() {
    let table = table_of(&[row_json(&[MUL], 64, "not_tractable")]);
    let refused = |operations: Vec<OperationWidth>| {
        matches!(table.decide(&features(operations)), Decision::Refuse { .. })
    };
    // Int[0, 2^32] is 33 bits, Int[0, 2^64 - 1] is 64, Int[0, 2^64] is 65.
    assert_eq!(interval_bits(0, 1 << 32), 33);
    assert_eq!(interval_bits(0, (1_i128 << 64) - 1), 64);
    assert_eq!(interval_bits(0, 1_i128 << 64), 65);
    assert!(refused(vec![op_width(MUL, 33)]));
    assert!(refused(vec![op_width(MUL, 64)]));
    assert!(!refused(vec![op_width(MUL, 65)]));
    // The row for multiply says nothing of add.
    assert!(!refused(vec![op_width(ADD, 8)]));
    // A clause holding both refuses on the multiply.
    match table.decide(&features(vec![op_width(ADD, 8), op_width(MUL, 64)])) {
        Decision::Refuse { reason, .. } => assert_eq!(reason.operation_id, MUL),
        Decision::Run => panic!("the multiply refuses"),
    }
}

/// FR-056-AC-5 through generation: a row at 21 bits refuses a 21-bit multiply and not a 22-bit
/// one.
#[trace("FR-056-AC-5", "TC-070")]
#[test]
fn tc_070_the_boundary_holds_through_generation() {
    let directory = TemporaryDirectory::new("ir1526-boundary");
    let path = write_table(
        &directory,
        "table.json",
        &[row_json(&[MUL], 21, "not_tractable")],
    );
    // Int[0, 2^20] is 21 bits, Int[0, 2^21 - 1] is 21 bits, Int[0, 2^21] is 22.
    for (upper, refused) in [
        (1_i64 << 20, true),
        ((1_i64 << 21) - 1, true),
        (1_i64 << 21, false),
    ] {
        let (package, clause) = package(upper, upper, "other * offset + 3 <= 100");
        let claim = crate::common::v2_claim::claim_for(&package, &clause);
        let outcome = negotiate_with(&[item(&package, &clause, &claim)], &options(&path)).unwrap();
        let is_refused = matches!(
            disposition(&outcome, 0),
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::V2ClauseNotTractable { .. }
            }
        );
        assert_eq!(is_refused, refused, "upper = {upper}");
    }
}

/// An operation whose complete population the native kernel cannot retain has no propagated
/// interval, so no width: the decider is not asked and the clause proceeds as before (the native
/// route refuses it as `NativeV2ClauseCapability` itself).
#[trace("TC-070")]
#[test]
fn tc_070_a_clause_beyond_native_capacity_has_no_width_and_is_not_refused_here() {
    let directory = TemporaryDirectory::new("ir1526-capacity");
    let path = write_table(
        &directory,
        "table.json",
        &[row_json(&[MUL], 128, "not_tractable")],
    );
    let (package, clause) = package(1 << 40, 1 << 40, "other * offset + 3 <= 100");
    assert!(derive_v2_clause_operation_widths(&package, &clause).is_err());
    let claim = crate::common::v2_claim::claim_for(&package, &clause);
    let outcome = negotiate_with(&[item(&package, &clause, &claim)], &options(&path)).unwrap();
    assert!(matches!(
        disposition(&outcome, 0),
        ObligationDisposition::Supported { .. }
    ));
}

/// FR-056-AC-6: a clause no row qualifies is generated byte-for-byte as with an empty table, and
/// a `tractable` row never refuses.
#[trace("FR-056-AC-6", "TC-070")]
#[test]
fn tc_070_an_unqualified_clause_is_generated_unchanged() {
    let directory = TemporaryDirectory::new("ir1526-unchanged");
    let (package, clause) = package(1 << 8, 1 << 8, "other * offset + 3 <= 100");
    let claim = crate::common::v2_claim::claim_for(&package, &clause);
    let items = [item(&package, &clause, &claim)];
    let baseline = negotiate_with(&items, &GenerationOptions::default()).unwrap();
    assert_eq!(harness_count(&baseline), 1);
    let nonmatching = write_table(
        &directory,
        "nonmatching.json",
        &[row_json(&["quire.op.integer.sub"], 64, "not_tractable")],
    );
    let tractable = write_table(
        &directory,
        "tractable.json",
        &[row_json(&[MUL], 64, "tractable")],
    );
    for path in [nonmatching, tractable] {
        let outcome = negotiate_with(&items, &options(&path)).unwrap();
        assert_eq!(outcome, baseline, "{}", path.display());
    }
    assert_eq!(
        negotiate_kani_obligations(&request(&items)).unwrap(),
        baseline,
        "the table-free entry point is the empty table"
    );
}

/// FR-056-AC-7: a row against another generator or kernel version is ignored; the Kani, CBMC and
/// solver versions do not make a row stale.
#[trace("FR-056-AC-7", "TC-070")]
#[test]
fn tc_070_a_stale_row_is_ignored() {
    let directory = TemporaryDirectory::new("ir1526-stale");
    let (package, clause) = package(1 << 8, 1 << 8, "other * offset + 3 <= 100");
    let claim = crate::common::v2_claim::claim_for(&package, &clause);
    let items = [item(&package, &clause, &claim)];
    let refused = |row: Value| {
        let path = write_table(&directory, "table.json", &[row]);
        let outcome = negotiate_with(&items, &options(&path)).unwrap();
        matches!(
            disposition(&outcome, 0),
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::V2ClauseNotTractable { .. }
            }
        )
    };
    let current = row_json(&[MUL], 64, "not_tractable");
    assert!(refused(current.clone()));
    for field in ["generator_version", "kernel_version"] {
        let mut stale = current.clone();
        stale[field] = json!("0.0.0-stale");
        assert!(!refused(stale), "{field}");
    }
    for field in ["kani_version", "cbmc_version", "solver_version"] {
        let mut other = current.clone();
        other[field] = json!("0.0.0-other");
        assert!(refused(other), "{field}");
    }
}

/// FR-056-AC-8: of several qualifying rows the smallest `max_operand_bits` wins, then file order,
/// then the clause's operation order; unrelated rows do not move the choice.
#[trace("FR-056-AC-8", "TC-070")]
#[test]
fn tc_070_the_reported_row_is_deterministic() {
    let clause = features(vec![op_width(MUL, 20), op_width(ADD, 30)]);
    let reported = |rows: &[Value]| match table_of(rows).decide(&clause) {
        Decision::Refuse {
            reason,
            evidence_row,
        } => (reason.operation_id, evidence_row.max_operand_bits),
        Decision::Run => panic!("a row qualifies"),
    };
    let wide = row_json(&[MUL], 64, "not_tractable");
    let narrow = row_json(&[ADD], 48, "not_tractable");
    let unrelated = row_json(&["quire.op.integer.sub"], 8, "not_tractable");
    let expected = (ADD.to_owned(), 48);
    assert_eq!(reported(&[wide.clone(), narrow.clone()]), expected);
    assert_eq!(reported(&[narrow.clone(), wide.clone()]), expected);
    assert_eq!(
        reported(&[unrelated.clone(), wide.clone(), narrow.clone()]),
        expected
    );
    assert_eq!(
        reported(&[wide.clone(), unrelated, narrow.clone()]),
        expected
    );
    // Equal bits: file order.
    let first = row_json(&[MUL], 50, "not_tractable");
    let second = row_json(&[ADD], 50, "not_tractable");
    assert_eq!(
        reported(&[first.clone(), second.clone()]),
        (MUL.to_owned(), 50)
    );
    assert_eq!(reported(&[second, first]), (ADD.to_owned(), 50));
    // One row naming both operations: the clause's first qualifying operation.
    assert_eq!(
        reported(&[row_json(&[ADD, MUL], 64, "not_tractable")]),
        (MUL.to_owned(), 64)
    );
}

/// FR-056-AC-9: the runner never consults the decider, so a clause the decider lets through that
/// exceeds the wall-clock ceiling ends `Inconclusive` with the timed-out reason, which the
/// unchanged two-phase runner test `a_witness_phase_is_stopped_at_the_unchanged_wall_clock_ceiling`
/// asserts. Here: no non-test source under `src/kani/run/` mentions the decider.
#[trace("FR-056-AC-9", "TC-070")]
#[test]
fn tc_070_the_runner_does_not_consult_the_decider() {
    fn scan(directory: &Path, found: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                scan(&path, found);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                let source = fs::read_to_string(&path).unwrap();
                if !name.contains("test") && source.to_lowercase().contains("tractab") {
                    found.push(path);
                }
            }
        }
    }
    let mut found = Vec::new();
    scan(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/kani/run"),
        &mut found,
    );
    assert!(
        found.is_empty(),
        "the runner mentions the decider: {found:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// The corpus and its scoring (FR-056-AC-10 to AC-12)
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum GroundTruth {
    Verified,
    Falsified,
    TimedOut,
}

#[derive(Debug, Deserialize)]
struct CorpusRow {
    id: String,
    features: ClauseFeatures,
    ground_truth: GroundTruth,
    #[serde(default)]
    wall_seconds: Option<f64>,
    measurement_source: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Score {
    false_refuse: usize,
    false_accept: usize,
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/tractability")
        .join(name)
}

fn load_corpus(path: &Path) -> Vec<CorpusRow> {
    serde_json::from_slice(&fs::read(path).unwrap()).expect("a corpus file")
}

/// The false-refuse and false-accept counts of `decider` over `corpus`.
fn score(decider: &dyn TractabilityDecider, corpus: &[CorpusRow]) -> Score {
    let mut score = Score {
        false_refuse: 0,
        false_accept: 0,
    };
    for row in corpus {
        let refused = matches!(decider.decide(&row.features), Decision::Refuse { .. });
        match (&row.ground_truth, refused) {
            (GroundTruth::Verified | GroundTruth::Falsified, true) => score.false_refuse += 1,
            (GroundTruth::TimedOut, false) => score.false_accept += 1,
            _ => {}
        }
    }
    score
}

fn regressed(score: &Score, baseline: &Score) -> bool {
    score.false_refuse > baseline.false_refuse || score.false_accept > baseline.false_accept
}

/// FR-056-AC-10: every corpus row records its features, a measured ground truth and its source.
#[trace("FR-056-AC-10", "TC-070")]
#[test]
fn tc_070_every_corpus_row_names_its_measurement() {
    let corpus = load_corpus(&fixture_path("corpus.json"));
    assert!(!corpus.is_empty());
    let mut ids = std::collections::BTreeSet::new();
    for row in &corpus {
        assert!(ids.insert(row.id.clone()), "duplicate id {}", row.id);
        assert!(
            !row.measurement_source.trim().is_empty(),
            "{} names no measurement source",
            row.id
        );
        assert!(!row.features.operations.is_empty(), "{}", row.id);
        if matches!(
            row.ground_truth,
            GroundTruth::Verified | GroundTruth::Falsified
        ) {
            assert!(row.wall_seconds.is_some(), "{} was measured", row.id);
        }
    }
}

/// FR-056-AC-11: the scoring test prints both counts and fails only on regression; the two
/// trivial deciders each regress.
#[trace("FR-056-AC-11", "TC-070")]
#[test]
fn tc_070_the_current_decider_scores_no_worse_than_the_baseline() {
    let corpus = load_corpus(&fixture_path("corpus.json"));
    let baseline: Score =
        serde_json::from_slice(&fs::read(fixture_path("baseline.json")).unwrap()).unwrap();
    // CG holds no measured table: the current production decider runs with the empty table.
    let current = score(&TractabilityTable::empty(), &corpus);
    println!(
        "tractability corpus ({} rows): false-refuse {} (baseline {}), false-accept {} (baseline {})",
        corpus.len(),
        current.false_refuse,
        baseline.false_refuse,
        current.false_accept,
        baseline.false_accept,
    );
    assert!(
        !regressed(&current, &baseline),
        "the decider regressed: {current:?} against {baseline:?}"
    );

    // The mutants, against a corpus holding both kinds of ground truth and a zero baseline.
    let synthetic = vec![
        corpus_row("ok", MUL, 20, GroundTruth::Verified),
        corpus_row("slow", MUL, 60, GroundTruth::TimedOut),
    ];
    let zero = Score {
        false_refuse: 0,
        false_accept: 0,
    };
    let table = table_of(&[row_json(&[MUL], 40, "not_tractable")]);
    let table_score = score(&table, &synthetic);
    // The synthetic table refuses widths up to 40, so the 60-bit row is a false accept and the
    // 20-bit row a false refuse: both counts register.
    assert_eq!(
        table_score,
        Score {
            false_refuse: 1,
            false_accept: 1
        }
    );
    assert!(regressed(&table_score, &zero));
    let refuse_all = score(&AlwaysRefuse, &synthetic);
    assert_eq!(refuse_all.false_refuse, 1);
    assert!(
        regressed(&refuse_all, &zero),
        "refuse-everything fails on false-refuse"
    );
    let run_all = score(&AlwaysRun, &synthetic);
    assert_eq!(run_all.false_accept, 1);
    assert!(
        regressed(&run_all, &zero),
        "run-everything fails on false-accept"
    );
    // A count below the baseline passes.
    assert!(!regressed(
        &Score {
            false_refuse: 0,
            false_accept: 0
        },
        &Score {
            false_refuse: 1,
            false_accept: 1
        }
    ));
}

fn corpus_row(id: &str, operation: &str, bits: u32, truth: GroundTruth) -> CorpusRow {
    CorpusRow {
        id: id.to_owned(),
        features: ClauseFeatures::new(
            vec![op_width(operation, bits)],
            vec![InputBound {
                lower: 0,
                upper: 1,
                boolean: false,
            }],
            None,
        ),
        ground_truth: truth,
        wall_seconds: Some(1.0),
        measurement_source: "synthetic".to_owned(),
    }
}

/// FR-056-AC-12: appending a corpus row changes a scored count with no source change, and a
/// second decider is scored against the same file.
#[trace("FR-056-AC-12", "TC-070")]
#[test]
fn tc_070_a_new_corpus_row_and_a_second_decider_need_no_source_change() {
    let directory = TemporaryDirectory::new("ir1526-corpus");
    let mut rows: Vec<Value> =
        serde_json::from_slice(&fs::read(fixture_path("corpus.json")).unwrap()).unwrap();
    let copy = directory.path.join("corpus.json");
    fs::write(&copy, serde_json::to_vec(&rows).unwrap()).unwrap();
    let before = score(&AlwaysRun, &load_corpus(&copy));
    rows.push(json!({
        "id": "appended-timed-out-multiply",
        "features": {
            "operations": [{"operation_id": MUL, "bits": 63}],
            "inputs": [{"lower": 0, "upper": i64::MAX}],
            "collection_bound": null
        },
        "ground_truth": "timed_out",
        "wall_seconds": 600.0,
        "measurement_source": "synthetic row for this test"
    }));
    fs::write(&copy, serde_json::to_vec(&rows).unwrap()).unwrap();
    let after = score(&AlwaysRun, &load_corpus(&copy));
    assert_eq!(after.false_accept, before.false_accept + 1);
    // The table decider scores the same file through the same function.
    let table = table_of(&[row_json(&[MUL], 64, "not_tractable")]);
    assert_eq!(
        score(&table, &load_corpus(&copy)).false_accept,
        before.false_accept,
        "the appended multiply is now refused"
    );
}

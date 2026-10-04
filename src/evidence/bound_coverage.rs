//! Complete bound observations. No execution, authentication, or assurance verdict.
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write};

use quire_contract_model::{BooleanOperator, BoundPackage, ClauseRef, Expression, ExpressionKind};
use serde::Serialize;

use crate::{
    core::artifact::{MAX_ARTIFACTS, MAX_ARTIFACT_BYTES, MAX_BUNDLE_BYTES},
    core::source_map::SourceRegion,
    evidence::vacuity::{
        classify_clause, normalize_path, parse_llvm_coverage, ClauseCoverage, CoverageDiagnostic,
        CoverageErrorCode, LlvmCoverage, MAX_COVERAGE_BYTES,
    },
    oracle::bound_v1::{BoundOracleGeneration, GeneratedBoundOracles},
};

/// Domain observation format, not a native-run result or attestation format.
pub const BOUND_COVERAGE_FORMAT: &str = "codegen.bound-coverage-observations/v1";
/// Strict output schema for the domain observations emitted here.
pub const BOUND_COVERAGE_SCHEMA: &str =
    include_str!("../../schemas/bound-coverage-observations-v1.schema.json");
/// Maximum serialized analysis bytes, including explicit refusal outcomes.
pub const MAX_ANALYSIS_BYTES: usize = 16 * 1024 * 1024;
const MAX_SOURCE_ROOT_BYTES: usize = 4096;

/// Borrowed untrusted artifact bytes. The analyzer compares them with the generation.
pub struct ArtifactBytes<'a> {
    /// Exact bundle-relative path; publication aliases are not alternate identities.
    pub path: &'a str,
    /// Exact bytes supplied for this expected artifact.
    pub bytes: &'a [u8],
}

/// Complete caller inputs, consumed without filesystem reads or subprocess execution.
pub struct BoundCoverageInputs<'a> {
    /// Absolute lexical package root, at most 4096 bytes, using LLVM parser normalization.
    pub source_root: &'a str,
    /// Every generated artifact, including source-generation bodies, exactly once.
    pub artifacts: &'a [ArtifactBytes<'a>],
    /// Exact LLVM export bytes; absence is unavailable, never measured zero.
    pub llvm_export: Option<&'a [u8]>,
}

/// Domain computation status; none of these states authenticates execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundAnalysisState {
    /// Every expected clause has complete measured observations, possibly adverse.
    Complete,
    /// Bound population is valid but some observations are unavailable.
    Incomplete,
    /// A global binding, format, or semantic input check failed.
    InvalidInput,
    /// Profile or resource requirement is outside the supported boundary.
    Unsupported,
    /// Valid empty or informational-only population has no executable work.
    NoExecutable,
}

#[derive(Clone, Debug, Serialize)]
struct ArtifactObservation {
    path: String,
    bytes: usize,
}

#[derive(Clone, Debug, Serialize)]
struct Consequent {
    ordinal: usize,
    count: Option<u64>,
}

#[derive(Clone, Debug, Serialize)]
struct ClauseObservation {
    identity: ClauseRef,
    expected_consequents: usize,
    evaluation_count: Option<u64>,
    consequents: Vec<Consequent>,
    classification: Option<ClauseCoverage>,
    diagnostics: Vec<CoverageDiagnostic>,
}

#[derive(Debug, Serialize)]
struct ObservationBody {
    format: &'static str,
    state: BoundAnalysisState,
    population: &'static str,
    source_root: Option<String>,
    informational: Vec<ClauseRef>,
    artifacts: Vec<ArtifactObservation>,
    clauses: Vec<ClauseObservation>,
    diagnostics: Vec<CoverageDiagnostic>,
}

/// Immutable complete-package domain observations.
///
/// Only the analyzer constructs this type. The sole wire producer is bounded serialization;
/// there is no public Deserialize, mutable report body, or native qualification constructor.
#[derive(Debug)]
pub struct BoundCoverageAnalysis(ObservationBody);

impl BoundCoverageAnalysis {
    /// Domain computation state, independent of native-run authentication or clause favorability.
    #[must_use]
    pub fn state(&self) -> BoundAnalysisState {
        self.0.state
    }

    /// Emit deterministic bounded domain bytes, never a proof attestation or verdict.
    pub fn to_json_bytes(&self) -> Result<Vec<u8>, CoverageDiagnostic> {
        check_output_size(&self.0, MAX_ANALYSIS_BYTES)?;
        serde_json::to_vec(&self.0).map_err(|_| {
            diag(
                CoverageErrorCode::MalformedExport,
                "analysis serialization failed",
            )
        })
    }
}

fn diag(code: CoverageErrorCode, message: &str) -> CoverageDiagnostic {
    CoverageDiagnostic {
        code,
        message: message.to_owned(),
    }
}

/// Analyze every executable clause against its independently expected generated inventory.
///
/// Global population/artifact/map failures erase all measured classifications. Observation
/// gaps retain the complete expected clause population with unavailable counts as null.
/// Trace: TC-006, FR-004-AC-3, FR-004-AC-5, FR-004-AC-7, FR-004-AC-9
pub fn analyze_bound_coverage(
    package: &BoundPackage,
    generated: &BoundOracleGeneration,
    inputs: BoundCoverageInputs<'_>,
) -> BoundCoverageAnalysis {
    let mut body = ObservationBody {
        format: BOUND_COVERAGE_FORMAT,
        state: BoundAnalysisState::InvalidInput,
        population: "not_emitted",
        source_root: None,
        informational: package.informational().to_vec(),
        artifacts: Vec::new(),
        clauses: Vec::new(),
        diagnostics: Vec::new(),
    };
    if let Err(error) = analyze_inner(package, generated, &inputs, &mut body) {
        body.state = state_for(&error);
        body.population = "not_emitted";
        body.clauses.clear();
        body.diagnostics.push(error);
    }
    if check_output_size(&body, MAX_ANALYSIS_BYTES).is_err() {
        body.state = BoundAnalysisState::Unsupported;
        body.population = "not_emitted";
        body.clauses.clear();
        body.artifacts.clear();
        body.informational.clear();
        body.diagnostics =
            vec![diag(CoverageErrorCode::ResourceLimitExceeded,
            "analysis output exceeds 16 MiB; population not emitted, no observations classified")];
    }
    BoundCoverageAnalysis(body)
}

fn state_for(error: &CoverageDiagnostic) -> BoundAnalysisState {
    match error.code {
        CoverageErrorCode::UnsupportedProfile | CoverageErrorCode::ResourceLimitExceeded => {
            BoundAnalysisState::Unsupported
        }
        _ => BoundAnalysisState::InvalidInput,
    }
}

fn analyze_inner(
    package: &BoundPackage,
    generated: &BoundOracleGeneration,
    inputs: &BoundCoverageInputs<'_>,
    body: &mut ObservationBody,
) -> Result<(), CoverageDiagnostic> {
    if inputs.source_root.len() > MAX_SOURCE_ROOT_BYTES {
        return Err(diag(
            CoverageErrorCode::ResourceLimitExceeded,
            "source root exceeds 4096 bytes",
        ));
    }
    let root = normalize_path(inputs.source_root)?;
    if !root.starts_with('/') || root == "/" {
        return Err(diag(
            CoverageErrorCode::InvalidPath,
            "source root must be an absolute package directory",
        ));
    }
    body.source_root = Some(root.clone());
    if let Some(bytes) = inputs.llvm_export {
        if bytes.len() > MAX_COVERAGE_BYTES {
            return Err(diag(
                CoverageErrorCode::ResourceLimitExceeded,
                "LLVM JSON exceeds 16 MiB",
            ));
        }
    }
    let generated = match generated {
        BoundOracleGeneration::NoExecutable(_) => {
            if !inputs.artifacts.is_empty() || inputs.llvm_export.is_some() {
                return Err(diag(
                    CoverageErrorCode::ArtifactMismatch,
                    "no-executable input must not attach artifacts or coverage",
                ));
            }
            body.state = BoundAnalysisState::NoExecutable;
            body.population = "complete";
            return Ok(());
        }
        BoundOracleGeneration::Generated(g) => g,
    };
    check_artifacts(generated, inputs.artifacts, body)?;
    let maps = check_maps(package, generated)?;
    let coverage = inputs
        .llvm_export
        .map(|bytes| parse_llvm_coverage(bytes, &root))
        .transpose()?;
    if let Some(coverage) = &coverage {
        let expected: BTreeSet<_> = generated
            .clauses()
            .iter()
            .map(|c| c.bundle().rust.path.as_str())
            .collect();
        if coverage
            .paths()
            .any(|path| path.starts_with("src/generated/") && !expected.contains(path))
        {
            return Err(diag(
                CoverageErrorCode::ForeignGeneratedFile,
                "foreign generated source in coverage export",
            ));
        }
    }
    body.state = BoundAnalysisState::Complete;
    body.population = "complete";
    for (clause, map) in package.clauses().iter().zip(maps) {
        let row = observe_clause(clause, &map, coverage.as_ref());
        if row.classification.is_none() {
            body.state = BoundAnalysisState::Incomplete;
        }
        body.clauses.push(row);
    }
    Ok(())
}

fn check_artifacts(
    generated: &GeneratedBoundOracles,
    supplied: &[ArtifactBytes<'_>],
    body: &mut ObservationBody,
) -> Result<(), CoverageDiagnostic> {
    if supplied.len() > MAX_ARTIFACTS {
        return Err(diag(
            CoverageErrorCode::ResourceLimitExceeded,
            "artifact count exceeded",
        ));
    }
    if supplied.len() != generated.bundle().artifacts().len() {
        return Err(diag(
            CoverageErrorCode::ArtifactMismatch,
            "complete artifact inventory required",
        ));
    }
    let mut total = 0usize;
    let mut index = BTreeMap::new();
    for artifact in supplied {
        total = total.saturating_add(artifact.bytes.len());
        if artifact.bytes.len() > MAX_ARTIFACT_BYTES || total > MAX_BUNDLE_BYTES {
            return Err(diag(
                CoverageErrorCode::ResourceLimitExceeded,
                "artifact byte budget exceeded",
            ));
        }
        if index.insert(artifact.path, artifact.bytes).is_some() {
            return Err(diag(
                CoverageErrorCode::ArtifactMismatch,
                "duplicate supplied artifact path",
            ));
        }
    }
    for artifact in generated.bundle().artifacts() {
        let bytes = index.get(artifact.path.as_str()).ok_or_else(|| {
            diag(
                CoverageErrorCode::ArtifactMismatch,
                "missing or foreign artifact path",
            )
        })?;
        if *bytes != artifact.contents.as_bytes() {
            return Err(diag(
                CoverageErrorCode::ArtifactMismatch,
                "artifact bytes differ from immutable generation",
            ));
        }
        body.artifacts.push(ArtifactObservation {
            path: artifact.path.clone(),
            bytes: bytes.len(),
        });
    }
    Ok(())
}

// Left-own-right follows the consequent-emission events, not ordinary node pre-order.
fn implication_census(expression: &Expression) -> Vec<&Expression> {
    let mut pending = vec![(expression, false)];
    let mut implications = Vec::new();
    while let Some((node, visited_left)) = pending.pop() {
        match node.kind() {
            ExpressionKind::Boolean {
                operator,
                left,
                right,
            } => {
                if visited_left {
                    if *operator == BooleanOperator::Implication {
                        implications.push(node);
                    }
                    pending.push((right, false));
                } else {
                    pending.push((node, true));
                    pending.push((left, false));
                }
            }
            ExpressionKind::BooleanNot { operand } => pending.push((operand, false)),
            _ => {}
        }
    }
    implications
}

fn check_maps(
    package: &BoundPackage,
    generated: &GeneratedBoundOracles,
) -> Result<Vec<Vec<SourceRegion>>, CoverageDiagnostic> {
    let mut maps = Vec::new();
    for (clause, output) in package.clauses().iter().zip(generated.clauses()) {
        let bundle = output.bundle();
        let regions: Vec<SourceRegion> = serde_json::from_str(&bundle.source_map.contents)
            .map_err(|_| {
                diag(
                    CoverageErrorCode::MapMismatch,
                    "generated map shape is invalid",
                )
            })?;
        let count = implication_census(clause.expression().expression()).len();
        let id = clause.identity();
        let owner = id.requirement();
        let lines: Vec<_> = bundle.rust.contents.lines().collect();
        let mut probes = BTreeSet::new();
        if regions.len() != count + 2 {
            return Err(diag(
                CoverageErrorCode::MapMismatch,
                "typed implication census differs from map",
            ));
        }
        for (index, region) in regions.iter().enumerate() {
            let valid = region.artifact_path == bundle.rust.path
                && region.package_id == owner.package().as_str()
                && region.requirement_id == owner.requirement().as_str()
                && region.requirement_revision == owner.revision().get()
                && region.clause_id == id.clause().as_str()
                && region.start_line > 0
                && region.end_line >= region.start_line
                && region.end_line as usize <= lines.len();
            if !valid {
                return Err(diag(
                    CoverageErrorCode::MapMismatch,
                    "map identity or source range differs",
                ));
            }
            if index == 0 {
                if region.role != "clause"
                    || region.probe.is_some()
                    || region.expected_consequents != Some(count as u32)
                    || region.start_line != 1
                    || region.end_line as usize != lines.len()
                {
                    return Err(diag(
                        CoverageErrorCode::MapMismatch,
                        "clause envelope differs from typed census",
                    ));
                }
            } else {
                let expected_role = if index == 1 {
                    "oracle_evaluation"
                } else {
                    "implication_consequent"
                };
                let p = region.probe.ok_or_else(|| {
                    diag(CoverageErrorCode::MapMismatch, "missing semantic probe")
                })?;
                if region.role != expected_role
                    || region.expected_consequents.is_some()
                    || p.line < region.start_line
                    || p.line > region.end_line
                    || p.start_column == 0
                    || p.end_column <= p.start_column
                    || p.end_column as usize > lines[p.line as usize - 1].len() + 1
                    || !lines[p.line as usize - 1].is_char_boundary(p.start_column as usize - 1)
                    || !lines[p.line as usize - 1].is_char_boundary(p.end_column as usize - 1)
                    || !probes.insert((p.line, p.start_column, p.end_column))
                {
                    return Err(diag(
                        CoverageErrorCode::MapMismatch,
                        "invalid, duplicate, or misidentified semantic probe",
                    ));
                }
                if index > 1 && region.start_line <= regions[1].end_line {
                    return Err(diag(
                        CoverageErrorCode::MapMismatch,
                        "consequent overlaps evaluation entry",
                    ));
                }
            }
        }
        maps.push(regions);
    }
    Ok(maps)
}

fn observe_clause(
    clause: &quire_contract_model::BoundClause,
    map: &[SourceRegion],
    coverage: Option<&LlvmCoverage>,
) -> ClauseObservation {
    let mut row = ClauseObservation {
        identity: clause.identity().clone(),
        expected_consequents: implication_census(clause.expression().expression()).len(),
        evaluation_count: None,
        consequents: Vec::new(),
        classification: None,
        diagnostics: Vec::new(),
    };
    let observe = |region: &SourceRegion| {
        coverage
            .ok_or_else(|| {
                diag(
                    CoverageErrorCode::UnavailableObservation,
                    "LLVM export was not supplied",
                )
            })
            .and_then(|c| {
                region
                    .probe
                    .ok_or_else(|| diag(CoverageErrorCode::MapMismatch, "missing semantic probe"))
                    .and_then(|probe| c.observe(&region.artifact_path, probe))
            })
    };
    // The clause envelope, the oracle evaluation, then the consequents: the shape `check_maps`
    // requires. A shorter map is the census mismatch it reports, with nothing observed.
    let [_, evaluation_region, consequent_regions @ ..] = map else {
        row.diagnostics.push(diag(
            CoverageErrorCode::MapMismatch,
            "typed implication census differs from map",
        ));
        return row;
    };
    let evaluation = observe(evaluation_region);
    match &evaluation {
        Ok(value) => row.evaluation_count = Some(value.count()),
        Err(error) => row.diagnostics.push(error.clone()),
    }
    let mut observations = Vec::new();
    for (ordinal, region) in consequent_regions.iter().enumerate() {
        let count = match observe(region) {
            Ok(value) => {
                observations.push(value);
                Some(value.count())
            }
            Err(error) => {
                row.diagnostics.push(error);
                None
            }
        };
        row.consequents.push(Consequent { ordinal, count });
    }
    if let Ok(evaluation) = &evaluation {
        if observations
            .iter()
            .any(|count| count.count() > evaluation.count())
        {
            row.diagnostics.push(diag(
                CoverageErrorCode::InconsistentObservation,
                "consequent count exceeds its loop-free generated oracle evaluation count",
            ));
        }
    }
    if let (Ok(evaluation), true) = (evaluation, row.diagnostics.is_empty()) {
        match classify_clause(evaluation, row.expected_consequents as u32, &observations) {
            Ok(classification) => row.classification = Some(classification),
            Err(error) => row.diagnostics.push(error),
        }
    }
    row
}

struct CountWriter {
    bytes: usize,
    limit: usize,
}
impl Write for CountWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.bytes = self.bytes.saturating_add(bytes.len());
        if self.bytes > self.limit {
            return Err(io::Error::other("bounded output exceeded"));
        }
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn check_output_size(body: &ObservationBody, limit: usize) -> Result<(), CoverageDiagnostic> {
    serde_json::to_writer(CountWriter { bytes: 0, limit }, body).map_err(|_| {
        diag(
            CoverageErrorCode::ResourceLimitExceeded,
            "bounded analysis serialization refused",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quire_contract_model::EXECUTABLE_PROJECTION_FORMAT;
    use serde_json::{json, Value};

    const PACKAGE: &str = "test/no-generation-panics";
    const PATH: &str = "src/generated/clause.rs";

    fn span() -> Value {
        let source = json!({"document": "no-generation-panics", "revision": 1});
        json!({"start": {"source": source, "line": 1, "column": 1, "byte_offset": 0},
            "end": {"source": source, "line": 1, "column": 2, "byte_offset": 1}})
    }

    fn read(name: &str) -> Value {
        json!({"node": "value_reference", "name": name, "observation": "current",
            "source": span()})
    }

    /// One precondition clause whose expression is the single implication `a => b`.
    fn implication_package() -> BoundPackage {
        let owner = json!({"package": PACKAGE, "requirement": "FR-001", "revision": 1});
        let anchor = json!({"kind": "pre", "operation": "check"});
        let reference = |name: &str| {
            json!({"node": "reference", "identity": {"requirement": owner, "kind": "input",
                "path": [name], "observation": "current"}})
        };
        let value = |name: &str| {
            json!({"name": name, "kind": "input", "value_type": {"kind": "boolean"},
                "source": span()})
        };
        let expression = json!({"node": "boolean", "operator": "implication",
            "left": read("a"), "right": read("b"), "source": span()});
        let clause = json!({"id": "c0", "kind": "precondition", "anchor": anchor,
            "source": span(),
            "body": {"node": "composite", "children": [reference("a"), reference("b")]}});
        let binding = json!({"clause": {"requirement": owner, "clause": "c0"},
            "expression": {"owner": owner, "types": [], "values": [value("a"), value("b")],
                "functions": [], "expression": expression,
                "expected_type": {"kind": "boolean"}, "execution_point": anchor,
                "clause_root": true}});
        let projection = json!({"format": EXECUTABLE_PROJECTION_FORMAT,
            "package": {"id": PACKAGE, "schema_version": {"major": 1, "minor": 1},
                "source": {"document": "no-generation-panics", "revision": 1},
                "requirements": [{"id": "FR-001", "revision": 1, "source": span(),
                    "clauses": [clause]}]},
            "bindings": [binding]});
        BoundPackage::from_json_bytes(&serde_json::to_vec(&projection).unwrap())
            .unwrap_or_else(|diagnostics| panic!("the fixture must bind: {diagnostics:?}"))
    }

    fn region(role: &str, probe: Option<(u32, u32)>) -> SourceRegion {
        SourceRegion {
            artifact_path: PATH.to_owned(),
            role: role.to_owned(),
            start_line: 1,
            end_line: 3,
            package_id: PACKAGE.to_owned(),
            requirement_id: "FR-001".to_owned(),
            requirement_revision: 1,
            clause_id: "c0".to_owned(),
            probe: probe.map(
                |(line, start_column)| crate::core::source_map::SourceProbe {
                    line,
                    start_column,
                    end_column: start_column + 1,
                },
            ),
            expected_consequents: None,
        }
    }

    /// The map of `a => b`: the clause envelope, the oracle evaluation (probe on line 1) and the
    /// one implication consequent (probe on line 2), each probe present as named.
    fn map(evaluation_probe: bool, consequent_probe: bool) -> Vec<SourceRegion> {
        let mut envelope = region("clause", None);
        envelope.expected_consequents = Some(1);
        vec![
            envelope,
            region("oracle_evaluation", evaluation_probe.then_some((1, 1))),
            region("implication_consequent", consequent_probe.then_some((2, 1))),
        ]
    }

    /// An export that measures both probes of [`map`].
    fn coverage() -> LlvmCoverage {
        let export = json!({"type": "llvm.coverage.json.export",
            "cargo_llvm_cov": {"manifest_path": "/fixture/Cargo.toml"},
            "data": [{"files": [{"filename": PATH, "segments": [
                [1, 1, 1, true, true, false], [1, 2, 0, false, false, false],
                [2, 1, 1, true, true, false], [2, 2, 0, false, false, false]]}]}]});
        parse_llvm_coverage(&serde_json::to_vec(&export).unwrap(), "/fixture")
            .expect("the export parses")
    }

    fn codes(row: &ClauseObservation) -> Vec<(CoverageErrorCode, &str)> {
        row.diagnostics
            .iter()
            .map(|diagnostic| (diagnostic.code, diagnostic.message.as_str()))
            .collect()
    }

    /// Trace: NFR-005-AC-4, TC-042. With a coverage export supplied, a map region with no probe
    /// is the `MapMismatch` diagnostic and no classification; with no export it is
    /// `UnavailableObservation`.
    #[test]
    fn tc_042_ac4_a_region_without_a_probe_is_a_map_mismatch_not_a_panic() {
        let package = implication_package();
        let clause = &package.clauses()[0];
        let supplied = coverage();

        let complete = observe_clause(clause, &map(true, true), Some(&supplied));
        assert!(complete.diagnostics.is_empty(), "{:?}", codes(&complete));
        assert!(complete.classification.is_some());

        let missing_evaluation = observe_clause(clause, &map(false, true), Some(&supplied));
        assert!(codes(&missing_evaluation)
            .contains(&(CoverageErrorCode::MapMismatch, "missing semantic probe")));
        assert!(missing_evaluation.classification.is_none());
        assert_eq!(missing_evaluation.evaluation_count, None);

        let missing_consequent = observe_clause(clause, &map(true, false), Some(&supplied));
        assert_eq!(
            codes(&missing_consequent),
            vec![(CoverageErrorCode::MapMismatch, "missing semantic probe")]
        );
        assert!(missing_consequent.classification.is_none());

        for probes in [(false, true), (true, false)] {
            let unavailable = observe_clause(clause, &map(probes.0, probes.1), None);
            assert!(unavailable.classification.is_none());
            assert!(
                unavailable
                    .diagnostics
                    .iter()
                    .all(|diagnostic| diagnostic.code == CoverageErrorCode::UnavailableObservation),
                "{:?}",
                codes(&unavailable)
            );
            assert!(!unavailable.diagnostics.is_empty());
        }
    }

    /// Trace: NFR-005-AC-7, TC-042. A map with fewer than two regions is the census `MapMismatch`
    /// with nothing observed, with or without a coverage export; two probed regions are not.
    #[test]
    fn tc_042_ac7_a_map_shorter_than_two_regions_is_a_census_mismatch_not_a_panic() {
        let package = implication_package();
        let clause = &package.clauses()[0];
        let supplied = coverage();
        let full = map(true, true);

        for length in [0, 1] {
            for export in [Some(&supplied), None] {
                let row = observe_clause(clause, &full[..length], export);
                assert_eq!(
                    codes(&row),
                    vec![(
                        CoverageErrorCode::MapMismatch,
                        "typed implication census differs from map"
                    )],
                    "{length} regions, export supplied: {}",
                    export.is_some()
                );
                assert!(row.classification.is_none());
                assert_eq!(row.evaluation_count, None);
                assert!(row.consequents.is_empty());
            }
        }

        let two = observe_clause(clause, &full[..2], Some(&supplied));
        assert!(two.consequents.is_empty());
        assert_eq!(two.evaluation_count, Some(1));
        assert!(
            two.diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != CoverageErrorCode::MapMismatch),
            "{:?}",
            codes(&two)
        );
    }

    /// Trace: TC-006, FR-004-AC-5
    #[test]
    fn output_bound_counts_exact_bytes_before_allocating_output() {
        let mut writer = CountWriter { bytes: 0, limit: 2 };
        assert_eq!(writer.write(b"ok").unwrap(), 2);
        assert!(writer.write(b"!").is_err());
        assert_eq!(writer.bytes, 3);
    }
}

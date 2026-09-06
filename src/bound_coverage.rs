//! Complete bound observations. No execution, authentication, or assurance verdict.
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write};

use quire_contract_ir::{BooleanOperator, BoundPackage, ClauseRef, Expression, ExpressionKind};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{
    classify_clause, parse_llvm_coverage, publication, vacuity::normalize_path,
    BoundOracleGeneration, ClauseCoverage, CoverageDiagnostic, CoverageErrorCode,
    GeneratedBoundOracles, LlvmCoverage, SourceRegion, MAX_COVERAGE_BYTES,
};

/// Domain observation format, not a native-run result or attestation format.
pub const BOUND_COVERAGE_FORMAT: &str = "codegen.bound-coverage-observations/v1";
/// Strict output schema for the domain observations emitted here.
pub const BOUND_COVERAGE_SCHEMA: &str =
    include_str!("../schemas/bound-coverage-observations-v1.schema.json");
/// Maximum serialized analysis bytes, including explicit refusal outcomes.
pub const MAX_ANALYSIS_BYTES: usize = 16 * 1024 * 1024;

/// Borrowed untrusted artifact bytes. The analyzer recomputes their identity.
pub struct ArtifactBytes<'a> {
    /// Exact bundle-relative path; publication aliases are not alternate identities.
    pub path: &'a str,
    /// Exact bytes supplied for this expected artifact.
    pub bytes: &'a [u8],
}

/// Complete caller inputs, consumed without filesystem reads or subprocess execution.
pub struct BoundCoverageInputs<'a> {
    /// Absolute lexical package root, following the qualified LLVM parser's rules.
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
struct ArtifactIdentity {
    path: String,
    bytes: usize,
    sha256: String,
}

#[derive(Clone, Debug, Serialize)]
struct Consequent {
    ordinal: usize,
    count: Option<u64>,
}

#[derive(Clone, Debug, Serialize)]
struct ClauseObservation {
    identity: ClauseRef,
    expression_sha256: String,
    declaration_sha256: String,
    expected_consequents: usize,
    evaluation_count: Option<u64>,
    consequents: Vec<Consequent>,
    classification: Option<ClauseCoverage>,
    diagnostics: Vec<CoverageDiagnostic>,
}

#[derive(Debug, Serialize)]
struct ObservationBody {
    format: &'static str,
    schema_sha256: String,
    state: BoundAnalysisState,
    provenance: &'static str,
    population: &'static str,
    analyzer_revision: &'static str,
    analyzer_source_state: &'static str,
    analyzer_implementation_sha256: String,
    bound_sha256: String,
    export_sha256: Option<String>,
    informational: Vec<ClauseRef>,
    artifacts: Vec<ArtifactIdentity>,
    clauses: Vec<ClauseObservation>,
    diagnostics: Vec<CoverageDiagnostic>,
}

/// Immutable complete-package domain observations; always unqualified provenance.
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

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Analyze every executable clause against its independently expected generated inventory.
///
/// Global population/artifact/map failures erase all measured classifications. Observation
/// gaps retain the complete expected clause population with unavailable counts as null.
/// Complete observations, including all-exercised observations, remain unqualified.
/// Trace: TC-006, FR-004-AC-3, FR-004-AC-5, FR-004-AC-7, FR-004-AC-9
pub fn analyze_bound_coverage(
    package: &BoundPackage,
    generated: &BoundOracleGeneration,
    inputs: BoundCoverageInputs<'_>,
) -> BoundCoverageAnalysis {
    let mut body = ObservationBody {
        format: BOUND_COVERAGE_FORMAT,
        schema_sha256: sha(BOUND_COVERAGE_SCHEMA.as_bytes()),
        state: BoundAnalysisState::InvalidInput,
        provenance: "unqualified",
        population: "not_emitted",
        analyzer_revision: crate::GENERATOR_SOURCE_REVISION,
        analyzer_source_state: if crate::generator_source_is_dirty() {
            "dirty"
        } else {
            "clean"
        },
        analyzer_implementation_sha256: implementation_digest(),
        bound_sha256: package.digest().to_string(),
        export_sha256: None,
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
    let root = normalize_path(inputs.source_root)?;
    if !root.starts_with('/') || root == "/" {
        return Err(diag(
            CoverageErrorCode::InvalidPath,
            "source root must be an absolute package directory",
        ));
    }
    if let Some(bytes) = inputs.llvm_export {
        if bytes.len() > MAX_COVERAGE_BYTES {
            return Err(diag(
                CoverageErrorCode::ResourceLimitExceeded,
                "LLVM JSON exceeds 16 MiB",
            ));
        }
        body.export_sha256 = Some(sha(bytes));
    }
    let generated = match generated {
        BoundOracleGeneration::NoExecutable(no_work) => {
            if !package.clauses().is_empty()
                || no_work.bound_digest() != package.digest()
                || no_work.informational() != package.informational()
            {
                return Err(diag(
                    CoverageErrorCode::BindingMismatch,
                    "no-executable generation differs from package",
                ));
            }
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
    check_binding(package, generated)?;
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

fn check_binding(
    package: &BoundPackage,
    generated: &GeneratedBoundOracles,
) -> Result<(), CoverageDiagnostic> {
    if generated.bound_digest() != package.digest()
        || generated.informational() != package.informational()
        || generated.clauses().len() != package.clauses().len()
        || package.clauses().is_empty()
        || generated
            .clauses()
            .iter()
            .zip(package.clauses())
            .any(|(a, b)| {
                a.identity() != b.identity()
                    || a.expression_digest() != b.expression_digest()
                    || a.declaration_digest() != b.declaration_digest()
            })
    {
        return Err(diag(
            CoverageErrorCode::BindingMismatch,
            "immutable generated population differs from bound package",
        ));
    }
    Ok(())
}

fn check_artifacts(
    generated: &GeneratedBoundOracles,
    supplied: &[ArtifactBytes<'_>],
    body: &mut ObservationBody,
) -> Result<(), CoverageDiagnostic> {
    if supplied.len() > publication::MAX_ARTIFACTS {
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
        if artifact.bytes.len() > publication::MAX_ARTIFACT_BYTES
            || total > publication::MAX_BUNDLE_BYTES
        {
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
        let digest = sha(bytes);
        if *bytes != artifact.contents.as_bytes() || digest != artifact.sha256 {
            return Err(diag(
                CoverageErrorCode::ArtifactMismatch,
                "artifact bytes differ from immutable generation",
            ));
        }
        body.artifacts.push(ArtifactIdentity {
            path: artifact.path.clone(),
            bytes: bytes.len(),
            sha256: digest,
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
    clause: &quire_contract_ir::BoundClause,
    map: &[SourceRegion],
    coverage: Option<&LlvmCoverage>,
) -> ClauseObservation {
    let mut row = ClauseObservation {
        identity: clause.identity().clone(),
        expression_sha256: clause.expression_digest().to_string(),
        declaration_sha256: clause.declaration_digest().to_string(),
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
            .and_then(|c| c.observe(&region.artifact_path, region.probe.expect("map preflight")))
    };
    let evaluation = observe(&map[1]);
    match &evaluation {
        Ok(value) => row.evaluation_count = Some(value.count()),
        Err(error) => row.diagnostics.push(error.clone()),
    }
    let mut observations = Vec::new();
    for (ordinal, region) in map[2..].iter().enumerate() {
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
    if row.diagnostics.is_empty() {
        match classify_clause(
            evaluation.expect("observed evaluation"),
            row.expected_consequents as u32,
            &observations,
        ) {
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

fn implementation_digest() -> String {
    let mut digest = Sha256::new();
    for source in [
        include_bytes!("bound_coverage.rs").as_slice(),
        include_bytes!("vacuity.rs").as_slice(),
        include_bytes!("../Cargo.lock").as_slice(),
        BOUND_COVERAGE_SCHEMA.as_bytes(),
    ] {
        digest.update((source.len() as u64).to_le_bytes());
        digest.update(source);
    }
    format!("{:x}", digest.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Trace: TC-006, FR-004-AC-5
    #[test]
    fn output_bound_counts_exact_bytes_before_allocating_output() {
        let mut writer = CountWriter { bytes: 0, limit: 2 };
        assert_eq!(writer.write(b"ok").unwrap(), 2);
        assert!(writer.write(b"!").is_err());
        assert_eq!(writer.bytes, 3);
    }
}

//! Bounded LLVM observation primitives, not a campaign or coverage-obligation verdict.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::SourceProbe;

/// Maximum accepted raw LLVM JSON size; checked before deserialization.
pub const MAX_COVERAGE_BYTES: usize = 16 * 1024 * 1024;
const MAX_FILES: usize = 4096;
const MAX_SEGMENTS: usize = 250_000;

/// Stable refusal classes for low-level coverage observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageErrorCode {
    /// Bound package and immutable generated population differ.
    BindingMismatch,
    /// Supplied artifact inventory or bytes differ from the generated bundle.
    ArtifactMismatch,
    /// Generated map does not match independently derived typed clause semantics.
    MapMismatch,
    /// An eligible generated source file is outside the expected population.
    ForeignGeneratedFile,
    /// The raw export or decoded collection exceeds the bounded profile.
    ResourceLimitExceeded,
    /// JSON shape, tuple, or source coordinate ordering is invalid.
    MalformedExport,
    /// The producer or LLVM format is outside the qualified pair.
    UnsupportedProfile,
    /// A path is traversing, ambiguous, or outside its declared root.
    InvalidPath,
    /// Multiple LLVM file entries normalize to one eligible path.
    DuplicateFile,
    /// No eligible generated file or complete measured span contains the probe.
    UnavailableObservation,
    /// The independently expected population differs from the supplied observations.
    PopulationMismatch,
    /// A positive consequent has a measured-zero owning oracle entry.
    InconsistentObservation,
}

/// A refusal preserves its diagnostic instead of inventing measured-zero coverage.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CoverageDiagnostic {
    /// Stable machine-readable category.
    pub code: CoverageErrorCode,
    /// Input location or explanation, not execution evidence.
    pub message: String,
}

fn diagnostic(code: CoverageErrorCode, message: impl Into<String>) -> CoverageDiagnostic {
    CoverageDiagnostic {
        code,
        message: message.into(),
    }
}

/// Measured count from a complete count-bearing, non-gap LLVM span.
///
/// Construction is restricted to successful export/probe observation. Zero is a measured value,
/// never a stand-in for a missing file, probe, or coverage producer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProbeObservation(u64);

impl ProbeObservation {
    /// LLVM count for this particular entry probe, not a runtime campaign count.
    #[must_use]
    pub fn count(self) -> u64 {
        self.0
    }
}

/// Four mutually exclusive classifications for fully measured clause probes.
///
/// This is not an attestation, a native-run binding, or discharge of an obligation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClauseCoverage {
    /// The oracle entry was measured zero.
    Unexecuted,
    /// An entered implication-bearing oracle evaluated no consequent.
    Vacuous,
    /// An entered oracle evaluated a proper subset of its expected consequents.
    PartiallyExercised,
    /// An entered oracle evaluated every expected consequent, including the empty set.
    Exercised,
}

/// Classify measured probes against an independently derived typed-expression census.
///
/// Callers must not derive `expected_consequents` by counting source-map rows. The future bound
/// population API will enforce that ownership; this primitive alone cannot establish completeness.
pub fn classify_clause(
    evaluation: ProbeObservation,
    expected_consequents: u32,
    consequents: &[ProbeObservation],
) -> Result<ClauseCoverage, CoverageDiagnostic> {
    if expected_consequents as usize != consequents.len() {
        return Err(diagnostic(
            CoverageErrorCode::PopulationMismatch,
            "typed implication census differs from observations",
        ));
    }
    let observed = consequents.iter().filter(|count| count.0 > 0).count();
    if evaluation.0 == 0 {
        return if observed == 0 {
            Ok(ClauseCoverage::Unexecuted)
        } else {
            Err(diagnostic(
                CoverageErrorCode::InconsistentObservation,
                "consequent observed with zero oracle entry count",
            ))
        };
    }
    Ok(if observed == consequents.len() {
        ClauseCoverage::Exercised
    } else if observed == 0 {
        ClauseCoverage::Vacuous
    } else {
        ClauseCoverage::PartiallyExercised
    })
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct Segment(u32, u32, u64, bool, bool, bool);

impl Segment {
    fn position(self) -> (u32, u32) {
        (self.0, self.1)
    }
}

#[derive(Deserialize)]
struct Export {
    #[serde(rename = "type")]
    kind: String,
    version: String,
    cargo_llvm_cov: Producer,
    data: Vec<ExportData>,
}

#[derive(Deserialize)]
struct Producer {
    version: String,
    manifest_path: String,
}

#[derive(Deserialize)]
struct ExportData {
    files: Vec<ExportFile>,
}

#[derive(Deserialize)]
struct ExportFile {
    filename: String,
    segments: Vec<Segment>,
}

/// Parsed, normalized LLVM segment facts for the qualified structural profile.
///
/// Export metadata is self-declared. This type does not claim that a particular executable ran,
/// validate generated source digests, or bind runtime counters. Those are native-producer duties.
#[derive(Debug)]
pub struct LlvmCoverage {
    files: BTreeMap<String, Vec<Segment>>,
    export_sha256: String,
}

impl LlvmCoverage {
    pub(crate) fn paths(&self) -> impl Iterator<Item = &str> {
        self.files.keys().map(String::as_str)
    }
    /// Digest of the exact supplied bytes, not canonicalized JSON.
    #[must_use]
    pub fn export_sha256(&self) -> &str {
        &self.export_sha256
    }

    /// Observe one complete single-line entry token in a root-relative generated source file.
    ///
    /// Partial intersection, gap/non-count spans, and an unterminated final span are unavailable,
    /// not measured-zero. A probe crossing a segment boundary must be narrowed by the generator;
    /// the analyzer never upgrades a partial overlap to observed execution.
    pub fn observe(
        &self,
        artifact_path: &str,
        probe: SourceProbe,
    ) -> Result<ProbeObservation, CoverageDiagnostic> {
        let path = normalize_path(artifact_path)?;
        if path.starts_with('/')
            || probe.line == 0
            || probe.start_column == 0
            || probe.end_column <= probe.start_column
        {
            return Err(diagnostic(
                CoverageErrorCode::InvalidPath,
                "expected a relative path and a nonempty one-based probe",
            ));
        }
        let unavailable = || {
            diagnostic(
                CoverageErrorCode::UnavailableObservation,
                format!(
                    "no complete measured span for {path}:{}:{}",
                    probe.line, probe.start_column
                ),
            )
        };
        let segments = self.files.get(&path).ok_or_else(unavailable)?;
        let start = (probe.line, probe.start_column);
        let end = (probe.line, probe.end_column);
        for pair in segments.windows(2) {
            let segment = pair[0];
            if segment.position() <= start && end <= pair[1].position() && segment.3 && !segment.5 {
                return Ok(ProbeObservation(segment.2));
            }
        }
        Err(unavailable())
    }
}

/// Parse full cargo-llvm-cov 0.9.0 / LLVM JSON 3.0.1 without executing any producer.
///
/// `source_root` must be an absolute lexical path to the fixture/package containing Cargo.toml.
/// Absolute external dependency files are checked structurally but cannot match generated paths.
/// Parent traversal and backslashes are refused; dot/repeated-separator aliases normalize before
/// duplicate detection. No suffix matching, sorting of malformed segments, or summary fallback.
pub fn parse_llvm_coverage(
    bytes: &[u8],
    source_root: &str,
) -> Result<LlvmCoverage, CoverageDiagnostic> {
    if bytes.len() > MAX_COVERAGE_BYTES {
        return Err(diagnostic(
            CoverageErrorCode::ResourceLimitExceeded,
            "LLVM JSON exceeds 16 MiB",
        ));
    }
    let root = normalize_path(source_root)?;
    if !root.starts_with('/') || root == "/" {
        return Err(diagnostic(
            CoverageErrorCode::InvalidPath,
            "source root must be an absolute package directory",
        ));
    }
    let export: Export = serde_json::from_slice(bytes)
        .map_err(|error| diagnostic(CoverageErrorCode::MalformedExport, error.to_string()))?;
    if export.kind != "llvm.coverage.json.export"
        || export.version != "3.0.1"
        || export.cargo_llvm_cov.version != "0.9.0"
    {
        return Err(diagnostic(
            CoverageErrorCode::UnsupportedProfile,
            format!(
                "expected cargo-llvm-cov 0.9.0 / llvm.coverage.json.export 3.0.1; found {} / {} {}",
                export.cargo_llvm_cov.version, export.kind, export.version
            ),
        ));
    }
    if normalize_path(&export.cargo_llvm_cov.manifest_path)? != format!("{root}/Cargo.toml") {
        return Err(diagnostic(
            CoverageErrorCode::InvalidPath,
            "producer manifest does not match source root",
        ));
    }
    if export.data.is_empty() {
        return Err(diagnostic(
            CoverageErrorCode::MalformedExport,
            "missing coverage data",
        ));
    }
    let mut files = BTreeMap::new();
    let mut file_count = 0usize;
    let mut segment_count = 0usize;
    for data in export.data {
        for file in data.files {
            file_count += 1;
            segment_count += file.segments.len();
            if file_count > MAX_FILES || segment_count > MAX_SEGMENTS {
                return Err(diagnostic(
                    CoverageErrorCode::ResourceLimitExceeded,
                    "LLVM file/segment census exceeded",
                ));
            }
            validate_segments(&file.segments)?;
            let normalized = normalize_path(&file.filename)?;
            let relative = if normalized.starts_with('/') {
                let Some(relative) = normalized.strip_prefix(&format!("{root}/")) else {
                    continue;
                };
                relative.to_owned()
            } else {
                normalized
            };
            if files.insert(relative.clone(), file.segments).is_some() {
                return Err(diagnostic(CoverageErrorCode::DuplicateFile, relative));
            }
        }
    }
    Ok(LlvmCoverage {
        files,
        export_sha256: format!("{:x}", Sha256::digest(bytes)),
    })
}

fn validate_segments(segments: &[Segment]) -> Result<(), CoverageDiagnostic> {
    let mut previous = (0, 0);
    for segment in segments {
        if segment.0 == 0 || segment.1 == 0 || segment.position() <= previous {
            return Err(diagnostic(
                CoverageErrorCode::MalformedExport,
                "segment coordinates must be nonzero and strictly ordered",
            ));
        }
        // Region-entry is parsed as a strict boolean even though active-span observation does
        // not require a transition to begin a region (a resumed region is also measurable).
        let _is_region_entry = segment.4;
        previous = segment.position();
    }
    Ok(())
}

pub(crate) fn normalize_path(path: &str) -> Result<String, CoverageDiagnostic> {
    if path.is_empty() || path.contains(['\\', '\0']) {
        return Err(diagnostic(
            CoverageErrorCode::InvalidPath,
            "empty path, backslash, or NUL",
        ));
    }
    let mut parts = Vec::new();
    for part in path.split('/') {
        match part {
            ".." => {
                return Err(diagnostic(
                    CoverageErrorCode::InvalidPath,
                    "parent traversal is forbidden",
                ))
            }
            "" | "." => {}
            part => parts.push(part),
        }
    }
    if parts.is_empty() {
        return Err(diagnostic(
            CoverageErrorCode::InvalidPath,
            "empty normalized path",
        ));
    }
    Ok(format!(
        "{}{}",
        if path.starts_with('/') { "/" } else { "" },
        parts.join("/")
    ))
}

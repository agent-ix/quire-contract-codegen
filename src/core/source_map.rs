//! Source-map records that trace generated source back to a requirement clause (AD-004 step 2a).
//!
//! This module is `core/source_map.rs`. `evidence` (coverage) and `oracle` both read these
//! records, so they live below both.

use serde::{Deserialize, Serialize};

/// Trace from a generated source range back to one requirement clause.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SourceRegion {
    /// Generated source path.
    pub artifact_path: String,
    /// Semantic role, such as `clause` or `implication_consequent`.
    pub role: String,
    /// One-based inclusive starting line.
    pub start_line: u32,
    /// One-based inclusive ending line.
    pub end_line: u32,
    /// Package identity completing the clause reference.
    pub package_id: String,
    /// Requirement identity.
    pub requirement_id: String,
    /// Exact requirement revision.
    pub requirement_revision: u64,
    /// Clause identity.
    pub clause_id: String,
    /// Entry-token probe for executable semantic roles; never the whole clause envelope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe: Option<SourceProbe>,
    /// Implication census independently traversed from typed IR, on the clause envelope only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_consequents: Option<u32>,
}

/// Single-line source token to be contained by one measured LLVM active span.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SourceProbe {
    /// One-based source line.
    pub line: u32,
    /// One-based UTF-8 byte column, inclusive.
    pub start_column: u32,
    /// One-based UTF-8 byte column, exclusive.
    pub end_column: u32,
}

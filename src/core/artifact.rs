//! Generated files and the validated bundle every generator builds (AD-004 step 2a).
//!
//! This module is `core/artifact.rs`: it depends on no generator, strategy, evidence, Kani
//! or publication module. The atomic writer that consumes a bundle lives in `publication`.

use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

use serde::{Deserialize, Serialize};

use crate::core::diagnostic::GenerationTerminalState;

const BUNDLE_SCHEMA: &str = "quire.artifact-bundle/v1";
pub(crate) const MAX_ARTIFACTS: usize = 4096;
pub(crate) const MAX_ARTIFACT_BYTES: usize = 16 * 1024 * 1024;
pub(crate) const MAX_BUNDLE_BYTES: usize = 128 * 1024 * 1024;
/// Maximum generated Rust bytes for one clause.
pub const MAX_GENERATED_SOURCE_BYTES: usize = 1_048_576;

/// One generated file.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Artifact {
    /// Deterministic bundle-relative path.
    pub path: String,
    /// UTF-8 artifact contents.
    pub contents: String,
}

impl Artifact {
    /// One generated file at `path` holding `contents`.
    #[must_use]
    pub fn new(path: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            contents: contents.into(),
        }
    }
}

/// Stable reason a bundle could not be validated or published.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationErrorCode {
    /// The bundle contains no artifacts or exceeds a bounded resource limit.
    InvalidBundle,
    /// An artifact path is absolute, non-canonical, or traverses a parent.
    UnsafeArtifactPath,
    /// Two artifacts claim the same bundle-relative path.
    DuplicateArtifactPath,
    /// Staging, swapping, rollback, or cleanup encountered an I/O error.
    IoFailed,
}

/// Observable destination state when publication returns a diagnostic.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDestinationState {
    /// The destination was not changed by this call, or the prior bundle was restored.
    Unchanged,
    /// The new bundle was committed, but a post-commit operation failed.
    Published,
    /// Rollback itself failed, so callers must inspect the destination before retrying.
    Unknown,
}

impl PublicationErrorCode {
    /// Maps the publication failure onto the codegen terminal-state vocabulary.
    #[must_use]
    pub const fn terminal_state(self) -> GenerationTerminalState {
        match self {
            Self::InvalidBundle | Self::UnsafeArtifactPath | Self::DuplicateArtifactPath => {
                GenerationTerminalState::InvalidInput
            }
            Self::IoFailed => GenerationTerminalState::IoFailed,
        }
    }
}

/// Structured publication failure.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PublicationDiagnostic {
    /// Stable diagnostic category.
    pub code: PublicationErrorCode,
    /// Terminal state implied by `code`.
    pub terminal_state: GenerationTerminalState,
    /// Whether this call left the destination unchanged, published, or uncertain.
    pub destination_state: PublicationDestinationState,
    /// Stable bundle or filesystem path associated with the failure.
    pub path: String,
    /// Human-readable detail not used as machine identity.
    pub message: String,
}

/// Deterministic set of generated artifacts ready for publication.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ArtifactBundle {
    schema_version: String,
    artifacts: Vec<Artifact>,
}

impl ArtifactBundle {
    /// Validates and path-sorts a complete artifact set.
    pub fn new(mut artifacts: Vec<Artifact>) -> Result<Self, PublicationDiagnostic> {
        validate_artifacts(&artifacts)?;
        artifacts.sort_by(|left, right| left.path.cmp(&right.path));
        Ok(Self {
            schema_version: BUNDLE_SCHEMA.to_owned(),
            artifacts,
        })
    }

    /// Stable artifact-bundle schema identity.
    #[must_use]
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    /// Path-sorted artifacts.
    #[must_use]
    pub fn artifacts(&self) -> &[Artifact] {
        &self.artifacts
    }

    pub(crate) fn revalidate(&self) -> Result<(), PublicationDiagnostic> {
        if self.schema_version != BUNDLE_SCHEMA {
            return Err(publication_diagnostic(
                PublicationErrorCode::InvalidBundle,
                "bundle.schemaVersion",
                "the artifact bundle schema version is unsupported",
            ));
        }
        validate_artifacts(&self.artifacts)?;
        if self
            .artifacts
            .windows(2)
            .any(|pair| pair[0].path >= pair[1].path)
        {
            return Err(publication_diagnostic(
                PublicationErrorCode::InvalidBundle,
                "bundle.artifacts",
                "the artifact bundle is not sorted by path",
            ));
        }
        Ok(())
    }
}

fn validate_artifacts(artifacts: &[Artifact]) -> Result<(), PublicationDiagnostic> {
    if artifacts.is_empty() || artifacts.len() > MAX_ARTIFACTS {
        return Err(publication_diagnostic(
            PublicationErrorCode::InvalidBundle,
            "bundle.artifacts",
            "a bundle must contain between one and 4096 artifacts",
        ));
    }
    let mut paths = BTreeSet::new();
    let mut total = 0usize;
    for (index, artifact) in artifacts.iter().enumerate() {
        validate_path(&artifact.path, index)?;
        if !paths.insert(artifact.path.as_str()) {
            return Err(publication_diagnostic(
                PublicationErrorCode::DuplicateArtifactPath,
                &format!("bundle.artifacts[{index}].path"),
                "artifact paths must be unique",
            ));
        }
        if artifact.contents.len() > MAX_ARTIFACT_BYTES {
            return Err(publication_diagnostic(
                PublicationErrorCode::InvalidBundle,
                &format!("bundle.artifacts[{index}].contents"),
                "one artifact exceeds the bounded size",
            ));
        }
        total = total.saturating_add(artifact.contents.len());
    }
    if total > MAX_BUNDLE_BYTES {
        return Err(publication_diagnostic(
            PublicationErrorCode::InvalidBundle,
            "bundle.artifacts",
            "the complete bundle exceeds the bounded size",
        ));
    }
    Ok(())
}

fn validate_path(path: &str, index: usize) -> Result<(), PublicationDiagnostic> {
    let parsed = Path::new(path);
    let valid = !path.is_empty()
        && !path.contains('\\')
        && !path.ends_with('/')
        && path
            .split('/')
            .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
        && !path.chars().any(char::is_control)
        && parsed
            .components()
            .all(|component| matches!(component, Component::Normal(_)));
    if valid {
        Ok(())
    } else {
        Err(publication_diagnostic(
            PublicationErrorCode::UnsafeArtifactPath,
            &format!("bundle.artifacts[{index}].path"),
            "artifact paths must be canonical relative paths inside the generated boundary",
        ))
    }
}

pub(crate) fn publication_diagnostic(
    code: PublicationErrorCode,
    path: &str,
    message: &str,
) -> PublicationDiagnostic {
    PublicationDiagnostic {
        code,
        terminal_state: code.terminal_state(),
        destination_state: PublicationDestinationState::Unchanged,
        path: path.to_owned(),
        message: message.to_owned(),
    }
}

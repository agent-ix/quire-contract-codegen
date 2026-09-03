//! Validated, rollback-protected publication of generated artifact bundles.

use std::{
    collections::BTreeSet,
    fmt::Write as _,
    fs::{self, OpenOptions},
    io::Write as _,
    path::{Component, Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::{Artifact, GenerationTerminalState};

const MARKER_NAME: &str = ".quire-codegen-owned.json";
const MARKER_SCHEMA: &str = "quire.codegen-owned-bundle/v1";
const MAX_ARTIFACTS: usize = 4096;
const MAX_ARTIFACT_BYTES: usize = 16 * 1024 * 1024;
const MAX_BUNDLE_BYTES: usize = 128 * 1024 * 1024;
static PUBLICATION_NONCE: AtomicU64 = AtomicU64::new(0);

/// Stable reason a bundle could not be validated or published.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationErrorCode {
    /// The bundle contains no artifacts or exceeds a bounded resource limit.
    InvalidBundle,
    /// An artifact path is absolute, non-canonical, reserved, or traverses a parent.
    UnsafeArtifactPath,
    /// Two artifacts claim the same bundle-relative path.
    DuplicateArtifactPath,
    /// An artifact digest does not match its exact bytes.
    ArtifactDigestMismatch,
    /// The existing destination is not a completely verified generator-owned boundary.
    DestinationNotOwned,
    /// Staging, swapping, rollback, or cleanup encountered an I/O error.
    IoFailed,
}

impl PublicationErrorCode {
    /// Maps the publication failure onto the codegen terminal-state vocabulary.
    #[must_use]
    pub const fn terminal_state(self) -> GenerationTerminalState {
        match self {
            Self::InvalidBundle
            | Self::UnsafeArtifactPath
            | Self::DuplicateArtifactPath
            | Self::ArtifactDigestMismatch
            | Self::DestinationNotOwned => GenerationTerminalState::InvalidInput,
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
    bundle_sha256: String,
}

impl ArtifactBundle {
    /// Validates, path-sorts, and identifies a complete artifact set.
    pub fn new(mut artifacts: Vec<Artifact>) -> Result<Self, PublicationDiagnostic> {
        validate_artifacts(&artifacts)?;
        artifacts.sort_by(|left, right| left.path.cmp(&right.path));
        let bundle_sha256 = bundle_digest(&artifacts);
        Ok(Self {
            schema_version: "quire.artifact-bundle/v1".to_owned(),
            artifacts,
            bundle_sha256,
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

    /// Lowercase SHA-256 over every length-delimited path and content digest.
    #[must_use]
    pub fn bundle_sha256(&self) -> &str {
        &self.bundle_sha256
    }

    fn revalidate(&self) -> Result<(), PublicationDiagnostic> {
        if self.schema_version != "quire.artifact-bundle/v1" {
            return Err(diagnostic(
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
            || self.bundle_sha256 != bundle_digest(&self.artifacts)
        {
            return Err(diagnostic(
                PublicationErrorCode::InvalidBundle,
                "bundle.bundleSha256",
                "the artifact bundle is not canonically sorted and identified",
            ));
        }
        Ok(())
    }
}

/// Identity of a successfully published bundle.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PublishedBundleIdentity {
    /// Destination directory supplied by the caller.
    pub destination: String,
    /// Published bundle digest.
    pub bundle_sha256: String,
    /// Number of published artifacts, excluding the ownership marker.
    pub artifact_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct OwnershipMarker {
    schema_version: String,
    bundle_sha256: String,
    artifacts: Vec<PublishedArtifact>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct PublishedArtifact {
    path: String,
    sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PublicationFault {
    None,
    BeforeArtifact(usize),
    BeforeMarker,
    BeforeSwap,
    DuringSwap,
}

/// Publishes a complete bundle without editing any file outside its destination boundary.
///
/// Callers must serialize publishers and other writers to the destination and its generated sibling
/// names for the duration of this call.
// Implements: FR-005, NFR-001
pub fn write_bundle_atomic(
    bundle: &ArtifactBundle,
    destination: &Path,
) -> Result<PublishedBundleIdentity, PublicationDiagnostic> {
    publish(bundle, destination, PublicationFault::None)
}

fn publish(
    bundle: &ArtifactBundle,
    destination: &Path,
    fault: PublicationFault,
) -> Result<PublishedBundleIdentity, PublicationDiagnostic> {
    bundle.revalidate()?;
    let parent = destination.parent().ok_or_else(|| {
        diagnostic(
            PublicationErrorCode::InvalidBundle,
            "destination",
            "the destination must have an existing parent directory",
        )
    })?;
    let name = destination
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            diagnostic(
                PublicationErrorCode::InvalidBundle,
                "destination",
                "the destination must have a UTF-8 final component",
            )
        })?;
    if name.is_empty() || !parent.is_dir() {
        return Err(diagnostic(
            PublicationErrorCode::InvalidBundle,
            "destination",
            "the destination must have an existing directory parent",
        ));
    }
    let replacing = match fs::symlink_metadata(destination) {
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => {
            return Err(io_diagnostic(destination, "inspect destination", &error));
        }
    };
    if replacing {
        verify_owned_destination(destination)?;
    }

    let staging = unique_sibling(parent, name, "stage")?;
    let backup = unique_sibling(parent, name, "backup")?;
    fs::create_dir(&staging).map_err(|error| io_diagnostic(&staging, "create staging", &error))?;
    let stage_result = stage_bundle(bundle, &staging, fault);
    if let Err(error) = stage_result {
        cleanup(&staging, "clean failed staging")?;
        return Err(error);
    }
    if fault == PublicationFault::BeforeSwap {
        cleanup(&staging, "clean staged bundle before swap")?;
        return Err(injected(&staging, "before destination swap"));
    }

    if replacing {
        if let Err(error) = fs::rename(destination, &backup) {
            cleanup(&staging, "clean staging after old-bundle move failure")?;
            return Err(io_diagnostic(destination, "move old bundle", &error));
        }
        let replacement = if fault == PublicationFault::DuringSwap {
            Err(injected(destination, "during destination swap"))
        } else {
            fs::rename(&staging, destination)
                .map_err(|error| io_diagnostic(destination, "publish staged bundle", &error))
        };
        if let Err(error) = replacement {
            let rollback = fs::rename(&backup, destination);
            if let Err(rollback_error) = rollback {
                return Err(io_diagnostic(
                    destination,
                    "restore old bundle after failed swap",
                    &rollback_error,
                ));
            }
            cleanup(&staging, "clean staging after failed swap")?;
            return Err(error);
        }
        cleanup(&backup, "clean replaced bundle backup")?;
    } else {
        if fault == PublicationFault::DuringSwap {
            cleanup(&staging, "clean staging after injected swap failure")?;
            return Err(injected(destination, "during destination swap"));
        }
        if let Err(error) = fs::rename(&staging, destination) {
            cleanup(&staging, "clean staging after publication failure")?;
            return Err(io_diagnostic(destination, "publish staged bundle", &error));
        }
    }

    Ok(PublishedBundleIdentity {
        destination: destination.to_string_lossy().into_owned(),
        bundle_sha256: bundle.bundle_sha256.clone(),
        artifact_count: bundle.artifacts.len(),
    })
}

fn validate_artifacts(artifacts: &[Artifact]) -> Result<(), PublicationDiagnostic> {
    if artifacts.is_empty() || artifacts.len() > MAX_ARTIFACTS {
        return Err(diagnostic(
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
            return Err(diagnostic(
                PublicationErrorCode::DuplicateArtifactPath,
                &format!("bundle.artifacts[{index}].path"),
                "artifact paths must be unique",
            ));
        }
        if artifact.contents.len() > MAX_ARTIFACT_BYTES {
            return Err(diagnostic(
                PublicationErrorCode::InvalidBundle,
                &format!("bundle.artifacts[{index}].contents"),
                "one artifact exceeds the bounded size",
            ));
        }
        total = total.saturating_add(artifact.contents.len());
        if artifact.sha256 != sha256(artifact.contents.as_bytes()) {
            return Err(diagnostic(
                PublicationErrorCode::ArtifactDigestMismatch,
                &format!("bundle.artifacts[{index}].sha256"),
                "the artifact digest does not match its exact contents",
            ));
        }
    }
    if total > MAX_BUNDLE_BYTES {
        return Err(diagnostic(
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
        && path.split('/').all(|segment| !segment.is_empty())
        && path != MARKER_NAME
        && !path.chars().any(char::is_control)
        && parsed
            .components()
            .all(|component| matches!(component, Component::Normal(_)));
    if valid {
        Ok(())
    } else {
        Err(diagnostic(
            PublicationErrorCode::UnsafeArtifactPath,
            &format!("bundle.artifacts[{index}].path"),
            "artifact paths must be canonical relative paths inside the generated boundary",
        ))
    }
}

fn bundle_digest(artifacts: &[Artifact]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"quire.artifact-bundle/v1\0");
    for artifact in artifacts {
        for value in [&artifact.path, &artifact.sha256] {
            let length = u64::try_from(value.len()).expect("bounded artifact identity length");
            hasher.update(length.to_be_bytes());
            hasher.update(value.as_bytes());
        }
    }
    hex(hasher.finalize())
}

fn marker(bundle: &ArtifactBundle) -> OwnershipMarker {
    OwnershipMarker {
        schema_version: MARKER_SCHEMA.to_owned(),
        bundle_sha256: bundle.bundle_sha256.clone(),
        artifacts: bundle
            .artifacts
            .iter()
            .map(|artifact| PublishedArtifact {
                path: artifact.path.clone(),
                sha256: artifact.sha256.clone(),
            })
            .collect(),
    }
}

fn stage_bundle(
    bundle: &ArtifactBundle,
    staging: &Path,
    fault: PublicationFault,
) -> Result<(), PublicationDiagnostic> {
    for (index, artifact) in bundle.artifacts.iter().enumerate() {
        if fault == PublicationFault::BeforeArtifact(index) {
            return Err(injected(staging, "during staged artifact writes"));
        }
        let path = staging.join(&artifact.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| io_diagnostic(parent, "create artifact directory", &error))?;
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|error| io_diagnostic(&path, "create artifact", &error))?;
        file.write_all(artifact.contents.as_bytes())
            .and_then(|()| file.sync_all())
            .map_err(|error| io_diagnostic(&path, "write artifact", &error))?;
    }
    if fault == PublicationFault::BeforeMarker {
        return Err(injected(staging, "during staged ownership-marker write"));
    }
    let marker = deterministic_json(&marker(bundle)).map_err(|message| {
        diagnostic(
            PublicationErrorCode::InvalidBundle,
            "bundle.marker",
            &message,
        )
    })?;
    let marker_path = staging.join(MARKER_NAME);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&marker_path)
        .map_err(|error| io_diagnostic(&marker_path, "create ownership marker", &error))?;
    file.write_all(marker.as_bytes())
        .and_then(|()| file.sync_all())
        .map_err(|error| io_diagnostic(&marker_path, "write ownership marker", &error))?;
    Ok(())
}

fn verify_owned_destination(destination: &Path) -> Result<(), PublicationDiagnostic> {
    if !destination.is_dir()
        || destination
            .symlink_metadata()
            .map_or(true, |value| value.file_type().is_symlink())
    {
        return Err(not_owned(
            destination,
            "destination is not a regular directory boundary",
        ));
    }
    let marker_path = destination.join(MARKER_NAME);
    let bytes = fs::read(&marker_path)
        .map_err(|_| not_owned(destination, "ownership marker is absent or unreadable"))?;
    if bytes.len() > MAX_ARTIFACT_BYTES {
        return Err(not_owned(
            destination,
            "ownership marker exceeds the bounded size",
        ));
    }
    let marker: OwnershipMarker = serde_json::from_slice(&bytes)
        .map_err(|_| not_owned(destination, "ownership marker is malformed"))?;
    if marker.schema_version != MARKER_SCHEMA
        || marker.artifacts.is_empty()
        || marker.artifacts.len() > MAX_ARTIFACTS
    {
        return Err(not_owned(
            destination,
            "ownership marker identity is invalid",
        ));
    }
    let mut expected = BTreeSet::from([MARKER_NAME.to_owned()]);
    let mut marker_artifacts = Vec::with_capacity(marker.artifacts.len());
    for (index, artifact) in marker.artifacts.iter().enumerate() {
        validate_path(&artifact.path, index)
            .map_err(|_| not_owned(destination, "ownership marker contains an unsafe path"))?;
        if !expected.insert(artifact.path.clone()) {
            return Err(not_owned(
                destination,
                "ownership marker contains duplicate paths",
            ));
        }
        let mut parent_path = PathBuf::new();
        if let Some(parent) = Path::new(&artifact.path).parent() {
            for component in parent.components() {
                parent_path.push(component.as_os_str());
                expected.insert(format!(
                    "{}/",
                    parent_path.to_string_lossy().replace('\\', "/")
                ));
            }
        }
        let path = destination.join(&artifact.path);
        let metadata = path
            .symlink_metadata()
            .map_err(|_| not_owned(destination, "a marked artifact is absent"))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(not_owned(
                destination,
                "a marked artifact is not a regular file",
            ));
        }
        let contents = fs::read(&path)
            .map_err(|_| not_owned(destination, "a marked artifact is unreadable"))?;
        if sha256(&contents) != artifact.sha256 {
            return Err(not_owned(destination, "a marked artifact digest changed"));
        }
        marker_artifacts.push(Artifact {
            path: artifact.path.clone(),
            sha256: artifact.sha256.clone(),
            contents: String::new(),
        });
    }
    if marker.bundle_sha256 != bundle_digest(&marker_artifacts) {
        return Err(not_owned(
            destination,
            "ownership marker bundle digest is inconsistent",
        ));
    }
    let mut observed = BTreeSet::new();
    collect_entries(destination, destination, &mut observed)?;
    if observed != expected {
        return Err(not_owned(
            destination,
            "destination contains unmarked or missing files",
        ));
    }
    Ok(())
}

fn collect_entries(
    root: &Path,
    directory: &Path,
    observed: &mut BTreeSet<String>,
) -> Result<(), PublicationDiagnostic> {
    for entry in fs::read_dir(directory)
        .map_err(|error| io_diagnostic(directory, "read owned destination", &error))?
    {
        let entry =
            entry.map_err(|error| io_diagnostic(directory, "read destination entry", &error))?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| io_diagnostic(&path, "read destination metadata", &error))?;
        if metadata.file_type().is_symlink() {
            return Err(not_owned(root, "destination contains a symbolic link"));
        }
        if metadata.is_dir() {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| not_owned(root, "destination entry escaped its boundary"))?;
            observed.insert(format!(
                "{}/",
                relative.to_string_lossy().replace('\\', "/")
            ));
            collect_entries(root, &path, observed)?;
        } else if metadata.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| not_owned(root, "destination entry escaped its boundary"))?;
            observed.insert(relative.to_string_lossy().replace('\\', "/"));
        } else {
            return Err(not_owned(root, "destination contains a non-file entry"));
        }
    }
    Ok(())
}

fn unique_sibling(parent: &Path, name: &str, role: &str) -> Result<PathBuf, PublicationDiagnostic> {
    for _ in 0..1024 {
        let nonce = PUBLICATION_NONCE.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(
            ".{name}.quire-{role}-{}-{nonce}",
            std::process::id()
        ));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err(diagnostic(
        PublicationErrorCode::IoFailed,
        "destination",
        "no unused staging name was available",
    ))
}

fn deterministic_json(value: &impl Serialize) -> Result<String, String> {
    let mut bytes = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    String::from_utf8(bytes).map_err(|error| error.to_string())
}

fn sha256(bytes: &[u8]) -> String {
    hex(Sha256::digest(bytes))
}

fn hex(bytes: impl AsRef<[u8]>) -> String {
    let bytes = bytes.as_ref();
    let mut value = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(value, "{byte:02x}");
    }
    value
}

fn cleanup(path: &Path, action: &str) -> Result<(), PublicationDiagnostic> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(io_diagnostic(path, action, &error)),
    };
    let result = if metadata.is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    };
    result.map_err(|error| io_diagnostic(path, action, &error))
}

fn diagnostic(code: PublicationErrorCode, path: &str, message: &str) -> PublicationDiagnostic {
    PublicationDiagnostic {
        code,
        terminal_state: code.terminal_state(),
        path: path.to_owned(),
        message: message.to_owned(),
    }
}

fn io_diagnostic(path: &Path, action: &str, error: &std::io::Error) -> PublicationDiagnostic {
    diagnostic(
        PublicationErrorCode::IoFailed,
        &path.to_string_lossy(),
        &format!("could not {action}: {error}"),
    )
}

fn not_owned(path: &Path, message: &str) -> PublicationDiagnostic {
    diagnostic(
        PublicationErrorCode::DestinationNotOwned,
        &path.to_string_lossy(),
        message,
    )
}

fn injected(path: &Path, point: &str) -> PublicationDiagnostic {
    diagnostic(
        PublicationErrorCode::IoFailed,
        &path.to_string_lossy(),
        &format!("injected publication failure {point}"),
    )
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    fn temporary(name: &str) -> PathBuf {
        let nonce = PUBLICATION_NONCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "quire-publication-{name}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn generated(path: &str, contents: &str) -> Artifact {
        Artifact {
            path: path.to_owned(),
            contents: contents.to_owned(),
            sha256: sha256(contents.as_bytes()),
        }
    }

    fn bundle(version: &str) -> ArtifactBundle {
        ArtifactBundle::new(vec![
            generated("src/generated.rs", version),
            generated("attestations/generated.json", "{}\n"),
        ])
        .unwrap()
    }

    fn residue(parent: &Path) -> Vec<String> {
        fs::read_dir(parent)
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|name| name.contains(".quire-stage-") || name.contains(".quire-backup-"))
            .collect()
    }

    /// Trace: TC-001, TC-002, FR-005-AC-1, FR-005-AC-2, NFR-001-AC-1, NFR-001-AC-2, NFR-001-AC-3
    #[test]
    fn publication_is_order_independent_and_replaces_only_owned_boundaries() {
        let parent = temporary("replace");
        let destination = parent.join("generated");
        let developer = parent.join("developer.rs");
        fs::write(&developer, "developer-owned\n").unwrap();
        let first = bundle("first\n");
        let reversed =
            ArtifactBundle::new(first.artifacts.iter().cloned().rev().collect()).unwrap();
        assert_eq!(first, reversed);
        let identity = write_bundle_atomic(&first, &destination).unwrap();
        assert_eq!(identity.bundle_sha256, first.bundle_sha256());
        assert_eq!(
            fs::read_to_string(destination.join("src/generated.rs")).unwrap(),
            "first\n"
        );
        write_bundle_atomic(&bundle("second\n"), &destination).unwrap();
        assert_eq!(
            fs::read_to_string(destination.join("src/generated.rs")).unwrap(),
            "second\n"
        );
        assert_eq!(fs::read_to_string(&developer).unwrap(), "developer-owned\n");
        assert!(residue(&parent).is_empty());
        fs::remove_dir_all(parent).unwrap();
    }

    /// Trace: TC-002, FR-005-AC-1, NFR-001-AC-2, NFR-001-AC-3
    #[test]
    fn every_injected_failure_preserves_old_and_developer_owned_bytes() {
        let faults = (0..bundle("new\n").artifacts().len())
            .map(PublicationFault::BeforeArtifact)
            .chain([
                PublicationFault::BeforeMarker,
                PublicationFault::BeforeSwap,
                PublicationFault::DuringSwap,
            ]);
        for fault in faults {
            let parent = temporary("rollback");
            let destination = parent.join("generated");
            let developer = parent.join("developer.rs");
            fs::write(&developer, "developer-owned\n").unwrap();
            write_bundle_atomic(&bundle("old\n"), &destination).unwrap();
            let error = publish(&bundle("new\n"), &destination, fault).unwrap_err();
            assert_eq!(error.code, PublicationErrorCode::IoFailed);
            assert_eq!(
                fs::read_to_string(destination.join("src/generated.rs")).unwrap(),
                "old\n"
            );
            assert_eq!(fs::read_to_string(&developer).unwrap(), "developer-owned\n");
            assert!(residue(&parent).is_empty());
            fs::remove_dir_all(parent).unwrap();
        }
    }

    /// Trace: TC-002, FR-005-AC-1, NFR-001-AC-3
    #[test]
    fn unsafe_bundles_and_unowned_or_modified_destinations_are_refused() {
        for path in [
            "../escape",
            "/absolute",
            "./alias",
            "nested//alias",
            "trailing/",
            "nested\\windows",
            MARKER_NAME,
        ] {
            let error = ArtifactBundle::new(vec![generated(path, "x")]).unwrap_err();
            assert_eq!(
                error.code,
                PublicationErrorCode::UnsafeArtifactPath,
                "{path}"
            );
        }
        let mut wrong_digest = generated("safe", "x");
        wrong_digest.sha256 = "0".repeat(64);
        assert_eq!(
            ArtifactBundle::new(vec![wrong_digest]).unwrap_err().code,
            PublicationErrorCode::ArtifactDigestMismatch
        );

        let parent = temporary("ownership");
        let destination = parent.join("generated");
        fs::create_dir(&destination).unwrap();
        fs::write(destination.join("developer.rs"), "owned elsewhere\n").unwrap();
        let error = write_bundle_atomic(&bundle("new\n"), &destination).unwrap_err();
        assert_eq!(error.code, PublicationErrorCode::DestinationNotOwned);
        assert_eq!(
            fs::read_to_string(destination.join("developer.rs")).unwrap(),
            "owned elsewhere\n"
        );

        fs::remove_dir_all(&destination).unwrap();
        write_bundle_atomic(&bundle("old\n"), &destination).unwrap();
        fs::write(
            destination.join("src/generated.rs"),
            "developer changed this\n",
        )
        .unwrap();
        let error = write_bundle_atomic(&bundle("new\n"), &destination).unwrap_err();
        assert_eq!(error.code, PublicationErrorCode::DestinationNotOwned);
        assert_eq!(
            fs::read_to_string(destination.join("src/generated.rs")).unwrap(),
            "developer changed this\n"
        );

        fs::remove_dir_all(&destination).unwrap();
        write_bundle_atomic(&bundle("old\n"), &destination).unwrap();
        fs::write(destination.join("extra.rs"), "extra\n").unwrap();
        let error = write_bundle_atomic(&bundle("new\n"), &destination).unwrap_err();
        assert_eq!(error.code, PublicationErrorCode::DestinationNotOwned);

        fs::remove_file(destination.join("extra.rs")).unwrap();
        fs::create_dir(destination.join("empty-developer-directory")).unwrap();
        let error = write_bundle_atomic(&bundle("new\n"), &destination).unwrap_err();
        assert_eq!(error.code, PublicationErrorCode::DestinationNotOwned);

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;

            fs::remove_dir_all(&destination).unwrap();
            write_bundle_atomic(&bundle("old\n"), &destination).unwrap();
            let external = parent.join("external.rs");
            fs::write(&external, "old\n").unwrap();
            fs::remove_file(destination.join("src/generated.rs")).unwrap();
            symlink(&external, destination.join("src/generated.rs")).unwrap();
            let error = write_bundle_atomic(&bundle("new\n"), &destination).unwrap_err();
            assert_eq!(error.code, PublicationErrorCode::DestinationNotOwned);
            assert_eq!(fs::read_to_string(&external).unwrap(), "old\n");

            fs::remove_dir_all(&destination).unwrap();
            fs::remove_file(&external).unwrap();
            symlink(&external, &destination).unwrap();
            let error = write_bundle_atomic(&bundle("new\n"), &destination).unwrap_err();
            assert_eq!(error.code, PublicationErrorCode::DestinationNotOwned);
            assert!(fs::symlink_metadata(&destination)
                .unwrap()
                .file_type()
                .is_symlink());
            assert!(residue(&parent).is_empty());
        }
        fs::remove_dir_all(parent).unwrap();
    }

    /// Trace: TC-002, FR-005-AC-1, NFR-001-AC-2, NFR-001-AC-3
    #[cfg(unix)]
    #[test]
    fn cleanup_removes_a_sibling_symlink_without_following_it() {
        use std::os::unix::fs::symlink;

        let parent = temporary("cleanup-symlink");
        let external = parent.join("external");
        let sibling = parent.join(".generated.quire-stage-raced");
        fs::create_dir(&external).unwrap();
        fs::write(external.join("developer.rs"), "developer-owned\n").unwrap();
        symlink(&external, &sibling).unwrap();

        cleanup(&sibling, "clean raced sibling").unwrap();

        assert!(!sibling.exists());
        assert_eq!(
            fs::read_to_string(external.join("developer.rs")).unwrap(),
            "developer-owned\n"
        );
        fs::remove_dir_all(parent).unwrap();
    }
}

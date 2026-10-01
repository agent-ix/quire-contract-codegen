//! Validated, rollback-protected publication of generated artifact bundles.

use std::{
    fs::{self, OpenOptions},
    io::Write as _,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use serde::{Deserialize, Serialize};

use crate::artifact::{
    diagnostic, ArtifactBundle, PublicationDestinationState, PublicationDiagnostic,
    PublicationErrorCode,
};

static PUBLICATION_NONCE: AtomicU64 = AtomicU64::new(0);

/// A successfully published bundle.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PublishedBundleIdentity {
    /// Destination directory supplied by the caller.
    pub destination: String,
    /// Number of published artifacts.
    pub artifact_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PublicationFault {
    None,
    BeforeArtifact(usize),
    BeforeSwap,
    DuringSwap,
    DuringRollback,
    AfterCommitBeforeBackupCleanup,
}

/// Publishes a complete bundle, replacing the destination directory as one unit, without editing
/// any file outside its destination boundary.
///
/// Callers must serialize publishers and other writers to the destination and its generated sibling
/// names for the duration of this call. An existing destination is replaced whole.
/// The rollback guarantee does not cover process crashes or power-loss durability.
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
        let replacement = if matches!(
            fault,
            PublicationFault::DuringSwap | PublicationFault::DuringRollback
        ) {
            Err(injected(destination, "during destination swap"))
        } else {
            fs::rename(&staging, destination)
                .map_err(|error| io_diagnostic(destination, "publish staged bundle", &error))
        };
        if let Err(error) = replacement {
            let rollback = if fault == PublicationFault::DuringRollback {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "injected rollback failure",
                ))
            } else {
                fs::rename(&backup, destination)
            };
            if let Err(rollback_error) = rollback {
                return Err(io_diagnostic_with_state(
                    destination,
                    "restore old bundle after failed swap",
                    &rollback_error,
                    PublicationDestinationState::Unknown,
                ));
            }
            cleanup(&staging, "clean staging after failed swap")?;
            return Err(error);
        }
        if fault == PublicationFault::AfterCommitBeforeBackupCleanup {
            return Err(injected_with_state(
                &backup,
                "after commit before backup cleanup",
                PublicationDestinationState::Published,
            ));
        }
        if let Err(mut error) = cleanup(&backup, "clean replaced bundle backup") {
            error.destination_state = PublicationDestinationState::Published;
            return Err(error);
        }
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
        artifact_count: bundle.artifacts().len(),
    })
}

fn stage_bundle(
    bundle: &ArtifactBundle,
    staging: &Path,
    fault: PublicationFault,
) -> Result<(), PublicationDiagnostic> {
    for (index, artifact) in bundle.artifacts().iter().enumerate() {
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

fn io_diagnostic(path: &Path, action: &str, error: &std::io::Error) -> PublicationDiagnostic {
    diagnostic(
        PublicationErrorCode::IoFailed,
        &path.to_string_lossy(),
        &format!("could not {action}: {error}"),
    )
}

fn io_diagnostic_with_state(
    path: &Path,
    action: &str,
    error: &std::io::Error,
    destination_state: PublicationDestinationState,
) -> PublicationDiagnostic {
    let mut diagnostic = io_diagnostic(path, action, error);
    diagnostic.destination_state = destination_state;
    diagnostic
}

fn injected(path: &Path, point: &str) -> PublicationDiagnostic {
    diagnostic(
        PublicationErrorCode::IoFailed,
        &path.to_string_lossy(),
        &format!("injected publication failure {point}"),
    )
}

fn injected_with_state(
    path: &Path,
    point: &str,
    destination_state: PublicationDestinationState,
) -> PublicationDiagnostic {
    let mut diagnostic = injected(path, point);
    diagnostic.destination_state = destination_state;
    diagnostic
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;
    use crate::{
        artifact::{Artifact, MAX_ARTIFACTS, MAX_ARTIFACT_BYTES, MAX_BUNDLE_BYTES},
        diagnostic::GenerationTerminalState,
    };

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
        Artifact::new(path, contents)
    }

    fn bundle(version: &str) -> ArtifactBundle {
        ArtifactBundle::new(vec![
            generated("src/generated.rs", version),
            generated("src/generated/nested.rs", "{}\n"),
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
    fn publication_is_order_independent_and_replaces_only_its_destination() {
        let parent = temporary("replace");
        let destination = parent.join("generated");
        let developer = parent.join("developer.rs");
        fs::write(&developer, "developer-owned\n").unwrap();
        let first = bundle("first\n");
        let reversed =
            ArtifactBundle::new(first.artifacts().iter().cloned().rev().collect()).unwrap();
        assert_eq!(first, reversed);
        let identity = write_bundle_atomic(&first, &destination).unwrap();
        assert_eq!(identity.artifact_count, 2);
        assert_eq!(
            fs::read_to_string(destination.join("src/generated.rs")).unwrap(),
            "first\n"
        );
        fs::write(destination.join("stale.rs"), "stale\n").unwrap();
        write_bundle_atomic(&bundle("second\n"), &destination).unwrap();
        assert_eq!(
            fs::read_to_string(destination.join("src/generated.rs")).unwrap(),
            "second\n"
        );
        assert!(
            !destination.join("stale.rs").exists(),
            "the destination is replaced whole"
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
            .chain([PublicationFault::BeforeSwap, PublicationFault::DuringSwap]);
        for fault in faults {
            for replacing in [false, true] {
                let parent = temporary("rollback");
                let destination = parent.join("generated");
                let developer = parent.join("developer.rs");
                fs::write(&developer, "developer-owned\n").unwrap();
                if replacing {
                    write_bundle_atomic(&bundle("old\n"), &destination).unwrap();
                }
                let error = publish(&bundle("new\n"), &destination, fault).unwrap_err();
                assert_eq!(error.code, PublicationErrorCode::IoFailed);
                assert_eq!(
                    error.destination_state,
                    PublicationDestinationState::Unchanged
                );
                if replacing {
                    assert_eq!(
                        fs::read_to_string(destination.join("src/generated.rs")).unwrap(),
                        "old\n"
                    );
                } else {
                    assert!(!destination.exists());
                }
                assert_eq!(fs::read_to_string(&developer).unwrap(), "developer-owned\n");
                assert!(residue(&parent).is_empty());
                fs::remove_dir_all(parent).unwrap();
            }
        }
    }

    /// Trace: TC-002, FR-005-AC-1, NFR-001-AC-2, NFR-001-AC-3
    #[test]
    fn failed_rollback_reports_unknown_and_preserves_complete_recovery_bundles() {
        let parent = temporary("failed-rollback");
        let destination = parent.join("generated");
        let developer = parent.join("developer.rs");
        fs::write(&developer, "developer-owned\n").unwrap();
        let old = bundle("old\n");
        let new = bundle("new\n");
        write_bundle_atomic(&old, &destination).unwrap();

        let error = publish(&new, &destination, PublicationFault::DuringRollback).unwrap_err();
        assert_eq!(error.code, PublicationErrorCode::IoFailed);
        assert_eq!(error.terminal_state, GenerationTerminalState::IoFailed);
        assert_eq!(
            error.destination_state,
            PublicationDestinationState::Unknown
        );
        assert!(error
            .message
            .contains("restore old bundle after failed swap"));
        assert!(!destination.exists());
        assert_eq!(fs::read_to_string(&developer).unwrap(), "developer-owned\n");
        let recovery = residue(&parent);
        assert_eq!(recovery.len(), 2);
        for (role, expected) in [(".quire-backup-", old), (".quire-stage-", new)] {
            let path = parent.join(recovery.iter().find(|name| name.contains(role)).unwrap());
            for artifact in expected.artifacts() {
                assert_eq!(
                    fs::read_to_string(path.join(&artifact.path)).unwrap(),
                    artifact.contents
                );
            }
        }
        fs::remove_dir_all(parent).unwrap();
    }

    /// Trace: TC-002, FR-005-AC-1, NFR-001-AC-2, NFR-001-AC-3
    #[test]
    fn post_commit_cleanup_failure_reports_that_the_new_bundle_is_published() {
        let parent = temporary("post-commit-cleanup");
        let destination = parent.join("generated");
        let developer = parent.join("developer.rs");
        fs::write(&developer, "developer-owned\n").unwrap();
        write_bundle_atomic(&bundle("old\n"), &destination).unwrap();

        let error = publish(
            &bundle("new\n"),
            &destination,
            PublicationFault::AfterCommitBeforeBackupCleanup,
        )
        .unwrap_err();

        assert_eq!(error.code, PublicationErrorCode::IoFailed);
        assert_eq!(
            error.destination_state,
            PublicationDestinationState::Published
        );
        assert_eq!(
            fs::read_to_string(destination.join("src/generated.rs")).unwrap(),
            "new\n"
        );
        assert_eq!(fs::read_to_string(&developer).unwrap(), "developer-owned\n");
        let residue = residue(&parent);
        assert_eq!(residue.len(), 1);
        cleanup(&parent.join(&residue[0]), "test cleanup").unwrap();
        fs::remove_dir_all(parent).unwrap();
    }

    /// Trace: TC-002, FR-005-AC-1, NFR-001-AC-3
    #[test]
    fn unsafe_and_duplicate_artifact_paths_are_refused() {
        for path in [
            "../escape",
            "/absolute",
            "./alias",
            "a/./b",
            "a/b/.",
            "nested//alias",
            "trailing/",
            "nested\\windows",
        ] {
            let error = ArtifactBundle::new(vec![generated(path, "x")]).unwrap_err();
            assert_eq!(
                error.code,
                PublicationErrorCode::UnsafeArtifactPath,
                "{path}"
            );
        }
        let alias_pair =
            ArtifactBundle::new(vec![generated("a/b", "x"), generated("a/./b", "y")]).unwrap_err();
        assert_eq!(alias_pair.code, PublicationErrorCode::UnsafeArtifactPath);
        assert_eq!(alias_pair.path, "bundle.artifacts[1].path");
        let duplicate =
            ArtifactBundle::new(vec![generated("a/b", "x"), generated("a/b", "y")]).unwrap_err();
        assert_eq!(duplicate.code, PublicationErrorCode::DuplicateArtifactPath);
        assert_eq!(duplicate.path, "bundle.artifacts[1].path");
    }

    /// Trace: TC-002, FR-005-AC-5
    #[test]
    fn bundle_construction_refuses_more_than_the_bounded_artifact_count() {
        let artifacts: Vec<Artifact> = (0..=MAX_ARTIFACTS)
            .map(|index| generated(&format!("a/{index}.rs"), "x"))
            .collect();
        assert_eq!(artifacts.len(), MAX_ARTIFACTS + 1);
        let error = ArtifactBundle::new(artifacts).unwrap_err();
        assert_eq!(error.code, PublicationErrorCode::InvalidBundle);
        assert_eq!(error.path, "bundle.artifacts");
        assert!(error.message.contains("4096"));
    }

    /// Trace: TC-002, FR-005-AC-5
    #[test]
    fn bundle_construction_refuses_an_artifact_over_the_bounded_size() {
        let oversized = "x".repeat(MAX_ARTIFACT_BYTES + 1);
        let error =
            ArtifactBundle::new(vec![generated("src/oversized.rs", &oversized)]).unwrap_err();
        assert_eq!(error.code, PublicationErrorCode::InvalidBundle);
        assert_eq!(error.path, "bundle.artifacts[0].contents");
    }

    /// Trace: TC-002, FR-005-AC-5
    #[test]
    fn bundle_construction_refuses_a_complete_bundle_over_the_bounded_size() {
        let large = "x".repeat(MAX_ARTIFACT_BYTES);
        let mut artifacts: Vec<Artifact> = (0..(MAX_BUNDLE_BYTES / MAX_ARTIFACT_BYTES))
            .map(|index| generated(&format!("src/large_{index}.rs"), &large))
            .collect();
        artifacts.push(generated("src/extra.rs", "x"));
        let total: usize = artifacts
            .iter()
            .map(|artifact| artifact.contents.len())
            .sum();
        assert!(
            total > MAX_BUNDLE_BYTES,
            "fixture must exceed the bundle ceiling: {total}"
        );
        let error = ArtifactBundle::new(artifacts).unwrap_err();
        assert_eq!(error.code, PublicationErrorCode::InvalidBundle);
        assert_eq!(error.path, "bundle.artifacts");
        assert!(error.message.contains("complete bundle"));
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

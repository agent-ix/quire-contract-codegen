//! Pinned execution of one generated Kani obligation harness (FR-015).
//!
//! The pins a harness was generated against are re-measured from the installed
//! backend before anything runs: a launcher, driver, CBMC, toolchain or target
//! that differs from the harness identity is a typed refusal and no proof is
//! attempted. The run outcome is read from the backend's own output and is never
//! defaulted: a harness this module did not observe verifying is not `verified`.

use std::{
    env, fmt, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::{
    kani::{is_sha256, sha256},
    kani_obligations::{KaniObligationHarness, ObligationKind},
};

/// Execution evidence schema identity.
pub const KANI_EXECUTION_SCHEMA: &str = "quire.codegen.kani-execution/v1";

/// The installed-backend identity a harness is generated against and run under.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct KaniToolPins {
    /// `cargo-kani --version` without its program prefix, for example `0.67.0`.
    pub kani_version: String,
    /// Lowercase SHA-256 of the `cargo-kani` launcher executable that is invoked.
    pub launcher_sha256: String,
    /// Lowercase SHA-256 of the `kani-driver` executable the launcher dispatches to.
    pub driver_sha256: String,
    /// `cbmc --version` of the CBMC bundled with that Kani release.
    pub cbmc_version: String,
    /// Rust toolchain Kani compiles with, as recorded by the Kani release.
    pub rust_toolchain: String,
    /// Host target triple of that toolchain.
    pub target_triple: String,
}

/// One pinned field of [`KaniToolPins`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniPinField {
    /// [`KaniToolPins::kani_version`].
    KaniVersion,
    /// [`KaniToolPins::launcher_sha256`].
    LauncherSha256,
    /// [`KaniToolPins::driver_sha256`].
    DriverSha256,
    /// [`KaniToolPins::cbmc_version`].
    CbmcVersion,
    /// [`KaniToolPins::rust_toolchain`].
    RustToolchain,
    /// [`KaniToolPins::target_triple`].
    TargetTriple,
}

impl KaniToolPins {
    /// Every field with its value, in declaration order.
    #[must_use]
    pub fn fields(&self) -> [(KaniPinField, &str); 6] {
        [
            (KaniPinField::KaniVersion, &self.kani_version),
            (KaniPinField::LauncherSha256, &self.launcher_sha256),
            (KaniPinField::DriverSha256, &self.driver_sha256),
            (KaniPinField::CbmcVersion, &self.cbmc_version),
            (KaniPinField::RustToolchain, &self.rust_toolchain),
            (KaniPinField::TargetTriple, &self.target_triple),
        ]
    }

    /// The first field whose value is malformed: an executable digest that is not
    /// lowercase SHA-256, or any other field that is empty or has surrounding space.
    pub(crate) fn first_malformed(&self) -> Option<KaniPinField> {
        self.fields().into_iter().find_map(|(field, value)| {
            let valid = match field {
                KaniPinField::LauncherSha256 | KaniPinField::DriverSha256 => is_sha256(value),
                KaniPinField::KaniVersion
                | KaniPinField::CbmcVersion
                | KaniPinField::RustToolchain
                | KaniPinField::TargetTriple => {
                    !value.is_empty() && value.trim() == value && !value.contains('\n')
                }
            };
            (!valid).then_some(field)
        })
    }
}

/// A backend component this module measures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniTool {
    /// The `cargo-kani` launcher.
    Launcher,
    /// The `kani-driver` of the launcher's release.
    Driver,
    /// The bundled `cbmc`.
    Cbmc,
    /// The release's recorded Rust toolchain.
    RustToolchain,
    /// The release's `rustc`, asked for its host triple.
    Rustc,
    /// The generated crate's `Cargo.lock`, written by the run.
    Lockfile,
    /// The Kani home directory.
    KaniHome,
}

/// Why the installed backend could not be measured.
#[derive(Debug)]
pub enum KaniToolError {
    /// The component is absent at the path it was looked for.
    Missing {
        /// Component.
        tool: KaniTool,
        /// Path looked at.
        path: PathBuf,
    },
    /// The component exists but could not be read or started.
    Io {
        /// Component.
        tool: KaniTool,
        /// Path read or started.
        path: PathBuf,
        /// Underlying error.
        error: io::Error,
    },
    /// The component ran and exited unsuccessfully.
    Failed {
        /// Component.
        tool: KaniTool,
        /// Exit code, when the process was not killed by a signal.
        exit_code: Option<i32>,
    },
    /// The component's output was not the shape this module reads.
    UnexpectedOutput {
        /// Component.
        tool: KaniTool,
        /// Its trimmed output.
        output: String,
    },
}

impl fmt::Display for KaniToolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { tool, path } => {
                write!(formatter, "{tool:?} is absent at {}", path.display())
            }
            Self::Io { tool, path, error } => {
                write!(formatter, "{tool:?} at {}: {error}", path.display())
            }
            Self::Failed { tool, exit_code } => {
                write!(formatter, "{tool:?} exited unsuccessfully: {exit_code:?}")
            }
            Self::UnexpectedOutput { tool, output } => {
                write!(formatter, "{tool:?} printed unexpected output: {output}")
            }
        }
    }
}

impl std::error::Error for KaniToolError {}

/// Where the Kani launcher and its release live.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniInstallation {
    /// The `cargo-kani` executable that is invoked directly.
    pub launcher: PathBuf,
    /// The Kani home directory holding `kani-<version>/`.
    pub kani_home: PathBuf,
}

impl KaniInstallation {
    /// Locates the launcher the way Cargo resolves a subcommand — `$CARGO_HOME/bin`
    /// (default `$HOME/.cargo/bin`) first, then `PATH` — and the Kani home as
    /// `$KANI_HOME`, else `$HOME/.kani`.
    pub fn discover() -> Result<Self, KaniToolError> {
        let home = env::var_os("HOME").map(PathBuf::from);
        let launcher_name = format!("cargo-kani{}", env::consts::EXE_SUFFIX);
        let cargo_home = env::var_os("CARGO_HOME")
            .map(PathBuf::from)
            .or_else(|| home.as_ref().map(|home| home.join(".cargo")));
        let launcher = cargo_home
            .map(|directory| directory.join("bin").join(&launcher_name))
            .into_iter()
            .chain(
                env::var_os("PATH")
                    .map(|value| env::split_paths(&value).collect::<Vec<_>>())
                    .unwrap_or_default()
                    .into_iter()
                    .map(|directory| directory.join(&launcher_name)),
            )
            .find(|candidate| candidate.is_file())
            .ok_or_else(|| KaniToolError::Missing {
                tool: KaniTool::Launcher,
                path: PathBuf::from(&launcher_name),
            })?;
        let kani_home = env::var_os("KANI_HOME")
            .map(PathBuf::from)
            .or_else(|| home.map(|home| home.join(".kani")))
            .ok_or_else(|| KaniToolError::Missing {
                tool: KaniTool::KaniHome,
                path: PathBuf::from("$HOME/.kani"),
            })?;
        Ok(Self {
            launcher,
            kani_home,
        })
    }

    /// Measures every pin from the installed files and processes.
    pub fn observe(&self) -> Result<KaniToolPins, KaniToolError> {
        let version_output = run(KaniTool::Launcher, &self.launcher, &["kani", "--version"])?;
        let kani_version = version_output
            .strip_prefix("cargo-kani ")
            .filter(|version| !version.is_empty() && !version.contains(char::is_whitespace))
            .ok_or_else(|| KaniToolError::UnexpectedOutput {
                tool: KaniTool::Launcher,
                output: version_output.clone(),
            })?
            .to_owned();
        let release = self.kani_home.join(format!("kani-{kani_version}"));
        let driver = release
            .join("bin")
            .join(format!("kani-driver{}", env::consts::EXE_SUFFIX));
        let cbmc = release
            .join("bin")
            .join(format!("cbmc{}", env::consts::EXE_SUFFIX));
        let toolchain_file = release.join("rust-toolchain-version");
        let rustc = release
            .join("toolchain")
            .join("bin")
            .join(format!("rustc{}", env::consts::EXE_SUFFIX));
        let rust_toolchain = read_file(KaniTool::RustToolchain, &toolchain_file)
            .map(|bytes| String::from_utf8_lossy(&bytes).trim().to_owned())?;
        let rustc_output = run(KaniTool::Rustc, &rustc, &["-vV"])?;
        let target_triple = rustc_output
            .lines()
            .find_map(|line| line.strip_prefix("host: "))
            .map(str::to_owned)
            .ok_or_else(|| KaniToolError::UnexpectedOutput {
                tool: KaniTool::Rustc,
                output: rustc_output.clone(),
            })?;
        Ok(KaniToolPins {
            launcher_sha256: file_sha256(KaniTool::Launcher, &self.launcher)?,
            driver_sha256: file_sha256(KaniTool::Driver, &driver)?,
            cbmc_version: run(KaniTool::Cbmc, &cbmc, &["--version"])?,
            kani_version,
            rust_toolchain,
            target_triple,
        })
    }
}

/// One execution of one harness in a crate the caller wrote.
pub struct KaniExecutionRequest<'a> {
    /// Backend to measure and invoke.
    pub installation: &'a KaniInstallation,
    /// The generated harness; its identity carries the pins to hold the backend to.
    pub harness: &'a KaniObligationHarness,
    /// Crate root whose `src/lib.rs` contains the harness source byte-for-byte.
    pub crate_directory: &'a Path,
    /// Cargo target directory for the run.
    pub target_directory: &'a Path,
}

/// Why a harness was not run.
#[derive(Debug)]
pub enum KaniExecutionRefusal {
    /// The installed backend could not be measured.
    Tool(KaniToolError),
    /// An installed backend component differs from the harness pin.
    PinDrift {
        /// The differing field.
        field: KaniPinField,
        /// The harness pin.
        expected: String,
        /// The installed value.
        observed: String,
    },
    /// The crate's `src/lib.rs` does not contain the harness source.
    HarnessNotInCrate {
        /// The generated artifact path.
        harness_path: String,
    },
}

impl fmt::Display for KaniExecutionRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tool(error) => write!(formatter, "{error}"),
            Self::PinDrift {
                field,
                expected,
                observed,
            } => write!(
                formatter,
                "{field:?} drifted: pinned {expected:?}, installed {observed:?}"
            ),
            Self::HarnessNotInCrate { harness_path } => {
                write!(formatter, "the crate does not contain {harness_path}")
            }
        }
    }
}

impl std::error::Error for KaniExecutionRefusal {}

impl From<KaniToolError> for KaniExecutionRefusal {
    fn from(error: KaniToolError) -> Self {
        Self::Tool(error)
    }
}

/// Why a completed run proves nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniInconclusiveReason {
    /// Kani reported failure but printed no concrete playback.
    FailedWithoutCounterexample,
    /// Kani printed no verification verdict: a build, launcher or solver failure.
    NoVerdict,
    /// Kani reported success for a precondition harness without a cover summary.
    MissingCoverSummary,
}

/// The backend-reported outcome of one run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum KaniRunOutcome {
    /// Every property held; for a precondition harness, every cover was satisfied.
    Verified,
    /// A property failed and Kani printed a concrete counterexample.
    Falsified {
        /// The concrete-playback test Kani printed, verbatim.
        counterexample: String,
    },
    /// A precondition harness's cover was not satisfiable within the bounds: every
    /// obligation assuming that precondition holds vacuously.
    CoverUnsatisfied {
        /// Satisfied cover properties.
        satisfied: u64,
        /// Total cover properties.
        total: u64,
    },
    /// The run established nothing.
    Inconclusive {
        /// Why.
        reason: KaniInconclusiveReason,
    },
}

/// Pinned identity and backend-reported outcome of one harness run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KaniExecutionEvidence {
    /// [`KANI_EXECUTION_SCHEMA`].
    pub schema: &'static str,
    /// Identity digest of the harness that ran.
    pub obligation_identity_sha256: String,
    /// Obligation kind.
    pub kind: ObligationKind,
    /// Generated harness path.
    pub harness_path: String,
    /// Generated harness source digest.
    pub harness_sha256: String,
    /// Backend pins measured immediately before the run; equal to the harness pins.
    pub observed_pins: KaniToolPins,
    /// Invoked launcher path.
    pub launcher_path: String,
    /// Complete argument vector after the launcher.
    pub arguments: Vec<String>,
    /// SHA-256 of the generated crate's `Cargo.lock` after the run.
    pub cargo_lock_sha256: String,
    /// Digest of the oracle sources the harness embeds.
    pub oracle_digest: String,
    /// Contract Runtime revision the generated crate depends on.
    pub runtime_revision: String,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Solver.
    pub solver: String,
    /// Process exit code, when not killed by a signal.
    pub exit_code: Option<i32>,
    /// Backend-reported outcome.
    pub outcome: KaniRunOutcome,
}

/// Measures the backend, refuses on any pin drift, runs the harness, and reports
/// the backend's own outcome.
pub fn execute_kani_obligation(
    request: &KaniExecutionRequest<'_>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    let identity = &request.harness.identity;
    let observed = request.installation.observe()?;
    for ((field, expected), (_, installed)) in
        identity.pins.fields().into_iter().zip(observed.fields())
    {
        if expected != installed {
            return Err(KaniExecutionRefusal::PinDrift {
                field,
                expected: expected.to_owned(),
                observed: installed.to_owned(),
            });
        }
    }
    let library_path = request.crate_directory.join("src").join("lib.rs");
    let library = read_file(KaniTool::Lockfile, &library_path).map_err(|_| {
        KaniExecutionRefusal::HarnessNotInCrate {
            harness_path: request.harness.rust.path.clone(),
        }
    })?;
    if !String::from_utf8_lossy(&library).contains(&request.harness.rust.contents) {
        return Err(KaniExecutionRefusal::HarnessNotInCrate {
            harness_path: request.harness.rust.path.clone(),
        });
    }
    let mut arguments = vec!["kani".to_owned()];
    arguments.extend(identity.options.iter().cloned());
    let output = Command::new(&request.installation.launcher)
        .args(&arguments)
        .env("CARGO_TARGET_DIR", request.target_directory)
        .current_dir(request.crate_directory)
        .output()
        .map_err(|error| KaniToolError::Io {
            tool: KaniTool::Launcher,
            path: request.installation.launcher.clone(),
            error,
        })?;
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let cargo_lock_sha256 = file_sha256(
        KaniTool::Lockfile,
        &request.crate_directory.join("Cargo.lock"),
    )?;
    Ok(KaniExecutionEvidence {
        schema: KANI_EXECUTION_SCHEMA,
        obligation_identity_sha256: request.harness.identity_sha256.clone(),
        kind: identity.kind,
        harness_path: request.harness.rust.path.clone(),
        harness_sha256: request.harness.rust.sha256.clone(),
        observed_pins: observed,
        launcher_path: request.installation.launcher.display().to_string(),
        arguments,
        cargo_lock_sha256,
        oracle_digest: identity.oracle_digest.clone(),
        runtime_revision: identity.runtime_revision.to_owned(),
        unwind: identity.unwind,
        solver: identity.solver.clone(),
        exit_code: output.status.code(),
        outcome: classify_run(identity.kind, output.status.success(), &text),
    })
}

const SUCCESS: &str = "VERIFICATION:- SUCCESSFUL";
const FAILURE: &str = "VERIFICATION:- FAILED";

fn classify_run(kind: ObligationKind, exited_successfully: bool, text: &str) -> KaniRunOutcome {
    if exited_successfully && text.contains(SUCCESS) && !text.contains(FAILURE) {
        if kind != ObligationKind::Precondition {
            return KaniRunOutcome::Verified;
        }
        return match cover_summary(text) {
            Some((satisfied, total)) if total > 0 && satisfied == total => KaniRunOutcome::Verified,
            Some((satisfied, total)) => KaniRunOutcome::CoverUnsatisfied { satisfied, total },
            None => KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::MissingCoverSummary,
            },
        };
    }
    if text.contains(FAILURE) {
        return match concrete_playback(text) {
            Some(counterexample) => KaniRunOutcome::Falsified { counterexample },
            None => KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::FailedWithoutCounterexample,
            },
        };
    }
    KaniRunOutcome::Inconclusive {
        reason: KaniInconclusiveReason::NoVerdict,
    }
}

/// Reads Kani's `** <satisfied> of <total> cover properties satisfied` line.
fn cover_summary(text: &str) -> Option<(u64, u64)> {
    text.lines().find_map(|line| {
        let rest = line.trim().strip_prefix("** ")?;
        let (counts, tail) = rest.split_once(" cover properties satisfied")?;
        if !tail.is_empty() {
            return None;
        }
        let (satisfied, total) = counts.split_once(" of ")?;
        Some((satisfied.parse().ok()?, total.parse().ok()?))
    })
}

/// Extracts the fenced concrete-playback unit test Kani prints after a failure.
fn concrete_playback(text: &str) -> Option<String> {
    let start = text.find("Concrete playback unit test")?;
    let tail = &text[start..];
    let fence = tail.find("```")?;
    let body = &tail[fence + 3..];
    let end = body.find("```")?;
    let test = body[..end].trim();
    test.contains("kani::concrete_playback_run")
        .then(|| test.to_owned())
}

fn run(tool: KaniTool, program: &Path, arguments: &[&str]) -> Result<String, KaniToolError> {
    if !program.is_file() {
        return Err(KaniToolError::Missing {
            tool,
            path: program.to_path_buf(),
        });
    }
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| KaniToolError::Io {
            tool,
            path: program.to_path_buf(),
            error,
        })?;
    if !output.status.success() {
        return Err(KaniToolError::Failed {
            tool,
            exit_code: output.status.code(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn read_file(tool: KaniTool, path: &Path) -> Result<Vec<u8>, KaniToolError> {
    if !path.is_file() {
        return Err(KaniToolError::Missing {
            tool,
            path: path.to_path_buf(),
        });
    }
    fs::read(path).map_err(|error| KaniToolError::Io {
        tool,
        path: path.to_path_buf(),
        error,
    })
}

fn file_sha256(tool: KaniTool, path: &Path) -> Result<String, KaniToolError> {
    read_file(tool, path).map(|bytes| sha256(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Trace: FR-015-AC-2, TC-025.
    #[test]
    fn tc_025_run_classification_never_defaults_to_verified() {
        let playback = "Concrete playback unit test for `m::h`:\n```\n#[test]\nfn kani_concrete_playback_h() {\n    kani::concrete_playback_run(vec![], m::h);\n}\n```\n";
        assert_eq!(
            classify_run(
                ObligationKind::Postcondition,
                true,
                "VERIFICATION:- SUCCESSFUL"
            ),
            KaniRunOutcome::Verified
        );
        assert!(matches!(
            classify_run(
                ObligationKind::Postcondition,
                false,
                &format!("VERIFICATION:- FAILED\n{playback}")
            ),
            KaniRunOutcome::Falsified { counterexample } if counterexample.contains("concrete_playback_run")
        ));
        for (kind, success, text, expected) in [
            (
                ObligationKind::Postcondition,
                false,
                "VERIFICATION:- FAILED",
                KaniInconclusiveReason::FailedWithoutCounterexample,
            ),
            (
                ObligationKind::Postcondition,
                false,
                "error[E0308]: mismatched types",
                KaniInconclusiveReason::NoVerdict,
            ),
            // Success text from a process that exited unsuccessfully is not success.
            (
                ObligationKind::Invariant,
                false,
                "VERIFICATION:- SUCCESSFUL",
                KaniInconclusiveReason::NoVerdict,
            ),
            (
                ObligationKind::Precondition,
                true,
                "VERIFICATION:- SUCCESSFUL",
                KaniInconclusiveReason::MissingCoverSummary,
            ),
        ] {
            assert_eq!(
                classify_run(kind, success, text),
                KaniRunOutcome::Inconclusive { reason: expected },
                "{text}"
            );
        }
        assert_eq!(
            classify_run(
                ObligationKind::Precondition,
                true,
                "SUMMARY:\n ** 0 of 1 cover properties satisfied\n\nVERIFICATION:- SUCCESSFUL"
            ),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1
            }
        );
        assert_eq!(
            classify_run(
                ObligationKind::Precondition,
                true,
                " ** 1 of 1 cover properties satisfied\nVERIFICATION:- SUCCESSFUL"
            ),
            KaniRunOutcome::Verified
        );
    }
}

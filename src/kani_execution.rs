//! Pinned execution of one generated Kani obligation harness (FR-017).
//!
//! The backend is pinned by committed values ([`KaniToolPins::pinned`]). Before
//! anything runs, the harness identity's pins and the pins re-measured from the
//! installed backend are both compared with them: any differing Kani version,
//! launcher, driver, CBMC, toolchain or target is a typed refusal and no proof is
//! attempted. The run outcome is read from the backend's own output and is never
//! defaulted: a harness this module did not observe verifying is not `verified`.

use std::{
    env, fmt, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::{
    kani::{sha256, KANI_BACKEND_VERSION},
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

/// The one installed backend this adapter is pinned to: the values measured by
/// [`KaniInstallation::observe`] from the Kani 0.67.0 installation the kani lane was recorded
/// against on x86_64 Linux. A backend with any other value is refused before a harness is
/// generated and again before one runs.
const PINNED_LAUNCHER_SHA256: &str =
    "7f143a251d11c7e6e232bbf2cbccf56f9ce66a5f0107eeb3008698e6715f55d9";
const PINNED_DRIVER_SHA256: &str =
    "683f3ad1216e67686a39b2fcd6dc661f5090574f55fde9dd407966dca42cbfad";
const PINNED_CBMC_VERSION: &str = "6.8.0 (cbmc-6.8.0)";
const PINNED_RUST_TOOLCHAIN: &str = "nightly-2025-11-21-x86_64-unknown-linux-gnu";
const PINNED_TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";

impl KaniToolPins {
    /// The committed backend pins: Kani [`KANI_BACKEND_VERSION`], its launcher and driver
    /// digests, CBMC 6.8.0, toolchain `nightly-2025-11-21` and target `x86_64-unknown-linux-gnu`.
    #[must_use]
    pub fn pinned() -> Self {
        Self {
            kani_version: KANI_BACKEND_VERSION.to_owned(),
            launcher_sha256: PINNED_LAUNCHER_SHA256.to_owned(),
            driver_sha256: PINNED_DRIVER_SHA256.to_owned(),
            cbmc_version: PINNED_CBMC_VERSION.to_owned(),
            rust_toolchain: PINNED_RUST_TOOLCHAIN.to_owned(),
            target_triple: PINNED_TARGET_TRIPLE.to_owned(),
        }
    }

    /// The first field that differs from `expected`, with the expected and actual values.
    pub(crate) fn first_difference(
        &self,
        expected: &Self,
    ) -> Option<(KaniPinField, String, String)> {
        self.fields()
            .into_iter()
            .zip(expected.fields())
            .find(|((_, actual), (_, wanted))| actual != wanted)
            .map(|((field, actual), (_, wanted))| (field, wanted.to_owned(), actual.to_owned()))
    }

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
    /// The generated crate's `src/lib.rs`, checked for the harness's generated source.
    Library,
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
    /// The harness identity, or else the installed backend, differs from the committed pins
    /// ([`KaniToolPins::pinned`]).
    PinDrift {
        /// The differing field.
        field: KaniPinField,
        /// The committed pin.
        expected: String,
        /// The harness's value, or the installed value when the harness matches.
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
                "{field:?} drifted: pinned {expected:?}, found {observed:?}"
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
    /// Kani reported success without a readable, non-empty cover summary, so non-vacuity was
    /// not observed.
    MissingCoverSummary,
    /// An unwinding assertion failed: the loop bound was exhausted before the property
    /// could be decided, so no failure is a counterexample.
    UnwindBoundExhausted,
}

/// The backend-reported outcome of one run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum KaniRunOutcome {
    /// Every property held and every cover was satisfied, so the harness's assumptions,
    /// requires and IR bounds are jointly satisfiable.
    Verified,
    /// A property other than an unwinding assertion failed and Kani printed a concrete
    /// counterexample for it.
    Falsified {
        /// The concrete-playback test Kani printed for the failed check, verbatim.
        counterexample: String,
    },
    /// Vacuous: no property failed, but a cover was not satisfied within the bounds. For a
    /// precondition harness the precondition is unsatisfiable; for a contract harness the
    /// requires and IR bounds are jointly unsatisfiable, so the ensures was never checked.
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
    /// SHA-256 of the generated crate's `Cargo.lock` after the run, or `None` when the run
    /// happened but the lockfile could not be read afterward — the outcome is then
    /// [`KaniInconclusiveReason::NoVerdict`], because evidence about a run that occurred is
    /// incomplete rather than absent.
    pub cargo_lock_sha256: Option<String>,
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
    // The harness identity's pins are known from the harness alone, without touching the
    // backend, so a drifted identity refuses with zero processes started rather than after
    // `observe()` has already spawned `cargo-kani kani --version`, `cbmc --version` and
    // `rustc -vV`.
    if let Some((field, expected, observed)) =
        identity.pins.first_difference(&KaniToolPins::pinned())
    {
        return Err(KaniExecutionRefusal::PinDrift {
            field,
            expected,
            observed,
        });
    }
    let observed = request.installation.observe()?;
    if let Some((field, expected, observed)) = observed.first_difference(&KaniToolPins::pinned()) {
        return Err(KaniExecutionRefusal::PinDrift {
            field,
            expected,
            observed,
        });
    }
    let library_path = request.crate_directory.join("src").join("lib.rs");
    let library = read_file(KaniTool::Library, &library_path).map_err(|_| {
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
    // The backend already ran by this point: a lockfile that cannot be read now is incomplete
    // evidence about a real run, not grounds to refuse as though nothing happened. The outcome
    // is downgraded to inconclusive rather than the run being reported as a pre-run refusal.
    let (cargo_lock_sha256, outcome) = match file_sha256(
        KaniTool::Lockfile,
        &request.crate_directory.join("Cargo.lock"),
    ) {
        Ok(digest) => (Some(digest), classify_run(output.status.success(), &text)),
        Err(_) => (
            None,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::NoVerdict,
            },
        ),
    };
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
        outcome,
    })
}

const SUCCESS: &str = "VERIFICATION:- SUCCESSFUL";
const FAILURE: &str = "VERIFICATION:- FAILED";
const FAILED_CHECKS: &str = "Failed Checks: ";
const UNWINDING_ASSERTION: &str = "unwinding assertion";

/// Classifies one run. Every generated harness, of every kind, carries exactly the covers that
/// witness its assumptions are satisfiable, so success without every cover satisfied is vacuous
/// and never `Verified`.
fn classify_run(exited_successfully: bool, text: &str) -> KaniRunOutcome {
    if exited_successfully && text.contains(SUCCESS) && !text.contains(FAILURE) {
        return match cover_summary(text) {
            Some((satisfied, total)) if total > 0 && satisfied == total => KaniRunOutcome::Verified,
            Some((satisfied, total)) if total > 0 => {
                KaniRunOutcome::CoverUnsatisfied { satisfied, total }
            }
            Some(_) | None => KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::MissingCoverSummary,
            },
        };
    }
    if text.contains(FAILURE) {
        let unwound = text.lines().any(|line| {
            line.trim()
                .strip_prefix(FAILED_CHECKS)
                .is_some_and(|check| check.starts_with(UNWINDING_ASSERTION))
        });
        if unwound {
            return KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::UnwindBoundExhausted,
            };
        }
        return match failure_playback(text) {
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

/// Reads Kani's `** <satisfied> of <total> cover properties satisfied[ (<n> unreachable)]` line.
fn cover_summary(text: &str) -> Option<(u64, u64)> {
    text.lines().find_map(|line| {
        let rest = line.trim().strip_prefix("** ")?;
        let (counts, tail) = rest.split_once(" cover properties satisfied")?;
        let tail_is_summary = tail.is_empty()
            || tail
                .strip_prefix(" (")
                .and_then(|inner| inner.strip_suffix(" unreachable)"))
                .is_some_and(|count| count.parse::<u64>().is_ok());
        if !tail_is_summary {
            return None;
        }
        let (satisfied, total) = counts.split_once(" of ")?;
        Some((satisfied.parse().ok()?, total.parse().ok()?))
    })
}

/// Extracts the fenced concrete-playback unit test Kani prints for a failed check. Kani also
/// prints playbacks for satisfied covers; those witness reachability and are not counterexamples.
fn failure_playback(text: &str) -> Option<String> {
    let mut rest = text;
    while let Some(start) = rest.find("Concrete playback unit test") {
        let tail = &rest[start..];
        let fence = tail.find("```")?;
        let body = &tail[fence + 3..];
        let end = body.find("```")?;
        let test = body[..end].trim();
        let is_cover = test
            .lines()
            .any(|line| line.starts_with("/// Check for `cover`"));
        if test.contains("kani::concrete_playback_run") && !is_cover {
            return Some(test.to_owned());
        }
        rest = &body[end + 3..];
    }
    None
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

    const COVER_PLAYBACK: &str = "Concrete playback unit test for `m::h`:\n```\n/// Test generated for harness `m::h` that checks contract for `c`\n///\n/// Check for `cover`: \"contract assumptions are jointly satisfiable\"\n\n#[test]\nfn kani_concrete_playback_h_1() {\n    let concrete_vals: Vec<Vec<u8>> = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];\n    kani::concrete_playback_run(concrete_vals, h);\n}\n```\n";
    const ASSERTION_PLAYBACK: &str = "Concrete playback unit test for `m::h`:\n```\n/// Test generated for harness `m::h` that checks contract for `c`\n///\n/// Check for `assertion`: \"|post_state: &i64| *post_state <= 5\"\n\n#[test]\nfn kani_concrete_playback_h_2() {\n    let concrete_vals: Vec<Vec<u8>> = vec![vec![8, 0, 0, 0, 0, 0, 0, 0]];\n    kani::concrete_playback_run(concrete_vals, h);\n}\n```\n";

    /// Success is `Verified` only with every cover satisfied, for every obligation kind; a
    /// vacuous run is `CoverUnsatisfied`. Summaries are Kani 0.67.0's own output.
    ///
    /// Trace: FR-017-AC-4, FR-017-AC-5, TC-027
    #[test]
    fn tc_027_run_classification_never_defaults_to_verified() {
        let verified = format!(
            "SUMMARY:\n ** 0 of 43 failed\n\n ** 1 of 1 cover properties satisfied\n\n\nVERIFICATION:- SUCCESSFUL\n{COVER_PLAYBACK}"
        );
        assert_eq!(classify_run(true, &verified), KaniRunOutcome::Verified);
        // Jointly unsatisfiable requires: every check succeeds, the ensures is unreachable, and
        // the cover after the contract call is unreachable. Reproduced under Kani 0.67.0.
        let vacuous = "SUMMARY:\n ** 0 of 49 failed (1 unreachable)\n\n ** 0 of 1 cover properties satisfied (1 unreachable)\n\n\nVERIFICATION:- SUCCESSFUL\n";
        assert_eq!(
            classify_run(true, vacuous),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1
            }
        );
        assert_eq!(
            classify_run(
                true,
                " ** 1 of 2 cover properties satisfied\nVERIFICATION:- SUCCESSFUL"
            ),
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 1,
                total: 2
            }
        );
        // The failure's counterexample is the assertion playback, not the cover playback.
        let falsified = format!(
            "SUMMARY:\n ** 1 of 43 failed\nFailed Checks: |post_state: &i64| *post_state <= 5\n\n ** 1 of 1 cover properties satisfied\n\nVERIFICATION:- FAILED\n{COVER_PLAYBACK}{ASSERTION_PLAYBACK}"
        );
        assert!(matches!(
            classify_run(false, &falsified),
            KaniRunOutcome::Falsified { counterexample }
                if counterexample.contains("Check for `assertion`") && !counterexample.contains("Check for `cover`")
        ));
        for (success, text, expected) in [
            (
                false,
                format!("VERIFICATION:- FAILED\n{COVER_PLAYBACK}"),
                KaniInconclusiveReason::FailedWithoutCounterexample,
            ),
            (
                false,
                "error[E0308]: mismatched types".to_owned(),
                KaniInconclusiveReason::NoVerdict,
            ),
            // Success text from a process that exited unsuccessfully is not success.
            (false, verified.clone(), KaniInconclusiveReason::NoVerdict),
            (
                true,
                "VERIFICATION:- SUCCESSFUL".to_owned(),
                KaniInconclusiveReason::MissingCoverSummary,
            ),
            (
                true,
                " ** 0 of 0 cover properties satisfied\nVERIFICATION:- SUCCESSFUL".to_owned(),
                KaniInconclusiveReason::MissingCoverSummary,
            ),
            (
                true,
                " ** 1 of 1 cover properties satisfied (garbage)\nVERIFICATION:- SUCCESSFUL"
                    .to_owned(),
                KaniInconclusiveReason::MissingCoverSummary,
            ),
        ] {
            assert_eq!(
                classify_run(success, &text),
                KaniRunOutcome::Inconclusive { reason: expected },
                "{text}"
            );
        }
    }

    /// An exhausted unwind bound is inconclusive even when Kani prints a playback. The output
    /// is Kani 0.67.0's for a loop past `--unwind 4`.
    ///
    /// Trace: FR-017-AC-5, TC-027
    #[test]
    fn tc_027_an_exhausted_unwind_bound_is_inconclusive_not_falsified() {
        let unwound = format!(
            "VERIFICATION RESULT:\n ** 1 of 39 failed (38 undetermined)\n\n ** 1 of 1 cover properties satisfied\n\nFailed Checks: unwinding assertion loop 0\n File: \"src/lib.rs\", line 10, in looping\n\nVERIFICATION:- FAILED\n[Kani] info: Verification output shows one or more unwinding failures.\n{COVER_PLAYBACK}{ASSERTION_PLAYBACK}"
        );
        assert_eq!(
            classify_run(false, &unwound),
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::UnwindBoundExhausted
            }
        );
        // A succeeded unwinding check in the results listing is not a failure.
        let listed = format!(
            "Check 1: f.unwind.1\n\t - Status: SUCCESS\n\t - Description: \"unwinding assertion loop 0\"\n ** 1 of 1 cover properties satisfied\nVERIFICATION:- SUCCESSFUL\n{COVER_PLAYBACK}"
        );
        assert_eq!(classify_run(true, &listed), KaniRunOutcome::Verified);
    }

    /// Only the committed pins pass; each field is compared.
    ///
    /// Trace: FR-017-AC-1, FR-017-AC-3, TC-027
    #[test]
    fn tc_027_pins_are_compared_field_by_field_against_the_committed_backend() {
        let pinned = KaniToolPins::pinned();
        assert_eq!(pinned.kani_version, "0.67.0");
        assert_eq!(pinned.cbmc_version, "6.8.0 (cbmc-6.8.0)");
        assert_eq!(
            pinned.rust_toolchain,
            "nightly-2025-11-21-x86_64-unknown-linux-gnu"
        );
        assert_eq!(pinned.target_triple, "x86_64-unknown-linux-gnu");
        assert_eq!(pinned.first_difference(&KaniToolPins::pinned()), None);
        let mut other = KaniToolPins::pinned();
        other.target_triple = "aarch64-unknown-linux-gnu".to_owned();
        assert_eq!(
            other.first_difference(&pinned),
            Some((
                KaniPinField::TargetTriple,
                "x86_64-unknown-linux-gnu".to_owned(),
                "aarch64-unknown-linux-gnu".to_owned()
            ))
        );
    }
}

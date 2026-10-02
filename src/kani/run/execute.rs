//! Execution of one generated Kani obligation harness (FR-017).
//!
//! For every outcome but one, the run outcome is read from the
//! backend's own exported report -- read into a typed value by [`crate::kani::output::report`], the
//! only place Kani's output is read -- and is never defaulted: a harness this module did not
//! observe verifying is not `verified`, and a report it cannot read is a typed refusal, not an
//! outcome. The one exception is [`KaniInconclusiveReason::TimedOut`], which is never read from
//! output at all — a timed-out run is killed before it writes a report.
//!
//! The caller states a wall-clock budget on every request
//! ([`KaniExecutionRequest::timeout`]); nothing here defaults one. A run that does
//! not conclude within it is killed and classified `TimedOut` rather than left to
//! block the caller forever (agent-ix/quire-contract-codegen#58). This module's
//! own call always returns within that budget plus a small constant, regardless
//! of what the launcher forked: the launcher runs as the leader of its own
//! process group and a timeout kills the whole group, so CBMC and every other
//! descendant die with it. A descendant that leaves the group is not reached, and
//! the caller is never made to wait on output from it beyond a short, fixed drain.

use std::{fmt, fs, path::Path, process::Command, time::Duration};

use serde::Serialize;

use crate::kani::{
    abi::KaniSolver,
    classify::{classify_kani_run, ClassifiedRun, KaniInconclusiveReason, KaniRunOutcome},
    identity::ObligationKind,
    output::report::{KaniCheckResult, KaniReportRefusal},
    run::{
        harness::KaniExecutableHarness,
        launch::{run_launcher_with_timeout, LaunchOutcome},
        report_file::{fresh_report_path, read_report, remove_stale_report},
        tool::{KaniInstallation, KaniTool, KaniToolError},
    },
};

/// One execution of one harness in a crate the caller wrote.
pub struct KaniExecutionRequest<'a> {
    /// Backend to invoke.
    pub installation: &'a KaniInstallation,
    /// The generated harness.
    pub harness: KaniExecutableHarness<'a>,
    /// Crate root whose `src/lib.rs` contains the harness source byte-for-byte.
    pub crate_directory: &'a Path,
    /// Cargo target directory for the run.
    pub target_directory: &'a Path,
    /// Wall-clock budget for the launcher. The caller states this explicitly on every
    /// request; there is no default that would let a run go unbounded silently. A run
    /// that has not concluded when the budget elapses is killed and reported as
    /// [`KaniRunOutcome::Inconclusive`] with [`KaniInconclusiveReason::TimedOut`].
    pub timeout: Duration,
}

/// Why a harness was not run.
#[derive(Debug)]
pub enum KaniExecutionRefusal {
    /// The installed backend could not be located or started.
    Tool(KaniToolError),
    /// The crate's `src/lib.rs` does not contain the harness source.
    HarnessNotInCrate {
        /// The generated artifact path.
        harness_path: String,
    },
    /// The run exported a Kani report this crate cannot read exactly. The run proved nothing
    /// and decided nothing; the refusal is never an outcome.
    Report(KaniReportRefusal),
}

impl fmt::Display for KaniExecutionRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tool(error) => write!(formatter, "{error}"),
            Self::HarnessNotInCrate { harness_path } => {
                write!(formatter, "the crate does not contain {harness_path}")
            }
            Self::Report(refusal) => write!(formatter, "{refusal}"),
        }
    }
}

impl std::error::Error for KaniExecutionRefusal {}

impl From<KaniReportRefusal> for KaniExecutionRefusal {
    fn from(refusal: KaniReportRefusal) -> Self {
        Self::Report(refusal)
    }
}

impl From<KaniToolError> for KaniExecutionRefusal {
    fn from(error: KaniToolError) -> Self {
        Self::Tool(error)
    }
}

/// What ran and the backend-reported outcome of one harness run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KaniExecutionEvidence {
    /// Contract role of a contract harness; `None` for an exact-scalar harness, whose claim
    /// has no contract role.
    pub kind: Option<ObligationKind>,
    /// Generated harness path.
    pub harness_path: String,
    /// Invoked launcher path.
    pub launcher_path: String,
    /// Complete argument vector after the launcher.
    pub arguments: Vec<String>,
    /// Loop unwind bound.
    pub unwind: u32,
    /// Solver.
    pub solver: KaniSolver,
    /// Process exit code, or `None` when the process was killed by a signal — including the
    /// kill this module itself sends on [`KaniInconclusiveReason::TimedOut`].
    pub exit_code: Option<i32>,
    /// Backend-reported outcome.
    pub outcome: KaniRunOutcome,
    /// How many checks the report lists as holding: the non-cover checks with status success,
    /// plus, for a precondition harness, whose one property is its cover, the satisfied covers.
    /// Zero when the run produced no report. It is the SUCCESS-check count FR-017 defines; the
    /// terminal map that will read it (FR-029) is not implemented yet.
    pub success_checks: u32,
    /// Every check Kani reported, with its class, source location and status, in report order.
    /// Empty when the run produced no report. A consumer attributes proof to source with it.
    pub checks: Vec<KaniCheckResult>,
}

/// Runs the harness and reports the backend's own outcome.
pub fn execute_kani_obligation(
    request: &KaniExecutionRequest<'_>,
) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
    let harness = request.harness.view();
    let library_path = request.crate_directory.join("src").join("lib.rs");
    let library = read_file(KaniTool::Library, &library_path).map_err(|_| {
        KaniExecutionRefusal::HarnessNotInCrate {
            harness_path: harness.rust.path.clone(),
        }
    })?;
    if !String::from_utf8_lossy(&library).contains(&harness.rust.contents) {
        return Err(KaniExecutionRefusal::HarnessNotInCrate {
            harness_path: harness.rust.path.clone(),
        });
    }
    let report_path = fresh_report_path(request.target_directory);
    remove_stale_report(&report_path)?;
    let (arguments, command) = launch_command(request, &report_path);
    let launch =
        run_launcher_with_timeout(command, request.timeout).map_err(|error| KaniToolError::Io {
            tool: KaniTool::Launcher,
            path: request.installation.launcher.clone(),
            error,
        })?;
    let report = match launch {
        LaunchOutcome::Completed { .. } => read_report(&report_path),
        LaunchOutcome::TimedOut => Ok(None),
    };
    // Only this run's own file is removed, whatever the read found.
    let _ = fs::remove_file(&report_path);
    let (run, exit_code) = launch_evidence(launch, report?.as_deref(), harness.kind)?;
    Ok(KaniExecutionEvidence {
        kind: harness.kind,
        harness_path: harness.rust.path.clone(),
        launcher_path: request.installation.launcher.display().to_string(),
        arguments,
        unwind: harness.unwind,
        solver: harness.solver,
        exit_code,
        outcome: run.outcome,
        success_checks: run.success_checks,
        checks: run.checks,
    })
}

/// Builds the exact argument vector and [`Command`] [`execute_kani_obligation`] launches for
/// `request`, without spawning it, so a caller driving [`run_launcher_with_timeout`] itself
/// launches exactly what `execute_kani_obligation` does.
///
/// The vector is the harness identity's option vector followed by the flags that make Kani export
/// its report to a file in the target directory, which is where the verdict is read from. The
/// file's name is unique to this call (the last argument), so two runs sharing a target
/// directory never write, remove or read each other's report.
pub fn kani_launch_command(request: &KaniExecutionRequest<'_>) -> (Vec<String>, Command) {
    launch_command(request, &fresh_report_path(request.target_directory))
}

fn launch_command(
    request: &KaniExecutionRequest<'_>,
    report_path: &Path,
) -> (Vec<String>, Command) {
    let mut arguments = vec!["kani".to_owned()];
    arguments.extend(request.harness.view().options.iter().cloned());
    arguments.extend([
        "-Z".to_owned(),
        "unstable-options".to_owned(),
        "--export-json".to_owned(),
        report_path.display().to_string(),
    ]);
    let mut command = Command::new(&request.installation.launcher);
    command
        .args(&arguments)
        .env("CARGO_TARGET_DIR", request.target_directory)
        .current_dir(request.crate_directory);
    (arguments, command)
}

/// Maps a concluded [`LaunchOutcome`] and the report its run exported to the `(run, exit_code)`
/// pair [`KaniExecutionEvidence`] stores, exactly as `execute_kani_obligation` does. `report` is
/// the exported report's bytes, `None` when the run exported none; a timed-out run is killed
/// before it can, so its report is not consulted. `kind` is the harness's contract role (`None`
/// for an exact-scalar harness); see [`classify_kani_run`] for how it affects the zero-checks
/// rule. Kept as its own pure function so the mapping is tested directly with a value rather than
/// a real subprocess.
pub fn launch_evidence(
    launch: LaunchOutcome,
    report: Option<&[u8]>,
    kind: Option<ObligationKind>,
) -> Result<(ClassifiedRun, Option<i32>), KaniReportRefusal> {
    match launch {
        LaunchOutcome::Completed {
            exited_successfully,
            exit_code,
            text,
        } => Ok((
            classify_kani_run(exited_successfully, report, &text, kind)?,
            exit_code,
        )),
        LaunchOutcome::TimedOut => Ok((
            ClassifiedRun {
                outcome: KaniRunOutcome::Inconclusive {
                    reason: KaniInconclusiveReason::TimedOut,
                },
                success_checks: 0,
                checks: Vec::new(),
            },
            None,
        )),
    }
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::kani::{
        identity::StateFrameProperty,
        test_support::{discover_scratch, report, state_frame_harness, COVER_NO, COVER_OK, PASSED},
    };

    /// A precondition harness asserts nothing; its only property is its cover. The same report
    /// that is a vacuous proof for any other harness therefore decides by the cover alone for a
    /// precondition harness, whose satisfied cover is its one successful check.
    ///
    /// Trace: FR-017-AC-13, TC-027
    #[test]
    fn a_precondition_harness_with_no_checks_is_decided_by_its_cover_not_the_zero_checks_rule() {
        let run = |kind: Option<ObligationKind>, checks: &[(&str, &str)]| {
            launch_evidence(
                LaunchOutcome::Completed {
                    exited_successfully: true,
                    exit_code: Some(0),
                    text: String::new(),
                },
                Some(&report("Success", checks)),
                kind,
            )
            .unwrap()
            .0
        };
        let satisfied = run(Some(ObligationKind::Precondition), &[COVER_OK]);
        assert_eq!(satisfied.outcome, KaniRunOutcome::Verified);
        assert_eq!(satisfied.success_checks, 1);
        for kind in [Some(ObligationKind::Postcondition), None] {
            assert_eq!(
                run(kind, &[COVER_OK]).outcome,
                KaniRunOutcome::Inconclusive {
                    reason: KaniInconclusiveReason::VacuousProof
                }
            );
        }
        assert_eq!(
            run(Some(ObligationKind::Precondition), &[COVER_NO]).outcome,
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1
            }
        );
        assert_eq!(
            run(Some(ObligationKind::Precondition), &[]).outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::MissingCoverSummary
            }
        );
    }

    /// Drives `execute_kani_obligation` against a launcher stand-in that exits with `status` and,
    /// when given a report, writes it where `--export-json` names. `stale` is left in the target
    /// directory beforehand, as an earlier run would leave it.
    fn run_stand_in(
        name: &str,
        status: i32,
        exported: Option<&str>,
        stale: Option<&str>,
    ) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
        run_stand_in_into(name, status, exported, stale, None)
    }

    /// [`run_stand_in`] with an optional target directory shared with other runs; a shared run
    /// lingers before exporting so that runs sharing the directory overlap.
    fn run_stand_in_into(
        name: &str,
        status: i32,
        exported: Option<&str>,
        stale: Option<&str>,
        shared_target: Option<&Path>,
    ) -> Result<KaniExecutionEvidence, KaniExecutionRefusal> {
        use std::os::unix::fs::PermissionsExt;
        let directory = discover_scratch(name);
        let crate_directory = directory.join("crate");
        let target_directory =
            shared_target.map_or_else(|| directory.join("target"), Path::to_path_buf);
        fs::create_dir_all(crate_directory.join("src")).unwrap();
        fs::create_dir_all(&target_directory).unwrap();
        fs::write(crate_directory.join("src/lib.rs"), "").unwrap();
        if let Some(stale) = stale {
            fs::write(
                target_directory.join("quire-kani-report-other-run.json"),
                stale,
            )
            .unwrap();
        }
        if let Some(exported) = exported {
            fs::write(directory.join("exported.json"), exported).unwrap();
        }
        let launcher = directory.join("cargo-kani");
        let linger = if shared_target.is_some() {
            "sleep 1\n"
        } else {
            ""
        };
        let copy = if exported.is_some() {
            format!(
                "cp '{}' \"$last\"",
                directory.join("exported.json").display()
            )
        } else {
            ":".to_owned()
        };
        fs::write(
            &launcher,
            format!("#!/bin/sh\nfor last; do :; done\n{linger}{copy}\nexit {status}\n"),
        )
        .unwrap();
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755)).unwrap();
        let harness = state_frame_harness(
            StateFrameProperty::Frame {
                granted: Vec::new(),
                checked: Vec::new(),
            },
            Vec::new(),
        );
        let result = execute_kani_obligation(&KaniExecutionRequest {
            installation: &KaniInstallation { launcher },
            harness: KaniExecutableHarness::from(&harness),
            crate_directory: &crate_directory,
            target_directory: &target_directory,
            timeout: Duration::from_secs(30),
        });
        if stale.is_some() {
            assert!(
                target_directory
                    .join("quire-kani-report-other-run.json")
                    .is_file(),
                "another run's report is not this run's to remove"
            );
        }
        if shared_target.is_none() {
            let leftovers = fs::read_dir(&target_directory)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_name() != "quire-kani-report-other-run.json")
                .count();
            assert_eq!(leftovers, 0, "a run removes its own report file");
        }
        let _ = fs::remove_dir_all(directory);
        result
    }

    /// A run's report is read from the file the launch names, the previous run's file is never
    /// read in its place, and a successful exit without a report is refused.
    ///
    /// Trace: FR-017-AC-18, FR-017-AC-19, TC-027
    #[test]
    fn tc_027_execution_reads_only_the_report_its_own_run_exported() {
        let verified = String::from_utf8(report("Success", &[PASSED, COVER_OK])).unwrap();
        let evidence = run_stand_in("exported", 0, Some(&verified), None).unwrap();
        assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
        assert_eq!(evidence.success_checks, 1);
        assert_eq!(evidence.exit_code, Some(0));
        assert!(matches!(
            run_stand_in("missing", 0, None, None),
            Err(KaniExecutionRefusal::Report(KaniReportRefusal::Missing))
        ));
        assert!(
            matches!(
                run_stand_in("stale", 0, None, Some(&verified)),
                Err(KaniExecutionRefusal::Report(KaniReportRefusal::Missing))
            ),
            "a report left by an earlier run is not this run's verdict"
        );
        assert!(matches!(
            run_stand_in("garbage", 1, Some("not json"), None),
            Err(KaniExecutionRefusal::Report(
                KaniReportRefusal::Malformed { .. }
            ))
        ));
    }

    /// The command exports the report to the request's target directory after the harness
    /// identity's options, which are passed unchanged.
    ///
    /// Trace: FR-017-AC-6, FR-017-AC-19, TC-027
    #[test]
    fn tc_027_the_launch_exports_the_report_after_the_harness_options() {
        let harness = state_frame_harness(
            StateFrameProperty::Frame {
                granted: Vec::new(),
                checked: Vec::new(),
            },
            vec![
                "--harness".to_owned(),
                "check".to_owned(),
                "--exact".to_owned(),
            ],
        );
        let installation = KaniInstallation {
            launcher: PathBuf::from("cargo-kani"),
        };
        let request = KaniExecutionRequest {
            installation: &installation,
            harness: KaniExecutableHarness::from(&harness),
            crate_directory: Path::new("/crate"),
            target_directory: Path::new("/target"),
            timeout: Duration::from_secs(1),
        };
        let (arguments, _) = kani_launch_command(&request);
        let options = request.harness.view().options;
        assert_eq!(arguments[1..=options.len()], options[..]);
        assert_eq!(
            arguments[options.len() + 1..arguments.len() - 1],
            ["-Z", "unstable-options", "--export-json"]
        );
        let report = arguments.last().unwrap();
        assert!(
            report.starts_with("/target/quire-kani-report-") && report.ends_with(".json"),
            "{report}"
        );
        // Two launches into one target directory never name the same report file.
        let (again, _) = kani_launch_command(&request);
        assert_ne!(again.last(), arguments.last());
    }

    /// Two runs sharing a target directory at the same time each read their own report: the
    /// report of a run that is still going is neither read nor removed by another.
    ///
    /// Trace: FR-017-AC-19, TC-027
    #[test]
    fn tc_027_concurrent_runs_in_one_target_directory_keep_their_own_reports() {
        let verified = String::from_utf8(report("Success", &[PASSED, COVER_OK])).unwrap();
        let failed = String::from_utf8(report("Failure", &[("Failure", "assertion")])).unwrap();
        let shared = discover_scratch("concurrent-target");
        let outcomes: Vec<_> = std::thread::scope(|scope| {
            let runs: Vec<_> = (0..4)
                .map(|n| {
                    let exported = if n % 2 == 0 { &verified } else { &failed };
                    let (name, shared) = (format!("concurrent-{n}"), shared.as_path());
                    scope.spawn(move || {
                        run_stand_in_into(
                            &name,
                            i32::from(n % 2 == 1),
                            Some(exported),
                            None,
                            Some(shared),
                        )
                        .unwrap()
                        .outcome
                    })
                })
                .collect();
            runs.into_iter().map(|run| run.join().unwrap()).collect()
        });
        let _ = fs::remove_dir_all(shared);
        assert_eq!(outcomes[0], KaniRunOutcome::Verified);
        assert_eq!(outcomes[2], KaniRunOutcome::Verified);
        assert!(matches!(outcomes[1], KaniRunOutcome::Inconclusive { .. }));
    }

    /// A timed-out launch maps to no exit code and `Inconclusive { reason: TimedOut }`.
    #[test]
    fn a_timed_out_launch_carries_no_exit_code_into_the_evidence() {
        // A timed-out run is killed before it writes a report, so none is consulted.
        let (run, exit_code) = launch_evidence(LaunchOutcome::TimedOut, None, None).unwrap();
        assert_eq!(exit_code, None);
        assert_eq!(
            run.outcome,
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::TimedOut
            }
        );
        assert_eq!(run.success_checks, 0);
    }
}

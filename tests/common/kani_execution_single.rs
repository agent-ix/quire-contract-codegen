//! Shared single-run regressions: normal-library Linux IT and unsupported-platform report semantics.

#[cfg(not(target_os = "linux"))]
use super::*;
#[cfg(target_os = "linux")]
use crate::common::kani_run::{
    discover_scratch, report, state_frame_harness, write_launcher, COVER_NO, COVER_OK, PASSED,
};
#[cfg(not(target_os = "linux"))]
use crate::kani::test_support::{
    discover_scratch, report, state_frame_harness, write_launcher, COVER_NO, COVER_OK, PASSED,
};
use quire_contract_codegen::*;
#[cfg(target_os = "linux")]
use std::{fs, path::Path};
fn guardian_path() -> &'static std::path::Path {
    #[cfg(target_os = "linux")]
    {
        crate::common::guardian_path()
    }
    #[cfg(not(target_os = "linux"))]
    {
        std::path::Path::new("/unavailable-native-guardian")
    }
}
fn original_stdin() -> &'static OriginalStdin {
    #[cfg(target_os = "linux")]
    {
        crate::common::original_stdin()
    }
    #[cfg(not(target_os = "linux"))]
    {
        static CLOSED: OriginalStdin = OriginalStdin::Closed;
        &CLOSED
    }
}
#[cfg(target_os = "linux")]
mod report_fixture {
    use super::*;
    pub(super) type FixtureExecution = KaniExecutionEvidence;
    pub(super) fn single(
        request: &KaniExecutionRequest<'_>,
    ) -> Result<FixtureExecution, KaniExecutionRefusal> {
        execute_kani_obligation(request)
    }
}

/// Drives the shared command/report pipeline against a launcher stand-in that exits with `status` and,
/// when given a report, writes it where `--export-json` names. `stale` is left in the target
/// directory beforehand, as an earlier run would leave it.
fn run_stand_in(
    name: &str,
    status: i32,
    exported: Option<&str>,
    stale: Option<&str>,
) -> Result<report_fixture::FixtureExecution, KaniExecutionRefusal> {
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
) -> Result<report_fixture::FixtureExecution, KaniExecutionRefusal> {
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
    write_launcher(
        &launcher,
        &format!("for last; do :; done\n{linger}{copy}\nexit {status}\n"),
    );
    let harness = state_frame_harness(
        StateFrameProperty::Frame {
            granted: Vec::new(),
            checked: Vec::new(),
        },
        Vec::new(),
    );
    let result = report_fixture::single(&KaniExecutionRequest {
        guardian_path: guardian_path(),
        original_stdin: original_stdin(),
        installation: &KaniInstallation { launcher },
        harness: KaniExecutableHarness::from(&harness),
        crate_directory: &crate_directory,
        target_directory: &target_directory,
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
/// Trace: FR-017-AC-18, TC-027
#[test]
fn tc_027_execution_reads_only_the_report_its_own_run_exported() {
    let verified = String::from_utf8(report("Success", &[PASSED, COVER_OK])).unwrap();
    let evidence = run_stand_in("exported", 0, Some(&verified), None).unwrap();
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert_eq!(evidence.success_checks, 1);
    assert_eq!(evidence.exit_code, Some(0));
    let unreached = String::from_utf8(report("Success", &[PASSED, COVER_NO])).unwrap();
    assert_eq!(
        run_stand_in("unreached-cover", 0, Some(&unreached), None)
            .unwrap()
            .outcome,
        KaniRunOutcome::CoverUnsatisfied {
            satisfied: 0,
            total: 1
        }
    );
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

/// Trace: FR-034-AC-17
#[cfg(target_os = "linux")]
#[test]
fn normal_guardian_preserves_non_utf8_program_cwd_environment_and_report_argument() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    let directory = discover_scratch("guardian-raw-paths");
    let crate_directory = directory.join(OsString::from_vec(b"cwd-\xfe".to_vec()));
    let target_directory = directory.join(OsString::from_vec(b"target-\xfd".to_vec()));
    let launcher = directory.join(OsString::from_vec(b"launcher-\xff".to_vec()));
    fs::create_dir_all(crate_directory.join("src")).unwrap();
    fs::create_dir_all(&target_directory).unwrap();
    fs::write(crate_directory.join("src/lib.rs"), "").unwrap();
    let exported = String::from_utf8(report("Success", &[PASSED, COVER_OK])).unwrap();
    let literal = serde_json::to_string(&exported).unwrap();
    write_launcher(
        &launcher,
        &format!(
            "python3 - \"$@\" <<'PY'\n\
             import os,sys\n\
             assert os.getcwdb().endswith(b'/cwd-\\xfe')\n\
             assert os.environb[b'CARGO_TARGET_DIR'].endswith(b'/target-\\xfd')\n\
             assert b'/target-\\xfd/' in os.fsencode(sys.argv[-1])\n\
             with open(sys.argv[-1],'wb') as report_file: report_file.write({literal}.encode())\n\
             PY\n"
        ),
    );
    let harness = state_frame_harness(
        StateFrameProperty::Frame {
            granted: Vec::new(),
            checked: Vec::new(),
        },
        Vec::new(),
    );
    let result = execute_kani_obligation(&KaniExecutionRequest {
        guardian_path: guardian_path(),
        original_stdin: original_stdin(),
        installation: &KaniInstallation { launcher },
        harness: KaniExecutableHarness::from(&harness),
        crate_directory: &crate_directory,
        target_directory: &target_directory,
    });
    fs::remove_dir_all(directory).unwrap();
    let evidence = result.unwrap();
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert_eq!(evidence.success_checks, 1);
    assert_eq!(evidence.exit_code, Some(0));
}

/// Two runs sharing a target directory at the same time each read their own report: the
/// report of a run that is still going is neither read nor removed by another.
///
/// Trace: TC-027
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

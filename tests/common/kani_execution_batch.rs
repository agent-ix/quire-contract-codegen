//! Shared batch regression assertions: normal-library Linux IT and unsupported-platform report semantics.

use std::path::PathBuf;

#[cfg(not(target_os = "linux"))]
use super::*;
#[cfg(target_os = "linux")]
use crate::common::kani_run::{
    batch_report, discover_scratch, named_state_frame_harness, write_launcher, BatchEntry,
    COVER_OK, PASSED,
};
#[cfg(not(target_os = "linux"))]
use crate::kani::test_support::{
    batch_report, discover_scratch, named_state_frame_harness, write_launcher, BatchEntry,
    COVER_OK, PASSED,
};
use quire_contract_codegen::*;
#[cfg(target_os = "linux")]
use std::{fs, path::Path, time::Duration};
const CAPTURE_LIMIT: usize = 8 * 1024 * 1024;

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

#[cfg(target_os = "linux")]
mod report_fixture {
    use super::*;
    pub(super) type FixtureExecution = KaniExecutionEvidence;
    pub(super) struct ReportGroup {
        pub(super) members: Vec<usize>,
        pub(super) reports: Result<Vec<FixtureExecution>, KaniExecutionRefusal>,
    }
    pub(super) fn single(
        request: &KaniExecutionRequest<'_>,
    ) -> Result<FixtureExecution, KaniExecutionRefusal> {
        execute_kani_obligation(request)
    }
    pub(super) fn groups(
        requests: &[KaniExecutionRequest<'_>],
    ) -> Result<Vec<ReportGroup>, KaniExecutionRefusal> {
        execute_kani_obligations(requests).map(|groups| {
            groups
                .into_iter()
                .map(|group| ReportGroup {
                    members: group.members,
                    reports: group.evidence,
                })
                .collect()
        })
    }
}

/// The request timeout T of the tests that do not vary it.
const T: Duration = Duration::from_secs(30);

/// The options every harness of these tests shares once its `--harness <path> --exact`
/// selection is taken out, written out so the test is not the code's own vector.
// Expected argv for the Linux full bounded-process count test only.
#[cfg(target_os = "linux")]
const SHARED_OPTIONS: [&str; 12] = [
    "-Z",
    "function-contracts",
    "-Z",
    "concrete-playback",
    "--unwind",
    "4",
    "--solver",
    "cadical",
    "--output-format",
    "regular",
    "--concrete-playback",
    "print",
];

/// A small shell launcher with two children, one of which allocates resident bytes.
#[cfg(target_os = "linux")]
const CHILD_MEMORY_OVERAGE: &str = r#"sleep 45 &
echo $! > "$CALLS.sibling"
python3 -c 'import os, pathlib, signal, sys
root = pathlib.Path(sys.argv[1])
child = os.fork()
if child:
os._exit(0)
os.setsid()
while os.getppid() != 1:
os.sched_yield()
pathlib.Path(str(root) + ".namespace").write_text(os.readlink("/proc/self/ns/pid"))
pathlib.Path(str(root) + ".groups").write_text(str(os.getpgrp()) + " " + str(os.getpgid(os.getppid())))
parent = pathlib.Path("/proc") / str(os.getppid()) / "status"
rss = next(line.split()[1] for line in parent.read_text().splitlines() if line.startswith("VmRSS:"))
pathlib.Path(str(root) + ".parent-rss").write_text(rss)
pathlib.Path(str(root) + ".child").write_text(str(os.getpid()))
allocated = bytearray(64 * 1024 * 1024)
signal.pause()
' "$CALLS" &
wait
"#;

#[cfg(target_os = "linux")]
fn recorded_process_gone(path: &Path) {
    let pid = fs::read_to_string(path).expect("a child must have started before the overage");
    let namespace_path = path.with_file_name("calls.namespace");
    let namespace = fs::read_to_string(namespace_path)
        .expect("the allocating descendant records its namespace");
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        let alive = fs::read_dir("/proc")
            .unwrap()
            .filter_map(Result::ok)
            .any(|entry| {
                let directory = entry.path();
                let same_namespace = fs::read_link(directory.join("ns/pid"))
                    .is_ok_and(|link| link.as_os_str() == namespace.as_str());
                if !same_namespace {
                    return false;
                }
                let Ok(status) = fs::read_to_string(directory.join("status")) else {
                    return false;
                };
                status
                    .lines()
                    .find_map(|line| line.strip_prefix("NSpid:"))
                    .is_some_and(|ids| ids.split_whitespace().last() == Some(pid.trim()))
            });
        if !alive {
            return;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "the backend tree child must be killed"
        );
        std::thread::yield_now();
    }
}

fn memory_member(module: &str, bytes: u64) -> StateFrameHarness {
    let mut harness = member(module, "check", 4);
    harness.identity.ceilings.memory_bytes = std::num::NonZeroU64::new(bytes).unwrap();
    harness
}

/// Trace: FR-028-AC-3, FR-028-AC-21.
// Exercises the real Linux namespace/procfs bounded execution API.
#[cfg(target_os = "linux")]
#[test]
fn child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree() {
    let ceiling = 32 * 1024 * 1024;
    let harness = memory_member("a", ceiling);
    let stand_in = StandIn::running("child-memory-overage", CHILD_MEMORY_OVERAGE);
    let evidence = execute_kani_obligation(&stand_in.request(&harness, T)).unwrap();
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::MemoryExhausted
        }
    );
    assert_eq!(evidence.ceilings, harness.identity.ceilings);
    assert_eq!(
        evidence.memory.mechanism,
        MemoryMechanism::LinuxPidNamespaceProcfsTreeRss
    );
    assert!(evidence.memory.peak_resident_bytes.unwrap() > ceiling);
    let parent_kib: u64 = fs::read_to_string(stand_in.directory.join("calls.parent-rss"))
        .unwrap()
        .parse()
        .unwrap();
    assert!(
        parent_kib * 1024 < ceiling,
        "the launcher's own resident memory must be below the ceiling"
    );
    let groups = fs::read_to_string(stand_in.directory.join("calls.groups")).unwrap();
    let groups: Vec<u32> = groups
        .split_whitespace()
        .map(|pid| pid.parse().unwrap())
        .collect();
    assert_eq!(groups.len(), 2);
    assert_ne!(
        groups[0], groups[1],
        "the allocating descendant must have left the launcher group"
    );
    recorded_process_gone(&stand_in.directory.join("calls.child"));
    recorded_process_gone(&stand_in.directory.join("calls.sibling"));
    assert_eq!(evidence.exit_code, None);
    assert_eq!(evidence.success_checks, 0);
    assert!(evidence.checks.is_empty());
    assert_eq!(
        serde_json::to_value(evidence).unwrap()["outcome"]["reason"],
        "memory_exhausted"
    );
}

/// Trace: FR-028-AC-21.
// Exercises the real Linux namespace/procfs bounded execution API.
#[cfg(target_os = "linux")]
#[test]
fn batch_memory_overage_refuses_every_member_without_classifying_a_partial_report() {
    let ceiling = 32 * 1024 * 1024;
    let harnesses = [memory_member("a", ceiling), memory_member("b", ceiling)];
    let stand_in = StandIn::running("batch-child-memory-overage", CHILD_MEMORY_OVERAGE);
    let refusal = only_group(stand_in.batch(&harnesses, T)).unwrap_err();
    match refusal {
        KaniExecutionRefusal::BatchMemoryExhausted {
            members,
            memory_bytes,
            memory,
        } => {
            assert_eq!(members, 2);
            assert_eq!(memory_bytes, ceiling);
            assert_eq!(
                memory.mechanism,
                MemoryMechanism::LinuxPidNamespaceProcfsTreeRss
            );
            assert!(memory.peak_resident_bytes.unwrap() > ceiling);
        }
        other => panic!("a memory-overage batch must be refused as a whole: {other}"),
    }
    recorded_process_gone(&stand_in.directory.join("calls.child"));
    recorded_process_gone(&stand_in.directory.join("calls.sibling"));
    assert_eq!(stand_in.calls().len(), 1);
}

/// Trace: FR-028-AC-2, FR-028-AC-4, FR-028-AC-21.
// Exercises the real Linux namespace/procfs bounded execution API.
#[cfg(target_os = "linux")]
#[test]
fn identity_ceilings_govern_execution_and_successful_evidence_records_observed_memory() {
    let mut harness = memory_member("a", 128 * 1024 * 1024);
    harness.identity.state_fields = vec!["input".to_owned()];
    harness.identity.domains = vec![StateFieldDomain {
        field: "input".to_owned(),
        minimum: -3,
        maximum: 7,
    }];
    let verifying = StandIn::verifying("successful-memory-bound");
    let evidence = execute_kani_obligation(&verifying.request(&harness, T)).unwrap();
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
    assert_eq!(evidence.ceilings, harness.identity.ceilings);
    assert_eq!(
        evidence.symbolic_arguments,
        vec![SymbolicArgumentBounds {
            identifier: "input".to_owned(),
            bounds: SymbolicBounds::Integer {
                minimum: -3,
                maximum: 7
            },
        }]
    );
    assert_eq!(
        evidence.memory.mechanism,
        MemoryMechanism::LinuxPidNamespaceProcfsTreeRss
    );
    assert!(evidence.memory.peak_resident_bytes.is_some());
    harness.identity.ceilings.wall_clock = Duration::from_millis(200);
    let stopped = StandIn::running("identity-wall-clock-bound", "sleep 45");
    let evidence =
        execute_kani_obligation(&stopped.request(&harness, Duration::from_millis(200))).unwrap();
    assert_eq!(evidence.ceilings.wall_clock, Duration::from_millis(200));
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::TimedOut
        }
    );
    harness.identity.ceilings.memory_bytes = std::num::NonZeroU64::MIN;
    let stopped = StandIn::running("identity-memory-bound", "sleep 45");
    let evidence =
        execute_kani_obligation(&stopped.request(&harness, Duration::from_millis(200))).unwrap();
    assert_eq!(evidence.ceilings.memory_bytes, std::num::NonZeroU64::MIN);
    assert_eq!(
        evidence.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::MemoryExhausted
        }
    );
}

/// Trace: FR-028-AC-21.
// Exercises the real Linux namespace/procfs bounded execution API.
#[cfg(target_os = "linux")]
#[test]
fn unequal_identity_memory_ceilings_run_in_separate_backend_processes() {
    let harnesses = [
        memory_member("a", 128 * 1024 * 1024),
        memory_member("b", 256 * 1024 * 1024),
    ];
    let stand_in = StandIn::verifying("different-memory-ceilings");
    let requests: Vec<_> = harnesses
        .iter()
        .map(|harness| stand_in.request(harness, T))
        .collect();
    let groups = execute_kani_obligations(&requests).unwrap();
    assert_eq!(groups.len(), 2);
    assert_eq!(stand_in.calls().len(), 2);
    for (group, harness) in groups.into_iter().zip(&harnesses) {
        let evidence = group.evidence.unwrap();
        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].ceilings, harness.identity.ceilings);
        assert!(evidence[0].batch.is_none());
    }
}

/// What a launcher stand-in does before anything else: it records that it ran and what it was
/// given, and finds the report path (its last argument) and the harnesses it was asked for.
const PROLOGUE: &str = r#"printf 'RUN\n' >> "$CALLS"
for argument; do printf '%s\n' "$argument" >> "$CALLS"; done
last=; ids=
while [ $# -gt 0 ]; do
  last="$1"
  if [ "$1" = --harness ]; then ids="$ids $2"; last="$2"; shift; fi
  shift
done
"#;

/// A stand-in that reports every harness it is asked for as verified: one passed check and
/// one satisfied cover.
const VERIFY_EVERY_HARNESS: &str = r#"results=; sep=
for id in $ids; do
  results="$results$sep{\"harness_id\":\"$id\",\"status\":\"Success\",\"checks\":[{\"id\":1,\"status\":\"Success\",\"category\":\"assertion\",\"location\":{\"file\":\"src/lib.rs\",\"line\":\"1\"}},{\"id\":2,\"status\":\"Satisfied\",\"category\":\"cover\",\"location\":{\"file\":\"src/lib.rs\",\"line\":\"2\"}}]}"
  sep=,
done
printf '{"metadata":{"version":"1.0"},"verification_results":{"results":[%s]}}' "$results" > "$last"
exit 0
"#;

fn member(module: &str, harness: &str, unwind: u32) -> StateFrameHarness {
    let mut built = named_state_frame_harness(
        module,
        harness,
        StateFrameProperty::Frame {
            granted: Vec::new(),
            checked: Vec::new(),
        },
        [
            "-Z".to_owned(),
            "function-contracts".to_owned(),
            "-Z".to_owned(),
            "concrete-playback".to_owned(),
            "--harness".to_owned(),
            format!("{module}::{harness}"),
            "--exact".to_owned(),
            "--unwind".to_owned(),
            unwind.to_string(),
            "--solver".to_owned(),
            "cadical".to_owned(),
            "--output-format".to_owned(),
            "regular".to_owned(),
            "--concrete-playback".to_owned(),
            "print".to_owned(),
        ]
        .to_vec(),
    );
    built.identity.unwind = unwind;
    built
}

// Expected selections for the Linux full bounded-process count test only.
#[cfg(target_os = "linux")]
fn path_of(harness: &StateFrameHarness) -> String {
    format!(
        "{}::{}",
        harness.identity.module_symbol, harness.identity.harness_symbol
    )
}

/// A launcher stand-in in a scratch crate and target directory.
struct StandIn {
    directory: PathBuf,
    crate_directory: PathBuf,
    target_directory: PathBuf,
    installation: KaniInstallation,
}

impl StandIn {
    /// A scratch crate and target directory, and the path a launcher will be installed at.
    fn scaffold(name: &str) -> Self {
        let directory = discover_scratch(name);
        let crate_directory = directory.join("crate");
        let target_directory = directory.join("target");
        fs::create_dir_all(crate_directory.join("src")).unwrap();
        fs::create_dir_all(&target_directory).unwrap();
        fs::write(crate_directory.join("src/lib.rs"), "").unwrap();
        let launcher = directory.join("cargo-kani");
        Self {
            directory,
            crate_directory,
            target_directory,
            installation: KaniInstallation { launcher },
        }
    }

    /// Installs the launcher: the prologue, then `body`.
    fn install(&self, body: &str) {
        write_launcher(
            &self.installation.launcher,
            &format!(
                "CALLS='{}'\n{PROLOGUE}{body}\n",
                self.directory.join("calls").display()
            ),
        );
    }

    /// A stand-in running `body` after its prologue.
    fn running(name: &str, body: &str) -> Self {
        let stand_in = Self::scaffold(name);
        stand_in.install(body);
        stand_in
    }

    /// A stand-in that verifies every harness it is asked for.
    fn verifying(name: &str) -> Self {
        Self::running(name, VERIFY_EVERY_HARNESS)
    }

    /// A stand-in that exports `report` (when given), prints `console` and exits with the
    /// status `exit`.
    fn replaying(name: &str, report: Option<&[u8]>, console: &str, exit: i32) -> Self {
        let stand_in = Self::scaffold(name);
        let mut body = String::new();
        if let Some(report) = report {
            let exported = stand_in.directory.join("report.json");
            fs::write(&exported, report).unwrap();
            body.push_str(&format!("cp '{}' \"$last\"\n", exported.display()));
        }
        let printed = stand_in.directory.join("console");
        fs::write(&printed, console).unwrap();
        body.push_str(&format!("cat '{}'\nexit {exit}\n", printed.display()));
        stand_in.install(&body);
        stand_in
    }

    fn request<'a>(
        &'a self,
        harness: &'a StateFrameHarness,
        timeout: Duration,
    ) -> KaniExecutionRequest<'a> {
        assert_eq!(harness.identity.ceilings.wall_clock, timeout);
        KaniExecutionRequest {
            guardian_path: guardian_path(),
            installation: &self.installation,
            harness: harness.into(),
            crate_directory: &self.crate_directory,
            target_directory: &self.target_directory,
        }
    }

    /// Runs `harnesses` as a batch at `timeout`.
    fn batch(
        &self,
        harnesses: &[StateFrameHarness],
        timeout: Duration,
    ) -> Result<Vec<KaniGroupRun>, KaniExecutionRefusal> {
        let harnesses: Vec<_> = harnesses
            .iter()
            .cloned()
            .map(|mut harness| {
                harness.identity.ceilings.wall_clock = timeout;
                harness
            })
            .collect();
        let requests: Vec<_> = harnesses
            .iter()
            .map(|harness| self.request(harness, timeout))
            .collect();
        execute_kani_obligations(&requests)
    }

    /// Exercise actual bounded execution on Linux and shared POSIX report semantics on
    /// unsupported targets. Only the Linux result carries a memory-enforcement attestation.
    fn reported_batch(
        &self,
        harnesses: &[StateFrameHarness],
        timeout: Duration,
    ) -> Result<Vec<report_fixture::ReportGroup>, KaniExecutionRefusal> {
        let harnesses: Vec<_> = harnesses
            .iter()
            .cloned()
            .map(|mut harness| {
                harness.identity.ceilings.wall_clock = timeout;
                harness
            })
            .collect();
        let requests: Vec<_> = harnesses
            .iter()
            .map(|harness| self.request(harness, timeout))
            .collect();
        report_fixture::groups(&requests)
    }

    /// The argument vector of each process the stand-in ran, as it received them.
    fn calls(&self) -> Vec<Vec<String>> {
        let recorded = fs::read_to_string(self.directory.join("calls")).unwrap_or_default();
        let mut calls: Vec<Vec<String>> = Vec::new();
        for line in recorded.lines() {
            if line == "RUN" {
                calls.push(Vec::new());
            } else if let Some(call) = calls.last_mut() {
                call.push(line.to_owned());
            }
        }
        calls
    }
}

impl Drop for StandIn {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

/// The evidence of the one process of a batch that ran as one group, or its refusal.
#[cfg(target_os = "linux")]
fn only_group(
    runs: Result<Vec<KaniGroupRun>, KaniExecutionRefusal>,
) -> Result<Vec<KaniExecutionEvidence>, KaniExecutionRefusal> {
    let mut runs = runs.expect("no harness is missing from the crate");
    assert_eq!(runs.len(), 1, "the harnesses share one process");
    runs.remove(0).evidence
}

#[cfg(target_os = "linux")]
fn outcomes(evidence: &[KaniExecutionEvidence]) -> Vec<KaniRunOutcome> {
    evidence
        .iter()
        .map(|evidence| evidence.outcome.clone())
        .collect()
}

fn only_report_group(
    runs: Result<Vec<report_fixture::ReportGroup>, KaniExecutionRefusal>,
) -> Result<Vec<report_fixture::FixtureExecution>, KaniExecutionRefusal> {
    let mut runs = runs.expect("no harness is missing from the crate");
    assert_eq!(runs.len(), 1, "the harnesses share one process");
    runs.remove(0).reports
}

fn reported_outcomes(reports: &[report_fixture::FixtureExecution]) -> Vec<KaniRunOutcome> {
    reports
        .iter()
        .map(|reported| reported.outcome.clone())
        .collect()
}

const PLAYBACK_BODY: &str = "fn from_{}() { kani::concrete_playback_run(v, h); }";

/// A playback block headed for `path` that checks a `kind` property.
fn block(path: &str, kind: &str, name: &str) -> String {
    format!(
        "Concrete playback unit test for `{path}`:\n```\n/// Test generated for harness `{path}`\n///\n/// Check for `{kind}`: \"c\"\n\n#[test]\n{}\n```\n",
        PLAYBACK_BODY.replace("{}", name)
    )
}

fn entry<'a>(
    harness_id: &'a str,
    status: &'a str,
    checks: &'a [(&'a str, &'a str)],
) -> BatchEntry<'a> {
    BatchEntry {
        harness_id,
        status,
        checks,
        exit_status: None,
    }
}

const FAILED: (&str, &str) = ("Failure", "assertion");

fn pair() -> Vec<StateFrameHarness> {
    vec![member("a", "check", 4), member("b", "check", 4)]
}

/// Trace: FR-028-AC-21.
#[cfg(not(target_os = "linux"))]
#[test]
fn unsupported_platform_refuses_single_and_batch_before_backend_dispatch() {
    let stand_in = StandIn::verifying("unsupported-platform");
    let harnesses = pair();
    assert!(matches!(
        execute_kani_obligation(&stand_in.request(&harnesses[0], T)),
        Err(KaniExecutionRefusal::MemoryMechanismUnavailable { .. })
    ));
    let runs = stand_in.batch(&harnesses, T).unwrap();
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].members, [0, 1]);
    assert!(matches!(
        runs[0].evidence,
        Err(KaniExecutionRefusal::MemoryMechanismUnavailable { .. })
    ));
    assert!(
        !stand_in.directory.join("calls").exists(),
        "backend never dispatched"
    );
}

/// N harnesses with equal options and timeout start one launcher process, whose arguments hold
/// one `--harness <module::harness> --exact` pair per member in request order, then
/// `--harness-timeout` T, then the shared options; one harness starts one process with the
/// single-run vector. Counting the processes before (one run each) and after batching gives
/// N and 1 for N = 1, 10 and 50.
///
/// Trace: FR-017-AC-21, FR-017-AC-22, TC-043
// Exercises the real Linux namespace/procfs bounded execution API.
#[cfg(target_os = "linux")]
#[test]
fn tc_043_compatible_harnesses_start_one_process_with_one_selection_each_in_request_order() {
    for count in [1_usize, 10, 50] {
        let harnesses: Vec<_> = (0..count)
            .map(|index| member(&format!("m{index}"), "check", 4))
            .collect();

        let before = StandIn::verifying("batching-before");
        for harness in &harnesses {
            execute_kani_obligation(&before.request(harness, T)).unwrap();
        }
        assert_eq!(before.calls().len(), count, "one process each, before");

        let after = StandIn::verifying("batching-after");
        let runs = after.batch(&harnesses, T).unwrap();
        let calls = after.calls();
        assert_eq!(
            calls.len(),
            1,
            "one process for {count} compatible harnesses"
        );
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].members, (0..count).collect::<Vec<_>>());
        let evidence = runs.into_iter().next().unwrap().evidence.unwrap();
        assert_eq!(evidence.len(), count);
        assert!(evidence
            .iter()
            .all(|evidence| evidence.outcome == KaniRunOutcome::Verified));

        let received = &calls[0];
        let (report_flag, export) = received.split_at(received.len() - 1);
        assert_eq!(
            report_flag[report_flag.len() - 3..],
            ["-Z", "unstable-options", "--export-json"]
        );
        assert!(
            export[0].starts_with(&format!(
                "{}/quire-kani-report-",
                after.target_directory.display()
            )),
            "{export:?}"
        );
        let mut expected = vec!["kani".to_owned()];
        if count == 1 {
            expected.extend(harnesses[0].identity.options.iter().cloned());
        } else {
            for harness in &harnesses {
                expected.extend([
                    "--harness".to_owned(),
                    path_of(harness),
                    "--exact".to_owned(),
                ]);
            }
            expected.extend(["--harness-timeout".to_owned(), "30".to_owned()]);
            expected.extend(SHARED_OPTIONS.map(str::to_owned));
        }
        expected.extend(["-Z", "unstable-options", "--export-json"].map(str::to_owned));
        assert_eq!(report_flag, expected, "the vector the launcher received");
        for evidence in &evidence {
            assert_eq!(
                evidence.arguments, *received,
                "evidence names the real vector"
            );
            assert_eq!(evidence.batch.is_some(), count > 1);
            // Public wire claims must use actual bounded evidence, not a report fixture.
            let wire = serde_json::to_value(evidence).unwrap();
            if count == 1 {
                assert!(wire.get("batch").is_none());
            } else {
                let selections: Vec<_> = harnesses.iter().map(path_of).collect();
                assert_eq!(
                    wire["batch"]["members"],
                    serde_json::to_value(selections).unwrap()
                );
                assert_eq!(wire["batch"]["timeoutSeconds"], 30);
            }
        }
    }
}

/// Harnesses with a different option vector or a different request timeout do not share a
/// process: N harnesses in G such groups start G processes, each group's members in request
/// order.
///
/// Trace: FR-017-AC-21, TC-043
#[test]
fn tc_043_harnesses_group_by_option_vector_and_by_request_timeout() {
    // Two unwind bounds times two timeouts, two harnesses in each: eight, four groups,
    // requested interleaved so that a group is not a contiguous run of the request list.
    let cells = [(4, 30), (5, 30), (4, 60), (5, 60)];
    let harnesses: Vec<_> = (0..8)
        .map(|index| {
            let mut harness = member(&format!("m{index}"), "check", cells[index % 4].0);
            harness.identity.ceilings.wall_clock = Duration::from_secs(cells[index % 4].1);
            harness
        })
        .collect();
    let stand_in = StandIn::verifying("grouping");
    let requests: Vec<_> = harnesses
        .iter()
        .enumerate()
        .map(|(index, harness)| stand_in.request(harness, Duration::from_secs(cells[index % 4].1)))
        .collect();
    let runs = report_fixture::groups(&requests).unwrap();
    assert_eq!(stand_in.calls().len(), 4, "eight harnesses in four groups");
    assert_eq!(
        runs.iter()
            .map(|run| run.members.clone())
            .collect::<Vec<_>>(),
        [vec![0, 4], vec![1, 5], vec![2, 6], vec![3, 7]]
    );
    for run in runs {
        assert!(run
            .reports
            .unwrap()
            .iter()
            .all(|evidence| evidence.batch.is_some()));
    }
    // Equal options and timeouts again but in other crates would be two processes; with
    // one crate, one harness of each shape is two single runs, not a batch.
    let separate = StandIn::verifying("grouping-separate");
    let pair = [member("x", "check", 4), member("y", "check", 5)];
    let runs = separate.reported_batch(&pair, T).unwrap();
    assert_eq!(separate.calls().len(), 2);
    assert!(runs.iter().all(|run| run
        .reports
        .as_ref()
        .is_ok_and(|evidence| evidence.len() == 1 && evidence[0].batch.is_none())));
}

/// A process has one launcher, one working directory and one target directory: requests that
/// name another of any of the three are never grouped, however equal their options and
/// timeouts. Each case runs two requests and counts the processes each launcher was started
/// as.
///
/// Trace: FR-017-AC-21, TC-043
#[test]
fn tc_043_requests_naming_another_launcher_crate_or_target_directory_are_not_grouped() {
    let (a, b) = (member("a", "check", 4), member("b", "check", 4));

    // Another launcher: each launcher runs one harness.
    let (first, second) = (StandIn::verifying("key-l1"), StandIn::verifying("key-l2"));
    let requests = [
        first.request(&a, T),
        KaniExecutionRequest {
            guardian_path: guardian_path(),
            installation: &second.installation,
            ..first.request(&b, T)
        },
    ];
    let runs = report_fixture::groups(&requests).unwrap();
    assert_eq!(runs.len(), 2);
    assert_eq!((first.calls().len(), second.calls().len()), (1, 1));

    // Another crate directory: two processes of the one launcher, one in each crate.
    let (first, second) = (StandIn::verifying("key-c1"), StandIn::verifying("key-c2"));
    let requests = [
        first.request(&a, T),
        KaniExecutionRequest {
            guardian_path: guardian_path(),
            crate_directory: &second.crate_directory,
            ..first.request(&b, T)
        },
    ];
    let runs = report_fixture::groups(&requests).unwrap();
    assert_eq!(runs.len(), 2);
    assert_eq!(first.calls().len(), 2);

    // Another target directory: the same.
    let (first, second) = (StandIn::verifying("key-t1"), StandIn::verifying("key-t2"));
    let requests = [
        first.request(&a, T),
        KaniExecutionRequest {
            guardian_path: guardian_path(),
            target_directory: &second.target_directory,
            ..first.request(&b, T)
        },
    ];
    let runs = report_fixture::groups(&requests).unwrap();
    assert_eq!(runs.len(), 2);
    assert_eq!(first.calls().len(), 2);
}

/// A batch of N is bounded by N times T, not by T: four members at T = 3 s run for eight
/// seconds, longer than T and than N times T halved (6 s), and inside N times T (12 s), and
/// complete with every member verified.
///
/// The stand-in's sleep never ends early, so it outlasts T and N times T halved however the
/// scheduler treats the test; only the 4 s between it and the 12 s bound can be eaten by a
/// loaded host (an earlier 1 s / 2 s / 4 s version left 2 s and flaked on a loaded runner).
///
/// Trace: FR-017-AC-21, FR-028-AC-12, TC-039, TC-043
// Exercises the real Linux namespace/procfs bounded execution API.
#[cfg(target_os = "linux")]
#[test]
fn tc_043_a_batch_may_run_longer_than_t_when_it_is_inside_n_times_t() {
    let stand_in = StandIn::running(
        "inside-n-times-t",
        &format!("sleep 8\n{VERIFY_EVERY_HARNESS}"),
    );
    let harnesses: Vec<_> = (0..4)
        .map(|index| member(&format!("m{index}"), "check", 4))
        .collect();
    let evidence = only_group(stand_in.batch(&harnesses, Duration::from_secs(3))).unwrap();
    assert_eq!(
        outcomes(&evidence),
        vec![KaniRunOutcome::Verified; 4],
        "a bound of T, not N times T, would have killed this batch"
    );
}

/// A batch still running at N times T is killed and refused as timed out, and no member is
/// classified: the stand-in sleeps far past the bound and writes no report.
///
/// Trace: FR-017-AC-21, FR-028-AC-12, TC-039, TC-043
// Exercises the real Linux namespace/procfs bounded execution API.
#[cfg(target_os = "linux")]
#[test]
fn tc_043_a_batch_still_running_at_its_outer_bound_is_refused_as_timed_out() {
    let stand_in = StandIn::running("outer-bound", "sleep 45");
    let outcome = only_group(stand_in.batch(&pair(), Duration::from_millis(200)));
    assert!(
        matches!(
            outcome,
            Err(KaniExecutionRefusal::BatchTimedOut { members: 2, timeout })
                if timeout == Duration::from_millis(200)
        ),
        "{outcome:?}"
    );
    assert_eq!(
        fs::read_dir(&stand_in.target_directory).unwrap().count(),
        0,
        "the killed batch's report path is removed"
    );
}

/// A `Duration::MAX` batch launches without `--harness-timeout` and still runs (its bound
/// never elapses); a batch at 4294967295 seconds carries it and runs.
///
/// Trace: FR-017-AC-21, FR-028-AC-12, TC-039, TC-043
#[test]
fn tc_043_a_duration_max_batch_runs_without_a_member_timeout_and_the_maximum_runs_with_one() {
    let without = StandIn::verifying("duration-max");
    let evidence = only_report_group(without.reported_batch(&pair(), Duration::MAX)).unwrap();
    assert!(evidence
        .iter()
        .all(|evidence| evidence.outcome == KaniRunOutcome::Verified));
    assert!(!without.calls()[0].contains(&"--harness-timeout".to_owned()));
    assert_eq!(
        evidence[0].batch.as_ref().unwrap().timeout_seconds,
        u64::MAX
    );

    let maximum = StandIn::verifying("kani-maximum");
    let seconds = u64::from(u32::MAX);
    let evidence =
        only_report_group(maximum.reported_batch(&pair(), Duration::from_secs(seconds))).unwrap();
    assert!(evidence
        .iter()
        .all(|evidence| evidence.outcome == KaniRunOutcome::Verified));
    let received = &maximum.calls()[0];
    let at = received
        .iter()
        .position(|a| a == "--harness-timeout")
        .unwrap();
    assert_eq!(received[at + 1], seconds.to_string());
}

/// A batch that exits successfully and exported no report is refused as a single run is, and
/// no member is classified; one that exits unsuccessfully with none leaves every member
/// inconclusive `NoVerdict`, carrying the exit code and the batch statement.
///
/// Trace: FR-017-AC-21, TC-043
#[test]
fn tc_043_a_batch_with_no_report_is_refused_after_a_clean_exit_and_no_verdict_after_a_failed_one() {
    let clean = StandIn::replaying("no-report-clean", None, "", 0);
    assert!(matches!(
        only_report_group(clean.reported_batch(&pair(), T)),
        Err(KaniExecutionRefusal::Report(KaniReportRefusal::Missing))
    ));
    let failed = StandIn::replaying("no-report-failed", None, "error: bad argument", 2);
    let evidence = only_report_group(failed.reported_batch(&pair(), T)).unwrap();
    assert_eq!(
        reported_outcomes(&evidence),
        vec![
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::NoVerdict
            };
            2
        ]
    );
    for evidence in &evidence {
        assert_eq!(evidence.exit_code, Some(2));
        assert_eq!(evidence.success_checks, 0);
        assert!(evidence.checks.is_empty());
        assert_eq!(
            evidence.batch.as_ref().unwrap().members,
            ["a::check", "b::check"]
        );
    }
}

/// Members are matched by `module::harness` path, never by the bare symbol both share, and
/// each member's outcome, checks and SUCCESS count come from its own entry: the report lists
/// the members in the reverse of the request order, one falsified and one verified.
///
/// Trace: FR-017-AC-22, TC-043
#[test]
fn tc_043_members_are_matched_by_path_and_classified_from_their_own_entry() {
    let report = batch_report(&[
        entry("b::check", "Failure", &[FAILED, COVER_OK]),
        entry("a::check", "Success", &[PASSED, PASSED, COVER_OK]),
    ]);
    let console = format!(
        "{}{}{}",
        block("a::check", "cover", "cover_a"),
        block("b::check", "cover", "cover_b"),
        block("b::check", "assertion", "falsify_b")
    );
    let stand_in = StandIn::replaying("by-path", Some(&report), &console, 1);
    let evidence = only_report_group(stand_in.reported_batch(&pair(), T)).unwrap();
    assert_eq!(evidence[0].harness_path, "src/generated/a.rs");
    assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
    assert_eq!(evidence[0].success_checks, 2);
    assert_eq!(evidence[0].checks.len(), 3);
    assert_eq!(evidence[1].harness_path, "src/generated/b.rs");
    assert!(matches!(
        &evidence[1].outcome,
        KaniRunOutcome::Falsified { counterexample } if counterexample.contains("fn from_falsify_b")
    ));
    assert_eq!(evidence[1].success_checks, 0);
    assert_eq!(evidence[1].checks.len(), 2);
}

/// Each member's evidence carries its kind, harness path, launcher path, unwind bound, solver,
/// outcome and checks beside the batch's argument vector, the member list, the statement that
/// it was a batch and the exit code.
///
/// Trace: FR-017-AC-22, TC-043
#[test]
fn tc_043_a_member_evidence_carries_its_own_fields_and_the_batch_invocation() {
    let report = batch_report(&[
        entry("a::check", "Success", &[PASSED, COVER_OK]),
        entry("b::check", "Success", &[PASSED, COVER_OK]),
    ]);
    let stand_in = StandIn::replaying("evidence-fields", Some(&report), "", 0);
    // Members share one option vector (so one process) but each evidence reads its own
    // identity's unwind bound.
    let mut second = member("b", "check", 4);
    second.identity.unwind = 7;
    let harnesses = [member("a", "check", 4), second];
    let evidence = only_report_group(stand_in.reported_batch(&harnesses, T)).unwrap();
    let received = stand_in.calls().remove(0);
    for (evidence, expected_unwind, module) in [(&evidence[0], 4, "a"), (&evidence[1], 7, "b")] {
        assert_eq!(evidence.kind, Some(ObligationKind::Frame));
        assert_eq!(evidence.harness_path, format!("src/generated/{module}.rs"));
        assert_eq!(
            evidence.launcher_path,
            stand_in.installation.launcher.display().to_string()
        );
        assert_eq!(evidence.unwind, expected_unwind);
        assert_eq!(evidence.solver, KaniSolver::Cadical);
        assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
        assert_eq!(evidence.checks.len(), 2);
        assert_eq!(evidence.arguments, received);
        assert_eq!(evidence.exit_code, Some(0));
        assert_eq!(
            evidence.batch,
            Some(KaniBatchInvocation {
                members: vec!["a::check".to_owned(), "b::check".to_owned()],
                timeout_seconds: 30,
            })
        );
    }
    // Only actual bounded execution yields public wire evidence. Unsupported hosts keep
    // every metadata assertion above without serializing a second wire contract.
    #[cfg(target_os = "linux")]
    for (actual, module) in evidence.iter().zip(["a", "b"]) {
        let wire = serde_json::to_value(actual).unwrap();
        assert_eq!(
            wire["launcherPath"],
            stand_in.installation.launcher.display().to_string()
        );
        assert_eq!(wire["harnessPath"], format!("src/generated/{module}.rs"));
        assert_eq!(
            wire["kind"],
            serde_json::to_value(ObligationKind::Frame).unwrap()
        );
        assert_eq!(
            wire["solver"],
            serde_json::to_value(KaniSolver::Cadical).unwrap()
        );
        assert_eq!(wire["checks"].as_array().unwrap().len(), 2);
        assert_eq!(wire["batch"]["members"][1], "b::check");
        assert_eq!(wire["batch"]["timeoutSeconds"], 30);
    }
    // A single run's evidence has no `batch` member at all.
    let single = StandIn::verifying("evidence-single");
    let alone = report_fixture::single(&single.request(&harnesses[0], T)).unwrap();
    assert!(alone.batch.is_none());
    #[cfg(target_os = "linux")]
    assert!(serde_json::to_value(&alone).unwrap().get("batch").is_none());
}

/// A member's success is believed beside a non-zero exit only when some entry states the
/// failure that exit stands for: every member Success with exit 1 is inconclusive, and one
/// Failure entry makes the other member's success verified.
///
/// Trace: FR-017-AC-22, TC-043
#[test]
fn tc_043_a_success_beside_a_non_zero_exit_is_verified_only_when_an_entry_states_failure() {
    let no_verdict = KaniRunOutcome::Inconclusive {
        reason: KaniInconclusiveReason::NoVerdict,
    };
    let all_success = batch_report(&[
        entry("a::check", "Success", &[PASSED, COVER_OK]),
        entry("b::check", "Success", &[PASSED, COVER_OK]),
    ]);
    let unexplained = StandIn::replaying("exit-unexplained", Some(&all_success), "", 1);
    assert_eq!(
        reported_outcomes(&only_report_group(unexplained.reported_batch(&pair(), T)).unwrap()),
        vec![no_verdict.clone(); 2]
    );
    let explained_report = batch_report(&[
        entry("a::check", "Success", &[PASSED, COVER_OK]),
        entry("b::check", "Failure", &[FAILED, COVER_OK]),
    ]);
    let console = block("b::check", "assertion", "falsify_b");
    let explained = StandIn::replaying("exit-explained", Some(&explained_report), &console, 1);
    let evidence = only_report_group(explained.reported_batch(&pair(), T)).unwrap();
    assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
    assert!(matches!(
        evidence[1].outcome,
        KaniRunOutcome::Falsified { .. }
    ));
    // The same two entries and a zero exit are the same results: the exit only withholds.
    let clean = StandIn::replaying("exit-zero", Some(&all_success), "", 0);
    assert_eq!(
        reported_outcomes(&only_report_group(clean.reported_batch(&pair(), T)).unwrap()),
        vec![KaniRunOutcome::Verified; 2]
    );
}

/// An entry that states failure with no checks and exit status `timeout` is inconclusive as
/// timed out, naming T, while the others keep their results; one with no checks and no
/// timeout is inconclusive with no counterexample.
///
/// Trace: FR-017-AC-22, FR-028-AC-12, TC-039, TC-043
#[test]
fn tc_043_a_timed_out_entry_is_timed_out_naming_t_and_a_bare_failure_has_no_counterexample() {
    let report = batch_report(&[
        entry("a::check", "Success", &[PASSED, COVER_OK]),
        BatchEntry {
            harness_id: "b::check",
            status: "Failure",
            checks: &[],
            exit_status: Some("timeout"),
        },
        entry("c::check", "Failure", &[]),
        // Kani's timeout flag with checks listed: a failure that has checks is classified
        // from them, not as a cut-off.
        BatchEntry {
            harness_id: "d::check",
            status: "Failure",
            checks: &[FAILED],
            exit_status: Some("timeout"),
        },
    ]);
    let stand_in = StandIn::replaying("timeout-entry", Some(&report), "", 1);
    let harnesses = [
        member("a", "check", 4),
        member("b", "check", 4),
        member("c", "check", 4),
        member("d", "check", 4),
    ];
    // 90.5 s: Kani is told 91 whole seconds and the evidence names 91, rounded up.
    let evidence =
        only_report_group(stand_in.reported_batch(&harnesses, Duration::from_millis(90_500)))
            .unwrap();
    assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
    assert_eq!(
        evidence[1].outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::TimedOut
        }
    );
    assert_eq!(evidence[1].batch.as_ref().unwrap().timeout_seconds, 91);
    let received = &stand_in.calls()[0];
    let at = received
        .iter()
        .position(|a| a == "--harness-timeout")
        .unwrap();
    assert_eq!(
        received[at + 1],
        "91",
        "the T the timed-out member is told it had"
    );
    assert_eq!(
        evidence[2].outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::FailedWithoutCounterexample
        }
    );
    assert_eq!(
        evidence[3].outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::FailedWithoutCounterexample
        },
        "a failed check and no block for d: no counterexample, not a timeout"
    );
}

/// A batch report that lacks a requested harness, holds one twice or holds one that was not
/// requested refuses the whole batch with its own typed cause.
///
/// Trace: FR-017-AC-23, TC-043
#[test]
fn tc_043_a_report_that_lacks_repeats_or_adds_a_harness_refuses_the_batch() {
    let verified = [PASSED, COVER_OK];
    let refusal = |name: &str, entries: &[BatchEntry<'_>]| {
        let stand_in = StandIn::replaying(name, Some(&batch_report(entries)), "", 0);
        only_report_group(stand_in.reported_batch(&pair(), T)).unwrap_err()
    };
    assert!(matches!(
        refusal("lacks", &[entry("a::check", "Success", &verified)]),
        KaniExecutionRefusal::Report(KaniReportRefusal::HarnessMissing { harness })
            if harness == "b::check"
    ));
    assert!(matches!(
        refusal(
            "repeats",
            &[
                entry("a::check", "Success", &verified),
                entry("a::check", "Success", &verified),
                entry("b::check", "Success", &verified),
            ]
        ),
        KaniExecutionRefusal::Report(KaniReportRefusal::HarnessDuplicated { harness })
            if harness == "a::check"
    ));
    assert!(matches!(
        refusal(
            "adds",
            &[
                entry("a::check", "Success", &verified),
                entry("b::check", "Success", &verified),
                entry("c::check", "Success", &verified),
            ]
        ),
        KaniExecutionRefusal::Report(KaniReportRefusal::HarnessUnrequested { harness })
            if harness == "c::check"
    ));
}

/// A member whose source is not in the crate refuses the whole batch with
/// `HarnessNotInCrate` before any process starts, even when the other members are present.
///
/// Trace: FR-017-AC-23, TC-043
#[test]
fn tc_043_a_member_missing_from_the_crate_refuses_the_batch_and_starts_nothing() {
    let stand_in = StandIn::verifying("missing-member");
    let mut harnesses = pair();
    harnesses[1].rust = Artifact::new("src/generated/b.rs", "pub fn absent_from_the_crate() {}");
    match stand_in.batch(&harnesses, T) {
        Err(KaniExecutionRefusal::HarnessNotInCrate { harness_path }) => {
            assert_eq!(harness_path, "src/generated/b.rs");
        }
        other => panic!("expected HarnessNotInCrate, got {other:?}"),
    }
    assert!(stand_in.calls().is_empty(), "nothing was launched");
}

/// A falsified member's playback is the console block headed for its own path, never an
/// earlier member's failing block; a member whose entry names a failed property and has no
/// block headed for it is inconclusive with no counterexample, and its neighbours keep their
/// results.
///
/// Trace: FR-017-AC-23, TC-043
#[test]
fn tc_043_a_member_is_given_the_playback_headed_for_its_own_path() {
    let report = batch_report(&[
        entry("c::check", "Failure", &[FAILED, COVER_OK]),
        entry("a::check", "Failure", &[FAILED, COVER_OK]),
        entry("b::check", "Success", &[PASSED, COVER_OK]),
        entry("d::check", "Failure", &[FAILED, COVER_OK]),
    ]);
    // `d` is falsified but Kani printed no block headed for it.
    let console = format!(
        "{}{}{}{}",
        block("a::check", "assertion", "falsify_a"),
        block("b::check", "cover", "cover_b"),
        block("c::check", "cover", "cover_c"),
        block("c::check", "assertion", "falsify_c"),
    );
    let stand_in = StandIn::replaying("own-playback", Some(&report), &console, 1);
    let harnesses = [
        member("a", "check", 4),
        member("b", "check", 4),
        member("c", "check", 4),
        member("d", "check", 4),
    ];
    let evidence = only_report_group(stand_in.reported_batch(&harnesses, T)).unwrap();
    assert!(matches!(
        &evidence[0].outcome,
        KaniRunOutcome::Falsified { counterexample } if counterexample.contains("fn from_falsify_a")
    ));
    assert_eq!(evidence[1].outcome, KaniRunOutcome::Verified);
    assert!(
        matches!(
            &evidence[2].outcome,
            KaniRunOutcome::Falsified { counterexample }
                if counterexample.contains("fn from_falsify_c")
                    && !counterexample.contains("falsify_a")
        ),
        "c takes its own block, not a's earlier one: {:?}",
        evidence[2].outcome
    );
    assert_eq!(
        evidence[3].outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::FailedWithoutCounterexample
        }
    );
}

/// The playback section of Kani 0.68's console for a batch of `two::check`, whose two
/// assertions fail on independent paths at different valuations, and `ok::check`, verbatim
/// from a real run: one cover block and one counterexample block per distinct failing
/// valuation, all headed `two::check`, then
/// `ok::check`'s cover block. (The trailing space after the harness path in the `Test
/// generated` line is Kani's.)
const TWO_FAILED_CHECKS_CONSOLE: &str = r#"Concrete playback unit test for `two::check`:
```
/// Test generated for harness `two::check`
///
/// Check for `cover`: "x can be three"

#[test]
fn kani_concrete_playback_check_1077496887511954657() {
let concrete_vals: Vec<Vec<u8>> = vec![
    // 3
    vec![3],
];
kani::concrete_playback_run(concrete_vals, check);
}
```
Concrete playback unit test for `two::check`:
```
/// Test generated for harness `two::check`
///
/// Check for `assertion`: ""first""

#[test]
fn kani_concrete_playback_check_1550118722174123344() {
let concrete_vals: Vec<Vec<u8>> = vec![
    // 8
    vec![8],
    // 1
    vec![1],
];
kani::concrete_playback_run(concrete_vals, check);
}
```
Concrete playback unit test for `two::check`:
```
/// Test generated for harness `two::check`
///
/// Check for `assertion`: ""second""

#[test]
fn kani_concrete_playback_check_13421931990910725816() {
let concrete_vals: Vec<Vec<u8>> = vec![
    // 7
    vec![7],
    // 0
    vec![0],
];
kani::concrete_playback_run(concrete_vals, check);
}
```
INFO: To automatically add the concrete playback unit test(s) to the src code, run Kani with `--concrete-playback=inplace`.
Concrete playback unit test for `ok::check`:
```
/// Test generated for harness `ok::check`
///
/// Check for `cover`: "x can be three"

#[test]
fn kani_concrete_playback_check_1077496887511954657() {
let concrete_vals: Vec<Vec<u8>> = vec![
    // 3
    vec![3],
];
kani::concrete_playback_run(concrete_vals, check);
}
```
"#;

/// A member that fails two property checks has two counterexample blocks under its own path in
/// real Kani's console. It takes the first, as the same harness run alone does, and neither
/// it nor its verified neighbour loses its evidence: the group is not refused.
///
/// Trace: FR-017-AC-22, FR-017-AC-23, TC-043
#[test]
fn tc_043_a_member_failing_two_checks_takes_its_first_block_and_keeps_its_neighbour() {
    let report = batch_report(&[
        entry(
            "two::check",
            "Failure",
            &[("Satisfied", "cover"), FAILED, FAILED],
        ),
        entry("ok::check", "Success", &[PASSED, COVER_OK]),
    ]);
    let stand_in = StandIn::replaying(
        "two-failed-checks",
        Some(&report),
        TWO_FAILED_CHECKS_CONSOLE,
        1,
    );
    let harnesses = [member("two", "check", 4), member("ok", "check", 4)];
    let evidence = only_report_group(stand_in.reported_batch(&harnesses, T)).unwrap();
    let KaniRunOutcome::Falsified { counterexample } = &evidence[0].outcome else {
        panic!("two::check is falsified: {:?}", evidence[0].outcome);
    };
    assert!(counterexample.contains("\"\"first\"\""), "{counterexample}");
    assert!(!counterexample.contains("second"), "{counterexample}");
    assert_eq!(evidence[0].success_checks, 0);
    assert_eq!(evidence[1].outcome, KaniRunOutcome::Verified);
    // The same harness alone, with the same console, is falsified with that same block.
    let alone = classify_kani_run(
        false,
        Some(&batch_report_single_two()),
        TWO_FAILED_CHECKS_CONSOLE,
        None,
    )
    .unwrap();
    assert_eq!(alone.outcome, evidence[0].outcome);
}

/// The report of `two::check` alone, as a single run exports it.
fn batch_report_single_two() -> Vec<u8> {
    batch_report(&[entry(
        "two::check",
        "Failure",
        &[("Satisfied", "cover"), FAILED, FAILED],
    )])
}

/// A console block headed for a path that is not a member refuses the whole group and
/// classifies no member.
///
/// Trace: FR-017-AC-23, TC-043
#[test]
fn tc_043_an_unattributable_playback_refuses_the_whole_group() {
    let report = batch_report(&[
        entry("a::check", "Failure", &[FAILED, COVER_OK]),
        entry("b::check", "Success", &[PASSED, COVER_OK]),
    ]);
    let stranger = format!(
        "{}{}",
        block("a::check", "assertion", "falsify_a"),
        block("z::check", "cover", "cover_z")
    );
    let stand_in = StandIn::replaying("playback-stranger", Some(&report), &stranger, 1);
    assert!(matches!(
        only_report_group(stand_in.reported_batch(&pair(), T)),
        Err(KaniExecutionRefusal::PlaybackForNonMember { harness }) if harness == "z::check"
    ));
}

/// A run over the limit for its harness count is refused with the stable code naming the
/// stream, the limit and the count: one harness at 8 MiB, a batch of two at 16 MiB; a batch
/// at exactly its limit completes with its real exit status.
///
/// Trace: FR-017-AC-14, FR-028-AC-21, TC-043
#[test]
fn tc_043_an_over_limit_stream_refuses_a_single_run_and_a_batch_by_its_member_count() {
    let flood = |bytes: usize, exit: i32| {
        // On Linux an observable descendant remains alive until the namespace is killed;
        // over-limit and completed conclusions must both confirm its teardown.
        #[cfg(target_os = "linux")]
        let descendant = r#"sleep 45 &
printf '%s' "$!" > "$CALLS.child"
printf '%s' "$(readlink /proc/self/ns/pid)" > "$CALLS.namespace"
"#;
        #[cfg(not(target_os = "linux"))]
        let descendant = "";
        format!("{descendant}head -c {bytes} /dev/zero; exit {exit}")
    };
    let one = StandIn::running("flood-one", &flood(CAPTURE_LIMIT + 1, 0));
    let harness = member("a", "check", 4);
    let single = report_fixture::single(&one.request(&harness, T)).unwrap_err();
    assert!(matches!(
        single,
        KaniExecutionRefusal::OutputOverLimit {
            stream: CaptureStream::Stdout,
            limit,
            harnesses: 1
        } if limit == CAPTURE_LIMIT
    ));
    assert_eq!(single.code(), Some("kani_output_over_limit"));
    #[cfg(target_os = "linux")]
    recorded_process_gone(&one.directory.join("calls.child"));

    let two = StandIn::running("flood-two", &flood(2 * CAPTURE_LIMIT + 1, 0));
    let batch = only_report_group(two.reported_batch(&pair(), T)).unwrap_err();
    assert!(matches!(
        batch,
        KaniExecutionRefusal::OutputOverLimit {
            harnesses: 2,
            limit,
            ..
        } if limit == 2 * CAPTURE_LIMIT
    ));
    #[cfg(target_os = "linux")]
    recorded_process_gone(&two.directory.join("calls.child"));

    let exact = StandIn::running("flood-exact", &flood(2 * CAPTURE_LIMIT, 1));
    let evidence = only_report_group(exact.reported_batch(&pair(), T)).unwrap();
    #[cfg(target_os = "linux")]
    recorded_process_gone(&exact.directory.join("calls.child"));
    assert!(evidence
        .iter()
        .all(|evidence| evidence.exit_code == Some(1)));
    assert_eq!(
        reported_outcomes(&evidence),
        vec![
            KaniRunOutcome::Inconclusive {
                reason: KaniInconclusiveReason::NoVerdict
            };
            2
        ]
    );
}

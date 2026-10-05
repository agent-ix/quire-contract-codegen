//! FR-017 harness batching on the real prover (IR-277, TC-043 steps 5, 7 and 10).
//!
//! The default lane checks batching against launcher stand-ins (`src/kani/run/execute.rs`). These
//! tests are `#[ignore]`d into the `kani` lane (`make kani`) and run real `cargo kani`: the
//! number of launcher processes before and after batching for N = 1, 10 and 50, a batch whose
//! members finish, falsify and are cut off by Kani's own per-harness timeout, and a batch at the
//! largest `--harness-timeout` Kani accepts. Evidence is written under
//! `CARGO_TARGET_TMPDIR/kani-batching-evidence`.

use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use quire_contract_codegen::{
    execute_kani_obligation, execute_kani_obligations, Artifact, HarnessSymbol,
    KaniExecutionEvidence, KaniExecutionRequest, KaniInconclusiveReason, KaniInstallation,
    KaniRunOutcome, KaniSolver, ModuleSymbol, StateFrameHarness, StateFrameIdentity,
    StateFrameProperty, StateFrameScope,
};
use quire_contract_model::CheckedNodeId;
use serde_json::json;

/// Budget for a real run: a ceiling against a genuine hang, not a performance target.
const REAL_KANI_TIMEOUT: Duration = Duration::from_secs(600);

fn node_id(digit: &str) -> CheckedNodeId {
    serde_json::from_value(json!({
        "domain": "quire.checked-semantic-node/v1",
        "digest": digit.repeat(64),
    }))
    .expect("a node id")
}

const VERIFIED: &str = "        let x: u8 = kani::any();\n        kani::assume(x < 10);\n        kani::cover!(x == 3, \"x can be three\");\n        assert!(x < 10);";
const FALSIFIED: &str = "        let x: u8 = kani::any();\n        kani::assume(x < 10);\n        kani::cover!(x == 3, \"x can be three\");\n        assert!(x < 5);";
/// Two assertions that fail on independent paths, at different valuations: Kani prints one
/// counterexample block for each (one block per distinct valuation).
const TWO_FAILED_CHECKS: &str = "        let x: u8 = kani::any();\n        kani::assume(x < 10);\n        kani::cover!(x == 3, \"x can be three\");\n        if kani::any() {\n            assert!(x < 5, \"first\");\n        } else {\n            assert!(x != 7, \"second\");\n        }";
/// A loop of 64-bit multiplications over symbolic values: still unfinished after a minute at unwind 400.
const SLOW: &str = "        let n: u32 = kani::any();\n        kani::assume(n < 400);\n        let mut i: u32 = 0;\n        let mut acc: u64 = 1;\n        while i < n {\n            acc = acc.wrapping_mul(acc ^ (i as u64) | 1).wrapping_add(kani::any::<u64>() % 7);\n            i += 1;\n        }\n        assert!(acc != 12_345_678_901_234);";

/// A harness whose source is a module `name` holding one proof `check` over `body`, run with the
/// option vector `--harness <name>::check --exact` and `unwind`.
fn harness(name: &str, body: &str, unwind: u32) -> StateFrameHarness {
    let source = format!(
        "#[cfg(kani)]\nmod {name} {{\n    #[kani::proof]\n    pub fn check() {{\n{body}\n    }}\n}}\n"
    );
    let options = [
        "-Z",
        "concrete-playback",
        "--harness",
        &format!("{name}::check"),
        "--exact",
        "--unwind",
        &unwind.to_string(),
        "--solver",
        "cadical",
        "--output-format",
        "regular",
        "--concrete-playback",
        "print",
    ]
    .map(str::to_owned)
    .to_vec();
    StateFrameHarness {
        identity: StateFrameIdentity {
            ceilings: quire_contract_codegen::ProofCeilings {
                memory_bytes: std::num::NonZeroU64::new(16 * 1024 * 1024 * 1024).unwrap(),
                wall_clock: REAL_KANI_TIMEOUT,
            },
            clause: node_id("1"),
            scope: StateFrameScope {
                operation: "check".to_owned(),
                object: node_id("2"),
                anchor: node_id("3"),
                frame: node_id("4"),
            },
            property: StateFrameProperty::Frame {
                granted: Vec::new(),
                checked: Vec::new(),
            },
            state_fields: Vec::new(),
            domains: Vec::new(),
            unranged: Vec::new(),
            state_path: "crate::State".to_owned(),
            subject_path: "crate::operate".to_owned(),
            module_symbol: ModuleSymbol::try_from(name).expect("a module symbol"),
            harness_symbol: HarnessSymbol::try_from("check").expect("a harness symbol"),
            solver: KaniSolver::Cadical,
            unwind,
            options,
        },
        rust: Artifact::new(format!("src/generated/{name}.rs"), source),
        record: Artifact::new(format!("kani-obligations/{name}.json"), String::new()),
    }
}

/// A scratch crate holding the source of every harness, and a launcher that counts its own
/// invocations before running the real `cargo-kani`.
struct Lane {
    directory: PathBuf,
    crate_directory: PathBuf,
    installation: KaniInstallation,
    counts: PathBuf,
}

impl Lane {
    fn new(label: &str, harnesses: &[StateFrameHarness]) -> Self {
        let real = KaniInstallation::discover().expect("cargo-kani is installed");
        let directory = std::env::temp_dir().join(format!(
            "quire-kani-batching-{label}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir_all(directory.join("crate/src")).unwrap();
        let library: String = harnesses
            .iter()
            .map(|harness| format!("{}\n", harness.rust.contents))
            .collect();
        fs::write(directory.join("crate/src/lib.rs"), library).unwrap();
        fs::write(
            directory.join("crate/Cargo.toml"),
            "[package]\nname = \"generated-batching\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[workspace]\n",
        )
        .unwrap();
        fs::write(
            directory.join("crate/build.rs"),
            "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
        )
        .unwrap();
        let counts = directory.join("counts");
        let launcher = directory.join("cargo-kani");
        fs::write(
            &launcher,
            format!(
                "#!/bin/sh\necho run >> '{}'\nexec '{}' \"$@\"\n",
                counts.display(),
                real.launcher.display()
            ),
        )
        .unwrap();
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755)).unwrap();
        Self {
            crate_directory: directory.join("crate"),
            directory,
            installation: KaniInstallation { launcher },
            counts,
        }
    }

    fn processes(&self) -> usize {
        fs::read_to_string(&self.counts)
            .map(|counts| counts.lines().count())
            .unwrap_or(0)
    }

    fn request<'a>(
        &'a self,
        harness: &'a StateFrameHarness,
        target: &'a Path,
        timeout: Duration,
    ) -> KaniExecutionRequest<'a> {
        assert_eq!(harness.identity.ceilings.wall_clock, timeout);
        KaniExecutionRequest {
            installation: &self.installation,
            harness: harness.into(),
            crate_directory: &self.crate_directory,
            target_directory: target,
        }
    }
}

impl Drop for Lane {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn target() -> PathBuf {
    let target = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-batching");
    fs::create_dir_all(&target).unwrap();
    target
}

fn evidence_directory() -> PathBuf {
    let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-batching-evidence");
    fs::create_dir_all(&directory).unwrap();
    directory
}

/// For N = 1, 10 and 50 harnesses with equal options and timeout, real Kani is launched N times
/// before batching and once after, every harness verifies either way, and each batched member's
/// evidence names the batch. The counts and wall times are written to
/// `process-counts.json`.
///
/// Trace: FR-017-AC-21, TC-043
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_043_real_kani_one_process_runs_n_harnesses_where_n_ran_before() {
    let target = target();
    let mut rows = Vec::new();
    for count in [1_usize, 10, 50] {
        let harnesses: Vec<_> = (0..count)
            .map(|index| harness(&format!("v{index}"), VERIFIED, 4))
            .collect();

        let before = Lane::new(&format!("before-{count}"), &harnesses);
        let started = Instant::now();
        for harness in &harnesses {
            let evidence =
                execute_kani_obligation(&before.request(harness, &target, REAL_KANI_TIMEOUT))
                    .unwrap_or_else(|refusal| panic!("single run: {refusal}"));
            assert_eq!(evidence.outcome, KaniRunOutcome::Verified);
        }
        let (before_processes, before_time) = (before.processes(), started.elapsed());

        let after = Lane::new(&format!("after-{count}"), &harnesses);
        let requests: Vec<_> = harnesses
            .iter()
            .map(|harness| after.request(harness, &target, REAL_KANI_TIMEOUT))
            .collect();
        let started = Instant::now();
        let runs = execute_kani_obligations(&requests).expect("every harness is in the crate");
        let (after_processes, after_time) = (after.processes(), started.elapsed());
        assert_eq!(runs.len(), 1, "one group for {count} compatible harnesses");
        let evidence = runs
            .into_iter()
            .next()
            .unwrap()
            .evidence
            .unwrap_or_else(|refusal| panic!("batch of {count}: {refusal}"));
        assert_eq!(evidence.len(), count);
        for evidence in &evidence {
            assert_eq!(evidence.outcome, KaniRunOutcome::Verified, "{evidence:?}");
            assert_eq!(evidence.batch.is_some(), count > 1);
        }

        assert_eq!(before_processes, count);
        assert_eq!(after_processes, 1);
        rows.push(json!({
            "harnesses": count,
            "processesBefore": before_processes,
            "processesAfter": after_processes,
            "wallMillisBefore": before_time.as_millis(),
            "wallMillisAfter": after_time.as_millis(),
        }));
    }
    fs::write(
        evidence_directory().join("process-counts.json"),
        serde_json::to_string_pretty(&rows).unwrap(),
    )
    .unwrap();
}

/// One real batch of a verified member, a falsified member, one that fails two assertions (two
/// counterexample blocks under its path) and one Kani's own per-harness timeout cuts off: the
/// verified member stays verified, each falsified member carries the playback Kani headed for its
/// own path (the first, for the one with two), and the cut-off member is inconclusive as timed
/// out. Two harnesses then run in a batch at the largest `--harness-timeout` Kani accepts.
///
/// Trace: FR-017-AC-21, FR-017-AC-22, FR-017-AC-23, FR-028-AC-12, TC-039, TC-043
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_043_real_kani_batch_keeps_each_members_own_result_and_times_out_one_member() {
    let target = target();
    let mut harnesses = [
        harness("verified", VERIFIED, 400),
        harness("falsified", FALSIFIED, 400),
        harness("two", TWO_FAILED_CHECKS, 400),
        harness("slow", SLOW, 400),
    ];
    let lane = Lane::new("mixed", &harnesses);
    // Warm the build first: the batch's outer bound is four times T and covers the compile of the
    // crate too, so a cold build must not eat into the time `slow` is given. `slow` runs for
    // well over a minute at this unwind bound, so T below cuts it off whatever the host does.
    let warm = execute_kani_obligation(&lane.request(&harnesses[0], &target, REAL_KANI_TIMEOUT))
        .unwrap_or_else(|refusal| panic!("the warm-up run: {refusal}"));
    assert_eq!(warm.outcome, KaniRunOutcome::Verified);
    let timeout = Duration::from_secs(20);
    for harness in &mut harnesses {
        harness.identity.ceilings.wall_clock = timeout;
    }
    let requests: Vec<_> = harnesses
        .iter()
        .map(|harness| lane.request(harness, &target, timeout))
        .collect();
    let evidence: Vec<KaniExecutionEvidence> = execute_kani_obligations(&requests)
        .expect("every harness is in the crate")
        .into_iter()
        .next()
        .unwrap()
        .evidence
        .unwrap_or_else(|refusal| panic!("the batch: {refusal}"));
    assert_eq!(
        lane.processes(),
        2,
        "the warm-up run and the one batch process"
    );
    fs::write(
        evidence_directory().join("mixed-batch.json"),
        serde_json::to_string_pretty(&evidence).unwrap(),
    )
    .unwrap();
    assert_eq!(evidence[0].outcome, KaniRunOutcome::Verified);
    assert!(
        matches!(
            &evidence[1].outcome,
            KaniRunOutcome::Falsified { counterexample }
                if counterexample.contains("falsified::check")
                    && counterexample.contains("Check for `assertion`")
        ),
        "{:?}",
        evidence[1].outcome
    );
    // `two` fails two assertions, so Kani printed two counterexample blocks under its path: it
    // takes the first, and neither it nor its neighbours are refused.
    assert!(
        matches!(
            &evidence[2].outcome,
            KaniRunOutcome::Falsified { counterexample }
                if counterexample.contains("two::check")
                    && counterexample.contains("Check for `assertion`")
        ),
        "{:?}",
        evidence[2].outcome
    );
    assert_eq!(
        evidence[3].outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::TimedOut
        }
    );
    assert_eq!(evidence[3].batch.as_ref().unwrap().timeout_seconds, 20);
    assert_eq!(evidence[0].exit_code, Some(1));

    let mut at_maximum = [
        harness("verified", VERIFIED, 4),
        harness("verified_again", VERIFIED, 4),
    ];
    for harness in &mut at_maximum {
        harness.identity.ceilings.wall_clock = Duration::from_secs(u64::from(u32::MAX));
    }
    let lane = Lane::new("maximum", &at_maximum);
    let requests: Vec<_> = at_maximum
        .iter()
        .map(|harness| lane.request(harness, &target, Duration::from_secs(u64::from(u32::MAX))))
        .collect();
    let evidence = execute_kani_obligations(&requests)
        .expect("every harness is in the crate")
        .into_iter()
        .next()
        .unwrap()
        .evidence
        .unwrap_or_else(|refusal| panic!("the batch at 4294967295 seconds: {refusal}"));
    assert!(evidence
        .iter()
        .all(|evidence| evidence.outcome == KaniRunOutcome::Verified));
    assert!(evidence[0]
        .arguments
        .windows(2)
        .any(|pair| pair == ["--harness-timeout", "4294967295"]));
}

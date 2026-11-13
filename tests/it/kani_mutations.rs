//! Controls for the `kani-mutations` target (FR-055, TC-069). A stand-in `cargo-kani` on `PATH`
//! and a stand-in test runner exercise the target's control flow; the installed-Kani run
//! (FR-055-AC-7, and AC-1 against real proofs) is made separately in the Kani lane.

use ix_trace_rs::trace;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_SANDBOX: AtomicU64 = AtomicU64::new(0);

/// The stand-in runner receives the owning proof as its last argument and judges it by name:
/// `ok` rejects the injected defect, `survive` verifies it, `silent` exits non-zero without the
/// rejection marker (a compile error), `hang` never finishes. It also requires that the mutant
/// replacement reached the scratch copy it runs in.
const RUNNER: &str = r##"#!/usr/bin/env bash
proof="${@: -1}"
grep -q 'MUTATED' src/lib.rs || { echo 'replacement was not injected'; exit 9; }
case "$proof" in
  *ok*) echo 'VERIFICATION:- FAILED'; exit 101 ;;
  *survive*) echo 'VERIFICATION:- SUCCESSFUL'; exit 0 ;;
  *silent*) echo 'error[E0425]: cannot find value'; exit 101 ;;
  *hang*) sleep 60 ;;
esac
"##;

struct Sandbox {
    root: PathBuf,
    bin: PathBuf,
}

impl Sandbox {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "cg-kani-mutations-{}-{}",
            std::process::id(),
            NEXT_SANDBOX.fetch_add(1, Ordering::Relaxed)
        ));
        let bin = root.join("bin");
        fs::create_dir_all(&bin).expect("stand-in directory");
        fs::create_dir_all(root.join("src")).expect("source directory");
        fs::write(
            root.join("src/lib.rs"),
            "fn a() { ORIGINAL }\nfn b() { OTHER }\n",
        )
        .expect("source");
        let sandbox = Self { root, bin };
        sandbox.executable("cargo-kani", "#!/bin/sh\nexit 0\n");
        sandbox.executable("runner", RUNNER);
        sandbox
    }

    fn executable(&self, name: &str, contents: &str) {
        let path = self.bin.join(name);
        fs::write(&path, contents).expect("write stand-in");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("chmod");
    }

    fn declare(&self, entries: &[(&str, &str)]) {
        let rows: Vec<String> = entries
            .iter()
            .map(|(anchor, proof)| {
                format!(
                    r#"{{"file":"src/lib.rs","anchor":"{anchor}","replacement":"MUTATED","proof":"{proof}"}}"#
                )
            })
            .collect();
        fs::write(
            self.root.join("mutants.json"),
            format!("[{}]", rows.join(",")),
        )
        .expect("declaration");
    }

    fn run(&self, timeout: u32, budget: u32, with_kani: bool) -> Output {
        let path = if with_kani {
            format!("{}:/usr/bin:/bin", self.bin.display())
        } else {
            "/usr/bin:/bin".into()
        };
        Command::new(env!("CARGO_BIN_EXE_cg_gates"))
            .args([
                "kani-mutations",
                "mutants.json",
                &timeout.to_string(),
                &budget.to_string(),
                "--",
            ])
            .arg(self.bin.join("runner"))
            .env("PATH", path)
            .current_dir(&self.root)
            .output()
            .expect("run the target")
    }

    fn source(&self) -> String {
        fs::read_to_string(self.root.join("src/lib.rs")).expect("source")
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn out(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn err(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn three_ok(sandbox: &Sandbox) {
    sandbox.declare(&[
        ("ORIGINAL", "m::ok_1"),
        ("OTHER", "m::ok_2"),
        ("fn a", "m::ok_3"),
    ]);
}

#[trace("FR-055-AC-1", "FR-055-AC-2", "TC-069")]
#[test]
fn every_rejected_mutant_passes_is_listed_and_leaves_the_tree_untouched() {
    let sandbox = Sandbox::new();
    three_ok(&sandbox);
    let before = sandbox.source();
    let output = sandbox.run(30, 120, true);
    assert!(output.status.success(), "{}{}", out(&output), err(&output));
    let text = out(&output);
    for proof in ["m::ok_1", "m::ok_2", "m::ok_3"] {
        let line = text
            .lines()
            .find(|l| l.contains(&format!("harness={proof} ")))
            .unwrap_or_else(|| panic!("a line for {proof}: {text}"));
        assert!(
            line.contains("mutant=src/lib.rs") && line.ends_with("outcome=pass"),
            "{line}"
        );
    }
    assert!(
        text.contains("total=3 pass=3 fail=0 malformed=0 unavailable=0"),
        "{text}"
    );
    assert_eq!(
        sandbox.source(),
        before,
        "the working tree is byte-identical"
    );
    assert!(!sandbox.source().contains("MUTATED"));
}

#[trace("FR-055-AC-3", "TC-069")]
#[test]
fn ci_does_not_reach_kani_mutations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dry = Command::new("make")
        .args(["-n", "ci"])
        .current_dir(root)
        .output()
        .expect("make dry run");
    assert!(dry.status.success(), "{}", err(&dry));
    assert!(!out(&dry).contains("kani-mutations"));
    let makefile = fs::read_to_string(root.join("Makefile")).expect("Makefile");
    let ci = makefile
        .lines()
        .find(|l| l.starts_with("ci:"))
        .expect("ci target");
    assert!(!ci.contains("kani-mutations"));
    let target = Command::new("make")
        .args(["-n", "kani-mutations"])
        .current_dir(root)
        .output()
        .expect("make dry run");
    assert!(out(&target).contains("kani-mutations"), "the target exists");
}

#[trace("FR-055-AC-4", "TC-069")]
#[test]
fn a_surviving_mutant_is_fail_and_exits_non_zero() {
    let sandbox = Sandbox::new();
    sandbox.declare(&[
        ("ORIGINAL", "m::ok_1"),
        ("OTHER", "m::survive_2"),
        ("fn a", "m::ok_3"),
    ]);
    let output = sandbox.run(30, 120, true);
    assert!(!output.status.success());
    let text = out(&output);
    assert!(
        text.lines()
            .any(|l| l.contains("harness=m::survive_2 ") && l.ends_with("outcome=fail")),
        "{text}"
    );
    assert!(text.contains("total=3 pass=2 fail=1"), "{text}");
}

#[trace("FR-055-AC-5", "TC-069")]
#[test]
fn absent_or_repeated_anchors_are_malformed_and_other_failures_are_fail() {
    let sandbox = Sandbox::new();
    // `fn` occurs twice in the source; `ABSENT` never.
    sandbox.declare(&[
        ("ABSENT", "m::ok_1"),
        ("fn ", "m::ok_2"),
        ("ORIGINAL", "m::silent_3"),
    ]);
    let output = sandbox.run(30, 120, true);
    assert_eq!(output.status.code(), Some(1));
    let text = out(&output);
    assert!(
        text.lines()
            .any(|l| l.contains("harness=m::ok_1 ") && l.ends_with("outcome=malformed")),
        "{text}"
    );
    assert!(
        text.lines()
            .any(|l| l.contains("harness=m::ok_2 ") && l.ends_with("outcome=malformed")),
        "{text}"
    );
    assert!(
        text.lines()
            .any(|l| l.contains("harness=m::silent_3 ") && l.ends_with("outcome=fail")),
        "{text}"
    );
    assert!(text.contains("total=3 pass=0 fail=1 malformed=2"), "{text}");
}

#[trace("FR-055-AC-10", "TC-069")]
#[test]
fn a_mutant_past_the_timeout_and_a_run_past_the_budget_are_fail() {
    let sandbox = Sandbox::new();
    sandbox.declare(&[
        ("ORIGINAL", "m::hang_1"),
        ("OTHER", "m::ok_2"),
        ("fn a", "m::ok_3"),
    ]);
    let output = sandbox.run(1, 120, true);
    assert!(!output.status.success());
    let text = out(&output);
    assert!(
        text.lines()
            .any(|l| l.contains("harness=m::hang_1 ") && l.ends_with("outcome=fail")),
        "{text}"
    );
    assert!(text.contains("total=3 pass=2 fail=1"), "{text}");

    // A budget of zero is spent before any mutant runs, so every one is a fail.
    three_ok(&sandbox);
    let output = sandbox.run(30, 0, true);
    assert!(!output.status.success());
    assert!(
        out(&output).contains("total=3 pass=0 fail=3"),
        "{}",
        out(&output)
    );
}

#[trace("FR-055-AC-6", "TC-069")]
#[test]
fn without_cargo_kani_every_mutant_is_unavailable_and_nothing_runs() {
    let sandbox = Sandbox::new();
    three_ok(&sandbox);
    let output = sandbox.run(30, 120, false);
    assert!(!output.status.success());
    let text = out(&output);
    assert_eq!(
        text.lines()
            .filter(|l| l.ends_with("outcome=unavailable"))
            .count(),
        3,
        "{text}"
    );
    assert!(
        text.contains("total=3 pass=0 fail=0 malformed=0 unavailable=3"),
        "{text}"
    );
}

#[trace("FR-055-AC-8", "TC-069")]
#[test]
fn the_target_is_documented_as_a_standalone_gate_beside_kani_gate() {
    let makefile = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Makefile"))
        .expect("Makefile");
    assert!(makefile.contains("make kani-mutations   - Require every declared mutant"));
    assert!(makefile.contains("standalone, never part of `make ci`, a Kani-affecting merge gate"));
}

#[trace("FR-055-AC-9", "TC-069")]
#[test]
fn fewer_than_three_declared_mutants_is_refused_with_exit_two_and_runs_nothing() {
    let sandbox = Sandbox::new();
    for entries in [
        &[][..],
        &[("ORIGINAL", "m::ok_1"), ("OTHER", "m::ok_2")][..],
    ] {
        sandbox.declare(entries);
        let output = sandbox.run(30, 120, true);
        assert_eq!(output.status.code(), Some(2), "{}", err(&output));
        assert!(err(&output).contains("refused"), "{}", err(&output));
        assert!(!out(&output).contains("outcome="), "nothing ran");
    }
}

#[trace("FR-055-AC-9", "TC-069")]
#[test]
fn the_shipped_declaration_holds_at_least_three_complete_entries_with_unique_anchors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let text = fs::read_to_string(root.join("scripts/kani-mutants.json")).expect("declaration");
    let entries: Vec<serde_json::Value> = serde_json::from_str(&text).expect("declaration JSON");
    assert!(entries.len() >= 3);
    for entry in &entries {
        let field = |name| {
            entry[name]
                .as_str()
                .filter(|v| !v.is_empty())
                .unwrap_or_else(|| panic!("{name} in {entry}"))
        };
        let source = fs::read_to_string(root.join(field("file"))).expect("mutant source file");
        assert_eq!(
            source.matches(field("anchor")).count(),
            1,
            "anchor occurs once: {entry}"
        );
        assert_ne!(field("anchor"), field("replacement"));
        field("proof");
    }
}

//! Tests for the shared assurance intake path (FR-006).
//!
//! These follow this repository's own binding idiom: a `/// Trace:` comment above
//! each `#[test]`, which is what Quire's census reads. They invoke the gates
//! rather than reimplementing them, because a test that recomputes what a gate
//! computes is a second implementation that can agree with itself while both are
//! wrong.
//!
//! A missing prerequisite is a failure here, never a skip. A gate that stands
//! down when its dependency is absent reports the same green as one that ran.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The interpreter `make assurance-env` builds. Its absence is an error.
fn assurance_python() -> PathBuf {
    let path = std::env::var_os("ASSURANCE_PYTHON")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join(".venv-assurance/bin/python"));
    assert!(
        path.is_file(),
        "the pinned assurance interpreter is missing at {}. Run `make assurance-env`. \
         This is a failure and not a skip: a gate that stands down when its dependency \
         is absent reports the same green as one that ran.",
        path.display()
    );
    path
}

fn run(program: &Path, arguments: &[&str]) -> (i32, String, String) {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(root())
        .output()
        .unwrap_or_else(|error| panic!("failed to run {}: {error}", program.display()));
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn json_gate(program: &Path, arguments: &[&str]) -> Value {
    let (code, stdout, stderr) = run(program, arguments);
    assert_eq!(code, 0, "{arguments:?} exited {code}\n{stdout}\n{stderr}");
    serde_json::from_str(&stdout)
        .unwrap_or_else(|error| panic!("{arguments:?} did not emit JSON: {error}\n{stdout}"))
}

fn head_revision() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root())
        .output()
        .expect("git rev-parse failed");
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

fn sha256_of(path: &Path) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum failed");
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .expect("sha256sum output")
        .to_owned()
}

/// The chain is expensive and several tests read it. It runs once per test
/// binary, and every reader sees the same run rather than a different one.
static CHAIN: OnceLock<Value> = OnceLock::new();

fn chain_report() -> &'static Value {
    CHAIN.get_or_init(|| {
        // The chain runs under the system interpreter: it only shells out to
        // quoin and never imports engineering-assurance.
        let revision = head_revision();
        let (code, stdout, stderr) = run(
            Path::new("python3"),
            &[
                "scripts/assurance_chain.py",
                "--candidate-revision",
                &revision,
                "--json",
            ],
        );
        assert_eq!(code, 0, "the assurance chain exited {code}\n{stderr}");
        serde_json::from_str(&stdout).expect("the assurance chain did not emit JSON")
    })
}

/// The producer output `make assurance-inputs` wrote. Absent is a failure.
fn producer_output(name: &str) -> String {
    let path = root().join("target/assurance").join(name);
    fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "{} is absent: {error}. Run `make assurance-inputs`. A test that can \
             produce its own inputs can produce a green run out of nothing.",
            path.display()
        )
    })
}

/// Trace: TC-008, FR-006-AC-1
#[test]
fn tc_008_every_shared_pin_is_classified_by_the_packaged_matrix() {
    let python = assurance_python();
    let report = json_gate(&python, &["scripts/check_shared_pins.py", "--json"]);

    let components = report["components"].as_array().expect("components array");
    assert_eq!(
        components.len(),
        4,
        "the matrix pins four components; this run classified {}",
        components.len()
    );
    for component in components {
        assert_eq!(
            component["verdict"], "compatible",
            "{} is {} ({})",
            component["component"], component["verdict"], component["reason"]
        );
    }
    assert_eq!(report["accepted"], true);
    assert!(report["artifact_mismatches"].as_array().unwrap().is_empty());
    assert!(report["mirror_references"].as_array().unwrap().is_empty());
    assert!(report["incompatible_install_references"]
        .as_array()
        .unwrap()
        .is_empty());

    // Acceptance is reported and never gated on: the pinned release records
    // `pending_human_acceptance` and ships no predicate for it
    // (agent-ix/engineering-assurance#20). Reading an absent field as approval,
    // in either direction, is the mistake this asserts against.
    assert_eq!(report["acceptance_recorded_here"], false);
    assert!(report["acceptance_state"].is_string());

    // Both scans must be seen to refuse. Without this they are indistinguishable
    // from checks that match nothing — which is how this repository's own
    // workflow carried an incompatible quoin pin for a whole wave.
    let (code, stdout, stderr) = run(
        &python,
        &[
            "-c",
            "import json,sys;sys.path.insert(0,'scripts');\
             import check_shared_pins as m;\
             pins=json.load(open('assurance/pins.json'));\
             pins['engineering_assurance']['requirement']+=' --registry=https://npm.ix/';\
             print(json.dumps(m.mirror_references(pins)))",
        ],
    );
    assert_eq!(code, 0, "the mirror probe failed: {stderr}");
    let offenders: Vec<String> = serde_json::from_str(stdout.trim()).unwrap();
    assert!(
        !offenders.is_empty(),
        "a mirror registry reference was not detected; the check matches nothing"
    );

    // The install scan reads the forbidden versions out of the matrix at run
    // time, so this probe hands it a matrix whose forbidden version is one that
    // really is written down in this tree — the quire-cli pin — and requires it
    // to be found. A scan that cannot find a version that is present is a scan
    // that would not have found the one that was.
    let (code, stdout, stderr) = run(
        &python,
        &[
            "-c",
            "import json,sys;sys.path.insert(0,'scripts');\
             import check_shared_pins as m;\
             matrix={'components':[{'name':'quire-cli','incompatible':['0.31.0'],\
             'incompatible_reasons':{'0.31.0':'probe'}}]};\
             print(json.dumps(m.incompatible_install_references(matrix)))",
        ],
    );
    assert_eq!(code, 0, "the install-pin probe failed: {stderr}");
    let offenders: Vec<String> = serde_json::from_str(stdout.trim()).unwrap();
    assert!(
        !offenders.is_empty(),
        "an install line naming a forbidden version was not detected; the scan \
         matches nothing"
    );
}

/// Trace: TC-009, FR-006-AC-2
#[test]
fn tc_009_the_chain_reaches_quoin_without_quoin_or_quire_executing_a_producer() {
    let report = chain_report();
    assert_eq!(report["matched"], true, "{report:#}");

    for group in ["scenarios", "controls", "adapter_probes"] {
        let items = report[group]
            .as_array()
            .unwrap_or_else(|| panic!("{group}"));
        assert!(!items.is_empty(), "{group} is empty");
        for item in items {
            assert_eq!(
                item["matched"], true,
                "{group} entry did not match: {item:#}"
            );
        }
    }

    // The adapter transcribes one named protocol and refuses another, rather than
    // guessing. A verdict recovered from an unrecognised stream is a verdict
    // recovered from nothing.
    let probes = report["adapter_probes"].as_array().unwrap();
    for required in [
        "refuses-a-foreign-protocol",
        "refuses-an-unnamed-outcome",
        "refuses-an-empty-stream",
        // The undecodable-bytes arm of the adapter. Required by name because it
        // is the branch nothing else reaches, and because the probe that used to
        // stand where it does could not fail.
        "refuses-an-undecodable-row-by-name",
        "accepts-the-real-run",
    ] {
        assert!(
            probes.iter().any(|probe| probe["probe"] == required),
            "adapter probe {required} is missing"
        );
    }

    // Every producer this repository owns must have been attested from its own
    // bytes and have reported success at this revision.
    let attested = &report["attested_results"];
    for proof in [
        "PROOF-generation-conformance",
        "PROOF-upstream-identity",
        "PROOF-quire-static-export",
        "PROOF-msrv",
    ] {
        assert_eq!(
            attested[proof], "passed",
            "{proof} was attested as {}",
            attested[proof]
        );
    }
}

/// Trace: TC-009, FR-006-AC-2
#[test]
fn tc_009_every_declared_proof_command_is_the_command_make_actually_runs() {
    // A declared command that is not the executed command is a lie inside a
    // sealed attestation, and it is the kind of lie nothing downstream can catch:
    // Quoin records what the caller says the command was.
    //
    // So it is asked of Make. `make -n assurance-inputs` prints the plan without
    // running it; line continuations are rejoined; every proof obligation's
    // declared argv must appear in that plan verbatim.
    let root = root();
    let declaration: Value = serde_json::from_str(
        &fs::read_to_string(root.join("assurance/change-assurance.json")).unwrap(),
    )
    .expect("the change-assurance declaration is JSON");

    // `cargo test` exports CARGO as an absolute path to the toolchain binary. The
    // Makefile overrides CARGO to a trusted absolute path of its own, so the
    // declaration's tool names have to be matched against the plan's basenames
    // rather than its argv[0]. The declaration names tools; the Makefile resolves
    // them; both are correct and this reconciles them without weakening either.
    let plan = Command::new("make")
        .args(["-n", "assurance-inputs"])
        .current_dir(&root)
        .env_remove("CARGO")
        .output()
        .expect("make -n assurance-inputs failed to run");
    assert!(
        plan.status.success(),
        "make -n assurance-inputs did not resolve: {}",
        String::from_utf8_lossy(&plan.stderr)
    );
    let joined = String::from_utf8_lossy(&plan.stdout).replace("\\\n", " ");
    // Reduce every absolute path to its basename so `…/.cargo/bin/cargo run` is
    // read as `cargo run`. Nothing else is normalised.
    let normalised: String = joined
        .split_whitespace()
        .map(|word| {
            if word.starts_with('/') {
                word.rsplit('/').next().unwrap_or(word)
            } else {
                word
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let obligations = declaration["record"]["definition"]["proof_obligations"]
        .as_array()
        .expect("proof_obligations");
    assert_eq!(
        obligations.len(),
        4,
        "the declaration names {} proof obligations; the chain and this test expect 4",
        obligations.len()
    );
    for proof in obligations {
        let argv: Vec<String> = proof["command"]["argv"]
            .as_array()
            .expect("argv")
            .iter()
            .map(|value| value.as_str().expect("argv element").to_owned())
            .collect();
        let command = argv.join(" ");
        assert!(
            normalised.contains(&command),
            "{} declares `{command}`, which `make assurance-inputs` does not run.\nPlan: {normalised}",
            proof["proof_id"]
        );
    }
}

/// Run a Python snippet against a producer module and return its stdout.
fn producer_probe(snippet: &str) -> String {
    let (code, stdout, stderr) = run(Path::new("python3"), &["-c", snippet]);
    assert_eq!(code, 0, "the producer probe failed\n{stdout}\n{stderr}");
    stdout.trim().to_owned()
}

/// Trace: TC-009, FR-006-AC-2, FR-006-AC-5
#[test]
fn tc_009_the_producers_report_failure_when_the_thing_they_measure_disagrees() {
    // Every other test in this file asks a producer whether it says `pass` when
    // the thing it measures is healthy. None of them asks whether it can say
    // anything else. A producer hollowed out to `return "pass"` satisfies all of
    // them, runs in milliseconds, and turns the whole chain green.

    // The upstream-identity producer, made to read a lockfile that resolved a
    // different revision. Disagreement must be `fail`.
    let outcome = producer_probe(
        "import json,sys; sys.path.insert(0,'scripts')\n\
         import check_upstream_pins as m\n\
         m.lockfile_revision = lambda lock, pkg: 'f'*40\n\
         print(json.dumps(sorted({e['outcome'] for e in m.collect()['entries']})))",
    );
    assert_eq!(
        outcome, "[\"fail\"]",
        "a lockfile resolving a different revision did not produce a failing row; got {outcome}"
    );

    // A place that states no revision at all is `not-computed`, not `fail`. The
    // comparison never happened, and saying it failed would claim one that did.
    let outcome = producer_probe(
        "import json,sys; sys.path.insert(0,'scripts')\n\
         import check_upstream_pins as m\n\
         m.manifest_revision = lambda manifest, pkg: None\n\
         print(json.dumps(sorted({e['outcome'] for e in m.collect()['entries']})))",
    );
    assert_eq!(
        outcome, "[\"not-computed\"]",
        "an absent revision was not reported as uncomputed; got {outcome}"
    );

    // And a revision that is not a revision is `malformed`, which is a third
    // answer and not either of the first two.
    let outcome = producer_probe(
        "import json,sys; sys.path.insert(0,'scripts')\n\
         import check_upstream_pins as m\n\
         m.lockfile_revision = lambda lock, pkg: 'not-a-revision'\n\
         print(json.dumps(sorted({e['outcome'] for e in m.collect()['entries']})))",
    );
    assert_eq!(
        outcome, "[\"malformed\"]",
        "a malformed revision was not reported as malformed; got {outcome}"
    );

    // The chain's own measurement floor. A conformance row that says `pass` while
    // discharging fewer checks than its declared floor must be refused outright,
    // because that is what a hand-written forgery looks like: the verdict is
    // cheap and the numbers are not.
    let refused = producer_probe(
        "import json,sys; sys.path.insert(0,'scripts')\n\
         import assurance_chain as c\n\
         row={'protocol':c.CONFORMANCE_PROTOCOL,'symbol':'x','outcome':'pass',\
         'checksDischarged':0,'floor':6}\n\
         try:\n\
         \x20   c._derive('PROOF-generation-conformance', json.dumps(row)+'\\n', \
         __import__('pathlib').Path('x.jsonl'))\n\
         \x20   print('accepted')\n\
         except c.ChainError:\n\
         \x20   print('refused')",
    );
    assert_eq!(
        refused, "refused",
        "a conformance row passing below its declared floor was accepted"
    );

    // A row with no measurement at all is refused for the same reason.
    let refused = producer_probe(
        "import json,sys; sys.path.insert(0,'scripts')\n\
         import assurance_chain as c\n\
         row={'protocol':c.CONFORMANCE_PROTOCOL,'symbol':'x','outcome':'pass'}\n\
         try:\n\
         \x20   c._derive('PROOF-generation-conformance', json.dumps(row)+'\\n', \
         __import__('pathlib').Path('x.jsonl'))\n\
         \x20   print('accepted')\n\
         except c.ChainError:\n\
         \x20   print('refused')",
    );
    assert_eq!(
        refused, "refused",
        "a conformance row passing with no measurement behind it was accepted"
    );
}

/// Trace: TC-009, FR-006-AC-2
#[test]
fn tc_009_the_generation_producer_reports_what_the_generator_did() {
    // The corpus as it stands, read from the bytes `make assurance-inputs` wrote.
    // Every case must have reached its declared terminal state, and the census
    // row must be at or above its floor — a corpus that stopped covering half its
    // vocabulary would still be a corpus of passing cases.
    let stream = producer_output("generation-conformance.jsonl");
    let rows: Vec<Value> = stream
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).expect("a conformance row is JSON"))
        .collect();
    assert!(rows.len() >= 9, "the corpus shrank to {} rows", rows.len());

    for row in &rows {
        assert_eq!(
            row["outcome"], "pass",
            "corpus case {} is {}: {}",
            row["symbol"], row["outcome"], row["detail"]
        );
        assert!(
            row["checksDischarged"].as_u64().unwrap() >= row["floor"].as_u64().unwrap(),
            "corpus case {} passed below its floor",
            row["symbol"]
        );
        if !row["expectedTerminalState"].is_null() {
            assert_eq!(
                row["terminalState"], row["expectedTerminalState"],
                "corpus case {} reached {} and declared {}",
                row["symbol"], row["terminalState"], row["expectedTerminalState"]
            );
        }
        if !row["expectedDiagnosticCode"].is_null() {
            assert_eq!(
                row["diagnosticCode"], row["expectedDiagnosticCode"],
                "corpus case {} produced diagnostic {} and declared {}",
                row["symbol"], row["diagnosticCode"], row["expectedDiagnosticCode"]
            );
        }
    }

    // The generator's own terminal states must not have collapsed. `unsupported`
    // and `invalid-input` are different facts for a caller and both have to be
    // reachable, or the diagnostics have stopped discriminating.
    let states: BTreeSet<&str> = rows
        .iter()
        .filter_map(|row| row["terminalState"].as_str())
        .collect();
    for required in ["generated", "unsupported", "invalid-input"] {
        assert!(
            states.contains(required),
            "the corpus never reached the terminal state {required}; states seen: {states:?}"
        );
    }
}

/// Write an executable shim for each name that records every invocation.
///
/// The log is the point. A shim that is never consulted and a producer that is
/// never run look identical from the outside, so the shims write down every call
/// and the test reads the file rather than assuming.
///
/// `--version` is answered rather than refused, and deliberately so. Asking a
/// tool its version is an observation — it is what the compatibility matrix's own
/// `observe` column does — and it is not the thing this test forbids. What is
/// forbidden is asking a tool to build, compile, test, generate, or run anything.
/// Every such invocation is logged and the log must be empty.
fn producer_shims(directory: &Path, names: &[&str]) -> PathBuf {
    // The directory is emptied, not just topped up. A shim left behind by an
    // earlier run silently changes what the next run measures, and it does so in
    // the direction that hides failures.
    let _ = fs::remove_dir_all(directory);
    fs::create_dir_all(directory).unwrap();
    let log = directory.join("invocations.log");
    for name in names {
        let path = directory.join(name);
        fs::write(
            &path,
            format!(
                // Every invocation is logged, and each one says which of the two
                // kinds it is. Asking a tool its version is an observation — it is
                // what the compatibility matrix's own `observe` column does — and
                // it is not the thing this test forbids. Asking a tool to build,
                // compile, test, generate or export is work, and the log must
                // carry none of it.
                //
                // The caller is recorded too, not just the call. `quoin evidence
                // audit` legitimately shells out to `quire coverage` against its
                // own scratch repository, so "was quire asked for a coverage
                // export" is not on its own an answer to "did the driver run a
                // producer". The parent's command line tells those two apart, and
                // asserting on it is what closes the hole an adversarial review
                // found: injecting `subprocess.run(["quire","coverage",...])`
                // straight into the driver was not detected, because the only
                // assertion was on the subcommand.
                "#!/bin/sh\n\
                 parent=$(tr '\\0' ' ' < /proc/$PPID/cmdline 2>/dev/null)\n\
                 case \"$1\" in\n\
                 --version|-V)\n\
                   echo \"observe $0 $@ <<caller=$parent\" >> {log}\n\
                   echo \"{name} 9.9.9 (shim)\"; exit 0 ;;\n\
                 provenance)\n\
                   echo \"observe $0 $@ <<caller=$parent\" >> {log}\n\
                   echo '{{\"cli\":{{\"version\":\"9.9.9\"}},\"engine\":{{\"version\":\"9.9.9\"}}}}'\n\
                   exit 0 ;;\n\
                 esac\n\
                 echo \"work $0 $@ <<caller=$parent\" >> {log}\n\
                 exit 97\n",
                log = log.display(),
                name = name,
            ),
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }
    log
}

fn run_chain_with_path(shims: &Path) -> std::process::Output {
    let inherited = std::env::var("PATH").unwrap_or_default();
    let revision = head_revision();
    // Launched by absolute path on purpose. `python3` is one of the names the
    // shims replace — the input target runs two of its five producers through it
    // — and resolving the driver itself through the shimmed PATH would stub the
    // thing under test instead of the things it must not call.
    Command::new("/usr/bin/python3")
        .args([
            "scripts/assurance_chain.py",
            "--candidate-revision",
            &revision,
        ])
        .current_dir(root())
        .env("PATH", format!("{}:{inherited}", shims.display()))
        .output()
        .expect("failed to run the assurance chain")
}

/// Trace: TC-009, FR-006-AC-2
#[test]
fn tc_009_the_chain_never_executes_a_producer_and_the_probe_can_prove_it() {
    // Four runs, because one proves nothing and three proved too little.
    //
    // Run A replaces every producer `make assurance-inputs` actually runs —
    // `cargo`, `rustup`, `rustc`, and `python3`/`python`, which is how two of the
    // five producers are invoked — with a stub that logs and fails. The chain must
    // finish and the log must carry no work.
    //
    // The interpreter names are in that list because leaving them out was a real
    // hole: an adversarial review made the driver regenerate a deleted
    // `upstream-identity.json` by running `python3 scripts/check_upstream_pins.py`
    // itself, and every gate here stayed green.
    //
    // Run B is the control. It stubs `quoin`, which the chain is supposed to run,
    // and requires the chain to fail and the log to be non-empty. Without it, an
    // empty log in run A would be equally consistent with PATH never being
    // consulted at all.
    //
    // Run C reads what Quire is asked to do, and by whom.
    //
    // Run D is the total form of the claim, and it does not depend on anyone
    // having thought to shim the right tool: the driver must not have written into
    // the directory it reads its inputs from, by any means at all.
    let producers = root().join("target/producer-shims");
    let producer_log = producer_shims(
        &producers,
        &["cargo", "rustup", "rustc", "python3", "python"],
    );
    let output = run_chain_with_path(&producers);
    let logged = fs::read_to_string(&producer_log).unwrap_or_default();
    assert!(
        output.status.success(),
        "the assurance chain failed with producers stubbed, which means it ran one:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let work: Vec<&str> = logged
        .lines()
        .filter(|line| line.starts_with("work "))
        .collect();
    assert!(
        work.is_empty(),
        "the assurance driver asked a producer to do work, not just to name its \
         version:\n{work:#?}"
    );

    let tools = root().join("target/tool-shims");
    let tool_log = producer_shims(&tools, &["quoin"]);
    let control = run_chain_with_path(&tools);
    let tool_logged = fs::read_to_string(&tool_log).unwrap_or_default();
    assert!(
        !tool_logged.trim().is_empty(),
        "stubbing quoin produced no invocation, so PATH is not being consulted by \
         the subprocess and the run above proves nothing"
    );
    assert!(
        !control.status.success(),
        "the chain succeeded with quoin stubbed out, so it is not actually using it"
    );

    // Run C: what is Quire asked to do, and who asks it?
    //
    // Quire is excluded from run A because `quoin evidence audit` shells out to
    // `quire coverage --scope <its own scratch repo> --json`. That is Quoin
    // reading static facts, which is what the architecture says Quoin does with
    // Quire's export; it is not the driver running a producer. Asserting only that
    // the subcommand is `coverage` or `provenance` therefore permits the driver to
    // run `quire coverage` itself, which is exactly the producer command — and an
    // adversarial review did precisely that and was not caught.
    //
    // So the caller is what is asserted. Every invocation whose parent is the
    // driver must be an observation. A coverage export requested by the driver is
    // a producer run and is refused; the same request from Quoin is fine.
    // This run's chain is expected to fail — a shim cannot serve a real export —
    // and that is fine, because what is being read is the log, not the exit code.
    let quire_shims = root().join("target/quire-shims");
    let quire_log = producer_shims(&quire_shims, &["quire"]);
    let _ = run_chain_with_path(&quire_shims);
    let quire_logged = fs::read_to_string(&quire_log).unwrap_or_default();
    assert!(
        !quire_logged.trim().is_empty(),
        "stubbing quire produced no invocation, so this run observed nothing"
    );
    let mut driver_calls = 0;
    let mut other_calls = 0;
    for line in quire_logged.lines().filter(|line| !line.trim().is_empty()) {
        let (invocation, caller) = line.split_once("<<caller=").unwrap_or((line, ""));
        assert!(
            !caller.is_empty(),
            "the shim recorded no caller, so this run cannot tell the driver from \
             Quoin and proves nothing: {line}"
        );
        if caller.contains("assurance_chain.py") {
            driver_calls += 1;
            assert!(
                line.starts_with("observe "),
                "the driver asked Quire to do work rather than to name its own \
                 version: {invocation}"
            );
        } else {
            other_calls += 1;
        }
    }
    // Both halves must have happened, or the discrimination is untested. The
    // driver must have been seen asking Quire something, or the caller check
    // matched nothing; and something other than the driver must have been seen
    // asking too, or the exemption that makes this run necessary was never
    // exercised.
    assert!(
        driver_calls > 0,
        "no Quire invocation was attributed to the driver, so the caller check \
         matched nothing:\n{quire_logged}"
    );
    assert!(
        other_calls > 0,
        "every Quire invocation was attributed to the driver, so this run never \
         exercised the case it exists to permit:\n{quire_logged}"
    );

    // Run D: the driver wrote nothing into its own input directory.
    //
    // Every run above depends on someone having shimmed the tool a driver would
    // use. This one does not: it digests `target/assurance/` before and after and
    // requires it unchanged, so a driver that regenerated an input using a tool
    // nobody named is caught anyway. The driver performs the same census itself
    // and exits 2 if it fails; this asserts it from outside, against the file
    // system, so a driver that stopped performing it would still be caught.
    let inputs = root().join("target/assurance");
    let before = directory_digests(&inputs);
    assert!(
        !before.is_empty(),
        "target/assurance is empty; run `make assurance-inputs`"
    );
    let revision = head_revision();
    let honest = Command::new("/usr/bin/python3")
        .args([
            "scripts/assurance_chain.py",
            "--candidate-revision",
            &revision,
        ])
        .current_dir(root())
        .output()
        .expect("failed to run the assurance chain");
    assert!(
        honest.status.success(),
        "the chain failed on the honest path:\n{}\n{}",
        String::from_utf8_lossy(&honest.stdout),
        String::from_utf8_lossy(&honest.stderr)
    );
    let after = directory_digests(&inputs);
    assert_eq!(
        before, after,
        "the driver wrote into the directory it reads its inputs from; a driver \
         that can produce its own inputs can produce a green run out of nothing"
    );
}

/// Digest every file under `directory`, keyed by path relative to it.
fn directory_digests(directory: &Path) -> BTreeMap<String, String> {
    let mut found = BTreeMap::new();
    let Ok(entries) = fs::read_dir(directory) else {
        return found;
    };
    for entry in entries {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("")
                .to_owned();
            for (child, digest) in directory_digests(&path) {
                found.insert(format!("{name}/{child}"), digest);
            }
            continue;
        }
        found.insert(
            path.file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("")
                .to_owned(),
            sha256_of(&path),
        );
    }
    found
}

/// Trace: TC-010, FR-006-AC-3
#[test]
fn tc_010_the_sealed_records_impact_snapshot_is_the_quire_export() {
    let report = chain_report();
    let export = root().join(report["quire_export"].as_str().expect("quire_export"));
    let bytes =
        fs::read(&export).unwrap_or_else(|error| panic!("{} is absent: {error}", export.display()));

    assert_eq!(
        report["impact_snapshot_digest"],
        sha256_of(&export),
        "the sealed record's impact snapshot does not name the Quire export it claims"
    );
    // An empty object has a digest too. The snapshot is only worth its content,
    // so the export is required to actually carry the coverage facts the record
    // claims it snapshotted, and to name every requirement this repository has.
    let parsed: Value = serde_json::from_slice(&bytes).expect("the Quire export is JSON");
    let text = String::from_utf8_lossy(&bytes);
    for requirement in [
        "FR-001", "FR-002", "FR-003", "FR-004", "FR-005", "FR-006", "NFR-001", "NFR-002", "StR-001",
    ] {
        assert!(
            text.contains(requirement),
            "the Quire export does not mention {requirement}; it is not a coverage \
             export of this repository"
        );
    }
    assert!(
        parsed.is_object() && !parsed.as_object().unwrap().is_empty(),
        "the Quire export is not a populated document"
    );

    // A Coverage Status column that contradicts its own row is reported by Quire
    // and does not change its exit code — `quire coverage --strict` prints the
    // contradiction and returns 0. The local checker that used to compensate was
    // a second traceability implementation with a hand-copied matrix and went
    // with the rest of the generic machinery. What replaces it is this: the
    // export's own `status_lies` list, read as a gate. That is Quire's answer
    // being enforced here rather than recomputed here.
    let lies = parsed["status_lies"].as_array().expect("status_lies");
    assert!(
        lies.is_empty(),
        "the test matrix claims a coverage status its own rows contradict: {lies:#?}"
    );

    // And the chain must have read the export as a populated one rather than as a
    // run whose result was not computed.
    assert_eq!(
        report["attested_results"]["PROOF-quire-static-export"], "passed",
        "the Quire export was attested as {}",
        report["attested_results"]["PROOF-quire-static-export"]
    );
}

/// Collect every readable source file under `directory`, recursively.
fn collect_sources(directory: &Path, into: &mut Vec<PathBuf>) {
    // Excluded, and each for its own reason: `.git` is not source, `target` is
    // build output, and `.venv-assurance` is the pinned upstream release rather
    // than anything this repository wrote. `evidence/` used to be excluded here
    // as retained history that legitimately named the schemas its records were
    // sealed against; it is deleted, so the exemption it needed is gone with it.
    // `__pycache__` joins the three: it is generated, it is not authored here,
    // and its `.pyc` files are binary. They were being read and silently dropped
    // by the old unreadable-file `continue`, which is how the wide-encoding
    // assertion below found them.
    const EXCLUDED: [&str; 4] = [".git", "target", ".venv-assurance", "__pycache__"];
    // Fail closed, exactly as the file arm does. A silent `return` here was the
    // directory half of the same class as the unreadable-file skip: an exclusion
    // no deny-list entry names, which also quietly lowers `inspected` and the
    // per-directory counts. Measured: a live import inside a chmod-000 directory
    // was invisible.
    let entries = fs::read_dir(directory).unwrap_or_else(|error| {
        panic!(
            "{} could not be read ({error}); the census cannot claim to have \
             examined what is inside it",
            directory.display()
        )
    });
    for entry in entries {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("");
            if EXCLUDED.contains(&name) {
                continue;
            }
            collect_sources(&path, into);
            continue;
        }
        // Deny-list, not allow-list, and that direction was chosen after an
        // allow-list was measured failing twice.
        //
        // An allow-list is a list of the places a reintroduced reference has to
        // avoid, and it only has to be incomplete once. `Makefile` has no
        // extension, so an extension-only filter never saw the one file a Make
        // target name can live in — appending the deleted compatibility-view
        // target verbatim was observed to leave the census green. Naming
        // `Makefile` back explicitly fixed that one file and left the next one:
        // an extensionless `scripts/reintroduced_reader` is just as invisible to
        // a filter that admits three names by hand, and a reader is exactly the
        // kind of thing that gets committed without a suffix.
        //
        // So everything is scanned unless there is a reason not to. The reasons
        // are: it is not text and would never carry a readable reference, or it
        // is a generated lock or licence whose contents this repository does not
        // author. Each exclusion is named individually below; the count in this
        // comment is the length of the array, so it cannot drift from it.
        const NOT_SCANNED: [&str; 6] = [
            "Cargo.lock",
            "LICENSE",
            "LICENSE-APACHE",
            "LICENSE-MIT",
            "SHA256SUMS",
            ".gitkeep",
        ];
        const NOT_SCANNED_EXTENSIONS: [&str; 12] = [
            "lock", "png", "jpg", "jpeg", "gif", "ico", "pdf", "zip", "gz", "golden", "pyc", "pyo",
        ];
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        // `EXCLUDED` is consulted on directories only, and in a linked worktree
        // `.git` is a *file* pointing at the real git directory. Harmless here --
        // it carries no forbidden name -- but an exclusion should mean what it
        // says wherever the repository happens to be checked out.
        if EXCLUDED.contains(&name) {
            continue;
        }
        if NOT_SCANNED.contains(&name) || NOT_SCANNED_EXTENSIONS.contains(&extension) {
            continue;
        }
        // Regular files only. A FIFO is not a directory, so it fell through to
        // `fs::read` and blocked the whole census with no timeout -- denial
        // rather than escape, but a walk over a working tree should not have an
        // unbounded step in it.
        if !path.is_file() {
            continue;
        }
        into.push(path);
    }
}

/// Trace: TC-012, TC-003, FR-006-AC-5, NFR-002-AC-3
#[test]
fn tc_012_every_demonstrable_verification_outcome_is_demonstrated_and_paired_with_controls() {
    // The shared vocabulary is twelve states. Ten are demonstrated on this
    // repository's intake path and are required here. A state nobody
    // demonstrates is a state nobody would notice the loss of.
    const REQUIRED: [&str; 10] = [
        "pass",
        "fail",
        "unavailable",
        "inconclusive",
        "not-computed",
        "partial",
        "stale",
        "suspect",
        "vacuous",
        "tampered",
    ];

    // `unsupported` and `malformed` are the other two and are deliberately
    // absent. Measured on the pre-deletion tree, per state and per source: the
    // assurance chain alone reached ten, and the compatibility census over the
    // retained evidence supplied exactly these two. That census is deleted, and
    // neither state is re-demonstrated here.
    //
    // `unsupported` is not in the adapter's producer vocabulary and never was. It
    // was a property of the compatibility mapping — a retained record carrying an
    // unknown PGM-01 schema version — and nothing on the intake path produces it.
    // The corpus does reach an `unsupported` *Interface-001 terminal state*, and
    // borrowing that would be exactly the collapse of two vocabularies this test
    // exists to prevent.
    //
    // `malformed` is in that vocabulary, and it was still withdrawn, because
    // being a declared key is not the same as being distinguishable. Both
    // `ROW_RESULTS` and `CONFORMANCE_OUTCOMES` map it onto the same `fail`, so a
    // scenario feeding the adapter a stream declaring `outcome: malformed`
    // produces receipt reasons byte-identical to the `fail` case — which is why
    // `non-success-states-stay-distinguishable` could not have included it. Such
    // a scenario was written, measured, found to be `attested-failed` under
    // another name, and removed. Keeping it would have held this list at eleven
    // by painting one on, which is worse than ten that are real.
    //
    // Both claims went with the evidence rather than being restated over a weaker
    // substitute. The protection against a silent weakening is not a manufactured
    // demonstration; it is the assertion below that both states stay absent, so
    // re-acquiring either goes red instead of drifting.
    const NOT_DEMONSTRABLE_HERE: [&str; 2] = ["unsupported", "malformed"];

    let report = chain_report();

    let demonstrated: BTreeSet<String> = report["states_demonstrated"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect();

    let missing: Vec<&str> = REQUIRED
        .iter()
        .filter(|state| !demonstrated.contains(**state))
        .copied()
        .collect();
    assert!(
        missing.is_empty(),
        "these verification outcomes were never demonstrated: {missing:?}; \
         demonstrated: {demonstrated:?}"
    );

    // The two absent states must stay absent by measurement rather than by
    // comment. If a later change gives one of them a real demonstration, this
    // fails and the requirement is widened deliberately instead of drifting.
    for state in NOT_DEMONSTRABLE_HERE {
        assert!(
            !demonstrated.contains(state),
            "{state} is demonstrated again; FR-006-AC-5 and MP-001 record it as \
             having lost its only demonstration with the retained evidence. If a \
             real demonstration now exists, widen the requirement deliberately — \
             but check first that the new case is distinguishable in the receipt \
             and not another state under a second name"
        );
    }

    // Every negative names the positive control that proves the step it refuses
    // is a step that works.
    let controls = report["controls"].as_array().unwrap();
    assert!(!controls.is_empty(), "no positive controls were run");
    let negatives: BTreeSet<&str> = controls
        .iter()
        .map(|control| control["pairs_with"].as_str().unwrap())
        .collect();
    for required in [
        "retained-bytes-changed-after-sealing",
        "refuse-an-edited-receipt",
        "stale-candidate-binding",
        "attested-failed",
    ] {
        assert!(
            negatives.contains(required),
            "the negative {required} has no positive control"
        );
    }
}

/// Trace: TC-013, FR-006-AC-6
#[test]
fn tc_013_no_local_evidence_framework_remains_and_the_deleted_schemas_are_unreferenced() {
    let root = root();

    // Removed by a drop guard rather than by the line below alone: the census
    // loop panics on a real finding, and a gate that litters the source tree on
    // the way out leaves a file a later `git add -A` would commit. `.gitignore`
    // names it too, belt and braces.
    struct ProbeGuard(PathBuf);
    impl Drop for ProbeGuard {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.0);
        }
    }

    let probe_path = root.join("scripts/.census-probe.py");
    let _probe_guard = ProbeGuard(probe_path.clone());

    // The generic machinery is gone, by name.
    for removed in [
        "scripts/build_foundation_envelope.py",
        "scripts/collect_foundation_evidence.sh",
        "scripts/verify_foundation_evidence.py",
        "scripts/update_evidence_anchors.py",
        "scripts/check_failure_propagation.py",
        "scripts/check_coverage_status.py",
        "scripts/validate_json_schema.py",
        "scripts/evidence_policy.py",
        "scripts/run_python_tests.py",
        "tests/test_foundation_evidence_tooling.py",
        "requirements-evidence.txt",
        // The retained-evidence reader and its fixtures. They were kept only to
        // read the records under evidence/; those records are deleted, so the
        // reader that mapped them is too.
        "scripts/legacy_evidence_view.py",
        "tests/fixtures/legacy-compat/expectations.json",
    ] {
        assert!(
            !root.join(removed).exists(),
            "{removed} is still present; the generic evidence machinery was not removed"
        );
    }

    // The retained tree itself, and the two schemas that were frozen only because
    // retained records named them by SHA-256. With the records deleted there is
    // nothing left for those digests to keep intact, so the schemas went too.
    //
    // The set was two and not four, and that was measured rather than inherited:
    // schemas/pgm01-derivation-evidence-envelope-v1.schema.json is a live domain
    // contract here, included by src/oracle.rs as PGM_SCHEMA and validated
    // against on every generation. Sharing a directory and a `pgm01` in the
    // filename with the retained material did not make it retained material. It
    // is asserted live below for exactly that reason.
    let deleted_schemas = [
        "schemas/foundation-evidence-input-v1.schema.json",
        "schemas/foundation-evidence-manifest-v1.schema.json",
    ];
    for path in deleted_schemas {
        assert!(
            !root.join(path).exists(),
            "{path} is still present; it was frozen only for the retained records \
             that named it, and those are deleted"
        );
    }
    assert!(
        !root.join("evidence").exists(),
        "evidence/ is still present; the retained records were not deleted"
    );

    // And the three live ones stay live. Asserting this is what stops a later
    // tidy-up from deleting them on the strength of the directory they happen to
    // share with the two above.
    let generator = fs::read_to_string(root.join("src/oracle.rs")).unwrap();
    for live in [
        "schemas/pgm01-derivation-evidence-envelope-v1.schema.json",
        "schemas/generated-rust-oracle-v1.schema.json",
        "schemas/oracle-source-map-v1.schema.json",
    ] {
        assert!(
            root.join(live).is_file(),
            "{live} is a live domain contract and was deleted"
        );
        let name = Path::new(live).file_name().unwrap().to_str().unwrap();
        assert!(
            generator.contains(name),
            "{live} is declared live but src/oracle.rs does not include it"
        );
    }

    // Nothing references the deleted pair, or the deleted reader, any more. The
    // census walks the repository root and excludes, rather than naming the
    // directories it will look in: an inclusion list is a list of the places a
    // reintroduced validator would have to avoid, and it only has to be
    // incomplete once.

    // Scanned broadly, counted narrowly, and the two are different populations
    // on purpose.
    //
    // The reference scan runs over everything the walk finds — tracked and
    // untracked alike — because untracked is exactly the state a reintroduced
    // reader is in while someone is still writing it, and a census that only
    // reads committed files would not see it until after it was `git add`ed.
    //
    // The counts below use the tracked set only. A walk-based count can be
    // inflated by anything lying around: `proptest-regressions/*.txt` and a
    // scratch `.json` in the repository root are both admitted by the extension
    // filter, and two stray files are enough to restore headroom that real
    // deletions had consumed. Scratch cannot vote on whether the census is big
    // enough to prove anything.
    let tracked_output = Command::new("git")
        .args(["ls-files", "-z"])
        .current_dir(&root)
        .output()
        .expect("git ls-files failed");
    assert!(
        tracked_output.status.success(),
        "git ls-files failed; the census cannot tell tracked files from scratch"
    );
    let tracked: BTreeSet<PathBuf> = String::from_utf8_lossy(&tracked_output.stdout)
        .split('\0')
        .filter(|entry| !entry.is_empty())
        .map(|entry| root.join(entry))
        .collect();
    assert!(
        !tracked.is_empty(),
        "git ls-files reported no tracked files; the counts below would be vacuous"
    );

    // Stems, not filenames, and the rule is applied to every entry rather than
    // to the one that prompted it.
    //
    // The reader was listed by its full filename, and a Python import of the
    // same module -- the likeliest form of reintroduction -- spells it without
    // the extension and went straight past. Fixing that one name and leaving the
    // other ten was the same mistake at a smaller scale, and it was measured: a
    // workflow running the deleted test by its Rust function-name spelling was
    // green, because the list carried only the hyphenated document id while this
    // repository's own convention for test functions is the underscored form;
    // and the two schemas kept their full `.schema.json` filenames while the
    // census that justified deleting them searched by the bare stem.
    //
    // So: suffixes stripped, and every name that has a hyphen/underscore pair in
    // circulation carries both. A suffixed or decorated spelling is a superstring
    // of the stem, so the stem subsumes it; the filename subsumed nothing.
    //
    // Only the arrays below are exempt from the census, not the prose around
    // them, so this comment must not spell any of the names it is about.
    // Two statements of the same list, compared. The probe below is built from
    // `gone`, so a `gone` that loses an entry loses it from the probe too and the
    // control agrees with itself -- measured green, which is the third time this
    // shape has appeared in this test. `EXPECTED_GONE` is the independent second
    // statement, and the `assert_eq!` is what a single-line deletion lands on.
    //
    // It does not defend against someone editing both, and nothing in one file
    // can. It defends against the realistic failure, which is an entry quietly
    // dropped while the control keeps reporting success.
    const EXPECTED_GONE: [&str; 14] = [
        "COMPAT_RESULT",
        "FR-006-AC-4",
        "PROOF-legacy-compatibility",
        "SUITE-005",
        "TC-011",
        "compat-view",
        "compat_view",
        "foundation-evidence-input-v1",
        "foundation-evidence-manifest-v1",
        "legacy-compat",
        "legacy-compatibility",
        "legacy_compat",
        "legacy_evidence_view",
        "tc_011",
    ];
    let gone: Vec<&str> = deleted_schemas
        .iter()
        .map(|path| {
            Path::new(path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_end_matches(".schema.json")
        })
        .chain([
            "legacy_evidence_view",
            "legacy-compat",
            "legacy_compat",
            "PROOF-legacy-compatibility",
            "compat-view",
            "compat_view",
            "COMPAT_RESULT",
            "legacy-compatibility",
            "SUITE-005",
            "TC-011",
            "tc_011",
            "FR-006-AC-4",
        ])
        .collect();
    assert_eq!(
        gone.iter().copied().collect::<BTreeSet<&str>>(),
        EXPECTED_GONE.into_iter().collect::<BTreeSet<&str>>(),
        "the forbidden-name list changed. Every name here is a thing this change \
         deleted, so removing one silently stops checking for it -- and the probe \
         is built from this list, so the control below would agree"
    );

    // The probe carries **every** name in `gone`, and the control below requires
    // each one to have been matched inside it.
    //
    // Carrying one name was enough to show the census sees untracked files and
    // nothing more. `gone` could then be reduced to that single entry and the
    // control still agreed, which is precisely the defect the directory guard's
    // `assert_eq!` was added to fix, left standing here: a declaration whose
    // shrinking removes the check. Nine of these names are hand-written and
    // corroborated by nothing else, so the probe corroborates them.
    let probe_body = format!(
        "# Census probe, written and removed by tc_013. Never commit this file.\n\
         # It names every deleted artifact on purpose: the census must detect each.\n\
         {}\n",
        gone.iter()
            .map(|name| format!("# {name}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    fs::write(&probe_path, &probe_body).expect("write the census probe");
    assert!(
        !tracked.contains(&probe_path),
        "the census probe at {} is tracked. The control below is about untracked \
         files, and a committed probe satisfies it while the census is blind to \
         the state a reintroduced reader is actually in",
        probe_path.display()
    );

    // Collected after the probe is written, so the probe is in the census's own
    // set rather than in a second walk taken on its behalf.
    let mut sources = Vec::new();
    collect_sources(&root, &mut sources);

    // The claim is that nothing *reads* them, so the assertion runs over the
    // surfaces that can: code, configuration, and workflow files. Markdown is
    // excluded and deliberately so — prose cannot read anything, and this
    // repository's planning documents and reviews are a record of what was
    // deleted and why.
    //
    // Every name a live surface could still reach for, not just the files. An
    // earlier form of this census listed three basenames and exempted this file
    // wholesale, and a stale reference to the deleted reader in the `make -n ci`
    // list two hundred lines below survived it: the exemption covered the very
    // file the dangling reference was in. So the list is wide, and the exemption
    // is the literal declaration below rather than the file that holds it.

    // The exemptions, and they are three disjoint ranges over literal
    // declarations rather than one span between the first and the last.
    //
    // The span form covered ~131 lines: the `ProbeGuard` impl, three assertion
    // blocks, the live-schema loop and the `collect_sources` call all sat inside
    // it, so a live `Command::new` invoking the deleted reader, inserted among
    // them, was exempt. Measured green. Only the arrays that spell
    // the forbidden names need an exemption, and only those get one.
    let this_file = fs::read_to_string(root.join("tests/shared_assurance.rs")).unwrap();
    let literal_ranges: Vec<(usize, usize)> = [
        ("    for removed in [", "    ] {"),
        ("    let deleted_schemas = [", "    ];"),
        ("    const EXPECTED_GONE:", "    ];"),
        (
            "    let gone: Vec<&str> = deleted_schemas",
            "        .collect();",
        ),
    ]
    .iter()
    .map(|(open, close)| {
        let start = this_file
            .find(open)
            .unwrap_or_else(|| panic!("the declaration beginning {open:?} moved or was renamed"));
        let end = this_file[start..]
            .find(close)
            .map(|offset| start + offset + close.len())
            .unwrap_or_else(|| panic!("the declaration beginning {open:?} has no {close:?}"));
        (start, end)
    })
    .collect();
    for window in literal_ranges.windows(2) {
        assert!(
            window[0].1 <= window[1].0,
            "the exempt ranges overlap or are out of order, so one of them is not the \
             declaration it is meant to be: {literal_ranges:?}"
        );
    }

    let mut inspected = 0;
    let mut probe_matched: BTreeSet<&str> = BTreeSet::new();
    for path in &sources {
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if extension == "md" {
            continue;
        }
        // Decoded lossily rather than skipped. `read_to_string` returns `Err` on
        // the first non-UTF-8 byte, and a `continue` there is a silent exclusion
        // that no deny-list entry names: one `\xe9` in a comment made a live
        // invocation of the deleted reader invisible, and three
        // `__pycache__/*.pyc` files were already being dropped this way. It also
        // quietly lowered `inspected` and the per-directory counts, eroding the
        // floors at the same time. An unreadable file is a read failure and is
        // reported; an undecodable one is decoded as far as it goes.
        let Ok(bytes) = fs::read(path) else {
            panic!(
                "{} was admitted by the filter but could not be read; the census \
                 cannot claim to have examined it",
                path.display()
            )
        };
        // Interleaved NULs mean a wide encoding, and `from_utf8_lossy` turns a
        // UTF-16 spelling of any forbidden name into `l\0e\0g\0...` that no
        // substring ever matches. Measured: a UTF-16 live import was invisible while the
        // stray-byte case was caught. Rust would reject such a source file, but
        // Python's `json` decoder autodetects UTF-16 and this repository reads
        // JSON configuration from Python, so it is refused rather than scanned
        // badly.
        assert!(
            !bytes.windows(2).any(|pair| pair[1] == 0 && pair[0] != 0),
            "{} looks like a wide encoding (interleaved NUL bytes). The census \
             matches byte substrings and would silently see nothing in it",
            path.display()
        );
        let source = String::from_utf8_lossy(&bytes).into_owned();
        if tracked.contains(path) {
            inspected += 1;
        }
        // The change declaration is the one document whose job is to say what was
        // deleted, so it necessarily names the obligation, the target and the
        // criteria that went. Exempting it does not weaken the claim, because
        // nothing in it is checked by substring search anyway: every resolvable
        // element it carries has a stricter gate. `record_body` raises if a
        // declared source or proof configuration path is missing, and `tc_009`
        // requires every declared argv to appear verbatim in
        // `make -n assurance-inputs`. A dangling path or a stale command in this
        // file fails those, not this census.
        // Compared by full path. A bare basename check exempted any
        // `change-assurance.json` anywhere in the tree, so dropping one into
        // `scripts/` bought a whole-file exemption for free.
        if path == &root.join("assurance/change-assurance.json") {
            continue;
        }
        let is_this_file = path.file_name().and_then(|value| value.to_str())
            == Some("shared_assurance.rs")
            && source == this_file;
        for name in &gone {
            // `match_indices`, not `find`. `find` returns the first occurrence
            // only, and in this file every one of these names occurs first inside
            // the exempt declarations below — so a `find`-based loop `continue`d
            // on the exemption and never looked at anything after it. That made
            // the byte range behave exactly like the whole-file exemption it
            // replaced: six stale references appended to the end of this file
            // were measured passing. Every occurrence is now judged on its own.
            for (at, _) in source.match_indices(name) {
                // Inside this test's own declarations, naming the artifact is the
                // point. Anywhere else — including elsewhere in this same file —
                // it is a live reference to something that does not exist.
                if is_this_file
                    && literal_ranges
                        .iter()
                        .any(|(start, end)| at >= *start && at < *end)
                {
                    continue;
                }
                // The census probe is the positive control, and it is detected
                // here rather than asserted about somewhere else. It is a real
                // untracked file carrying a real deleted name, judged by the real
                // predicate on the real code path; the only thing special about
                // it is that finding it is the expected outcome instead of a
                // failure. A census that cannot see untracked files, or whose
                // matching is broken, does not set this flag.
                if path == &probe_path {
                    probe_matched.insert(name);
                    continue;
                }
                panic!(
                    "{} references the deleted artifact {name} at byte {at}",
                    path.display()
                );
            }
        }
    }
    // The control for the untracked half of the scan.
    //
    // Three weaker forms were written before this one and each was measured
    // passing while the census was blind. A second `collect_sources` call tests
    // the walker, not the census. Asserting on the collected `sources` misses
    // anything that narrows the set afterwards. Asserting on a `scanned` set
    // recorded at the top of the loop misses anything that narrows it between
    // there and the check that matters — which is where the narrowing would be.
    //
    // The only form that cannot be satisfied by bookkeeping is this one: the
    // probe is an untracked file carrying a real deleted name, and the flag is
    // set by the same `match_indices` branch that panics on every other file. To
    // pass, the census has to have walked to an untracked file, read it, and
    // matched a deleted name in it. There is no separate accounting to keep in
    // step.
    fs::remove_file(&probe_path).expect("remove the census probe");
    assert!(
        !probe_path.exists(),
        "the census probe was not cleaned up at {}",
        probe_path.display()
    );
    let unmatched: Vec<&str> = gone
        .iter()
        .copied()
        .filter(|name| !probe_matched.contains(name))
        .collect();
    assert!(
        unmatched.is_empty(),
        "the reference census did not detect {unmatched:?} in an untracked file that \
         contains every forbidden name. Untracked is the state a reintroduced reader is \
         in before anyone commits it; and a name missing here is a name the census would \
         not have caught anywhere, so shrinking the list below lands on this assertion"
    );

    // Re-derived rather than inherited, and every figure below was taken from the
    // walk and from `git ls-files` rather than from any document describing the
    // layout. This repository has no `schemas/README.md` or equivalent, and no
    // number here is sourced from prose.
    //
    // Measured with the deny-list above, so both sides are the same filter:
    //
    //     origin/main, `evidence/` excluded as it was then   45
    //     this head                                          31
    //     removed  14 = 1 reader + 11 fixtures + 2 schemas
    //     added     0
    //     45 - 14 + 0 = 31, and the census counts 31
    //
    // `inspected` counts a tracked file once it has been read, before the
    // change-declaration exemption below, so it is the walked tracked count and
    // not that count minus the exemptions. An earlier form incremented after the
    // exemption and reported 30 while the comment claimed 31 — a derivation
    // beside a threshold that its own arithmetic did not support, which is the
    // exact defect this comment was rewritten to remove.
    //
    // The old floor was `> 20`, which would have kept passing while a third of
    // the inspectable surface disappeared.
    //
    // The margin is 4 and it is chosen, not derived — no rule fixes it, and
    // printing arithmetic that does not close would be worse than printing none.
    // What makes 4 safe is that this floor is not the instrument that catches a
    // directory disappearing: the set comparison and the per-directory floors
    // below do that, and they trip on losses far smaller than 4. This number only
    // has to catch diffuse attrition across the tree.
    assert!(
        inspected >= 27,
        "the executable and configuration census is unexpectedly small ({inspected}, \
         floor 27, measured 31 at the time it was derived) to make this claim"
    );

    // A total-only floor is the wrong instrument on a tree this small. `scripts`
    // and `tests` are five files each and `src` is four, so an entire directory
    // could vanish and move the total by less than ordinary churn would.
    //
    // The guard below is built by *discovery* — the directories are whatever the
    // walk actually found — and then compared against a declared set. That
    // ordering is the whole point, and it was arrived at by probing the obvious
    // version and watching it fail. A guard whose directory list is the same list
    // the walk iterates cannot catch that list shrinking: deleting an entry
    // removes the directory from the check and from the walk at the same moment,
    // and the test stays green. Measured here before this form replaced it —
    // removing the `scripts` entry left `tc_013` passing.
    //
    // So the names are asserted first and the counts second. A directory that
    // empties out fails the set comparison; an entry deleted from the declared
    // set fails it too; and a directory that merely shrinks fails its floor.
    let mut per_directory: BTreeMap<String, usize> = BTreeMap::new();
    for path in &sources {
        if !tracked.contains(path) {
            continue;
        }
        let Ok(relative) = path.strip_prefix(&root) else {
            continue;
        };
        if relative.extension().and_then(|value| value.to_str()) == Some("md") {
            continue;
        }
        let top = if relative.components().count() <= 1 {
            "<root>".to_owned()
        } else {
            relative
                .components()
                .next()
                .map(|component| component.as_os_str().to_string_lossy().into_owned())
                .unwrap_or_else(|| "<root>".to_owned())
        };
        *per_directory.entry(top).or_default() += 1;
    }

    // Declared, and every floor beside it was measured from this walk rather than
    // read off a document describing the layout.
    let declared: BTreeMap<&str, (usize, usize)> = BTreeMap::from([
        ("<root>", (7usize, 9usize)),
        (".github", (1, 2)),
        ("assurance", (2, 2)),
        ("examples", (1, 1)),
        ("schemas", (3, 3)),
        ("scripts", (4, 5)),
        ("src", (4, 4)),
        ("tests", (4, 5)),
    ]);
    let found: BTreeSet<&str> = per_directory.keys().map(String::as_str).collect();
    let expected: BTreeSet<&str> = declared.keys().copied().collect();
    assert_eq!(
        found, expected,
        "the set of directories the census walks changed. A directory that stopped being \
         walked, or an entry removed from the declared set, both land here — which is the \
         failure a per-directory floor built from the walk's own list cannot see"
    );
    for (directory, (floor, measured)) in &declared {
        let seen = per_directory.get(*directory).copied().unwrap_or(0);
        assert!(
            seen >= *floor,
            "the census saw {seen} inspectable files under {directory}, below its floor of \
             {floor} (measured {measured} when derived). A directory that empties out is how \
             a reference census stops proving anything while its total still looks healthy"
        );
    }

    // The Makefile is orchestration, not a trust root, and carries no gate that
    // polices its own execution. Target definitions are matched, not bare words.
    let makefile = fs::read_to_string(root.join("Makefile")).unwrap();
    for gone in [
        "\nverify-evidence:",
        "\nevidence-tool:",
        "\nci-guard:",
        "\ncoverage:",
    ] {
        assert!(
            !makefile.contains(gone),
            "the Makefile still defines the {gone} self-attestation target"
        );
    }
    assert!(
        !makefile.contains("MAKEFLAGS"),
        "the Makefile still polices its own execution controls"
    );

    // The directive scan below reads this one file, so an `include` would carry
    // it past. Measured: a scratch `.mk` declaring `.IGNORE:` plus an `include`
    // line left this test green while `make` on a failing recipe exited 0. Issue
    // #14 already records that a green `make ci` says nothing about an *edited*
    // Makefile; this assertion is about the narrower claim the scan itself
    // makes, which was false. There is no include in this file, and adding one
    // has to be a deliberate change that comes here first.
    for line in makefile.lines() {
        if line.starts_with('\t') {
            continue;
        }
        let statement = line.split('#').next().unwrap_or("").trim();
        // Matched on the directive word, not on `"include "` with a trailing
        // space: make honours `include<TAB>file.mk` just as happily, and a
        // one-character change put `.IGNORE:` back through an include with this
        // test green.
        let directive = statement.split_whitespace().next().unwrap_or("");
        for form in ["include", "-include", "sinclude"] {
            assert!(
                directive != form,
                "the Makefile includes another file, which the directive scan below \
                 does not follow: {line}"
            );
        }
    }

    // Make can be told to ignore failure, and a single line does it. `.IGNORE:`
    // at the top, a `-` prefix on a recipe line, or an assignment to `SHELL` each
    // turn a red gate into `make` exit 0 while the gate itself still prints its
    // failure. The recipe-failure policer that used to catch this went with the
    // collector it was protecting.
    //
    // What replaces it is not another policer target — that would be the Makefile
    // attesting to itself again. It is this assertion, in the test suite, that the
    // file declares none of the directives whose only purpose is to stop a failure
    // propagating. That protects a reviewer reading a diff and nothing more; the
    // residual is recorded in NFR-002, AA-001, the Makefile header and
    // agent-ix/quire-contract-codegen#14, with the numbers measured here.
    //
    // Directives are matched structurally, on the lines that could be one: not
    // comments, not recipe bodies. The header of that file explains this very
    // hazard and names `.IGNORE:` to do so, and a substring scan would report the
    // rule being written down as a violation of itself.
    for line in makefile.lines() {
        if line.starts_with('\t') {
            continue;
        }
        let statement = line.split('#').next().unwrap_or("").trim();
        if statement.is_empty() {
            continue;
        }
        for directive in [".IGNORE", ".SILENT", ".ONESHELL", ".SHELLFLAGS"] {
            assert!(
                !statement.starts_with(directive),
                "the Makefile declares {directive}, which stops a failing gate from \
                 failing the build: {line}"
            );
        }
        // `override` and `export` are stripped first. Without that,
        // `override SHELL := /usr/bin/true` split to the target
        // `"override SHELL"` and passed -- and `override` is this Makefile's own
        // house spelling, used for six variables already, so it is the likely
        // form rather than an exotic one. Measured: with it prepended, a failing
        // recipe exited 0 and this test stayed green.
        let bare = statement
            .strip_prefix("override ")
            .or_else(|| statement.strip_prefix("export "))
            .unwrap_or(statement)
            .trim_start();
        let assigns_shell = bare
            .split_once([':', '=', '?'])
            .map(|(target, _)| target.trim() == "SHELL")
            .unwrap_or(false);
        assert!(
            !assigns_shell,
            "the Makefile assigns SHELL, which can make every recipe report success: {line}"
        );
    }
    for (number, line) in makefile.lines().enumerate() {
        let Some(recipe) = line.strip_prefix('\t') else {
            continue;
        };
        let command = recipe.trim_start_matches(['@', '+']);
        assert!(
            !command.starts_with('-'),
            "Makefile:{} prefixes a recipe line with `-`, which ignores its exit status: {line}",
            number + 1
        );
    }

    // And the gates that replaced it are actually reachable from `ci:`.
    //
    // This asks Make what it would run, not what the file says. A text assertion
    // that the Makefile mentions a script is satisfied by the script being
    // mentioned in a comment, and survives the entire `ci:` prerequisite list
    // being deleted. `make -n` expands the dependency graph, so removing a
    // prerequisite removes its recipe line from this output.
    let dry_run = Command::new("make")
        .args(["-n", "ci"])
        .current_dir(&root)
        .env_remove("CARGO")
        .output()
        .expect("make -n ci failed to run");
    assert!(
        dry_run.status.success(),
        "make -n ci did not resolve: {}",
        String::from_utf8_lossy(&dry_run.stderr)
    );
    let planned = String::from_utf8_lossy(&dry_run.stdout);
    for required in [
        "--example generation_conformance",
        "scripts/check_upstream_pins.py",
        "scripts/check_shared_pins.py",
        "scripts/assurance_chain.py",
        "scripts/check_unsafe_comments.sh",
    ] {
        assert!(
            planned.contains(required),
            "`make ci` would not run {required}; it is defined but unreachable"
        );
    }

    // The two test runners, told apart.
    //
    // Without this, deleting `test` from the `ci:` prerequisite list is invisible:
    // `assurance-inputs` still supplies every script named above, so `make ci`
    // stays green while TC-008 through TC-013 — the whole enforcement layer for
    // FR-006 — never runs.
    //
    // Matching the bare string `test --locked` was not enough, and an adversarial
    // review proved it: `msrv:` is `cargo +1.75.0 test --locked`, so it supplies
    // that string on its own and `test` could be deleted from `ci:` undetected.
    // The two are distinguished by the toolchain selector, which is the only thing
    // that differs between them.
    let mut stable_runner = false;
    let mut msrv_runner = false;
    for line in planned.lines() {
        if !line.contains("test --locked") {
            continue;
        }
        if line.contains("+1.75.0") {
            msrv_runner = true;
        } else {
            stable_runner = true;
        }
    }
    assert!(
        stable_runner,
        "`make ci` would not run `cargo test --locked` on the default toolchain; \
         the `test` prerequisite is unreachable and the FR-006 gates never run.\n\
         Plan: {planned}"
    );
    assert!(
        msrv_runner,
        "`make ci` would not run the MSRV test lane; the `msrv` prerequisite is \
         unreachable.\nPlan: {planned}"
    );
}

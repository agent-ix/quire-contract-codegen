//! Test helpers shared by the tests of more than one `kani/` file.

use std::{env, fs, path::PathBuf};

use crate::{
    core::{
        artifact::Artifact,
        identity::{HarnessSymbol, ModuleSymbol},
    },
    kani::{
        abi::KaniSolver,
        identity::{StateFrameHarness, StateFrameIdentity, StateFrameProperty, StateFrameScope},
    },
};

pub(crate) fn state_frame_harness(
    property: StateFrameProperty,
    options: Vec<String>,
) -> StateFrameHarness {
    named_state_frame_harness("m", "check", property, options)
}

/// [`state_frame_harness`] in module `module` with proof function `harness`, so that two
/// harnesses share a bare symbol and differ in their `module::harness` path.
pub(crate) fn named_state_frame_harness(
    module: &str,
    harness: &str,
    property: StateFrameProperty,
    options: Vec<String>,
) -> StateFrameHarness {
    let id = |digit: &str| -> quire_contract_model::CheckedNodeId {
        serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": digit.repeat(64),
        }))
        .expect("a node id")
    };
    StateFrameHarness {
        identity: StateFrameIdentity {
            ceilings: crate::kani::test_support::proof_ceilings_with_wall_clock(
                std::time::Duration::from_secs(30),
            ),
            clause: id("1"),
            scope: StateFrameScope {
                operation: "deposit".to_owned(),
                object: id("2"),
                anchor: id("3"),
                frame: id("4"),
            },
            property,
            state_fields: Vec::new(),
            domains: Vec::new(),
            unranged: Vec::new(),
            state_path: "crate::State".to_owned(),
            subject_path: "crate::operate".to_owned(),
            module_symbol: ModuleSymbol::try_from(module).unwrap(),
            harness_symbol: HarnessSymbol::try_from(harness).unwrap(),
            solver: KaniSolver::Cadical,
            unwind: 4,
            options,
        },
        rust: Artifact::new(format!("src/generated/{module}.rs"), String::new()),
        record: Artifact::new(format!("kani-obligations/{module}.json"), String::new()),
    }
}

/// One harness result of a batch report: the `harness_id` Kani names it by, its status, its
/// checks as `(status, category)` and, when Kani's own timeout stopped it, the `timeout` exit
/// status of its `error_details` entry.
pub(crate) struct BatchEntry<'a> {
    pub(crate) harness_id: &'a str,
    pub(crate) status: &'a str,
    pub(crate) checks: &'a [(&'a str, &'a str)],
    pub(crate) exit_status: Option<&'a str>,
}

/// A report of several harnesses, as Kani 0.68 writes one: results in the order given (which is
/// not the order the launch asked in), and an `error_details` entry for each harness, carrying an
/// `exit_status` only for one that errored.
pub(crate) fn batch_report(entries: &[BatchEntry<'_>]) -> Vec<u8> {
    let results: Vec<_> = entries
        .iter()
        .map(|entry| {
            let checks: Vec<_> = entry
                .checks
                .iter()
                .enumerate()
                .map(|(index, (status, category))| {
                    serde_json::json!({
                        "id": index + 1,
                        "status": status,
                        "category": category,
                        "location": { "file": "src/lib.rs", "line": "10", "column": "5" },
                    })
                })
                .collect();
            serde_json::json!({
                "harness_id": entry.harness_id, "status": entry.status, "checks": checks
            })
        })
        .collect();
    let details: Vec<_> = entries
        .iter()
        .map(|entry| match entry.exit_status {
            Some(exit_status) => serde_json::json!({
                "harness_id": entry.harness_id, "has_errors": true, "exit_status": exit_status
            }),
            None => serde_json::json!({ "harness_id": entry.harness_id, "has_errors": false }),
        })
        .collect();
    serde_json::to_vec(&serde_json::json!({
        "metadata": { "version": "1.0" },
        "error_details": details,
        "verification_results": { "results": results }
    }))
    .unwrap()
}

/// A report of one harness, as the checks Kani listed for it.
pub(crate) fn report(status: &str, checks: &[(&str, &str)]) -> Vec<u8> {
    let checks: Vec<_> = checks
        .iter()
        .enumerate()
        .map(|(index, (status, category))| {
            serde_json::json!({
                "id": index + 1,
                "status": status,
                "category": category,
                "location": { "file": "src/lib.rs", "line": "10", "column": "5" },
            })
        })
        .collect();
    serde_json::to_vec(&serde_json::json!({
        "metadata": { "version": "1.0" },
        "verification_results": {
            "results": [{ "harness_id": "m::h", "status": status, "checks": checks }]
        }
    }))
    .unwrap()
}

pub(crate) const PASSED: (&str, &str) = ("Success", "assertion");
pub(crate) const COVER_OK: (&str, &str) = ("Satisfied", "cover");
pub(crate) const COVER_NO: (&str, &str) = ("Unsatisfiable", "cover");

/// Writes the launcher stand-in `body` (a `sh` script) at `path`, executable, and returns once it
/// can be started.
///
/// A process another test thread forked while the script was open for writing holds that
/// descriptor until it execs, and `exec` of a file open for writing is refused as busy. The
/// script exits at once when run with `--probe`, and this runs it that way until the refusal
/// stops, so no test run of the stand-in meets it.
pub(crate) fn write_launcher(path: &std::path::Path, body: &str) {
    use std::os::unix::fs::PermissionsExt;
    fs::write(
        path,
        format!("#!/bin/sh\n[ \"$1\" = --probe ] && exit 0\n{body}"),
    )
    .unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    for _ in 0..500 {
        match std::process::Command::new(path).arg("--probe").status() {
            Err(error) if error.kind() == std::io::ErrorKind::ExecutableFileBusy => {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            _ => break,
        }
    }
}

/// A scratch directory unique to this process and this call, so parallel tests never
/// collide and nothing here touches a real `$HOME` or `$CARGO_HOME`.
pub(crate) fn discover_scratch(name: &str) -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = env::temp_dir().join(format!(
        "quire-kani-discover-{name}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&path).unwrap();
    path
}

use crate::kani::identity::ProofCeilings;
#[path = "../../tests/common/proof_ceilings.rs"]
mod proof_ceilings;
pub(crate) use proof_ceilings::{proof_ceilings, proof_ceilings_with_wall_clock};

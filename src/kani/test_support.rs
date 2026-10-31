//! Test helpers shared by the tests of more than one `kani/` file.

use std::fs;

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
            "package": "test-kani-harness",
            "path": [digit],
            "ordinal": 0,
        }))
        .expect("a node id")
    };
    StateFrameHarness {
        identity: StateFrameIdentity {
            ceilings: crate::kani::test_support::proof_ceilings::proof_ceilings_with_wall_clock(
                std::time::Duration::from_secs(30),
            ),
            package: serde_json::from_value(serde_json::json!({
                "name": "test-kani-harness",
                "lock": {
                    "sources": [],
                    "edition": {"role": "edition", "definition": {"authority": "test", "identity": "edition"}},
                    "profile_selections": [],
                    "definition_selections": [],
                    "required_features": [],
                    "dependency_selections": []
                }
            }))
            .expect("test package source"),
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

use crate::kani::identity::ProofCeilings;
#[path = "../../tests/common/proof_ceilings.rs"]
pub(crate) mod proof_ceilings;

/// Autoref probe: `implements_deserialize` is `true` through `Yes` only for a type that is
/// `DeserializeOwned`, and `false` through `No` otherwise, with no dependency beyond serde.
pub(crate) struct Probe<T>(pub(crate) std::marker::PhantomData<T>);

pub(crate) trait Yes {
    fn implements_deserialize(&self) -> bool;
}

impl<T: serde::de::DeserializeOwned> Yes for Probe<T> {
    fn implements_deserialize(&self) -> bool {
        true
    }
}

pub(crate) trait No {
    fn implements_deserialize(&self) -> bool;
}

impl<T> No for &Probe<T> {
    fn implements_deserialize(&self) -> bool {
        false
    }
}

/// Literal source and identity for runner-only host checks; no generator is invoked.
pub(crate) fn scalar_source_fixture(
    wall_clock: std::time::Duration,
) -> (
    crate::kani::identity::KaniScalarObligationHarness,
    Vec<Artifact>,
) {
    use crate::kani::identity::{
        KaniScalarObligationHarness, ScalarObligationArgument, ScalarObligationIdentity,
    };
    let package_name = "quire-exact-scalar-oracles";
    let identity = ScalarObligationIdentity {
        integer_representation: crate::kani::identity::ScalarIntegerRepresentation::General,
        native_integer_selection: None,
        ceilings: proof_ceilings::proof_ceilings_with_wall_clock(wall_clock),
        node_id: serde_json::from_value(serde_json::json!({
            "package": package_name,
            "path": ["host_scalar", "oracle"],
            "ordinal": 0,
        }))
        .expect("generated host_scalar::oracle structural node id"),
        operation_identity: "quire.op.integer.add".into(),
        oracle_symbol: "oracle".into(),
        module_symbol: ModuleSymbol::try_from("host_scalar").unwrap(),
        harness_symbol: HarnessSymbol::try_from("check").unwrap(),
        arguments: ["left", "right"]
            .into_iter()
            .map(|identifier| ScalarObligationArgument {
                identifier: identifier.into(),
                minimum: 0,
                maximum: 9,
            })
            .collect(),
        inferred_operands: Vec::new(),
        solver: KaniSolver::Cadical,
        unwind: 3,
        options: vec![
            "--harness".into(),
            "host_scalar::check".into(),
            "--unwind".into(),
            "3".into(),
        ],
    };
    let source = "pub fn oracle(left: i64, right: i64, meter: &mut ()) -> i64 { let _ = meter; left + right }\n";
    let rust = Artifact::new("src/generated/host_scalar.rs", source);
    let record = Artifact::new(
        "kani-obligations/host_scalar.json",
        serde_json::json!({
            "identity": identity, "rustPath": rust.path,
        })
        .to_string(),
    );
    let artifacts = vec![
        Artifact::new(
            "Cargo.toml",
            crate::core::profile::oracle_crate_manifest(package_name),
        ),
        Artifact::new("src/lib.rs", source),
        Artifact::new("claim-map.json", "{}"),
        Artifact::new("source-maps/lib.json", "{}"),
    ];
    let mut harness = KaniScalarObligationHarness::generated(identity, rust, record);
    harness.bind_artifact_package(&artifacts);
    (harness, artifacts)
}

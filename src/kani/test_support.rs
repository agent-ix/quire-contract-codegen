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
    let id = |digit: &str| -> quire_contract_model::CheckedNodeId {
        serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": digit.repeat(64),
        }))
        .expect("a node id")
    };
    StateFrameHarness {
        identity: StateFrameIdentity {
            clause: id("1"),
            scope: StateFrameScope {
                operation: "deposit".to_owned(),
                object: id("2"),
                anchor: id("3"),
                frame: id("4"),
            },
            property,
            domains: Vec::new(),
            state_path: "crate::State".to_owned(),
            subject_path: "crate::operate".to_owned(),
            module_symbol: ModuleSymbol::try_from("m").unwrap(),
            harness_symbol: HarnessSymbol::try_from("check").unwrap(),
            solver: KaniSolver::Cadical,
            unwind: 4,
            options,
        },
        rust: Artifact::new("src/generated/m.rs".to_owned(), String::new()),
        record: Artifact::new("kani-obligations/m.json".to_owned(), String::new()),
    }
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

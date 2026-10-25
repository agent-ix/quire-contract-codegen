//! Compiled production-oracle controls for real native and general Meter behavior.

use super::{package, routed_native_scalar_with_artifacts, Bound, NativeMutationScratch};
use ix_trace_rs::trace;
use std::{fs, path::PathBuf, time::Duration};

#[trace("FR-015-AC-127", "TC-027")]
#[test]
fn routed_native_oracle_preserves_denial_cancellation_counters_and_refusal() {
    let (harness, artifacts) = routed_native_scalar_with_artifacts(
        package::bounded_increment_package(),
        package::BOUNDED_INCREMENT,
        49_100,
        Bound::Integer(0, 9),
        Duration::from_secs(600),
    );
    let scratch = NativeMutationScratch::create();
    for artifact in &artifacts {
        let path = scratch.0.join(&artifact.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("artifact parent");
        }
        fs::write(path, &artifact.contents).expect("routed production artifact");
    }
    fs::create_dir_all(scratch.0.join("src/bin")).expect("control parent");
    let control = include_str!("kani_scalar_native_meter_control.rs.txt")
        .replace("__ORACLE__", &harness.identity.oracle_symbol);
    fs::write(scratch.0.join("src/bin/meter_control.rs"), control).expect("meter control");
    fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
        scratch.0.join("Cargo.lock"),
    )
    .expect("seeded owner dependency lock");
    let cargo = |args: &[&str]| {
        let output = std::process::Command::new("cargo")
            .args(args)
            .env(
                "CARGO_TARGET_DIR",
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"),
            )
            .current_dir(&scratch.0)
            .output()
            .expect("run compiled production meter control");
        assert!(
            output.status.success(),
            "cargo {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    cargo(&["metadata", "--offline", "--format-version", "1"]);
    cargo(&[
        "run",
        "--offline",
        "--locked",
        "--quiet",
        "--bin",
        "meter_control",
    ]);
}

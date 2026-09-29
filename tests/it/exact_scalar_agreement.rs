//! FR-014-AC-6: generated exact scalar oracles and direct Contract Runtime
//! execution agree on the QSpec TC-185, TC-186, TC-187, TC-192 and TC-193
//! vectors, in outcome, every admitted charge, every consumed counter, and
//! the outcome of denying each admitted charge.
//!
//! Nothing generated is committed. This module generates the corpus crate now,
//! builds it as a scratch crate whose one integration test is
//! `tests/exact_scalar_support/agreement_cases.rs`, and runs that test. The
//! scratch-crate runner, [`run_agreement_cases`], is shared with the composite
//! equality and function-application agreement modules.

use std::{
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Development dependencies the agreement support modules use beyond the
/// Contract Runtime the generated crate already depends on. Same exact
/// versions as this repository's own manifest, so `--offline` resolves them
/// from the lockfile copied alongside.
const AGREEMENT_DEV_DEPENDENCIES: &str = "serde_json = \"=1.0.151\"\nsha2 = \"=0.10.9\"\n";

/// Writes `artifacts` (path, contents) as a crate at `<target tmp>/<label>`,
/// adds `extra_files` beside them, registers `cases` (a path under this
/// repository's root) as the crate's one `agreement` integration test, runs it
/// offline, and asserts that it succeeded and that every `#[test]` in `cases`
/// ran and passed.
///
/// The scratch crates share one target directory under this build's own
/// `target/`, so the Contract Runtime is compiled once, not per run.
pub(super) fn run_agreement_cases<'a>(
    label: &str,
    artifacts: impl IntoIterator<Item = (&'a str, &'a str)>,
    extra_files: &[(&'a str, &'a str)],
    cases: &str,
) {
    let scratch = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let root = scratch.join(label);
    let _ = fs::remove_dir_all(&root);
    let mut wrote_manifest = false;
    for (path, contents) in artifacts.into_iter().chain(extra_files.iter().copied()) {
        wrote_manifest |= path == "Cargo.toml";
        let destination = root.join(path);
        fs::create_dir_all(destination.parent().expect("artifact paths are relative"))
            .expect("create scratch crate directory");
        fs::write(&destination, contents).expect("write scratch crate file");
    }
    assert!(wrote_manifest, "{label}: generation emitted no Cargo.toml");

    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cases_path = repository.join(cases);
    let manifest_path = root.join("Cargo.toml");
    let mut manifest = fs::read_to_string(&manifest_path).expect("read generated manifest");
    write!(
        manifest,
        "\n[[test]]\nname = \"agreement\"\npath = {:?}\n\n[dev-dependencies]\n{AGREEMENT_DEV_DEPENDENCIES}",
        cases_path.to_str().expect("UTF-8 repository path"),
    )
    .expect("write to String");
    fs::write(&manifest_path, manifest).expect("extend generated manifest");
    fs::copy(repository.join("Cargo.lock"), root.join("Cargo.lock"))
        .expect("seed the scratch crate's lockfile");

    let output = Command::new(env!("CARGO"))
        .args(["test", "--offline", "--test", "agreement"])
        .env("CARGO_TARGET_DIR", scratch.join("agreement-target"))
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(&root)
        .output()
        .expect("spawn cargo");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "{label}: generated crate did not pass its agreement cases:\n{stdout}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let declared = fs::read_to_string(&cases_path)
        .expect("read agreement cases")
        .matches("#[test]")
        .count();
    let passed = stdout
        .lines()
        .find_map(|line| line.strip_prefix("test result: ok. "))
        .and_then(|rest| rest.split(' ').next())
        .and_then(|count| count.parse::<usize>().ok())
        .unwrap_or_else(|| panic!("{label}: no test summary in cargo output:\n{stdout}"));
    assert!(declared > 0, "{label}: {cases} declares no test");
    assert_eq!(
        passed, declared,
        "{label}: every agreement case must run and pass:\n{stdout}"
    );
}

/// Trace: FR-014-AC-6, FR-014-AC-9, TC-024. The crate the generator emits now
/// for the corpus request compiles against the pinned runtime, and every
/// agreement case in `tests/exact_scalar_support/agreement_cases.rs` passes
/// against it: each generated oracle equals the direct runtime call in outcome,
/// admitted charges, consumed counters and single-charge denials, and a width
/// mismatch stops before any charge.
#[test]
fn tc_024_generated_scalar_crate_agrees_with_direct_runtime() {
    let oracles = super::exact_scalar_generation::corpus_oracles();
    run_agreement_cases(
        "exact-scalar-agreement",
        oracles
            .artifacts
            .iter()
            .map(|artifact| (artifact.path.as_str(), artifact.contents.as_str())),
        &[],
        "tests/exact_scalar_support/agreement_cases.rs",
    );
}

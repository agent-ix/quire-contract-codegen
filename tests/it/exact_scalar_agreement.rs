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

/// The `names.rs` a scratch crate's agreement cases `include!` into their `generated` module:
/// `pub use` of each generated item of crate `crate_name` under the name the cases call it by, then
/// `extra` verbatim. Generated names are readable plus a positional counter, so the cases never
/// spell one; the harness maps them from the claim map here.
pub(super) fn names_file(crate_name: &str, aliases: &[(String, String)], extra: &str) -> String {
    let crate_ident = crate_name.replace('-', "_");
    let mut names = String::new();
    for (generated, alias) in aliases {
        writeln!(names, "pub use {crate_ident}::{generated} as {alias};").expect("write to String");
    }
    names.push_str(extra);
    names
}

/// Each corpus node the scalar agreement cases execute, by fixture code, and the name they call
/// its generated oracle by.
const SCALAR_ORACLE_NAMES: &[(u32, &str)] = &[
    (1001, "integer_add"),
    (1002, "integer_negate"),
    (1003, "integer_subtract"),
    (1004, "integer_multiply"),
    (1011, "divide_truncating"),
    (1012, "divide_floor"),
    (1013, "divide_euclidean"),
    (1014, "divide_bounded"),
    (1021, "modulo_bounded"),
    (1031, "rational_add"),
    (1032, "rational_divide"),
    (1033, "integer_divide"),
    (1034, "rational_subtract"),
    (1035, "rational_multiply"),
    (1036, "rational_negate"),
    (1041, "integer_less"),
    (1042, "decimal_at_most"),
    (1043, "rational_greater"),
    (1044, "integer_at_most"),
    (1045, "integer_at_least"),
    (1051, "decimal_add"),
    (1052, "decimal_divide"),
    (1053, "decimal_round"),
    (1054, "decimal_subtract"),
    (1055, "decimal_multiply"),
    (1056, "decimal_negate"),
    (1061, "binary32_add"),
    (1062, "binary64_divide"),
    (1063, "binary64_total_order"),
    (1064, "narrow_to_binary32"),
    (1065, "binary32_subtract"),
    (1066, "binary64_multiply"),
    (1067, "binary32_numeric_equal"),
    (1068, "binary64_bit_identical"),
    (1072, "text_less"),
    (1073, "enum_less"),
    (1081, "quantity_add"),
    (1082, "quantity_multiply"),
    (1083, "quantity_power"),
    (1084, "quantity_less"),
    (1086, "convert_decimal"),
    (1087, "convert_integer"),
    (1088, "quantity_subtract"),
    (1089, "quantity_divide"),
    (1111, "text_equal"),
    (1112, "text_not_equal"),
    (1113, "text_at_most"),
    (1114, "text_greater"),
    (1115, "text_at_least"),
    (1121, "enum_equal"),
    (1122, "enum_not_equal"),
    (1123, "enum_at_most"),
    (1124, "enum_greater"),
    (1125, "enum_at_least"),
    (1131, "quantity_equal"),
    (1132, "quantity_not_equal"),
    (1133, "quantity_at_most"),
    (1134, "quantity_greater"),
    (1135, "quantity_at_least"),
];

/// Trace: FR-014-AC-6, FR-014-AC-9, TC-024. The crate the generator emits now
/// for the corpus request compiles against the runtime, and every
/// agreement case in `tests/exact_scalar_support/agreement_cases.rs` passes
/// against it: each generated oracle equals the direct runtime call in outcome,
/// admitted charges, consumed counters and single-charge denials, and a width
/// mismatch stops before any charge.
#[test]
fn tc_024_generated_scalar_crate_agrees_with_direct_runtime() {
    let oracles = super::exact_scalar_generation::corpus_oracles();
    let aliases = SCALAR_ORACLE_NAMES
        .iter()
        .map(|(code, alias)| {
            (
                super::exact_scalar_generation::symbol(&oracles, *code),
                (*alias).to_owned(),
            )
        })
        .collect::<Vec<_>>();
    let names = names_file(
        quire_contract_codegen::EXACT_SCALAR_CRATE_NAME,
        &aliases,
        "",
    );
    run_agreement_cases(
        "exact-scalar-agreement",
        oracles
            .artifacts
            .iter()
            .map(|artifact| (artifact.path.as_str(), artifact.contents.as_str())),
        &[("names.rs", names.as_str())],
        "tests/exact_scalar_support/agreement_cases.rs",
    );
}

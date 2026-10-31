//! FR-014-AC-6: generated exact scalar oracles and direct Contract Runtime
//! execution agree on the QSpec TC-185, TC-186, TC-187, TC-192 and TC-193
//! vectors, in outcome, every admitted charge, every consumed counter, and
//! the outcome of denying each admitted charge.
//!
//! Nothing generated is committed. This module generates bounded corpus cohorts,
//! compiles their oracle functions in one scratch crate whose integration test is
//! `tests/exact_scalar_support/agreement_cases.rs`, and runs that test. The
//! scratch-crate runner, [`run_agreement_cases`], also serves the function
//! application agreement module. Composite equality uses the runner variant
//! that assigns distinct test-only package names before sharing a target dir.

use ix_trace_rs::trace;
use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// Development dependencies the agreement support modules use beyond the
/// Contract Runtime the generated crate already depends on. Matching
/// constraints from this repository's manifest use its seeded dependency
/// resolution before the locked native test.
const AGREEMENT_DEV_DEPENDENCIES: &str = concat!(
    "qsl-replay = { version = \"=0.1.0\", git = \"https://github.com/agent-ix/quire-spec-language\", branch = \"main\" }\n",
    "serde_json = \"=1.0.151\"\nsha2 = \"=0.10.9\"\n",
    "ix-trace-rs = { version = \"0.1\", git = \"https://github.com/agent-ix/ix-trace-rs\", branch = \"main\" }\n",
    "quire-exact = { git = \"https://github.com/agent-ix/quire-exact\", branch = \"main\", features = [\"test-support\"] }\n",
);

/// Writes `artifacts` (path, contents) as a crate at `<target tmp>/<label>`,
/// adds `extra_files` beside them, registers `cases` (a path under this
/// repository's root) as the crate's one `agreement` integration test, runs it
/// offline, and asserts that it succeeded and that every `#[test]` in `cases`
/// ran and passed.
///
/// By default each scratch crate has its own target directory: generated corpora may share a
/// package identity, which makes a common target unsafe. The composite corpus has its own
/// identity-aware runner below.
pub(super) fn run_agreement_cases<'a>(
    label: &str,
    artifacts: impl IntoIterator<Item = (&'a str, &'a str)>,
    extra_files: &[(&'a str, &'a str)],
    cases: &str,
) {
    run_agreement_cases_with_expectation(label, artifacts, extra_files, cases, None, None);
}

/// Composite equality's fixed corpus uses `label` as each scratch crate's test-only package
/// identity. The labels must be distinct valid Cargo package names; each generated manifest
/// retains its original identity in `artifacts`, while these test copies reuse one target dir.
pub(super) fn run_composite_agreement_cases<'a>(
    label: &str,
    artifacts: impl IntoIterator<Item = (&'a str, &'a str)>,
    extra_files: &[(&'a str, &'a str)],
    cases: &str,
) {
    run_agreement_cases_with_expectation(label, artifacts, extra_files, cases, None, Some(label));
}

/// Require a named native assertion failure, never a setup or compiler failure.
pub(super) fn run_agreement_mutant<'a>(
    label: &str,
    artifacts: impl IntoIterator<Item = (&'a str, &'a str)>,
    extra_files: &[(&'a str, &'a str)],
    cases: &str,
    failed_test: &str,
) {
    run_agreement_cases_with_expectation(
        label,
        artifacts,
        extra_files,
        cases,
        Some(failed_test),
        None,
    );
}

fn run_agreement_cases_with_expectation<'a>(
    label: &str,
    artifacts: impl IntoIterator<Item = (&'a str, &'a str)>,
    extra_files: &[(&'a str, &'a str)],
    cases: &str,
    failed_test: Option<&str>,
    composite_package: Option<&str>,
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
    let target_dir = if let Some(package_name) = composite_package {
        manifest = composite_manifest_for_shared_target(&manifest, package_name);
        scratch.join("composite-equality-agreement-target")
    } else {
        // Keep each generated crate's own target across repeated runs. The crate
        // directory is cleared to replace its artifacts, but rebuilding every
        // dependency from scratch masks the generated agreement result.
        scratch.join(format!("{label}-target"))
    };
    write!(
        manifest,
        "\n[[test]]\nname = \"agreement\"\npath = {:?}\n\n[dev-dependencies]\n{AGREEMENT_DEV_DEPENDENCIES}",
        cases_path.to_str().expect("UTF-8 repository path"),
    )
    .expect("write to String");
    fs::write(&manifest_path, manifest).expect("extend generated manifest");
    super::scratch_crate::resolve_seeded_lock(&root);

    let output = Command::new(env!("CARGO"))
        .args(["test", "--locked", "--offline", "--test", "agreement"])
        .env("CARGO_TARGET_DIR", target_dir)
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(&root)
        .output()
        .expect("spawn cargo");
    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Some(failed_test) = failed_test {
        assert_eq!(
            output.status.code(),
            Some(101),
            "{label}: expected Cargo native test failure"
        );
        assert!(
            stdout
                .lines()
                .any(|line| line.starts_with("running ") && line.ends_with(" tests")),
            "{label}: no native test dispatch: {stdout}\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            stdout
                .lines()
                .any(|line| line == format!("test {failed_test} ... FAILED")),
            "{label}: required assertion did not fail: {stdout}"
        );
        assert!(
            stdout.contains("test result: FAILED."),
            "{label}: missing native failure summary: {stdout}"
        );
        return;
    }
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

fn composite_manifest_for_shared_target(manifest: &str, package_name: &str) -> String {
    let original = format!(
        "[package]\nname = \"{}\"\n",
        quire_contract_codegen::COMPOSITE_EQUALITY_CRATE_NAME
    );
    assert_eq!(
        manifest.matches(&original).count(),
        1,
        "composite corpus must retain its generated package manifest"
    );
    assert_eq!(
        manifest.matches("\nname = ").count(),
        1,
        "composite corpus must not declare an explicit library name"
    );
    manifest.replacen(
        &original,
        &format!("[package]\nname = \"{package_name}\"\n"),
        1,
    )
}

/// A common target must never cause Cargo to run a sibling crate's compiled library.
/// Provenance: IR-1116.
#[test]
fn composite_shared_target_keeps_generated_libraries_distinct() {
    let scratch = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let root = scratch.join("composite-shared-target-isolation");
    let _ = fs::remove_dir_all(&root);
    let target = root.join("target");
    let original = format!(
        "[package]\nname = \"{}\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[lib]\npath = \"src/lib.rs\"\n\n[workspace]\n",
        quire_contract_codegen::COMPOSITE_EQUALITY_CRATE_NAME
    );

    for (case, value) in [("alpha", 11), ("beta", 29), ("alpha", 11)] {
        let package_name = format!(
            "{}-{case}",
            quire_contract_codegen::COMPOSITE_EQUALITY_CRATE_NAME
        );
        let crate_name = package_name.replace('-', "_");
        let directory = root.join(case);
        fs::create_dir_all(directory.join("src")).expect("create isolated scratch library");
        fs::create_dir_all(directory.join("tests")).expect("create isolated scratch tests");
        let manifest = composite_manifest_for_shared_target(&original, &package_name);
        fs::write(directory.join("Cargo.toml"), manifest).expect("write isolated manifest");
        fs::write(
            directory.join("src/lib.rs"),
            format!("pub fn source_value() -> u32 {{ {value} }}\n"),
        )
        .expect("write isolated library");
        fs::write(
            directory.join("tests/agreement.rs"),
            format!(
                "#[test] fn source_is_this_case() {{ assert_eq!({crate_name}::source_value(), {value}); }}\n"
            ),
        )
        .expect("write isolated test");
        let output = Command::new(env!("CARGO"))
            .args(["test", "--offline", "--test", "agreement"])
            .env("CARGO_TARGET_DIR", &target)
            .env_remove("RUSTFLAGS")
            .env_remove("CARGO_ENCODED_RUSTFLAGS")
            .current_dir(&directory)
            .output()
            .expect("run isolated test");
        assert!(
            output.status.success(),
            "{case} linked a sibling artifact: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            String::from_utf8_lossy(&output.stdout).contains("test result: ok. 1 passed; 0 failed"),
            "{case} did not run its isolation assertion: {}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
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
    (1140, "quantity_convert"),
    (1084, "quantity_less"),
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

/// The bounded admitted cohorts emit every corpus oracle, and the unchanged
/// agreement cases execute those generated functions together against direct
/// runtime calls. The scratch crate shares one stop type across the cohorts;
/// only the duplicate generated stop declarations are replaced, while every
/// generated oracle function and its helper remains as emitted.
#[trace("FR-014-AC-6", "FR-014-AC-9", "TC-024")]
#[test]
fn tc_024_generated_scalar_crate_agrees_with_direct_runtime() {
    let cohorts = super::exact_scalar_generation::corpus_cohort_oracles();
    let mut artifacts = Vec::<(String, String)>::new();
    let mut exports = BTreeMap::new();
    let mut stop_variants = BTreeMap::new();
    let mut modules = String::new();
    for (index, oracles) in cohorts.iter().enumerate() {
        let manifest = oracles
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "Cargo.toml")
            .expect("bounded cohort emits a manifest");
        if index == 0 {
            artifacts.push((manifest.path.clone(), manifest.contents.clone()));
        } else {
            assert_eq!(manifest.contents, artifacts[0].1, "cohort manifest drift");
        }
        let source = oracles
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .expect("bounded cohort emits Rust source");
        let source = shared_stop_source(&source.contents, &mut stop_variants);
        let module = format!("cohort_{index}");
        writeln!(modules, "mod {module};").expect("write to String");
        artifacts.push((format!("src/{module}.rs"), source));
        for claim in &oracles.claim_map.items {
            let quire_contract_codegen::ClaimDisposition::Generated(generated) = &claim.result
            else {
                continue;
            };
            let export = format!("{module}_{}", generated.symbol);
            assert!(
                exports
                    .insert(claim.node_id.clone(), export.clone())
                    .is_none(),
                "one requested scalar node must have one generated oracle"
            );
            writeln!(
                modules,
                "pub use {module}::{} as {export};",
                generated.symbol
            )
            .expect("write to String");
        }
    }
    assert!(!cohorts.is_empty(), "bounded scalar corpus has no cohorts");
    let mut root = String::from(
        "use quire_exact as rt;\n\
         /// Why a generated oracle stopped before an operator outcome.\n\
         #[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
         pub enum OracleStop {\n",
    );
    for variant in stop_variants.values() {
        writeln!(root, "    {variant}").expect("write to String");
    }
    root.push_str("}\n");
    root.push_str(&modules);
    artifacts.push(("src/lib.rs".to_owned(), root));
    let aliases = SCALAR_ORACLE_NAMES
        .iter()
        .map(|(code, alias)| {
            let node = super::exact_scalar_generation::package::code_id(*code);
            (
                exports
                    .get(&node)
                    .unwrap_or_else(|| panic!("scalar corpus code {code} has no oracle"))
                    .clone(),
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
        artifacts
            .iter()
            .map(|(path, contents)| (path.as_str(), contents.as_str())),
        &[("names.rs", names.as_str())],
        "tests/exact_scalar_support/agreement_cases.rs",
    );
}

/// Keep the generator's function bytes and local helpers intact while making
/// all cohort modules use the same stop type for one agreement test crate.
fn shared_stop_source(source: &str, variants: &mut BTreeMap<String, String>) -> String {
    const STOP_DOC: &str = "/// Why a generated oracle stopped before an operator outcome.\n";
    let start = source
        .find(STOP_DOC)
        .expect("generated scalar source declares OracleStop");
    let enum_start = source[start..]
        .find("pub enum OracleStop {\n")
        .map(|offset| start + offset)
        .expect("generated scalar source declares OracleStop enum");
    let end = source[enum_start..]
        .find("\n}\n")
        .map(|offset| enum_start + offset + "\n}\n".len())
        .expect("generated OracleStop enum closes");
    for line in source[enum_start..end].lines().skip(1) {
        let line = line.trim();
        if line.is_empty() || line.starts_with("///") || line == "}" {
            continue;
        }
        assert!(line.ends_with(','), "generated OracleStop variant: {line}");
        let name = line.split(['(', ',']).next().expect("variant name");
        if let Some(previous) = variants.insert(name.to_owned(), line.to_owned()) {
            assert_eq!(
                previous, line,
                "generated OracleStop variant {name} drifted"
            );
        }
    }
    let mut shared = String::with_capacity(source.len());
    shared.push_str(&source[..start]);
    shared.push_str("pub use crate::OracleStop;\n");
    shared.push_str(&source[end..]);
    shared
}

/// Each Boolean connective and scalar equality generated from admitted V2 nodes agrees with
/// direct Contract Runtime calls in outcomes, charges, counters and each single-charge denial.
#[trace("FR-014-AC-37", "TC-024")]
#[test]
fn tc_024_v2_boolean_and_equality_oracles_agree_with_direct_runtime() {
    let oracles = super::exact_scalar_generation::boolean_oracles();
    let aliases = [
        (
            super::exact_scalar_generation::package::DERIVE_BOOLEAN_AND,
            "boolean_and",
        ),
        (
            super::exact_scalar_generation::package::DERIVE_BOOLEAN_OR,
            "boolean_or",
        ),
        (
            super::exact_scalar_generation::package::DERIVE_BOOLEAN_NOT,
            "boolean_not",
        ),
        (
            super::exact_scalar_generation::package::DERIVE_BOOLEAN_IMPLIES,
            "boolean_implies",
        ),
        (
            super::exact_scalar_generation::package::DERIVE_BOOLEAN_EQ,
            "boolean_eq",
        ),
        (
            super::exact_scalar_generation::package::DERIVE_BOOLEAN_NE,
            "boolean_ne",
        ),
        (
            super::exact_scalar_generation::package::DERIVE_BOOLEAN_SHORT_CIRCUIT,
            "boolean_short_circuit",
        ),
        (
            super::exact_scalar_generation::package::DERIVE_BOOLEAN_LAZY_AND,
            "boolean_lazy_and",
        ),
        (
            super::exact_scalar_generation::package::DERIVE_BOOLEAN_LAZY_OR,
            "boolean_lazy_or",
        ),
        (
            super::exact_scalar_generation::package::DERIVE_INTEGER_EQ,
            "integer_eq",
        ),
        (
            super::exact_scalar_generation::package::DERIVE_INTEGER_NE,
            "integer_ne",
        ),
        (1041, "integer_lt"),
        (1044, "integer_le"),
        (
            super::exact_scalar_generation::package::DERIVE_INTEGER_GT,
            "integer_gt",
        ),
        (1045, "integer_ge"),
    ]
    .map(|(code, alias)| {
        (
            super::exact_scalar_generation::symbol(&oracles, code),
            alias.to_owned(),
        )
    });
    let names = names_file(
        quire_contract_codegen::EXACT_SCALAR_CRATE_NAME,
        &aliases,
        "",
    );
    run_agreement_cases(
        "v2-boolean-scalar-agreement",
        oracles
            .artifacts
            .iter()
            .map(|artifact| (artifact.path.as_str(), artifact.contents.as_str())),
        &[("names.rs", names.as_str())],
        "tests/exact_scalar_support/boolean_agreement_cases.rs",
    );
}

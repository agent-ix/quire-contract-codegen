//! FR-015-AC-132 and FR-015-AC-151: the controls of the V2 clause fixture
//! `result = (amount < 1000 implies amount + 1 <= 1000)` over `amount: Int[0, 1000]`.
//!
//! The package is compiled from the QSL source of `Twin::with_bounded_amount_implication`; the
//! sealed harness is what `negotiate_kani_native_obligations` generates for it. The default lane
//! runs the exhaustive host enumeration of all 1001 values (no Kani launch), the sealed
//! `amount_current = 999` playback and the enumeration-record checks. The `kani` lane
//! (`#[ignore]`, run one at a time by `make kani`, whose `kani_obligations` filter this module's
//! name matches) runs the installed controls through CG's runner, the two-phase run of
//! FR-017-AC-63.
//!
//! The owner mutants are made in isolated copies of the locked `quire-exact` and
//! `quire-contract-runtime` checkouts under the scratch directory; the shared checkouts are never
//! edited.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};

use ix_trace_rs::trace;
use quire_contract_codegen::{
    execute_kani_concrete_playback, execute_kani_obligation, negotiate_kani_native_obligations,
    KaniConcretePlayback, KaniConcretePlaybackOutcome, KaniExecutionRequest,
    KaniInconclusiveReason, KaniInstallation, KaniObligationOutcome, KaniObligationRequest,
    KaniRunOutcome, KaniV2ClauseObligationHarness, MemoryObservation, MemoryUnit, ObligationItem,
    PeakMemorySource, PeakReading, ProofCeilings,
};

use super::kani_obligations_state_frame::native_twin::Twin;
use crate::scratch_crate::{
    agreement_target, lock_agreement_build, resolve_seeded_lock, runtime_dependency,
    write_manifest, TemporaryDirectory,
};

const SUBJECT: &str = "crate::subject::check";
const DOMAIN_SIZE: usize = 1001;
const HEALTHY_SUBJECT: &str = "quire_contract_runtime::operators::implies_short_circuit(amount < 1000, || amount + 1 <= 1000)";
const ARITHMETIC_MESSAGE: &str = "arithmetic subterm agrees with independent i128 expectation";
const EXACT_SITE: &str = "left.0.checked_add(right.0)";
const HELPER_SITE: &str =
    "    if antecedent {\n        consequent()\n    } else {\n        R::from(true)\n    }";

/// The ceilings of the installed runs: the 24 GiB cgroup of the measurements and 30 minutes.
const INSTALLED_CEILINGS: ProofCeilings = ProofCeilings {
    memory_bytes: std::num::NonZeroU64::new(24 * 1024 * 1024 * 1024).unwrap(),
    wall_clock: Duration::from_secs(30 * 60),
};

/// The host stand-in for the `kani` crate. `any` yields the bytes `concrete_playback_run`
/// queued, `assume(false)` panics so that it fails its value, and `cover!` does nothing.
const HOST_KANI: &str = r#"
mod kani {
    use std::cell::RefCell;
    thread_local! { static QUEUE: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) }; }
    pub trait Arbitrary: Sized { fn from_bytes(bytes: &[u8]) -> Self; }
    impl Arbitrary for i64 {
        fn from_bytes(bytes: &[u8]) -> Self { i64::from_le_bytes(bytes.try_into().expect("eight bytes")) }
    }
    pub fn any<T: Arbitrary>() -> T {
        QUEUE.with(|queue| T::from_bytes(&queue.borrow_mut().remove(0)))
    }
    pub fn assume(condition: bool) { assert!(condition, "kani::assume failed"); }
    pub fn concrete_playback_run(values: Vec<Vec<u8>>, proof: fn()) {
        QUEUE.with(|queue| *queue.borrow_mut() = values);
        proof();
    }
    macro_rules! cover { ($($tokens:tt)*) => {}; }
    pub(crate) use cover;
}
"#;

/// The appended enumeration: every value once, ascending, whatever an earlier value did. Each
/// value runs the unchanged harness body, then the compiled subject against an independent
/// `i128` expectation that calls neither the oracle nor the helper.
const ENUMERATION_TEST: &str = r#"
    #[test]
    fn enumerate_clause_domain() {
        std::panic::set_hook(Box::new(|_| {}));
        for amount_current in 0..=1000_i64 {
            let harness = std::panic::catch_unwind(|| {
                crate::kani::concrete_playback_run(vec![amount_current.to_le_bytes().to_vec()], prove_clause)
            });
            let subject = std::panic::catch_unwind(|| crate::subject::check(amount_current));
            let expected = !(i128::from(amount_current) < 1000_i128) || (i128::from(amount_current) + 1_i128 <= 1000_i128);
            let agrees = matches!(subject, Ok(result) if result == expected);
            println!("\nENUM {amount_current} harness={} subject={agrees}", harness.is_ok());
        }
    }
"#;

/// The sealed-artifact playback at `amount_current = 999`: the unchanged subject returns
/// normally, then the sealed harness body runs on that value and must fail the generated
/// arithmetic assertion by its message.
fn playback_test() -> String {
    playback_test_with(true)
}

/// The playback test, with or without the preamble that first runs the compiled subject at 999.
fn playback_test_with(subject_preamble: bool) -> String {
    let preamble = if subject_preamble {
        "        assert!(crate::subject::check(999), \"the unchanged subject returns normally at 999\");\n"
    } else {
        ""
    };
    format!(
        "    #[test]\n    #[should_panic(expected = \"{ARITHMETIC_MESSAGE}\")]\n    fn playback_999() {{\n{preamble}        crate::kani::concrete_playback_run(vec![999_i64.to_le_bytes().to_vec()], prove_clause);\n    }}\n"
    )
}

fn fixture() -> KaniV2ClauseObligationHarness {
    fixture_of(Twin::with_bounded_amount_implication(), "BalanceNeverDrops")
}

fn fixture_of(twin: Twin, clause_name: &str) -> KaniV2ClauseObligationHarness {
    let (package, clause) = twin.emitted_package(clause_name);
    let item = ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        claim: &crate::common::v2_claim::claim_for(&package, &clause),
        dependencies: None,
    };
    let outcome = negotiate_kani_native_obligations(&KaniObligationRequest {
        ceilings: INSTALLED_CEILINGS,
        items: &[item],
        subject_path: SUBJECT,
        unwind: 3,
    })
    .expect("the clause negotiates");
    let KaniObligationOutcome::Emitted {
        mut v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the clause fixture emits a harness");
    };
    assert_eq!(v2_clause_harnesses.len(), 1, "one contract harness");
    v2_clause_harnesses.remove(0)
}

pub(crate) fn subject_module(body: &str) -> String {
    format!("\npub mod subject {{\n    pub fn check(amount: i64) -> bool {{ {body} }}\n}}\n")
}

/// `sealed` with `appended` inside the generated module, immediately before its closing brace.
fn append_inside_module(sealed: &str, appended: &str) -> String {
    let close = sealed.rfind('}').expect("the generated module closes");
    format!("{}{appended}{}", &sealed[..close], &sealed[close..])
}

/// `with_appended` less exactly the appended bytes.
fn remove_appended(with_appended: &str, appended: &str) -> String {
    assert_eq!(
        with_appended.matches(appended).count(),
        1,
        "the appended bytes occur once"
    );
    with_appended.replacen(appended, "", 1)
}

/// The sealed text as a host crate source: only the two attribute lines are replaced; the draw,
/// the assumption, the cover and every assertion are the generated ones.
fn host_source(sealed: &str) -> String {
    assert_eq!(sealed.matches("#[cfg(kani)]\n").count(), 1);
    assert_eq!(sealed.matches("#[kani::proof]\n").count(), 1);
    sealed
        .replacen("#[cfg(kani)]\n", "", 1)
        .replacen("#[kani::proof]\n", "", 1)
}

/// A mutation of an owner crate's source file, made in an isolated copy.
pub(crate) struct OwnerMutation {
    package: &'static str,
    file: &'static str,
    original: &'static str,
    mutated: &'static str,
}

const PLUS_ONE: OwnerMutation = OwnerMutation {
    package: "quire-exact",
    file: "src/native_integer.rs",
    original: EXACT_SITE,
    mutated: "left.0.checked_add(right.0)?.checked_add(1)",
};
const LEFT_ONLY: OwnerMutation = OwnerMutation {
    package: "quire-exact",
    file: "src/native_integer.rs",
    original: EXACT_SITE,
    mutated: "Some(left.0)",
};
const HELPER_FALSE: OwnerMutation = OwnerMutation {
    package: "quire-contract-runtime",
    file: "src/operators.rs",
    original: HELPER_SITE,
    mutated: "    let _ = (antecedent, consequent);\n    R::from(false)",
};

fn locked_package_directory(package: &str) -> PathBuf {
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--offline", "--locked", "--format-version", "1"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo metadata");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let manifest = metadata["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"] == package)
        .unwrap_or_else(|| panic!("CG's lock names {package}"))["manifest_path"]
        .as_str()
        .unwrap()
        .to_owned();
    Path::new(&manifest).parent().unwrap().to_path_buf()
}

fn copy_directory(from: &Path, to: &Path) {
    let status = Command::new("cp")
        .arg("-r")
        .arg(from)
        .arg(to)
        .status()
        .expect("copy the locked owner checkout");
    assert!(status.success());
    let status = Command::new("chmod")
        .args(["-R", "u+w"])
        .arg(to)
        .status()
        .expect("make the isolated copy writable");
    assert!(status.success());
}

/// One scratch crate: the generated source with a subject, optionally resolved against an
/// isolated, mutated owner checkout.
pub(crate) struct ScratchCrate {
    pub(crate) directory: TemporaryDirectory,
}

impl ScratchCrate {
    pub(crate) fn new(label: &str, lib: &str, owner: Option<&OwnerMutation>) -> Self {
        let directory = TemporaryDirectory::new(label);
        let mut patch = String::new();
        if let Some(mutation) = owner {
            let copy = directory.path.join("owner-mutant");
            copy_directory(&locked_package_directory(mutation.package), &copy);
            let file = copy.join(mutation.file);
            let text = fs::read_to_string(&file).expect("owner source");
            assert_eq!(
                text.matches(mutation.original).count(),
                1,
                "the mutation site occurs once in {}",
                mutation.file
            );
            fs::write(&file, text.replacen(mutation.original, mutation.mutated, 1))
                .expect("write the isolated mutant");
            patch = format!(
                "\n[patch.\"https://github.com/agent-ix/{0}\"]\n{0} = {{ path = \"owner-mutant\" }}\n",
                mutation.package
            );
        }
        let manifest = format!(
            "[package]\nname = \"ir1509-clause-controls\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\nquire-exact = {{ git = \"https://github.com/agent-ix/quire-exact\", branch = \"main\" }}\nquire-semantic-value = {{ git = \"https://github.com/agent-ix/quire-semantic-value\", branch = \"main\" }}\n{patch}\n[workspace]\n",
            runtime_dependency(&["exact"])
        );
        write_manifest(&directory.path, &manifest);
        fs::write(
            directory.path.join("build.rs"),
            "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
        )
        .unwrap();
        fs::write(directory.path.join("src/lib.rs"), lib).unwrap();
        resolve_seeded_lock(&directory.path);
        if let Some(mutation) = owner {
            let resolved = locked_resolution(&directory.path, mutation.package);
            assert_eq!(
                resolved,
                directory.path.join("owner-mutant/Cargo.toml"),
                "{} must resolve to the isolated checkout",
                mutation.package
            );
        }
        Self { directory }
    }

    /// `cargo test` of this crate on the host, returning its stdout.
    fn host_test(&self, filter: &str) -> (bool, String) {
        let build = lock_agreement_build();
        let output = Command::new(env!("CARGO"))
            .args(["test", "--locked", "--offline", "-j2", "--lib", filter])
            .args(["--", "--nocapture", "--test-threads=1"])
            .env("CARGO_TARGET_DIR", agreement_target())
            .env_remove("RUSTFLAGS")
            .env_remove("CARGO_ENCODED_RUSTFLAGS")
            .current_dir(&self.directory.path)
            .output()
            .expect("run the host test");
        drop(build);
        (
            output.status.success(),
            format!(
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ),
        )
    }
}

fn locked_resolution(directory: &Path, package: &str) -> PathBuf {
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--offline", "--locked", "--format-version", "1"])
        .current_dir(directory)
        .output()
        .expect("scratch metadata");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let matching: Vec<_> = metadata["packages"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| entry["name"] == package)
        .collect();
    assert_eq!(matching.len(), 1, "one {package}");
    PathBuf::from(matching[0]["manifest_path"].as_str().unwrap())
}

/// The visited values and the per-value outcomes the enumeration printed.
#[derive(Debug, PartialEq)]
struct Visit {
    value: i64,
    harness_passed: bool,
    subject_agrees: bool,
}

fn parse_visits(output: &str) -> Vec<Visit> {
    output
        .lines()
        .filter_map(|line| line.strip_prefix("ENUM "))
        .map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let [value, harness, subject] = fields[..] else {
                panic!("malformed enumeration line `{line}`");
            };
            Visit {
                value: value.parse().expect("visited value"),
                harness_passed: harness.strip_prefix("harness=") == Some("true"),
                subject_agrees: subject.strip_prefix("subject=") == Some("true"),
            }
        })
        .collect()
}

/// The enumeration record of FR-017-AC-64, as printed JSON.
#[derive(Debug, PartialEq)]
struct EnumerationRecord {
    sequence_exact: bool,
    checked: usize,
    passed: usize,
    failing_inputs: Vec<i64>,
    clause: String,
}

fn enumeration_record(visits: &[Visit], clause: &str) -> EnumerationRecord {
    let failing_inputs: Vec<i64> = visits
        .iter()
        .filter(|visit| !(visit.harness_passed && visit.subject_agrees))
        .map(|visit| visit.value)
        .collect();
    let mut failing_inputs_sorted = failing_inputs.clone();
    failing_inputs_sorted.sort_unstable();
    failing_inputs_sorted.dedup();
    EnumerationRecord {
        sequence_exact: visits.iter().map(|visit| visit.value).eq(0..=1000_i64),
        checked: visits.len(),
        passed: visits.len() - failing_inputs.len(),
        failing_inputs: failing_inputs_sorted,
        clause: clause.to_owned(),
    }
}

const BANNED_TOKENS: [&str; 5] = ["Proved", "proved", "Verified", "verified", "Kani proof"];

/// The printed record: the fields and wording of FR-017-AC-64.
fn render_record(record: &EnumerationRecord) -> String {
    let rendered = serde_json::json!({
        "evidence_kind": "exhaustive_concrete_enumeration",
        "symbolic_proof": false,
        "domain": "Int[0, 1000]",
        "domain_size": DOMAIN_SIZE,
        "sequence_exact": record.sequence_exact,
        "checked": record.checked,
        "passed": record.passed,
        "failing_inputs": record.failing_inputs,
        "clause": record.clause,
        "wording": "enumeration evidence",
    });
    serde_json::to_string(&rendered).unwrap()
}

fn assert_record_wording(printed: &str) {
    for token in BANNED_TOKENS {
        assert!(
            !printed.contains(token),
            "the record must not contain `{token}`: {printed}"
        );
    }
    let value: serde_json::Value = serde_json::from_str(printed).expect("record JSON");
    assert_eq!(value["evidence_kind"], "exhaustive_concrete_enumeration");
    assert_eq!(value["symbolic_proof"], false);
    assert_eq!(value["domain"], "Int[0, 1000]");
    assert_eq!(value["domain_size"], 1001);
    assert!(value["wording"]
        .as_str()
        .unwrap()
        .contains("enumeration evidence"));
}

/// The enumeration of one scratch crate: builds it, runs the 1001 values, prints and returns its
/// record.
fn enumerate(
    label: &str,
    harness: &KaniV2ClauseObligationHarness,
    subject_body: &str,
    owner: Option<&OwnerMutation>,
) -> EnumerationRecord {
    let sealed = &harness.rust.contents;
    let with_test = append_inside_module(sealed, ENUMERATION_TEST);
    assert_eq!(remove_appended(&with_test, ENUMERATION_TEST), *sealed);
    let lib = format!(
        "{}{HOST_KANI}{}",
        host_source(&with_test),
        subject_module(subject_body)
    );
    let scratch = ScratchCrate::new(label, &lib, owner);
    let (success, output) = scratch.host_test("enumerate_clause_domain");
    assert!(success, "the enumeration test runs to completion: {output}");
    let visits = parse_visits(&output);
    let record = enumeration_record(&visits, &format!("{:?}", harness.identity.clause));
    let printed = render_record(&record);
    println!("ENUMERATION RECORD {label}: {printed}");
    assert_record_wording(&printed);
    record
}

#[trace("FR-015-AC-132", "FR-017-AC-64", "TC-025")]
#[test]
fn the_healthy_clause_passes_all_1001_values_in_ascending_order() {
    let harness = fixture();
    let record = enumerate("ir1509-enum-healthy", &harness, HEALTHY_SUBJECT, None);
    assert!(record.sequence_exact);
    assert_eq!((record.checked, record.passed), (DOMAIN_SIZE, DOMAIN_SIZE));
    assert!(record.failing_inputs.is_empty());
}

#[trace("FR-015-AC-132", "FR-017-AC-64", "TC-025")]
#[test]
fn compiled_subject_mutants_fail_exactly_their_values() {
    let harness = fixture();
    for (label, body, expected) in [
        (
            "ir1509-enum-plus-two",
            "quire_contract_runtime::operators::implies_short_circuit(amount < 1000, || amount + 2 <= 1000)",
            vec![999],
        ),
        (
            "ir1509-enum-strict",
            "quire_contract_runtime::operators::implies_short_circuit(amount < 1000, || amount + 1 < 1000)",
            vec![999],
        ),
        (
            "ir1509-enum-antecedent",
            "quire_contract_runtime::operators::implies_short_circuit(amount <= 1000, || amount + 1 <= 1000)",
            vec![1000],
        ),
    ] {
        let record = enumerate(label, &harness, body, None);
        assert!(record.sequence_exact, "{label}");
        assert_eq!(record.checked, DOMAIN_SIZE, "{label}");
        assert_eq!(record.failing_inputs, expected, "{label}");
        assert_eq!(record.passed, DOMAIN_SIZE - expected.len(), "{label}");
    }
}

#[trace("FR-015-AC-132", "FR-017-AC-64", "TC-025")]
#[test]
fn the_helper_mutant_fails_every_value_from_zero() {
    let harness = fixture();
    let record = enumerate(
        "ir1509-enum-helper",
        &harness,
        HEALTHY_SUBJECT,
        Some(&HELPER_FALSE),
    );
    assert!(record.sequence_exact);
    assert_eq!(record.failing_inputs, (0..=1000).collect::<Vec<_>>());
    assert_eq!(record.passed, 0);
}

#[trace("FR-015-AC-132", "FR-017-AC-64", "TC-025")]
#[test]
fn the_kernel_mutants_fail_through_the_generated_probe_at_0_through_999() {
    let harness = fixture();
    for (label, mutation) in [
        ("ir1509-enum-kernel-plus-one", &PLUS_ONE),
        ("ir1509-enum-kernel-left", &LEFT_ONLY),
    ] {
        let record = enumerate(label, &harness, HEALTHY_SUBJECT, Some(mutation));
        assert!(record.sequence_exact, "{label}");
        assert_eq!(record.checked, DOMAIN_SIZE, "{label}");
        assert_eq!(
            record.failing_inputs,
            (0..=999).collect::<Vec<_>>(),
            "{label}"
        );
    }
}

fn visits_of(values: impl IntoIterator<Item = i64>) -> Vec<Visit> {
    values
        .into_iter()
        .map(|value| Visit {
            value,
            harness_passed: true,
            subject_agrees: true,
        })
        .collect()
}

#[trace("FR-015-AC-132", "FR-017-AC-64", "TC-025")]
#[test]
fn a_mutant_of_the_enumeration_fails_the_sequence_check() {
    let exact = enumeration_record(&visits_of(0..=1000), "clause");
    assert!(exact.sequence_exact);
    assert_eq!((exact.checked, exact.passed), (DOMAIN_SIZE, DOMAIN_SIZE));

    let omits_999 = enumeration_record(
        &visits_of((0..=1000).filter(|value| *value != 999)),
        "clause",
    );
    assert_eq!(omits_999.checked, 1000);
    assert!(!omits_999.sequence_exact);

    let twice_998 = enumeration_record(
        &visits_of((0..=1000).filter(|value| *value != 999).flat_map(|value| {
            if value == 998 {
                vec![998, 998]
            } else {
                vec![value]
            }
        })),
        "clause",
    );
    assert_eq!(twice_998.checked, 1001);
    assert!(!twice_998.sequence_exact);

    let out_of_order = enumeration_record(&visits_of((0..=1000).rev()), "clause");
    assert!(!out_of_order.sequence_exact);
}

#[trace("FR-015-AC-132", "FR-017-AC-64", "TC-025")]
#[test]
fn a_record_renderer_that_prints_a_banned_token_fails_the_wording_check() {
    let record = enumeration_record(&visits_of(0..=1000), "clause");
    assert_record_wording(&render_record(&record));
    for token in BANNED_TOKENS {
        // The required phrase stays, so only the banned-token check can reject the record.
        let tainted = render_record(&record).replace(
            "enumeration evidence",
            &format!("enumeration evidence, {token}"),
        );
        assert!(tainted.contains("enumeration evidence"));
        let result = std::panic::catch_unwind(|| assert_record_wording(&tainted));
        let message = *result
            .expect_err("a banned token must fail the wording check")
            .downcast::<String>()
            .expect("formatted panic message");
        assert!(
            message.contains("the record must not contain"),
            "`{token}` is rejected by the banned-token check: {message}"
        );
    }
    let symbolic =
        render_record(&record).replace("\"symbolic_proof\":false", "\"symbolic_proof\":true");
    assert!(std::panic::catch_unwind(|| assert_record_wording(&symbolic)).is_err());
}

#[trace("FR-015-AC-151", "FR-015-AC-41", "TC-025")]
#[test]
fn the_sealed_playback_at_999_fails_the_generated_arithmetic_assertion_under_the_plus_one_kernel() {
    let harness = fixture();
    let regenerated = fixture();
    assert_eq!(regenerated.rust.contents, harness.rust.contents);
    assert_eq!(regenerated.record.contents, harness.record.contents);

    let sealed = &harness.rust.contents;
    let appended = playback_test();
    let with_test = append_inside_module(sealed, &appended);
    assert_eq!(remove_appended(&with_test, &appended), *sealed);
    let lib = format!(
        "{}{HOST_KANI}{}",
        host_source(&with_test),
        subject_module(HEALTHY_SUBJECT)
    );

    // The plus-one owner mutant: the playback test passes by the arithmetic assertion's message.
    let mutant = ScratchCrate::new("ir1509-playback-plus-one", &lib, Some(&PLUS_ONE));
    let (success, output) = mutant.host_test("playback_999");
    assert!(success, "{output}");
    assert!(
        output.contains("playback_999 - should panic ... ok"),
        "{output}"
    );
    assert!(output.contains("1 passed; 0 failed"), "{output}");

    // The control: the unmutated owner does not fail the arithmetic assertion, so the same
    // expected-panic test fails.
    let healthy = ScratchCrate::new("ir1509-playback-healthy", &lib, None);
    let (success, output) = healthy.host_test("playback_999");
    assert!(
        !success,
        "the healthy owner must fail the playback control: {output}"
    );
    assert!(
        output.contains("playback_999 - should panic ... FAILED"),
        "{output}"
    );
    assert!(
        output.contains("test did not panic as expected"),
        "{output}"
    );

    // A mutant that fails some other check does not satisfy it. The compiled-subject `+ 2` mutant
    // runs the sealed harness body without the preamble, so it reaches the produced-result
    // comparison, which is not the expected arithmetic assertion.
    let appended = playback_test_with(false);
    let with_bare_test = append_inside_module(sealed, &appended);
    assert_eq!(remove_appended(&with_bare_test, &appended), *sealed);
    let other_lib = format!(
        "{}{HOST_KANI}{}",
        host_source(&with_bare_test),
        subject_module("quire_contract_runtime::operators::implies_short_circuit(amount < 1000, || amount + 2 <= 1000)")
    );
    let other = ScratchCrate::new("ir1509-playback-other-check", &other_lib, None);
    let (success, output) = other.host_test("playback_999");
    assert!(!success, "{output}");
    assert!(
        output.contains(
            "the produced V2 Boolean result agrees with the independent checked condition"
        ),
        "the +2 subject fails the produced-result comparison: {output}"
    );
    assert!(
        output.contains("playback_999 - should panic ... FAILED"),
        "{output}"
    );
    assert!(
        output.contains("panic did not contain expected string"),
        "{output}"
    );
}

// ---- kani lane ---------------------------------------------------------------------------

fn kani_crate(
    label: &str,
    owner: Option<&OwnerMutation>,
    subject_body: &str,
) -> (ScratchCrate, KaniV2ClauseObligationHarness) {
    let harness = fixture();
    let lib = format!("{}{}", harness.rust.contents, subject_module(subject_body));
    (ScratchCrate::new(label, &lib, owner), harness)
}

struct InstalledRun {
    evidence: quire_contract_codegen::KaniExecutionEvidence,
}

fn run_installed(scratch: &ScratchCrate, harness: &KaniV2ClauseObligationHarness) -> InstalledRun {
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let target = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ir1509-kani-target");
    let started = Instant::now();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: harness.into(),
        crate_directory: &scratch.directory.path,
        target_directory: &target,
        home_directory: None,
    })
    .unwrap_or_else(|refusal| panic!("the run must start: {refusal}"));
    let wall = started.elapsed();
    let receipts = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ir1509-evidence");
    fs::create_dir_all(&receipts).unwrap();
    let name = std::thread::current()
        .name()
        .unwrap_or("run")
        .replace("::", "_");
    fs::write(
        receipts.join(format!("{name}.evidence.json")),
        serde_json::to_vec_pretty(&evidence).unwrap(),
    )
    .unwrap();
    let failed_checks = evidence
        .checks
        .iter()
        .filter(|check| format!("{:?}", check.status) == "Failure")
        .map(|check| format!("{}:{:?}", check.id, check.class))
        .collect::<Vec<_>>();
    println!(
        "IR1509 {name}: outcome={} wall={:.1}s runner_peak_resident={:?} success_checks={} checks={} failed_checks={failed_checks:?} two_phase={} proof_phase_outcome={:?}",
        match &evidence.outcome {
            KaniRunOutcome::Verified => "Verified".to_owned(),
            KaniRunOutcome::Falsified { .. } => "Falsified".to_owned(),
            other => format!("{other:?}"),
        },
        wall.as_secs_f64(),
        evidence.memory.peak_resident_bytes,
        evidence.success_checks,
        evidence.checks.len(),
        evidence.proof_phase.is_some(),
        evidence.proof_phase.as_ref().map(|proof| &proof.outcome)
    );
    InstalledRun { evidence }
}

/// The `i64` values Kani's playback assigns the harness's one symbolic draw.
fn playback_values(counterexample: &str) -> Vec<i64> {
    counterexample
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("vec![") && !line.contains("concrete_vals"))
        .map(|line| {
            let bytes = line
                .trim_start_matches("vec![")
                .trim_end_matches("],")
                .split(',')
                .map(|byte| byte.trim().parse::<u8>().expect("a playback byte"))
                .collect::<Vec<_>>();
            i64::from_le_bytes(bytes.try_into().expect("eight bytes"))
        })
        .collect()
}

/// A falsified two-phase run: the verdict and the in-domain playback are the witness phase's,
/// the proof phase's evidence is nested, and one identity covers both.
fn assert_two_phase_falsified(
    run: &InstalledRun,
    harness: &KaniV2ClauseObligationHarness,
    check: &str,
) -> Vec<i64> {
    let evidence = &run.evidence;
    let KaniRunOutcome::Falsified { counterexample } = &evidence.outcome else {
        panic!("expected Falsified, got {:?}", evidence.outcome);
    };
    let proof = evidence
        .proof_phase
        .as_ref()
        .expect("the Falsified verdict comes from a witness phase with a proof phase beneath it");
    assert!(
        evidence
            .arguments
            .iter()
            .any(|argument| argument.contains("concrete-playback")),
        "the witness vector carries concrete playback: {:?}",
        evidence.arguments
    );
    assert!(
        !proof
            .arguments
            .iter()
            .any(|argument| argument.contains("concrete-playback")),
        "the proof vector carries none: {:?}",
        proof.arguments
    );
    assert_eq!(
        evidence.harness_path, proof.harness_path,
        "one harness identity"
    );
    assert_eq!(
        evidence.ceilings, proof.ceilings,
        "equal ceilings across the phases"
    );
    assert_eq!(evidence.ceilings, harness.identity.ceilings);
    assert_eq!(
        proof.outcome,
        KaniRunOutcome::Inconclusive {
            reason: KaniInconclusiveReason::FailedWithoutCounterexample
        },
        "the proof vector prints no playback"
    );
    assert!(
        counterexample.contains(check),
        "the failing check names `{check}`:\n{counterexample}"
    );
    let values = playback_values(counterexample);
    assert!(!values.is_empty(), "a concrete playback: {counterexample}");
    assert!(
        values.iter().all(|value| (0..=1000).contains(value)),
        "in-domain playback: {values:?}"
    );
    println!("IR1509 playback values={values:?} failing check `{check}`");
    values
}

const RESULT_CHECK: &str =
    "the produced V2 Boolean result agrees with the independent checked condition";

#[trace("FR-015-AC-151", "FR-015-AC-126", "FR-017-AC-63", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_runner_verifies_the_healthy_clause() {
    let (scratch, harness) = kani_crate("ir1509-kani-healthy", None, HEALTHY_SUBJECT);
    let run = run_installed(&scratch, &harness);
    assert_eq!(run.evidence.outcome, KaniRunOutcome::Verified);
    assert!(
        run.evidence.proof_phase.is_none(),
        "a verified proof makes one launch"
    );
    assert!(run.evidence.success_checks > 0);
    assert!(
        run.evidence
            .checks
            .iter()
            .any(|check| format!("{:?}", check.status) == "Satisfied"),
        "the final cover is satisfied"
    );
}

#[trace("FR-015-AC-151", "FR-017-AC-63", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_runner_falsifies_the_exact_999_subject_mutant_at_999() {
    let (scratch, harness) = kani_crate(
        "ir1509-kani-plus-two",
        None,
        "quire_contract_runtime::operators::implies_short_circuit(amount < 1000, || amount + 2 <= 1000)",
    );
    let run = run_installed(&scratch, &harness);
    let values = assert_two_phase_falsified(&run, &harness, RESULT_CHECK);
    assert_eq!(values, vec![999]);
}

#[trace("FR-015-AC-151", "FR-015-AC-41", "FR-017-AC-63", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_runner_falsifies_the_plus_one_kernel_mutant_by_the_arithmetic_probe() {
    let (scratch, harness) = kani_crate(
        "ir1509-kani-kernel-plus-one",
        Some(&PLUS_ONE),
        HEALTHY_SUBJECT,
    );
    let run = run_installed(&scratch, &harness);
    let values = assert_two_phase_falsified(&run, &harness, ARITHMETIC_MESSAGE);
    assert!(values.iter().all(|value| *value < 1000), "{values:?}");
}

#[trace("FR-015-AC-151", "FR-015-AC-41", "FR-017-AC-63", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_runner_falsifies_the_left_only_kernel_mutant_by_the_arithmetic_probe() {
    let (scratch, harness) =
        kani_crate("ir1509-kani-kernel-left", Some(&LEFT_ONLY), HEALTHY_SUBJECT);
    let run = run_installed(&scratch, &harness);
    let values = assert_two_phase_falsified(&run, &harness, ARITHMETIC_MESSAGE);
    assert!(values.iter().all(|value| *value < 1000), "{values:?}");
}

#[trace("FR-015-AC-151", "FR-015-AC-119", "FR-017-AC-63", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_runner_falsifies_the_helper_mutant_by_the_produced_result_comparison() {
    let (scratch, harness) = kani_crate("ir1509-kani-helper", Some(&HELPER_FALSE), HEALTHY_SUBJECT);
    let run = run_installed(&scratch, &harness);
    let _ = assert_two_phase_falsified(&run, &harness, RESULT_CHECK);
}

// ---- the public playback entry over a launcher stand-in (FR-017-AC-60) ---------------------

/// A scratch crate holding `lib` and a `cargo-kani` stand-in script that records its arguments
/// and prints `stdout`.
struct PlaybackStandIn {
    directory: TemporaryDirectory,
    installation: KaniInstallation,
    calls: PathBuf,
}

impl PlaybackStandIn {
    fn new(label: &str, lib: &str, stdout: &str) -> Self {
        use std::os::unix::fs::PermissionsExt;
        let directory = TemporaryDirectory::new(label);
        fs::create_dir_all(directory.path.join("src")).unwrap();
        fs::write(directory.path.join("src/lib.rs"), lib).unwrap();
        let launcher = directory.path.join("cargo-kani");
        let calls = directory.path.join("calls");
        let printed = directory.path.join("stdout");
        fs::write(&printed, stdout).unwrap();
        fs::write(
            &launcher,
            format!(
                "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then echo 'cargo-kani 0.68.0'; exit 0; fi\necho \"$@\" >> '{}'\ncat '{}'\n",
                calls.display(),
                printed.display()
            ),
        )
        .unwrap();
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755)).unwrap();
        Self {
            directory,
            installation: KaniInstallation::new(launcher),
            calls,
        }
    }

    fn launches(&self) -> Vec<String> {
        fs::read_to_string(&self.calls)
            .map(|calls| calls.lines().map(str::to_owned).collect())
            .unwrap_or_default()
    }

    fn run(
        &self,
        harness: &KaniV2ClauseObligationHarness,
        test_name: &str,
        appended: &str,
    ) -> Result<KaniConcretePlaybackOutcome, quire_contract_codegen::KaniExecutionRefusal> {
        let cancel = quire_exact::Cancel::new();
        execute_kani_concrete_playback(
            &KaniExecutionRequest {
                cancel: &cancel,
                installation: &self.installation,
                harness: harness.into(),
                crate_directory: &self.directory.path,
                target_directory: &self.directory.path.join("target"),
                home_directory: None,
            },
            &KaniConcretePlayback {
                test_name,
                appended_test: appended,
            },
        )
    }
}

#[trace("FR-017-AC-57", "FR-017-AC-59", "TC-027")]
#[test]
fn the_playback_entry_launches_only_for_the_sealed_artifact_with_one_appended_test() {
    use quire_contract_codegen::KaniExecutionRefusal as Refusal;
    let harness = fixture();
    let sealed = &harness.rust.contents;
    let name = "playback_999_arithmetic";
    let appended = kani_playback_test(name, true, true);
    let test_name = format!("{}::{name}", harness.identity.module_symbol);
    let with_test = append_inside_module(sealed, &appended);
    let ok_output = format!(
        "running 1 test\ntest {test_name} - should panic ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n"
    );

    // The sealed artifact plus exactly the declared test launches one exact playback.
    let accepted = PlaybackStandIn::new("ir917-playback-accepted", &with_test, &ok_output);
    let outcome = accepted
        .run(&harness, &test_name, &appended)
        .expect("launches");
    let KaniConcretePlaybackOutcome::Completed {
        succeeded, stdout, ..
    } = outcome
    else {
        panic!("the stand-in exits on its own");
    };
    assert!(succeeded);
    assert!(String::from_utf8_lossy(&stdout).contains("should panic ... ok"));
    let launches = accepted.launches();
    assert_eq!(launches.len(), 1, "{launches:?}");
    for argument in [
        "kani playback",
        "--lib",
        "-Z function-contracts",
        "-Z concrete-playback",
        &test_name,
        "--exact",
    ] {
        assert!(launches[0].contains(argument), "{argument}: {launches:?}");
    }

    // Each of these refuses before any launch.
    let altered_oracle = with_test.replacen("fn oracle", "fn  oracle", 1);
    let altered_header = with_test.replacen(
        "SPDX-License-Identifier: MIT",
        "SPDX-License-Identifier: XIT",
        1,
    );
    assert_ne!(
        altered_header, with_test,
        "the sealed header carries the SPDX line"
    );
    assert_ne!(
        altered_oracle, with_test,
        "the sealed text carries the oracle function"
    );
    let duplicated = format!(
        "{}{appended}{}",
        &with_test[..with_test.rfind('}').unwrap()],
        &with_test[with_test.rfind('}').unwrap()..]
    );
    for (case, lib) in [
        ("test absent", sealed.to_string()),
        ("oracle altered", altered_oracle),
        ("header altered", altered_header),
        ("test duplicated", duplicated),
        ("second sealed copy", format!("{with_test}\n{sealed}")),
    ] {
        let refused = PlaybackStandIn::new(
            &format!("ir917-playback-{}", case.replace(' ', "-")),
            &lib,
            &ok_output,
        );
        assert!(
            matches!(
                refused.run(&harness, &test_name, &appended),
                Err(Refusal::ConcretePlaybackSourceMismatch { .. })
            ),
            "{case}"
        );
        assert!(refused.launches().is_empty(), "{case}: no launch");
    }
    let wrong_module = PlaybackStandIn::new("ir917-playback-wrong-module", &with_test, &ok_output);
    assert!(matches!(
        wrong_module.run(&harness, &format!("other::{name}"), &appended),
        Err(Refusal::ConcretePlaybackSourceMismatch { .. })
    ));
    assert!(wrong_module.launches().is_empty());

    // A zero-match or failed libtest run is never read as a passing playback.
    for (case, output) in [
        (
            "zero matches",
            "running 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out\n".to_owned(),
        ),
        (
            "pass line without a summary",
            format!("running 1 test\ntest {test_name} - should panic ... ok\n"),
        ),
        (
            "another test passed",
            "running 1 test\ntest other::t ... ok\n\ntest result: ok. 1 passed; 0 failed;\n".to_owned(),
        ),
    ] {
        let unobserved = PlaybackStandIn::new(&format!("ir917-playback-{}", case.replace(' ', "-")), &with_test, &output);
        assert!(
            matches!(
                unobserved.run(&harness, &test_name, &appended),
                Err(Refusal::ConcretePlaybackTestNotObserved { .. })
            ),
            "{case}"
        );
    }
}

// ---- Kani-backed concrete playback at 999 (FR-017-AC-60) ---------------------------------

/// The Kani playback test: the real `kani` crate replays the bytes of 999 through the sealed
/// harness body. `expected` is the arithmetic assertion's message, or absent for the healthy run.
fn kani_playback_test(name: &str, expect_arithmetic_panic: bool, subject_preamble: bool) -> String {
    let attribute = if expect_arithmetic_panic {
        format!("    #[should_panic(expected = \"{ARITHMETIC_MESSAGE}\")]\n")
    } else {
        String::new()
    };
    let preamble = if subject_preamble {
        "        assert!(crate::subject::check(999), \"the unchanged subject returns normally at 999\");\n"
    } else {
        ""
    };
    format!(
        "    #[test]\n{attribute}    fn {name}() {{\n{preamble}        kani::concrete_playback_run(vec![999_i64.to_le_bytes().to_vec()], prove_clause);\n    }}\n"
    )
}

/// The environment's peak: the `memory.peak` of the cgroup scope this process runs in (the
/// `systemd-run` scope that caps the run). It is cumulative over the scope, so it bounds the
/// playback's own peak from above.
#[derive(Debug)]
pub(crate) struct ScopePeak;

impl PeakMemorySource for ScopePeak {
    fn name(&self) -> &str {
        "cgroup-memory.peak"
    }

    fn peak(&self, _launcher: u32) -> Option<PeakReading> {
        let membership = fs::read_to_string("/proc/self/cgroup").ok()?;
        let path = membership
            .lines()
            .find_map(|line| line.strip_prefix("0::"))?;
        let value = fs::read_to_string(format!("/sys/fs/cgroup{path}/memory.peak")).ok()?;
        Some(PeakReading {
            value: value.trim().parse().ok()?,
            unit: MemoryUnit::Bytes,
        })
    }
}

/// The command lines of every live process whose command line contains `needle` (`pgrep -f`).
fn live_processes_naming(needle: &str) -> Vec<String> {
    let output = Command::new("pgrep")
        .args(["-a", "-f", "--", &escape_pattern(needle)])
        .output()
        .expect("pgrep");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect()
}

fn escape_pattern(text: &str) -> String {
    text.chars()
        .flat_map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '/' | '_' | '-' | ':') {
                vec![character]
            } else {
                vec!['\\', character]
            }
        })
        .collect()
}

/// Owned teardown: once a playback has concluded, no process of the run is left alive. The cargo
/// and rustc children carry the target directory in their paths and the playback test binary
/// carries it in its executable path and the exact test name in its arguments; the scratch
/// crate directory is only a working directory and is not on any command line.
fn assert_no_process_of_playback(target: &Path, test_name: &str) {
    for needle in [target.to_string_lossy().into_owned(), test_name.to_owned()] {
        let alive = live_processes_naming(&needle);
        assert!(
            alive.is_empty(),
            "a process naming {needle} survived the concluded playback: {alive:?}"
        );
    }
}

/// The teardown check can see a live process: a process carrying the target directory in its
/// command line is found while it runs, and the check fails; it passes once the process is gone.
#[trace("FR-017-AC-60", "TC-027")]
#[test]
fn the_teardown_check_sees_a_live_process() {
    let target = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ir917-teardown-control");
    let test_name = "ir917_teardown_control::playback_999";
    let mut live = Command::new("sh")
        .args(["-c", "sleep 60; true", test_name])
        .arg(&target)
        .spawn()
        .expect("spawn the live process");
    let survived = std::panic::catch_unwind(|| assert_no_process_of_playback(&target, test_name));
    live.kill().expect("stop the live process");
    live.wait().expect("reap the live process");
    assert!(survived.is_err(), "the check missed a live process");
    assert_no_process_of_playback(&target, test_name);
}

/// Writes the sealed artifact with `appended` inside its module, beside the subject, into the
/// scratch crate's library and runs the playback through CG's public entry.
fn run_installed_playback(
    scratch: &ScratchCrate,
    harness: &KaniV2ClauseObligationHarness,
    subject_body: &str,
    name: &str,
    appended: &str,
) -> (bool, Option<i32>, String, String, MemoryObservation) {
    let sealed = &harness.rust.contents;
    let with_test = append_inside_module(sealed, appended);
    assert_eq!(
        remove_appended(&with_test, appended),
        *sealed,
        "removing exactly the appended bytes restores the sealed artifact"
    );
    fs::write(
        scratch.directory.path.join("src/lib.rs"),
        format!("{with_test}{}", subject_module(subject_body)),
    )
    .unwrap();
    let test_name = format!("{}::{name}", harness.identity.module_symbol);
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover()
        .expect("cargo-kani is installed")
        .with_peak_memory_source(std::sync::Arc::new(ScopePeak));
    let target = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ir917-kani-target");
    let started = Instant::now();
    let outcome = execute_kani_concrete_playback(
        &KaniExecutionRequest {
            cancel: &cancel,
            installation: &installation,
            harness: harness.into(),
            crate_directory: &scratch.directory.path,
            target_directory: &target,
            home_directory: None,
        },
        &KaniConcretePlayback {
            test_name: &test_name,
            appended_test: appended,
        },
    );
    let wall = started.elapsed();
    let outcome = outcome.unwrap_or_else(|refusal| panic!("{test_name}: {refusal}"));
    assert_no_process_of_playback(&target, &test_name);
    let KaniConcretePlaybackOutcome::Completed {
        succeeded,
        exit_code,
        stdout,
        stderr,
        memory,
    } = outcome
    else {
        panic!("{test_name}: the playback must complete on its own: {outcome:?}");
    };
    println!(
        "IR917 {test_name}: succeeded={succeeded} exit={exit_code:?} wall={:.1}s memory={memory:?}",
        wall.as_secs_f64()
    );
    (
        succeeded,
        exit_code,
        String::from_utf8_lossy(&stdout).into_owned(),
        String::from_utf8_lossy(&stderr).into_owned(),
        memory,
    )
}

#[trace("FR-017-AC-60", "FR-015-AC-41", "TC-027")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_027_kani_playback_at_999_passes_the_healthy_clause() {
    let (scratch, harness) = kani_crate("ir917-playback-healthy", None, HEALTHY_SUBJECT);
    let appended = kani_playback_test("playback_999_healthy", false, true);
    let (succeeded, _, stdout, _, memory) = run_installed_playback(
        &scratch,
        &harness,
        HEALTHY_SUBJECT,
        "playback_999_healthy",
        &appended,
    );
    assert!(succeeded, "{stdout}");
    assert!(stdout.contains("playback_999_healthy ... ok"), "{stdout}");
    assert!(stdout.contains("1 passed; 0 failed"), "{stdout}");
    assert_eq!(memory.source.as_deref(), Some("cgroup-memory.peak"));
    assert!(memory.peak_resident_bytes.is_some_and(|peak| peak > 0));
}

#[trace("FR-017-AC-60", "FR-015-AC-41", "FR-017-AC-63", "TC-027")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_027_kani_playback_at_999_fails_the_arithmetic_assertion_under_the_plus_one_owner_mutant() {
    let (scratch, harness) =
        kani_crate("ir917-playback-plus-one", Some(&PLUS_ONE), HEALTHY_SUBJECT);
    // The separate unconstrained two-phase run of the same sealed artifact comes first.
    let run = run_installed(&scratch, &harness);
    let values = assert_two_phase_falsified(&run, &harness, ARITHMETIC_MESSAGE);
    assert!(values.iter().all(|value| *value < 1000), "{values:?}");

    let name = "playback_999_arithmetic";
    let appended = kani_playback_test(name, true, true);
    let (succeeded, _, stdout, stderr, memory) =
        run_installed_playback(&scratch, &harness, HEALTHY_SUBJECT, name, &appended);
    assert!(succeeded, "{stdout}");
    assert!(
        stdout.contains(&format!("{name} - should panic ... ok")),
        "{stdout}"
    );
    assert!(stdout.contains("1 passed; 0 failed"), "{stdout}");
    assert!(
        format!("{stdout}{stderr}").contains(ARITHMETIC_MESSAGE),
        "the panic names the generated arithmetic assertion:\n{stdout}\n{stderr}"
    );
    assert_eq!(memory.source.as_deref(), Some("cgroup-memory.peak"));
    assert!(memory.peak_resident_bytes.is_some_and(|peak| peak > 0));
}

#[trace("FR-017-AC-60", "TC-027")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_027_kani_playback_at_999_controls_do_not_satisfy_the_arithmetic_expectation() {
    // The unmutated owner does not fail the arithmetic assertion at 999.
    let name = "playback_999_arithmetic";
    let (scratch, harness) = kani_crate("ir917-playback-control-owner", None, HEALTHY_SUBJECT);
    let appended = kani_playback_test(name, true, true);
    let (succeeded, _, stdout, _, _) =
        run_installed_playback(&scratch, &harness, HEALTHY_SUBJECT, name, &appended);
    assert!(!succeeded, "{stdout}");
    assert!(
        stdout.contains("test did not panic as expected"),
        "{stdout}"
    );

    // The compiled-subject `+ 2` mutant reaches the produced-result comparison, not the
    // arithmetic assertion.
    let other = "quire_contract_runtime::operators::implies_short_circuit(amount < 1000, || amount + 2 <= 1000)";
    let (scratch, harness) = kani_crate("ir917-playback-control-subject", None, other);
    let appended = kani_playback_test(name, true, false);
    let (succeeded, _, stdout, _, _) =
        run_installed_playback(&scratch, &harness, other, name, &appended);
    assert!(!succeeded, "{stdout}");
    assert!(
        stdout.contains(
            "the produced V2 Boolean result agrees with the independent checked condition"
        ),
        "{stdout}"
    );
    assert!(
        stdout.contains("panic did not contain expected string"),
        "{stdout}"
    );
}

/// The result-bearing subject `check() -> i64`, drawing nothing, returns `value`.
fn result_subject_module(value: i64) -> String {
    format!("\npub mod subject {{\n    pub fn check() -> i64 {{ {value} }}\n}}\n")
}

fn run_result_domain(label: &str, produced: i64) -> InstalledRun {
    let harness = fixture_of(Twin::with_bounded_result_postcondition(), "DepositResult");
    let lib = format!(
        "{}{}",
        harness.rust.contents,
        result_subject_module(produced)
    );
    let scratch = ScratchCrate::new(label, &lib, None);
    run_installed(&scratch, &harness)
}

/// A produced Integer inside the admitted `Int[0, 1000]` result domain verifies.
#[trace("FR-025-AC-13", "TC-036")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_036_real_kani_runner_verifies_an_in_domain_produced_result() {
    let run = run_result_domain("ir948-result-domain-healthy", 1000);
    assert!(
        matches!(run.evidence.outcome, KaniRunOutcome::Verified),
        "{:?}",
        run.evidence.outcome
    );
}

/// A produced Integer outside the domain (1001 satisfies the clause `result >= 0`) falsifies
/// exactly the domain assertion, so the clause cannot mask an out-of-domain return.
#[trace("FR-025-AC-13", "TC-036")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_036_real_kani_runner_falsifies_an_out_of_domain_produced_result() {
    let run = run_result_domain("ir948-result-domain-out", 1001);
    let KaniRunOutcome::Falsified { counterexample } = &run.evidence.outcome else {
        panic!("expected Falsified, got {:?}", run.evidence.outcome);
    };
    assert!(
        counterexample.contains("the produced V2 result lies in its admitted domain"),
        "the failing check is the domain assertion:\n{counterexample}"
    );
    assert!(
        !counterexample.contains("the V2 postcondition holds for the produced result"),
        "the clause assertion is not the failing one:\n{counterexample}"
    );
    let failed = run
        .evidence
        .checks
        .iter()
        .filter(|check| format!("{:?}", check.status) == "Failure")
        .count();
    assert_eq!(failed, 1, "only the domain assertion fails");
}

/// The QSL precondition `other > 1000` over the `Int[0, 1000]` parameter `other` cannot hold on
/// any admitted input, so the installed proof ends with its final cover unsatisfied, neither
/// resource exhaustion nor a vacuous `Verified`.
#[trace("FR-015-AC-126", "FR-017-AC-63", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_runner_reports_an_impossible_precondition_as_cover_unsatisfied() {
    let harness = fixture_of(
        Twin::with_bounded_other_precondition_condition("other > 1000"),
        "OtherBelowLimit",
    );
    let lib = format!("{}{}", harness.rust.contents, subject_module("true"));
    let scratch = ScratchCrate::new("ir917-kani-impossible-precondition", &lib, None);
    let run = run_installed(&scratch, &harness);
    assert!(
        matches!(
            run.evidence.outcome,
            KaniRunOutcome::CoverUnsatisfied { .. }
        ),
        "{:?}",
        run.evidence.outcome
    );
}

#[trace("FR-015-AC-132", "FR-017-AC-64", "TC-025")]
#[test]
fn the_matrix_status_of_the_enumeration_rows_says_enumeration_evidence_without_proof_wording() {
    let matrix =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("spec/kani/matrix/tests.md"))
            .expect("the matrix index");
    for id in ["FR-015-AC-132", "FR-017-AC-64"] {
        let row = matrix
            .lines()
            .find(|line| line.starts_with("| FR-0") && line.contains(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("a matrix row for {id}"));
        let status = row.split('|').nth(4).expect("status column");
        assert!(status.contains("enumeration evidence"), "{id}: {status}");
        for token in BANNED_TOKENS {
            assert!(
                !status.contains(token),
                "{id} status contains `{token}`: {status}"
            );
        }
    }
}

/// FR-056-AC-13 (IR-1526): through CG's runner under installed Kani, the clause a measured table
/// lets through ends with an executed verdict, recorded with its wall time, the Kani version and
/// CG's generator and kernel versions; and integer multiply under a measured-bad row is refused
/// before any Kani launch. The Decimal multiply row waits for IR-1444.
#[trace("FR-056-AC-13", "TC-070")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_070_real_kani_runs_the_clause_the_table_lets_through_and_refuses_the_measured_bad_multiply() {
    use quire_contract_codegen::{
        negotiate_kani_obligations_with_options, GenerationOptions, IntegerRoute,
        GENERATOR_VERSION, KERNEL_VERSION,
    };
    let scratch = TemporaryDirectory::new("ir1526-kani-table");
    let table = scratch.path.join("table.json");
    fs::write(
        &table,
        serde_json::to_vec(&[serde_json::json!({
            "operation_ids": ["quire.op.integer.mul"],
            "max_operand_bits": 21,
            "collection_bound": null,
            "verdict": "not_tractable",
            "wall_seconds": 600,
            "peak_bytes": 17_179_869_184_u64,
            "kani_version": "0.68.0",
            "cbmc_version": "6.11.0",
            "solver_version": "cadical",
            "generator_version": GENERATOR_VERSION,
            "kernel_version": KERNEL_VERSION,
        })])
        .unwrap(),
    )
    .unwrap();
    let options = GenerationOptions {
        tractability_table: Some(table),
    };
    let negotiate = |package: &quire_contract_model::CheckedPackageV2,
                     clause: &quire_contract_model::CheckedNodeId,
                     route: IntegerRoute| {
        negotiate_kani_obligations_with_options(
            &KaniObligationRequest {
                ceilings: INSTALLED_CEILINGS,
                items: &[ObligationItem::V2Clause {
                    package,
                    clause,
                    claim: &crate::common::v2_claim::claim_for(package, clause),
                    dependencies: None,
                }],
                subject_path: SUBJECT,
                unwind: 3,
            },
            &options,
            route,
        )
        .expect("the request negotiates")
    };

    // The multiply is refused before a harness exists, so no Kani launch can follow.
    let (package, clause) = Twin::with_two_parameter_precondition(
        (0, 1 << 20),
        (0, 1 << 20),
        "other * offset + 3 <= 100",
    )
    .emitted_package("TwoParameters");
    let refused = negotiate(&package, &clause, IntegerRoute::General);
    assert!(matches!(
        &refused.records()[0].disposition,
        quire_contract_codegen::ObligationDisposition::Unsupported {
            reason: quire_contract_codegen::UnsupportedObligation::V2ClauseNotTractable { .. }
        }
    ));
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = refused
    else {
        panic!("the request is valid");
    };
    assert!(v2_clause_harnesses.is_empty(), "no harness, so no launch");
    // A multiply narrower than the row's bound is not refused. The native route renders no
    // two-parameter precondition, so the clause is negotiated on the general route; the
    // decision is the same on both and no Kani is launched for it.
    let (narrow, narrow_clause) = Twin::with_two_parameter_precondition(
        (0, 1 << 9),
        (0, 1 << 9),
        "other * offset + 3 <= 100",
    )
    .emitted_package("TwoParameters");
    assert!(matches!(
        &negotiate(&narrow, &narrow_clause, IntegerRoute::General).records()[0].disposition,
        quire_contract_codegen::ObligationDisposition::Supported { .. }
    ));

    // The clause the table lets through runs under installed Kani to an executed verdict.
    let (package, clause) =
        Twin::with_bounded_amount_implication().emitted_package("BalanceNeverDrops");
    let KaniObligationOutcome::Emitted {
        mut v2_clause_harnesses,
        ..
    } = negotiate(&package, &clause, IntegerRoute::Native)
    else {
        panic!("the clause negotiates");
    };
    let harness = v2_clause_harnesses.remove(0);
    let lib = format!(
        "{}{}",
        harness.rust.contents,
        subject_module(HEALTHY_SUBJECT)
    );
    let crate_scratch = ScratchCrate::new("ir1526-kani-healthy", &lib, None);
    let started = Instant::now();
    let run = run_installed(&crate_scratch, &harness);
    let wall = started.elapsed();
    assert!(
        matches!(
            run.evidence.outcome,
            KaniRunOutcome::Verified | KaniRunOutcome::Falsified { .. }
        ),
        "an executed verdict, got {:?}",
        run.evidence.outcome
    );
    let kani_version = Command::new("cargo")
        .args(["kani", "--version"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default();
    println!(
        "IR1526 verdict={:?} wall={:.1}s kani=`{kani_version}` generator={GENERATOR_VERSION} kernel={KERNEL_VERSION}",
        run.evidence.outcome,
        wall.as_secs_f64()
    );
}

//! IR-211 kani lane: joins a real `cargo kani` falsification transcript to the generator's own
//! persisted obligation schema (`kani-obligations/{module}.json`), then proves the join actually
//! depends on that persisted schema by mutating it on disk and showing the decode refuses.
//!
//! Like `tests/kani_obligations.rs`'s own pinned lane, this needs the real installed Kani 0.67.0
//! backend and is `#[ignore]`d by default. Run it with:
//!
//! ```text
//! cargo test --locked --test kani_witness_join -- --ignored --test-threads=1
//! ```
//!
//! `KaniObligationIdentity` and `ObligationBinding` are `Serialize`-only (no `Deserialize`), by
//! design consistent with this crate's one-way generate-then-persist flow, so nothing in this
//! crate can literally re-hydrate a typed identity from the persisted JSON. This test reads the
//! fields the join actually needs (`identifier`, `role`, `primitiveType`) generically out of the
//! real persisted `serde_json::Value` instead, and calls this crate's own `witness_schema` on the
//! result — the same production function `quire_contract_codegen::decode_falsification` uses.

use std::{
    env, fs,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    execute_kani_obligation, negotiate_kani_obligations, witness_schema, write_bundle_atomic,
    ArtifactBundle, AttestationContext, KaniBindingRole, KaniExecutionRequest, KaniInstallation,
    KaniObligationHarness, KaniObligationOutcome, KaniObligationRequest, KaniPrimitiveType,
    KaniRunOutcome, KaniToolPins, ObligationBinding, ObligationDisposition, ObligationItem,
    ObligationRecord, IR_CANDIDATE_REVISION, RUNTIME_REVISION,
};
use quire_contract_ir::{
    kani::{Witness, WitnessValue, WitnessValueType},
    BoundPackage, ClauseId, ClauseRef, RequirementRef, EXECUTABLE_PROJECTION_FORMAT,
};
use serde_json::{json, Value};

const PACKAGE: &str = "test/kani-witness-join";
const PRECONDITION: &str = "amount-within-balance";
const POSTCONDITION: &str = "balance-never-grows";
const INVARIANT: &str = "balance-nonnegative";

/// Budget for the real `cargo-kani` run. Generous for the same reason
/// `tests/kani_obligations.rs::REAL_KANI_TIMEOUT` is: CBMC is memory- and time-heavy even on this
/// small obligation, and this is a ceiling against a genuine hang, not a performance target.
const REAL_KANI_TIMEOUT: Duration = Duration::from_secs(600);

/// The postcondition (`balance` never grows) is falsified against this seeded defect, which
/// credits instead of debiting. The precondition and invariant harnesses this lane also
/// negotiates (only to give the postcondition's union ABI its real two-argument shape — see
/// `withdraw_harnesses`) are never run against any subject.
const SEEDED_FAILING_SUBJECT: &str = "/// Seeded defect: credits instead of debiting.\n#[must_use]\npub fn withdraw(amount_current: i64, balance_pre: i64) -> i64 {\n    balance_pre + amount_current\n}\n";

fn context() -> AttestationContext<'static> {
    AttestationContext {
        record_digest: "0000000000000000000000000000000000000000000000000000000000000000",
        candidate_revision: IR_CANDIDATE_REVISION,
    }
}

// ---- minimal V1 fixture: withdraw's precondition, postcondition and invariant ----------------
//
// Trimmed from `tests/kani_obligations.rs`'s own `clauses()`/`bound_package()` fixture (not
// touched by this change) to just the one operation this lane needs.

fn span(line: u64) -> Value {
    let source = json!({"document":"kani-witness-join", "revision":1});
    json!({"start":{"source":source,"line":line,"column":1,"byte_offset":line - 1},
        "end":{"source":source,"line":line,"column":2,"byte_offset":line}})
}

fn int(minimum: i64, maximum: i64) -> Value {
    json!({"kind":"integer","domain":"signed","minimum":minimum,"maximum":maximum,"overflow":"reject"})
}

fn owner() -> Value {
    json!({"package":PACKAGE,"requirement":"FR-200","revision":1})
}

fn read(name: &str, observation: &str, line: u64) -> Value {
    json!({"node":"value_reference","name":name,"observation":observation,"source":span(line)})
}

fn identity_ref(kind: &str, name: &str, observation: &str) -> Value {
    json!({"node":"reference","identity":{"requirement":owner(),"kind":kind,"observation":observation,"path":[name]}})
}

struct ClauseFixture {
    id: &'static str,
    kind: &'static str,
    anchor: Value,
    line: u64,
    references: Vec<Value>,
    values: Vec<Value>,
    expression: Value,
}

fn amount(line: u64) -> Value {
    json!({"name":"amount","kind":"input","value_type":int(0, 1000),"source":span(line)})
}

fn balance(line: u64, maximum: i64) -> Value {
    json!({"name":"balance","kind":"state","value_type":int(0, maximum),"source":span(line)})
}

fn clauses() -> Vec<ClauseFixture> {
    let pre = json!({"kind":"pre","operation":"withdraw"});
    vec![
        ClauseFixture {
            id: PRECONDITION,
            kind: "precondition",
            anchor: pre.clone(),
            line: 10,
            references: vec![
                identity_ref("input", "amount", "current"),
                identity_ref("state", "balance", "current"),
            ],
            values: vec![amount(11), balance(12, 1000)],
            expression: json!({"node":"compare","operator":"less_equal",
                "left":read("amount", "current", 13),"right":read("balance", "current", 14),
                "source":span(11)}),
        },
        ClauseFixture {
            id: POSTCONDITION,
            kind: "postcondition",
            anchor: json!({"kind":"post","operation":"withdraw"}),
            line: 20,
            references: vec![
                identity_ref("state", "balance", "post"),
                identity_ref("state", "balance", "pre"),
            ],
            values: vec![balance(21, 1000)],
            expression: json!({"node":"compare","operator":"less_equal",
                "left":read("balance", "post", 22),"right":read("balance", "pre", 23),
                "source":span(21)}),
        },
        ClauseFixture {
            id: INVARIANT,
            kind: "invariant",
            anchor: json!({"kind":"handler","name":"withdraw"}),
            line: 30,
            references: vec![identity_ref("state", "balance", "current")],
            values: vec![balance(31, 1000)],
            expression: json!({"node":"compare","operator":"greater_equal",
                "left":read("balance", "current", 32),
                "right":{"node":"integer_literal","value":0,"value_type":int(0, 1000),"source":span(33)},
                "source":span(31)}),
        },
    ]
}

fn projection() -> Value {
    let fixtures = clauses();
    let package_clauses = fixtures
        .iter()
        .map(|clause| {
            let body = match clause.references.as_slice() {
                [single] => single.clone(),
                many => json!({"node":"composite","children":many}),
            };
            json!({"id":clause.id,"kind":clause.kind,"anchor":clause.anchor,
                "source":span(clause.line),"body":body})
        })
        .collect::<Vec<_>>();
    let bindings = fixtures
        .iter()
        .map(|clause| {
            json!({"clause":{"requirement":owner(),"clause":clause.id},
                "expression":{"owner":owner(),"types":[],"values":clause.values,"functions":[],
                    "expression":clause.expression,"expected_type":{"kind":"boolean"},
                    "execution_point":clause.anchor,"clause_root":true}})
        })
        .collect::<Vec<_>>();
    json!({
        "format":EXECUTABLE_PROJECTION_FORMAT,
        "package":{"id":PACKAGE,"schema_version":{"major":1,"minor":1},
            "source":{"document":"kani-witness-join","revision":1},
            "requirements":[{"id":"FR-200","revision":1,"source":span(1),"clauses":package_clauses}]},
        "bindings":bindings
    })
}

fn bound_package() -> BoundPackage {
    BoundPackage::from_json_bytes(&serde_json::to_vec(&projection()).unwrap())
        .unwrap_or_else(|diagnostics| panic!("fixture projection must bind: {diagnostics:?}"))
}

fn clause(id: &str) -> ClauseRef {
    ClauseRef::new(
        RequirementRef::parse(PACKAGE, "FR-200", 1).unwrap(),
        ClauseId::new(id).unwrap(),
    )
}

fn emitted(outcome: KaniObligationOutcome) -> (Vec<ObligationRecord>, Vec<KaniObligationHarness>) {
    match outcome {
        KaniObligationOutcome::Emitted { records, harnesses } => (records, harnesses),
        KaniObligationOutcome::Rejected { records } => panic!("unexpected rejection: {records:#?}"),
    }
}

/// The precondition, postcondition and invariant harnesses for `withdraw`, negotiated together so
/// their subject ABI is the union `unify_subject_signatures` computes — matching
/// `HEALTHY_SUBJECT`'s two-argument signature.
fn withdraw_harnesses(pins: &KaniToolPins) -> Vec<KaniObligationHarness> {
    let package = bound_package();
    let refs = [
        clause(PRECONDITION),
        clause(POSTCONDITION),
        clause(INVARIANT),
    ];
    let items = refs
        .iter()
        .map(|clause| ObligationItem::BoundClause {
            package: &package,
            clause,
        })
        .collect::<Vec<_>>();
    let request = KaniObligationRequest {
        items: &items,
        subject_path: "crate::withdraw",
        pins,
        unwind: 4,
        attestation: context(),
    };
    let (records, harnesses) = emitted(negotiate_kani_obligations(&request).unwrap());
    assert!(
        records
            .iter()
            .all(|record| matches!(record.disposition, ObligationDisposition::Supported { .. })),
        "{records:#?}"
    );
    // `bound_package` leaks past this function's return, so `package` must outlive the borrow the
    // request holds; negotiation is complete by the time we get here, so this is fine to drop.
    harnesses
}

// ---- real Kani execution plumbing (trimmed from tests/kani_obligations.rs) --------------------

fn scratch(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = env::temp_dir().join(format!(
        "quire-kani-witness-join-{name}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(path.join("src")).unwrap();
    path
}

fn write_crate(harness: &KaniObligationHarness, subject: &str) -> PathBuf {
    let directory = scratch("crate");
    fs::write(
        directory.join("src/lib.rs"),
        format!(
            "//! Generated obligation check crate.\n\n{}\n{subject}",
            harness.rust.contents
        ),
    )
    .unwrap();
    fs::write(
        directory.join("Cargo.toml"),
        format!(
            "[package]\nname = \"generated-kani-witness-join\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nquire-contract-runtime = {{ git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"{RUNTIME_REVISION}\" }}\n\n[workspace]\n"
        ),
    )
    .unwrap();
    fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .unwrap();
    directory
}

/// Runs the seeded defect through the real pinned `execute_kani_obligation` (the actual code path
/// this test's join relies on) and returns its real counterexample transcript, alongside the raw
/// `** <failed> of <total> failed` summary line Kani printed for that same run — captured by a
/// second, independent invocation of the same pinned launcher with the same options in the same
/// crate and target directory (so it reuses the first run's build), purely to report how many
/// checks the harness actually ran; `execute_kani_obligation` classifies but does not retain that
/// raw count.
fn run_falsifying(
    installation: &KaniInstallation,
    harness: &KaniObligationHarness,
) -> (String, String) {
    let crate_directory = write_crate(harness, SEEDED_FAILING_SUBJECT);
    let target_directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-witness-join");
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        installation,
        harness,
        crate_directory: &crate_directory,
        target_directory: &target_directory,
        timeout: REAL_KANI_TIMEOUT,
    })
    .unwrap_or_else(|refusal| panic!("real run must not refuse: {refusal}"));
    let counterexample = match evidence.outcome {
        KaniRunOutcome::Falsified { counterexample } => counterexample,
        other => panic!("the seeded defect must be falsified, got {other:?}"),
    };
    let mut arguments = vec!["kani".to_owned()];
    arguments.extend(harness.identity.options.iter().cloned());
    let raw = std::process::Command::new(&installation.launcher)
        .args(&arguments)
        .env("CARGO_TARGET_DIR", &target_directory)
        .current_dir(&crate_directory)
        .output()
        .expect("the pinned launcher runs a second time against the already-built crate");
    let summary = format!(
        "{}\n{}",
        String::from_utf8_lossy(&raw.stdout),
        String::from_utf8_lossy(&raw.stderr)
    )
    .lines()
    .find(|line| line.trim_start().starts_with("**") && line.contains("failed"))
    .map(str::trim)
    .unwrap_or("(no '** N of M failed' summary line found in the second run's output)")
    .to_owned();
    let _ = fs::remove_dir_all(crate_directory);
    (counterexample, summary)
}

// ---- reading the persisted schema back off disk ------------------------------------------------

/// Extracts `identity.arguments` from a persisted `kani-obligations/{module}.json` record
/// (`serde_json::Value`) into the `ObligationBinding`s `witness_schema` needs.
///
/// `KaniObligationIdentity`/`ObligationBinding` are `Serialize`-only in this crate (by design: the
/// generator persists, it never re-hydrates), so there is no typed `Deserialize` path back from
/// the file. This reads exactly the three fields the join uses (`identifier`, `role`,
/// `primitiveType`) generically instead, straight from the bytes on disk — nothing here is
/// hand-built, it is the generator's own persisted values, just not passed through a `Deserialize`
/// impl that does not exist.
fn persisted_arguments(record: &Value) -> Vec<ObligationBinding> {
    record["identity"]["arguments"]
        .as_array()
        .unwrap_or_else(|| panic!("persisted record has no identity.arguments array: {record}"))
        .iter()
        .map(|value| {
            let identifier = value["identifier"]
                .as_str()
                .expect("persisted binding has an identifier")
                .to_owned();
            let role = match value["role"]
                .as_str()
                .expect("persisted binding has a role")
            {
                "argument" => KaniBindingRole::Argument,
                "result" => KaniBindingRole::Result,
                other => panic!("unknown persisted role {other}"),
            };
            let primitive_type = match value["primitiveType"]
                .as_str()
                .expect("persisted binding has a primitiveType")
            {
                "boolean" => KaniPrimitiveType::Boolean,
                "i64" => KaniPrimitiveType::I64,
                other => panic!("unknown persisted primitiveType {other}"),
            };
            ObligationBinding {
                identifier,
                role,
                primitive_type,
                integer_bounds: None,
                dependencies: Vec::new(),
            }
        })
        .collect()
}

/// Real backend, real transcript, real persisted schema: negotiates the withdraw obligations,
/// publishes the postcondition harness's record to a real `kani-obligations/{module}.json` on
/// disk (`write_bundle_atomic`, the same call a real generation pipeline makes), runs the seeded
/// defect under the pinned Kani 0.67.0 backend to get a real falsification, reads the schema back
/// off disk, and decodes the transcript with it. Then mutates the file two ways — drops a binding,
/// and separately changes a declared primitive type (hence byte width) — and shows the decode
/// refuses by name both times, against the very same transcript that decoded cleanly.
///
/// Trace: IR-211.
#[test]
#[ignore = "kani lane: run serially through `cargo test --test kani_witness_join -- --ignored`"]
fn ir_211_real_falsification_decodes_against_the_persisted_schema_and_mutation_refuses() {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let pins = installation.observe().expect("the backend is measurable");
    assert_eq!(
        pins,
        KaniToolPins::pinned(),
        "the installed backend is the committed one"
    );

    let harnesses = withdraw_harnesses(&pins);
    // precondition, postcondition, invariant, in request order (see `withdraw_harnesses`).
    let postcondition_harness = &harnesses[1];

    // Publish the real record to a real `kani-obligations/{module}.json` on disk — the same call
    // (`ArtifactBundle::new` + `write_bundle_atomic`) a real generation pipeline makes. Nothing
    // downstream of this point reads the in-memory `KaniObligationHarness` again.
    let bundle = ArtifactBundle::new(vec![
        postcondition_harness.rust.clone(),
        postcondition_harness.record.clone(),
    ])
    .expect("the harness's own artifacts form a valid bundle");
    let publish_root = scratch("published");
    let published = publish_root.join("out");
    write_bundle_atomic(&bundle, &published)
        .expect("publishing the harness's own artifacts must not fail");
    let record_path = published.join(&postcondition_harness.record.path);
    assert!(
        record_path.to_str().unwrap().contains("kani-obligations/"),
        "the record must publish under kani-obligations/, got {}",
        record_path.display()
    );
    let record_bytes = fs::read_to_string(&record_path).expect("the record was published");
    let record: Value = serde_json::from_str(&record_bytes).expect("the record is valid JSON");
    let harness_symbol = record["identity"]["harnessSymbol"]
        .as_str()
        .expect("the record names its harness symbol")
        .to_owned();
    let module_symbol = record["identity"]["moduleSymbol"]
        .as_str()
        .expect("the record names its module symbol")
        .to_owned();

    // Real falsification: the seeded defect credits instead of debiting, so the postcondition
    // (`balance` never grows) is falsified under the real pinned backend.
    let (transcript, check_summary) = run_falsifying(&installation, postcondition_harness);
    assert!(transcript.contains("kani::concrete_playback_run"));
    assert!(transcript.contains(&harness_symbol));
    println!("IR-211 real Kani run check summary: {check_summary}");
    println!("IR-211 real transcript:\n{transcript}");

    // ---- green: the unmutated persisted schema decodes the real transcript -------------------
    let schema = witness_schema(&persisted_arguments(&record))
        .expect("every persisted argument binding maps to a witness binding");
    assert_eq!(
        schema.len(),
        2,
        "withdraw's union ABI has two arguments (amount_current, balance_pre): {schema:?}"
    );
    let witness = Witness::parse(&harness_symbol, &module_symbol, &transcript)
        .expect("a real transcript parses");
    let decoded = witness
        .decode(&schema)
        .expect("the generator's own persisted schema must decode its own real transcript");
    assert_eq!(decoded.len(), 2);
    assert!(
        decoded
            .iter()
            .all(|(_, value)| matches!(value, WitnessValue::Integer(_))),
        "{decoded:?}"
    );
    println!("IR-211 green decode against the persisted schema: {decoded:?}");

    // ---- red 1: drop a binding from the persisted schema -> arity mismatch -------------------
    let mut dropped = record.clone();
    dropped["identity"]["arguments"]
        .as_array_mut()
        .unwrap()
        .pop();
    let dropped_schema = witness_schema(&persisted_arguments(&dropped)).unwrap();
    assert_eq!(dropped_schema.len(), 1);
    let arity_refusal = witness
        .decode(&dropped_schema)
        .expect_err("a dropped binding must refuse, not silently decode fewer values");
    assert_eq!(
        arity_refusal.code, "kani_witness_arity_mismatch",
        "{arity_refusal:?}"
    );
    println!("IR-211 red (dropped binding) refusal: {arity_refusal:?}");

    // ---- red 2: change a declared primitive type (hence byte width) -> width mismatch --------
    let mut retyped = record.clone();
    let arguments = retyped["identity"]["arguments"].as_array_mut().unwrap();
    let first_identifier = arguments[0]["identifier"].as_str().unwrap().to_owned();
    arguments[0]["primitiveType"] = json!("boolean");
    let retyped_schema = witness_schema(&persisted_arguments(&retyped)).unwrap();
    assert_eq!(retyped_schema[0].value_type, WitnessValueType::Boolean);
    let width_refusal = witness
        .decode(&retyped_schema)
        .expect_err("an 8-byte i64 value against a declared 1-byte boolean must refuse");
    assert_eq!(
        width_refusal.code, "kani_witness_width_mismatch",
        "{width_refusal:?}"
    );
    assert_eq!(width_refusal.context, first_identifier);
    println!("IR-211 red (retyped binding) refusal: {width_refusal:?}");

    let _ = fs::remove_dir_all(publish_root);
}

//! Public generation controls for the general V2 clause integer-leaf policy.

#[path = "../checked_package_support/rekey.rs"]
#[allow(dead_code, clippy::duplicate_mod)]
mod rekey;

use ix_trace_rs::trace;
use quire_contract_codegen::generate_v2_clause_oracle;
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, ContractSourceV1,
};
use serde_json::{json, Value};

use super::kani_obligations_state_frame::native_twin::Twin;

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn public_canonical_i64_literals_choose_the_primitive_constructor() {
    let mut failures = Vec::new();
    for spelling in ["0", "1", "17", "42", "1000", "9223372036854775807"] {
        let (package, clause) =
            Twin::with_balance_condition(&format!("self.balance >= {spelling}"))
                .emitted_package("BalanceNeverDrops");
        let generated = generate_v2_clause_oracle(&package, &clause)
            .unwrap_or_else(|error| panic!("{spelling}: {error:?}"));
        assert_eq!(generated.descriptor.node_id, clause);
        assert_eq!(
            generated,
            generate_v2_clause_oracle(&package, &clause).unwrap()
        );
        assert!(generated
            .descriptor
            .operation_identities
            .iter()
            .any(|id| id == "quire.op.integer.ge"));
        if !generated
            .source
            .contains(&format!("Ok(rt::Integer::from({spelling}_i64))"))
        {
            failures.push(spelling);
        }
    }
    assert!(
        failures.is_empty(),
        "public constructor policy missing: {failures:?}"
    );
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn public_large_literals_keep_arbitrary_precision_parse() {
    for spelling in [
        "9223372036854775808",
        "170141183460469231731687303715884105727",
    ] {
        let (package, clause) = Twin::with_balance_condition(&format!("self.balance < {spelling}"))
            .emitted_package("BalanceNeverDrops");
        let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
        assert!(
            generated.source.contains(&format!(
                "\"{spelling}\".parse().map_err(|_| ClauseStop::InvalidConstant)"
            )),
            "large literal lost its parse fallback: {spelling}"
        );
    }
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn public_negated_positive_two_to_63_keeps_its_child_and_owner_call() {
    let (package, clause) = Twin::with_balance_condition("self.balance >= -9223372036854775808")
        .emitted_package("BalanceNeverDrops");
    let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
    assert!(generated
        .source
        .contains("\"9223372036854775808\".parse().map_err(|_| ClauseStop::InvalidConstant)"));
    assert!(!generated.source.contains("Integer::from(i64::MIN)"));
    assert!(generated
        .descriptor
        .operation_identities
        .iter()
        .any(|id| id == "quire.op.integer.negate"));
    assert!(generated.source.contains("IntegerArithmetic::Negate"));
}

// QSL rejects beyond-i128 source literals before CG. These separate IR-reader
// controls vary an admitted source package's literal, then ask the owner's public
// candidate/reader APIs for identities and admission. They are not QSL emission
// claims for those values and never alter the actual proof fixture.
fn reader_variant(
    condition: &str,
    rekey_nodes: bool,
    edit: impl FnOnce(&mut Value),
) -> (CheckedPackageV2ReadResult, CheckedNodeId) {
    let twin = Twin::with_balance_condition(condition);
    let (package, clause) = twin.emitted_package("BalanceNeverDrops");
    let mut wire = json!({
        "contract_version": "quire.checked-package/v2",
        "name": package.name(),
        "lock": package.lock(),
        "semantic_graph": package.graph(),
        "source_map": package.source_map(),
        "capability_report": package.capability_report(),
        "diagnostics": package.diagnostics(),
    });
    edit(&mut wire);
    let inputs = twin.state_clause_inputs(&package, &clause, "BalanceNeverDrops", (0, 0), (0, 0));
    let [document] = inputs.packages.as_slice() else {
        panic!("one original model document")
    };
    let digest = wire["lock"]["model_selections"][0]["digest"]
        .as_str()
        .unwrap()
        .to_owned();
    let mut evidence = CheckedPackageEvidence::new();
    evidence
        .insert_domain_package_document(digest, document.bytes.clone())
        .unwrap();
    evidence.support_feature("quire.value.complete/v1");
    if !rekey_nodes {
        rekey::refresh_identity(&mut wire);
        let bytes = serde_json::to_vec(&wire).unwrap();
        return (
            CheckedPackageV2::read(&bytes, CheckedPackageReadLimits::bounded(), &evidence),
            clause,
        );
    }
    let resolved = rekey::resolve_wire(&wire, &evidence, CheckedPackageReadLimits::bounded());
    let clause = resolved.ids.resolve(&clause);
    (resolved.outcome, clause)
}

fn reader_literal_with_identity_policy(
    spelling: &str,
    rekey_nodes: bool,
) -> (CheckedPackageV2ReadResult, CheckedNodeId) {
    fn replace(value: &mut Value, spelling: &str, changed: &mut usize) {
        if value["term"] == "literal" && value["value_kind"] == "integer" && value["value"] == "17"
        {
            value["value"] = json!(spelling);
            *changed += 1;
        } else {
            match value {
                Value::Array(items) => {
                    for value in items {
                        replace(value, spelling, changed);
                    }
                }
                Value::Object(items) => {
                    for value in items.values_mut() {
                        replace(value, spelling, changed);
                    }
                }
                _ => (),
            }
        }
    }
    reader_variant("self.balance >= 17", rekey_nodes, |wire| {
        let mut changed = 0;
        replace(&mut wire["semantic_graph"], spelling, &mut changed);
        assert_eq!(changed, 1, "one literal control");
    })
}

fn reader_literal(spelling: &str) -> (CheckedPackageV2ReadResult, CheckedNodeId) {
    reader_literal_with_identity_policy(spelling, true)
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn reader_admitted_signed_i64_leaves_choose_exact_construction() {
    let mut failures = Vec::new();
    for (spelling, expression) in [
        ("-9223372036854775808", "i64::MIN"),
        ("-9223372036854775807", "-9223372036854775807_i64"),
        ("-42", "-42_i64"),
        ("-1", "-1_i64"),
        ("0", "0_i64"),
        ("9223372036854775807", "9223372036854775807_i64"),
    ] {
        let (CheckedPackageV2ReadResult::Admitted(package), clause) = reader_literal(spelling)
        else {
            panic!("canonical literal must be admitted: {spelling}")
        };
        let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
        if !generated
            .source
            .contains(&format!("Ok(rt::Integer::from({expression}))"))
        {
            failures.push(spelling);
        }
    }
    assert!(
        failures.is_empty(),
        "admitted literal construction missing: {failures:?}"
    );
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn reader_admitted_large_values_keep_general_parse_without_narrowing() {
    for spelling in [
        "-9223372036854775809",
        "9223372036854775808",
        "-170141183460469231731687303715884105728",
        "170141183460469231731687303715884105727",
        "-170141183460469231731687303715884105729",
        "170141183460469231731687303715884105728",
        "340282366920938463463374607431768211456",
    ] {
        let (CheckedPackageV2ReadResult::Admitted(package), clause) = reader_literal(spelling)
        else {
            panic!("canonical large literal must be admitted: {spelling}")
        };
        let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
        assert!(generated.source.contains(&format!(
            "\"{spelling}\".parse().map_err(|_| ClauseStop::InvalidConstant)"
        )));
    }
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn noncanonical_literal_admission_and_runtime_fallback_remain_distinct() {
    for spelling in ["+1", "01", "-0", " 1", "1 ", "1a", "", "--1"] {
        let (stale, _) = reader_literal_with_identity_policy(spelling, false);
        let CheckedPackageV2ReadResult::Refused(refusal) = stale else {
            panic!("a changed literal under its old identity must refuse")
        };
        assert_eq!(
            refusal.cause,
            Some(quire_contract_model::CheckedPackageRefusalCause::StaleNodeKey)
        );
        let (result, clause) = reader_literal(spelling);
        let CheckedPackageV2ReadResult::Admitted(package) = result else {
            panic!("the existing general literal reader admits this rekeyed spelling: {spelling:?}")
        };
        let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
        assert!(generated.source.contains(&format!(
            "\"{spelling}\".parse().map_err(|_| ClauseStop::InvalidConstant)"
        )));
        assert!(!generated.source.contains("Integer::from("));
        eprintln!("literal-reader {spelling:?}: stale identity -> StaleNodeKey; owner-rekeyed -> Admitted; generated -> original parse/InvalidConstant fallback");
    }
}

// This is a separately labelled IR-reader control, not a QSL-emitted bounded
// arithmetic claim. Its result type is an existing checked integer range.
fn bounded_addition_reader_attempt(coherent: bool) -> (CheckedPackageV2ReadResult, CheckedNodeId) {
    let (result, clause) = reader_variant("(self.balance + 1) > 1000", true, |wire| {
        let nodes = wire["semantic_graph"]["nodes"].as_array_mut().unwrap();
        let bound = nodes
            .iter()
            .find(|node| {
                node["node_tag"] == "bounded_domain" && node["semantic_form"] == "integer_range"
            })
            .expect("original declared range")["node_id"]
            .clone();
        let mut changed = 0;
        for node in nodes {
            if node["body"]["operation"]["identity"] == "quire.op.integer.add" {
                node["semantic_type"] = bound.clone();
                if coherent {
                    node["body"]["result_type"] = bound.clone();
                }
                changed += 1;
            }
        }
        assert_eq!(changed, 1, "one arithmetic result type control");
    });
    (result, clause)
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn incoherent_arithmetic_result_types_keep_the_owner_type_refusal() {
    let (result, _) = bounded_addition_reader_attempt(false);
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("incoherent node/body result types must not admit")
    };
    assert_eq!(
        refusal.code,
        quire_contract_model::CheckedPackageRefusalCode::IllTyped
    );
    assert_eq!(
        refusal.cause,
        Some(quire_contract_model::CheckedPackageRefusalCause::TypeMismatch)
    );
    eprintln!("incoherent result-type control: public owner-candidate/reader -> IllTyped/TypeMismatch; coherent result types are tested separately");
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn original_parameter_precondition_regenerates_with_unchanged_proof_semantics() {
    use quire_contract_codegen::{
        negotiate_kani_obligations, KaniObligationOutcome, KaniObligationRequest, KaniSolver,
        ObligationItem,
    };
    let (package, clause) =
        Twin::with_bounded_other_precondition().emitted_package("OtherBelowLimit");
    let oracle = generate_v2_clause_oracle(&package, &clause).unwrap();
    assert_eq!(oracle.descriptor.node_id, clause);
    assert_eq!(
        oracle.descriptor.operation_identities,
        ["quire.op.integer.lt"]
    );
    let item = ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        dependencies: None,
    };
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items: std::slice::from_ref(&item),
        subject_path: "crate::request::unrelated",
        unwind: 8,
    })
    .unwrap();
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the original parameter precondition is emitted")
    };
    let [harness] = v2_clause_harnesses.as_slice() else {
        panic!("one original clause harness")
    };
    assert_eq!(
        harness.identity.package,
        ContractSourceV1 {
            name: package.name().into(),
            lock: package.lock().clone(),
        }
    );
    assert_eq!(harness.identity.clause, clause);
    assert_eq!(harness.identity.unwind, 8);
    assert_eq!(harness.identity.solver, KaniSolver::Cadical);
    assert_eq!(
        harness.identity.ceilings.memory_bytes.get(),
        16 * 1024 * 1024 * 1024
    );
    assert_eq!(harness.identity.ceilings.wall_clock.as_secs(), 600);
    let [input] = harness.identity.arguments.as_slice() else {
        panic!("one symbolic input")
    };
    assert_eq!(input.identifier, "parameter_other");
    assert_eq!((input.minimum, input.maximum), (0, 1000));
    let source = &harness.rust.contents;
    assert!(source.contains(&oracle.source));
    syn::parse_file(source).expect("the public proof is valid Rust syntax");
    assert_eq!(source.matches("kani::any()").count(), 1);
    assert_eq!(source.matches("kani::assume(").count(), 1);
    assert!(source.contains("parameter_other >= 0_i64 && parameter_other <= 1000_i64"));
    assert_eq!(source.matches("kani::cover!(").count(), 1);
    assert!(source.contains("matches!(outcome, Ok(rt::Outcome::Completed(true)))"));
    assert!(source
        .contains("rt::OrderingOperator::Less, rt::OrderedOperands::Integers(left, right), meter"));
    assert_eq!(source.matches(": u64::MAX,").count(), 10);
    eprintln!("PUBLIC_PARAMETER_PROOF_BEGIN\n{source}PUBLIC_PARAMETER_PROOF_END");
    eprintln!(
        "PUBLIC_PARAMETER_RECORD_BEGIN\n{}\nPUBLIC_PARAMETER_RECORD_END",
        harness.record.contents
    );
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn coherent_bounded_arithmetic_control_is_decided_by_the_owner_reader() {
    let (result, clause) = bounded_addition_reader_attempt(true);
    let CheckedPackageV2ReadResult::Admitted(package) = result else {
        panic!("coherent type positions reader result: {result:?}")
    };
    let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
    assert!(generated.source.contains("Some(&domain)"));
    assert!(generated.source.contains("IntegerArithmetic::Add"));
    assert_eq!(generated.descriptor.inputs.len(), 1);
    assert_eq!(
        (
            generated.descriptor.inputs[0].lower,
            generated.descriptor.inputs[0].upper
        ),
        (0, 1000)
    );
    eprintln!("coherent bounded-result fixture: node.semantic_type AND body.result_type use originalInt[0,1000]; owner-candidate/rekey/read Admitted; public bound oracle generated; native refusal assertion pending");
}

#[trace("TC-024", "FR-014-AC-53")]
#[test]
fn reader_admitted_literal_fallbacks_and_bounded_stops_preserve_native_accounting() {
    use std::{fmt::Write as _, fs, process::Command};

    use quire_contract_codegen::{EXACT_DEPENDENCY_SOURCE, SEMANTIC_VALUE_DEPENDENCY_SOURCE};

    use super::{scratch_crate, v2_clause_integer_literal_agreement::NATIVE_SUPPORT};

    let scratch = scratch_crate::TemporaryDirectory::new("v2-controlled-literal-native");
    let mut source = String::from(NATIVE_SUPPORT);
    let cases = [
        ("-9223372036854775808", Some("true")),
        ("-9223372036854775807", Some("true")),
        ("-42", Some("true")),
        ("-1", Some("true")),
        ("0", Some("true")),
        ("1", Some("x >= 1")),
        ("17", Some("x >= 17")),
        ("42", Some("x >= 42")),
        ("9223372036854775807", Some("false")),
        ("-9223372036854775809", Some("true")),
        ("9223372036854775808", Some("false")),
        ("-170141183460469231731687303715884105728", Some("true")),
        ("170141183460469231731687303715884105727", Some("false")),
        ("-170141183460469231731687303715884105729", Some("true")),
        ("170141183460469231731687303715884105728", Some("false")),
        ("340282366920938463463374607431768211456", Some("false")),
        ("+1", None),
        ("01", None),
        ("-0", None),
        (" 1", None),
        ("1 ", None),
        ("1a", None),
        ("", None),
        ("--1", None),
    ];
    for (index, (spelling, expected)) in cases.iter().enumerate() {
        let (CheckedPackageV2ReadResult::Admitted(package), clause) = reader_literal(spelling)
        else {
            panic!("reader-controlled literal was not admitted: {spelling:?}")
        };
        let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
        writeln!(
            source,
            "mod controlled_{index} {{\n{}\n}}",
            generated.source
        )
        .unwrap();
        append_native_observer(&mut source, index);
        let check = if let Some(expected) = expected {
            format!("exercise({spelling:?}, &expression, |x| {{ let _ = x; {expected} }}, false, invoke_{index});")
        } else {
            format!(
                r#"
    for x in [0, 1, 4, 5, 6, 1000] {{
        let baseline = agree({spelling:?}, &expression, x, &invoke_{index}, generous(), None);
        assert_eq!(baseline.outcome, Observation::InvalidConstant);
        assert!(baseline.charges.is_empty());
        assert_eq!(baseline.counters, [0; 10]);
        for kind in rt::LimitKind::ALL {{
            let mut limits = generous();
            set_limit(&mut limits, kind, 0);
            assert_eq!(agree({spelling:?}, &expression, x, &invoke_{index}, limits, None), baseline);
        }}
        let denial = rt::InjectedDenial {{ point: rt::ChargePoint::OrderingOperands, occurrence: NonZeroU64::new(1).unwrap() }};
        assert_eq!(agree({spelling:?}, &expression, x, &invoke_{index}, generous(), Some(denial)), baseline);
    }}
"#
            )
        };
        writeln!(
            source,
            r#"
#[ix_trace_rs::trace("TC-024", "FR-014-AC-53")]
#[test]
fn controlled_{index}() {{
    let expression = c("ge", input(), lit({spelling:?}));
    {check}
}}
"#
        )
        .unwrap();
    }
    let index = cases.len();
    let (CheckedPackageV2ReadResult::Admitted(package), clause) =
        bounded_addition_reader_attempt(true)
    else {
        panic!("coherent bounded control is admitted by the owner")
    };
    let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
    writeln!(
        source,
        "mod controlled_{index} {{\n{}\n}}",
        generated.source
    )
    .unwrap();
    append_native_observer(&mut source, index);
    writeln!(
        source,
        r#"
#[ix_trace_rs::trace("TC-024", "FR-014-AC-53")]
#[test]
fn controlled_{index}() {{
    let expression = c("gt", ab("add", input(), lit("1"), "0", "1000"), lit("1000"));
    exercise_bounded("reader-admitted bounded Add", &expression, invoke_{index});
}}
"#
    )
    .unwrap();
    let manifest = format!(
        "[package]\nname = \"controlled-literal-native\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nquire-exact = {{ {EXACT_DEPENDENCY_SOURCE}, features = [\"test-support\"] }}\nquire-semantic-value = {{ {SEMANTIC_VALUE_DEPENDENCY_SOURCE} }}\n{}\nix-trace-rs = {{ git = \"https://github.com/agent-ix/ix-trace-rs\", branch = \"main\" }}\n[workspace]\n",
        scratch_crate::runtime_dependency(&["exact"]),
    );
    scratch_crate::write_manifest(&scratch.path, &manifest);
    fs::write(scratch.path.join("src/lib.rs"), source).unwrap();
    scratch_crate::resolve_seeded_lock(&scratch.path);
    let literal_cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = Command::new(literal_cargo)
        .args([
            "test",
            "--locked",
            "--offline",
            "-j2",
            "--lib",
            "--",
            "--test-threads=1",
            "--nocapture",
        ])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(&scratch.path)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{stdout}\n{stderr}");
    assert!(
        output.status.success(),
        "controlled native agreement failed: {stdout}\n{stderr}"
    );
    for index in 0..=cases.len() {
        assert!(
            stdout
                .lines()
                .any(|line| line == format!("test controlled_{index} ... ok")),
            "missing native control_{index}"
        );
    }
}

fn append_native_observer(source: &mut String, index: usize) {
    use std::fmt::Write as _;

    writeln!(source, r#"
fn invoke_{index}(x: u64, meter: &mut rt::Meter) -> Observation {{
    let input: rt::Integer = x.to_string().parse().expect("canonical field input");
    match controlled_{index}::oracle_clause(&input, meter) {{
        Ok(outcome) | Err(controlled_{index}::ClauseStop::Runtime(outcome)) => Observation::Runtime(outcome),
        Err(controlled_{index}::ClauseStop::InvalidConstant) => Observation::InvalidConstant,
        Err(controlled_{index}::ClauseStop::Oracle(stop)) => match stop {{
            controlled_{index}::OracleStop::IllTyped(cause) => Observation::IllTyped(cause),
            controlled_{index}::OracleStop::InvalidConstant => Observation::InvalidConstant,
            controlled_{index}::OracleStop::OperandWidth => Observation::OperandWidth,
        }}
    }}
}}
"#).unwrap();
}

//! Public generation controls for the general V2 clause integer-leaf policy.

#[path = "../checked_package_support/rekey.rs"]
#[allow(dead_code, clippy::duplicate_mod)]
mod rekey;

use ix_trace_rs::trace;
use quire_contract_codegen::generate_v2_clause_oracle;
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult,
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
fn reader_literal_with_identity_policy(
    spelling: &str,
    rekey_nodes: bool,
) -> (CheckedPackageV2ReadResult, CheckedNodeId) {
    let twin = Twin::with_balance_condition("self.balance >= 17");
    let (package, clause) = twin.emitted_package("BalanceNeverDrops");
    let mut wire = json!({
        "contract_version": "quire.checked-package/v2",
        "identity_preimage": package.identity_preimage(),
        "package_id": package.package_id(),
        "lock": package.lock(),
        "semantic_graph": package.graph(),
        "source_map": package.source_map(),
        "capability_report": package.capability_report(),
        "diagnostics": package.diagnostics(),
    });
    let mut changed = 0;
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
    replace(&mut wire["semantic_graph"], spelling, &mut changed);
    assert_eq!(changed, 1, "one literal control");
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

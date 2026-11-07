// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Genuine producer forms at the strict-reader and native scalar seams.

use ix_trace_rs::trace;
use qsl_replay::{
    compile_package, CompileUnit, DependencyInput, ReplayLimits, SourceIdentity, StageLimits,
};
use quire_contract_codegen::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, ClaimDisposition, ExactScalarOracles,
    OperationProvenance,
};
use quire_contract_model::{
    CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult,
    CheckedTerm,
};

fn emitted(declarations: &str) -> Vec<u8> {
    let source = format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n{declarations}\n");
    let emitted = compile_package(
        CompileUnit {
            source: SourceIdentity::new("test", "scalar-source-forms.native"),
            package_name: "scalar-source-forms".to_owned(),
            path: "scalar-source-forms.native",
            bytes: source.as_bytes(),
        },
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("public QSL facade emits the authored source");
    emitted.bytes().to_vec()
}

fn compiled(declarations: &str) -> CheckedPackageV2 {
    admit(&emitted(declarations))
}

fn admit(bytes: &[u8]) -> CheckedPackageV2 {
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(bytes, CheckedPackageReadLimits::bounded(), &evidence) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("strict IR reader must admit unchanged genuine QSL bytes: {other:?}"),
    }
}

fn generated(package: &CheckedPackageV2, identity: &str) -> (ExactScalarOracles, String) {
    let nodes = package.graph().nodes.iter().filter(|node| matches!(&node.body, CheckedTerm::Application { operation, .. } if operation.identity() == Some(identity))).collect::<Vec<_>>();
    let [node] = nodes.as_slice() else {
        panic!("one genuine {identity} application: {nodes:?}");
    };
    let items = derive_exact_scalar_items(package, &[node.node_id.clone()])
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("derive genuine source operation");
    let oracles =
        generate_exact_scalar_oracles(package, &items).expect("generate genuine source operation");
    let [claim] = oracles.claim_map.items.as_slice() else {
        panic!("one selected source claim");
    };
    let ClaimDisposition::Generated(generated) = &claim.result else {
        panic!("genuine source form must generate: {:?}", claim.result);
    };
    assert_eq!(claim.operation.provenance, OperationProvenance::IrConfirmed);
    assert_eq!(claim.operation.identity, identity);
    assert_eq!(generated.node_id, node.node_id);
    let symbol = generated.symbol.clone();
    (oracles, symbol)
}

#[trace("TC-024", "FR-014-AC-67", "FR-014-AC-69")]
#[test]
fn genuine_ieee_equality_and_intrinsics_generate() {
    let mut native = String::new();
    let mut calls = String::new();
    let mut manifest = String::new();
    let mut ordinal = 0;
    for (expression, identity) in [
        ("a = b", "numeric_equal"),
        ("quire::value::ieee::numericEqual(a, b)", "numeric_equal"),
        ("quire::value::ieee::totalOrder(a, b)", "total_order"),
        ("quire::value::ieee::bitIdentical(a, b)", "bit_identical"),
    ] {
        for width in ["Float32", "Float64"] {
            let package = compiled(&format!("function subject using v(a: {width}[nearest-even], b: {width}[nearest-even]): Boolean pure {{ {expression} }}"));
            let identity = format!("quire.op.ieee.{identity}");
            let (oracles, symbol) = generated(&package, &identity);
            manifest = oracles
                .artifacts
                .iter()
                .find(|artifact| artifact.path == "Cargo.toml")
                .unwrap()
                .contents
                .clone();
            let claim = match &oracles.claim_map.items[0].result {
                ClaimDisposition::Generated(claim) => claim,
                other => panic!("generated claim: {other:?}"),
            };
            assert_eq!(claim.semantic_form, "call");
            native.push_str(&format!(
                "mod case_{ordinal} {{ {} }}\n",
                claim.oracle_source
            ));
            let comparison = match identity.as_str() {
                "quire.op.ieee.numeric_equal" => "NumericEqual",
                "quire.op.ieee.total_order" => "TotalOrder",
                "quire.op.ieee.bit_identical" => "BitIdentical",
                _ => unreachable!("test cases name only IEEE relations"),
            };
            calls.push_str(&format!("check(|a, b, meter| crate::case_{ordinal}::{symbol}(a, b, meter).map_err(|stop| format!(\"{{stop:?}}\")), rt::IeeeComparison::{comparison}, {});\n", width == "Float32"));
            ordinal += 1;
        }
    }
    native.push_str(&native_ieee_checks(&calls));
    run_native("ieee-source-forms", &manifest, &native, "");
}

#[trace("TC-024", "FR-014-AC-66", "FR-014-AC-69")]
#[test]
fn genuine_carried_quantity_u01_conversion_generates() {
    let package = compiled("dimension Length;\nunit m : Length = rational(1, 1);\nunit cm : Length = rational(1, 100) * m;\nfunction subject using v(a: Quantity<cm, Int[0, 1000]>): Quantity<m> pure { convert<Quantity<m>>(a) }");
    let (oracles, symbol) = generated(&package, "quire.op.quantity.convert");
    let manifest = &oracles
        .artifacts
        .iter()
        .find(|artifact| artifact.path == "Cargo.toml")
        .unwrap()
        .contents;
    let source = &oracles
        .artifacts
        .iter()
        .find(|artifact| artifact.path == "src/lib.rs")
        .unwrap()
        .contents;
    let native = format!("{source}\n{}", native_quantity_checks(&symbol));
    run_native("quantity-source-forms", manifest, &native, "qsl-replay = { git = \"https://github.com/agent-ix/quire-spec-language\", branch = \"main\" }\n");
}

fn run_native(label: &str, manifest: &str, source: &str, dependencies: &str) {
    use std::{fs, process::Command};
    let scratch = crate::scratch_crate::TemporaryDirectory::new(label);
    fs::create_dir_all(scratch.path.join("src")).unwrap();
    let manifest = format!("{manifest}\n[dev-dependencies]\nquire-exact = {{ git = \"https://github.com/agent-ix/quire-exact\", branch = \"main\", features = [\"test-support\"] }}\n{dependencies}");
    fs::write(scratch.path.join("Cargo.toml"), manifest).unwrap();
    fs::write(scratch.path.join("src/lib.rs"), source).unwrap();
    crate::scratch_crate::resolve_seeded_lock(&scratch.path);
    let _build = crate::scratch_crate::lock_agreement_build();
    let output = Command::new(env!("CARGO"))
        .args(["test", "--locked", "--offline", "--lib"])
        .env("CARGO_TARGET_DIR", crate::scratch_crate::agreement_target())
        .current_dir(&scratch.path)
        .output()
        .expect("compile and execute generated native scalar oracles");
    assert!(
        output.status.success(),
        "native scalar execution failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("test native::values_and_ledgers ... ok"),
        "native value and ledger assertions must actually execute"
    );
}

fn native_ieee_checks(calls: &str) -> String {
    let support = r#"
#[cfg(test)]
mod native {
use quire_exact as rt;
const LIMITS: rt::ScalarLimits = rt::ScalarLimits {
    integer_bits: 4096, decimal_digits: 4096, scale_expansion: 4096,
    text_input_bytes: 4096, text_scalars: 4096, normalized_scalars: 4096,
    unit_edges: 4096, value_occurrences: 4096, work_units: 4096, result_units: 4096,
};
fn check(oracle: impl Fn(rt::IeeeValue, rt::IeeeValue, &mut rt::Meter) -> Result<rt::Outcome<bool>, String>, comparison: rt::IeeeComparison, narrow: bool) {
    let bits = if narrow {
        [0, 0x80000000, 0x3f800000, 0x40000000, 0x7f800000, 0xff800000, 0x7fc00001, 0x7fc00002]
    } else {
        [0, 0x8000000000000000, 0x3ff0000000000000, 0x4000000000000000, 0x7ff0000000000000, 0xfff0000000000000, 0x7ff8000000000001, 0x7ff8000000000002]
    };
    let value = |bits| if narrow { rt::IeeeValue::binary32(u32::try_from(bits).unwrap()) } else { rt::IeeeValue::binary64(bits) };
    for (a, b, equal, ordered, identical) in [
        (0, 1, true, false, false), (1, 0, true, true, false),
        (2, 2, true, true, true), (2, 3, false, true, false),
        (3, 2, false, false, false), (4, 4, true, true, true),
        (5, 4, false, true, false), (6, 6, false, true, true),
        (6, 7, false, true, false), (7, 6, false, false, false),
    ] {
        let expected = match comparison { rt::IeeeComparison::NumericEqual => equal, rt::IeeeComparison::TotalOrder => ordered, rt::IeeeComparison::BitIdentical => identical, _ => panic!("unexpected owner comparison") };
        for limits in [LIMITS, rt::ScalarLimits { work_units: 0, ..LIMITS }, rt::ScalarLimits { result_units: 0, ..LIMITS }] {
            let mut direct_meter = rt::Meter::new(limits);
            let mut generated_meter = rt::Meter::new(limits);
            let direct = rt::compare_ieee(comparison, value(bits[a]), value(bits[b]), &mut direct_meter).unwrap();
            let actual = oracle(value(bits[a]), value(bits[b]), &mut generated_meter).unwrap();
            assert_eq!(actual, direct);
            assert_eq!(generated_meter.admitted_charges(), direct_meter.admitted_charges());
            for kind in rt::LimitKind::ALL { assert_eq!(generated_meter.consumed(kind), direct_meter.consumed(kind)); }
            if limits == LIMITS {
                assert_eq!(actual, rt::Outcome::Completed(expected), "{comparison:?} ({a}, {b})");
                assert!(!generated_meter.admitted_charges().is_empty());
            } else { assert!(matches!(actual, rt::Outcome::Incomplete(_))); }
        }
    }
}
#[test]
fn values_and_ledgers() {
"#;
    format!("{support}{calls}\n}}\n}}\n")
}

#[trace("TC-024", "FR-014-AC-67", "FR-014-AC-68")]
#[test]
fn ieee_extension_preserves_binary_support_and_refuses_other_shapes() {
    use crate::exact_scalar_generation::package::read_mutated_wire;
    use quire_contract_codegen::{
        ExactScalarItem, ExactScalarOperation, ExactScalarRefusal, ScalarForm,
    };
    use quire_exact::{IeeeComparison, IeeeWidth};
    let bytes = emitted("function subject using v(a: Float64[nearest-even], b: Float64[nearest-even]): Boolean pure { a = b }\nfunction sibling using v(a: Float64[nearest-even], b: Float64[nearest-even]): Boolean pure { quire::value::ieee::totalOrder(a, b) }");
    let original: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    for form in ["binary", "unary", "conversion"] {
        let mut wire = original.clone();
        let node = wire["semantic_graph"]["nodes"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|node| node["body"]["operation"]["identity"] == "quire.op.ieee.numeric_equal")
            .unwrap();
        node["semantic_form"] = form.into();
        let id = serde_json::from_value(node["node_id"].clone()).unwrap();
        let CheckedPackageV2ReadResult::Admitted(package) = read_mutated_wire(wire) else {
            panic!("IR independently admits expression forms with catalogued call bodies");
        };
        let item = ExactScalarItem {
            node_id: id,
            operation: ExactScalarOperation::IeeeComparison {
                comparison: IeeeComparison::NumericEqual,
                width: IeeeWidth::Binary64,
            },
        };
        let output = generate_exact_scalar_oracles(&package, &[item]).unwrap();
        let [claim] = output.claim_map.items.as_slice() else {
            panic!("one claim");
        };
        if form == "binary" {
            assert!(matches!(claim.result, ClaimDisposition::Generated(_)));
            assert_eq!(claim.operation.provenance, OperationProvenance::IrConfirmed);
        } else {
            assert!(
                matches!(&claim.result, ClaimDisposition::Refused { refusal: ExactScalarRefusal::FormMismatch { expected: "binary", found } } if found == form)
            );
        }
    }
    for mutation in [
        "operator", "arity", "operand", "result", "identity", "law", "mode",
    ] {
        let mut wire = original.clone();
        let nodes = wire["semantic_graph"]["nodes"].as_array_mut().unwrap();
        let boolean = nodes
            .iter()
            .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "boolean")
            .unwrap()["node_id"]
            .clone();
        let position = nodes
            .iter()
            .position(|node| node["body"]["operation"]["identity"] == "quire.op.ieee.numeric_equal")
            .unwrap();
        let node = &mut nodes[position];
        match mutation {
            "operator" => node["body"]["operator"] = "binary".into(),
            "arity" => {
                node["body"]["arguments"].as_array_mut().unwrap().pop();
            }
            "operand" => {
                node["body"]["arguments"][0] = serde_json::json!({"term": "literal", "type_id": boolean, "value_kind": "boolean", "value": "true"})
            }
            "result" => {
                node["body"]["result_type"] = node["body"]["arguments"][0]["target"].clone()
            }
            "identity" => node["body"]["operation"]["identity"] = "quire.op.integer.eq".into(),
            "law" => node["body"]["operation"]["laws"] = serde_json::json!([]),
            "mode" => {
                node["body"]["operation"]["mode"] =
                    serde_json::json!({"kind": "rounding", "value": "nearest-even"})
            }
            _ => unreachable!("fixed mutation cases"),
        }
        assert!(
            matches!(
                read_mutated_wire(wire),
                CheckedPackageV2ReadResult::Refused(_)
            ),
            "strict reader retains {mutation} refusal"
        );
    }
    let package = admit(&bytes);
    let node = package.graph().nodes.iter().find(|node| matches!(&node.body, CheckedTerm::Application { operation, .. } if operation.identity() == Some("quire.op.ieee.numeric_equal"))).unwrap();
    let sibling = package.graph().nodes.iter().find(|node| matches!(&node.body, CheckedTerm::Application { operation, .. } if operation.identity() == Some("quire.op.ieee.total_order"))).unwrap();
    let good = derive_exact_scalar_items(&package, &[sibling.node_id.clone()])
        .pop()
        .unwrap()
        .unwrap();
    let output = generate_exact_scalar_oracles(
        &package,
        &[
            ExactScalarItem {
                node_id: node.node_id.clone(),
                operation: ExactScalarOperation::IeeeComparison {
                    comparison: IeeeComparison::NumericEqual,
                    width: IeeeWidth::Binary32,
                },
            },
            good,
        ],
    )
    .unwrap();
    assert!(matches!(
        output
            .claim_map
            .items
            .iter()
            .find(|claim| claim.node_id == node.node_id)
            .unwrap()
            .result,
        ClaimDisposition::Refused {
            refusal: ExactScalarRefusal::OperandTypeMismatch {
                expected: ScalarForm::Float32,
                ..
            }
        }
    ));
    assert!(
        matches!(
            output
                .claim_map
                .items
                .iter()
                .find(|claim| claim.node_id == sibling.node_id)
                .unwrap()
                .result,
            ClaimDisposition::Generated(_)
        ),
        "a refused selected shape leaves its supported sibling generated"
    );
}

fn native_quantity_checks(symbol: &str) -> String {
    let support = r#"
#[cfg(test)]
mod native {
use quire_exact as rt;
use quire_semantic_value::quantity as owner;
#[test]
fn values_and_ledgers() {
    let source = "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\ndimension Length;\nunit m : Length = rational(1, 1);\nunit cm : Length = rational(1, 100) * m;\nfunction subject using v(a: Quantity<cm, Int[0, 1000]>): Quantity<m> pure { convert<Quantity<m>>(a) }";
    let admitted = qsl_replay::compile_admitted(qsl_replay::CompileUnit {
        source: qsl_replay::SourceIdentity::new("test", "scalar-source-forms.native"),
        package_name: "scalar-source-forms".to_owned(), path: "scalar-source-forms.native", bytes: source.as_bytes(),
    }, [], &qsl_replay::DependencyInput::default(), qsl_replay::StageLimits::default(), qsl_replay::ReplayLimits::default()).unwrap();
    let context = admitted.context();
    let unit = |name: &str| context.units().find(|unit| unit.qualified_name().last().map(|part| part.as_str()) == Some(name)).unwrap();
    let cm = unit("cm");
    let m = owner::IdentifiedUnit::new(unit("m").quantity_unit().clone()).unwrap();
    let quantity = rt::Quantity::new(rt::Rational::from_integer(rt::Integer::from(250)), cm.id());
    let operand = context.unit_table().resolve(&quantity).unwrap();
    let limits = rt::ScalarLimits { integer_bits: 4096, decimal_digits: 4096, scale_expansion: 4096, text_input_bytes: 4096, text_scalars: 4096, normalized_scalars: 4096, unit_edges: 4096, value_occurrences: 4096, work_units: 4096, result_units: 4096 };
    for budget in [limits, rt::ScalarLimits { work_units: 0, ..limits }, rt::ScalarLimits { result_units: 0, ..limits }] {
        let mut direct_meter = rt::Meter::new(budget);
        let mut generated_meter = rt::Meter::new(budget);
        let direct = owner::convert_quantity(operand, m.unit(), &owner::QuantityTarget::Exact, &mut direct_meter).unwrap();
        let actual = ORACLE(operand, &m, &mut generated_meter).unwrap();
        match (actual, direct) {
            (rt::Outcome::Completed(actual), rt::Outcome::Completed(direct)) => {
                let expected = rt::Rational::new(rt::Integer::from(5), rt::Integer::from(2)).unwrap();
                assert_eq!(actual.magnitude(), &expected);
                assert_eq!(actual.unit(), &m.id());
                assert_eq!(direct.value(), &owner::ConvertedValue::Exact(expected));
                assert_eq!(direct.source(), &quantity);
                assert_eq!(direct.unit(), m.unit());
                assert_eq!(budget, limits);
            }
            (rt::Outcome::Incomplete(actual), rt::Outcome::Incomplete(direct)) => { assert_eq!(actual, direct); assert_ne!(budget, limits); }
            other => panic!("generated and owner outcomes must agree: {other:?}"),
        }
        assert_eq!(generated_meter.admitted_charges(), direct_meter.admitted_charges());
        for kind in rt::LimitKind::ALL { assert_eq!(generated_meter.consumed(kind), direct_meter.consumed(kind)); }
    }
}
}
"#;
    support.replace("ORACLE", &format!("crate::{symbol}"))
}

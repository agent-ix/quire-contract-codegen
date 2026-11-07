// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Genuine QSL operand bounds at the scalar derivation/generation boundary.

use std::{fmt::Write as _, fs, process::Command};

use ix_trace_rs::trace;
use qsl_replay::{
    compile_package, CompileUnit, DependencyInput, ReplayLimits, SourceIdentity, StageLimits,
};
use quire_contract_codegen::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, ClaimDisposition, ExactScalarItem,
    ExactScalarOperation, ExactScalarRefusal, OperationProvenance, EXACT_DEPENDENCY_SOURCE,
};
use quire_contract_model::{
    CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult,
};
use quire_exact::{IntegerDomain, IntegerInterval};

use crate::{exact_function_agreement::body_id, scratch_crate};

fn compiled(parameters: &str, result: &str, expression: &str) -> CheckedPackageV2 {
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction subject using v({parameters}): {result} pure {{ {expression} }}\n"
    );
    let compiled = compile_package(
        CompileUnit {
            source: SourceIdentity::new("test", "integer-division-bounds"),
            package_name: "integer-division-bounds".to_owned(),
            path: "integer-division-bounds.native",
            bytes: source.as_bytes(),
        },
        [],
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("QSL compiles genuine integer source");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(
        compiled.bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("IR admits the QSL-emitted bytes: {other:?}"),
    }
}

fn derived(package: &CheckedPackageV2) -> ExactScalarItem {
    let node = body_id(package, "subject");
    derive_exact_scalar_items(package, &[node])
        .remove(0)
        .expect("finite operands give an honest derived result interval")
}

fn interval(lower: &str, upper: &str) -> IntegerInterval {
    IntegerInterval::new(lower.parse().unwrap(), upper.parse().unwrap()).unwrap()
}

fn domain(item: &ExactScalarItem) -> &IntegerDomain {
    match &item.operation {
        ExactScalarOperation::IntegerDivision { domain, .. }
        | ExactScalarOperation::IntegerModulo { domain } => domain,
        other => panic!("expected an integer division/modulo operation: {other:?}"),
    }
}

#[trace("TC-024", "FR-014-AC-18")]
#[test]
fn bounded_division_and_modulo_derive_sound_domains_and_execute_generated_values() {
    let scratch = scratch_crate::TemporaryDirectory::new("integer-division-bounds");
    let mut native = String::from(NATIVE_SUPPORT);
    let cases = [
        (
            "x: Int[-9, 9], y: Int[1, 4]",
            "x div y",
            "-9",
            "9",
            "-7",
            "3",
            "-2",
        ),
        (
            "x: Int[-9, 9], y: Int[-4, -1]",
            "x div y",
            "-9",
            "9",
            "7",
            "-3",
            "-2",
        ),
        (
            "x: Int[-9, 9], y: Int[1, 4]",
            "x mod y",
            "0",
            "3",
            "-7",
            "3",
            "2",
        ),
        (
            "x: Int[-9, 9], y: Int[-4, -1]",
            "x mod y",
            "0",
            "3",
            "-7",
            "-3",
            "2",
        ),
        (
            "x: Int[-9223372036854775808, 9223372036854775807], y: Int[-1, -1]",
            "x div y",
            "-9223372036854775808",
            "9223372036854775808",
            "-9223372036854775808",
            "-1",
            "9223372036854775808",
        ),
        (
            "x: Int[-9, 9], y: Int[-9223372036854775808, -1]",
            "x mod y",
            "0",
            "9223372036854775807",
            "-1",
            "-9223372036854775808",
            "9223372036854775807",
        ),
        (
            "x: Int[0, 0], y: Int[1, 1]",
            "x div y",
            "0",
            "0",
            "0",
            "1",
            "0",
        ),
        (
            "x: Int[-9, 9], y: Int[-1, -1]",
            "x mod y",
            "0",
            "0",
            "-9",
            "-1",
            "0",
        ),
    ];
    for (index, (parameters, expression, lower, upper, left, right, expected)) in
        cases.iter().enumerate()
    {
        let package = compiled(parameters, "Integer", expression);
        let item = derived(&package);
        assert_eq!(
            domain(&item),
            &IntegerDomain::Bounded(interval(lower, upper)),
            "{expression}: {parameters}"
        );
        let generated =
            generate_exact_scalar_oracles(&package, &[item]).expect("generate derived oracle");
        let claim = &generated.claim_map.items[0];
        assert_eq!(claim.operation.provenance, OperationProvenance::IrConfirmed);
        let ClaimDisposition::Generated(claim) = &claim.result else {
            panic!("derived oracle refused: {claim:?}")
        };
        // All reported checked bounds are actual operand typing nodes, never a synthetic result id.
        assert!(!claim.checked_bounds.is_empty());
        for id in &claim.checked_bounds {
            let bound = package
                .graph()
                .nodes
                .iter()
                .find(|node| &node.node_id == id)
                .unwrap();
            assert_eq!(bound.node_tag.as_ref(), "bounded_domain");
        }
        writeln!(
            native,
            "mod generated_{index} {{ {} }}",
            claim.oracle_source
        )
        .unwrap();
        writeln!(
            native,
            r#"
#[test]
fn case_{index}() {{
    let left = {left:?}.parse().unwrap();
    let right = {right:?}.parse().unwrap();
    let outcome = generated_{index}::{symbol}(&left, &right, &mut meter()).expect("generated ABI");
    assert_eq!(outcome, rt::Outcome::Completed({expected:?}.parse::<rt::Integer>().unwrap()));
    // The original runtime zero-divisor outcome is retained, not a derived-range refusal.
    let zero = rt::Integer::zero();
    assert_eq!(generated_{index}::{symbol}(&left, &zero, &mut meter()).unwrap(),
        rt::Outcome::Undefined(rt::Undefined::DivisionByZero));
}}
"#,
            symbol = claim.symbol
        )
        .unwrap();
    }
    let manifest = format!("[package]\nname=\"integer-division-derived-native\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[dependencies]\nquire-exact = {{ {EXACT_DEPENDENCY_SOURCE} }}\n[workspace]\n");
    scratch_crate::write_manifest(&scratch.path, &manifest);
    fs::write(scratch.path.join("src/lib.rs"), native).unwrap();
    scratch_crate::resolve_seeded_lock(&scratch.path);
    let build = scratch_crate::lock_agreement_build();
    let output = Command::new(env!("CARGO"))
        .args(["test", "--locked", "--offline", "-j2", "--lib"])
        .env("CARGO_TARGET_DIR", scratch_crate::agreement_target())
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(&scratch.path)
        .output()
        .expect("execute generated native oracles");
    drop(build);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stdout}\n{stderr}");
    assert!(
        stdout.contains("8 passed; 0 failed"),
        "all eight generated cases execute: {stdout}"
    );
}

#[trace("TC-024", "FR-014-AC-21")]
#[test]
fn unbounded_dividends_still_refuse_and_caller_domains_cannot_replace_inference() {
    for expression in ["x div y", "x mod y"] {
        let package = compiled("x: Integer, y: Int[1, 4]", "Integer", expression);
        let node = body_id(&package, "subject");
        assert!(matches!(
            derive_exact_scalar_items(&package, &[node]).remove(0),
            Err(ExactScalarRefusal::MissingBound { .. })
        ));

        let package = compiled("x: Int[-9, 9], y: Int[1, 4]", "Integer", expression);
        let mut item = derived(&package);
        match &mut item.operation {
            ExactScalarOperation::IntegerDivision { domain, .. }
            | ExactScalarOperation::IntegerModulo { domain } => {
                *domain = IntegerDomain::Bounded(interval("-1", "1"))
            }
            _ => unreachable!(),
        }
        let node_id = item.node_id.clone();
        let generated = generate_exact_scalar_oracles(&package, &[item]).unwrap();
        assert_eq!(
            generated.claim_map.items[0].result,
            ClaimDisposition::Refused {
                refusal: ExactScalarRefusal::DerivedBoundMismatch { node_id }
            }
        );
    }
}

#[trace("TC-024", "FR-014-AC-21")]
#[test]
fn explicit_result_narrowing_keeps_its_own_bound() {
    for expression in ["x div y", "x mod y"] {
        let package = compiled("x: Int[-9, 9], y: Int[1, 4]", "Int[-100, 100]", expression);
        // The function body is a narrowing conversion; select its arithmetic child.
        let identity = if expression.contains("div") {
            "quire.op.integer.div"
        } else {
            "quire.op.integer.mod"
        };
        let node = package
            .graph()
            .nodes
            .iter()
            .find(|node| match &node.body {
                quire_contract_model::CheckedTerm::Application { operation, .. } => {
                    operation.identity() == Some(identity)
                }
                _ => false,
            })
            .unwrap();
        let item = derive_exact_scalar_items(&package, std::slice::from_ref(&node.node_id))
            .remove(0)
            .unwrap();
        assert_eq!(
            domain(&item),
            &IntegerDomain::Bounded(interval("-100", "100"))
        );
        let generated = generate_exact_scalar_oracles(&package, &[item]).unwrap();
        assert!(matches!(
            &generated.claim_map.items[0].result,
            ClaimDisposition::Generated(_)
        ));
    }
}

const NATIVE_SUPPORT: &str = r#"
use quire_exact as rt;
fn meter() -> rt::Meter {
    let ceiling = u64::from(u32::MAX);
    rt::Meter::new(rt::ScalarLimits {
        integer_bits: ceiling, decimal_digits: ceiling, scale_expansion: ceiling,
        text_input_bytes: ceiling, text_scalars: ceiling, normalized_scalars: ceiling,
        unit_edges: ceiling, value_occurrences: ceiling, work_units: ceiling, result_units: ceiling,
    })
}
"#;

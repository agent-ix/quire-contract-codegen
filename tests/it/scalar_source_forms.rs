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

fn compiled(declarations: &str) -> CheckedPackageV2 {
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
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    match CheckedPackageV2::read(
        emitted.bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
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
    assert_eq!(claim.operation.identity, identity);
    assert_eq!(claim.operation.provenance, OperationProvenance::IrConfirmed);
    let ClaimDisposition::Generated(generated) = &claim.result else {
        panic!("genuine source form must generate: {:?}", claim.result);
    };
    let symbol = generated.symbol.clone();
    (oracles, symbol)
}

#[trace("TC-024", "FR-014-AC-67", "FR-014-AC-69")]
#[test]
fn genuine_ieee_equality_and_intrinsics_generate() {
    for (expression, identity) in [
        ("a = b", "numeric_equal"),
        ("quire::value::ieee::numericEqual(a, b)", "numeric_equal"),
        ("quire::value::ieee::totalOrder(a, b)", "total_order"),
        ("quire::value::ieee::bitIdentical(a, b)", "bit_identical"),
    ] {
        for width in ["Float32", "Float64"] {
            let package = compiled(&format!("function subject using v(a: {width}[nearest-even], b: {width}[nearest-even]): Boolean pure {{ {expression} }}"));
            let identity = format!("quire.op.ieee.{identity}");
            generated(&package, &identity);
        }
    }
}

#[trace("TC-024", "FR-014-AC-66", "FR-014-AC-69")]
#[test]
fn genuine_carried_quantity_u01_conversion_generates() {
    let package = compiled("dimension Length;\nunit m : Length = rational(1, 1);\nunit cm : Length = rational(1, 100) * m;\nfunction subject using v(a: Quantity<cm, Int[0, 1000]>): Quantity<m> pure { convert<Quantity<m>>(a) }");
    generated(&package, "quire.op.quantity.convert");
}

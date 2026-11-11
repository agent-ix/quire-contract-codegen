// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! IEEE `sqrt` and `fma` derivation and generation (FR-014-AC-72 through FR-014-AC-80, IR-1502).
//! Genuine QSL source is passed unchanged through the strict reader; the generated native oracles
//! are compiled and compared with a direct owner `evaluate_ieee` call.

use crate::scalar_source_forms::{compiled, generated, run_native};
use ix_trace_rs::trace;
use quire_contract_codegen::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, ClaimDerivationRefusal,
    ClaimDisposition, ExactScalarItem, ExactScalarOperation, ExactScalarRefusal,
    IeeeArithmeticOperator, OperationProvenance, ScalarForm,
};
use quire_contract_model::{CheckedPackageV2, CheckedTerm};
use quire_exact::{IeeeWidth, RoundingMode};

/// One generated oracle under test.
struct Case {
    width: &'static str,
    rounding: &'static str,
    callee: &'static str,
}

impl Case {
    fn source(&self) -> String {
        let (width, rounding) = (self.width, self.rounding);
        let (parameters, arguments) = if self.callee == "sqrt" {
            (format!("a: {width}[{rounding}]"), "a")
        } else {
            (
                format!("a: {width}[{rounding}], b: {width}[{rounding}], c: {width}[{rounding}]"),
                "a, b, c",
            )
        };
        format!(
            "function subject using v({parameters}): {width}[{rounding}] pure {{ quire::value::ieee::{}({arguments}) }}",
            self.callee
        )
    }

    fn identity(&self) -> String {
        let width = if self.width == "Float32" {
            "float32"
        } else {
            "float64"
        };
        format!("quire.op.ieee.{width}.{}", self.callee)
    }
}

const ALL: [Case; 4] = [
    Case {
        width: "Float32",
        rounding: "nearest-even",
        callee: "sqrt",
    },
    Case {
        width: "Float64",
        rounding: "nearest-even",
        callee: "sqrt",
    },
    Case {
        width: "Float32",
        rounding: "nearest-even",
        callee: "fma",
    },
    Case {
        width: "Float64",
        rounding: "nearest-even",
        callee: "fma",
    },
];

fn only_node<'a>(
    package: &'a CheckedPackageV2,
    identity: &str,
) -> &'a quire_contract_model::CheckedSemanticNodeV2<CheckedTerm> {
    let nodes = package
        .resolved_graph()
        .nodes
        .iter()
        .filter(|node| matches!(&node.body, CheckedTerm::Application { operation, .. } if operation.identity() == Some(identity)))
        .collect::<Vec<_>>();
    let [node] = nodes.as_slice() else {
        panic!("one genuine {identity} application: {nodes:?}");
    };
    node
}

/// The four identities derive from genuine QSL-emitted `call` forms, are `ir_confirmed`, and the
/// descriptor names the operation and width the identity names (identity, descriptor, identity).
#[trace("TC-024", "FR-014-AC-72", "FR-014-AC-73", "FR-014-AC-74")]
#[test]
fn genuine_sqrt_and_fma_calls_derive_as_ir_confirmed_and_round_trip() {
    for case in &ALL {
        let package = compiled(&case.source());
        let identity = case.identity();
        let node = only_node(&package, &identity);
        let [item] = derive_exact_scalar_items(&package, std::slice::from_ref(&node.node_id))
            .try_into()
            .unwrap();
        let item = item.unwrap_or_else(|refusal| panic!("{identity} derives: {refusal:?}"));
        let operator = if case.callee == "sqrt" {
            IeeeArithmeticOperator::SquareRoot
        } else {
            IeeeArithmeticOperator::FusedMultiplyAdd
        };
        let width = if case.width == "Float32" {
            IeeeWidth::Binary32
        } else {
            IeeeWidth::Binary64
        };
        assert_eq!(
            item.operation,
            ExactScalarOperation::IeeeArithmetic {
                operator,
                width,
                rounding: RoundingMode::NearestEven
            },
            "{identity}: descriptor"
        );
        // Generation maps the descriptor back to the same identity and confirms it from the node.
        let (oracles, _) = generated(&package, &identity);
        let [claim] = oracles.claim_map.items.as_slice() else {
            panic!("one claim");
        };
        assert_eq!(claim.operation.identity, identity);
        assert_eq!(claim.operation.provenance, OperationProvenance::IrConfirmed);
        let ClaimDisposition::Generated(claim) = &claim.result else {
            panic!("generated");
        };
        assert_eq!(claim.semantic_form, "call");
    }
}

/// A `sqrt` descriptor never confirms against an `fma` identity, the reverse, or the other width:
/// the descriptor of a different operation or width does not map to the node's identity, so its
/// generation is refused at the defensive seam rather than confirmed (FR-014-AC-74).
#[trace("TC-024", "FR-014-AC-74", "FR-014-AC-79")]
#[test]
fn descriptors_do_not_cross_operation_or_width() {
    let sqrt64 = compiled(&ALL[1].source());
    let fma32 = compiled(&ALL[2].source());
    let sqrt_node = only_node(&sqrt64, "quire.op.ieee.float64.sqrt")
        .node_id
        .clone();
    let fma_node = only_node(&fma32, "quire.op.ieee.float32.fma")
        .node_id
        .clone();
    let descriptor = |operator, width| ExactScalarOperation::IeeeArithmetic {
        operator,
        width,
        rounding: RoundingMode::NearestEven,
    };
    // fma descriptor against the sqrt node, sqrt descriptor against the fma node: wrong operand
    // count, so the call body is a `BodyMismatch`.
    for (package, node, operation, expected) in [
        (
            &sqrt64,
            &sqrt_node,
            descriptor(
                IeeeArithmeticOperator::FusedMultiplyAdd,
                IeeeWidth::Binary64,
            ),
            3,
        ),
        (
            &fma32,
            &fma_node,
            descriptor(IeeeArithmeticOperator::SquareRoot, IeeeWidth::Binary32),
            1,
        ),
    ] {
        let refusal = refusal_of(package, node, operation);
        assert!(
            matches!(&refusal, ExactScalarRefusal::BodyMismatch { expected_operator: "call", expected_arguments } if *expected_arguments == expected),
            "{refusal:?}"
        );
    }
    // The other width: the result type names the node's width, not the descriptor's.
    let refusal = refusal_of(
        &sqrt64,
        &sqrt_node,
        descriptor(IeeeArithmeticOperator::SquareRoot, IeeeWidth::Binary32),
    );
    assert!(
        matches!(
            refusal,
            ExactScalarRefusal::ResultTypeMismatch {
                expected: ScalarForm::Float32,
                ..
            }
        ),
        "{refusal:?}"
    );
}

fn refusal_of(
    package: &CheckedPackageV2,
    node: &quire_contract_model::CheckedNodeId,
    operation: ExactScalarOperation,
) -> ExactScalarRefusal {
    let output = generate_exact_scalar_oracles(
        package,
        &[ExactScalarItem {
            node_id: node.clone(),
            operation,
        }],
    )
    .expect("a shape defect is a typed per-item refusal");
    let [claim] = output.claim_map.items.as_slice() else {
        panic!("one claim");
    };
    let ClaimDisposition::Refused { refusal } = &claim.result else {
        panic!("a descriptor against a node of another shape generates no oracle: {claim:?}");
    };
    assert!(
        !matches!(claim.result, ClaimDisposition::Generated(_)),
        "no oracle"
    );
    refusal.clone()
}

/// A `sqrt` or `fma` descriptor presented against nodes of another shape is refused at CG's
/// defensive seam with no oracle: `add`'s `binary` node, a different arity, a different width and
/// a non-float operand (FR-014-AC-79). The strict reader's own refusals of wrong arity, cross-width
/// and non-float operands are not claimed here.
#[trace("TC-024", "FR-014-AC-79")]
#[test]
fn defensive_seam_refuses_other_shapes_for_sqrt_and_fma() {
    let package = compiled(
        "function add32 using v(a: Float32[nearest-even], b: Float32[nearest-even]): Float32[nearest-even] pure { a + b }\n\
         function cmp32 using v(a: Float32[nearest-even], b: Float32[nearest-even]): Boolean pure { quire::value::ieee::totalOrder(a, b) }\n\
         function neg32 using v(a: Float32[nearest-even]): Float32[nearest-even] pure { quire::value::ieee::sqrt(a) }",
    );
    let descriptor = |operator| ExactScalarOperation::IeeeArithmetic {
        operator,
        width: IeeeWidth::Binary32,
        rounding: RoundingMode::NearestEven,
    };
    let add = only_node(&package, "quire.op.ieee.float32.add")
        .node_id
        .clone();
    let cmp = only_node(&package, "quire.op.ieee.total_order")
        .node_id
        .clone();
    let sqrt = only_node(&package, "quire.op.ieee.float32.sqrt")
        .node_id
        .clone();
    for operator in [
        IeeeArithmeticOperator::SquareRoot,
        IeeeArithmeticOperator::FusedMultiplyAdd,
    ] {
        // Wrong semantic form: `add` is a `binary` expression.
        let refusal = refusal_of(&package, &add, descriptor(operator));
        assert!(
            matches!(&refusal, ExactScalarRefusal::FormMismatch { expected: "call", found } if found == "binary"),
            "{operator:?} over add: {refusal:?}"
        );
        // A `call` node of the wrong result type (Boolean) is a `ResultTypeMismatch`.
        let refusal = refusal_of(&package, &cmp, descriptor(operator));
        assert!(
            matches!(
                refusal,
                ExactScalarRefusal::BodyMismatch { .. }
                    | ExactScalarRefusal::ResultTypeMismatch { .. }
            ),
            "{operator:?} over a comparison: {refusal:?}"
        );
    }
    // The descriptor's own shape is accepted over its own node.
    let item = derive_exact_scalar_items(&package, std::slice::from_ref(&sqrt))
        .pop()
        .unwrap()
        .unwrap();
    assert!(matches!(
        generate_exact_scalar_oracles(&package, &[item])
            .unwrap()
            .claim_map
            .items[0]
            .result,
        ClaimDisposition::Generated(_)
    ));
}

/// `from_exact`, `to_rational` and both `negate` identities keep the `OperationNotDerivable`
/// refusal over genuine QSL-emitted nodes, and a supported sibling is unaffected (FR-014-AC-80).
#[trace("TC-024", "FR-014-AC-80")]
#[test]
fn from_exact_to_rational_and_negate_stay_not_derivable() {
    let package = compiled(
        "function neg32 using v(a: Float32[nearest-even]): Float32[nearest-even] pure { -a }\n\
         function neg64 using v(a: Float64[nearest-even]): Float64[nearest-even] pure { -a }\n\
         function root using v(a: Float32[nearest-even]): Float32[nearest-even] pure { quire::value::ieee::sqrt(a) }",
    );
    let sibling = only_node(&package, "quire.op.ieee.float32.sqrt")
        .node_id
        .clone();
    for identity in [
        "quire.op.ieee.float32.negate",
        "quire.op.ieee.float64.negate",
    ] {
        let node = only_node(&package, identity).node_id.clone();
        let derived = derive_exact_scalar_items(&package, &[node, sibling.clone()]);
        assert_eq!(
            derived[0].as_ref().err(),
            Some(&ExactScalarRefusal::NoDerivableClaim {
                reason: ClaimDerivationRefusal::OperationNotDerivable {
                    operation_identity: identity.to_owned()
                }
            }),
            "{identity}"
        );
        assert!(derived[1].is_ok(), "supported sibling is unchanged");
    }
}

/// The native comparison harness. `{calls}` is spliced inside `values_and_ledgers`.
fn native_checks(calls: &str) -> String {
    let support = r#"
#[cfg(test)]
mod native {
use quire_exact as rt;
const LIMITS: rt::ScalarLimits = rt::ScalarLimits {
    integer_bits: 4096, decimal_digits: 4096, scale_expansion: 4096,
    text_input_bytes: 4096, text_scalars: 4096, normalized_scalars: 4096,
    unit_edges: 4096, value_occurrences: 4096, work_units: 4096, result_units: 4096,
};
type Oracle<'a> = &'a dyn Fn(&[rt::IeeeValue], &mut rt::Meter) -> Result<rt::Outcome<rt::IeeeResult>, String>;

fn value(narrow: bool, bits: u64) -> rt::IeeeValue {
    if narrow { rt::IeeeValue::binary32(u32::try_from(bits).unwrap()) } else { rt::IeeeValue::binary64(bits) }
}
fn operation(operands: &[rt::IeeeValue]) -> rt::IeeeOperation<rt::IeeeValue> {
    match operands {
        [a] => rt::IeeeOperation::SquareRoot(*a),
        [a, b, c] => rt::IeeeOperation::FusedMultiplyAdd(*a, *b, *c),
        _ => panic!("sqrt takes one operand and fma three"),
    }
}
fn sweep_limits(limits: rt::ScalarLimits, kind: rt::LimitKind, limit: u64) -> rt::ScalarLimits {
    match kind {
        rt::LimitKind::WorkUnits => rt::ScalarLimits { work_units: limit, ..limits },
        rt::LimitKind::ResultUnits => rt::ScalarLimits { result_units: limit, ..limits },
        _ => panic!("only the metered limits are swept"),
    }
}
/// One vector: the generated outcome, ledger and counters equal the owner's at the generous
/// limits and at every next-charge denial boundary of the work and result counters.
fn agree(label: &str, narrow: bool, rounding: rt::RoundingMode, operands: &[u64], expected: Option<u64>, oracle: Oracle) {
    let operands = operands.iter().map(|bits| value(narrow, *bits)).collect::<Vec<_>>();
    let mut direct_meter = rt::Meter::new(LIMITS);
    let direct = rt::evaluate_ieee(operation(&operands), rounding, &mut direct_meter).unwrap();
    let mut generated_meter = rt::Meter::new(LIMITS);
    let actual = oracle(&operands, &mut generated_meter).unwrap();
    assert_eq!(actual, direct, "{label}");
    assert_eq!(generated_meter.admitted_charges(), direct_meter.admitted_charges(), "{label}");
    assert!(!generated_meter.admitted_charges().is_empty(), "{label}: the owner charges");
    if let Some(expected) = expected {
        let rt::Outcome::Completed(result) = &actual else { panic!("{label}: completes: {actual:?}") };
        assert_eq!(result.value().bits(), expected, "{label}: {:#x}", result.value().bits());
    }
    for kind in [rt::LimitKind::WorkUnits, rt::LimitKind::ResultUnits] {
        let total = direct_meter.consumed(kind);
        for limit in 0..=total {
            let limits = sweep_limits(LIMITS, kind, limit);
            let mut direct_meter = rt::Meter::new(limits);
            let direct = rt::evaluate_ieee(operation(&operands), rounding, &mut direct_meter).unwrap();
            let mut generated_meter = rt::Meter::new(limits);
            let actual = oracle(&operands, &mut generated_meter).unwrap();
            assert_eq!(actual, direct, "{label}: {kind:?} limit {limit}");
            assert_eq!(generated_meter.admitted_charges(), direct_meter.admitted_charges(), "{label}: {kind:?} limit {limit}");
            for counter in rt::LimitKind::ALL { assert_eq!(generated_meter.consumed(counter), direct_meter.consumed(counter), "{label}: {kind:?} limit {limit}"); }
            if limit < total { assert!(matches!(actual, rt::Outcome::Incomplete(_)), "{label}: {kind:?} limit {limit} denies"); }
        }
    }
}
/// A wrong-width operand stops as the oracle's own `OperandWidth` before any charge.
fn width_stops(label: &str, narrow: bool, arity: usize, oracle: Oracle) {
    let right = value(narrow, 0);
    let wrong = value(!narrow, 0);
    for position in 0..arity {
        let mut operands = vec![right; arity];
        operands[position] = wrong;
        let mut meter = rt::Meter::new(LIMITS);
        let stop = oracle(&operands, &mut meter).unwrap_err();
        assert!(stop.contains("OperandWidth"), "{label}[{position}]: {stop}");
        assert!(meter.admitted_charges().is_empty(), "{label}[{position}]: stops before any charge");
        for kind in rt::LimitKind::ALL { assert_eq!(meter.consumed(kind), 0, "{label}[{position}]"); }
    }
}
#[test]
fn values_and_ledgers() {
"#;
    format!("{support}{calls}\n}}\n}}\n")
}

struct Vector {
    case: Case,
    operands: &'static [u64],
    expected: Option<u64>,
}

fn vectors() -> Vec<Vector> {
    let case = |width, rounding, callee| Case {
        width,
        rounding,
        callee,
    };
    let sqrt = |width, rounding, bits: &'static [u64], expected| Vector {
        case: case(width, rounding, "sqrt"),
        operands: bits,
        expected,
    };
    let fma = |width, bits: &'static [u64], expected| Vector {
        case: case(width, "nearest-even", "fma"),
        operands: bits,
        expected,
    };
    vec![
        sqrt("Float32", "nearest-even", &[0x40000000], Some(0x3FB504F3)),
        sqrt(
            "Float32",
            "toward-positive",
            &[0x40000000],
            Some(0x3FB504F4),
        ),
        sqrt("Float32", "nearest-even", &[0x40800000], Some(0x40000000)),
        sqrt("Float32", "nearest-even", &[0xBF800000], None),
        sqrt("Float32", "nearest-even", &[0x7FC00001], None),
        sqrt(
            "Float64",
            "nearest-even",
            &[0x4000000000000000],
            Some(0x3FF6A09E667F3BCD),
        ),
        sqrt(
            "Float64",
            "toward-negative",
            &[0x4000000000000000],
            Some(0x3FF6A09E667F3BCC),
        ),
        sqrt(
            "Float64",
            "toward-zero",
            &[0x4000000000000000],
            Some(0x3FF6A09E667F3BCC),
        ),
        sqrt(
            "Float64",
            "nearest-even",
            &[0x4010000000000000],
            Some(0x4000000000000000),
        ),
        fma(
            "Float32",
            &[0x3F800001, 0x3F800001, 0xBF800002],
            Some(0x28800000),
        ),
        fma(
            "Float32",
            &[0x40000000, 0x40400000, 0x3F800000],
            Some(0x40E00000),
        ),
        fma(
            "Float64",
            &[0x3FF0000000000001, 0x3FF0000000000001, 0xBFF0000000000002],
            Some(0x3970000000000000),
        ),
        fma(
            "Float64",
            &[0x4000000000000000, 0x4008000000000000, 0x3FF0000000000000],
            Some(0x401C000000000000),
        ),
    ]
}

/// Each generated crate is compiled and every vector is compared with a direct owner call: values,
/// flags, ordered admitted charges, consumed counters and every next-charge denial (FR-014-AC-75,
/// -76), and a wrong-width operand stops as `OperandWidth` before any charge (FR-014-AC-77).
#[trace("TC-024", "FR-014-AC-75", "FR-014-AC-76", "FR-014-AC-77")]
#[test]
fn generated_sqrt_and_fma_agree_with_the_owner_and_stop_on_wrong_width() {
    let mut native = String::new();
    let mut calls = String::new();
    let mut manifest = String::new();
    let mut modules: Vec<(String, String)> = Vec::new();
    for (ordinal, vector) in vectors().iter().enumerate() {
        let case = &vector.case;
        let key = format!("{}/{}/{}", case.width, case.rounding, case.callee);
        let module = match modules.iter().find(|(known, _)| *known == key) {
            Some((_, module)) => module.clone(),
            None => {
                let package = compiled(&case.source());
                let (oracles, symbol) = generated(&package, &case.identity());
                manifest = oracles
                    .artifacts
                    .iter()
                    .find(|artifact| artifact.path == "Cargo.toml")
                    .unwrap()
                    .contents
                    .clone();
                let ClaimDisposition::Generated(claim) = &oracles.claim_map.items[0].result else {
                    panic!("generated claim");
                };
                let module = format!("case_{}", modules.len());
                native.push_str(&format!("mod {module} {{ {} }}\n", claim.oracle_source));
                let arity = if case.callee == "sqrt" { 1 } else { 3 };
                let arguments = (0..arity)
                    .map(|index| format!("operands[{index}]"))
                    .collect::<Vec<_>>()
                    .join(", ");
                calls.push_str(&format!(
                    "let {module} = |operands: &[rt::IeeeValue], meter: &mut rt::Meter| crate::{module}::{symbol}({arguments}, meter).map_err(|stop| format!(\"{{stop:?}}\"));\n"
                ));
                calls.push_str(&format!(
                    "width_stops(\"{key}\", {}, {arity}, &{module});\n",
                    case.width == "Float32"
                ));
                modules.push((key.clone(), module.clone()));
                module
            }
        };
        let rounding = match case.rounding {
            "nearest-even" => "NearestEven",
            "toward-positive" => "TowardPositive",
            "toward-negative" => "TowardNegative",
            "toward-zero" => "TowardZero",
            other => panic!("{other}"),
        };
        calls.push_str(&format!(
            "agree(\"vector {ordinal} {key}\", {}, rt::RoundingMode::{rounding}, &{:?}, {:?}, &{module});\n",
            case.width == "Float32",
            vector.operands,
            vector.expected
        ));
    }
    native.push_str(&native_checks(&calls));
    run_native("ieee-sqrt-fma", &manifest, &native, "");
}

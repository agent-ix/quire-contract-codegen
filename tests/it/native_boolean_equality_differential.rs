// SPDX-License-Identifier: AGPL-3.0-or-later
//! The native Boolean equality the clause-native path renders (the quire-exact kernel, with `!=`
//! negating after the same charges) against the typed route it replaces
//! (`TypeEnvironment::check_equality(..).evaluate(..)`): value, outcome kind, every captured
//! charge event and every counter, unlimited and at each tightened limit.

use ix_trace_rs::trace;
use quire_exact as rt;
use quire_semantic_value::declaration as sv;

const KINDS: [rt::LimitKind; 10] = [
    rt::LimitKind::IntegerBits,
    rt::LimitKind::DecimalDigits,
    rt::LimitKind::ScaleExpansion,
    rt::LimitKind::TextInputBytes,
    rt::LimitKind::TextScalars,
    rt::LimitKind::NormalizedScalars,
    rt::LimitKind::UnitEdges,
    rt::LimitKind::ValueOccurrences,
    rt::LimitKind::WorkUnits,
    rt::LimitKind::ResultUnits,
];

fn limits() -> rt::ScalarLimits {
    rt::ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    }
}

type Run = (rt::Outcome<bool>, Vec<rt::ChargeEvent>, Vec<u64>);

fn run(limits: rt::ScalarLimits, f: &dyn Fn(&mut rt::Meter) -> rt::Outcome<bool>) -> Run {
    let mut meter = rt::Meter::new(limits);
    meter.enable_charge_capture(64, 256);
    let outcome = f(&mut meter);
    let counters = KINDS.iter().map(|kind| meter.consumed(*kind)).collect();
    let events = match meter.disable_charge_capture() {
        Some(Ok(capture)) => capture.events().to_vec(),
        _ => panic!("charge capture missing or truncated"),
    };
    (outcome, events, counters)
}

/// The rendered native route (`render.rs`, Equality over Boolean operands on the native path).
fn native(
    operator: sv::EqualityOperator,
    left: bool,
    right: bool,
    meter: &mut rt::Meter,
) -> rt::Outcome<bool> {
    let equal = matches!(operator, sv::EqualityOperator::Equal);
    match rt::evaluate_native_boolean_equality(left, right, meter) {
        rt::Outcome::Completed(related) => rt::Outcome::Completed(related == equal),
        other => other,
    }
}

/// The general route the native path replaces.
fn typed(
    operator: sv::EqualityOperator,
    left: bool,
    right: bool,
    meter: &mut rt::Meter,
) -> rt::Outcome<bool> {
    let environment = sv::TypeEnvironment::default();
    let Ok(checked) = environment.check_equality(
        operator,
        sv::EqualityOperand::typed(rt::ValueType::Boolean),
        sv::EqualityOperand::typed(rt::ValueType::Boolean),
        &quire_semantic_value::enumeration::EnumMemberIndex::default(),
    ) else {
        panic!("Boolean equality is well typed");
    };
    checked.evaluate(
        &environment,
        &rt::Value::Boolean(left),
        &rt::Value::Boolean(right),
        meter,
    )
}

#[trace("TC-025", "FR-015-AC-146")]
#[test]
fn native_boolean_equality_and_inequality_match_the_typed_route() {
    for operator in [sv::EqualityOperator::Equal, sv::EqualityOperator::NotEqual] {
        for left in [false, true] {
            for right in [false, true] {
                let native_run =
                    |l: rt::ScalarLimits| run(l, &|m| native(operator, left, right, m));
                let typed_run = |l: rt::ScalarLimits| run(l, &|m| typed(operator, left, right, m));
                let (outcome, events, counters) = typed_run(limits());
                let expected = (left == right) == matches!(operator, sv::EqualityOperator::Equal);
                assert_eq!(outcome, rt::Outcome::Completed(expected));
                assert_eq!(events.len(), 4, "plan-form, plan, one pair, result-retain");
                assert_eq!(native_run(limits()), (outcome, events, counters.clone()));
                for (kind, spend) in KINDS.iter().zip(counters) {
                    for bound in 0..=spend {
                        let mut tight = limits();
                        match kind {
                            rt::LimitKind::ValueOccurrences => tight.value_occurrences = bound,
                            rt::LimitKind::WorkUnits => tight.work_units = bound,
                            rt::LimitKind::ResultUnits => tight.result_units = bound,
                            _ => continue,
                        }
                        assert_eq!(
                            native_run(tight),
                            typed_run(tight),
                            "{operator:?} {left} {right} {kind:?} {bound}"
                        );
                    }
                }
            }
        }
    }
}

//! FR-014-AC-37: admitted V2 Boolean and scalar equality oracles against direct Runtime calls.

#[path = "agreement.rs"]
#[macro_use]
mod support;

mod generated {
    pub use quire_exact_scalar_oracles::*;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}

use generated::{
    boolean_and, boolean_eq, boolean_implies, boolean_lazy_and, boolean_lazy_or, boolean_ne,
    boolean_not, boolean_or, boolean_short_circuit, integer_eq, integer_ge, integer_gt, integer_le,
    integer_lt, integer_ne, OracleStop,
};
use ix_trace_rs::trace;
use quire_exact::ValueType;
use quire_exact::{Outcome, Refusal};
use quire_semantic_value::declaration::{EqualityOperand, EqualityOperator, TypeEnvironment};
use quire_semantic_value::enumeration::EnumMemberIndex;

fn done(result: Result<Outcome<bool>, OracleStop>) -> Outcome<bool> {
    result.expect("a generated oracle over admitted operands does not stop")
}

#[trace("FR-014-AC-37", "TC-024")]
#[test]
fn boolean_connectives_agree_in_outcome_charges_counters_and_denials() {
    for left in [false, true] {
        agree2! {
            limits: support::rt_side::UNLIMITED,
            setup: {},
            direct: |m| evaluate_boolean(BooleanConnective::Not(left), m),
            generated: |m| done(boolean_not(left, m)),
        };
        for right in [false, true] {
            agree2! {
                limits: support::rt_side::UNLIMITED,
                setup: {},
                direct: |m| (
                    evaluate_boolean(BooleanConnective::And(left, right), m),
                    evaluate_boolean(BooleanConnective::Or(left, right), m),
                    evaluate_boolean(BooleanConnective::Implies(left, right), m),
                ),
                generated: |m| (
                    done(boolean_and(left, right, m)),
                    done(boolean_or(left, right, m)),
                    done(boolean_implies(left, right, m)),
                ),
            };
        }
    }
    for left in [false, true] {
        for right in [
            Outcome::Completed(false),
            Outcome::Completed(true),
            Outcome::Refused(Refusal::CheckedInvariant {
                cause: quire_exact::CheckedInvariantCause::GeneratedDescriptorReconstructionFailed,
            }),
        ] {
            agree2! {
                limits: support::rt_side::UNLIMITED,
                setup: {},
                direct: |m| (
                    evaluate_boolean_short_circuit(ShortCircuitConnective::Implies, left, || right.clone(), m),
                    evaluate_boolean_short_circuit(ShortCircuitConnective::And, left, || right.clone(), m),
                    evaluate_boolean_short_circuit(ShortCircuitConnective::Or, left, || right.clone(), m),
                ),
                generated: |m| (
                    done(boolean_short_circuit(left, |_| right.clone(), m)),
                    done(boolean_lazy_and(left, |_| right.clone(), m)),
                    done(boolean_lazy_or(left, |_| right.clone(), m)),
                ),
            };
        }
    }
}

#[trace("FR-014-AC-37", "TC-024")]
#[test]
fn typed_scalar_equalities_agree_in_outcome_charges_counters_and_denials() {
    let environment = TypeEnvironment::default();
    let enum_members = EnumMemberIndex::default();
    let boolean_equal = environment
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Boolean),
            EqualityOperand::typed(ValueType::Boolean),
            &enum_members,
        )
        .expect("Boolean equality");
    let boolean_not_equal = environment
        .check_equality(
            EqualityOperator::NotEqual,
            EqualityOperand::typed(ValueType::Boolean),
            EqualityOperand::typed(ValueType::Boolean),
            &enum_members,
        )
        .expect("Boolean inequality");
    let integer_equal = environment
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Integer),
            EqualityOperand::typed(ValueType::Integer),
            &enum_members,
        )
        .expect("Integer equality");
    let integer_not_equal = environment
        .check_equality(
            EqualityOperator::NotEqual,
            EqualityOperand::typed(ValueType::Integer),
            EqualityOperand::typed(ValueType::Integer),
            &enum_members,
        )
        .expect("Integer inequality");
    for left in [false, true] {
        for right in [false, true] {
            agree2! {
                limits: support::rt_side::UNLIMITED,
                setup: { let (l, r) = (Value::Boolean(left), Value::Boolean(right)); },
                direct: |m| (
                    boolean_equal.evaluate(&environment, &l, &r, m),
                    boolean_not_equal.evaluate(&environment, &l, &r, m),
                ),
                generated: |m| (
                    done(boolean_eq(left, right, m)),
                    done(boolean_ne(left, right, m)),
                ),
            };
        }
    }
    for left in [-1_i64, 0, 1] {
        for right in [-1_i64, 0, 1] {
            agree2! {
                limits: support::rt_side::UNLIMITED,
                setup: {
                    let (l, r) = (Integer::from(left), Integer::from(right));
                    let (lv, rv) = (Value::Integer(l.clone()), Value::Integer(r.clone()));
                },
                direct: |m| (
                    integer_equal.evaluate(&environment, &lv, &rv, m),
                    integer_not_equal.evaluate(&environment, &lv, &rv, m),
                    order_numbers(OrderingOperator::Less, OrderedOperands::Integers(&l, &r), m),
                    order_numbers(OrderingOperator::LessOrEqual, OrderedOperands::Integers(&l, &r), m),
                    order_numbers(OrderingOperator::Greater, OrderedOperands::Integers(&l, &r), m),
                    order_numbers(OrderingOperator::GreaterOrEqual, OrderedOperands::Integers(&l, &r), m),
                ),
                generated: |m| (
                    done(integer_eq(&l, &r, m)),
                    done(integer_ne(&l, &r, m)),
                    done(integer_lt(&l, &r, m)),
                    done(integer_le(&l, &r, m)),
                    done(integer_gt(&l, &r, m)),
                    done(integer_ge(&l, &r, m)),
                ),
            };
        }
    }
}

/// The caller prefix, reached equality and parent retention share one meter.
#[trace("TC-024", "FR-014-AC-71")]
#[test]
fn lazy_connectives_charge_reached_right_on_the_caller_meter() {
    use support::rt_side::{Meter, ShortCircuitConnective, Value};

    let environment = TypeEnvironment::default();
    let equality = environment
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Boolean),
            EqualityOperand::typed(ValueType::Boolean),
            &EnumMemberIndex::default(),
        )
        .expect("actual owner Boolean equality");

    let reference = |kind: ShortCircuitConnective, left: bool, right: bool, meter: &mut Meter| {
        use support::rt_side::retain_boolean;
        if let Err(incomplete) = retain_boolean(true, meter) {
            return (Outcome::Incomplete(incomplete), 0);
        }
        let decided = match kind {
            ShortCircuitConnective::And if !left => Some(false),
            ShortCircuitConnective::Or if left => Some(true),
            ShortCircuitConnective::Implies if !left => Some(true),
            ShortCircuitConnective::And
            | ShortCircuitConnective::Or
            | ShortCircuitConnective::Implies => None,
        };
        if let Some(value) = decided {
            return (
                match retain_boolean(value, meter) {
                    Ok(value) => Outcome::Completed(value),
                    Err(incomplete) => Outcome::Incomplete(incomplete),
                },
                0,
            );
        }
        let outcome = equality.evaluate(
            &environment,
            &Value::Boolean(true),
            &Value::Boolean(right),
            meter,
        );
        let outcome = match outcome {
            Outcome::Completed(value) => match retain_boolean(value, meter) {
                Ok(value) => Outcome::Completed(value),
                Err(incomplete) => Outcome::Incomplete(incomplete),
            },
            stopped => stopped,
        };
        (outcome, 1)
    };

    macro_rules! compare {
        ($oracle:ident, $kind:ident, $left:expr, $calls:expr) => {{
            for right in [false, true] {
                let execute = |meter: &mut Meter| {
                    if let Err(incomplete) = support::rt_side::retain_boolean(true, meter) {
                        return (Outcome::Incomplete(incomplete), 0);
                    }
                    let mut calls = 0;
                    let outcome = done($oracle(
                        $left,
                        |same_meter| {
                            calls += 1;
                            equality.evaluate(
                                &environment,
                                &Value::Boolean(true),
                                &Value::Boolean(right),
                                same_meter,
                            )
                        },
                        meter,
                    ));
                    (outcome, calls)
                };
                let mut baseline = Meter::new(support::rt_side::UNLIMITED);
                let (outcome, calls) =
                    reference(ShortCircuitConnective::$kind, $left, right, &mut baseline);
                assert!(matches!(outcome, Outcome::Completed(_)));
                assert_eq!(calls, $calls);
                let mut scenarios = vec![None];
                let mut seen = Vec::new();
                for charge in baseline.admitted_charges() {
                    seen.push(charge.point);
                    let occurrence =
                        u64::try_from(seen.iter().filter(|point| **point == charge.point).count())
                            .unwrap();
                    scenarios.push(Some(support::rt_side::InjectedDenial {
                        point: charge.point,
                        occurrence: support::rt_side::to_occurrence(occurrence),
                    }));
                }
                for denial in scenarios {
                    let new_meter = || {
                        let meter = Meter::new(support::rt_side::UNLIMITED);
                        match denial {
                            Some(denial) => meter.with_injected_denial(denial),
                            None => meter,
                        }
                    };
                    let mut expected_meter = new_meter();
                    let mut actual_meter = new_meter();
                    let expected = reference(
                        ShortCircuitConnective::$kind,
                        $left,
                        right,
                        &mut expected_meter,
                    );
                    let actual = execute(&mut actual_meter);
                    assert_eq!(actual, expected, "typed outcome and invocation count");
                    assert_eq!(
                        actual_meter.admitted_charges(),
                        expected_meter.admitted_charges()
                    );
                    for kind in support::rt_side::LimitKind::ALL {
                        assert_eq!(actual_meter.consumed(kind), expected_meter.consumed(kind));
                    }
                }
            }
        }};
    }
    compare!(boolean_lazy_and, And, false, 0);
    compare!(boolean_lazy_and, And, true, 1);
    compare!(boolean_lazy_or, Or, false, 1);
    compare!(boolean_lazy_or, Or, true, 0);
    compare!(boolean_short_circuit, Implies, false, 0);
    compare!(boolean_short_circuit, Implies, true, 1);
}

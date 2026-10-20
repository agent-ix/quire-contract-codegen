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
    boolean_not, boolean_or, boolean_short_circuit, integer_eq, integer_ge, integer_gt, integer_le, integer_lt,
    integer_ne, OracleStop,
};
use ix_trace_rs::trace;
use quire_exact::{Outcome, Refusal};
use quire_contract_runtime::exact::{
    EqualityOperand, EqualityOperator, TypeEnvironment, ValueType,
};

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
            Outcome::Refused(Refusal::CheckedInvariant),
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
                    done(boolean_short_circuit(left, || right.clone(), m)),
                    done(boolean_lazy_and(left, || right.clone(), m)),
                    done(boolean_lazy_or(left, || right.clone(), m)),
                ),
            };
        }
    }
}

#[trace("FR-014-AC-37", "TC-024")]
#[test]
fn typed_scalar_equalities_agree_in_outcome_charges_counters_and_denials() {
    let environment = TypeEnvironment::default();
    let boolean_equal = environment
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Boolean),
            EqualityOperand::typed(ValueType::Boolean),
        )
        .expect("Boolean equality");
    let boolean_not_equal = environment
        .check_equality(
            EqualityOperator::NotEqual,
            EqualityOperand::typed(ValueType::Boolean),
            EqualityOperand::typed(ValueType::Boolean),
        )
        .expect("Boolean inequality");
    let integer_equal = environment
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Integer),
            EqualityOperand::typed(ValueType::Integer),
        )
        .expect("Integer equality");
    let integer_not_equal = environment
        .check_equality(
            EqualityOperator::NotEqual,
            EqualityOperand::typed(ValueType::Integer),
            EqualityOperand::typed(ValueType::Integer),
        )
        .expect("Integer inequality");
    for left in [false, true] {
        for right in [false, true] {
            agree2! {
                limits: support::rt_side::UNLIMITED,
                setup: { let (l, r) = (Value::Boolean(left), Value::Boolean(right)); },
                direct: |m| (
                    boolean_equal.evaluate(&l, &r, m),
                    boolean_not_equal.evaluate(&l, &r, m),
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
                    integer_equal.evaluate(&lv, &rv, m),
                    integer_not_equal.evaluate(&lv, &rv, m),
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

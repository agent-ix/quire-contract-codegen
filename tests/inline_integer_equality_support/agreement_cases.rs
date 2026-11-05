// SPDX-License-Identifier: AGPL-3.0-or-later
//! Execute the generated inline-literal equality oracle against direct runtime equality.

mod generated {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}

use quire_exact::{Integer, Meter, Outcome, ScalarLimits, Value, ValueType};
use quire_semantic_value::declaration::{EqualityOperand, EqualityOperator, TypeEnvironment};

const LIMITS: ScalarLimits = ScalarLimits {
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
};

#[ix_trace_rs::trace("FR-018-AC-47", "TC-029")]
#[test]
fn generated_integer_equality_values_match_direct_runtime() {
    let environment = TypeEnvironment::default();
    let checked = environment
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Integer),
            EqualityOperand::typed(ValueType::Integer),
            &Default::default(),
        )
        .expect("direct runtime admits Integer equality");
    let prepared = generated::environment_literal_equal(None)
        .expect("generated Integer environment needs no nominal context");
    for (left, right, expected) in [(2_i64, 2_i64, true), (2, 3, false), (3, 2, false)] {
        let left = Value::Integer(Integer::from(left));
        let right = Value::Integer(Integer::from(right));
        let actual =
            generated::oracle_literal_equal(&prepared, &left, &right, &mut Meter::new(LIMITS));
        let direct = checked.evaluate(&environment, &left, &right, &mut Meter::new(LIMITS));
        assert_eq!(actual, direct, "generated and direct equality must agree");
        assert_eq!(actual, Outcome::Completed(expected));
    }
}

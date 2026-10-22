// SPDX-License-Identifier: AGPL-3.0-or-later
//! Original numeric domains and adverse vectors, with independently admitted source identities.
#[macro_use]
#[path = "../agreement.rs"]
mod support;
#[path = "../codes.rs"]
mod package;
#[path = "../authentic_native.rs"]
mod fixture;
mod generated {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));

}
use ix_trace_rs::trace;

#[trace("FR-018-AC-2", "TC-029")]
#[test]
fn original_free_conversion_vectors() {
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree2! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let left = Value::Integer(Integer::from(l));
                let right = Value::Integer(Integer::from(r));
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(
                    ValueType::Int(IntegerInterval::new(Integer::from(-100_i64), Integer::from(100_i64)).unwrap()),
                    ValueType::Integer,
                ),
                operand_typed(ValueType::Integer),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_e_conv_equal( // E_CONV Equal
                &generated::environment_e_conv_equal(Some(support::context())).expect("same original admitted context"), &left, &right, g,
            ),
        };
    }
}

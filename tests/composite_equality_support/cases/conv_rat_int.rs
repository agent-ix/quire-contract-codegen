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
fn original_conv_rat_int_vectors() {
    // Rational[-50, 50; 1, 1] -> Integer.
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree2! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let left = rational_value(l, 1);
                let right = Value::Integer(Integer::from(r));
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(rational_type(-50, 50, 1, 1), ValueType::Integer),
                operand_typed(ValueType::Integer),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_e_conv_rat_int_equal( // E_CONV_RAT_INT Equal
                &generated::environment_e_conv_rat_int_equal(Some(support::context())).expect("same original admitted context"), &left, &right, g,
            ),
        };
    }


}

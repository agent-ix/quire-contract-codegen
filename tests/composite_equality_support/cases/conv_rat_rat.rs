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
fn original_conv_rat_rat_vectors() {
    // Rational[-10, 10; 1, 1] -> Rational[-100, 100; 1, 5].
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree2! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let left = rational_value(l, 1);
                let right = rational_value(r, 1);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(
                    rational_type(-10, 10, 1, 1),
                    rational_type(-100, 100, 1, 5),
                ),
                operand_typed(rational_type(-100, 100, 1, 5)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_e_conv_rat_rat_equal( // E_CONV_RAT_RAT Equal
                &generated::environment_e_conv_rat_rat_equal(Some(support::context())).expect("same original admitted context"), &left, &right, g,
            ),
        };
    }


}

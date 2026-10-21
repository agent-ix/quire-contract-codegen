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
use package::*;

#[trace("FR-018-AC-2", "FR-018-AC-9", "FR-018-AC-10", "TC-029")]
#[test]
fn original_shape_vectors_tuple_1() {
    // Tuple (E_TUPLE): equal, and unequal on each position.
    for (ln, lt, rn, rt_) in [
        (1, "abc", 1, "abc"),
        (1, "abc", 2, "abc"),
        (1, "abc", 1, "xyz"),
    ] {
        agree2! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_tuple();
                let left = tuple_pair(&environment, ln, lt);
                let right = tuple_pair(&environment, rn, rt_);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(composite_type(TUP_PAIR)),
                operand_typed(composite_type(TUP_PAIR)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_e_tuple_equal( // E_TUPLE Equal
                &generated::environment_e_tuple_equal(Some(support::context())).expect("same original admitted context"), &left, &right, g,
            ),
        };
    }


}

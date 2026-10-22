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
fn original_shape_vectors_recursive_4() {
    // Recursive (E_SELF): both leaves, and one leaf vs. one nested one level.
    for nested_right in [false, true] {
        agree2! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_self();
                let leaf = record_self(&environment, FieldValue::Absent);
                let nested = record_self(&environment, FieldValue::Present(record_self(&environment, FieldValue::Absent)));
                let left = leaf.clone();
                let right = if nested_right { nested } else { leaf };
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(composite_type(R_SELF)),
                operand_typed(composite_type(R_SELF)),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_e_self_equal( // E_SELF Equal
                &generated::environment_e_self_equal(Some(support::context())).expect("same original admitted context"), &left, &right, g,
            ),
        };
    }


}

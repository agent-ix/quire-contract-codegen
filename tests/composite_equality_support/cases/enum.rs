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

#[trace("FR-018-AC-2", "FR-018-AC-9", "FR-018-AC-10", "TC-029")]
#[test]
fn original_shape_vectors_enum_8() {
    // Enum (E_ENUM, `equal`): equal and unequal members of the original local
    // `Phase` source declaration, using the independently compiled facade identities.
    for (l, r) in [("OPEN", "OPEN"), ("OPEN", "SHUT")] {
        agree2! {
            limits: UNLIMITED,
            setup: {
                let environment = environment_option();
                let declaration = enum_status();
                let left = declaration.value(l);
                let right = declaration.value(r);
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_typed(enum_status_type()),
                operand_typed(enum_status_type()),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_e_enum_equal( // E_ENUM Equal
                &generated::environment_e_enum_equal(Some(support::context())).expect("same original admitted context"), &left, &right, g,
            ),
        };
    }
}

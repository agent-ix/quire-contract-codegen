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

#[trace("FR-018-AC-9", "TC-029")]
#[test]
fn original_charged_conversion_vectors() {
    for (l, r) in [(5_i64, 5_i64), (5, 6)] {
        agree2! {
            limits: UNLIMITED,
            setup: {
                let target = DecimalType::new(
                    Integer::from(-100_i64), Integer::from(100_i64), 0, 0, RoundingMode::NearestEven,
                ).unwrap();
                let environment = environment_option();
                let left = Value::Integer(Integer::from(l));
                let right = Value::Decimal(Decimal::new(Integer::from(r), 0));
            },
            direct: |m| direct_equality(
                &environment,
                EqualityOperator::Equal,
                operand_converted(
                    ValueType::Int(IntegerInterval::new(Integer::from(-100_i64), Integer::from(100_i64)).unwrap()),
                    ValueType::Decimal(target.clone()),
                ),
                operand_typed(ValueType::Decimal(target.clone())),
                &left,
                &right,
                m,
            ),
            generated: |g| generated::oracle_e_conv_charge_equal( // E_CONV_CHARGE Equal
                &generated::environment_e_conv_charge_equal(Some(support::context())).expect("same original admitted context"), &left, &right, g,
            ),
        };
    }
}

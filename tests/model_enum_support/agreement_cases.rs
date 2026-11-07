// SPDX-License-Identifier: AGPL-3.0-or-later
//! Generated functions execute against original selected model member identities.

mod fixture;
mod generated {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}

use quire_exact::{ComparisonOperator, LimitKind, Meter, Outcome, ScalarLimits};
use quire_semantic_value::enumeration::{compare_enum, EnumValue};

type Compare = fn(
    &EnumValue,
    &EnumValue,
    &mut Meter,
) -> Result<Outcome<bool>, quire_exact_scalar_oracles::OracleStop>;

#[ix_trace_rs::trace("TC-024", "FR-014-AC-2")]
#[ix_trace_rs::trace("TC-024", "FR-014-AC-6")]
#[test]
fn original_red_green_truth_table_agrees_with_owner_and_counters() {
    let (compiled, package) = fixture::compile(
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/colors.native")),
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/colors.json")),
    );
    let values = fixture::colors(&compiled, &package);
    let red = &values["Red"];
    let green = &values["Green"];
    let cases: [(Compare, ComparisonOperator, &EnumValue, bool); 4] = [
        (
            generated::redEqualsRed,
            ComparisonOperator::Equal,
            red,
            true,
        ),
        (
            generated::redEqualsGreen,
            ComparisonOperator::Equal,
            green,
            false,
        ),
        (
            generated::redDiffersFromRed,
            ComparisonOperator::NotEqual,
            red,
            false,
        ),
        (
            generated::redDiffersFromGreen,
            ComparisonOperator::NotEqual,
            green,
            true,
        ),
    ];
    for (oracle, operator, right, expected) in cases {
        for work_units in [u64::MAX, 0] {
            let limits = ScalarLimits {
                integer_bits: u64::MAX,
                decimal_digits: u64::MAX,
                scale_expansion: u64::MAX,
                text_input_bytes: u64::MAX,
                text_scalars: u64::MAX,
                normalized_scalars: u64::MAX,
                unit_edges: u64::MAX,
                value_occurrences: u64::MAX,
                work_units,
                result_units: u64::MAX,
            };
            let mut direct_meter = Meter::new(limits);
            let direct = compare_enum(operator, red, right, &mut direct_meter);
            let mut generated_meter = Meter::new(limits);
            let actual = oracle(red, right, &mut generated_meter)
                .expect("a genuine model member is a valid generated input");
            if work_units == u64::MAX {
                assert_eq!(
                    actual,
                    Outcome::Completed(expected),
                    "the authored truth arm"
                );
            }
            assert_eq!(format!("{actual:?}"), format!("{direct:?}"));
            for kind in LimitKind::ALL {
                assert_eq!(generated_meter.consumed(kind), direct_meter.consumed(kind));
            }
            assert_eq!(
                generated_meter.admitted_charges(),
                direct_meter.admitted_charges()
            );
        }
    }
}

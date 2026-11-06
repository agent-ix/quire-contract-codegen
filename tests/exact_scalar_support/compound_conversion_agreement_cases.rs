//! Generated conversions over reader-admitted compound quantity forms.

#[path = "agreement.rs"]
#[macro_use]
mod support;

mod generated {
    pub use quire_exact_scalar_oracles::*;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}

use generated::{compound_source, compound_target, quantity_square, OracleStop};
use ix_trace_rs::trace;
use quire_exact::{IllTyped, Integer, Meter, Outcome, Quantity, Rational};
use quire_semantic_value::quantity::{IdentifiedUnit, UnitQuantity};

fn typed(
    result: Result<Outcome<Quantity>, OracleStop>,
) -> Result<Outcome<Quantity>, IllTyped> {
    result.map_err(|stop| match stop {
        OracleStop::IllTyped(cause) => cause,
        other => panic!("unexpected generated stop: {other:?}"),
    })
}

fn direct(
    source: UnitQuantity<'_>,
    target: &IdentifiedUnit,
    meter: &mut Meter,
) -> Result<Outcome<Quantity>, IllTyped> {
    use quire_semantic_value::quantity::{convert_quantity, ConvertedValue, QuantityTarget};

    convert_quantity(source, target.unit(), &QuantityTarget::Exact, meter).map(|outcome| {
        match outcome {
            Outcome::Completed(converted) => {
                let ConvertedValue::Exact(value) = converted.value() else {
                    panic!("exact conversion returned a rounded value")
                };
                Outcome::Completed(Quantity::new(value.clone(), target.id()))
            }
            Outcome::Undefined(reason) => Outcome::Undefined(reason),
            Outcome::Refused(reason) => Outcome::Refused(reason),
            Outcome::Incomplete(reason) => Outcome::Incomplete(reason),
        }
    })
}

fn compound(
    fixture: &mut support::rt_side::Fixture,
    magnitude: i64,
    terms: &[(&str, i64)],
) -> (Quantity, IdentifiedUnit) {
    use quire_semantic_value::quantity::{result_unit, UnitOperation};

    let metre = fixture.unit("m");
    let mut unit = result_unit(UnitOperation::Divide, metre.unit(), metre.unit())
        .expect("dimensionless base");
    for (name, exponent) in terms {
        let operand = fixture.unit(name);
        let operation = if *exponent < 0 {
            UnitOperation::Divide
        } else {
            UnitOperation::Multiply
        };
        for _ in 0..exponent.unsigned_abs() {
            unit = result_unit(operation, &unit, operand.unit()).expect("compound term");
        }
    }
    let identified = IdentifiedUnit::new(unit.clone()).expect("compound identity");
    assert_eq!(fixture.units.insert(unit).expect("retain compound"), identified.id());
    (
        Quantity::new(Rational::from_integer(Integer::from(magnitude)), identified.id()),
        identified,
    )
}

#[trace("FR-014-AC-61", "TC-024")]
#[test]
fn compound_source_u22_agrees_in_value_charges_counters_and_denials() {
    let mut fixture = support::rt_side::named_fixture(&support::GraphSpec::conversions());
    for (target_name, bounds, expected) in [
        ("m2", support::rt_side::unit_tuple([3, 0, 0, 1, 3, 1]), Some(4)),
        ("cm2", support::rt_side::unit_tuple([17, 0, 1, 1, 6, 1]), Some(40000)),
        ("cm2", support::rt_side::unit_tuple([16, 0, 1, 1, 6, 1]), None),
    ] {
        let (source, _) = compound(&mut fixture, 4, &[("m", 2)]);
        let target = fixture.unit(target_name);
        let source = fixture.resolve(&source);
        agree2! {
            limits: bounds,
            setup: {},
            direct: |meter| direct(source, &target, meter),
            generated: |meter| typed(compound_source(source, &target, meter)),
        };
        let actual = support::rt_side::metered(bounds, |meter| {
            typed(compound_source(source, &target, meter))
        });
        match expected {
            Some(value) => assert_eq!(
                actual.0,
                Ok(Outcome::Completed(Quantity::new(
                    support::rt_side::ratio(value, 1),
                    target.id(),
                ))),
                "{target_name}"
            ),
            None => assert!(matches!(actual.0, Ok(Outcome::Incomplete(_)))),
        }
    }
}

#[trace("FR-014-AC-60", "TC-024")]
#[test]
fn compound_target_u25_first_step_agrees_in_value_charges_counters_and_denials() {
    let mut fixture = support::rt_side::named_fixture(&support::GraphSpec::conversions());
    let (_, target) = compound(&mut fixture, 1, &[("kg", 1), ("m", 2), ("s", -2)]);
    let source = fixture.qi(1, "N_m");
    let source = fixture.resolve(&source);
    let bounds = support::rt_side::unit_tuple([1, 0, 0, 1, 3, 1]);
    agree2! {
        limits: bounds,
        setup: {},
        direct: |meter| direct(source, &target, meter),
        generated: |meter| typed(compound_target(source, &target, meter)),
    };
    let actual = support::rt_side::metered(bounds, |meter| {
        typed(compound_target(source, &target, meter))
    });
    assert_eq!(
        actual.0,
        Ok(Outcome::Completed(Quantity::new(
            support::rt_side::ratio(1, 1),
            target.id(),
        )))
    );
}

/// The exponent is an inline Integer literal in the admitted package. Its
/// generated oracle must still agree with the owning runtime on the U12 square.
#[trace("FR-014-AC-59", "TC-024")]
#[test]
fn inline_integer_exponent_u12_square_agrees_in_value_charges_counters_and_denials() {
    use quire_semantic_value::quantity::{evaluate_quantity_unit, QuantityOperation};

    let mut fixture = support::rt_side::named_fixture(&support::GraphSpec::conversions());
    let (expected, expected_unit) = compound(&mut fixture, 4, &[("m", 2)]);
    let base = fixture.qi(2, "m");
    let base = fixture.resolve(&base);
    let exponent = Integer::from(2);
    let bounds = support::rt_side::UNLIMITED;
    let generated = |meter: &mut Meter| {
        quantity_square(base, &exponent, meter).map_err(|stop| match stop {
            OracleStop::Quantity(cause) => cause,
            other => panic!("unexpected generated stop: {other:?}"),
        })
    };
    agree2! {
        limits: bounds,
        setup: {},
        direct: |meter| evaluate_quantity_unit(QuantityOperation::Power(base, &exponent), meter),
        generated: |meter| generated(meter),
    };
    let actual = support::rt_side::metered(bounds, generated);
    let (value, unit) = match actual.0 {
        Ok((Outcome::Completed(value), unit)) => (value, unit),
        other => panic!("U12 square did not complete: {other:?}"),
    };
    assert_eq!(value, expected);
    assert_eq!(unit, expected_unit);
}

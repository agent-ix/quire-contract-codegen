//! Generated conversions over reader-admitted compound quantity forms.

#[path = "agreement.rs"]
#[macro_use]
mod support;

mod generated {
    pub use quire_exact_scalar_oracles::*;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}

use generated::{
    compound_chain, compound_source, compound_target, quantity_product, quantity_square, OracleStop,
};
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

fn compound_unit(fixture: &support::rt_side::Fixture, terms: &[(&str, i64)]) -> IdentifiedUnit {
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
    IdentifiedUnit::new(unit).expect("compound identity")
}

fn record_compound_source(
    fixture: &mut support::rt_side::Fixture,
    magnitude: i64,
    terms: &[(&str, i64)],
) -> (Quantity, IdentifiedUnit) {
    let identified = compound_unit(fixture, terms);
    assert_eq!(
        fixture
            .units
            .insert(identified.unit().clone())
            .expect("retain compound"),
        identified.id()
    );
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
        let (source, _) = record_compound_source(&mut fixture, 4, &[("m", 2)]);
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
    let fixture = support::rt_side::named_fixture(&support::GraphSpec::conversions());
    let target = compound_unit(&fixture, &[("kg", 1), ("m", 2), ("s", -2)]);
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
    let first = actual.0.expect("first conversion is well typed").completed().unwrap();
    let mut scope = support::rt_side::UnitScope::new(&fixture.units);
    assert!(scope.resolve(&first).is_none(), "compound metadata is not in the source table");
    scope.form(target);
    let joule = fixture.unit("J");
    let input = scope.resolve(&first).expect("caller retained the compound unit");
    agree2! {
        limits: bounds,
        setup: {},
        direct: |meter| direct(input, &joule, meter),
        generated: |meter| typed(compound_source(input, &joule, meter)),
    };
    let second = support::rt_side::metered(bounds, |meter| {
        typed(compound_source(input, &joule, meter))
    });
    assert_eq!(
        second.0,
        Ok(Outcome::Completed(Quantity::new(
            support::rt_side::ratio(1, 1),
            joule.id(),
        )))
    );
}

/// The exponent is an inline Integer literal in the admitted package. Its
/// generated oracle must still agree with the owning runtime on the U12 square.
#[trace("FR-014-AC-58", "TC-024")]
#[test]
fn inline_integer_exponent_u12_square_agrees_in_value_charges_counters_and_denials() {
    let fixture = support::rt_side::named_fixture(&support::GraphSpec::conversions());
    let expected_unit = compound_unit(&fixture, &[("m", 2)]);
    let expected = Quantity::new(support::rt_side::ratio(4, 1), expected_unit.id());
    let base = fixture.qi(2, "m");
    let base = fixture.resolve(&base);
    let exponent = Integer::from(2_i64);
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

#[trace("FR-014-AC-59", "TC-024")]
#[test]
fn generated_compound_product_chains_into_another_multiply() {
    use quire_semantic_value::quantity::UnitScope;

    let fixture = support::rt_side::named_fixture(&support::GraphSpec::conversions());
    let (left, right, following) = (
        fixture.qi(2, "m"),
        fixture.qi(3, "s"),
        fixture.qi(2, "m"),
    );
    let left = fixture.resolve(&left);
    let right = fixture.resolve(&right);
    let bounds = support::rt_side::UNLIMITED;
    let typed_quantity = |result: Result<_, OracleStop>| {
        result.map_err(|stop| match stop {
            OracleStop::Quantity(cause) => cause,
            other => panic!("unexpected generated stop: {other:?}"),
        })
    };
    agree2! {
        limits: bounds,
        setup: {},
        direct: |meter| evaluate_quantity_unit(QuantityOperation::Multiply(left, right), meter),
        generated: |meter| typed_quantity(quantity_product(left, right, meter)),
    };
    let first = support::rt_side::metered(bounds, |meter| {
        typed_quantity(quantity_product(left, right, meter))
    });
    let (product, product_unit) = match first.0 {
        Ok((Outcome::Completed(product), unit)) => (product, unit),
        other => panic!("first generated product did not complete: {other:?}"),
    };
    assert_eq!(*product.magnitude(), support::rt_side::ratio(6, 1));
    let mut scope = UnitScope::new(&fixture.units);
    assert!(scope.resolve(&product).is_none(), "compound result needs its paired unit");
    scope.form(product_unit);
    let product = scope.resolve(&product).expect("retained compound result");
    let following = scope.resolve(&following).expect("retained source unit");
    agree2! {
        limits: bounds,
        setup: {},
        direct: |meter| evaluate_quantity_unit(QuantityOperation::Multiply(product, following), meter),
        generated: |meter| typed_quantity(compound_chain(product, following, meter)),
    };
    let second = support::rt_side::metered(bounds, |meter| {
        typed_quantity(compound_chain(product, following, meter))
    });
    let (value, unit) = match second.0 {
        Ok((Outcome::Completed(value), unit)) => (value, unit),
        other => panic!("chained generated product did not complete: {other:?}"),
    };
    assert_eq!(*value.magnitude(), support::rt_side::ratio(12, 1));
    assert_eq!(value.unit(), &unit.id());
    assert_eq!(unit, compound_unit(&fixture, &[("m", 2), ("s", 1)]));
}

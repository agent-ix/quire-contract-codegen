//! Native execution of the published CQ-E vectors, without reproducing conversion semantics.
#[path = "agreement.rs"]
#[macro_use]
mod support;
mod generated {
    pub use quire_exact_scalar_oracles::*;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/names.rs"));
}
use ix_trace_rs::trace;
use quire_exact::{
    CarriedQuantityType, DecimalLoss, DecimalType, IllTyped, Integer, IntegerInterval, Meter,
    Outcome, QuantityCarrier, Rational, RationalDomain, RoundingMode,
};
use quire_semantic_value::quantity::{
    convert_quantity, Conversion, ConvertedValue, QuantityTarget,
};
use serde_json::Value;

fn carrier(vectors: &Value) -> QuantityCarrier {
    let target = &vectors["nodes"][generated::TARGET];
    let node = &vectors["nodes"][target["carrier"].as_str().unwrap()];
    let integer = |name: &str| node[name].to_string().parse::<Integer>().unwrap();
    let interval = |lo: &str, hi: &str| IntegerInterval::new(integer(lo), integer(hi)).unwrap();
    let rounding = RoundingMode::from_code(target["rounding"].as_str().unwrap()).unwrap();
    match node["form"].as_str().unwrap() {
        "integer_range" => QuantityCarrier::Integer {
            domain: interval("min", "max"),
            rounding,
        },
        "decimal_range" => QuantityCarrier::Decimal(
            DecimalType::new(
                integer("coefficient_min"),
                integer("coefficient_max"),
                node["scale_min"].as_u64().unwrap(),
                node["scale_max"].as_u64().unwrap(),
                rounding,
            )
            .unwrap(),
        ),
        "rational_range" => QuantityCarrier::Rational(
            RationalDomain::new(
                interval("numerator_min", "numerator_max"),
                interval("denominator_min", "denominator_max"),
            )
            .unwrap(),
        ),
        other => panic!("unknown published carrier {other}"),
    }
}
fn loss_json(loss: Option<&DecimalLoss>) -> Value {
    loss.map_or(Value::Null, |loss| {
        serde_json::json!({
            "exact_numerator":loss.exact_numerator().to_string().parse::<i64>().unwrap(),
            "exact_denominator":loss.exact_denominator().to_string().parse::<i64>().unwrap(),
            "rounded_coefficient":loss.rounded_coefficient().to_string().parse::<i64>().unwrap(),
            "rounded_scale":loss.rounded_scale(),"mode":loss.mode().as_str()
        })
    })
}
fn project(
    result: Result<Outcome<(Conversion, CarriedQuantityType)>, generated::OracleStop>,
    expected: &CarriedQuantityType,
) -> Result<Outcome<Conversion>, IllTyped> {
    result
        .map_err(|stop| match stop {
            generated::OracleStop::IllTyped(cause) => cause,
            other => panic!("unexpected generated stop {other:?}"),
        })
        .map(|outcome| match outcome {
            Outcome::Completed((conversion, retained)) => {
                assert_eq!(
                    &retained, expected,
                    "full bounded carrier identity survives generation"
                );
                Outcome::Completed(conversion)
            }
            Outcome::Refused(reason) => Outcome::Refused(reason),
            Outcome::Incomplete(reason) => Outcome::Incomplete(reason),
            Outcome::Undefined(reason) => Outcome::Undefined(reason),
        })
}
#[trace(
    "FR-014-AC-62",
    "FR-014-AC-63",
    "FR-014-AC-64",
    "FR-014-AC-65",
    "TC-024"
)]
#[test]
fn published_carried_values_losses_units_and_accounting_agree() {
    let root =
        std::path::PathBuf::from(std::env::var_os("QSPEC_DIR").expect("published QSpec checkout"));
    let vectors: Value = serde_json::from_slice(
        &std::fs::read(root.join("proposals/checked-package-v2/quantity-carrier-vectors.json"))
            .unwrap(),
    )
    .unwrap();
    let rows = vectors["evaluation"].as_array().unwrap();
    assert_eq!(
        rows.iter()
            .map(|r| r["id"].as_str().unwrap().to_owned())
            .collect::<Vec<_>>(),
        (1..=26).map(|i| format!("CQ-E{i:02}")).collect::<Vec<_>>()
    );
    let fixture = support::rt_side::named_fixture(&support::GraphSpec::conversions());
    let target = fixture.unit("m");
    let carrier = carrier(&vectors);
    let expected_type = CarriedQuantityType::new(target.id(), carrier.clone());
    let mut executed = 0;
    for row in rows.iter().filter(|row| row["target"] == generated::TARGET) {
        executed += 1;
        let id = row["id"].as_str().unwrap();
        let spelling = row["source"]["magnitude"].as_str().unwrap();
        let (n, d) = spelling.split_once('/').unwrap_or((spelling, "1"));
        let source = fixture.q(
            Rational::new(n.parse().unwrap(), d.parse().unwrap()).unwrap(),
            row["source"]["unit"].as_str().unwrap(),
        );
        let source = fixture.resolve(&source);
        let limits = if row["limits"].is_null() {
            support::rt_side::UNLIMITED
        } else {
            support::rt_side::unit_tuple(std::array::from_fn(|i| {
                row["limits"][i].as_u64().unwrap()
            }))
        };
        agree2! { limits: limits, setup: {}, direct: |meter| convert_quantity(source,target.unit(),&QuantityTarget::Carried(carrier.clone()),meter), generated: |meter| project(generated::convert(source,&target,meter),&expected_type), };
        let observed = support::rt_side::metered(limits, |meter| {
            project(generated::convert(source, &target, meter), &expected_type)
        });
        let expected = &row["expected"];
        match observed.0.expect("compatible quantity dimensions") {
            Outcome::Completed(conversion) => {
                assert_eq!(expected["disposition"], "completed", "{id}");
                assert_eq!(conversion.unit(), target.unit(), "{id}: target unit");
                match conversion.value() {
                    ConvertedValue::Exact(value) => {
                        assert_eq!(
                            value.numerator().to_string(),
                            expected["numerator"].as_str().unwrap(),
                            "{id}"
                        );
                        assert_eq!(
                            value.denominator().to_string(),
                            expected["denominator"].as_str().unwrap(),
                            "{id}"
                        );
                    }
                    ConvertedValue::Decimal(result) => {
                        assert_eq!(
                            result.value().coefficient().to_string(),
                            expected["coefficient"].to_string(),
                            "{id}"
                        );
                        assert_eq!(
                            u64::from(result.value().scale()),
                            expected["scale"].as_u64().unwrap(),
                            "{id}"
                        );
                        assert_eq!(loss_json(result.loss()), expected["loss"], "{id}");
                    }
                    ConvertedValue::Integer { value, loss } => {
                        assert_eq!(
                            value.value().to_string(),
                            expected["integer"].to_string(),
                            "{id}"
                        );
                        assert_eq!(loss_json(loss.as_ref()), expected["loss"], "{id}");
                    }
                }
            }
            Outcome::Refused(reason) => {
                assert_eq!(expected["disposition"], "refused", "{id}");
                assert_eq!(reason.code(), expected["code"].as_str(), "{id}");
                assert_eq!(reason.cause(), expected["cause"].as_str(), "{id}");
            }
            Outcome::Incomplete(reason) => {
                assert_eq!(expected["disposition"], "incomplete", "{id}");
                assert_eq!(
                    reason.limit_kind.as_str(),
                    expected["limit_kind"].as_str().unwrap(),
                    "{id}"
                );
                assert_eq!(reason.limit, expected["limit"].as_u64().unwrap(), "{id}");
                assert_eq!(
                    reason.consumed,
                    expected["consumed"].as_u64().unwrap(),
                    "{id}"
                );
                assert_eq!(
                    reason.next_charge.to_string(),
                    expected["next_charge"].to_string(),
                    "{id}"
                );
                assert_eq!(
                    reason.charge_point.as_str(),
                    expected["charge_point"].as_str().unwrap(),
                    "{id}"
                );
            }
            other => panic!("{id}: unexpected outcome {other:?}"),
        }
    }
    assert!(
        executed > 0,
        "every generated carrier executes published vectors"
    );
}

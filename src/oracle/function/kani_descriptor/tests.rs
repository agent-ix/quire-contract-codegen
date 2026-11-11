use super::*;
use ix_trace_rs::trace;
use quire_exact::{
    BoundedQuantityType, CardinalityBound, CarriedQuantityType, CollectionKind, CollectionType,
    DecimalType, EffectiveId, EnumShape, FloatType, IeeeWidth, Integer, IntegerInterval, MapType,
    NodeKey, OriginalDeclarationKey, PathSegment, Quantity, QuantityCarrier, Rational,
    RationalDomain, RoundingMode, TextProfile, TextType, UnitId, ValueType, VariantId,
};
use std::path::Path;
use std::process::Command;

fn int(text: &str) -> Integer {
    text.parse().expect("canonical integer")
}

fn interval(lower: &str, upper: &str) -> IntegerInterval {
    IntegerInterval::new(int(lower), int(upper)).expect("ordered interval")
}

fn key(ordinal: u64) -> NodeKey {
    NodeKey::new("pkg\"é", vec![PathSegment::Text("a\n".to_owned())], ordinal).expect("key")
}

fn nested_key() -> NodeKey {
    NodeKey::new(
        "pkg",
        vec![
            PathSegment::Text(String::new()),
            PathSegment::Node(key(1)),
            PathSegment::Node(
                NodeKey::new("pkg", vec![PathSegment::Node(key(2))], 3).expect("nested"),
            ),
        ],
        4,
    )
    .expect("nested key")
}

fn quantity(numerator: &str, denominator: &str, unit: UnitId) -> Quantity {
    Quantity::new(
        Rational::new(int(numerator), int(denominator)).expect("rational"),
        unit,
    )
}

fn compound() -> UnitId {
    UnitId::compound(vec![(key(5), int("-2")), (nested_key(), int("3"))])
}

/// One value per `ValueType` variant (twenty), plus the distinct shapes inside a variant
/// (carriers, units, bounds) and nested wrappers.
fn every_variant() -> Vec<ValueType> {
    let decimal = DecimalType::new(
        int("-100000000000000000000000000000000000000"),
        int("100"),
        0,
        2,
        RoundingMode::NearestEven,
    )
    .expect("decimal");
    let rational = RationalDomain::new(interval("-3", "4"), interval("1", "9")).expect("domain");
    let mut reference = EffectiveId::new(None, OriginalDeclarationKey::new("p", "n"));
    reference = EffectiveId::new(Some(reference), OriginalDeclarationKey::new("q", "m\t"));
    vec![
        ValueType::Boolean,
        ValueType::Integer,
        ValueType::Int(interval("-5", "100000000000000000000000000000000000000")),
        ValueType::Rational(rational.clone()),
        ValueType::Decimal(decimal.clone()),
        ValueType::Float(FloatType::new(
            IeeeWidth::Binary32,
            RoundingMode::TowardZero,
        )),
        ValueType::Float(FloatType::exact(IeeeWidth::Binary64)),
        ValueType::Quantity(UnitId::declared(key(6))),
        ValueType::Quantity(compound()),
        ValueType::carried_quantity(CarriedQuantityType::new(
            UnitId::declared(key(7)),
            QuantityCarrier::Decimal(decimal),
        )),
        ValueType::carried_quantity(CarriedQuantityType::new(
            compound(),
            QuantityCarrier::Integer {
                domain: interval("0", "9"),
                rounding: RoundingMode::TowardNegative,
            },
        )),
        ValueType::carried_quantity(CarriedQuantityType::new(
            UnitId::declared(key(8)),
            QuantityCarrier::Rational(rational),
        )),
        ValueType::Text(TextType::new(0, 16, TextProfile::Nfc).expect("text")),
        ValueType::Text(TextType::new(2, 2, TextProfile::BinaryUtf8).expect("text")),
        ValueType::String,
        ValueType::Uuid,
        ValueType::Timestamp,
        ValueType::Enum(EnumShape::new(
            true,
            [
                VariantId::new(key(9), "low").expect("variant"),
                VariantId::new(key(9), "high").expect("variant"),
            ],
        )),
        ValueType::option(ValueType::Boolean),
        ValueType::Composite(nested_key()),
        ValueType::collection(CollectionType::new(
            CollectionKind::OrderedSet,
            ValueType::Integer,
            Some(CardinalityBound::new(1, 3).expect("bound")),
        )),
        ValueType::collection(CollectionType::new(
            CollectionKind::Bag,
            ValueType::Boolean,
            None,
        )),
        ValueType::map(
            MapType::new(
                ValueType::String,
                ValueType::option(ValueType::Integer),
                Some(CardinalityBound::at_most(4)),
            )
            .expect("map"),
        ),
        ValueType::bounded_quantity(
            BoundedQuantityType::new(
                UnitId::declared(key(10)),
                Some(quantity("-1", "3", UnitId::declared(key(10)))),
                None,
            )
            .expect("bounded"),
        ),
        ValueType::Reference(reference),
        ValueType::Population(Some(7)),
        ValueType::Population(None),
        ValueType::option(ValueType::collection(CollectionType::new(
            CollectionKind::Sequence,
            ValueType::map(
                MapType::new(ValueType::Uuid, ValueType::Composite(key(11)), None).expect("map"),
            ),
            None,
        ))),
    ]
}

fn variant_name(value: &ValueType) -> &'static str {
    match value {
        ValueType::Boolean => "Boolean",
        ValueType::Integer => "Integer",
        ValueType::Int(_) => "Int",
        ValueType::Rational(_) => "Rational",
        ValueType::Decimal(_) => "Decimal",
        ValueType::Float(_) => "Float",
        ValueType::Quantity(_) => "Quantity",
        ValueType::CarriedQuantity(_) => "CarriedQuantity",
        ValueType::Text(_) => "Text",
        ValueType::String => "String",
        ValueType::Uuid => "Uuid",
        ValueType::Timestamp => "Timestamp",
        ValueType::Enum(_) => "Enum",
        ValueType::Option(_) => "Option",
        ValueType::Composite(_) => "Composite",
        ValueType::Collection(_) => "Collection",
        ValueType::Map(_) => "Map",
        ValueType::BoundedQuantity(_) => "BoundedQuantity",
        ValueType::Reference(_) => "Reference",
        ValueType::Population(_) => "Population",
    }
}

/// The scratch crate that rebuilds every rendered expression and prints each value's `Debug`.
fn rebuilt_debug(name: &str, expressions: &[String]) -> String {
    let base = std::env::var_os("CARGO_TARGET_DIR").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("target"),
        std::path::PathBuf::from,
    );
    let directory = base.join(format!("kani-descriptor-roundtrip-{name}"));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(directory.join("src")).expect("scratch directory");
    std::fs::write(
        directory.join("Cargo.toml"),
        format!(
            "[package]\nname = \"kani-descriptor-roundtrip-{}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\nquire-exact = {{ {} }}\n\n[workspace]\n",
            name,
            crate::core::profile::EXACT_DEPENDENCY_SOURCE
        ),
    )
    .expect("scratch manifest");
    std::fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
        directory.join("Cargo.lock"),
    )
    .expect("seed the lock");
    std::fs::write(
        directory.join("src/main.rs"),
        format!(
            "use quire_exact as exact;\nfn rebuilt<T, E>(result: Result<T, E>) -> T {{ match result {{ Ok(value) => value, Err(_) => panic!(\"rebuild failed\") }} }}\nfn main() {{\n    let values: Vec<exact::ValueType> = vec![{}];\n    for value in &values {{ print!(\"{{value:?}}\\u{{1f}}\"); }}\n}}\n",
            expressions.join(",\n        ")
        ),
    )
    .expect("scratch main");
    // Adapt the seeded lock to the scratch package offline, then require it unchanged.
    let adapted = Command::new(env!("CARGO"))
        .args(["metadata", "--offline", "--format-version", "1"])
        .env("CARGO_TARGET_DIR", &base)
        .current_dir(&directory)
        .output()
        .expect("adapt the seeded lock");
    assert!(
        adapted.status.success(),
        "lock adaptation failed:\n{}",
        String::from_utf8_lossy(&adapted.stderr)
    );
    let output = Command::new(env!("CARGO"))
        .args(["run", "--locked", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", &base)
        .current_dir(&directory)
        .output()
        .expect("run the scratch crate");
    assert!(
        output.status.success(),
        "rendered descriptors failed to build or rebuild:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("utf8 output")
}

/// FR-021-AC-44: the renderer covers every `ValueType` variant, and each rendered expression
/// rebuilds, through the public constructors alone, a value whose `Debug` equals the original.
#[trace("TC-031", "FR-021-AC-44")]
#[test]
fn every_value_type_variant_rebuilds_from_its_rendered_descriptor() {
    let values = every_variant();
    let covered: std::collections::BTreeSet<_> = values.iter().map(variant_name).collect();
    assert_eq!(covered.len(), 20, "every variant has a case: {covered:?}");
    let expressions = values
        .iter()
        .map(|value| value_type(value).expect("every variant renders"))
        .collect::<Vec<_>>();
    for expression in &expressions {
        syn::parse_str::<syn::Expr>(expression).expect("rendered descriptor is an expression");
    }
    let printed = rebuilt_debug("all", &expressions);
    let rebuilt: Vec<&str> = printed.trim_end_matches('\u{1f}').split('\u{1f}').collect();
    assert_eq!(rebuilt.len(), values.len());
    for (value, rebuilt) in values.iter().zip(rebuilt) {
        assert_eq!(format!("{value:?}"), rebuilt, "{}", variant_name(value));
    }
}

/// The round trip can fail: a renderer that drops a part yields a value with a different `Debug`.
#[trace("TC-031", "FR-021-AC-44")]
#[test]
fn the_round_trip_distinguishes_a_dropped_part() {
    let original = ValueType::Int(interval("0", "9"));
    let expression = value_type(&original).expect("renders");
    let mutated = expression.replace("\"9\"", "\"8\"");
    assert_ne!(expression, mutated, "the mutation changed the rendering");
    let printed = rebuilt_debug("mutated", &[mutated]);
    assert_ne!(
        format!("{original:?}"),
        printed.trim_end_matches('\u{1f}'),
        "a changed bound rebuilds a different value"
    );
}

#[test]
fn node_keys_render_without_recursion_at_depth() {
    let mut key =
        NodeKey::new("deep", vec![PathSegment::Text("leaf".to_owned())], 0).expect("leaf");
    for ordinal in 1..2_000u64 {
        key = NodeKey::new("deep", vec![PathSegment::Node(key)], ordinal).expect("wrapper");
    }
    let rendered = node_key(&key).expect("a deep key renders");
    assert_eq!(rendered.matches("exact::NodeKey::new(").count(), 2_000);
}

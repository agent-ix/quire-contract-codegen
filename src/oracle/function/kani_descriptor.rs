// SPDX-License-Identifier: AGPL-3.0-or-later
//! Literal descriptors for the Kani function harness (FR-021, IR-1244).
//!
//! `NodeKey`, `ValueType` and their parts hold heap or `Arc` state behind fallible
//! constructors, so none is a Rust `const`. Each value is instead rendered as an
//! expression that rebuilds it through its genuine public constructors inside the
//! proof. Rendering is iterative, so type and key depth do not use the call stack.

use quire_exact::{
    CardinalityBound, CarriedQuantityType, CollectionKind, CollectionType, DecimalType,
    EffectiveId, EnumShape, FloatType, IeeeWidth, Integer, IntegerInterval, MapType, NodeKey,
    PathSegment, Quantity, QuantityCarrier, RationalDomain, RoundingMode, TextProfile, UnitId,
    ValueType,
};

use crate::oracle::claim::OracleGenerationError;

type Rendered = Result<String, OracleGenerationError>;

fn pop(rendered: &mut Vec<String>) -> Result<String, OracleGenerationError> {
    rendered
        .pop()
        .ok_or(OracleGenerationError::InvalidPlanState)
}

/// An expression rebuilding `key` through `NodeKey::new`, without recursion.
pub(super) fn node_key(key: &NodeKey) -> Rendered {
    enum Step {
        Visit(NodeKey, bool),
        Text(String),
        Finish {
            package: String,
            ordinal: u64,
            len: usize,
            root: bool,
        },
    }
    let mut pending = vec![Step::Visit(key.clone(), true)];
    let mut rendered: Vec<String> = Vec::new();
    while let Some(step) = pending.pop() {
        match step {
            Step::Visit(node, root) => {
                let path = node.path();
                pending.push(Step::Finish {
                    package: node.package().to_owned(),
                    ordinal: node.ordinal(),
                    len: path.len(),
                    root,
                });
                for segment in path.into_iter().rev() {
                    match segment {
                        PathSegment::Text(text) => pending.push(Step::Text(text)),
                        PathSegment::Node(child) => pending.push(Step::Visit(child, false)),
                    }
                }
            }
            Step::Text(text) => {
                rendered.push(format!("exact::PathSegment::Text({text:?}.to_owned())"))
            }
            Step::Finish {
                package,
                ordinal,
                len,
                root,
            } => {
                let start = rendered
                    .len()
                    .checked_sub(len)
                    .ok_or(OracleGenerationError::InvalidPlanState)?;
                let path = rendered.split_off(start).join(", ");
                let expression =
                    format!("rebuilt(exact::NodeKey::new({package:?}, vec![{path}], {ordinal}))");
                rendered.push(if root {
                    expression
                } else {
                    format!("exact::PathSegment::Node({expression})")
                });
            }
        }
    }
    let node = pop(&mut rendered)?;
    if rendered.is_empty() {
        Ok(node)
    } else {
        Err(OracleGenerationError::InvalidPlanState)
    }
}

fn integer(value: &Integer) -> String {
    format!("rebuilt({:?}.parse::<exact::Integer>())", value.to_string())
}

fn interval(value: &IntegerInterval) -> String {
    format!(
        "rebuilt(exact::IntegerInterval::new({}, {}))",
        integer(value.lower()),
        integer(value.upper())
    )
}

fn rational_domain(value: &RationalDomain) -> String {
    format!(
        "rebuilt(exact::RationalDomain::new({}, {}))",
        interval(value.numerator()),
        interval(value.denominator())
    )
}

fn rounding(mode: RoundingMode) -> Result<&'static str, OracleGenerationError> {
    Ok(match mode {
        RoundingMode::Exact => "exact::RoundingMode::Exact",
        RoundingMode::TowardZero => "exact::RoundingMode::TowardZero",
        RoundingMode::TowardPositive => "exact::RoundingMode::TowardPositive",
        RoundingMode::TowardNegative => "exact::RoundingMode::TowardNegative",
        RoundingMode::NearestEven => "exact::RoundingMode::NearestEven",
        RoundingMode::NearestAway => "exact::RoundingMode::NearestAway",
    })
}

fn unit(value: &UnitId) -> Rendered {
    match value {
        UnitId::Declared(key) => Ok(format!("exact::UnitId::declared({})", node_key(key)?)),
        UnitId::Compound(terms) => {
            let terms = terms
                .iter()
                .map(|(key, exponent)| Ok(format!("({}, {})", node_key(key)?, integer(exponent))))
                .collect::<Result<Vec<_>, OracleGenerationError>>()?;
            Ok(format!(
                "exact::UnitId::compound(vec![{}])",
                terms.join(", ")
            ))
        }
    }
}

fn quantity(value: &Quantity) -> Rendered {
    Ok(format!(
        "exact::Quantity::new(rebuilt(exact::Rational::new({}, {})), {})",
        integer(value.magnitude().numerator()),
        integer(value.magnitude().denominator()),
        unit(value.unit())?
    ))
}

fn optional_quantity(value: Option<&Quantity>) -> Rendered {
    value.map_or_else(
        || Ok("None".to_owned()),
        |quantity_value| Ok(format!("Some({})", quantity(quantity_value)?)),
    )
}

fn bound(value: Option<CardinalityBound>) -> String {
    value.map_or_else(
        || "None".to_owned(),
        |bound| {
            format!(
                "Some(rebuilt(exact::CardinalityBound::new({}, {})))",
                bound.minimum(),
                bound.maximum()
            )
        },
    )
}

fn kind(value: CollectionKind) -> &'static str {
    match value {
        CollectionKind::Sequence => "exact::CollectionKind::Sequence",
        CollectionKind::Set => "exact::CollectionKind::Set",
        CollectionKind::Bag => "exact::CollectionKind::Bag",
        CollectionKind::OrderedSet => "exact::CollectionKind::OrderedSet",
    }
}

fn profile(value: TextProfile) -> &'static str {
    match value {
        TextProfile::UnicodeScalars => "exact::TextProfile::UnicodeScalars",
        TextProfile::Nfc => "exact::TextProfile::Nfc",
        TextProfile::Nfd => "exact::TextProfile::Nfd",
        TextProfile::Nfkc => "exact::TextProfile::Nfkc",
        TextProfile::Nfkd => "exact::TextProfile::Nfkd",
        TextProfile::BinaryUtf8 => "exact::TextProfile::BinaryUtf8",
    }
}

fn width(value: IeeeWidth) -> &'static str {
    match value {
        IeeeWidth::Binary32 => "exact::IeeeWidth::Binary32",
        IeeeWidth::Binary64 => "exact::IeeeWidth::Binary64",
    }
}

fn effective_id(value: &EffectiveId) -> String {
    let mut text = String::from("{ let id = None;");
    for original in value.owner_chain() {
        text.push_str(&format!(
            " let id = Some(exact::EffectiveId::new(id, exact::OriginalDeclarationKey::new({:?}, {:?})));",
            original.package(),
            original.node()
        ));
    }
    text.push_str(" rebuilt(id.ok_or(())) }");
    text
}

fn enum_shape(shape: &EnumShape) -> Rendered {
    let variants = shape
        .variants()
        .map(|variant| {
            Ok(format!(
                "rebuilt(exact::VariantId::new({}, {:?}))",
                node_key(variant.owner())?,
                variant.case()
            ))
        })
        .collect::<Result<Vec<_>, OracleGenerationError>>()?;
    Ok(format!(
        "exact::EnumShape::new({}, vec![{}])",
        shape.is_ordered(),
        variants.join(", ")
    ))
}

fn carrier(value: &QuantityCarrier) -> Rendered {
    Ok(match value {
        QuantityCarrier::Decimal(decimal) => {
            format!(
                "exact::QuantityCarrier::Decimal({})",
                decimal_type(decimal)?
            )
        }
        QuantityCarrier::Integer {
            domain,
            rounding: mode,
        } => format!(
            "exact::QuantityCarrier::Integer {{ domain: {}, rounding: {} }}",
            interval(domain),
            rounding(*mode)?
        ),
        QuantityCarrier::Rational(domain) => {
            format!(
                "exact::QuantityCarrier::Rational({})",
                rational_domain(domain)
            )
        }
    })
}

fn decimal_type(value: &DecimalType) -> Rendered {
    Ok(format!(
        "rebuilt(exact::DecimalType::new({}, {}, {}, {}, {}))",
        integer(value.lower()),
        integer(value.upper()),
        value.min_scale(),
        value.max_scale(),
        rounding(value.rounding())?
    ))
}

/// An expression rebuilding `value_type` through the public `ValueType` constructors, covering
/// every variant of the closed type.
pub(super) fn value_type(value_type: &ValueType) -> Rendered {
    enum Step<'a> {
        Visit(&'a ValueType),
        Option,
        Collection(&'a CollectionType),
        Map(&'a MapType),
    }
    let mut pending = vec![Step::Visit(value_type)];
    let mut rendered: Vec<String> = Vec::new();
    while let Some(step) = pending.pop() {
        match step {
            Step::Visit(node) => match node {
                ValueType::Boolean => rendered.push("exact::ValueType::Boolean".to_owned()),
                ValueType::Integer => rendered.push("exact::ValueType::Integer".to_owned()),
                ValueType::Int(domain) => {
                    rendered.push(format!("exact::ValueType::Int({})", interval(domain)));
                }
                ValueType::Rational(domain) => {
                    rendered.push(format!("exact::ValueType::Rational({})", rational_domain(domain)));
                }
                ValueType::Decimal(decimal) => {
                    rendered.push(format!("exact::ValueType::Decimal({})", decimal_type(decimal)?));
                }
                ValueType::Float(float) => rendered.push(float_type(*float)?),
                ValueType::Quantity(id) => {
                    rendered.push(format!("exact::ValueType::Quantity({})", unit(id)?));
                }
                ValueType::CarriedQuantity(carried) => rendered.push(carried_quantity(carried)?),
                ValueType::Text(text) => rendered.push(format!(
                    "exact::ValueType::Text(rebuilt(exact::TextType::new({}, {}, {})))",
                    text.min(),
                    text.max(),
                    profile(text.profile())
                )),
                ValueType::String => rendered.push("exact::ValueType::String".to_owned()),
                ValueType::Uuid => rendered.push("exact::ValueType::Uuid".to_owned()),
                ValueType::Timestamp => rendered.push("exact::ValueType::Timestamp".to_owned()),
                ValueType::Enum(shape) => {
                    rendered.push(format!("exact::ValueType::Enum({})", enum_shape(shape)?));
                }
                ValueType::Option(payload) => {
                    pending.push(Step::Option);
                    pending.push(Step::Visit(payload));
                }
                ValueType::Composite(key) => {
                    rendered.push(format!("exact::ValueType::Composite({})", node_key(key)?));
                }
                ValueType::Collection(collection) => {
                    pending.push(Step::Collection(collection));
                    pending.push(Step::Visit(collection.element()));
                }
                ValueType::Map(map) => {
                    pending.push(Step::Map(map));
                    pending.push(Step::Visit(map.values()));
                    pending.push(Step::Visit(map.key()));
                }
                ValueType::BoundedQuantity(bounded) => rendered.push(format!(
                    "exact::ValueType::bounded_quantity(rebuilt(exact::BoundedQuantityType::new({}, {}, {})))",
                    unit(bounded.unit())?,
                    optional_quantity(bounded.min())?,
                    optional_quantity(bounded.max())?
                )),
                ValueType::Reference(id) => {
                    rendered.push(format!("exact::ValueType::Reference({})", effective_id(id)));
                }
                ValueType::Population(maximum) => {
                    rendered.push(match maximum {
                        Some(maximum) => format!("exact::ValueType::Population(Some({maximum}))"),
                        None => "exact::ValueType::Population(None)".to_owned(),
                    });
                }
            },
            Step::Option => {
                let payload = pop(&mut rendered)?;
                rendered.push(format!("exact::ValueType::option({payload})"));
            }
            Step::Collection(collection) => {
                let element = pop(&mut rendered)?;
                rendered.push(format!(
                    "exact::ValueType::collection(exact::CollectionType::new({}, {element}, {}))",
                    kind(collection.kind()),
                    bound(collection.bound())
                ));
            }
            Step::Map(map) => {
                let values = pop(&mut rendered)?;
                let key = pop(&mut rendered)?;
                rendered.push(format!(
                    "exact::ValueType::map(rebuilt(exact::MapType::new({key}, {values}, {})))",
                    bound(map.bound())
                ));
            }
        }
    }
    let rendered_type = pop(&mut rendered)?;
    if rendered.is_empty() {
        Ok(rendered_type)
    } else {
        Err(OracleGenerationError::InvalidPlanState)
    }
}

fn float_type(value: FloatType) -> Rendered {
    Ok(format!(
        "exact::ValueType::Float(exact::FloatType::new({}, {}))",
        width(value.width()),
        rounding(value.rounding())?
    ))
}

fn carried_quantity(value: &CarriedQuantityType) -> Rendered {
    Ok(format!(
        "exact::ValueType::carried_quantity(exact::CarriedQuantityType::new({}, {}))",
        unit(value.unit())?,
        carrier(value.carrier())?
    ))
}

#[cfg(test)]
mod tests;

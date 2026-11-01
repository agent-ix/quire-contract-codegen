//! Decode every whole-object symbolic draw in the generated operation ABI order.

use super::{decode_falsification, DecodeFailure};
use crate::kani::{
    abi::{KaniBindingRole, KaniPrimitiveType},
    identity::{
        CollectionElementIdentity, CollectionKindIdentity, ObligationBinding, SymbolicBounds,
        V2ClauseObligationIdentity,
    },
    output::playback::{Playback, WitnessDecodeLimits},
};
use qsl_replay::WitnessValue;

fn primitive(bounds: &SymbolicBounds) -> Option<KaniPrimitiveType> {
    match bounds {
        SymbolicBounds::Boolean => Some(KaniPrimitiveType::Boolean),
        SymbolicBounds::Integer { .. } => Some(KaniPrimitiveType::I64),
        SymbolicBounds::Sequence { .. } => None,
    }
}

fn scalar_domain(domain: &CollectionElementIdentity) -> SymbolicBounds {
    match domain {
        CollectionElementIdentity::Boolean => SymbolicBounds::Boolean,
        CollectionElementIdentity::Integer { minimum, maximum } => SymbolicBounds::Integer {
            minimum: *minimum,
            maximum: *maximum,
        },
    }
}

fn append(bindings: &mut Vec<ObligationBinding>, name: String, ty: KaniPrimitiveType) {
    bindings.push(ObligationBinding {
        identifier: name,
        role: KaniBindingRole::Argument,
        primitive_type: ty,
        integer_bounds: None,
        dependencies: Vec::new(),
    });
}

fn schema(
    bindings: &mut Vec<ObligationBinding>,
    name: &str,
    bounds: &SymbolicBounds,
    limit: usize,
) -> Result<(), DecodeFailure> {
    let failure = || {
        DecodeFailure::new(
            "cg_witness_decode_values_limit",
            name,
            "whole object input schema",
        )
    };
    match bounds {
        SymbolicBounds::Sequence {
            maximum_length,
            element,
            ..
        } => {
            let maximum = usize::try_from(*maximum_length).map_err(|_| failure())?;
            let total = bindings
                .len()
                .checked_add(maximum)
                .and_then(|size| size.checked_add(1))
                .ok_or_else(failure)?;
            if total > limit {
                return Err(failure());
            }
            append(bindings, format!("{name}_len"), KaniPrimitiveType::I64);
            let ty = primitive(element).ok_or_else(failure)?;
            for index in 0..maximum {
                append(bindings, format!("{name}_{index}"), ty);
            }
        }
        _ => {
            if bindings.len() >= limit {
                return Err(failure());
            }
            append(
                bindings,
                name.to_owned(),
                primitive(bounds).ok_or_else(failure)?,
            );
        }
    }
    Ok(())
}

fn check(value: &WitnessValue, bounds: &SymbolicBounds) -> bool {
    match (value, bounds) {
        (WitnessValue::Boolean(_), SymbolicBounds::Boolean) => true,
        (WitnessValue::Integer(value), SymbolicBounds::Integer { minimum, maximum }) => {
            (i128::from(*minimum)..=i128::from(*maximum)).contains(value)
        }
        _ => false,
    }
}

fn value<'a>(
    decoded: &mut impl Iterator<Item = &'a (String, WitnessValue)>,
    name: &str,
    bounds: &SymbolicBounds,
    kind: Option<CollectionKindIdentity>,
) -> Result<WitnessValue, DecodeFailure> {
    let failure = || DecodeFailure::new("cg_witness_out_of_domain", name, "whole object input");
    match bounds {
        SymbolicBounds::Sequence {
            minimum_length,
            maximum_length,
            element,
        } => {
            let (_, WitnessValue::Integer(length)) = decoded.next().ok_or_else(failure)? else {
                return Err(failure());
            };
            if !(i128::from(*minimum_length)..=i128::from(*maximum_length)).contains(length) {
                return Err(failure());
            }
            let mut active = Vec::new();
            for index in 0..*maximum_length {
                let (_, item) = decoded.next().ok_or_else(failure)?;
                if i128::from(index) < *length {
                    if !check(item, element)
                        || (kind == Some(CollectionKindIdentity::OrderedSet)
                            && active.contains(item))
                    {
                        return Err(failure());
                    }
                    active.push(item.clone());
                }
            }
            match kind {
                Some(CollectionKindIdentity::Sequence) => Ok(WitnessValue::Sequence(active)),
                Some(CollectionKindIdentity::OrderedSet) => Ok(WitnessValue::OrderedSet(active)),
                None => Err(failure()),
            }
        }
        _ => {
            let (_, value) = decoded.next().ok_or_else(failure)?;
            if !check(value, bounds) {
                return Err(failure());
            }
            Ok(value.clone())
        }
    }
}

pub(super) fn decode(
    identity: &V2ClauseObligationIdentity,
    playback: &Playback<'_>,
    limits: WitnessDecodeLimits,
) -> Result<Vec<(String, WitnessValue)>, DecodeFailure> {
    let context = identity.collection_operation.as_ref().ok_or_else(|| {
        DecodeFailure::new(
            "cg_witness_schema_limit",
            identity.harness_symbol.as_str(),
            "missing operation context",
        )
    })?;
    let mut bindings = Vec::new();
    for field in &context.fields {
        schema(&mut bindings, &field.slot, &field.bounds, limits.values)?;
    }
    for parameter in &context.parameters {
        schema(
            &mut bindings,
            &parameter.binding,
            &scalar_domain(&parameter.domain),
            limits.values,
        )?;
    }
    let decoded = decode_falsification(
        identity.harness_symbol.as_str(),
        identity.module_symbol.as_str(),
        &bindings,
        playback,
        limits,
    )?;
    let mut cursor = decoded.iter();
    let mut result = Vec::new();
    for field in &context.fields {
        let name = format!("self.{}", field.field_name);
        result.push((
            name.clone(),
            value(&mut cursor, &name, &field.bounds, field.collection_kind)?,
        ));
    }
    for parameter in &context.parameters {
        result.push((
            parameter.name.clone(),
            value(
                &mut cursor,
                &parameter.name,
                &scalar_domain(&parameter.domain),
                None,
            )?,
        ));
    }
    Ok(result)
}

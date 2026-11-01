//! Emission of whole exact-value model states and typed collection operation calls.

use super::super::{
    outcome::UnsupportedObligation,
    spec::{CollectionCondition, CollectionOperationSpec, CollectionSpec, HarnessSpec},
};
use crate::{
    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},
    kani::{
        abi::i64_literal,
        identity::{
            CollectionElementIdentity, CollectionKindIdentity, CollectionObjectFieldIdentity,
            CollectionPredicateIdentity, SymbolicBounds,
        },
    },
};
use std::fmt::Write;

fn primitive_type(domain: &CollectionElementIdentity) -> &'static str {
    match domain {
        CollectionElementIdentity::Boolean => "bool",
        CollectionElementIdentity::Integer { .. } => "i64",
    }
}

fn bounds_type(bounds: &SymbolicBounds) -> Option<String> {
    match bounds {
        SymbolicBounds::Boolean => Some("quire_exact::ValueType::Boolean".into()),
        SymbolicBounds::Integer { minimum, maximum } => Some(format!("quire_exact::ValueType::Int(quire_exact::IntegerInterval::new(quire_exact::Integer::from({}), quire_exact::Integer::from({})).expect(\"checked interval\"))", i64_literal(*minimum), i64_literal(*maximum))),
        SymbolicBounds::Sequence { .. } => None,
    }
}

fn domain_check(value: &str, bounds: &SymbolicBounds) -> Option<String> {
    match bounds {
        SymbolicBounds::Boolean => Some(format!("matches!({value}, quire_exact::Value::Boolean(_))")),
        SymbolicBounds::Integer { minimum, maximum } => Some(format!("matches!({value}, quire_exact::Value::Integer(value) if value >= &quire_exact::Integer::from({}) && value <= &quire_exact::Integer::from({}))", i64_literal(*minimum), i64_literal(*maximum))),
        SymbolicBounds::Sequence { .. } => None,
    }
}

fn scalar_bounds(domain: &CollectionElementIdentity) -> SymbolicBounds {
    match domain {
        CollectionElementIdentity::Boolean => SymbolicBounds::Boolean,
        CollectionElementIdentity::Integer { minimum, maximum } => SymbolicBounds::Integer {
            minimum: *minimum,
            maximum: *maximum,
        },
    }
}

fn draw_scalar(source: &mut String, name: &str, domain: &SymbolicBounds) -> Option<String> {
    match domain {
        SymbolicBounds::Boolean => {
            writeln!(source, "let {name}: bool = kani::any();").ok()?;
            Some(format!("quire_exact::Value::Boolean({name})"))
        }
        SymbolicBounds::Integer { minimum, maximum } => {
            writeln!(
                source,
                "let {name}: i64 = kani::any();\nkani::assume({name} >= {} && {name} <= {});",
                i64_literal(*minimum),
                i64_literal(*maximum)
            )
            .ok()?;
            Some(format!(
                "quire_exact::Value::Integer(quire_exact::Integer::from({name}))"
            ))
        }
        SymbolicBounds::Sequence { .. } => None,
    }
}

fn collection_schema(source: &mut String, field: &CollectionObjectFieldIdentity) -> Option<()> {
    let SymbolicBounds::Sequence {
        minimum_length,
        maximum_length,
        element,
    } = &field.bounds
    else {
        return Some(());
    };
    let ty = bounds_type(element)?;
    let check = domain_check("value", element)?;
    let kind = match field.collection_kind? {
        CollectionKindIdentity::Sequence => "Sequence",
        CollectionKindIdentity::OrderedSet => "OrderedSet",
    };
    writeln!(source, "fn type_{}() -> quire_exact::CollectionType {{ quire_exact::CollectionType::new(quire_exact::CollectionKind::{kind}, {ty}, Some(quire_exact::CardinalityBound::new({minimum_length}, {maximum_length}).expect(\"checked cardinality\"))) }}", field.slot).ok()?;
    writeln!(source, "fn admit_{}(ty: &quire_exact::ValueType, value: &quire_exact::Value) -> Result<(), quire_exact::ConstructionCause> {{\nif ty == &{ty} && {check} {{ Ok(()) }} else {{ Err(quire_exact::ConstructionCause::TypeMismatch) }}\n}}", field.slot).ok()?;
    Some(())
}

fn draw_field(source: &mut String, field: &CollectionObjectFieldIdentity) -> Option<()> {
    let name = &field.slot;
    match &field.bounds {
        SymbolicBounds::Sequence {
            minimum_length,
            maximum_length,
            element,
        } => {
            let (ty, conversion, assumption) = match element.as_ref() {
                SymbolicBounds::Boolean => (
                    "bool",
                    "quire_exact::Value::Boolean(item)".to_owned(),
                    String::new(),
                ),
                SymbolicBounds::Integer { minimum, maximum } => (
                    "i64",
                    "quire_exact::Value::Integer(quire_exact::Integer::from(item))".to_owned(),
                    format!(
                        "kani::assume(item >= {} && item <= {});",
                        i64_literal(*minimum),
                        i64_literal(*maximum)
                    ),
                ),
                SymbolicBounds::Sequence { .. } => return None,
            };
            writeln!(source, "let {name}_len: i64 = kani::any();\nkani::assume({name}_len >= {minimum_length} && {name}_len <= {maximum_length});\nlet {name}_items: [{ty}; {maximum_length}] = kani::any();\nlet mut {name}_occurrences = Vec::new();\nfor index in 0..{name}_len {{\nlet item = {name}_items[index as usize];\n{assumption}").ok()?;
            match field.collection_kind? {
                CollectionKindIdentity::Sequence => {}
                CollectionKindIdentity::OrderedSet => {
                    writeln!(source, "for prior in 0..index {{ kani::assume(item != {name}_items[prior as usize]); }}").ok()?;
                }
            }
            writeln!(source, "{name}_occurrences.push({conversion});\n}}\nlet {name} = match quire_exact::collection::build_typed_prestate(&type_{name}(), {name}_occurrences, &mut meter(), Some(&mut admit_{name})) {{\nOk(quire_exact::Outcome::Completed(value)) => value, _ => panic!(\"the complete admitted prestate must construct\"),\n}};").ok()?;
        }
        bounds => {
            let value = draw_scalar(source, &format!("{name}_input"), bounds)?;
            writeln!(source, "let {name} = {value};").ok()?;
        }
    }
    Some(())
}

fn predicate(predicate: &CollectionPredicateIdentity) -> String {
    match predicate {
        CollectionPredicateIdentity::IntegerGreaterOrEqual { threshold } => format!(
            "integer(item) >= &quire_exact::Integer::from({})",
            i64_literal(*threshold)
        ),
        CollectionPredicateIdentity::IntegerEqual { value } => format!(
            "integer(item) == &quire_exact::Integer::from({})",
            i64_literal(*value)
        ),
        CollectionPredicateIdentity::BooleanNotEqual { value } => {
            format!("boolean(item) != {value}")
        }
    }
}

fn quantifier(
    field: usize,
    operation: &str,
    claim: &CollectionPredicateIdentity,
    state: &str,
) -> Option<String> {
    let method = match operation {
        "quire.op.collection.forall" => "all",
        "quire.op.collection.exists" => "any",
        _ => return None,
    };
    Some(format!(
        "collection(&{state}.field_{field}).elements().iter().{method}(|item| {})",
        predicate(claim)
    ))
}

fn emit(
    spec: &HarnessSpec,
    selected: &CollectionSpec,
    operation: &CollectionOperationSpec,
) -> Option<String> {
    let path = spec.path();
    let mut source = format!("// SPDX-License-Identifier: MIT OR Apache-2.0\n#[cfg(kani)]\nmod {} {{\nuse super::*;\n#[derive(Clone)]\nstruct CgObjectState {{\n", path.module);
    for field in &operation.identity.fields {
        writeln!(source, "{}: quire_exact::Value,", field.slot).ok()?;
    }
    source.push_str("}\nfn integer(value: &quire_exact::Value) -> &quire_exact::Integer { match value { quire_exact::Value::Integer(value) => value, _ => panic!(\"checked integer field\") } }\nfn boolean(value: &quire_exact::Value) -> bool { match value { quire_exact::Value::Boolean(value) => *value, _ => panic!(\"checked Boolean field\") } }\nfn collection(value: &quire_exact::Value) -> &quire_exact::CollectionValue { match value { quire_exact::Value::Collection(value) => value, _ => panic!(\"checked collection field\") } }\nfn meter() -> quire_exact::Meter { quire_exact::Meter::new(quire_exact::ScalarLimits { integer_bits:u64::MAX, decimal_digits:u64::MAX, scale_expansion:u64::MAX, text_input_bytes:u64::MAX, text_scalars:u64::MAX, normalized_scalars:u64::MAX, unit_edges:u64::MAX, value_occurrences:u64::MAX, work_units:u64::MAX, result_units:u64::MAX }) }\n");
    for field in &operation.identity.fields {
        collection_schema(&mut source, field)?;
    }
    source.push_str("#[allow(dead_code)] enum ClauseEvaluation { Completed(bool), Undefined, Refused, Incomplete }\nfn oracle_clause(state: &CgObjectState) -> ClauseEvaluation {\n");
    let operation_name = match selected.quantifier {
        super::super::spec::CollectionQuantifier::ForAll => "quire.op.collection.forall",
        super::super::spec::CollectionQuantifier::Exists => "quire.op.collection.exists",
    };
    let claim = quantifier(
        operation.identity.selected_field,
        operation_name,
        &selected.predicate,
        "state",
    )?;
    writeln!(
        source,
        "ClauseEvaluation::Completed({claim})\n}}\n#[kani::proof]\nfn {}() {{",
        path.harness
    )
    .ok()?;
    for field in &operation.identity.fields {
        draw_field(&mut source, field)?;
    }
    for parameter in &operation.identity.parameters {
        draw_scalar(
            &mut source,
            &parameter.binding,
            &scalar_bounds(&parameter.domain),
        )?;
    }
    source.push_str("let pre = CgObjectState {\n");
    for field in &operation.identity.fields {
        writeln!(source, "{},", field.slot).ok()?;
    }
    source.push_str("};\n");
    if operation.postcondition {
        for condition in &operation.preconditions {
            let condition = match condition {
                CollectionCondition::Scalar(expression) => expression.as_str().to_owned(),
                CollectionCondition::Quantifier {
                    field,
                    operation,
                    predicate,
                } => quantifier(*field, operation, predicate, "pre")?,
            };
            writeln!(source, "kani::assume({condition});").ok()?;
        }
        source.push_str("let mut post = pre.clone();\n");
        let mut arguments = operation
            .identity
            .fields
            .iter()
            .map(|field| format!("&mut post.{}", field.slot))
            .collect::<Vec<_>>();
        arguments.extend(
            operation
                .identity
                .parameters
                .iter()
                .map(|parameter| parameter.binding.clone()),
        );
        // The subject is a typed function item, so Rust checks every field, parameter and result.
        let result_type = operation
            .identity
            .result
            .as_ref()
            .map_or("()", |result| primitive_type(&result.domain));
        writeln!(
            source,
            "let subject_result: {result_type} = {}({});",
            selected_subject(spec)?,
            arguments.join(", ")
        )
        .ok()?;
        for field in &operation.identity.fields {
            match &field.bounds {
                SymbolicBounds::Sequence { .. } => {
                    writeln!(source, "assert!(matches!(quire_exact::collection::inspect_typed_poststate(&type_{}(), &post.{}, &mut meter(), Some(&mut admit_{})), Ok(quire_exact::Outcome::Completed(_))), \"every produced collection retains its admitted type and domain\");", field.slot, field.slot, field.slot).ok()?;
                }
                bounds => {
                    writeln!(source, "assert!({}, \"every produced field retains its admitted type and domain\");", domain_check(&format!("&post.{}", field.slot), bounds)?).ok()?;
                }
            }
        }
        if let Some(result) = &operation.identity.result {
            if let CollectionElementIdentity::Integer { minimum, maximum } = result.domain {
                writeln!(source, "assert!(subject_result >= {} && subject_result <= {}, \"the produced result retains its admitted domain\");", i64_literal(minimum), i64_literal(maximum)).ok()?;
            }
        }
        source.push_str("let outcome = oracle_clause(&post);\nassert!(matches!(outcome, ClauseEvaluation::Completed(true)), \"the produced state satisfies the authored collection postcondition\");\n");
        let field = &operation.identity.fields[operation.identity.selected_field];
        writeln!(source, "kani::cover!(collection(&pre.{}).elements().len() == {} || collection(&pre.{}).elements().len() == {}, \"a selected field cardinality endpoint is reachable\");", field.slot, selected.minimum_length, field.slot, selected.maximum_length).ok()?;
    } else {
        writeln!(
            source,
            "let outcome = oracle_clause(&pre);\n{}\nkani::cover!({}, \"{}\");",
            crate::kani::identity::collection_precondition_totality_assertion(),
            spec.cover().condition.as_str(),
            spec.cover().message
        )
        .ok()?;
    }
    source.push_str("}\n}\n");
    Some(source)
}

// The validated subject path is retained with the operation description, not parsed from text.
fn selected_subject(spec: &HarnessSpec) -> Option<&str> {
    let super::super::spec::HarnessShape::CollectionOperation { operation, .. } = spec.shape()
    else {
        return None;
    };
    Some(&operation.subject_path)
}

pub(super) fn render(
    spec: &HarnessSpec,
    selected: &CollectionSpec,
    operation: &CollectionOperationSpec,
) -> Result<Artifact, UnsupportedObligation> {
    let source = emit(spec, selected, operation).ok_or(UnsupportedObligation::RenderFailed)?;
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(UnsupportedObligation::ResourceLimitExceeded {
            bytes: source.len(),
        });
    }
    syn::parse_file(&source).map_err(|error| UnsupportedObligation::InvalidGeneratedSyntax {
        error: error.to_string(),
    })?;
    Ok(Artifact::new(
        format!("src/generated/{}.rs", spec.path().module),
        source,
    ))
}

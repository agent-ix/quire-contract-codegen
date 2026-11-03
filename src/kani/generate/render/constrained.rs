// SPDX-License-Identifier: AGPL-3.0-or-later
//! Render finite state transitions, observing safety before the constraint cutoff.

use crate::core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES};
use crate::core::ir::clause::ClauseInputSide;
use crate::kani::generate::{
    outcome::constrained::ConstrainedMethodRefusal,
    spec::{ConstrainedConditionSpec, ConstrainedConditionsSpec},
};
use crate::kani::identity::{constrained::*, SymbolicBounds};
use std::fmt::Write;

fn literal(
    value: &serde_json::Value,
    domain: &SymbolicBounds,
) -> Result<String, ConstrainedMethodRefusal> {
    use ConstrainedMethodRefusal as R;
    match domain {
        SymbolicBounds::Boolean => value["boolean"]
            .as_bool()
            .map(|value| value.to_string())
            .ok_or(R::InitialState),
        SymbolicBounds::Integer { minimum, maximum } => {
            let text = value["integer"].as_str().ok_or(R::InitialState)?;
            let integer = text.parse::<i64>().map_err(|_| R::InitialState)?;
            if integer < *minimum || integer > *maximum || integer.to_string() != text {
                return Err(R::InitialState);
            }
            Ok(crate::kani::abi::i64_literal(integer))
        }
        SymbolicBounds::Sequence {
            minimum_length,
            maximum_length,
            element,
        } => {
            let values = value
                .get("sequence")
                .or_else(|| value.get("ordered-set"))
                .and_then(serde_json::Value::as_array)
                .ok_or(R::InitialState)?;
            let count = i64::try_from(values.len()).map_err(|_| R::InitialState)?;
            if count < *minimum_length || count > *maximum_length {
                return Err(R::InitialState);
            }
            let values = values
                .iter()
                .map(|value| literal(value, element))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(format!("vec![{}]", values.join(",")))
        }
    }
}

pub(in crate::kani::generate) fn initial(
    field: &ConstrainedFieldIdentity,
) -> Result<String, ConstrainedMethodRefusal> {
    let value = field
        .initial
        .to_json()
        .map_err(|_| ConstrainedMethodRefusal::InitialState)?;
    if let SymbolicBounds::Sequence { .. } = &field.domain {
        let key = if field.unique {
            "ordered-set"
        } else {
            "sequence"
        };
        let values = value
            .get(key)
            .and_then(serde_json::Value::as_array)
            .ok_or(ConstrainedMethodRefusal::InitialState)?;
        if field.unique {
            for (index, value) in values.iter().enumerate() {
                if values[..index].contains(value) {
                    return Err(ConstrainedMethodRefusal::InitialState);
                }
            }
        }
    }
    literal(&value, &field.domain)
}

fn ty(domain: &SymbolicBounds) -> String {
    match domain {
        SymbolicBounds::Boolean => "bool".into(),
        SymbolicBounds::Integer { .. } => "i64".into(),
        SymbolicBounds::Sequence { element, .. } => format!("Vec<{}>", ty(element)),
    }
}
fn check(value: &str, domain: &SymbolicBounds, unique: bool) -> String {
    match domain {
        SymbolicBounds::Boolean => "true".into(),
        SymbolicBounds::Integer { minimum, maximum } => format!(
            "({value} >= {} && {value} <= {})",
            crate::kani::abi::i64_literal(*minimum),
            crate::kani::abi::i64_literal(*maximum)
        ),
        SymbolicBounds::Sequence {
            minimum_length,
            maximum_length,
            element,
        } => {
            let inner = check("*item", element, false);
            format!("({value}.len() >= {minimum_length}usize && {value}.len() <= {maximum_length}usize && {value}.iter().all(|item| {inner}) {})",if unique {format!("&& {value}.iter().enumerate().all(|(index,item)| !{value}[..index].contains(item))")} else {String::new()})
        }
    }
}
fn draw(
    source: &mut String,
    name: &str,
    domain: &SymbolicBounds,
) -> Result<(), ConstrainedMethodRefusal> {
    match domain {
        SymbolicBounds::Boolean=>writeln!(source,"let {name}: bool = kani::any();"),
        SymbolicBounds::Integer{..}=>writeln!(source,"let {name}: i64 = kani::any();\nkani::assume({});",check(name,domain,false)),
        SymbolicBounds::Sequence{minimum_length,maximum_length,element}=>{
            writeln!(source,"let {name}_length: usize = kani::any();\nkani::assume({name}_length >= {minimum_length}usize && {name}_length <= {maximum_length}usize);\nlet mut {name}: {} = Vec::new();\nfor _ in 0..{name}_length {{",ty(domain)).map_err(|_|ConstrainedMethodRefusal::Render)?;
            draw(source,"item",element)?;
            writeln!(source,"{name}.push(item);\n}}")
        }
    }.map_err(|_|ConstrainedMethodRefusal::Render)
}

fn oracle(
    source: &mut String,
    name: &str,
    condition: &ConstrainedConditionSpec,
    identity: &ConstrainedMethodIdentity,
    operation: Option<&ConstrainedOperationIdentity>,
) -> Result<(), ConstrainedMethodRefusal> {
    use ConstrainedMethodRefusal as R;
    writeln!(source, "mod {name} {{ {} }}", condition.oracle.source).map_err(|_| R::Render)?;
    let mut arguments = Vec::new();
    for input in &condition.inputs {
        let value = match input.side {
            ClauseInputSide::Current => {
                let index = identity
                    .fields
                    .iter()
                    .position(|field| {
                        field.name == input.name
                            && matches!(
                                field.domain,
                                SymbolicBounds::Integer { .. } | SymbolicBounds::Boolean
                            )
                    })
                    .ok_or(R::Condition)?;
                format!("state.{index}")
            }
            ClauseInputSide::Parameter => {
                let index = operation
                    .ok_or(R::Condition)?
                    .parameters
                    .iter()
                    .position(|(name, domain)| {
                        name == &input.name
                            && matches!(
                                domain.bounds,
                                SymbolicBounds::Integer { .. } | SymbolicBounds::Boolean
                            )
                    })
                    .ok_or(R::Condition)?;
                format!("*parameter_{index}")
            }
            ClauseInputSide::Pre | ClauseInputSide::Result => return Err(R::Condition),
        };
        arguments.push(if input.boolean {
            value
        } else if condition.oracle.native_integer_selection.is_some() {
            format!("&quire_exact::NativeInteger::new(i128::from({value}))")
        } else {
            format!("&quire_exact::Integer::from({value})")
        });
    }
    if !condition.oracle.descriptor.operation_identities.is_empty() {
        arguments.push("&mut meter()".into());
    }
    let state_type = tuple_type(identity);
    let parameters = operation
        .map(|operation| {
            operation
                .parameters
                .iter()
                .enumerate()
                .map(|(index, (_, domain))| format!(", parameter_{index}: &{}", ty(&domain.bounds)))
                .collect::<String>()
        })
        .unwrap_or_default();
    // Exact-kernel outcomes are retained rather than coerced to a Boolean predicate.
    let result = if condition.oracle.descriptor.operation_identities.is_empty() {
        "bool".to_owned()
    } else {
        format!("Result<quire_exact::Outcome<bool>, {name}::ClauseStop>")
    };
    writeln!(source,"fn evaluate_{name}(state: &{state_type}{parameters}) -> {result} {{ {name}::oracle_clause({}) }}",arguments.join(",")).map_err(|_|R::Render)
}

fn tuple_type(identity: &ConstrainedMethodIdentity) -> String {
    if identity.fields.is_empty() {
        "()".into()
    } else {
        format!(
            "({},)",
            identity
                .fields
                .iter()
                .map(|field| ty(&field.domain))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

pub(super) fn render(
    identity: &ConstrainedMethodIdentity,
    prepared: &ConstrainedConditionsSpec,
) -> Result<Artifact, ConstrainedMethodRefusal> {
    use ConstrainedMethodRefusal as R;
    let mut source = format!(
        "// @generated by quire-contract-codegen.\nmod {} {{\n",
        identity.module_symbol
    );
    let names = [
        "integer_bits",
        "decimal_digits",
        "scale_expansion",
        "text_input_bytes",
        "text_scalars",
        "normalized_scalars",
        "unit_edges",
        "value_occurrences",
        "work_units",
        "result_units",
    ];
    writeln!(source,"fn meter() -> quire_exact::Meter {{ quire_exact::Meter::new(quire_exact::ScalarLimits {{ {} }}) }}",names.iter().zip(identity.evaluator_limits).map(|(name,value)|format!("{name}: {value}u64")).collect::<Vec<_>>().join(",")).map_err(|_|R::Render)?;
    oracle(&mut source, "safety", &prepared.safety, identity, None)?;
    if let Some(constraint) = &prepared.constraint {
        oracle(&mut source, "constraint", constraint, identity, None)?;
    }
    for (index, (operation, conditions)) in identity
        .operations
        .iter()
        .zip(&prepared.preconditions)
        .enumerate()
    {
        for (condition_index, condition) in conditions.iter().enumerate() {
            oracle(
                &mut source,
                &format!("pre_{index}_{condition_index}"),
                condition,
                identity,
                Some(operation),
            )?;
        }
    }
    let initial = if identity.fields.is_empty() {
        "()".to_owned()
    } else {
        format!(
            "({},)",
            identity
                .fields
                .iter()
                .map(initial)
                .collect::<Result<Vec<_>, _>>()?
                .join(",")
        )
    };
    writeln!(
        source,
        "#[kani::proof]\nfn search() {{\nlet mut state: {} = {};",
        tuple_type(identity),
        initial
    )
    .map_err(|_| R::Render)?;
    if prepared.constraint.is_some() {
        source.push_str("let mut boundary = false;\n");
    }
    writeln!(
        source,
        "let mut method_reached = false;\nfor depth in 0..={}u32 {{\nmethod_reached = true;",
        identity.maximum_steps
    )
    .map_err(|_| R::Render)?;
    if prepared
        .safety
        .oracle
        .descriptor
        .operation_identities
        .is_empty()
    {
        source.push_str("assert!(evaluate_safety(&state), \"safety violation\");\n");
    } else {
        source.push_str("match evaluate_safety(&state) {\nOk(quire_exact::Outcome::Completed(value)) => assert!(value, \"safety violation\"),\nOk(quire_exact::Outcome::Undefined(_)) => panic!(\"undefined safety evaluation\"),\nOk(quire_exact::Outcome::Refused(_)) => panic!(\"refused safety evaluation\"),\nOk(quire_exact::Outcome::Incomplete(_)) => panic!(\"incomplete safety evaluation\"),\nErr(_) => panic!(\"owner call failure\"),\n}\n");
    }
    if let Some(constraint) = &prepared.constraint {
        if constraint.oracle.descriptor.operation_identities.is_empty() {
            source.push_str("if !evaluate_constraint(&state) { boundary = true; break; }\n");
        } else {
            source.push_str("match evaluate_constraint(&state) {\nOk(quire_exact::Outcome::Completed(false)) => { boundary = true; break; },\nOk(quire_exact::Outcome::Completed(true)) => {},\nOk(quire_exact::Outcome::Undefined(_)) => panic!(\"undefined constraint evaluation\"),\nOk(quire_exact::Outcome::Refused(_)) => panic!(\"refused constraint evaluation\"),\nOk(quire_exact::Outcome::Incomplete(_)) => panic!(\"incomplete constraint evaluation\"),\nErr(_) => panic!(\"owner call failure\"),\n}\n");
        }
    }
    writeln!(
        source,
        "if depth == {}u32 {{ break; }}",
        identity.maximum_steps
    )
    .map_err(|_| R::Render)?;
    if identity.operations.is_empty() {
        source.push_str("break;\n");
    } else {
        writeln!(source,"let operation: usize = kani::any();\nkani::assume(operation < {}usize);\nmatch operation {{",identity.operations.len()).map_err(|_|R::Render)?;
        for (index, operation) in identity.operations.iter().enumerate() {
            writeln!(source, "{} => {{", operation.choice).map_err(|_| R::Render)?;
            for (parameter, (_, domain)) in operation.parameters.iter().enumerate() {
                draw(
                    &mut source,
                    &format!("parameter_{parameter}"),
                    &domain.bounds,
                )?;
                if domain.collection_kind
                    == Some(crate::kani::identity::CollectionKindIdentity::OrderedSet)
                {
                    writeln!(
                        source,
                        "kani::assume({});",
                        check(&format!("parameter_{parameter}"), &domain.bounds, true)
                    )
                    .map_err(|_| R::Render)?;
                }
            }
            let parameters = (0..operation.parameters.len())
                .map(|parameter| format!(", &parameter_{parameter}"))
                .collect::<String>();
            for condition in 0..prepared.preconditions[index].len() {
                if prepared.preconditions[index][condition]
                    .oracle
                    .descriptor
                    .operation_identities
                    .is_empty()
                {
                    writeln!(
                        source,
                        "if !evaluate_pre_{index}_{condition}(&state{parameters}) {{ break; }}"
                    )
                    .map_err(|_| R::Render)?;
                } else {
                    writeln!(source,"match evaluate_pre_{index}_{condition}(&state{parameters}) {{\nOk(quire_exact::Outcome::Completed(false)) => break,\nOk(quire_exact::Outcome::Completed(true)) => {{}},\nOk(quire_exact::Outcome::Undefined(_)) => panic!(\"undefined precondition evaluation\"),\nOk(quire_exact::Outcome::Refused(_)) => panic!(\"refused precondition evaluation\"),\nOk(quire_exact::Outcome::Incomplete(_)) => panic!(\"incomplete precondition evaluation\"),\nErr(_) => panic!(\"owner call failure\"),\n}}").map_err(|_|R::Render)?;
                }
            }
            let mut arguments = (0..identity.fields.len())
                .map(|field| format!("&mut state.{field}"))
                .collect::<Vec<_>>();
            arguments.extend(
                (0..operation.parameters.len()).map(|parameter| format!("parameter_{parameter}")),
            );
            let result_type = operation
                .result
                .as_ref()
                .map(|domain| ty(&domain.bounds))
                .unwrap_or_else(|| "()".into());
            writeln!(
                source,
                "let result: {result_type} = {}({});",
                operation.subject_path,
                arguments.join(",")
            )
            .map_err(|_| R::Render)?;
            if let Some(domain) = &operation.result {
                writeln!(
                    source,
                    "assert!({}, \"operation result outside admitted domain\");",
                    check(
                        "result",
                        &domain.bounds,
                        domain.collection_kind
                            == Some(crate::kani::identity::CollectionKindIdentity::OrderedSet)
                    )
                )
                .map_err(|_| R::Render)?;
            } else {
                source.push_str("let _ = result;\n");
            }
            for (index, field) in identity.fields.iter().enumerate() {
                writeln!(
                    source,
                    "assert!({}, \"operation state outside admitted domain\");",
                    check(&format!("state.{index}"), &field.domain, field.unique)
                )
                .map_err(|_| R::Render)?;
            }
            source.push_str("},\n");
        }
        source.push_str("_ => unreachable!(),\n}\n");
    }
    source.push_str("}\n");
    source.push_str("kani::cover!(method_reached, \"constrained ordinary nonvacuity\");");
    source.push('\n');
    if prepared.constraint.is_some() {
        source.push_str("kani::cover!(boundary, \"constrained boundary reachability\");");
        source.push('\n');
    }
    source.push_str("}\n}\n");
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(R::Render);
    }
    let rust = Artifact::new(
        format!("src/generated/{}.rs", identity.module_symbol),
        source,
    );
    Ok(rust)
}

/// Attribute the renderer's own assertion and distinct covers, never backend text.
pub(in crate::kani::generate) fn observation_lines(
    rust: &Artifact,
    constrained: bool,
) -> Result<(u32, u32, Option<u32>), ConstrainedMethodRefusal> {
    use ConstrainedMethodRefusal as R;
    let mut safety = rust.contents.lines().enumerate().filter(|(_, line)| {
        line.trim_start()
            .starts_with("Ok(quire_exact::Outcome::Completed(value)) => assert!(value,")
            || line
                .trim_start()
                .starts_with("assert!(evaluate_safety(&state),")
    });
    let (index, _) = safety.next().ok_or(R::Render)?;
    if safety.next().is_some() {
        return Err(R::Render);
    }
    let safety = u32::try_from(index + 1).map_err(|_| R::Render)?;
    let rows = rust
        .contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.trim_start().starts_with("kani::cover!("))
        .map(|(index, _)| u32::try_from(index + 1))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| R::Render)?;
    match rows.as_slice() {
        [ordinary, boundary] if constrained => Ok((safety, *ordinary, Some(*boundary))),
        [ordinary] if !constrained => Ok((safety, *ordinary, None)),
        _ => Err(R::Render),
    }
}

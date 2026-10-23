//! V2 state-clause condition oracles over admitted checked packages.

use std::collections::BTreeMap;

use quire_contract_model::{CheckedNodeId, CheckedPackageV2};
use serde::{Deserialize, Serialize};

use crate::core::artifact::MAX_GENERATED_SOURCE_BYTES;
use crate::core::ir::clause::{
    read_clause, ClauseGraphView, ClauseInput, ClauseInputSide, ClauseTerm,
};
use crate::core::source_map::SourceProbe;

use super::claim::{ClaimDisposition, ClaimMap};
use super::scalar::{
    clause_scalar_operation, render_clause_operation_helpers, BooleanOperator, EqualityOperandKind,
    ExactScalarOperation, OperationProvenance, OrderingOperandKind,
};

/// A state field or bounded operation value supplied to one generated clause oracle.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClauseOracleInput {
    /// The generated parameter name.
    pub name: String,
    /// Inclusive lower bound from the admitted model member or parameter type.
    pub lower: i128,
    /// Inclusive upper bound from the admitted model member or parameter type.
    pub upper: i128,
}

/// Checked inputs and operations of one clause-identified Boolean condition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct V2ClauseDescriptor {
    /// The clause node that owns the oracle claim.
    pub node_id: CheckedNodeId,
    /// Referenced condition expression, if the third argument is a reference.
    pub condition_node_id: Option<CheckedNodeId>,
    /// Ordered operand values and their admitted inclusive bounds.
    pub inputs: Vec<ClauseOracleInput>,
    /// Checked scalar operation identities in evaluation-plan order.
    pub operation_identities: Vec<String>,
    /// Number of implication consequents derived from the condition tree.
    pub expected_consequents: u32,
}

/// One generated oracle of a V2 clause's Boolean condition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct V2ClauseOracle {
    /// The operation's checked provenance.
    pub provenance: OperationProvenance,
    /// Self-contained generated Rust source. Retained in memory for harness embedding.
    #[serde(skip)]
    pub source: String,
    /// Descriptor derived from the admitted condition.
    pub descriptor: V2ClauseDescriptor,
    /// Probe regions derived from the generated source and the checked condition.
    pub source_map: Vec<V2ClauseCoverageRegion>,
}

/// The one requested clause's generated-or-refused disposition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct V2ClauseClaim {
    /// Exact requested node.
    pub node_id: CheckedNodeId,
    /// Generated oracle or typed refusal.
    pub result: ClaimDisposition<V2ClauseOracle, V2ClauseRefusal>,
}

/// One source span for a V2 clause condition oracle.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct V2ClauseCoverageRegion {
    /// Generated Rust artifact containing the probe.
    pub artifact_path: String,
    /// Checked clause node that owns the condition.
    pub node_id: CheckedNodeId,
    /// Evaluation entry or implication consequent.
    pub role: String,
    /// One-based first source line.
    pub start_line: u32,
    /// One-based last source line.
    pub end_line: u32,
    /// Exact executable token to measure.
    pub probe: SourceProbe,
    /// Consequent census, present on the evaluation entry only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_consequents: Option<u32>,
}

/// Why a selected clause condition could not be generated.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V2ClauseRefusal {
    /// The checked clause's body is outside this generator's supported shape.
    UnsupportedCondition,
    /// An admitted node is absent or is not a state clause.
    NotClause,
    /// Source rendering exceeded a finite ceiling or failed internally.
    Render,
}

/// Derive the typed descriptor of an admitted V2 clause condition without rendering source.
pub fn derive_v2_clause_descriptor(
    package: &CheckedPackageV2,
    node_id: &CheckedNodeId,
) -> Result<V2ClauseDescriptor, V2ClauseRefusal> {
    let nodes = ClauseGraphView::new(package);
    derive_v2_clause_descriptor_with_view(node_id, &nodes)
}

pub(crate) fn derive_v2_clause_descriptor_with_view(
    node_id: &CheckedNodeId,
    nodes: &ClauseGraphView<'_>,
) -> Result<V2ClauseDescriptor, V2ClauseRefusal> {
    Ok(plan_clause(node_id, nodes)?.descriptor)
}

/// Settle one selected clause as one typed claim-map entry.
pub fn generate_v2_clause_claim_map(
    package: &CheckedPackageV2,
    node_id: &CheckedNodeId,
) -> ClaimMap<V2ClauseClaim> {
    let result = match generate_v2_clause_oracle(package, node_id) {
        Ok(generated) => ClaimDisposition::Generated(Box::new(generated)),
        Err(refusal) => ClaimDisposition::Refused { refusal },
    };
    ClaimMap {
        package_id: package.package_id().clone(),
        blocked: Vec::new(),
        items: vec![V2ClauseClaim {
            node_id: node_id.clone(),
            result,
        }],
    }
}

struct PreparedClause {
    descriptor: V2ClauseDescriptor,
    names: BTreeMap<ClauseInput, String>,
    plan: Vec<Step>,
    root: usize,
}

fn plan_clause(
    node_id: &CheckedNodeId,
    nodes: &ClauseGraphView<'_>,
) -> Result<PreparedClause, V2ClauseRefusal> {
    let clause = nodes.node(node_id).ok_or(V2ClauseRefusal::NotClause)?;
    if &*clause.node_tag != "state" || &*clause.semantic_form != "state_clause" {
        return Err(V2ClauseRefusal::NotClause);
    }
    let condition = read_clause(node_id, nodes).ok_or(V2ClauseRefusal::UnsupportedCondition)?;
    let names = condition
        .inputs
        .iter()
        .enumerate()
        .map(|(index, input)| {
            let side = match input.side {
                ClauseInputSide::Current => "current",
                ClauseInputSide::Pre => "pre",
                ClauseInputSide::Parameter => "parameter",
            };
            let candidate = format!("{side}_{}", input.name);
            let name = if syn::parse_str::<syn::Ident>(&candidate).is_ok() {
                candidate
            } else {
                format!("input_{index}")
            };
            (input.clone(), name)
        })
        .collect::<BTreeMap<_, _>>();
    let mut plan = Vec::new();
    let root = lower(&condition.term, &names, &mut plan)?;
    if plan.get(root).ok_or(V2ClauseRefusal::Render)?.kind != ValueKind::Boolean {
        return Err(V2ClauseRefusal::UnsupportedCondition);
    }
    let operation_identities = plan
        .iter()
        .filter_map(|step| match &step.term {
            StepTerm::Operation { identity, .. } => Some(identity.clone()),
            _ => None,
        })
        .collect();
    let inputs = condition
        .inputs
        .iter()
        .map(|input| {
            Ok(ClauseOracleInput {
                name: names.get(input).ok_or(V2ClauseRefusal::Render)?.clone(),
                lower: input.lower,
                upper: input.upper,
            })
        })
        .collect::<Result<Vec<_>, V2ClauseRefusal>>()?;
    let descriptor = V2ClauseDescriptor {
        node_id: node_id.clone(),
        condition_node_id: condition.condition_node_id,
        inputs,
        operation_identities,
        expected_consequents: consequent_count(&plan)?,
    };
    Ok(PreparedClause {
        descriptor,
        names,
        plan,
        root,
    })
}

/// Generate a checked V2 clause's Boolean condition oracle.
pub fn generate_v2_clause_oracle(
    package: &CheckedPackageV2,
    node_id: &CheckedNodeId,
) -> Result<V2ClauseOracle, V2ClauseRefusal> {
    let nodes = ClauseGraphView::new(package);
    generate_v2_clause_oracle_with_view(node_id, &nodes)
}

pub(crate) fn generate_v2_clause_oracle_with_view(
    node_id: &CheckedNodeId,
    nodes: &ClauseGraphView<'_>,
) -> Result<V2ClauseOracle, V2ClauseRefusal> {
    let PreparedClause {
        descriptor,
        names,
        plan,
        root,
    } = plan_clause(node_id, nodes)?;
    let operations = plan
        .iter()
        .enumerate()
        .filter_map(|(index, step)| match &step.term {
            StepTerm::Operation {
                identity,
                operation,
                ..
            } => Some((
                format!("scalar_{index}"),
                identity.clone(),
                operation.clone(),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut source =
        render_clause_operation_helpers(&operations).map_err(|_| V2ClauseRefusal::Render)?;
    source.push_str(STOP_TYPE);
    if !operations.is_empty() {
        source.push_str(MEASURED_HELPER);
    }
    let parameters = names
        .values()
        .map(|name| format!("{name}: &rt::Integer, "))
        .collect::<String>();
    let arguments = names
        .values()
        .map(|name| format!("{name}, "))
        .collect::<String>();
    for (index, step) in plan.iter().enumerate() {
        render_step(&mut source, index, step, &names, &parameters, &arguments)?;
    }
    source.push_str(&format!("\n/// The admitted V2 clause condition.\npub fn oracle_clause({parameters}meter: &mut rt::Meter) -> Result<rt::Outcome<bool>, ClauseStop> {{\n    match value_{root}({arguments}meter) {{\n        Ok(value) => Ok(rt::Outcome::Completed(value)),\n        Err(ClauseStop::Runtime(outcome)) => Ok(outcome),\n        Err(other) => Err(other),\n    }}\n}}\n"));
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(V2ClauseRefusal::Render);
    }
    let source_map = coverage_regions(&source, node_id, &plan)?;
    Ok(V2ClauseOracle {
        provenance: OperationProvenance::IrConfirmed,
        source,
        descriptor,
        source_map,
    })
}

fn coverage_regions(
    source: &str,
    node_id: &CheckedNodeId,
    plan: &[Step],
) -> Result<Vec<V2ClauseCoverageRegion>, V2ClauseRefusal> {
    let lines = source.lines().collect::<Vec<_>>();
    let entry = lines
        .iter()
        .position(|line| line.starts_with("pub fn oracle_clause("))
        .ok_or(V2ClauseRefusal::Render)?;
    let census = consequent_count(plan)?;
    let mut regions = vec![coverage_region(
        node_id,
        "oracle_evaluation",
        entry,
        0,
        3,
        Some(census),
    )?];
    for (index, step) in plan.iter().enumerate() {
        let StepTerm::Operation {
            operation:
                ExactScalarOperation::Boolean {
                    operator: BooleanOperator::Implies,
                    ..
                },
            arguments,
            ..
        } = &step.term
        else {
            continue;
        };
        let consequent = arguments.get(1).ok_or(V2ClauseRefusal::Render)?;
        let declaration = format!("fn value_{index}(");
        let start = lines
            .iter()
            .position(|line| line.starts_with(&declaration))
            .ok_or(V2ClauseRefusal::Render)?;
        let token = format!("value_{consequent}(");
        let (line, column) = lines
            .iter()
            .enumerate()
            .skip(start + 1)
            .take_while(|(_, line)| !line.starts_with('}'))
            .find_map(|(line, text)| text.find(&token).map(|column| (line, column)))
            .ok_or(V2ClauseRefusal::Render)?;
        regions.push(coverage_region(
            node_id,
            "implication_consequent",
            line,
            column,
            // LLVM can split the call after its `value_` prefix when the right side is
            // skipped. Keep the probe inside that first count-bearing span.
            "value_".len(),
            None,
        )?);
    }
    Ok(regions)
}

fn consequent_count(plan: &[Step]) -> Result<u32, V2ClauseRefusal> {
    u32::try_from(
        plan.iter()
            .filter(|step| {
                matches!(
                    &step.term,
                    StepTerm::Operation {
                        operation: ExactScalarOperation::Boolean {
                            operator: BooleanOperator::Implies,
                            ..
                        },
                        ..
                    }
                )
            })
            .count(),
    )
    .map_err(|_| V2ClauseRefusal::Render)
}

fn coverage_region(
    node_id: &CheckedNodeId,
    role: &str,
    line: usize,
    column: usize,
    width: usize,
    expected_consequents: Option<u32>,
) -> Result<V2ClauseCoverageRegion, V2ClauseRefusal> {
    let line = u32::try_from(line + 1).map_err(|_| V2ClauseRefusal::Render)?;
    let start_column = u32::try_from(column + 1).map_err(|_| V2ClauseRefusal::Render)?;
    let end_column = start_column
        .checked_add(u32::try_from(width).map_err(|_| V2ClauseRefusal::Render)?)
        .ok_or(V2ClauseRefusal::Render)?;
    Ok(V2ClauseCoverageRegion {
        artifact_path: "src/lib.rs".to_owned(),
        node_id: node_id.clone(),
        role: role.to_owned(),
        start_line: line,
        end_line: line,
        probe: SourceProbe {
            line,
            start_column,
            end_column,
        },
        expected_consequents,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ValueKind {
    Boolean,
    Integer,
}

enum StepTerm {
    Boolean(bool),
    Integer(String),
    Input(ClauseInput),
    Operation {
        identity: String,
        operation: ExactScalarOperation,
        arguments: Vec<usize>,
    },
}

struct Step {
    kind: ValueKind,
    term: StepTerm,
}

fn lower(
    term: &ClauseTerm,
    names: &BTreeMap<ClauseInput, String>,
    plan: &mut Vec<Step>,
) -> Result<usize, V2ClauseRefusal> {
    if plan.len() >= 4096 {
        return Err(V2ClauseRefusal::UnsupportedCondition);
    }
    let step = match term {
        ClauseTerm::Boolean(value) => Step {
            kind: ValueKind::Boolean,
            term: StepTerm::Boolean(*value),
        },
        ClauseTerm::Integer(value) => Step {
            kind: ValueKind::Integer,
            term: StepTerm::Integer(value.clone()),
        },
        ClauseTerm::Input(input) if names.contains_key(input) => Step {
            kind: ValueKind::Integer,
            term: StepTerm::Input(input.clone()),
        },
        ClauseTerm::Input(_) => return Err(V2ClauseRefusal::UnsupportedCondition),
        ClauseTerm::Operation {
            identity,
            arguments,
            result_bound,
        } => {
            let operation = clause_scalar_operation(identity, *result_bound)
                .ok_or(V2ClauseRefusal::UnsupportedCondition)?;
            let children = arguments
                .iter()
                .map(|child| lower(child, names, plan))
                .collect::<Result<Vec<_>, _>>()?;
            let actual = children
                .iter()
                .map(|index| {
                    plan.get(*index)
                        .map(|step| step.kind)
                        .ok_or(V2ClauseRefusal::Render)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let (kind, expected) = match &operation {
                ExactScalarOperation::Boolean {
                    operator: BooleanOperator::Not,
                    ..
                } => (ValueKind::Boolean, vec![ValueKind::Boolean]),
                ExactScalarOperation::Boolean { .. } => {
                    (ValueKind::Boolean, vec![ValueKind::Boolean; 2])
                }
                ExactScalarOperation::Equality {
                    operands: EqualityOperandKind::Boolean,
                    ..
                } => (ValueKind::Boolean, vec![ValueKind::Boolean; 2]),
                ExactScalarOperation::Equality {
                    operands: EqualityOperandKind::Integer,
                    ..
                }
                | ExactScalarOperation::Ordering {
                    operands: OrderingOperandKind::Integer,
                    ..
                } => (ValueKind::Boolean, vec![ValueKind::Integer; 2]),
                ExactScalarOperation::IntegerArithmetic {
                    operator: crate::oracle::operator_catalog::IntegerOperator::Negate,
                    ..
                } => (ValueKind::Integer, vec![ValueKind::Integer]),
                ExactScalarOperation::IntegerArithmetic { .. } => {
                    (ValueKind::Integer, vec![ValueKind::Integer; 2])
                }
                _ => return Err(V2ClauseRefusal::UnsupportedCondition),
            };
            if actual != expected {
                return Err(V2ClauseRefusal::UnsupportedCondition);
            }
            Step {
                kind,
                term: StepTerm::Operation {
                    identity: identity.clone(),
                    operation,
                    arguments: children,
                },
            }
        }
    };
    let index = plan.len();
    plan.push(step);
    Ok(index)
}

const STOP_TYPE: &str = r#"
/// A generated condition stopped.
#[derive(Debug)]
pub enum ClauseStop {
    /// A typed runtime evaluation stop.
    Runtime(rt::Outcome<bool>),
    /// A scalar oracle construction stop.
    Oracle(OracleStop),
    /// An admitted integer literal could not be represented.
    InvalidConstant,
}
"#;

const MEASURED_HELPER: &str = r#"
fn measured<T>(outcome: rt::Outcome<T>) -> Result<T, ClauseStop> {
    match outcome {
        rt::Outcome::Completed(value) => Ok(value),
        rt::Outcome::Undefined(reason) => Err(ClauseStop::Runtime(rt::Outcome::Undefined(reason))),
        rt::Outcome::Refused(reason) => Err(ClauseStop::Runtime(rt::Outcome::Refused(reason))),
        rt::Outcome::Incomplete(record) => Err(ClauseStop::Runtime(rt::Outcome::Incomplete(record))),
    }
}
"#;

fn render_step(
    source: &mut String,
    index: usize,
    step: &Step,
    names: &BTreeMap<ClauseInput, String>,
    parameters: &str,
    arguments: &str,
) -> Result<(), V2ClauseRefusal> {
    let result = match step.kind {
        ValueKind::Boolean => "bool",
        ValueKind::Integer => "rt::Integer",
    };
    source.push_str(&format!("\nfn value_{index}({parameters}meter: &mut rt::Meter) -> Result<{result}, ClauseStop> {{\n"));
    source.push_str("    let _ = (&meter, ");
    for name in names.values() {
        source.push_str(name);
        source.push_str(", ");
    }
    source.push_str(");\n");
    match &step.term {
        StepTerm::Boolean(value) => source.push_str(&format!("    Ok({value})\n")),
        StepTerm::Integer(value) => {
            if let Some(integer) = value
                .parse::<i64>()
                .ok()
                .filter(|integer| integer.to_string() == *value)
            {
                let expression = if integer == i64::MIN {
                    "i64::MIN".to_owned()
                } else {
                    format!("{integer}_i64")
                };
                source.push_str(&format!("    Ok(rt::Integer::from({expression}))\n"));
            } else {
                source.push_str(&format!(
                    "    \"{value}\".parse().map_err(|_| ClauseStop::InvalidConstant)\n"
                ));
            }
        }
        StepTerm::Input(input) => source.push_str(&format!(
            "    Ok({}.clone())\n",
            names.get(input).ok_or(V2ClauseRefusal::Render)?
        )),
        StepTerm::Operation {
            operation,
            arguments: children,
            ..
        } => {
            let arg = |position| -> Result<String, V2ClauseRefusal> {
                let child = children.get(position).ok_or(V2ClauseRefusal::Render)?;
                Ok(format!("value_{child}({arguments}meter)?"))
            };
            let call = match operation {
                ExactScalarOperation::Boolean {
                    operator: BooleanOperator::Not,
                    ..
                } => {
                    format!("let operand = {};\n    let outcome = scalar_{index}(operand, meter).map_err(ClauseStop::Oracle)?;", arg(0)?)
                }
                ExactScalarOperation::Boolean { operator, .. } => {
                    let decided = match operator {
                        BooleanOperator::And => "!left",
                        BooleanOperator::Or => "left",
                        BooleanOperator::Implies => "!left",
                        BooleanOperator::Not => return Err(V2ClauseRefusal::Render),
                    };
                    format!("let left = {};\n    let right = if {decided} {{ false }} else {{ {} }};\n    let outcome = scalar_{index}(left, || rt::Outcome::Completed(right), meter).map_err(ClauseStop::Oracle)?;", arg(0)?, arg(1)?)
                }
                ExactScalarOperation::IntegerArithmetic {
                    operator: crate::oracle::operator_catalog::IntegerOperator::Negate,
                    ..
                } => {
                    format!("let operand = {};\n    let outcome = scalar_{index}(&operand, meter).map_err(ClauseStop::Oracle)?;", arg(0)?)
                }
                ExactScalarOperation::IntegerArithmetic { .. }
                | ExactScalarOperation::Ordering { .. }
                | ExactScalarOperation::Equality {
                    operands: EqualityOperandKind::Integer,
                    ..
                } => {
                    format!("let left = {};\n    let right = {};\n    let outcome = scalar_{index}(&left, &right, meter).map_err(ClauseStop::Oracle)?;", arg(0)?, arg(1)?)
                }
                ExactScalarOperation::Equality {
                    operands: EqualityOperandKind::Boolean,
                    ..
                } => {
                    format!("let left = {};\n    let right = {};\n    let outcome = scalar_{index}(left, right, meter).map_err(ClauseStop::Oracle)?;", arg(0)?, arg(1)?)
                }
                _ => return Err(V2ClauseRefusal::Render),
            };
            source.push_str("    ");
            source.push_str(&call);
            source.push_str("\n    measured(outcome)\n");
        }
    }
    source.push_str("}\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;

    use super::*;

    fn integer_leaf(spelling: &str) -> String {
        let step = Step {
            kind: ValueKind::Integer,
            term: StepTerm::Integer(spelling.to_owned()),
        };
        let mut source = String::new();
        render_step(&mut source, 0, &step, &BTreeMap::new(), "", "")
            .expect("render a standalone integer step");
        syn::parse_file(&source).expect("valid Rust leaf");
        source
    }

    #[trace("TC-024", "FR-014-AC-53")]
    #[test]
    fn canonical_i64_leaves_use_the_public_primitive_constructor() {
        let mut failures = Vec::new();
        for (spelling, value) in [
            ("-9223372036854775808", "i64::MIN"),
            ("-9223372036854775807", "-9223372036854775807_i64"),
            ("-17", "-17_i64"),
            ("-1", "-1_i64"),
            ("0", "0_i64"),
            ("1", "1_i64"),
            ("42", "42_i64"),
            ("1000", "1000_i64"),
            ("9223372036854775807", "9223372036854775807_i64"),
        ] {
            let source = integer_leaf(spelling);
            if !source.contains(&format!("Ok(rt::Integer::from({value}))")) {
                failures.push(spelling);
            }
            assert!(!source.contains("meter.charge"));
        }
        assert!(
            failures.is_empty(),
            "primitive constructor missing: {failures:?}"
        );
    }

    #[trace("TC-024", "FR-014-AC-53")]
    #[test]
    fn every_ineligible_leaf_keeps_the_original_parse_and_typed_failure() {
        for spelling in [
            "-9223372036854775809",
            "9223372036854775808",
            "-170141183460469231731687303715884105728",
            "170141183460469231731687303715884105727",
            "-170141183460469231731687303715884105729",
            "170141183460469231731687303715884105728",
            "+1",
            "01",
            "-0",
            " 1",
            "1 ",
            "1a",
            "",
            "--1",
        ] {
            let source = integer_leaf(spelling);
            assert!(
                source.contains(&format!(
                    "\"{spelling}\".parse().map_err(|_| ClauseStop::InvalidConstant)"
                )),
                "original fallback changed for {spelling:?}: {source}"
            );
            assert!(!source.contains("Integer::from("));
            assert!(!source.contains("meter.charge"));
        }
    }
}

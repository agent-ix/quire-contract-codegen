//! Whole-function Rational predicate native source preparation (FR-046).

use quire_contract_model::{
    CheckedNodeId, CheckedSemanticId, CheckedSemanticNodeV2, CheckedSourceMapEntry,
    ContractFeatureRecordV1, ContractRootViewError, ContractRootViewV1,
};
use quire_exact::{Integer, IntegerInterval, RationalDomain};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

use crate::kani::generate::outcome::FunctionPredicateItem;
use crate::oracle::scalar::RATIONAL_RANGE_MEMBERS;

/// Original finite inclusive component domains for one Rational parameter.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RationalPredicateDomain {
    /// Reduced numerator lower bound.
    pub numerator_minimum: i64,
    /// Reduced numerator upper bound.
    pub numerator_maximum: i64,
    /// Positive denominator lower bound.
    pub denominator_minimum: i64,
    /// Positive denominator upper bound.
    pub denominator_maximum: i64,
}

/// One real function parameter, in ascending original node-id order.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RationalPredicateParameter {
    /// Owner-issued parameter identity.
    pub node: CheckedNodeId,
    /// Original authored name.
    pub name: String,
    /// Original inclusive domain, without clipping or normalization duplicates.
    pub domain: RationalPredicateDomain,
}

/// A native oracle prepared from a complete admitted function closure.
///
/// This is not a Kani harness or a claim that public replay is implemented.
#[derive(Clone, Debug)]
pub struct RationalPredicateOracle {
    /// Owner-admitted package identity.
    pub package: CheckedSemanticId,
    /// Original selected function identity.
    pub function: CheckedNodeId,
    /// Original source correspondence for the reached closure.
    pub sources: Vec<CheckedSourceMapEntry>,
    /// Native Rust source over the public exact-value kernel.
    pub source: String,
    parameters: Vec<RationalPredicateParameter>,
}

impl RationalPredicateOracle {
    /// Logical parameters in ascending original node-id order.
    pub fn parameters(&self) -> &[RationalPredicateParameter] {
        &self.parameters
    }
}

/// A preparation refusal; no proof artifact is emitted for a refused item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RationalPredicateRefusal {
    /// The owner refused or could not reproduce the selected reached view.
    RootView(ContractRootViewError),
    /// The owner's exact non-lowered accounting disposition.
    Lowering(Box<ContractFeatureRecordV1>),
    /// The selected root has no admitted lowered function closure.
    SelectionUnavailable {
        /// Requested original node.
        node: CheckedNodeId,
    },
    /// The body uses an operation outside this exact predicate slice.
    UnsupportedOperation {
        /// Original reached node.
        node: CheckedNodeId,
        /// Original operation or semantic form.
        operation: String,
    },
    /// A Rational parameter has no finite domain.
    RequiresBound {
        /// Original parameter needing the bound.
        parameter: CheckedNodeId,
    },
    /// A finite endpoint cannot be represented by this component ABI.
    UnsupportedEndpoint {
        /// Original parameter.
        parameter: CheckedNodeId,
        /// Named original component endpoint.
        component: String,
        /// Exact original endpoint, not a narrowed substitute.
        value: String,
    },
    /// A reached typed shape cannot be represented by this renderer.
    UnsupportedShape {
        /// Original reached node.
        node: CheckedNodeId,
    },
    /// Preparation exceeded its existing source/work bound.
    PreparationLimit,
}

/// Prepare native source for the complete selected Boolean function predicate.
///
/// # Errors
///
/// Returns a source-associated refusal when the original closure or exact domain
/// is not representable. This entry does not negotiate or run a Kani proof.
pub fn generate_rational_predicate_oracle(
    item: FunctionPredicateItem<'_>,
) -> Result<RationalPredicateOracle, RationalPredicateRefusal> {
    let closure = match item
        .package
        .reached_closure(item.function)
        .map_err(RationalPredicateRefusal::RootView)?
    {
        ContractRootViewV1::Lowered(closure) => closure,
        ContractRootViewV1::NonLowered(record) => {
            return Err(RationalPredicateRefusal::Lowering(Box::new(record.clone())))
        }
        ContractRootViewV1::Absent => {
            return Err(RationalPredicateRefusal::SelectionUnavailable {
                node: item.function.clone(),
            })
        }
    };
    let root = closure.root;
    if root.node_tag.as_ref() != "function" || root.semantic_form.as_ref() != "pure_function" {
        return Err(shape(&root.node_id));
    }
    let graph: BTreeMap<_, _> = closure
        .nodes
        .iter()
        .map(|entry| (&entry.node.node_id, entry.node))
        .collect();
    if node(&graph, &root.semantic_type)?.semantic_form.as_ref() != "boolean" {
        return Err(shape(&root.node_id));
    }
    let parameter_refs = member(&root.body, "parameters")
        .and_then(|p| p.get("members"))
        .and_then(Value::as_array)
        .ok_or_else(|| shape(&root.node_id))?;
    let mut parameters = Vec::new();
    for reference in parameter_refs {
        let id = reference_id(reference).ok_or_else(|| shape(&root.node_id))?;
        let parameter = node(&graph, &id)?;
        if parameter.node_tag.as_ref() != "value" || parameter.semantic_form.as_ref() != "parameter"
        {
            return Err(shape(&id));
        }
        let bounds = domain(&graph, &parameter.semantic_type)?.ok_or_else(|| {
            RationalPredicateRefusal::RequiresBound {
                parameter: id.clone(),
            }
        })?;
        let mut endpoints = [0; 4];
        for (index, (component, value)) in RATIONAL_RANGE_MEMBERS.iter().zip(&bounds).enumerate() {
            endpoints[index] = value.parse::<i64>().map_err(|_| {
                RationalPredicateRefusal::UnsupportedEndpoint {
                    parameter: id.clone(),
                    component: (*component).into(),
                    value: value.clone(),
                }
            })?;
        }
        let name = member(&parameter.body, "name")
            .and_then(|n| n.get("value"))
            .and_then(Value::as_str)
            .ok_or_else(|| shape(&id))?
            .to_owned();
        parameters.push(RationalPredicateParameter {
            node: id,
            name,
            domain: RationalPredicateDomain {
                numerator_minimum: endpoints[0],
                numerator_maximum: endpoints[1],
                denominator_minimum: endpoints[2],
                denominator_maximum: endpoints[3],
            },
        });
    }
    parameters.sort_by(|a, b| a.node.cmp(&b.node));
    if parameters.windows(2).any(|p| p[0].node == p[1].node) {
        return Err(shape(&root.node_id));
    }
    let body = member(&root.body, "body")
        .and_then(reference_id)
        .ok_or_else(|| shape(&root.node_id))?;
    let equality = node(&graph, &body)?;
    if operation(equality) != Some("quire.op.rational.eq") {
        return Err(unsupported(equality));
    }
    let operands = arguments(equality)?;
    let (instructions, left, right) = instructions(&graph, &parameters, &operands)?;
    let source = render(&parameters, &instructions, left, right)?;
    Ok(RationalPredicateOracle {
        package: item.package.identity().clone(),
        function: item.function.clone(),
        sources: closure
            .nodes
            .iter()
            .flat_map(|entry| entry.sources.iter().cloned())
            .collect(),
        source,
        parameters,
    })
}

type Graph<'a> = BTreeMap<&'a CheckedNodeId, &'a CheckedSemanticNodeV2>;

fn shape(node: &CheckedNodeId) -> RationalPredicateRefusal {
    RationalPredicateRefusal::UnsupportedShape { node: node.clone() }
}
fn unsupported(node: &CheckedSemanticNodeV2) -> RationalPredicateRefusal {
    RationalPredicateRefusal::UnsupportedOperation {
        node: node.node_id.clone(),
        operation: operation(node).unwrap_or(&node.semantic_form).into(),
    }
}
fn node<'a>(
    graph: &Graph<'a>,
    id: &CheckedNodeId,
) -> Result<&'a CheckedSemanticNodeV2, RationalPredicateRefusal> {
    graph.get(id).copied().ok_or_else(|| shape(id))
}
fn member<'a>(body: &'a Value, name: &str) -> Option<&'a Value> {
    body.get("members")?
        .as_array()?
        .iter()
        .find(|m| m.get("name").and_then(Value::as_str) == Some(name))?
        .get("value")
}
fn reference_id(value: &Value) -> Option<CheckedNodeId> {
    (value.get("term")?.as_str()? == "reference")
        .then(|| serde_json::from_value(value.get("target")?.clone()).ok())?
}
fn operation(node: &CheckedSemanticNodeV2) -> Option<&str> {
    node.body.get("operation")?.get("identity")?.as_str()
}
fn arguments(node: &CheckedSemanticNodeV2) -> Result<[CheckedNodeId; 2], RationalPredicateRefusal> {
    let values = node
        .body
        .get("arguments")
        .and_then(Value::as_array)
        .ok_or_else(|| shape(&node.node_id))?;
    if values.len() != 2 {
        return Err(shape(&node.node_id));
    }
    Ok([
        reference_id(&values[0]).ok_or_else(|| shape(&node.node_id))?,
        reference_id(&values[1]).ok_or_else(|| shape(&node.node_id))?,
    ])
}
fn domain(
    graph: &Graph<'_>,
    id: &CheckedNodeId,
) -> Result<Option<[String; 4]>, RationalPredicateRefusal> {
    let ty = node(graph, id)?;
    if ty.node_tag.as_ref() == "scalar_type" && ty.semantic_form.as_ref() == "rational" {
        return Ok(None);
    }
    if ty.node_tag.as_ref() != "bounded_domain" || ty.semantic_form.as_ref() != "rational_range" {
        return Err(shape(id));
    }
    let mut bounds = [String::new(), String::new(), String::new(), String::new()];
    let mut values = Vec::new();
    for (index, name) in RATIONAL_RANGE_MEMBERS.iter().enumerate() {
        let text = member(&ty.body, name)
            .and_then(|v| v.get("value"))
            .and_then(Value::as_str)
            .ok_or_else(|| shape(id))?;
        values.push(text.parse::<Integer>().map_err(|_| shape(id))?);
        bounds[index] = text.into();
    }
    let numerator =
        IntegerInterval::new(values[0].clone(), values[1].clone()).map_err(|_| shape(id))?;
    let denominator =
        IntegerInterval::new(values[2].clone(), values[3].clone()).map_err(|_| shape(id))?;
    RationalDomain::new(numerator, denominator).map_err(|_| shape(id))?;
    Ok(Some(bounds))
}

enum Instruction {
    Parameter(usize),
    Add {
        left: usize,
        right: usize,
        domain: Option<[String; 4]>,
    },
}
enum Work {
    Visit(CheckedNodeId),
    FinishAdd(Option<[String; 4]>),
}
fn instructions(
    graph: &Graph<'_>,
    parameters: &[RationalPredicateParameter],
    roots: &[CheckedNodeId; 2],
) -> Result<(Vec<Instruction>, usize, usize), RationalPredicateRefusal> {
    let mut pending = vec![Work::Visit(roots[1].clone()), Work::Visit(roots[0].clone())];
    let mut values = Vec::new();
    let mut code = Vec::new();
    let mut work = 0;
    while let Some(next) = pending.pop() {
        work += 1;
        if work > super::scalar::SCALAR_LOWERING_WORK_LIMIT {
            return Err(RationalPredicateRefusal::PreparationLimit);
        }
        let instruction = match next {
            Work::Visit(id) => {
                if let Some(index) = parameters.iter().position(|p| p.node == id) {
                    Instruction::Parameter(index)
                } else {
                    let node = node(graph, &id)?;
                    if operation(node) != Some("quire.op.rational.add") {
                        return Err(unsupported(node));
                    }
                    let args = arguments(node)?;
                    pending.push(Work::FinishAdd(domain(graph, &node.semantic_type)?));
                    pending.push(Work::Visit(args[1].clone()));
                    pending.push(Work::Visit(args[0].clone()));
                    continue;
                }
            }
            Work::FinishAdd(domain) => {
                let right = values
                    .pop()
                    .ok_or(RationalPredicateRefusal::PreparationLimit)?;
                let left = values
                    .pop()
                    .ok_or(RationalPredicateRefusal::PreparationLimit)?;
                Instruction::Add {
                    left,
                    right,
                    domain,
                }
            }
        };
        values.push(code.len());
        code.push(instruction);
    }
    if values.len() != 2 {
        return Err(RationalPredicateRefusal::PreparationLimit);
    }
    Ok((code, values[0], values[1]))
}

const HEADER: &str = r#"#![forbid(unsafe_code)]
use quire_exact as exact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PredicateInputRefusal { ArgumentCount, ArgumentDomain { index: usize }, InvalidConstant }
fn domain(parts:[&str;4])->Result<exact::RationalDomain,PredicateInputRefusal> {
    let integer=|s:&str|s.parse::<exact::Integer>().map_err(|_|PredicateInputRefusal::InvalidConstant);
    let n=exact::IntegerInterval::new(integer(parts[0])?,integer(parts[1])?).map_err(|_|PredicateInputRefusal::InvalidConstant)?;
    let d=exact::IntegerInterval::new(integer(parts[2])?,integer(parts[3])?).map_err(|_|PredicateInputRefusal::InvalidConstant)?;
    exact::RationalDomain::new(n,d).map_err(|_|PredicateInputRefusal::InvalidConstant)
}
pub fn predicate(arguments:&[exact::Rational],meter:&mut exact::Meter)->Result<exact::Outcome<bool>,PredicateInputRefusal> {
"#;
fn array(bounds: &[String; 4]) -> String {
    format!(
        "[{:?},{:?},{:?},{:?}]",
        bounds[0], bounds[1], bounds[2], bounds[3]
    )
}
fn render(
    parameters: &[RationalPredicateParameter],
    instructions: &[Instruction],
    left: usize,
    right: usize,
) -> Result<String, RationalPredicateRefusal> {
    let mut source = HEADER.to_owned();
    source.push_str(&format!(
        "if arguments.len()!={} {{return Err(PredicateInputRefusal::ArgumentCount);}}\n",
        parameters.len()
    ));
    for (index, p) in parameters.iter().enumerate() {
        let b = &p.domain;
        let bounds = [
            b.numerator_minimum.to_string(),
            b.numerator_maximum.to_string(),
            b.denominator_minimum.to_string(),
            b.denominator_maximum.to_string(),
        ];
        source.push_str(&format!("if !domain({})?.contains(&arguments[{index}]) {{return Err(PredicateInputRefusal::ArgumentDomain{{index:{index}}});}}\n",array(&bounds)));
    }
    for (index, instruction) in instructions.iter().enumerate() {
        match instruction {
            Instruction::Parameter(parameter) => source.push_str(&format!(
                "let value_{index}=arguments[{parameter}].clone();\n"
            )),
            Instruction::Add {
                left,
                right,
                domain,
            } => {
                let bound = match domain {
                    Some(bounds) => {
                        source
                            .push_str(&format!("let domain_{index}=domain({})?;\n", array(bounds)));
                        format!("Some(&domain_{index})")
                    }
                    None => "None".into(),
                };
                source.push_str(&format!("let value_{index}=match exact::evaluate_rational_arithmetic(exact::RationalArithmetic::Add(&value_{left},&value_{right}),{bound},meter) {{\nexact::Outcome::Completed(value)=>value,\nexact::Outcome::Undefined(reason)=>return Ok(exact::Outcome::Undefined(reason)),\nexact::Outcome::Refused(reason)=>return Ok(exact::Outcome::Refused(reason)),\nexact::Outcome::Incomplete(reason)=>return Ok(exact::Outcome::Incomplete(reason)),\n}};\n"));
            }
        }
        if source.len() > crate::core::artifact::MAX_GENERATED_SOURCE_BYTES {
            return Err(RationalPredicateRefusal::PreparationLimit);
        }
    }
    source.push_str(&format!("Ok(exact::planned_equality(&exact::Value::Rational(value_{left}),&exact::Value::Rational(value_{right}),meter))\n}}\n"));
    if source.len() > crate::core::artifact::MAX_GENERATED_SOURCE_BYTES {
        return Err(RationalPredicateRefusal::PreparationLimit);
    }
    Ok(source)
}

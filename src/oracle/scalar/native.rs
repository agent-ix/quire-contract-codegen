// SPDX-License-Identifier: AGPL-3.0-or-later
//! Production rendering of kernel-admitted native scalar arithmetic.

use quire_contract_model::CheckedNodeId;
use quire_exact::{NativeIntegerInterval, NativeIntegerOperation, NativeIntegerPlan};

use crate::core::native_integer::{
    NativeIntegerIntervalDecision, NativeIntegerOperationDecision, NativeIntegerSelection,
};
use crate::oracle::operator_catalog::{IntegerOperator, Operator as CatalogOperator};

pub(crate) fn select(
    node_id: &CheckedNodeId,
    identity: &str,
    symbol: &str,
    operator: IntegerOperator,
    operands: &[(i64, i64)],
    result: (i64, i64),
) -> Option<(String, NativeIntegerSelection)> {
    let interval = |position: usize| {
        let &(lower, upper) = operands.get(position)?;
        NativeIntegerInterval::new(i128::from(lower), i128::from(upper)).ok()
    };
    let left = interval(0)?;
    let operation = match operator {
        IntegerOperator::Add => NativeIntegerOperation::Add {
            left,
            right: interval(1)?,
        },
        IntegerOperator::Subtract => NativeIntegerOperation::Subtract {
            left,
            right: interval(1)?,
        },
        IntegerOperator::Multiply => NativeIntegerOperation::Multiply {
            left,
            right: interval(1)?,
        },
        IntegerOperator::Negate => NativeIntegerOperation::Negate { operand: left },
    };
    let domain = NativeIntegerInterval::new(i128::from(result.0), i128::from(result.1)).ok()?;
    let plan = NativeIntegerPlan::new(operation, Some(domain)).ok()?;
    let mut selection = NativeIntegerSelection::new();
    selection.operations.push(NativeIntegerOperationDecision {
        node_id: node_id.clone(),
        step: 0,
        operation_identity: identity.to_owned(),
        operands: operands
            .iter()
            .map(|&(lower, upper)| NativeIntegerIntervalDecision {
                lower: lower.to_string(),
                upper: upper.to_string(),
            })
            .collect(),
        capacity: Some(plan.capacity().into()),
        semantic_domain: Some(domain.into()),
    });
    Some((render(symbol, operator, &selection)?, selection))
}

fn literal(endpoint: i128) -> String {
    if endpoint == i128::MIN {
        "i128::MIN".to_owned()
    } else {
        format!("{endpoint}_i128")
    }
}

fn interval_source(interval: NativeIntegerInterval) -> String {
    format!(
        "nx::NativeIntegerInterval::new({}, {}).map_err(OracleStop::NativeAdmission)?",
        literal(interval.lower()),
        literal(interval.upper())
    )
}

fn decision_interval(interval: &NativeIntegerIntervalDecision) -> Option<NativeIntegerInterval> {
    NativeIntegerInterval::new(interval.lower.parse().ok()?, interval.upper.parse().ok()?).ok()
}

/// Render a selected plan for both the routed artifact and its proof harness.
pub(crate) fn render(
    symbol: &str,
    operator: IntegerOperator,
    selection: &NativeIntegerSelection,
) -> Option<String> {
    let [decision] = selection.operations.as_slice() else {
        return None;
    };
    let identity = decision.operation_identity.as_str();
    if CatalogOperator::from_identity(identity)?
        .entry()
        .scalar_operator
        != Some(operator)
    {
        return None;
    }
    let (parameters, descriptor, payloads) = match (operator, decision.operands.as_slice()) {
        (IntegerOperator::Negate, [operand]) => (
            "operand: &nx::NativeInteger",
            format!(
                "nx::NativeIntegerOperation::Negate {{ operand: {} }}",
                interval_source(decision_interval(operand)?)
            ),
            "nx::NativeIntegerOperands::Unary { operand: *operand }",
        ),
        (operator, [left, right]) => {
            let variant = match operator {
                IntegerOperator::Add => "Add",
                IntegerOperator::Subtract => "Subtract",
                IntegerOperator::Multiply => "Multiply",
                IntegerOperator::Negate => return None,
            };
            (
                "left: &nx::NativeInteger, right: &nx::NativeInteger",
                format!(
                    "nx::NativeIntegerOperation::{variant} {{ left: {}, right: {} }}",
                    interval_source(decision_interval(left)?),
                    interval_source(decision_interval(right)?)
                ),
                "nx::NativeIntegerOperands::Binary { left: *left, right: *right }",
            )
        }
        _ => return None,
    };
    let domain = decision_interval(decision.semantic_domain.as_ref()?)?;
    let capacity = decision_interval(decision.capacity.as_ref()?)?;
    let operation = match (operator, decision.operands.as_slice()) {
        (IntegerOperator::Negate, [operand]) => NativeIntegerOperation::Negate {
            operand: decision_interval(operand)?,
        },
        (IntegerOperator::Add, [left, right]) => NativeIntegerOperation::Add {
            left: decision_interval(left)?,
            right: decision_interval(right)?,
        },
        (IntegerOperator::Subtract, [left, right]) => NativeIntegerOperation::Subtract {
            left: decision_interval(left)?,
            right: decision_interval(right)?,
        },
        (IntegerOperator::Multiply, [left, right]) => NativeIntegerOperation::Multiply {
            left: decision_interval(left)?,
            right: decision_interval(right)?,
        },
        _ => return None,
    };
    if NativeIntegerPlan::new(operation, Some(domain))
        .ok()?
        .capacity()
        != capacity
    {
        return None;
    }
    let mut source = format!(
        "// @generated by quire-contract-codegen. Do not edit.\nuse quire_exact as rt;\nuse quire_exact as nx;\n\n/// Why native admission stopped.\n#[derive(Debug)]\npub enum OracleStop {{\n    /// The exact kernel rejected the plan or operand membership.\n    NativeAdmission(nx::NativeIntegerAdmissionError),\n}}\n\n/// Admitted checked {identity} operation.\npub fn {symbol}({parameters}, meter: &mut rt::Meter) -> Result<rt::Outcome<nx::NativeInteger>, OracleStop> {{\n    let plan = nx::NativeIntegerPlan::new({descriptor}, Some({})).map_err(OracleStop::NativeAdmission)?;\n    let admitted = plan.admit({payloads}).map_err(OracleStop::NativeAdmission)?;\n    Ok(nx::evaluate_native_integer_arithmetic(admitted, meter))\n}}\n",
        interval_source(domain)
    );
    source.push_str(&selection.annotation()?);
    let subject = crate::core::provenance::node_subject(&decision.node_id);
    Some(crate::core::provenance::with_rust_header(
        source,
        [subject.as_str()],
    ))
}

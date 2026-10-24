// SPDX-License-Identifier: AGPL-3.0-or-later
//! Kernel admission and rendering policy for the shared checked clause plan.

use quire_exact::{NativeIntegerInterval, NativeIntegerOperation, NativeIntegerPlan};

use super::{
    native_literal, render_clause_operation_helpers, ExactScalarOperation,
    NativeClauseCapabilityReason, Step, StepTerm, V2ClauseRefusal,
};
use crate::core::native_integer::{NativeIntegerOperationDecision, NativeIntegerSelection};
use crate::oracle::operator_catalog::IntegerOperator;
use quire_contract_model::CheckedNodeId;
use quire_exact::{IntegerDomain, OrderingOperator};

fn refusal(
    plan: &[Step],
    node_id: &CheckedNodeId,
    step: usize,
    reason: NativeClauseCapabilityReason,
) -> V2ClauseRefusal {
    V2ClauseRefusal::NativeCapability {
        node_id: node_id.clone(),
        step,
        reason,
        operands: Vec::new(),
        operation_identity: plan.get(step).and_then(|step| match &step.term {
            StepTerm::Operation { identity, .. } => Some(identity.clone()),
            _ => None,
        }),
    }
}

fn interval_source(interval: NativeIntegerInterval) -> String {
    format!(
        "nx::NativeIntegerInterval::new({}, {}).map_err(OracleStop::NativeAdmission)?",
        native_literal(interval.lower()),
        native_literal(interval.upper())
    )
}

pub(super) fn helpers(
    plan: &[Step],
    node_id: &CheckedNodeId,
) -> Result<(String, NativeIntegerSelection), V2ClauseRefusal> {
    let booleans = plan
        .iter()
        .enumerate()
        .filter_map(|(index, step)| match &step.term {
            StepTerm::Operation {
                identity,
                operation: operation @ ExactScalarOperation::Boolean { .. },
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
        render_clause_operation_helpers(&booleans).map_err(|_| V2ClauseRefusal::Render)?;
    source.push_str("\nuse quire_exact as nx;\n");
    source = source.replace(
        "    OperandWidth,",
        "    OperandWidth,\n    /// The production kernel rejected native interval or operand admission.\n    NativeAdmission(nx::NativeIntegerAdmissionError),",
    );
    let mut selected = NativeIntegerSelection::new();
    let mut ranges = Vec::<Option<NativeIntegerInterval>>::with_capacity(plan.len());
    for (index, step) in plan.iter().enumerate() {
        let range = match &step.term {
            StepTerm::Boolean(_) => None,
            StepTerm::Integer(value) => {
                let value = value.parse::<i128>().map_err(|_| {
                    refusal(
                        plan,
                        node_id,
                        index,
                        NativeClauseCapabilityReason::EndpointNotRepresentable,
                    )
                })?;
                Some(
                    NativeIntegerInterval::new(value, value)
                        .map_err(|_| V2ClauseRefusal::Render)?,
                )
            }
            StepTerm::Input(input) => Some(
                NativeIntegerInterval::new(input.lower, input.upper)
                    .map_err(|_| V2ClauseRefusal::Render)?,
            ),
            StepTerm::Operation {
                identity,
                operation,
                arguments,
            } => {
                let operand = |position| {
                    arguments
                        .get(position)
                        .and_then(|child| ranges.get(*child))
                        .copied()
                        .flatten()
                        .ok_or(V2ClauseRefusal::Render)
                };
                match operation {
                    ExactScalarOperation::Boolean { .. } => None,
                    ExactScalarOperation::Ordering { operator, .. } => {
                        let operator = match operator {
                            OrderingOperator::Less => "Less",
                            OrderingOperator::LessOrEqual => "LessOrEqual",
                            OrderingOperator::Greater => "Greater",
                            OrderingOperator::GreaterOrEqual => "GreaterOrEqual",
                        };
                        let operands = vec![operand(0)?.into(), operand(1)?.into()];
                        selected.operations.push(NativeIntegerOperationDecision {
                            node_id: node_id.clone(),
                            step: index,
                            operation_identity: identity.clone(),
                            operands,
                            capacity: None,
                            semantic_domain: None,
                        });
                        source.push_str(&format!("\n/// `{identity}`.\npub fn scalar_{index}(left: &nx::NativeInteger, right: &nx::NativeInteger, meter: &mut rt::Meter) -> Result<rt::Outcome<bool>, OracleStop> {{\n    Ok(nx::evaluate_native_integer_ordering(nx::OrderingOperator::{operator}, *left, *right, meter))\n}}\n"));
                        None
                    }
                    ExactScalarOperation::IntegerArithmetic { operator, domain } => {
                        let left = operand(0)?;
                        let left_source = interval_source(left);
                        let (native, native_source, parameters, payloads) = match operator {
                            IntegerOperator::Negate => (
                                NativeIntegerOperation::Negate { operand: left },
                                format!("nx::NativeIntegerOperation::Negate {{ operand: {left_source} }}"),
                                "operand: &nx::NativeInteger",
                                "nx::NativeIntegerOperands::Unary { operand: *operand }",
                            ),
                            operator => {
                                let right = operand(1)?;
                                let (native, variant) = match operator {
                                    IntegerOperator::Add => (NativeIntegerOperation::Add { left, right }, "Add"),
                                    IntegerOperator::Subtract => (NativeIntegerOperation::Subtract { left, right }, "Subtract"),
                                    IntegerOperator::Multiply => (NativeIntegerOperation::Multiply { left, right }, "Multiply"),
                                    _ => return Err(refusal(plan, node_id, index, NativeClauseCapabilityReason::UnsupportedOperation)),
                                };
                                (
                                    native,
                                    format!("nx::NativeIntegerOperation::{variant} {{ left: {left_source}, right: {} }}", interval_source(right)),
                                    "left: &nx::NativeInteger, right: &nx::NativeInteger",
                                    "nx::NativeIntegerOperands::Binary { left: *left, right: *right }",
                                )
                            }
                        };
                        let domain = match domain {
                            IntegerDomain::Mathematical => None,
                            IntegerDomain::Bounded(interval) => {
                                Some(NativeIntegerInterval::try_from(interval).map_err(|_| {
                                    refusal(
                                        plan,
                                        node_id,
                                        index,
                                        NativeClauseCapabilityReason::EndpointNotRepresentable,
                                    )
                                })?)
                            }
                        };
                        let operands = if *operator == IntegerOperator::Negate {
                            vec![left.into()]
                        } else {
                            vec![left.into(), operand(1)?.into()]
                        };
                        let admitted = NativeIntegerPlan::new(native, domain).map_err(|_| {
                            let mut error = refusal(
                                plan,
                                node_id,
                                index,
                                NativeClauseCapabilityReason::CapacityExceeded,
                            );
                            if let V2ClauseRefusal::NativeCapability {
                                operands: refused, ..
                            } = &mut error
                            {
                                *refused = operands.clone();
                            }
                            error
                        })?;
                        selected.operations.push(NativeIntegerOperationDecision {
                            node_id: node_id.clone(),
                            step: index,
                            operation_identity: identity.clone(),
                            operands,
                            capacity: Some(admitted.capacity().into()),
                            semantic_domain: domain.map(Into::into),
                        });
                        let domain_source = domain.map_or_else(
                            || "None".to_owned(),
                            |value| format!("Some({})", interval_source(value)),
                        );
                        source.push_str(&format!("\n/// `{identity}` with kernel-admitted complete interval capacity.\npub fn scalar_{index}({parameters}, meter: &mut rt::Meter) -> Result<rt::Outcome<nx::NativeInteger>, OracleStop> {{\n    let plan = nx::NativeIntegerPlan::new({native_source}, {domain_source}).map_err(OracleStop::NativeAdmission)?;\n    let operation = plan.admit({payloads}).map_err(OracleStop::NativeAdmission)?;\n    Ok(nx::evaluate_native_integer_arithmetic(operation, meter))\n}}\n"));
                        // Following nodes can only execute after a successful semantic-domain
                        // check. A disjoint domain has no successful value; retain the whole
                        // capacity conservatively rather than inventing a narrowed population.
                        let capacity = admitted.capacity();
                        Some(completed_interval(capacity, domain))
                    }
                    _ => {
                        return Err(refusal(
                            plan,
                            node_id,
                            index,
                            NativeClauseCapabilityReason::UnsupportedOperation,
                        ))
                    }
                }
            }
        };
        ranges.push(range);
    }
    source.push_str(&selected.annotation().ok_or(V2ClauseRefusal::Render)?);
    Ok((source, selected))
}

// Only values that completed the real prior semantic-domain check reach the next
// operation. Empty successful populations conservatively retain the full image.
fn completed_interval(
    capacity: NativeIntegerInterval,
    domain: Option<NativeIntegerInterval>,
) -> NativeIntegerInterval {
    if let Some(domain) = domain {
        NativeIntegerInterval::new(
            capacity.lower().max(domain.lower()),
            capacity.upper().min(domain.upper()),
        )
        .unwrap_or(capacity)
    } else {
        capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_exact::{
        evaluate_native_integer_arithmetic, Meter, NativeInteger, NativeIntegerOperands, Outcome,
        Refusal, ScalarLimits,
    };

    fn interval(lower: i128, upper: i128) -> NativeIntegerInterval {
        NativeIntegerInterval::new(lower, upper).unwrap()
    }

    fn limits() -> ScalarLimits {
        ScalarLimits {
            integer_bits: 10000,
            decimal_digits: 10000,
            scale_expansion: 10000,
            text_input_bytes: 10000,
            text_scalars: 10000,
            normalized_scalars: 10000,
            unit_edges: 10000,
            value_occurrences: 10000,
            work_units: 10000,
            result_units: 10000,
        }
    }

    #[trace("FR-015-AC-125", "FR-015-AC-127")]
    #[test]
    fn completed_prefix_bounds_preserve_the_original_semantic_refusal() {
        let maximum = i128::from(i32::MAX);
        let domain = interval(0, maximum);
        let add = NativeIntegerPlan::new(
            NativeIntegerOperation::Add {
                left: domain,
                right: interval(1, 1),
            },
            Some(domain),
        )
        .unwrap();
        assert_eq!(
            (add.capacity().lower(), add.capacity().upper()),
            (1, maximum + 1)
        );
        let completed = completed_interval(add.capacity(), Some(domain));
        assert_eq!((completed.lower(), completed.upper()), (1, maximum));
        let multiply = NativeIntegerPlan::new(
            NativeIntegerOperation::Multiply {
                left: completed,
                right: interval(0, 0),
            },
            None,
        )
        .unwrap();
        let mut meter = Meter::new(limits());
        let stopped = evaluate_native_integer_arithmetic(
            add.admit(NativeIntegerOperands::Binary {
                left: NativeInteger::new(maximum),
                right: NativeInteger::new(1),
            })
            .unwrap(),
            &mut meter,
        );
        assert!(matches!(
            stopped,
            Outcome::Refused(Refusal::IntegerOutOfDomain { .. })
        ));
        let mut meter = Meter::new(limits());
        let Outcome::Completed(value) = evaluate_native_integer_arithmetic(
            add.admit(NativeIntegerOperands::Binary {
                left: NativeInteger::new(maximum - 1),
                right: NativeInteger::new(1),
            })
            .unwrap(),
            &mut meter,
        ) else {
            panic!("the original in-domain prefix completes");
        };
        let outcome = evaluate_native_integer_arithmetic(
            multiply
                .admit(NativeIntegerOperands::Binary {
                    left: value,
                    right: NativeInteger::new(0),
                })
                .unwrap(),
            &mut meter,
        );
        assert_eq!(outcome, Outcome::Completed(NativeInteger::new(0)));
    }

    #[trace("FR-015-AC-125")]
    #[test]
    fn a_fitting_semantic_result_or_zero_product_cannot_repair_multiply_capacity() {
        let wide = interval(i128::from(i32::MAX) + 1, i128::from(i32::MAX) + 1);
        assert!(NativeIntegerPlan::new(
            NativeIntegerOperation::Multiply {
                left: wide,
                right: interval(0, 0),
            },
            Some(interval(0, 0))
        )
        .is_err());
        // A wholly disjoint semantic result has no completed population. Keeping
        // the full conservative image avoids manufacturing a native capability.
        assert_eq!(completed_interval(wide, Some(interval(0, 0))), wide);
    }
}

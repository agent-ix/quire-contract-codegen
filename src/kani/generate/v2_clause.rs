//! One bounded postcondition over an admitted V2 state clause (FR-015).

use quire_contract_model::{CheckedNodeId, CheckedPackageV2};

use crate::{
    core::ir::clause::{read_clause, ClauseTerm},
    kani::{
        abi::{KaniPrimitiveType, KaniSolver},
        generate::{
            outcome::{KaniObligationRequest, UnsupportedObligation},
            record::{harness_path, record},
            render,
            spec::{
                Argument, Cover, ExprText, HarnessSpec, Init, LetBinding, ProofKind, Statement,
            },
        },
        identity::{
            KaniV2ClauseObligationHarness, ObligationKind, ScalarObligationArgument,
            V2ClauseObligationIdentity,
        },
    },
    oracle::clause::{generate_v2_clause_oracle, V2ClauseOracle},
};

pub(super) struct LoweredV2Clause {
    pub(super) node_id: CheckedNodeId,
    pub(super) kind: ObligationKind,
    oracle: V2ClauseOracle,
    native_expectation: String,
    arithmetic: Vec<ArithmeticProbe>,
    symbolic_name: String,
    pub(super) module_symbol: String,
    pub(super) harness_symbol: String,
}

enum NativeTerm {
    Integer {
        expression: String,
        lower: i128,
        upper: i128,
    },
    Boolean(String),
}

struct ArithmeticProbe {
    operation: &'static str,
    operands: Vec<String>,
    expected: String,
}

impl NativeTerm {
    fn boolean(self) -> Option<String> {
        match self {
            Self::Boolean(expression) => Some(expression),
            Self::Integer { .. } => None,
        }
    }
}

fn native_term(
    term: &ClauseTerm,
    input_name: &str,
    symbolic_name: &str,
    arithmetic: &mut Vec<ArithmeticProbe>,
) -> Option<NativeTerm> {
    match term {
        ClauseTerm::Boolean(value) => Some(NativeTerm::Boolean(value.to_string())),
        ClauseTerm::Integer(value) => {
            let parsed = value.parse::<i128>().ok()?;
            Some(NativeTerm::Integer {
                expression: format!("{parsed}i128"),
                lower: parsed,
                upper: parsed,
            })
        }
        ClauseTerm::Input(input)
            if input.name == input_name
                && input.lower <= input.upper
                && i64::try_from(input.lower).is_ok()
                && i64::try_from(input.upper).is_ok() =>
        {
            Some(NativeTerm::Integer {
                expression: format!("i128::from({symbolic_name})"),
                lower: input.lower,
                upper: input.upper,
            })
        }
        ClauseTerm::Input(_) => None,
        ClauseTerm::Operation {
            identity,
            arguments,
            ..
        } => {
            let values = arguments
                .iter()
                .map(|argument| native_term(argument, input_name, symbolic_name, arithmetic))
                .collect::<Option<Vec<_>>>()?;
            match (identity.as_str(), values.as_slice()) {
                ("quire.op.boolean.not", [value]) => Some(NativeTerm::Boolean(format!(
                    "(!{})",
                    value.clone_boolean()?
                ))),
                ("quire.op.boolean.and", [left, right]) => Some(NativeTerm::Boolean(format!(
                    "({} && {})",
                    left.clone_boolean()?,
                    right.clone_boolean()?
                ))),
                ("quire.op.boolean.or", [left, right]) => Some(NativeTerm::Boolean(format!(
                    "({} || {})",
                    left.clone_boolean()?,
                    right.clone_boolean()?
                ))),
                ("quire.op.boolean.implies", [left, right]) => Some(NativeTerm::Boolean(format!(
                    "(!{} || {})",
                    left.clone_boolean()?,
                    right.clone_boolean()?
                ))),
                ("quire.op.integer.negate", [value]) => {
                    let (expression, lower, upper) = value.clone_integer()?;
                    let expected = format!("(-{expression})");
                    arithmetic.push(ArithmeticProbe {
                        operation: "Negate",
                        operands: vec![expression],
                        expected: expected.clone(),
                    });
                    Some(NativeTerm::Integer {
                        expression: expected,
                        lower: upper.checked_neg()?,
                        upper: lower.checked_neg()?,
                    })
                }
                (
                    "quire.op.integer.add" | "quire.op.integer.sub" | "quire.op.integer.mul",
                    [left, right],
                ) => {
                    let (a, al, ah) = left.clone_integer()?;
                    let (b, bl, bh) = right.clone_integer()?;
                    let (operator, operation, bounds) = match identity.as_str() {
                        "quire.op.integer.add" => {
                            ("+", "Add", (al.checked_add(bl)?, ah.checked_add(bh)?))
                        }
                        "quire.op.integer.sub" => {
                            ("-", "Subtract", (al.checked_sub(bh)?, ah.checked_sub(bl)?))
                        }
                        "quire.op.integer.mul" => {
                            let corners = [
                                al.checked_mul(bl)?,
                                al.checked_mul(bh)?,
                                ah.checked_mul(bl)?,
                                ah.checked_mul(bh)?,
                            ];
                            (
                                "*",
                                "Multiply",
                                (*corners.iter().min()?, *corners.iter().max()?),
                            )
                        }
                        _ => return None,
                    };
                    let expected = format!("({a} {operator} {b})");
                    arithmetic.push(ArithmeticProbe {
                        operation,
                        operands: vec![a, b],
                        expected: expected.clone(),
                    });
                    Some(NativeTerm::Integer {
                        expression: expected,
                        lower: bounds.0,
                        upper: bounds.1,
                    })
                }
                (
                    "quire.op.integer.lt"
                    | "quire.op.integer.le"
                    | "quire.op.integer.gt"
                    | "quire.op.integer.ge",
                    [left, right],
                ) => {
                    let (a, _, _) = left.clone_integer()?;
                    let (b, _, _) = right.clone_integer()?;
                    let operator = match identity.as_str() {
                        "quire.op.integer.lt" => "<",
                        "quire.op.integer.le" => "<=",
                        "quire.op.integer.gt" => ">",
                        "quire.op.integer.ge" => ">=",
                        _ => return None,
                    };
                    Some(NativeTerm::Boolean(format!("({a} {operator} {b})")))
                }
                ("quire.op.integer.eq" | "quire.op.integer.ne", [left, right]) => {
                    let (a, _, _) = left.clone_integer()?;
                    let (b, _, _) = right.clone_integer()?;
                    let operator = if identity == "quire.op.integer.eq" {
                        "=="
                    } else {
                        "!="
                    };
                    Some(NativeTerm::Boolean(format!("({a} {operator} {b})")))
                }
                _ => None,
            }
        }
    }
}

impl NativeTerm {
    fn clone_integer(&self) -> Option<(String, i128, i128)> {
        match self {
            Self::Integer {
                expression,
                lower,
                upper,
            } => Some((expression.clone(), *lower, *upper)),
            Self::Boolean(_) => None,
        }
    }

    fn clone_boolean(&self) -> Option<String> {
        match self {
            Self::Boolean(expression) => Some(expression.clone()),
            Self::Integer { .. } => None,
        }
    }
}

pub(super) fn lower(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
) -> Result<LoweredV2Clause, UnsupportedObligation> {
    let oracle = generate_v2_clause_oracle(package, clause).map_err(|_| {
        UnsupportedObligation::V2ClauseConditionNotRendered {
            node_id: clause.clone(),
        }
    })?;
    let [input] = oracle.descriptor.inputs.as_slice() else {
        return Err(UnsupportedObligation::V2ClauseConditionNotRendered {
            node_id: clause.clone(),
        });
    };
    let Some(input_name) = input.name.strip_prefix("parameter_") else {
        return Err(UnsupportedObligation::V2ClauseConditionNotRendered {
            node_id: clause.clone(),
        });
    };
    let input_name = input_name.to_owned();
    let symbolic_name = format!("{input_name}_current");
    let condition = read_clause(package, clause).ok_or_else(|| {
        UnsupportedObligation::V2ClauseConditionNotRendered {
            node_id: clause.clone(),
        }
    })?;
    let mut arithmetic = Vec::new();
    let native_expectation = native_term(
        &condition.term,
        &input_name,
        &symbolic_name,
        &mut arithmetic,
    )
    .and_then(NativeTerm::boolean)
    .ok_or_else(|| UnsupportedObligation::V2ClauseConditionNotRendered {
        node_id: clause.clone(),
    })?;
    let module_symbol = format!(
        "kob_v2_clause_{}_{}",
        package.package_id().digest,
        clause.digest
    );
    Ok(LoweredV2Clause {
        node_id: clause.clone(),
        kind: ObligationKind::Postcondition,
        oracle,
        native_expectation,
        arithmetic,
        symbolic_name,
        harness_symbol: "prove_clause".to_owned(),
        module_symbol,
    })
}

fn expression(text: impl Into<String>) -> Result<ExprText, UnsupportedObligation> {
    ExprText::new(text).map_err(Into::into)
}

fn let_binding(name: &str, init: Init, mutable: bool) -> Statement {
    Statement::Let(LetBinding {
        name: name.to_owned(),
        ty: None,
        mutable,
        init,
    })
}

pub(super) fn render_clause(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredV2Clause,
) -> Result<KaniV2ClauseObligationHarness, UnsupportedObligation> {
    let input = &lowered.oracle.descriptor.inputs[0];
    let (minimum, maximum) = (i64::try_from(input.lower), i64::try_from(input.upper));
    let (Ok(minimum), Ok(maximum)) = (minimum, maximum) else {
        return Err(UnsupportedObligation::V2ClauseConditionNotRendered {
            node_id: lowered.node_id.clone(),
        });
    };
    let mut body = vec![
        Statement::Input {
            name: lowered.symbolic_name.clone(),
            ty: KaniPrimitiveType::I64,
            domain: (input.lower, input.upper),
            then: vec![LetBinding {
                name: input.name.clone(),
                ty: None,
                mutable: false,
                init: Init::Expression(expression(format!(
                    "rt::Integer::from({})",
                    lowered.symbolic_name
                ))?),
            }],
        },
        let_binding(
            "expected",
            Init::Expression(expression(lowered.native_expectation.clone())?),
            false,
        ),
        let_binding(
            "meter",
            Init::ScalarMeter(crate::kani::abi::SCALAR_HARNESS_LIMITS),
            true,
        ),
        Statement::Call {
            result: "outcome".to_owned(),
            callee: "oracle_clause".to_owned(),
            args: vec![
                Argument::Borrow(input.name.clone()),
                Argument::BorrowMut("meter".to_owned()),
            ],
        },
        Statement::ClauseSubjectCall {
            callee: request.subject_path.to_owned(),
            argument: lowered.symbolic_name.clone(),
        },
    ];
    for (index, probe) in lowered.arithmetic.iter().enumerate() {
        let operand_names = probe
            .operands
            .iter()
            .enumerate()
            .map(|(operand_index, operand)| {
                let name = format!("arithmetic_operand_{index}_{operand_index}");
                body.push(let_binding(
                    &name,
                    Init::Expression(expression(format!("rt::Integer::from({operand})"))?),
                    false,
                ));
                Ok(name)
            })
            .collect::<Result<Vec<_>, UnsupportedObligation>>()?;
        body.push(let_binding(
            &format!("arithmetic_expected_{index}"),
            Init::Expression(expression(format!(
                "rt::Integer::from({})",
                probe.expected
            ))?),
            false,
        ));
        body.push(let_binding(
            &format!("arithmetic_meter_{index}"),
            Init::ScalarMeter(crate::kani::abi::SCALAR_HARNESS_LIMITS),
            true,
        ));
        let operands = operand_names
            .iter()
            .map(|name| format!("&{name}"))
            .collect::<Vec<_>>()
            .join(", ");
        body.push(let_binding(
            &format!("arithmetic_outcome_{index}"),
            Init::Expression(expression(format!(
                "rt::evaluate_integer_arithmetic(rt::IntegerArithmetic::{}({operands}), None, &mut arithmetic_meter_{index})",
                probe.operation
            ))?),
            false,
        ));
        body.push(Statement::Assert {
            condition: expression(format!(
                "arithmetic_outcome_{index} == rt::Outcome::Completed(arithmetic_expected_{index})"
            ))?,
            message: "arithmetic subterm agrees with native i128".to_owned(),
        });
    }
    body.extend([
        Statement::Assert {
            condition: expression("matches!(&outcome, Ok(rt::Outcome::Completed(_)))")?,
            message: "the clause oracle completes within the admitted parameter domain".to_owned(),
        },
        Statement::Assert {
            condition: expression(
                "matches!(&outcome, Ok(rt::Outcome::Completed(true))) == expected",
            )?,
            message: "the clause oracle agrees with independent native implication and arithmetic"
                .to_owned(),
        },
        Statement::Assert {
            condition: expression("subject == expected")?,
            message: "the compiled subject agrees with the independent clause expectation"
                .to_owned(),
        },
    ]);
    let path = harness_path(&lowered.module_symbol, &lowered.harness_symbol)?;
    let spec = HarnessSpec::new(
        format!(
            "V2 postcondition {}/{}",
            lowered.node_id.domain, lowered.node_id.digest
        ),
        path,
        lowered.oracle.source.clone(),
        ProofKind::Proof,
        body,
        vec![Cover {
            condition: expression("matches!(outcome, Ok(rt::Outcome::Completed(_)))")?,
            message: "the V2 clause condition completes for an admitted parameter".to_owned(),
        }],
        request.unwind,
        KaniSolver::Cadical,
    )?;
    let rust = render::render(&spec)?;
    let identity = V2ClauseObligationIdentity {
        ceilings: request.ceilings,
        clause: lowered.node_id.clone(),
        kind: lowered.kind,
        oracle_symbol: "oracle_clause".to_owned(),
        subject_path: request.subject_path.to_owned(),
        arguments: vec![ScalarObligationArgument {
            identifier: lowered.symbolic_name.clone(),
            minimum,
            maximum,
        }],
        module_symbol: spec.path().module.clone(),
        harness_symbol: spec.path().harness.clone(),
        solver: spec.solver(),
        unwind: spec.unwind(),
        options: spec.options().to_vec(),
    };
    let record = record(&lowered.module_symbol, &identity, &rust)?;
    Ok(KaniV2ClauseObligationHarness::generated(
        identity, rust, record,
    ))
}

//! Bounded cover harness for an admitted V2 parameter precondition (FR-015).

use std::fmt::Write as _;

use quire_contract_model::{CheckedNodeId, CheckedPackageV2, ContractSourceV1};
use quote::ToTokens;

use crate::{
    core::ir::clause::{read_clause, ClauseGraphView, ClauseTerm},
    kani::{
        abi::{KaniPrimitiveType, KaniSolver},
        census::{dependency_readiness, ProofDependencyEdge, ProofExecutionState},
        generate::{
            outcome::{node_label, KaniObligationRequest, UnsupportedObligation},
            record::{harness_path, record},
            render,
            spec::{
                Argument, Cover, ExprText, HarnessSpec, Init, LetBinding, NativeAdmissionStage,
                ProofKind, Statement,
            },
        },
        identity::{
            KaniV2ClauseObligationHarness, ObligationKind, ScalarObligationArgument,
            V2ClauseObligationIdentity,
        },
    },
    oracle::clause::{
        generate_v2_clause_native_oracle, generate_v2_clause_oracle_with_view, V2ClauseOracle,
    },
};

pub(in crate::kani::generate) struct LoweredV2Clause {
    dependencies: Vec<ProofDependencyEdge>,
    pub(super) package: ContractSourceV1,
    pub(super) node_id: CheckedNodeId,
    kind: ObligationKind,
    oracle: V2ClauseOracle,
    native_integer: bool,
    native_expectation: Option<String>,
    arithmetic: Vec<ArithmeticProbe>,
    symbolic_name: Option<String>,
    pub(super) module_symbol: String,
    pub(super) harness_symbol: String,
}

/// Encode the complete structural key for a generated message. Escaping `%` keeps
/// this reversible even when a source name itself contains an escape-looking span.
fn message_safe_node_label(node: &CheckedNodeId) -> String {
    escape_message_label(&node_label(node).to_string())
}

fn escape_message_label(raw: &str) -> String {
    let mut label = String::new();
    for character in raw.chars() {
        match character {
            '%' | '"' | '\\' | '{' | '}' | '\n' | '\r' => {
                write!(&mut label, "%{:02X}", u32::from(character))
                    .expect("writing to a String cannot fail");
            }
            other => label.push(other),
        }
    }
    label
}

pub(in crate::kani::generate) fn lower(
    view: &ClauseGraphView<'_>,
    clause: &CheckedNodeId,
    kind: ObligationKind,
) -> Result<LoweredV2Clause, UnsupportedObligation> {
    let package = view.package();
    let refusal = || UnsupportedObligation::V2ClauseConditionNotRendered {
        node_id: clause.clone(),
    };
    let oracle = generate_v2_clause_oracle_with_view(clause, view).map_err(|_| refusal())?;
    let [input] = oracle.descriptor.inputs.as_slice() else {
        return Err(refusal());
    };
    if input.name.strip_prefix("parameter_").is_none()
        || input.lower > input.upper
        || i64::try_from(input.lower).is_err()
        || i64::try_from(input.upper).is_err()
    {
        return Err(refusal());
    }
    let (native_expectation, arithmetic, symbolic_name) = if kind == ObligationKind::Postcondition {
        let input_name = input.name.strip_prefix("parameter_").ok_or_else(refusal)?;
        let symbolic_name = format!("{input_name}_current");
        let condition = read_clause(clause, view).ok_or_else(refusal)?;
        let mut arithmetic = Vec::new();
        let expectation = native_term(&condition.term, input_name, &symbolic_name, &mut arithmetic)
            .and_then(NativeTerm::boolean)
            .ok_or_else(refusal)?;
        (Some(expectation), arithmetic, Some(symbolic_name))
    } else {
        (None, Vec::new(), None)
    };
    // Kani includes the module in generated filenames. Use the full admitted pair so two
    // revisions of one declaration cannot collide on a readable clause name.
    let package_identity = ContractSourceV1 {
        name: package.name().into(),
        lock: package.lock().clone(),
    };
    let node_key = node_label(clause).to_string();
    let preimage = crate::core::canonical::preimage_value(&(&package_identity, &node_key))
        .map_err(|_| UnsupportedObligation::RenderFailed)?;
    let digest = crate::core::canonical::content_digest(&preimage)
        .map_err(|_| UnsupportedObligation::RenderFailed)?;
    let module_symbol = format!(
        "{}_{}",
        if kind == ObligationKind::Postcondition {
            "kob_v2_clause"
        } else {
            "kob_v2_pre"
        },
        digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    );
    Ok(LoweredV2Clause {
        dependencies: Vec::new(),
        package: package_identity,
        node_id: clause.clone(),
        kind,
        oracle,
        native_integer: false,
        native_expectation,
        arithmetic,
        symbolic_name,
        module_symbol,
        harness_symbol: if kind == ObligationKind::Postcondition {
            "prove_clause"
        } else {
            "cover_precondition"
        }
        .to_owned(),
    })
}

impl LoweredV2Clause {
    pub(in crate::kani::generate) fn set_dependencies(
        &mut self,
        dependencies: Vec<ProofDependencyEdge>,
    ) {
        self.dependencies = dependencies;
    }

    pub(in crate::kani::generate) fn select_native_integer(
        &mut self,
        package: &CheckedPackageV2,
    ) -> Result<(), UnsupportedObligation> {
        let oracle = generate_v2_clause_native_oracle(package, &self.node_id)
            .map_err(|refusal| UnsupportedObligation::NativeV2ClauseCapability { refusal })?;
        let selection = oracle
            .native_integer_selection
            .as_ref()
            .ok_or(UnsupportedObligation::RenderFailed)?;
        bind_native_arithmetic_probes(&mut self.arithmetic, selection)?;
        self.oracle = oracle;
        self.native_integer = true;
        Ok(())
    }
}

pub(in crate::kani::generate) fn render_clause(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredV2Clause,
) -> Result<KaniV2ClauseObligationHarness, UnsupportedObligation> {
    if lowered.kind == ObligationKind::Postcondition {
        return render_postcondition(request, lowered);
    }
    let [input] = lowered.oracle.descriptor.inputs.as_slice() else {
        return Err(UnsupportedObligation::V2ClauseConditionNotRendered {
            node_id: lowered.node_id.clone(),
        });
    };
    let minimum = i64::try_from(input.lower).map_err(|_| UnsupportedObligation::RenderFailed)?;
    let maximum = i64::try_from(input.upper).map_err(|_| UnsupportedObligation::RenderFailed)?;
    let body = vec![
        Statement::Input {
            name: input.name.clone(),
            ty: KaniPrimitiveType::I64,
            domain: (input.lower, input.upper),
            then: vec![LetBinding {
                name: input.name.clone(),
                ty: None,
                mutable: false,
                init: Init::Expression(ExprText::new(if lowered.native_integer {
                    format!("nx::NativeInteger::new(i128::from({}))", input.name)
                } else {
                    format!("rt::Integer::from({})", input.name)
                })?),
            }],
        },
        Statement::Let(LetBinding {
            name: "meter".to_owned(),
            ty: None,
            mutable: true,
            init: Init::ScalarMeter(crate::kani::abi::SCALAR_HARNESS_LIMITS),
        }),
        Statement::Call {
            result: "outcome".to_owned(),
            callee: "oracle_clause".to_owned(),
            args: vec![
                Argument::Borrow(input.name.clone()),
                Argument::BorrowMut("meter".to_owned()),
            ],
        },
    ];
    let path = harness_path(&lowered.module_symbol, &lowered.harness_symbol)?;
    let spec = HarnessSpec::new(
        format!(
            "V2 precondition {}",
            message_safe_node_label(&lowered.node_id)
        ),
        path,
        lowered.oracle.source.clone(),
        ProofKind::Proof,
        body,
        vec![Cover {
            condition: ExprText::new("matches!(outcome, Ok(rt::Outcome::Completed(true)))")?,
            message: "the V2 precondition is satisfiable within admitted parameter bounds"
                .to_owned(),
        }],
        request.unwind,
        KaniSolver::Cadical,
    )?;
    let rust = render::render(&spec)?;
    let identity = V2ClauseObligationIdentity {
        dependencies: lowered.dependencies.clone(),
        readiness: dependency_readiness(&lowered.dependencies),
        proof_execution_state: ProofExecutionState::NotRun,
        integer_representation: if lowered.native_integer {
            crate::kani::identity::ScalarIntegerRepresentation::Native
        } else {
            crate::kani::identity::ScalarIntegerRepresentation::General
        },
        native_integer_selection: lowered.oracle.native_integer_selection.clone(),
        ceilings: request.ceilings,
        package: lowered.package.clone(),
        clause: lowered.node_id.clone(),
        collection_precondition: None,
        kind: ObligationKind::Precondition,
        oracle_symbol: "oracle_clause".to_owned(),
        subject_path: request.subject_path.to_owned(),
        arguments: vec![ScalarObligationArgument {
            identifier: input.name.clone(),
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

fn expression(text: impl Into<String>) -> Result<ExprText, UnsupportedObligation> {
    ExprText::new(text).map_err(Into::into)
}

fn binding(name: &str, init: Init, mutable: bool) -> Statement {
    Statement::Let(LetBinding {
        name: name.to_owned(),
        ty: None,
        mutable,
        init,
    })
}

fn native_expected_expression(text: &str) -> Result<String, UnsupportedObligation> {
    let mut expected =
        syn::parse_str::<syn::Expr>(text).map_err(|_| UnsupportedObligation::RenderFailed)?;
    while let syn::Expr::Paren(parenthesized) = expected {
        expected = *parenthesized.expr;
    }
    Ok(expected.into_token_stream().to_string())
}

fn i128_literal(value: i128) -> String {
    if value == i128::MIN {
        "i128::MIN".to_owned()
    } else {
        format!("{value}_i128")
    }
}

fn render_postcondition(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredV2Clause,
) -> Result<KaniV2ClauseObligationHarness, UnsupportedObligation> {
    let [input] = lowered.oracle.descriptor.inputs.as_slice() else {
        return Err(UnsupportedObligation::RenderFailed);
    };
    let symbolic = lowered
        .symbolic_name
        .as_deref()
        .ok_or(UnsupportedObligation::RenderFailed)?;
    let expectation = lowered
        .native_expectation
        .as_deref()
        .ok_or(UnsupportedObligation::RenderFailed)?;
    let minimum = i64::try_from(input.lower).map_err(|_| UnsupportedObligation::RenderFailed)?;
    let maximum = i64::try_from(input.upper).map_err(|_| UnsupportedObligation::RenderFailed)?;
    let mut body = vec![
        Statement::Input {
            name: symbolic.to_owned(),
            ty: KaniPrimitiveType::I64,
            domain: (input.lower, input.upper),
            then: vec![LetBinding {
                name: input.name.clone(),
                ty: None,
                mutable: false,
                init: Init::Expression(expression(format!(
                    "nx::NativeInteger::new(i128::from({symbolic}))"
                ))?),
            }],
        },
        binding(
            "expected",
            Init::Expression(expression(native_expected_expression(expectation)?)?),
            false,
        ),
        binding(
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
        binding(
            "subject",
            Init::Expression(expression(format!("{}({symbolic})", request.subject_path))?),
            false,
        ),
    ];
    let mut completed_prefix = "true".to_owned();
    for (index, probe) in lowered.arithmetic.iter().enumerate() {
        let guard = format!("({}) && ({completed_prefix})", probe.guard);
        let mut probe_body = Vec::new();
        let expected = native_expected_expression(&probe.expected)?;
        let in_domain = probe.semantic_domain.map_or_else(
            || "true".to_owned(),
            |(lower, upper)| {
                format!(
                    "({expected} >= {} && {expected} <= {})",
                    i128_literal(lower),
                    i128_literal(upper)
                )
            },
        );

        let mut operands = Vec::new();
        let mut intervals = Vec::new();
        for (position, (operand, (lower, upper))) in
            probe.operands.iter().zip(&probe.operand_bounds).enumerate()
        {
            let value = format!("arithmetic_operand_{index}_{position}");
            probe_body.push(binding(
                &value,
                Init::Expression(expression(format!(
                    "nx::NativeInteger::new(i128::from({operand}))"
                ))?),
                false,
            ));
            operands.push(value);
            let interval = format!("arithmetic_interval_{index}_{position}");
            probe_body.push(binding(
                &interval,
                Init::NativeAdmission {
                    checked: expression(format!(
                        "nx::NativeIntegerInterval::new({}, {})",
                        i128_literal(*lower),
                        i128_literal(*upper)
                    ))?,
                    stage: NativeAdmissionStage::OperandInterval,
                },
                false,
            ));
            intervals.push(interval);
        }
        let (descriptor, payload) = if probe.operation == "Negate" {
            let ([operand], [interval]) = (operands.as_slice(), intervals.as_slice()) else {
                return Err(UnsupportedObligation::RenderFailed);
            };
            (
                format!("nx::NativeIntegerOperation::Negate {{ operand: {interval} }}"),
                format!("nx::NativeIntegerOperands::Unary {{ operand: {operand} }}"),
            )
        } else {
            let ([left, right], [left_interval, right_interval]) =
                (operands.as_slice(), intervals.as_slice())
            else {
                return Err(UnsupportedObligation::RenderFailed);
            };
            (
                format!(
                    "nx::NativeIntegerOperation::{} {{ left: {left_interval}, right: {right_interval} }}",
                    probe.operation
                ),
                format!("nx::NativeIntegerOperands::Binary {{ left: {left}, right: {right} }}"),
            )
        };
        let domain = probe.semantic_domain.map_or_else(
            || "None".to_owned(),
            |(lower, upper)| {
                format!(
                    "Some(nx::NativeIntegerInterval::new({}, {}).expect(\"selected semantic domain\"))",
                    i128_literal(lower),
                    i128_literal(upper)
                )
            },
        );
        let plan = format!("arithmetic_plan_{index}");
        probe_body.push(binding(
            &plan,
            Init::NativeAdmission {
                checked: expression(format!(
                    "nx::NativeIntegerPlan::new({descriptor}, {domain})"
                ))?,
                stage: NativeAdmissionStage::PlanCapacity,
            },
            false,
        ));
        let admitted = format!("arithmetic_admitted_{index}");
        probe_body.push(binding(
            &admitted,
            Init::NativeAdmission {
                checked: expression(format!("{plan}.admit({payload})"))?,
                stage: NativeAdmissionStage::OperandPopulation,
            },
            false,
        ));
        let meter = format!("arithmetic_meter_{index}");
        probe_body.push(binding(
            &meter,
            Init::ScalarMeter(crate::kani::abi::SCALAR_HARNESS_LIMITS),
            true,
        ));
        let outcome = format!("arithmetic_outcome_{index}");
        probe_body.push(binding(
            &outcome,
            Init::Expression(expression(format!(
                "nx::evaluate_native_integer_arithmetic({admitted}, &mut {meter})"
            ))?),
            false,
        ));
        probe_body.push(Statement::Assert {
            condition: expression(format!(
                "match &{outcome} {{ rt::Outcome::Completed(value) => ({in_domain}) && value.value() == {expected}, rt::Outcome::Refused(rt::Refusal::IntegerOutOfDomain {{ .. }}) => !({in_domain}), _ => false }}"
            ))?,
            message: "arithmetic subterm agrees with independent i128 expectation".to_owned(),
        });
        body.push(Statement::ConditionalProbe {
            condition: expression(guard)?,
            body: probe_body,
        });
        completed_prefix = format!(
            "({completed_prefix}) && (!({}) || ({in_domain}))",
            probe.guard
        );
    }
    body.extend([
        Statement::Assert {
            condition: expression(format!(
                "(({completed_prefix}) && matches!(&outcome, Ok(rt::Outcome::Completed(_)))) || (!({completed_prefix}) && matches!(&outcome, Ok(rt::Outcome::Refused(rt::Refusal::IntegerOutOfDomain {{ .. }}))))"
            ))?,
            message: "the clause oracle preserves completed and refused semantic domains".to_owned(),
        },
        Statement::Assert {
            condition: expression(format!(
                "!({completed_prefix}) || (matches!(&outcome, Ok(rt::Outcome::Completed(true))) == expected)"
            ))?,
            message: "the clause oracle agrees with the independent checked condition".to_owned(),
        },
        Statement::Assert {
            condition: expression(format!("!({completed_prefix}) || subject == expected"))?,
            message: "the compiled subject agrees with the checked condition".to_owned(),
        },
    ]);
    let spec = HarnessSpec::new(
        format!(
            "V2 postcondition {}",
            message_safe_node_label(&lowered.node_id)
        ),
        harness_path(&lowered.module_symbol, &lowered.harness_symbol)?,
        lowered.oracle.source.clone(),
        ProofKind::Proof,
        body,
        vec![Cover {
            condition: expression(format!(
                "({completed_prefix}) && matches!(outcome, Ok(rt::Outcome::Completed(_)))"
            ))?,
            message: "the original V2 condition completes on an admitted parameter".to_owned(),
        }],
        request.unwind,
        KaniSolver::Cadical,
    )?;
    let rust = render::render(&spec)?;
    let identity = V2ClauseObligationIdentity {
        dependencies: lowered.dependencies.clone(),
        readiness: dependency_readiness(&lowered.dependencies),
        proof_execution_state: ProofExecutionState::NotRun,
        integer_representation: crate::kani::identity::ScalarIntegerRepresentation::Native,
        native_integer_selection: lowered.oracle.native_integer_selection.clone(),
        ceilings: request.ceilings,
        package: lowered.package.clone(),
        clause: lowered.node_id.clone(),
        collection_precondition: None,
        kind: ObligationKind::Postcondition,
        oracle_symbol: "oracle_clause".to_owned(),
        subject_path: request.subject_path.to_owned(),
        arguments: vec![ScalarObligationArgument {
            identifier: symbolic.to_owned(),
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
enum NativeTerm {
    Integer {
        expression: String,
        lower: i128,
        upper: i128,
        small_expression: Option<String>,
    },
    Boolean(String),
}

struct ArithmeticProbe {
    operation_identity: String,
    operation: &'static str,
    operands: Vec<String>,
    expected: String,
    operand_bounds: Vec<(i128, i128)>,
    semantic_domain: Option<(i128, i128)>,
    guard: String,
}

fn bind_native_arithmetic_probes(
    probes: &mut [ArithmeticProbe],
    selection: &crate::core::native_integer::NativeIntegerSelection,
) -> Result<(), UnsupportedObligation> {
    let decisions = selection
        .operations
        .iter()
        .filter(|decision| decision.capacity.is_some())
        .collect::<Vec<_>>();
    if decisions.len() != probes.len() {
        return Err(UnsupportedObligation::RenderFailed);
    }
    for (probe, decision) in probes.iter_mut().zip(decisions) {
        if probe.operation_identity != decision.operation_identity {
            return Err(UnsupportedObligation::RenderFailed);
        }
        probe.operand_bounds = decision
            .operands
            .iter()
            .map(|interval| {
                Ok::<_, UnsupportedObligation>((
                    interval
                        .lower
                        .parse()
                        .map_err(|_| UnsupportedObligation::RenderFailed)?,
                    interval
                        .upper
                        .parse()
                        .map_err(|_| UnsupportedObligation::RenderFailed)?,
                ))
            })
            .collect::<Result<Vec<_>, UnsupportedObligation>>()?;
        probe.semantic_domain = decision
            .semantic_domain
            .as_ref()
            .map(|interval| {
                Ok::<_, UnsupportedObligation>((
                    interval
                        .lower
                        .parse()
                        .map_err(|_| UnsupportedObligation::RenderFailed)?,
                    interval
                        .upper
                        .parse()
                        .map_err(|_| UnsupportedObligation::RenderFailed)?,
                ))
            })
            .transpose()?;
    }
    Ok(())
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
                expression: i128_literal(parsed),
                lower: parsed,
                upper: parsed,
                small_expression: i64::try_from(parsed)
                    .ok()
                    .map(|value| format!("{value}i64")),
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
                small_expression: Some(symbolic_name.to_owned()),
            })
        }
        ClauseTerm::Input(_) => None,
        ClauseTerm::Operation {
            identity,
            arguments,
            ..
        } => {
            if let [left, right] = arguments.as_slice() {
                let gate = match identity.as_str() {
                    "quire.op.boolean.and" | "quire.op.boolean.implies" => Some(false),
                    "quire.op.boolean.or" => Some(true),
                    _ => None,
                };
                if let Some(invert) = gate {
                    let left = native_term(left, input_name, symbolic_name, arithmetic)?
                        .clone_boolean()?;
                    let first_right = arithmetic.len();
                    let right = native_term(right, input_name, symbolic_name, arithmetic)?
                        .clone_boolean()?;
                    let condition = if invert {
                        format!("!({left})")
                    } else {
                        left.clone()
                    };
                    for probe in &mut arithmetic[first_right..] {
                        probe.guard = format!("({}) && ({condition})", probe.guard);
                    }
                    let expression = match identity.as_str() {
                        "quire.op.boolean.and" => format!("({left} && {right})"),
                        "quire.op.boolean.or" => format!("({left} || {right})"),
                        "quire.op.boolean.implies" => format!("(!{left} || {right})"),
                        _ => return None,
                    };
                    return Some(NativeTerm::Boolean(expression));
                }
            }
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
                    let (expression, lower, upper, small) = value.clone_integer()?;
                    let operand_bounds = vec![(lower, upper)];
                    let expected = format!("(-{expression})");
                    let (lower, upper) = (upper.checked_neg()?, lower.checked_neg()?);
                    let small_expression =
                        if i64::try_from(lower).is_ok() && i64::try_from(upper).is_ok() {
                            small.as_ref().map(|value| format!("(-{value})"))
                        } else {
                            None
                        };
                    let operand = if small_expression.is_some() {
                        small?
                    } else {
                        expression
                    };
                    arithmetic.push(ArithmeticProbe {
                        operation_identity: identity.clone(),
                        operation: "Negate",
                        operand_bounds,
                        operands: vec![operand],
                        expected: expected.clone(),
                        semantic_domain: None,
                        guard: "true".to_owned(),
                    });
                    Some(NativeTerm::Integer {
                        expression: expected,
                        lower,
                        upper,
                        small_expression,
                    })
                }
                (
                    "quire.op.integer.add" | "quire.op.integer.sub" | "quire.op.integer.mul",
                    [left, right],
                ) => {
                    let (a, al, ah, a_small) = left.clone_integer()?;
                    let (b, bl, bh, b_small) = right.clone_integer()?;
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
                    let small_expression =
                        if i64::try_from(bounds.0).is_ok() && i64::try_from(bounds.1).is_ok() {
                            a_small
                                .as_ref()
                                .zip(b_small.as_ref())
                                .map(|(a, b)| format!("({a} {operator} {b})"))
                        } else {
                            None
                        };
                    let operands = if small_expression.is_some() {
                        vec![a_small?, b_small?]
                    } else {
                        vec![a, b]
                    };
                    arithmetic.push(ArithmeticProbe {
                        operation_identity: identity.clone(),
                        operation,
                        operand_bounds: vec![(al, ah), (bl, bh)],
                        operands,
                        expected: expected.clone(),
                        semantic_domain: None,
                        guard: "true".to_owned(),
                    });
                    Some(NativeTerm::Integer {
                        expression: expected,
                        lower: bounds.0,
                        upper: bounds.1,
                        small_expression,
                    })
                }
                (
                    "quire.op.integer.lt"
                    | "quire.op.integer.le"
                    | "quire.op.integer.gt"
                    | "quire.op.integer.ge",
                    [left, right],
                ) => {
                    let (a, _, _, _) = left.clone_integer()?;
                    let (b, _, _, _) = right.clone_integer()?;
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
                    let (a, _, _, _) = left.clone_integer()?;
                    let (b, _, _, _) = right.clone_integer()?;
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
    fn clone_integer(&self) -> Option<(String, i128, i128, Option<String>)> {
        match self {
            Self::Integer {
                expression,
                lower,
                upper,
                small_expression,
            } => Some((expression.clone(), *lower, *upper, small_expression.clone())),
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

#[cfg(test)]
mod native_probe_tests {
    use super::{bind_native_arithmetic_probes, escape_message_label, ArithmeticProbe};
    use crate::core::native_integer::{
        NativeIntegerIntervalDecision as Interval, NativeIntegerOperationDecision as Decision,
        NativeIntegerSelection,
    };
    use ix_trace_rs::trace;

    #[test]
    fn structural_message_label_escapes_delimiters_and_percent_injectively() {
        let label = escape_message_label("p%7B{\"q\"}\\\n\r");
        assert_eq!(label, "p%257B%7B%22q%22%7D%5C%0A%0D");
        assert_ne!(escape_message_label("%7B"), escape_message_label("{"));
        assert!(!label.contains(['"', '\\', '{', '}', '\n', '\r']));
    }

    #[trace("FR-015-AC-125", "TC-025")]
    #[test]
    fn bounded_add_feeding_multiply_uses_the_typed_completed_interval() {
        let node_id: quire_contract_model::CheckedNodeId =
            serde_json::from_value(serde_json::json!({
                "package": "kani-clause-v2-test",
                "path": ["bounded-add"],
                "ordinal": 0,
            }))
            .expect("checked clause key");
        let interval = |lower: i128, upper: i128| Interval {
            lower: lower.to_string(),
            upper: upper.to_string(),
        };
        let mut selection = NativeIntegerSelection::new();
        selection.operations = vec![
            Decision {
                node_id: node_id.clone(),
                step: 0,
                operation_identity: "quire.op.integer.add".to_owned(),
                operands: vec![interval(0, 1000), interval(1, 1)],
                capacity: Some(interval(1, 1001)),
                semantic_domain: Some(interval(0, 1000)),
            },
            Decision {
                node_id,
                step: 1,
                operation_identity: "quire.op.integer.mul".to_owned(),
                operands: vec![interval(1, 1000), interval(2, 2)],
                capacity: Some(interval(2, 2000)),
                semantic_domain: None,
            },
        ];
        let mut probes = vec![
            ArithmeticProbe {
                operation_identity: "quire.op.integer.add".to_owned(),
                operation: "Add",
                operands: vec!["amount".to_owned(), "1".to_owned()],
                expected: "amount + 1".to_owned(),
                operand_bounds: vec![(0, 1000), (1, 1)],
                semantic_domain: None,
                guard: "true".to_owned(),
            },
            ArithmeticProbe {
                operation_identity: "quire.op.integer.mul".to_owned(),
                operation: "Multiply",
                operands: vec!["amount + 1".to_owned(), "2".to_owned()],
                expected: "(amount + 1) * 2".to_owned(),
                operand_bounds: vec![(1, 1001), (2, 2)],
                semantic_domain: None,
                guard: "true".to_owned(),
            },
        ];
        bind_native_arithmetic_probes(&mut probes, &selection).expect("admitted typed plan");
        assert_eq!(probes[0].semantic_domain, Some((0, 1000)));
        assert_eq!(probes[1].operand_bounds, [(1, 1000), (2, 2)]);
    }
}

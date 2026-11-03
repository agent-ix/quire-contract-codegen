//! Bounded harnesses for admitted V2 scalar clauses (FR-015).

#[cfg(test)]
#[allow(dead_code, clippy::duplicate_mod)]
#[path = "../../../../tests/state_frame_support/model.rs"]
mod model;
#[cfg(test)]
#[allow(dead_code, clippy::duplicate_mod)]
#[path = "../../../../tests/state_frame_support/native_twin.rs"]
mod native_twin;

use quire_contract_model::{
    ApplicationOperator, CheckedMemberType, CheckedModelOperations, CheckedNodeId, CheckedNodeKind,
    CheckedOperationParameter, CheckedOperationReadError, CheckedPackageV2, CheckedTerm,
    CompositeTypeForm, ContractSourceV1, LiteralKind, ScalarTypeForm,
};
use quote::ToTokens;
use std::collections::BTreeSet;
use std::fmt::Write as _;

use crate::{
    core::ir::{
        checked_term::{application, bound_members, literal, reference},
        clause::{read_clause, ClauseGraphView, ClauseInputSide, ClauseTerm, MAX_CONDITION_TERMS},
    },
    kani::{
        abi::{KaniPrimitiveType, KaniSolver},
        census::{dependency_readiness, ProofDependencyEdge, ProofExecutionState},
        generate::{
            outcome::{node_label, KaniObligationRequest, UnsupportedObligation},
            record::{harness_path, record},
            render,
            spec::{
                Argument, Cover, ExprText, HarnessSpec, Init, LetBinding, LetType,
                NativeAdmissionStage, ProofKind, Statement,
            },
        },
        identity::{
            KaniV2ClauseObligationHarness, ObligationKind, ScalarObligationArgument,
            V2ClauseObligationIdentity, V2ClauseStateField,
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
    result_postcondition: Option<ResultPostcondition>,
    pub(super) module_symbol: String,
    pub(super) harness_symbol: String,
}

#[derive(Clone)]
enum MachineScalar {
    Boolean,
    Integer { minimum: i64, maximum: i64 },
}

struct ResultParameter {
    name: String,
    scalar: MachineScalar,
}

struct ResultPostcondition {
    result: Option<MachineScalar>,
    parameters: Vec<ResultParameter>,
}

/// Encode the complete structural key for a generated message. Escaping `%` keeps
/// this reversible even when a source name itself contains an escape-looking span.
pub(in crate::kani::generate) fn message_safe_node_label(node: &CheckedNodeId) -> String {
    escape_message_label(&node_label(node).to_string())
}

fn escape_message_label(raw: &str) -> String {
    let mut label = String::new();
    for character in raw.chars() {
        match character {
            '%' | '"' | '\\' | '{' | '}' | '\n' | '\r' => {
                let _ = write!(&mut label, "%{:02X}", u32::from(character));
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
    lower_with_census(view, clause, kind, |package, context| {
        package.model_operations(context, 65_536)
    })
}

fn lower_with_census<'a>(
    view: &ClauseGraphView<'a>,
    clause: &CheckedNodeId,
    kind: ObligationKind,
    census: impl FnOnce(
        &'a CheckedPackageV2,
        &CheckedNodeId,
    ) -> Result<CheckedModelOperations<'a>, CheckedOperationReadError>,
) -> Result<LoweredV2Clause, UnsupportedObligation> {
    let package = view.package();
    let refusal = || UnsupportedObligation::V2ClauseConditionNotRendered {
        node_id: clause.clone(),
    };
    if let Some(operation_identity) = unsupported_checked_clause_operator(view, clause) {
        return Err(UnsupportedObligation::V2ClauseOperatorNotSupported {
            node_id: clause.clone(),
            operation_identity,
        });
    }
    let condition = read_clause(clause, view).ok_or_else(refusal)?;
    if let Some(operation_identity) = unsupported_clause_operator(&condition.term) {
        return Err(UnsupportedObligation::V2ClauseOperatorNotSupported {
            node_id: clause.clone(),
            operation_identity: operation_identity.to_owned(),
        });
    }
    let result_postcondition = if kind == ObligationKind::Postcondition {
        Some(read_postcondition_signature(view, clause, census)?)
    } else {
        None
    };
    let oracle = generate_v2_clause_oracle_with_view(clause, view).map_err(|_| refusal())?;
    let input_count_supported = match kind {
        ObligationKind::Precondition => true,
        ObligationKind::Postcondition => {
            result_postcondition
                .as_ref()
                .is_some_and(|signature| signature.result.is_some())
                || oracle.descriptor.inputs.len() == 1
        }
        ObligationKind::Invariant => true,
        ObligationKind::Frame => false,
    };
    if !input_count_supported
        || oracle.descriptor.inputs.iter().any(|input| {
            let supported = match kind {
                ObligationKind::Precondition => {
                    input.name.starts_with("parameter_") && input.state_field.is_none()
                        || input.name.starts_with("current_") && input.state_field.is_some()
                }
                ObligationKind::Postcondition => {
                    input.state_field.is_none()
                        && (input.name.starts_with("parameter_")
                            || result_postcondition
                                .as_ref()
                                .is_some_and(|signature| signature.result.is_some())
                                && input.name == "result_result")
                }
                ObligationKind::Invariant => {
                    input.name.starts_with("current_") && input.state_field.is_some()
                }
                ObligationKind::Frame => false,
            };
            !supported
                || (input.boolean
                    && result_postcondition
                        .as_ref()
                        .is_some_and(|signature| signature.result.is_none()))
                || input.lower > input.upper
                || i64::try_from(input.lower).is_err()
                || i64::try_from(input.upper).is_err()
        })
    {
        return Err(refusal());
    }
    // Arithmetic clauses require independent subterm probes before they can be asserted.
    if kind == ObligationKind::Invariant
        && oracle
            .descriptor
            .operation_identities
            .iter()
            .any(|identity| {
                matches!(
                    identity.as_str(),
                    "quire.op.integer.add"
                        | "quire.op.integer.sub"
                        | "quire.op.integer.mul"
                        | "quire.op.integer.negate"
                )
            })
    {
        return Err(refusal());
    }
    let (native_expectation, arithmetic, symbolic_name) = if kind == ObligationKind::Postcondition
        && result_postcondition
            .as_ref()
            .is_some_and(|signature| signature.result.is_none())
    {
        let signature = result_postcondition.as_ref().ok_or_else(refusal)?;
        let [parameter] = signature.parameters.as_slice() else {
            return Err(refusal());
        };
        let input = oracle.descriptor.inputs.first().ok_or_else(refusal)?;
        let input_name = input.name.strip_prefix("parameter_").ok_or_else(refusal)?;
        if input_name != parameter.name
            || !matches!(
                parameter.scalar,
                MachineScalar::Integer { minimum, maximum }
                    if input.lower == i128::from(minimum) && input.upper == i128::from(maximum)
            )
        {
            return Err(refusal());
        }
        let symbolic_name = format!("{input_name}_current");
        let mut arithmetic = Vec::new();
        let expectation = native_term(&condition.term, input_name, &symbolic_name, &mut arithmetic)
            .and_then(NativeTerm::boolean)
            .ok_or_else(refusal)?;
        (Some(expectation), arithmetic, Some(symbolic_name))
    } else if kind == ObligationKind::Postcondition
        && result_postcondition
            .as_ref()
            .is_some_and(|signature| matches!(signature.result, Some(MachineScalar::Boolean)))
    {
        let signature = result_postcondition.as_ref().ok_or_else(refusal)?;
        if let [parameter] = signature.parameters.as_slice() {
            let symbolic_name = format!("parameter_{}_current", parameter.name);
            let mut arithmetic = Vec::new();
            let expectation = match (&parameter.scalar, result_expectation_term(&condition.term)) {
                (MachineScalar::Integer { .. }, Some(term)) => {
                    native_term(term, &parameter.name, &symbolic_name, &mut arithmetic)
                        .and_then(NativeTerm::boolean)
                }
                _ => None,
            };
            (expectation, arithmetic, Some(symbolic_name))
        } else {
            (None, Vec::new(), None)
        }
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
        } else if kind == ObligationKind::Invariant {
            "kob_v2_inv"
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
        result_postcondition,
        module_symbol,
        harness_symbol: if kind == ObligationKind::Postcondition {
            "prove_clause"
        } else if kind == ObligationKind::Invariant {
            "assert_invariant"
        } else {
            "cover_precondition"
        }
        .to_owned(),
    })
}

fn machine_scalar(member: &CheckedMemberType) -> Option<MachineScalar> {
    match member {
        CheckedMemberType::Boolean => Some(MachineScalar::Boolean),
        CheckedMemberType::IntRange { lower, upper } if lower <= upper => {
            Some(MachineScalar::Integer {
                minimum: i64::try_from(*lower).ok()?,
                maximum: i64::try_from(*upper).ok()?,
            })
        }
        _ => None,
    }
}

fn binder_matches_scalar(
    view: &ClauseGraphView<'_>,
    binder: &CheckedNodeId,
    scalar: &MachineScalar,
) -> bool {
    let Some(node) = view.node(binder) else {
        return false;
    };
    match scalar {
        MachineScalar::Boolean => {
            view.kind(&node.semantic_type)
                == Some(CheckedNodeKind::ScalarType(ScalarTypeForm::Boolean))
        }
        MachineScalar::Integer { minimum, maximum } => {
            let Some(CheckedTerm::Aggregate { members }) = view.package().term(&node.semantic_type)
            else {
                return false;
            };
            let number = |name| {
                members.iter().find_map(|entry| match entry {
                    CheckedTerm::Binding {
                        name: actual,
                        value,
                    } if actual.as_ref() == name => {
                        literal(value, LiteralKind::Integer)?.parse::<i64>().ok()
                    }
                    _ => None,
                })
            };
            view.kind(&node.semantic_type)
                == Some(CheckedNodeKind::BoundedDomain(
                    quire_contract_model::BoundedDomainForm::IntegerRange,
                ))
                && number("min") == Some(*minimum)
                && number("max") == Some(*maximum)
        }
    }
}

fn self_binder_matches_context(
    view: &ClauseGraphView<'_>,
    binder: &CheckedNodeId,
    context: &CheckedNodeId,
) -> bool {
    let Some(node) = view.node(binder) else {
        return false;
    };
    view.kind(&node.semantic_type)
        == Some(CheckedNodeKind::CompositeType(CompositeTypeForm::Reference))
        && matches!(
            view.package().term(&node.semantic_type),
            Some(CheckedTerm::Aggregate { members })
                if matches!(members.as_slice(), [CheckedTerm::Reference { target }] if target == context)
        )
}

fn read_postcondition_signature<'a>(
    view: &ClauseGraphView<'a>,
    clause: &CheckedNodeId,
    census_read: impl FnOnce(
        &'a CheckedPackageV2,
        &CheckedNodeId,
    ) -> Result<CheckedModelOperations<'a>, CheckedOperationReadError>,
) -> Result<ResultPostcondition, UnsupportedObligation> {
    let mismatch = || UnsupportedObligation::V2ClauseSignatureMismatch {
        node_id: clause.clone(),
    };
    let unsupported = || UnsupportedObligation::V2ClauseResultDomainUnsupported {
        node_id: clause.clone(),
    };
    let package = view.package();
    let [binders, anchor, _] = application(
        package.term(clause).ok_or_else(mismatch)?,
        ApplicationOperator::StateClause,
        "quire.op.state.clause",
    )
    .ok_or_else(mismatch)?
    else {
        return Err(mismatch());
    };
    let CheckedTerm::Aggregate { members } = binders else {
        return Err(mismatch());
    };
    let anchor = reference(anchor).ok_or_else(mismatch)?;
    let Some(CheckedTerm::OperationAnchorBody(body)) = package.term(anchor) else {
        return Err(mismatch());
    };
    let [context, operation_name, _] =
        bound_members(&body.members, ["context", "operation", "frame"]).ok_or_else(mismatch)?;
    let context = reference(context).ok_or_else(mismatch)?;
    let operation_name = literal(operation_name, LiteralKind::Text).ok_or_else(mismatch)?;
    let census = census_read(package, context).map_err(|source| {
        UnsupportedObligation::V2ClauseOperationRead {
            node_id: clause.clone(),
            source,
        }
    })?;
    let mut matches = census
        .operations()
        .iter()
        .filter(|operation| operation.name() == operation_name);
    let operation = matches.next().ok_or_else(mismatch)?;
    if matches.next().is_some() {
        return Err(mismatch());
    }
    let result = operation
        .result()
        .map(|member| machine_scalar(member.ok_or_else(unsupported)?).ok_or_else(unsupported))
        .transpose()?;
    validate_signature_binders(
        view,
        clause,
        context,
        members,
        operation.parameters(),
        result,
    )
}

fn validate_signature_binders(
    view: &ClauseGraphView<'_>,
    clause: &CheckedNodeId,
    context: &CheckedNodeId,
    members: &[CheckedTerm],
    signature_parameters: &[CheckedOperationParameter<'_>],
    result: Option<MachineScalar>,
) -> Result<ResultPostcondition, UnsupportedObligation> {
    let mismatch = || UnsupportedObligation::V2ClauseSignatureMismatch {
        node_id: clause.clone(),
    };
    let package = view.package();
    let expected = signature_parameters
        .len()
        .checked_add(if result.is_some() { 2 } else { 1 })
        .ok_or_else(mismatch)?;
    if members.len() != expected {
        return Err(mismatch());
    }
    let mut ids = members.iter().map(reference);
    let self_id = ids.next().flatten().ok_or_else(mismatch)?;
    if package
        .parameter_declaration(self_id)
        .map_err(|_| mismatch())?
        .name
        .as_ref()
        != "self"
        || !self_binder_matches_context(view, self_id, context)
    {
        return Err(mismatch());
    }
    if let Some(result) = &result {
        let result_id = ids.next().flatten().ok_or_else(mismatch)?;
        if package
            .parameter_declaration(result_id)
            .map_err(|_| mismatch())?
            .name
            .as_ref()
            != "result"
            || !binder_matches_scalar(view, result_id, result)
        {
            return Err(mismatch());
        }
    }
    let mut parameters = Vec::new();
    let mut parameter_names = BTreeSet::new();
    for (id, parameter) in ids.zip(signature_parameters) {
        let id = id.ok_or_else(mismatch)?;
        if matches!(parameter.name(), "self" | "result")
            || !parameter_names.insert(parameter.name())
        {
            return Err(mismatch());
        }
        let scalar =
            machine_scalar(parameter.member_type().ok_or_else(mismatch)?).ok_or_else(mismatch)?;
        if package
            .parameter_declaration(id)
            .map_err(|_| mismatch())?
            .name
            .as_ref()
            != parameter.name()
            || !binder_matches_scalar(view, id, &scalar)
        {
            return Err(mismatch());
        }
        parameters.push(ResultParameter {
            name: parameter.name().to_owned(),
            scalar,
        });
    }
    Ok(ResultPostcondition { result, parameters })
}

fn unsupported_checked_clause_operator(
    view: &ClauseGraphView<'_>,
    clause: &CheckedNodeId,
) -> Option<String> {
    let [_, _, condition] = application(
        view.package().term(clause)?,
        ApplicationOperator::StateClause,
        "quire.op.state.clause",
    )?
    else {
        return None;
    };
    let mut terms = vec![condition];
    let mut visited = BTreeSet::new();
    let mut visits = 0;
    while let Some(term) = terms.pop() {
        if visits >= MAX_CONDITION_TERMS {
            return None;
        }
        visits += 1;
        match term {
            CheckedTerm::Reference { target } => {
                if visited.insert(target) {
                    terms.push(view.package().term(target)?);
                }
            }
            CheckedTerm::Application {
                operation,
                arguments,
                ..
            } => {
                let identity = operation.identity()?;
                if matches!(
                    identity,
                    "quire.op.state.pre" | "quire.op.record.project" | "quire.op.model.deref"
                ) {
                    continue;
                }
                if !supported_clause_operator(identity) {
                    return Some(identity.to_owned());
                }
                terms.extend(arguments.iter().rev());
            }
            _ => {}
        }
    }
    None
}

fn unsupported_clause_operator(term: &ClauseTerm) -> Option<&str> {
    let ClauseTerm::Operation {
        identity,
        arguments,
        ..
    } = term
    else {
        return None;
    };
    if !supported_clause_operator(identity) {
        return Some(identity);
    }
    arguments.iter().find_map(unsupported_clause_operator)
}

fn supported_clause_operator(identity: &str) -> bool {
    matches!(
        identity,
        "quire.op.boolean.and"
            | "quire.op.boolean.or"
            | "quire.op.boolean.not"
            | "quire.op.boolean.implies"
            | "quire.op.boolean.eq"
            | "quire.op.boolean.ne"
            | "quire.op.integer.eq"
            | "quire.op.integer.ne"
            | "quire.op.integer.lt"
            | "quire.op.integer.le"
            | "quire.op.integer.gt"
            | "quire.op.integer.ge"
            | "quire.op.integer.add"
            | "quire.op.integer.sub"
            | "quire.op.integer.mul"
            | "quire.op.integer.negate"
    )
}

fn result_expectation_term(term: &ClauseTerm) -> Option<&ClauseTerm> {
    let ClauseTerm::Operation {
        identity,
        arguments,
        ..
    } = term
    else {
        return None;
    };
    if identity != "quire.op.boolean.eq" {
        return None;
    }
    let [left, right] = arguments.as_slice() else {
        return None;
    };
    match (left, right) {
        (ClauseTerm::Input(input), expression) | (expression, ClauseTerm::Input(input))
            if input.side == ClauseInputSide::Result && input.boolean =>
        {
            Some(expression)
        }
        _ => None,
    }
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
    if let Some(signature) = &lowered.result_postcondition {
        if signature.result.is_some() {
            return render_result_postcondition(request, lowered, signature);
        }
    }
    if lowered.kind == ObligationKind::Postcondition {
        return render_postcondition(request, lowered);
    }
    let inputs = &lowered.oracle.descriptor.inputs;
    let invariant = lowered.kind == ObligationKind::Invariant;
    let direct_boolean = lowered.oracle.descriptor.operation_identities.is_empty();
    let mut body = Vec::new();
    let mut arguments = Vec::new();
    let mut state_fields = Vec::new();
    let mut reachable = Vec::new();
    for (index, input) in inputs.iter().enumerate() {
        let minimum =
            i64::try_from(input.lower).map_err(|_| UnsupportedObligation::RenderFailed)?;
        let maximum =
            i64::try_from(input.upper).map_err(|_| UnsupportedObligation::RenderFailed)?;
        let mut then = Vec::new();
        if invariant {
            let domain = format!("invariant_domain_{index}");
            then.push(LetBinding {
                name: domain.clone(),
                ty: None,
                mutable: false,
                init: Init::Expression(ExprText::new(format!(
                    "{} >= {minimum} && {} <= {maximum}",
                    input.name, input.name
                ))?),
            });
            reachable.push(domain);
        }
        then.push(LetBinding {
            name: input.name.clone(),
            ty: None,
            mutable: false,
            init: Init::Expression(ExprText::new(if lowered.native_integer {
                format!("nx::NativeInteger::new(i128::from({}))", input.name)
            } else {
                format!("rt::Integer::from({})", input.name)
            })?),
        });
        body.push(Statement::Input {
            name: input.name.clone(),
            ty: KaniPrimitiveType::I64,
            domain: (input.lower, input.upper),
            then,
        });
        if let Some(field) = &input.state_field {
            state_fields.push(V2ClauseStateField {
                declaring_node_id: field.declaring_node_id.clone(),
                field_name: field.field_name.clone(),
                identifier: input.name.clone(),
                minimum,
                maximum,
            });
        } else {
            arguments.push(ScalarObligationArgument {
                identifier: input.name.clone(),
                minimum,
                maximum,
            });
        }
    }
    if invariant && inputs.is_empty() {
        // The projection onto no read fields has exactly one empty valuation.
        body.push(binding(
            "invariant_domain",
            Init::Expression(expression("()")?),
            false,
        ));
    }
    let mut oracle_arguments = inputs
        .iter()
        .map(|input| Argument::Borrow(input.name.clone()))
        .collect::<Vec<_>>();
    if !direct_boolean {
        body.push(Statement::Let(LetBinding {
            name: "meter".to_owned(),
            ty: None,
            mutable: true,
            init: Init::ScalarMeter(crate::kani::abi::SCALAR_HARNESS_LIMITS),
        }));
        oracle_arguments.push(Argument::BorrowMut("meter".to_owned()));
    }
    body.push(Statement::Call {
        result: "outcome".to_owned(),
        callee: "oracle_clause".to_owned(),
        args: oracle_arguments,
    });
    if invariant {
        body.push(Statement::Assert {
            condition: ExprText::new(if direct_boolean {
                "outcome"
            } else {
                "matches!(&outcome, Ok(rt::Outcome::Completed(true)))"
            })?,
            message: "the V2 invariant holds for every admitted state".to_owned(),
        });
    }
    let (label, cover) = if invariant {
        (
            "invariant",
            Cover {
                condition: ExprText::new(if inputs.is_empty() {
                    "matches!(invariant_domain, ())".to_owned()
                } else {
                    reachable.join(" && ")
                })?,
                message: "the V2 invariant's admitted state domain is reachable".to_owned(),
            },
        )
    } else {
        (
            "precondition",
            Cover {
                condition: ExprText::new(if direct_boolean {
                    "outcome"
                } else {
                    "matches!(outcome, Ok(rt::Outcome::Completed(true)))"
                })?,
                message: "the V2 precondition is satisfiable within admitted input bounds"
                    .to_owned(),
            },
        )
    };
    let path = harness_path(&lowered.module_symbol, &lowered.harness_symbol)?;
    let spec = HarnessSpec::new(
        format!("V2 {label} {}", message_safe_node_label(&lowered.node_id)),
        path,
        lowered.oracle.source.clone(),
        ProofKind::Proof,
        body,
        vec![cover],
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
        collection_operation: None,
        kind: lowered.kind,
        oracle_symbol: "oracle_clause".to_owned(),
        subject_path: request.subject_path.to_owned(),
        arguments,
        state_fields,
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

fn render_result_postcondition(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredV2Clause,
    signature: &ResultPostcondition,
) -> Result<KaniV2ClauseObligationHarness, UnsupportedObligation> {
    let refusal = || UnsupportedObligation::V2ClauseSignatureMismatch {
        node_id: lowered.node_id.clone(),
    };
    let mut body = Vec::new();
    let mut arguments = Vec::new();
    let mut oracle_bindings = BTreeSet::new();
    let mut drawn_parameters = signature.parameters.iter().collect::<Vec<_>>();
    drawn_parameters.sort_by(|left, right| left.name.cmp(&right.name));
    for parameter in drawn_parameters {
        let oracle_name = format!("parameter_{}", parameter.name);
        let symbolic = format!("{oracle_name}_current");
        if syn::parse_str::<syn::Ident>(&symbolic).is_err() {
            return Err(refusal());
        }
        let (ty, domain, then) = match parameter.scalar {
            MachineScalar::Boolean => (
                KaniPrimitiveType::Boolean,
                (0, 1),
                vec![LetBinding {
                    name: oracle_name.clone(),
                    ty: None,
                    mutable: false,
                    init: Init::Expression(expression(&symbolic)?),
                }],
            ),
            MachineScalar::Integer { minimum, maximum } => (
                KaniPrimitiveType::I64,
                (i128::from(minimum), i128::from(maximum)),
                vec![LetBinding {
                    name: oracle_name.clone(),
                    ty: None,
                    mutable: false,
                    init: Init::Expression(expression(if lowered.native_integer {
                        format!("nx::NativeInteger::new(i128::from({symbolic}))")
                    } else {
                        format!("rt::Integer::from({symbolic})")
                    })?),
                }],
            ),
        };
        body.push(Statement::Input {
            name: symbolic.clone(),
            ty,
            domain,
            then,
        });
        arguments.push(ScalarObligationArgument {
            identifier: symbolic.clone(),
            minimum: i64::try_from(domain.0).map_err(|_| refusal())?,
            maximum: i64::try_from(domain.1).map_err(|_| refusal())?,
        });
        oracle_bindings.insert(oracle_name);
    }
    let subject_arguments = signature
        .parameters
        .iter()
        .map(|parameter| format!("parameter_{}_current", parameter.name))
        .collect::<Vec<_>>();
    let (result_type, result_domain) = match signature.result {
        Some(MachineScalar::Boolean) => (LetType::Boolean, None),
        Some(MachineScalar::Integer { minimum, maximum }) => {
            (LetType::I64, Some((minimum, maximum)))
        }
        None => return Err(refusal()),
    };
    if let (true, Some(expectation)) = (lowered.native_integer, &lowered.native_expectation) {
        body.push(binding(
            "expected",
            Init::Expression(expression(native_expected_expression(expectation)?)?),
            false,
        ));
    }
    body.push(Statement::Let(LetBinding {
        name: "subject_result".to_owned(),
        ty: Some(result_type),
        mutable: false,
        init: Init::Expression(expression(format!(
            "{}({})",
            request.subject_path,
            subject_arguments.join(", ")
        ))?),
    }));
    if lowered
        .oracle
        .descriptor
        .inputs
        .iter()
        .any(|input| input.name == "result_result")
    {
        body.push(binding(
            "result_result",
            Init::Expression(expression(match signature.result {
                Some(MachineScalar::Boolean) => "subject_result".to_owned(),
                Some(MachineScalar::Integer { .. }) if lowered.native_integer => {
                    "nx::NativeInteger::new(i128::from(subject_result))".to_owned()
                }
                Some(MachineScalar::Integer { .. }) => {
                    "rt::Integer::from(subject_result)".to_owned()
                }
                None => return Err(refusal()),
            })?),
            false,
        ));
        oracle_bindings.insert("result_result".to_owned());
    }
    if lowered
        .oracle
        .descriptor
        .inputs
        .iter()
        .any(|input| !oracle_bindings.contains(&input.name))
    {
        return Err(refusal());
    }
    let direct_boolean = lowered.oracle.descriptor.operation_identities.is_empty();
    if !direct_boolean {
        body.push(binding(
            "meter",
            Init::ScalarMeter(crate::kani::abi::SCALAR_HARNESS_LIMITS),
            true,
        ));
    }
    if let Some((minimum, maximum)) = result_domain {
        body.push(Statement::ProducedResultDomainAssert {
            condition: expression(format!(
                "subject_result >= {} && subject_result <= {}",
                crate::kani::abi::i64_literal(minimum),
                crate::kani::abi::i64_literal(maximum)
            ))?,
            message: "the produced V2 result lies in its admitted domain".to_owned(),
        });
    }
    let mut oracle_arguments = lowered
        .oracle
        .descriptor
        .inputs
        .iter()
        .map(|input| {
            if input.boolean {
                Argument::Value(input.name.clone())
            } else {
                Argument::Borrow(input.name.clone())
            }
        })
        .collect::<Vec<_>>();
    if !direct_boolean {
        oracle_arguments.push(Argument::BorrowMut("meter".to_owned()));
    }
    body.push(Statement::Call {
        result: "outcome".to_owned(),
        callee: "oracle_clause".to_owned(),
        args: oracle_arguments,
    });
    if lowered.native_integer && lowered.native_expectation.is_some() {
        let completed_prefix = append_arithmetic_probes(&mut body, &lowered.arithmetic)?;
        body.push(Statement::Assert {
            condition: expression(format!(
                "!({completed_prefix}) || subject_result == expected"
            ))?,
            message: "the produced V2 Boolean result agrees with the independent checked condition"
                .to_owned(),
        });
    }
    body.push(Statement::Assert {
        condition: expression(if direct_boolean {
            "outcome"
        } else {
            "matches!(&outcome, Ok(rt::Outcome::Completed(true)))"
        })?,
        message: "the V2 postcondition holds for the produced result".to_owned(),
    });
    let path = harness_path(&lowered.module_symbol, &lowered.harness_symbol)?;
    let spec = HarnessSpec::new(
        format!(
            "V2 postcondition {}",
            message_safe_node_label(&lowered.node_id)
        ),
        path,
        lowered.oracle.source.clone(),
        ProofKind::Proof,
        body,
        vec![Cover {
            condition: expression(if direct_boolean {
                "outcome"
            } else {
                "matches!(&outcome, Ok(rt::Outcome::Completed(_)))"
            })?,
            message: "the V2 result postcondition reaches its admitted domain".to_owned(),
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
        collection_operation: None,
        kind: ObligationKind::Postcondition,
        oracle_symbol: "oracle_clause".to_owned(),
        subject_path: request.subject_path.to_owned(),
        arguments,
        state_fields: Vec::new(),
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

fn append_arithmetic_probes(
    body: &mut Vec<Statement>,
    arithmetic: &[ArithmeticProbe],
) -> Result<String, UnsupportedObligation> {
    let mut completed_prefix = "true".to_owned();
    for (index, probe) in arithmetic.iter().enumerate() {
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
    Ok(completed_prefix)
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
        Statement::Let(LetBinding {
            name: "subject".to_owned(),
            ty: Some(LetType::Unit),
            mutable: false,
            init: Init::Expression(expression(format!("{}({symbolic})", request.subject_path))?),
        }),
    ];
    let completed_prefix = append_arithmetic_probes(&mut body, &lowered.arithmetic)?;
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
        collection_operation: None,
        kind: ObligationKind::Postcondition,
        oracle_symbol: "oracle_clause".to_owned(),
        subject_path: request.subject_path.to_owned(),
        arguments: vec![ScalarObligationArgument {
            identifier: symbolic.to_owned(),
            minimum,
            maximum,
        }],
        state_fields: Vec::new(),
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
                && !input.boolean
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
mod unsupported_operator_tests {
    use super::unsupported_clause_operator;
    use crate::core::ir::clause::ClauseTerm;
    use ix_trace_rs::trace;

    #[trace("FR-015-AC-42", "TC-025")]
    #[test]
    fn nested_absolute_value_names_its_exact_operator() {
        let term = ClauseTerm::Operation {
            identity: "quire.op.integer.ge".to_owned(),
            arguments: vec![
                ClauseTerm::Operation {
                    identity: "quire.op.integer.abs".to_owned(),
                    arguments: vec![ClauseTerm::Integer("-2".to_owned())],
                    result_bound: None,
                },
                ClauseTerm::Integer("0".to_owned()),
            ],
            result_bound: None,
        };
        assert_eq!(
            unsupported_clause_operator(&term),
            Some("quire.op.integer.abs")
        );
    }
}

#[cfg(test)]
mod postcondition_signature_tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_contract_model::CheckedOperationReadErrorCode;

    #[trace("FR-025-AC-11", "TC-036")]
    #[test]
    fn refused_complete_census_keeps_its_original_typed_work_record() {
        let (package, clause) =
            native_twin::Twin::with_bounded_result_postcondition().emitted_package("DepositResult");
        let view = ClauseGraphView::new(&package);
        let refusal =
            match lower_with_census(&view, &clause, ObligationKind::Postcondition, |p, c| {
                p.model_operations(c, 0)
            }) {
                Err(refusal) => refusal,
                Ok(_) => panic!("a refused complete read cannot expose a lowerable harness"),
            };
        let UnsupportedObligation::V2ClauseOperationRead { node_id, source } = refusal else {
            panic!("the original typed read refusal is retained: {refusal:?}");
        };
        assert_eq!(node_id, clause);
        assert_eq!(source.code, CheckedOperationReadErrorCode::WorkLimit);
        assert_eq!(source.limit, 0);
        assert_eq!(source.work, 1);
        let context = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.semantic_form.as_ref() == "object_type")
            .expect("selected object context");
        assert_eq!(source.context, context.node_id);
    }

    #[trace("FR-025-AC-11", "TC-036")]
    #[test]
    fn binder_role_type_and_order_mismatches_refuse_before_harness_lowering() {
        let (package, clause) = native_twin::Twin::with_two_parameter_result_postcondition()
            .emitted_package("ResultWithParameters");
        let view = ClauseGraphView::new(&package);
        let context = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.semantic_form.as_ref() == "object_type")
            .expect("selected object context");
        let operations = package
            .model_operations(&context.node_id, 65_536)
            .expect("authentic complete signature");
        let operation = operations
            .operations()
            .iter()
            .find(|operation| operation.name() == "deposit")
            .expect("selected deposit operation");
        let [CheckedTerm::Aggregate { members }, _, _] = application(
            package.term(&clause).expect("admitted clause body"),
            ApplicationOperator::StateClause,
            "quire.op.state.clause",
        )
        .expect("admitted state clause") else {
            panic!("admitted ordered binders");
        };
        assert_eq!(members.len(), 4);
        let result = machine_scalar(
            operation
                .result()
                .flatten()
                .expect("bounded produced result"),
        )
        .expect("signed-i64 result domain");
        let mut wrong_role = members.clone();
        wrong_role.swap(0, 1);
        let mut wrong_order = members.clone();
        wrong_order.swap(2, 3);
        for (label, changed, expected_result) in [
            ("role", wrong_role, Some(result.clone())),
            ("order", wrong_order, Some(result)),
            ("type", members.clone(), Some(MachineScalar::Boolean)),
        ] {
            let refusal = match validate_signature_binders(
                &view,
                &clause,
                &context.node_id,
                &changed,
                operation.parameters(),
                expected_result,
            ) {
                Err(refusal) => refusal,
                Ok(_) => panic!("{label} mismatch would expose a lowerable harness"),
            };
            assert!(matches!(
                refusal,
                UnsupportedObligation::V2ClauseSignatureMismatch { node_id }
                    if node_id == clause
            ));
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

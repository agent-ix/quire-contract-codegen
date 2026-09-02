//! Deterministic tri-state harness generation.

use std::{collections::BTreeMap, fmt::Write as _};

use quire_contract_ir::{
    ClauseId, DependencyIdentity, DependencyKind, RequirementRef, StateObservation, TypedExpression,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::{
    generate_boolean_oracle,
    oracle::{
        attestation_context_is_valid, bounded_readable_component, dependency_parameters,
        generated_artifact_bundle, length_delimited_identity, oracle_symbol, reference_identifier,
    },
    Artifact, AttestationContext, GeneratedArtifactBundle, GenerationDiagnostic,
    GenerationErrorCode, GenerationTerminalState, OracleRequest,
};

/// Explicit inputs for one generated pre/post harness.
pub struct HarnessRequest<'a> {
    /// Requirement identity and revision retained by every verdict.
    pub requirement: &'a RequirementRef,
    /// Clause used when a precondition rejects an input.
    pub precondition_clause: &'a ClauseId,
    /// Clause used when an accepted case fails its postcondition.
    pub postcondition_clause: &'a ClauseId,
    /// Validated Boolean precondition lowered into the harness artifact.
    pub precondition: &'a TypedExpression,
    /// Validated Boolean postcondition lowered into the harness artifact.
    pub postcondition: &'a TypedExpression,
    /// Stable execution-point name reported by the runtime verdict.
    pub execution_point: &'a str,
    /// Minimum accepted cases required before the generated campaign can conclude.
    pub minimum_accepted_cases: u32,
    /// Maximum explicit framework discards allowed before the generated campaign fails.
    pub maximum_discarded_cases: u32,
    /// Caller-owned attestation binding shared by both clause derivations.
    pub attestation: AttestationContext<'a>,
}

struct HarnessShellRequest<'a> {
    requirement: &'a RequirementRef,
    precondition_clause: &'a ClauseId,
    postcondition_clause: &'a ClauseId,
    execution_point: &'a str,
}

/// Stable reason a harness artifact could not be generated.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HarnessErrorCode {
    /// The execution-point identity was empty or contained control characters.
    InvalidExecutionPoint,
    /// Precondition and postcondition identities were not distinct.
    DuplicateClauseIdentity,
    /// The campaign policy cannot establish a meaningful acceptance floor.
    InvalidCampaignPolicy,
    /// A typed clause could not be lowered without approximation.
    ClauseGenerationFailed,
    /// Clause dependencies cannot be bound to the supported harness shape.
    UnsupportedHarnessBinding,
    /// Generated source did not parse as Rust.
    InvalidGeneratedSyntax,
    /// A proof attestation could not be validated or serialized.
    AttestationGenerationFailed,
}

impl HarnessErrorCode {
    const fn terminal_state(
        self,
        generation_code: Option<GenerationErrorCode>,
    ) -> GenerationTerminalState {
        if let Some(code) = generation_code {
            return code.terminal_state();
        }
        match self {
            Self::InvalidExecutionPoint
            | Self::DuplicateClauseIdentity
            | Self::InvalidCampaignPolicy => GenerationTerminalState::InvalidInput,
            Self::UnsupportedHarnessBinding => GenerationTerminalState::Unsupported,
            Self::ClauseGenerationFailed
            | Self::InvalidGeneratedSyntax
            | Self::AttestationGenerationFailed => GenerationTerminalState::Inconclusive,
        }
    }
}

/// Structured harness-generation failure.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HarnessDiagnostic {
    /// Stable diagnostic category.
    pub code: HarnessErrorCode,
    /// Interface-001 terminal state for this failure.
    pub terminal_state: GenerationTerminalState,
    /// Preserved lower-level clause or attestation failure code, when one exists.
    pub generation_code: Option<GenerationErrorCode>,
    /// Stable input path associated with the failure.
    pub path: String,
    /// Human-readable detail not used as machine identity.
    pub message: String,
}

struct HarnessBinding {
    inputs: Vec<(String, String)>,
    state: Option<(String, String)>,
}

/// Generates a deterministic harness bound to the supplied typed pre- and postcondition clauses.
///
/// The generated public entry points accept only clause dependencies plus a subject invocation. The
/// lower-level closure shell is private, so callers cannot substitute different pre/post semantics.
/// The harness snapshots state before evaluating the generated precondition, never invokes a
/// rejected case, and evaluates the generated postcondition over both snapshots.
// Implements: FR-002
pub fn generate_tristate_harness(
    request: &HarnessRequest<'_>,
) -> Result<GeneratedArtifactBundle, Vec<HarnessDiagnostic>> {
    if request.minimum_accepted_cases == 0 {
        return Err(vec![HarnessDiagnostic {
            code: HarnessErrorCode::InvalidCampaignPolicy,
            terminal_state: GenerationTerminalState::InvalidInput,
            generation_code: None,
            path: "minimum_accepted_cases".to_owned(),
            message: "minimum accepted cases must be greater than zero".to_owned(),
        }]);
    }
    if !attestation_context_is_valid(&request.attestation) {
        return Err(vec![HarnessDiagnostic {
            code: HarnessErrorCode::AttestationGenerationFailed,
            terminal_state: GenerationTerminalState::InvalidInput,
            generation_code: Some(GenerationErrorCode::InvalidAttestationContext),
            path: "attestation.context".to_owned(),
            message: "the harness attestation binding is invalid".to_owned(),
        }]);
    }
    if request.precondition_clause == request.postcondition_clause {
        return Err(vec![HarnessDiagnostic {
            code: HarnessErrorCode::DuplicateClauseIdentity,
            terminal_state: HarnessErrorCode::DuplicateClauseIdentity.terminal_state(None),
            generation_code: None,
            path: "clauses".to_owned(),
            message: "precondition and postcondition clause identities must be distinct".to_owned(),
        }]);
    }
    let shell_request = HarnessShellRequest {
        requirement: request.requirement,
        precondition_clause: request.precondition_clause,
        postcondition_clause: request.postcondition_clause,
        execution_point: request.execution_point,
    };
    let shell = generate_harness_shell(&shell_request).map_err(|error| vec![error])?;
    let precondition_request = OracleRequest {
        requirement: request.requirement,
        clause: request.precondition_clause,
        expression: request.precondition,
        attestation: request.attestation,
    };
    let postcondition_request = OracleRequest {
        requirement: request.requirement,
        clause: request.postcondition_clause,
        expression: request.postcondition,
        attestation: request.attestation,
    };
    let precondition = generate_boolean_oracle(&precondition_request)
        .map_err(|diagnostics| map_clause_diagnostics("precondition", diagnostics))?;
    let postcondition = generate_boolean_oracle(&postcondition_request)
        .map_err(|diagnostics| map_clause_diagnostics("postcondition", diagnostics))?;
    let precondition_parameters = dependency_parameters(&precondition_request)
        .map_err(|diagnostics| map_clause_diagnostics("precondition", diagnostics))?;
    let postcondition_parameters = dependency_parameters(&postcondition_request)
        .map_err(|diagnostics| map_clause_diagnostics("postcondition", diagnostics))?;
    let binding = harness_binding(&precondition_parameters, &postcondition_parameters)?;

    let requirement = request.requirement.requirement().as_str();
    let revision = request.requirement.revision().get();
    let base_symbol = harness_symbol(
        requirement,
        revision,
        request.precondition_clause.as_str(),
        request.postcondition_clause.as_str(),
    );
    let shell_symbol = format!("{base_symbol}_shell");
    let shell_adapter_symbol = format!("{shell_symbol}_proptest");
    let adapter_symbol = format!("{base_symbol}_proptest");
    let runner_symbol = format!("{base_symbol}_run_campaign");
    let conclude_symbol = format!("{base_symbol}_conclude_campaign");
    let accepted_case_symbol = format!("{base_symbol}_accepted_case");
    let rejected_case_symbol = format!("{base_symbol}_rejected_case");
    let discarded_case_symbol = format!("{base_symbol}_discarded_case");
    let summary_type = format!("{}CampaignSummary", to_upper_camel(&base_symbol));
    let case_type = format!("{}CampaignCase", to_upper_camel(&base_symbol));
    let disposition_type = format!("{}CampaignDisposition", to_upper_camel(&base_symbol));
    let precondition_symbol =
        oracle_symbol(requirement, revision, request.precondition_clause.as_str());
    let postcondition_symbol =
        oracle_symbol(requirement, revision, request.postcondition_clause.as_str());
    let shell_precondition_identity_symbol =
        format!("{}_PRECONDITION", shell_symbol.to_ascii_uppercase());
    let shell_postcondition_identity_symbol =
        format!("{}_POSTCONDITION", shell_symbol.to_ascii_uppercase());
    let minimum_accepted_cases = request.minimum_accepted_cases.to_string();
    let maximum_discarded_cases = request.maximum_discarded_cases.to_string();
    let input_parameters = binding
        .inputs
        .iter()
        .map(|(_, identifier)| format!("{identifier}: bool"))
        .collect::<Vec<_>>();
    let mut public_parameters = input_parameters.clone();
    if let Some((_, state)) = &binding.state {
        public_parameters.push(format!("{state}: &mut bool"));
    }
    let input_value = match binding.inputs.as_slice() {
        [] => "()".to_owned(),
        [(_, identifier)] => format!("({identifier},)"),
        values => format!(
            "({})",
            values
                .iter()
                .map(|(_, identifier)| identifier.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    let invoke_types = {
        let mut values = vec!["bool"; binding.inputs.len()];
        if binding.state.is_some() {
            values.push("&mut bool");
        }
        values.join(", ")
    };
    let invoke_arguments = {
        let mut values = binding
            .inputs
            .iter()
            .enumerate()
            .map(|(index, _)| format!("input.{index}"))
            .collect::<Vec<_>>();
        if binding.state.is_some() {
            values.push("state".to_owned());
        }
        values.join(", ")
    };
    let invoke_input = if binding.inputs.is_empty() {
        "_input"
    } else {
        "input"
    };
    let invoke_state = if binding.state.is_none() {
        "_state"
    } else {
        "state"
    };
    let precondition_arguments = oracle_arguments(&precondition_parameters, &binding, false)?;
    let postcondition_arguments = oracle_arguments(&postcondition_parameters, &binding, true)?;
    let precondition_input = if precondition_parameters
        .iter()
        .any(|(dependency, _)| dependency.kind() == DependencyKind::Input)
    {
        "input"
    } else {
        "_input"
    };
    let precondition_state = if precondition_parameters
        .iter()
        .any(|(dependency, _)| dependency.kind() == DependencyKind::State)
    {
        "pre_state"
    } else {
        "_pre_state"
    };
    let postcondition_input = if postcondition_parameters
        .iter()
        .any(|(dependency, _)| dependency.kind() == DependencyKind::Input)
    {
        "input"
    } else {
        "_input"
    };
    let postcondition_pre_state = if postcondition_parameters.iter().any(|(dependency, _)| {
        dependency.kind() == DependencyKind::State
            && dependency.observation() == Some(StateObservation::Pre)
    }) {
        "pre_state"
    } else {
        "_pre_state"
    };
    let postcondition_post_state = if postcondition_parameters.iter().any(|(dependency, _)| {
        dependency.kind() == DependencyKind::State
            && dependency.observation() == Some(StateObservation::Post)
    }) {
        "post_state"
    } else {
        "_post_state"
    };
    let state_setup = if binding.state.is_some() {
        String::new()
    } else {
        "    let mut generated_unit_state = ();\n".to_owned()
    };
    let state_argument = binding
        .state
        .as_ref()
        .map(|(_, identifier)| identifier.as_str())
        .unwrap_or("&mut generated_unit_state");
    let public_parameter_text = if public_parameters.is_empty() {
        String::new()
    } else {
        format!("    {},\n", public_parameters.join(",\n    "))
    };
    let invoke_bound = if invoke_types.is_empty() {
        "Invoke: FnOnce()".to_owned()
    } else {
        format!("Invoke: FnOnce({invoke_types})")
    };
    let campaign_invoke_bound = if invoke_types.is_empty() {
        "Invoke: Fn()".to_owned()
    } else {
        format!("Invoke: Fn({invoke_types})")
    };
    let mut case_fields = binding
        .inputs
        .iter()
        .map(|(_, identifier)| {
            format!("    /// Generated Boolean input `{identifier}`.\n    {identifier}: bool,\n")
        })
        .collect::<String>();
    if let Some((_, state)) = &binding.state {
        let _ = write!(
            case_fields,
            "    /// Initial generated Boolean state `{state}`.\n    {state}: bool,\n"
        );
    }
    let mut case_parameters = binding
        .inputs
        .iter()
        .map(|(_, identifier)| format!("{identifier}: bool"))
        .collect::<Vec<_>>();
    if let Some((_, state)) = &binding.state {
        case_parameters.push(format!("{state}: bool"));
    }
    let case_parameter_text = case_parameters.join(", ");
    let mut case_initializers = binding
        .inputs
        .iter()
        .map(|(_, identifier)| identifier.clone())
        .collect::<Vec<_>>();
    if let Some((_, state)) = &binding.state {
        case_initializers.push(state.clone());
    }
    let case_initializer_text = if case_initializers.is_empty() {
        String::new()
    } else {
        format!(
            "            {},\n",
            case_initializers.join(",\n            ")
        )
    };
    let mut runner_arguments = binding
        .inputs
        .iter()
        .map(|(_, identifier)| format!("case.{identifier}"))
        .collect::<Vec<_>>();
    if let Some((_, state)) = &binding.state {
        runner_arguments.push(format!("&mut case.{state}"));
    }
    let runner_argument_text = if runner_arguments.is_empty() {
        String::new()
    } else {
        format!(
            "            {},\n",
            runner_arguments.join(",\n            ")
        )
    };
    let runner_invoke = if invoke_types.is_empty() {
        "|| invoke()".to_owned()
    } else {
        let arguments = (0..binding.inputs.len())
            .map(|index| format!("input_{index}"))
            .chain(binding.state.is_some().then_some("state".to_owned()))
            .collect::<Vec<_>>();
        format!(
            "|{}| invoke({})",
            arguments.join(", "),
            arguments.join(", ")
        )
    };
    let runner_case_binding = if binding.state.is_some() {
        "mut case"
    } else {
        "case"
    };
    let facade = format!(
        "\n/// Evaluates the generated precondition, subject, and postcondition in contract order.\n\
pub fn {base_symbol}<'evidence, Invoke>(\n\
{public_parameter_text}    invoke: Invoke,\n\
    observations: &'evidence mut [quire_contract_runtime::Observation<'static>; 2],\n\
) -> quire_contract_runtime::Verdict<'evidence>\n\
where\n\
    {invoke_bound},\n\
{{\n\
{state_setup}    let input = {input_value};\n\
    {shell_symbol}(\n\
        &input,\n\
        {state_argument},\n\
        |value| *value,\n\
        |{precondition_input}, {precondition_state}| {precondition_symbol}({precondition_arguments}),\n\
        |{invoke_input}, {invoke_state}| invoke({invoke_arguments}),\n\
        |{postcondition_input}, {postcondition_pre_state}, {postcondition_post_state}| {postcondition_symbol}({postcondition_arguments}),\n\
        observations,\n\
    )\n\
}}\n\
\n\
/// Records and adapts one generated bound harness invocation for the owned campaign runner.\n\
fn {adapter_symbol}<'evidence, Invoke>(\n\
    report: &mut quire_contract_runtime::CampaignReport<'static>,\n\
    expected_rejection: bool,\n\
{public_parameter_text}    invoke: Invoke,\n\
    observations: &'evidence mut [quire_contract_runtime::Observation<'static>; 2],\n\
) -> proptest::test_runner::TestCaseResult\n\
where\n\
    {invoke_bound},\n\
{{\n\
{state_setup}    let input = {input_value};\n\
    {shell_adapter_symbol}(\n\
        report,\n\
        expected_rejection,\n\
        &input,\n\
        {state_argument},\n\
        |value| *value,\n\
        |{precondition_input}, {precondition_state}| {precondition_symbol}({precondition_arguments}),\n\
        |{invoke_input}, {invoke_state}| invoke({invoke_arguments}),\n\
        |{postcondition_input}, {postcondition_pre_state}, {postcondition_post_state}| {postcondition_symbol}({postcondition_arguments}),\n\
        observations,\n\
    )\n\
}}\n\
\n\
/// One generated campaign disposition.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
enum {disposition_type} {{\n\
    /// Exercise the harness and require the declared domain result.\n\
    Exercise {{\n\
        /// Whether the generated precondition must reject this case.\n\
        expected_rejection: bool,\n\
    }},\n\
    /// Count an explicit framework discard without invoking the subject.\n\
    Discard,\n\
}}\n\
\n\
/// One generated, expectation-bound campaign case.\n\
#[derive(Clone, Debug)]\n\
pub struct {case_type} {{\n\
{case_fields}    /// Generated disposition bound to these exact input values.\n\
    disposition: {disposition_type},\n\
}}\n\
\n\
/// Constructs a generated case that the bound harness must accept.\n\
pub fn {accepted_case_symbol}({case_parameter_text}) -> {case_type} {{\n\
    {case_type} {{\n\
{case_initializer_text}        disposition: {disposition_type}::Exercise {{ expected_rejection: false }},\n\
    }}\n\
}}\n\
\n\
/// Constructs a generated case that the bound harness must reject as a precondition.\n\
pub fn {rejected_case_symbol}({case_parameter_text}) -> {case_type} {{\n\
    {case_type} {{\n\
{case_initializer_text}        disposition: {disposition_type}::Exercise {{ expected_rejection: true }},\n\
    }}\n\
}}\n\
\n\
/// Constructs an explicit framework discard that never invokes the subject.\n\
pub fn {discarded_case_symbol}({case_parameter_text}) -> {case_type} {{\n\
    {case_type} {{\n\
{case_initializer_text}        disposition: {disposition_type}::Discard,\n\
    }}\n\
}}\n\
\n\
/// Complete accounting returned after the generated campaign acceptance floor is met.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub struct {summary_type} {{\n\
    /// Accepted cases, including postcondition failures.\n\
    pub accepted: u64,\n\
    /// Precondition-rejected cases.\n\
    pub rejected: u64,\n\
    /// Accepted cases that failed a postcondition.\n\
    pub failed: u64,\n\
    /// Framework-discarded cases.\n\
    pub discarded: u64,\n\
}}\n\
\n\
/// Enforces the generated campaign accepted-case floor and returns retained accounting.\n\
fn {conclude_symbol}(\n\
    report: &quire_contract_runtime::CampaignReport<'static>,\n\
) -> Result<{summary_type}, proptest::test_runner::TestCaseError> {{\n\
    const MINIMUM_ACCEPTED_CASES: u64 = {minimum_accepted_cases};\n\
    const MAXIMUM_DISCARDED_CASES: u64 = {maximum_discarded_cases};\n\
    let counts = report.counts();\n\
    let summary = {summary_type} {{\n\
        accepted: counts.accepted(),\n\
        rejected: counts.rejected(),\n\
        failed: counts.failed(),\n\
        discarded: counts.discarded(),\n\
    }};\n\
    if summary.accepted < MINIMUM_ACCEPTED_CASES {{\n\
        return Err(proptest::test_runner::TestCaseError::fail(format!(\n\
            \"generated campaign accepted {{}} cases; minimum is {{MINIMUM_ACCEPTED_CASES}}; rejected={{}} failed={{}} discarded={{}}\",\n\
            summary.accepted, summary.rejected, summary.failed, summary.discarded,\n\
        )));\n\
    }}\n\
    if summary.discarded > MAXIMUM_DISCARDED_CASES {{\n\
        return Err(proptest::test_runner::TestCaseError::fail(format!(\n\
            \"generated campaign discarded {{}} cases; maximum is {{MAXIMUM_DISCARDED_CASES}}; accepted={{}} rejected={{}} failed={{}}\",\n\
            summary.discarded, summary.accepted, summary.rejected, summary.failed,\n\
        )));\n\
    }}\n\
    Ok(summary)\n\
}}\n\
\n\
/// Runs the generated campaign, owns every discard, and enforces final accounting.\n\
pub fn {runner_symbol}<Strategy, Invoke>(\n\
    runner: &mut proptest::test_runner::TestRunner,\n\
    strategy: &Strategy,\n\
    report: &mut quire_contract_runtime::CampaignReport<'static>,\n\
    invoke: Invoke,\n\
) -> Result<{summary_type}, proptest::test_runner::TestError<{case_type}>>\n\
where\n\
    Strategy: proptest::strategy::Strategy<Value = {case_type}>,\n\
    {campaign_invoke_bound},\n\
{{\n\
    const MINIMUM_ACCEPTED_CASES: u32 = {minimum_accepted_cases};\n\
    if runner.config().cases < MINIMUM_ACCEPTED_CASES {{\n\
        return Err(proptest::test_runner::TestError::Abort(format!(\n\
            \"proptest configured {{}} cases; generated campaign requires at least {{MINIMUM_ACCEPTED_CASES}}\",\n\
            runner.config().cases,\n\
        ).into()));\n\
    }}\n\
    let report = core::cell::RefCell::new(report);\n\
    runner.run(strategy, |{runner_case_binding}| {{\n\
        let mut report = report.borrow_mut();\n\
        match case.disposition {{\n\
            {disposition_type}::Discard => {{\n\
                report.record_discard();\n\
                if report.counts().discarded() > u64::from({maximum_discarded_cases}u32) {{\n\
                    Err(proptest::test_runner::TestCaseError::fail(\n\
                        \"generated campaign exceeded its explicit discard limit\",\n\
                    ))\n\
                }} else {{\n\
                    Err(proptest::test_runner::TestCaseError::reject(\n\
                        \"explicit generated campaign discard\",\n\
                    ))\n\
                }}\n\
            }}\n\
            {disposition_type}::Exercise {{ expected_rejection }} => {{\n\
                let mut observations = [\n\
                    quire_contract_runtime::Observation::new(\n\
                        {shell_precondition_identity_symbol},\n\
                        quire_contract_runtime::ClauseKind::Precondition,\n\
                        quire_contract_runtime::ClauseOutcome::NotEvaluated,\n\
                        None,\n\
                    ),\n\
                    quire_contract_runtime::Observation::new(\n\
                        {shell_postcondition_identity_symbol},\n\
                        quire_contract_runtime::ClauseKind::Postcondition,\n\
                        quire_contract_runtime::ClauseOutcome::NotEvaluated,\n\
                        None,\n\
                    ),\n\
                ];\n\
                {adapter_symbol}(\n\
                    &mut report,\n\
                    expected_rejection,\n\
{runner_argument_text}                    {runner_invoke},\n\
                    &mut observations,\n\
                )\n\
            }}\n\
        }}\n\
    }})?;\n\
    {conclude_symbol}(report.into_inner()).map_err(|error| {{\n\
        proptest::test_runner::TestError::Abort(error.to_string().into())\n\
    }})\n\
}}\n"
    );
    let source = format!(
        "#![deny(missing_docs)]\n//! Generated tri-state contract harness.\n{}\n{}\n{}\n{}",
        shell, precondition.rust.contents, postcondition.rust.contents, facade
    );
    syn::parse_file(&source).map_err(|error| {
        vec![HarnessDiagnostic {
            code: HarnessErrorCode::InvalidGeneratedSyntax,
            terminal_state: HarnessErrorCode::InvalidGeneratedSyntax
                .terminal_state(Some(GenerationErrorCode::InvalidGeneratedSyntax)),
            generation_code: Some(GenerationErrorCode::InvalidGeneratedSyntax),
            path: "generated.rust".to_owned(),
            message: error.to_string(),
        }]
    })?;
    let rust = artifact(format!("src/generated/{base_symbol}.rs"), source);
    let revision_text = revision.to_string();
    let input = length_delimited_identity(&[
        requirement,
        &revision_text,
        request.precondition_clause.as_str(),
        request.postcondition_clause.as_str(),
        request.execution_point,
        &minimum_accepted_cases,
        &maximum_discarded_cases,
        &precondition.rust.sha256,
        &postcondition.rust.sha256,
    ]);
    generated_artifact_bundle(
        &request.attestation,
        request.requirement,
        "generate_tristate_harness",
        &base_symbol,
        input.as_bytes(),
        "generated-rust-harness",
        "quire.codegen.rust-harness/v1",
        rust,
    )
    .map_err(|code| {
        vec![HarnessDiagnostic {
            code: HarnessErrorCode::AttestationGenerationFailed,
            terminal_state: HarnessErrorCode::AttestationGenerationFailed
                .terminal_state(Some(code)),
            generation_code: Some(code),
            path: "generated.attestation".to_owned(),
            message: "the harness proof attestation could not be emitted".to_owned(),
        }]
    })
}

fn map_clause_diagnostics(
    role: &str,
    diagnostics: Vec<GenerationDiagnostic>,
) -> Vec<HarnessDiagnostic> {
    diagnostics
        .into_iter()
        .map(|diagnostic| HarnessDiagnostic {
            code: HarnessErrorCode::ClauseGenerationFailed,
            terminal_state: HarnessErrorCode::ClauseGenerationFailed
                .terminal_state(Some(diagnostic.code)),
            generation_code: Some(diagnostic.code),
            path: format!("{role}.{}", diagnostic.path),
            message: format!("{:?}: {}", diagnostic.code, diagnostic.message),
        })
        .collect()
}

fn harness_binding(
    precondition: &[(DependencyIdentity, String)],
    postcondition: &[(DependencyIdentity, String)],
) -> Result<HarnessBinding, Vec<HarnessDiagnostic>> {
    let mut inputs = BTreeMap::new();
    let mut states = BTreeMap::new();
    let mut generated_names = BTreeMap::new();
    for (is_postcondition, parameters) in [(false, precondition), (true, postcondition)] {
        for (dependency, _) in parameters {
            if dependency.path().len() != 1 {
                return Err(binding_error(
                    "clauses.dependencies",
                    "harnesses support only direct input and state dependencies",
                ));
            }
            let name = dependency.path()[0].as_str();
            match dependency.kind() {
                DependencyKind::Input
                    if matches!(
                        dependency.observation(),
                        None | Some(StateObservation::Current)
                    ) =>
                {
                    let identifier = format!("input_{}", reference_identifier(name, None));
                    reject_generated_collision(&mut generated_names, &identifier, name)?;
                    inputs.entry(name.to_owned()).or_insert(identifier);
                }
                DependencyKind::State
                    if dependency.observation() == Some(StateObservation::Pre)
                        || (is_postcondition
                            && dependency.observation() == Some(StateObservation::Post)) =>
                {
                    let identifier = format!("state_{}", reference_identifier(name, None));
                    reject_generated_collision(&mut generated_names, &identifier, name)?;
                    states.entry(name.to_owned()).or_insert(identifier);
                }
                DependencyKind::State => {
                    return Err(binding_error(
                        "clauses.dependencies",
                        "preconditions may observe only pre-state; postconditions may observe pre- and post-state",
                    ));
                }
                _ => {
                    return Err(binding_error(
                        "clauses.dependencies",
                        "harnesses support only current inputs and explicitly observed state",
                    ));
                }
            }
        }
    }
    if states.len() > 1 {
        return Err(binding_error(
            "clauses.dependencies",
            "the first harness slice supports at most one Boolean state binding",
        ));
    }
    Ok(HarnessBinding {
        inputs: inputs.into_iter().collect(),
        state: states.into_iter().next(),
    })
}

fn reject_generated_collision(
    generated: &mut BTreeMap<String, String>,
    identifier: &str,
    source: &str,
) -> Result<(), Vec<HarnessDiagnostic>> {
    if let Some(existing) = generated.get(identifier) {
        if existing != source {
            return Err(binding_error(
                "clauses.dependencies",
                "distinct dependencies claim the same generated harness parameter",
            ));
        }
    } else {
        generated.insert(identifier.to_owned(), source.to_owned());
    }
    Ok(())
}

fn oracle_arguments(
    parameters: &[(DependencyIdentity, String)],
    binding: &HarnessBinding,
    postcondition: bool,
) -> Result<String, Vec<HarnessDiagnostic>> {
    parameters
        .iter()
        .map(|(dependency, _)| match dependency.kind() {
            DependencyKind::Input => binding
                .inputs
                .iter()
                .position(|(name, _)| name == dependency.path()[0].as_str())
                .map(|index| format!("input.{index}"))
                .ok_or_else(|| binding_error("clauses.dependencies", "input binding disappeared")),
            DependencyKind::State => match dependency.observation() {
                Some(StateObservation::Pre) => Ok("*pre_state".to_owned()),
                Some(StateObservation::Post) if postcondition => Ok("*post_state".to_owned()),
                _ => Err(binding_error(
                    "clauses.dependencies",
                    "state observation is not available at this harness phase",
                )),
            },
            _ => Err(binding_error(
                "clauses.dependencies",
                "unsupported harness dependency kind",
            )),
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|values| values.join(", "))
}

fn binding_error(path: &str, message: &str) -> Vec<HarnessDiagnostic> {
    vec![HarnessDiagnostic {
        code: HarnessErrorCode::UnsupportedHarnessBinding,
        terminal_state: HarnessErrorCode::UnsupportedHarnessBinding.terminal_state(None),
        generation_code: None,
        path: path.to_owned(),
        message: message.to_owned(),
    }]
}

fn generate_harness_shell(request: &HarnessShellRequest<'_>) -> Result<String, HarnessDiagnostic> {
    if request.execution_point.is_empty()
        || request
            .execution_point
            .chars()
            .any(|value| value.is_control())
    {
        return Err(HarnessDiagnostic {
            code: HarnessErrorCode::InvalidExecutionPoint,
            terminal_state: HarnessErrorCode::InvalidExecutionPoint.terminal_state(None),
            generation_code: None,
            path: "execution_point".to_owned(),
            message: "execution point must be non-empty and contain no control characters"
                .to_owned(),
        });
    }

    let requirement = request.requirement.requirement().as_str();
    let revision = request.requirement.revision().get();
    let precondition = request.precondition_clause.as_str();
    let postcondition = request.postcondition_clause.as_str();
    let symbol = format!(
        "{}_shell",
        harness_symbol(requirement, revision, precondition, postcondition)
    );
    let identity_symbol = format!("{}_IDENTITY", symbol.to_ascii_uppercase());
    let execution_symbol = format!("{}_EXECUTION_POINT", symbol.to_ascii_uppercase());
    let precondition_symbol = format!("{}_PRECONDITION", symbol.to_ascii_uppercase());
    let postcondition_symbol = format!("{}_POSTCONDITION", symbol.to_ascii_uppercase());
    let adapter_symbol = format!("{symbol}_proptest");

    let source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Generated by quire-contract-codegen {}; DO NOT EDIT.\n\
// Requirement: {requirement}@{revision}; Preconditions: {precondition}; Postconditions: {postcondition}\n\
\n\
/// Generated contract identity for `{requirement}@{revision}`.\n\
pub const {identity_symbol}: quire_contract_runtime::ContractIdentity<'static> =\n\
    quire_contract_runtime::ContractIdentity::new(\n\
        quire_contract_runtime::RequirementId::new({requirement:?}),\n\
        quire_contract_runtime::RevisionId::new({:?}),\n\
    );\n\
/// Generated execution point for this harness.\n\
pub const {execution_symbol}: quire_contract_runtime::ExecutionPoint<'static> =\n\
    quire_contract_runtime::ExecutionPoint::new({:?});\n\
/// Generated precondition clause identity.\n\
pub const {precondition_symbol}: quire_contract_runtime::ClauseId<'static> =\n\
    quire_contract_runtime::ClauseId::new({precondition:?});\n\
/// Generated postcondition clause identity.\n\
pub const {postcondition_symbol}: quire_contract_runtime::ClauseId<'static> =\n\
    quire_contract_runtime::ClauseId::new({postcondition:?});\n\
\n\
/// Snapshots state, checks the precondition, invokes the subject, and checks post-state.\n\
fn {symbol}<'evidence, Input, State, Snapshot, Precondition, Invoke, Postcondition>(\n\
    input: &Input,\n\
    state: &mut State,\n\
    snapshot: Snapshot,\n\
    precondition: Precondition,\n\
    invoke: Invoke,\n\
    postcondition: Postcondition,\n\
    observations: &'evidence mut [quire_contract_runtime::Observation<'static>; 2],\n\
) -> quire_contract_runtime::Verdict<'evidence>\n\
where\n\
    Snapshot: FnOnce(&State) -> State,\n\
    Precondition: FnOnce(&Input, &State) -> bool,\n\
    Invoke: FnOnce(&Input, &mut State),\n\
    Postcondition: FnOnce(&Input, &State, &State) -> bool,\n\
{{\n\
    let pre_state = snapshot(state);\n\
    if !precondition(input, &pre_state) {{\n\
        let rejection = quire_contract_runtime::FailureDetail::new(\n\
            {precondition_symbol},\n\
            quire_contract_runtime::FailureKind::Precondition,\n\
            1,\n\
            Some(\"generated precondition rejected case\"),\n\
        );\n\
        observations[0] = quire_contract_runtime::Observation::new(\n\
            {precondition_symbol},\n\
            quire_contract_runtime::ClauseKind::Precondition,\n\
            quire_contract_runtime::ClauseOutcome::Rejected,\n\
            Some(rejection),\n\
        );\n\
        observations[1] = quire_contract_runtime::Observation::new(\n\
            {postcondition_symbol},\n\
            quire_contract_runtime::ClauseKind::Postcondition,\n\
            quire_contract_runtime::ClauseOutcome::NotEvaluated,\n\
            None,\n\
        );\n\
        let context = quire_contract_runtime::VerdictContext::new(\n\
            {identity_symbol},\n\
            {execution_symbol},\n\
            observations,\n\
        );\n\
        return quire_contract_runtime::Verdict::rejected_precondition(context, rejection);\n\
    }}\n\
\n\
    observations[0] = quire_contract_runtime::Observation::new(\n\
        {precondition_symbol},\n\
        quire_contract_runtime::ClauseKind::Precondition,\n\
        quire_contract_runtime::ClauseOutcome::Passed,\n\
        None,\n\
    );\n\
    invoke(input, state);\n\
    if postcondition(input, &pre_state, state) {{\n\
        observations[1] = quire_contract_runtime::Observation::new(\n\
            {postcondition_symbol},\n\
            quire_contract_runtime::ClauseKind::Postcondition,\n\
            quire_contract_runtime::ClauseOutcome::Passed,\n\
            None,\n\
        );\n\
        let context = quire_contract_runtime::VerdictContext::new(\n\
            {identity_symbol},\n\
            {execution_symbol},\n\
            observations,\n\
        );\n\
        quire_contract_runtime::Verdict::passed(context)\n\
    }} else {{\n\
        let failure = quire_contract_runtime::FailureDetail::new(\n\
            {postcondition_symbol},\n\
            quire_contract_runtime::FailureKind::Postcondition,\n\
            2,\n\
            Some(\"generated postcondition failed\"),\n\
        );\n\
        observations[1] = quire_contract_runtime::Observation::new(\n\
            {postcondition_symbol},\n\
            quire_contract_runtime::ClauseKind::Postcondition,\n\
            quire_contract_runtime::ClauseOutcome::Failed,\n\
            Some(failure),\n\
        );\n\
        let context = quire_contract_runtime::VerdictContext::new(\n\
            {identity_symbol},\n\
            {execution_symbol},\n\
            observations,\n\
        );\n\
        quire_contract_runtime::Verdict::failed_postcondition(context, failure)\n\
    }}\n\
}}\n\
\n\
/// Records and adapts one generated harness verdict for proptest.\n\
fn {adapter_symbol}<'evidence, Input, State, Snapshot, Precondition, Invoke, Postcondition>(\n\
    report: &mut quire_contract_runtime::CampaignReport<'static>,\n\
    expected_rejection: bool,\n\
    input: &Input,\n\
    state: &mut State,\n\
    snapshot: Snapshot,\n\
    precondition: Precondition,\n\
    invoke: Invoke,\n\
    postcondition: Postcondition,\n\
    observations: &'evidence mut [quire_contract_runtime::Observation<'static>; 2],\n\
) -> proptest::test_runner::TestCaseResult\n\
where\n\
    Snapshot: FnOnce(&State) -> State,\n\
    Precondition: FnOnce(&Input, &State) -> bool,\n\
    Invoke: FnOnce(&Input, &mut State),\n\
    Postcondition: FnOnce(&Input, &State, &State) -> bool,\n\
{{\n\
    let verdict = {symbol}(\n\
        input, state, snapshot, precondition, invoke, postcondition, observations,\n\
    );\n\
    let expectation_matches = if expected_rejection {{\n\
        verdict.kind() == quire_contract_runtime::VerdictKind::RejectedPrecondition\n\
    }} else {{\n\
        verdict.kind() != quire_contract_runtime::VerdictKind::RejectedPrecondition\n\
    }};\n\
    let adapted = quire_contract_runtime::proptest_adapter::adapt_recording(report, &verdict);\n\
    if !expectation_matches {{\n\
        return Err(proptest::test_runner::TestCaseError::fail(format!(\n\
            \"generated expected-domain mismatch: expected_rejection={{expected_rejection}} actual={{:?}}\",\n\
            verdict.kind(),\n\
        )));\n\
    }}\n\
    adapted\n\
}}\n",
        env!("CARGO_PKG_VERSION"),
        revision.to_string(),
        request.execution_point,
    );

    syn::parse_file(&source).map_err(|error| HarnessDiagnostic {
        code: HarnessErrorCode::InvalidGeneratedSyntax,
        terminal_state: HarnessErrorCode::InvalidGeneratedSyntax
            .terminal_state(Some(GenerationErrorCode::InvalidGeneratedSyntax)),
        generation_code: Some(GenerationErrorCode::InvalidGeneratedSyntax),
        path: "generated.rust".to_owned(),
        message: error.to_string(),
    })?;

    Ok(source)
}

fn harness_symbol(
    requirement: &str,
    revision: u64,
    precondition: &str,
    postcondition: &str,
) -> String {
    let readable = bounded_readable_component(requirement);
    let revision_text = revision.to_string();
    let identity =
        length_delimited_identity(&[requirement, &revision_text, precondition, postcondition]);
    format!(
        "harness_{readable}_{revision}_{}",
        sha256(identity.as_bytes())
    )
}

fn to_upper_camel(value: &str) -> String {
    value
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut characters = part.chars();
            match characters.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + characters.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn artifact(path: String, contents: String) -> Artifact {
    let sha256 = sha256(contents.as_bytes());
    Artifact {
        path,
        contents,
        sha256,
    }
}

fn sha256(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        let _ = write!(result, "{byte:02x}");
    }
    result
}

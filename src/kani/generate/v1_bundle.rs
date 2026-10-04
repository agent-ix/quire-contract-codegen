//! INTERIM: the V1 bounded Boolean Kani bundle generator, still live. `generate_kani_bundle`
//! and what only it uses; step 4f deletes this file whole (AD-004).

use std::{collections::BTreeMap, fmt::Write as _};

use quire_contract_model::{
    ClauseId, DependencyIdentity, DependencyKind, RequirementRef, SourceSpan, StateObservation,
    TypedExpression,
};
use serde::{Deserialize, Serialize};

use crate::{
    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},
    core::naming::{bounded_readable_component, oracle_symbol, unique_pair},
    kani::abi::{
        adapter_options, i64_literal, KaniBindingRole, KaniIntegerBounds, KaniPrimitiveType,
        KaniSolver,
    },
    kani::census::{
        dependency_readiness, dependency_site, normalize_dependencies, ProofDependencyEdge,
        ProofDependencyKind, ProofDependencyRequest, ProofReadiness,
    },
    kani::generate::census_validation::{
        deterministic_json, single_diagnostic, validate_dependencies, validate_path,
        validate_plain_identity, KaniDiagnostic, KaniErrorCode,
    },
    kani::generate::outcome::MAX_OBLIGATION_UNWIND,
    oracle::boolean_v1::{
        generate_named_boolean_oracle, typed_dependency_parameters, DependencyParameter,
        OracleRequest, OracleShape, RustValueType,
    },
};

/// One normalized primitive argument or result in the generated subject ABI.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct KaniSubjectBinding {
    /// Complete checked dependency identity used for uniqueness and ordering.
    pub dependency: DependencyIdentity,
    /// Deterministic generated Rust identifier.
    pub identifier: String,
    /// Argument or result position.
    pub role: KaniBindingRole,
    /// Rust primitive type.
    pub primitive_type: KaniPrimitiveType,
    /// Exact integer bounds, or `None` for Boolean bindings.
    pub integer_bounds: Option<KaniIntegerBounds>,
    /// Every authored occurrence contributing this normalized binding.
    pub source_spans: Vec<SourceSpan>,
}

/// Explicit inputs for one bounded Boolean Kani proof bundle.
pub struct KaniRequest<'a> {
    /// Requirement identity and revision retained by every artifact.
    pub requirement: &'a RequirementRef,
    /// Boolean precondition clause.
    pub precondition_clause: &'a ClauseId,
    /// Boolean postcondition clause.
    pub postcondition_clause: &'a ClauseId,
    /// Validated Boolean precondition expression.
    pub precondition: &'a TypedExpression,
    /// Validated Boolean postcondition expression.
    pub postcondition: &'a TypedExpression,
    /// Stable proof identity within the requirement revision.
    pub proof_id: &'a str,
    /// Rust path to a customer function with signature `fn(bool, bool) -> bool`.
    pub subject_path: &'a str,
    /// Explicit bounded loop unwind value.
    pub unwind: u32,
    /// Explicit solver choice.
    pub solver: KaniSolver,
    /// Complete caller-owned dependency census.
    pub dependencies: &'a [ProofDependencyRequest<'a>],
}

/// Deterministic Kani proof-dependency graph.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProofDependencyGraph {
    /// Stable graph schema identity.
    pub schema_version: String,
    /// Root proof identity.
    pub proof_id: String,
    /// Requirement identity.
    pub requirement_id: String,
    /// Requirement revision.
    pub requirement_revision: u64,
    /// Complete Kani option vector.
    pub options: Vec<String>,
    /// Derived dependency readiness; this is not a proof execution result.
    pub readiness: ProofReadiness,
    /// Explicit proof execution state; generation never executes or classifies a proof.
    pub proof_execution_state: String,
    /// Ordered copied current/pre values supplied to the customer subject.
    pub subject_arguments: Vec<KaniSubjectBinding>,
    /// Ordered post-state values returned by the customer subject.
    pub subject_results: Vec<KaniSubjectBinding>,
    /// Generated Rust artifact path.
    pub source_artifact_path: String,
    /// Sorted complete dependency census.
    pub dependencies: Vec<ProofDependencyEdge>,
}

/// All-or-nothing Kani source and proof graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniArtifactBundle {
    /// Generated Rust contract and proof source.
    pub rust: Artifact,
    /// Deterministic proof-dependency graph.
    pub proof_graph: Artifact,
}

struct SubjectAbi {
    arguments: Vec<KaniSubjectBinding>,
    results: Vec<KaniSubjectBinding>,
}

struct KaniSource<'a> {
    requirement: &'a str,
    revision: u64,
    proof_id: &'a str,
    subject_path: &'a str,
    module_symbol: &'a str,
    contract_symbol: &'a str,
    harness_symbol: &'a str,
    precondition_symbol: &'a str,
    postcondition_symbol: &'a str,
    precondition_arguments: &'a str,
    postcondition_arguments: &'a str,
    precondition_source: &'a str,
    postcondition_source: &'a str,
    stub_attributes: &'a str,
    dependency_assumptions: &'a str,
    abi: &'a SubjectAbi,
}

/// Generates one bounded Kani contract/proof bundle or structured diagnostics with no partial output.
pub fn generate_kani_bundle(
    request: &KaniRequest<'_>,
) -> Result<KaniArtifactBundle, Vec<KaniDiagnostic>> {
    validate_request(request)?;
    let precondition_request = OracleRequest {
        requirement: request.requirement,
        clause: request.precondition_clause,
        expression: request.precondition,
    };
    let postcondition_request = OracleRequest {
        requirement: request.requirement,
        clause: request.postcondition_clause,
        expression: request.postcondition,
    };
    let requirement = request.requirement.requirement().as_str();
    let revision = request.requirement.revision().get();
    let (precondition_clause, postcondition_clause) = (
        request.precondition_clause.as_str(),
        request.postcondition_clause.as_str(),
    );
    let (precondition_symbol, postcondition_symbol) = unique_pair(
        (
            oracle_symbol(requirement, revision, precondition_clause),
            precondition_clause,
        ),
        (
            oracle_symbol(requirement, revision, postcondition_clause),
            postcondition_clause,
        ),
    );
    let shape = OracleShape::KaniBundle;
    let precondition =
        generate_named_boolean_oracle(&precondition_request, &precondition_symbol, shape)
            .map_err(|values| map_clause_diagnostics("precondition", values))?;
    let postcondition =
        generate_named_boolean_oracle(&postcondition_request, &postcondition_symbol, shape)
            .map_err(|values| map_clause_diagnostics("postcondition", values))?;
    let precondition_parameters = typed_dependency_parameters(&precondition_request, shape)
        .map_err(|values| map_clause_diagnostics("precondition", values))?;
    let postcondition_parameters = typed_dependency_parameters(&postcondition_request, shape)
        .map_err(|values| map_clause_diagnostics("postcondition", values))?;
    let abi = derive_subject_abi(&precondition_parameters, &postcondition_parameters)?;
    let symbol = kani_symbol(requirement, revision, request.proof_id);
    let contract_symbol = format!("{symbol}_contract");
    let harness_symbol = format!("{symbol}_proof");
    let module_symbol = format!("{symbol}_module");
    let precondition_arguments = predicate_arguments(&precondition_parameters, &abi, false)?;
    let postcondition_arguments = predicate_arguments(&postcondition_parameters, &abi, true)?;
    let exact_harness = format!("{module_symbol}::{harness_symbol}");
    let options = adapter_options(
        &exact_harness,
        request.unwind,
        request.solver,
        request
            .dependencies
            .iter()
            .any(|dependency| dependency.kind == ProofDependencyKind::Stubbed),
    );
    let normalized_dependencies = normalize_dependencies(request.dependencies);
    let mut source_dependencies = request.dependencies.iter().collect::<Vec<_>>();
    source_dependencies.sort_by(|left, right| left.proof_id.cmp(right.proof_id));
    let stub_attributes = source_dependencies
        .iter()
        .filter(|dependency| dependency.kind == ProofDependencyKind::Stubbed)
        .filter_map(|dependency| {
            dependency
                .original_path
                .zip(dependency.replacement_path)
                .map(|(original, replacement)| {
                    let site = dependency_site("stub", dependency.proof_id);
                    format!(
                        "    // proof-dependency-site: {site}\n    #[kani::stub({original}, {replacement})]\n"
                    )
                })
        })
        .collect::<String>();
    let dependency_assumptions = source_dependencies
        .iter()
        .filter(|dependency| dependency.kind == ProofDependencyKind::Assumed)
        .filter_map(|dependency| {
            dependency.original_path.map(|path| {
                let site = dependency_site("assumption", dependency.proof_id);
                format!(
                    "        // proof-dependency-site: {site}\n        kani::assume({path}());\n"
                )
            })
        })
        .collect::<String>();
    let source = render_kani_source(&KaniSource {
        requirement,
        revision,
        proof_id: request.proof_id,
        subject_path: request.subject_path,
        module_symbol: &module_symbol,
        contract_symbol: &contract_symbol,
        harness_symbol: &harness_symbol,
        precondition_symbol: &precondition_symbol,
        postcondition_symbol: &postcondition_symbol,
        precondition_arguments: &precondition_arguments,
        postcondition_arguments: &postcondition_arguments,
        precondition_source: &precondition.rust.contents,
        postcondition_source: &postcondition.rust.contents,
        stub_attributes: &stub_attributes,
        dependency_assumptions: &dependency_assumptions,
        abi: &abi,
    });
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(single_diagnostic(
            KaniErrorCode::ResourceLimitExceeded,
            "generated.rust",
            "generated Kani source exceeds the bounded artifact size",
        ));
    }
    syn::parse_file(&source).map_err(|error| {
        single_diagnostic(
            KaniErrorCode::InvalidGeneratedSyntax,
            "generated.rust",
            &error.to_string(),
        )
    })?;
    let rust = artifact(format!("src/generated/{symbol}.rs"), source);
    let graph_value = ProofDependencyGraph {
        schema_version: "quire.kani-proof-graph/v2".to_owned(),
        proof_id: request.proof_id.to_owned(),
        requirement_id: requirement.to_owned(),
        requirement_revision: revision,
        options,
        readiness: dependency_readiness(&normalized_dependencies),
        proof_execution_state: "not_run".to_owned(),
        subject_arguments: abi.arguments.clone(),
        subject_results: abi.results.clone(),
        source_artifact_path: rust.path.clone(),
        dependencies: normalized_dependencies,
    };
    let graph_contents = deterministic_json(&graph_value).map_err(|message| {
        single_diagnostic(
            KaniErrorCode::SerializationFailed,
            "generated.proof_graph",
            &message,
        )
    })?;
    let proof_graph = artifact(format!("proof-graphs/{symbol}.json"), graph_contents);
    Ok(KaniArtifactBundle { rust, proof_graph })
}

fn validate_request(request: &KaniRequest<'_>) -> Result<(), Vec<KaniDiagnostic>> {
    if request.precondition_clause == request.postcondition_clause {
        return Err(single_diagnostic(
            KaniErrorCode::InvalidIdentity,
            "clauses",
            "precondition and postcondition clause identities must be distinct",
        ));
    }
    validate_plain_identity(request.proof_id, "proof_id")?;
    validate_path(request.subject_path, "subject_path")?;
    if request.unwind == 0 || request.unwind > MAX_OBLIGATION_UNWIND {
        return Err(single_diagnostic(
            KaniErrorCode::InvalidUnwind,
            "unwind",
            &format!("unwind must be between 1 and {MAX_OBLIGATION_UNWIND}"),
        ));
    }
    validate_dependencies(request.dependencies, request.proof_id)?;
    Ok(())
}

fn derive_subject_abi(
    precondition: &[DependencyParameter],
    postcondition: &[DependencyParameter],
) -> Result<SubjectAbi, Vec<KaniDiagnostic>> {
    let mut arguments: BTreeMap<DependencyIdentity, KaniSubjectBinding> = BTreeMap::new();
    let mut results: BTreeMap<DependencyIdentity, KaniSubjectBinding> = BTreeMap::new();
    let mut logical_types: BTreeMap<(DependencyKind, Vec<String>), RustValueType> = BTreeMap::new();
    let mut generated_names: BTreeMap<String, DependencyIdentity> = BTreeMap::new();
    for (is_postcondition, parameters) in [(false, precondition), (true, postcondition)] {
        for parameter in parameters {
            let dependency = &parameter.dependency;
            let role = match (dependency.kind(), dependency.observation()) {
                (DependencyKind::Input, None | Some(StateObservation::Current))
                | (
                    DependencyKind::State,
                    Some(StateObservation::Current | StateObservation::Pre),
                ) => KaniBindingRole::Argument,
                (DependencyKind::State, Some(StateObservation::Post)) if is_postcondition => {
                    KaniBindingRole::Result
                }
                (DependencyKind::State, Some(StateObservation::Post)) => {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "precondition.dependencies",
                        "post-state data cannot be bound in a Kani precondition",
                    ));
                }
                _ => {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "clauses.dependencies",
                        "the Kani adapter supports only direct current input/current state/pre-state arguments and post-state results",
                    ));
                }
            };
            let logical_key = (
                dependency.kind(),
                dependency
                    .path()
                    .iter()
                    .map(|part| part.as_str().to_owned())
                    .collect::<Vec<_>>(),
            );
            if let Some(existing) = logical_types.get(&logical_key) {
                if existing != &parameter.value_type {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "clauses.dependencies",
                        "one logical dependency has conflicting primitive types or integer domains",
                    ));
                }
            } else {
                logical_types.insert(logical_key, parameter.value_type.clone());
            }
            if let Some(existing) = generated_names.get(&parameter.identifier) {
                if existing != dependency {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "clauses.dependencies",
                        "distinct dependency identities collide in the generated subject ABI",
                    ));
                }
            } else {
                generated_names.insert(parameter.identifier.clone(), dependency.clone());
            }
            let bindings = match role {
                KaniBindingRole::Argument => &mut arguments,
                KaniBindingRole::Result => &mut results,
            };
            if let Some(existing) = bindings.get_mut(dependency) {
                if existing.identifier != parameter.identifier
                    || existing.role != role
                    || !binding_matches_value_type(existing, &parameter.value_type)
                {
                    return Err(single_diagnostic(
                        KaniErrorCode::UnsupportedBinding,
                        "clauses.dependencies",
                        "one dependency identity has incompatible generated bindings",
                    ));
                }
                existing.source_spans.push(parameter.source.clone());
                existing.source_spans.sort();
                existing.source_spans.dedup();
            } else {
                bindings.insert(dependency.clone(), subject_binding(parameter, role));
            }
        }
    }
    Ok(SubjectAbi {
        arguments: arguments.into_values().collect(),
        results: results.into_values().collect(),
    })
}

fn subject_binding(parameter: &DependencyParameter, role: KaniBindingRole) -> KaniSubjectBinding {
    let (primitive_type, integer_bounds) = match &parameter.value_type {
        RustValueType::Boolean => (KaniPrimitiveType::Boolean, None),
        RustValueType::Integer(value) => (
            KaniPrimitiveType::I64,
            Some(KaniIntegerBounds {
                domain: value.domain(),
                minimum: value.minimum(),
                maximum: value.maximum(),
                overflow: value.overflow(),
            }),
        ),
    };
    KaniSubjectBinding {
        dependency: parameter.dependency.clone(),
        identifier: parameter.identifier.clone(),
        role,
        primitive_type,
        integer_bounds,
        source_spans: vec![parameter.source.clone()],
    }
}

fn binding_matches_value_type(binding: &KaniSubjectBinding, value_type: &RustValueType) -> bool {
    match (value_type, binding.primitive_type, &binding.integer_bounds) {
        (RustValueType::Boolean, KaniPrimitiveType::Boolean, None) => true,
        (RustValueType::Integer(value), KaniPrimitiveType::I64, Some(bounds)) => {
            bounds.domain == value.domain()
                && bounds.minimum == value.minimum()
                && bounds.maximum == value.maximum()
                && bounds.overflow == value.overflow()
        }
        _ => false,
    }
}

fn predicate_arguments(
    parameters: &[DependencyParameter],
    abi: &SubjectAbi,
    postcondition: bool,
) -> Result<String, Vec<KaniDiagnostic>> {
    parameters
        .iter()
        .map(|parameter| {
            if let Some(binding) = abi
                .arguments
                .iter()
                .find(|binding| binding.dependency == parameter.dependency)
            {
                return Ok(binding.identifier.clone());
            }
            if postcondition {
                if let Some(index) = abi
                    .results
                    .iter()
                    .position(|binding| binding.dependency == parameter.dependency)
                {
                    return Ok(result_access(abi.results.len(), index));
                }
            }
            Err(single_diagnostic(
                KaniErrorCode::UnsupportedBinding,
                "clauses.dependencies",
                "a clause dependency has no position in the normalized subject ABI",
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|values| values.join(", "))
}

fn result_access(result_count: usize, index: usize) -> String {
    if result_count == 1 {
        "*post_state".to_owned()
    } else {
        format!("post_state.{index}")
    }
}

fn render_kani_source(value: &KaniSource<'_>) -> String {
    let argument_declarations = value
        .abi
        .arguments
        .iter()
        .map(|binding| {
            format!(
                "{}: {}",
                binding.identifier,
                binding.primitive_type.source_name()
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let argument_names = value
        .abi
        .arguments
        .iter()
        .map(|binding| binding.identifier.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let result_type = result_type(&value.abi.results);
    let framing = render_framing(value.abi);
    let symbolic_arguments = render_symbolic_arguments(&value.abi.arguments);
    let result_bounds = render_result_bounds(&value.abi.results);
    let postcondition_call = format!(
        "{}({})",
        value.postcondition_symbol, value.postcondition_arguments
    );
    let ensures = if result_bounds.is_empty() {
        postcondition_call
    } else {
        format!("({result_bounds}) && ({postcondition_call})")
    };
    let post_state_name = if value.abi.results.is_empty() {
        "_post_state"
    } else {
        "post_state"
    };
    format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Requirement: {}@{}; Proof: {}\n\
\n\
{}\n\
{}\n\
#[cfg(kani)]\n\
mod {} {{\n\
    use super::*;\n\
\n\
    // BEGIN framing\n\
    // proof-id: {}\n\
{}    // END framing\n\
\n\
    // BEGIN binding\n\
    fn call_subject({argument_declarations}) -> {result_type} {{\n\
        {}({argument_names})\n\
    }}\n\
    // END binding\n\
\n\
    // BEGIN contract\n\
    #[kani::requires({}({}))]\n\
    #[kani::ensures(|{post_state_name}: &{result_type}| {ensures})]\n\
    fn {}({argument_declarations}) -> {result_type} {{\n\
        call_subject({argument_names})\n\
    }}\n\
    // END contract\n\
\n\
    // BEGIN proof harness\n\
{}    #[kani::proof_for_contract({})]\n\
    fn {}() {{\n\
{}{}        let _post_state = {}({argument_names});\n\
        kani::cover!(true, \"bundle requires and IR bounds are jointly satisfiable\");\n\
    }}\n\
    // END proof harness\n\
}}\n",
        value.requirement,
        value.revision,
        value.proof_id,
        value.precondition_source,
        value.postcondition_source,
        value.module_symbol,
        value.proof_id,
        framing,
        value.subject_path,
        value.precondition_symbol,
        value.precondition_arguments,
        value.contract_symbol,
        value.stub_attributes,
        value.contract_symbol,
        value.harness_symbol,
        value.dependency_assumptions,
        symbolic_arguments,
        value.contract_symbol,
    )
}

fn render_framing(abi: &SubjectAbi) -> String {
    abi.arguments
        .iter()
        .chain(&abi.results)
        .map(|binding| {
            let role = match binding.role {
                KaniBindingRole::Argument => "argument",
                KaniBindingRole::Result => "result",
            };
            format!(
                "    // {role}-binding: {}:{}\n",
                binding.identifier,
                binding.primitive_type.source_name()
            )
        })
        .collect()
}

fn result_type(results: &[KaniSubjectBinding]) -> String {
    match results {
        [] => "()".to_owned(),
        [binding] => binding.primitive_type.source_name().to_owned(),
        values => format!(
            "({})",
            values
                .iter()
                .map(|binding| binding.primitive_type.source_name())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn render_symbolic_arguments(arguments: &[KaniSubjectBinding]) -> String {
    let mut source = String::new();
    for binding in arguments {
        let _ = writeln!(
            source,
            "        let {}: {} = kani::any();",
            binding.identifier,
            binding.primitive_type.source_name()
        );
        if let Some(bounds) = &binding.integer_bounds {
            let _ = writeln!(
                source,
                "        kani::assume({} >= {} && {} <= {});",
                binding.identifier,
                i64_literal(bounds.minimum),
                binding.identifier,
                i64_literal(bounds.maximum)
            );
        }
    }
    source
}

fn render_result_bounds(results: &[KaniSubjectBinding]) -> String {
    results
        .iter()
        .enumerate()
        .filter_map(|(index, binding)| {
            binding.integer_bounds.as_ref().map(|bounds| {
                let access = result_access(results.len(), index);
                format!(
                    "{access} >= {} && {access} <= {}",
                    i64_literal(bounds.minimum),
                    i64_literal(bounds.maximum)
                )
            })
        })
        .collect::<Vec<_>>()
        .join(" && ")
}

fn map_clause_diagnostics(
    role: &str,
    diagnostics: Vec<crate::core::diagnostic::GenerationDiagnostic>,
) -> Vec<KaniDiagnostic> {
    diagnostics
        .into_iter()
        .map(|diagnostic| KaniDiagnostic {
            code: KaniErrorCode::ClauseGenerationFailed,
            terminal_state: diagnostic.terminal_state,
            generation_code: Some(diagnostic.code),
            path: format!("{role}.{}", diagnostic.path),
            source_span: diagnostic.source_span,
            message: diagnostic.message,
        })
        .collect()
}

/// The generated module, contract and proof name stem, read from the requirement, revision and
/// proof id. Kani synthesizes contract symbols and object-file names from these names, so each
/// readable component is bounded; the complete identity remains in framing and the graph.
fn kani_symbol(requirement: &str, revision: u64, proof_id: &str) -> String {
    format!(
        "kani_{}_{revision}_{}",
        bounded_readable_component(requirement),
        bounded_readable_component(proof_id)
    )
}

fn artifact(path: String, contents: String) -> Artifact {
    Artifact::new(path, contents)
}

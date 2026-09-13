//! Public bound-package admission and complete numeric strategy bundle generation.

use std::fmt::Write as _;

use quire_contract_ir::{
    BoundClause, BoundPackage, ClauseKind, ClauseRef, ComparisonOperator as IrComparisonOperator,
    ExecutionPoint, Expression, ExpressionKind, StateObservation, ValueType,
    BOUND_IDENTITY_PROFILE,
};
use sha2::{Digest as _, Sha256};

use super::{
    census::{compute_census, render_boundary_constants, render_edge_constants, CensusNames},
    population::{render_population, Population, PopulationRequest, RenderedPopulation},
    relation::{ComparisonOperator, Domain, OperandPosition, Relation},
};
use crate::{
    bound::BoundGenerationError,
    generate_bound_oracles,
    oracle::{
        generated_output_attestation, generator_implementation_digest, length_delimited_identity,
        oracle_symbol, reference_identifier, GeneratedAttestationSpec,
    },
    Artifact, AttestationContext, BoundOracleGeneration, GeneratedArtifactBundle,
    GenerationErrorCode, GenerationTerminalState, StrategyDiagnostic, StrategyErrorCode,
    MAX_GENERATED_SOURCE_BYTES,
};

/// Population selected for one bound numeric strategy bundle.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BoundStrategyPopulation {
    /// Construct only valuations for which the relation holds.
    Satisfying,
    /// Construct only valuations for which the relation is false.
    Violating,
    /// Construct both sides while preserving the selected side during shrinking.
    Broad,
    /// Enumerate the complete tagged in-domain boundary census without sampling or shrinking.
    Boundary,
}

impl BoundStrategyPopulation {
    fn name(self) -> &'static str {
        match self {
            Self::Satisfying => "Satisfying",
            Self::Violating => "Violating",
            Self::Broad => "Broad",
            Self::Boundary => "Boundary",
        }
    }

    const fn sampled(self) -> Option<Population> {
        match self {
            Self::Satisfying => Some(Population::Satisfying),
            Self::Violating => Some(Population::Violating),
            Self::Broad => Some(Population::Broad),
            Self::Boundary => None,
        }
    }
}

/// Inputs for one bound numeric strategy artifact.
pub struct BoundStrategyRequest<'a> {
    /// Public IR-owned, validated executable projection.
    pub package: &'a BoundPackage,
    /// Complete identity of the one clause to generate.
    pub clause: &'a ClauseRef,
    /// Constructive or exhaustive population to expose.
    pub population: BoundStrategyPopulation,
    /// Minimum accepted verdicts required at campaign conclusion.
    pub minimum_accepted_cases: u64,
    /// Minimum rejected-precondition verdicts required at campaign conclusion.
    pub minimum_rejected_cases: u64,
    /// Maximum explicit framework discards permitted in the complete supplied report.
    pub maximum_discarded_cases: u64,
    /// Caller-owned binding for the generated proof-attestation body.
    pub attestation: AttestationContext<'a>,
}

#[derive(Clone)]
struct Read {
    name: String,
    observation: StateObservation,
    identifier: String,
}

struct AdmittedRelation {
    relation: Relation,
    domain: Domain,
    reads: Vec<Read>,
}

/// Generates a deterministic strategy, census, oracle-conformance runner, and one attestation.
///
/// The operation first proves that the complete package is admitted by bound oracle generation,
/// then narrows the selected clause to one supported integer comparison. Any refusal returns one
/// structured diagnostic and no artifact or attestation.
// Implements: FR-008, FR-009, FR-010, FR-011, FR-012, FR-013
pub fn generate_bound_strategy(
    request: &BoundStrategyRequest<'_>,
) -> Result<GeneratedArtifactBundle, StrategyDiagnostic> {
    let Some(clause) = request
        .package
        .clauses()
        .iter()
        .find(|clause| clause.identity() == request.clause)
    else {
        return Err(bound_diagnostic(
            StrategyErrorCode::UnknownClause,
            GenerationTerminalState::InvalidInput,
            None,
            request.clause,
            None,
            "clause",
            "the requested clause is not present in the bound package",
        ));
    };

    let generated_oracles =
        generate_bound_oracles(request.package, request.attestation).map_err(|error| {
            let diagnostic = map_oracle_error(request.clause, error);
            if diagnostic.clause.as_deref() == Some(request.clause)
                && matches!(
                    diagnostic.generation_code,
                    Some(
                        GenerationErrorCode::UnsupportedExpression
                            | GenerationErrorCode::UnsupportedDependency
                    )
                )
            {
                if let Some(locus) = explicit_relation_locus(clause) {
                    return relation_diagnostic(
                        clause,
                        locus,
                        "comparison operands must be bounded integer reads or integer literals",
                    );
                }
            }
            diagnostic
        })?;
    let BoundOracleGeneration::Generated(generated_oracles) = generated_oracles else {
        return Err(bound_diagnostic(
            StrategyErrorCode::UnknownClause,
            GenerationTerminalState::InvalidInput,
            None,
            request.clause,
            None,
            "clause",
            "the requested executable clause produced no bound oracle",
        ));
    };
    let oracle = generated_oracles
        .clauses()
        .iter()
        .find(|generated| generated.identity() == request.clause)
        .ok_or_else(|| {
            bound_diagnostic(
                StrategyErrorCode::UnknownClause,
                GenerationTerminalState::InvalidInput,
                None,
                request.clause,
                None,
                "clause",
                "the requested clause is absent from bound oracle output",
            )
        })?;

    if !matches!(
        clause.kind(),
        ClauseKind::Precondition | ClauseKind::Postcondition | ClauseKind::Invariant
    ) {
        return Err(bound_diagnostic(
            StrategyErrorCode::UnsupportedClauseKind,
            GenerationTerminalState::Unsupported,
            None,
            request.clause,
            None,
            "clause.kind",
            "bound numeric strategies support preconditions, postconditions, and invariants",
        ));
    }

    let admitted = admit_relation(clause)?;
    let read_identifiers = admitted
        .reads
        .iter()
        .map(|read| read.identifier.as_str())
        .collect::<Vec<_>>();
    let census = compute_census(admitted.relation, admitted.domain)
        .map_err(|diagnostic| scope_diagnostic(diagnostic, request.clause))?;
    let boundary_available = census.boundary_population().is_ok();
    if request.population == BoundStrategyPopulation::Boundary && !boundary_available {
        return Err(scope_diagnostic(
            census.boundary_population().expect_err("checked above"),
            request.clause,
        ));
    }
    let identity = strategy_identity(request, &admitted);
    let suffix = sha256(identity.as_bytes());
    let item_suffix = &suffix[..16];

    let render_population_kind = request.population.sampled().unwrap_or(Population::Broad);
    let rendered_population = render_population(&PopulationRequest {
        relation: admitted.relation,
        domain: admitted.domain,
        population: render_population_kind,
        read_identifiers: &read_identifiers,
    })
    .map_err(|diagnostic| scope_diagnostic(diagnostic, request.clause))?;

    let census_names = CensusNames {
        item_suffix,
        fields: &read_identifiers,
    };
    let census_source = if boundary_available {
        render_boundary_constants(&census, census_names)
    } else {
        render_edge_constants(&census, census_names)
    }
    .map_err(|diagnostic| scope_diagnostic(diagnostic, request.clause))?;

    let requirement = request.clause.requirement();
    let oracle_function = oracle_symbol(
        requirement.package().as_str(),
        requirement.requirement().as_str(),
        requirement.revision().get(),
        request.clause.clause().as_str(),
    );
    let source = render_complete_source(
        request,
        clause,
        &admitted,
        &rendered_population,
        &census_source,
        boundary_available,
        &oracle_function,
        item_suffix,
        &suffix,
        &oracle.bundle().rust.contents,
    )?;
    let path = format!("src/generated/bound_strategy_{suffix}.rs");
    let rust = artifact(path, source);
    let digest = request.package.digest().to_string();
    let extra_argv = vec![
        "--clause".to_owned(),
        request.clause.clause().as_str().to_owned(),
        "--population".to_owned(),
        request.population.name().to_ascii_lowercase(),
    ];
    let attestation = generated_output_attestation(
        &request.attestation,
        requirement,
        &GeneratedAttestationSpec {
            operation: "generate_bound_strategy",
            stable_identity: &identity,
            input_bytes: identity.as_bytes(),
            input_digest: Some(&digest),
            output_role: "generated-rust-strategy",
            media_type: "text/x-rust",
            output_schema: "quire.codegen.rust-strategy/v1",
            schema_digest: None,
            canonical_profile: BOUND_IDENTITY_PROFILE,
            backend: "none",
            configuration_digest: generator_implementation_digest(),
            extra_argv: &extra_argv,
        },
        &rust,
    )
    .map_err(|code| {
        bound_diagnostic(
            if code == GenerationErrorCode::ResourceLimitExceeded {
                StrategyErrorCode::ResourceLimitExceeded
            } else {
                StrategyErrorCode::AttestationGenerationFailed
            },
            code.terminal_state(),
            Some(code),
            request.clause,
            None,
            "generated.attestation",
            "the bound strategy proof attestation could not be emitted",
        )
    })?;
    Ok(GeneratedArtifactBundle { rust, attestation })
}

// Implements: FR-008-CON-1
fn admit_relation(clause: &BoundClause) -> Result<AdmittedRelation, StrategyDiagnostic> {
    let expression = clause.expression().expression();
    let ExpressionKind::Compare {
        operator,
        left,
        right,
    } = expression.kind()
    else {
        return Err(relation_diagnostic(
            clause,
            expression,
            "the clause root must be exactly one integer comparison",
        ));
    };

    let left = read_operand(clause, left)?;
    let right = read_operand(clause, right)?;
    if left.is_none() && right.is_none() {
        return Err(relation_diagnostic(
            clause,
            expression,
            "a literal-only comparison has no constructible read population",
        ));
    }

    let (primary, partner, position) = match (left, right) {
        (Some(left), Some(right)) => (left, Some(right), OperandPosition::Left),
        (Some(left), None) => (left, None, OperandPosition::Left),
        (None, Some(right)) => (right, None, OperandPosition::Right),
        (None, None) => {
            return Err(relation_diagnostic(
                clause,
                expression,
                "a literal-only comparison has no constructible read population",
            ));
        }
    };
    if partner.as_ref().is_some_and(|partner| {
        partner.name == primary.name && partner.observation == primary.observation
    }) {
        return Err(relation_diagnostic(
            clause,
            expression,
            "a comparison cannot read the same declaration at the same observation twice",
        ));
    }
    if partner.as_ref().is_some_and(|partner| {
        partner.name == primary.name
            && matches!(
                (primary.observation, partner.observation),
                (
                    StateObservation::Current,
                    StateObservation::Pre | StateObservation::Post
                ) | (
                    StateObservation::Pre | StateObservation::Post,
                    StateObservation::Current
                )
            )
    }) {
        return Err(relation_diagnostic(
            clause,
            expression,
            "current and pre/post observations of one declaration cannot be mixed",
        ));
    }

    let declaration = clause
        .environment()
        .values()
        .iter()
        .find(|declaration| declaration.name().as_str() == primary.name)
        .ok_or_else(|| {
            relation_diagnostic(
                clause,
                expression,
                "the primary read has no bound declaration",
            )
        })?;
    let ValueType::Integer { value } = declaration.value_type() else {
        return Err(relation_diagnostic(
            clause,
            expression,
            "the primary read declaration is not a bounded integer",
        ));
    };
    let domain = Domain {
        minimum: value.minimum(),
        maximum: value.maximum(),
    };
    let operator = map_operator(*operator);
    let relation = match partner {
        Some(_) => Relation::between_reads(operator),
        None => {
            let literal = match (expression.kind(), position) {
                (ExpressionKind::Compare { right, .. }, OperandPosition::Left) => {
                    integer_literal(right)
                }
                (ExpressionKind::Compare { left, .. }, OperandPosition::Right) => {
                    integer_literal(left)
                }
                _ => None,
            }
            .ok_or_else(|| {
                relation_diagnostic(
                    clause,
                    expression,
                    "the non-read comparison operand is not an integer literal",
                )
            })?;
            Relation::with_literal(operator, position, literal)
        }
    };
    let mut reads = vec![primary];
    if let Some(partner) = partner {
        reads.push(partner);
    }
    Ok(AdmittedRelation {
        relation,
        domain,
        reads,
    })
}

fn read_operand(
    clause: &BoundClause,
    operand: &Expression,
) -> Result<Option<Read>, StrategyDiagnostic> {
    match operand.kind() {
        ExpressionKind::ValueReference { name, observation } => {
            let declaration = clause
                .environment()
                .values()
                .iter()
                .find(|declaration| declaration.name() == name)
                .ok_or_else(|| {
                    relation_diagnostic(clause, operand, "the read has no bound declaration")
                })?;
            if !matches!(declaration.value_type(), ValueType::Integer { .. }) {
                return Err(relation_diagnostic(
                    clause,
                    operand,
                    "comparison reads must be bounded integers",
                ));
            }
            Ok(Some(Read {
                name: name.as_str().to_owned(),
                observation: *observation,
                identifier: reference_identifier(name.as_str(), Some(*observation)),
            }))
        }
        ExpressionKind::IntegerLiteral { .. } => Ok(None),
        _ => Err(relation_diagnostic(
            clause,
            operand,
            "comparison operands must be integer reads or integer literals",
        )),
    }
}

fn explicit_relation_locus(clause: &BoundClause) -> Option<&Expression> {
    let ExpressionKind::Compare { left, right, .. } = clause.expression().expression().kind()
    else {
        return None;
    };
    for operand in [left.as_ref(), right.as_ref()] {
        match operand.kind() {
            ExpressionKind::IntegerLiteral { .. } => {}
            ExpressionKind::ValueReference { name, .. } => {
                let integer = clause
                    .environment()
                    .values()
                    .iter()
                    .find(|declaration| declaration.name() == name)
                    .is_some_and(|declaration| {
                        matches!(declaration.value_type(), ValueType::Integer { .. })
                    });
                if !integer {
                    return Some(operand);
                }
            }
            ExpressionKind::Numeric { .. } | ExpressionKind::NumericNegate { .. } => return None,
            _ => return Some(operand),
        }
    }
    None
}

fn integer_literal(expression: &Expression) -> Option<i64> {
    match expression.kind() {
        ExpressionKind::IntegerLiteral { value, .. } => Some(*value),
        _ => None,
    }
}

const fn map_operator(operator: IrComparisonOperator) -> ComparisonOperator {
    match operator {
        IrComparisonOperator::Equal => ComparisonOperator::Equal,
        IrComparisonOperator::NotEqual => ComparisonOperator::NotEqual,
        IrComparisonOperator::Less => ComparisonOperator::Less,
        IrComparisonOperator::LessEqual => ComparisonOperator::LessEqual,
        IrComparisonOperator::Greater => ComparisonOperator::Greater,
        IrComparisonOperator::GreaterEqual => ComparisonOperator::GreaterEqual,
    }
}

#[allow(clippy::too_many_arguments)]
fn render_complete_source(
    request: &BoundStrategyRequest<'_>,
    clause: &BoundClause,
    admitted: &AdmittedRelation,
    population: &RenderedPopulation,
    census_source: &str,
    boundary_available: bool,
    oracle_function: &str,
    item_suffix: &str,
    suffix: &str,
    oracle_source: &str,
) -> Result<String, StrategyDiagnostic> {
    let full_clause = full_clause_ref(request.clause);
    let digest = request.package.digest().to_string();
    let mut source = format!(
        "#![deny(missing_docs)]\n//! Generated bound numeric strategy artifact.\n\
// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Generated by quire-contract-codegen {}; DO NOT EDIT.\n\
// BoundPackage: {digest}; ClauseRef: {full_clause}\n\n",
        env!("CARGO_PKG_VERSION")
    );
    source.push_str(oracle_source);
    source.push('\n');
    source.push_str(&population_component(&population.source));
    source.push('\n');
    source.push_str(census_source);
    source.push('\n');
    source.push_str(&case_metadata(population, admitted));
    source.push('\n');
    source.push_str(&runner_source(
        request,
        clause,
        admitted,
        population,
        boundary_available,
        oracle_function,
        item_suffix,
        suffix,
    )?);
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(bound_diagnostic(
            StrategyErrorCode::ResourceLimitExceeded,
            GenerationTerminalState::Unsupported,
            Some(GenerationErrorCode::ResourceLimitExceeded),
            request.clause,
            None,
            "generated.rust",
            "the generated bound strategy exceeds the source-size limit",
        ));
    }
    syn::parse_file(&source).map_err(|error| {
        bound_diagnostic(
            StrategyErrorCode::InvalidGeneratedSyntax,
            GenerationTerminalState::Inconclusive,
            Some(GenerationErrorCode::InvalidGeneratedSyntax),
            request.clause,
            None,
            "generated.rust",
            &error.to_string(),
        )
    })?;
    Ok(source)
}

fn population_component(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.starts_with("#!["))
        .map(|line| {
            line.strip_prefix("//!")
                .map_or_else(|| line.to_owned(), |rest| format!("//{rest}"))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn case_metadata(population: &RenderedPopulation, admitted: &AdmittedRelation) -> String {
    let mut source = format!("impl {} {{\n", population.case_type);
    for read in &admitted.reads {
        let constant = read.identifier.to_ascii_uppercase();
        let _ = writeln!(
            source,
            "    /// Exact IR declaration name for `{}`.\n    pub const {constant}_DECLARATION: &'static str = {:?};",
            read.identifier, read.name
        );
        let _ = writeln!(
            source,
            "    /// Exact IR observation name for `{}`.\n    pub const {constant}_OBSERVATION: &'static str = {:?};",
            read.identifier,
            observation_name(read.observation)
        );
    }
    source.push_str("}\n");
    source
}

#[allow(clippy::too_many_arguments)]
// Implements: FR-013-AC-5
fn runner_source(
    request: &BoundStrategyRequest<'_>,
    clause: &BoundClause,
    admitted: &AdmittedRelation,
    population: &RenderedPopulation,
    boundary_available: bool,
    oracle_function: &str,
    item_suffix: &str,
    suffix: &str,
) -> Result<String, StrategyDiagnostic> {
    let base = format!("bound_campaign_{suffix}");
    let summary = format!("BoundCampaignSummary{}", &suffix[..16]);
    let error = format!("BoundCampaignError{}", &suffix[..16]);
    let selected_strategy = format!("bound_strategy_{suffix}");
    let runner = format!("{base}_run");
    let census_runner = format!("{base}_run_census");
    let evaluate = format!("{base}_evaluate");
    let conclude = format!("{base}_conclude");
    let case_type = &population.case_type;
    let expectation = &population.expectation_type;
    let population_strategy = &population.strategy_function;
    let oracle_arguments = clause
        .expression()
        .dependencies()
        .iter()
        .map(|dependency| {
            let name = dependency.path()[0].as_str();
            format!(
                "case.{}",
                reference_identifier(name, dependency.observation())
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let (runtime_kind, false_verdict, failure_kind) = match clause.kind() {
        ClauseKind::Precondition => ("Precondition", "RejectedPrecondition", "Precondition"),
        ClauseKind::Postcondition => ("Postcondition", "FailedPostcondition", "Postcondition"),
        ClauseKind::Invariant => ("Invariant", "FailedPostcondition", "Contract"),
        _ => {
            return Err(bound_diagnostic(
                StrategyErrorCode::UnsupportedClauseKind,
                GenerationTerminalState::Unsupported,
                None,
                request.clause,
                None,
                "clause.kind",
                "bound numeric strategies support preconditions, postconditions, and invariants",
            ));
        }
    };
    let false_outcome = if clause.kind() == ClauseKind::Precondition {
        "Rejected"
    } else {
        "Failed"
    };
    let execution_point = execution_point_name(clause.anchor());
    let identity_symbol = format!("{}_IDENTITY", oracle_function.to_ascii_uppercase());
    let clause_symbol = format!("{}_CLAUSE", oracle_function.to_ascii_uppercase());
    let minimum_accepted = request.minimum_accepted_cases;
    let minimum_rejected = request.minimum_rejected_cases;
    let maximum_discarded = request.maximum_discarded_cases;
    let accepted_floor = if minimum_accepted > 0 {
        format!("    if summary.accepted < {minimum_accepted} {{ return Err({error}::BelowAcceptedFloor {{ summary }}); }}\n")
    } else {
        String::new()
    };
    let rejected_floor = if minimum_rejected > 0 {
        format!("    if summary.rejected < {minimum_rejected} {{ return Err({error}::BelowRejectedFloor {{ summary }}); }}\n")
    } else {
        String::new()
    };

    let selected_body = if request.population == BoundStrategyPopulation::Boundary {
        let constant = format!("IN_DOMAIN_CENSUS_{}", item_suffix.to_ascii_uppercase());
        let census_type = format!("CensusTag{item_suffix}");
        let field_values = admitted
            .reads
            .iter()
            .map(|read| format!("{}: census.{}", read.identifier, read.identifier))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "    use proptest::strategy::Strategy as _;\n    proptest::sample::select({constant}.to_vec())\n        .prop_map(|census| {case_type} {{ {field_values}, expected: match census.tag {{ {census_type}::Holds => {expectation}::Holds, {census_type}::Violated => {expectation}::Violated }} }})\n        .boxed()"
        )
    } else {
        format!("    {population_strategy}()")
    };

    let census_support = if boundary_available {
        let constant = format!("IN_DOMAIN_CENSUS_{}", item_suffix.to_ascii_uppercase());
        let census_type = format!("CensusTag{item_suffix}");
        let field_values = admitted
            .reads
            .iter()
            .map(|read| format!("{}: census.{}", read.identifier, read.identifier))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "\n/// Evaluates every in-domain census case exactly once in deterministic order.\n\
pub fn {census_runner}(report: &mut quire_contract_runtime::CampaignReport<'static>) -> Result<{summary}, {error}> {{\n\
    for census in &{constant} {{\n\
        let case = {case_type} {{ {field_values}, expected: match census.tag {{ {census_type}::Holds => {expectation}::Holds, {census_type}::Violated => {expectation}::Violated }} }};\n\
        if let Err(failure) = {evaluate}(&case, report) {{ return Err(failure); }}\n\
    }}\n\
    {conclude}(report, None)\n\
}}\n"
        )
    } else {
        String::new()
    };

    Ok(format!(
        "/// Returns the selected `{population_name}` bound population.\n\
pub fn {selected_strategy}() -> proptest::strategy::BoxedStrategy<{case_type}> {{\n{selected_body}\n}}\n\n\
/// Complete exact invocation accounting for one bound numeric campaign.\n\
#[derive(Clone, Copy, Debug, Eq, PartialEq)]\n\
pub struct {summary} {{\n\
    /// Oracle evaluations, including shrink replays.\n    pub attempted: u64,\n\
    /// Passed and failed-postcondition verdicts.\n    pub accepted: u64,\n\
    /// Rejected-precondition verdicts.\n    pub rejected: u64,\n\
    /// Failed-postcondition verdicts.\n    pub failed: u64,\n\
    /// Explicit framework discards; this runner emits none.\n    pub discarded: u64,\n\
    at_limit: bool,\n\
}}\n\n\
impl {summary} {{\n\
    /// Returns exact `(discarded, attempted)` counts, or `None` for zero/saturated totals.\n\
    #[must_use]\n    pub const fn discard_rate(self) -> Option<(u64, u64)> {{\n\
        if self.attempted == 0 || self.at_limit {{ None }} else {{ Some((self.discarded, self.attempted)) }}\n\
    }}\n\
    /// Returns exact `(rejected, attempted)` counts, or `None` for zero/saturated totals.\n\
    #[must_use]\n    pub const fn rejection_rate(self) -> Option<(u64, u64)> {{\n\
        if self.attempted == 0 || self.at_limit {{ None }} else {{ Some((self.rejected, self.attempted)) }}\n\
    }}\n\
}}\n\n\
/// Machine-readable terminal failure for the bound numeric campaign.\n\
#[derive(Clone, Debug, Eq, PartialEq)]\n\
pub enum {error} {{\n\
    /// Accepted verdicts did not meet the requested floor.\n    BelowAcceptedFloor {{ /// Retained accounting.\n        summary: {summary} }},\n\
    /// Rejected verdicts did not meet the requested floor.\n    BelowRejectedFloor {{ /// Retained accounting.\n        summary: {summary} }},\n\
    /// The complete supplied report already exceeds the discard ceiling.\n    AboveDiscardCeiling {{ /// Retained accounting.\n        summary: {summary} }},\n\
    /// Proptest exhausted its configured search.\n    Exhausted {{ /// Retained accounting.\n        summary: {summary}, /// Framework detail.\n        reason: String }},\n\
    /// The generated oracle disagreed with the case's immutable tag.\n    ConformanceMismatch {{ /// Retained accounting.\n        summary: {summary}, /// Primary read value.\n        primary: i64, /// Partner read value when present.\n        partner: Option<i64>, /// Immutable expectation tag carried by the case.\n        expected: {expectation}, /// Observed runtime verdict kind.\n        observed: quire_contract_runtime::VerdictKind }},\n\
    /// The supplied campaign report names a different requirement revision.\n    IdentityMismatch {{ /// Retained accounting.\n        summary: {summary}, /// Expected report requirement.\n        expected_requirement: String, /// Expected report revision.\n        expected_revision: String, /// Actual generated requirement.\n        actual_requirement: String, /// Actual generated revision.\n        actual_revision: String }},\n\
}}\n\n\
impl {error} {{\n\
    /// Returns accounting retained for this failure.\n\
    #[must_use]\n    pub const fn summary(&self) -> &{summary} {{\n\
        match self {{ Self::BelowAcceptedFloor {{ summary }} | Self::BelowRejectedFloor {{ summary }} | Self::AboveDiscardCeiling {{ summary }} | Self::Exhausted {{ summary, .. }} | Self::ConformanceMismatch {{ summary, .. }} | Self::IdentityMismatch {{ summary, .. }} => summary }}\n\
    }}\n\
}}\n\n\
/// Derives exact rate-bearing accounting from one immutable runtime snapshot.\n\
#[must_use]\n\
pub fn {base}_summary_from_snapshot(snapshot: &quire_contract_runtime::CampaignSnapshot<'_>) -> {summary} {{\n\
    let counts = snapshot.counts();\n\
    {summary} {{ attempted: counts.total(), accepted: counts.accepted(), rejected: counts.rejected(), failed: counts.failed(), discarded: counts.discarded(), at_limit: snapshot.at_limit() }}\n\
}}\n\n\
fn {base}_summary(report: &quire_contract_runtime::CampaignReport<'static>) -> {summary} {{\n\
    let snapshot = report.snapshot();\n\
    {base}_summary_from_snapshot(&snapshot)\n\
}}\n\n\
fn {conclude}(report: &quire_contract_runtime::CampaignReport<'static>, framework_error: Option<String>) -> Result<{summary}, {error}> {{\n\
    let summary = {base}_summary(report);\n\
    if summary.discarded > {maximum_discarded} {{ return Err({error}::AboveDiscardCeiling {{ summary }}); }}\n\
    if let Some(reason) = framework_error {{ return Err({error}::Exhausted {{ summary, reason }}); }}\n\
{accepted_floor}{rejected_floor}\
    Ok(summary)\n\
}}\n\n\
fn {evaluate}(case: &{case_type}, report: &mut quire_contract_runtime::CampaignReport<'static>) -> Result<(), {error}> {{\n\
    let holds = {oracle_function}({oracle_arguments});\n\
    let failure = quire_contract_runtime::FailureDetail::new({clause_symbol}, quire_contract_runtime::FailureKind::{failure_kind}, 0, None);\n\
    let outcome = if holds {{ quire_contract_runtime::ClauseOutcome::Passed }} else {{ quire_contract_runtime::ClauseOutcome::{false_outcome} }};\n\
    let detail = if holds {{ None }} else {{ Some(failure) }};\n\
    let observations = [quire_contract_runtime::Observation::new({clause_symbol}, quire_contract_runtime::ClauseKind::{runtime_kind}, outcome, detail)];\n\
    let context = quire_contract_runtime::VerdictContext::new({identity_symbol}, quire_contract_runtime::ExecutionPoint::new({execution_point:?}), &observations);\n\
    let verdict = if holds {{ quire_contract_runtime::Verdict::passed(context) }} else {{ quire_contract_runtime::Verdict::{false_verdict} {{ context, {failure_field}: failure }} }};\n\
    let observed = verdict.kind();\n\
    if let Err(mismatch) = report.record_verdict(&verdict) {{\n\
        return Err({error}::IdentityMismatch {{ summary: {base}_summary(report), expected_requirement: mismatch.expected().requirement.as_str().to_owned(), expected_revision: mismatch.expected().revision.as_str().to_owned(), actual_requirement: mismatch.actual().requirement.as_str().to_owned(), actual_revision: mismatch.actual().revision.as_str().to_owned() }});\n\
    }}\n\
    let expected_holds = matches!(case.expected, {expectation}::Holds);\n\
    let expected_kind = if expected_holds {{ quire_contract_runtime::VerdictKind::Passed }} else {{ quire_contract_runtime::VerdictKind::{false_verdict} }};\n\
    if observed != expected_kind {{ return Err({error}::ConformanceMismatch {{ summary: {base}_summary(report), primary: case.{primary}, partner: {partner_value}, expected: case.expected, observed }}); }}\n\
    Ok(())\n\
}}\n\n\
/// Runs the selected subject-free oracle-conformance campaign.\n\
pub fn {runner}<Strategy>(runner: &mut proptest::test_runner::TestRunner, strategy: &Strategy, report: &mut quire_contract_runtime::CampaignReport<'static>) -> Result<{summary}, {error}>\n\
where Strategy: proptest::strategy::Strategy<Value = {case_type}> {{\n\
    let report = core::cell::RefCell::new(report);\n\
    let failure = core::cell::RefCell::new(None);\n\
    let run_result = runner.run(strategy, |case| {{\n\
        match {evaluate}(&case, &mut report.borrow_mut()) {{\n\
            Ok(()) => Ok(()),\n\
            Err(error) => {{ *failure.borrow_mut() = Some(error); Err(proptest::test_runner::TestCaseError::fail(\"bound oracle conformance mismatch\")) }}\n\
        }}\n\
    }});\n\
    let report = report.into_inner();\n\
    if let Some(error) = failure.into_inner() {{\n\
        let summary = {base}_summary(report);\n\
        if summary.discarded > {maximum_discarded} {{ return Err({error}::AboveDiscardCeiling {{ summary }}); }}\n\
        return Err(match error {{ {error}::ConformanceMismatch {{ primary, partner, expected, observed, .. }} => {error}::ConformanceMismatch {{ summary, primary, partner, expected, observed }}, {error}::IdentityMismatch {{ expected_requirement, expected_revision, actual_requirement, actual_revision, .. }} => {error}::IdentityMismatch {{ summary, expected_requirement, expected_revision, actual_requirement, actual_revision }}, other => other }});\n\
    }}\n\
    match run_result {{ Ok(()) => {conclude}(report, None), Err(proptest::test_runner::TestError::Abort(reason)) => {conclude}(report, Some(reason.to_string())), Err(proptest::test_runner::TestError::Fail(reason, _)) => {conclude}(report, Some(reason.to_string())) }}\n\
}}\n{census_support}",
        population_name = request.population.name(),
        failure_field = if clause.kind() == ClauseKind::Precondition {
            "rejection"
        } else {
            "failure"
        },
        primary = admitted.reads[0].identifier,
        partner_value = admitted
            .reads
            .get(1)
            .map_or_else(|| "None".to_owned(), |read| format!("Some(case.{})", read.identifier)),
    ))
}

fn strategy_identity(request: &BoundStrategyRequest<'_>, admitted: &AdmittedRelation) -> String {
    let digest = request.package.digest().to_string();
    let revision = request.clause.requirement().revision().get().to_string();
    let relation = format!("{:?}", admitted.relation);
    let domain = format!("{:?}", admitted.domain);
    length_delimited_identity(&[
        "bound-strategy/v1",
        &digest,
        request.clause.requirement().package().as_str(),
        request.clause.requirement().requirement().as_str(),
        &revision,
        request.clause.clause().as_str(),
        request.population.name(),
        &request.minimum_accepted_cases.to_string(),
        &request.minimum_rejected_cases.to_string(),
        &request.maximum_discarded_cases.to_string(),
        &relation,
        &domain,
    ])
}

fn execution_point_name(point: &ExecutionPoint) -> &'static str {
    match point {
        ExecutionPoint::Initialization { .. } => "initialization",
        ExecutionPoint::Handler { .. } => "handler",
        ExecutionPoint::Pre { .. } => "pre",
        ExecutionPoint::Post { .. } => "post",
    }
}

fn observation_name(observation: StateObservation) -> &'static str {
    match observation {
        StateObservation::Current => "current",
        StateObservation::Pre => "pre",
        StateObservation::Post => "post",
    }
}

fn full_clause_ref(reference: &ClauseRef) -> String {
    format!(
        "{}/{}@{}/{}",
        reference.requirement().package().as_str(),
        reference.requirement().requirement().as_str(),
        reference.requirement().revision().get(),
        reference.clause().as_str()
    )
}

fn relation_diagnostic(
    clause: &BoundClause,
    expression: &Expression,
    message: &str,
) -> StrategyDiagnostic {
    bound_diagnostic(
        StrategyErrorCode::UnsupportedRelation,
        GenerationTerminalState::Unsupported,
        None,
        clause.identity(),
        Some(expression.source().clone()),
        "expression.relation",
        message,
    )
}

fn map_oracle_error(requested: &ClauseRef, error: BoundGenerationError) -> StrategyDiagnostic {
    match error {
        BoundGenerationError::NameCollision(identity) => bound_diagnostic(
            StrategyErrorCode::UnsupportedClause,
            GenerationTerminalState::InvalidInput,
            Some(GenerationErrorCode::NameCollision),
            &identity,
            None,
            "expression.dependencies",
            "bound oracle generation found a dependency or symbol collision",
        ),
        BoundGenerationError::Clause {
            identity,
            diagnostics,
        } => {
            let diagnostic = diagnostics.into_iter().next();
            bound_diagnostic(
                StrategyErrorCode::UnsupportedClause,
                diagnostic
                    .as_ref()
                    .map_or(GenerationTerminalState::Inconclusive, |item| {
                        item.terminal_state
                    }),
                diagnostic.as_ref().map(|item| item.code),
                &identity,
                diagnostic
                    .as_ref()
                    .and_then(|item| item.source_span.clone()),
                diagnostic
                    .as_ref()
                    .map_or("clause", |item| item.path.as_str()),
                diagnostic.as_ref().map_or(
                    "bound oracle generation refused the selected package",
                    |item| item.message.as_str(),
                ),
            )
        }
        BoundGenerationError::ResourceLimitExceeded => bound_diagnostic(
            StrategyErrorCode::UnsupportedClause,
            GenerationTerminalState::Unsupported,
            Some(GenerationErrorCode::ResourceLimitExceeded),
            requested,
            None,
            "package",
            "bound oracle generation exceeded its package resource limit",
        ),
        BoundGenerationError::Bundle(_) => bound_diagnostic(
            StrategyErrorCode::UnsupportedClause,
            GenerationTerminalState::Inconclusive,
            None,
            requested,
            None,
            "package.bundle",
            "bound oracle generation could not assemble its complete artifact bundle",
        ),
    }
}

fn scope_diagnostic(mut diagnostic: StrategyDiagnostic, clause: &ClauseRef) -> StrategyDiagnostic {
    diagnostic.clause = Some(Box::new(clause.clone()));
    diagnostic
}

#[allow(clippy::too_many_arguments)]
fn bound_diagnostic(
    code: StrategyErrorCode,
    terminal_state: GenerationTerminalState,
    generation_code: Option<GenerationErrorCode>,
    clause: &ClauseRef,
    source_span: Option<quire_contract_ir::SourceSpan>,
    path: &str,
    message: &str,
) -> StrategyDiagnostic {
    StrategyDiagnostic {
        code,
        terminal_state,
        generation_code,
        clause: Some(Box::new(clause.clone())),
        source_span: source_span.map(Box::new),
        path: path.to_owned(),
        message: message.to_owned(),
    }
}

fn artifact(path: String, contents: String) -> Artifact {
    Artifact {
        path,
        sha256: sha256(contents.as_bytes()),
        contents,
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

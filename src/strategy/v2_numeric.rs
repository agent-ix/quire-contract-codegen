// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Complete V2 numeric source and map publication for one selected checked claim.

use quire_contract_model::{
    CheckedClauseComparison, CheckedComparisonRefusal, CheckedExecutionPointError,
    CheckedExecutionPointRefusal, CheckedNodeId, CheckedOccurrence, CheckedPackageV2,
    CheckedSemanticId, ExecutionPoint,
};
use serde::Serialize;

use crate::core::artifact::{Artifact, ArtifactBundle, MAX_GENERATED_SOURCE_BYTES};
use crate::strategy::bound::census::BoundaryCensus;
use crate::strategy::bound::population::Population;
use crate::strategy::campaign::StrategyErrorCode;

use super::v2_comparison::{
    admit_v2_integer_comparison, V2ComparisonDiagnostic, V2ComparisonRefusal,
};
use super::v2_population::{prepare_v2_numeric_population, V2PopulationDiagnostic};
use super::v2_selected_claim::{
    select_v2_claim_execution_point, V2SelectedClaimKey, V2SelectedClaimRequest,
};

/// Campaign floors and ceiling for one generated V2 numeric runner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct V2NumericPolicy {
    /// Required accepted verdicts.
    pub minimum_accepted_cases: u64,
    /// Required rejected preconditions.
    pub minimum_rejected_cases: u64,
    /// Maximum explicit framework discards in the complete supplied report.
    pub maximum_discarded_cases: u64,
}

/// One admitted V2 checked claim and requested population.
#[derive(Clone, Copy, Debug)]
pub struct V2NumericRequest<'a> {
    /// IR-admitted owner package.
    pub package: &'a CheckedPackageV2,
    /// Complete selected claim identity.
    pub key: V2SelectedClaimKey<'a>,
    /// Constructive side to generate.
    pub population: Population,
    /// Runtime campaign policy.
    pub policy: V2NumericPolicy,
    /// Separately read FR-331 process request owner, if this is a process request.
    /// Linked requests have no second owner field.
    pub process_package_id: Option<&'a CheckedSemanticId>,
}

/// Exact selected identity published in the source and map.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct V2NumericIdentity {
    /// Checked clause node selected for this runner.
    pub clause: CheckedNodeId,
    /// Node that owns the authentic claim.
    pub claim_owner: CheckedNodeId,
    /// Authentic claim occurrence.
    pub occurrence: CheckedOccurrence,
    /// Same-owner admitted package identity.
    pub package_id: CheckedSemanticId,
    /// Canonical qocc2 rendering used as runtime requirement identity.
    pub requirement: String,
    /// Canonical qpkg2 rendering used as runtime revision identity.
    pub revision: String,
    /// Exact point selected by IR and rendered for the runtime verdict.
    pub execution_point: String,
    /// Rust symbol identity suffix retaining both canonical renderings.
    pub symbol_suffix: String,
}

/// Complete atomic publication candidate; no campaign evidence is inferred from it.
#[derive(Clone, Debug)]
pub struct V2NumericBundle {
    /// Source and map published together or not at all.
    pub artifacts: ArtifactBundle,
    /// Exact identity that the source and map carry.
    pub identity: V2NumericIdentity,
    /// Exact IR projection used to emit the oracle and constructive census.
    pub comparison: CheckedClauseComparison,
    /// Deterministic in-domain and out-of-domain census of the selected relation.
    pub census: BoundaryCensus,
}

/// Stable refusal category of a V2 numeric strategy request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum V2NumericErrorCode {
    /// The selected checked clause does not exist.
    UnknownClause,
    /// The selected claim owner or occurrence is not authentic.
    MissingClaimOccurrence,
    /// The checked clause kind cannot drive a numeric campaign.
    UnsupportedClauseKind,
    /// IR refused the comparison shape before CG's relation check.
    ComparisonRefused,
    /// A finite read or literal cannot form an i64 relation.
    UnsupportedRelation,
    /// The authentic claim has no accepted point.
    MissingExecutionPoint,
    /// The requested population has no valuation on its side.
    EmptyPopulation,
    /// The selected campaign cannot satisfy its required population shape.
    UnsupportedCampaignConstraint,
    /// Process request package identity disagrees with the admitted package.
    OwnerIdentityMismatch,
    /// The selected point changed during rendering.
    ExecutionPointMismatch,
    /// The generated Rust or map is invalid.
    InvalidGeneratedSyntax,
    /// Generated source exceeded its finite byte ceiling.
    ResourceLimitExceeded,
    /// A different existing constructive-core refusal arose.
    PopulationRefused,
}

/// One typed no-artifact outcome retaining the selected key and reached cause.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct V2NumericDiagnostic {
    /// Selected checked clause.
    pub clause: CheckedNodeId,
    /// Supplied claim owner.
    pub claim_owner: CheckedNodeId,
    /// Supplied occurrence.
    pub occurrence: CheckedOccurrence,
    /// Stable refusal category.
    pub code: V2NumericErrorCode,
    /// Exact comparison refusal, when that boundary was reached.
    pub comparison: Option<V2ComparisonDiagnostic>,
    /// Exact point refusal, when that boundary was reached.
    pub point: Option<CheckedExecutionPointError>,
    /// Exact population refusal, when that boundary was reached.
    pub population: Option<V2PopulationDiagnostic>,
    /// Structural or generated artifact locus.
    pub path: String,
}

impl V2NumericDiagnostic {
    fn new(key: V2SelectedClaimKey<'_>, code: V2NumericErrorCode, path: &str) -> Self {
        Self {
            clause: key.clause.clone(),
            claim_owner: key.claim_owner.clone(),
            occurrence: key.occurrence.clone(),
            code,
            comparison: None,
            point: None,
            population: None,
            path: path.to_owned(),
        }
    }
}

fn map_comparison(
    key: V2SelectedClaimKey<'_>,
    comparison: V2ComparisonDiagnostic,
) -> V2NumericDiagnostic {
    let code = match &comparison.cause {
        V2ComparisonRefusal::Ir(error) => match error.kind {
            CheckedComparisonRefusal::UnknownClause => V2NumericErrorCode::UnknownClause,
            CheckedComparisonRefusal::MissingClaimOccurrence => {
                V2NumericErrorCode::MissingClaimOccurrence
            }
            CheckedComparisonRefusal::NotStateClause
            | CheckedComparisonRefusal::UnsupportedClauseKind => {
                V2NumericErrorCode::UnsupportedClauseKind
            }
            CheckedComparisonRefusal::MissingComparison
            | CheckedComparisonRefusal::UnknownOperator
            | CheckedComparisonRefusal::IneligibleOperator
            | CheckedComparisonRefusal::InvalidComparisonShape
            | CheckedComparisonRefusal::IneligibleOperand
            | CheckedComparisonRefusal::MissingChild
            | CheckedComparisonRefusal::NoRead
            | CheckedComparisonRefusal::MissingRange
            | CheckedComparisonRefusal::UnboundedRange
            | CheckedComparisonRefusal::RangeOutOfI128 => V2NumericErrorCode::ComparisonRefused,
        },
        V2ComparisonRefusal::WrongClaimOwner => V2NumericErrorCode::MissingClaimOccurrence,
        V2ComparisonRefusal::OutsideI64 | V2ComparisonRefusal::UnequalReadDomains => {
            V2NumericErrorCode::UnsupportedRelation
        }
        V2ComparisonRefusal::InvalidAccessorProjection => V2NumericErrorCode::ComparisonRefused,
    };
    let path = format!("{:?}", comparison.path);
    let mut result = V2NumericDiagnostic::new(key, code, &path);
    result.comparison = Some(comparison);
    result
}

fn map_point(
    key: V2SelectedClaimKey<'_>,
    point: CheckedExecutionPointError,
) -> V2NumericDiagnostic {
    let code = match point.code {
        CheckedExecutionPointRefusal::UnknownClause => V2NumericErrorCode::UnknownClause,
        CheckedExecutionPointRefusal::NotStateClause => V2NumericErrorCode::UnsupportedClauseKind,
        CheckedExecutionPointRefusal::MissingClaimOccurrence => {
            V2NumericErrorCode::MissingClaimOccurrence
        }
        CheckedExecutionPointRefusal::MissingExecutionPoint => {
            V2NumericErrorCode::MissingExecutionPoint
        }
    };
    let mut result = V2NumericDiagnostic::new(key, code, "clause.execution_point");
    result.point = Some(point);
    result
}

fn map_population(
    key: V2SelectedClaimKey<'_>,
    population: V2PopulationDiagnostic,
) -> V2NumericDiagnostic {
    let code = match population.code {
        StrategyErrorCode::EmptyPopulation => V2NumericErrorCode::EmptyPopulation,
        StrategyErrorCode::UnsupportedCampaignConstraint => {
            V2NumericErrorCode::UnsupportedCampaignConstraint
        }
        StrategyErrorCode::InvalidGeneratedSyntax => V2NumericErrorCode::InvalidGeneratedSyntax,
        StrategyErrorCode::ResourceLimitExceeded => V2NumericErrorCode::ResourceLimitExceeded,
        StrategyErrorCode::InvalidRange
        | StrategyErrorCode::InvalidMembership
        | StrategyErrorCode::CorrelationOverflow
        | StrategyErrorCode::InvalidStrategyIdentity
        | StrategyErrorCode::InvalidEnumIdentity
        | StrategyErrorCode::UnknownClause
        | StrategyErrorCode::UnsupportedClause
        | StrategyErrorCode::UnsupportedClauseKind
        | StrategyErrorCode::UnsupportedRelation => V2NumericErrorCode::PopulationRefused,
    };
    let path = population.path.clone();
    let mut result = V2NumericDiagnostic::new(key, code, &path);
    result.population = Some(population);
    result
}

fn hex_bytes(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn counted_hex(value: &str) -> String {
    format!("{}_{}", value.len(), hex_bytes(value))
}

fn render_requirement(key: V2SelectedClaimKey<'_>) -> String {
    // The only admitted role is Claim; the typed accessor refuses every other role.
    format!(
        "qocc2_{}_{}_{}",
        key.clause.digest,
        counted_hex("claim"),
        key.occurrence.ordinal
    )
}

fn render_revision(package_id: &CheckedSemanticId) -> String {
    format!(
        "qpkg2_{}_{}_{}",
        counted_hex(&package_id.domain),
        counted_hex(&package_id.algorithm),
        counted_hex(&package_id.digest)
    )
}

fn render_execution_point(point: &ExecutionPoint) -> String {
    let (kind, name) = match point {
        ExecutionPoint::Initialization { name } => ("initialization", name.as_str()),
        ExecutionPoint::Handler { name } => ("handler", name.as_str()),
        ExecutionPoint::Pre { operation } => ("pre", operation.as_str()),
        ExecutionPoint::Post { operation } => ("post", operation.as_str()),
    };
    format!("{kind}:{}:{name}", name.len())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SourceMap<'a> {
    identity: &'a V2NumericIdentity,
    comparison: &'a CheckedNodeId,
    source_path: &'a str,
    oracle_line: usize,
    population_line: usize,
    census_line: usize,
    runner_line: usize,
}

fn component(source: &str) -> String {
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

/// Generate the complete V2 numeric campaign source and map for one selected claim.
///
/// Every refusal occurs before construction of the atomic artifact bundle. Publishing
/// the result does not itself qualify coverage or prove the selected claim.
pub fn generate_v2_numeric_strategy(
    request: &V2NumericRequest<'_>,
) -> Result<V2NumericBundle, V2NumericDiagnostic> {
    generate_with_control(request, RenderControl::default())
}

#[derive(Default)]
struct RenderControl<'a> {
    point: Option<&'a str>,
    source_tail: Option<&'a str>,
    max_source_bytes: Option<usize>,
}

fn generate_with_control(
    request: &V2NumericRequest<'_>,
    control: RenderControl<'_>,
) -> Result<V2NumericBundle, V2NumericDiagnostic> {
    let key = request.key;
    let admitted = admit_v2_integer_comparison(request.package, key)
        .map_err(|cause| map_comparison(key, cause))?;
    let point = select_v2_claim_execution_point(V2SelectedClaimRequest {
        package: request.package,
        key,
    })
    .map_err(|diagnostic| map_point(key, diagnostic.refusal))?;
    let population = prepare_v2_numeric_population(&admitted, request.population)
        .map_err(|cause| map_population(key, cause))?;
    if request
        .process_package_id
        .is_some_and(|id| id != request.package.package_id())
    {
        return Err(V2NumericDiagnostic::new(
            key,
            V2NumericErrorCode::OwnerIdentityMismatch,
            "request.package_id",
        ));
    }
    let expected_point = render_execution_point(&point);
    let rendered_point = control.point.unwrap_or(&expected_point);
    if rendered_point != expected_point {
        return Err(V2NumericDiagnostic::new(
            key,
            V2NumericErrorCode::ExecutionPointMismatch,
            "generated.execution_point",
        ));
    }
    let requirement = render_requirement(key);
    let revision = render_revision(request.package.package_id());
    let identity = V2NumericIdentity {
        clause: key.clause.clone(),
        claim_owner: key.claim_owner.clone(),
        occurrence: key.occurrence.clone(),
        package_id: request.package.package_id().clone(),
        symbol_suffix: format!("cg_{requirement}_{revision}"),
        requirement,
        revision,
        execution_point: rendered_point.to_owned(),
    };
    let source_path = "src/generated/v2_numeric.rs";
    let mut source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
         // @generated by quire-contract-codegen. Do not edit.\n\
         // Requirement: {}; Revision: {}; Clause: {}\n\
         // Symbol: {}\n\
         #![deny(missing_docs)]\n\
         //! Selected V2 numeric oracle-conformance campaign.\n\n",
        identity.requirement, identity.revision, identity.clause.digest, identity.symbol_suffix,
    );
    let oracle_line = source.lines().count() + 1;
    source.push_str(&admitted.oracle.source);
    source.push('\n');
    let population_line = source.lines().count() + 1;
    source.push_str(&component(&population.population.source));
    source.push('\n');
    let census_line = source.lines().count() + 1;
    source.push_str(&population.census_source);
    source.push('\n');
    let runner_line = source.lines().count() + 1;
    let runner = render_runner(&identity, &admitted.projection.kind, &population, request);
    source.push_str(runner.trim_start_matches('\n'));
    if let Some(tail) = control.source_tail {
        source.push_str(tail);
    }
    syn::parse_file(&source).map_err(|_| {
        V2NumericDiagnostic::new(key, V2NumericErrorCode::InvalidGeneratedSyntax, source_path)
    })?;
    if source.len()
        > control
            .max_source_bytes
            .unwrap_or(MAX_GENERATED_SOURCE_BYTES)
    {
        return Err(V2NumericDiagnostic::new(
            key,
            V2NumericErrorCode::ResourceLimitExceeded,
            source_path,
        ));
    }
    let map = SourceMap {
        identity: &identity,
        comparison: &admitted.projection.comparison,
        source_path,
        oracle_line,
        population_line,
        census_line,
        runner_line,
    };
    let map = serde_json::to_string_pretty(&map).map_err(|_| {
        V2NumericDiagnostic::new(
            key,
            V2NumericErrorCode::InvalidGeneratedSyntax,
            "maps/v2_numeric.json",
        )
    })?;
    let artifacts = ArtifactBundle::new(vec![
        Artifact::new(source_path, source),
        Artifact::new("maps/v2_numeric.json", map),
    ])
    .map_err(|_| {
        V2NumericDiagnostic::new(key, V2NumericErrorCode::ResourceLimitExceeded, "bundle")
    })?;
    Ok(V2NumericBundle {
        artifacts,
        identity,
        comparison: admitted.projection,
        census: population.census,
    })
}

fn render_runner(
    identity: &V2NumericIdentity,
    kind: &quire_contract_model::CheckedComparisonClauseKind,
    population: &super::v2_population::V2NumericPopulation,
    request: &V2NumericRequest<'_>,
) -> String {
    let (runtime_kind, failed_kind, failure_kind, outcome) = match kind {
        quire_contract_model::CheckedComparisonClauseKind::Precondition => (
            "Precondition",
            "RejectedPrecondition",
            "Precondition",
            "Rejected",
        ),
        quire_contract_model::CheckedComparisonClauseKind::Postcondition => (
            "Postcondition",
            "FailedPostcondition",
            "Postcondition",
            "Failed",
        ),
        quire_contract_model::CheckedComparisonClauseKind::Invariant => {
            ("Invariant", "FailedPostcondition", "Contract", "Failed")
        }
        quire_contract_model::CheckedComparisonClauseKind::Constraint => {
            ("Invariant", "FailedPostcondition", "Contract", "Failed")
        }
    };
    let case_type = &population.population.case_type;
    let expectation = &population.population.expectation_type;
    let strategy = &population.population.strategy_function;
    let partner_case = if population.census.relation().has_partner_read() {
        "Some(case.partner)"
    } else {
        "None"
    };
    let partner_census = if population.census.relation().has_partner_read() {
        ", partner: census.partner"
    } else {
        ""
    };
    let partner_error = if population.census.relation().has_partner_read() {
        "Some(case.partner)"
    } else {
        "None"
    };
    let failure_field = if *kind == quire_contract_model::CheckedComparisonClauseKind::Precondition
    {
        "rejection"
    } else {
        "failure"
    };
    let minimum_accepted = request.policy.minimum_accepted_cases;
    let minimum_rejected = request.policy.minimum_rejected_cases;
    let maximum_discarded = request.policy.maximum_discarded_cases;
    format!(
        r#"
/// Exact selected V2 requirement and admitted package revision.
pub const V2_NUMERIC_IDENTITY: quire_contract_runtime::ContractIdentity<'static> =
    quire_contract_runtime::ContractIdentity::new(
        quire_contract_runtime::RequirementId::new({requirement:?}),
        quire_contract_runtime::RevisionId::new({revision:?}),
    );
/// The selected checked clause, independent of its occurrence key.
pub const V2_NUMERIC_CLAUSE: quire_contract_runtime::ClauseId<'static> =
    quire_contract_runtime::ClauseId::new({clause:?});
/// The exact point retained by Contract IR for this selected claim.
pub const V2_NUMERIC_POINT: quire_contract_runtime::ExecutionPoint<'static> =
    quire_contract_runtime::ExecutionPoint::new({point:?});

/// Complete exact accounting from one selected V2 campaign.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct V2NumericSummary {{
    /// Oracle evaluations, including shrink replays.
    pub attempted: u64,
    /// Passed or failed-postcondition verdicts.
    pub accepted: u64,
    /// Rejected preconditions.
    pub rejected: u64,
    /// Failed postconditions.
    pub failed: u64,
    /// Explicit framework discards.
    pub discarded: u64,
    at_limit: bool,
}}

impl V2NumericSummary {{
    /// Exact `(discarded, attempted)`, absent for zero or saturated totals.
    pub const fn discard_rate(self) -> Option<(u64, u64)> {{
        if self.attempted == 0 || self.at_limit {{ None }}
        else {{ Some((self.discarded, self.attempted)) }}
    }}
    /// Exact `(rejected, attempted)`, absent for zero or saturated totals.
    pub const fn rejection_rate(self) -> Option<(u64, u64)> {{
        if self.attempted == 0 || self.at_limit {{ None }}
        else {{ Some((self.rejected, self.attempted)) }}
    }}
}}

/// Typed terminal outcome of this generated oracle-conformance campaign.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum V2NumericCampaignError {{
    /// A case shape does not match the selected relation.
    InvalidCaseShape {{ /// Retained accounting.
        summary: V2NumericSummary }},
    /// The supplied report names a different checked claim or package.
    IdentityMismatch {{ /// Retained accounting.
        summary: V2NumericSummary }},
    /// The checked oracle disagreed with an immutable generated tag.
    ConformanceMismatch {{
        /// Retained accounting.
        summary: V2NumericSummary,
        /// Primary read.
        primary: i64,
        /// Partner read when selected.
        partner: Option<i64>,
        /// Immutable expected relation side.
        expected: {expectation},
        /// Actual runtime verdict kind.
        observed: quire_contract_runtime::VerdictKind,
    }},
    /// Required accepted verdict floor was not met.
    BelowAcceptedFloor {{ /// Retained accounting.
        summary: V2NumericSummary }},
    /// Required precondition rejection floor was not met.
    BelowRejectedFloor {{ /// Retained accounting.
        summary: V2NumericSummary }},
    /// The complete supplied report exceeded its discard ceiling.
    AboveDiscardCeiling {{ /// Retained accounting.
        summary: V2NumericSummary }},
    /// Proptest exhausted its configured search.
    Exhausted {{ /// Retained accounting.
        summary: V2NumericSummary, /// Framework detail.
        reason: String }},
}}

/// Read exact rate-bearing counters from a runtime snapshot.
pub fn v2_numeric_summary_from_snapshot(
    snapshot: &quire_contract_runtime::CampaignSnapshot<'_>,
) -> V2NumericSummary {{
    let counts = snapshot.counts();
    V2NumericSummary {{
        attempted: counts.total(), accepted: counts.accepted(),
        rejected: counts.rejected(), failed: counts.failed(),
        discarded: counts.discarded(), at_limit: snapshot.at_limit(),
    }}
}}

fn v2_numeric_summary(report: &quire_contract_runtime::CampaignReport<'static>) -> V2NumericSummary {{
    v2_numeric_summary_from_snapshot(&report.snapshot())
}}

fn v2_numeric_conclude(
    report: &quire_contract_runtime::CampaignReport<'static>,
    framework_error: Option<String>,
) -> Result<V2NumericSummary, V2NumericCampaignError> {{
    let summary = v2_numeric_summary(report);
    if summary.discarded > {maximum_discarded} {{
        return Err(V2NumericCampaignError::AboveDiscardCeiling {{ summary }});
    }}
    if let Some(reason) = framework_error {{
        return Err(V2NumericCampaignError::Exhausted {{ summary, reason }});
    }}
    if summary.accepted < {minimum_accepted} {{
        return Err(V2NumericCampaignError::BelowAcceptedFloor {{ summary }});
    }}
    if summary.rejected < {minimum_rejected} {{
        return Err(V2NumericCampaignError::BelowRejectedFloor {{ summary }});
    }}
    Ok(summary)
}}

fn v2_numeric_evaluate(
    case: &{case_type},
    report: &mut quire_contract_runtime::CampaignReport<'static>,
) -> Result<(), V2NumericCampaignError> {{
    let Some(holds) = oracle_numeric(case.primary, {partner_case}) else {{
        return Err(V2NumericCampaignError::InvalidCaseShape {{ summary: v2_numeric_summary(report) }});
    }};
    let failure = quire_contract_runtime::FailureDetail::new(
        V2_NUMERIC_CLAUSE, quire_contract_runtime::FailureKind::{failure_kind}, 0, None,
    );
    let outcome = if holds {{ quire_contract_runtime::ClauseOutcome::Passed }}
        else {{ quire_contract_runtime::ClauseOutcome::{outcome} }};
    let observations = [quire_contract_runtime::Observation::new(
        V2_NUMERIC_CLAUSE,
        quire_contract_runtime::ClauseKind::{runtime_kind},
        outcome,
        if holds {{ None }} else {{ Some(failure) }},
    )];
    let context = quire_contract_runtime::VerdictContext::new(
        V2_NUMERIC_IDENTITY, V2_NUMERIC_POINT, &observations,
    );
    let verdict = if holds {{ quire_contract_runtime::Verdict::passed(context) }}
        else {{ quire_contract_runtime::Verdict::{failed_kind} {{ context, {failure_field}: failure }} }};
    let observed = verdict.kind();
    if report.record_verdict(&verdict).is_err() {{
        return Err(V2NumericCampaignError::IdentityMismatch {{ summary: v2_numeric_summary(report) }});
    }}
    let expected = if matches!(case.expected, {expectation}::Holds) {{
        quire_contract_runtime::VerdictKind::Passed
    }} else {{
        quire_contract_runtime::VerdictKind::{failed_kind}
    }};
    if observed != expected {{
        return Err(V2NumericCampaignError::ConformanceMismatch {{
            summary: v2_numeric_summary(report), primary: case.primary,
            partner: {partner_error}, expected: case.expected, observed,
        }});
    }}
    Ok(())
}}

/// Run the selected constructive population with no subject or global rejects.
pub fn run_v2_numeric_population(
    runner: &mut proptest::test_runner::TestRunner,
    report: &mut quire_contract_runtime::CampaignReport<'static>,
) -> Result<V2NumericSummary, V2NumericCampaignError> {{
    let strategy = {strategy}();
    let report = core::cell::RefCell::new(report);
    let failure = core::cell::RefCell::new(None);
    let result = runner.run(&strategy, |case| {{
        match v2_numeric_evaluate(&case, &mut report.borrow_mut()) {{
            Ok(()) => Ok(()),
            Err(error) => {{
                *failure.borrow_mut() = Some(error);
                Err(proptest::test_runner::TestCaseError::fail("V2 oracle conformance mismatch"))
            }}
        }}
    }});
    let report = report.into_inner();
    if let Some(error) = failure.into_inner() {{
        let summary = v2_numeric_summary(report);
        if summary.discarded > {maximum_discarded} {{
            return Err(V2NumericCampaignError::AboveDiscardCeiling {{ summary }});
        }}
        return Err(match error {{
            V2NumericCampaignError::ConformanceMismatch {{ primary, partner, expected, observed, .. }} =>
                V2NumericCampaignError::ConformanceMismatch {{ summary, primary, partner, expected, observed }},
            V2NumericCampaignError::IdentityMismatch {{ .. }} =>
                V2NumericCampaignError::IdentityMismatch {{ summary }},
            V2NumericCampaignError::InvalidCaseShape {{ .. }} =>
                V2NumericCampaignError::InvalidCaseShape {{ summary }},
            other => other,
        }});
    }}
    match result {{
        Ok(()) => v2_numeric_conclude(report, None),
        Err(proptest::test_runner::TestError::Abort(reason)) |
        Err(proptest::test_runner::TestError::Fail(reason, _)) =>
            v2_numeric_conclude(report, Some(reason.to_string())),
    }}
}}

/// Evaluate every tagged in-domain census case exactly once in census order.
pub fn run_v2_numeric_census(
    report: &mut quire_contract_runtime::CampaignReport<'static>,
) -> Result<V2NumericSummary, V2NumericCampaignError> {{
    for census in &IN_DOMAIN_CENSUS {{
        let case = {case_type} {{
            primary: census.primary{partner_census},
            expected: match census.tag {{
                CensusTag::Holds => {expectation}::Holds,
                CensusTag::Violated => {expectation}::Violated,
            }},
        }};
        v2_numeric_evaluate(&case, report)?;
    }}
    v2_numeric_conclude(report, None)
}}
"#,
        requirement = identity.requirement,
        revision = identity.revision,
        clause = identity.clause.digest,
        point = identity.execution_point,
    )
}

#[cfg(test)]
#[path = "../../tests/state_frame_support/model.rs"]
mod model;
#[cfg(test)]
#[path = "../../tests/state_frame_support/native_twin.rs"]
#[allow(clippy::duplicate_mod)]
mod native_twin;

#[cfg(test)]
mod tests {

    use quire_contract_model::{CheckedOccurrenceRole, CheckedPackageV2};

    use super::{
        generate_with_control, native_twin, RenderControl, V2NumericErrorCode, V2NumericPolicy,
        V2NumericRequest, V2SelectedClaimKey,
    };
    use crate::strategy::bound::population::Population;

    fn fixture() -> (CheckedPackageV2, quire_contract_model::CheckedNodeId) {
        native_twin::Twin::new().emitted_package("BalanceNeverDrops")
    }

    /// Trace: TC-020
    #[test]
    fn renderer_mutants_refuse_before_any_artifact_exists() {
        let (package, clause) = fixture();
        let claim = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == clause)
            .unwrap()
            .occurrences
            .iter()
            .find(|occurrence| occurrence.role == CheckedOccurrenceRole::Claim)
            .unwrap();
        let request = V2NumericRequest {
            package: &package,
            key: V2SelectedClaimKey {
                clause: &clause,
                claim_owner: &clause,
                occurrence: claim,
            },
            population: Population::Broad,
            policy: V2NumericPolicy {
                minimum_accepted_cases: 0,
                minimum_rejected_cases: 0,
                maximum_discarded_cases: 0,
            },
            process_package_id: None,
        };
        for (control, expected) in [
            (
                RenderControl {
                    point: Some("handler:6:mutant"),
                    ..RenderControl::default()
                },
                V2NumericErrorCode::ExecutionPointMismatch,
            ),
            (
                RenderControl {
                    source_tail: Some("\nthis is not Rust;\n"),
                    ..RenderControl::default()
                },
                V2NumericErrorCode::InvalidGeneratedSyntax,
            ),
            (
                RenderControl {
                    max_source_bytes: Some(1),
                    ..RenderControl::default()
                },
                V2NumericErrorCode::ResourceLimitExceeded,
            ),
            (
                RenderControl {
                    source_tail: Some("\nthis is not Rust;\n"),
                    max_source_bytes: Some(1),
                    ..RenderControl::default()
                },
                V2NumericErrorCode::InvalidGeneratedSyntax,
            ),
        ] {
            let diagnostic = generate_with_control(&request, control).expect_err("mutant refusal");
            assert_eq!(diagnostic.code, expected);
            assert_eq!(diagnostic.clause, clause);
            assert_eq!(diagnostic.occurrence, *claim);
        }
    }
}

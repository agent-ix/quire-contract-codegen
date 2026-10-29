// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! V2 constructive populations and complete census from one admitted typed relation.

use quire_contract_model::{CheckedNodeId, CheckedOccurrence};

use super::bound::census::{
    compute_census, render_v2_census_constants, BoundaryCensus, CensusNames,
};
use super::bound::population::{
    render_v2_population, Population, PopulationRequest, RenderedPopulation,
};
use super::campaign::{StrategyDiagnostic, StrategyErrorCode};
use super::v2_comparison::V2ComparisonAdmission;

/// Constructed source and exact domain-edge census for a selected V2 claim.
#[derive(Clone, Debug)]
pub struct V2NumericPopulation {
    /// Source and names of the directly constructed population.
    pub population: RenderedPopulation,
    /// Source of the complete tagged in-domain and untagged edge arrays.
    pub census_source: String,
    /// Independently inspectable deterministic census values.
    pub census: BoundaryCensus,
}

/// A typed V2 population refusal retaining the selected checked claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct V2PopulationDiagnostic {
    /// Selected clause.
    pub clause: CheckedNodeId,
    /// Supplied claim owner.
    pub claim_owner: CheckedNodeId,
    /// Supplied authentic occurrence.
    pub occurrence: CheckedOccurrence,
    /// Stable population or census refusal kind.
    pub code: StrategyErrorCode,
    /// Structural path reached by the constructive core.
    pub path: String,
    /// Human-readable detail, never used as a discriminant.
    pub message: String,
}

fn diagnostic(
    admission: &V2ComparisonAdmission,
    source: StrategyDiagnostic,
) -> V2PopulationDiagnostic {
    V2PopulationDiagnostic {
        clause: admission.clause.clone(),
        claim_owner: admission.claim_owner.clone(),
        occurrence: admission.occurrence.clone(),
        code: source.code,
        path: source.path,
        message: source.message,
    }
}

/// Construct a population and full in-domain census from the same admitted V2 comparison.
///
/// The relation, domain and selected claim are owned by `admission`. The renderer is the
/// existing IR-independent constructive core, with V2 names and no V1 bound request.
// Implements: FR-009, FR-010
pub fn prepare_v2_numeric_population(
    admission: &V2ComparisonAdmission,
    requested: Population,
) -> Result<V2NumericPopulation, V2PopulationDiagnostic> {
    let fields: &[&str] = if admission.relation.has_partner_read() {
        &["primary", "partner"]
    } else {
        &["primary"]
    };
    let population = render_v2_population(&PopulationRequest {
        relation: admission.relation,
        domain: admission.domain,
        population: requested,
        read_identifiers: fields,
    })
    .map_err(|source| diagnostic(admission, source))?;
    let census = compute_census(admission.relation, admission.domain)
        .map_err(|source| diagnostic(admission, source))?;
    let census_source = render_v2_census_constants(
        &census,
        CensusNames {
            item_suffix: "",
            fields,
        },
    )
    .map_err(|source| diagnostic(admission, source))?;
    Ok(V2NumericPopulation {
        population,
        census_source,
        census,
    })
}

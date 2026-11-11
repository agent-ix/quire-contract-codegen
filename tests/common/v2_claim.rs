//! A matching V2 clause claim for tests that do not exercise FR-015-AC-48 themselves.

use qsl_replay::ObligationIdentity;
use quire_contract_codegen::{
    negotiate_kani_obligations, KaniObligationRequest, ObligationDisposition, ObligationItem,
    ObligationKind, RequirementOccurrence, UnsupportedObligation, V2ClauseObligationClaim,
    V2ClauseObligationRecord,
};
use quire_contract_model::{CheckedNodeId, CheckedNodePathSegment, CheckedPackageV2, CheckedTerm};
use quire_exact::{NodeKey, PathSegment};

fn node_key(id: &CheckedNodeId) -> NodeKey {
    let path = id
        .path
        .iter()
        .map(|segment| match segment {
            CheckedNodePathSegment::Name(name) => PathSegment::Text(name.to_string()),
            CheckedNodePathSegment::Node(child) => PathSegment::Node(node_key(child)),
            other => PathSegment::Text(format!("{other:?}")),
        })
        .collect();
    NodeKey::new(id.package.to_string(), path, id.ordinal).expect("admitted structural key")
}

/// The clause's claim at ordinal 0, probed alone.
pub fn claim_for(package: &CheckedPackageV2, clause: &CheckedNodeId) -> V2ClauseObligationClaim {
    claims_for(package, std::slice::from_ref(clause)).remove(0)
}

/// Claims at ordinal 0 for `clauses`: each one's true occurrence and kind, with parameters and
/// state reads read back from the generator's refusal when it names them. The clauses are probed
/// in one request so that a postcondition sees its sibling preconditions.
pub fn claims_for(
    package: &CheckedPackageV2,
    clauses: &[CheckedNodeId],
) -> Vec<V2ClauseObligationClaim> {
    let mut claims = clauses
        .iter()
        .map(|clause| bare_claim(package, clause))
        .collect::<Vec<_>>();
    // A refused sibling masks its postcondition's own refusal, so settle to a fixed point.
    for _ in 0..4 {
        let items = clauses
            .iter()
            .zip(&claims)
            .map(|(clause, claim)| ObligationItem::V2Clause {
                package,
                clause,
                claim,
                dependencies: None,
            })
            .collect::<Vec<_>>();
        let outcome = negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: super::proof_ceilings::proof_ceilings(),
            items: &items,
            subject_path: "crate::request::unrelated",
            unwind: 8,
        });
        let selected = outcome.ok().map(|outcome| {
            outcome
                .records()
                .iter()
                .map(|record| match &record.disposition {
                    ObligationDisposition::Unsupported {
                        reason:
                            UnsupportedObligation::V2ClauseClaimMismatch {
                                selected: Some(selected),
                                ..
                            },
                    } => Some(selected.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
        });
        for (claim, selected) in claims.iter_mut().zip(selected.into_iter().flatten()) {
            if let Some(selected) = selected {
                claim.record.parameters = selected.parameters;
                claim.record.state_reads = selected.state_reads;
            }
        }
    }
    claims
}

fn bare_claim(package: &CheckedPackageV2, clause: &CheckedNodeId) -> V2ClauseObligationClaim {
    let entry = package
        .source_map()
        .iter()
        .find(|entry| &entry.node_id == clause);
    let occurrence = entry.map_or_else(
        || RequirementOccurrence {
            node_id: clause.clone(),
            role: quire_contract_model::CheckedOccurrenceRole::Expression,
            ordinal: 0,
        },
        |entry| RequirementOccurrence {
            node_id: entry.node_id.clone(),
            role: entry.role.clone(),
            ordinal: entry.ordinal,
        },
    );
    let kind = match package.term(clause) {
        Some(CheckedTerm::Application { operation, .. }) => {
            match operation.state_clause_kind().ok().flatten() {
                Some(quire_contract_model::StateClauseKind::Postcondition) => {
                    ObligationKind::Postcondition
                }
                Some(quire_contract_model::StateClauseKind::Invariant) => ObligationKind::Invariant,
                _ => ObligationKind::Precondition,
            }
        }
        _ => ObligationKind::Precondition,
    };
    V2ClauseObligationClaim {
        obligation: ObligationIdentity::new(node_key(clause), None, 0),
        record: V2ClauseObligationRecord {
            occurrence,
            kind,
            parameters: Vec::new(),
            state_reads: Vec::new(),
        },
    }
}

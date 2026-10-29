// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Direct integer comparison admission from Contract IR's clause-context accessor (FR-008).

use quire_contract_model::{
    CheckedClauseComparison, CheckedComparisonError, CheckedComparisonOperandKind,
    CheckedComparisonOperator, CheckedComparisonPath, CheckedComparisonRead, CheckedNodeId,
    CheckedOccurrence, CheckedPackageV2,
};

use super::bound::relation::{ComparisonOperator, Domain, OperandPosition, Relation};
use super::v2_selected_claim::V2SelectedClaimKey;

/// A direct comparison and its exact checked source context, ready for population generation.
#[derive(Clone, Debug)]
pub struct V2ComparisonAdmission {
    /// Complete selected clause identity.
    pub clause: CheckedNodeId,
    /// Node carrying the authentic claim.
    pub claim_owner: CheckedNodeId,
    /// Authentic selected claim occurrence.
    pub occurrence: CheckedOccurrence,
    /// Exact source projection, including operand provenance and authored order.
    pub projection: CheckedClauseComparison,
    /// One-domain relation with authored operand order retained.
    pub relation: Relation,
    /// Inclusive domain common to every read.
    pub domain: Domain,
    /// Source compiled only from the typed projection; accepts one or two read values.
    pub oracle: V2IntegerOracle,
}

/// Source of the selected V2 integer-comparison oracle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct V2IntegerOracle {
    /// Standalone Rust function `oracle_numeric(primary, partner)` returning `Option<bool>`.
    pub source: String,
}

/// Stable cause of a selected V2 comparison refusal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum V2ComparisonRefusal {
    /// Contract IR refused before CG's relation admission.
    Ir(CheckedComparisonError),
    /// A different node was supplied as claim owner.
    WrongClaimOwner,
    /// An exact endpoint or literal cannot be represented as i64.
    OutsideI64,
    /// Two finite reads have unequal domains.
    UnequalReadDomains,
    /// IR returned a direct comparison without the read it promises.
    InvalidAccessorProjection,
}

/// One typed refusal retaining the complete selected key and reached operand locus.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct V2ComparisonDiagnostic {
    /// Selected checked clause.
    pub clause: CheckedNodeId,
    /// Supplied claim owner.
    pub claim_owner: CheckedNodeId,
    /// Supplied occurrence.
    pub occurrence: CheckedOccurrence,
    /// First refusal.
    pub cause: V2ComparisonRefusal,
    /// Structural locus when reached.
    pub path: CheckedComparisonPath,
    /// Last reached graph child, if IR supplied one.
    pub child: Option<CheckedNodeId>,
}

fn diagnostic(
    key: V2SelectedClaimKey<'_>,
    cause: V2ComparisonRefusal,
    path: CheckedComparisonPath,
    child: Option<CheckedNodeId>,
) -> V2ComparisonDiagnostic {
    V2ComparisonDiagnostic {
        clause: key.clause.clone(),
        claim_owner: key.claim_owner.clone(),
        occurrence: key.occurrence.clone(),
        cause,
        path,
        child,
    }
}

fn operator(operator: CheckedComparisonOperator) -> ComparisonOperator {
    match operator {
        CheckedComparisonOperator::Eq => ComparisonOperator::Equal,
        CheckedComparisonOperator::Ne => ComparisonOperator::NotEqual,
        CheckedComparisonOperator::Lt => ComparisonOperator::Less,
        CheckedComparisonOperator::Le => ComparisonOperator::LessEqual,
        CheckedComparisonOperator::Gt => ComparisonOperator::Greater,
        CheckedComparisonOperator::Ge => ComparisonOperator::GreaterEqual,
    }
}

fn operand_value(
    key: V2SelectedClaimKey<'_>,
    projection: &CheckedClauseComparison,
    ordinal: usize,
) -> Result<(Option<CheckedComparisonRead>, i64, i64), V2ComparisonDiagnostic> {
    let operand = &projection.operands[ordinal];
    let path = CheckedComparisonPath::Operand(ordinal as u8);
    let convert = |value| {
        i64::try_from(value).map_err(|_| {
            diagnostic(
                key,
                V2ComparisonRefusal::OutsideI64,
                path,
                match &operand.identity {
                    quire_contract_model::CheckedComparisonOperandIdentity::GraphChild(id) => {
                        Some(id.clone())
                    }
                    quire_contract_model::CheckedComparisonOperandIdentity::InlineLiteral {
                        ..
                    } => None,
                },
            )
        })
    };
    Ok((
        match &operand.kind {
            CheckedComparisonOperandKind::Read { provenance, .. } => Some(provenance.clone()),
            CheckedComparisonOperandKind::Literal => None,
        },
        convert(operand.range.lower)?,
        convert(operand.range.upper)?,
    ))
}

/// Admit one selected claim through IR's typed clause-context comparison accessor.
///
/// IR's refusal is returned before CG checks i64 representability or common read domains.
/// No V1 projection, private node body, or caller-supplied operand is inspected.
// Implements: FR-008-AC-1, FR-008-AC-2, FR-008-AC-4, FR-008-AC-6
pub fn admit_v2_integer_comparison(
    package: &CheckedPackageV2,
    key: V2SelectedClaimKey<'_>,
) -> Result<V2ComparisonAdmission, V2ComparisonDiagnostic> {
    let projection = package
        .direct_integer_comparison(key.clause, key.occurrence)
        .map_err(|error| {
            diagnostic(
                key,
                V2ComparisonRefusal::Ir(error.clone()),
                error.path,
                error.child,
            )
        })?;
    if key.claim_owner != key.clause {
        return Err(diagnostic(
            key,
            V2ComparisonRefusal::WrongClaimOwner,
            CheckedComparisonPath::Clause,
            Some(key.claim_owner.clone()),
        ));
    }
    let left = operand_value(key, &projection, 0)?;
    let right = operand_value(key, &projection, 1)?;
    let (primary, partner, position) = match (&left.0, &right.0) {
        (Some(_), Some(_)) => (&left, &right, OperandPosition::Left),
        (Some(_), None) => (&left, &right, OperandPosition::Left),
        (None, Some(_)) => (&right, &left, OperandPosition::Right),
        (None, None) => {
            return Err(diagnostic(
                key,
                V2ComparisonRefusal::InvalidAccessorProjection,
                CheckedComparisonPath::Comparison,
                Some(projection.comparison.clone()),
            ));
        }
    };
    let domain = Domain {
        minimum: primary.1,
        maximum: primary.2,
    };
    let op = operator(projection.operator);
    let relation = if partner.0.is_some() {
        if (primary.1, primary.2) != (partner.1, partner.2) {
            return Err(diagnostic(
                key,
                V2ComparisonRefusal::UnequalReadDomains,
                CheckedComparisonPath::Operand(1),
                None,
            ));
        }
        Relation::between_reads(op)
    } else {
        Relation::with_literal(op, position, partner.1)
    };
    let oracle = V2IntegerOracle {
        source: crate::oracle::integer_comparison::lower_typed_integer_comparison(
            &projection,
            relation,
        ),
    };
    Ok(V2ComparisonAdmission {
        clause: key.clause.clone(),
        claim_owner: key.claim_owner.clone(),
        occurrence: key.occurrence.clone(),
        projection,
        relation,
        domain,
        oracle,
    })
}

// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Source lowering for the six closed direct integer comparisons.

use quire_contract_model::{CheckedClauseComparison, CheckedComparisonOperator};

use crate::strategy::bound::relation::{OperandPosition, Partner, Relation};

/// Render the selected checked comparison without reading its node body.
///
/// The relation is admitted from this same projection before the call. The generated
/// function accepts the primary read and an optional partner read; absence/presence
/// disagreeing with the admitted shape returns `None`, never a false oracle verdict.
pub(crate) fn lower_typed_integer_comparison(
    projection: &CheckedClauseComparison,
    relation: Relation,
) -> String {
    let symbol = match projection.operator {
        CheckedComparisonOperator::Eq => "==",
        CheckedComparisonOperator::Ne => "!=",
        CheckedComparisonOperator::Lt => "<",
        CheckedComparisonOperator::Le => "<=",
        CheckedComparisonOperator::Gt => ">",
        CheckedComparisonOperator::Ge => ">=",
    };
    let other = match relation.partner() {
        Partner::Read => "let Some(other) = partner else { return None; };".to_owned(),
        Partner::Literal(value) => {
            format!("if partner.is_some() {{ return None; }} let other: i64 = {value};")
        }
    };
    let (left, right) = match relation.primary() {
        OperandPosition::Left => ("primary", "other"),
        OperandPosition::Right => ("other", "primary"),
    };
    format!(
        "/// Evaluates the selected checked integer comparison.\n\
         pub fn oracle_numeric(primary: i64, partner: Option<i64>) -> Option<bool> {{\n\
             {other}\n\
             Some({left} {symbol} {right})\n\
         }}\n"
    )
}

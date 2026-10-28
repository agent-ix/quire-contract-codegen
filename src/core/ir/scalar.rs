// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Typed scalar access to admitted application operations and node shape.

use quire_contract_model::{
    CheckedApplicationOperation, CheckedNodeId, CheckedTerm, OperationModeKind,
};

use super::frame::application_identity;

fn operation(term: Option<&CheckedTerm>) -> Option<&CheckedApplicationOperation> {
    match term? {
        CheckedTerm::Application { operation, .. } => Some(operation),
        _ => None,
    }
}

/// The admitted application's catalog identity, if present.
pub(crate) fn identity(term: Option<&CheckedTerm>) -> Option<&str> {
    application_identity(term?)
}

/// The value of an operation mode with the requested kind.
pub(crate) fn mode<'a>(term: Option<&'a CheckedTerm>, kind: &str) -> Option<&'a str> {
    let mode = operation(term)?.mode()?;
    (mode.kind == OperationModeKind::from_wire(kind)?).then_some(mode.value)
}

/// Whether an operation names the selected law definition.
pub(crate) fn has_law(term: Option<&CheckedTerm>, identity: &str) -> bool {
    operation(term)
        .map(CheckedApplicationOperation::law_definitions)
        .into_iter()
        .flatten()
        .any(|definition| definition.identity.as_ref() == identity)
}

/// The declaration named by a quantity conversion operation.
pub(crate) fn quantity_conversion_target(
    term: Option<&CheckedTerm>,
    conversion_identity: &str,
) -> Option<CheckedNodeId> {
    let operation = operation(term)?;
    (operation.identity()? == conversion_identity)
        .then(|| operation.member_declaration().cloned())?
}

#[cfg(test)]
mod tests {
    /// Trace: AD-004 L-8.
    #[test]
    fn scalar_oracle_reads_operation_fields_through_core_ir() {
        let derive = include_str!("../../oracle/scalar/derive.rs");
        let lower = include_str!("../../oracle/scalar/lower.rs");
        for source in [derive, lower] {
            let compact: String = source
                .chars()
                .filter(|character| !character.is_whitespace())
                .collect();
            assert!(!compact.contains(".get(\""));
            assert!(!compact.contains(".operation("));
            assert!(!compact.contains("[\""));
            assert!(!compact.contains("serde_json::from_value"));
            assert!(!compact.contains("Value::"));
            assert!(!compact.contains("Application{operation"));
            // The V1 graph's generic node-body type is still Value; no scalar
            // operation payload is read through it in these consumer modules.
            assert_eq!(compact.matches("useserde_json::Value;").count(), 1);
            assert_eq!(compact.matches("serde_json::Value").count(), 1);
        }
        assert!(derive.contains("scalar_ir::identity("));
        assert!(lower.contains("scalar_ir::identity("));
    }
}

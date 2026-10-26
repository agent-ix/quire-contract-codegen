// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Typed scalar access to admitted application operations and node shape.

use quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2, CheckedTerm};

use super::checked_term::node_id;

/// The admitted node's scalar classification fields.
pub(crate) struct NodeShape<'a> {
    pub(crate) tag: &'a str,
    pub(crate) form: &'a str,
}

pub(crate) fn shape<Body>(node: &CheckedSemanticNodeV2<Body>) -> NodeShape<'_> {
    NodeShape {
        tag: &node.node_tag,
        form: &node.semantic_form,
    }
}

fn operation(term: Option<&CheckedTerm>) -> Option<&serde_json::Value> {
    match term? {
        CheckedTerm::Application { operation, .. } => Some(operation),
        _ => None,
    }
}

/// The admitted application's catalog identity, if present.
pub(crate) fn identity(term: Option<&CheckedTerm>) -> Option<&str> {
    operation(term)?.get("identity")?.as_str()
}

/// The value of an operation mode with the requested kind.
pub(crate) fn mode<'a>(term: Option<&'a CheckedTerm>, kind: &str) -> Option<&'a str> {
    let mode = operation(term)?.get("mode")?;
    (mode.get("kind")?.as_str()? == kind).then(|| mode.get("value")?.as_str())?
}

/// Whether an operation names the selected law definition.
pub(crate) fn has_law(term: Option<&CheckedTerm>, identity: &str) -> bool {
    operation(term)
        .and_then(|operation| operation.get("laws"))
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .any(|law| {
            law.get("definition")
                .and_then(|definition| definition.get("identity"))
                .and_then(serde_json::Value::as_str)
                == Some(identity)
        })
}

/// The declaration named by a quantity conversion operation.
pub(crate) fn quantity_conversion_target(
    term: Option<&CheckedTerm>,
    conversion_identity: &str,
) -> Option<CheckedNodeId> {
    let operation = operation(term)?;
    (operation.get("identity")?.as_str()? == conversion_identity)
        .then(|| node_id(operation.get("member")?.get("declaration")?))?
}

#[cfg(test)]
mod tests {
    /// Trace: AD-004 L-8.
    #[test]
    fn scalar_oracle_reads_operation_fields_through_core_ir() {
        let derive = include_str!("../../oracle/scalar/derive.rs");
        let lower = include_str!("../../oracle/scalar/lower.rs");
        for source in [derive, lower] {
            assert!(!source.contains(".get(\""));
            assert!(!source.contains(".operation("));
            assert!(!source.contains("[\"identity\"]"));
            assert!(!source.contains("[\"mode\"]"));
            assert!(!source.contains("[\"laws\"]"));
        }
        assert!(derive.contains("scalar_ir::identity("));
        assert!(lower.contains("scalar_ir::identity("));
    }
}

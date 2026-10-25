// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Typed access to admitted state-frame graph nodes and operation payloads.

use std::collections::BTreeMap;

use quire_contract_model::{
    ApplicationOperator, CheckedNodeId, CheckedPackageV2, CheckedSemanticNodeV2, CheckedTerm,
    LiteralKind,
};
use serde_json::Value;

use super::checked_term::{aggregate_members, bound_members, literal};

/// A graph node's checked shape, independent of its body representation.
pub(crate) struct NodeShape<'a> {
    pub(crate) tag: &'a str,
    pub(crate) form: &'a str,
}

impl NodeShape<'_> {
    pub(crate) fn is(&self, tag: &str, form: &str) -> bool {
        self.tag == tag && self.form == form
    }
}

/// The admitted graph by node id.
pub(crate) struct Graph<'g> {
    pub(crate) package: &'g CheckedPackageV2,
    nodes: BTreeMap<&'g CheckedNodeId, &'g CheckedSemanticNodeV2>,
}

impl<'g> Graph<'g> {
    pub(crate) fn of(package: &'g CheckedPackageV2) -> Self {
        Self {
            package,
            nodes: package
                .graph()
                .nodes
                .iter()
                .map(|node| (&node.node_id, node))
                .collect(),
        }
    }

    pub(crate) fn package(&self) -> &'g CheckedPackageV2 {
        self.package
    }

    pub(crate) fn term(&self, id: &CheckedNodeId) -> Option<&'g CheckedTerm> {
        self.package.term(id)
    }

    pub(crate) fn shape(&self, id: &CheckedNodeId) -> Option<NodeShape<'g>> {
        self.nodes.get(id).map(|node| NodeShape {
            tag: &node.node_tag,
            form: &node.semantic_form,
        })
    }

    /// `term` itself when inline, else the node a `reference` term names.
    pub(crate) fn follow(
        &self,
        term: &'g CheckedTerm,
        parent: &'g CheckedNodeId,
    ) -> Option<Located<'g>> {
        let CheckedTerm::Reference { target } = term else {
            return Some(Located {
                id: parent,
                referenced_node: None,
                body: term,
            });
        };
        let node = *self.nodes.get(target)?;
        Some(Located {
            id: &node.node_id,
            referenced_node: Some(&node.node_id),
            body: self.package.term(&node.node_id)?,
        })
    }

    /// The typed operation scope named by an anchor's context, operation and frame.
    pub(crate) fn scope(&self, anchor: &CheckedNodeId) -> Option<FrameScope> {
        if !self.shape(anchor)?.is("state", "operation_anchor") {
            return None;
        }
        let members = aggregate_members(self.term(anchor))?;
        let [context, operation, frame] =
            bound_members(members, ["context", "operation", "frame"])?;
        let object = reference(context)?.clone();
        let frame = reference(frame)?.clone();
        if !self.shape(&frame)?.is("state", "frame") {
            return None;
        }
        Some(FrameScope {
            operation: literal(operation, LiteralKind::Text)?.to_owned(),
            object,
            anchor: anchor.clone(),
            frame,
        })
    }
}

/// A term resolved to the node holding it, if it names one.
pub(crate) struct Located<'g> {
    pub(crate) id: &'g CheckedNodeId,
    pub(crate) referenced_node: Option<&'g CheckedNodeId>,
    pub(crate) body: &'g CheckedTerm,
}

/// The selected operation and frame declaration identities.
pub(crate) struct FrameScope {
    pub(crate) operation: String,
    pub(crate) object: CheckedNodeId,
    pub(crate) anchor: CheckedNodeId,
    pub(crate) frame: CheckedNodeId,
}

/// The arguments of an application of `operator`/`identity`.
pub(crate) fn application<'a>(
    body: &'a CheckedTerm,
    operator: ApplicationOperator,
    identity: &str,
) -> Option<&'a [CheckedTerm]> {
    match body {
        CheckedTerm::Application {
            operator: actual,
            operation,
            arguments,
            ..
        } if *actual == operator && operation_identity(operation)? == identity => Some(arguments),
        _ => None,
    }
}

/// The catalog identity of a checked application.
pub(crate) fn application_identity(body: &CheckedTerm) -> Option<&str> {
    let CheckedTerm::Application { operation, .. } = body else {
        return None;
    };
    operation_identity(operation)
}

/// The clause kind carried by a checked state-clause application.
pub(crate) fn clause_kind(body: &CheckedTerm) -> Option<&str> {
    let CheckedTerm::Application { operation, .. } = body else {
        return None;
    };
    operation.get("member")?.get("clause")?.as_str()
}

/// The selected field declaration and name of a record projection.
pub(crate) fn projected_field(body: &CheckedTerm) -> Option<(CheckedNodeId, &str)> {
    let CheckedTerm::Application { operation, .. } = body else {
        return None;
    };
    let member = operation.get("member")?;
    if member.get("kind")?.as_str()? != "field" {
        return None;
    }
    Some((
        node_id(member.get("declaration")?)?,
        member.get("name")?.as_str()?,
    ))
}

fn operation_identity(operation: &Value) -> Option<&str> {
    operation.get("identity")?.as_str()
}

fn node_id(value: &Value) -> Option<CheckedNodeId> {
    serde_json::from_value(value.clone()).ok()
}

fn reference(term: &CheckedTerm) -> Option<&CheckedNodeId> {
    match term {
        CheckedTerm::Reference { target } => Some(target),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    /// Trace: AD-004 L-8.
    #[test]
    fn frame_generator_uses_typed_graph_and_operation_access() {
        let source = include_str!("../../kani/generate/frame.rs");
        assert!(!source.contains("serde_json::Value"));
        assert!(!source.contains(".get(\""));
        assert!(!source.contains(".node_tag"));
        assert!(!source.contains(".semantic_form"));
        assert!(!source.contains(".nodes"));
    }
}

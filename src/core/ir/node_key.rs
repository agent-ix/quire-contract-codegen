// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Convert an admitted IR node identity to the exact structural key used by QSL.

use quire_contract_model::{CheckedNodeId, CheckedNodePathSegment};
use quire_exact::{NodeKey, PathSegment};

/// Rebuild an admitted structural identity without recursing through nested operands.
pub(crate) fn checked_node_key(id: &CheckedNodeId) -> Option<NodeKey> {
    enum Step<'a> {
        Visit(&'a CheckedNodeId),
        Text(&'a str),
        Finish {
            package: &'a str,
            path_len: usize,
            ordinal: u64,
        },
    }

    let mut pending = vec![Step::Visit(id)];
    let mut path = Vec::new();
    while let Some(step) = pending.pop() {
        match step {
            Step::Visit(node) => {
                pending.push(Step::Finish {
                    package: &node.package,
                    path_len: node.path.len(),
                    ordinal: node.ordinal,
                });
                for segment in node.path.iter().rev() {
                    match segment {
                        CheckedNodePathSegment::Name(name) => pending.push(Step::Text(name)),
                        CheckedNodePathSegment::Node(child) => pending.push(Step::Visit(child)),
                        CheckedNodePathSegment::Index(_) | CheckedNodePathSegment::Operand(_) => {
                            return None
                        }
                    }
                }
            }
            Step::Text(name) => path.push(PathSegment::Text(name.to_owned())),
            Step::Finish {
                package,
                path_len,
                ordinal,
            } => {
                let start = path.len().checked_sub(path_len)?;
                let segments = path.split_off(start);
                let node = NodeKey::new(package.to_owned(), segments, ordinal).ok()?;
                path.push(PathSegment::Node(node));
            }
        }
    }
    match path.pop()? {
        PathSegment::Node(node) if path.is_empty() => Some(node),
        _ => None,
    }
}

/// Resolve identities in the node table of an already checked producer document.
pub(crate) fn producer_node_keys(bytes: &[u8]) -> Option<qsl_replay::WireGraph> {
    let document: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    let ids = document
        .get("semantic_graph")?
        .get("nodes")?
        .as_array()?
        .iter()
        .map(|node| serde_json::from_value::<CheckedNodeId>(node.get("node_id")?.clone()).ok())
        .collect::<Option<Vec<_>>>()?;
    qsl_replay::WireGraph::new(ids.iter()).ok()
}

/// Emit a typed IR identity without recursion or a digest adaptation.
pub(crate) fn render_checked_node_id(key: &CheckedNodeId) -> Option<String> {
    enum Step<'a> {
        Visit(&'a CheckedNodeId, bool),
        Text(&'a str),
        Index(u64),
        Operand(u64),
        Finish {
            package: &'a str,
            ordinal: u64,
            len: usize,
            root: bool,
        },
    }

    let mut pending = vec![Step::Visit(key, true)];
    let mut rendered = Vec::new();
    while let Some(step) = pending.pop() {
        match step {
            Step::Visit(key, root) => {
                let segments = &key.path;
                pending.push(Step::Finish {
                    package: &key.package,
                    ordinal: key.ordinal,
                    len: segments.len(),
                    root,
                });
                for segment in segments.iter().rev() {
                    match segment {
                        quire_contract_model::CheckedNodePathSegment::Name(text) => {
                            pending.push(Step::Text(text))
                        }
                        quire_contract_model::CheckedNodePathSegment::Node(child) => {
                            pending.push(Step::Visit(child, false))
                        }
                        quire_contract_model::CheckedNodePathSegment::Operand(slot) => {
                            pending.push(Step::Operand(*slot))
                        }
                        quire_contract_model::CheckedNodePathSegment::Index(index) => {
                            pending.push(Step::Index(*index))
                        }
                    }
                }
            }
            Step::Text(text) => rendered.push(format!(
                "quire_contract_model::CheckedNodePathSegment::Name({text:?}.into())"
            )),
            Step::Operand(slot) => rendered.push(format!(
                "quire_contract_model::CheckedNodePathSegment::Operand({slot})"
            )),
            Step::Index(index) => rendered.push(format!(
                "quire_contract_model::CheckedNodePathSegment::Index({index})"
            )),
            Step::Finish {
                package,
                ordinal,
                len,
                root,
            } => {
                let start = rendered.len().checked_sub(len)?;
                let path = rendered.split_off(start).join(", ");
                let expression = format!("quire_contract_model::CheckedNodeId {{ package: {package:?}.into(), path: vec![{path}], ordinal: {ordinal} }}");
                rendered.push(if root { expression } else { format!("quire_contract_model::CheckedNodePathSegment::Node(std::sync::Arc::new({expression}))") });
            }
        }
    }
    match rendered.pop() {
        Some(value) if rendered.is_empty() => Some(value),
        _ => None,
    }
}

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

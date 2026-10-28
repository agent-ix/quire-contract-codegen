// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Shared typed walkers over admitted checked terms.

use quire_contract_model::{ApplicationOperator, CheckedNodeId, CheckedTerm, LiteralKind};
use quire_exact::Integer;
use serde_json::Value;

/// Decode a checked node id carried by an operation member.
pub(crate) fn node_id(value: &Value) -> Option<CheckedNodeId> {
    serde_json::from_value(value.clone()).ok()
}

/// The node named by a checked reference term.
pub(crate) fn reference(term: &CheckedTerm) -> Option<&CheckedNodeId> {
    match term {
        CheckedTerm::Reference { target } => Some(target),
        _ => None,
    }
}

/// Arguments of an application with the selected checked operator and identity.
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
        } if *actual == operator && operation.identity()? == identity => Some(arguments),
        _ => None,
    }
}

/// The members of an admitted aggregate, without interpreting its wire representation.
pub(crate) fn aggregate_members<'a>(
    term: impl Into<Option<&'a CheckedTerm>>,
) -> Option<&'a [CheckedTerm]> {
    match term.into()? {
        CheckedTerm::Aggregate { members } => Some(members),
        _ => None,
    }
}

/// Select the complete, unique set of named bound members in the requested order.
pub(crate) fn bound_members<'a, const N: usize>(
    members: &'a [CheckedTerm],
    names: [&str; N],
) -> Option<[&'a CheckedTerm; N]> {
    if members.len() != N {
        return None;
    }
    let mut found = [None; N];
    for member in members {
        let CheckedTerm::Binding { name, value } = member else {
            return None;
        };
        let slot = &mut found[names
            .iter()
            .position(|expected| *expected == name.as_ref())?];
        if slot.replace(value.as_ref()).is_some() {
            return None;
        }
    }
    found
        .into_iter()
        .collect::<Option<Vec<_>>>()?
        .try_into()
        .ok()
}

/// The unchanged string payload of a literal of the selected closed kind.
pub(crate) fn literal(term: &CheckedTerm, kind: LiteralKind) -> Option<&str> {
    match term {
        CheckedTerm::Literal {
            value_kind, value, ..
        } if *value_kind == kind => value.as_str(),
        _ => None,
    }
}

/// The same canonical arbitrary-precision integer accepted by the scalar bound reader.
pub(crate) fn literal_integer(term: &CheckedTerm) -> Option<Integer> {
    let spelling = literal(term, LiteralKind::Integer)?;
    let value: Integer = spelling.parse().ok()?;
    (value.to_string() == spelling).then_some(value)
}

/// A canonical integer literal whose value fits the cardinality counter.
pub(crate) fn literal_count(term: &CheckedTerm) -> Option<u64> {
    let spelling = literal(term, LiteralKind::Integer)?;
    let value: u64 = spelling.parse().ok()?;
    (value.to_string() == spelling).then_some(value)
}

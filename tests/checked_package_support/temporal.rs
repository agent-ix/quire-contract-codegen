// SPDX-License-Identifier: AGPL-3.0-or-later
//! Clock bindings authored beside synthetic temporal clauses.

use quire_contract_model::{CheckedNodeId, ClockConfiguration, TemporalBinding};
use serde_json::{json, Value};

/// Bind one clock-configured clause using its authored identity, clock argument, and lock profile.
#[ix_trace_rs::string_edge]
pub(super) fn bind_clause(wire: &mut Value, declaration: CheckedNodeId) {
    let node_id = json!(declaration);
    let clause = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("fixture graph nodes")
        .iter()
        .find(|node| node["node_id"] == node_id)
        .expect("authored temporal clause");
    assert_eq!(clause["node_tag"], "temporal");
    assert_eq!(clause["semantic_form"], "temporal_clause");
    let laws = clause["body"]["operation"]["laws"]
        .as_array()
        .expect("temporal clause laws");
    let [law] = laws.as_slice() else {
        panic!("fixture temporal clause has one profile law");
    };
    assert_eq!(law["role"], "temporal_profile");
    let clock_name = clause["body"]["arguments"][1]["value"]
        .as_str()
        .expect("fixture clause clock literal");
    assert_eq!(clause["body"]["arguments"][1]["term"], "literal");
    assert_eq!(clause["body"]["arguments"][1]["value_kind"], "text");
    assert!(!clock_name.is_empty());
    let selected = wire["lock"]["profile_selections"]
        .as_array()
        .expect("fixture profile selections")
        .iter()
        .any(|selection| {
            selection["role"] == "temporal_profile" && selection["definition"] == law["definition"]
        });
    assert!(selected, "clause profile must be selected by its lock");
    let profile = law["definition"]["identity"]
        .as_str()
        .expect("fixture temporal profile identity");
    let clock = match profile {
        "quire.temporal.event-position.false-extension/v1" => ClockConfiguration::EventPosition {
            sequence_authority: clock_name.into(),
        },
        other => panic!("fixture needs explicit clock parameters for profile {other}"),
    };
    let binding = TemporalBinding {
        declaration,
        clock_name: clock_name.into(),
        clock,
    };
    let root = wire.as_object_mut().expect("fixture package object");
    let mut rows: Vec<TemporalBinding> = serde_json::from_value(
        root.remove("temporal_bindings")
            .unwrap_or_else(|| json!([])),
    )
    .expect("fixture temporal bindings");
    rows.push(binding);
    rows.sort_by(|left, right| left.declaration.cmp(&right.declaration));
    assert!(rows
        .windows(2)
        .all(|pair| pair[0].declaration != pair[1].declaration));
    root.insert(
        "temporal_bindings".into(),
        serde_json::to_value(rows).expect("typed temporal bindings serialize"),
    );
}

/// Keep clock rows aligned with the graph nodes retained by a focused fixture.
pub(super) fn retain_for_nodes(wire: &mut Value, keep: impl Fn(&CheckedNodeId) -> bool) {
    let root = wire.as_object_mut().expect("fixture package object");
    let Some(rows) = root.remove("temporal_bindings") else {
        return;
    };
    let mut rows: Vec<TemporalBinding> =
        serde_json::from_value(rows).expect("fixture temporal bindings");
    rows.retain(|row| keep(&row.declaration));
    if !rows.is_empty() {
        root.insert(
            "temporal_bindings".into(),
            serde_json::to_value(rows).expect("typed temporal bindings serialize"),
        );
    }
}

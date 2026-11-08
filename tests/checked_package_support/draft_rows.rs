// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Explicit operand-row authoring for detached test drafts.
//!
//! Callers select each intentional draft holding node and supply its fresh row.
//! This never reads, repairs or normalizes an admitted package's wire graph.

use serde_json::{json, Value};

/// Encode document-index local IDs in one selected draft holding node.
/// Foreign package IDs retain their document frame. Reusing an authored row is
/// rejected so a caller cannot silently reinterpret existing operand slots.
pub fn author_row(package: &str, node: &mut Value, row: &mut Vec<u64>) {
    assert!(row.is_empty(), "draft row must be fresh");
    let mut pending = vec![node];
    while let Some(value) = pending.pop() {
        let is_id = value.as_object().is_some_and(|members| {
            members.len() == 3
                && members.contains_key("package")
                && members.contains_key("path")
                && members.contains_key("ordinal")
        });
        if is_id {
            if value["package"].as_str() != Some(package) {
                continue;
            }
            for segment in value["path"].as_array_mut().unwrap() {
                assert!(
                    segment.is_string() || segment.get("node").and_then(Value::as_u64).is_some(),
                    "selected draft row must contain document-index references, not existing slots"
                );
                if let Some(index) = segment.get("node").and_then(Value::as_u64) {
                    let slot = match row.iter().position(|value| *value == index) {
                        Some(slot) => slot,
                        None => {
                            row.push(index);
                            row.len() - 1
                        }
                    };
                    *segment = json!({"operand": u64::try_from(slot).unwrap()});
                }
            }
            continue;
        }
        match value {
            Value::Object(members) => pending.extend(members.values_mut().rev()),
            Value::Array(values) => pending.extend(values.iter_mut().rev()),
            _ => {}
        }
    }
}

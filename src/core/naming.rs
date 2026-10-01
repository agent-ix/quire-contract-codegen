//! Bounded readable name components, unique names and symbol derivation shared by every
//! generator (AD-004 step 2d-0).
//!
//! This module is `core/naming.rs`: it depends on no generator, strategy, evidence or Kani module.

use std::{collections::BTreeMap, fmt::Write as _};

use quire_contract_model::StateObservation;

pub(crate) fn reference_identifier(name: &str, observation: Option<StateObservation>) -> String {
    format!("{}_{}", rust_component(name), observation_name(observation))
}

pub(crate) fn observation_name(observation: Option<StateObservation>) -> &'static str {
    match observation {
        Some(StateObservation::Pre) => "pre",
        Some(StateObservation::Post) => "post",
        Some(StateObservation::Current) | None => "current",
    }
}

fn rust_component(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
            result.push(char::from(byte));
        } else {
            let _ = write!(result, "_{byte:02x}");
        }
    }
    if result.is_empty() {
        result.push_str("empty");
    }
    result
}

/// `value` as a readable snake-case name component of at most 24 characters.
pub(crate) fn bounded_readable_component(value: &str) -> String {
    readable_name_component(value, 24)
}

/// `value` lowercased, each run of non-alphanumeric bytes as one `_`, cut to at most `limit`
/// characters, with no leading or trailing `_`, so joining components with `_` never yields the
/// `__` that Rust's `non_snake_case` lint rejects. An empty result is `x`.
pub(crate) fn readable_name_component(value: &str, limit: usize) -> String {
    let mut result = String::with_capacity(value.len().min(limit));
    for byte in value.bytes() {
        if result.len() >= limit {
            break;
        }
        if byte.is_ascii_alphanumeric() {
            result.push(char::from(byte.to_ascii_lowercase()));
        } else if !result.is_empty() && !result.ends_with('_') {
            result.push('_');
        }
    }
    let trimmed = result.trim_end_matches('_');
    if trimmed.is_empty() {
        "x".to_owned()
    } else {
        trimmed.to_owned()
    }
}

/// `strategy_fr_001_1_x` as `StrategyFr0011X`: a generated snake-case name as a type-name stem.
pub(crate) fn upper_camel(value: &str) -> String {
    value
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut characters = part.chars();
            characters.next().map_or_else(String::new, |first| {
                first.to_ascii_uppercase().to_string() + characters.as_str()
            })
        })
        .collect()
}

/// The final names of the items one generation emits side by side. Each item offers a readable
/// `stem` and its full identity `key`. A stem held by one item is that item's name. Items sharing a
/// stem are named `{stem}_{ordinal}`, numbered from 1 in ascending `key` order, so a name depends on
/// the item and on which siblings share its stem, never on request order. Suffixing repeats until
/// every name is distinct, since a suffixed name can meet another item's stem.
pub(crate) fn unique_names<K: Ord>(items: Vec<(String, K)>) -> Vec<String> {
    let (mut names, keys): (Vec<String>, Vec<K>) = items.into_iter().unzip();
    loop {
        let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (index, name) in names.iter().enumerate() {
            groups.entry(name.clone()).or_default().push(index);
        }
        if groups.values().all(|group| group.len() == 1) {
            return names;
        }
        for (stem, mut group) in groups {
            if group.len() > 1 {
                group.sort_by(|left, right| keys[*left].cmp(&keys[*right]).then(left.cmp(right)));
                for (ordinal, index) in group.into_iter().enumerate() {
                    names[index] = format!("{stem}_{}", ordinal + 1);
                }
            }
        }
    }
}

/// [`unique_names`] for exactly two items.
pub(crate) fn unique_pair<K: Ord>(first: (String, K), second: (String, K)) -> (String, String) {
    let mut names = unique_names(vec![first, second]).into_iter();
    let first = names.next().unwrap_or_default();
    let second = names.next().unwrap_or_default();
    (first, second)
}

/// The readable stem of one clause's oracle name: its requirement, revision and clause. Two
/// clauses can share a stem; generators emitting several oracles together name them through
/// [`unique_names`].
pub(crate) fn oracle_symbol(requirement: &str, revision: u64, clause: &str) -> String {
    format!(
        "oracle_{}_{revision}_{}",
        bounded_readable_component(requirement),
        bounded_readable_component(clause)
    )
}

#[cfg(test)]
mod tests {
    use super::{oracle_symbol, unique_names};

    /// Items sharing a stem are numbered in key order, whatever order they arrive in; an item
    /// with a stem of its own keeps it.
    ///
    /// Trace: FR-022-AC-9, TC-033
    #[test]
    fn equal_stems_take_ordinals_in_key_order_not_arrival_order() {
        let forward = unique_names(vec![
            ("oracle_add".to_owned(), "node-a"),
            ("oracle_sub".to_owned(), "node-b"),
            ("oracle_add".to_owned(), "node-c"),
        ]);
        let reversed = unique_names(vec![
            ("oracle_add".to_owned(), "node-c"),
            ("oracle_sub".to_owned(), "node-b"),
            ("oracle_add".to_owned(), "node-a"),
        ]);
        assert_eq!(forward, ["oracle_add_1", "oracle_sub", "oracle_add_2"]);
        assert_eq!(reversed, ["oracle_add_2", "oracle_sub", "oracle_add_1"]);
    }

    /// A suffixed name that meets another item's own stem is suffixed again, so every name is
    /// distinct.
    ///
    /// Trace: FR-022-AC-9, TC-033
    #[test]
    fn a_suffixed_name_meeting_another_stem_is_disambiguated_again() {
        let names = unique_names(vec![
            ("x".to_owned(), 1),
            ("x".to_owned(), 2),
            ("x_1".to_owned(), 3),
        ]);
        assert_eq!(names, ["x_1_1", "x_2", "x_1_2"]);
    }

    /// Two clause identities whose readable forms coincide get distinct names: a requirement,
    /// revision and clause split differently, and two clause ids equal in their first 24
    /// characters.
    ///
    /// Trace: FR-022-AC-9, TC-033
    #[test]
    fn readable_stems_that_coincide_still_yield_distinct_names() {
        let split = [("fr-1", 2, "c"), ("fr", 1, "2-c")];
        let long = [
            ("FR-001", 7, "a-very-long-shared-clause-prefix-one"),
            ("FR-001", 7, "a-very-long-shared-clause-prefix-two"),
        ];
        for pair in [split, long] {
            let stems = pair.map(|(requirement, revision, clause)| {
                oracle_symbol(requirement, revision, clause)
            });
            assert_eq!(stems[0], stems[1], "the readable stems coincide");
            let names = unique_names(
                pair.iter()
                    .zip(&stems)
                    .map(|(identity, stem)| (stem.clone(), *identity))
                    .collect(),
            );
            assert_ne!(names[0], names[1]);
            assert_eq!(
                names[0],
                format!("{}_{}", stems[0], 2 - usize::from(pair[0] < pair[1]))
            );
        }
    }
}

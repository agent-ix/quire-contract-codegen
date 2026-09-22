//! IR-237: the integration test crate. Cargo builds a `tests/<name>/main.rs` directory as one
//! test target automatically (no `[[test]]` entry needed), so every former top-level `tests/*.rs`
//! file below is a module of the single `it` binary instead of its own executable -- each still
//! links the crate and its dependencies once per `cargo test` invocation rather than once per
//! former file.
//!
//! `common`, `composite_equality_support`, `exact_scalar_support`, `exact_function_support` and
//! `fixtures` stay at `tests/`, one level up from here; the modules that use them reach them with `#[path =
//! "../..."]` or `include!("../...")` rather than moving them, so a file's relative-path literals
//! changed only by the one extra path segment this directory adds.
//!
//! Tests that need their own process (a real, unshimmed `cargo kani` run under the host-wide
//! `flock`) are `#[ignore]`d here exactly as they were before the merge and stay reachable by
//! name filter: see `Makefile`'s `kani` target, which now runs
//! `cargo test --test it kani_obligations -- --ignored --test-threads=1`.
//!
//! `common` is `mod`-included by six former top-level files. Cargo tolerated that -- each file
//! used to be its own crate, so each had its own copy -- but `clippy::duplicate_mod` correctly
//! flags the same source file loaded as more than one module inside one crate now that they all
//! share the `it` crate. It has no process-global state, so it is declared exactly once here and
//! the modules that use it write `use crate::common;` instead of redeclaring it.
//!
//! `composite_equality_support/package.rs` and `exact_scalar_support/package.rs` are `mod`-
//! included by two former top-level files each, and stay duplicated (each consuming file still
//! declares its own `#[path = ...] mod package;`, `#[allow(clippy::duplicate_mod)]`d with a
//! comment) rather than being centralized the same way: each holds a process-global
//! `application_registry()` static keyed by small integer fixture codes that the two consuming
//! files pick independently, on the assumption of an isolated registry per binary. Centralizing
//! them was tried and reverted -- it produced real cross-file code collisions and
//! Mutex-poisoning cascades once both files' tests shared one process-wide registry.
//!
//! `exact_function_support/package.rs` is likewise `mod`-included by two former top-level files
//! and kept duplicated, but for uniformity with the pattern above, not the same reason: it holds
//! no process-global state, so nothing forces the duplication here -- it is not load-bearing.
#[path = "../common/mod.rs"]
pub(crate) mod common;

mod bound_census;
mod bound_coverage;
mod bound_generation;
mod bound_populations;
mod bound_strategy_generation;
mod bounded_kani_corpus;
mod capability_settlement;
mod composite_equality_agreement;
mod composite_equality_generation;
mod exact_function_agreement;
mod exact_function_generation;
mod exact_scalar_agreement;
mod exact_scalar_generation;
mod harness_generation;
mod integration;
mod interface_001;
mod kani_argument_order;
mod kani_generation;
mod kani_obligations;
mod kani_witness_join;
mod oracle_generation;
mod shared_assurance;
mod strategy_generation;
mod vacuity_primitives;

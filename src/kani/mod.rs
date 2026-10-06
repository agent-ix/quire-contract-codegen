//! The Kani subsystem: harness generation, the run and the reading of its output (AD-004,
//! FR-015, FR-017, FR-025, FR-028 to FR-030).
//!
//! The subject ABI vocabulary, the proof-dependency census types and the harness and identity
//! record types sit below the generators, the output reader, the classifier and the runner.

// The subject ABI vocabulary: binding roles, primitive types, bounds, solver and options.
pub(crate) mod abi;
// The proof-dependency census types.
pub(crate) mod census;
// Run classification.
// Implements: FR-017
pub(crate) mod classify;
// The one generator.
// Implements: FR-015
pub(crate) mod generate;
// The harness and identity record types.
// Implements: FR-015
pub(crate) mod identity;
// The one reader of Kani output.
// Implements: FR-017
pub(crate) mod output;
// Launch, capture, timeout and the execution of one harness.
// Implements: FR-017
pub(crate) mod run;
// The terminal-value map of a run.
// Implements: FR-029
pub(crate) mod terminal;
// Test helpers shared by the tests of more than one `kani/` file.
#[cfg(test)]
#[path = "../../tests/common/kani_run.rs"]
pub(crate) mod test_support;

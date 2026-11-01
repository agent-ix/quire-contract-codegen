//! The Kani subsystem: harness generation, the run and the reading of its output (AD-004,
//! FR-015, FR-017, FR-025, FR-028 to FR-030).
//!
//! The subject ABI vocabulary, the proof-dependency census types and the harness and identity
//! record types sit below the generators, the output reader, the classifier and the runner.

/// Whether `$type` implements `Deserialize`, decided by method resolution through the probe of
/// `test_support`. Defined here, ahead of every submodule, so each can use it by name.
#[cfg(test)]
macro_rules! implements_deserialize {
    ($type:ty) => {{
        // Exactly one of the two traits is chosen by resolution, so the other import is unused
        // in each expansion.
        #[allow(unused_imports)]
        use $crate::kani::test_support::{No, Yes};
        (&$crate::kani::test_support::Probe::<$type>(std::marker::PhantomData))
            .implements_deserialize()
    }};
}

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
pub(crate) mod test_support;

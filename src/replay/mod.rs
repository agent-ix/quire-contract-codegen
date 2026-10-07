//! The replay subsystem: decoding a Kani witness and replaying it natively (AD-004, FR-016,
//! FR-024).
//!
//! The witness decode types a Kani playback against the persisted argument bindings, and the
//! function and frame replays build QSL's replay request from the decoded values.

// Binding of QSL composite reports to their retained sent claim.
pub(crate) mod composite;
// Native replay of a frame counterexample.
// Implements: FR-015-AC-33
pub(crate) mod frame;
// The replay adapter for a function counterexample.
// Implements: FR-016
pub(crate) mod function;
// The function-contract obligation identity of the function path.
// Implements: FR-016-AC-21
pub(crate) mod obligation;
// Native replay of a postcondition state-clause counterexample.
// Implements: FR-024-AC-11
pub(crate) mod state_clause;
// Joins a real Kani witness to the generator's own persisted obligation schema.
pub(crate) mod witness;

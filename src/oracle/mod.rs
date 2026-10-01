//! The oracle subsystem: generation of the Rust oracles (AD-004, FR-014, FR-018, FR-021).
//!
//! The shared generation-result and claim vocabulary, the V1 Boolean and bound oracles, and the
//! exact scalar, composite equality and exact function oracle generators live here.

// V1 Boolean oracle; retired with V1.
pub(crate) mod boolean_v1;
// V1 bound oracle generation; retired with V1.
pub(crate) mod bound_v1;
// Shared generation-result and claim vocabulary (FR-014, FR-018, FR-021).
pub(crate) mod claim;
// Implements: FR-018
pub(crate) mod equality;
// Implements: FR-021
pub(crate) mod function;
// Implements: FR-014
pub(crate) mod scalar;

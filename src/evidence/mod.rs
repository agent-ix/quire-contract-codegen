//! The evidence subsystem: bounded coverage observations (AD-004, FR-004).
//!
//! The LLVM coverage parse and clause classification, and the complete bound-domain observations,
//! live here.

// Complete bound observations; V1 input until rebased.
// Implements: FR-004 (complete domain observations, always unqualified).
pub(crate) mod bound_coverage;
// Bounded LLVM observation primitives; no aggregate coverage verdict.
// Implements: FR-004 (bounded observation primitives; no aggregate coverage verdict).
pub(crate) mod vacuity;

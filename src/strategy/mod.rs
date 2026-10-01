//! The strategy subsystem: generated proptest strategies and tri-state harnesses (AD-004, FR-002,
//! FR-008 to FR-013).
//!
//! The tri-state harness generator, the enum and `i64` strategy campaigns, and the bound numeric
//! strategy generation live here.

// The bound numeric strategy generation; V1 input, retired with V1.
pub(crate) mod bound;
// Enum and `i64` strategy campaigns (was `strategy`); no V1 input.
// Implements: FR-002
pub(crate) mod campaign;
// Tri-state harness generation; V1 input.
// Implements: FR-002
pub(crate) mod harness;

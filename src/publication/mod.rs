//! The publication subsystem: validated, rollback-protected publication of a generated bundle
//! (AD-004, FR-005).

// The atomic bundle writer and the published identity.
// Implements: FR-005, NFR-001
pub(crate) mod publish;

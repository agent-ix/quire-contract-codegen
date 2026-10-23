//! The routed subsystem: capability settlement and generation for routed items (AD-004, FR-019,
//! FR-022, FR-026).
//!
//! The capability settlement point and the generation arm that runs a routed backend kind live
//! here.

// Capability settlement.
// Implements: FR-019
pub(crate) mod capability;
// Production package access lives in the shared core; these controls exercise routed use.
#[cfg(test)]
mod contract_package_tests;
// Generation for items the driver has already settled and routed.
// Implements: FR-022
pub(crate) mod generate;
// Solver run records and evidence mapping for QSL-authored queries.
pub mod smt;

//! The routed subsystem: capability settlement and generation for routed items (AD-004, FR-019,
//! FR-022, FR-026).
//!
//! The capability settlement point and the generation arm that runs a routed backend kind live
//! here.

// Capability settlement.
// Implements: FR-019
pub(crate) mod capability;
// Typed access to one admitted FR-330 package.
// Implements: FR-039
pub(crate) mod contract_package;
// Generation for items the driver has already settled and routed.
// Implements: FR-022
pub(crate) mod generate;

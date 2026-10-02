//! The one Kani generator: the request and result vocabulary, the negotiation entry and the
//! family lowerers.

// Census validation; interim, see the file header.
pub(crate) mod census_validation;
// The V1 clause lowering the precondition and contract families share.
pub(crate) mod clause;
// The contract family.
pub(crate) mod contract;
// The bounded-Kani corpus family.
pub(crate) mod corpus;
// The state-frame family.
// Implements: FR-015 (IR-412: state-clause operation contract and frame effects).
pub(crate) mod frame;
// The Kani family lowerings over Contract IR.
pub(crate) mod lower;
// `negotiate_kani_obligations`, the one public generation entry.
pub(crate) mod negotiate;
// The request and result vocabulary every family speaks.
pub(crate) mod outcome;
// The precondition family.
pub(crate) mod precondition;
// The validated harness path and the persisted per-harness record.
pub(crate) mod record;
// The scalar family.
pub(crate) mod scalar;
// The V1 bundle generator; interim, see the file header.
pub(crate) mod v1_bundle;

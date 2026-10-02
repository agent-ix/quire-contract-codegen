//! Bound numeric strategy generation over admitted single-comparison clauses.
//!
//! The relation model is IR-independent: clause admission (FR-008) lowers a `BoundPackage` clause
//! into a [`relation::Relation`] over a [`relation::Domain`], and every submodule here consumes only
//! that model.

// Implements: FR-010
pub(crate) mod census;
// Implements: FR-008 through FR-013
pub(crate) mod generation;
// Implements: FR-009, FR-012
pub(crate) mod population;
// Implements: FR-010
pub(crate) mod relation;

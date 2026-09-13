//! Bound numeric strategy generation over admitted single-comparison clauses.
//!
//! The relation model is IR-independent: clause admission (FR-008) lowers a `BoundPackage` clause
//! into a [`relation::Relation`] over a [`relation::Domain`], and every submodule here consumes only
//! that model.

// Implements: FR-010
pub mod census;
// Implements: FR-010
pub mod relation;

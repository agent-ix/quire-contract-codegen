//! Typed access to admitted V2 clause conditions and their model-owned fields.

pub(crate) mod clause;

/// Shared typed walkers for admitted checked terms.
pub(crate) mod checked_term;
/// Typed graph and operation access for state-frame clauses.
pub(crate) mod frame;
/// Fallible structural node identity conversion at the IR/QSL boundary.
pub(crate) mod node_key;
/// Typed scalar node and operation access.
pub(crate) mod scalar;
/// Shared scalar bound readers used by scalar and equality oracles.
pub(crate) mod scalar_bounds;

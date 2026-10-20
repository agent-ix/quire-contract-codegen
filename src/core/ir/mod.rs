//! Typed access to admitted V2 clause conditions and their model-owned fields.

mod clause;

pub(crate) use clause::{read_clause, ClauseInput, ClauseInputSide, ClauseTerm};

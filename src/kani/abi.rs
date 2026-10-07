//! The Kani subject ABI vocabulary: binding roles, primitive types, integer bounds, the solver
//! choice and the adapter option vector.

use quire_contract_model::{IntegerDomain, IntegerType, OverflowPolicy};
use serde::{Deserialize, Serialize};

/// Position of one primitive dependency in the generated subject ABI.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniBindingRole {
    /// A copied current/pre value passed to the customer subject.
    Argument,
    /// A copied post-state value returned by the customer subject.
    Result,
}

/// Rust primitive used for one generated Kani subject binding.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniPrimitiveType {
    /// Rust `bool`.
    Boolean,
    /// Rust `i64`.
    I64,
}

impl KaniPrimitiveType {
    pub(crate) const fn source_name(self) -> &'static str {
        match self {
            Self::Boolean => "bool",
            Self::I64 => "i64",
        }
    }
}

/// Exact checked IR domain retained for one bounded-integer binding.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct KaniIntegerBounds {
    /// Signed or unsigned checked IR domain.
    pub domain: IntegerDomain,
    /// Inclusive checked minimum.
    pub minimum: i64,
    /// Inclusive checked maximum.
    pub maximum: i64,
    /// Checked overflow policy; generation never replaces it.
    pub overflow: OverflowPolicy,
}

impl KaniIntegerBounds {
    /// Narrows an IR interval only when the generated `i64` subject can represent both ends.
    pub(crate) fn from_model(value: &IntegerType) -> Option<Self> {
        Some(Self {
            domain: value.domain(),
            minimum: i64::try_from(value.minimum()).ok()?,
            maximum: i64::try_from(value.maximum()).ok()?,
            overflow: value.overflow(),
        })
    }
}

/// Supported solver choice for the Kani adapter.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniSolver {
    /// Kani's CaDiCaL SAT solver.
    Cadical,
}

impl KaniSolver {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Cadical => "cadical",
        }
    }
}

pub(crate) fn i64_literal(value: i64) -> String {
    match value {
        i64::MIN => "i64::MIN".to_owned(),
        i64::MAX => "i64::MAX".to_owned(),
        _ => format!("{value}_i64"),
    }
}

pub(crate) fn adapter_options(
    harness: &str,
    unwind: u32,
    solver: KaniSolver,
    uses_stubbing: bool,
) -> Vec<String> {
    let mut options = vec!["-Z".to_owned(), "function-contracts".to_owned()];
    if uses_stubbing {
        options.extend(["-Z".to_owned(), "stubbing".to_owned()]);
    }
    options.extend([
        "-Z".to_owned(),
        "concrete-playback".to_owned(),
        "--harness".to_owned(),
        harness.to_owned(),
        "--exact".to_owned(),
        "--unwind".to_owned(),
        unwind.to_string(),
        "--solver".to_owned(),
        solver.as_str().to_owned(),
        "--output-format".to_owned(),
        "regular".to_owned(),
        "--concrete-playback".to_owned(),
        "print".to_owned(),
    ]);
    options
}

/// `value` as a readable snake-case name component of at most 12 characters.
pub(crate) fn readable_component(value: &str) -> String {
    crate::core::naming::readable_name_component(value, 12)
}

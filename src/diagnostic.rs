//! The generation diagnostic vocabulary shared by every generator (AD-004 step 2a).
//!
//! This module becomes `core/diagnostic.rs`. `artifact` needs [`GenerationTerminalState`] for
//! `PublicationDiagnostic`, so the vocabulary moves out of `oracle` with it rather than leaving
//! `core` importing the oracle subsystem.

use quire_contract_model::SourceSpan;
use serde::{Deserialize, Serialize};

/// Interface-001 terminal state for a generation result.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GenerationTerminalState {
    /// A complete supported artifact was generated.
    Generated,
    /// The input uses semantics outside the bounded generator slice.
    Unsupported,
    /// The input is invalid for the requested generation operation.
    InvalidInput,
    /// The configured backend is unavailable.
    BackendUnavailable,
    /// Atomic publication failed.
    IoFailed,
    /// An internal generation control could not reach a conclusion.
    Inconclusive,
}

impl GenerationTerminalState {
    /// Every terminal state, in declaration order. [`Self::label`]'s `match` is exhaustive, so an
    /// added variant fails the build until it is named there; add it to this array in the same
    /// edit.
    pub const ALL: [Self; 6] = [
        Self::Generated,
        Self::Unsupported,
        Self::InvalidInput,
        Self::BackendUnavailable,
        Self::IoFailed,
        Self::Inconclusive,
    ];

    /// interface-001's declared label for this terminal state. Exhaustive: a variant not named
    /// here fails the build.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Generated => "generated",
            Self::Unsupported => "unsupported",
            Self::InvalidInput => "invalid-input",
            Self::BackendUnavailable => "backend-unavailable",
            Self::IoFailed => "io-failed",
            Self::Inconclusive => "inconclusive",
        }
    }
}

/// Stable machine-readable generation failure category.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationErrorCode {
    /// The clause root is not Boolean.
    NonBooleanRoot,
    /// The first slice cannot lower an expression without approximation.
    UnsupportedExpression,
    /// A dependency cannot be represented in the generated signature.
    UnsupportedDependency,
    /// The typed expression carries definedness obligations this slice cannot preserve.
    UnsupportedObligations,
    /// Two input identities would claim the same generated name.
    NameCollision,
    /// The bounded output resource would be exceeded.
    ResourceLimitExceeded,
    /// Generated tokens did not parse as a Rust source file.
    InvalidGeneratedSyntax,
    /// A deterministic source-map value could not be encoded.
    SerializationFailed,
}

impl GenerationErrorCode {
    /// Maps the diagnostic category to its interface-001 terminal state.
    #[must_use]
    pub const fn terminal_state(self) -> GenerationTerminalState {
        match self {
            Self::NonBooleanRoot | Self::NameCollision => GenerationTerminalState::InvalidInput,
            Self::UnsupportedExpression
            | Self::UnsupportedDependency
            | Self::UnsupportedObligations
            | Self::ResourceLimitExceeded => GenerationTerminalState::Unsupported,
            Self::InvalidGeneratedSyntax | Self::SerializationFailed => {
                GenerationTerminalState::Inconclusive
            }
        }
    }
}

/// Structured diagnostic returned without a partial artifact bundle.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GenerationDiagnostic {
    /// Stable diagnostic category.
    pub code: GenerationErrorCode,
    /// Interface-001 terminal state implied by `code`.
    pub terminal_state: GenerationTerminalState,
    /// Requirement identity associated with the failure.
    pub requirement_id: String,
    /// Exact requirement revision associated with the failure.
    pub requirement_revision: u64,
    /// Clause identity associated with the failure.
    pub clause_id: String,
    /// Stable path to the rejected input element.
    pub path: String,
    /// Exact IR-owned locus for expression-related failures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span: Option<SourceSpan>,
    /// Human-readable detail that is not used as machine identity.
    pub message: String,
}

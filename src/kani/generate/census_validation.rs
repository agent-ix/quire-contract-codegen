//! INTERIM: census validation, the V1 bundle's error type it returns, and `deterministic_json`.
//! The bounded-Kani corpus calls them. This file survives the deletion of `v1_bundle.rs` at
//! step 4f; step 4g, or the V2 census input, deletes `validate_dependencies` and the error
//! type, and step 1a deletes `deterministic_json` (AD-004).

use std::collections::BTreeSet;

use quire_contract_model::SourceSpan;
use serde::{Deserialize, Serialize};

use crate::{
    core::diagnostic::{GenerationErrorCode, GenerationTerminalState},
    kani::census::{ProofDependencyKind, ProofDependencyRequest, ProofDependencyState},
};

/// Stable reason a Kani bundle could not be generated.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KaniErrorCode {
    /// A proof, subject, assumption, or stub identity is invalid or duplicated.
    InvalidIdentity,
    /// A dependency kind/state/path combination is invalid.
    InvalidDependency,
    /// The first slice cannot bind the supplied clause dependencies to `fn(bool, bool) -> bool`.
    UnsupportedBinding,
    /// A Boolean clause could not be lowered without approximation.
    ClauseGenerationFailed,
    /// The explicit unwind value is zero or exceeds the first-slice bound.
    InvalidUnwind,
    /// Generated Rust did not parse.
    InvalidGeneratedSyntax,
    /// A deterministic graph could not be serialized.
    SerializationFailed,
    /// The generated source exceeds the bounded artifact size.
    ResourceLimitExceeded,
}

impl KaniErrorCode {
    /// Maps a Kani diagnostic category to interface-001 terminal state.
    #[must_use]
    pub const fn terminal_state(self) -> GenerationTerminalState {
        match self {
            Self::InvalidIdentity | Self::InvalidDependency | Self::InvalidUnwind => {
                GenerationTerminalState::InvalidInput
            }
            Self::UnsupportedBinding | Self::ResourceLimitExceeded => {
                GenerationTerminalState::Unsupported
            }
            Self::ClauseGenerationFailed
            | Self::InvalidGeneratedSyntax
            | Self::SerializationFailed => GenerationTerminalState::Inconclusive,
        }
    }
}

/// Structured Kani-generation failure returned without a partial bundle.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct KaniDiagnostic {
    /// Stable Kani diagnostic category.
    pub code: KaniErrorCode,
    /// Interface-001 terminal state.
    pub terminal_state: GenerationTerminalState,
    /// Preserved Boolean-lowering code, when the failure originated in an oracle clause.
    pub generation_code: Option<GenerationErrorCode>,
    /// Stable path to the rejected request element.
    pub path: String,
    /// Exact IR-owned locus when clause lowering identified one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span: Option<SourceSpan>,
    /// Human-readable detail not used as machine identity.
    pub message: String,
}

/// Validates one caller-declared proof-dependency census: every declared identity is non-empty,
/// unique within the census, and distinct from `proof_id` (the root proof this census is declared
/// against), and its kind/state/path combination is one of the three closed shapes (`Required`,
/// `Assumed`, `Stubbed`).
///
/// Shared by [`generate_kani_bundle`](super::v1_bundle::generate_kani_bundle)'s request validation and the bounded-Kani corpus's
/// declared-census validation, so there is exactly one definition of what a valid
/// proof-dependency census looks like rather than two that can drift apart.
pub(crate) fn validate_dependencies(
    dependencies: &[ProofDependencyRequest<'_>],
    proof_id: &str,
) -> Result<(), Vec<KaniDiagnostic>> {
    let mut identities = BTreeSet::new();
    for (index, dependency) in dependencies.iter().enumerate() {
        let base_path = format!("dependencies[{index}]");
        validate_plain_identity(dependency.proof_id, &format!("{base_path}.proof_id"))?;
        if dependency.proof_id == proof_id || !identities.insert(dependency.proof_id) {
            return Err(single_diagnostic(
                KaniErrorCode::InvalidDependency,
                &format!("{base_path}.proof_id"),
                "dependency identities must be unique and distinct from the root proof",
            ));
        }
        match (dependency.kind, dependency.state) {
            (
                ProofDependencyKind::Required,
                ProofDependencyState::Passed
                | ProofDependencyState::Missing
                | ProofDependencyState::Failed,
            ) if dependency.original_path.is_none() && dependency.replacement_path.is_none() => {}
            (ProofDependencyKind::Assumed, ProofDependencyState::Assumed) => {
                let Some(original_path) = dependency.original_path else {
                    return Err(single_diagnostic(
                        KaniErrorCode::InvalidDependency,
                        &base_path,
                        "an assumed dependency requires exactly one predicate path",
                    ));
                };
                if dependency.replacement_path.is_some() {
                    return Err(single_diagnostic(
                        KaniErrorCode::InvalidDependency,
                        &base_path,
                        "an assumed dependency cannot declare a replacement path",
                    ));
                }
                validate_path(original_path, &format!("{base_path}.original_path"))?;
            }
            (ProofDependencyKind::Stubbed, ProofDependencyState::Stubbed) => {
                let (Some(original_path), Some(replacement_path)) =
                    (dependency.original_path, dependency.replacement_path)
                else {
                    return Err(single_diagnostic(
                        KaniErrorCode::InvalidDependency,
                        &base_path,
                        "a stubbed dependency requires original and replacement paths",
                    ));
                };
                validate_path(original_path, &format!("{base_path}.original_path"))?;
                validate_path(replacement_path, &format!("{base_path}.replacement_path"))?;
            }
            _ => {
                return Err(single_diagnostic(
                    KaniErrorCode::InvalidDependency,
                    &base_path,
                    "dependency kind, state, and source paths are inconsistent",
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn validate_plain_identity(value: &str, path: &str) -> Result<(), Vec<KaniDiagnostic>> {
    if value.is_empty() || value.chars().any(char::is_control) {
        Err(single_diagnostic(
            KaniErrorCode::InvalidIdentity,
            path,
            "identity must be non-empty and contain no control characters",
        ))
    } else {
        Ok(())
    }
}

pub(super) fn validate_path(value: &str, path: &str) -> Result<(), Vec<KaniDiagnostic>> {
    if syn::parse_str::<syn::Path>(value).is_err() {
        Err(single_diagnostic(
            KaniErrorCode::InvalidIdentity,
            path,
            "value must be a valid Rust path",
        ))
    } else {
        Ok(())
    }
}

pub(super) fn single_diagnostic(
    code: KaniErrorCode,
    path: &str,
    message: &str,
) -> Vec<KaniDiagnostic> {
    vec![KaniDiagnostic {
        code,
        terminal_state: code.terminal_state(),
        generation_code: None,
        path: path.to_owned(),
        source_span: None,
        message: message.to_owned(),
    }]
}

// `?Sized` so an unsized `[T]` slice (e.g. `&[ProofDependencyEdge]`) can be passed directly, with
// no intermediate owned `Vec` allocation at the call site, alongside every already-`Sized` caller
// (ir#80 review finding F10).
pub(crate) fn deterministic_json(value: &(impl Serialize + ?Sized)) -> Result<String, String> {
    let mut bytes = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    String::from_utf8(bytes).map_err(|error| error.to_string())
}

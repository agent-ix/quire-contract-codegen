//! The generation-result and claim vocabulary every oracle generator returns.
//!
//! Each generator keeps its own item, claim and refusal types, because those
//! are family-specific. The shapes they share live here once: the
//! generated-or-refused disposition of one claim, the claim map of one
//! generation, a whole-generation failure, and the upstream work an item can
//! be blocked on. A new generator returns these with its own type parameters
//! and does not edit this module.

use quire_contract_model::CheckedSemanticId;
use serde::Serialize;

/// Upstream work an item, or a whole generation, is blocked on.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum UpstreamBlocker {
    /// Model, relation and reference-reaching semantics, including
    /// `reference` composite operands.
    #[serde(rename = "agent-ix/quire-spec-language#120")]
    QuireSpecLanguage120,
    /// State, temporal and protocol semantics.
    #[serde(rename = "agent-ix/quire-spec-language#121")]
    QuireSpecLanguage121,
    /// Function application: Contract Runtime publishes no
    /// function-application operator surface to call.
    #[serde(rename = "agent-ix/quire-contract-runtime#34")]
    QuireContractRuntime34,
    /// The generator classified the item from its body and the request's own
    /// descriptor and did not confirm the node's catalogued operation, so it
    /// reports the descriptor-derived identity instead. The cases are listed
    /// on [`OperationProvenance::CallerDeclared`](crate::oracle::scalar::OperationProvenance::CallerDeclared).
    #[serde(rename = "operation identity not consumed by codegen's generators")]
    OperationIdentityNotConsumed,
}

/// One claim's outcome: an emitted oracle, or a typed refusal and nothing
/// emitted.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum ClaimDisposition<G, R> {
    /// The oracle was emitted.
    Generated(Box<G>),
    /// Nothing was emitted.
    Refused {
        /// The typed reason.
        refusal: R,
    },
}

/// The per-item claim map of one generation, serialized as its
/// `claim-map.json`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClaimMap<C> {
    /// Source package identity.
    pub package_id: CheckedSemanticId,
    /// Upstream gaps every entry is subject to.
    pub blocked: Vec<UpstreamBlocker>,
    /// Entries, in the order the producing generator documents on its
    /// `*Oracles::claim_map`.
    pub items: Vec<C>,
}

/// Whole-generation failure; per-item problems are refusals, not errors.
/// Each generator documents which of these it can return:
/// `LocationMapSerialization` comes only from the function generator, the one
/// that emits a location map. `UnknownRuntimeVariant` comes from the
/// generators that match one of Contract Runtime's `#[non_exhaustive]` enums:
/// the exact-scalar generator (`check_parameters` and its shape, identity and
/// rendering sites, roughly two dozen in all) and the composite-equality
/// generator: unlike an ordinary per-item refusal, RT returning a variant this generator's own
/// closed match was not written to expect means the generator's
/// understanding of that type has gone stale, which puts every item's
/// result in doubt, not just the one that surfaced it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OracleGenerationError {
    /// Generated source exceeds the effective caller-selected byte ceiling.
    SourceTooLarge {
        /// First source size that exceeds the selected ceiling.
        bytes: usize,
        /// Effective source byte ceiling.
        limit: usize,
        /// Stable name of the setting that selected the ceiling.
        setting: &'static str,
    },
    /// A caller-selected type-resolution limit cannot represent its first denied count.
    InvalidTypeResolutionWorkLimit,
    /// One generated artifact exceeds its publication byte ceiling.
    ArtifactTooLarge {
        /// Artifact byte count.
        bytes: usize,
        /// Publication ceiling per artifact.
        limit: usize,
    },
    /// The generated crate has more artifacts than publication permits.
    ArtifactCountExceeded {
        /// Artifact count.
        count: usize,
        /// Publication ceiling for artifacts.
        limit: usize,
    },
    /// The complete generated bundle exceeds its publication byte ceiling.
    BundleTooLarge {
        /// Bundle byte count.
        bytes: usize,
        /// Publication ceiling for a bundle.
        limit: usize,
    },
    /// The claim map could not be serialized.
    ClaimMapSerialization,
    /// The location map could not be serialized.
    LocationMapSerialization,
    /// A Contract Runtime `#[non_exhaustive]` enum yielded a variant this
    /// generator's own exhaustive match does not know.
    UnknownRuntimeVariant {
        /// The RT enum's name, e.g. `"IntegerDomain"`.
        enum_name: &'static str,
    },
}

impl OracleGenerationError {
    /// The refusal for a variant of the Contract Runtime enum `enum_name`
    /// that a closed generator match does not know. Every catch-all arm over
    /// an RT `#[non_exhaustive]` enum returns this instead of panicking.
    pub(crate) const fn unknown_variant(enum_name: &'static str) -> Self {
        Self::UnknownRuntimeVariant { enum_name }
    }
}

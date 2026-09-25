//! The generation-result and claim vocabulary every oracle generator returns.
//!
//! Each generator keeps its own item, claim and refusal types, because those
//! are family-specific. The shapes they share live here once: the
//! generated-or-refused disposition of one claim, the claim map of one
//! generation, a whole-generation failure, and the upstream work an item can
//! be blocked on. A new generator returns these with its own type parameters
//! and does not edit this module.

use quire_contract_ir::CheckedSemanticId;
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
    /// on [`crate::OperationProvenance::CallerDeclared`].
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
    /// The generator's claim-map version.
    pub version: &'static str,
    /// Source package identity.
    pub package_id: CheckedSemanticId,
    /// Pinned runtime revision the oracles call.
    pub runtime_revision: &'static str,
    /// Upstream gaps every entry is subject to.
    pub blocked: Vec<UpstreamBlocker>,
    /// Entries, in the order the producing generator documents on its
    /// `*Oracles::claim_map`.
    pub items: Vec<C>,
}

/// Whole-generation failure; per-item problems are refusals, not errors.
/// Each generator documents which of these it can return:
/// `LocationMapSerialization` comes only from the function generator, the one
/// that emits a location map. `UnknownRuntimeVariant` comes only from a
/// generator that checks parameters against one of Contract Runtime's
/// `#[non_exhaustive]` enums (`exact_scalar`'s `check_parameters`): unlike an
/// ordinary per-item refusal, RT returning a variant this generator's own
/// closed match was not written to expect means the generator's
/// understanding of that type has gone stale, which puts every item's
/// result in doubt, not just the one that surfaced it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OracleGenerationError {
    /// The generated source exceeds [`crate::MAX_GENERATED_SOURCE_BYTES`].
    SourceTooLarge {
        /// Generated size.
        bytes: usize,
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

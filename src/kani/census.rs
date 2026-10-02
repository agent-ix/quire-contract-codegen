//! The proof-dependency census types.
//!
//! The FR-015 census input and the corpus use these types, and they survive the retirement of
//! the V1 bundle in `kani`. This module depends on no other module of this crate.

use serde::{Deserialize, Serialize};

/// Kind of proof dependency declared by one generated harness.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofDependencyKind {
    /// A separately executed proof that must pass.
    Required,
    /// A Boolean dependency predicate introduced with `kani::assume`.
    Assumed,
    /// A function replacement introduced with `kani::stub`.
    Stubbed,
}

/// State of one declared proof dependency at generation time.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofDependencyState {
    /// A required dependency proof passed under its retained identity.
    Passed,
    /// A required dependency proof has no retained result.
    Missing,
    /// A required dependency proof failed.
    Failed,
    /// The dependency is explicitly assumed rather than proved.
    Assumed,
    /// The dependency implementation is explicitly replaced by a stub.
    Stubbed,
}

/// Generation-time readiness derived from the complete dependency census.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofReadiness {
    /// Every required proof passed and no assumptions or stubs are present.
    Ready,
    /// Required proofs passed, but an assumption or stub makes the proof conditional.
    Conditional,
    /// A required proof is missing or failed.
    Incomplete,
}

/// One caller-declared proof dependency.
#[derive(Clone, Copy, Debug)]
pub struct ProofDependencyRequest<'a> {
    /// Stable dependency proof identity.
    pub proof_id: &'a str,
    /// Relationship to the generated root proof.
    pub kind: ProofDependencyKind,
    /// Current retained dependency state.
    pub state: ProofDependencyState,
    /// Assumption predicate or original stubbed function path, when required by `kind`.
    pub original_path: Option<&'a str>,
    /// Stub replacement function path, present only for `Stubbed`.
    pub replacement_path: Option<&'a str>,
}

/// One normalized proof dependency edge in the generated graph.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProofDependencyEdge {
    /// Stable dependency proof identity.
    pub proof_id: String,
    /// Dependency kind.
    pub kind: ProofDependencyKind,
    /// Dependency state.
    pub state: ProofDependencyState,
    /// Generated assumption/stub source-site identity, or `None` for required proof edges.
    pub source_site: Option<String>,
}

pub(crate) fn normalize_dependencies(
    dependencies: &[ProofDependencyRequest<'_>],
) -> Vec<ProofDependencyEdge> {
    let mut result = dependencies
        .iter()
        .map(|dependency| ProofDependencyEdge {
            proof_id: dependency.proof_id.to_owned(),
            kind: dependency.kind,
            state: dependency.state,
            source_site: match dependency.kind {
                ProofDependencyKind::Required => None,
                ProofDependencyKind::Assumed => {
                    Some(dependency_site("assumption", dependency.proof_id))
                }
                ProofDependencyKind::Stubbed => Some(dependency_site("stub", dependency.proof_id)),
            },
        })
        .collect::<Vec<_>>();
    result.sort_by(|left, right| left.proof_id.cmp(&right.proof_id));
    result
}

pub(crate) fn dependency_readiness(dependencies: &[ProofDependencyEdge]) -> ProofReadiness {
    if dependencies.iter().any(|dependency| {
        dependency.kind == ProofDependencyKind::Required
            && matches!(
                dependency.state,
                ProofDependencyState::Missing | ProofDependencyState::Failed
            )
    }) {
        ProofReadiness::Incomplete
    } else if dependencies.iter().any(|dependency| {
        matches!(
            dependency.kind,
            ProofDependencyKind::Assumed | ProofDependencyKind::Stubbed
        )
    }) {
        ProofReadiness::Conditional
    } else {
        ProofReadiness::Ready
    }
}

pub(crate) fn dependency_site(kind: &str, proof_id: &str) -> String {
    format!("{kind}:{proof_id}")
}

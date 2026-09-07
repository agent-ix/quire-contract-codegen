//! Complete-package Boolean lowering through the public IR binding boundary.

use std::collections::BTreeSet;

use quire_contract_ir::{BoundPackage, CanonicalDigest, ClauseRef};

use crate::{
    oracle::{generate_oracle_with_derivation, oracle_symbol},
    publication::{MAX_ARTIFACTS, MAX_ARTIFACT_BYTES, MAX_BUNDLE_BYTES},
    ArtifactBundle, AttestationContext, GenerationDiagnostic, OracleArtifactBundle, OracleRequest,
    PublicationDiagnostic,
};

/// Complete generation result, distinct from a native execution or coverage result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundOracleGeneration {
    /// Every executable clause was lowered, with a validated publishable bundle.
    Generated(GeneratedBoundOracles),
    /// The valid package contains no executable clauses and no publishable artifacts.
    NoExecutable(NoExecutableOracles),
}

/// Identity of a valid package with no executable work; never an attestation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoExecutableOracles {
    bound_digest: CanonicalDigest,
    informational: Vec<ClauseRef>,
}

impl NoExecutableOracles {
    /// Canonical public IR binding identity.
    #[must_use]
    pub fn bound_digest(&self) -> CanonicalDigest {
        self.bound_digest
    }
    /// Complete ordered informational population, excluded from executable work.
    #[must_use]
    pub fn informational(&self) -> &[ClauseRef] {
        &self.informational
    }
}

/// Immutable output for one fully identified executable clause.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundOracleClause {
    identity: ClauseRef,
    declaration_digest: CanonicalDigest,
    expression_digest: CanonicalDigest,
    bundle: OracleArtifactBundle,
}

impl BoundOracleClause {
    /// Full package/requirement/revision/clause identity.
    #[must_use]
    pub fn identity(&self) -> &ClauseRef {
        &self.identity
    }
    /// Canonical identity of the validated declaration environment.
    #[must_use]
    pub fn declaration_digest(&self) -> CanonicalDigest {
        self.declaration_digest
    }
    /// Canonical identity of the validated typed expression.
    #[must_use]
    pub fn expression_digest(&self) -> CanonicalDigest {
        self.expression_digest
    }
    /// Source, map, and source-generation attestation bodies for this clause.
    #[must_use]
    pub fn bundle(&self) -> &OracleArtifactBundle {
        &self.bundle
    }
}

/// Immutable complete executable population and independently publishable artifact set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedBoundOracles {
    bound_digest: CanonicalDigest,
    clauses: Vec<BoundOracleClause>,
    informational: Vec<ClauseRef>,
    bundle: ArtifactBundle,
}

impl GeneratedBoundOracles {
    /// Canonical public IR binding identity used in every generation attestation.
    #[must_use]
    pub fn bound_digest(&self) -> CanonicalDigest {
        self.bound_digest
    }
    /// Every executable clause, exactly once and ordered by full clause reference.
    #[must_use]
    pub fn clauses(&self) -> &[BoundOracleClause] {
        &self.clauses
    }
    /// Complete informational population, never counted as executable coverage.
    #[must_use]
    pub fn informational(&self) -> &[ClauseRef] {
        &self.informational
    }
    /// Validated complete set; publication requires a separate explicit call.
    #[must_use]
    pub fn bundle(&self) -> &ArtifactBundle {
        &self.bundle
    }
}

/// Whole-batch failure; never carries partial generated artifacts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundGenerationError {
    /// Shared publication artifact-count or byte limits would be exceeded.
    ResourceLimitExceeded,
    /// Two distinct clauses claim one generated symbol.
    NameCollision(ClauseRef),
    /// A fully identified executable clause could not be lowered without approximation.
    Clause {
        /// Complete identity of the failing clause.
        identity: ClauseRef,
        /// Existing Boolean-lowering diagnostics, with their terminal states.
        diagnostics: Vec<GenerationDiagnostic>,
    },
    /// Complete in-memory bundle validation failed; no publication was attempted.
    Bundle(PublicationDiagnostic),
}

/// Lowers every executable clause through IR's immutable validated public projection.
///
/// This is an in-process generation operation, not a CLI or native campaign run. It does not infer
/// pre/post pairings, omit unsupported clauses, publish files, or compute coverage sufficiency.
/// Trace: TC-001, TC-002
// Implements: FR-001
pub fn generate_bound_oracles(
    package: &BoundPackage,
    attestation: AttestationContext<'_>,
) -> Result<BoundOracleGeneration, BoundGenerationError> {
    if package.clauses().is_empty() {
        return Ok(BoundOracleGeneration::NoExecutable(NoExecutableOracles {
            bound_digest: package.digest(),
            informational: package.informational().to_vec(),
        }));
    }
    preflight(package)?;
    let digest = package.digest().to_string();
    let mut clauses = Vec::with_capacity(package.clauses().len());
    let mut artifacts = Vec::with_capacity(package.clauses().len() * 4);
    let mut total_bytes = 0usize;
    for clause in package.clauses() {
        let identity = clause.identity();
        let bundle = generate_oracle_with_derivation(
            &OracleRequest {
                requirement: identity.requirement(),
                clause: identity.clause(),
                expression: clause.expression(),
                attestation,
            },
            Some(&digest),
        )
        .map_err(|diagnostics| BoundGenerationError::Clause {
            identity: identity.clone(),
            diagnostics,
        })?;
        for artifact in [
            &bundle.rust,
            &bundle.source_map,
            &bundle.rust_attestation,
            &bundle.source_map_attestation,
        ] {
            reserve_bytes(&mut total_bytes, artifact.contents.len())?;
            artifacts.push(artifact.clone());
        }
        clauses.push(BoundOracleClause {
            identity: identity.clone(),
            declaration_digest: clause.declaration_digest(),
            expression_digest: clause.expression_digest(),
            bundle,
        });
    }
    let bundle = ArtifactBundle::new(artifacts).map_err(BoundGenerationError::Bundle)?;
    Ok(BoundOracleGeneration::Generated(GeneratedBoundOracles {
        bound_digest: package.digest(),
        clauses,
        informational: package.informational().to_vec(),
        bundle,
    }))
}

fn preflight(package: &BoundPackage) -> Result<(), BoundGenerationError> {
    if package.clauses().len() > MAX_ARTIFACTS / 4 {
        return Err(BoundGenerationError::ResourceLimitExceeded);
    }
    let mut symbols = BTreeSet::new();
    for clause in package.clauses() {
        let id = clause.identity();
        let owner = id.requirement();
        if !symbols.insert(oracle_symbol(
            owner.package().as_str(),
            owner.requirement().as_str(),
            owner.revision().get(),
            id.clause().as_str(),
        )) {
            return Err(BoundGenerationError::NameCollision(id.clone()));
        }
    }
    Ok(())
}

fn reserve_bytes(total: &mut usize, bytes: usize) -> Result<(), BoundGenerationError> {
    let next = total.saturating_add(bytes);
    if bytes > MAX_ARTIFACT_BYTES || next > MAX_BUNDLE_BYTES {
        return Err(BoundGenerationError::ResourceLimitExceeded);
    }
    *total = next;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-002: accounting control, not a claim to have allocated a maximum-sized bundle.
    #[test]
    fn incremental_bytes_refuse_one_past_each_bound_without_advancing_budget() {
        let mut total = 0;
        assert!(reserve_bytes(&mut total, MAX_ARTIFACT_BYTES).is_ok());
        assert!(reserve_bytes(&mut total, MAX_ARTIFACT_BYTES + 1).is_err());
        assert_eq!(total, MAX_ARTIFACT_BYTES);
        total = MAX_BUNDLE_BYTES - MAX_ARTIFACT_BYTES;
        assert!(reserve_bytes(&mut total, MAX_ARTIFACT_BYTES).is_ok());
        assert!(reserve_bytes(&mut total, 1).is_err());
        assert_eq!(total, MAX_BUNDLE_BYTES);
        assert!(reserve_bytes(&mut total, usize::MAX).is_err());
        assert_eq!(total, MAX_BUNDLE_BYTES);
    }
}

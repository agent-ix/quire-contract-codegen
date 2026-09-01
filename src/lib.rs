//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

// Implements: FR-001
mod oracle;
// Implements: FR-003
mod kani;

pub use kani::{
    generate_kani_bundle, KaniArtifactBundle, KaniBackendIdentity, KaniDerivationManifest,
    KaniDiagnostic, KaniErrorCode, KaniExtension, KaniRequest, KaniSolver, ProofDependencyEdge,
    ProofDependencyGraph, ProofDependencyKind, ProofDependencyRequest, ProofDependencyState,
    ProofReadiness, KANI_ADAPTER_PROFILE, KANI_BACKEND_VERSION,
};

pub use oracle::{
    generate_boolean_oracle, generator_source_is_dirty, Artifact, CodegenExtension,
    DerivationManifest, DigestIdentity, GenerationDiagnostic, GenerationEnvironment,
    GenerationErrorCode, GenerationProvenance, GenerationResult, GenerationTerminalState,
    ManifestArtifact, ManifestContext, NoBackend, OracleArtifactBundle, OracleRequest,
    ProducerIdentity, SchemaIdentity, SourceRegion, GENERATOR_SOURCE_REVISION,
    IR_CANDIDATE_REVISION, MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION,
};

//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

// Implements: FR-001
mod oracle;
// Implements: FR-005, NFR-001
mod publication;
// Implements: FR-002
mod harness;
// Implements: FR-002
mod strategy;

pub use harness::{generate_tristate_harness, HarnessDiagnostic, HarnessErrorCode, HarnessRequest};
pub use publication::{
    write_bundle_atomic, ArtifactBundle, PublicationDiagnostic, PublicationErrorCode,
    PublishedBundleIdentity,
};
pub use strategy::{
    generate_enum_strategy, generate_i64_strategy, EnumStrategyCampaign, EnumStrategyRequest,
    StrategyCampaign, StrategyConstraint, StrategyDiagnostic, StrategyErrorCode, StrategyRequest,
};

pub use oracle::{
    generate_boolean_oracle, generator_source_is_dirty, Artifact, AttestationCommand,
    AttestationContext, AttestationEnvironment, AttestationResult, AttestationTool,
    GeneratedArtifactBundle, GenerationDiagnostic, GenerationErrorCode, GenerationTerminalState,
    OracleArtifactBundle, OracleRequest, ProofAttestationBody, SourceRegion,
    GENERATOR_SOURCE_REVISION, IR_CANDIDATE_REVISION, MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION,
};

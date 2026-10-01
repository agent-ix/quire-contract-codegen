//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod bounded_collections;
mod bounded_kani_corpus;
mod bounded_kani_profile;
// The leaf directory: artifact, diagnostic, identity, profile and source-map (AD-004 step 2c).
// `core` here is this crate's own layout module (the AD names the directory `core/`). In this
// file it shadows the extern `core` crate, so `crate::core::` is the explicit spelling.
mod core;
mod definedness_arithmetic;
mod finite_reference_graphs;
mod kani;
// Proof-dependency census types (AD-004 step 2b); becomes `kani/census.rs`.
mod kani_census;
// Harness and identity record types (AD-004 step 2b); becomes `kani/identity.rs`.
// Implements: FR-015
mod kani_identity;
// The oracle subsystem.
mod oracle;
// Implements: FR-005, NFR-001
mod publication;
// Implements: FR-002
mod harness;
// Implements: FR-015
mod kani_obligations;
// Implements: FR-019
mod capability;
// Implements: FR-022
mod routed_generation;
// Implements: FR-015 (IR-412: state-clause operation contract and frame effects).
mod state_frame;
// Implements: FR-017
mod kani_execution;
mod kani_transcript;
// IR-211: joins a real Kani witness to the generator's own persisted obligation schema.
mod kani_witness_join;
// Implements: FR-016
mod spine_replay;
// Implements: FR-015-AC-33
mod frame_replay;
// Implements: FR-002
mod strategy;
// Implements: FR-004 (bounded observation primitives; no aggregate coverage verdict).
mod vacuity;
// Implements: FR-004 (complete domain observations, always unqualified).
mod bound_coverage;
pub mod bound_strategy;

pub use bound_strategy::{generate_bound_strategy, BoundStrategyPopulation, BoundStrategyRequest};

pub use bounded_collections::prepare_bounded_collection_query;
pub use bounded_kani_corpus::{
    generate_bounded_kani_corpus_case, BoundedCorpusArtifacts, BoundedCorpusCase,
    BoundedCorpusFamily, BoundedCorpusRequest, CorpusProofDependencyGraph, EmittedCorpusIdentities,
    CORPUS_PROOF_GRAPH_SCHEMA,
};
pub use bounded_kani_profile::{classify_bounded_kani_profile, BoundedKaniProfile};
pub use definedness_arithmetic::prepare_checked_arithmetic;
pub use finite_reference_graphs::prepare_finite_graph_reaches;
pub use oracle::claim::{ClaimDisposition, ClaimMap, OracleGenerationError, UpstreamBlocker};
pub use oracle::scalar::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, BoundForm, ClaimDerivationRefusal,
    DecimalOperator, ExactScalarClaim, ExactScalarItem, ExactScalarOperation, ExactScalarOracles,
    ExactScalarRefusal, GeneratedScalarClaim, IeeeArithmeticOperator, IntegerOperator,
    OperationClaim, OperationProvenance, OrderingOperandKind, QuantityOperator, RationalOperator,
    ScalarForm, EXACT_SCALAR_CRATE_NAME, SCALAR_LOWERING_SUPPORTED_TAGS,
    SCALAR_LOWERING_WORK_LIMIT,
};

pub use oracle::equality::{
    generate_composite_equality_oracles, CompositeEqualityClaim, CompositeEqualityItem,
    CompositeEqualityOracles, CompositeEqualityRefusal, CompositeOperationClaim,
    CompositeOperationProvenance, DeclarationRefusalCause, EqualityOperandDescriptor,
    EqualityOperatorKind, GeneratedCompositeEqualityClaim, IllTypedCauseKind, RecordedDescriptor,
    RecordedSchedule, RecursionEdgesKind, COMPOSITE_EQUALITY_CRATE_NAME,
    COMPOSITE_EQUALITY_LOWERING_WORK_LIMIT,
};

pub use oracle::function::{
    generate_exact_function_oracles, CallPointKind, ExactFunctionBody, ExactFunctionClaim,
    ExactFunctionDeclaration, ExactFunctionItem, ExactFunctionOracles, ExactFunctionRefusal,
    FunctionParameter, GeneratedExactFunctionClaim, LocationMapEntry, RecordedLocation,
    RecordedOrigin, EXACT_FUNCTION_CRATE_NAME, EXACT_FUNCTION_LOWERING_WORK_LIMIT,
};

pub use bound_coverage::{
    analyze_bound_coverage, ArtifactBytes, BoundAnalysisState, BoundCoverageAnalysis,
    BoundCoverageInputs, BOUND_COVERAGE_FORMAT, BOUND_COVERAGE_SCHEMA, MAX_ANALYSIS_BYTES,
};

pub use vacuity::{
    classify_clause, parse_llvm_coverage, ClauseCoverage, CoverageDiagnostic, CoverageErrorCode,
    LlvmCoverage, ProbeObservation, MAX_COVERAGE_BYTES,
};

pub use oracle::bound_v1::{
    generate_bound_oracles, BoundGenerationError, BoundOracleClause, BoundOracleGeneration,
    GeneratedBoundOracles, NoExecutableOracles,
};

pub use crate::core::identity::{HarnessPath, HarnessSymbol, ModuleSymbol, SymbolError};
pub use crate::core::profile::RUNTIME_DEPENDENCY_SOURCE;
pub use capability::{
    negotiate_backend_provider, BackendDescriptor, BackendKind, BackendProviderEnvelope, Candidate,
    Candidates, CapabilityKind, Cause, Disposition, EnvelopeRefusal, ExtentClassification,
    ItemSettlement, Mode, RequestItem, RequestedKind, BACKEND_PROVIDER_CONTRACT,
    CAPABILITY_VOCABULARY,
};
pub use harness::{generate_tristate_harness, HarnessDiagnostic, HarnessErrorCode, HarnessRequest};
pub use kani::{
    generate_kani_bundle, KaniArtifactBundle, KaniBindingRole, KaniDiagnostic, KaniErrorCode,
    KaniIntegerBounds, KaniPrimitiveType, KaniRequest, KaniSolver, KaniSubjectBinding,
    ProofDependencyGraph,
};
pub use kani_census::{
    ProofDependencyEdge, ProofDependencyKind, ProofDependencyRequest, ProofDependencyState,
    ProofReadiness,
};
pub use kani_execution::{
    classify_kani_run, execute_kani_obligation, kani_launch_command, launch_evidence,
    run_launcher_with_timeout, ClassifiedRun, KaniExecutableHarness, KaniExecutionEvidence,
    KaniExecutionRefusal, KaniExecutionRequest, KaniInconclusiveReason, KaniInstallation,
    KaniRunOutcome, KaniTool, KaniToolError, LaunchOutcome,
};
pub use kani_transcript::{
    KaniCheckClass, KaniCheckLocation, KaniCheckResult, KaniCheckStatus, KaniReportRefusal,
    OtherCheckClass,
};
pub use routed_generation::{
    generate_routed, GenerationContexts, KaniGenerationContext, KindOutput, RoutedGeneration,
    RoutedGenerationError, RoutedGenerationItem, RoutedItemOutput,
};

pub use crate::core::artifact::{
    Artifact, ArtifactBundle, PublicationDestinationState, PublicationDiagnostic,
    PublicationErrorCode, MAX_GENERATED_SOURCE_BYTES,
};
pub use crate::core::diagnostic::{
    GenerationDiagnostic, GenerationErrorCode, GenerationTerminalState,
};
pub use crate::core::source_map::{SourceProbe, SourceRegion};
pub use frame_replay::{FrameReplay, FrameReplayError, FrameReplayInputs, ProvidedDocument};
pub use kani_identity::{
    EmbeddedOracle, KaniObligationHarness, KaniObligationIdentity, KaniScalarObligationHarness,
    ObligationBinding, ObligationKind, ScalarObligationArgument, ScalarObligationIdentity,
    StateComparison, StateFieldDomain, StateFrameHarness, StateFrameIdentity, StateFrameProperty,
    StateFrameScope,
};
pub use kani_obligations::{
    negotiate_kani_obligations, DerivedDomain, InvalidObligationItem, KaniObligationError,
    KaniObligationOutcome, KaniObligationRequest, ObligationDisposition, ObligationItem,
    ObligationRecord, ObligationSubject, UnsupportedObligation, MAX_OBLIGATION_ITEMS,
    MAX_OBLIGATION_UNWIND,
};
pub use kani_witness_join::{decode_falsification, DecodeFailure};
pub use publication::{write_bundle_atomic, PublishedBundleIdentity};
pub use spine_replay::{
    replay_counterexample, replay_falsification, DependencyLock, DependencyLockError,
    EvidenceFailureCause, LockedSource, ReplayInputs, ReplayPackage, ReplayPackageError,
    ReplayParameter, ReplayVerdict, SpineReplayError,
};
pub use state_frame::{
    generate_state_frame_obligations, StateFrameObligations, StateFrameRefusal, StateFrameRequest,
    UnsupportedFrameEffect,
};
pub use strategy::{
    generate_enum_strategy, generate_i64_strategy, EnumStrategyCampaign, EnumStrategyRequest,
    StrategyCampaign, StrategyConstraint, StrategyDiagnostic, StrategyErrorCode, StrategyRequest,
};

pub use oracle::boolean_v1::{
    generate_boolean_oracle, GeneratedArtifactBundle, OracleArtifactBundle, OracleRequest,
};

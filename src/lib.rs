//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod bound;
mod bounded_collections;
mod bounded_kani_corpus;
mod bounded_kani_profile;
mod definedness_arithmetic;
mod finite_reference_graphs;
mod kani;
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
// Implements: FR-029
mod kani_terminal;
mod kani_transcript;
// IR-211: joins a real Kani witness to the generator's own persisted obligation schema.
mod kani_witness_join;
// Implements: FR-016
mod spine_replay;
// Implements: FR-015-AC-33
mod frame_replay;
// Implements: FR-002
mod strategy;
// Shared generation-result and claim vocabulary (FR-014, FR-018, FR-021).
mod generation;
// Implements: FR-014
mod exact_scalar;
// Implements: FR-018
mod composite_equality;
// Implements: FR-021
mod exact_function;
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
pub use exact_scalar::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, BoundForm, ClaimDerivationRefusal,
    DecimalOperator, ExactScalarClaim, ExactScalarItem, ExactScalarOperation, ExactScalarOracles,
    ExactScalarRefusal, GeneratedScalarClaim, IeeeArithmeticOperator, IntegerOperator,
    OperationClaim, OperationProvenance, OrderingOperandKind, QuantityOperator, RationalOperator,
    ScalarForm, EXACT_SCALAR_CRATE_NAME, SCALAR_LOWERING_SUPPORTED_TAGS,
    SCALAR_LOWERING_WORK_LIMIT,
};
pub use finite_reference_graphs::prepare_finite_graph_reaches;
pub use generation::{ClaimDisposition, ClaimMap, OracleGenerationError, UpstreamBlocker};

pub use composite_equality::{
    generate_composite_equality_oracles, CompositeEqualityClaim, CompositeEqualityItem,
    CompositeEqualityOracles, CompositeEqualityRefusal, CompositeOperationClaim,
    CompositeOperationProvenance, DeclarationRefusalCause, EqualityOperandDescriptor,
    EqualityOperatorKind, GeneratedCompositeEqualityClaim, IllTypedCauseKind, RecordedDescriptor,
    RecordedSchedule, RecursionEdgesKind, COMPOSITE_EQUALITY_CRATE_NAME,
    COMPOSITE_EQUALITY_LOWERING_WORK_LIMIT,
};

pub use exact_function::{
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

pub use bound::{
    generate_bound_oracles, BoundGenerationError, BoundOracleClause, BoundOracleGeneration,
    GeneratedBoundOracles, NoExecutableOracles,
};

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
    ProofDependencyEdge, ProofDependencyGraph, ProofDependencyKind, ProofDependencyRequest,
    ProofDependencyState, ProofReadiness,
};
pub use kani_execution::{
    classify_kani_run, execute_kani_obligation, kani_launch_command, launch_evidence,
    run_launcher_with_timeout, ClassifiedRun, KaniExecutableHarness, KaniExecutionEvidence,
    KaniExecutionRefusal, KaniExecutionRequest, KaniInconclusiveReason, KaniInstallation,
    KaniRunOutcome, KaniTool, KaniToolError, LaunchOutcome, REPORT_FILE,
};
pub use kani_terminal::{ir_outcome_terminal_value, proof_category, terminal_value};
pub use kani_transcript::{
    KaniCheckClass, KaniCheckLocation, KaniCheckResult, KaniCheckStatus, KaniReportRefusal,
};
pub use routed_generation::{
    generate_routed, GenerationContexts, KaniGenerationContext, KindOutput, RoutedGeneration,
    RoutedGenerationError, RoutedGenerationItem, RoutedItemOutput,
};

pub use frame_replay::{FrameReplay, FrameReplayError, FrameReplayInputs, ProvidedDocument};
pub use kani_obligations::{
    negotiate_kani_obligations, DerivedDomain, EmbeddedOracle, InvalidObligationItem,
    KaniObligationError, KaniObligationHarness, KaniObligationIdentity, KaniObligationOutcome,
    KaniObligationRequest, KaniScalarObligationHarness, ObligationBinding, ObligationDisposition,
    ObligationItem, ObligationKind, ObligationRecord, ObligationSubject, ScalarObligationArgument,
    ScalarObligationIdentity, UnsupportedObligation, MAX_OBLIGATION_ITEMS, MAX_OBLIGATION_UNWIND,
};
pub use kani_witness_join::{decode_falsification, DecodeFailure};
pub use publication::{
    write_bundle_atomic, ArtifactBundle, PublicationDestinationState, PublicationDiagnostic,
    PublicationErrorCode, PublishedBundleIdentity,
};
pub use spine_replay::{
    replay_counterexample, replay_falsification, DependencyLock, DependencyLockError,
    EvidenceFailureCause, LockedSource, ReplayInputs, ReplayPackage, ReplayPackageError,
    ReplayParameter, ReplayVerdict, SpineReplayError,
};
pub use state_frame::{
    generate_state_frame_obligations, StateComparison, StateFieldDomain, StateFrameHarness,
    StateFrameIdentity, StateFrameObligations, StateFrameProperty, StateFrameRefusal,
    StateFrameRequest, StateFrameScope, UnsupportedFrameEffect,
};
pub use strategy::{
    generate_enum_strategy, generate_i64_strategy, EnumStrategyCampaign, EnumStrategyRequest,
    StrategyCampaign, StrategyConstraint, StrategyDiagnostic, StrategyErrorCode, StrategyRequest,
};

pub use oracle::{
    generate_boolean_oracle, Artifact, GeneratedArtifactBundle, GenerationDiagnostic,
    GenerationErrorCode, GenerationTerminalState, OracleArtifactBundle, OracleRequest, SourceProbe,
    SourceRegion, MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION,
};

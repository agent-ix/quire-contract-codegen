//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

// Implements: FR-001
mod oracle;
// Implements: FR-001
mod bound;
// Implements: FR-005, NFR-001
mod publication;
// Implements: FR-002
mod harness;
// Implements: FR-003
mod kani;
// Implements: FR-015
mod kani_obligations;
// Implements: FR-019
mod capability;
// Implements: FR-017
mod kani_execution;
// IR-211: joins a real Kani witness to the generator's own persisted obligation schema.
mod kani_witness_join;
// Implements: FR-007
mod bounded_kani_profile;
// Implements: FR-007
mod definedness_arithmetic;
// Implements: FR-007
mod bounded_collections;
// Implements: FR-007
mod finite_reference_graphs;
// Implements: FR-007
mod bounded_kani_replay;
// Implements: FR-007
mod bounded_kani_corpus;
// Implements: FR-002
mod strategy;
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
    BoundedCorpusFamily, BoundedCorpusRequest, EmittedCorpusIdentities,
};
pub use bounded_kani_profile::{classify_bounded_kani_profile, BoundedKaniProfile};
pub use bounded_kani_replay::replay_codegen_counterexample;
pub use definedness_arithmetic::prepare_checked_arithmetic;
pub use exact_scalar::{
    generate_exact_scalar_oracles, BoundForm, DecimalOperator, ExactScalarClaim,
    ExactScalarClaimMap, ExactScalarDisposition, ExactScalarGenerationError, ExactScalarItem,
    ExactScalarOperation, ExactScalarOracles, ExactScalarRefusal, GeneratedScalarClaim,
    IeeeArithmeticOperator, IntegerOperator, OperationClaim, OperationProvenance,
    OrderingOperandKind, QuantityOperator, RationalOperator, ScalarForm, UpstreamBlocker,
    EXACT_SCALAR_CLAIM_MAP_VERSION, EXACT_SCALAR_CRATE_NAME, SCALAR_LOWERING_SUPPORTED_TAGS,
    SCALAR_LOWERING_WORK_LIMIT,
};
pub use finite_reference_graphs::prepare_finite_graph_reaches;

pub use composite_equality::{
    generate_composite_equality_oracles, CompositeEqualityClaim, CompositeEqualityClaimMap,
    CompositeEqualityDisposition, CompositeEqualityGenerationError, CompositeEqualityItem,
    CompositeEqualityOracles, CompositeEqualityRefusal, CompositeOperationClaim,
    CompositeOperationProvenance, DeclarationRefusalCause, EqualityOperandDescriptor,
    EqualityOperatorKind, GeneratedCompositeEqualityClaim, IllTypedCauseKind, RecordedDescriptor,
    RecordedSchedule, RecursionEdgesKind, UpstreamBlocker as CompositeEqualityUpstreamBlocker,
    COMPOSITE_EQUALITY_CLAIM_MAP_VERSION, COMPOSITE_EQUALITY_CRATE_NAME,
    COMPOSITE_EQUALITY_LOWERING_WORK_LIMIT,
};

pub use exact_function::{
    generate_exact_function_oracles, CallPointKind, ExactFunctionBody, ExactFunctionClaim,
    ExactFunctionClaimMap, ExactFunctionDeclaration, ExactFunctionDisposition,
    ExactFunctionGenerationError, ExactFunctionItem, ExactFunctionOracles, ExactFunctionRefusal,
    FunctionParameter, GeneratedExactFunctionClaim, LocationMapEntry, RecordedLocation,
    RecordedOrigin, UpstreamBlocker as ExactFunctionUpstreamBlocker,
    EXACT_FUNCTION_CLAIM_MAP_VERSION, EXACT_FUNCTION_CRATE_NAME,
    EXACT_FUNCTION_LOWERING_WORK_LIMIT,
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
    negotiate_backend_provider, record_tool_probe, BackendDescriptor, BackendKind,
    BackendProviderEnvelope, Candidate, Candidates, CapabilityKind, Cause, Disposition,
    EnvelopeRefusal, ExtentClassification, ItemResult, ItemSettlement, Mode, ProbePhase,
    RequestItem, RequestedKind, RoutedItem, ToolObservation, BACKEND_PROVIDER_CONTRACT,
    CAPABILITY_VOCABULARY,
};
pub use harness::{generate_tristate_harness, HarnessDiagnostic, HarnessErrorCode, HarnessRequest};
pub use kani::{
    generate_kani_bundle, KaniArtifactBundle, KaniBindingRole, KaniDiagnostic, KaniErrorCode,
    KaniIntegerBounds, KaniPrimitiveType, KaniRequest, KaniSolver, KaniSubjectBinding,
    ProofDependencyEdge, ProofDependencyGraph, ProofDependencyKind, ProofDependencyRequest,
    ProofDependencyState, ProofReadiness, KANI_ADAPTER_PROFILE, KANI_BACKEND_VERSION,
};
pub use kani_execution::{
    classify_kani_run, execute_kani_obligation, file_sha256, kani_launch_command, launch_evidence,
    run_launcher_with_timeout, KaniExecutionEvidence, KaniExecutionRefusal, KaniExecutionRequest,
    KaniInconclusiveReason, KaniInstallation, KaniPinField, KaniRunOutcome, KaniTool,
    KaniToolError, KaniToolPins, LaunchOutcome, KANI_EXECUTION_SCHEMA,
};

pub use kani_obligations::{
    negotiate_kani_obligations, DerivedDomain, EmbeddedOracle, InvalidObligationItem,
    KaniObligationError, KaniObligationHarness, KaniObligationIdentity, KaniObligationOutcome,
    KaniObligationRequest, KaniScalarObligationHarness, ObligationBinding, ObligationDisposition,
    ObligationItem, ObligationKind, ObligationRecord, ObligationSubject, ScalarObligationArgument,
    ScalarObligationIdentity, UnsupportedObligation, KANI_OBLIGATION_PROFILE,
    KANI_OBLIGATION_SCHEMA, KANI_SCALAR_OBLIGATION_SCHEMA, MAX_OBLIGATION_ITEMS,
    MAX_OBLIGATION_UNWIND,
};
pub use kani_witness_join::{decode_falsification, witness_schema, WitnessSchemaError};
pub use publication::{
    write_bundle_atomic, ArtifactBundle, PublicationDestinationState, PublicationDiagnostic,
    PublicationErrorCode, PublishedBundleIdentity,
};
pub use strategy::{
    generate_enum_strategy, generate_i64_strategy, EnumStrategyCampaign, EnumStrategyRequest,
    StrategyCampaign, StrategyConstraint, StrategyDiagnostic, StrategyErrorCode, StrategyRequest,
};

pub use oracle::{
    generate_boolean_oracle, generator_source_is_dirty, Artifact, AttestationCommand,
    AttestationContext, AttestationEnvironment, AttestationResult, AttestationTool,
    GeneratedArtifactBundle, GenerationDiagnostic, GenerationErrorCode, GenerationTerminalState,
    OracleArtifactBundle, OracleRequest, ProofAttestationBody, SourceProbe, SourceRegion,
    GENERATOR_SOURCE_REVISION, IR_CANDIDATE_REVISION, MAX_GENERATED_SOURCE_BYTES, RUNTIME_REVISION,
};

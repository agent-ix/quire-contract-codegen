//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

// The leaf directory: artifact, diagnostic, identity, profile and source-map.
// `core` here is this crate's own layout module (the AD names the directory `core/`). In this
// file it shadows the extern `core` crate, so `crate::core::` is the explicit spelling.
mod core;
// The Kani subsystem.
mod kani;
// The oracle subsystem.
mod oracle;
// The publication subsystem.
mod publication;
// The routed subsystem.
mod routed;
// The replay subsystem.
mod replay;
// The evidence subsystem.
mod evidence;
// The strategy subsystem.
mod strategy;

pub use strategy::bound::census::{
    compute_census, render_boundary_constants, render_edge_constants, BoundaryCensus, CensusCase,
    CensusEdge, CensusNames, CensusRead, CensusTag, EdgeDirection, OutOfDomainCase,
    UnrepresentableEdge,
};
pub use strategy::bound::generation::{
    generate_bound_strategy, BoundStrategyPopulation, BoundStrategyRequest,
};
pub use strategy::bound::population::{
    render_population, side_values, Interval, PartnerRule, Population, PopulationRequest,
    PopulationSide, RenderedPopulation, SideValues, ValueSet, EXPECTATION_FIELD,
};
pub use strategy::bound::relation::{
    ComparisonOperator, Domain, OperandPosition, Partner, Relation,
};

pub use kani::generate::corpus::bounded_kani_corpus::{
    generate_bounded_kani_corpus_case, BoundedCorpusArtifacts, BoundedCorpusCase,
    BoundedCorpusError, BoundedCorpusFamily, BoundedCorpusRequest, CorpusProofDependencyGraph,
    EmittedCorpusIdentities, CORPUS_PROOF_GRAPH_SCHEMA,
};
pub use kani::generate::lower::bounded_collections::prepare_bounded_collection_query;
pub use kani::generate::lower::bounded_kani_profile::{
    classify_bounded_kani_profile, BoundedKaniProfile,
};
pub use kani::generate::lower::definedness_arithmetic::prepare_checked_arithmetic;
pub use kani::generate::lower::finite_reference_graphs::prepare_finite_graph_reaches;
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

pub use evidence::bound_coverage::{
    analyze_bound_coverage, ArtifactBytes, BoundAnalysisState, BoundCoverageAnalysis,
    BoundCoverageInputs, BOUND_COVERAGE_FORMAT, BOUND_COVERAGE_SCHEMA, MAX_ANALYSIS_BYTES,
};

pub use evidence::vacuity::{
    classify_clause, parse_llvm_coverage, ClauseCoverage, CoverageDiagnostic, CoverageErrorCode,
    LlvmCoverage, ProbeObservation, MAX_COVERAGE_BYTES,
};

pub use oracle::bound_v1::{
    generate_bound_oracles, BoundGenerationError, BoundOracleClause, BoundOracleGeneration,
    GeneratedBoundOracles, NoExecutableOracles,
};

pub use crate::core::identity::{HarnessPath, HarnessSymbol, ModuleSymbol, SymbolError};
pub use crate::core::profile::RUNTIME_DEPENDENCY_SOURCE;
pub use kani::abi::{KaniBindingRole, KaniIntegerBounds, KaniPrimitiveType, KaniSolver};
pub use kani::census::{
    ProofDependencyEdge, ProofDependencyKind, ProofDependencyRequest, ProofDependencyState,
    ProofReadiness,
};
pub use kani::classify::{
    classify_kani_run, ClassifiedRun, KaniInconclusiveReason, KaniRunOutcome,
};
pub use kani::generate::census_validation::{KaniDiagnostic, KaniErrorCode};
pub use kani::generate::v1_bundle::{
    generate_kani_bundle, KaniArtifactBundle, KaniRequest, KaniSubjectBinding, ProofDependencyGraph,
};
pub use kani::output::report::{
    KaniCheckClass, KaniCheckLocation, KaniCheckResult, KaniCheckStatus, KaniReportRefusal,
    OtherCheckClass,
};
pub use kani::run::execute::{
    execute_kani_obligation, execute_kani_obligations, kani_launch_command, launch_evidence,
    KaniBatchInvocation, KaniExecutionEvidence, KaniExecutionRefusal, KaniExecutionRequest,
    KaniGroupRun, OUTPUT_OVER_LIMIT_CODE, OUTPUT_UNREAD_CODE,
};
pub use kani::run::harness::KaniExecutableHarness;
pub use kani::run::launch::{run_launcher_with_timeout, CaptureStream, LaunchOutcome};
pub use kani::run::tool::{KaniInstallation, KaniTool, KaniToolError};
pub use kani::terminal::{
    ir_outcome_terminal_value, run_terminal_value, ReplaySettlement, TerminalPairError,
};
pub use routed::capability::{
    negotiate_backend_provider, BackendDescriptor, BackendKind, BackendProviderEnvelope, Candidate,
    Candidates, CapabilityKind, Cause, Disposition, EnvelopeRefusal, ExtentClassification,
    ItemSettlement, Mode, RequestItem, RequestedKind, BACKEND_PROVIDER_CONTRACT,
    CAPABILITY_VOCABULARY,
};
pub use routed::generate::{
    generate_routed, GenerationContexts, KaniGenerationContext, KindOutput, RoutedGeneration,
    RoutedGenerationError, RoutedGenerationItem, RoutedItemOutput,
};
pub use strategy::harness::{
    generate_tristate_harness, HarnessDiagnostic, HarnessErrorCode, HarnessRequest,
};

pub use crate::core::artifact::{
    Artifact, ArtifactBundle, PublicationDestinationState, PublicationDiagnostic,
    PublicationErrorCode, MAX_GENERATED_SOURCE_BYTES,
};
pub use crate::core::diagnostic::{
    GenerationDiagnostic, GenerationErrorCode, GenerationTerminalState,
};
pub use crate::core::source_map::{SourceProbe, SourceRegion};
pub use core::canonical::DigestError;
pub use kani::generate::frame::{
    generate_state_frame_obligations, StateFrameObligations, StateFrameRefusal, StateFrameRequest,
    UnsupportedFrameEffect,
};
pub use kani::generate::negotiate::negotiate_kani_obligations;
pub use kani::generate::outcome::{
    DerivedDomain, InvalidObligationItem, KaniObligationError, KaniObligationOutcome,
    KaniObligationRequest, ObligationDisposition, ObligationItem, ObligationRecord,
    ObligationSubject, UnsupportedObligation, MAX_OBLIGATION_ITEMS, MAX_OBLIGATION_UNWIND,
};
pub use kani::identity::{
    EmbeddedOracle, KaniObligationHarness, KaniObligationIdentity, KaniScalarObligationHarness,
    ObligationBinding, ObligationKind, ScalarObligationArgument, ScalarObligationIdentity,
    StateComparison, StateFieldDomain, StateFrameHarness, StateFrameIdentity, StateFrameProperty,
    StateFrameScope,
};
pub use kani::output::playback::DecodeFailure;
pub use publication::publish::{write_bundle_atomic, PublishedBundleIdentity};
pub use replay::frame::{FrameReplay, FrameReplayError, FrameReplayInputs, ProvidedDocument};
pub use replay::function::{
    replay_counterexample, replay_counterexample_through, replay_falsification, DependencyLock,
    DependencyLockError, EvidenceFailureCause, LockedSource, ReplayInputs, ReplayPackage,
    ReplayPackageError, ReplayParameter, ReplayVerdict, SpineReplayError,
};
pub use replay::obligation::ObligationIdentityError;
pub use replay::witness::decode_falsification;
pub use strategy::campaign::{
    generate_enum_strategy, generate_i64_strategy, EnumStrategyCampaign, EnumStrategyRequest,
    StrategyCampaign, StrategyConstraint, StrategyDiagnostic, StrategyErrorCode, StrategyRequest,
};

pub use oracle::boolean_v1::{
    generate_boolean_oracle, GeneratedArtifactBundle, OracleArtifactBundle, OracleRequest,
};

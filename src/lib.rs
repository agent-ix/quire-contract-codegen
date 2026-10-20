//! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

// Integration fixture modules name this crate as an external dependency.
#[cfg(test)]
extern crate self as quire_contract_codegen;

// Private integration control needs internal generation and execution in the library test binary.
#[cfg(test)]
#[path = "../tests/it/kani_render_private.rs"]
mod kani_render_private;

#[cfg(test)]
#[path = "../tests/it/kani_scalar_native_private.rs"]
mod kani_scalar_native_private;

#[cfg(all(test, target_os = "linux"))]
#[path = "../tests/it/kani_terminal_private.rs"]
mod kani_terminal_private;

// The leaf directory: artifact, diagnostic, identity, profile and source-map.
// `core` here is this crate's own layout module (the AD names the directory `core/`). In this
// file it shadows the extern `core` crate, so `crate::core::` is the explicit spelling.
mod core;
// The Kani subsystem.
mod kani;
// The oracle subsystem and its canonical operation catalog path.
pub mod oracle;
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
pub use kani::generate::lower::bounded_collections::{
    prepare_bounded_collection_query, CollectionLowering, CollectionQuery, QueryKind,
};
pub use kani::generate::lower::bounded_kani_profile::{
    classify_bounded_kani_profile, BoundedKaniProfile,
};
pub use kani::generate::lower::finite_reference_graphs::{
    prepare_finite_graph_reaches, GraphLowering, GraphRequest,
};
pub use kani::generate::lower::FamilyLoweringError;
pub use oracle::claim::{ClaimDisposition, ClaimMap, OracleGenerationError, UpstreamBlocker};
pub use oracle::scalar::{
    derive_exact_scalar_items, generate_exact_scalar_oracles, plan_exact_scalar_oracles,
    render_exact_scalar_plan, BooleanOperator, BoundForm, ClaimDerivationRefusal,
    ContractScalarError, DecimalOperator, EqualityOperandKind, ExactScalarClaim, ExactScalarItem,
    ExactScalarOperation, ExactScalarOracles, ExactScalarPlan, ExactScalarRefusal,
    GeneratedScalarClaim, IeeeArithmeticOperator, OperandRangeLimitKind, OperationClaim,
    OperationProvenance, OrderingOperandKind, QuantityOperator, RationalOperator, ScalarForm,
    EXACT_SCALAR_CRATE_NAME, SCALAR_LOWERING_SUPPORTED_TAGS, SCALAR_LOWERING_WORK_LIMIT,
};

pub use oracle::equality::{
    generate_composite_equality_oracles, CompositeEqualityClaim, CompositeEqualityItem,
    CompositeEqualityLimits, CompositeEqualityOracles, CompositeEqualityRefusal,
    CompositeOperationClaim, CompositeOperationProvenance, DeclarationRefusalCause,
    EqualityOperandDescriptor, EqualityOperatorKind, GeneratedCompositeEqualityClaim,
    IllTypedCauseKind, RecordedDescriptor, RecordedSchedule, RecursionEdgesKind,
    COMPOSITE_EQUALITY_CRATE_NAME, COMPOSITE_EQUALITY_LOWERING_WORK_LIMIT,
};

pub use oracle::function::{
    generate_exact_function_oracles, generate_function_schedules,
    generate_function_schedules_with_limits, CallPointKind, ExactFunctionBody, ExactFunctionClaim,
    ExactFunctionDeclaration, ExactFunctionItem, ExactFunctionOracles, ExactFunctionRefusal,
    FunctionParameter, FunctionScheduleClaim, FunctionScheduleDeclaration,
    FunctionScheduleDisposition, FunctionScheduleGeneration, FunctionScheduleItem,
    FunctionScheduleLimits, FunctionScheduleParameter, GeneratedExactFunctionClaim,
    LocationMapEntry, RecordedLocation, RecordedOrigin, RefusedFunctionBody, ScheduledFunctionBody,
    EXACT_FUNCTION_CRATE_NAME, EXACT_FUNCTION_LOWERING_WORK_LIMIT,
};

pub use oracle::function::schedule::{
    BlockId as FunctionScheduleBlockId, ControlAction as FunctionScheduleControlAction,
    ControlBlock as FunctionScheduleControlBlock, ControlEdge as FunctionScheduleControlEdge,
    EdgeCondition as FunctionScheduleEdgeCondition, Instruction as FunctionScheduleInstruction,
    LocationQueryRefusal as FunctionScheduleLocationQueryRefusal,
    LocationRecord as FunctionScheduleLocation, Occurrence as FunctionScheduleOccurrence,
    Operand as FunctionScheduleOperand, Operation as FunctionScheduleOperation,
    Schedule as FunctionBodySchedule, ScheduleRefusal as FunctionScheduleRefusal,
    ScheduledOperation as FunctionScheduledOperation, SsaSlot as FunctionScheduleSlot,
    ValueKind as FunctionScheduleValueKind, DEFAULT_GENERATION_WORK_LIMIT,
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
pub use core::contract_package::{
    routed_contract_lowering_profile, AdmittedItemAccounting, ContractDriverResultSelection,
    ContractRequestPosition, PackageRequestError,
};
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
pub use kani::output::diagnostics::{
    KaniFailurePhase, KaniNoVerdictDiagnostics, KaniStreamExcerpt,
};
pub use kani::output::report::{
    KaniCheckClass, KaniCheckLocation, KaniCheckResult, KaniCheckStatus, KaniFailureCause,
    KaniReportRefusal, OtherCheckClass,
};
pub use kani::run::execute::{
    execute_kani_obligation, execute_kani_obligations, KaniBatchInvocation, KaniExecutionEvidence,
    KaniExecutionRefusal, KaniExecutionRequest, KaniGroupRun, OUTPUT_OVER_LIMIT_CODE,
    OUTPUT_UNREAD_CODE,
};
pub use kani::run::harness::KaniExecutableHarness;
pub use kani::run::launch::CaptureStream;
pub use kani::run::scalar::{
    execute_kani_scalar_native, KaniScalarNativeError, KaniScalarNativeErrorClass,
    KaniScalarNativeReceipt, KaniScalarNativeRequest, NativeMeterObservation, ScalarOracleStopKind,
    ScalarProofContentIdentity, SCALAR_NATIVE_CODE_ARTIFACT_PACKAGE,
    SCALAR_NATIVE_CODE_CODEGEN_LOCK, SCALAR_NATIVE_CODE_DEPENDENCY_LOCK,
    SCALAR_NATIVE_CODE_IDENTITY_ENCODE, SCALAR_NATIVE_CODE_INVALID_REQUEST,
    SCALAR_NATIVE_CODE_KANI_EXECUTION, SCALAR_NATIVE_CODE_NATIVE_LAUNCH,
    SCALAR_NATIVE_CODE_NATIVE_RESULT, SCALAR_NATIVE_CODE_ORACLE_STOP,
    SCALAR_NATIVE_CODE_PROOF_NOT_VERIFIED, SCALAR_NATIVE_CODE_SNAPSHOT_CHANGED,
    SCALAR_NATIVE_CODE_SNAPSHOT_IO,
};
pub use kani::run::tool::{KaniInstallation, KaniTool, KaniToolError};
pub use kani::terminal::{
    ir_outcome_terminal_value, run_terminal_value, ReplaySettlement, TerminalPairError,
};
pub use quire_contract_runtime::exact::ScalarLimits;
pub use routed::capability::{
    negotiate_backend_provider, negotiate_contract_package, project_contract_package_incomplete,
    project_contract_package_refusal, BackendDescriptor, BackendId, BackendKind,
    BackendProviderEnvelope, Candidate, Candidates, CapabilityKind, Cause, ContractItemSettlement,
    ContractNegotiationError, ContractPackageIncompleteProjection,
    ContractPackageRefusalProjection, Disposition, EnvelopeRefusal, ExtentClassification,
    ExtentDomain, ItemSettlement, KaniForm, KaniFormFamily, KaniFormIdentity, KaniFormName,
    KaniObligationRole, KaniStateFrameRole, Mode, ProviderOrigin, RequestItem, RequestedKind,
    BACKEND_PROVIDER_CONTRACT, CAPABILITY_VOCABULARY,
};
pub use routed::generate::{
    generate_routed, GenerationContexts, KaniGenerationContext, KindOutput, RoutedGeneration,
    RoutedGenerationError, RoutedGenerationItem, RoutedItemConstructionError, RoutedItemOutput,
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
pub use crate::oracle::clause::{
    derive_v2_clause_descriptor, generate_v2_clause_claim_map, generate_v2_clause_oracle,
    ClauseOracleInput, V2ClauseClaim, V2ClauseCoverageRegion, V2ClauseDescriptor, V2ClauseOracle,
    V2ClauseRefusal,
};
pub use core::canonical::DigestError;
pub use kani::generate::frame::{
    generate_state_frame_obligations, StateFrameObligations, StateFrameRequest,
};
pub use kani::generate::negotiate::negotiate_kani_obligations;
pub use kani::generate::outcome::{
    BoundNotResolvedCause, CollectionUnsupportedReason, DerivedDomain, InvalidObligationItem,
    KaniObligationError, KaniObligationOutcome, KaniObligationRequest, ModelMemberRangeReason,
    ObligationDisposition, ObligationItem, ObligationRecord, ObligationSubject,
    StateFrameLoweringRefusal, StateFrameRefusal, StateFrameRole, UnsupportedFrameEffect,
    UnsupportedObligation, MAX_OBLIGATION_ITEMS, MAX_OBLIGATION_UNWIND,
};
pub use kani::identity::{
    CollectionElementIdentity, CollectionKindIdentity, CollectionObligationIdentity,
    CollectionPredicateIdentity, EmbeddedOracle, KaniCollectionObligationHarness,
    KaniObligationHarness, KaniObligationIdentity, KaniScalarObligationHarness,
    KaniV2ClauseObligationHarness, ObligationBinding, ObligationKind, ScalarObligationArgument,
    ScalarObligationIdentity, ScalarOriginalLimitsError, StateComparison, StateFieldDomain,
    StateFrameHarness, StateFrameIdentity, StateFrameProperty, StateFrameRecordError,
    StateFrameScope, StateUnrangedField, StateUnrangedReason, V2ClauseObligationIdentity,
};
pub use kani::output::playback::DecodeFailure;
pub use publication::publish::{write_bundle_atomic, PublishedBundleIdentity};
pub use replay::composite::{
    composite_parity_terminal_value, verified_shadow_terminal_value, CompositeParitySettlement,
    CompositeReportError, VerifiedShadowSettlement,
};
pub use replay::composite_builder::{
    CompositeBuildError, OriginalCompositeEqContext, OriginalCompositeEqReport,
    OriginalCompositeEqRequest,
};
pub use replay::frame::{
    FrameReplay, FrameReplayError, FrameReplayInputs, PreStateFault, ScopeMember,
};
pub use replay::function::{
    replay_counterexample, replay_counterexample_through, replay_falsification, DependencyLock,
    DependencyLockError, EvidenceFailureCause, LockedSource, ProvidedDocument, ReplayInputs,
    ReplayPackage, ReplayPackageError, ReplayParameter, ReplayVerdict, SpineReplayError,
};
pub use replay::obligation::ObligationIdentityError;
pub use replay::scalar::decode_scalar_falsification;
pub use replay::state_clause::{
    DocumentError, DocumentLabel, ModelError, OperationDeclaration, StateClauseHarnessMember,
    StateClauseModelFieldsCause, StateClauseReplay, StateClauseReplayError,
    StateClauseReplayInputs, StateObjectAddress,
};
pub use replay::witness::{
    decode_falsification, decode_v2_clause_falsification, WitnessDecodeLimits,
};
pub use strategy::campaign::{
    generate_enum_strategy, generate_i64_strategy, EnumStrategyCampaign, EnumStrategyRequest,
    StrategyCampaign, StrategyConstraint, StrategyDiagnostic, StrategyErrorCode, StrategyRequest,
};

pub use oracle::boolean_v1::{
    generate_boolean_oracle, GeneratedArtifactBundle, OracleArtifactBundle, OracleRequest,
};

/// Resource ceilings bound into each generated proof identity.
pub use kani::identity::{ProofCeilings, SymbolicArgumentBounds, SymbolicBounds};

/// The actual tree memory mechanism and observations recorded by bounded backend runs.
pub use kani::run::memory::{MemoryMechanism, MemoryObservation};

/// QSL-owned canonical observation digest record.
pub use qsl_replay::DigestRecord;

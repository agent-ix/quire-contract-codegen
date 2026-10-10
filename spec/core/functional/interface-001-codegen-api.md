---
id: interface_001
title: "Contract code-generation API"
type: interface
---
# [interface-001] Contract code-generation API

## Contract

```yaml
name: ContractCodegen
version: draft-codegen-v1
input:
  contract_package: public IR BoundPackage from the public IR derived executable projection decoder; no private wire or codegen input schema
  configuration: backend selection, customer bindings, output profile
operations:
  - name: generate_bound_oracles
    inputs: [public BoundPackage reference]
    output: BoundOracleGeneration | BoundGenerationError
    semantics: complete ordered executable-clause batch over the supported Boolean and obligation-free bounded-integer comparison grammar, or explicit NoExecutable with informational references but no publishable artifact; unsupported executable content fails the entire batch
  - name: generate_bundle
    status: planned; the implemented first consumer is library-only generate_bound_oracles
    inputs: [contract package bytes, generation configuration]
    output: ArtifactBundle | DiagnosticSet
    semantics: planned multi-backend operation
  - name: generate_tristate_harness
    inputs: [typed precondition, typed postcondition, explicit bindings, minimum accepted cases, minimum rejected cases, maximum discarded cases]
    output: GeneratedArtifactBundle | HarnessDiagnosticSet
    semantics: source, request-bound campaign policy, owned execution loop, retained campaign accounting
  - name: generate_i64_strategy
    inputs: [requirement identity, constraint, campaign]
    output: GeneratedArtifactBundle | StrategyDiagnostic
    semantics: shaped cases whose expected domain is checked against runtime VerdictKind
  - name: generate_enum_strategy
    inputs: [requirement identity, customer enum path and variants, campaign]
    output: GeneratedArtifactBundle | StrategyDiagnostic
    semantics: finite shaped cases with an explicit quire-contract-runtime consumer dependency
  - name: generate_bound_strategy
    status: implemented for the numeric/state slice on the bounded-integer comparison grammar
    inputs: [public BoundPackage, full ClauseRef, population Satisfying|Violating|Broad|Boundary, campaign policy]
    output: GeneratedArtifactBundle | StrategyDiagnostic
    semantics: admits only clauses bound oracle generation admits, narrowed to one Compare; domains taken from the clause's IR IntegerType declarations; constructive Holds/Violated populations, an exhaustive in-domain census and an untagged out-of-domain array; a subject-free oracle-conformance runner whose summary reports discard and rejection rates; see bound_strategy_slice
  - name: generate_kani_bundle
    status: live interim V1 entry; AD-004 step 4f removes it after the V2 contract arm serves its families; FR-015 owns the current bundle cover and ADR-001 Q1 owns the target
    inputs: [typed precondition, typed postcondition, direct checked dependency types and observations, subject path, dependency census]
    output: KaniArtifactBundle | KaniDiagnosticSet
    semantics: deterministic Boolean/i64 Kani source and v2 dependency/binding graph using exact oracle predicates and IR-owned model bounds; generation records proof execution as not_run and never claims proof completion
  - name: write_bundle_atomic
    inputs: [ArtifactBundle, destination directory]
    output: PublishedBundleIdentity | IO diagnostic
    semantics: stage and validate the complete bundle, then replace the destination directory; caller serializes destination writers; inspection/read failures return io_failed with unchanged state; failed rollback reports unknown and preserves backup/staging for recovery; post-commit cleanup failures report published; process crashes between directory renames and power-loss durability remain outside the portable rollback guarantee
  - name: analyze_coverage
    status: planned; public IR-owned bound population is available, QSL native-run-result/2 production and aggregate analysis integration remain pending
    inputs: [bound executable population, generated source and maps actually used by the native coverage producer, LLVM coverage JSON bytes and producer tool/version, CG-owned authenticated producer/artifact binding receipt associating the exact QSL result, generated-campaign execution, generated source and map, and exact LLVM export bytes with the producing run, QSL strict-reader typed clause-run or command-error result for native-run-result/2 or its located refusal, source root, runtime campaign report]
    output: versioned structured AnalysisOutcome including non-success diagnostics, independent QSL clause-run disposition and generated-campaign execution facts, run and LLVM producer identities, and available clause observations
    semantics: consume QSL's typed /2 reader result without a CG wire parser or /1 fallback; authenticate the separate CG-owned receipt against the exact result, producer execution, generated source and map, and exact LLVM export bytes/probe data from that run before classifying measured coverage; absent, unauthenticated or mismatched associations yield structured non-success retaining the decoded outcome and available identities; a QSL clause-run success, decisive witness or command-error envelope never authenticates execution of CG-generated Rust, source/map bytes or LLVM probes; coverage obligation succeeds only for nonempty complete exercised population with successful bound generated execution, so oracle success with an unobserved implication consequent is adverse; successful serialization is not successful coverage; IR FR-045 owns proof eligibility and credit; never executes LLVM
  - name: analyze_bound_coverage
    status: implemented phase A; full native-run analysis remains pending
    inputs: [public BoundPackage, immutable BoundOracleGeneration, complete artifact bytes, optional LLVM export bytes, source root]
    output: immutable versioned BoundCoverageAnalysis domain observations
    semantics: exact independent clause and implication census; whole-batch artifact/map binding; measured clauses or explicit unavailable states; valid informational-only population is no_executable; no runtime transport, producer execution, or assurance verdict
  - name: generate_boolean_oracle
    inputs: [OracleRequest over one typed Boolean clause]
    output: OracleArtifactBundle | GenerationDiagnostic list
    semantics: one deterministic Boolean oracle and its source map, or diagnostics with no partial bundle; the single-clause core generate_bound_oracles batches
  - name: classify_clause
    inputs: [oracle entry probe observation, independently derived expected consequent count, consequent probe observations]
    output: ClauseCoverage | CoverageDiagnostic
    semantics: bounded observation primitive classifying one clause's measured probes; never an aggregate coverage verdict, and the expected count must not be derived from source-map rows (FR-004)
  - name: parse_llvm_coverage
    inputs: [LLVM coverage JSON bytes, absolute source root]
    output: LlvmCoverage | CoverageDiagnostic
    semantics: parses an LLVM full JSON coverage export without executing any producer; parent traversal and backslashes refuse, with no suffix matching or summary fallback (FR-004)
  - name: classify_bounded_kani_profile
    status: live finite-input auxiliary under FR-015-AC-82 and AC-83; AD-004 step 4g governs retirement of the hand-built corpus path
    inputs: [KaniProfile, requested constructs, source id]
    output: CapabilityEntry list | KaniOutcome
    semantics: classifies one request through the public bounded Kani profile with no reverse Contract IR dependency; a malformed request is a typed refusal, not a disposition
  - name: prepare_checked_arithmetic
    status: live finite-input auxiliary under FR-015-AC-84 and AC-85
    inputs: [KaniProfile, DispatchIndex, ValidatedFiniteInput, CheckedArithmeticRequest]
    output: ArithmeticLowering | FamilyLoweringError (IR KaniOutcome or KaniOutcomeError)
    semantics: CG-owned bounded checked-arithmetic lowering over the IR finite-input/profile/dispatch ABI; division by zero, overflow, invalid ranges, profile refusal and dispatch mismatch stay IR typed non-Boolean outcomes, never an assumption or partial artifact
  - name: prepare_bounded_collection_query
    status: live finite-input auxiliary under FR-015-AC-88 and AC-89
    inputs: [KaniProfile, DispatchIndex, ValidatedFiniteInput, CollectionQuery]
    output: CollectionLowering | FamilyLoweringError (IR KaniOutcome or KaniOutcomeError)
    semantics: CG-owned bounded collection-query lowering; bound exhaustion and profile or dispatch refusal stay IR typed non-Boolean outcomes with no partial artifact
  - name: prepare_finite_graph_reaches
    status: live finite-input auxiliary under FR-015-AC-86 and AC-87
    inputs: [KaniProfile, DispatchIndex, ValidatedFiniteInput, GraphRequest]
    output: GraphLowering | FamilyLoweringError (IR KaniOutcome or KaniOutcomeError)
    semantics: CG-owned positive-length finite reference-graph reachability over IR-validated objects and references, with sorted depth-first expansion and a bound before each new identity; malformed or exhausted requests stay IR typed non-Boolean outcomes
  - name: generate_bounded_kani_corpus_case
    status: live interim four-artifact corpus under FR-015-AC-51 to AC-58 and AC-90 to AC-94; native replay remains planned; AD-004 step 4g removes its generation-time KaniOutcome under planned FR-015-AC-95
    inputs: [KaniProfile, DispatchIndex, ValidatedFiniteInput, BoundedCorpusRequest, proof dependency census, shared EmittedCorpusIdentities]
    output: BoundedCorpusCase | BoundedCorpusError
    semantics: one bounded Kani corpus case and its proof dependency graph; a refusal is a BoundedCorpusError whose Outcome variant carries the typed KaniOutcome (its code a quire_contract_model Std001Code) and whose OutcomeConstruction variant carries Contract IR's refusal to build a non-success outcome; a case whose identity the shared registry already holds refuses as kani_corpus_identity_collision rather than overwriting earlier artifacts; a serialization failure of the case's proof graph refuses as kani_corpus_serialization_failed (NFR-005)
  - name: generate_exact_scalar_oracles
    inputs: [admitted CheckedPackageV2, ExactScalarItem list]
    output: ExactScalarOracles | OracleGenerationError
    semantics: exact scalar oracles plus a typed claim map identical to claim-map.json; per-item problems are refusals, and only a whole-generation failure is an error (FR-014)
  - name: derive_exact_scalar_items
    inputs: [admitted CheckedPackageV2, CheckedNodeId list]
    output: one ExactScalarItem | ExactScalarRefusal per node id, in input order
    semantics: builds each item's descriptor from the node's operation identity, laws, mode, operand forms and the one bound of the needed form on its result type; a node with no derivable descriptor is a typed refusal, never a guess (FR-014)
  - name: negotiate_kani_obligations
    inputs: [KaniObligationRequest]
    output: KaniObligationOutcome | KaniObligationError
    semantics: settles every item in request order and, only when no item is invalid, emits one harness per supported item; an invalid item returns no harness bytes (FR-015); planned: an item is also a V2 clause claim over a precondition, postcondition or invariant node of an admitted CheckedPackageV2, with an optional declared census (FR-015-AC-38 to FR-015-AC-49); an item is also a StateFrame item (IR-461; FR-015-AC-60 and FR-015-AC-68 stay planned: the comparison against generate_state_frame_role is not testable from tests/it (crate-private) and no V2 package builder exists under src/; byte identity holds by construction; the tests compare against the public entry for a clause whose roles both succeed; one state_clause postcondition node, a contract or frame role, the state struct path and fields and the item's own subject path), recorded with the existing ObligationDisposition and reasons plus StateFrameRefused, InvalidStatePath, InvalidStateField, InvalidStateUnwind and MixedStatePackages, by one total mapping from StateFrameRefusal, the two roles settling independently, and its harnesses returned in the state_frame_harnesses field of KaniObligationOutcome::Emitted (FR-015-AC-59 to FR-015-AC-68)
  - name: generate_state_frame_obligations
    inputs: [StateFrameRequest]
    output: StateFrameObligations | StateFrameRefusal
    semantics: from one postcondition state_clause of an admitted CheckedPackageV2 over one integer field, returns one operation-contract harness and one frame-effect harness of the clause's operation, each with a scoped identity and one non-vacuity cover; every other shape is a typed refusal with no harness (FR-015-AC-26 to FR-015-AC-29); returns on the first refusal of its one clause, and the StateFrame item of negotiate_kani_obligations is how a package gets a disposition per clause (IR-461; AD-004 step 4d retires this entry as public)
  - name: generate_state_frame_role
    inputs: [StateFrameRequest, StateFrameRole (contract | frame)]
    output: StateFrameHarness | StateFrameRefusal
    semantics: IR-461 (FR-015-AC-60, FR-015-AC-68). Builds the harness of one role of the clause or returns that role's first refusal, so a clause whose roles differ gives one harness and one refusal; the StateFrame item of negotiate_kani_obligations calls it once per item, and generate_state_frame_obligations returns both roles' harnesses when both succeed and otherwise the first refusal. Not a second public generation entry: it is crate-internal, and AD-004 step 4d retires generate_state_frame_obligations as a public entry and keeps negotiate_kani_obligations as the one entry
  - name: execute_kani_obligation
    inputs: [KaniExecutionRequest, whose harness is a KaniExecutableHarness: Contract, Scalar or StateFrame]
    output: KaniExecutionEvidence | KaniExecutionRefusal
    semantics: refuses a crate that does not contain the harness, runs the harness and reports the backend's own outcome; see kani_obligation_execution_slice (FR-017)
  - name: execute_kani_obligations
    inputs: [list of KaniExecutionRequest, one per harness]
    output: one KaniGroupRun per launcher process (the request positions of its members and either one KaniExecutionEvidence per member or the KaniExecutionRefusal of the whole process), or one KaniExecutionRefusal for the whole call
    semantics: the batch entry (IR-277, FR-017-AC-21 to FR-017-AC-25, FR-028-AC-12). Refuses the whole call with HarnessNotInCrate before any process starts when a harness is not in its crate; otherwise groups the requests by option vector (harness selection removed), request timeout, launcher, crate directory and target directory, runs each group in one launcher process, and splits the report per harness by its module::harness path. A group of one is the single run. Each member's evidence carries the group's argument vector, the member list with the timeout T in whole seconds (the `batch` statement) and the exit code beside the fields of a single run
  - name: kani_launch_command
    inputs: [KaniExecutionRequest]
    output: argv and Command
    semantics: the exact launch command execute_kani_obligation runs, exposed so a caller interleaving its own steps runs that command rather than a copy (FR-017)
  - name: run_launcher_with_timeout
    inputs: [Command, timeout]
    output: LaunchOutcome | io error
    semantics: the bounded launch execute_kani_obligation performs, draining stdout and stderr on their own threads and killing the process group at the timeout, when a stream is over 8 MiB or unread, and when the launcher exits on its own (FR-017)
  - name: launch_evidence
    inputs: [LaunchOutcome, exported report bytes, optional obligation kind]
    output: ClassifiedRun and exit code | KaniExecutionRefusal
    semantics: the mapping execute_kani_obligation applies from a concluded launch to evidence; a timed-out launch has no exit code and its report is not read; a launch stopped over a stream's limit or over a stream that was not read is the refusal kani_output_over_limit or kani_output_unread, never an outcome (FR-017)
  - name: classify_kani_run
    inputs: [process exit success, exported Kani report bytes, console text, optional obligation kind]
    output: ClassifiedRun (KaniRunOutcome, SUCCESS-check count and the per-check KaniCheckResult view) | KaniReportRefusal
    semantics: the classifier execute_kani_obligation uses, so a test asserting falsification routes through production classification and an inconclusive run is never read as a decided failure; the verdict is read from the report only and the console text is read only for the falsifying playback; a report it cannot read exactly is a refusal, never an outcome (FR-017)
  - name: run_terminal_value
    inputs: [KaniRunOutcome, the SUCCESS-check count, an optional ReplaySettlement (reproduced | disagreement carrying QSL's DisagreementCause | refused carrying a ReplayRefusal | setup refusal carrying a QSL Code | fault | CG defect), which the From impls of SpineReplayError, ReplayPackageError, FrameReplayError, StateClauseReplayError, DependencyLockError, EvidenceFailureCause, ReplayVerdict and the QSL CallSiteRefusal build; the EvidenceFailureCause impl reads a decode failure and a value outside the proof bound as a CG defect, a disagreement as its QSL cause, and a reproduction in a category other than violation as a CG defect]
    output: qsl-replay TerminalValue | TerminalPairError (MissingSettlement | UnexpectedSettlement; planned, IR-241: NonProductionProof | ShadowCounterexample)
    semantics: >-
      the ordinary source-predicate match has no wildcard arm; falsified is Refuted only with
      reproduced replay, and this ReplaySettlement accompanies falsified only (FR-029 AC-15).
      Planned IR-635 composite parity takes a distinct typed same-claim settlement input: FR-029
      AC-17 and AC-19 to AC-27 own closed strength projection, cross-outcome disagreement priority,
      ordinary shadow-inconclusive/resource rows, vacuous category and completed Proved/Tested
      outcomes; TC-048 owns its scenarios. UNVERIFIED SOURCE and gated on actual QSL-640 API
      delivery. IR-241 owns the proof-subject/verified-strength inputs and interim
      NonProductionProof / ShadowCounterexample variants; IR-635 widens NonProductionProof to
      bounded-shadow inconclusive/cover-unsatisfied outcomes retaining refinement_failed. These
      refusals hold until delivery, not as the final map.
  - name: ir_outcome_terminal_value
    inputs: [Contract IR KaniOutcome, the SUCCESS-check count, an optional ReplaySettlement as run_terminal_value takes it]
    output: qsl-replay TerminalValue | TerminalPairError (MissingSettlement | UnexpectedSettlement)
    semantics: the one match over the pair (outcome kind, replay settlement) with no wildcard arm; a Counterexample is Refuted only with a reproduced replay, and the settlement accompanies a Counterexample only; Refused, InvalidInput and IncompleteInput are Declined carrying the outcome's Std001Code as DeclineCode::Std001, unchanged and unchecked for registration; Unavailable and Inconclusive read the code (FR-030)
  - name: composite_parity_terminal_value
    inputs: [retained CompositeIdentity of the falsified claim actually sent, optional genuine QSL CompositeParityReport]
    output: CompositeParitySettlement | CompositeReportError (MissingReport | ClaimMismatch)
    semantics: compare the full report.claim() with the retained sent identity before reading its typed result; a missing or different report has no terminal value; the private settlement stores QSL's CompositeParityResult and derives terminal_value() from that result, preserving QSL's F-row order, cause and code (FR-033 AC-9, FR-029 AC-28)
  - name: OriginalCompositeEqContext::new
    inputs: [original admitted CheckedPackageV2, retained ReplayInputs, selected function identifier, retained canonical proof-content DigestRecord, CheckedPackageReadLimits]
    output: OriginalCompositeEqContext | CompositeBuildError
    semantics: retain immutable original inputs and require the exact source owner and source-byte binding in the package lock; content identity remains a caller-retained binding input and does not authenticate a generated artifact
  - name: OriginalCompositeEqContext::request
    inputs: [original CheckedNodeId, original CheckedOccurrence, harness ProofBound list, IR operand-projection work ceiling]
    output: immutable OriginalCompositeEqRequest | CompositeBuildError
    semantics: Eq-only graph-child route through IR composite_application_operands; require admitted original node/occurrence presence, preserve positional parameter bounds and empty literal Bounds, compute O-09 through owning parity_obligation, decode the real wire to obtain QSL stage limits, and require exact recompiled package/context equality before invocation; projection/encoder refusal can precede context authentication because public stage-limit decoding needs the genuine O-09 wire; QSL owns selected-function body and occurrence-origin membership at invocation through its composite-site locator, with binding-checked Refused reports; imported contexts return ImportedContextUnsupported until original admitted dependency packages are retained by CG; Ne and inline operands refuse
  - name: OriginalCompositeEqRequest::settle_verified
    inputs: [retained VerifiedShadow evidence]
    output: OriginalCompositeEqReport | CompositeBuildError
    semantics: retain actual sent CompositeIdentity before calling the public QSL settle_verified_shadow facade, then feed its genuine report to verified_shadow_terminal_value; the owned result exposes the report, sent identity and binding-checked settlement; no Kani, proof-strength or native observation is produced
  - name: verified_shadow_terminal_value
    inputs: [retained CompositeIdentity of the verified claim actually sent, optional genuine QSL VerifiedShadowReport]
    output: VerifiedShadowSettlement | CompositeReportError (MissingReport | ClaimMismatch)
    semantics: compare the full report.claim() with the retained sent identity before reading its typed result; a missing or different report has no terminal value; the private settlement stores QSL's VerifiedShadowResult and derives terminal_value() from that result (FR-033 AC-9)
  - name: generate_composite_equality_oracles
    inputs: [admitted CheckedPackageV2, CompositeEqualityItem list]
    output: CompositeEqualityOracles | OracleGenerationError
    semantics: composite equality oracles plus a typed claim map identical to claim-map.json; per-item problems are refusals, and only a whole-generation failure is an error (FR-018)
  - name: negotiate_backend_provider
    inputs: [BackendProviderEnvelope]
    output: ItemSettlement list | EnvelopeRefusal
    semantics: settles every item of the envelope in request order against a closed backend kind; an item naming a backend nothing here can settle for settles as invalid-request/unknown-backend inside the Ok list, never as one that can; the whole envelope is refused only for an unsupported contract_version or capability_vocabulary (FR-019)
  - name: BackendKind::from_descriptor
    inputs: [CG BackendDescriptor reference]
    output: Option<BackendKind>
    semantics: classifies by typed origin and identity; Linked kani gives Kani, unknown Linked gives None, and Process gives Process of the descriptor's exact identity (FR-019)
  - name: RoutedGenerationItem::from_descriptor
    inputs: [request_index usize, node_id CheckedNodeId, backend Candidate, CG BackendDescriptor reference]
    output: RoutedGenerationItem | RoutedItemConstructionError (DescriptorBackendMismatch{backend, descriptor} | UnknownLinkedBackend{backend})
    semantics: checks that backend identity equals descriptor identity and derives the private kind through BackendKind::from_descriptor; backend and kind are private, request_index and node_id are public, so the driver cannot inject a hand-built Process kind or reassign the backend (FR-022)
  - name: generate_routed
    inputs: [admitted CheckedPackageV2 reference, RoutedGenerationItem list (public request_index usize, public node_id CheckedNodeId, private backend Candidate, private kind BackendKind), GenerationContexts (optional kani KaniGenerationContext of subject_path and unwind u32; Process needs no context)]
    output: RoutedGeneration (items, one RoutedItemOutput of request_index, backend and KindOutput per routed item in ascending request_index; rejected BackendKind list; claim_map Option<ClaimMap<ExactScalarClaim>>, Some after a Kani group; oracle_artifacts Option<Vec<Artifact>>, the FR-014 oracle crate of that group, Some after a Kani group) | RoutedGenerationError (DuplicateRequestIndex{request_index} | BackendKindDisagrees{request_index, backend, routed, converted Option<BackendKind>} | MissingKindContext{kind} | DuplicateHarness{harness HarnessPath (module ModuleSymbol, harness HarnessSymbol)} | KaniRecordCountMismatch{records usize, items usize} | KaniDuplicatePositionOutOfRange{first_index usize, items usize} | Kani(KaniObligationError) | Oracle(OracleGenerationError))
    semantics: runs each routed item's backend-kind generation arm over an exhaustive BackendKind match without re-settling, re-selecting or re-routing a backend; the Kani arm derives each node's claim with derive_exact_scalar_items and generate_exact_scalar_oracles, then runs negotiate_kani_obligations over the routed Kani items in ascending request_index, with every record index rewritten to the driver's request index and harnesses joined by harness_symbol, both done by route_records (two harnesses with one symbol refuse the call as DuplicateHarness, and a record count other than the group's item count as KaniRecordCountMismatch, and a DuplicateItem position outside the group as KaniDuplicatePositionOutOfRange), and returns the generated oracle crate unchanged; no FR-019 Disposition is constructed (FR-022)
  - name: generate_exact_function_oracles
    inputs: [admitted CheckedPackageV2, ExactFunctionDeclaration list, ExactFunctionItem list]
    output: ExactFunctionOracles | OracleGenerationError
    semantics: function-application oracles over the declared functions, plus a typed claim map; per-item problems are refusals, and only a whole-generation failure is an error (FR-021)
  - name: decode_falsification
    inputs: [harness symbol, module symbol, ObligationBinding list, Kani playback transcript]
    output: named qsl-replay WitnessValue list | DecodeFailure (code, source_id, context)
    semantics: selects the one assertion playback block of a Kani run and types its concrete bytes with the harness's persisted obligation schema position for position with its kani::any() calls, refusing on a non-argument binding, transcript, harness-identity or decode mismatch (FR-016)
  - name: replay_falsification
    inputs: [harness symbol, check text, named qsl-replay WitnessValue list, ReplayParameter list (argument name, node id), a function from the witness ReplaySource to the complete ReplayRequestWire]
    output: the qsl-replay WitnessArmResult | SpineReplayError (UnboundArgument{argument} | FieldDelimiter | Transcript(MalformedTranscript) | Refused(ReplayRefusal) | WrongArm)
    semantics: builds the backend-witness transcript keyed by parameter node id and calls qsl_replay::replay, returning QSL's own settlement; a Boolean value is replayed as 1 or 0 (FR-016)
  - name: ReplayPackage::new
    inputs: [ReplayInputs (proved unit LockedSource, DependencyLock list, accounting and stage limits), function name]
    output: ReplayPackage | ReplayPackageError (InvalidFunction{function} | Dependencies(DependencyLockError: Input(DependencyInputRefusal), where a repeated dependency identity is QSL's DuplicateIdentity refusal from DependencyInput::new, FR-016-AC-24) | CallSite(CallSiteRefusal))
    semantics: compiles the proved unit, together with the lock's dependency selections as its dependency input, through qsl_replay::call_site and keeps the package id and each parameter's node id; ReplayPackage::request builds the complete ReplayRequestWire from the same lock, filling package.dependencies with one entry per lock dependency selection in ascending identity order (identity, the lock's recorded package_id, the dependency's own source); every source's bytes, the unit's and each dependency's, are provided by digest; QSL refuses a request naming a dependency the unit does not import (FR-016)
  - name: ReplayPackage::obligation_identity
    inputs: [ReplayPackage, KaniObligationIdentity (the harness replayed)]
    output: qsl-replay ObligationIdentity | ObligationIdentityError (UnboundArgument{argument} | UnboundParameter{parameter} | UnboundedDomain{argument} | BoundedBoolean{argument} | Digest(DigestError: the encoder refused the preimage))
    semantics: the ADR-013 O-09 function-contract obligation identity of the selected function for that harness: SHA-256 over the RFC 8785 encoding, made by quire-canonical in core::canonical, of the FunctionSite's function node id and declaration occurrence key, the harness's ObligationKind and the arguments (parameter node id and declared domain) ascending by identifier; the arguments must be exactly the function's parameters, each bound; the spelling is CG's own and interim (AD-003 E-1; FR-016-AC-21 to AC-23)
  - name: ReplayPackage::request
    inputs: [ReplayPackage, KaniObligationIdentity (the harness replayed), ReplaySource]
    output: the complete qsl-replay ReplayRequestWire | ObligationIdentityError
    semantics: builds the request whose obligation_identity slot is ReplayPackage::obligation_identity of that harness, so a slot that is not that identity is not representable (FR-016-AC-21)
  - name: FrameReplay::new
    inputs: [FrameReplayInputs (proving-run ReplayInputs, domain package ProvidedDocuments, invocation and snapshot ProvidedDocuments, qsl-replay OperationName, invocation DocumentRef, ClaimedChange, the falsified harness's StateFrameIdentity, the playback text; no obligation identity and no transcript)]
    output: FrameReplay (request ReplayRequestWire, envelope WitnessPacket) | FrameReplayError (Dependencies | CallSite(CallSiteRefusal) | Name | Transcript | Envelope | Refused | NotAFrame | FieldSetMismatch | Decode(DecodeFailure) | OutOfDomain | PreState(PreStateFault) | ScopeMismatch | Identity)
    semantics: checks the harness is a frame over exactly its state fields scoped to the requested operation, decodes the playback against the harness's state_fields, checks each decoded value against its declared range, compiles the proved unit with its domain packages and dependencies through qsl_replay::call_site selected by the operation's name, checks the harness's anchor and frame against the answer and the decoded pre state against the pre snapshot the invocation names, then builds the frame counterexample payload from the answer (anchor, frame, frame occurrence) and the obligation identity minted from the answer and the harness; the envelope's clause_node is the payload's frame node and its occurrence_key the payload's frame occurrence; FrameReplay::replay reconstructs the envelope and calls qsl_replay::replay_frame, returning QSL's result (FR-015, FR-024-AC-20 to AC-30)
  - name: StateClauseReplay::new
    inputs: [StateClauseReplayInputs (proving-run ReplayInputs, domain package ProvidedDocuments, the admitted package and the clause's node, qsl-replay OperationName and clause name, the state object's address, three document identity labels, the decoded playback, the post-state values, the obligation identity)]
    output: StateClauseReplay (request ReplayRequestWire, envelope WitnessPacket; replay and replay_through return QSL's StateClauseReplayResult) | StateClauseReplayError (Dependencies | CallSite(CallSiteRefusal) | Name | Transcript | Envelope | Document(DocumentError: Encode | Model(ModelError) | Clause{refusal StateFrameRefusal}) | MissingField{field} | UndeclaredField{field} | DuplicateField{field} | OutOfDomain{field} | UnsupportedOperationShape{operation, declaration} | Refused)
    semantics: builds the invocation and snapshot documents from the playback and the post-state values through core::canonical, locates the clause through qsl_replay::call_site selected by ClauseName, and builds the Witness-arm envelope whose clause_node and occurrence_key are the ClauseSite's; replay_through hands the request and envelope to a caller-supplied executor and replay uses qsl_replay::replay_state_clause (FR-024-AC-11 to AC-19; results settle per FR-029-AC-16)
  - name: replay_counterexample
    inputs: [KaniObligationIdentity, Kani playback transcript, ReplayPackage]
    output: ReplayVerdict (Reproduced | EvidenceFailure(Decode(DecodeFailure) | Domain | Verdict, which carries QSL's DisagreementCause when the settlement is inconclusive)) | SpineReplayError (the replay_falsification variants | Identity(ObligationIdentityError))
    semantics: decodes the transcript, checks every integer value against its argument's declared bounds before any replay, builds the O-09 obligation identity of the harness (ReplayPackage::obligation_identity), replays natively, and reports a decode, domain or verdict mismatch as the one EvidenceFailure verdict, never as a clause success or failure; Boolean and i64 values only (FR-016)
  - name: replay_counterexample_through
    inputs: [KaniObligationIdentity, Kani playback transcript, ReplayPackage, an executor from the built ReplayRequestWire to a qsl-replay ReplayResult or ReplayRefusal]
    output: as replay_counterexample
    semantics: replay_counterexample with the request handed to the caller's executor, which ends in qsl_replay::replay, so a caller can observe the request QSL receives; kept public because the harness fixtures that build a KaniObligationIdentity and its playback live in the integration tests (FR-016-AC-21)
  - name: bound_strategy::census::compute_census
    inputs: [Relation, Domain]
    output: BoundaryCensus | StrategyDiagnostic
    semantics: the Boundary census for one relation over its domain, with the untagged out-of-domain and unrepresentable-edge arrays; fails only for a reversed domain, and a single-tagged census is still returned so its refusal is reported by the census (FR-010)
  - name: bound_strategy::census::render_edge_constants
    inputs: [BoundaryCensus, CensusNames]
    output: rendered Rust source | StrategyDiagnostic
    semantics: renders the untagged out-of-domain and unrepresentable-edge constants that every bundle for a non-refused population request carries (FR-010)
  - name: bound_strategy::census::render_boundary_constants
    inputs: [BoundaryCensus, CensusNames]
    output: rendered Rust source | StrategyDiagnostic
    semantics: renders the tagged in-domain census with the edge constants; refuses UnsupportedCampaignConstraint when the in-domain census is single-tagged (FR-010)
  - name: bound_strategy::population::side_values
    inputs: [Relation, Domain, PopulationSide]
    output: SideValues | StrategyDiagnostic
    semantics: the constructive valuations of one side of a relation over its domain; an empty side refuses EmptyPopulation with terminal state unsupported (FR-009, FR-012)
  - name: bound_strategy::population::render_population
    inputs: [PopulationRequest]
    output: RenderedPopulation | StrategyDiagnostic
    semantics: renders the requested Satisfying, Violating or Broad population; refuses a wrong or non-unique identifier set, the first empty requested side, and source that does not parse or exceeds the source limit (FR-009, FR-012)
  - name: cli_generate
    status: planned; no executable CLI is provided by this library candidate
    inputs: [serialized package path, destination, backend flags]
    output: stable exit status, diagnostics, and published bundle identity
    semantics: equivalent to the library API and never edits developer-owned regions
artifact_bundle:
  scope: planned multi-backend generate_bundle; implemented bound-oracle batches contain only oracle source and source maps
  required:
    - executable Rust oracles
    - tri-state harnesses
    - shaped proptest strategies
    - Kani obligations and proof dependency graph
    - coverage source map and vacuity map
    - diagnostics
diagnostics:
  bound_batch_errors: typed ResourceLimitExceeded, Clause(full ClauseRef plus existing lower-level diagnostics and exact rejected IR source spans for expression failures), or Bundle(existing publication diagnostic); no partial artifacts
  no_executable: separate successful non-artifact result for valid empty or informational-only populations, not a terminal-state claim
  terminal_states: [generated, unsupported, invalid-input, backend-unavailable, io-failed, inconclusive]
  implemented_mapping:
    generated: successful supported Boolean-root lowering, including obligation-free bounded-integer comparisons over direct input and state scalar observations
    unsupported: unsupported expression, dependency, obligation, saturating arithmetic (UnsupportedSaturatingArithmetic), or bounded resource
    invalid-input: non-Boolean root or generated-name collision
    inconclusive: internal syntax or serialization control failure
    backend-unavailable: reserved for external backends
    io-failed: reserved for atomic publication
  rule: no non-generated state may be converted into a complete artifact claim
  fields: [stable code, terminal state, stable input path, exact IR source span required for NonBooleanRoot/UnsupportedExpression/UnsupportedDependency/UnsupportedObligations/UnsupportedSaturatingArithmetic and absent for non-expression failures, optional preserved lower-level generation code, human detail]
oracle_slice:
  schemas: generated Rust and source-map outputs each identify and validate against their own versioned schema
  source_limit: 1048576 bytes per clause, enforced during rendering
  artifact_names: readable names built from the requirement, revision and clause names, used bare when one clause holds the name; clauses sharing a name are suffixed `_{n}`, numbered from 1 in ascending full-identity order; with per-clause source-map paths
  supported_expression_grammar: Boolean literals, Boolean direct value references, Boolean not/operators, bounded i64 literals, bounded i64 direct input/state value references, all six comparisons with a Boolean clause root, and integer add, subtract, multiply, divide and remainder over a `reject` integer type, whose meaning and outcomes FR-031 states (a native oracle with any arithmetic node returns `Outcome<bool>`, one with none returns `bool`, and the V1 Kani bundle's oracle keeps fixed-width `i64` operators under Kani's own checks and returns `bool`)
  integer_representation_boundary: an IR integer declaration or literal may be i128, but this V1 oracle and its generated dependencies require every endpoint and literal to fit i64; the first out-of-range expression refuses as UnsupportedExpression at its source span, with no generated source (FR-031-AC-28)
  dependency_types: Boolean dependencies render as bool; bounded-integer dependencies render as i64; current/pre/post observations remain distinct parameters
  undefined_result_boundary: a typed expression carrying a definedness obligation other than a discharged non-zero divisor or checked range refuses before rendering; numeric negation and every add, subtract, multiply, divide or remainder node over a `saturate` integer type are unsupported; an add, subtract, multiply, divide or remainder node over a `reject` integer type is rendered, other than in the V1 Kani bundle, as an exact-kernel (`quire-exact`) operation whose refused or undefined outcome is the oracle's outcome (FR-031), so no checked result is unwrapped or defaulted into bool; in the V1 Kani bundle oracle it is a fixed-width operator whose overflow or zero divisor is Kani's falsifiable check, never a wrapped value (FR-031)
  refusal_locus: NonBooleanRoot carries the clause-root SourceSpan; UnsupportedExpression, UnsupportedDependency and UnsupportedSaturatingArithmetic carry the first rejected expression node's SourceSpan in authored preorder (the last is a refusal whose message names the missing runtime saturating operation, FR-031); UnsupportedObligations carries the first retained obligation's SourceSpan in IR order; none produces a partial artifact
  invalid_generated_syntax: generate_boolean_oracle's GenerationErrorCode::InvalidGeneratedSyntax means generated source that did not parse, or whose rendered lines did not match its own source map (the missing-line case of NFR-005); the stable code is unchanged and no artifact is produced
harness_strategy_slice:
  output: generated Rust artifact
  campaign_policy: minimum accepted, minimum rejected, and maximum explicit-discard invocation counts are caller-supplied, rendered once as generated constants, and bound into deterministic request identity; zero is the valid rejected floor for a declared total precondition
  campaign_execution: the public generated runner is the campaign-level entry point; it owns the proptest loop, creates observations, records every explicit discard, invokes the private verdict adapter, and always classifies retained accounting as passed, below an accepted/rejected floor, above the explicit-discard ceiling, exhausted, or failed
  accounting_unit: accepted, rejected, and failed count adapter invocations; discarded counts explicit discards that do not invoke the adapter; attempted equals accepted plus rejected plus discarded, including global-reject retries and shrink replays rather than distinct generated values, and supplies the exact denominator for rejected/attempted and discarded/attempted rates
  discard_boundary: a precondition rejection returned by the test closure is a proptest global reject recorded in rejected, while the generated discarded constructor is a separate explicit-discard channel recorded in discarded; the caller-owned max_global_rejects setting governs framework search exhaustion and the generated policy governs the retained invocation counters
  campaign_conclusion: policy applies to the complete supplied report including prior invocations/discards; completed searches enforce the discard ceiling then accepted/rejected floors; every framework Abort returns Exhausted with its reason, summary, and optional boxed policy failure, including all-rejected and zero-attempt aborted searches; a completed fresh zero-case campaign returns BelowAcceptedFloor, and explicit discards above the requested ceiling produce a distinct typed result
  expected_domain: generated integer cases expose executable accepted/rejected verdict checks; generated enum populations contain declared admissible members only and execute their admission expectation; generated Boolean campaign constructors bind accepted, rejected, or explicit-discarded disposition to the exact values consumed by the owned runner
  generated_crate_lints: generated crate roots deny missing documentation and compile under denied warnings
  source_limit: harness and strategy Rust are rejected above 1048576 bytes before bundling
  artifact_names: readable names built from the request's requirement, revision and clause names
bound_strategy_slice:
  requirements: [FR-008, FR-009, FR-010, FR-011, FR-012, FR-013, NFR-004]
  depends_on: FR-014 bounded-integer comparison oracles; quire-contract-ir FR-012 through FR-015 and FR-023; quire-contract-runtime FR-001, FR-003, FR-004 and interface-001
  terms: the domain of a read is the inclusive minimum..=maximum of its integer declaration, not the IR IntegerType.domain representation field (quire-contract-ir FR-013); a read is one value-reference operand identified by declaration SymbolName and observation current, pre, or post (quire-contract-ir FR-014); the primary read is the left operand when it is a read, otherwise the right; the partner read is the other operand when both are reads
  admission: first the same admission generate_bound_oracles applies; then clause kind Precondition, Postcondition, or Invariant at any anchor the IR accepts; then a root of exactly one Compare whose operands are each an integer read or an IntegerLiteral, with at least one read, not the same read twice, and not Current mixed with Pre or Post of one declaration
  refusals: new variants of the existing StrategyErrorCode, checked in order UnknownClause (invalid-input), UnsupportedClause (preserves the oracle's code, terminal state, and span), UnsupportedClauseKind (unsupported), UnsupportedRelation (unsupported); then per population EmptyPopulation (unsupported) and UnsupportedCampaignConstraint (unsupported); each carries the full ClauseRef; UnsupportedClause carries the oracle diagnostic's span and UnsupportedRelation the first offending node's SourceSpan in authored preorder, per the oracle refusal locus, while UnknownClause, UnsupportedClauseKind, EmptyPopulation, and UnsupportedCampaignConstraint carry no span; a literal-only Compare and a Boolean or Text comparison refuse as UnsupportedRelation; no partial bundle or artifact; UnknownClause adds invalid-input for an unknown strategy ClauseRef to implemented_mapping, and UnsupportedClause keeps the oracle's terminal state, including invalid-input for a non-Boolean root
  domains: inclusive IntegerType minimum/maximum; both operands of one Compare share one IntegerType; no caller-supplied range
  cases: a complete valuation of the clause's reads, including Post reads; the tag is the value the generated oracle returns for it and claims nothing about any other clause
  populations: Satisfying and Violating are constructed directly (interval draws and primary-dependent partner draws, 128-bit checked size and index arithmetic); Broad holds both with side-preserving tags and requires both sides non-empty; no filter, assume, reject, or discard
  census: Boundary is a generated constant in-domain census of primary edges min, min+1, max-1, max and literal edges k-1, k, k+1, each paired with partners v-1, v, v+1 in domain, tagged Holds or Violated, ordered by primary then partner, deduplicated, refused when single-tagged
  out_of_domain: every bundle generated for a non-refused population request carries a generated constant array of untagged out-of-domain edge cases for consumer domain-admission tests; the codegen runner never evaluates them and no counter records them
  unrepresentable_edges: edge values outside i64 are listed in a generated constant array by read, edge, and direction, never clamped or wrapped
  runner: subject-free oracle conformance; the oracle result maps by clause kind (Precondition false to RejectedPrecondition, Postcondition false to FailedPostcondition, Invariant false to FailedPostcondition with an Invariant observation, true to Passed); verdicts are built through runtime construct_verdict with the IR ExecutionPoint's serialized name and recorded through runtime record_campaign_verdict, so counters follow quire-contract-runtime FR-004; the runtime adapt_to_proptest operations are deliberately not used, and rejected preconditions stay counted only in rejected, never as successful evidence; a verdict equal to the tag's prediction returns a passing proptest result, otherwise ConformanceMismatch; never a global reject or explicit discard; the census runner evaluates each in-domain census case once in order without shrinking; a false invariant uses the failure detail the runtime provides for a contract clause until quire-contract-runtime specifies one; an identity mismatch from recording fails the campaign; FR-002 floors, ceiling, and Exhausted conclusions apply unchanged, and a proptest failure concludes ConformanceMismatch after the discard-ceiling check, never Failed
  rates: the bound-strategy summary alone exposes discard_rate() and rejection_rate(), each Some((numerator, attempted)), or None at zero attempted or when the report snapshot is at_limit (quire-contract-runtime FR-004); a public generated summary_from_snapshot operation exposes this calculation without inventing a resumable report or local counter format; the harness summary is unchanged
  shrinking: partner values derive from the current primary value at every step; tags never change; Broad never shrinks across sides; shrink replays count in attempted
  consumer: case types expose one i64 field per read named by the generated oracle's dependency parameter identifier, plus constant IR declaration SymbolName and observation-name (current, pre, post; input declarations are always current) strings, where a quire-spec-language field projection's SymbolName is SL's deterministic field alias mapped back through SL FR-034 read correspondence; generated code depends only on proptest, quire-contract-runtime, and core/std; no serialized case, census, or summary format and no new schemas/ file; the generated header states the full ClauseRef
  out_of_scope: Boolean connectives over comparisons; Boolean literal, reference, and negation roots; a Compare of one read with itself; Current mixed with Pre or Post of one declaration; arithmetic, negation, and definedness obligations: relation admission accepts only integer reads and literals as comparison operands, so an arithmetic operand is refused as UnsupportedRelation when its oracle generates, and oracle admission refuses negation, an undischarged obligation and, from FR-031, divide, remainder and `saturate` arithmetic, so no admitted clause has an overflow edge; a numeric subject harness where an operation produces the post-state; StatePinned and NoEvent campaigns, which stay on the caller-constraint generate_i64_strategy API
kani_slice:
  semantics_source: the executable-oracle analyzer and rendered predicates are reused exactly; the Kani adapter does not carry a second expression interpreter. One exception: integer add, subtract and multiply in the V1 bundle oracle are rendered as fixed-width infix `i64` operators under Kani's overflow check, where the native oracle calls Contract Runtime. That is a second implementation of the arithmetic rule, kept because exact arithmetic gave no proof verdict for the exemplar clause and the exemplar's mutation control targets the infix text, and held in step with the native oracle by FR-031-AC-21
  argument_binding: unique direct current input, current state and pre-state dependencies become ordered subject arguments; order is normalized dependency identity, not source spelling or traversal accident
  result_binding: unique direct post-state dependencies become the subject result; zero uses `()`, one uses its primitive, and multiple use an ordered tuple
  binding_identity: uniqueness and ordering use the full checked DependencyIdentity of kind, normalized path and observation; repeated identical references unify, while incompatible declarations for one logical kind/path refuse
  primitive_types: Boolean maps to bool; bounded integer maps to i64 with the checked IntegerType domain, inclusive minimum, inclusive maximum and overflow policy retained
  bounds: every symbolic i64 argument receives an inclusive `kani::assume` from its checked IR model domain; post-state i64 results must satisfy the same domain in ensures; no caller range, proptest strategy, clamp or widened machine range substitutes
  compatibility: the existing one-Boolean-input plus one-Boolean-pre/post-state transition is the corresponding generalized ABI instance and retains its semantics
  outputs: generated Rust profile `quire.codegen.rust-kani/v2` plus schema-validated graph `quire.codegen.kani-proof-graph/v2`
  completion_boundary: graph readiness is derived from the full dependency census, but `proofExecutionState` is always `not_run`; generation does not claim that Kani proved the contract
  dependency_rule: missing or failed required edges yield incomplete; any assumption or stub yields conditional; only passed required edges yield ready
  source_sites: every proof assumption and stub has one source marker and one graph edge; model-domain assumptions are separate typed binding records and never completed proof edges
  framing: generated contracts quantify only over copied primitive arguments and returned primitive values; they claim no unmodeled global, heap, alias, indirect, object or graph state
  subject_boundary: generation validates the customer subject as a Rust path and derives its required signature from checked bindings, but does not inspect or execute the external function; signature/link/body failures are external Rust or Kani observations and cannot become successful generation or proof claims
  options: exact adapter options are `-Z function-contracts`, optional `-Z stubbing`, `-Z concrete-playback`, exact fully qualified harness, `--exact`, explicit unwind, explicit solver, `--output-format regular`, and `--concrete-playback print`; the graph retains the complete ordered vector
  refusal: UnsupportedBinding identifies post-state in a precondition, unsupported observations/dependency shapes, cross-clause type/domain conflicts and unrepresentable ABI; ClauseGenerationFailed retains the originating executable-oracle code and SourceSpan for definedness obligations or unsupported expressions; InvalidIdentity, InvalidDependency, InvalidUnwind, InvalidGeneratedSyntax, SerializationFailed and ResourceLimitExceeded remain distinct stable codes; every refusal returns no partial output
  replay_boundary: printed Kani concrete-playback data and the graph's typed binding order are the inputs CG types into QSL's backend-witness transcript for `qsl_replay::replay`; the native verdict is QSL's evaluation, and CG produces none
kani_obligation_execution_slice:
  requirements: [FR-017]
  scope: running FR-015 harnesses, one per call (`execute_kani_obligation`) or many in one call (`execute_kani_obligations`, which forms groups of harnesses that share one launcher process; a batch is the caller's list, a group is one process); FR-015 generation, and the FR-014 oracles it embeds, have no slice of their own yet and are governed by their requirements alone
  refusals: a refusal when the crate's library source does not contain the harness source byte for byte (`HarnessNotInCrate`; in a batch it refuses the whole call before any process starts), a tool refusal naming a launcher that cannot be started and its path, and, after the run, a report refusal (`KaniExecutionRefusal::Report`) when the exported report is absent after a successful exit, unreadable, over the read bound, malformed, of another schema version, holding other than one harness result in a single run, or contradicting itself (a success harness that lists a failed, errored, undetermined or unknown check). The first two run nothing; a report refusal is never an outcome. A launcher stream over 8 MiB per harness is `OutputOverLimit` (stable code `kani_output_over_limit`, naming the stream, the limit and the harness count) and one that was not read (a capture thread panicked, or a poll or read failed) is `OutputUnread` (`kani_output_unread`); each stops the run and kills the process group. A group refused in a batch (its process is the unit, other groups keep their results, no member of it is classified): `BatchTimedOut` (the outer bound of N times T elapsed), a batch report that `HarnessMissing`, `HarnessDuplicated` or `HarnessUnrequested` (typed `KaniReportRefusal` causes), a missing report after a clean exit (`Report(Missing)`), and `PlaybackForNonMember` (a console block headed for a path that is not a member; a member's several counterexample blocks are not this, it takes the first)
  outcomes: `verified`, `falsified` with the concrete playback verbatim, `cover_unsatisfied` with satisfied and total counts, and `inconclusive` with one of `failed_without_counterexample`, `no_verdict`, `missing_cover_summary`, `vacuous_proof`, `unwind_bound_exhausted`, `timed_out`. Success is never defaulted: without a report that lists at least one successful check and every cover satisfied, a successful run is not `verified`. A run is given a caller-declared wall-clock budget on every request; one that has not concluded when the budget elapses is killed, along with every process it forked that a `/proc` walk taken at that moment can still see (one forked or reparented away in the instant before that walk is not guaranteed reached, only that the caller is never made to wait for it), and reported `inconclusive`/`timed_out` rather than left running
  evidence: the kind, harness path, launcher path, complete argument vector, unwind bound, solver, exit code, outcome, SUCCESS-check count and the per-check view (id, class, source file and line, status); for a member of a group of more than one harness, the argument vector is the group's complete vector and the evidence also carries the `batch` statement (`KaniBatchInvocation`: the members' `module::harness` paths in request order and the request timeout T in whole seconds, rounded up), which is absent from a single run's wire form. `launch_evidence` returns `KaniExecutionRefusal`. This repository retains none of it and computes no aggregate verdict
  outcome_source: the outcome is read from the report Kani exports under `-Z unstable-options --export-json`, in `src/kani/output/report.rs`, which returns a typed report; a report that is absent after a successful exit, unreadable, over the read bound, of another schema version, malformed, of an unknown check status or holding other than one harness result, or stating success while listing a failed, errored, undetermined or unknown check (`Inconsistent`) is a typed KaniReportRefusal, never an outcome; the report carries no concrete playback, so a falsifying playback block is taken from the console as a payload only and passed through verbatim to the FR-016 witness join
coverage_analysis_slice:
  implementation_boundary: parse_llvm_coverage, LlvmCoverage.observe, and classify_clause remain unbound observation primitives; the separate bound observation API performs no native-qualified aggregate analysis, campaign binding, or obligation discharge
  bound_observation_boundary: analyze_bound_coverage emits the strict codegen.bound-coverage-observations/v1 domain schema from a complete immutable generated bundle plus independently validated BoundPackage; no campaign binding, native execution, or obligation discharge
  bound_observation_output: complete ordered full-ClauseRef observations with independent typed implication census; computation state complete/incomplete/invalid_input/unsupported/no_executable; global refusal emits no clause observations and population not_emitted; null means unavailable, never measured zero; at most 16 MiB serialized bytes
  bound_source_root: at most 4096 UTF-8 bytes before normalization; the accepted canonical absolute root is retained in the report as the mapping configuration; root aliases use existing LLVM parser normalization, not filesystem resolution
  bound_count_consistency: within exact loop-free generated Boolean source, every measured consequent count is at most its owning oracle evaluation count; greater counts are retained with inconsistent_observation and no classification, never upgraded to exercised; the unbound primitive remains unchanged
  expected_population: immutable IR-owned executable clauses with typed expression, clause kind, execution anchor, declarations, dependencies and spans; exact artifact and map population equality; empty executable population is not_computed
  source_map: exactly one clause envelope carrying expectedConsequents counted independently from typed IR, one oracle_evaluation entry probe, and every implication_consequent entry probe
  probes: one-based single-line UTF-8 byte columns with exclusive end; function-declaration entry token for evaluation and first expression token for each consequent; probes are required for semantic roles and forbidden on clause envelopes
  llvm_export: full JSON export with type llvm.coverage.json.export, exact version 3.0.1, file segments present, and exactly one lexically normalized filename matching the generated artifact path under the caller-declared source root
  input_bounds: at most 16 MiB raw export before deserialization, 4096 files and 250000 segments across the export; strictly ordered nonzero coordinates and exactly six typed tuple fields; final transition has no inferred infinite extent
  path_semantics: normalize dot and repeated-separator aliases, reject parent traversal and backslashes, strip exact absolute package-root boundary; external absolute dependency paths cannot match generated paths; reject duplicate eligible normalized filenames
  segment_semantics: a single count-bearing non-gap active span must contain the entire entry-token probe; measured zero is distinct from unavailable data; no partial intersection or summary fallback
  classification:
    unexecuted: measured zero evaluation and no positive consequent observations
    vacuous: positive evaluation with a nonempty expected implication population and zero positive consequents
    partially_exercised: positive evaluation with a proper nonempty subset of expected consequents observed
    exercised: positive evaluation with every expected consequent observed, including the independently known empty implication set
    unavailable: no measured classification when input or a complete probe observation is absent
    inconsistent: positive consequent observation with zero oracle evaluation is rejected
  campaign_report: the analyzer consumes the quire-contract-runtime CampaignReport type so its complete counter set and failed-is-a-subset-of-accepted invariant are not restated by a caller-owned lookalike
  test_outcomes: [passed, failed, aborted, not_computed]
  independent_facts: accepted, rejected, failed, and discarded runtime counts plus test outcome are retained verbatim and never used to upgrade coverage classification
  accounting: failed postconditions prohibit passed execution; positive oracle coverage with zero invocations is inconsistent when the generated campaign is declared the only execution source
  revision: compare IR and source-map u64 revision to runtime RevisionId using the canonical decimal string without leading zeroes
  invalid_input: stable non-success analysis outcome retains available diagnostics without fabricated measured classifications
  default_gate: every executable clause exercised and native execution passed; adverse or unavailable coverage cannot discharge obligation; no automated human sufficiency or exception decision
  qsl_native_result: the planned analyze_coverage entry accepts only the QSL strict reader's typed clause-run or command-error native-run-result/2 variant, or a located wire refusal; clause-run stage/category/truth, basis and optional witness remain QSL semantic facts, separate from CG generated-campaign execution, CampaignReport counts and LLVM observations (FR-004)
  qsl_result_refusals: missing or non-/2 format and malformed or mixed clause-run/command-error shape remain located QSL reader refusals; a valid command-error variant retains stage/code/optional cause/message/details and unavailable basis as non-success; no content sniffing, /1 fallback, message parsing or invented witness member
  qsl_result_gate: the separate CG-owned receipt authenticates the exact QSL result, producer execution, generated source, source map and exact LLVM export bytes from that same run, including its producer tool/version; absent, unauthenticated or mismatched association refuses binding before measured classification while retaining the decoded QSL outcome; even matching authentication does not promote success, adverse semantic result or command error to generated-campaign execution or coverage
compatibility:
  generated_runtime_dependency: quire-contract-runtime, proptest, plus declared customer types only
  licensing: crate AGPL-3.0-or-later; emitted Rust carries the MIT OR Apache-2.0 SPDX identity required by NFR-002
  publication: disabled through the human v0.1 source-release decision
open_design_gates:
  native_campaign_transport: the runtime exposes CampaignSnapshot and an optional snapshot-json decoder; this codegen package does not yet consume that feature, and structural decoding cannot authenticate native execution; no Display parsing, fake verdict replay, or private counter lookalike may substitute
  serialized_package_cli: the public IR derived-projection decoder now supplies BoundPackage; cli_generate remains unimplemented, and normal projection production remains the authoritative frontend/model lane rather than a codegen-owned authored sidecar
```

## Features

The interface's features in declaration order: every operation the contract above declares, one row each.

| Feature | Kind |
|---|---|
| generate_bound_oracles | operation |
| generate_bundle | operation |
| generate_tristate_harness | operation |
| generate_i64_strategy | operation |
| generate_enum_strategy | operation |
| generate_bound_strategy | operation |
| generate_kani_bundle | operation |
| write_bundle_atomic | operation |
| analyze_coverage | operation |
| analyze_bound_coverage | operation |
| generate_boolean_oracle | operation |
| classify_clause | operation |
| parse_llvm_coverage | operation |
| classify_bounded_kani_profile | operation |
| prepare_checked_arithmetic | operation |
| prepare_bounded_collection_query | operation |
| prepare_finite_graph_reaches | operation |
| generate_bounded_kani_corpus_case | operation |
| generate_exact_scalar_oracles | operation |
| derive_exact_scalar_items | operation |
| negotiate_kani_obligations | operation |
| execute_kani_obligation | operation |
| execute_kani_obligations | operation |
| kani_launch_command | operation |
| run_launcher_with_timeout | operation |
| launch_evidence | operation |
| classify_kani_run | operation |
| run_terminal_value | operation |
| ir_outcome_terminal_value | operation |
| composite_parity_terminal_value | operation |
| verified_shadow_terminal_value | operation |
| generate_composite_equality_oracles | operation |
| negotiate_backend_provider | operation |
| generate_routed | operation |
| generate_exact_function_oracles | operation |
| decode_falsification | operation |
| replay_falsification | operation |
| bound_strategy::census::compute_census | operation |
| bound_strategy::census::render_edge_constants | operation |
| bound_strategy::census::render_boundary_constants | operation |
| bound_strategy::population::side_values | operation |
| bound_strategy::population::render_population | operation |
| cli_generate | operation |

## Open items

- `generate_bundle`, `analyze_coverage` and `cli_generate` are declared `status: planned`. Criteria
  for their semantics belong with the requirement that implements them, once one exists.
- The remaining prose fields this contract's slices carry — admission order, refusal vocabulary,
  domain and campaign rules, and so on — are the executable half of the FR that owns each slice
  (FR-008 through FR-013 for `bound_strategy_slice`, FR-017 for
  `kani_obligation_execution_slice`, and so on) and are backed there rather than restated here.

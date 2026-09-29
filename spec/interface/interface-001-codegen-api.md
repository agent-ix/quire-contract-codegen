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
    status: implemented for the codegen#3 numeric/state slice on the codegen#4 numeric oracle grammar from PR #29
    inputs: [public BoundPackage, full ClauseRef, population Satisfying|Violating|Broad|Boundary, campaign policy]
    output: GeneratedArtifactBundle | StrategyDiagnostic
    semantics: admits only clauses bound oracle generation admits, narrowed to one Compare; domains taken from the clause's IR IntegerType declarations; constructive Holds/Violated populations, an exhaustive in-domain census and an untagged out-of-domain array; a subject-free oracle-conformance runner whose summary reports discard and rejection rates; see bound_strategy_slice
  - name: generate_kani_bundle
    inputs: [typed precondition, typed postcondition, direct checked dependency types and observations, subject path, dependency census]
    output: KaniArtifactBundle | KaniDiagnosticSet
    semantics: deterministic Boolean/i64 Kani source and v2 dependency/binding graph using exact oracle predicates and IR-owned model bounds; generation records proof execution as not_run and never claims proof completion
  - name: write_bundle_atomic
    inputs: [ArtifactBundle, destination directory]
    output: PublishedBundleIdentity | IO diagnostic
    semantics: stage and validate the complete bundle, then replace the destination directory; caller serializes destination writers; inspection/read failures return io_failed with unchanged state; failed rollback reports unknown and preserves backup/staging for recovery; post-commit cleanup failures report published; process crashes between directory renames and power-loss durability remain outside the portable rollback guarantee
  - name: analyze_coverage
    status: planned; public IR-owned bound population is available, native run-result contract and aggregate analysis integration remain pending
    inputs: [bound executable population, generated source and maps, LLVM coverage JSON bytes, native producer and run identities, source root, runtime campaign report, execution outcome]
    output: versioned structured AnalysisOutcome including non-success diagnostics and available input identities
    semantics: coverage obligation succeeds only for nonempty complete exercised population with successful bound execution; successful serialization is not successful coverage; never executes LLVM
  - name: analyze_bound_coverage
    status: implemented phase A after coordinator approval of REV-017; full native-run analysis remains pending
    inputs: [public BoundPackage, immutable BoundOracleGeneration, complete artifact bytes, optional LLVM export bytes, source root]
    output: immutable versioned BoundCoverageAnalysis domain observations
    semantics: exact independent clause and implication census; whole-batch artifact/map binding; measured clauses or explicit unavailable states; valid informational-only population is no_executable; no runtime transport, producer execution, or assurance verdict
  - name: generate_boolean_oracle
    inputs: [OracleRequest over one typed Boolean clause]
    output: OracleArtifactBundle | GenerationDiagnostic list
    semantics: one deterministic Boolean oracle and its source map, or diagnostics with no partial bundle; the single-clause core generate_bound_oracles batches (FR-001)
  - name: classify_clause
    inputs: [oracle entry probe observation, independently derived expected consequent count, consequent probe observations]
    output: ClauseCoverage | CoverageDiagnostic
    semantics: bounded observation primitive classifying one clause's measured probes; never an aggregate coverage verdict, and the expected count must not be derived from source-map rows (FR-004)
  - name: parse_llvm_coverage
    inputs: [LLVM coverage JSON bytes, absolute source root]
    output: LlvmCoverage | CoverageDiagnostic
    semantics: parses an LLVM full JSON coverage export without executing any producer; parent traversal and backslashes refuse, with no suffix matching or summary fallback (FR-004)
  - name: classify_bounded_kani_profile
    inputs: [KaniProfile, requested constructs, source id]
    output: CapabilityEntry list | KaniOutcome
    semantics: classifies one request through the public bounded Kani profile with no reverse Contract IR dependency; a malformed request is a typed refusal, not a disposition (FR-007)
  - name: prepare_checked_arithmetic
    inputs: [KaniProfile, DispatchIndex, ValidatedFiniteInput, CheckedArithmeticRequest]
    output: ArithmeticLowering | KaniOutcome
    semantics: bounded checked-arithmetic lowering; division by zero, overflow, invalid ranges, profile refusal and dispatch mismatch stay Contract IR typed non-Boolean outcomes, never an assumption or partial artifact (FR-007)
  - name: prepare_bounded_collection_query
    inputs: [KaniProfile, DispatchIndex, ValidatedFiniteInput, CollectionQuery]
    output: CollectionLowering | KaniOutcome
    semantics: bounded collection-query lowering; bound exhaustion and profile or dispatch refusal stay typed non-Boolean outcomes with no partial artifact (FR-007)
  - name: prepare_finite_graph_reaches
    inputs: [KaniProfile, DispatchIndex, ValidatedFiniteInput, GraphRequest]
    output: GraphLowering | KaniOutcome
    semantics: finite reference-graph reachability lowering; identity, snapshot and reference validation stay Contract IR-owned, and malformed or exhausted requests stay typed non-Boolean outcomes (FR-007)
  - name: generate_bounded_kani_corpus_case
    inputs: [KaniProfile, DispatchIndex, ValidatedFiniteInput, BoundedCorpusRequest, proof dependency census, shared EmittedCorpusIdentities]
    output: BoundedCorpusCase | KaniOutcome
    semantics: one bounded Kani corpus case and its proof dependency graph; a case whose identity the shared registry already holds refuses as kani_corpus_identity_collision rather than overwriting earlier artifacts (FR-007)
  - name: replay_codegen_counterexample
    inputs: [Contract IR CounterexamplePacket, native executor over the finite input]
    output: ReplayAgreement | KaniOutcome
    semantics: replays a Kani counterexample natively; packet and population validation and the requirement for a native false result stay Contract IR-owned, and a disagreement is never repaired into a proof (FR-007)
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
    semantics: settles every item in request order and, only when no item is invalid, emits one harness per supported item; an invalid item returns no harness bytes (FR-015)
  - name: execute_kani_obligation
    inputs: [KaniExecutionRequest]
    output: KaniExecutionEvidence | KaniExecutionRefusal
    semantics: refuses a crate that does not contain the harness, runs the harness and reports the backend's own outcome; see kani_obligation_execution_slice (FR-017)
  - name: kani_launch_command
    inputs: [KaniExecutionRequest]
    output: argv and Command
    semantics: the exact launch command execute_kani_obligation runs, exposed so a caller interleaving its own steps runs that command rather than a copy (FR-017)
  - name: run_launcher_with_timeout
    inputs: [Command, timeout]
    output: LaunchOutcome | io error
    semantics: the bounded launch execute_kani_obligation performs, draining stdout and stderr on their own threads and killing the process at the timeout (FR-017)
  - name: launch_evidence
    inputs: [LaunchOutcome]
    output: KaniRunOutcome and exit code
    semantics: the mapping execute_kani_obligation applies from a concluded launch to evidence; a timed-out launch has no exit code (FR-017)
  - name: classify_kani_run
    inputs: [process exit success, Kani transcript text]
    output: KaniRunOutcome
    semantics: the transcript classifier execute_kani_obligation uses, so a test asserting falsification routes through production classification and an inconclusive run is never read as a decided failure (FR-017)
  - name: generate_composite_equality_oracles
    inputs: [admitted CheckedPackageV2, CompositeEqualityItem list]
    output: CompositeEqualityOracles | OracleGenerationError
    semantics: composite equality oracles plus a typed claim map identical to claim-map.json; per-item problems are refusals, and only a whole-generation failure is an error (FR-018)
  - name: negotiate_backend_provider
    inputs: [BackendProviderEnvelope]
    output: ItemSettlement list | EnvelopeRefusal
    semantics: settles every item of the envelope in request order against a closed backend kind; an item naming a backend nothing here can settle for settles as invalid-request/unknown-backend inside the Ok list, never as one that can; the whole envelope is refused only for an unsupported contract_version or capability_vocabulary (FR-019)
  - name: generate_routed
    inputs: [admitted CheckedPackageV2 reference, RoutedGenerationItem list (request_index usize, node_id CheckedNodeId, backend Candidate, kind BackendKind), GenerationContexts (one Option field per BackendKind; kani is KaniGenerationContext of subject_path and unwind u32)]
    output: RoutedGeneration (items, one RoutedItemOutput of request_index, backend and KindOutput per routed item in ascending request_index; rejected BackendKind list; claim_map Option<ClaimMap<ExactScalarClaim>>, Some after a Kani group; oracle_artifacts Option<Vec<Artifact>>, the FR-014 oracle crate of that group, Some after a Kani group) | RoutedGenerationError (DuplicateRequestIndex{request_index} | BackendKindDisagrees{request_index, backend, routed, converted Option<BackendKind>} | MissingKindContext{kind} | Kani(KaniObligationError) | Oracle(OracleGenerationError))
    semantics: runs each routed item's backend-kind generation arm over an exhaustive BackendKind match without re-settling, re-selecting or re-routing a backend; the Kani arm derives each node's claim with derive_exact_scalar_items and generate_exact_scalar_oracles, then runs negotiate_kani_obligations over the routed Kani items in ascending request_index, with every record index rewritten to the driver's request index and harnesses joined by harness_symbol, and returns the generated oracle crate unchanged; no FR-019 Disposition is constructed (FR-022)
  - name: generate_exact_function_oracles
    inputs: [admitted CheckedPackageV2, ExactFunctionDeclaration list, ExactFunctionItem list]
    output: ExactFunctionOracles | OracleGenerationError
    semantics: function-application oracles over the declared functions, plus a typed claim map; per-item problems are refusals, and only a whole-generation failure is an error (FR-021)
  - name: witness_schema
    inputs: [ObligationBinding list of one harness]
    output: WitnessBinding list | WitnessSchemaError
    semantics: the witness schema for a harness's symbolic arguments, position for position with its kani::any() calls; a binding that is not a symbolic argument refuses (FR-016)
  - name: decode_falsification
    inputs: [harness symbol, module symbol, ObligationBinding list, Kani playback transcript]
    output: named WitnessValue list | KaniOutcome
    semantics: joins one Kani assertion-playback witness to the harness's persisted obligation schema, refusing on schema, transcript, harness-identity or decode mismatch (FR-016)
  - name: replay_falsification
    inputs: [harness symbol, check text, named WitnessValue list, ReplayParameter list (argument name, node id), a function from the witness ReplaySource to the complete ReplayRequestWire]
    output: the qsl-replay WitnessArmResult | SpineReplayError (UnboundArgument{argument} | FieldDelimiter | Transcript(MalformedTranscript) | Refused(ReplayRefusal) | WrongArm)
    semantics: builds the backend-witness transcript keyed by parameter node id and calls qsl_replay::replay, returning QSL's own settlement; a Boolean value is replayed as 1 or 0 (FR-016)
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
  bound_batch_errors: typed ResourceLimitExceeded, NameCollision(full ClauseRef), Clause(full ClauseRef plus existing lower-level diagnostics and exact rejected IR source spans for expression failures), or Bundle(existing publication diagnostic); no partial artifacts
  no_executable: separate successful non-artifact result for valid empty or informational-only populations, not a terminal-state claim
  terminal_states: [generated, unsupported, invalid-input, backend-unavailable, io-failed, inconclusive]
  implemented_mapping:
    generated: successful supported Boolean-root lowering, including obligation-free bounded-integer comparisons over direct input and state scalar observations
    unsupported: unsupported expression, dependency, obligation, or bounded resource
    invalid-input: non-Boolean root or generated-name collision
    inconclusive: internal syntax or serialization control failure
    backend-unavailable: reserved for external backends
    io-failed: reserved for atomic publication
  rule: no non-generated state may be converted into a complete artifact claim
  fields: [stable code, terminal state, stable input path, exact IR source span required for NonBooleanRoot/UnsupportedExpression/UnsupportedDependency/UnsupportedObligations and absent for non-expression failures, optional preserved lower-level generation code, human detail]
oracle_slice:
  schemas: generated Rust and source-map outputs each identify and validate against their own versioned schema
  source_limit: 1048576 bytes per clause, enforced during rendering
  artifact_names: readable names built from the requirement and clause names plus a positional counter, with per-clause source-map paths
  supported_expression_grammar: Boolean literals, Boolean direct value references, Boolean not/operators, bounded i64 literals, bounded i64 direct input/state value references, and all six comparisons with a Boolean clause root
  dependency_types: Boolean dependencies render as bool; bounded-integer dependencies render as i64; current/pre/post observations remain distinct parameters
  undefined_result_boundary: a typed expression carrying any definedness obligation refuses before rendering; every numeric arithmetic and numeric-negation node remains unsupported even without an obligation until a versioned IR/runtime result can distinguish invalid from false; no checked result is unwrapped or defaulted into bool
  refusal_locus: NonBooleanRoot carries the clause-root SourceSpan; UnsupportedExpression and UnsupportedDependency carry the first rejected expression node's SourceSpan in authored preorder; UnsupportedObligations carries the first retained obligation's SourceSpan in IR order; none produces a partial artifact
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
  artifact_names: readable names built from the request's requirement and clause names plus a positional counter
bound_strategy_slice:
  requirements: [FR-008, FR-009, FR-010, FR-011, FR-012, FR-013, NFR-004]
  depends_on: FR-001-AC-8 bounded-integer oracle grammar (codegen#4, merged by PR #29); quire-contract-ir FR-012 through FR-015 and FR-023; quire-contract-runtime FR-001, FR-003, FR-004 and interface-001
  terms: the domain of a read is the inclusive minimum..=maximum of its integer declaration, not the IR IntegerType.domain representation field (quire-contract-ir FR-013); a read is one value-reference operand identified by declaration SymbolName and observation current, pre, or post (quire-contract-ir FR-014); the primary read is the left operand when it is a read, otherwise the right; the partner read is the other operand when both are reads
  admission: first the same admission generate_bound_oracles applies; then clause kind Precondition, Postcondition, or Invariant at any anchor the IR accepts; then a root of exactly one Compare whose operands are each an integer read or an IntegerLiteral, with at least one read, not the same read twice, and not Current mixed with Pre or Post of one declaration
  refusals: new variants of the existing StrategyErrorCode, checked in order UnknownClause (invalid-input), UnsupportedClause (preserves the oracle's code, terminal state, and span), UnsupportedClauseKind (unsupported), UnsupportedRelation (unsupported); then per population EmptyPopulation (unsupported) and UnsupportedCampaignConstraint (unsupported); each carries the full ClauseRef; UnsupportedClause carries the oracle diagnostic's span and UnsupportedRelation the first offending node's SourceSpan in authored preorder, per the codegen#4 refusal locus, while UnknownClause, UnsupportedClauseKind, EmptyPopulation, and UnsupportedCampaignConstraint carry no span; a literal-only Compare and a Boolean or Text comparison refuse as UnsupportedRelation; no partial bundle or artifact; UnknownClause adds invalid-input for an unknown strategy ClauseRef to implemented_mapping, and UnsupportedClause keeps the oracle's terminal state, including invalid-input for a non-Boolean root or a batch NameCollision
  domains: inclusive IntegerType minimum/maximum; both operands of one Compare share one IntegerType; no caller-supplied range
  cases: a complete valuation of the clause's reads, including Post reads; the tag is the value the generated oracle returns for it and claims nothing about any other clause
  populations: Satisfying and Violating are constructed directly (interval draws and primary-dependent partner draws, 128-bit checked size and index arithmetic); Broad holds both with side-preserving tags and requires both sides non-empty; no filter, assume, reject, or discard
  census: Boundary is a generated constant in-domain census of primary edges min, min+1, max-1, max and literal edges k-1, k, k+1, each paired with partners v-1, v, v+1 in domain, tagged Holds or Violated, ordered by primary then partner, deduplicated, refused when single-tagged
  out_of_domain: every bundle generated for a non-refused population request carries a generated constant array of untagged out-of-domain edge cases for consumer domain-admission tests; the codegen runner never evaluates them and no counter records them
  unrepresentable_edges: edge values outside i64 are listed in a generated constant array by read, edge, and direction, never clamped or wrapped
  runner: subject-free oracle conformance; the oracle result maps by clause kind (Precondition false to RejectedPrecondition, Postcondition false to FailedPostcondition, Invariant false to FailedPostcondition with an Invariant observation, true to Passed); verdicts are built through runtime construct_verdict with the IR ExecutionPoint's serialized name and recorded through runtime record_campaign_verdict, so counters follow quire-contract-runtime FR-004; the runtime adapt_to_proptest operations are deliberately not used, and rejected preconditions stay counted only in rejected, never as successful evidence; a verdict equal to the tag's prediction returns a passing proptest result, otherwise ConformanceMismatch; never a global reject or explicit discard; the census runner evaluates each in-domain census case once in order without shrinking; a false invariant uses the failure detail the runtime provides for a contract clause until quire-contract-runtime specifies one; an identity mismatch from recording fails the campaign; FR-002 floors, ceiling, and Exhausted conclusions apply unchanged, and a proptest failure concludes ConformanceMismatch after the discard-ceiling check, never Failed
  rates: the bound-strategy summary alone exposes discard_rate() and rejection_rate(), each Some((numerator, attempted)), or None at zero attempted or when the report snapshot is at_limit (quire-contract-runtime FR-004); a public generated summary_from_snapshot operation exposes this calculation without inventing a resumable report or local counter format; the PR #22 harness summary is unchanged
  shrinking: partner values derive from the current primary value at every step; tags never change; Broad never shrinks across sides; shrink replays count in attempted
  consumer: case types expose one i64 field per read named by the generated oracle's dependency parameter identifier, plus constant IR declaration SymbolName and observation-name (current, pre, post; input declarations are always current) strings, where a quire-spec-language field projection's SymbolName is SL's deterministic field alias mapped back through SL FR-034 read correspondence; generated code depends only on proptest, quire-contract-runtime, and core/std; no serialized case, census, or summary format and no new schemas/ file; the generated header states the full ClauseRef
  out_of_scope: Boolean connectives over comparisons; Boolean literal, reference, and negation roots; a Compare of one read with itself; Current mixed with Pre or Post of one declaration; arithmetic, negation, and definedness obligations, which oracle admission refuses so no admitted clause has an overflow edge (owned by codegen#4 and the qcir/runtime undefined-result decision under quire-spec-language#83); a numeric subject harness where an operation produces the post-state; StatePinned and NoEvent campaigns, which stay on the caller-constraint generate_i64_strategy API
kani_slice:
  semantics_source: the executable-oracle analyzer and rendered predicates are reused exactly; the Kani adapter does not carry a second expression interpreter
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
  scope: running one FR-015 harness; FR-015 generation, and the FR-014 oracles it embeds, have no slice of their own yet and are governed by their requirements alone
  adapter: distinct from the FR-003 slice above; the obligation path fixes solver `cadical` and emits no stubbing option, so FR-003's caller-supplied solver is not carried into it
  refusals: a refusal when the crate's library source does not contain the harness source byte for byte, and a tool refusal naming a launcher that cannot be started and its path. Neither runs anything
  outcomes: `verified`, `falsified` with the concrete playback verbatim, `cover_unsatisfied` with satisfied and total counts, and `inconclusive` with one of `failed_without_counterexample`, `no_verdict`, `missing_cover_summary`, `unwind_bound_exhausted`, `timed_out`. Success is never defaulted: without a readable, fully satisfied cover summary a successful run is not `verified`. A run is given a caller-declared wall-clock budget on every request; one that has not concluded when the budget elapses is killed, along with every process it forked that a `/proc` walk taken at that moment can still see (one forked or reparented away in the instant before that walk is not guaranteed reached, only that the caller is never made to wait for it), and reported `inconclusive`/`timed_out` rather than left running
  evidence: the kind, harness path, launcher path, complete argument vector, unwind bound, solver, exit code and outcome. This repository retains none of it and computes no aggregate verdict
  outcome_source: the outcome is read from the backend's `--output-format regular` prose, because Kani publishes no machine-readable verdict; this crate reads that prose to decide a verdict only in `src/kani_transcript.rs`, which returns a typed transcript; a falsifying playback block is passed through verbatim to the FR-016 witness join (codegen#59)
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
| replay_codegen_counterexample | operation |
| generate_exact_scalar_oracles | operation |
| derive_exact_scalar_items | operation |
| negotiate_kani_obligations | operation |
| execute_kani_obligation | operation |
| kani_launch_command | operation |
| run_launcher_with_timeout | operation |
| launch_evidence | operation |
| classify_kani_run | operation |
| generate_composite_equality_oracles | operation |
| negotiate_backend_provider | operation |
| generate_routed | operation |
| generate_exact_function_oracles | operation |
| witness_schema | operation |
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
  (FR-008 through FR-013 for `bound_strategy_slice`, FR-003 for `kani_slice`, FR-017 for
  `kani_obligation_execution_slice`, and so on) and are backed there rather than restated here.

---
id: interface-001
title: "Contract code-generation API"
type: interface
---
# [interface-001] Contract code-generation API

## Contract

```yaml
name: ContractCodegen
version: draft-codegen-v1
input:
  contract_package: public IR BoundPackage from the pinned derived executable projection decoder; no private wire or codegen input schema
  configuration: backend versions, customer bindings, output profile
operations:
  - name: generate_bound_oracles
    inputs: [public BoundPackage reference, attestation context]
    output: BoundOracleGeneration | BoundGenerationError
    semantics: complete ordered executable-clause batch over the supported Boolean and obligation-free bounded-integer comparison grammar, or explicit NoExecutable with bound digest and informational references but no publishable artifact; unsupported executable content fails the entire batch
  - name: generate_bundle
    inputs: [contract package bytes, generation configuration]
    output: ArtifactBundle | DiagnosticSet
    semantics: planned multi-backend operation; the implemented first consumer is library-only generate_bound_oracles
  - name: generate_tristate_harness
    inputs: [typed precondition, typed postcondition, explicit bindings, minimum accepted cases, minimum rejected cases, maximum discarded cases, attestation context]
    output: GeneratedArtifactBundle | HarnessDiagnosticSet
    semantics: source plus one proof attestation, request-bound campaign policy, owned execution loop, retained campaign accounting
  - name: generate_i64_strategy
    inputs: [requirement identity, constraint, campaign, attestation context]
    output: GeneratedArtifactBundle | StrategyDiagnostic
    semantics: shaped cases whose expected domain is checked against runtime VerdictKind
  - name: generate_enum_strategy
    inputs: [requirement identity, customer enum path and variants, campaign, attestation context]
    output: GeneratedArtifactBundle | StrategyDiagnostic
    semantics: finite shaped cases with an explicit quire-contract-runtime consumer dependency
  - name: generate_bound_strategy
    status: implemented for the codegen#3 numeric/state slice on the codegen#4 numeric oracle grammar from PR #29
    inputs: [public BoundPackage, full ClauseRef, population Satisfying|Violating|Broad|Boundary, campaign policy, attestation context]
    output: GeneratedArtifactBundle | StrategyDiagnostic
    semantics: admits only clauses bound oracle generation admits, narrowed to one Compare; domains taken from the clause's IR IntegerType declarations; constructive Holds/Violated populations, an exhaustive in-domain census and an untagged out-of-domain array; a subject-free oracle-conformance runner whose summary reports discard and rejection rates; see bound_strategy_slice
  - name: generate_kani_bundle
    inputs: [typed precondition, typed postcondition, direct checked dependency types and observations, subject path, pinned backend identity, dependency census, attestation context]
    output: KaniArtifactBundle | KaniDiagnosticSet
    semantics: deterministic Boolean/i64 Kani source and v2 dependency/binding graph using exact oracle predicates and IR-owned model bounds; generation records proof execution as not_run and never claims proof completion
  - name: write_bundle_atomic
    inputs: [ArtifactBundle, destination directory]
    output: PublishedBundleIdentity | IO diagnostic
    semantics: replace only a destination whose complete contents match its local ownership marker after staged validation; the marker is a writable consistency declaration, not authenticated provenance; caller serializes destination writers; missing inputs refuse ownership while inspection/read failures return io_failed with unchanged state; failed rollback reports unknown and preserves backup/staging for recovery; post-commit cleanup failures report published; process crashes between directory renames and power-loss durability remain outside the portable rollback guarantee
  - name: analyze_coverage
    status: planned; public IR-owned bound population is available, native run-result contract and aggregate analysis integration remain pending
    inputs: [bound executable population, generated source and maps, LLVM coverage JSON bytes, native producer and run identities, source root, runtime campaign report, execution outcome, attestation context]
    output: versioned structured AnalysisOutcome including non-success diagnostics and available input identities
    semantics: coverage obligation succeeds only for nonempty complete exercised population with successful bound execution; successful serialization is not successful coverage; never executes LLVM
  - name: analyze_bound_coverage
    status: implemented phase A after coordinator approval of REV-017; full native-run analysis remains pending
    inputs: [public BoundPackage, immutable BoundOracleGeneration, complete artifact bytes, optional LLVM export bytes, source root]
    output: immutable versioned BoundCoverageAnalysis domain observations
    semantics: exact independent clause and implication census; whole-batch artifact/map binding; measured clauses or explicit unavailable states; valid informational-only population is no_executable; provenance remains unqualified even when all clauses are exercised; no runtime transport, producer execution, attestation, or assurance verdict
  - name: cli_generate
    status: planned; no executable CLI is provided by this library candidate
    inputs: [serialized package path, destination, backend flags]
    output: stable exit status, diagnostics, and published bundle identity
    semantics: equivalent to the library API and never edits developer-owned regions
artifact_bundle:
  scope: planned multi-backend generate_bundle; implemented bound-oracle batches contain only oracle source, source maps and their generation attestation bodies
  required:
    - executable Rust oracles
    - tri-state harnesses
    - shaped proptest strategies
    - Kani obligations and proof dependency graph
    - coverage source map and vacuity map
    - diagnostics and one proof attestation per generated artifact
diagnostics:
  bound_batch_errors: typed ResourceLimitExceeded, NameCollision(full ClauseRef), Clause(full ClauseRef plus existing lower-level diagnostics and exact rejected IR source spans for expression failures), or Bundle(existing publication diagnostic); no partial artifacts
  no_executable: separate successful non-artifact result for valid empty or informational-only populations, not a terminal-state or proof-attestation claim
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
identity_envelope:
  schema: Quoin's packaged ProofAttestationV1 (proof-attestation-v1.schema.json), read from `quoin change-assurance schema` and never copied here
  emitted_form: that schema without digest and without retained_output, which `quoin change-assurance seal-attestation` derives from the retained bytes and refuses from a caller
  required: [schema_version, record_type, attestation_id, record_digest, candidate_revision, proof_id, command, tool, environment, observed_at, result]
  results: [passed, failed, unavailable, not_computed]
  binding: one attestation per generated artifact, because an attestation binds exactly one retained output
  backend_rule: oracle, harness, and strategy lowering declare `--backend none`; Kani generation declares `--backend cargo-kani` together with exact version, executable digest, adapter profile, options, readiness, and `--proof-execution-state not_run`. These are enforced by tests because argv is a free-form string array
  observed_at: the generator's own source-commit time, frozen at build so that regeneration is byte-identical. It is not an observation of when generation ran, and a consumer generating months later emits an attestation whose observed_at predates the generation. Verification receipts derive staleness from candidate_revision, not from this field
  argv: a faithful rendering of an in-process call, not a runnable command line. The crate declares a library and no binary and cli_generate is unimplemented, so argv[0] names no program that exists. Recorded as UNKNOWN-attested-command-is-not-runnable rather than dressed up
  not_carried:
    - reviewer identity, which belongs to the ix-flow decision event a verification receipt binds, where the packaged receipt schema carries one recorded_actor rather than a list
    - contribution method, which has no field in any of the three packaged schemas and is dropped outright rather than rehomed
    - result summary and requirement references, which belong to the record's own proof obligations, as statement and obligation_ids
    - the crate's semantic version, superseded by tool.version's exact revision and still present in the generated Rust header
    - the input's role, media type and schema identity; the backend's free-text reason; the always-generated terminal state; the reviewer-role prose
    - for the harness and strategy slices only, the output schema's digest, because no schema document exists for the identifier they name
oracle_slice:
  attestations: one ProofAttestationV1 body per generated artifact, under proof obligations PROOF-codegen-generated-rust-oracle and PROOF-codegen-oracle-source-map
  attestation_context: caller supplies the sealed change-assurance record digest and the candidate revision, and nothing else
  provenance_rule: generator source identity, command, environment, time and result are observed by the crate; the consuming package's record and candidate binding are never hardcoded by the lowering core
  archive_build: exact archive revision/time may be supplied explicitly; absent Git/archive identity is marked unavailable and dirty rather than aborting compilation
  schemas: generated Rust and source-map outputs each identify and validate against their own versioned schema
  source_limit: 1048576 bytes per clause, enforced during rendering
  artifact_names: bounded readable prefix plus full SHA-256 package/requirement/revision/clause identity with per-clause source-map and per-artifact attestation paths
  supported_expression_grammar: Boolean literals, Boolean direct value references, Boolean not/operators, bounded i64 literals, bounded i64 direct input/state value references, and all six comparisons with a Boolean clause root
  dependency_types: Boolean dependencies render as bool; bounded-integer dependencies render as i64; current/pre/post observations remain distinct parameters
  undefined_result_boundary: a typed expression carrying any definedness obligation refuses before rendering; every numeric arithmetic and numeric-negation node remains unsupported even without an obligation until a versioned IR/runtime result can distinguish invalid from false; no checked result is unwrapped or defaulted into bool
  refusal_locus: NonBooleanRoot carries the clause-root SourceSpan; UnsupportedExpression and UnsupportedDependency carry the first rejected expression node's SourceSpan in authored preorder; UnsupportedObligations carries the first retained obligation's SourceSpan in IR order; none produces a partial artifact
harness_strategy_slice:
  output: generated Rust artifact plus one ProofAttestationV1 body, under proof obligations PROOF-codegen-generated-rust-harness and PROOF-codegen-generated-rust-strategy
  attestation_context: required for harness, integer-strategy, enum-strategy, and bound-strategy generation
  campaign_policy: minimum accepted, minimum rejected, and maximum explicit-discard invocation counts are caller-supplied, rendered once as generated constants, and bound into deterministic request identity; zero is the valid rejected floor for a declared total precondition
  campaign_execution: the public generated runner is the campaign-level entry point; it owns the proptest loop, creates observations, records every explicit discard, invokes the private verdict adapter, and always classifies retained accounting as passed, below an accepted/rejected floor, above the explicit-discard ceiling, exhausted, or failed
  accounting_unit: accepted, rejected, and failed count adapter invocations; discarded counts explicit discards that do not invoke the adapter; attempted equals accepted plus rejected plus discarded, including global-reject retries and shrink replays rather than distinct generated values, and supplies the exact denominator for rejected/attempted and discarded/attempted rates
  discard_boundary: a precondition rejection returned by the test closure is a proptest global reject recorded in rejected, while the generated discarded constructor is a separate explicit-discard channel recorded in discarded; the caller-owned max_global_rejects setting governs framework search exhaustion and the generated policy governs the retained invocation counters
  campaign_conclusion: policy applies to the complete supplied report including prior invocations/discards; completed searches enforce the discard ceiling then accepted/rejected floors; every framework Abort returns Exhausted with its reason, summary, and optional boxed policy failure, including all-rejected and zero-attempt aborted searches; a completed fresh zero-case campaign returns BelowAcceptedFloor, and explicit discards above the requested ceiling produce a distinct typed result
  expected_domain: generated integer cases expose executable accepted/rejected verdict checks; generated enum populations contain declared admissible members only and execute their admission expectation; generated Boolean campaign constructors bind accepted, rejected, or explicit-discarded disposition to the exact values consumed by the owned runner
  generated_crate_lints: generated crate roots deny missing documentation and compile under denied warnings
  source_limit: harness and strategy Rust are rejected above 1048576 bytes before bundling, matching the maximum-source-bytes value retained in ProofAttestationV1 command argv
  artifact_names: bounded readable prefix plus full SHA-256 over length-delimited request identity
bound_strategy_slice:
  requirements: [FR-008, FR-009, FR-010, FR-011, FR-012, FR-013, NFR-004]
  depends_on: FR-001-AC-8 bounded-integer oracle grammar (codegen#4, merged by PR #29 at e0be330); quire-contract-ir FR-012 through FR-015 and FR-023 at 04eb6f8; quire-contract-runtime FR-001, FR-003, FR-004 and interface-001 at 8a4d02b; quoin FR-064 and FR-068
  terms: the domain of a read is the inclusive minimum..=maximum of its integer declaration, not the IR IntegerType.domain representation field (quire-contract-ir FR-013); a read is one value-reference operand identified by declaration SymbolName and observation current, pre, or post (quire-contract-ir FR-014); the primary read is the left operand when it is a read, otherwise the right; the partner read is the other operand when both are reads
  admission: first the same admission generate_bound_oracles applies; then clause kind Precondition, Postcondition, or Invariant at any anchor the IR accepts; then a root of exactly one Compare whose operands are each an integer read or an IntegerLiteral, with at least one read, not the same read twice, and not Current mixed with Pre or Post of one declaration
  refusals: new variants of the existing StrategyErrorCode, checked in order UnknownClause (invalid-input), UnsupportedClause (preserves the oracle's code, terminal state, and span), UnsupportedClauseKind (unsupported), UnsupportedRelation (unsupported); then per population EmptyPopulation (unsupported) and UnsupportedCampaignConstraint (unsupported); each carries the full ClauseRef; UnsupportedClause carries the oracle diagnostic's span and UnsupportedRelation the first offending node's SourceSpan in authored preorder, per the codegen#4 refusal locus, while UnknownClause, UnsupportedClauseKind, EmptyPopulation, and UnsupportedCampaignConstraint carry no span; a literal-only Compare and a Boolean or Text comparison refuse as UnsupportedRelation; no partial bundle, artifact, or attestation; UnknownClause adds invalid-input for an unknown strategy ClauseRef to implemented_mapping, and UnsupportedClause keeps the oracle's terminal state, including invalid-input for a non-Boolean root or a batch NameCollision
  domains: inclusive IntegerType minimum/maximum; both operands of one Compare share one IntegerType; no caller-supplied range
  cases: a complete valuation of the clause's reads, including Post reads; the tag is the value the generated oracle returns for it and claims nothing about any other clause
  populations: Satisfying and Violating are constructed directly (interval draws and primary-dependent partner draws, 128-bit checked size and index arithmetic); Broad holds both with side-preserving tags and requires both sides non-empty; no filter, assume, reject, or discard
  census: Boundary is a generated constant in-domain census of primary edges min, min+1, max-1, max and literal edges k-1, k, k+1, each paired with partners v-1, v, v+1 in domain, tagged Holds or Violated, ordered by primary then partner, deduplicated, refused when single-tagged
  out_of_domain: every bundle generated for a non-refused population request carries a generated constant array of untagged out-of-domain edge cases for consumer domain-admission tests; the codegen runner never evaluates them and no counter records them
  unrepresentable_edges: edge values outside i64 are listed in a generated constant array by read, edge, and direction, never clamped or wrapped
  runner: subject-free oracle conformance; the oracle result maps by clause kind (Precondition false to RejectedPrecondition, Postcondition false to FailedPostcondition, Invariant false to FailedPostcondition with an Invariant observation, true to Passed); verdicts are built through runtime construct_verdict with the IR ExecutionPoint's serialized name and recorded through runtime record_campaign_verdict, so counters follow quire-contract-runtime FR-004; the runtime adapt_to_proptest operations are deliberately not used, and rejected preconditions stay counted only in rejected, never as successful evidence; a verdict equal to the tag's prediction returns a passing proptest result, otherwise ConformanceMismatch; never a global reject or explicit discard; the census runner evaluates each in-domain census case once in order without shrinking; a false invariant uses the failure detail the pinned runtime provides for a contract clause until quire-contract-runtime specifies one; an identity mismatch from recording fails the campaign; FR-002 floors, ceiling, and Exhausted conclusions apply unchanged, and a proptest failure concludes ConformanceMismatch after the discard-ceiling check, never Failed
  rates: the bound-strategy summary alone exposes discard_rate() and rejection_rate(), each Some((numerator, attempted)), or None at zero attempted or when the report snapshot is at_limit (quire-contract-runtime FR-004); a public generated summary_from_snapshot operation exposes this calculation without inventing a resumable report or local counter format; the PR #22 harness summary is unchanged
  shrinking: partner values derive from the current primary value at every step; tags never change; Broad never shrinks across sides; shrink replays count in attempted
  consumer: case types expose one i64 field per read named by the generated oracle's dependency parameter identifier, plus constant IR declaration SymbolName and observation-name (current, pre, post; input declarations are always current) strings, where a quire-spec-language field projection's SymbolName is SL's deterministic field alias mapped back through SL FR-034 read correspondence; generated code depends only on proptest, quire-contract-runtime, and core/std; no serialized case, census, or summary format and no new schemas/ file; ProofAttestationV1 body (quoin FR-064, sealed through quoin FR-068) under PROOF-codegen-generated-rust-strategy whose codegen-owned argv renders --requirement <requirement>@<revision> --clause <clause id> as bound oracle attestations do and binds the quire-contract-ir FR-023 BoundPackage digest through --input-digest; the generated header states the BoundPackage digest and full ClauseRef
  out_of_scope: Boolean connectives over comparisons; Boolean literal, reference, and negation roots; a Compare of one read with itself; Current mixed with Pre or Post of one declaration; arithmetic, negation, and definedness obligations, which oracle admission refuses so no admitted clause has an overflow edge (owned by codegen#4 and the qcir/runtime undefined-result decision under quire-spec-language#83); a numeric subject harness where an operation produces the post-state; StatePinned and NoEvent campaigns, which stay on the caller-constraint generate_i64_strategy API
kani_slice:
  adapter: exactly `cargo-kani 0.67.0` under profile `kani-0.67.0-function-contracts-v2`; another requested version is backend-unavailable
  semantics_source: the executable-oracle analyzer and rendered predicates are reused exactly; the Kani adapter does not carry a second expression interpreter
  argument_binding: unique direct current input, current state and pre-state dependencies become ordered subject arguments; order is normalized dependency identity, not source spelling or traversal accident
  result_binding: unique direct post-state dependencies become the subject result; zero uses `()`, one uses its primitive, and multiple use an ordered tuple
  binding_identity: uniqueness and ordering use the full checked DependencyIdentity of kind, normalized path and observation; repeated identical references unify, while incompatible declarations for one logical kind/path refuse
  primitive_types: Boolean maps to bool; bounded integer maps to i64 with the checked IntegerType domain, inclusive minimum, inclusive maximum and overflow policy retained
  bounds: every symbolic i64 argument receives an inclusive `kani::assume` from its checked IR model domain; post-state i64 results must satisfy the same domain in ensures; no caller range, proptest strategy, clamp or widened machine range substitutes
  compatibility: the existing one-Boolean-input plus one-Boolean-pre/post-state transition is the corresponding generalized ABI instance and retains its semantics
  outputs: generated Rust profile `quire.codegen.rust-kani/v2` plus schema-validated graph `quire.codegen.kani-proof-graph/v2`, each with its own Quoin ProofAttestationV1 body; v1 schema files remain historical and are not relabeled
  completion_boundary: graph readiness is derived from the full dependency census, but `proofExecutionState` is always `not_run`; artifact-generation attestations use output-specific proof obligations and do not attest that Kani proved the contract
  dependency_rule: missing or failed required edges yield incomplete; any assumption or stub yields conditional; only passed required edges yield ready
  source_sites: every proof assumption and stub has one digest-bound source marker and one graph edge; model-domain assumptions are separate typed binding records and never completed proof edges
  framing: generated contracts quantify only over copied primitive arguments and returned primitive values; they claim no unmodeled global, heap, alias, indirect, object or graph state
  subject_boundary: generation validates the customer subject as a Rust path and derives its required signature from checked bindings, but does not inspect or execute the external function; signature/link/body failures are external Rust or Kani observations and cannot become successful generation or proof claims
  options: exact adapter options are `-Z function-contracts`, optional `-Z stubbing`, `-Z concrete-playback`, exact fully qualified harness, `--exact`, explicit unwind, explicit solver, `--output-format regular`, and `--concrete-playback print`; graph and attestations retain the complete ordered vector
  refusal: UnsupportedBinding identifies post-state in a precondition, unsupported observations/dependency shapes, cross-clause type/domain conflicts and unrepresentable ABI; ClauseGenerationFailed retains the originating executable-oracle code and SourceSpan for definedness obligations or unsupported expressions; UnsupportedBackendVersion, InvalidIdentity, InvalidDependency, InvalidUnwind, InvalidAttestationContext, InvalidGeneratedSyntax, SerializationFailed and ResourceLimitExceeded remain distinct stable codes; every refusal returns no partial output
  replay_boundary: printed Kani concrete-playback data and the graph's typed binding order are sufficient inputs for the downstream SL IT-010 replay; runtime input construction and `runtime::execute` verdict ownership remain in quire-spec-language
kani_obligation_execution_slice:
  requirements: [FR-017]
  scope: running one FR-015 harness; FR-015 generation, and the FR-014 oracles it embeds, have no slice of their own yet and are governed by their requirements alone
  adapter: profile `kani-0.67.0-separate-obligations-v1`, distinct from the FR-003 `kani-0.67.0-function-contracts-v2` slice above; the obligation path fixes solver `cadical` and emits no stubbing option, so FR-003's caller-supplied solver is not carried into it
  pins: six measured fields — Kani version, `cargo-kani` launcher SHA-256, `kani-driver` SHA-256, CBMC version, the release's recorded Rust toolchain, and the host target triple from `rustc -vV`; the committed values are one installation's, and both the harness identity's pins and the measured pins must equal them before a process runs
  refusals: pin drift naming the first differing field with expected and observed values; a tool refusal naming an absent, unreadable, unsuccessful or unparsable backend component and its path; and a refusal when the crate's library source does not contain the harness source byte for byte. Every one of them runs nothing
  outcomes: `verified`, `falsified` with the concrete playback verbatim, `cover_unsatisfied` with satisfied and total counts, and `inconclusive` with one of `failed_without_counterexample`, `no_verdict`, `missing_cover_summary`, `unwind_bound_exhausted`. Success is never defaulted: without a readable, fully satisfied cover summary a successful run is not `verified`
  evidence: schema `quire.codegen.kani-execution/v1`, carrying the obligation identity digest, kind, harness path and source digest, the pins measured immediately before the run, the launcher path, the complete argument vector, the generated crate's `Cargo.lock` digest, the oracle digest, the runtime revision, the unwind bound, the solver, the exit code and the outcome. This repository retains none of it and computes no aggregate verdict; retention and attestation stay Quoin's
  known_defects: the run carries no wall-clock budget and has no timed-out state (codegen#58); the outcome is parsed from the backend's `--output-format regular` prose rather than a machine-readable form (codegen#59)
coverage_analysis_slice:
  qualified_profile: cargo-llvm-cov 0.9.0 with rustc 1.94.1 on x86_64-unknown-linux-gnu in test profile producing llvm.coverage.json.export version 3.0.1; primitives validate export metadata, not native executable provenance
  implementation_boundary: parse_llvm_coverage, LlvmCoverage.observe, and classify_clause remain unbound observation primitives; the separate bound observation API does not implement native-qualified aggregate analysis, campaign binding, coverage attestation, or obligation discharge
  bound_observation_boundary: analyze_bound_coverage emits the strict codegen.bound-coverage-observations/v1 domain schema from a complete immutable generated bundle plus independently validated BoundPackage; its provenance is always unqualified; no campaign binding, native execution, attestation, or obligation discharge
  bound_observation_output: complete ordered full-ClauseRef observations with independent typed implication census and available digests; computation state complete/incomplete/invalid_input/unsupported/no_executable; global refusal emits no clause observations and population not_emitted; null means unavailable, never measured zero; at most 16 MiB serialized bytes
  bound_source_root: at most 4096 UTF-8 bytes before normalization; the accepted canonical absolute root is retained in the report as the mapping configuration; root aliases use existing LLVM parser normalization, not filesystem resolution
  bound_count_consistency: within exact loop-free generated Boolean source, every measured consequent count is at most its owning oracle evaluation count; greater counts are retained with inconsistent_observation and no classification, never upgraded to exercised; the unbound primitive remains unchanged
  expected_population: immutable IR-owned executable clauses with typed expression, clause kind, execution anchor, declarations, dependencies, spans and canonical digest; exact artifact and map population equality; empty executable population is not_computed
  source_map: exactly one clause envelope carrying expectedConsequents counted independently from typed IR, one oracle_evaluation entry probe, and every implication_consequent entry probe; recompute artifact digests before aggregate use
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
  campaign_report: the analyzer consumes the pinned quire-contract-runtime CampaignReport type so its complete counter set and failed-is-a-subset-of-accepted invariant are not restated by a caller-owned lookalike
  test_outcomes: [passed, failed, aborted, not_computed]
  independent_facts: accepted, rejected, failed, and discarded runtime counts plus test outcome are retained verbatim and never used to upgrade coverage classification
  accounting: failed postconditions prohibit passed execution; positive oracle coverage with zero invocations is inconsistent when the generated campaign is declared the only execution source
  identity: native result binds command, executable, source, binary, profiles, toolchain, target, optimization profile, campaign, candidate and bound-population digest; JSON producer version and manifest are cross-checks, not executable provenance
  revision: compare IR and source-map u64 revision to runtime RevisionId using the canonical decimal string without leading zeroes
  invalid_input: stable non-success analysis outcome retains available identities and diagnostics without fabricated measured classifications or a passed coverage attestation
  default_gate: every executable clause exercised and native execution passed; adverse or unavailable coverage cannot discharge obligation; no automated human sufficiency or exception decision
compatibility:
  draft_pins: must be reconciled before leaving draft
  generated_runtime_dependency: quire-contract-runtime, proptest, plus declared customer types only
  licensing: crate AGPL-3.0-or-later; emitted Rust carries the MIT OR Apache-2.0 SPDX identity required by NFR-002
  publication: disabled through the human v0.1 source-release decision
open_design_gates:
  native_campaign_transport: pinned runtime CampaignReport has no validated native-process snapshot decoder; no Display parsing, fake verdict replay, or private counter lookalike may substitute
  native_run_authentication: producer result digests establish consistency only; Quoin-owned authorized producer and expected record/candidate/run verification must precede qualification; no caller verified flag
  serialized_package_cli: the pinned public IR derived-projection decoder now supplies BoundPackage; cli_generate remains unimplemented, and normal projection production remains the authoritative frontend/model lane rather than a codegen-owned authored sidecar
```

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| interface-001-AC-1 | Every `operations` entry this contract declares without a `status: planned` caveat is exported as a public function of `quire_contract_codegen` under the exact name given here: `generate_bound_oracles`, `generate_tristate_harness`, `generate_i64_strategy`, `generate_enum_strategy`, `generate_bound_strategy`, `generate_kani_bundle`, `write_bundle_atomic`, `analyze_bound_coverage`. | Test |
| interface-001-AC-2 | Every `operations` entry this contract marks `status: planned` — `generate_bundle`, `analyze_coverage`, `cli_generate` — is absent from the public API, so an implementation cannot silently outrun the status this contract declares for it. | Test |
| interface-001-AC-3 | `identity_envelope.required` names exactly the fields of `ProofAttestationBody`, and `identity_envelope.results` names exactly the four `AttestationResult` variants, so the envelope this contract describes is the envelope the generator emits. | Test |
| interface-001-AC-4 | `diagnostics.terminal_states` names exactly the six `GenerationTerminalState` variants, and no seventh state exists for `implemented_mapping` to omit. | Test |
| interface-001-AC-5 | `kani_obligation_execution_slice.pins` names exactly the six measured fields of `KaniToolPins`. | Test |

## Open items

- `generate_bundle`, `analyze_coverage` and `cli_generate` are declared `status: planned` and have
  no acceptance criteria beyond interface-001-AC-2's absence check: a criterion asserting behavior
  for an operation this contract itself says is not implemented would be written to be satisfied by
  nothing. Criteria for their real semantics belong with the requirement that implements them, once
  one exists.
- The remaining prose fields this contract's slices carry — admission order, refusal vocabulary,
  domain and campaign rules, and so on — are the executable half of the FR that owns each slice
  (FR-008 through FR-013 for `bound_strategy_slice`, FR-003 for `kani_slice`, FR-017 for
  `kani_obligation_execution_slice`, and so on) and are backed there rather than restated here.

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
  contract_package: pinned serialized quire-contract-ir package
  configuration: backend versions, customer bindings, output profile
operations:
  - name: generate_bundle
    inputs: [contract package bytes, generation configuration]
    output: ArtifactBundle | DiagnosticSet
    semantics: deterministic, all-or-nothing lowering; unsupported semantics prevent false completeness
  - name: generate_tristate_harness
    inputs: [typed precondition, typed postcondition, explicit bindings, minimum accepted cases, maximum discarded cases, attestation context]
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
  - name: generate_kani_bundle
    inputs: [typed precondition, typed postcondition, subject path, pinned backend identity, dependency census, attestation context]
    output: KaniArtifactBundle | KaniDiagnosticSet
    semantics: deterministic Kani source and dependency graph; generation records proof execution as not_run and never claims proof completion
  - name: write_bundle_atomic
    inputs: [ArtifactBundle, destination directory]
    output: PublishedBundleIdentity | IO diagnostic
    semantics: replace only a destination whose complete contents match its local ownership marker after staged validation; the marker is a writable consistency declaration, not authenticated provenance; caller serializes destination writers; missing inputs refuse ownership while inspection/read failures return io_failed with unchanged state; failed rollback reports unknown and preserves backup/staging for recovery; post-commit cleanup failures report published; process crashes between directory renames and power-loss durability remain outside the portable rollback guarantee
  - name: analyze_coverage
    status: planned; awaits public IR-owned bound executable population and native run-result contract
    inputs: [bound executable population, generated source and maps, LLVM coverage JSON bytes, native producer and run identities, source root, runtime campaign report, execution outcome, attestation context]
    output: versioned structured AnalysisOutcome including non-success diagnostics and available input identities
    semantics: coverage obligation succeeds only for nonempty complete exercised population with successful bound execution; successful serialization is not successful coverage; never executes LLVM
  - name: cli_generate
    inputs: [serialized package path, destination, backend flags]
    output: stable exit status, diagnostics, and published bundle identity
    semantics: equivalent to the library API and never edits developer-owned regions
artifact_bundle:
  required:
    - executable Rust oracles
    - tri-state harnesses
    - shaped proptest strategies
    - Kani obligations and proof dependency graph
    - coverage source map and vacuity map
    - diagnostics and one proof attestation per generated artifact
diagnostics:
  terminal_states: [generated, unsupported, invalid-input, backend-unavailable, io-failed, inconclusive]
  implemented_mapping:
    generated: successful supported Boolean lowering only
    unsupported: unsupported expression, dependency, obligation, or bounded resource
    invalid-input: non-Boolean root or generated-name collision
    inconclusive: internal syntax or serialization control failure
    backend-unavailable: reserved for external backends
    io-failed: reserved for atomic publication
  rule: no non-generated state may be converted into a complete artifact claim
  fields: [stable code, terminal state, stable input path, optional preserved lower-level generation code, human detail]
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
  artifact_names: bounded readable prefix plus full SHA-256 requirement/revision/clause identity with per-clause source-map and per-artifact attestation paths
harness_strategy_slice:
  output: generated Rust artifact plus one ProofAttestationV1 body, under proof obligations PROOF-codegen-generated-rust-harness and PROOF-codegen-generated-rust-strategy
  attestation_context: required for harness, integer-strategy, and enum-strategy generation
  campaign_policy: minimum accepted, minimum rejected, and maximum explicit-discard invocation counts are caller-supplied, rendered once as generated constants, and bound into deterministic request identity; zero is the valid rejected floor for a declared total precondition
  campaign_execution: the public generated runner is the campaign-level entry point; it owns the proptest loop, creates observations, records every explicit discard, invokes the private verdict adapter, and always classifies retained accounting as passed, below an accepted/rejected floor, above the explicit-discard ceiling, exhausted, or failed
  accounting_unit: accepted, rejected, and failed count adapter invocations; discarded counts explicit discards that do not invoke the adapter; attempted equals accepted plus rejected plus discarded, including global-reject retries and shrink replays rather than distinct generated values, and supplies the exact denominator for rejected/attempted and discarded/attempted rates
  discard_boundary: a precondition rejection returned by the test closure is a proptest global reject recorded in rejected, while the generated discarded constructor is a separate explicit-discard channel recorded in discarded; the caller-owned max_global_rejects setting governs framework search exhaustion and the generated policy governs the retained invocation counters
  campaign_conclusion: policy applies to the complete supplied report including prior invocations/discards; completed searches enforce the discard ceiling then accepted/rejected floors; every framework Abort returns Exhausted with its reason, summary, and optional boxed policy failure, including all-rejected and zero-attempt aborted searches; a completed fresh zero-case campaign returns BelowAcceptedFloor, and explicit discards above the requested ceiling produce a distinct typed result
  expected_domain: generated integer cases expose executable accepted/rejected verdict checks; generated enum populations contain declared admissible members only and execute their admission expectation; generated Boolean campaign constructors bind accepted, rejected, or explicit-discarded disposition to the exact values consumed by the owned runner
  generated_crate_lints: generated crate roots deny missing documentation and compile under denied warnings
  source_limit: harness and strategy Rust are rejected above 1048576 bytes before bundling, matching the maximum-source-bytes value retained in ProofAttestationV1 command argv
  artifact_names: bounded readable prefix plus full SHA-256 over length-delimited request identity
kani_slice:
  adapter: exactly `cargo-kani 0.67.0` under profile `kani-0.67.0-function-contracts-v1`; another requested version is backend-unavailable
  binding: one Boolean input, one Boolean pre/post state, and an explicit customer `fn(bool, bool) -> bool` path
  outputs: generated Rust plus a schema-validated proof-dependency graph, each with its own Quoin ProofAttestationV1 body
  completion_boundary: graph readiness is derived from the full dependency census, but `proofExecutionState` is always `not_run`; artifact-generation attestations use output-specific proof obligations and do not attest that Kani proved the contract
  dependency_rule: missing or failed required edges yield incomplete; any assumption or stub yields conditional; only passed required edges yield ready
  source_sites: every assumption and stub has one digest-bound source marker and one graph edge
coverage_analysis_slice:
  qualified_profile: cargo-llvm-cov 0.9.0 with rustc 1.94.1 on x86_64-unknown-linux-gnu in test profile producing llvm.coverage.json.export version 3.0.1; primitives validate export metadata, not native executable provenance
  implementation_boundary: parse_llvm_coverage, LlvmCoverage.observe, and classify_clause are unbound observation primitives; no aggregate report, campaign binding, coverage attestation, or obligation discharge is implemented
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
  licensing: MIT OR Apache-2.0
  publication: disabled through the human v0.1 source-release decision
open_design_gates:
  serialized_package_cli: the accepted ContractPackage wire format binds ReferenceBody metadata but no executable TypedExpression, and the IR wire decoder is private; cli_generate cannot truthfully lower serialized packages until IR owns that binding and decoding contract
```

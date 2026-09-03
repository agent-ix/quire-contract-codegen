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
    inputs: [typed precondition, typed postcondition, explicit bindings, attestation context]
    output: GeneratedArtifactBundle | HarnessDiagnosticSet
    semantics: source plus one proof attestation, accepted-case floor, retained campaign accounting
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
    semantics: replace only generator-owned bundle boundaries after complete staged validation
  - name: analyze_coverage
    inputs: [source map, LLVM coverage export, runtime campaign report]
    output: per-requirement vacuity and rejection report
    semantics: exercised coverage requires observed consequent execution
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
  campaign_conclusion: reads accepted, rejected, failed, and discarded counters and requires at least one accepted case
  expected_domain: generated integer cases expose a rejection expectation and verdict check; the generated harness proptest adapter requires and checks that expectation against quire_contract_runtime::VerdictKind
  generated_crate_lints: generated crate roots deny missing documentation and compile under denied warnings
  artifact_names: bounded readable prefix plus full SHA-256 over length-delimited request identity
kani_slice:
  adapter: exactly `cargo-kani 0.67.0` under profile `kani-0.67.0-function-contracts-v1`; another requested version is backend-unavailable
  binding: one Boolean input, one Boolean pre/post state, and an explicit customer `fn(bool, bool) -> bool` path
  outputs: generated Rust plus a schema-validated proof-dependency graph, each with its own Quoin ProofAttestationV1 body
  completion_boundary: graph readiness is derived from the full dependency census, but `proofExecutionState` is always `not_run`; artifact-generation attestations use output-specific proof obligations and do not attest that Kani proved the contract
  dependency_rule: missing or failed required edges yield incomplete; any assumption or stub yields conditional; only passed required edges yield ready
  source_sites: every assumption and stub has one digest-bound source marker and one graph edge
compatibility:
  draft_pins: must be reconciled before leaving draft
  generated_runtime_dependency: quire-contract-runtime, proptest, plus declared customer types only
  licensing: MIT OR Apache-2.0
  publication: disabled through the human v0.1 source-release decision
```

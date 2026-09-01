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
    - diagnostics and derivation manifest
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
identity_envelope:
  schema: quire.derivation-evidence/v1
  required: [producer, inputs, backend, outputs, parameters digest, environment, provenance, result]
  terminal_states: [conclusive, inconclusive, unsupported, rejected, timed-out, pending, error]
  rule: omitted backend identity is invalid; in-process lowering uses kind none with a reason
oracle_slice:
  manifest: validates as quire.derivation-evidence/v1 and identifies producer source plus lowering digest
  manifest_context: caller supplies candidate revision, contribution method, reviewers, bounded result status and summary, and supported requirement refs
  provenance_rule: generator source identity is recorded by the crate; consuming-package review and result claims are never hardcoded by the lowering core
  archive_build: exact archive revision/time may be supplied explicitly; absent Git/archive identity is marked unavailable and dirty rather than aborting compilation
  schemas: generated Rust and source-map outputs each identify and validate against their own versioned schema
  source_limit: 1048576 bytes per clause, enforced during rendering
  artifact_names: bounded readable prefix plus full SHA-256 requirement/revision/clause identity with per-clause source-map and manifest paths
compatibility:
  draft_pins: must be reconciled before leaving draft
  generated_runtime_dependency: quire-contract-runtime plus declared customer types only
  licensing: MIT OR Apache-2.0
  publication: disabled through the human v0.1 source-release decision
```

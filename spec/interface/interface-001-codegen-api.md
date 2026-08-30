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
  rule: no non-generated state may be converted into a complete artifact claim
identity_envelope:
  required: [requirement, revision, input digest, schema, tool, backend, configuration, output digest]
compatibility:
  draft_pins: must be reconciled before leaving draft
  generated_runtime_dependency: quire-contract-runtime plus declared customer types only
  licensing: MIT OR Apache-2.0
  publication: disabled through the human v0.1 source-release decision
```

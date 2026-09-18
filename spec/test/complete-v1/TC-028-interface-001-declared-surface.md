---
id: TC-028
title: "Verify interface-001's declared API surface and identity envelope match the generator"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: verifies
---
# TC-028: Verify interface-001's declared API surface and identity envelope match the generator

## Description

Verify that `spec/interface/interface-001-codegen-api.md` describes the crate as it actually is:
every operation it declares implemented is exported under the name it gives, every operation it
declares `status: planned` is absent, the identity envelope it describes is the attestation body
the generator emits, the terminal-state vocabulary it declares is complete, and the Kani obligation
pin vocabulary it declares is complete.

## Test Procedure

Reference each of `generate_bound_oracles`, `generate_tristate_harness`, `generate_i64_strategy`,
`generate_enum_strategy`, `generate_bound_strategy`, `generate_kani_bundle` and
`write_bundle_atomic` by its declared name, and confirm `analyze_bound_coverage` is named in
`src/lib.rs`. Confirm `src/lib.rs` names none of `generate_bundle`, `analyze_coverage` or
`cli_generate`. Serialize a complete `ProofAttestationBody` and compare its top-level field names
with `identity_envelope.required`. Match every `AttestationResult` variant against
`identity_envelope.results`. Match every `GenerationTerminalState` variant against
`diagnostics.terminal_states`. Serialize a `KaniToolPins` value and compare its field names with
`kani_obligation_execution_slice.pins`.

## Expected Results

Every implemented operation compiles by its declared name; none of the three planned operations
appears in `src/lib.rs`; the attestation body's eleven top-level fields are exactly
`identity_envelope.required`; the four `AttestationResult` variants are exactly
`identity_envelope.results`; the six `GenerationTerminalState` variants are exactly
`diagnostics.terminal_states`; and `KaniToolPins`'s six fields are exactly
`kani_obligation_execution_slice.pins`. Each `match` used to enumerate a Rust enum is exhaustive, so
an added variant this test does not name fails the build rather than passing silently.

## Implementation

`tests/interface_001.rs`.

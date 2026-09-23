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

Parse `src/lib.rs` and collect the function names its `pub use` items re-export. Compare that set
with the contract's non-planned `operations` entries in both directions, and confirm none of the
`status: planned` entries is exported. Serialize a complete `ProofAttestationBody` and compare its top-level field names
with `identity_envelope.required`. Match every `AttestationResult` variant against
`identity_envelope.results`. Match every `GenerationTerminalState` variant against
`diagnostics.terminal_states`. Serialize a `KaniToolPins` value and compare its field names with
`kani_obligation_execution_slice.pins`.

## Expected Results

The exported function set equals the declared non-planned operations exactly, so a new export
with no contract entry and a declared entry the crate no longer exports each fail; none of the
three planned operations is exported; the attestation body's eleven top-level fields are exactly
`identity_envelope.required`; the four `AttestationResult` variants are exactly
`identity_envelope.results`; the six `GenerationTerminalState` variants are exactly
`diagnostics.terminal_states`; and `KaniToolPins`'s six fields are exactly
`kani_obligation_execution_slice.pins`. Each `match` used to enumerate a Rust enum is exhaustive, so
an added variant this test does not name fails the build rather than passing silently.

## Implementation

`tests/interface_001.rs`.

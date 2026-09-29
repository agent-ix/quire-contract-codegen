---
id: CAC-001
title: Codegen component assurance contract
type: ComponentAssuranceContract
status: proposed
owner: codegen-maintainers
kind: deterministic
responsibility: derive semantically aligned reproducible verification artifacts from one contract package
inputs: [serialized contract package, backend configuration, customer type bindings]
outputs: [artifact bundle, diagnostics]
invariants: [no silent approximation, one shared clause semantics, atomic publication]
failure_behaviors: [emit explicit diagnostics, retain incomplete states, publish no partial bundle]
version_pins:
  rust-msrv: "1.98.1"
controls:
  surfaces: [library API, CLI, backend adapters, bundle validator, CI]
  fallback: emit no backend artifact and retain an explicit diagnostic
  abstention: classify unsupported failed unavailable or inconclusive without completeness
  escalation: human release owner reviews unresolved gaps and dependency changes
isolation: no dependency on Quoin Quire or engineering-assurance repositories
replacement: preserve input output diagnostics atomicity and semantic parity contracts
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AP-001
    type: references
---
# Codegen component assurance contract

## Component Boundary

The component owns deterministic lowering, backend adaptation, bundle validation/publication, and
derivation evidence. It does not own canonical IR semantics, runtime verdict semantics, external
engines, customer code, accreditation, or release decisions.

## Required Behavior

Every supported clause is lowered from one shared semantic plan. Outputs retain requirement/revision
identity and backend/source-map relationships. Repeated generation is byte-identical. Proof
dependencies and vacuity/rejection/discard evidence remain complete and atomic publication never
touches developer-owned regions.

## Failure Handling

Invalid or unsupported input, backend incompatibility, unavailable tools, I/O failure, proof gaps, and
differential discrepancies produce explicit non-success diagnostics. No such state publishes or
counts as a complete artifact bundle.

## Controls

Requirement-tagged tests, differential corpora, regeneration determinism checks, backend parity,
fault injection, cargo-deny, unsafe/panic audits, local gates, and human review constrain the generator.

## Replacement

A replacement must consume the same versioned package, pass the same corpus and parity vectors,
preserve every requirement identity and non-success state, meet atomic/reproducible output contracts, and receive a
new human decision.

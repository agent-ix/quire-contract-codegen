---
id: CAC-001
title: Codegen component assurance contract
type: ComponentAssuranceContract
status: proposed
owner: codegen-maintainers
kind: deterministic
responsibility: derive semantically aligned reproducible verification artifacts from one contract package
inputs: [serialized contract package, backend configuration, customer type bindings]
outputs: [artifact bundle, diagnostics, derivation manifest]
invariants: [no silent approximation, one shared clause semantics, complete identity, atomic publication]
failure_behaviors: [emit explicit diagnostics, retain incomplete states, publish no partial bundle]
version_pins:
  rust-msrv: "1.75"
  governance: agent-ix/quire-contract-ir@7dac9d8c19952412b56a0347387666e2ca81e01d
  ir-corpus: agent-ix/quire-contract-ir@37eb00153d5c139ebc01622b6e12a4ab79256f88
  runtime: agent-ix/quire-contract-runtime@e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3
controls:
  surfaces: [library API, CLI, backend adapters, bundle validator, CI, retained evidence]
  fallback: emit no backend artifact and retain an explicit diagnostic
  abstention: classify unsupported failed unavailable or inconclusive without completeness
  escalation: human release owner reviews unresolved gaps and dependency changes
isolation: no dependency on Quoin Quire or engineering-assurance repositories
replacement: preserve input output diagnostics identity atomicity and semantic parity contracts
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

Pinned compatibility checks, requirement-tagged tests, golden/differential corpora, backend parity,
fault injection, cargo-deny, unsafe/panic audits, reproducibility measurements, protected CI, and
human review constrain the generator.

## Replacement

A replacement must consume the same versioned package, pass the same corpus and parity vectors,
preserve every identity and non-success state, meet atomic/reproducible output contracts, and receive a
new human decision.

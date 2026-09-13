---
type: master-requirements
name: quire-contract-codegen
org: agent-ix
component_type: rust-library
implementation_language: rust
tags: [contract-codegen, rust, proptest, kani, assurance]
depends_on:
  - ix://agent-ix/quire-contract-ir/PGM-01
  - ix://agent-ix/quire-contract-ir/issues/10
  - ix://agent-ix/quire-contract-runtime/issues/3
standards_alignment: [iso-iec-ieee-29148]
relationships:
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: depends_on
    cardinality: "1:1"
  - target: ix://agent-ix/quire-contract-ir/issues/10
    type: depends_on
    cardinality: "1:1"
  - target: ix://agent-ix/quire-contract-runtime/issues/3
    type: depends_on
    cardinality: "1:1"
security_critical: false
---
# Master Requirements Specification

## Purpose

This specification defines deterministic lowering from a validated contract package into Rust
oracles, tri-state test harnesses, shaped proptest strategies, Kani obligations, coverage maps, and
derivation evidence. Generated artifacts remain traceable to one authoritative source contract.

## Scope

### In Scope

- Library-first and CLI-driven deterministic generation.
- Executable, property-test, proof, vacuity, source-map, and derivation outputs.
- Explicit diagnostics for unsupported or unproved constructs.
- Golden, differential, and cross-backend semantic conformance.

### Out of Scope

- A Rust compiler, property-testing framework, proof engine, or coverage engine.
- Contract parsing or canonicalization owned by `quire-contract-ir`.
- Quoin or Quire integration and project-specific certification or accreditation.

## System Overview

### System Description

The crate consumes a versioned serialized contract package and emits a deterministic artifact bundle.
It uses a bounded deterministic renderer plus `syn` validation for Rust syntax, depends generated customer code only on
`quire-contract-runtime` and declared customer types, adapts to proptest and Kani, and consumes LLVM
coverage exports rather than implementing those engines.

### Intended Users

Assurance engineers generate reproducible verification artifacts. Developers compile and execute the
outputs. Reviewers inspect proof attestations, diagnostics, proof dependencies, coverage evidence,
and semantic-parity results. A human release owner alone decides source release suitability.

## Requirements Architecture

StR-001 is refined by FR-001 through FR-005 and constrained by NFR-001 and NFR-002.
FR-006 adopts the shared assurance intake contract for this repository's own verification
results and is constrained by NFR-002.
`interface-001` defines the serialized input, library/CLI operation, artifact bundle, diagnostics, and
evidence contract. TC-001 through TC-007 form the initial verification matrix. Assurance artifacts
bind the intended use, trusted boundary, risks, measurement policy, and open human decision.

FR-008 through FR-013 refine FR-002 for bound numeric and state-scalar clauses
(agent-ix/quire-contract-codegen#3 under agent-ix/quire-spec-language#83), admit only clauses that the
FR-001 bounded-integer oracle grammar of agent-ix/quire-contract-codegen#4 admits, and are constrained
by NFR-004. TC-017 through TC-022 verify them. Numbers skip FR-007, NFR-003, and TC-014 through TC-016
because a parked expansion branch allocated them.

### Subsystem layout

Specification files are grouped by crate subsystem. Identifiers stay flat and globally sequential;
the directory carries the subsystem.

| Subsystem | Source | Directory | Artifacts |
|---|---|---|---|
| Strategies and harness campaigns | `src/strategy.rs`, `src/harness.rs` | `functional/strategies/`, `nonfunctional/strategies/`, `test/strategies/` | FR-008–FR-013, NFR-004, TC-017–TC-022 |

The earlier flat artifacts move into subsystem directories after the remaining in-flight codegen#2
branch lands, so its edits do not conflict with the move:

- oracles: FR-001, TC-001, TC-003
- strategies: FR-002, TC-004
- kani: FR-003, TC-005, TC-007
- vacuity and coverage: FR-004, TC-006
- publication and CLI: FR-005, TC-002
- assurance intake: FR-006, TC-008–TC-010, TC-012, TC-013
- cross-cutting: NFR-001, NFR-002, StR-001

## References

- [Program umbrella](https://github.com/agent-ix/quire-contract-ir/issues/1).
- [PGM-01 governance gate](https://github.com/agent-ix/quire-contract-ir/issues/3), identified as
  `ix://agent-ix/quire-contract-ir/PGM-01`; this specification does not redefine it.
- [IR schema and corpus gate](https://github.com/agent-ix/quire-contract-ir/issues/10).
- [Runtime helper gate](https://github.com/agent-ix/quire-contract-runtime/issues/3).
- [Codegen epic](https://github.com/agent-ix/quire-contract-codegen/issues/7).

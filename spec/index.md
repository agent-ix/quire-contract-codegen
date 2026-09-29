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
- Differential and cross-backend semantic conformance, with determinism checked by regeneration.

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
outputs. Reviewers inspect diagnostics, proof dependencies, coverage evidence and semantic-parity
results. A human release owner alone decides source release suitability.

## Requirements Architecture

StR-001 is the one stakeholder requirement. Every functional requirement satisfies it. NFR-001 and
NFR-002 constrain it, and NFR-004 constrains FR-008 to FR-013.
`interface-001` defines the serialized input, the library and CLI operations, the artifact bundle,
the diagnostics and the evidence contract. AD-001 describes the architecture and its seams to
Contract IR, Contract Runtime and QSL. ADR-001 to ADR-004 hold the questions the owner has not ruled
on. `test-matrix.md` maps every criterion to its test case.

| Area | Requirements | Test cases |
|---|---|---|
| V1 oracles, strategies and Kani lowering | FR-001 to FR-005, FR-007 | TC-001 to TC-007, TC-014, TC-023 |
| Bound numeric and state strategies | FR-008 to FR-013, NFR-004 | TC-017 to TC-022 |
| Complete-V1 oracles | FR-014 scalar, FR-018 composite equality, FR-021 function application | TC-024, TC-029, TC-031 |
| Kani obligations and execution | FR-015 generation, FR-017 execution, FR-025 subject ABI | TC-025, TC-027, TC-036 |
| Counterexample replay | FR-016 witness decode and native replay, FR-024 submission in QSL's counterexample envelope | TC-026, TC-035 |
| Capability settlement and routing | FR-019 settlement, FR-022 routed generation | TC-030, TC-033 |

FR-020 is reserved for temporal and protocol oracles and is unwritten. FR-018 and FR-014 refuse the
model graph, relation, temporal and protocol families with typed blockers. Function application is
FR-021's. FR-006, FR-023, NFR-003, TC-008 to TC-013, TC-015, TC-016, TC-028, TC-032 and TC-034
are unassigned.

### Subsystem layout

Specification files are grouped by crate subsystem. Identifiers stay flat and globally sequential;
the directory carries the subsystem.

| Subsystem | Directory | Artifacts |
|---|---|---|
| Strategies and harness campaigns | `functional/strategies/`, `nonfunctional/strategies/`, `test/strategies/` | FR-008 to FR-013, NFR-004, TC-017 to TC-022 |
| Complete-V1 generation, execution and replay | `functional/complete-v1/`, `test/complete-v1/` | FR-014 to FR-019, FR-021, FR-022, FR-024, FR-025, TC-024 to TC-027, TC-029 to TC-031, TC-033, TC-035, TC-036 |
| Architecture decisions | `decisions/` | ADR-001 to ADR-004 |
| Everything else | the flat `functional/`, `nonfunctional/`, `test/`, `stakeholder/` directories | FR-001 to FR-005, FR-007, NFR-001, NFR-002, StR-001, the remaining TCs |

## References

- [Program umbrella](https://github.com/agent-ix/quire-contract-ir/issues/1).
- [PGM-01 governance gate](https://github.com/agent-ix/quire-contract-ir/issues/3), identified as
  `ix://agent-ix/quire-contract-ir/PGM-01`; this specification does not redefine it.
- [IR schema and corpus gate](https://github.com/agent-ix/quire-contract-ir/issues/10).
- [Runtime helper gate](https://github.com/agent-ix/quire-contract-runtime/issues/3).
- [Codegen epic](https://github.com/agent-ix/quire-contract-codegen/issues/7).

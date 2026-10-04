---
type: master-requirements
name: quire-contract-codegen
org: agent-ix
component_type: rust-library
implementation_language: rust
tags: [contract-codegen, rust, proptest, kani, assurance]
standards_alignment: [iso-iec-ieee-29148]
security_critical: false
---
# Master Requirements Specification

## Purpose

This specification defines deterministic lowering from a validated contract package into Rust
oracles, tri-state test harnesses, shaped proptest strategies, Kani obligations and coverage maps. Generated artifacts remain traceable to one authoritative source contract.

## Scope

### In Scope

- Library-first and CLI-driven deterministic generation.
- Executable, property-test, proof, vacuity and source-map outputs.
- Explicit diagnostics for unsupported or unproved constructs.
- Differential and cross-backend semantic conformance, with determinism checked by regeneration.

### Out of Scope

- A Rust compiler, property-testing framework, proof engine, or coverage engine.
- Contract parsing or canonicalization owned by `quire-contract-ir`.
- Project-specific certification or accreditation.

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
NFR-002 constrain it, NFR-004 constrains FR-008 to FR-013, and NFR-005 constrains the panic-free
generation of FR-014, FR-015, FR-019, FR-021 and FR-022. NFR-006 constrains FR-015, FR-017 and
FR-031 by gating a change that can alter what real Kani proves on the real-Kani lane.
`interface-001` defines the serialized input, the library and CLI operations, the artifact bundle,
the diagnostics and the evidence contract. AD-001 describes the architecture and its seams to
Contract IR, Contract Runtime and QSL. ADR-001 to ADR-004 record the owner's decisions on the
generators, the input model, the backend adapter, Kani tractability and the generated subject ABI.
[tests.md](tests.md) indexes the per-subsystem matrices that map every criterion to its test case.

| Area | Requirements | Test cases |
|---|---|---|
| Tri-state harnesses, vacuity and publication | FR-002, FR-004, FR-005 | TC-001 to TC-004, TC-006, TC-007 |
| No panic on a generation or analysis path | NFR-005 | TC-042 |
| Bound numeric and state strategies | FR-008 to FR-013, NFR-004 | TC-017 to TC-022 |
| Complete-V1 oracles | FR-014 scalar, FR-018 composite equality, FR-021 function application | TC-024, TC-029, TC-031 |
| Boolean oracle integer arithmetic and comparison | FR-031 | TC-044 |
| Kani obligations and execution | FR-015 generation, FR-017 execution, FR-025 subject ABI, FR-028 bounds and ceilings | TC-025, TC-027, TC-036, TC-039 |
| Real-Kani lane gating | NFR-006 | TC-045 |
| Backend adapter (routed) | FR-026 adapter trait and registration | TC-037 |
| Terminal-value maps (kani) | FR-029 and FR-030 terminal-value maps | TC-040, TC-041 |
| Counterexample replay | FR-016 witness decode and native replay, FR-024 submission in QSL's counterexample envelope | TC-026, TC-035 |
| Capability settlement and routing | FR-019 settlement, FR-022 routed generation | TC-030, TC-033 |

FR-018 and FR-014 refuse the
model graph, relation, temporal and protocol families with typed blockers. Function application is
FR-021's.

## Subsystems

Specification files are grouped by subsystem under `spec/<subsystem>/`, following
`ix://agent-ix/quire-contract-ir/ADR-0056` (the layout `quire-contract-ir` already uses). Identifiers
stay flat and globally sequential; the directory carries the subsystem. Each subsystem directory
holds `stakeholder/`, `functional/`, `non-functional/` and `matrix/` as it needs them: the matrix is
`matrix/tests.md` and the `TC-###` artifacts it declares sit beside it. Architecture descriptions
are in `assurance/`, decision records in `decisions/`, and every SpecReview in the repository-root
`reviews/`. [tests.md](tests.md) indexes the matrices.

### Subsystem Registry

| Subsystem | Path | Role | Owning crates/modules | ADs | Owner |
| --- | --- | --- | --- | --- | --- |
| Core | `spec/core/` | The stakeholder need, the library and CLI interface and publication conformance, and the reproducibility, atomic-publication and provenance-boundary properties every subsystem shares | `lib` (crate root), `publication`, `oracle` (the shared lowering core, imported by strategy, kani and oracle modules) | AD-001, quire-contract-ir:ADR-0056 | Contract codegen lane |
| Strategy | `spec/strategy/` | Tri-state proptest harnesses, bound numeric and state strategies, constructive populations, boundary campaigns and shrinking, and the strategy output consumable by Contract Runtime | `harness`, `strategy`, `bound`, `bound_strategy` | AD-001, ADR-001 | Contract codegen lane |
| Oracle | `spec/oracle/` | Exact complete-V1 scalar, composite-equality and function-application oracle generation and its agreement with the runtime | `exact_scalar`, `composite_equality`, `exact_function`, `generation` | AD-001, ADR-001 | Contract codegen lane |
| Kani | `spec/kani/` | Bounded Kani obligation generation, the generated subject ABI, proof ceilings, execution evidence, and the maps from a Kani run outcome and a Contract IR Kani outcome to QSL's terminal value | `kani`, `kani_obligations`, `kani_execution`, `kani_transcript`, `bounded_kani_profile`, `bounded_kani_corpus`, `definedness_arithmetic`, `bounded_collections`, `finite_reference_graphs`, `state_frame` | AD-001, AD-003, ADR-002, ADR-003, ADR-004 | Contract codegen lane |
| Routed | `spec/routed/` | Capability settlement at one negotiation point, routed generation per backend kind, and the backend adapter contract | `capability`, `routed_generation` | AD-001, ADR-002 | Contract codegen lane |
| Replay | `spec/replay/` | Witness decoding and native replay of Kani counterexamples, and their submission in QSL's counterexample envelope | `replay::witness`, `replay::function`, `replay::frame`, `kani::output::playback` | AD-001, AD-002, AD-003, ADR-001 | Contract codegen lane |
| Evidence | `spec/evidence/` | Vacuity and unexecuted-flow evidence | `vacuity`, `bound_coverage` | AD-001, AD-003 | Contract codegen lane |

Two recorded exceptions to the layout convention, both to be settled at the layout AD (IR-344):

- FR-005 and its TC-001, TC-002 and TC-007 stay in `core` although FR-005 depends on FR-002
  (strategy), FR-015 (kani) and FR-004 (evidence), a deliberate exception to ADR-0056 rule 4: the
  CLI conformance requirement spans those subsystems and this repository has no publication
  subsystem yet.
- `core/matrix/suites.md` (SUR-001, a SuiteRegistry) is not a file kind ADR-0056 allows in
  `matrix/`; the ADR has no slot for it, so it sits beside the core matrix until IR-344
  settles where it goes.

## References

- [CG to QSL replay seam](assurance/AD-002-cg-qsl-replay-seam.md).
- [Evidence chain across IR, CG and QSL](assurance/AD-003-evidence-chain.md).
- [CG crate layout](assurance/AD-004-cg-crate-layout.md).
- ISO/IEC/IEEE 29148 (requirements engineering), per `standards_alignment`.

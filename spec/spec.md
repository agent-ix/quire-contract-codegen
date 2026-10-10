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
ADR-005 records the open original guardian-gate architecture decision and its activation hold.
[tests.md](tests.md) indexes the per-subsystem matrices that map every criterion to its test case.

| Area | Requirements | Test cases |
|---|---|---|
| Tri-state harnesses, vacuity and publication | FR-002, FR-004, FR-005 | TC-001 to TC-004, TC-006, TC-007 |
| No panic on a generation or analysis path | NFR-005 | TC-042 |
| Bound numeric and state strategies | FR-008 to FR-013, NFR-004 | TC-017 to TC-022 |
| Complete-V1 oracles | FR-014 scalar, FR-018 composite equality, FR-021 function application | TC-024, TC-029, TC-031 |
| Boolean oracle integer arithmetic and comparison | FR-031 | TC-044 |
| Kani obligations and execution | FR-015 generation, FR-017 execution, FR-025 subject ABI, FR-028 bounds and ceilings, [FR-034](kani/functional/FR-034-caller-death-ownership.md) planned original-caller lifecycle ownership | TC-025, TC-027, TC-036, TC-039, [TC-049](kani/matrix/TC-049-caller-death-ownership.md) planned production guardian verification |
| Real-Kani lane gating | NFR-006 | TC-045 |
| Backend adapter (routed) | FR-026 adapter trait and registration | TC-037 |
| Terminal-value maps (kani) | FR-029 and FR-030 terminal-value maps | TC-040, TC-041 |
| Counterexample replay | FR-016 witness decode and native replay, FR-024 submission in QSL's counterexample envelope, [FR-032](replay/functional/FR-032-routed-scalar-replay-binding.md) planned scalar binding and proposition preservation, [FR-033](replay/functional/FR-033-composite-parity-replay-binding.md) planned composite equality parity | TC-026, TC-035, [TC-047](replay/matrix/TC-047-routed-scalar-replay-binding.md) and [TC-048](replay/matrix/TC-048-composite-parity-replay-binding.md) planned/gated |
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
| Kani | `spec/kani/` | Bounded Kani obligation generation, the generated subject ABI, proof ceilings, execution evidence, and the maps from a Kani run outcome and a Contract IR Kani outcome to QSL's terminal value | `src/kani/` (generation, finite-input lowerers, corpus, run, output and terminal maps) | AD-001, AD-003, AD-004, ADR-001 to ADR-005 | Contract codegen lane |
| Routed | `spec/routed/` | Capability settlement at one negotiation point, routed generation per backend kind, and the backend adapter contract | `capability`, `routed_generation` | AD-001, ADR-002 | Contract codegen lane |
| Replay | `spec/replay/` | Witness decoding and native replay of Kani counterexamples, and their submission in QSL's counterexample envelope | `replay::witness`, `replay::function`, `replay::frame`, `kani::output::playback` | AD-001, AD-002, AD-003, ADR-001 | Contract codegen lane |
| Evidence | `spec/evidence/` | Vacuity and unexecuted-flow evidence | `vacuity`, `bound_coverage` | AD-001, AD-003 | Contract codegen lane |

The current Kani module and public-surface ownership map is:

| Current source or public operation | Normative owner | State |
| --- | --- | --- |
| `kani/generate/{negotiate,outcome,scalar,precondition,contract,frame}` and `negotiate_kani_obligations` | FR-015; FR-025 for argument/subject ABI | V2 scalar and state/frame items live; V2 contract-clause items planned (FR-015-AC-38 to AC-49). |
| `kani/generate/{clause,v1_bundle}` and `generate_kani_bundle` | FR-015 interim bundle criteria; FR-025 ABI | V1 input and bundle live pending AD-004 steps 4c to 4f; no second enduring Kani generator is authorized by ADR-001 Q1. |
| `kani/generate/lower/{bounded_kani_profile,definedness_arithmetic,finite_reference_graphs,bounded_collections}` and their four public classifier/lowering operations | FR-015-AC-82 to AC-89 | Live finite-input auxiliaries pending the AD-004 V2 migration. |
| `kani/generate/corpus/bounded_kani_corpus` and `generate_bounded_kani_corpus_case` | FR-015-AC-51 to AC-58 and AC-90 to AC-95 | Live four-artifact finite-input corpus; its generated-case classification retires at AD-004 step 4g under AC-95, and its native replay is planned. |
| `kani/{abi,census,identity}` | FR-015 proof census and identity; FR-025 symbolic argument and subject ABI | Live support vocabulary; composite leaf ABI remains planned. |
| `kani/{run,output,classify}` and the execution operations | FR-017, with FR-028 proof ceilings and NFR-006's real-Kani lane | Live run and report processing. |
| `kani/terminal` | FR-029 and FR-030 | Live terminal maps; the backend adapter trait remains planned. |
| `replay/{witness,function,frame,state_clause}` and Kani playback scanning in `kani/output/playback` | FR-016 witness decode/native replay and FR-024 QSL envelope intake | Live function/frame paths; common typed intake and corpus replay remain planned. |

FR-003, FR-007 and FR-023 have no active CG requirement artifacts or generator
ownership. Their historical mentions do not supersede ADR-001's accepted FR-015
and `CheckedPackageV2` decisions. This map records the current implementation;
it does not mark V1 retirement or the full IR-329 reconciliation complete.

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

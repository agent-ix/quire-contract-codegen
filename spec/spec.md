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
generation of FR-014, FR-015, FR-019, FR-021 and FR-022. NFR-006 constrains FR-015, FR-017,
FR-031 and FR-041 by gating a change that can alter what real Kani proves on the real-Kani lane.
`interface-001` defines the serialized input, the library and CLI operations, the artifact bundle,
the diagnostics and the evidence contract. AD-001 describes the architecture and its seams to
Contract IR, Contract Runtime and QSL. ADR-001 to ADR-004 record the owner's decisions on the
generators, the input model, the backend adapter, Kani tractability and the generated subject ABI.
ADR-005 records the open original guardian-gate architecture decision and its activation hold.
ADR-006 proposes the distinct caller Text-admission route and scopes ADR-001's checked-package
input decision to expression and contract claims.
The planned direct-owner migration in
[FR-018](oracle/functional/FR-018-composite-equality-oracles.md) and
[interface-001](core/functional/interface-001-codegen-api.md) binds equality
generation and its emitted prepared environment to an optional genuine
replay-root `AdmittedContext` of the original checked package. Standalone
context-free traversals may use None; every reached nominal reconstruction
requires genuine context. An explicit wrong context always refuses before
classification, and absence does not authenticate source. The native
source-retaining route keeps mandatory full authentication. Exact owns kernel
values/types/metering, QSV owns declaration/equality/enum semantics, and Runtime
retains only its own residual functions. All enum cases use checker-issued
keys, including unused cases. The existing record/tuple closure requirements
remain; their genuine owner-key lookup is a separate capability gap in the
currently enum-only facade. Native enum qualification and the original
scalar proof qualification are distinct, both still required.
[tests.md](tests.md) indexes the per-subsystem matrices that map every criterion to its test case.

| Area | Requirements | Test cases |
|---|---|---|
| Tri-state harnesses, vacuity and publication | FR-002, FR-004, FR-005 | TC-001 to TC-004, TC-006, TC-007 |
| No panic on a generation or analysis path | NFR-005 | TC-042 |
| Bound numeric and state strategies | FR-008 to FR-013, NFR-004 | TC-017 to TC-022 |
| Complete-V1 oracles | FR-014 scalar, FR-018 composite equality, FR-021 function application, [FR-042](oracle/functional/FR-042-generated-checked-invariant-causes.md) typed checked-invariant causes | TC-024, TC-029, TC-031 |
| Generated-closure evaluator corpus | [FR-044](oracle/functional/FR-044-export-generated-closure-evaluation-corpus.md) planned FR-402 evaluator-case source export, with expectations from quire-eval once available | [TC-058](oracle/matrix/TC-058-generated-closure-evaluation-corpus.md) planned |
| Checked-package fixture identity ownership | [FR-037](core/functional/FR-037-fixture-identity.md) application and nominal fixture identities | [TC-052](core/matrix/TC-052-fixture-identity.md) planned consumer regression |
| Typed V2 operator selection | [FR-038](core/functional/FR-038-typed-operator-catalog.md) internal CG catalog over IR-admitted identities | Criterion-tagged V2 oracle and Kani consumer tests planned |
| Production ContractPackage consumption | [FR-039](core/functional/FR-039-contract-package-consumption.md) one admitted FR-330 package for negotiation and generation | [TC-053](core/matrix/TC-053-contract-package-consumption.md) planned production integration checks |
| Caller Text admission | [FR-035](oracle/functional/FR-035-caller-text-admission.md) proposed generated admission oracle and bounded proof; [FR-017](kani/functional/FR-017-kani-execution-evidence.md) planned caller-harness execution evidence | [TC-050](oracle/matrix/TC-050-caller-text-admission.md) planned runtime and real-Kani checks |
| Boolean oracle integer arithmetic and comparison | FR-031 | TC-044 |
| Kani obligations and execution | FR-015 generation, FR-017 execution, FR-025 subject ABI, FR-028 bounds and ceilings, [FR-034](kani/functional/FR-034-caller-death-ownership.md) planned original-caller lifecycle ownership, [FR-040](kani/functional/FR-040-cancellation-producer.md) live caller cancellation producer | TC-025, TC-027, TC-036, TC-039, [TC-049](kani/matrix/TC-049-caller-death-ownership.md) planned production guardian verification, [TC-055](kani/matrix/TC-055-cancellation-producer.md) tagged cancellation path |
| Finite rational function predicates | [FR-046](kani/functional/FR-046-rational-function-predicate-harnesses.md) planned whole-function proof and canonical finite encoding | [TC-060](kani/matrix/TC-060-rational-function-predicate-harnesses.md) planned generated-proof and replay controls |
| Current-V2 function proof producer | [FR-047](kani/functional/FR-047-v2-function-proof-producer.md) planned exact claim-occurrence harness and replay context | [TC-061](kani/matrix/TC-061-v2-function-proof-producer.md) planned installed proof, replay and refusal controls |
| Real-Kani lane gating | NFR-006 live installed-Kani lane controls; [FR-041](kani/functional/FR-041-claimed-module-proof-gate.md) current-build module discharge and mutation controls Open after retirement of the fixed V1 inventory and hold, with V2 implication and same-claim FR-003@4 producers planned | TC-045 live lane controls; [TC-056](kani/matrix/TC-056-claimed-module-proof-gate.md) module proof and mutation controls Open pending genuine current-build claims |
| Typed obligation inventories, criterion-keyed candidate tuples and separate producer facts | [FR-036](evidence/functional/FR-036-source-obligation-census.md) | [TC-051](evidence/matrix/TC-051-source-obligation-census.md) planned; Gated on IR-759 |
| Backend adapter (routed) | FR-026 adapter trait and registration | TC-037 |
| HP-2 SMT induction producer | [FR-045](routed/functional/FR-045-hyper-smt-induction-map.md) CG solver-run evidence and outcome handoff over QSL-owned hyper queries | [TC-059](routed/matrix/TC-059-hyper-smt-induction-map.md) partial |
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
| Evidence | `spec/evidence/` | Vacuity and unexecuted-flow evidence; planned complete source-obligation census and separate producer facts | `vacuity`, `bound_coverage`; planned source census export | AD-001, AD-003, [AD-005](assurance/AD-005-source-obligation-producer.md) | Contract codegen lane |

The current Kani module and public-surface ownership map, measured against the
source tree and crate-root re-exports, is:

| Current source or public operation | Normative owner | State |
| --- | --- | --- |
| `kani/generate/{negotiate,outcome,scalar,frame}`; `negotiate_kani_obligations`, `negotiate_kani_native_obligations`, `generate_state_frame_obligations`, and the public request/result types | FR-015; FR-025 for argument/subject ABI | Live V2 scalar, clause, collection and state/frame dispatch. V2 invariant rendering and full contract-family controls remain planned (IR-266, IR-489, IR-917). |
| `kani/generate/{record,spec,render}` and `kani/generate/collection` | FR-015; FR-025 for recorded harness identity and ABI | Validated private `HarnessSpec` and shared renderer are live for scalar, V2 parameter-precondition cover and symbolic collection after CG #588. Collection AC-96/99 remain partial; the renderer is not proof of their installed-backend controls. |
| `kani/generate/clause.rs` and `kani/generate/clause/v2.rs` | FR-015 V2 clause criteria, including AC-38 to AC-49 | `clause.rs` is now only the V2 module declaration. Supported V2 parameter precondition and postcondition paths exist. AC-44/45 census validation and persisted readiness are implemented and tagged; AC-38 to AC-43, AC-46, AC-48/49 and full AC-47 remain planned or partial. Invariant still refuses (IR-266); QSL-authored clause mutation control remains open (IR-917). |
| `kani/generate/census_validation.rs`, `kani/census.rs` and their public diagnostic/dependency types | FR-015-AC-22/25/44/45 for V2 census; FR-015 bounded-corpus criteria | The V2 clause path uses typed census requests from `kani/census.rs`. `census_validation.rs` remains for the live corpus and exposes `KaniDiagnostic`/`KaniErrorCode`; it no longer supports a V1 bundle. Its remaining helpers retire with their last corpus callers under AD-004 step 4g. |
| `kani/generate/corpus/{mod.rs,bounded_kani_corpus.rs}`; `generate_bounded_kani_corpus_case` and its public request/result types | FR-015-AC-51 to AC-58, AC-90 to AC-95, AC-109 to AC-111 and AC-123 until AC-111 | Live graph/collection finite-input corpus. Arithmetic retired at AC-109; collection AC-110 and graph/final API AC-111 remain planned, gated as those criteria state. Corpus replay and the API's AC-123 direct-result obligation persist until AC-111. |
| `kani/generate/lower/{mod.rs,bounded_kani_profile.rs,finite_reference_graphs.rs,bounded_collections.rs}`; `classify_bounded_kani_profile`, `prepare_finite_graph_reaches`, `prepare_bounded_collection_query` and public types | FR-015-AC-82 to AC-89, AC-109 to AC-111 and AC-123 while each operation remains | Remaining finite-input auxiliaries are live. The collection lowerer and its AC-123 direct-result obligation retire at AC-110; graph/profile lowerers and module entry retire at AC-111, ending the graph operation's AC-123 direct-result obligation. |
| `kani/{abi,identity}.rs` and the public Kani ABI, identity and harness types | FR-015 proof identity; FR-025 generated subject ABI; FR-028 ceilings | Live support vocabulary. Composite leaf ABI remains planned. |
| `kani/run/{execute,harness,launch,memory,namespace,owners,process,report_file,scalar,tool,mod.rs}` and the public execution, native scalar, process, tool and memory operations/types | FR-017 execution/evidence; FR-028 ceilings; FR-034 caller-death ownership; FR-040 cancellation producer; NFR-006 lane | Live bounded run and report handling. FR-040 caller cancellation is live through `KaniExecutionRequest.cancel`, bounded launcher observation and typed cancellation results (IR-701, CG #383). FR-034 original-caller guardian remains planned; existing launch ownership does not complete it. |
| `kani/output/{diagnostics,playback,report,mod.rs}`, `kani/classify.rs` and the public parse/classification/report types | FR-017 output and verdict classification; FR-016 playback decode; FR-024 state-clause replay intake through `parse_state_clause_falsification` | Live output processing, playback extraction and the public FR-024 parser added by CG #605. |
| `kani/terminal.rs` and public terminal-map operations | FR-029 and FR-030 | Live terminal maps; the backend adapter trait remains planned under FR-026. |
| `kani/generate/role_parity_tests.rs` and `kani/test_support.rs` | FR-014-AC-38 and FR-015-AC-60/68; FR-015/FR-017 tests using shared helpers | Test-only role parity and fixture helpers. |
| `kani/mod.rs` and `kani/generate/mod.rs` | The child-module owners above | Module declarations and test-only helpers; no separate generation API. |
| `replay/{witness,function,frame,state_clause}` and Kani playback scanning in `kani/output/playback.rs` | FR-016 witness decode/native replay and FR-024 QSL envelope intake | Live function/frame paths; common typed intake remains planned. Corpus replay remains live until FR-015-AC-111. |

The reverse inventory is: FR-015 owns generation, census, corpus and proof identity;
FR-025 owns the generated binding/subject ABI; FR-017 owns execution and run
classification; FR-028 and NFR-006 constrain ceilings and the real-Kani lane;
FR-029/030 own terminal mapping; FR-016/024 own playback and envelope intake,
including FR-024's public parser in `kani/classify.rs`. FR-040's cancellation producer
is live in `kani/run/execute.rs` and `kani/run/launch.rs`, with tagged tests for
AC-1 to AC-7. FR-041's module discharge and mutation controls remain Open:
the fixed V1 claim inventory, its held row, and its coupled test were retired.
`scripts/cg_gates.rs` and `tests/it/kani_gate.rs` retain the separate NFR-006
real-Kani lane controls. FR-041's V2 implication and historical FR-003@4
claims (AC-6 to AC-10) remain planned. FR-034 and
FR-046 retain planned obligations without a completed implementation claim here.
FR-015's V2 contract/invariant, composite and graph obligations remain planned
or partial where their individual criteria say so.
The computed matrix supplies trace-tag status, not a proof verdict: a tagged
criterion can still have a partial or planned installed-backend obligation.

The V1 bundle file, V1-only clause arm and `generate_kani_bundle` public operation
were deleted by CG #601; they have no current module or public owner to assign.
FR-003, FR-007 and FR-023 have no active CG requirement artifacts or generator
ownership. Their historical mentions do not supersede ADR-001's accepted FR-015
and `CheckedPackageV2` decisions. This inventory records the current implementation
and leaves the planned obligations intact.

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

## Developer Gate Adoption

[FR-043](core/functional/FR-043-owned-lock-coherence-gate.md) retires the planned
IR-641 lock-coherence gate under IR-1105. Cargo's manifest and lock remain the
dependency authority; `make deny` retains its unrelated `cargo deny` checks.

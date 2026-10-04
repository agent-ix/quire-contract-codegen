---
id: FR-030
title: "Map every Contract IR Kani outcome to exactly one QSL terminal value, preserving refusal causes"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/ADR-002
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-031
    type: references
---
# FR-030: Map every Contract IR Kani outcome to exactly one QSL terminal value, preserving refusal causes

## Description

The generator shall map every Contract IR `KaniOutcome`, paired with the settlement of its replay, to
exactly one `qsl_replay::TerminalValue`. The map is total over the pair (outcome, replay
settlement), not over the outcome alone, and it preserves the outcome's refusal cause and, for
a refusal before Kani runs, its catalog code: the outcome
kinds that QSL's terminal value collapses into one variant stay distinguishable through that
variant's typed cause. This is QSL ADR-013 C-09 as merged, whose two inputs are the IR `KaniOutcome`
and, for a `Counterexample`, the result of its replay. The replay settlement, its six readings and
the rules that classify them are those of [FR-029](./FR-029-run-outcome-terminal-record.md), which
this requirement refers to and does not restate; they apply here to a `Counterexample` exactly as
they apply there to `falsified`.

Contract IR retired its own outcome-to-terminal map (Contract IR FR-031-AC-5, Linear IR-358) because the
terminal value belongs to QSL and Contract IR must not depend on QSL. This repository owns the map.

Two maps exist and their domains do not overlap. [FR-029](./FR-029-run-outcome-terminal-record.md)
maps this repository's own `KaniRunOutcome`, which describes a run this repository executed. This
requirement maps a `KaniOutcome` that Contract IR's Kani boundary produced and handed in. One run
has one outcome of one of those two types and so one terminal value. Precedence: when this
repository executed the run, FR-029 governs and FR-030 is not applied to any outcome derived from
that run; FR-030 applies only to an outcome that arrived from Contract IR and was not produced by
a run of this repository. Items settled at negotiation (`unsupported`, `requires-bound`,
`invalid-request`; [FR-019](../../routed/functional/FR-019-capability-settlement.md)) have no `KaniOutcome` and no
terminal value, so neither map applies to them.

## Inputs

- One `quire_contract_ir::kani::KaniOutcome`: its closed `KaniOutcomeKind` and its stable cause
  `code`. The IR outcome carries kind, `code`, `source_id` and `context` only; it has no
  SUCCESS-check count.
- For a `Proved` outcome, the SUCCESS-check count taken from the Kani transcript this generator
  parsed, passed to the map as an explicit input, as [FR-029](./FR-029-run-outcome-terminal-record.md)
  takes it.
- For a `Counterexample` outcome, the replay settlement of its counterexample, as
  [FR-029](./FR-029-run-outcome-terminal-record.md) states it. No other kind takes a settlement.

## Outputs

- One `qsl_replay::TerminalValue`.

## Behavior

- The generator shall map outcomes in exactly one function, public so the driver calls it, whose
  `match` over the pair (`KaniOutcomeKind`, replay settlement) has no wildcard arm, so a kind added
  to Contract IR fails to compile here.
- The generator shall map each outcome kind as the table states:

  | Contract IR kind | Result |
  |---|---|
  | `Proved`, with `n` SUCCESS checks from the transcript, `n` at least one | `Proved { success_checks: n }` |
  | `Proved`, with zero SUCCESS checks from the transcript | `Proved { success_checks: 0 }` |
  | `Counterexample`, with a reproduced replay | `Refuted` |
  | `Counterexample`, with a replay disagreement | `Inconclusive(InconclusiveCause::ReplayParity)` |
  | `Counterexample`, with a non-fault replay refusal | `Inconclusive(InconclusiveCause::ReplayRefused)` carrying the refusal's QSL catalog code |
  | `Counterexample`, with a setup refusal on data (a non-fault `CallSiteRefusal` or `DependencyLockError::Input`) | `Inconclusive(InconclusiveCause::ReplayRefused)` carrying the refusal's QSL catalog code |
  | `Counterexample`, with a fault or a CG defect (the errors FR-029-AC-11 lists) | `Failed` |
  | `Refused` | `Declined` with `ProofRefusalCause::Refused` and the catalog code of the refusal that caused it |
  | `InvalidInput` | `Declined` with `ProofRefusalCause::InvalidInput` and the catalog code of the refusal that caused it |
  | `IncompleteInput` | `Declined` with `ProofRefusalCause::IncompleteInput` and the catalog code of the refusal that caused it |
  | `Unavailable` with cause `kani_solver_absent` | `Unsupported(UnavailabilityCause::SolverAbsent)` |
  | `Unavailable` with cause `kani_backend_absent` | `Unsupported(UnavailabilityCause::BackendAbsent)` |
  | `Unavailable`, any other cause | `Unsupported(UnavailabilityCause::BackendAbsent)` |
  | `TimedOut` | `Incomplete(IncompleteCause::TimedOut)` |
  | `ResourceExhausted` | `Incomplete(IncompleteCause::ResourceExhausted)` |
  | `Cancelled` | `Incomplete(IncompleteCause::Cancelled)` |
  | `Inconclusive` with cause `kani_vacuous_proof` | `Proved { success_checks: 0 }` |
  | `Inconclusive`, any other cause | `Failed` |

- The generator shall produce `Declined` only from `Refused`, `InvalidInput` and `IncompleteInput`,
  which refuse the obligation's own input before Kani runs, so nothing was proved.
- The generator shall carry in `Declined` the catalog code of the refusal that caused the outcome,
  unchanged and not remapped: for `Refused`, the code of the `CheckedPackageRefusal` itself; for
  `InvalidInput`, `invalid_runtime_input` with its cause; for `IncompleteInput`, `missing_import`
  with cause `missing-selection`, naming the requested record.
- The generator shall not map a setup refusal that arises after Kani refuted to `Declined`: a
  non-fault `CallSiteRefusal` and `DependencyLockError::Input` are `Inconclusive(ReplayRefused)`
  with their QSL code, as [FR-029](./FR-029-run-outcome-terminal-record.md) states, because QSL
  reserves `Declined` for a refusal before any backend run. A repeated dependency identity is
  refused by QSL's `DependencyInput::new` and arrives as `DependencyLockError::Input` with
  `invalid_package`, so it is `ReplayRefused` too (FR-029-AC-14).
  `ReplayPackageError::InvalidFunction` and `FrameReplayError::Name` carry no QSL code and map to
  `Failed`.
- The generator shall map a `Counterexample` to `Refuted` only with a reproduced replay.
- The generator shall classify a fault and the CG-raised failures FR-029-AC-11 lists as
  [FR-029](./FR-029-run-outcome-terminal-record.md) states: by walking the whole error, and with
  `ReplayRefused` carrying only QSL's closed catalog of codes.
- The generator shall expose no `proof_category` function
  ([AD-003](../../assurance/AD-003-evidence-chain.md) E-9).
- The generator shall map no outcome to `Tested`.
- The generator shall use QSL's terminal-value type, defining none of its own.
- The generator shall not read the outcome's `source_id` or `context` to choose the result. It reads
  the kind, and the `code` only for `Unavailable` and `Inconclusive`, as the table states.
- The map preserves the refusal kind only. The outcome's stable `code` is not carried into the
  result, because `TerminalRecord` holds an item and a value only.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-030-AC-1 | Every pair (`KaniOutcomeKind`, replay settlement) that the map's input can express maps to exactly one `TerminalValue`. | Test (TC-041) |
| FR-030-AC-2 | `Refused`, `InvalidInput` and `IncompleteInput` map to `Declined` with `ProofRefusalCause::Refused`, `InvalidInput` and `IncompleteInput` respectively and with the catalog code of the refusal that caused each (the `CheckedPackageRefusal`'s own code; `invalid_runtime_input` with its cause; `missing_import` with cause `missing-selection`), so neither the cause nor the code is lost or remapped. | Test (TC-041) |
| FR-030-AC-3 | `TimedOut`, `ResourceExhausted` and `Cancelled` map to `Incomplete` with `IncompleteCause::TimedOut`, `ResourceExhausted` and `Cancelled` respectively. | Test (TC-041) |
| FR-030-AC-4 | `Proved` with a transcript count of three SUCCESS checks maps to `Proved { success_checks: 3 }`, `Proved` with a count of zero maps to `Proved { success_checks: 0 }`, and `Counterexample` with a reproduced replay maps to `Refuted`. | Test (TC-041) |
| FR-030-AC-5 | `Inconclusive` with cause `kani_vacuous_proof` maps to `Proved { success_checks: 0 }`, and `Inconclusive` with any other cause maps to `Failed`. | Test (TC-041) |
| FR-030-AC-6 | No outcome maps to `Tested`. | Test (TC-041) |
| FR-030-AC-7 | The map is one `match` over the pair (`KaniOutcomeKind`, replay settlement) with no wildcard arm. | Inspection (TC-041) |
| FR-030-AC-8 | `Unavailable` with cause `kani_solver_absent` maps to `Unsupported(SolverAbsent)`; with `kani_backend_absent` or any other cause it maps to `Unsupported(BackendAbsent)`. | Test (TC-041) |
| FR-030-AC-9 | `Counterexample` with a replay disagreement maps to `Inconclusive(ReplayParity)`, and with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying `ReplayRefusal::code()` of that refusal. | Test (TC-041) |
| FR-030-AC-10 | `Counterexample` with a fault, walked through every wrapper FR-029-AC-10 lists, and with each CG-raised failure FR-029-AC-11 lists, maps to `Failed`. | Test (TC-041) |
| FR-030-AC-11 | Across every replay settlement other than reproduced, `Counterexample` maps to a value other than `Refuted`. | Test (TC-041) |
| FR-030-AC-12 | `Counterexample` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`, each bare and wrapped, maps to `Inconclusive(ReplayRefused)` carrying that refusal's QSL catalog code, never to `Declined`. | Test (TC-041) |
| FR-030-AC-13 | `Counterexample` with a lock that selects one library identity twice, refused by QSL's `DependencyInput::new` and arriving as `DependencyLockError::Input`, maps to `Inconclusive(ReplayRefused)` carrying `invalid_package`. | Test (TC-041) |

## Dependencies

- **Upstream**: Contract IR's `KaniOutcome` (its FR-030, FR-031); QSL's `qsl-replay`, which defines
  `TerminalValue` and the replay result and refusal types; QSL ADR-013 O-16 and C-09, ADR-011 T-13;
  and FR-121; QSpec FR-331; [FR-029](./FR-029-run-outcome-terminal-record.md). The `Inconclusive`
  terminal value and its `ReplayParity` and `ReplayRefused` causes are not yet in QSL.
- **Downstream**: [TC-041](../matrix/TC-041-ir-outcome-terminal-map.md).

## Status

Planned (Linear IR-465, IR-358). No code implements this map at this revision. The code half is
blocked on QSL types that are not merged. Merged in QSL `main`, read at this revision:
`TerminalValue::Declined(ProofRefusalCause)`, with a cause and no code, `Unsupported` and
`Incomplete` exist, and `CallSiteRefusal::code()` and `DependencyInputRefusal::code()` are public.
`TerminalValue::Inconclusive`, `InconclusiveCause::ReplayParity` and `ReplayRefused(Code)`, and a
`Declined` that carries a code, are not in QSL `main`; they are pending in QSL (QSL-351, in
progress, with only the `ToolPin` deletion merged). FR-030-AC-2 (the code half), AC-9, AC-12 and
AC-13 cannot be built until those types merge; the cause half of AC-2 and the other rows can. QSL
ruled, relayed on IR-465 (a QSL ruling recorded by the planner), that vacuity stays
`Proved { success_checks: 0 }`, that a setup refusal after a refutation is `ReplayRefused`, as
[FR-029](./FR-029-run-outcome-terminal-record.md)'s Status states, and that `Declined { cause, code }`
carries the catalog code of the refusal that caused it, unchanged: the `CheckedPackageRefusal`'s
own code for `Refused` (for example `malformed_wire` or `invalid_package`), the input refusal's
`invalid_runtime_input` with its cause for `InvalidInput`, and `missing_import` with
`missing-selection` naming the requested record for `IncompleteInput`.

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

The generator shall map every Contract IR `KaniOutcome` to exactly one `qsl_replay::TerminalValue`.
The map is total, and it preserves the outcome's refusal cause: the outcome kinds that QSL's terminal
value collapses into one variant stay distinguishable through that variant's typed cause.

Contract IR retired its own outcome-to-terminal map (FR-031-AC-5, Linear IR-358) because the
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

## Outputs

- One `qsl_replay::TerminalValue`.

## Behavior

- The generator shall map outcomes in exactly one function whose `match` over `KaniOutcomeKind` has
  no wildcard arm, so a kind added to Contract IR fails to compile here.
- The generator shall map each outcome kind as the table states:

  | Contract IR kind | Result |
  |---|---|
  | `Proved`, with `n` SUCCESS checks from the transcript, `n` at least one | `Proved { success_checks: n }` |
  | `Proved`, with zero SUCCESS checks from the transcript | `Proved { success_checks: 0 }` |
  | `Counterexample` | `Refuted` |
  | `Refused` | `Declined(ProofRefusalCause::Refused)` |
  | `InvalidInput` | `Declined(ProofRefusalCause::InvalidInput)` |
  | `IncompleteInput` | `Declined(ProofRefusalCause::IncompleteInput)` |
  | `Unavailable` with cause `kani_solver_absent` | `Unsupported(UnavailabilityCause::SolverAbsent)` |
  | `Unavailable` with cause `kani_backend_absent` | `Unsupported(UnavailabilityCause::BackendAbsent)` |
  | `Unavailable`, any other cause | `Unsupported(UnavailabilityCause::BackendAbsent)` |
  | `TimedOut` | `Incomplete(IncompleteCause::TimedOut)` |
  | `ResourceExhausted` | `Incomplete(IncompleteCause::ResourceExhausted)` |
  | `Cancelled` | `Incomplete(IncompleteCause::Cancelled)` |
  | `Inconclusive` with cause `kani_vacuous_proof` | `Proved { success_checks: 0 }` |
  | `Inconclusive`, any other cause | `Failed` |

- The generator shall map no outcome to `Tested`.
- The generator shall use QSL's terminal-value type, defining none of its own.
- The generator shall not read the outcome's `source_id` or `context` to choose the result. It reads
  the kind, and the `code` only for `Unavailable` and `Inconclusive`, as the table states.
- The map preserves the refusal kind only. The outcome's stable `code` is not carried into the
  result, because `TerminalRecord` holds an item and a value only.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-030-AC-1 | Every `KaniOutcomeKind` maps to exactly one `TerminalValue`. | Test (TC-041) |
| FR-030-AC-2 | `Refused`, `InvalidInput` and `IncompleteInput` map to `Declined` with `ProofRefusalCause::Refused`, `InvalidInput` and `IncompleteInput` respectively, so no refusal cause is lost. | Test (TC-041) |
| FR-030-AC-3 | `TimedOut`, `ResourceExhausted` and `Cancelled` map to `Incomplete` with `IncompleteCause::TimedOut`, `ResourceExhausted` and `Cancelled` respectively. | Test (TC-041) |
| FR-030-AC-4 | `Proved` with a transcript count of three SUCCESS checks maps to `Proved { success_checks: 3 }`, `Proved` with a count of zero maps to `Proved { success_checks: 0 }`, and `Counterexample` maps to `Refuted`. | Test (TC-041) |
| FR-030-AC-5 | `Inconclusive` with cause `kani_vacuous_proof` maps to `Proved { success_checks: 0 }`, and `Inconclusive` with any other cause maps to `Failed`. | Test (TC-041) |
| FR-030-AC-6 | No outcome maps to `Tested`. | Test (TC-041) |
| FR-030-AC-7 | The map is one `match` over `KaniOutcomeKind` with no wildcard arm. | Inspection (TC-041) |
| FR-030-AC-8 | `Unavailable` with cause `kani_solver_absent` maps to `Unsupported(SolverAbsent)`; with `kani_backend_absent` or any other cause it maps to `Unsupported(BackendAbsent)`. | Test (TC-041) |

## Dependencies

- **Upstream**: Contract IR's `KaniOutcome` (its FR-030, FR-031); QSL's `qsl-replay`, which defines
  `TerminalValue`; QSL ADR-013 O-16; QSpec FR-331.
- **Downstream**: [TC-041](../matrix/TC-041-ir-outcome-terminal-map.md).

## Status

Implemented as `kani_terminal::ir_outcome_terminal_value`. Tracked under Linear IR-358.

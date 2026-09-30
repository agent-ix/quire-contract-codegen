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
[FR-029](./FR-029-run-outcome-terminal-record.md) maps this repository's own `KaniRunOutcome`; this
requirement maps the Contract IR outcome that the Kani boundary produces at negotiation and input
validation.

## Inputs

- One `quire_contract_ir::kani::KaniOutcome`: its closed `KaniOutcomeKind` and its stable cause
  `code`.

## Outputs

- One `qsl_replay::TerminalValue`.

## Behavior

- The generator shall map outcomes in exactly one function whose `match` over `KaniOutcomeKind` has
  no wildcard arm, so a kind added to Contract IR fails to compile here.
- The generator shall map each outcome kind as the table states:

  | Contract IR kind | Result |
  |---|---|
  | `Proved` | `Proved { success_checks: n }`, `n` at least one |
  | `Counterexample` | `Refuted` |
  | `Refused` | `Declined(ProofRefusalCause::Refused)` |
  | `InvalidInput` | `Declined(ProofRefusalCause::InvalidInput)` |
  | `IncompleteInput` | `Declined(ProofRefusalCause::IncompleteInput)` |
  | `Unavailable` | `Unsupported(UnavailabilityCause)` |
  | `TimedOut` | `Incomplete(IncompleteCause::TimedOut)` |
  | `ResourceExhausted` | `Incomplete(IncompleteCause::ResourceExhausted)` |
  | `Cancelled` | `Incomplete(IncompleteCause::Cancelled)` |
  | `Inconclusive` with cause `kani_vacuous_proof` | `Proved { success_checks: 0 }` |
  | `Inconclusive`, any other cause | `Failed` |

- The generator shall map no outcome to `Tested`.
- The generator shall use QSL's terminal-value type, defining none of its own.
- The generator shall not read the outcome's message, `source_id` or `context` to choose the result.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-030-AC-1 | Every `KaniOutcomeKind` maps to exactly one `TerminalValue`, and the map is one `match` with no wildcard arm. | Test (TC-041) |
| FR-030-AC-2 | `Refused`, `InvalidInput` and `IncompleteInput` map to `Declined` with `ProofRefusalCause::Refused`, `InvalidInput` and `IncompleteInput` respectively, so no refusal cause is lost. | Test (TC-041) |
| FR-030-AC-3 | `TimedOut`, `ResourceExhausted` and `Cancelled` map to `Incomplete` with `IncompleteCause::TimedOut`, `ResourceExhausted` and `Cancelled` respectively. | Test (TC-041) |
| FR-030-AC-4 | `Proved` and `Counterexample` map to `Proved` with at least one SUCCESS check and to `Refuted`; `Unavailable` maps to `Unsupported`. | Test (TC-041) |
| FR-030-AC-5 | `Inconclusive` with cause `kani_vacuous_proof` maps to `Proved { success_checks: 0 }`, and `Inconclusive` with any other cause maps to `Failed`. | Test (TC-041) |
| FR-030-AC-6 | No outcome maps to `Tested`. | Test (TC-041) |

## Dependencies

- **Upstream**: Contract IR's `KaniOutcome` (its FR-030, FR-031); QSL's `qsl-replay`, which defines
  `TerminalValue`; QSL ADR-013 O-16; QSpec FR-331.
- **Downstream**: [TC-041](../../test/complete-v1/TC-041-ir-outcome-terminal-map.md).

## Status

Planned. No code implements this map at this revision. Tracked under Linear IR-358.

---
id: FR-029
title: "Map every Kani run outcome, paired with its replay settlement, to QSL's terminal value in one total match"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/ADR-002
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-031
    type: references
---
# FR-029: Map every Kani run outcome, paired with its replay settlement, to QSL's terminal value in one total match

## Description

The Kani adapter shall map every `KaniRunOutcome`, with its inconclusive reason, paired with the
settlement of that run's replay, to QSL's FR-331 terminal value (`qsl_replay::TerminalValue`) in
one match with no catch-all arm ([ADR-002](../../decisions/ADR-002-backend-adapter-boundary.md)
Q3). The map is total over the pair (outcome, replay settlement), not over the outcome alone: a
falsified run's value is a function of both, because a counterexample is a refutation only once its
replay reproduces it. Every pair maps to exactly one terminal value, so every run item has exactly
one terminal record, as QSpec FR-331 requires.

The replay settlement is the result of replaying the falsified run's counterexample through
`qsl_replay::replay` (QSL ADR-013 C-09, ADR-011 T-13), read as one of:

- reproduced: the replay settled `WitnessSettlement::ReproducedWithEvaluatedWitness`;
- disagreement: the replay settled `WitnessSettlement::Inconclusive` with its `DisagreementCause`,
  or completed no value;
- refused: `qsl_replay::replay` returned a `ReplayRefusal` that is not a fault, carrying that
  refusal's QSL catalog code;
- fault: an `InternalFault` anywhere in the error the replay path returned;
- CG defect: a failure this repository raised before any replay result existed;
- setup refusal on data: this repository's replay setup was refused on data after the run was
  falsified, carrying a QSL catalog code (QSL-352).

The driver runs the Kani obligation and the replay, pairs the two and writes each item's terminal
record (QSL ADR-011 T-13); this repository provides the map and its typed inputs. A replay settlement
accompanies a falsified outcome only, because the replay replays a counterexample; every other
outcome is mapped with no settlement.

The map covers outcomes of runs only. An item that settles `unsupported`, `requires-bound` or
`invalid-request` settles at negotiation ([FR-019](../../routed/functional/FR-019-capability-settlement.md)), with a
warning naming its capability kind from `quire.capability-kind/v1` (QSpec FR-290) for
`unsupported`. It has no run, no artifact and no terminal value.

## Inputs

- One `KaniRunOutcome` ([FR-017](./FR-017-kani-execution-evidence.md),
  [FR-028](./FR-028-bounded-proof-ceilings.md)) and the SUCCESS check count its transcript reports.
- For a falsified outcome, the replay settlement of its counterexample, as the Description states
  it.

## Outputs

- One `qsl_replay::TerminalValue`.

## Behavior

- The Kani adapter shall map run outcomes in exactly one function, public so the driver calls it,
  whose `match` over the pair (`KaniRunOutcome` with `KaniInconclusiveReason`, replay settlement)
  has no wildcard arm.
- The Kani adapter shall map each pair as the table states:

  | Outcome | Replay settlement | Result |
  |---|---|---|
  | `verified`, with `n` SUCCESS checks, `n` at least one | none | `Proved { success_checks: n }` |
  | `inconclusive` with the vacuous-proof reason (zero SUCCESS checks) | none | `Proved { success_checks: 0 }` |
  | `cover-unsatisfied` | none | `Proved { success_checks: 0 }` |
  | `falsified` | reproduced | `Refuted` |
  | `falsified` | disagreement | `Inconclusive(InconclusiveCause::ReplayParity)`, carrying the `DisagreementCause` |
  | `falsified` | refused | `Inconclusive(InconclusiveCause::ReplayRefused)`, carrying the refusal's QSL catalog code |
  | `falsified` | setup refusal on data | `Inconclusive(InconclusiveCause::ReplayRefused)`, carrying its QSL catalog code |
  | `falsified` | fault | `Failed` |
  | `falsified` | CG defect | `Failed` |

  The remaining outcomes take no settlement:

  | Outcome | Result |
  |---|---|
  | `inconclusive` with the timed-out reason | `Incomplete(IncompleteCause::TimedOut)` |
  | `inconclusive` with the memory-exhausted reason | `Incomplete(IncompleteCause::ResourceExhausted)` |
  | `inconclusive` with the exhausted-unwind-bound reason | `Incomplete(IncompleteCause::ResourceExhausted)` |
  | `inconclusive` with the no-verdict reason | `Failed` |
  | `inconclusive` with the failure-without-counterexample reason | `Failed` |
  | `inconclusive` with the missing-cover-summary reason | `Failed` |

- QSL reads `Proved { success_checks: 0 }` as category `inconclusive` with the vacuity cause
  `KaniVacuousProof`, which is the vacuity record QSpec FR-331-AC-8 requires. A vacuous proof and a
  cover-unsatisfied run are both vacuous: the run established no property over a satisfiable
  domain.
- An exhausted unwind bound is a configured resource bound the run exhausted before it completed. A
  run with no verdict is a build, launcher or solver failure, so the tool failed, not the property.
- A failure without a counterexample and a missing cover summary each map to `Failed`: under the
  option vector every harness identity records, the backend prints a concrete playback for every
  failed check and a cover summary for every run, so either omission is the tool breaking its output
  contract, not a verdict on the property.
- The Kani adapter shall map a falsified outcome to `Refuted` only with a reproduced replay. A
  falsified outcome whose replay did not reproduce it is never `Refuted`.
- The Kani adapter shall classify a fault by walking the whole error the replay path returned, not
  by its top variant. A fault is `ReplayRefusal::Fault`, `ReplayRefusal::Admission` carrying
  `AdmissionFailure::Fault`, and `CallSiteRefusal::Fault`, which reaches the map directly and wrapped
  in `ReplayPackageError::CallSite` and in `FrameReplayError::CallSite`. Each maps to `Failed`.
- `Inconclusive(InconclusiveCause::ReplayRefused)` carries only a code from QSL's closed catalog of
  `ReplayRefusal` codes, never a code this repository defines. A failure this repository raises
  maps to `Failed`: `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm}`, the
  envelope failure, a Kani playback outside the harness proof bound, and a decode failure
  (`DecodeFailure`), which is a playback that does not type against the bindings this repository
  persisted. [AD-001](../../assurance/AD-001-codegen-architecture.md)'s failure view keeps each a
  distinct typed state before the map.
- The Kani adapter shall expose no `proof_category` function. A value's category is
  `TerminalValue::category()` of it and nothing else
  ([AD-003](../../assurance/AD-003-evidence-chain.md) E-9).
- The Kani adapter shall map no outcome to `Tested`.
- The generator shall use QSL's terminal-value type, defining none of its own and importing none
  from Contract IR.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-029-AC-1 | `verified` with three SUCCESS checks maps to `Proved { success_checks: 3 }`, and `falsified` with a reproduced replay maps to `Refuted`. | Test (TC-040) |
| FR-029-AC-2 | `inconclusive` with the vacuous-proof reason and `cover-unsatisfied` each map to `Proved { success_checks: 0 }`, whose QSL category is `inconclusive` with cause `KaniVacuousProof`. | Test (TC-040) |
| FR-029-AC-3 | The timed-out reason maps to `Incomplete(TimedOut)`, and the memory-exhausted and exhausted-unwind-bound reasons each map to `Incomplete(ResourceExhausted)`. | Test (TC-040) |
| FR-029-AC-4 | The no-verdict reason maps to `Failed`. | Test (TC-040) |
| FR-029-AC-5 | The failure-without-counterexample and missing-cover-summary reasons each map to `Failed`. | Test (TC-040) |
| FR-029-AC-6 | No outcome maps to `Tested`. | Test (TC-040) |
| FR-029-AC-8 | `falsified` with a replay disagreement, and `falsified` with a replay that completed no value, each map to `Inconclusive(ReplayParity)`, and the disagreement case carries the replay's `DisagreementCause`. | Test (TC-040) |
| FR-029-AC-9 | `falsified` with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying `ReplayRefusal::code()` of that refusal. | Test (TC-040) |
| FR-029-AC-10 | `falsified` with a fault maps to `Failed` for each of `ReplayRefusal::Fault`, `ReplayRefusal::Admission(AdmissionFailure::Fault)`, `CallSiteRefusal::Fault`, `CallSiteRefusal::Fault` wrapped in `ReplayPackageError::CallSite`, and `CallSiteRefusal::Fault` wrapped in `FrameReplayError::CallSite`. | Test (TC-040) |
| FR-029-AC-11 | `falsified` with each CG-origin failure (`SpineReplayError::UnboundArgument`, `FieldDelimiter`, `Transcript` and `WrongArm`, the envelope failure, a playback outside the harness proof bound, and a decode failure) maps to `Failed`, and no `Inconclusive(ReplayRefused)` value carries a code outside QSL's `ReplayRefusal` codes. | Test (TC-040) |
| FR-029-AC-12 | Across every replay settlement other than reproduced, `falsified` maps to a value other than `Refuted`. | Test (TC-040) |
| FR-029-AC-13 | `falsified` with a setup refusal on data that carries a QSL catalog code maps to `Inconclusive(ReplayRefused)` carrying that code. | Test (TC-040) |

## Dependencies

- **Upstream**: [FR-017](./FR-017-kani-execution-evidence.md) and
  [FR-028](./FR-028-bounded-proof-ceilings.md), which classify the outcome; QSL's `qsl-replay`, which
  defines `TerminalValue` and the replay result and refusal types; QSL ADR-013 O-16 and C-09, ADR-011
  T-13; QSL-351 (`Inconclusive(cause)`) and QSL-352 (catalogued refusal codes); QSpec FR-331.
- **Downstream**: [TC-040](../matrix/TC-040-run-outcome-terminal-record.md).

## Status

Planned (Linear IR-465). No code implements this map at this revision. The Cargo lock pins an older
`qsl-replay`, so the code lands with the lock move. Against QSL `main` when this revision was
written: `TerminalValue` has seven variants and no `Inconclusive`, `InconclusiveCause` has only
`KaniVacuousProof`, and `Proved` carries a `u32`. `ReplayParity`, `ReplayRefused` and the
`Inconclusive` variant are named in QSL's merged ADR-013 C-09 and land with QSL-351, so FR-029-AC-8,
AC-9 and AC-13 cannot be built until QSL-351 merges. AC-13 also needs QSL-352's catalogued codes to
be constructible from this repository; until then a setup refusal on data reaches the map as a CG
defect and maps to `Failed`
([AD-004](../../assurance/AD-004-cg-crate-layout.md) step 5).

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
- disagreement: the replay settled `WitnessSettlement::Inconclusive`, which always carries its
  `DisagreementCause` (`Verdicts`, `Witness`, or `NoValue` for a replay that completed no value);
- refused: `qsl_replay::replay` returned a `ReplayRefusal` that is not a fault, carrying that
  refusal's QSL catalog code (`ReplayRefusal::code()`);
- fault: an `InternalFault` anywhere in the error the replay path returned;
- CG defect: a failure this repository raised, which has no QSL catalog code: the errors
  FR-029-AC-11 lists, and this repository's own setup errors that carry no QSL code
  (`ReplayPackageError::InvalidFunction`, `FrameReplayError::Name` and
  `DependencyLockError::Duplicate`);
- setup refusal on data (HELD, see Status): a QSL refusal of the replay setup that this repository
  reaches after the run was falsified and that is not a `ReplayRefusal` returned by
  `qsl_replay::replay`: a `CallSiteRefusal` other than `Fault`, and `DependencyLockError::Input`
  carrying a `DependencyInputRefusal`, each bare or wrapped in `ReplayPackageError` or
  `FrameReplayError`.

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
  | `falsified` | setup refusal on data | HELD: no value is stated until QSL or the owner rules (Status) |
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
- The Kani adapter shall put only a code from QSL's closed catalog (`ReplayRefusal::code()`) inside
  `Inconclusive(InconclusiveCause::ReplayRefused)`, and shall define no code of its own for it.
- The Kani adapter shall map to `Failed` every failure this repository raises that has no QSL
  catalog code: `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm,
  Identity}`, `FrameReplayError::{Transcript, Envelope, Name}`, `ReplayPackageError::InvalidFunction`,
  `DependencyLockError::Duplicate`, a Kani playback outside the harness proof bound, and a decode
  failure (`DecodeFailure`), which is a playback that does not type against the bindings this
  repository persisted. A `ReplayRefusal` wrapped in `SpineReplayError::Refused` or
  `FrameReplayError::Refused` is the refused or fault reading by its walked content, not a CG
  defect. [AD-001](../../assurance/AD-001-codegen-architecture.md)'s failure view keeps each a
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
| FR-029-AC-8 | `falsified` with a replay disagreement of each `DisagreementCause` (`Verdicts`, `Witness` and `NoValue`) maps to `Inconclusive(ReplayParity)` carrying that `DisagreementCause`. | Test (TC-040) |
| FR-029-AC-9 | `falsified` with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying `ReplayRefusal::code()` of that refusal. | Test (TC-040) |
| FR-029-AC-10 | `falsified` with a fault maps to `Failed` for each of `ReplayRefusal::Fault`, `ReplayRefusal::Admission(AdmissionFailure::Fault)`, `CallSiteRefusal::Fault`, `CallSiteRefusal::Fault` wrapped in `ReplayPackageError::CallSite`, and `CallSiteRefusal::Fault` wrapped in `FrameReplayError::CallSite`. | Test (TC-040) |
| FR-029-AC-11 | `falsified` with each CG-raised failure that has no QSL code maps to `Failed`: `SpineReplayError::UnboundArgument`, `FieldDelimiter`, `Transcript`, `WrongArm` and `Identity`; `FrameReplayError::Transcript`, `Envelope` and `Name`; `ReplayPackageError::InvalidFunction`; `DependencyLockError::Duplicate`; a playback outside the harness proof bound; and a decode failure. No `Inconclusive(ReplayRefused)` value carries a code outside QSL's `ReplayRefusal` codes. | Test (TC-040) |
| FR-029-AC-12 | Across every replay settlement other than reproduced, `falsified` maps to a value other than `Refuted`. | Test (TC-040) |
| FR-029-AC-13 | HELD on a QSL or owner ruling (Status). `falsified` with a setup refusal on data maps to the value that ruling states. | Test (TC-040) |

## Dependencies

- **Upstream**: [FR-017](./FR-017-kani-execution-evidence.md) and
  [FR-028](./FR-028-bounded-proof-ceilings.md), which classify the outcome; QSL's `qsl-replay`, which
  defines `TerminalValue` and the replay result and refusal types; QSL ADR-013 O-16 and C-09, ADR-011
  T-13 and FR-121; QSpec FR-331. The `Inconclusive` terminal value and its `ReplayParity` and
  `ReplayRefused` causes are not yet in QSL.
- **Downstream**: [TC-040](../matrix/TC-040-run-outcome-terminal-record.md).

## Status

Planned (Linear IR-465). No code implements this map at this revision. The code half is blocked on
types QSL has not merged. Against QSL `main` when this revision was written: `TerminalValue` has
seven variants and no `Inconclusive`, `InconclusiveCause` has only `KaniVacuousProof`, and `Proved`
carries a `u32`. `TerminalValue::Inconclusive`, `InconclusiveCause::ReplayParity` and `ReplayRefused`
are named in QSL's merged ADR-013 C-09 as types `qsl-replay` gains, and are pending in QSL: what
merged under QSL-351 is #551, which deletes the `ToolPin` only. FR-029-AC-8 and AC-9 cannot be built
until those types merge. `ReplayRefusal::code()`, `CallSiteRefusal::code()` and
`DependencyInputRefusal::code()` are already in QSL `main` and in the `qsl-replay` this repository
locks, so no catalog code is missing.

Vacuous and cover-unsatisfied rows. Two statements conflict, and neither is decided here. Merged
ADR-013 C-09 keeps a vacuous proof as `Proved { success_checks: 0 }` and never produces
`Inconclusive(KaniVacuousProof)`; the rows above and FR-029-AC-2 follow it, and it holds now.
[AD-003](../../assurance/AD-003-evidence-chain.md) R-Q1 option B and
[AD-004](../../assurance/AD-004-cg-crate-layout.md) step 5 expect a `NonZero` `Proved` with vacuity
moved into `Inconclusive(cause)`; that is pending in QSL and would change FR-029-AC-2.

Held item: setup refusal on data (FR-029-AC-13, FR-030-AC-12). AD-003 R-Q1 (b) states that a
`CallSiteRefusal` other than `Fault`, reached after Kani refuted, settles
`Inconclusive(ReplayRefused)` with a QSL code. Merged QSL FR-121 states that every `CallSiteRefusal`
other than `Fault` refuses the obligation's own input before any backend run and that a consumer
settles it `declined` with its code, and that only a `ReplayRefusal` after a backend refutation
settles `inconclusive`, `ReplayRefused`. This repository calls `call_site` after Kani refuted, so
FR-121's stated premise does not hold here, and `Declined` carries no code. Which value the class
takes is a QSL or owner ruling and is open; the table row and both criteria stay held, and until
the ruling the code cannot make the map total over this class.
